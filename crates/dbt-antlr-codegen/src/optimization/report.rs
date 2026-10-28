// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeSet;

use crate::grammar::source::SourceSet;
use crate::grammar::transform::TransformReport;

use super::config::OptimizationPlan;
use super::descriptor::PassDescriptor;
use super::manifest::render_optimization_manifest;
use super::static_lexer_dfa::StaticLexerDfaReport;

/// Everything the optimization pipeline reports about one compilation.
///
/// Each optimization stage appends its own pass report here, so the manifest
/// and user-facing messages are rendered from one place.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct OptimizationReport {
    pub(crate) transforms: TransformReport,
    pub(crate) static_lexer_dfa: Vec<StaticLexerDfaReport>,
}

impl OptimizationReport {
    pub(crate) const fn new(transforms: TransformReport) -> Self {
        Self {
            transforms,
            static_lexer_dfa: Vec::new(),
        }
    }

    pub(crate) fn push_static_lexer_dfa(&mut self, report: StaticLexerDfaReport) {
        self.static_lexer_dfa.push(report);
    }
}

impl OptimizationPlan {
    pub(crate) fn diagnostic_messages(
        &self,
        report: &OptimizationReport,
        sources: &SourceSet,
    ) -> Vec<String> {
        self.selected
            .iter()
            .filter_map(|pass| {
                pass.messages
                    .map(|render| render(report, sources, pass.descriptor))
            })
            .flatten()
            .collect()
    }

    pub(crate) fn render_manifest(
        &self,
        report: &OptimizationReport,
        sources: &SourceSet,
    ) -> Option<String> {
        let manifest_passes = self
            .selected
            .iter()
            .filter(|pass| pass.emit_manifest)
            .collect::<Vec<_>>();
        if manifest_passes.is_empty() {
            return None;
        }
        Some(render_optimization_manifest(
            report,
            &manifest_passes,
            sources,
            self.report_only,
        ))
    }
}

pub(super) fn pruned_unreachable_rule_messages(
    report: &OptimizationReport,
    sources: &SourceSet,
    descriptor: &PassDescriptor,
) -> Vec<String> {
    let pass_ids = report
        .transforms
        .entries
        .iter()
        .filter(|entry| entry.name == descriptor.id)
        .map(|entry| entry.id)
        .collect::<BTreeSet<_>>();
    report
        .transforms
        .rule_removals
        .iter()
        .filter(|removal| pass_ids.contains(&removal.pass))
        .map(|removal| {
            let source = sources.get(removal.source_span.source);
            let path = source.map_or_else(
                || "<grammar>".to_owned(),
                |source| source.logical_path().display().to_string(),
            );
            let position = source
                .and_then(|source| source.line_column(removal.source_span.bytes.start))
                .map_or_else(String::new, |(line, column)| format!(":{line}:{column}"));
            format!(
                "pruned: {path}{position}: unreachable parser rule {}.{}",
                removal.grammar, removal.rule
            )
        })
        .collect()
}
