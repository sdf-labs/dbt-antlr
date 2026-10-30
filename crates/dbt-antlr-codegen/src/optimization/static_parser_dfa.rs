// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-parser-dfa` optimization pass (parser scope).
//!
//! The pass classifies the prediction decisions of every root parser unit,
//! builds the static prediction tables, and hands the emitter both the tables
//! (which it turns into `dfa_predict` call sites) and the `SDFA` word stream
//! that rides in the trailing section of the packed parser ATN. It reads the
//! runtime ATN of the compiled unit directly, so the analysis never serializes
//! or deserializes words.
//!
//! `--report-decisions` renders the decision inventory of the pass to stdout
//! and `--dump-dfas` renders the tables of the selected decisions. Both flags
//! imply `--report-only`.
//!
//! Under `--static-dfa-only`, the pass resolves the escape states of every
//! partial table statically (see `parser_dfa::static_only`). A decision that
//! still needs `adaptivePredict` is an error (`G4S190`), and a decision with
//! resolved escape states gets a warning (`G4S192`).

use std::collections::BTreeMap;

use minijinja::Value;

use super::config::AtnFallback;
use super::descriptor::PassDescriptor;
use super::manifest::{object, source_value};
use dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables as RuntimeTables;

use super::parser_dfa::{
    Category, DecisionClassifier, DecisionResult, EscapeResolution, EscapeResolutions,
    StaticDfaTable,
};
use crate::grammar::atn::CompiledParser;
use crate::grammar::diagnostic::Diagnostic;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::{
    Alternative, Block, Element, ElementKind, ModelNodeId, Quantifier, RecognizerModel, Rule,
};
use crate::grammar::provenance::Origin;
use crate::grammar::source::SourceSet;
use crate::grammar::transform::SafetyClass;

/// The diagnostic code of the "decision is not fully static" warnings, and of
/// the errors that replace them under `--static-dfa-only`.
const NOT_FULLY_STATIC: &str = "G4S190";
/// The diagnostic code of an unmatched `--dump-dfas` selector.
const DUMP_SELECTOR_NO_MATCH: &str = "G4S191";
/// The diagnostic code of a decision whose escape states `--static-dfa-only`
/// resolved statically.
const STATIC_ONLY_RESOLVED: &str = "G4S192";

/// The static tables of one parser unit, as the emitter and the packed ATN
/// need them.
#[derive(Clone, Debug, Default)]
pub(crate) struct StaticParserTables {
    tables: RuntimeTables,
    /// `true` per decision that has at least one static table.
    has_table: Vec<bool>,
    /// `true` per decision that keeps an adaptive path, so the generated code
    /// has to defer to `adaptivePredict` when prediction returns no
    /// alternative.
    defers: Vec<bool>,
    atn_fallback: AtnFallback,
}

impl StaticParserTables {
    fn new(results: &[DecisionResult], atn_fallback: AtnFallback) -> Self {
        let mut has_table = Vec::with_capacity(results.len());
        let mut defers = Vec::with_capacity(results.len());
        for result in results {
            has_table.push(
                result.table.is_some()
                    || result
                        .precedence
                        .as_ref()
                        .is_some_and(|precedence| precedence.tables.iter().any(Option::is_some)),
            );
            defers.push(!result.is_fully_static());
        }
        Self {
            tables: super::parser_dfa::build_tables(results),
            has_table,
            defers,
            atn_fallback,
        }
    }

    /// `true` when the decision predicts from a static table.
    pub(crate) fn has_table(&self, decision: usize) -> bool {
        self.has_table.get(decision).copied().unwrap_or(false)
    }

    /// `true` when the decision still reaches the adaptive simulation.
    pub(crate) fn defers(&self, decision: usize) -> bool {
        self.defers.get(decision).copied().unwrap_or(false)
    }

    /// Whether the generated parser can call `adaptivePredict` at all.
    pub(crate) const fn atn_fallback(&self) -> AtnFallback {
        self.atn_fallback
    }

    /// The `SDFA` word stream, empty when the unit has no table.
    pub(crate) fn words(&self) -> Vec<u32> {
        self.tables.to_words()
    }
}

