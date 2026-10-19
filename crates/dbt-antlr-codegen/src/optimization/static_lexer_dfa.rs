// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-lexer-dfa` optimization pass (lexer scope).
//!
//! When the flag is on, the pass deserializes the compiled lexer ATN with the
//! runtime `ATNDeserializer`. Then it runs the runtime static lexer DFA
//! builder (`static-dfa-build` feature). The builder drives the reference
//! `LexerATNSimulator` move and closure code exhaustively. As a result, the
//! tables match the lazy path by construction, and the build is deterministic:
//! one grammar produces one word stream.
//!
//! The build result becomes the `StaticLexerTablesModel` of the lexer output
//! model. Modes that the builder cannot compile, and edges that escape to the
//! lazy simulator, produce compiler warnings (`G4S188` and `G4S189`).

use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::dfa::static_dfa_build::{
    DEFAULT_MAX_STATES_PER_MODE, EscapeReason, EscapedEdge, ModeFailure, StaticLexerDfaBuilder,
};
use dbt_antlr_runtime::int_stream::EOF;

use crate::emission::model::StaticLexerTablesModel;
use crate::grammar::atn::{CompiledLexer, FinalizedTransitionKind};
use crate::grammar::diagnostic::Diagnostic;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::{
    Block, BuildTransitionId, ElementId, ElementKind, GrammarUnit, ModelNodeId, Rule, RuleId,
};
use crate::grammar::provenance::{Origin, ProvenanceIndex};
use crate::grammar::transform::SafetyClass;

pub(crate) struct StaticLexerDfa;

impl StaticLexerDfa {
    pub(crate) const NAME: &'static str = "static-lexer-dfa";
    pub(crate) const SAFETY: SafetyClass = SafetyClass::TreeAndApiPreserving;
}

/// The table payload and the warnings of one `--static-lexer-dfa` lexer build.
#[derive(Clone, Debug)]
pub(crate) struct StaticLexerBuild {
    pub(crate) model: StaticLexerTablesModel,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

/// Builds the static lexer DFA tables of one compiled lexer unit.
pub(crate) fn build_static_lexer_tables(compiled: &CompiledLexer) -> StaticLexerBuild {
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
    let mut has_escapes = false;
    for (index, mode) in build.modes.iter().enumerate() {
        let name = mode_name(index);
        if let Some(failure) = mode.failure {
            all_present = false;
            diagnostics.push(mode_failure_diagnostic(
                grammar_name,
                name,
                failure,
                compiled,
            ));
        }
        if !mode.escaped_edges.is_empty() {
            has_escapes = true;
            for edge in &mode.escaped_edges {
                diagnostics.push(escaped_edge_diagnostic(grammar_name, name, edge, compiled));
            }
        }
    }

    let omit_atn = all_present && !has_escapes;
    StaticLexerBuild {
        model: StaticLexerTablesModel {
            words: build.words,
            omit_atn,
        },
        diagnostics,
    }
}

/// Per-mode failure warning when a whole mode has no static table.
fn mode_failure_diagnostic(
    grammar_name: &str,
    mode_name: &str,
    failure: ModeFailure,
    compiled: &CompiledLexer,
) -> Diagnostic {
    let unit = &compiled.semantic.unit;
    let span = mode_span(unit, mode_name);
    let detail = edge_attribution_note(grammar_name, mode_name, None);

    let reason = match failure {
        ModeFailure::StartClosurePredicate => format!(
            "start-state closure crosses a predicate{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Predicate)),
        ),
        ModeFailure::StartClosureCustomAction => format!(
            "start-state closure runs a position-dependent action{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Action)),
        ),
        ModeFailure::StartClosureRecursiveContext => format!(
            "start-state closure reaches a recursive rule context{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Recursive)),
        ),
        ModeFailure::StartStateBudgetExceeded => {
            return Diagnostic::warning(
                "G4S189",
                span,
                format!(
                    "{detail}: static DFA expansion exceeded the cap of {DEFAULT_MAX_STATES_PER_MODE} states; \
                     the whole mode is deferred to runtime expansion"
                ),
            );
        }
    };
    Diagnostic::warning(
        "G4S188",
        span,
        format!(
            "{detail}: static DFA expansion failed: {reason}; the whole mode is deferred to runtime expansion"
        ),
    )
}

/// Per-edge warning for one escaped DFA edge, with a source span in the `.g4` file.
fn escaped_edge_diagnostic(
    grammar_name: &str,
    mode_name: &str,
    edge: &EscapedEdge,
    compiled: &CompiledLexer,
) -> Diagnostic {
    let (span, rule_name, reason) = resolve_escape_site(compiled, edge.reason);
    let detail = edge_attribution_note(grammar_name, mode_name, rule_name);
    let symbol = format_edge_label(edge.lo, edge.hi);
    Diagnostic::warning(
        "G4S188",
        span,
        format!(
            "{detail}: static-lexer-dfa failed to expand edge on {symbol} (state {}) \
             due to: {reason}",
            edge.state,
        ),
    )
}

fn resolve_escape_site(
    compiled: &CompiledLexer,
    reason: EscapeReason,
) -> (SourceSpan, Option<&str>, &'static str) {
    let unit = &compiled.semantic.unit;
    match reason {
        EscapeReason::Predicate {
            rule_index,
            pred_index,
        } => {
            let binding = compiled
                .semantic
                .bindings
                .predicates
                .values()
                .find(|binding| binding.index == pred_index);
            let rule = binding
                .and_then(|binding| rule_by_id(unit, binding.rule))
                .or_else(|| unit.rules.get(rule_index));
            let span = binding
                .and_then(|binding| element_span(compiled, binding.rule, binding.element))
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "cross a predicate",
            )
        }
        EscapeReason::CustomAction {
            rule_index,
            action_index,
        } => {
            let binding = compiled
                .semantic
                .bindings
                .actions
                .values()
                .find(|binding| binding.index == action_index);
            let rule = binding
                .and_then(|binding| rule_by_id(unit, binding.rule))
                .or_else(|| unit.rules.get(rule_index));
            let span = binding
                .and_then(|binding| element_span(compiled, binding.rule, binding.element))
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "run a position-dependent action",
            )
        }
        EscapeReason::RecursiveContext {
            rule_index,
            state_number,
            return_state,
        } => {
            let rule_transition = compiled.graph.transitions.iter().find(|transition| {
                matches!(
                    transition.kind,
                    FinalizedTransitionKind::Rule { follow, .. } if follow == return_state
                )
            });
            let rule = rule_transition
                .and_then(|transition| compiled.graph.states.get(transition.source))
                .and_then(|state| state.rule)
                .and_then(|id| rule_by_id(unit, id))
                .or_else(|| {
                    compiled
                        .graph
                        .states
                        .get(return_state)
                        .and_then(|state| state.rule)
                        .and_then(|id| rule_by_id(unit, id))
                })
                .or_else(|| unit.rules.get(rule_index));
            let span = rule_transition
                .and_then(|transition| transition_span(compiled, transition.original))
                .or_else(|| state_span(compiled, return_state))
                .or_else(|| state_span(compiled, state_number))
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "reach a recursive rule context",
            )
        }
        EscapeReason::Budget {
            rule_index,
            state_number,
        } => {
            let rule = compiled
                .graph
                .states
                .get(state_number)
                .and_then(|state| state.rule)
                .and_then(|id| rule_by_id(unit, id))
                .or_else(|| unit.rules.get(rule_index));
            let span = state_span(compiled, state_number)
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "exceed the state budget",
            )
        }
    }
}

