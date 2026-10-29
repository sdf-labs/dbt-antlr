// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-parser-dfa` optimization pass (parser scope).
//!
//! The pass classifies the decisions of every root parser unit and, once the
//! SLL classifier lands, precomputes the static prediction tables that ride in
//! the trailing `SDFA` section of the packed parser ATN. It reads the runtime
//! ATN of the compiled unit directly, so the analysis never serializes or
//! deserializes words.
//!
//! `--report-decisions` renders the decision inventory of the pass to stdout
//! and `--dump-dfas` renders the tables of the selected decisions. Both flags
//! imply `--report-only`.

use minijinja::Value;

use super::descriptor::PassDescriptor;
use super::manifest::{object, source_value};
use crate::grammar::atn::CompiledParser;
use crate::grammar::diagnostic::Diagnostic;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::RecognizerModel;
use crate::grammar::source::SourceSet;
use crate::grammar::transform::SafetyClass;
use dbt_antlr_runtime::atn_state::ATNState;

pub(crate) struct StaticParserDfa;

impl StaticParserDfa {
    pub(crate) const NAME: &'static str = "static-parser-dfa";
    pub(crate) const SAFETY: SafetyClass = SafetyClass::TreeAndApiPreserving;

    /// Inventories the decisions of one compiled parser unit.
    pub(crate) fn report(compiled: &CompiledParser) -> StaticParserDfaReport {
        let atn = compiled.atn;
        let recognizer = &compiled.semantic.recognizer;
        let decisions = atn
            .decision_to_state
            .iter()
            .enumerate()
            .map(|(decision, state)| StaticParserDecisionReport {
                decision,
                state: usize::try_from(state.get_state_number())
                    .expect("decision state has a state number"),
                rule: rule_name(state, recognizer),
            })
            .collect();
        let vocabulary = (1..=atn.max_token_type)
            .map(|token_type| token_display_name(recognizer, token_type))
            .collect();
        StaticParserDfaReport {
            grammar: compiled.semantic.unit.name.clone(),
            source_span: compiled.semantic.unit.span.clone(),
            vocabulary,
            decisions,
        }
    }

    /// Renders the `static-parser-dfa` entry of the optimization manifest.
    pub(super) fn manifest_pass_value(
        id: usize,
        descriptor: &PassDescriptor,
        parsers: &[StaticParserDfaReport],
        sources: &SourceSet,
    ) -> Value {
        let decisions: usize = parsers.iter().map(|parser| parser.decisions.len()).sum();
        // No decision has a static table until the SLL classifier lands.
        let static_decisions = 0_usize;
        let metrics = object(vec![
            ("parsers", Value::from(parsers.len())),
            ("decisions", Value::from(decisions)),
            ("staticDecisions", Value::from(static_decisions)),
            ("adaptiveDecisions", Value::from(decisions)),
        ]);
        let rendered = Value::from_iter(
            parsers
                .iter()
                .map(|parser| parser_report_value(parser, sources)),
        );
        super::manifest::pass_value(
            id,
            descriptor,
            static_decisions > 0,
            metrics,
            vec![("parsers", rendered)],
        )
    }

    /// Renders the `--report-decisions` report of every parser unit.
    pub(crate) fn render_decision_report(reports: &[StaticParserDfaReport]) -> String {
        render_blocks(&reports.iter().map(decision_report_text).collect::<Vec<_>>())
    }

    /// Renders the `--dump-dfas` dump of every parser unit, together with the
    /// diagnostics of the selectors that match no decision.
    pub(crate) fn render_dfa_dump(
        reports: &[StaticParserDfaReport],
        spec: &str,
    ) -> (String, Vec<Diagnostic>) {
        let mut blocks = Vec::new();
        let mut diagnostics = Vec::new();
        for report in reports {
            let (block, block_diagnostics) = dfa_dump_text(report, spec);
            blocks.push(block);
            diagnostics.extend(block_diagnostics);
        }
        (render_blocks(&blocks), diagnostics)
    }
}

/// The report modes `--report-decisions` and `--dump-dfas` select.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct StaticParserDfaSelection {
    pub(crate) report_decisions: bool,
    pub(crate) dump_dfas: Option<String>,
}

