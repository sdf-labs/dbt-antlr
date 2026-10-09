// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! ANTLR v4 grammar compiler and Rust source generator for the `dbt-antlr-runtime`
//! runtime.
//!
//! The emission layer renders recognizers from the minijinja template
//! `templates/dbt/Rust.stg.jinja` on top of the grammar analysis in `grammar`.
//! There are two ways to run a generation:
//!
//! - [`Config`]: the build-script API. Add this crate as a build dependency
//!   and call it from `build.rs`; the generated code compiles against the
//!   `dbt-antlr-runtime` crate. No tool installation is needed.
//! - The `dbt-antlr-codegen` binary: the Java-tool-like command line for
//!   one-shot or checked-in generation.
//!
//! # Crate features
//!
//! - `generator` (default): the in-process grammar compiler and emission
//!   layer. Disable it to use this crate as a thin client.
//! - `download`: enables [`Config::pinned_release`], which downloads and runs
//!   a pinned prebuilt release binary instead of compiling the generator.
//! - `fancy` (default): pretty miette diagnostics for the command line.
//! - `atn-export`: internal, for this workspace's own tests.
#[cfg(feature = "atn-export")]
pub mod atn_export;
#[cfg(feature = "generator")]
mod compile_error;
mod config;
#[cfg(feature = "generator")]
pub mod dbt;
#[cfg(feature = "download")]
mod download;
mod error;
#[cfg(feature = "generator")]
#[allow(dead_code)]
pub(crate) mod grammar;
#[cfg(feature = "generator")]
mod optimization;

pub use config::Config;
pub use error::{Diagnostic, EmitError, Error, ErrorKind, Severity};

/// Version of this generator package.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Runs the `dbt-antlr-codegen` command using the process arguments and
/// streams.
#[cfg(feature = "generator")]
#[doc(hidden)]
pub fn run_cli() -> miette::Result<()> {
    dbt::cli::run_cli()
}
