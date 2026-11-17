// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! What the table builder decided about one prediction decision.

use std::fmt::{self, Display, Formatter};

use dbt_antlr_runtime::analysis::ATNStateRef;

use super::cost::DecisionCost;
use super::descent::DescentPlan;
use super::{DecisionBudget, EscapeResolutions, StaticDfaTable};

/// How the generated parser predicts one decision.
///
/// The kinds are the vocabulary of the optimization manifest, `--dump-dfas`,
/// and `--profile-decisions`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum DecisionKind {
    /// The alternatives have disjoint first tokens: plain LL(1) match arms in
    /// code, no table or `adaptive_predict`.
    Ll1,
    /// Fully covered by a static DFA table.
    Static,
    /// Covered by a partial DFA table -- some of its states escape to
    /// `adaptive_predict` or test a stack guard. `--static-dfa-only` resolves
    /// these states statically (via heuristic guesses).
    Hybrid,
    /// No static table: the whole decision is deferred to runtime
    /// `adaptive_predict`.
    #[default]
    Adaptive,
}

impl DecisionKind {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Ll1 => "LL1",
            Self::Static => "STATIC",
            Self::Hybrid => "HYBRID",
            Self::Adaptive => "ADAPTIVE",
        }
    }
}

impl Display for DecisionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Why a decision, or a precedence class of it, has no static table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum NoTableReason {
    /// The generated parser predicts non-greedy subrules only with
    /// `adaptive_predict`.
    NonGreedy,
    /// The decision has more alternatives than a table can encode.
    TooManyAlternatives,
    /// The precedence loop has no precedence predicates to build class
    /// tables from.
    EmptyPrecedenceLoop,
    /// A predicate is in front of the decision, so no lookahead decides it.
    /// The value is the first predicate as `(rule index, predicate index)`,
    /// when it is known.
    PredicatedStart(Option<(i32, i32)>),
    /// The start state has an SLL conflict that no static resolution
    /// decides.
    StartConflict,
    /// The closure or the expansion of the start state spent the state cap
    /// or the visit budget.
    StartBudget,
    /// The table escapes to `adaptive_predict` and decides too small a share
    /// of the start lookahead to be worth emitting.
    LowCoverage { coverage_percent: u32 },
    /// Some precedence classes have no table. `first` is why the first of
    /// them has none.
    PrecedenceClasses {
        failed: usize,
        total: usize,
        first: Box<NoTableReason>,
    },
}

/// The precedence classes of one precedence-loop decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrecedenceTables {
    /// The precedence cut points of the loop, ascending.
    pub(crate) cutoffs: Vec<i32>,
    /// One table per class: `p<=cutoffs[0]`, `p=cutoffs[i-1]+1..=cutoffs[i]`,
    /// and `p>cutoffs[last]`.
    pub(crate) tables: Vec<Option<StaticDfaTable>>,
    pub(crate) adaptive_classes: usize,
    pub(crate) escapes: usize,
}

/// What the table builder decided about one prediction decision.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DecisionOutcome {
    pub(crate) decision: usize,
    pub(crate) rule_index: usize,
    pub(crate) state_number: i32,
    pub(crate) kind: DecisionKind,
    /// Why the decision, or some precedence class of it, has no table.
    pub(crate) no_table: Option<NoTableReason>,
    /// Longest lookahead of a finite table, `-1` when unbounded or unknown.
    pub(crate) k: i32,
    pub(crate) num_dfa_states: usize,
    /// The shared-descent groups the table accepts with.
    pub(crate) descent: Option<DescentPlan>,
    pub(crate) conflict_witness: Option<Vec<i32>>,
    pub(crate) budget_witness: Option<Vec<i32>>,
    /// The alternative sets of the conflicts that no static resolution
    /// decides.
    pub(crate) conflicts: Vec<Vec<i32>>,
    pub(crate) escapes: usize,
    /// Share of the start alphabet that reaches an accept state, in percent.
    pub(crate) coverage_percent: u32,
    pub(crate) table: Option<StaticDfaTable>,
    pub(crate) precedence: Option<PrecedenceTables>,
    /// The escape states that `--static-dfa-only` resolved, set once every
    /// table of the decision predicts without `adaptive_predict`.
    pub(crate) static_only: Option<EscapeResolutions>,
    /// The cost of the table builds of the decision.
    pub(crate) cost: DecisionCost,
    /// The analysis budget of the decision.
    pub(crate) budget: DecisionBudget,
    /// The table escapes the states that the state cap or the visit budget
    /// did not let the construction expand.
    pub(crate) budget_cutoff: bool,
}

impl DecisionOutcome {
    pub(super) fn new(decision: usize, state: ATNStateRef) -> Self {
        Self {
            decision,
            rule_index: usize::try_from(state.get_rule_index()).unwrap_or(0),
            state_number: state.get_state_number(),
            k: -1,
            ..Self::default()
        }
    }

    /// Every table of the decision: its single table, or the tables of its
    /// precedence classes.
    pub(crate) fn tables(&self) -> impl Iterator<Item = &StaticDfaTable> {
        self.table.iter().chain(
            self.precedence
                .iter()
                .flat_map(|precedence| precedence.tables.iter().flatten()),
        )
    }

    /// `true` for the precedence-loop decision of a left-recursive rule.
    pub(crate) fn is_precedence_loop(&self) -> bool {
        self.precedence.is_some() || self.no_table == Some(NoTableReason::EmptyPrecedenceLoop)
    }

