// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]

#[cfg(feature = "generator")]
mod config;
#[cfg(feature = "generator")]
mod descriptor;
#[cfg(feature = "generator")]
mod manifest;
#[cfg(feature = "generator")]
mod metrics;
mod options;
#[cfg(feature = "generator")]
mod parser_dfa;
#[cfg(feature = "generator")]
mod report;
#[cfg(feature = "generator")]
mod static_lexer_dfa;
#[cfg(feature = "generator")]
mod static_parser_dfa;

#[cfg(feature = "generator")]
pub(crate) use config::{AtnFallback, OptimizationPlan, StaticDfaPass};
#[cfg(feature = "generator")]
pub(crate) use metrics::{StructuralMetrics, integrated_grammar_metrics};
pub use options::OptimizationOptions;
#[cfg(feature = "generator")]
pub(crate) use report::OptimizationReport;
#[cfg(feature = "generator")]
pub(crate) use static_lexer_dfa::{StaticLexerDfa, StaticLexerTables};
#[cfg(feature = "generator")]
pub(crate) use static_parser_dfa::{StaticParserDfa, StaticParserTables};
