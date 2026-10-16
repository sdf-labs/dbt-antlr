// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-lexer-dfa` support of the dbt emission layer (lexer scope).
//!
//! When the flag is on, the compiled lexer ATN (the Java-format word stream
//! of the runtime artifact) is deserialized with the runtime's
//! `ATNDeserializer` and handed to the runtime's static lexer DFA builder
//! (`static-dfa-build` feature). The builder drives the reference
//! `LexerATNSimulator` move/closure code exhaustively, so the tables match
//! the lazy path by construction, and the build is deterministic: one
//! grammar produces one word stream.
//!
//! The build result becomes the `StaticLexerTablesModel` of the lexer
//! output model. Modes the builder could not compile, and edges that
//! escape to the lazy simulator, become codegen warnings in the style of
//! the upstream tool's `ErrorType` 188/189.

use std::path::{Path, PathBuf};

use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::dfa::static_dfa_build::{
    DEFAULT_MAX_STATES_PER_MODE, EscapeCounts, ModeFailure, StaticLexerDfaBuild,
    StaticLexerDfaBuilder,
};

use crate::emission::model::StaticLexerTablesModel;
use crate::error::{Diagnostic, Severity};
use crate::grammar::atn::CompiledLexer;
use crate::grammar::model::{Block, ElementKind, GrammarUnit, Rule, RuleId};

/// The table payload and the warnings of one `--static-lexer-dfa` lexer build.
#[derive(Clone, Debug)]
pub(crate) struct StaticLexerBuild {
    pub(crate) model: StaticLexerTablesModel,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

/// Builds the static lexer DFA tables of one compiled lexer unit.
///
/// `grammar_path` becomes the path of the emitted diagnostics; the build
/// itself is a pure function of the compiled unit.
pub(crate) fn build_static_lexer_tables(
    compiled: &CompiledLexer,
    grammar_path: &Path,
) -> StaticLexerBuild {
    let words = &compiled.runtime_artifact.atn_words;
    let atn = ATNDeserializer::new(None).deserialize(&mut words.iter());
    let build = StaticLexerDfaBuilder::new().build(&atn);

    let unit = &compiled.semantic.unit;
    let grammar_name = unit.name.as_str();
    let mode_name = |index: usize| {
        compiled
            .semantic
            .recognizer
            .mode_names
            .get(index)
            .map_or("DEFAULT_MODE", String::as_str)
    };

    let mut diagnostics = Vec::new();
    let mut all_present = true;
    let mut total_escapes = EscapeCounts::default();
    for (index, mode) in build.modes.iter().enumerate() {
        let name = mode_name(index);
        if let Some(failure) = mode.failure {
            all_present = false;
            diagnostics.push(mode_failure_diagnostic(
                grammar_path,
                grammar_name,
                name,
                failure,
                compiled,
            ));
        }
        total_escapes.predicate += mode.escape_counts.predicate;
        total_escapes.custom_action += mode.escape_counts.custom_action;
        total_escapes.recursive_context += mode.escape_counts.recursive_context;
        total_escapes.budget += mode.escape_counts.budget;
    }
    if escape_total(total_escapes) > 0 {
        diagnostics.push(escape_summary_diagnostic(
            grammar_path,
            grammar_name,
            &build,
            compiled,
            total_escapes,
        ));
    }

    let omit_atn = all_present && escape_total(total_escapes) == 0;
    StaticLexerBuild {
        model: StaticLexerTablesModel {
            words: build.words,
            omit_atn,
        },
        diagnostics,
    }
}

fn escape_total(counts: EscapeCounts) -> usize {
    counts.predicate + counts.custom_action + counts.recursive_context + counts.budget
}

fn warning(grammar_path: &Path, code: &'static str, message: String) -> Diagnostic {
    Diagnostic::new(
        code,
        Severity::Warning,
        message,
        PathBuf::from(grammar_path),
        None,
        None,
    )
}

/// The per-mode failure warning: why the whole mode has no static table.
/// Mirrors the reason split of the upstream tool's `ErrorType` 188/189.
fn mode_failure_diagnostic(
    grammar_path: &Path,
    grammar_name: &str,
    mode_name: &str,
    failure: ModeFailure,
    compiled: &CompiledLexer,
) -> Diagnostic {
    let reason = match failure {
        ModeFailure::StartClosurePredicate => format!(
            "its start-state closure crosses a predicate{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Predicate)),
        ),
        ModeFailure::StartClosureCustomAction => format!(
            "its start-state closure runs a position-dependent action{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Action)),
        ),
        ModeFailure::StartClosureRecursiveContext => format!(
            "its start-state closure reaches a recursive rule context{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Recursive)),
        ),
        ModeFailure::StartStateBudgetExceeded => {
            return warning(
                grammar_path,
                "G4S189",
                format!(
                    "--static-lexer-dfa: static lexer DFA expansion of mode '{mode_name}' of lexer \
                     '{grammar_name}' exceeded the cap of {DEFAULT_MAX_STATES_PER_MODE} states; \
                     the mode uses the lazy simulator"
                ),
            );
        }
    };
    warning(
        grammar_path,
        "G4S188",
        format!(
            "--static-lexer-dfa: mode '{mode_name}' of lexer '{grammar_name}' is not compiled to a \
             static table: {reason}; the mode uses the lazy simulator"
        ),
    )
}

/// The per-grammar escape summary: one warning with the escape edge counts
/// by reason, with the modes and rules behind each reason where the grammar
/// model names them.
fn escape_summary_diagnostic(
    grammar_path: &Path,
    grammar_name: &str,
    build: &StaticLexerDfaBuild,
    compiled: &CompiledLexer,
    total: EscapeCounts,
) -> Diagnostic {
    let mut reasons = Vec::new();
    let mut collect = |count: usize, construct: Option<Construct>, text: &str| {
        if count == 0 {
            return;
        }
        let mut modes: Vec<&str> = Vec::new();
        for (index, mode) in build.modes.iter().enumerate() {
            let escapes = mode.escape_counts;
            let hit = match construct {
                Some(Construct::Predicate) => escapes.predicate,
                Some(Construct::Action) => escapes.custom_action,
                Some(Construct::Recursive) => escapes.recursive_context,
                None => escapes.budget,
            };
            if hit > 0 {
                modes.push(
                    compiled
                        .semantic
                        .recognizer
                        .mode_names
                        .get(index)
                        .map_or("DEFAULT_MODE", String::as_str),
                );
            }
        }
        let rules = match construct {
            Some(construct) => modes
                .iter()
                .flat_map(|mode| rules_with(compiled, mode, construct))
                .collect::<Vec<_>>(),
            None => Vec::new(),
        };
        let detail = attribution_note(&modes, &rules);
        reasons.push(format!("{count} {text}{detail}"));
    };
    collect(
        total.predicate,
        Some(Construct::Predicate),
        "cross a predicate",
    );
    collect(
        total.custom_action,
        Some(Construct::Action),
        "run a position-dependent action",
    );
    collect(
        total.recursive_context,
        Some(Construct::Recursive),
        "reach a recursive rule context",
    );
    collect(total.budget, None, "exceed the state budget");
    warning(
        grammar_path,
        "G4S188",
        format!(
            "--static-lexer-dfa: lexer '{grammar_name}' keeps {} edge(s) on the lazy simulator: {}",
            escape_total(total),
            reasons.join(", ")
        ),
    )
}

/// ` (rule 'A')` / ` (rules 'A', 'B')` for a non-empty rule list, else
/// empty.
fn rule_names_note(rules: Vec<&str>) -> String {
    if rules.is_empty() {
        return String::new();
    }
    let label = if rules.len() == 1 { "rule" } else { "rules" };
    format!(" ({label} {})", quote_join(rules.iter().copied()))
}

/// ` (mode 'DEFAULT_MODE', rule 'KWLET')`-style attribution of an escape
/// reason. Rules that the grammar model cannot name drop out.
fn attribution_note(modes: &[&str], rules: &[&str]) -> String {
    let mut parts = Vec::new();
    if !modes.is_empty() {
        let label = if modes.len() == 1 { "mode" } else { "modes" };
        parts.push(format!("{label} {}", quote_join(modes.iter().copied())));
    }
    if !rules.is_empty() {
        let label = if rules.len() == 1 { "rule" } else { "rules" };
        parts.push(format!("{label} {}", quote_join(rules.iter().copied())));
    }
    if parts.is_empty() {
        return String::new();
    }
    format!(" ({})", parts.join(", "))
}

fn quote_join(names: impl Iterator<Item = impl AsRef<str>>) -> String {
    names
        .map(|name| format!("'{}'", name.as_ref()))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The grammar constructs behind an escape reason, for rule attribution.
#[derive(Clone, Copy)]
enum Construct {
    Predicate,
    Action,
    Recursive,
}

/// The names of the rules of one lexer mode that contain the construct:
/// rules with a predicate, with an embedded action, or that can reach
/// themselves through the rule call graph.
fn rules_with<'u>(
    compiled: &'u CompiledLexer,
    mode_name: &str,
    construct: Construct,
) -> Vec<&'u str> {
    let unit = &compiled.semantic.unit;
    mode_rules(unit, mode_name)
        .into_iter()
        .filter(|rule| match construct {
            Construct::Predicate => block_has(&rule.block, &ElementDiscriminator::Predicate),
            Construct::Action => block_has(&rule.block, &ElementDiscriminator::Action),
            Construct::Recursive => is_recursive(compiled, rule.id),
        })
        .map(|rule| rule.name.as_str())
        .collect()
}

