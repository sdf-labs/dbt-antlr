// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Owned snapshot of a decision DFA, plus the graph queries the report needs.

use std::collections::VecDeque;

use dbt_antlr_runtime::dfa::static_parser_dfa::ParserContinuation;

use super::{CONFLICT_ESCAPE, ESCAPE, EscapeResolution, GUARDED};

/// One DFA state of a decision table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticDfaState {
    /// `> 0` is the predicted alternative of an accept state, [`ESCAPE`] and
    /// [`CONFLICT_ESCAPE`] mark escapes, and `0` marks a decision state that
    /// has outgoing edges.
    pub(crate) accept: i32,
    /// Lowest alternative that already left the decision (a rule stop reached
    /// with an empty context). The runtime falls back to it when no edge
    /// matches the lookahead.
    pub(crate) fallback: i32,
    /// Number of tokens read to reach this state.
    pub(crate) depth: usize,
    /// Alternatives SLL could not separate here, when this state is a
    /// conflict.
    pub(crate) conflicts: Vec<i32>,
    /// The lowest conflicting alternative of a conflict escape state, when the
    /// table can hand the conflict over to full-context prediction directly.
    pub(crate) conflict_min_alt: i32,
    /// The alternative a guarded state resolves to, `0` when the state is not
    /// guarded. A guarded state takes its alternative unless the runtime stack
    /// guard defers.
    pub(crate) guarded_alt: i32,
    /// The closed config set of a frontier escape state, when SLL can resume
    /// from it instead of restarting at the decision start.
    pub(crate) continuation: Option<ParserContinuation>,
    /// The sorted alternatives that are still viable in an escape state. The
    /// list is empty for the other states.
    pub(crate) viable_alts: Vec<i32>,
    /// `true` for an escape state that a predicate in its config set made
    /// undecidable.
    pub(crate) predicated: bool,
    /// How `--static-dfa-only` replaced the escape verdict of this state, when
    /// it did.
    pub(crate) resolution: Option<EscapeResolution>,
    pub(crate) edges: Vec<StaticDfaEdge>,
}

impl StaticDfaState {
    pub(crate) fn is_escape(&self) -> bool {
        self.accept == ESCAPE || self.accept == CONFLICT_ESCAPE
    }

    pub(crate) fn is_accept(&self) -> bool {
        self.accept > 0
    }

    pub(crate) fn is_guarded(&self) -> bool {
        self.accept == GUARDED
    }
}

/// One token-interval edge of a state. `lo == hi` is a single-token edge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct StaticDfaEdge {
    /// Inclusive token type; `-1` is the EOF edge.
    pub(crate) lo: i32,
    /// Inclusive token type.
    pub(crate) hi: i32,
    /// Index into [`StaticDfaTable::states`].
    pub(crate) target: usize,
}

/// The built DFA of one decision (or of one precedence class).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticDfaTable {
    pub(crate) decision: usize,
    /// `true` when the edge graph has a cycle: the lookahead is unbounded.
    pub(crate) cyclic: bool,
    /// Longest path from the start state, `-1` for a cyclic table.
    pub(crate) max_k: i32,
    /// State `0` is the start state.
    pub(crate) states: Vec<StaticDfaState>,
    /// Sorted invoking states whose follow region reaches a guard root: the
    /// runtime guard defers when the parse stack carries one.
    pub(crate) guard_danger: Vec<i32>,
    /// Sorted invoking states whose follow region can pop out of its rule
    /// without reaching a guard root: the runtime guard continues up the stack.
    pub(crate) guard_pass: Vec<i32>,
}

impl StaticDfaTable {
    /// Finalizes the graph-level facts of a table built state by state.
    pub(super) fn finish(decision: usize, states: Vec<StaticDfaState>) -> Self {
        let cyclic = is_cyclic(&states);
        let max_k = if cyclic { -1 } else { longest_path(&states) };
        Self {
            decision,
            cyclic,
            max_k,
            states,
            guard_danger: Vec::new(),
            guard_pass: Vec::new(),
        }
    }

    pub(crate) fn escapes(&self) -> usize {
        self.states.iter().filter(|state| state.is_escape()).count()
    }

