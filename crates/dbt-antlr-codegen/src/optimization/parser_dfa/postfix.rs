// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Optional-postfix decisions (`X Y?`) and their take/skip conflict.
//!
//! A two-alternative block where alternative 1 enters a postfix element and
//! alternative 2 skips straight to the block end is a dangling-else shape. Its
//! take/skip conflict resolves to *take* when the skip reading cannot win on
//! its own. In that case every token the skip reading consumes is also
//! consumed by the take structure, so both readings meet at the same block
//! end.
//!
//! The rule needs two facts about a conflict set:
//!
//! - **Mirror** ([`PostfixShape::skip_mirrors`]): every skip configuration
//!   consumes for the decision's own discipline. Inside the decision
//!   invocation, it consumes the continuation of the decision rule. After a
//!   boundary pop, it consumes on the take path of an enclosing invocation (the
//!   dangling-else reading), or in an invocation whose deepest context frame
//!   returns into the decision's block end. Any other skip consumes tokens
//!   outside the discipline, and skip can genuinely win:
//!   - The alias family bottoms out elsewhere: its clause continuations
//!     (`LIMIT`, `FROM`, ...) follow the skipped alias.
//!   - A new invocation of the decision rule after the skip owns its tokens:
//!     in `s : r r ';'` with `r : 'A' y? | '-' 'B'` and `y : '-' 'B'`, the
//!     second `r` reads the `- B` of `A - B ;`.
//!
//!   [`PostfixShape::is_foreign_call`] marks the call that starts such an
//!   invocation, because widening and merged contexts can remove its frame
//!   later.
//! - **Real take**: a take configuration that is not widened attests that
//!   taking is viable.
//!
//! The analyzer also rejects shapes where an element of the rule follows the
//! optional one and its FIRST set intersects the optional's, whether that
//! element is mandatory or optional. There the skipped input can serve that
//! element instead (`TRIM(x)`, `IN foo`), so skip genuinely wins and the rule
//! must not apply.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::Arc;

use dbt_antlr_runtime::analysis::{
    ATN, ATNStateRef, ATNStateType, BOUNDARY_TAINT, BasicBlockStartState, BitSet,
    FOREIGN_LINEAGE_TAINT, IntervalSetBuf, MIDWAY_TAINT, NotSetTransition, PlusBlockStartState,
    PredictionContextRef, ROOTED_TAINT, RuleTransition, StarBlockStartState,
    TOKEN_MIN_USER_TOKEN_TYPE, TransitionType,
};

use super::closure::{in_popped_frame, positions_owing_input, rule_positions};

/// The optional-postfix shape of one decision.
#[derive(Clone, Debug)]
pub(super) struct PostfixShape {
    /// State number of the decision block's end state. The mirror check
    /// requires every descended skip configuration's deepest frame to return
    /// into this state.
    pub(super) block_end_state: ATNStateRef,
    /// The positions of the take alternative, from the decision to the block
    /// end. Rule calls contribute their follow states.
    take_states: Arc<BitSet>,
    /// The take-path positions that still owe input before the block end.
    midway_states: Arc<BitSet>,
}

impl PostfixShape {
    /// Detects the optional-postfix shape of a decision, or returns `None`.
    pub(super) fn analyze(atn: &ATN, decision_state: ATNStateRef) -> Option<Self> {
        let block = block_end_state(decision_state)?;
        let transitions = decision_state.get_transitions();
        if transitions.len() != 2 {
            return None;
        }
        if !epsilon_path_to(transitions[1].get_target(), block) {
            return None;
        }
        let first = first_element(transitions[0].get_target())?;
        if !first
            .get_transitions()
            .iter()
            .any(|transition| transition.try_as::<RuleTransition>().is_some())
        {
            return None;
        }

        let rule_stop = atn.rule_to_stop_state[decision_state.get_rule_index() as usize];
        let continuation = look(atn, block, rule_stop);
        let take = look(atn, transitions[0].get_target(), block);
        if continuation.intersection(&take).next().is_some() {
            return None;
        }

        Some(Self {
            block_end_state: block,
            take_states: Arc::new(rule_positions(transitions[0].get_target(), Some(block))),
            midway_states: Arc::new(positions_owing_input(
                atn,
                decision_state.get_rule_index(),
                block,
            )),
        })
    }

    /// Whether a skip configuration at `state` consumes for the decision's own
    /// discipline (see the module documentation).
    ///
    /// A configuration that did not pop the decision boundary is inside the
    /// decision invocation, so the decision rule's positions are its own. In
    /// the frame that a configuration popped into, only the take path of the
    /// enclosing invocation mirrors the take reading, and only after the
    /// configuration came back through the decision ([`MIDWAY_TAINT`]). In any
    /// other invocation, the deepest context frame must return into the block
    /// end.
    pub(super) fn skip_mirrors(
        &self,
        state: ATNStateRef,
        context: Option<PredictionContextRef<'_>>,
        taint: i32,
    ) -> bool {
        if taint & (FOREIGN_LINEAGE_TAINT | MIDWAY_TAINT | ROOTED_TAINT) != 0 {
            return false;
        }
        if state.get_rule_index() == self.block_end_state.get_rule_index() {
            if taint & BOUNDARY_TAINT == 0 {
                return true;
            }
            if in_popped_frame(context) {
                return self.is_take_state(state);
            }
        }
        context.is_some_and(|context| {
            bottom_frame_is(context, self.block_end_state, &mut HashSet::new())
        })
    }

