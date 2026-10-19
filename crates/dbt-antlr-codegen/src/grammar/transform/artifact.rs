// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::{BTreeMap, BTreeSet};

use minijinja::Value;

use super::{
    SafetyClass, StructuralMetrics, TransformAlternativeMapping, TransformCallSite,
    TransformCandidateReport, TransformCandidateStatus, TransformLabelMapping, TransformProjection,
    TransformReport, TransformReportEntry,
};
use crate::grammar::frontend::{SourceId, SourceSpan};
use crate::grammar::source::SourceSet;
use crate::optimization::static_lexer_dfa::{
    StaticLexerAccelKindReport, StaticLexerAcceleratedStateReport, StaticLexerContinuationReport,
    StaticLexerDfa, StaticLexerDfaReport, StaticLexerEscapedEdgeReport,
    StaticLexerModeFailureReport, StaticLexerModeReport, StaticLexerModeStatus,
    StaticLexerRuleFootprintReport, StaticLexerSimpleRecursiveRuleReport,
};

/// Renders the optimization manifest with minijinja's built-in `Value`
/// formatting.
pub(crate) fn render_optimization_manifest(
    report: &TransformReport,
    manifest_passes: &BTreeSet<&'static str>,
    static_lexers: Option<&[&StaticLexerDfaReport]>,
    sources: &SourceSet,
    report_only: bool,
) -> String {
    let mut passes = report
        .entries
        .iter()
        .filter(|entry| manifest_passes.contains(entry.name))
        .map(|entry| optimization_pass_value(entry, report, sources))
        .collect::<Vec<_>>();
    if let Some(lexers) = static_lexers {
        passes.push(static_lexer_dfa_pass_value(passes.len(), lexers, sources));
    }
    let mut manifest = format!(
        "{:#?}",
        object(vec![
            ("version", Value::from(1u64)),
            ("reportOnly", Value::from(report_only)),
            ("passes", Value::from_iter(passes)),
        ])
    );
    manifest.push('\n');
    manifest
}

fn optimization_pass_value(
    entry: &TransformReportEntry,
    report: &TransformReport,
    sources: &SourceSet,
) -> Value {
    let candidates = report
        .candidates
        .iter()
        .filter(|candidate| candidate.pass == entry.id)
        .map(|candidate| transform_candidate_value(candidate, sources))
        .collect::<Vec<_>>();
    object(vec![
        ("id", Value::from(entry.id.index())),
        ("name", Value::from(entry.name)),
        ("safetyClass", Value::from(safety_name(entry.safety))),
        ("changed", Value::from(entry.changed)),
        (
            "metrics",
            object(vec![
                ("before", structural_metrics_value(&entry.before)),
                ("after", structural_metrics_value(&entry.after)),
            ]),
        ),
        ("candidates", Value::from_iter(candidates)),
    ])
}

fn structural_metrics_value(metrics: &StructuralMetrics) -> Value {
    object(vec![
        ("rules", Value::from(metrics.rules)),
        ("alternatives", Value::from(metrics.alternatives)),
        ("elements", Value::from(metrics.elements)),
    ])
}

fn transform_candidate_value(candidate: &TransformCandidateReport, sources: &SourceSet) -> Value {
    object(vec![
        ("grammar", Value::from(candidate.grammar.as_str())),
        ("entryRule", Value::from(candidate.entry_rule.as_str())),
        ("source", source_value(&candidate.source_span, sources)),
        (
            "status",
            Value::from(candidate_status_name(candidate.status)),
        ),
        ("reason", Value::from(candidate.reason.as_str())),
        (
            "rungs",
            Value::from_iter(
                candidate
                    .rungs
                    .iter()
                    .map(|rung| Value::from(rung.as_str())),
            ),
        ),
        (
            "boundaryRule",
            option(candidate.boundary_rule.as_deref(), Value::from),
        ),
        (
            "projected",
            option(candidate.projection.as_ref(), transform_projection_value),
        ),
        (
            "removedRules",
            Value::from_iter(candidate.removed_rules.iter().map(|removed| {
                object(vec![
                    ("rule", Value::from(removed.rule.as_str())),
                    ("targetRule", option(removed.target.as_deref(), Value::from)),
                ])
            })),
        ),
        (
            "alternatives",
            Value::from_iter(
                candidate
                    .alternatives
                    .iter()
                    .map(|mapping| alternative_mapping_value(mapping, sources)),
            ),
        ),
        (
            "labelRenames",
            Value::from_iter(
                candidate
                    .labels
                    .iter()
                    .map(|mapping| label_mapping_value(mapping, sources)),
            ),
        ),
        (
            "groupingChanges",
            Value::from_iter(candidate.grouping_changes.iter().map(|rule| {
                object(vec![
                    ("rule", Value::from(rule.as_str())),
                    ("from", Value::from("flat-loop")),
                    ("to", Value::from("left-recursive-nesting")),
                ])
            })),
        ),
        (
            "inlinedCallSites",
            Value::from_iter(
                candidate
                    .call_sites
                    .iter()
                    .map(|site| call_site_value(site, sources)),
            ),
        ),
    ])
}

