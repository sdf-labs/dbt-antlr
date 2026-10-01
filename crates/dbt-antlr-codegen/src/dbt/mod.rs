// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Emission layer generating Rust recognizers for the `dbt-antlr-runtime` runtime.
//!
//! Generated text comes from the minijinja template
//! `templates/dbt/Rust.stg.jinja`, which mirrors the Java tool's `Rust.stg`.
//! The output model ([`model`]) mirrors the Java `codegen.model` classes; the
//! factories ([`lexer_factory`], [`parser_factory`]) mirror the Java
//! `OutputModelController` / `LexerFactory` / `ParserFactory` walks on top of
//! the vendored `grammar` analysis layer.
//!
//! See the "Emission layer design" section of `MEMO.md` for the architecture.

mod action_translator;
pub mod cli;
pub mod emit;
mod lexer_factory;
mod listener_factory;
pub mod model;
mod parser_factory;
pub mod template;

pub use emit::{
    EmitError, EmittedFile, emit_files, emit_files_with_flags, emit_lexer_files,
    emit_listener_visitor_files,
};