/// What the pass reports about one parser unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticParserDfaReport {
    pub(crate) grammar: String,
    pub(crate) source_span: SourceSpan,
    /// Display names of the token types `1..=max_token_type`, indexed by
    /// `token_type - 1`.
    pub(crate) vocabulary: Vec<String>,
    pub(crate) decisions: Vec<StaticParserDecisionReport>,
}

/// One prediction decision of a parser unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticParserDecisionReport {
    pub(crate) decision: usize,
    /// ATN state number of the decision state.
    pub(crate) state: usize,
    pub(crate) rule: String,
}

fn parser_report_value(parser: &StaticParserDfaReport, sources: &SourceSet) -> Value {
    object(vec![
        ("grammar", Value::from(parser.grammar.as_str())),
        ("source", source_value(&parser.source_span, sources)),
        ("decisions", Value::from(parser.decisions.len())),
        ("staticDecisions", Value::from(0usize)),
        ("adaptiveDecisions", Value::from(parser.decisions.len())),
    ])
}

fn rule_name(state: &ATNState, recognizer: &RecognizerModel) -> String {
    usize::try_from(state.get_rule_index())
        .ok()
        .and_then(|rule| recognizer.rule_names.get(rule))
        .cloned()
        .unwrap_or_default()
}

/// The display name of one token type, as the Java tool prints it: the string
/// literal, then the token name, then the numeric type.
fn token_display_name(recognizer: &RecognizerModel, token_type: i32) -> String {
    let Ok(index) = usize::try_from(token_type) else {
        return "EOF".to_owned();
    };
    recognizer
        .literal_names
        .get(index)
        .and_then(Clone::clone)
        .or_else(|| recognizer.symbolic_names.get(index).and_then(Clone::clone))
        .unwrap_or_else(|| token_type.to_string())
}

fn decision_report_text(report: &StaticParserDfaReport) -> String {
    let mut lines = vec![format!(
        "=== parser decision report: {} ({} decisions) ===",
        report.grammar,
        report.decisions.len()
    )];
    lines.extend(decision_rows(&report.decisions));
    finish_block(lines)
}

/// The `#`, rule, and ATN-state columns of the decision inventory.
fn decision_rows(decisions: &[StaticParserDecisionReport]) -> Vec<String> {
    let decision_width = column_width(decisions, |decision| decision.decision.to_string());
    let rule_width = column_width(decisions, |decision| decision.rule.clone());
    let state_width = column_width(decisions, |decision| decision.state.to_string());
    let mut lines = vec![format!(
        "  {:>decision_width$}  {:<rule_width$}  {:>state_width$}",
        "#", "rule", "state",
    )];
    lines.extend(decisions.iter().map(|decision| {
        format!(
            "  {:>decision_width$}  {:<rule_width$}  {:>state_width$}",
            decision.decision, decision.rule, decision.state,
        )
    }));
    lines
}

fn column_width<T>(rows: &[T], cell: impl Fn(&T) -> String) -> usize {
    rows.iter().map(|row| cell(row).len()).max().unwrap_or(0)
}

fn dfa_dump_text(report: &StaticParserDfaReport, spec: &str) -> (String, Vec<Diagnostic>) {
    let selection = DfaDumpSelection::parse(spec);
    let mut diagnostics = Vec::new();
    let mut lines = vec![
        format!("=== static DFA dump: grammar {} ===", report.grammar),
        vocabulary_line(&report.vocabulary),
    ];

    let mut selected = vec![false; selection.selectors.len()];
    let mut shown = 0_usize;
    for decision in &report.decisions {
        let mut hit = selection.all;
        for (index, selector) in selection.selectors.iter().enumerate() {
            if selector.matches(decision) {
                selected[index] = true;
                hit = true;
            }
        }
        if !hit {
            continue;
        }
        shown += 1;
        lines.extend(decision_dump_lines(decision));
    }
    lines.push(String::new());
    lines.push(format!(
        "({shown} of {} decisions dumped)",
        report.decisions.len()
    ));

    if selection.selects_nothing() {
        diagnostics.push(no_matching_decision_error(report, spec));
    }
    diagnostics.extend(
        selection
            .malformed
            .iter()
            .map(|item| no_matching_decision_error(report, item)),
    );
    diagnostics.extend(
        selection
            .selectors
            .iter()
            .zip(&selected)
            .filter(|(_, matched)| !**matched)
            .map(|(selector, _)| no_matching_decision_error(report, &selector.text())),
    );
    (finish_block(lines), diagnostics)
}

