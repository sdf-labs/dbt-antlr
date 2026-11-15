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
    /// The alternatives have disjoint first tokens: plain LL(1) code
    /// predicts, without a table.
    Ll1,
    /// A non-greedy subrule: `adaptivePredict` predicts, without a table.
    NonGreedy,
    /// A table predicts every lookahead without `adaptivePredict`.
    Static,
    /// A table predicts, but some of its states escape to `adaptivePredict`
    /// or test a stack guard. `--static-dfa-only` resolves these states
    /// statically.
    Hybrid,
    /// The precedence loop of a left-recursive rule: one table per
    /// precedence class. A class without a table uses `adaptivePredict`.
    Precedence,
    /// No table: `adaptivePredict` predicts.
    #[default]
    Adaptive,
}

impl DecisionKind {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Ll1 => "LL1",
            Self::NonGreedy => "NON_GREEDY",
            Self::Static => "STATIC",
            Self::Hybrid => "HYBRID",
            Self::Precedence => "PRECEDENCE",
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
    /// `adaptivePredict`.
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
    /// The table escapes to `adaptivePredict` and decides too small a share
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
    pub(crate) saw_predicate: bool,
    pub(crate) first_predicate: Option<(i32, i32)>,
    /// The decision is an optional-postfix (`X Y?`) shape.
    pub(crate) has_postfix_shape: bool,
    /// Conflict states the optional-postfix take rule resolved to take.
    pub(crate) take_rule_fires: usize,
    /// Conflict states a guarded take resolved.
    pub(crate) guarded_takes: usize,
    /// Conflict states a guarded iterate resolved under the stack guard.
    pub(crate) guarded_iterates: usize,
    /// Conflict states the iterate reading resolved outright.
    pub(crate) iterate_accepts: usize,
    /// States that accept with the virtual alternative of a shared-descent
    /// group.
    pub(crate) descent_group_states: usize,
    /// The shared-descent groups the table accepts with.
    pub(crate) descent: Option<DescentPlan>,
    pub(crate) utrust_resolutions: usize,
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
    /// table of the decision predicts without `adaptivePredict`.
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
            DecisionKind::NonGreedy | DecisionKind::Adaptive => "all".to_owned(),
            DecisionKind::Hybrid if self.escapes > 0 => self.escapes.to_string(),
            DecisionKind::Hybrid => format!("{}g", self.guarded_states()),
            DecisionKind::Precedence => {
                let adaptive_classes = self
                    .precedence
                    .as_ref()
                    .map_or(0, |precedence| precedence.adaptive_classes);
                if self.escapes == 0 && adaptive_classes == 0 {
                    let guarded = self.guarded_states();
                    return if guarded > 0 {
                        format!("{guarded}g")
                    } else {
                        "0".to_owned()
                    };
                }
                let mut parts = Vec::new();
                if self.escapes > 0 {
                    parts.push(format!("{}e", self.escapes));
                }
                if adaptive_classes > 0 {
                    parts.push(format!("{adaptive_classes}c"));
                }
                parts.join("+")
            }
        }
    }

    /// `true` when the generated parser predicts the decision without
    /// `adaptivePredict`.
    pub(crate) fn is_fully_static(&self) -> bool {
        if self.static_only.is_some() {
            return true;
        }
        match self.kind {
            DecisionKind::Ll1 | DecisionKind::Static => true,
            DecisionKind::Precedence => {
                self.no_table.is_none()
                    && self
                        .tables()
                        .all(|table| table.escapes() == 0 && table.guarded() == 0)
            }
            DecisionKind::NonGreedy | DecisionKind::Hybrid | DecisionKind::Adaptive => false,
        }
    }
}
