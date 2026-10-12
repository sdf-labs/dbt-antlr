// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeMap;

use minijinja::Value;

use super::{
    SafetyClass, StructuralMetrics, TransformAlternativeMapping, TransformCallSite,
    TransformCandidateReport, TransformCandidateStatus, TransformLabelMapping, TransformProjection,
    TransformReport, TransformReportEntry,
};
use crate::grammar::frontend::{SourceId, SourceSpan};
use crate::grammar::source::SourceSet;

/// Renders the optimization manifest with minijinja's built-in `Value`
/// formatting. The manifest is inspection-only output — nothing parses it
/// back — so the debug-pretty dump (sorted keys, `True`/`False`/`None`
/// scalars, Python-literal compatible) suffices and keeps a dedicated JSON
/// serializer out of the `generator` feature.
pub(crate) fn render_optimization_manifest(
    report: &TransformReport,
    sources: &SourceSet,
    report_only: bool,
) -> String {
    let passes = report
        .entries
        .iter()
        .map(|entry| optimization_pass_value(entry, report, sources))
        .collect::<Vec<_>>();
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

pub(crate) fn render_unmodified_sources(sources: &SourceSet) -> BTreeMap<SourceId, String> {
    sources
        .iter()
        .map(|source| (source.id(), source.text().to_owned()))
        .collect()
}
