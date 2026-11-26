// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Builds the output model of the `<parser>_ffi.rs` grammar description for
//! `dbt_antlr_runtime::ffi` bundles.

use crate::emission::lexer_factory::ANTLR_VERSION;
use crate::emission::model::{FfiFileContext, FfiRuleModel, ParserFileContext};
use crate::error::EmitError;
use crate::grammar::atn::CompiledParser;

/// The lexer that tokenizes the input of a parser: the implicit lexer of a
/// combined grammar, or the `tokenVocab` grammar of a parser grammar.
fn lexer_name(
    compiled: &CompiledParser,
    combined_lexer: Option<&str>,
) -> Result<String, EmitError> {
    if let Some(lexer) = combined_lexer {
        return Ok(lexer.to_owned());
    }
    let unit = &compiled.semantic.unit;
    unit.options
        .iter()
        .find(|option| option.name.value == "tokenVocab")
        .map(|option| option.value.value.clone())
        .ok_or_else(|| {
            EmitError::Unsupported(format!(
                "parser grammar {} has no tokenVocab option: --ffi needs its lexer",
                unit.name
            ))
        })
}

/// Builds the model of the `Ffi.rs.jinja` template.
///
/// `combined_lexer` is the name of the implicit lexer of a combined grammar.
pub(crate) fn build_ffi_file_context(
    compiled: &CompiledParser,
    parser_context: &ParserFileContext,
    combined_lexer: Option<&str>,
) -> Result<FfiFileContext, EmitError> {
    let parser = &parser_context.parser;
    let lexer_name = lexer_name(compiled, combined_lexer)?;
    let rules: Vec<FfiRuleModel> = parser
        .funcs
        .iter()
        .map(|func| FfiRuleModel {
            name: func.name.clone(),
            rust_name: func.escaped_name.clone(),
            index: func.index,
            entry: func.args.is_empty() && func.modifiers.is_empty(),
            alt_labels: func
                .alt_label_ctxs
                .iter()
                .map(|ctx| ctx.derived_from_name.clone())
                .collect(),
        })
        .collect();
    Ok(FfiFileContext {
        grammar_file_name: parser_context.file.grammar_file_name.clone(),
        antlr_version: ANTLR_VERSION.to_owned(),
        grammar_name: parser_context.file.grammar_name.clone(),
        parser_name: parser.name.clone(),
        parser_module: parser.name.to_lowercase(),
        lexer_module: lexer_name.to_lowercase(),
        lexer_name,
        rules,
    })
}