fn call_site_value(site: &TransformCallSite, sources: &SourceSet) -> Value {
    object(vec![
        ("caller", Value::from(site.caller.as_str())),
        ("alternative", Value::from(site.alternative)),
        ("source", source_value(&site.source_span, sources)),
    ])
}

fn transform_projection_value(projection: &TransformProjection) -> Value {
    object(vec![
        (
            "contextsPerOperand",
            reduction_value(
                projection.contexts_per_operand_before,
                projection.contexts_per_operand_after,
            ),
        ),
        (
            "precedenceDecisions",
            reduction_value(
                projection.precedence_decisions_before,
                projection.precedence_decisions_after,
            ),
        ),
    ])
}

fn reduction_value(before: usize, after: usize) -> Value {
    object(vec![
        ("before", Value::from(before)),
        ("after", Value::from(after)),
        ("reduction", Value::from(before.saturating_sub(after))),
    ])
}

fn alternative_mapping_value(mapping: &TransformAlternativeMapping, sources: &SourceSet) -> Value {
    object(vec![
        ("sourceRule", Value::from(mapping.source_rule.as_str())),
        ("sourceAlternative", Value::from(mapping.source_alternative)),
        ("source", source_value(&mapping.source_span, sources)),
        ("targetRule", Value::from(mapping.target_rule.as_str())),
        (
            "targetAltLabels",
            Value::from_iter(
                mapping
                    .target_alternatives
                    .iter()
                    .map(|label| Value::from(label.as_str())),
            ),
        ),
    ])
}

fn label_mapping_value(mapping: &TransformLabelMapping, sources: &SourceSet) -> Value {
    object(vec![
        ("sourceRule", Value::from(mapping.source_rule.as_str())),
        ("sourceLabel", Value::from(mapping.source_label.as_str())),
        ("source", source_value(&mapping.source_span, sources)),
        ("targetLabel", Value::from(mapping.target_label.as_str())),
    ])
}

fn source_value(span: &SourceSpan, sources: &SourceSet) -> Value {
    object(vec![
        (
            "path",
            option(sources.logical_path(span.source), |path| {
                Value::from(path.to_string_lossy().into_owned())
            }),
        ),
        ("byteStart", Value::from(span.bytes.start)),
        ("byteEnd", Value::from(span.bytes.end)),
        (
            "start",
            option(
                sources.line_column(span.source, span.bytes.start),
                line_column_value,
            ),
        ),
        (
            "end",
            option(
                sources.line_column(span.source, span.bytes.end),
                line_column_value,
            ),
        ),
    ])
}

fn line_column_value((line, column): (usize, usize)) -> Value {
    object(vec![
        ("line", Value::from(line)),
        ("column", Value::from(column)),
    ])
}

fn object(entries: Vec<(&'static str, Value)>) -> Value {
    Value::from_iter(
        entries
            .into_iter()
            .map(|(key, value)| (Value::from(key), value)),
    )
}

fn option<T>(value: Option<T>, render: impl FnOnce(T) -> Value) -> Value {
    value.map_or_else(|| Value::from(()), render)
}

const fn safety_name(safety: SafetyClass) -> &'static str {
    match safety {
        SafetyClass::TreeAndApiPreserving => "tree-and-api-preserving",
        SafetyClass::RecognitionPreserving => "recognition-preserving",
    }
}

const fn candidate_status_name(status: TransformCandidateStatus) -> &'static str {
    match status {
        TransformCandidateStatus::Applied => "applied",
        TransformCandidateStatus::Eligible => "eligible",
        TransformCandidateStatus::Declined => "declined",
    }
}