/// The rules of one lexer mode: the default mode holds the rules without a
/// `mode` member, named modes hold their `mode.rules` list.
fn mode_rules<'u>(unit: &'u GrammarUnit, mode_name: &str) -> Vec<&'u Rule> {
    if mode_name == "DEFAULT_MODE" {
        return unit
            .rules
            .iter()
            .filter(|rule| rule.mode.is_none())
            .collect();
    }
    unit.modes
        .iter()
        .find(|mode| mode.name == mode_name)
        .map(|mode| {
            mode.rules
                .iter()
                .filter_map(|id| unit.rules.iter().find(|rule| rule.id == *id))
                .collect()
        })
        .unwrap_or_default()
}

enum ElementDiscriminator {
    Predicate,
    Action,
}

fn block_has(block: &Block, kind: &ElementDiscriminator) -> bool {
    block.alternatives.iter().any(|alternative| {
        alternative
            .elements
            .iter()
            .any(|element| match (&element.kind, kind) {
                (ElementKind::Predicate { .. }, ElementDiscriminator::Predicate)
                | (ElementKind::Action { .. }, ElementDiscriminator::Action) => true,
                (ElementKind::Block(nested), _) => block_has(nested, kind),
                _ => false,
            })
    })
}

/// Whether `rule` can reach itself through the lexer rule call graph
/// (direct or indirect recursion).
fn is_recursive(compiled: &CompiledLexer, rule: RuleId) -> bool {
    let call_graph = &compiled.semantic.call_graph;
    let mut visited = std::collections::BTreeSet::new();
    let mut stack = call_graph.get(&rule).cloned().unwrap_or_default();
    while let Some(target) = stack.pop() {
        if target == rule {
            return true;
        }
        if visited.insert(target)
            && let Some(next) = call_graph.get(&target)
        {
            stack.extend(next.iter().copied());
        }
    }
    false
}
