// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-parser-dfa` optimization pass (parser scope).
//!
//! The pass classifies the prediction decisions of every root parser unit,
//! builds the static prediction tables, and hands the emitter both the tables
//! (which it turns into `dfa_predict` call sites) and the `SDFA` table image
//! that the generated parser embeds beside its packed parser ATN. It reads the
//! runtime ATN of the compiled unit directly, so the analysis never serializes
//! or deserializes words.
//!
//! `--report-decisions` renders the decision inventory of the pass to stdout,
//! `--dump-dfas` renders the tables of the selected decisions, and
//! `--profile-decisions` renders the time and the work that the classification
//! of each decision took. These flags imply `--report-only`.
//!
//! A decision that spends the visit budget of the analysis gets a warning
//! (`G4S193`) in every mode, because such a decision makes the code generation
//! slow. A rule that many call sites enter gets a warning (`G4S194`) for each
//! block with many single-token alternatives that do not fold into one token
//! set. The analysis follows each of these alternatives at each site.
//!
//! Under `--static-dfa-only`, the pass resolves the escape states of every
//! partial table statically (see `parser_dfa::static_only`). A decision that
//! still needs `adaptivePredict` is an error (`G4S190`), and a decision with
//! resolved escape states gets a warning (`G4S192`).
//!
//! Each decision has an analysis budget (see `parser_dfa::budget`). The
//! budget options of the grammar, its rules, and its blocks set it.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use minijinja::Value;

use super::config::AtnFallback;
use super::descriptor::PassDescriptor;
use super::manifest::{object, source_value};
use dbt_antlr_runtime::dfa::static_parser_dfa::{
    StaticParserTables as RuntimeTables, VIRTUAL_ALT_BASE,
};

use super::parser_dfa::{
    AttemptOutcome, BudgetLimits, Category, DecisionBudget, DecisionClassifier, DecisionResult,
    DescentPlan, DescentRules, EscapeResolution, EscapeResolutions, ParserDfaBudgets,
    StaticDfaTable, has_non_budget_option, minimized_states,
};
use crate::grammar::atn::{CompiledParser, FinalizedTransitionKind, left_corner_rules};
use crate::grammar::diagnostic::Diagnostic;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::{
    Alternative, Block, Element, ElementKind, ModelNodeId, Quantifier, RecognizerModel, Rule,
    RuleKind, Terminal,
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
/// The diagnostic code of a decision whose analysis spent the visit budget.
const VISIT_BUDGET_SPENT: &str = "G4S193";
/// The diagnostic code of a block of single-token alternatives that do not
/// fold into one token set, in a rule with many call sites.
const TOKEN_BRANCHES: &str = "G4S194";

/// The minimum number of call sites that enter a rule, for a `G4S194`
/// warning.
const TOKEN_BRANCH_ENTRIES: usize = 20;
/// The minimum number of single-token alternatives in a block, for a `G4S194`
/// warning.
const TOKEN_BRANCH_ALTERNATIVES: usize = 64;

/// The number of rules that a `G4S193` warning and a profile row name.
const HOT_RULES: usize = 3;
/// The number of decisions in the table of `--profile-decisions`.
const PROFILE_ROWS: usize = 20;
/// A decision whose classification takes this long is slow.
const SLOW_DECISION: Duration = Duration::from_millis(500);

/// The static tables of one parser unit, as the emitter and the embedded
/// table image need them.
#[derive(Clone, Debug, Default)]
pub(crate) struct StaticParserTables {
    tables: RuntimeTables,
    /// `true` per decision that has at least one static table.
    has_table: Vec<bool>,
    /// `true` per decision that keeps an adaptive path, so the generated code
    /// has to defer to `adaptivePredict` when prediction returns no
    /// alternative.
    defers: Vec<bool>,
    /// The shared-descent groups each decision's table accepts with.
    descent: Vec<Option<DescentPlan>>,
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
            descent: results
                .iter()
                .map(|result| result.descent.clone())
                .collect(),
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

    /// The shared-descent groups the decision's table accepts with, when it
    /// has any: the decision must be emitted as a `DescentAltBlock`.
    pub(crate) fn descent(&self, decision: usize) -> Option<&DescentPlan> {
        self.descent.get(decision).and_then(Option::as_ref)
    }

    /// The shared rules of every decision's groups.
    pub(crate) fn shared_rules(&self) -> impl Iterator<Item = usize> + '_ {
        self.descent
            .iter()
            .flatten()
            .flat_map(|plan| plan.groups.iter().map(|group| group.rule))
    }

    /// Whether the generated parser can call `adaptivePredict` at all.
    pub(crate) const fn atn_fallback(&self) -> AtnFallback {
        self.atn_fallback
    }

    /// The `SDFA` table image, empty when the unit has no table.
    pub(crate) fn words(&self) -> Vec<u32> {
        self.tables.words().to_vec()
    }
}