pub(crate) struct StaticParserDfa;

impl StaticParserDfa {
    pub(crate) const NAME: &'static str = "static-parser-dfa";
    pub(crate) const SAFETY: SafetyClass = SafetyClass::TreeAndApiPreserving;

    /// Classifies the decisions of one compiled parser unit, builds the tables
    /// the generated parser will decode, and records the diagnostics of the
    /// decisions that stay partly adaptive.
    ///
    /// With [`AtnFallback::Forbidden`], the escape states of every partial
    /// table are resolved statically, and every decision that still needs
    /// `adaptivePredict` is an error.
    pub(crate) fn build(
        compiled: &CompiledParser,
        atn_fallback: AtnFallback,
    ) -> StaticParserDfaArtifact {
        let atn = compiled.atn;
        let recognizer = &compiled.semantic.recognizer;
        let ll1_disjoint = ll1_disjointness(compiled);
        let mut results = DecisionClassifier::new(atn, ll1_disjoint)
            .with_atn_fallback(atn_fallback)
            .classify_all();
        if atn_fallback == AtnFallback::Forbidden {
            for result in &mut results {
                result.resolve_static_only();
            }
        }
        let tables = StaticParserTables::new(&results, atn_fallback);
        let decisions = results
            .iter()
            .map(|result| StaticParserDecisionReport {
                decision: result.decision,
                state: usize::try_from(result.state_number).unwrap_or(0),
                rule: recognizer
                    .rule_names
                    .get(result.rule_index)
                    .cloned()
                    .unwrap_or_default(),
                result: result.clone(),
            })
            .collect::<Vec<_>>();
        let vocabulary = (1..=atn.max_token_type)
            .map(|token_type| token_display_name(recognizer, token_type))
            .collect();
        let report = StaticParserDfaReport {
            grammar: compiled.semantic.unit.name.clone(),
            source_span: compiled.semantic.unit.span.clone(),
            vocabulary,
            decisions,
            atn_fallback,
        };
        StaticParserDfaArtifact {
            diagnostics: match atn_fallback {
                AtnFallback::Allowed => not_fully_static_diagnostics(compiled, &report),
                AtnFallback::Forbidden => static_only_diagnostics(compiled, &report),
            },
            report,
            tables,
        }
    }