fn static_lexer_dfa_pass_value(
    id: usize,
    lexers: &[&StaticLexerDfaReport],
    sources: &SourceSet,
) -> Value {
    let changed = lexers.iter().any(|lexer| {
        lexer
            .modes
            .iter()
            .any(|mode| mode.status != StaticLexerModeStatus::Failed)
    });
    let fully_static_lexers = lexers.iter().filter(|lexer| lexer.omit_atn).count();
    let total_modes: usize = lexers.iter().map(|lexer| lexer.modes.len()).sum();
    let compiled_modes: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .filter(|mode| mode.status != StaticLexerModeStatus::Failed)
        .count();
    let failed_modes = total_modes.saturating_sub(compiled_modes);
    let total_states: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.states.total)
        .sum();
    let accept_states: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.states.accept)
        .sum();
    let accelerated_states: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.states.accelerated)
        .sum();
    let total_edges: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.edges.total)
        .sum();
    let target_edges: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.edges.target)
        .sum();
    let escaped_edges: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.edges.escaped)
        .sum();
    let continuations: usize = lexers
        .iter()
        .flat_map(|lexer| &lexer.modes)
        .map(|mode| mode.continuations.total)
        .sum();
    let table_words: usize = lexers.iter().map(|lexer| lexer.table_words).sum();
    let atn_words: usize = lexers.iter().map(|lexer| lexer.atn_words).sum();

    object(vec![
        ("id", Value::from(id)),
        ("name", Value::from(StaticLexerDfa::NAME)),
        (
            "safetyClass",
            Value::from(safety_name(StaticLexerDfa::SAFETY)),
        ),
        ("changed", Value::from(changed)),
        (
            "metrics",
            object(vec![
                ("lexers", Value::from(lexers.len())),
                ("fullyStaticLexers", Value::from(fully_static_lexers)),
                ("modes", Value::from(total_modes)),
                ("compiledModes", Value::from(compiled_modes)),
                ("failedModes", Value::from(failed_modes)),
                ("states", Value::from(total_states)),
                ("acceptStates", Value::from(accept_states)),
                ("acceleratedStates", Value::from(accelerated_states)),
                ("edges", Value::from(total_edges)),
                ("targetEdges", Value::from(target_edges)),
                ("escapedEdges", Value::from(escaped_edges)),
                ("continuations", Value::from(continuations)),
                ("tableWords", Value::from(table_words)),
                ("tableBytes", Value::from(table_words * 4)),
                ("atnWords", Value::from(atn_words)),
            ]),
        ),
        (
            "lexers",
            Value::from_iter(
                lexers
                    .iter()
                    .map(|lexer| static_lexer_report_value(lexer, sources)),
            ),
        ),
    ])
}

fn static_lexer_report_value(lexer: &StaticLexerDfaReport, sources: &SourceSet) -> Value {
    let compiled_modes = lexer
        .modes
        .iter()
        .filter(|mode| mode.status != StaticLexerModeStatus::Failed)
        .count();
    let failed_modes = lexer.modes.len().saturating_sub(compiled_modes);
    let total_states: usize = lexer.modes.iter().map(|mode| mode.states.total).sum();
    let accept_states: usize = lexer.modes.iter().map(|mode| mode.states.accept).sum();
    let accelerated_states: usize = lexer.modes.iter().map(|mode| mode.states.accelerated).sum();
    let total_edges: usize = lexer.modes.iter().map(|mode| mode.edges.total).sum();
    let target_edges: usize = lexer.modes.iter().map(|mode| mode.edges.target).sum();
    let escaped_edges: usize = lexer.modes.iter().map(|mode| mode.edges.escaped).sum();
    let continuations: usize = lexer
        .modes
        .iter()
        .map(|mode| mode.continuations.total)
        .sum();

    object(vec![
        ("grammar", Value::from(lexer.grammar.as_str())),
        ("source", source_value(&lexer.source_span, sources)),
        ("omitAtn", Value::from(lexer.omit_atn)),
        ("tableWords", Value::from(lexer.table_words)),
        ("tableBytes", Value::from(lexer.table_words * 4)),
        ("atnWords", Value::from(lexer.atn_words)),
        ("numModes", Value::from(lexer.modes.len())),
        ("compiledModes", Value::from(compiled_modes)),
        ("failedModes", Value::from(failed_modes)),
        ("totalStates", Value::from(total_states)),
        ("acceptStates", Value::from(accept_states)),
        ("acceleratedStates", Value::from(accelerated_states)),
        ("totalEdges", Value::from(total_edges)),
        ("targetEdges", Value::from(target_edges)),
        ("escapedEdges", Value::from(escaped_edges)),
        ("continuations", Value::from(continuations)),
        (
            "modes",
            Value::from_iter(
                lexer
                    .modes
                    .iter()
                    .map(|mode| static_lexer_mode_value(mode, sources)),
            ),
        ),
    ])
}

