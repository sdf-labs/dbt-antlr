// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Builds the output model of the foreign-language bindings of a parser: the
//! `<parser>_ffi.rs` grammar description for `dbt_antlr_runtime::ffi` bundles and the
//! `<name>_parser.py` module for the `dbt_antlr` Python package.

use crate::emission::lexer_factory::ANTLR_VERSION;
use crate::emission::model::{
    BindingRuleModel, BindingTokenModel, BindingsFileContext, ParserFileContext,
};
use crate::error::EmitError;
use crate::grammar::atn::CompiledParser;

/// The keywords of Python 3, which cannot be identifiers.
const PYTHON_KEYWORDS: &[&str] = &[
    "False", "None", "True", "and", "as", "assert", "async", "await", "break", "class", "continue",
    "def", "del", "elif", "else", "except", "finally", "for", "from", "global", "if", "import",
    "in", "is", "lambda", "nonlocal", "not", "or", "pass", "raise", "return", "try", "while",
    "with", "yield",
];

/// Appends `_` to Python keywords, so that `from` becomes `from_`.
fn python_identifier(name: &str) -> String {
    if PYTHON_KEYWORDS.contains(&name) {
        format!("{name}_")
    } else {
        name.to_owned()
    }
}

/// The name that the Python module of a parser looks its grammar up by:
/// the lowercase recognizer name without the `Parser` suffix (`CSVParser`
/// becomes `csv`). The Python module is `<name>_parser.py`.
pub(crate) fn bundle_name(parser_name: &str) -> String {
    let stem = parser_name.strip_suffix("Parser").unwrap_or(parser_name);
    if stem.is_empty() {
        parser_name.to_lowercase()
    } else {
        stem.to_lowercase()
    }
}

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
                "parser grammar {} has no tokenVocab option: the Python bindings need its lexer",
                unit.name
            ))
        })
}

/// Makes sure that the module-level names of the Python module are unique:
/// the `EOF` and token constants, the `RULE_<name>` constants, and the entry
/// functions.
fn check_python_names(
    parser_context: &ParserFileContext,
    rules: &[BindingRuleModel],
    tokens: &[BindingTokenModel],
) -> Result<(), EmitError> {
    let names = std::iter::once("EOF".to_owned())
        .chain(tokens.iter().map(|token| token.python_name.clone()))
        .chain(rules.iter().map(|rule| format!("RULE_{}", rule.name)))
        .chain(
            rules
                .iter()
                .filter(|rule| rule.entry)
                .map(|rule| rule.python_name.clone()),
        );
    let mut seen = std::collections::BTreeSet::new();
    for name in names {
        if !seen.insert(name.clone()) {
            return Err(EmitError::Unsupported(format!(
                "the Python module of {} defines {name} twice",
                parser_context.parser.name
            )));
        }
    }
    Ok(())
}

/// Builds the models of the `FfiFile` and `PythonFile` templates.
///
/// `combined_lexer` is the name of the implicit lexer of a combined grammar.
pub(crate) fn build_bindings_file_contexts(
    compiled: &CompiledParser,
    parser_context: &ParserFileContext,
    combined_lexer: Option<&str>,
) -> Result<[BindingsFileContext; 2], EmitError> {
    let recognizer = &compiled.semantic.recognizer;
    let parser = &parser_context.parser;
    let lexer_name = lexer_name(compiled, combined_lexer)?;
    let rules: Vec<BindingRuleModel> = parser
        .funcs
        .iter()
        .map(|func| BindingRuleModel {
            name: func.name.clone(),
            rust_name: func.escaped_name.clone(),
            python_name: python_identifier(&func.name),
            index: func.index,
            entry: func.args.is_empty() && func.modifiers.is_empty(),
            alt_labels: func
                .alt_label_ctxs
                .iter()
                .map(|ctx| ctx.derived_from_name.clone())
                .collect(),
        })
        .collect();
    let tokens: Vec<BindingTokenModel> = recognizer
        .vocabulary
        .name_order
        .iter()
        .filter_map(|name| {
            let value = recognizer.vocabulary.by_name.get(name).copied()?;
            (value > 0).then(|| BindingTokenModel {
                python_name: python_identifier(name),
                value,
            })
        })
        .collect();
    check_python_names(parser_context, &rules, &tokens)?;
    let context = BindingsFileContext {
        kind: "FfiFile",
        grammar_file_name: parser_context.file.grammar_file_name.clone(),
        antlr_version: ANTLR_VERSION.to_owned(),
        grammar_name: parser_context.file.grammar_name.clone(),
        parser_name: parser.name.clone(),
        parser_module: parser.name.to_lowercase(),
        lexer_module: lexer_name.to_lowercase(),
        lexer_name,
        bundle_name: bundle_name(&parser.name),
        rules,
        tokens,
    };
    let python = BindingsFileContext {
        kind: "PythonFile",
        ..context.clone()
    };
    Ok([context, python])
}

#[cfg(test)]
mod tests {
    use super::{bundle_name, python_identifier};

    #[test]
    fn bundle_names_drop_the_parser_suffix() {
        assert_eq!(bundle_name("CSVParser"), "csv");
        assert_eq!(bundle_name("ClickHouseParser"), "clickhouse");
        assert_eq!(bundle_name("Expr"), "expr");
        assert_eq!(bundle_name("Parser"), "parser");
    }

    #[test]
    fn python_keywords_get_a_trailing_underscore() {
        assert_eq!(python_identifier("from"), "from_");
        assert_eq!(python_identifier("None"), "None_");
        assert_eq!(python_identifier("match"), "match");
        assert_eq!(python_identifier("csvFile"), "csvFile");
    }
}