    /// The number of states a runtime stack guard decides.
    pub(crate) fn guarded(&self) -> usize {
        self.states
            .iter()
            .filter(|state| state.is_guarded())
            .count()
    }

    /// How much of the start state's alphabet leads to an accepting state.
    ///
    /// A table below [`super::classifier::HYBRID_MIN_COVERAGE_PERCENT`] is not
    /// worth its bytes: almost every lookahead would escape anyway. Escape
    /// states do not count as coverage - they are exactly the part of the
    /// alphabet the table does not decide.
    pub(crate) fn start_coverage(&self) -> f64 {
        let Some(start) = self.states.first() else {
            return 0.0;
        };
        let mut reaches_accept = vec![false; self.states.len()];
        let mut reverse = vec![Vec::new(); self.states.len()];
        for (index, state) in self.states.iter().enumerate() {
            for edge in &state.edges {
                reverse[edge.target].push(index);
            }
        }
        let mut work = VecDeque::new();
        for (index, state) in self.states.iter().enumerate() {
            if state.is_accept() || state.is_guarded() {
                reaches_accept[index] = true;
                work.push_back(index);
            }
        }
        while let Some(current) = work.pop_front() {
            for &previous in &reverse[current] {
                if !reaches_accept[previous] {
                    reaches_accept[previous] = true;
                    work.push_back(previous);
                }
            }
        }

        let mut total = 0_i64;
        let mut useful = 0_i64;
        for edge in &start.edges {
            let width = i64::from(edge.hi - edge.lo + 1);
            total += width;
            if reaches_accept[edge.target] {
                useful += width;
            }
        }
        if total == 0 {
            0.0
        } else {
            useful as f64 / total as f64
        }
    }

    /// A shortest lookahead path from the start state to `target`, one token
    /// per edge (the lowest token of the edge interval).
    pub(crate) fn witness(&self, target: usize) -> Option<Vec<i32>> {
        if target >= self.states.len() {
            return None;
        }
        let mut parent: Vec<Option<usize>> = vec![None; self.states.len()];
        let mut parent_token = vec![0_i32; self.states.len()];
        let mut queue = VecDeque::new();
        parent[0] = Some(0);
        queue.push_back(0_usize);
        while let Some(current) = queue.pop_front() {
            if current == target {
                break;
            }
            for edge in &self.states[current].edges {
                if parent[edge.target].is_none() {
                    parent[edge.target] = Some(current);
                    parent_token[edge.target] = edge.lo;
                    queue.push_back(edge.target);
                }
            }
        }
        parent[target]?;
        let mut path = Vec::new();
        let mut cursor = target;
        while cursor != 0 {
            path.push(parent_token[cursor]);
            cursor = parent[cursor]?;
        }
        path.reverse();
        Some(path)
    }

    /// The first escape state with the given marker.
    pub(crate) fn first_escape(&self, marker: i32) -> Option<usize> {
        self.states.iter().position(|state| state.accept == marker)
    }
}

fn is_cyclic(states: &[StaticDfaState]) -> bool {
    let mut color = vec![0_u8; states.len()];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for root in 0..states.len() {
        if color[root] != 0 {
            continue;
        }
        stack.push((root, 0));
        color[root] = 1;
        while let Some((state, edge_index)) = stack.last_mut() {
            let current = *state;
            if *edge_index < states[current].edges.len() {
                let target = states[current].edges[*edge_index].target;
                *edge_index += 1;
                match color[target] {
                    1 => return true,
                    0 => {
                        color[target] = 1;
                        stack.push((target, 0));
                    }
                    _ => {}
                }
            } else {
                color[current] = 2;
                stack.pop();
            }
        }
    }
    false
}

fn longest_path(states: &[StaticDfaState]) -> i32 {
    if states.is_empty() {
        return 0;
    }
    let mut memo = vec![-1_i32; states.len()];
    longest_path_from(0, states, &mut memo)
}

fn longest_path_from(state: usize, states: &[StaticDfaState], memo: &mut [i32]) -> i32 {
    if memo[state] >= 0 {
        return memo[state];
    }
    memo[state] = 0;
    let mut max_length = 0;
    for edge in &states[state].edges {
        max_length = max_length.max(1 + longest_path_from(edge.target, states, memo));
    }
    memo[state] = max_length;
    max_length
}