    /// Whether a rule call of a boundary-popped configuration at `state`
    /// starts an invocation outside the decision's discipline: the call leaves
    /// the frame that the configuration popped into from a position off the
    /// take path.
    pub(super) fn is_foreign_call(
        &self,
        state: ATNStateRef,
        context: Option<PredictionContextRef<'_>>,
    ) -> bool {
        in_popped_frame(context) && !self.is_take_state(state)
    }

    /// Whether a boundary chase that returns into `state` lands in the middle
    /// of the take path of an enclosing invocation, which still owes input
    /// before its block end.
    pub(super) fn is_midway(&self, state: ATNStateRef) -> bool {
        self.is_take_state(state)
            && self
                .midway_states
                .contains(state.get_state_number() as usize)
    }

    fn is_take_state(&self, state: ATNStateRef) -> bool {
        self.take_states.contains(state.get_state_number() as usize)
    }
}

/// Whether every context frame adjacent to the empty context (the deepest
/// frame of each context branch) is a return into `end_state`.
///
/// A skip configuration mirrors the take reading only when it re-entered the
/// decision through this decision's own take path. A branch that reaches the
/// empty context without such a frame consumed tokens fully outside the
/// decision's discipline.
fn bottom_frame_is(
    context: PredictionContextRef<'_>,
    end_state: ATNStateRef,
    visited: &mut HashSet<*const ()>,
) -> bool {
    if context.is_empty() {
        return false;
    }
    if !visited.insert(context.as_ref() as *const _ as *const ()) {
        return true;
    }
    for index in 0..context.length() {
        let return_state = context.get_return_state(index);
        match context.get_parent(index) {
            Some(parent) if !parent.is_empty() => {
                if !bottom_frame_is(parent, end_state, visited) {
                    return false;
                }
            }
            _ => {
                if return_state != end_state {
                    return false;
                }
            }
        }
    }
    true
}

/// The block start state's end state, when `state` starts a block.
fn block_end_state(state: ATNStateRef) -> Option<ATNStateRef> {
    let number = state.get_state_number();
    if number < 0 {
        return None;
    }
    match state.state_type() {
        ATNStateType::BasicBlockStart => state
            .try_as::<BasicBlockStartState>()
            .map(|start| start.end_state),
        ATNStateType::StarBlockStart => state
            .try_as::<StarBlockStartState>()
            .map(|start| start.end_state),
        ATNStateType::PlusBlockStart => state
            .try_as::<PlusBlockStartState>()
            .map(|start| start.end_state),
        _ => None,
    }
}

/// Whether an epsilon-only path leads from `from` to `to`.
fn epsilon_path_to(from: ATNStateRef, to: ATNStateRef) -> bool {
    let mut visited: BitSet = BitSet::new();
    let mut work = vec![from];
    while let Some(state) = work.pop() {
        if state == to {
            return true;
        }
        if state.get_state_number() < 0 || !visited.insert(state.get_state_number() as usize) {
            continue;
        }
        for transition in state.get_transitions() {
            if transition.is_epsilon() {
                work.push(transition.get_target());
            }
        }
    }
    false
}

/// The first state with a non-epsilon outgoing transition reachable over
/// epsilon links. A rule invocation is an element, not an epsilon link.
fn first_element(start: ATNStateRef) -> Option<ATNStateRef> {
    let mut visited: BitSet = BitSet::new();
    let mut work = vec![start];
    while let Some(state) = work.pop() {
        if state.get_state_number() < 0 || !visited.insert(state.get_state_number() as usize) {
            continue;
        }
        for transition in state.get_transitions() {
            if transition.try_as::<RuleTransition>().is_some() || !transition.is_epsilon() {
                return Some(state);
            }
            work.push(transition.get_target());
        }
    }
    None
}

/// `LOOK(start)` until `stop` or the end of the start state's rule.
fn look(atn: &ATN, start: ATNStateRef, stop: ATNStateRef) -> BitSet {
    let mut tokens = BitSet::new();
    let mut visited: HashSet<(i32, usize)> = HashSet::new();
    let mut work: Vec<(ATNStateRef, Vec<ATNStateRef>)> = vec![(start, Vec::new())];
    while let Some((state, returns)) = work.pop() {
        let number = state.get_state_number();
        if number < 0 || state == stop || !visited.insert((number, returns.len())) {
            continue;
        }
        if state.state_type() == ATNStateType::RuleStop {
            if let Some(follow) = returns.last() {
                let mut next = returns.clone();
                next.pop();
                work.push((*follow, next));
            }
            continue;
        }
        for transition in state.get_transitions() {
            if let Some(rule) = transition.try_as::<RuleTransition>() {
                let mut next = returns.clone();
                next.push(rule.follow_state);
                work.push((transition.get_target(), next));
                continue;
            }
            if transition.is_epsilon() {
                work.push((transition.get_target(), returns.clone()));
                continue;
            }
            let label: IntervalSetBuf = match transition.transition_type() {
                TransitionType::NotSet => transition
                    .try_as::<NotSetTransition>()
                    .expect("not-set transition")
                    .get_label()
                    .expect("not-set label")
                    .complement(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type),
                TransitionType::Wildcard => {
                    let mut all = IntervalSetBuf::new();
                    if atn.max_token_type >= TOKEN_MIN_USER_TOKEN_TYPE {
                        all.add_range(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type);
                    }
                    all
                }
                _ => match transition.get_label() {
                    Some(label) => label.to_interval_set_buf(),
                    None => continue,
                },
            };
            add_intervals(&mut tokens, &label);
        }
    }
    tokens
}

