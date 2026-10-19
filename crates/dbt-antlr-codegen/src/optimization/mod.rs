// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
#![warn(clippy::print_stdout)]
#![warn(clippy::print_stderr)]

#[cfg(feature = "generator")]
mod config;
#[cfg(feature = "generator")]
mod descriptor;
#[cfg(feature = "generator")]
pub(crate) mod metrics;
mod options;
#[cfg(feature = "generator")]
mod report;
#[cfg(feature = "generator")]
pub(crate) mod static_lexer_dfa;

#[cfg(feature = "generator")]
pub(crate) use config::OptimizationPlan;
pub use options::OptimizationOptions;
