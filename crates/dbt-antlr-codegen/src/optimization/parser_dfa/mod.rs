// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Compile-time static DFA tables of the parser decisions (tool side).
//!
//! [`DecisionTableBuilder`] builds the static prediction table of each
//! prediction decision of a compiled parser unit, so that the generated parser
//! predicts without `adaptive_predict` wherever a table can decide. A state
//! that no table can decide soundly escapes to the adaptive simulation, or,
//! under `--static-dfa-only`, gets a static resolution. The tables drive
//! `--static-parser-dfa`, `--dump-dfas`, `--profile-decisions`, and the
//! `G4S190`, `G4S192`, and `G4S193` diagnostics. The budget of each decision
//! (see [`DecisionBudget`]) limits the construction.
//!
//! # Where the machinery lives
//!
//! The builder is a *driver*: it owns the construction policy, the subset
//! construction over token intervals, and the frontier of a table, while the
//! ATN/DFA mechanics stay in `dbt-antlr-runtime` behind the `static-dfa-build`
//! feature (see `dbt_antlr_runtime::analysis`):
//!
//! - A [`ParserATNSimulatorManager`] is created from the unit's `&'static ATN`;
//!   its `get_simulator` borrows an arena and yields the parser simulator.
//! - DFA states and token edges are interned through the simulator's own
//!   [`DFA`] (`add_state`, `set_edge`), so config-set deduplication, the edge
//!   store, and the memory accounting are the runtime's.
//! - Closures run on [`MutableATNConfigSet`] with the runtime's
//!   [`MergeCache`], so build-time contexts merge exactly like the lazy
//!   simulator's, and conflicts are decided by the runtime
//!   [`has_sll_conflict_terminating_prediction`].
//!
//! What is *not* shared is the closure *policy*. The runtime closure has an
//! input stream, so recursion is bounded by the input, predicates are
//! evaluated against the parser, and a rule stop pops a real caller frame.
//! A build-time closure has none of that: it must bound recursion by a depth
//! budget (widening), treat predicates symbolically, and chase grammar-wide
//! FOLLOW links to model "any caller". `closure` implements that policy and
//! keeps the resulting approximations as taint bits in a side map per config
//! set - never on the configs themselves, whose identity defines the DFA
//! states.
//!
//! # Output
//!
//! A decision ends in one [`DecisionOutcome`]: its [`DecisionKind`], the
//! [`NoTableReason`] when it has no table, and, when the construction
//! succeeded, a [`StaticDfaTable`] snapshot: the DFA states, their
//! accept/fallback/escape verdicts, and their token edges. The snapshot is
//! what `--dump-dfas` prints and what the table emission phase serializes, so
//! it is plain owned data - the arena and the simulator are dropped as soon as
//! a decision is done.

#[cfg(test)]
mod tests;

mod budget;
mod builder;
mod closure;
mod cost;
mod descent;
mod hazard;
mod loop_block;
mod outcome;
mod postfix;
mod static_only;
mod table;
mod tables;

#[cfg(test)]
pub(crate) use budget::DEFAULT_VISIT_BUDGET;
pub(crate) use budget::{
    BUDGET_OPTIONS, BudgetLimits, DecisionBudget, ParserDfaBudgets, budget_option_problem,
    has_non_budget_option,
};
pub(crate) use builder::{DecisionTableBuilder, PARTIAL_MIN_COVERAGE_PERCENT};
pub(crate) use cost::AttemptOutcome;
pub(crate) use descent::{DescentGroup, DescentPlan, DescentRules, TailArm};
pub(crate) use hazard::{HazardLevel, HijackVerdict, StackHijackVerdict};
pub(crate) use outcome::{BudgetCutoff, DecisionKind, DecisionOutcome, NoTableReason};
pub(crate) use static_only::{EscapeResolution, EscapeResolutions, GuardGroup, TieBreak};
pub(crate) use table::StaticDfaTable;
pub(crate) use tables::{build_tables, minimized_states};

/// Table marker of a state whose decision must be recomputed by the adaptive
/// simulation: the static construction stopped here.
pub(crate) const ESCAPE: i32 = -1;

/// Table marker of a state where the static construction found an SLL conflict
/// it does not trust; the runtime can skip the SLL stage entirely.
pub(crate) const CONFLICT_ESCAPE: i32 = -2;

/// Table marker of a state whose alternative is decided by a runtime stack
/// guard: the take is resolved unless the real parse stack can reach one of the
/// decision's guard roots.
pub(crate) const GUARDED: i32 = -3;