fn add_intervals(tokens: &mut BitSet, label: &IntervalSetBuf) {
    for interval in label.as_interval_set().as_slice() {
        for token in interval.a..=interval.b {
            if token >= 0 {
                tokens.insert(token as usize);
            }
        }
    }
}

/// The ATN states that `start` reaches by epsilon, `start` included. Rule
/// stops are leaves.
pub(super) fn epsilon_closure(start: ATNStateRef) -> BitSet {
    let mut out = BitSet::new();
    let mut work = vec![start];
    while let Some(state) = work.pop() {
        let number = state.get_state_number();
        if number < 0 || !out.insert(number as usize) {
            continue;
        }
        if state.state_type() == ATNStateType::RuleStop {
            continue;
        }
        for transition in state.get_transitions() {
            if transition.is_epsilon() {
                work.push(transition.get_target());
            }
        }
    }
    out
}

/// The guard roots of one skip consumption outside the decision's discipline.
pub(super) fn collect_guard_roots(
    state: ATNStateRef,
    context: Option<PredictionContextRef<'_>>,
    mirror_return: Option<ATNStateRef>,
    sites: &HashMap<i32, i32>,
    out: &mut BTreeSet<i32>,
) {
    let Some(context) = context.filter(|context| !context.is_empty()) else {
        out.insert(state.get_state_number());
        return;
    };
    let mut visited: HashSet<*const ()> = HashSet::new();
    collect_roots_in(state, &context, mirror_return, sites, out, &mut visited)
}

fn collect_roots_in(
    state: ATNStateRef,
    context: &PredictionContextRef<'_>,
    mirror_return: Option<ATNStateRef>,
    sites: &HashMap<i32, i32>,
    out: &mut BTreeSet<i32>,
    visited: &mut HashSet<*const ()>,
) {
    if context.is_empty() {
        out.insert(state.get_state_number());
        return;
    }
    if !visited.insert(context.as_ref() as *const _ as *const ()) {
        return;
    }
    for index in 0..context.length() {
        let return_state = context.get_return_state(index);
        match context.get_parent(index) {
            Some(parent) if !parent.is_empty() => {
                collect_roots_in(state, &parent, mirror_return, sites, out, visited);
            }
            _ => {
                if return_state == ATNStateRef::empty_return() {
                    out.insert(state.get_state_number());
                } else if Some(return_state) != mirror_return {
                    let number = return_state.get_state_number();
                    out.insert(sites.get(&number).copied().unwrap_or(number));
                }
            }
        }
    }
}
/// The danger and pass sets of a guard-root set, as sorted invoking state
/// numbers.
///
/// `danger[state]`: the state invokes a rule whose follow region can reach a
/// guard root, so the take defers at such a frame. `pass[state]`: the follow
/// region can reach its own rule's stop without reaching a root, so the take is
/// resolved there. A state with several rule transitions is danger when any
/// follow is dangerous, and pass only when every follow can pop (conservative in
/// both directions).
pub(super) fn compute_guard_data(atn: &ATN, roots: &BTreeSet<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut danger = Vec::new();
    let mut pass = Vec::new();
    let mut reachable: HashMap<i32, BitSet> = HashMap::new();
    for state in atn.iter_states() {
        let rule_index = state.get_rule_index();
        let rule_stop = if rule_index >= 0 {
            atn.rule_to_stop_state[rule_index as usize]
        } else {
            ATNStateRef::invalid()
        };
        let mut any_danger = false;
        let mut all_pass = true;
        let mut has_call = false;
        for transition in state.get_transitions() {
            let Some(rule) = transition.try_as::<RuleTransition>() else {
                continue;
            };
            has_call = true;
            let follow = rule.follow_state.get_state_number();
            let region = reachable
                .entry(follow)
                .or_insert_with(|| epsilon_closure(rule.follow_state));
            if roots.iter().any(|root| region.contains(*root as usize)) {
                any_danger = true;
            }
            if rule_stop.get_state_number() < 0
                || !region.contains(rule_stop.get_state_number() as usize)
            {
                all_pass = false;
            }
        }
        if !has_call {
            continue;
        }
        if any_danger {
            danger.push(state.get_state_number());
        }
        if all_pass {
            pass.push(state.get_state_number());
        }
    }
    (danger, pass)
}
