// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Builds the dbt lexer output model from a compiled lexer grammar unit.
//!
//! This is a port of the Java tool's lexer-path model construction:
//! `OutputModelController.buildLexerOutputModel`,
//! `OutputModelController.buildLexerRuleActions`, and the `Lexer`,
//! `LexerFile`, and `Recognizer` model constructors, built on the vendored
//! `grammar` analysis layer instead of the Java `Grammar` object.

use crate::dbt::model::{
    ActionChunkModel, ChunkContextModel, IndexedActionModel, LexerFileContext, LexerModel,
    NamedActionsModel, OutputFileModel, RuleActionFunctionModel, SerializedAtnModel,
    TokenConstantModel,
};
use crate::grammar::atn::CompiledLexer;
use crate::grammar::model::{Block, ElementKind, GrammarUnit, RecognizerModel, Rule};
use std::fmt::Write as _;

/// Version string the generated `fileHeader` comment reports; matches the
/// `Tool.VERSION` of the dbt ANTLR fork that produced the golden files.
pub(crate) const ANTLR_VERSION: &str = "4.13.2";

/// Reserved words of `RustTarget`; `escapeIfNeeded` appends `_`.
const RUST_RESERVED_WORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "Self", "self", "static", "struct", "super", "trait", "true", "type", "union",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

/// `Target.escapeIfNeeded` for the Rust target.
pub(crate) fn escape_if_needed(identifier: &str) -> String {
    if RUST_RESERVED_WORDS.contains(&identifier) {
        format!("{identifier}_")
    } else {
        identifier.to_owned()
    }
}

/// The `Recognizer.tokens` map of a recognizer: token constants with a
/// positive token type, in declaration order.
pub(crate) fn token_constants(recognizer: &RecognizerModel) -> Vec<TokenConstantModel> {
    recognizer
        .vocabulary
        .name_order
        .iter()
        .filter_map(|name| {
            let number = recognizer.vocabulary.by_name.get(name).copied()?;
            (number > 0).then(|| TokenConstantModel {
                key: escape_if_needed(name),
                value: number,
            })
        })
        .collect()
}

/// `Utils.capitalize`: uppercases the first character.
pub(crate) fn capitalize(text: &str) -> String {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}

/// `Target.getRuleFunctionContextStructName`: `CsvFileContext`.
pub(crate) fn rule_context_struct_name(rule_name: &str) -> String {
    format!("{}Context", capitalize(rule_name))
}

/// Builds the full `LexerFile` render context for one compiled lexer unit.
///
/// `grammar_file_name` is the name that lands in the `fileHeader` comment.
pub(crate) fn build_lexer_file_context(
    compiled: &CompiledLexer,
    grammar_file_name: &str,
) -> LexerFileContext {
    let unit = &compiled.semantic.unit;
    let recognizer = &compiled.semantic.recognizer;

    let tokens = token_constants(recognizer);

    let literal_names = translate_literal_names(&recognizer.literal_names);
    let symbolic_names = translate_symbolic_names(&recognizer.symbolic_names);

    let mut action_funcs = Vec::new();
    let mut sempred_funcs = Vec::new();
    for rule in &unit.rules {
        collect_rule_functions(recognizer, rule, &mut action_funcs, &mut sempred_funcs);
    }

    let lexer = LexerModel {
        name: unit.name.clone(),
        grammar_name: unit.name.clone(),
        grammar_file_name: format!("{}.g4", unit.name),
        tokens,
        rule_names: recognizer.rule_names.clone(),
        literal_names,
        symbolic_names,
        channel_names: unit
            .channels
            .iter()
            .map(|channel| channel.name.value.clone())
            .collect(),
        modes: recognizer.mode_names.clone(),
        action_funcs,
        sempred_funcs,
        atn: SerializedAtnModel {
            serialized: compiled.runtime_artifact.atn_words.clone(),
        },
    };

    LexerFileContext {
        lexer_file: OutputFileModel {
            grammar_file_name: grammar_file_name.to_owned(),
            antlr_version: ANTLR_VERSION.to_owned(),
        },
        lexer,
        named_actions: collect_named_actions(unit, "lexer"),
    }
}