pub(crate) struct StaticParserDfa;

impl StaticParserDfa {
    pub(crate) const NAME: &'static str = "static-parser-dfa";
    pub(crate) const SAFETY: SafetyClass = SafetyClass::TreeAndApiPreserving;

    /// Classifies the decisions of one compiled parser unit, builds the tables
    /// the generated parser will embed, and records the diagnostics of the
    /// decisions that stay partly adaptive.
    ///
    /// With [`AtnFallback::Forbidden`], the escape states of every partial
    /// table are resolved statically, and every decision that still needs
    /// `adaptivePredict` is an error.
    ///
    /// With `profile`, the report also records the wall-clock times of the
    /// classification (`--profile-decisions`).
    ///
    /// `budgets` holds the base analysis budgets, which the options of the
    /// grammar, its rules, and its blocks change (see [`ParserDfaBudgets`]).
    pub(crate) fn build(
        compiled: &CompiledParser,
        atn_fallback: AtnFallback,
        profile: bool,
        budgets: &ParserDfaBudgets,
    ) -> StaticParserDfaArtifact {
        let atn = compiled.atn;
        let recognizer = &compiled.semantic.recognizer;
        let ll1_disjoint = ll1_disjointness(compiled);
        let (budget, decision_budgets) = decision_budgets(compiled, budgets);
        let classifier = DecisionClassifier::new(atn, ll1_disjoint)
            .with_atn_fallback(atn_fallback)
            .with_descent_rules(descent_rules(compiled))
            .with_budgets(decision_budgets)
            .with_profiling(profile);
        let started = profile.then(Instant::now);
        let mut results = classifier.classify_all();
        let profile = started.map(|started| ClassificationProfile {
            elapsed: started.elapsed(),
            threads: classifier.threads(),
        });
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
            rule_names: recognizer.rule_names.clone(),
            decisions,
            atn_fallback,
            budget,
            profile,
        };
        let mut diagnostics = match atn_fallback {
            AtnFallback::Allowed => not_fully_static_diagnostics(compiled, &report),
            AtnFallback::Forbidden => static_only_diagnostics(compiled, &report),
        };
        diagnostics.extend(visit_budget_diagnostics(compiled, &report));
        diagnostics.extend(token_branch_diagnostics(compiled));
        StaticParserDfaArtifact {
            diagnostics,
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
            ("guardedStates", Value::from(totals.guarded_states)),
            ("minAltStates", Value::from(totals.min_alt_states)),
            (
                "minViableAltStates",
                Value::from(totals.min_viable_alt_states),
            ),
            ("syntaxErrorStates", Value::from(totals.syntax_error_states)),
            (
                "descentGroupStates",
                Value::from(totals.descent_group_states),
            ),
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

    /// Renders the `--profile-decisions` report of every parser unit.
    pub(crate) fn render_decision_profile(reports: &[StaticParserDfaReport]) -> String {
        render_blocks(
            &reports
                .iter()
                .map(decision_profile_text)
                .collect::<Vec<_>>(),
        )
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
    /// The tables the generated parser embeds as its table image.
    pub(crate) tables: StaticParserTables,
}

/// The report modes `--report-decisions`, `--dump-dfas`, and
/// `--profile-decisions` select.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct StaticParserDfaSelection {
    pub(crate) report_decisions: bool,
    pub(crate) dump_dfas: Option<String>,
    pub(crate) profile_decisions: bool,
}

/// The wall-clock time of the classification of one parser unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ClassificationProfile {
    pub(crate) elapsed: Duration,
    /// The threads that classified the decisions.
    pub(crate) threads: usize,
}

/// What the pass reports about one parser unit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticParserDfaReport {
    pub(crate) grammar: String,
    pub(crate) source_span: SourceSpan,
    /// Display names of the token types `1..=max_token_type`, indexed by
    /// `token_type - 1`.
    pub(crate) vocabulary: Vec<String>,
    /// Rule names, indexed by rule index.
    pub(crate) rule_names: Vec<String>,
    pub(crate) decisions: Vec<StaticParserDecisionReport>,
    /// Whether the generated parser can defer to the adaptive simulation.
    pub(crate) atn_fallback: AtnFallback,
    /// The budget of a decision that no rule, block, or selector budget
    /// changes.
    pub(crate) budget: DecisionBudget,
    /// The times of the classification, for `--profile-decisions`.
    pub(crate) profile: Option<ClassificationProfile>,
}

