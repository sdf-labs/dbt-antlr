// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Rendering of the `optimizations.txt` manifest.
//!
//! The manifest lists every selected pass that emits a report, in plan order.
//! Each pass renders its own entry from the shared [`OptimizationReport`], so
//! passes stay independent of one another.

use minijinja::Value;

use super::config::{SelectedPass, SelectedPassStage, StaticDfaPass};
use super::descriptor::PassDescriptor;
use super::report::OptimizationReport;
use super::static_lexer_dfa::StaticLexerDfa;
use super::static_parser_dfa::StaticParserDfa;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::source::SourceSet;
use crate::grammar::transform::{
    SafetyClass, StructuralMetrics, TransformAlternativeMapping, TransformCallSite,
    TransformCandidateReport, TransformCandidateStatus, TransformLabelMapping, TransformProjection,
    TransformReport, TransformReportEntry,
};

/// Renders the optimization manifest with minijinja's built-in `Value`
/// formatting.
pub(crate) fn render_optimization_manifest(
    report: &OptimizationReport,
    manifest_passes: &[&SelectedPass],
    sources: &SourceSet,
    report_only: bool,
) -> String {
    let mut passes = Vec::new();
    for pass in manifest_passes {
        let id = passes.len();
        if let Some(value) = manifest_pass_value(id, pass, report, sources) {
            passes.push(value);
        }
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

fn manifest_pass_value(
    id: usize,
    pass: &SelectedPass,
    report: &OptimizationReport,
    sources: &SourceSet,
) -> Option<Value> {
    match pass.stage {
        SelectedPassStage::GrammarTransforms => {
            let entry = report
                .transforms
                .entries
                .iter()
                .find(|entry| entry.name == pass.descriptor.id);
            debug_assert!(
                entry.is_some(),
                "selected pass {} has no transform report entry",
                pass.descriptor.id
            );
            entry.map(|entry| {
                transform_pass_value(id, pass.descriptor, entry, &report.transforms, sources)
            })
        }
        SelectedPassStage::StaticDfaBuilds(StaticDfaPass::StaticLexerDfa) => {
            Some(StaticLexerDfa::manifest_pass_value(
                id,
                pass.descriptor,
                &report.static_lexer_dfa,
                sources,
            ))
        }
        SelectedPassStage::StaticDfaBuilds(StaticDfaPass::StaticParserDfa) => {
            Some(StaticParserDfa::manifest_pass_value(
                id,
                pass.descriptor,
                &report.static_parser_dfa,
                sources,
            ))
        }
    }
}

/// Renders the common manifest header of one selected optimization pass.
pub(super) fn pass_value(
    id: usize,
    descriptor: &PassDescriptor,
    changed: bool,
    metrics: Value,
    sections: Vec<(&'static str, Value)>,
) -> Value {
    let mut entries = vec![
        ("id", Value::from(id)),
        ("name", Value::from(descriptor.id)),
        ("safetyClass", Value::from(safety_name(descriptor.safety))),
        ("changed", Value::from(changed)),
        ("metrics", metrics),
    ];
    entries.extend(sections);
    object(entries)
}

fn transform_pass_value(
    id: usize,
    descriptor: &PassDescriptor,
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
    pass_value(
        id,
        descriptor,
        entry.changed,
        object(vec![
            ("before", structural_metrics_value(&entry.before)),
            ("after", structural_metrics_value(&entry.after)),
        ]),
        vec![("candidates", Value::from_iter(candidates))],
    )
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

pub(super) fn source_value(span: &SourceSpan, sources: &SourceSet) -> Value {
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

pub(super) fn object(entries: Vec<(&'static str, Value)>) -> Value {
    Value::from_iter(
        entries
            .into_iter()
            .map(|(key, value)| (Value::from(key), value)),
    )
}

pub(super) fn option<T>(value: Option<T>, render: impl FnOnce(T) -> Value) -> Value {
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
