// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Compile-time parser decision classification (tool side).
//!
//! [`DecisionClassifier`] answers one question per prediction decision of a
//! compiled parser unit: can an SLL DFA be built for it statically, and what
//! does that DFA cost? The answer drives `--static-parser-dfa`,
//! `--report-decisions`, `--dump-dfas`, and the `G4S190` diagnostics.
//!
//! # Where the machinery lives
//!
//! The classifier is a *driver*: it owns the analysis policy, the subset
//! construction over token intervals, and the report vocabulary, while the
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
//! A decision ends in one [`Category`] plus, when the construction succeeded,
//! a [`StaticDfaTable`] snapshot: the DFA states, their accept/fallback/escape
//! verdicts, and their token edges. The snapshot is what `--dump-dfas` prints
//! and what the table emission phase serializes, so it is plain owned data -
//! the arena and the simulator are dropped as soon as a decision is done.

#[cfg(test)]
mod tests;

mod category;
mod classifier;
mod closure;
mod loop_block;
mod postfix;
mod table;
mod tables;

pub(crate) use category::Category;
pub(crate) use classifier::{DecisionClassifier, DecisionResult, MAX_DFA_STATES};
pub(crate) use closure::DEFAULT_VISIT_BUDGET;
pub(crate) use table::StaticDfaTable;
pub(crate) use tables::build_tables;

/// Table marker of a state whose decision must be recomputed by the adaptive
/// simulation: the static construction stopped here.
pub(crate) const ESCAPE: i32 = -1;

/// Table marker of a state where the static construction found an SLL conflict
/// it does not trust; the runtime can skip the SLL stage entirely.
pub(crate) const CONFLICT_ESCAPE: i32 = -2;
