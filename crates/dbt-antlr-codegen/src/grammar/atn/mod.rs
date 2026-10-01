// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
mod analysis;
mod build;
#[cfg(test)]
mod general_bug_test;
#[cfg(test)]
mod interp_test;
mod interval_set;
mod lexer;
mod lexer_atn;
mod optimize;
mod packed;
mod parser;
mod state_kind;

pub(crate) use analysis::DecisionLookahead;
pub(crate) use build::{FinalizedAtnGraph, FinalizedTransitionKind};
pub(crate) use interval_set::IntervalSet;
pub(crate) use lexer::{CompiledLexer, compile_lexer};
#[cfg(test)]
pub(crate) use packed::ParserAtn;
pub(crate) use parser::{CompiledParser, compile_parser};
pub(crate) use state_kind::AtnStateKind;

/// Version of the serialized ATN format emitted in the ATN word streams.
pub(crate) const SERIALIZED_VERSION: i32 = 4;