impl StaticParserDfaReport {
    /// The number of decisions whose tables never escape to the adaptive
    /// simulation (see [`DecisionResult::is_fully_static`]).
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

    /// The states that accept with the virtual alternative of a shared-descent
    /// group, summed over the decisions that keep their table.
    fn descent_group_states(&self) -> usize {
        self.decisions
            .iter()
            .filter(|decision| decision.result.descent.is_some())
            .map(|decision| decision.result.descent_group_states)
            .sum()
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
            total.guarded += resolved.guarded;
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
    guarded_states: usize,
    min_alt_states: usize,
    min_viable_alt_states: usize,
    syntax_error_states: usize,
    descent_group_states: usize,
}

impl ReportMetrics {
    fn add(&mut self, parser: &StaticParserDfaReport) {
        self.decisions += parser.decisions.len();
        self.static_decisions += parser.static_decisions();
        self.hybrid_decisions += parser.hybrid_decisions();
        self.adaptive_decisions += parser.adaptive_decisions();
        self.escape_states += parser.escape_states();
        let resolved = parser.resolved_states();
        self.guarded_states += resolved.guarded;
        self.min_alt_states += resolved.min_alt;
        self.min_viable_alt_states += resolved.min_viable_alt;
        self.syntax_error_states += resolved.syntax_error;
        self.descent_group_states += parser.descent_group_states();
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
        ("guardedStates", Value::from(resolved.guarded)),
        ("minAltStates", Value::from(resolved.min_alt)),
        ("minViableAltStates", Value::from(resolved.min_viable_alt)),
        ("syntaxErrorStates", Value::from(resolved.syntax_error)),
        (
            "descentGroupStates",
            Value::from(parser.descent_group_states()),
        ),
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

/// The rule facts that decide which rules a shared-descent group may share,
/// indexed by rule index.
pub(super) fn descent_rules(compiled: &CompiledParser) -> DescentRules {
    let rules = &compiled.semantic.unit.rules;
    DescentRules {
        has_arguments: rules
            .iter()
            .map(|rule| {
                compiled
                    .semantic
                    .bindings
                    .attributes
                    .get(&rule.id)
                    .is_some_and(|attributes| !attributes.arguments.is_empty())
            })
            .collect(),
        has_actions: rules
            .iter()
            .map(|rule| {
                rule.actions
                    .iter()
                    .any(|action| matches!(action.name.as_str(), "init" | "after"))
                    || !rule.catches.is_empty()
                    || rule.finally_action.is_some()
                    || block_has_action(&rule.block)
            })
            .collect(),
    }
}

/// Whether a block runs an embedded action. The empty actions that the
/// left-recursion rewrite inserts run nothing.
fn block_has_action(block: &Block) -> bool {
    block.alternatives.iter().any(|alternative| {
        alternative
            .elements
            .iter()
            .any(|element| match &element.kind {
                ElementKind::Action { body, .. } => !body.trim().is_empty(),
                ElementKind::Block(block) => block_has_action(block),
                _ => false,
            })
    })
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
    lines.extend(guard_lines(&report.decisions));
    lines.extend(descent_line(&report.decisions));
    lines.extend(decision_rows(&report.decisions));
    lines.extend(k_summary(&report.decisions));
    finish_block(lines)
}

/// The `--profile-decisions` report of one parser unit: the totals, then the
/// slowest decisions.
fn decision_profile_text(report: &StaticParserDfaReport) -> String {
    let decisions = &report.decisions;
    let mut lines = vec![format!(
        "=== parser decision profile: {} ({} decisions) ===",
        report.grammar,
        decisions.len()
    )];
    let elapsed =
        |decision: &StaticParserDecisionReport| decision.result.cost.elapsed.unwrap_or_default();
    let summed: Duration = decisions.iter().map(elapsed).sum();
    if let Some(profile) = &report.profile {
        lines.push(format!(
            "classification: {} on {} {}; {} summed over the decisions",
            duration_text(profile.elapsed),
            profile.threads,
            if profile.threads == 1 {
                "thread"
            } else {
                "threads"
            },
            duration_text(summed),
        ));
    }
    let attempts: usize = decisions
        .iter()
        .map(|decision| decision.result.cost.attempts.len())
        .sum();
    let visits: u64 = decisions
        .iter()
        .map(|decision| decision.result.cost.visits())
        .sum();
    let states: usize = decisions
        .iter()
        .map(|decision| decision.result.cost.states())
        .sum();
    lines.push(format!(
        "work: {attempts} build attempts; {visits} configs visited; {states} DFA states built"
    ));
    let spent = |outcome| {
        decisions
            .iter()
            .filter(|decision| decision.result.cost.count(outcome) > 0)
            .count()
    };
    lines.push(format!(
        "budgets: {} decisions spent the visit budget ({} configs); {} decisions spent the \
         state budget ({} states)",
        spent(AttemptOutcome::VisitBudget),
        report.budget.max_visits,
        spent(AttemptOutcome::StateBudget),
        report.budget.max_states,
    ));
    lines.push(format!(
        "slow: {} decisions took {} ms or more",
        decisions
            .iter()
            .filter(|decision| elapsed(decision) >= SLOW_DECISION)
            .count(),
        SLOW_DECISION.as_millis(),
    ));
    lines.push(
        "attempts: U unbounded, W widened, H hybrid, S static-only hybrid, u and w witness \
         pass, (N) milliseconds, cN: precedence class N"
            .to_owned(),
    );
    lines.push(
        "outcomes: ! stopped at an untrusted conflict, $ spent the visit budget, ^ spent the \
         state budget"
            .to_owned(),
    );

    let mut slowest: Vec<&StaticParserDecisionReport> = decisions.iter().collect();
    slowest.sort_by(|left, right| {
        elapsed(right)
            .cmp(&elapsed(left))
            .then(right.result.cost.visits().cmp(&left.result.cost.visits()))
            .then(left.decision.cmp(&right.decision))
    });
    slowest.truncate(PROFILE_ROWS);
    lines.push(format!("slowest {} decisions:", slowest.len()));
    lines.extend(profile_rows(report, &slowest, elapsed));
    finish_block(lines)
}

/// The rows of the slowest decisions in the `--profile-decisions` report.
fn profile_rows(
    report: &StaticParserDfaReport,
    decisions: &[&StaticParserDecisionReport],
    elapsed: impl Fn(&StaticParserDecisionReport) -> Duration,
) -> Vec<String> {
    let rows: Vec<[String; 9]> = decisions
        .iter()
        .map(|decision| {
            let cost = &decision.result.cost;
            [
                decision.decision.to_string(),
                decision.rule.clone(),
                decision.state.to_string(),
                decision.result.category.as_str().to_owned(),
                format!("{:.1}", elapsed(decision).as_secs_f64() * 1000.0),
                cost.visits().to_string(),
                cost.states().to_string(),
                cost.attempts_text(),
                cost.hot_rules_text(&report.rule_names, HOT_RULES),
            ]
        })
        .collect();
    let headers = [
        "#",
        "rule",
        "state",
        "category",
        "ms",
        "visits",
        "statesBuilt",
        "attempts",
        "most visited rules",
    ];
    let right_aligned = [true, false, true, false, true, true, true, false, false];
    let widths: Vec<usize> = headers
        .iter()
        .enumerate()
        .map(|(column, header)| {
            rows.iter()
                .map(|row| row[column].len())
                .max()
                .unwrap_or(0)
                .max(header.len())
        })
        .collect();
    let line = |cells: Vec<&str>| {
        let mut text = String::from(" ");
        for (column, cell) in cells.iter().enumerate() {
            let width = widths[column];
            text.push(' ');
            if right_aligned[column] {
                text.push_str(&format!("{cell:>width$}"));
            } else if column + 1 < cells.len() {
                text.push_str(&format!("{cell:<width$}"));
            } else {
                text.push_str(cell);
            }
            if column + 1 < cells.len() {
                text.push(' ');
            }
        }
        text.trim_end().to_owned()
    };
    let mut lines = vec![line(headers.to_vec())];
    lines.extend(
        rows.iter()
            .map(|row| line(row.iter().map(String::as_str).collect())),
    );
    lines
}

/// A duration in milliseconds below one second, else in seconds.
fn duration_text(duration: Duration) -> String {
    if duration < Duration::from_secs(1) {
        format!("{:.1} ms", duration.as_secs_f64() * 1000.0)
    } else {
        format!("{:.2} s", duration.as_secs_f64())
    }
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

/// The postfix and guard summary lines: how many decisions have the optional
/// postfix shape, how many take/skip conflicts the take rule and the guards
/// resolved, and how many iterate guards the precedence loops carry.
fn guard_lines(decisions: &[StaticParserDecisionReport]) -> Vec<String> {
    let mut lines = Vec::new();
    let postfix_decisions = decisions
        .iter()
        .filter(|decision| decision.result.has_postfix_shape)
        .count();
    if postfix_decisions > 0 {
        let fires: usize = decisions
            .iter()
            .map(|decision| decision.result.take_rule_fires)
            .sum();
        let guarded: usize = decisions
            .iter()
            .map(|decision| decision.result.guarded_takes)
            .sum();
        lines.push(format!(
            "take-rule: {postfix_decisions} optional-postfix decisions; fires at {fires} conflict states; guarded takes={guarded}"
        ));
    }
    let guarded_iterates: usize = decisions
        .iter()
        .map(|decision| decision.result.guarded_iterates)
        .sum();
    let iterate_accepts: usize = decisions
        .iter()
        .map(|decision| decision.result.iterate_accepts)
        .sum();
    if guarded_iterates > 0 || iterate_accepts > 0 {
        lines.push(format!(
            "iterate-guard: {guarded_iterates} conflict states guarded, {iterate_accepts} resolved outright"
        ));
    }
    lines
}

/// The shared-descent summary: how many decisions accept with a group's virtual
/// alternative, and in how many states.
fn descent_line(decisions: &[StaticParserDecisionReport]) -> Option<String> {
    let with_groups = decisions
        .iter()
        .filter(|decision| decision.result.descent.is_some())
        .count();
    if with_groups == 0 {
        return None;
    }
    let states: usize = decisions
        .iter()
        .filter(|decision| decision.result.descent.is_some())
        .map(|decision| decision.result.descent_group_states)
        .sum();
    Some(format!(
        "descent: {with_groups} decisions with shared-descent groups; {states} group-accept states"
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
        (Some(table), None) => {
            lines.extend(descent_dump_lines(report, result.descent.as_ref()));
            lines.extend(table_dump_lines(report, table, with_paths));
        }
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
    let runtime_states = minimized_states(table);
    let mut lines = vec![format!(
        "  table: states={}{} cyclic={}{}",
        table.states.len(),
        if runtime_states < table.states.len() {
            format!(" (runtime {runtime_states})")
        } else {
            String::new()
        },
        table.cyclic,
        if table.cyclic {
            String::new()
        } else {
            format!(" maxK={}", table.max_k)
        }
    )];
    if table.guarded() > 0 {
        lines.push(format!(
            "  guards: {} guarded states in {} guard groups (pass={} states). A guard walks up the parse stack through the pass frames. It defers at an invoking state of its danger set.",
            table.guarded(),
            table.guard_groups.len(),
            table.guard_pass.len()
        ));
        for (group, danger) in table.guard_groups.iter().enumerate() {
            lines.push(format!("    group {group}: danger={danger:?}"));
        }
    }
    for (index, state) in table.states.iter().enumerate() {
        let mut line = format!("    s{index}: ");
        if state.accept >= VIRTUAL_ALT_BASE {
            line.push_str(&format!("GROUP {}", state.accept));
        } else if state.accept > 0 {
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
            let deferred = if state.resolution == Some(EscapeResolution::Guarded) {
                format!("alt={}", state.guard_defer_alt)
            } else {
                "adaptivePredict".to_owned()
            };
            line.push_str(&format!(
                "GUARDED alt={}, or {deferred} when the parse stack reaches a guard root of group {} (danger={:?})",
                state.guarded_alt,
                state.guard_group,
                table
                    .guard_groups
                    .get(state.guard_group as usize)
                    .map_or(&[][..], Vec::as_slice)
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

/// One line per shared-descent group of a decision.
fn descent_dump_lines(report: &StaticParserDfaReport, plan: Option<&DescentPlan>) -> Vec<String> {
    let Some(plan) = plan else {
        return Vec::new();
    };
    plan.groups
        .iter()
        .map(|group| {
            let mut text = format!(
                "  group {}: shared rule '{}' after {} prefix tokens, members {:?}; tails:",
                group.virtual_alt,
                report
                    .rule_names
                    .get(group.rule)
                    .map_or("?", String::as_str),
                group.prefix.len(),
                group.alts
            );
            for arm in &group.arms {
                let tokens: Vec<String> = arm
                    .tokens
                    .iter()
                    .map(|token| token_name(&report.vocabulary, *token))
                    .collect();
                text.push_str(&format!(" alt {} on {}", arm.alt, tokens.join(" ")));
                text.push(';');
            }
            if group.default_alt > 0 {
                text.push_str(&format!(" otherwise alt {}", group.default_alt));
            } else {
                text.push_str(&format!(" default alt {}", group.min_alt()));
            }
            text
        })
        .collect()
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
    let reason = category_reason(result);
    let escapes =
        matches!(result.category, Category::HYBRID | Category::LR_PRECEDENCE) && result.escapes > 0;
    if result.budget_cutoff && escapes {
        format!("{reason}; {}", budget_cutoff_text(result))
    } else {
        reason
    }
}

fn category_reason(result: &DecisionResult) -> String {
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
            } else if result.guarded_iterates > 0 && result.escapes == 0 {
                format!(
                    "{} iterate/exit conflicts are decided by a runtime stack guard, which defers to adaptivePredict when the parse stack reaches a guard root",
                    result.guarded_iterates
                )
            } else if result.guarded_iterates > 0 {
                format!(
                    "{} of {} precedence-class states escape to adaptivePredict, and {} iterate/exit conflicts are decided by a runtime stack guard",
                    result.escapes, result.num_dfa_states, result.guarded_iterates
                )
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
            result.budget.max_states, result.budget.max_visits
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

/// The text that tells how to raise the budget that cut a hybrid table.
fn budget_cutoff_text(result: &DecisionResult) -> String {
    format!(
        "the analysis budget ({} states or {} visited configs) stopped the lookahead; \
         raise it with the max_dfa_states and max_dfa_visits options of the grammar, \
         the rule, or the block",
        result.budget.max_states, result.budget.max_visits
    )
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
            if resolved.guarded > 0 {
                parts.push(resolution_text(
                    &report.vocabulary,
                    resolved.guarded,
                    ("guarded state returns", "guarded states return"),
                    "the skip or the exit where the stack guard defers",
                    resolved.guarded_witness.as_deref(),
                ));
            }
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
                let mut message = format!(
                    "parser decision {} of rule '{}' resolves its escape states without \
                     adaptivePredict: {}",
                    decision.decision,
                    decision.rule,
                    parts.join(", ")
                );
                if result.budget_cutoff {
                    message.push_str(&format!("; {}", budget_cutoff_text(result)));
                }
                Diagnostic::warning(STATIC_ONLY_RESOLVED, span, message)
            })
        })
        .collect()
}

/// The `G4S193` warnings: one per decision with a build attempt that spent the
/// visit budget. The counts in the message are the same on every machine.
fn visit_budget_diagnostics(
    compiled: &CompiledParser,
    report: &StaticParserDfaReport,
) -> Vec<Diagnostic> {
    report
        .decisions
        .iter()
        .filter_map(|decision| {
            let result = &decision.result;
            let cost = &result.cost;
            let spent = cost.count(AttemptOutcome::VisitBudget);
            if spent == 0 {
                return None;
            }
            let mut message = format!(
                "the analysis of parser decision {} of rule '{}' is slow: {spent} of {} build \
                 attempts spent the visit budget ({} configs); {} configs visited, {} DFA states \
                 built",
                decision.decision,
                decision.rule,
                cost.attempts.len(),
                result.budget.max_visits,
                cost.visits(),
                cost.states(),
            );
            let hot_rules = cost.hot_rules_text(&report.rule_names, HOT_RULES);
            if !hot_rules.is_empty() {
                message.push_str(&format!("; most visited rules: {hot_rules}"));
            }
            let span =
                decision_span(compiled, result).unwrap_or_else(|| report.source_span.clone());
            Some(Diagnostic::warning(VISIT_BUDGET_SPENT, span, message))
        })
        .collect()
}

/// The `G4S194` warnings: one per block with many single-token alternatives
/// that do not fold into one token set.
///
/// Each of these alternatives is a separate ATN branch. The closure of each
/// decision that enters the rule follows all of them. The check looks only at
/// the blocks that the rule can reach before it reads a token, and only at the
/// rules that many call sites enter.
fn token_branch_diagnostics(compiled: &CompiledParser) -> Vec<Diagnostic> {
    let mut candidates = Vec::new();
    for rule in &compiled.semantic.unit.rules {
        if rule.kind != RuleKind::Parser {
            continue;
        }
        let mut blocks = Vec::new();
        leading_blocks(compiled, &rule.block, &mut blocks);
        candidates.extend(
            blocks
                .into_iter()
                .filter_map(|block| TokenBranches::of(rule, block))
                .map(|branches| (rule, branches)),
        );
    }
    if candidates.is_empty() {
        return Vec::new();
    }
    let entries = rule_entries(compiled);
    candidates
        .into_iter()
        .filter_map(|(rule, branches)| {
            let entries = compiled
                .semantic
                .recognizer
                .rule_numbers
                .get(&rule.id)
                .and_then(|index| entries.get(*index))
                .copied()?;
            (entries >= TOKEN_BRANCH_ENTRIES).then(|| branches.diagnostic(rule, entries))
        })
        .collect()
}

/// Adds `block` and, in source order, each nested block that the rule can
/// reach from the start of `block` before it reads a token. Returns whether
/// `block` can match the empty string.
fn leading_blocks<'b>(
    compiled: &CompiledParser,
    block: &'b Block,
    out: &mut Vec<&'b Block>,
) -> bool {
    out.push(block);
    let mut nullable = false;
    for alternative in &block.alternatives {
        let mut empty = true;
        for element in &alternative.elements {
            let element_nullable = match &element.kind {
                ElementKind::Block(nested) => leading_blocks(compiled, nested, out),
                ElementKind::Action { .. }
                | ElementKind::Predicate { .. }
                | ElementKind::Epsilon => true,
                ElementKind::RuleCall(_) => compiled
                    .semantic
                    .bindings
                    .rule_calls
                    .get(&element.id)
                    .is_some_and(|call| compiled.analysis.nullable_rules.contains(&call.target)),
                _ => false,
            };
            if !element_nullable
                && !matches!(
                    element.quantifier,
                    Quantifier::Optional { .. } | Quantifier::ZeroOrMore { .. }
                )
            {
                empty = false;
                break;
            }
        }
        nullable |= empty;
    }
    nullable
}

/// The single-token alternatives of one block that did not fold into one
/// token set, with the causes.
struct TokenBranches<'b> {
    block: &'b Block,
    count: usize,
    causes: Vec<String>,
}

impl<'b> TokenBranches<'b> {
    /// The single-token alternatives of `block`, when there are enough of them
    /// for a `G4S194` warning and a cause is known.
    fn of(rule: &Rule, block: &'b Block) -> Option<Self> {
        // The left-recursion rewrite moves the operator alternatives out of
        // the primary block, and puts an empty action in front of the first
        // primary alternative.
        let operators = rule
            .left_recursion
            .as_ref()
            .and_then(|_| left_recursion_operators(rule, block))
            .unwrap_or(0);
        let single_tokens = block
            .alternatives
            .iter()
            .enumerate()
            .filter_map(|(index, alternative)| {
                let mut elements = alternative.elements.iter();
                if index == 0
                    && operators > 0
                    && let [first, _, ..] = alternative.elements.as_slice()
                    && matches!(&first.kind, ElementKind::Action { body, .. } if body.is_empty())
                {
                    elements.next();
                }
                match (elements.next(), elements.next()) {
                    (Some(element), None)
                        if element.quantifier == Quantifier::One
                            && matches!(
                                element.kind,
                                ElementKind::Terminal(Terminal::Token(_) | Terminal::Literal(_))
                            ) =>
                    {
                        Some((alternative, element))
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();
        if single_tokens.len() < TOKEN_BRANCH_ALTERNATIVES {
            return None;
        }
        let mut causes = Vec::new();
        let others = block.alternatives.len() - single_tokens.len() + operators;
        if others > 0 {
            causes.push(count_text(
                others,
                "alternative is not a single token",
                "alternatives are not a single token",
            ));
        }
        let labeled = single_tokens
            .iter()
            .filter(|(alternative, _)| alternative.label.is_some())
            .count();
        if labeled > 0 {
            causes.push(count_text(
                labeled,
                "alternative has a label",
                "alternatives have a label",
            ));
        }
        let element_labeled = single_tokens
            .iter()
            .filter(|(_, element)| element.label.is_some())
            .count();
        if element_labeled > 0 {
            causes.push(count_text(
                element_labeled,
                "token has an element label",
                "tokens have an element label",
            ));
        }
        if has_non_budget_option(&block.options)
            || single_tokens
                .iter()
                .any(|(alternative, _)| !alternative.options.is_empty())
        {
            causes.push("the block or its alternatives have options".to_owned());
        }
        // Without a cause, a transform after the block-set reduction made the
        // other alternatives single tokens.
        (!causes.is_empty()).then_some(Self {
            block,
            count: single_tokens.len(),
            causes,
        })
    }

    fn diagnostic(&self, rule: &Rule, entries: usize) -> Diagnostic {
        Diagnostic::warning(
            TOKEN_BRANCHES,
            self.block.span.clone(),
            format!(
                "rule '{}' has a block with {} single-token alternatives that do not fold into \
                 one token set. The block does not fold because {}. {entries} call sites enter \
                 the rule before they read a token. The analysis of the static parser DFA \
                 follows each alternative at each site. Move the single tokens into one \
                 unlabeled subrule, for example `( A | B | C )`, or into a token-only rule",
                rule.name,
                self.count,
                self.causes.join(", and "),
            ),
        )
    }
}

/// The number of operator alternatives of a left-recursive rule, when `block`
/// is the primary block of its rewrite.
fn left_recursion_operators(rule: &Rule, block: &Block) -> Option<usize> {
    let [alternative] = rule.block.alternatives.as_slice() else {
        return None;
    };
    let [primary, operators] = alternative.elements.as_slice() else {
        return None;
    };
    match (&primary.kind, &operators.kind) {
        (ElementKind::Block(primary), ElementKind::Block(operators))
            if std::ptr::eq(primary, block) =>
        {
            Some(operators.alternatives.len())
        }
        _ => None,
    }
}

/// The number of call sites that enter each rule before they read a token,
/// by rule index.
///
/// A call site of rule `R` enters `R`. When `R` can call rule `S` before it
/// reads a token, the call site also enters `S`.
fn rule_entries(compiled: &CompiledParser) -> Vec<usize> {
    let graph = &compiled.graph;
    let rules = graph.rule_starts.len();
    let mut direct = vec![0_usize; rules];
    for transition in &graph.transitions {
        if let FinalizedTransitionKind::Rule { rule_index, .. } = transition.kind
            && let Some(count) = direct.get_mut(rule_index)
        {
            *count += 1;
        }
    }
    let recognizer = &compiled.semantic.recognizer;
    let nullable = compiled
        .analysis
        .nullable_rules
        .iter()
        .filter_map(|rule| recognizer.rule_numbers.get(rule).copied())
        .collect();
    let mut callers = vec![Vec::new(); rules];
    for (caller, callees) in left_corner_rules(graph, &nullable).iter().enumerate() {
        for &callee in callees {
            if let Some(callers) = callers.get_mut(callee) {
                callers.push(caller);
            }
        }
    }
    (0..rules)
        .map(|rule| {
            let mut seen = vec![false; rules];
            seen[rule] = true;
            let mut pending = vec![rule];
            let mut entries = 0;
            while let Some(current) = pending.pop() {
                entries += direct[current];
                for &caller in &callers[current] {
                    if !seen[caller] {
                        seen[caller] = true;
                        pending.push(caller);
                    }
                }
            }
            entries
        })
        .collect()
}

/// `count` with the singular or the plural phrase, for example `1 alternative
/// has a label`.
fn count_text(count: usize, singular: &str, plural: &str) -> String {
    format!("{count} {}", if count == 1 { singular } else { plural })
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

/// The budget of a decision that no rule or block budget changes,
/// and the budget of each decision, by decision number.
fn decision_budgets(
    compiled: &CompiledParser,
    budgets: &ParserDfaBudgets,
) -> (DecisionBudget, Vec<DecisionBudget>) {
    let atn = compiled.atn;
    let unit = &compiled.semantic.unit;
    let grammar = BudgetLimits::from_options(&unit.options).or(budgets.global);
    let decisions = atn
        .decision_to_state
        .iter()
        .map(|state| {
            let rule_index = usize::try_from(state.get_rule_index()).unwrap_or(usize::MAX);
            let rule = unit.rules.get(rule_index);
            let block = rule
                .zip(usize::try_from(state.get_state_number()).ok())
                .map(|(rule, state)| block_limits(compiled, state, rule))
                .unwrap_or_default();
            let rule = rule
                .map(|rule| BudgetLimits::from_options(&rule.options))
                .unwrap_or_default();
            let limits = block.or(rule).or(grammar);
            DecisionBudget::resolve(limits, atn.max_token_type)
        })
        .collect();
    (
        DecisionBudget::resolve(grammar, atn.max_token_type),
        decisions,
    )
}

/// The limits that the options of the blocks around the decision at ATN state
/// `state` of rule `rule` set. An inner block wins over an outer block.
fn block_limits(compiled: &CompiledParser, state: usize, rule: &Rule) -> BudgetLimits {
    let Some(state) = compiled.graph.states.get(state) else {
        return BudgetLimits::default();
    };
    // The immediate owner of a state is its last synthetic origin.
    let owner = compiled
        .provenance
        .state_origins(state.original)
        .iter()
        .rev()
        .find_map(|origin| match origin {
            Origin::Synthetic {
                owner: ModelNodeId::Element(element),
                ..
            } => Some(*element),
            _ => None,
        });
    let mut blocks = Vec::new();
    if owner.is_none_or(|owner| !enclosing_blocks(&rule.block, owner, &mut blocks)) {
        return BudgetLimits::default();
    }
    blocks
        .iter()
        .rev()
        .fold(BudgetLimits::default(), |limits, block| {
            limits.or(BudgetLimits::from_options(&block.options))
        })
}

/// Pushes the blocks from `block` down to the element `target` onto `path`,
/// the block of `target` included. Returns `false` when `block` does not
/// contain `target`.
fn enclosing_blocks<'a>(
    block: &'a Block,
    target: crate::grammar::model::ElementId,
    path: &mut Vec<&'a Block>,
) -> bool {
    for element in block
        .alternatives
        .iter()
        .flat_map(|alternative| &alternative.elements)
    {
        let ElementKind::Block(inner) = &element.kind else {
            if element.id == target {
                return true;
            }
            continue;
        };
        path.push(inner);
        if element.id == target || enclosing_blocks(inner, target, path) {
            return true;
        }
        path.pop();
    }
    false
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