/// Collects the embedded actions and sempreds of one lexer rule, mirroring
/// `OutputModelController.buildLexerRuleActions`.
fn collect_rule_functions(
    recognizer: &RecognizerModel,
    rule: &Rule,
    action_funcs: &mut Vec<RuleActionFunctionModel>,
    sempred_funcs: &mut Vec<RuleActionFunctionModel>,
) {
    let mut actions = Vec::new();
    let mut predicates = Vec::new();
    collect_block_actions(&rule.block, &mut actions, &mut predicates);
    if actions.is_empty() && predicates.is_empty() {
        return;
    }
    let rule_index = recognizer.rule_numbers[&rule.id];
    let make_function = |entries: Vec<(usize, String)>| RuleActionFunctionModel {
        name: rule.name.clone(),
        escaped_name: escape_if_needed(&rule.name),
        ctx_type: "LexerContext".to_owned(),
        rule_index,
        is_lexer: true,
        actions: entries
            .into_iter()
            .map(|(index, text)| IndexedActionModel {
                index,
                // Lexer action and sempred bodies are verbatim text.
                chunks: vec![ActionChunkModel::ActionText {
                    ctx: ChunkContextModel::new("LexerContext"),
                    text,
                }],
            })
            .collect(),
    };
    if !actions.is_empty() {
        let entries = actions
            .into_iter()
            .map(|(id, body)| (recognizer.action_numbers[&id], body))
            .collect();
        action_funcs.push(make_function(entries));
    }
    if !predicates.is_empty() {
        let entries = predicates
            .into_iter()
            .map(|(id, body)| (recognizer.predicate_numbers[&id], body))
            .collect();
        sempred_funcs.push(make_function(entries));
    }
}

/// Walks a rule body in source order and collects embedded actions and
/// sempredicates, the way the Java tool's `Rule.actions` list is populated.
fn collect_block_actions(
    block: &Block,
    actions: &mut Vec<(crate::grammar::model::ActionId, String)>,
    predicates: &mut Vec<(crate::grammar::model::PredicateId, String)>,
) {
    for alternative in &block.alternatives {
        for element in &alternative.elements {
            match &element.kind {
                ElementKind::Action { id, body } => actions.push((*id, body.clone())),
                ElementKind::Predicate { id, body, .. } => predicates.push((*id, body.clone())),
                ElementKind::Block(nested) => collect_block_actions(nested, actions, predicates),
                _ => {}
            }
        }
    }
}

/// Collects the grammar's named actions for the template's
/// `namedActions.*` lookups. `scope` is `lexer` or `parser`; unscoped
/// actions apply to both, like the Java tool's `Grammar.namedActions`.
pub(crate) fn collect_named_actions(unit: &GrammarUnit, scope: &str) -> NamedActionsModel {
    let mut model = NamedActionsModel::default();
    for action in &unit.actions {
        if action.scope.as_deref().is_some_and(|named| named != scope) {
            continue;
        }
        let slot = match action.name.as_str() {
            "header" => &mut model.header,
            "init" => &mut model.init,
            "fields" => &mut model.fields,
            "members" => &mut model.members,
            "definitions" => &mut model.definitions,
            "extend" => &mut model.extend,
            _ => continue,
        };
        *slot = Some(action.body.clone());
    }
    model
}

/// `Recognizer.translateTokenStringsToTarget` for literal names: ANTLR
/// literals become `"'<escaped>'"`; trailing `None` entries are trimmed.
pub(crate) fn translate_literal_names(literal_names: &[Option<String>]) -> Vec<Option<String>> {
    let translated = literal_names
        .iter()
        .map(|name| {
            name.as_ref().map(|name| {
                if name.starts_with('\'') {
                    format!("\"'{}'\"", target_string_literal_from_antlr_literal(name))
                } else {
                    target_string_literal_from_string(name)
                }
            })
        })
        .collect::<Vec<_>>();
    trim_trailing_nones(translated)
}

/// `Recognizer.translateTokenStringsToTarget` for symbolic names.
pub(crate) fn translate_symbolic_names(symbolic_names: &[Option<String>]) -> Vec<Option<String>> {
    let translated = symbolic_names
        .iter()
        .map(|name| {
            name.as_ref()
                .map(|name| target_string_literal_from_string(name))
        })
        .collect::<Vec<_>>();
    trim_trailing_nones(translated)
}

