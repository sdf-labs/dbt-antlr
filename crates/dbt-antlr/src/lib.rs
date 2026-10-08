// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! ANTLR v4 grammar compiler and Rust source generator for the `dbt-antlr-runtime`
//! runtime.
//!
//! The emission layer ([`dbt`]) renders recognizers from the minijinja
//! template `templates/dbt/Rust.stg.jinja` on top of the grammar analysis in
//! `grammar`. There are two ways to run a generation:
//!
//! - [`Config`]: the build-script API. Add this crate as a build dependency
//!   and call it from `build.rs`; the generated code compiles against the
//!   `dbt-antlr-runtime` crate. No tool installation is needed.
//! - The `dbt-antlr` binary ([`run_cli`]): the Java-tool-like command line
//!   for one-shot or checked-in generation.

#[cfg(feature = "atn-export")]
pub mod atn_export;
mod compile_error;
mod config;
pub mod dbt;
mod error;
#[allow(dead_code)]
pub(crate) mod grammar;
mod optimization;

pub use config::Config;
pub use error::{Diagnostic, Error, ErrorKind, Severity};

/// Version of this generator package.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Runs the `dbt-antlr` command using the process arguments and streams.
#[doc(hidden)]
pub fn run_cli() -> miette::Result<()> {
    dbt::cli::run_cli()
}