fn static_lexer_mode_value(mode: &StaticLexerModeReport, sources: &SourceSet) -> Value {
    let counts = mode.edges.escape_counts;
    let escape_total =
        counts.predicate + counts.custom_action + counts.recursive_context + counts.budget;
    object(vec![
        ("index", Value::from(mode.index)),
        ("mode", Value::from(mode.mode.as_str())),
        ("source", source_value(&mode.source_span, sources)),
        ("status", Value::from(mode_status_name(mode.status))),
        (
            "failure",
            option(mode.failure.as_ref(), static_lexer_mode_failure_value),
        ),
        (
            "statistics",
            object(vec![
                (
                    "states",
                    object(vec![
                        ("total", Value::from(mode.states.total)),
                        ("accept", Value::from(mode.states.accept)),
                        ("nonAccept", Value::from(mode.states.non_accept)),
                        ("accelerated", Value::from(mode.states.accelerated)),
                        ("withEscapes", Value::from(mode.states.with_escapes)),
                        ("maxConfigs", Value::from(mode.states.max_configs)),
                        ("totalConfigs", Value::from(mode.states.total_configs)),
                    ]),
                ),
                (
                    "edges",
                    object(vec![
                        ("total", Value::from(mode.edges.total)),
                        ("target", Value::from(mode.edges.target)),
                        ("escaped", Value::from(mode.edges.escaped)),
                        ("eof", Value::from(mode.edges.eof)),
                        (
                            "selfLoopIntervals",
                            Value::from(mode.edges.self_loop_intervals),
                        ),
                        (
                            "asciiPopulatedCells",
                            Value::from(mode.edges.ascii_populated_cells),
                        ),
                        ("hiSymbolClasses", Value::from(mode.edges.hi_symbol_classes)),
                        ("hiDistinctRows", Value::from(mode.edges.hi_distinct_rows)),
                        (
                            "escapeCounts",
                            object(vec![
                                ("budget", Value::from(counts.budget)),
                                ("customAction", Value::from(counts.custom_action)),
                                ("predicate", Value::from(counts.predicate)),
                                ("recursiveContext", Value::from(counts.recursive_context)),
                                ("total", Value::from(escape_total)),
                            ]),
                        ),
                    ]),
                ),
                (
                    "continuations",
                    object(vec![
                        ("total", Value::from(mode.continuations.total)),
                        (
                            "movedConfigs",
                            Value::from(mode.continuations.moved_configs),
                        ),
                        ("sourceSet", Value::from(mode.continuations.source_set)),
                    ]),
                ),
            ]),
        ),
        (
            "simpleRecursiveRules",
            Value::from_iter(
                mode.simple_recursive_rules
                    .iter()
                    .map(|rule| simple_recursive_rule_value(rule, sources)),
            ),
        ),
        (
            "acceleratedStates",
            Value::from_iter(mode.accelerated_states.iter().map(accelerated_state_value)),
        ),
        (
            "escapedEdges",
            Value::from_iter(
                mode.escaped_edges
                    .iter()
                    .map(|edge| escaped_edge_value(edge, sources)),
            ),
        ),
        (
            "ruleFootprint",
            Value::from_iter(
                mode.rule_footprint
                    .iter()
                    .map(|footprint| rule_footprint_value(footprint, sources)),
            ),
        ),
    ])
}

fn static_lexer_mode_failure_value(failure: &StaticLexerModeFailureReport) -> Value {
    object(vec![
        ("code", Value::from(failure.code)),
        ("kind", Value::from(failure.kind)),
        ("reason", Value::from(failure.reason.as_str())),
        (
            "rules",
            Value::from_iter(failure.rules.iter().map(|rule| Value::from(rule.as_str()))),
        ),
        ("hint", Value::from(failure.hint)),
    ])
}

fn simple_recursive_rule_value(
    rule: &StaticLexerSimpleRecursiveRuleReport,
    sources: &SourceSet,
) -> Value {
    object(vec![
        ("rule", Value::from(rule.rule.as_str())),
        ("source", source_value(&rule.source_span, sources)),
        ("opener", Value::from(rule.opener.as_str())),
        ("closer", Value::from(rule.closer.as_str())),
        ("delimiterBytes", Value::from(rule.delimiter_bytes)),
    ])
}

fn accelerated_state_value(state: &StaticLexerAcceleratedStateReport) -> Value {
    object(vec![
        ("state", Value::from(state.state)),
        (
            "acceptRule",
            option(state.accept_rule.as_deref(), Value::from),
        ),
        (
            "acceptTokenType",
            option(state.accept_token_type, Value::from),
        ),
        (
            "rules",
            Value::from_iter(state.rules.iter().map(|rule| Value::from(rule.as_str()))),
        ),
        (
            "selfLoopIntervals",
            Value::from_iter(
                state
                    .self_loop_intervals
                    .iter()
                    .map(|interval| Value::from(interval.as_str())),
            ),
        ),
        ("accel", accel_kind_value(&state.accel)),
    ])
}

