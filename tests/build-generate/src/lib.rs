// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Dogfood for the build-script generation model: the recognizer under
//! [`exprlexer`] / [`exprparser`] is generated from `grammars/Expr.g4` by
//! this crate's `build.rs` via `dbt_antlr_codegen::Config`, then compiled
//! against `dbt-antlr-runtime` like any consumer crate would do.

// Generated recognizers: lint with the codegen templates, not the workspace
// lints.
#[allow(clippy::all, warnings)]
#[rustfmt::skip]
pub mod exprlexer {
    include!(concat!(env!("OUT_DIR"), "/exprlexer.rs"));
}

#[allow(clippy::all, warnings)]
#[rustfmt::skip]
pub mod exprparser {
    include!(concat!(env!("OUT_DIR"), "/exprparser.rs"));
}

#[allow(clippy::all, warnings)]
#[rustfmt::skip]
pub mod exprlistener {
    include!(concat!(env!("OUT_DIR"), "/exprlistener.rs"));
}

#[allow(clippy::all, warnings)]
#[rustfmt::skip]
pub mod exprbaselistener {
    include!(concat!(env!("OUT_DIR"), "/exprbaselistener.rs"));
}