fn decision_dump_lines(decision: &StaticParserDecisionReport) -> Vec<String> {
    vec![
        String::new(),
        format!(
            "--- decision {}: rule '{}' ---",
            decision.decision, decision.rule
        ),
        "  no static table: this decision runs on adaptivePredict".to_owned(),
    ]
}

fn vocabulary_line(vocabulary: &[String]) -> String {
    let tokens = vocabulary
        .iter()
        .enumerate()
        .map(|(index, name)| format!(" {name}={}", index + 1))
        .collect::<String>();
    format!("vocabulary: EOF=-1{tokens}")
}

/// Joins report blocks, keeping one blank line between them.
fn render_blocks(blocks: &[String]) -> String {
    let mut text = blocks.join("\n");
    if !blocks.is_empty() {
        text.push('\n');
    }
    text
}

fn finish_block(lines: Vec<String>) -> String {
    let mut text = lines.join("\n");
    text.push('\n');
    text
}

fn no_matching_decision_error(report: &StaticParserDfaReport, selector: &str) -> Diagnostic {
    Diagnostic::error(
        "G4S191",
        report.source_span.clone(),
        format!(
            "--dump-dfas: no parser decision of grammar {} matches '{selector}'",
            report.grammar
        ),
    )
}

/// The parsed selector list of one `--dump-dfas` value.
#[derive(Debug, Default)]
struct DfaDumpSelection {
    all: bool,
    selectors: Vec<DfaDumpSelector>,
    malformed: Vec<String>,
}

impl DfaDumpSelection {
    fn parse(spec: &str) -> Self {
        let mut selection = Self::default();
        for raw in spec.split(',') {
            let item = raw.trim();
            if item.is_empty() {
                continue;
            }
            if item.eq_ignore_ascii_case("all") {
                selection.all = true;
            } else if item.eq_ignore_ascii_case("paths") {
                // A modifier of the state dump: it annotates each state with a
                // shortest lookahead path and selects no decision on its own.
                continue;
            } else if let Some(selector) = DfaDumpSelector::parse(item) {
                if !selection.selectors.contains(&selector) {
                    selection.selectors.push(selector);
                }
            } else if !selection.malformed.iter().any(|known| known == item) {
                selection.malformed.push(item.to_owned());
            }
        }
        selection
    }

    fn selects_nothing(&self) -> bool {
        !self.all && self.selectors.is_empty() && self.malformed.is_empty()
    }
}

/// One `--dump-dfas` selector: a decision number, a rule name, or both.
#[derive(Clone, Debug, Eq, PartialEq)]
enum DfaDumpSelector {
    Decision(usize),
    Rule(String),
    RuleDecision { rule: String, decision: usize },
}

impl DfaDumpSelector {
    fn parse(item: &str) -> Option<Self> {
        if let Some((rule, decision)) = item.split_once(':') {
            let rule = rule.trim();
            let decision = decision_number(decision.trim())?;
            return (!rule.is_empty()).then(|| Self::RuleDecision {
                rule: rule.to_owned(),
                decision,
            });
        }
        Some(match decision_number(item) {
            Some(decision) => Self::Decision(decision),
            None => Self::Rule(item.to_owned()),
        })
    }

    fn matches(&self, decision: &StaticParserDecisionReport) -> bool {
        match self {
            Self::Decision(wanted) => *wanted == decision.decision,
            Self::Rule(wanted) => *wanted == decision.rule,
            Self::RuleDecision {
                rule,
                decision: wanted,
            } => *rule == decision.rule && *wanted == decision.decision,
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Decision(decision) => decision.to_string(),
            Self::Rule(rule) => rule.clone(),
            Self::RuleDecision { rule, decision } => format!("{rule}:{decision}"),
        }
    }
}

fn decision_number(text: &str) -> Option<usize> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}