fn trim_trailing_nones(mut names: Vec<Option<String>>) -> Vec<Option<String>> {
    while names.last().is_some_and(Option::is_none) {
        names.pop();
    }
    names
}

/// `Target.getTargetStringLiteralFromANTLRStringLiteral` with
/// `addQuotes=false, escapeSpecial=true`: returns the escaped content of an
/// ANTLR string literal without surrounding quotes.
fn target_string_literal_from_antlr_literal(literal: &str) -> String {
    let mut out = String::new();
    let chars = literal.chars().collect::<Vec<_>>();
    let mut index = 1; // skip opening quote
    while index < chars.len().saturating_sub(1) {
        let c = chars[index];
        if c == '\\' {
            index += 1;
            let Some(&escaped) = chars.get(index) else {
                break;
            };
            match escaped {
                'n' | 'r' | 't' | 'b' | 'f' | '\\' => {
                    if escaped != '\\' {
                        out.push('\\');
                    }
                    out.push('\\');
                    out.push(escaped);
                }
                'u' => {
                    let (code_point, next) = parse_unicode_escape(&chars, index);
                    index = next;
                    append_unicode_escaped_code_point(&mut out, code_point, true);
                    index -= 1; // adjust for the increment below
                }
                _ => {
                    if should_use_unicode_escape(u32::from(escaped)) {
                        append_unicode_escaped_code_point(&mut out, u32::from(escaped), true);
                    } else {
                        out.push(escaped);
                    }
                }
            }
        } else if c == '"' {
            out.push_str("\\\"");
        } else if should_use_unicode_escape(u32::from(c)) {
            append_unicode_escaped_code_point(&mut out, u32::from(c), true);
        } else {
            out.push(c);
        }
        index += 1;
    }
    out
}

/// Parses `\uXXXX` or `\u{XXXXXX}` starting at the `u`; returns the code
/// point and the index just past the escape.
fn parse_unicode_escape(chars: &[char], u_index: usize) -> (u32, usize) {
    let mut index = u_index + 1;
    let mut digits = String::new();
    if chars.get(index) == Some(&'{') {
        index += 1;
        while chars.get(index).is_some_and(|&c| c != '}') {
            digits.push(chars[index]);
            index += 1;
        }
        index += 1; // past '}'
    } else {
        for _ in 0..4 {
            if let Some(&c) = chars.get(index) {
                digits.push(c);
                index += 1;
            }
        }
    }
    (u32::from_str_radix(&digits, 16).unwrap_or(0xFFFD), index)
}

/// `Target.getTargetStringLiteralFromString` with `quoted=true`.
pub(crate) fn target_string_literal_from_string(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        let escaped = match c {
            '\t' => Some("\\t"),
            '\n' => Some("\\n"),
            '\r' => Some("\\r"),
            '"' => Some("\\\""),
            '\'' => Some("\\'"),
            '\\' => Some("\\\\"),
            _ => None,
        };
        if c != '\''
            && let Some(escaped) = escaped
        {
            out.push_str(escaped);
        } else if should_use_unicode_escape(u32::from(c)) {
            append_unicode_escaped_code_point(&mut out, u32::from(c), false);
        } else {
            out.push(c);
        }
    }
    out.push('"');
    out
}

/// `Target.shouldUseUnicodeEscapeForCodePointInDoubleQuotedString`.
const fn should_use_unicode_escape(code_point: u32) -> bool {
    code_point < 0x20 || code_point == 0x5C || code_point >= 0x7F
}

/// `Target.appendUnicodeEscapedCodePoint` for the Rust target, which does not
/// override the default `\uXXXX` formatting.
fn append_unicode_escaped_code_point(out: &mut String, code_point: u32, escape: bool) {
    if escape {
        out.push('\\');
    }
    if let Some(c) = char::from_u32(code_point) {
        let mut buffer = [0_u16; 2];
        for unit in c.encode_utf16(&mut buffer).iter() {
            let _ = write!(out, "\\u{unit:04X}");
        }
    } else {
        let _ = write!(out, "\\u{code_point:04X}");
    }
}