    /// Renders the `static-parser-dfa` entry of the optimization manifest.
    pub(super) fn manifest_pass_value(
        id: usize,
        descriptor: &PassDescriptor,
        parsers: &[StaticParserDfaReport],
        sources: &SourceSet,
    ) -> Value {
        let mut totals = ReportMetrics::default();
        for parser in parsers {
            totals.add(parser);
        }
        let metrics = object(vec![
            ("parsers", Value::from(parsers.len())),
            ("decisions", Value::from(totals.decisions)),
            ("staticDecisions", Value::from(totals.static_decisions)),
            ("hybridDecisions", Value::from(totals.hybrid_decisions)),
            ("adaptiveDecisions", Value::from(totals.adaptive_decisions)),
            ("escapeStates", Value::from(totals.escape_states)),
            ("minAltStates", Value::from(totals.min_alt_states)),
            (
                "minViableAltStates",
                Value::from(totals.min_viable_alt_states),
            ),
            ("syntaxErrorStates", Value::from(totals.syntax_error_states)),
        ]);
        let rendered = Value::from_iter(
            parsers
                .iter()
                .map(|parser| parser_report_value(parser, sources)),
        );
        super::manifest::pass_value(
            id,
            descriptor,
            totals.static_decisions > 0,
            metrics,
            vec![
                (
                    "staticDfaOnly",
                    Value::from(
                        parsers
                            .iter()
                            .any(|parser| parser.atn_fallback == AtnFallback::Forbidden),
                    ),
                ),
                ("parsers", rendered),
            ],
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

/// What the pass hands back to the compiler.
pub(crate) struct StaticParserDfaArtifact {
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) report: StaticParserDfaReport,
    /// The tables the generated parser decodes from its packed ATN.
    pub(crate) tables: StaticParserTables,
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
    /// Whether the generated parser can defer to the adaptive simulation.
    pub(crate) atn_fallback: AtnFallback,
}

impl StaticParserDfaReport {
    /// The number of decisions that run without any adaptive prediction.
    fn static_decisions(&self) -> usize {
        self.decisions
            .iter()
            .filter(|decision| decision.result.is_fully_static())
            .count()
    }

    /// The number of decisions that the classifier gave a partial table. Under
    /// `--static-dfa-only`, these decisions are also static decisions, because
    /// their escape states are resolved.
    fn hybrid_decisions(&self) -> usize {
        self.decisions
            .iter()
            .filter(|decision| decision.result.category == Category::HYBRID)
            .count()
    }

    /// The number of decisions that keep at least one adaptive path.
    fn adaptive_decisions(&self) -> usize {
        self.decisions
            .iter()
            .filter(|decision| !decision.result.is_fully_static())
            .count()
    }

    fn escape_states(&self) -> usize {
        self.decisions
            .iter()
            .map(|decision| decision.result.escapes)
            .sum()
    }

    /// The escape states that `--static-dfa-only` resolved, summed over the
    /// decisions.
    fn resolved_states(&self) -> EscapeResolutions {
        let mut total = EscapeResolutions::default();
        for resolved in self
            .decisions
            .iter()
            .filter_map(|decision| decision.result.static_only.as_ref())
        {
            total.min_alt += resolved.min_alt;
            total.min_viable_alt += resolved.min_viable_alt;
            total.syntax_error += resolved.syntax_error;
        }
        total
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ReportMetrics {
    decisions: usize,
    static_decisions: usize,
    hybrid_decisions: usize,
    adaptive_decisions: usize,
    escape_states: usize,
    min_alt_states: usize,
    min_viable_alt_states: usize,
    syntax_error_states: usize,
}

impl ReportMetrics {
    fn add(&mut self, parser: &StaticParserDfaReport) {
        self.decisions += parser.decisions.len();
        self.static_decisions += parser.static_decisions();
        self.hybrid_decisions += parser.hybrid_decisions();
        self.adaptive_decisions += parser.adaptive_decisions();
        self.escape_states += parser.escape_states();
        let resolved = parser.resolved_states();
        self.min_alt_states += resolved.min_alt;
        self.min_viable_alt_states += resolved.min_viable_alt;
        self.syntax_error_states += resolved.syntax_error;
    }
}

/// One prediction decision of a parser unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticParserDecisionReport {
    pub(crate) decision: usize,
    /// ATN state number of the decision state.
    pub(crate) state: usize,
    pub(crate) rule: String,
    pub(crate) result: DecisionResult,
}

fn parser_report_value(parser: &StaticParserDfaReport, sources: &SourceSet) -> Value {
    let resolved = parser.resolved_states();
    object(vec![
        ("grammar", Value::from(parser.grammar.as_str())),
        ("source", source_value(&parser.source_span, sources)),
        ("decisions", Value::from(parser.decisions.len())),
        ("staticDecisions", Value::from(parser.static_decisions())),
        ("hybridDecisions", Value::from(parser.hybrid_decisions())),
        (
            "adaptiveDecisions",
            Value::from(parser.adaptive_decisions()),
        ),
        ("escapeStates", Value::from(parser.escape_states())),
        ("minAltStates", Value::from(resolved.min_alt)),
        ("minViableAltStates", Value::from(resolved.min_viable_alt)),
        ("syntaxErrorStates", Value::from(resolved.syntax_error)),
    ])
}

/// Per-decision `LL(1)` disjointness, indexed by decision number.
///
/// `grammar::atn::analysis` already computes the Java `AnalysisPipeline.disjoint`
/// fast path from the grammar graph; the ATN decision numbers are the graph
/// state numbers, so the vector is a direct lookup.
pub(super) fn ll1_disjointness(compiled: &CompiledParser) -> Vec<bool> {
    let by_state = compiled
        .analysis
        .decision_lookahead
        .iter()
        .map(|lookahead| (lookahead.state, lookahead.disjoint))
        .collect::<BTreeMap<_, _>>();
    compiled
        .atn
        .decision_to_state
        .iter()
        .map(|state| {
            by_state
                .get(&usize::try_from(state.get_state_number()).unwrap_or(usize::MAX))
                .copied()
                .unwrap_or(false)
        })
        .collect()
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
    lines.push(summary_line(&report.decisions));
    lines.extend(take_rule_line(&report.decisions));
    lines.extend(decision_rows(&report.decisions));
    lines.extend(k_summary(&report.decisions));
    finish_block(lines)
}

/// The category histogram of a grammar, as one line.
fn summary_line(decisions: &[StaticParserDecisionReport]) -> String {
    let mut counts: BTreeMap<Category, usize> = BTreeMap::new();
    for decision in decisions {
        *counts.entry(decision.result.category).or_default() += 1;
    }
    let mut text = String::from("summary:");
    for category in Category::ALL {
        let count = counts.get(&category).copied().unwrap_or(0);
        if count > 0 {
            text.push_str(&format!(" {category}={count}"));
        }
    }
    text
}

/// The optional-postfix summary: how many decisions have the shape, how many
/// take/skip conflicts the take rule resolved, and how many guarded takes the
/// tables carry.
fn take_rule_line(decisions: &[StaticParserDecisionReport]) -> Option<String> {
    let postfix_decisions = decisions
        .iter()
        .filter(|decision| decision.result.has_postfix_shape)
        .count();
    if postfix_decisions == 0 {
        return None;
    }
    let fires: usize = decisions
        .iter()
        .map(|decision| decision.result.take_rule_fires)
        .sum();
    let guarded: usize = decisions
        .iter()
        .map(|decision| decision.result.guarded_takes)
        .sum();
    Some(format!(
        "take-rule: {postfix_decisions} optional-postfix decisions; fires at {fires} conflict states; guarded takes={guarded}"
    ))
}

/// The lookahead-depth summary: the LL(k) histogram and the deepest lookahead
/// of any decision the construction could bound.
fn k_summary(decisions: &[StaticParserDecisionReport]) -> Vec<String> {
    let mut llk_histogram: BTreeMap<i32, usize> = BTreeMap::new();
    let mut max_llk = 0_i32;
    let mut max_bounded = 0_i32;
    for decision in decisions {
        let result = &decision.result;
        if result.k >= 0 {
            max_bounded = max_bounded.max(result.k);
        }
        if result.category == Category::LLK {
            max_llk = max_llk.max(result.k);
            *llk_histogram.entry(result.k).or_default() += 1;
        }
    }
    let histogram = llk_histogram
        .iter()
        .map(|(k, count)| format!("{k}={count}"))
        .collect::<Vec<_>>()
        .join(", ");
    vec![
        format!("LLK k histogram: {{{histogram}}}"),
        format!("max k: LLK={max_llk}, any bounded non-LL(1)={max_bounded}"),
    ]
}

/// The `#`, rule, ATN-state, category, and cost columns of the inventory.
fn decision_rows(decisions: &[StaticParserDecisionReport]) -> Vec<String> {
    let decision_width = column_width("#", decisions, |decision| decision.decision.to_string());
    let rule_width = column_width("rule", decisions, |decision| decision.rule.clone());
    let state_width = column_width("state", decisions, |decision| decision.state.to_string());
    let category_width = column_width("category", decisions, |decision| {
        decision.result.category.as_str().to_owned()
    });
    let k_width = column_width("k", decisions, |decision| k_text(&decision.result));
    let states_width = column_width("dfaStates", decisions, |decision| {
        decision.result.num_dfa_states.to_string()
    });
    let fallback_width = column_width("fallback", decisions, |decision| {
        decision.result.fallback_text()
    });

    let mut lines = vec![format!(
        "  {:>decision_width$}  {:<rule_width$}  {:>state_width$}  {:<category_width$}  {:>k_width$}  {:>states_width$}  {:>fallback_width$}",
        "#", "rule", "state", "category", "k", "dfaStates", "fallback",
    )];
    lines.extend(decisions.iter().map(|decision| {
        let result = &decision.result;
        format!(
            "  {:>decision_width$}  {:<rule_width$}  {:>state_width$}  {:<category_width$}  {:>k_width$}  {:>states_width$}  {:>fallback_width$}",
            decision.decision,
            decision.rule,
            decision.state,
            result.category.as_str(),
            k_text(result),
            result.num_dfa_states,
            result.fallback_text(),
        )
    }));
    lines
}

fn k_text(result: &DecisionResult) -> String {
    if result.k < 0 {
        "-".to_owned()
    } else {
        result.k.to_string()
    }
}

/// The width of one report column: the widest cell, but never narrower than
/// its header.
fn column_width<T>(header: &str, rows: &[T], cell: impl Fn(&T) -> String) -> usize {
    rows.iter()
        .map(|row| cell(row).len())
        .max()
        .unwrap_or(0)
        .max(header.len())
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
        lines.extend(decision_dump_lines(report, decision, selection.paths));
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

fn decision_dump_lines(
    report: &StaticParserDfaReport,
    decision: &StaticParserDecisionReport,
    with_paths: bool,
) -> Vec<String> {
    let result = &decision.result;
    let headline = if result.table.is_none() && result.precedence.is_none() {
        format!(
            "--- decision {}: rule '{}' ({}) ---",
            decision.decision, decision.rule, result.category
        )
    } else {
        format!(
            "--- decision {}: rule '{}' ({}, k={}, {} states) ---",
            decision.decision,
            decision.rule,
            result.category,
            k_text(result),
            result.num_dfa_states
        )
    };
    let mut lines = vec![String::new(), headline];
    match (&result.table, &result.precedence) {
        (_, Some(precedence)) => {
            lines.push(format!("precedence classes: {:?}", precedence.cutoffs));
            for (index, table) in precedence.tables.iter().enumerate() {
                match table {
                    Some(table) => {
                        lines.push(format!("class {index}:"));
                        lines.extend(table_dump_lines(report, table, with_paths));
                    }
                    None => lines.push(format!(
                        "class {index}: no static table: this precedence class runs on adaptivePredict"
                    )),
                }
            }
        }
        (Some(table), None) => lines.extend(table_dump_lines(report, table, with_paths)),
        (None, None) if matches!(result.category, Category::LL1 | Category::NON_GREEDY) => {
            lines.push(format!("  no table needed: {}", adaptive_reason(result)));
        }
        (None, None) => lines.push(format!("  no static table: {}", adaptive_reason(result))),
    }
    lines
}

fn table_dump_lines(
    report: &StaticParserDfaReport,
    table: &StaticDfaTable,
    with_paths: bool,
) -> Vec<String> {
    let mut lines = vec![format!(
        "  table: states={} cyclic={}{}",
        table.states.len(),
        table.cyclic,
        if table.cyclic {
            String::new()
        } else {
            format!(" maxK={}", table.max_k)
        }
    )];
    if table.guarded() > 0 {
        lines.push(format!(
            "  guards: {} guarded states; defers iff the parse stack carries an invoking state of danger={:?} (pass={} states)",
            table.guarded(),
            table.guard_danger,
            table.guard_pass.len()
        ));
    }
    for (index, state) in table.states.iter().enumerate() {
        let mut line = format!("    s{index}: ");
        if state.accept > 0 {
            line.push_str(&format!("accept alt={}", state.accept));
            match state.resolution {
                Some(EscapeResolution::MinAlt) => {
                    line.push_str(" (static-dfa-only: minimum conflicting alternative)");
                }
                Some(EscapeResolution::MinViableAlt) => {
                    line.push_str(" (static-dfa-only: minimum viable alternative)");
                }
                _ => {}
            }
        } else if state.is_guarded() {
            line.push_str(&format!(
                "GUARDED alt={} unless the parse stack reaches a guard root (danger={:?})",
                state.guarded_alt, table.guard_danger
            ));
        } else if state.is_escape() {
            line.push_str("ESCAPE");
        } else if state.resolution == Some(EscapeResolution::SyntaxError) {
            line.push_str("SYNTAX ERROR (static-dfa-only: predicated lookahead)");
        } else {
            line.push_str("(non-accept)");
        }
        if state.fallback > 0 {
            line.push_str(&format!("; error-recovery fallback alt={}", state.fallback));
        }
        if with_paths && let Some(path) = table.witness(index).filter(|path| !path.is_empty()) {
            line.push_str(&format!("; path: {}", path_text(&report.vocabulary, &path)));
        }
        lines.push(line);
        for edge in &state.edges {
            lines.push(format!(
                "      s{index} {}({}) -> s{}",
                interval_text(&report.vocabulary, edge.lo, edge.hi),
                token_range_text(edge.lo, edge.hi),
                edge.target
            ));
        }
    }
    lines
}

/// The display name of one token type of the unit's vocabulary.
fn token_name(vocabulary: &[String], token: i32) -> String {
    if token < 0 {
        return "EOF".to_owned();
    }
    usize::try_from(token)
        .ok()
        .and_then(|token| token.checked_sub(1))
        .and_then(|index| vocabulary.get(index))
        .cloned()
        .unwrap_or_else(|| token.to_string())
}

fn interval_text(vocabulary: &[String], lo: i32, hi: i32) -> String {
    if lo == hi {
        token_name(vocabulary, lo)
    } else {
        format!(
            "{}..{}",
            token_name(vocabulary, lo),
            token_name(vocabulary, hi)
        )
    }
}

fn token_range_text(lo: i32, hi: i32) -> String {
    if lo == hi {
        lo.to_string()
    } else {
        format!("{lo}..{hi}")
    }
}

fn path_text(vocabulary: &[String], path: &[i32]) -> String {
    path.iter()
        .map(|token| token_name(vocabulary, *token))
        .collect::<Vec<_>>()
        .join(" ")
}

fn adaptive_reason(result: &DecisionResult) -> String {
    match result.category {
        Category::LL1 => {
            "the alternatives have disjoint first tokens, so plain LL(1) prediction decides"
                .to_owned()
        }
        Category::NON_GREEDY => {
            "the decision is a non-greedy loop: it always takes the lowest alternative".to_owned()
        }
        Category::PREDICATED => match result.first_predicate {
            Some((rule, index)) => format!(
                "the decision depends on predicate {rule}:{index}, so the prediction is not input-only"
            ),
            None => "the decision depends on a predicate, so the prediction is not input-only"
                .to_owned(),
        },
        Category::CONTEXT_SENSITIVE => {
            "SLL cannot separate the alternatives; only full-context prediction can".to_owned()
        }
        Category::LR_PRECEDENCE => {
            let precedence = result.precedence.as_ref();
            let adaptive = precedence.map_or(0, |precedence| precedence.adaptive_classes);
            if adaptive > 0 {
                let classes = precedence.map_or(0, |precedence| precedence.cutoffs.len() + 1);
                format!("{adaptive} of {classes} precedence classes have no static table")
            } else {
                // Every class has a table, but some frontier states still have
                // to escape to the adaptive simulation.
                format!(
                    "{} of {} precedence-class states escape to adaptivePredict",
                    result.escapes, result.num_dfa_states
                )
            }
        }
        Category::OVERFLOW => format!(
            "the SLL DFA exceeded the analysis budget ({} states or {} visited configs)",
            super::parser_dfa::MAX_DFA_STATES,
            super::parser_dfa::DEFAULT_VISIT_BUDGET
        ),
        Category::HYBRID if result.escapes > 0 && result.guarded_takes > 0 => format!(
            "{} of {} states escape to adaptivePredict, and {} take/skip conflicts are decided by a runtime stack guard",
            result.escapes, result.num_dfa_states, result.guarded_takes
        ),
        Category::HYBRID if result.guarded_takes > 0 => format!(
            "{} take/skip conflicts are decided by a runtime stack guard, which defers to adaptivePredict when the parse stack reaches a guard root",
            result.guarded_takes
        ),
        Category::HYBRID => format!(
            "{} of {} states escape to adaptivePredict",
            result.escapes, result.num_dfa_states
        ),
        _ => "no static table".to_owned(),
    }
}

/// The text of a witness path, `None` when there is no witness or the path is
/// empty.
fn witness_text(vocabulary: &[String], witness: Option<&[i32]>) -> Option<String> {
    witness
        .filter(|witness| !witness.is_empty())
        .map(|witness| path_text(vocabulary, witness))
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
        DUMP_SELECTOR_NO_MATCH,
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
    paths: bool,
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
                selection.paths = true;
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

/// The `G4S190` warnings: one per decision that keeps at least one adaptive
/// path, with the reason and, when the construction has one, the witness.
fn not_fully_static_diagnostics(
    compiled: &CompiledParser,
    report: &StaticParserDfaReport,
) -> Vec<Diagnostic> {
    report
        .decisions
        .iter()
        .filter_map(|decision| {
            let result = &decision.result;
            if result.is_fully_static() {
                return None;
            }
            let mut message = format!(
                "parser decision {} of rule '{}' is not fully static: {}",
                decision.decision,
                decision.rule,
                adaptive_reason(result)
            );
            if let Some(witness) = witness_text(
                &report.vocabulary,
                result
                    .conflict_witness
                    .as_deref()
                    .or(result.budget_witness.as_deref()),
            ) {
                message.push_str(&format!("; witness: {witness}"));
            }
            let span =
                decision_span(compiled, result).unwrap_or_else(|| report.source_span.clone());
            Some(Diagnostic::warning(NOT_FULLY_STATIC, span, message))
        })
        .collect()
}

/// The `--static-dfa-only` diagnostics: a `G4S190` error for each decision that
/// still needs `adaptivePredict`, and a `G4S192` warning for each decision with
/// resolved escape states.
fn static_only_diagnostics(
    compiled: &CompiledParser,
    report: &StaticParserDfaReport,
) -> Vec<Diagnostic> {
    report
        .decisions
        .iter()
        .filter_map(|decision| {
            let result = &decision.result;
            let span =
                decision_span(compiled, result).unwrap_or_else(|| report.source_span.clone());
            let Some(resolved) = &result.static_only else {
                let mut message = format!(
                    "parser decision {} of rule '{}' is not fully static: {}; \
                     --static-dfa-only requires a static table for every decision",
                    decision.decision,
                    decision.rule,
                    static_only_failure_reason(result)
                );
                if let Some(witness) = witness_text(
                    &report.vocabulary,
                    result
                        .conflict_witness
                        .as_deref()
                        .or(result.budget_witness.as_deref()),
                ) {
                    message.push_str(&format!("; witness: {witness}"));
                }
                return Some(Diagnostic::error(NOT_FULLY_STATIC, span, message));
            };
            let mut parts = Vec::new();
            if resolved.min_alt > 0 {
                parts.push(resolution_text(
                    &report.vocabulary,
                    resolved.min_alt,
                    ("conflict state predicts", "conflict states predict"),
                    "the minimum alternative",
                    resolved.min_alt_witness.as_deref(),
                ));
            }
            if resolved.min_viable_alt > 0 {
                parts.push(resolution_text(
                    &report.vocabulary,
                    resolved.min_viable_alt,
                    ("undecided state predicts", "undecided states predict"),
                    "the minimum viable alternative",
                    resolved.min_viable_alt_witness.as_deref(),
                ));
            }
            if resolved.syntax_error > 0 {
                parts.push(resolution_text(
                    &report.vocabulary,
                    resolved.syntax_error,
                    ("predicated state reports", "predicated states report"),
                    "a syntax error",
                    resolved.syntax_error_witness.as_deref(),
                ));
            }
            (!parts.is_empty()).then(|| {
                Diagnostic::warning(
                    STATIC_ONLY_RESOLVED,
                    span,
                    format!(
                        "parser decision {} of rule '{}' resolves its escape states without \
                         adaptivePredict: {}",
                        decision.decision,
                        decision.rule,
                        parts.join(", ")
                    ),
                )
            })
        })
        .collect()
}

/// One clause of a `G4S192` warning, for example `2 predicated states report a
/// syntax error (witness: ID EOF)`.
fn resolution_text(
    vocabulary: &[String],
    count: usize,
    (singular, plural): (&str, &str),
    verdict: &str,
    witness: Option<&[i32]>,
) -> String {
    let subject = if count == 1 { singular } else { plural };
    let mut text = format!("{count} {subject} {verdict}");
    if let Some(witness) = witness_text(vocabulary, witness) {
        text.push_str(&format!(" (witness: {witness})"));
    }
    text
}

/// Why `--static-dfa-only` cannot give the decision a static prediction.
fn static_only_failure_reason(result: &DecisionResult) -> String {
    match result.category {
        Category::NON_GREEDY => "the decision is a non-greedy subrule, and the generated parser \
             predicts non-greedy subrules only with adaptivePredict"
            .to_owned(),
        Category::LR_PRECEDENCE if result.precedence.is_none() => {
            "the precedence loop has no precedence predicates to build class tables from".to_owned()
        }
        _ => adaptive_reason(result),
    }
}

/// The source span of one decision, resolved through the ATN state's
/// provenance: the owning rule or element when the state was synthesized for
/// one, the authored span when the state came straight from the grammar, and
/// the whole rule as the fallback.
fn decision_span(compiled: &CompiledParser, result: &DecisionResult) -> Option<SourceSpan> {
    if result.category != Category::LR_PRECEDENCE
        && let Ok(state_number) = usize::try_from(result.state_number)
        && let Some(state) = compiled.graph.states.get(state_number)
    {
        let origins = compiled.provenance.state_origins(state.original);
        for owner in origins.iter().filter_map(|origin| match origin {
            Origin::Synthetic { owner, .. } => Some(*owner),
            _ => None,
        }) {
            if let Some(span) = model_node_span(compiled, owner) {
                return Some(span);
            }
        }
        if let Some(span) = origins.iter().find_map(|origin| match origin {
            Origin::Authored { span, .. } => Some(span),
            _ => None,
        }) {
            return Some(span.clone());
        }
    }
    let rule = compiled.semantic.unit.rules.get(result.rule_index)?;
    Some(rule.span.clone())
}

/// The span of a model node that has grammar syntax behind it.
fn model_node_span(compiled: &CompiledParser, node: ModelNodeId) -> Option<SourceSpan> {
    match node {
        ModelNodeId::Rule(rule_id) => compiled
            .semantic
            .unit
            .rules
            .iter()
            .find(|rule| rule.id == rule_id)
            .map(rule_location),
        ModelNodeId::Element(element_id) => {
            find_element(&compiled.semantic.unit.rules, element_id).map(element_location)
        }
        _ => None,
    }
}

/// The location of a decision that belongs to a whole rule: its block.
fn rule_location(rule: &Rule) -> SourceSpan {
    rule.block.span.clone()
}

/// The location of a decision owned by one element: the element itself, or its
/// whole syntax node (label and quantifier included) when the decision *is*
/// the loop or the optional.
fn element_location(element: &Element) -> SourceSpan {
    let is_quantifier_decision = matches!(
        element.quantifier,
        Quantifier::Optional { .. } | Quantifier::ZeroOrMore { .. } | Quantifier::OneOrMore { .. }
    );
    if is_quantifier_decision {
        return element.enclosing_span.clone();
    }
    element.span.clone()
}

fn find_element(rules: &[Rule], target: crate::grammar::model::ElementId) -> Option<&Element> {
    rules
        .iter()
        .find_map(|rule| find_element_in_block(&rule.block, target))
}

fn find_element_in_block(
    block: &Block,
    target: crate::grammar::model::ElementId,
) -> Option<&Element> {
    block
        .alternatives
        .iter()
        .find_map(|alternative| find_element_in_alt(alternative, target))
}

fn find_element_in_alt(
    alternative: &Alternative,
    target: crate::grammar::model::ElementId,
) -> Option<&Element> {
    for element in &alternative.elements {
        if element.id == target {
            return Some(element);
        }
        if let ElementKind::Block(inner) = &element.kind
            && let Some(found) = find_element_in_block(inner, target)
        {
            return Some(found);
        }
    }
    None
}