fn accel_kind_value(accel: &StaticLexerAccelKindReport) -> Value {
    match accel {
        StaticLexerAccelKindReport::Positive {
            ascii_count,
            ascii_intervals,
            bitmap_hex,
            hi_self_loop,
            excluded_ascii,
            hint,
        } => object(vec![
            ("kind", Value::from("positive")),
            ("asciiCount", Value::from(*ascii_count)),
            (
                "asciiIntervals",
                Value::from_iter(
                    ascii_intervals
                        .iter()
                        .map(|interval| Value::from(interval.as_str())),
                ),
            ),
            ("bitmapHex", Value::from(bitmap_hex.as_str())),
            ("hiSelfLoop", Value::from(*hi_self_loop)),
            (
                "excludedAscii",
                Value::from_iter(
                    excluded_ascii
                        .iter()
                        .map(|interval| Value::from(interval.as_str())),
                ),
            ),
            ("hint", option(hint.as_deref(), Value::from)),
        ]),
        StaticLexerAccelKindReport::Exclusion {
            needle_count,
            needles,
            needle_bytes,
        } => object(vec![
            ("kind", Value::from("exclusion")),
            ("needleCount", Value::from(*needle_count)),
            (
                "needles",
                Value::from_iter(needles.iter().map(|needle| Value::from(needle.as_str()))),
            ),
            (
                "needleBytes",
                Value::from_iter(needle_bytes.iter().copied().map(Value::from)),
            ),
        ]),
        StaticLexerAccelKindReport::CountedExclusion {
            opener,
            closer,
            delimiter_bytes,
        } => object(vec![
            ("kind", Value::from("counted-exclusion")),
            ("opener", Value::from(opener.as_str())),
            ("closer", Value::from(closer.as_str())),
            ("delimiterBytes", Value::from(*delimiter_bytes)),
        ]),
    }
}

fn escaped_edge_value(edge: &StaticLexerEscapedEdgeReport, sources: &SourceSet) -> Value {
    object(vec![
        ("state", Value::from(edge.state)),
        (
            "stateRules",
            Value::from_iter(
                edge.state_rules
                    .iter()
                    .map(|rule| Value::from(rule.as_str())),
            ),
        ),
        ("symbols", Value::from(edge.symbols.as_str())),
        ("symbolStart", Value::from(edge.symbol_start)),
        ("symbolEnd", Value::from(edge.symbol_end)),
        ("reason", Value::from(edge.reason)),
        ("message", Value::from(edge.message)),
        ("rule", option(edge.rule.as_deref(), Value::from)),
        ("source", source_value(&edge.source_span, sources)),
        ("hint", Value::from(edge.hint)),
        (
            "continuation",
            option(edge.continuation.as_ref(), continuation_value),
        ),
    ])
}

fn continuation_value(continuation: &StaticLexerContinuationReport) -> Value {
    object(vec![
        ("id", Value::from(continuation.id)),
        ("kind", Value::from(continuation.kind)),
        ("configCount", Value::from(continuation.config_count)),
        ("contextCount", Value::from(continuation.context_count)),
        (
            "nonGreedyConfigs",
            Value::from(continuation.non_greedy_configs),
        ),
        (
            "rules",
            Value::from_iter(
                continuation
                    .rules
                    .iter()
                    .map(|rule| Value::from(rule.as_str())),
            ),
        ),
    ])
}

fn rule_footprint_value(footprint: &StaticLexerRuleFootprintReport, sources: &SourceSet) -> Value {
    object(vec![
        ("rule", Value::from(footprint.rule.as_str())),
        ("source", source_value(&footprint.source_span, sources)),
        ("states", Value::from(footprint.states)),
        ("acceptStates", Value::from(footprint.accept_states)),
        (
            "acceleratedStates",
            Value::from(footprint.accelerated_states),
        ),
        ("escapedEdges", Value::from(footprint.escaped_edges)),
        ("countedExclusion", Value::from(footprint.counted_exclusion)),
    ])
}

const fn mode_status_name(status: StaticLexerModeStatus) -> &'static str {
    match status {
        StaticLexerModeStatus::Compiled => "compiled",
        StaticLexerModeStatus::Partial => "partial",
        StaticLexerModeStatus::Failed => "failed",
    }
}

pub(crate) fn render_unmodified_sources(sources: &SourceSet) -> BTreeMap<SourceId, String> {
    sources
        .iter()
        .map(|source| (source.id(), source.text().to_owned()))
        .collect()
}