    /// `true` when the decision has at least one table.
    pub(crate) fn has_table(&self) -> bool {
        self.tables().next().is_some()
    }

    /// The guarded states of the tables of the decision.
    pub(crate) fn guarded_states(&self) -> usize {
        self.tables().map(StaticDfaTable::guarded).sum()
    }

    /// The states that accept with the virtual alternative of a shared-descent
    /// group.
    pub(crate) fn descent_group_states(&self) -> usize {
        self.tables()
            .map(StaticDfaTable::descent_group_states)
            .sum()
    }

    /// A short description of how the decision falls back at runtime: `0`
    /// for none, `all` for a decision without a table, the number of escape
    /// states, and `Ng` for `N` guarded states. A precedence-loop decision
    /// counts its escape states as `Ne` and its classes without a table as
    /// `Nc`.
    ///
    /// Under `--static-dfa-only`, the text counts the resolved escape states
    /// instead:
    ///
    /// - `g`: a stack guard decides the state. It returns the skip or the exit
    ///   when it defers.
    /// - `m`: the state predicts the minimum conflicting alternative.
    /// - `v`: the state predicts the minimum viable alternative.
    /// - `x`: the state reports a syntax error.
    pub(crate) fn fallback_text(&self) -> String {
        if let Some(resolved) = &self.static_only {
            let mut parts = Vec::new();
            if resolved.guarded > 0 {
                parts.push(format!("{}g", resolved.guarded));
            }
            if resolved.min_alt > 0 {
                parts.push(format!("{}m", resolved.min_alt));
            }
            if resolved.min_viable_alt > 0 {
                parts.push(format!("{}v", resolved.min_viable_alt));
            }
            if resolved.syntax_error > 0 {
                parts.push(format!("{}x", resolved.syntax_error));
            }
            if parts.is_empty() {
                return "0".to_owned();
            }
            return parts.join("+");
        }
        match self.kind {
            DecisionKind::Ll1 | DecisionKind::Static => "0".to_owned(),
            DecisionKind::Adaptive => "all".to_owned(),
            DecisionKind::Hybrid => {
                let guarded = self.guarded_states();
                let mut parts = Vec::new();
                if let Some(precedence) = &self.precedence {
                    if self.escapes > 0 {
                        parts.push(format!("{}e", self.escapes));
                    }
                    if precedence.adaptive_classes > 0 {
                        parts.push(format!("{}c", precedence.adaptive_classes));
                    }
                } else if self.escapes > 0 {
                    parts.push(self.escapes.to_string());
                }
                if guarded > 0 {
                    parts.push(format!("{guarded}g"));
                }
                if parts.is_empty() {
                    "0".to_owned()
                } else {
                    parts.join("+")
                }
            }
        }
    }

    /// `true` when the generated parser predicts the decision without
    /// `adaptive_predict`.
    pub(crate) fn is_fully_static(&self) -> bool {
        self.static_only.is_some() || matches!(self.kind, DecisionKind::Ll1 | DecisionKind::Static)
    }
}

#[cfg(test)]
mod tests {
    use super::super::table::StaticDfaState;
    use super::super::{ESCAPE, GUARDED};
    use super::*;

    fn state(accept: i32) -> StaticDfaState {
        StaticDfaState {
            accept,
            fallback: 0,
            depth: 0,
            conflicts: Vec::new(),
            conflict_min_alt: 0,
            guarded_alt: 0,
            guard_defer_alt: 0,
            guard_group: 0,
            continuation: None,
            viable_alts: Vec::new(),
            predicated: false,
            resolution: None,
            hazard: None,
            stack_hazard: None,
            edges: Vec::new(),
        }
    }

    fn table_with(escapes: usize, guarded: usize) -> StaticDfaTable {
        let mut states = vec![state(1)];
        states.extend((0..escapes).map(|_| state(ESCAPE)));
        states.extend((0..guarded).map(|_| state(GUARDED)));
        StaticDfaTable::finish(0, states)
    }

    #[test]
    fn fallback_text_includes_guarded_states_alongside_escapes_and_classes() {
        let hybrid = DecisionOutcome {
            kind: DecisionKind::Hybrid,
            escapes: 2,
            table: Some(table_with(2, 3)),
            ..DecisionOutcome::default()
        };
        assert_eq!(hybrid.fallback_text(), "2+3g");

        let precedence = DecisionOutcome {
            kind: DecisionKind::Hybrid,
            escapes: 2,
            precedence: Some(PrecedenceTables {
                cutoffs: vec![1, 2],
                tables: vec![Some(table_with(2, 3)), None],
                adaptive_classes: 1,
                escapes: 2,
            }),
            ..DecisionOutcome::default()
        };
        assert_eq!(precedence.fallback_text(), "2e+1c+3g");

        let guarded_class_with_adaptive = DecisionOutcome {
            kind: DecisionKind::Hybrid,
            escapes: 0,
            precedence: Some(PrecedenceTables {
                cutoffs: vec![1],
                tables: vec![Some(table_with(0, 4)), None],
                adaptive_classes: 1,
                escapes: 0,
            }),
            ..DecisionOutcome::default()
        };
        assert_eq!(guarded_class_with_adaptive.fallback_text(), "1c+4g");
    }
}
