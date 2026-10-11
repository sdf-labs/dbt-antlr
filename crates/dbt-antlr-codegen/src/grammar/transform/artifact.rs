// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeMap;

use super::json::Json;
use super::{
    SafetyClass, StructuralMetrics, TransformAlternativeMapping, TransformCallSite,
    TransformCandidateReport, TransformCandidateStatus, TransformLabelMapping, TransformProjection,
    TransformReport, TransformReportEntry,
};
use crate::grammar::frontend::{SourceId, SourceSpan};
use crate::grammar::source::SourceSet;

pub(crate) fn render_optimization_manifest(
    report: &TransformReport,
    sources: &SourceSet,
    report_only: bool,
) -> String {
    let passes = report
        .entries
        .iter()
        .map(|entry| optimization_pass_json(entry, report, sources))
        .collect();
    Json::Object(vec![
        ("version", Json::Number(1)),
        ("reportOnly", Json::Bool(report_only)),
        ("passes", Json::Array(passes)),
    ])
    .to_pretty_string()
}

fn optimization_pass_json<'a>(
    entry: &'a TransformReportEntry,
    report: &'a TransformReport,
    sources: &'a SourceSet,
) -> Json<'a> {
    let candidates = report
        .candidates
        .iter()
        .filter(|candidate| candidate.pass == entry.id)
        .map(|candidate| transform_candidate_json(candidate, sources))
        .collect();
    Json::Object(vec![
        ("id", entry.id.index().into()),
        ("name", Json::string(entry.name)),
        ("safetyClass", Json::string(safety_name(entry.safety))),
        ("changed", Json::Bool(entry.changed)),
        (
            "metrics",
            Json::Object(vec![
                ("before", structural_metrics_json(&entry.before)),
                ("after", structural_metrics_json(&entry.after)),
            ]),
        ),
        ("candidates", Json::Array(candidates)),
    ])
}

fn structural_metrics_json(metrics: &StructuralMetrics) -> Json<'static> {
    Json::Object(vec![
        ("rules", metrics.rules.into()),
        ("alternatives", metrics.alternatives.into()),
        ("elements", metrics.elements.into()),
    ])
}

fn transform_candidate_json<'a>(
    candidate: &'a TransformCandidateReport,
    sources: &'a SourceSet,
) -> Json<'a> {
    Json::Object(vec![
        ("grammar", Json::string(&candidate.grammar)),
        ("entryRule", Json::string(&candidate.entry_rule)),
        ("source", source_json(&candidate.source_span, sources)),
        (
            "status",
            Json::string(candidate_status_name(candidate.status)),
        ),
        ("reason", Json::string(&candidate.reason)),
        (
            "rungs",
            Json::Array(
                candidate
                    .rungs
                    .iter()
                    .map(|rung| Json::string(rung))
                    .collect(),
            ),
        ),
        (
            "boundaryRule",
            option_json(candidate.boundary_rule.as_deref(), Json::string),
        ),
        (
            "projected",
            option_json(candidate.projection.as_ref(), transform_projection_json),
        ),
        (
            "removedRules",
            Json::Array(
                candidate
                    .removed_rules
                    .iter()
                    .map(|removed| {
                        Json::Object(vec![
                            ("rule", Json::string(&removed.rule)),
                            (
                                "targetRule",
                                option_json(removed.target.as_deref(), Json::string),
                            ),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "alternatives",
            Json::Array(
                candidate
                    .alternatives
                    .iter()
                    .map(|mapping| alternative_mapping_json(mapping, sources))
                    .collect(),
            ),
        ),
        (
            "labelRenames",
            Json::Array(
                candidate
                    .labels
                    .iter()
                    .map(|mapping| label_mapping_json(mapping, sources))
                    .collect(),
            ),
        ),
        (
            "groupingChanges",
            Json::Array(
                candidate
                    .grouping_changes
                    .iter()
                    .map(|rule| {
                        Json::Object(vec![
                            ("rule", Json::string(rule)),
                            ("from", Json::string("flat-loop")),
                            ("to", Json::string("left-recursive-nesting")),
                        ])
                    })
                    .collect(),
            ),
        ),
        (
            "inlinedCallSites",
            Json::Array(
                candidate
                    .call_sites
                    .iter()
                    .map(|site| call_site_json(site, sources))
                    .collect(),
            ),
        ),
    ])
}

fn call_site_json<'a>(site: &'a TransformCallSite, sources: &'a SourceSet) -> Json<'a> {
    Json::Object(vec![
        ("caller", Json::string(&site.caller)),
        ("alternative", site.alternative.into()),
        ("source", source_json(&site.source_span, sources)),
    ])
}

fn transform_projection_json(projection: &TransformProjection) -> Json<'static> {
    Json::Object(vec![
        (
            "contextsPerOperand",
            reduction_json(
                projection.contexts_per_operand_before,
                projection.contexts_per_operand_after,
            ),
        ),
        (
            "precedenceDecisions",
            reduction_json(
                projection.precedence_decisions_before,
                projection.precedence_decisions_after,
            ),
        ),
    ])
}

fn reduction_json(before: usize, after: usize) -> Json<'static> {
    Json::Object(vec![
        ("before", before.into()),
        ("after", after.into()),
        ("reduction", before.saturating_sub(after).into()),
    ])
}

fn alternative_mapping_json<'a>(
    mapping: &'a TransformAlternativeMapping,
    sources: &'a SourceSet,
) -> Json<'a> {
    Json::Object(vec![
        ("sourceRule", Json::string(&mapping.source_rule)),
        ("sourceAlternative", mapping.source_alternative.into()),
        ("source", source_json(&mapping.source_span, sources)),
        ("targetRule", Json::string(&mapping.target_rule)),
        (
            "targetAltLabels",
            Json::Array(
                mapping
                    .target_alternatives
                    .iter()
                    .map(|label| Json::string(label))
                    .collect(),
            ),
        ),
    ])
}

fn label_mapping_json<'a>(mapping: &'a TransformLabelMapping, sources: &'a SourceSet) -> Json<'a> {
    Json::Object(vec![
        ("sourceRule", Json::string(&mapping.source_rule)),
        ("sourceLabel", Json::string(&mapping.source_label)),
        ("source", source_json(&mapping.source_span, sources)),
        ("targetLabel", Json::string(&mapping.target_label)),
    ])
}

fn source_json<'a>(span: &SourceSpan, sources: &'a SourceSet) -> Json<'a> {
    Json::Object(vec![
        (
            "path",
            option_json(sources.logical_path(span.source), |path| {
                Json::String(path.to_string_lossy())
            }),
        ),
        ("byteStart", span.bytes.start.into()),
        ("byteEnd", span.bytes.end.into()),
        (
            "start",
            option_json(
                sources.line_column(span.source, span.bytes.start),
                line_column_json,
            ),
        ),
        (
            "end",
            option_json(
                sources.line_column(span.source, span.bytes.end),
                line_column_json,
            ),
        ),
    ])
}

fn line_column_json((line, column): (usize, usize)) -> Json<'static> {
    Json::Object(vec![("line", line.into()), ("column", column.into())])
}

fn option_json<'a, T>(value: Option<T>, render: impl FnOnce(T) -> Json<'a>) -> Json<'a> {
    value.map_or(Json::Null, render)
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
