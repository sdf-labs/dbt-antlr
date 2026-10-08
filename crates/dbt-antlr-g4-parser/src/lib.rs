// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Checked-in `ANTLRv4` recognizers and recovered source-syntax facade.
//!
//! This crate is a lockstep implementation dependency of
//! `dbt-antlr`. Its exported facade is intentionally narrow and is
//! not a standalone compatibility promise.

mod frontend;
#[doc(hidden)]
pub mod generated {
    // Generated recognizers: lint with the codegen templates, not the
    // workspace lints, and keep the files byte-identical to emitter output.
    #![allow(
        warnings,
        unsafe_code,
        missing_docs,
        clippy::all,
        clippy::pedantic,
        clippy::nursery
    )]
    #[doc(hidden)]
    #[rustfmt::skip]
    pub mod antlrv4lexer;
    #[doc(hidden)]
    #[rustfmt::skip]
    pub mod antlrv4parser;
    #[doc(hidden)]
    #[rustfmt::skip]
    pub mod antlrv4parserlistener;
}

#[doc(hidden)]
pub use frontend::{
    Cst, CstDescendants, DiagnosticStage, FrontendError, RecoveredSource, SourceFile, SourceId,
    SourceSpan, SyntaxDiagnostic, SyntaxId, SyntaxNode, SyntaxNodeKind, SyntaxToken, parse_source,
    parse_source_recovering,
};
