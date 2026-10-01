// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
pub(crate) mod action;
pub(crate) mod atn;
mod char_support;
pub(crate) mod compiler;
pub(crate) mod diagnostic;
mod escape_sequence;
pub(crate) mod integration;
mod left_recursion;
pub(crate) mod loader;
pub(crate) mod model;
mod mutual_recursion;
pub(crate) mod provenance;
pub(crate) mod rule_reachability;
mod semantics;
pub(crate) mod source;
mod syntax;
pub(crate) mod transform;
mod unicode;
mod unicode_escape;
pub(crate) mod validation;

pub(crate) mod frontend {
    pub(crate) use dbt_antlr_g4_parser::{
        Cst, FrontendError, SourceFile, SourceId, SourceSpan, SyntaxId, SyntaxNode, SyntaxNodeKind,
        SyntaxToken, parse_source, parse_source_recovering,
    };
}

pub(crate) mod generated {
    pub(crate) use dbt_antlr_g4_parser::generated::antlrv4parser;
}