fn mode_span(unit: &GrammarUnit, mode_name: &str) -> SourceSpan {
    if mode_name != "DEFAULT_MODE"
        && let Some(mode) = unit.modes.iter().find(|mode| mode.name == mode_name)
    {
        return mode.span.clone();
    }
    unit.span.clone()
}

fn rule_by_id(unit: &GrammarUnit, id: RuleId) -> Option<&Rule> {
    unit.rules.iter().find(|rule| rule.id == id)
}

fn element_span(
    compiled: &CompiledLexer,
    rule_id: RuleId,
    element_id: ElementId,
) -> Option<SourceSpan> {
    if let Some(rule) = rule_by_id(&compiled.semantic.unit, rule_id)
        && let Some(span) = find_element_span(&rule.block, element_id)
    {
        return Some(span);
    }
    origins_span(
        &compiled.provenance,
        compiled
            .provenance
            .origins(ModelNodeId::Element(element_id)),
    )
}

fn find_element_span(block: &Block, target: ElementId) -> Option<SourceSpan> {
    for alternative in &block.alternatives {
        for element in &alternative.elements {
            if element.id == target {
                return Some(element.span.clone());
            }
            if let ElementKind::Block(nested) = &element.kind
                && let Some(span) = find_element_span(nested, target)
            {
                return Some(span);
            }
        }
    }
    None
}

fn state_span(compiled: &CompiledLexer, state_number: usize) -> Option<SourceSpan> {
    let state = compiled.graph.states.get(state_number)?;
    origins_span(
        &compiled.provenance,
        compiled.provenance.state_origins(state.original),
    )
}

fn transition_span(compiled: &CompiledLexer, transition: BuildTransitionId) -> Option<SourceSpan> {
    origins_span(
        &compiled.provenance,
        compiled.provenance.transition_origins(transition),
    )
}

fn origins_span(provenance: &ProvenanceIndex, origins: &[Origin]) -> Option<SourceSpan> {
    for origin in origins {
        match origin {
            Origin::Authored { span, .. } => return Some(span.clone()),
            Origin::Synthetic { owner, .. } => {
                if let Some(span) = origins_span(provenance, provenance.origins(*owner)) {
                    return Some(span);
                }
            }
            _ => {}
        }
    }
    None
}

fn format_edge_label(lo: i32, hi: i32) -> String {
    if lo == hi {
        format_symbol(lo)
    } else {
        format!("{}..={}", format_symbol(lo), format_symbol(hi))
    }
}

fn format_symbol(symbol: i32) -> String {
    if symbol == EOF {
        return "EOF".to_owned();
    }
    match symbol {
        0x09 => "'\\t'".to_owned(),
        0x0A => "'\\n'".to_owned(),
        0x0D => "'\\r'".to_owned(),
        0x20..=0x7E => format!("{:?}", char::from_u32(symbol as u32).unwrap()),
        _ => format!("U+{symbol:04X}"),
    }
}

fn edge_attribution_note(grammar_name: &str, mode_name: &str, rule_name: Option<&str>) -> String {
    match rule_name {
        Some(rule) => format!(" ({grammar_name}, mode '{mode_name}', rule '{rule}')"),
        None => format!(" ({grammar_name}, mode '{mode_name}')"),
    }
}

fn rule_names_note(rules: Vec<&str>) -> String {
    if rules.is_empty() {
        return String::new();
    }
    let label = if rules.len() == 1 { "rule" } else { "rules" };
    format!(" ({label} {})", quote_join(rules.iter().copied()))
}

fn quote_join(names: impl Iterator<Item = impl AsRef<str>>) -> String {
    names
        .map(|name| format!("'{}'", name.as_ref()))
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Clone, Copy)]
enum Construct {
    Predicate,
    Action,
    Recursive,
}

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
