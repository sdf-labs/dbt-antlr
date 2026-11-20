// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The text report of `--explain`.
//!
//! The report is plain text with a fixed layout: it has no terminal width
//! dependent wrapping and no file system paths, so snapshot tests of it are
//! the same on every platform.

use std::collections::BTreeMap;
use std::fmt::Write as _;

use dbt_antlr_runtime::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF};

use super::diff::Verdict;
use super::interp::{
    DecisionEvent, ErrorKind, Frame, Outcome, Resolution, SyntaxError, TableKind, TableWalk, Trace,
};
use super::lex::{LexError, Lexed};
use crate::optimization::StaticParserDfaReport;
use crate::optimization::parser_dfa::BudgetCutoff;

/// Maximum number of "expected tokens" to show in a syntax error.
const EXPECTED_TOKENS: usize = 12;
/// Maximum number of lookahead tokens of a table walk to show in the report.
const LOOKAHEAD_TOKENS: usize = 8;
/// Maximum number of static guesses listed in the report.
const LISTED_GUESSES: usize = 10;

pub(super) struct Explanation<'a> {
    pub(super) start_rule: &'a str,
    pub(super) mode: &'static str,
    pub(super) input_name: &'a str,
    pub(super) input: &'a str,
    pub(super) lexed: &'a Lexed,
    pub(super) report: &'a StaticParserDfaReport,
    pub(super) static_trace: &'a Trace,
    pub(super) ll_trace: &'a Trace,
    pub(super) verdict: Verdict,
    /// The SLL prediction at the divergent decision.
    pub(super) probe: Option<Option<i32>>,
    pub(super) verbose: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum BatchCategory {
    Agree,
    StaticDfaDivergence { rule: String, decision: usize },
    SilentDivergence { rule: String, decision: usize },
    StaticDfaSyntaxError { target: Option<(String, usize)> },
    GrammarRejects,
    LexerError,
    Inconclusive,
}

pub(super) struct BatchEntry {
    pub(super) report: String,
    pub(super) collapsed: bool,
    pub(super) category: BatchCategory,
}

/// Renders the report of a lexer failure: the parsers do not run.
pub(super) fn render_lexer_errors(
    grammar: &str,
    input_name: &str,
    input: &str,
    errors: &[LexError],
) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "=== explain: {grammar} ===");
    let _ = writeln!(out, "input: {input_name}");
    let _ = writeln!(out);
    render_lexer_errors_body(&mut out, input, errors);
    out
}

pub(super) fn render_lexer_errors_in_batch(
    index: usize,
    total: usize,
    input_name: &str,
    input: &str,
    errors: &[LexError],
) -> BatchEntry {
    let mut out = String::new();
    let _ = writeln!(out, "--- [{index}/{total}] {input_name} ---");
    let _ = writeln!(out);
    render_lexer_errors_body(&mut out, input, errors);
    BatchEntry {
        report: out,
        collapsed: false,
        category: BatchCategory::LexerError,
    }
}

fn render_lexer_errors_body(out: &mut String, input: &str, errors: &[LexError]) {
    let _ = writeln!(out, "[VERDICT: LEXER ERROR]");
    let _ = writeln!(
        out,
        "  The lexer rejects the input, so no parser decision ran. Fix the input or the lexer grammar first."
    );
    for error in errors {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "[LEXER ERROR @ line {}:{}] {}",
            error.line,
            error.column + 1,
            error.message
        );
        source_excerpt(out, input, error.line, error.column, 1);
    }
}

pub(super) fn render_batch(grammar: &str, mode: &str, entries: &[BatchEntry]) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "=== explain batch: {grammar} ({} inputs, mode: {mode}) ===",
        entries.len()
    );
    let mut prev_collapsed = false;
    for (index, entry) in entries.iter().enumerate() {
        if index == 0 || !entry.collapsed || !prev_collapsed {
            let _ = writeln!(out);
        }
        out.push_str(&entry.report);
        prev_collapsed = entry.collapsed;
    }
    let _ = writeln!(out);
    render_batch_summary(&mut out, grammar, mode, entries);
    out
}

fn render_batch_summary(out: &mut String, grammar: &str, mode: &str, entries: &[BatchEntry]) {
    let mut agree = 0_usize;
    let mut static_div: BTreeMap<(usize, String), usize> = BTreeMap::new();
    let mut static_div_total = 0_usize;
    let mut silent_div: BTreeMap<(usize, String), usize> = BTreeMap::new();
    let mut silent_div_total = 0_usize;
    let mut static_err: BTreeMap<(usize, String), usize> = BTreeMap::new();
    let mut static_err_total = 0_usize;
    let mut grammar_rejects = 0_usize;
    let mut lexer_errors = 0_usize;
    let mut inconclusive = 0_usize;

    for entry in entries {
        match &entry.category {
            BatchCategory::Agree => agree += 1,
            BatchCategory::StaticDfaDivergence { rule, decision } => {
                static_div_total += 1;
                *static_div.entry((*decision, rule.clone())).or_default() += 1;
            }
            BatchCategory::SilentDivergence { rule, decision } => {
                silent_div_total += 1;
                *silent_div.entry((*decision, rule.clone())).or_default() += 1;
            }
            BatchCategory::StaticDfaSyntaxError { target } => {
                static_err_total += 1;
                if let Some((rule, decision)) = target {
                    *static_err.entry((*decision, rule.clone())).or_default() += 1;
                }
            }
            BatchCategory::GrammarRejects => grammar_rejects += 1,
            BatchCategory::LexerError => lexer_errors += 1,
            BatchCategory::Inconclusive => inconclusive += 1,
        }
    }

    let rows = [
        ("agree", agree, None),
        (
            "static-dfa divergence",
            static_div_total,
            format_decision_tally(&static_div),
        ),
        (
            "silent divergence",
            silent_div_total,
            format_decision_tally(&silent_div),
        ),
        (
            "static-dfa syntax error",
            static_err_total,
            format_decision_tally(&static_err),
        ),
        ("grammar rejects", grammar_rejects, None),
        ("lexer error", lexer_errors, None),
        ("inconclusive", inconclusive, None),
    ];
    let shown: Vec<_> = rows
        .into_iter()
        .filter(|(_, count, _)| *count > 0)
        .collect();
    let width = shown
        .iter()
        .map(|(label, _, _)| label.len() + 1)
        .max()
        .unwrap_or(0);
    let _ = writeln!(
        out,
        "=== batch summary: {grammar} ({} inputs, mode: {mode}) ===",
        entries.len()
    );
    for (label, count, detail) in shown {
        match detail {
            Some(detail) => {
                let _ = writeln!(out, "  {:<width$} {count}  ({detail})", format!("{label}:"));
            }
            None => {
                let _ = writeln!(out, "  {:<width$} {count}", format!("{label}:"));
            }
        }
    }
}

fn format_decision_tally(counts: &BTreeMap<(usize, String), usize>) -> Option<String> {
    if counts.is_empty() {
        return None;
    }
    let mut items: Vec<_> = counts.iter().collect();
    items.sort_by(
        |((left_dec, _), left_count), ((right_dec, _), right_count)| {
            right_count
                .cmp(left_count)
                .then_with(|| left_dec.cmp(right_dec))
        },
    );
    Some(
        items
            .into_iter()
            .map(|((decision, rule), count)| format!("{rule}:{decision} x{count}"))
            .collect::<Vec<_>>()
            .join(", "),
    )
}

impl Explanation<'_> {
    pub(super) fn render(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(
            out,
            "=== explain: {} (start rule: {}, mode: {}) ===",
            self.report.grammar, self.start_rule, self.mode
        );
        let _ = writeln!(
            out,
            "input: {} ({} tokens)",
            self.input_name,
            self.default_channel_tokens()
        );
        let _ = writeln!(out);
        self.render_body(&mut out);
        out
    }

    pub(super) fn render_in_batch(&self, index: usize, total: usize) -> BatchEntry {
        let category = self.batch_category();
        if !self.verbose && self.verdict == Verdict::Agree {
            return BatchEntry {
                report: format!(
                    "[{index}/{total}] {} (start rule: {}, {} tokens): AGREE\n",
                    self.input_name,
                    self.start_rule,
                    self.default_channel_tokens()
                ),
                collapsed: true,
                category,
            };
        }
        let mut out = String::new();
        let _ = writeln!(
            out,
            "--- [{index}/{total}] {} (start rule: {}, {} tokens) ---",
            self.input_name,
            self.start_rule,
            self.default_channel_tokens()
        );
        let _ = writeln!(out);
        self.render_body(&mut out);
        BatchEntry {
            report: out,
            collapsed: false,
            category,
        }
    }

    fn batch_category(&self) -> BatchCategory {
        let static_failed = matches!(self.static_trace.outcome, Outcome::Failed(_));
        let ll_failed = matches!(self.ll_trace.outcome, Outcome::Failed(_));
        match self.verdict {
            Verdict::Agree => BatchCategory::Agree,
            Verdict::Divergence { step } => {
                let event = &self.static_trace.events[step];
                let rule = self.rule_name(event.rule).to_owned();
                let decision = event.decision;
                if !static_failed && !ll_failed {
                    BatchCategory::SilentDivergence { rule, decision }
                } else {
                    BatchCategory::StaticDfaDivergence { rule, decision }
                }
            }
            Verdict::StaticFailure { step } => {
                let target = step.map(|step| {
                    let event = &self.static_trace.events[step];
                    (self.rule_name(event.rule).to_owned(), event.decision)
                });
                BatchCategory::StaticDfaSyntaxError { target }
            }
            Verdict::GrammarRejects => BatchCategory::GrammarRejects,
            Verdict::Inconclusive { .. } => BatchCategory::Inconclusive,
        }
    }

    fn render_body(&self, out: &mut String) {
        self.verdict_section(out);
        match self.verdict {
            Verdict::Divergence { step } => self.divergence_section(out, step),
            Verdict::StaticFailure { step: Some(step) } => {
                self.static_failure_section(out, step);
            }
            Verdict::Inconclusive { step } => self.inconclusive_section(out, step),
            Verdict::Agree | Verdict::GrammarRejects | Verdict::StaticFailure { step: None } => {}
        }
        let show_error = self.verbose
            || matches!(
                self.verdict,
                Verdict::GrammarRejects
                    | Verdict::StaticFailure { step: None }
                    | Verdict::Inconclusive { .. }
            );
        if show_error {
            if let Outcome::Failed(error) = &self.static_trace.outcome {
                let _ = writeln!(out);
                self.error_section(out, self.mode, error);
            } else if let Outcome::Failed(error) = &self.ll_trace.outcome {
                let _ = writeln!(out);
                self.error_section(out, "full LL(*)", error);
            }
        }
        if self.verbose {
            self.guesses_section(out);
        }
    }

    fn verdict_section(&self, out: &mut String) {
        let static_outcome =
            self.outcome_text(&self.static_trace.outcome, &self.static_trace.events);
        let ll_outcome = self.outcome_text(&self.ll_trace.outcome, &self.ll_trace.events);
        let static_failed = matches!(self.static_trace.outcome, Outcome::Failed(_));
        let ll_failed = matches!(self.ll_trace.outcome, Outcome::Failed(_));
        let (title, summary) = match self.verdict {
            Verdict::Agree => (
                "AGREE",
                "The static parser makes the same choices as full LL(*) on this input.".to_owned(),
            ),
            Verdict::Divergence { .. } if static_failed && !ll_failed => (
                "STATIC-DFA DIVERGENCE",
                format!(
                    "Full LL(*) accepts this input. The syntax error is a {} artifact: an earlier static choice went wrong.",
                    self.mode
                ),
            ),
            Verdict::Divergence { .. } if !static_failed && !ll_failed => (
                "SILENT DIVERGENCE",
                "Both parsers accept this input, but the static parser builds a different parse tree from full LL(*).".to_owned(),
            ),
            Verdict::Divergence { .. } => (
                "STATIC-DFA DIVERGENCE",
                "Both parsers reject this input, but the static parser chooses a different alternative from full LL(*), so they report different errors.".to_owned(),
            ),
            Verdict::StaticFailure { .. } if !ll_failed => (
                "STATIC-DFA SYNTAX ERROR",
                format!(
                    "Full LL(*) accepts this input, but the {} parser fails without an earlier divergent choice.",
                    self.mode
                ),
            ),
            Verdict::StaticFailure { .. } => (
                "STATIC-DFA SYNTAX ERROR",
                format!(
                    "Both parsers reject this input. The {} parser fails without an earlier divergent choice, but full LL(*) rejects the input later, so they report different errors.",
                    self.mode
                ),
            ),
            Verdict::GrammarRejects if self.same_failure() => (
                "GRAMMAR REJECTS INPUT",
                format!(
                    "Full LL(*) also rejects this input at the same place. The grammar itself does not accept it: this is not a {} artifact.",
                    self.mode
                ),
            ),
            Verdict::GrammarRejects => (
                "GRAMMAR REJECTS INPUT",
                "Full LL(*) also rejects this input, so the grammar itself does not accept it. The two parsers report the error at different places.".to_owned(),
            ),
            Verdict::Inconclusive { .. } => (
                "INCONCLUSIVE",
                "The two parses visit different decisions without a divergent choice. Please report this as an --explain bug.".to_owned(),
            ),
        };
        let _ = writeln!(out, "[VERDICT: {title}]");
        let _ = writeln!(out, "  {summary}");
        let width = self.mode.len().max("full LL(*)".len()) + 1;
        let _ = writeln!(
            out,
            "  {:<width$} {static_outcome}",
            format!("{}:", self.mode)
        );
        let _ = writeln!(out, "  {:<width$} {ll_outcome}", "full LL(*):");
    }

    /// Whether both parsers fail at the same token.
    fn same_failure(&self) -> bool {
        match (&self.static_trace.outcome, &self.ll_trace.outcome) {
            (Outcome::Failed(ours), Outcome::Failed(oracle)) => {
                ours.token_index == oracle.token_index
            }
            _ => false,
        }
    }

    fn outcome_text(&self, outcome: &Outcome, events: &[DecisionEvent]) -> String {
        match outcome {
            Outcome::Accepted { stop } => {
                let stop_text = if self.token_type(*stop) == TOKEN_EOF {
                    String::new()
                } else {
                    format!(", stops before token {}", self.token_text(*stop))
                };
                let decisions = match events.len() {
                    1 => "1 decision".to_owned(),
                    count => format!("{count} decisions"),
                };
                format!("accepted ({decisions}{stop_text})")
            }
            Outcome::Failed(error) => {
                format!(
                    "syntax error at token {}",
                    self.token_text(error.token_index)
                )
            }
        }
    }

    fn divergence_section(&self, out: &mut String, step: usize) {
        let ours = &self.static_trace.events[step];
        let oracle = &self.ll_trace.events[step];
        let (Some(our_alt), Some(oracle_alt)) = (ours.alt, oracle.alt) else {
            return;
        };
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "[FIRST DIVERGENCE @ step {step}, token {}]",
            self.token_text(ours.token_index)
        );
        self.decision_headline(out, ours);
        let _ = writeln!(
            out,
            "    {} chose {}",
            self.mode,
            self.alt_name(ours.decision, our_alt)
        );
        let _ = writeln!(out, "      via {}", self.resolution_text(ours));
        if let Some(walk) = ours.walk()
            && !walk.dropped.is_empty()
        {
            let _ = writeln!(
                out,
                "      dropped: {}",
                self.alt_list(ours.decision, &walk.dropped)
            );
        }
        let _ = writeln!(
            out,
            "    full LL(*) chose {}",
            self.alt_name(ours.decision, oracle_alt)
        );
        let diagnosis = self.diagnose_divergence(ours, our_alt, oracle_alt);
        if let Some(diagnosis) = &diagnosis {
            let _ = writeln!(out, "  root cause: {}", diagnosis.cause);
        }
        if let Outcome::Failed(error) = &self.static_trace.outcome {
            let distance = self.distance(ours.token_index, error.token_index);
            let later = match distance {
                0 => "at the same token".to_owned(),
                1 => "1 token later".to_owned(),
                n => format!("{n} tokens later"),
            };
            let _ = writeln!(
                out,
                "  impact: causes the syntax error {later}, at token {}",
                self.token_text(error.token_index)
            );
        }
        let mut remediations = diagnosis.map_or_else(Vec::new, |diagnosis| diagnosis.remediations);
        if let Some(hint) = self.dump_hint(ours.decision) {
            remediations.push(hint);
        }
        self.remediation_section(out, &remediations);
    }

    fn static_failure_section(&self, out: &mut String, step: usize) {
        let ours = &self.static_trace.events[step];
        let oracle = &self.ll_trace.events[step];
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "[FAILED STATIC PREDICTION @ step {step}, token {}]",
            self.token_text(ours.token_index)
        );
        self.decision_headline(out, ours);
        let _ = writeln!(
            out,
            "    {} has no alternative here ({})",
            self.mode,
            self.resolution_text(ours)
        );
        if let Some(alt) = oracle.alt {
            let _ = writeln!(
                out,
                "    full LL(*) chose {}",
                self.alt_name(ours.decision, alt)
            );
        }
        let mut remediations = self.static_failure_remediations(ours);
        if let Some(hint) = self.dump_hint(ours.decision) {
            remediations.push(hint);
        }
        self.remediation_section(out, &remediations);
    }

    fn inconclusive_section(&self, out: &mut String, step: usize) {
        let _ = writeln!(out);
        let _ = writeln!(out, "[TRACES SPLIT @ step {step}]");
        for (label, trace) in [
            (self.mode, self.static_trace),
            ("full LL(*)", self.ll_trace),
        ] {
            match trace.events.get(step) {
                Some(event) => {
                    let _ = writeln!(
                        out,
                        "  {label}: decision {} in rule '{}' at token {}",
                        event.decision,
                        self.rule_name(event.rule),
                        self.token_text(event.token_index)
                    );
                }
                None => {
                    let _ = writeln!(out, "  {label}: no more decisions");
                }
            }
        }
    }

    fn remediation_section(&self, out: &mut String, remediations: &[String]) {
        if remediations.is_empty() {
            return;
        }
        let _ = writeln!(out);
        let _ = writeln!(out, "[SUGGESTED REMEDIATION]");
        for item in remediations {
            let _ = writeln!(out, "  - {item}");
        }
    }

    fn dump_hint(&self, decision: usize) -> Option<String> {
        let has_table = self
            .report
            .decisions
            .get(decision)
            .is_some_and(|decision| decision.result.has_table());
        has_table.then(|| format!("Inspect the table with `--dump-dfas {decision}`."))
    }

    fn decision_headline(&self, out: &mut String, event: &DecisionEvent) {
        let kind = self
            .report
            .decisions
            .get(event.decision)
            .map_or_else(String::new, |decision| {
                format!(" ({})", decision.result.kind)
            });
        let _ = writeln!(
            out,
            "  rule '{}', decision {}{kind}:",
            self.rule_name(event.rule),
            event.decision
        );
    }

    fn diagnose_divergence(
        &self,
        ours: &DecisionEvent,
        our_alt: i32,
        oracle_alt: i32,
    ) -> Option<Diagnosis> {
        let rule = self.rule_name(ours.rule);
        if matches!(ours.resolution, Resolution::DescentFallback { .. }) {
            return Some(Diagnosis {
                cause: "shared-descent fallback. The group did not start its neutral parse and took its minimum member alternative.".to_owned(),
                remediations: vec![format!(
                    "Left-factor the shared prefix in rule '{rule}' so this decision does not rely on a shared-descent neutral parse."
                )],
            });
        }
        if let Some(walk) = ours.walk()
            && matches!(walk.kind, TableKind::GuardTaken | TableKind::GuardDeferred)
        {
            let caller = (walk.kind == TableKind::GuardDeferred)
                .then(|| self.active_danger_caller(ours, walk))
                .flatten();
            let suggestion = match caller {
                Some(caller) => format!(
                    "Refactor caller {caller} or rule '{rule}' so the stack guard does not defer, or raise the lookahead budget of rule '{rule}'."
                ),
                None => format!(
                    "Refactor rule '{rule}' or its callers so the decision does not rely on a stack guard, or raise the lookahead budget of rule '{rule}'."
                ),
            };
            return Some(Diagnosis {
                cause: format!(
                    "runtime stack guard. The guard sees only the parse stack, not the tokens after the lookahead of rule '{rule}'."
                ),
                remediations: vec![suggestion],
            });
        }
        let sll = self.probe?;
        let walk_kind = ours.walk().map(|walk| walk.kind);
        Some(match sll {
            Some(sll) if sll == oracle_alt && walk_kind == Some(TableKind::MinAlt) => Diagnosis {
                cause: format!(
                    "static SLL conflict. Unbounded SLL(*) also chooses alt {oracle_alt}, so this is not a lookahead budget limit: static table construction hit an SLL conflict under static closure approximation."
                ),
                remediations: vec![
                    "Raising max_dfa_states, max_dfa_visits, or max_k will not help here."
                        .to_owned(),
                    format!(
                        "Left-factor or restructure the conflicting alternatives in rule '{rule}'."
                    ),
                ],
            },
            Some(sll) if sll == oracle_alt => {
                let budget_options = match self
                    .report
                    .decisions
                    .get(ours.decision)
                    .and_then(|decision| decision.result.budget_cutoff)
                {
                    Some(BudgetCutoff::States) => "max_dfa_states",
                    Some(BudgetCutoff::Visits) => "max_dfa_visits",
                    Some(BudgetCutoff::Both) => "max_dfa_states and max_dfa_visits",
                    None => "max_dfa_states, max_dfa_visits, or max_k",
                };
                Diagnosis {
                    cause: format!(
                        "lookahead budget. Unbounded SLL(*) also chooses alt {oracle_alt}, so a longer static lookahead could resolve this input."
                    ),
                    remediations: vec![format!(
                        "Raise {budget_options} on rule '{rule}' or its block, or left-factor the conflicting alternatives in rule '{rule}'"
                    )],
                }
            }
            Some(sll) if sll == our_alt => Diagnosis {
                cause: format!(
                    "context sensitivity. SLL(*) also chooses alt {our_alt}: only full-context LL(*) sees that the tokens after rule '{rule}' select alt {oracle_alt}."
                ),
                remediations: vec![
                    "A larger lookahead budget will not help.".to_owned(),
                    format!(
                        "Change the grammar so that the caller decides, for example inline rule '{rule}' at this call site."
                    ),
                ],
            },
            Some(sll) => Diagnosis {
                cause: format!("SLL(*) chooses alt {sll} here."),
                remediations: Vec::new(),
            },
            None => Diagnosis {
                cause: "SLL(*) prediction fails at this decision.".to_owned(),
                remediations: Vec::new(),
            },
        })
    }

    fn static_failure_remediations(&self, ours: &DecisionEvent) -> Vec<String> {
        let rule = self.rule_name(ours.rule);
        match &ours.resolution {
            Resolution::NoStaticTable => vec![format!(
                "Make decision {} of rule '{rule}' statically decidable (left-factor, remove predicates, or raise its lookahead budget).",
                ours.decision
            )],
            _ if ours
                .walk()
                .is_some_and(|walk| walk.kind == TableKind::SyntaxError) =>
            {
                vec![format!(
                    "Move or remove the semantic predicate in rule '{rule}' so static lookahead decides without evaluating it."
                )]
            }
            _ => Vec::new(),
        }
    }

    fn active_danger_caller(&self, event: &DecisionEvent, walk: &TableWalk) -> Option<String> {
        let decision = self.report.decisions.get(event.decision)?;
        let table = match walk.class {
            None => decision.result.table.as_ref()?,
            Some(class) => decision
                .result
                .precedence
                .as_ref()?
                .tables
                .get(class)?
                .as_ref()?,
        };
        let state = table.states.get(walk.state)?;
        let danger = table
            .guard_groups
            .get(usize::try_from(state.guard_group).ok()?)?;
        for frame in event.stack.iter().rev() {
            if frame.invoking_state < 0 {
                break;
            }
            if danger.contains(&frame.invoking_state) {
                return self
                    .report
                    .invoking_states
                    .get(&frame.invoking_state)
                    .map(|caller| caller.called_from_text());
            }
            if !table.guard_pass.contains(&frame.invoking_state) {
                break;
            }
        }
        None
    }

    fn resolution_text(&self, event: &DecisionEvent) -> String {
        match &event.resolution {
            Resolution::Pending => "a failed prediction".to_owned(),
            Resolution::Ll1 { matched: true } => "the LL(1) token switch".to_owned(),
            Resolution::Ll1 { matched: false } => {
                "the LL(1) token switch (no alternative starts with this token)".to_owned()
            }
            Resolution::Table(walk) => self.walk_text(walk),
            Resolution::Descent {
                group, tail, walk, ..
            } => {
                let table = walk.as_ref().map_or_else(String::new, |walk| {
                    format!("{}, then ", self.walk_text(walk))
                });
                format!(
                    "{table}shared-descent group {group}, dispatched on token {}",
                    self.token_text(*tail)
                )
            }
            Resolution::DescentFallback { group } => {
                format!("shared-descent group {group} fallback")
            }
            Resolution::DescentAborted { group, walk, .. } => {
                let table = walk.as_ref().map_or_else(String::new, |walk| {
                    format!("{}, then ", self.walk_text(walk))
                });
                format!(
                    "{table}shared-descent group {group}, whose neutral parse of the shared rule failed"
                )
            }
            Resolution::NoStaticTable => {
                "no static table: --static-dfa-only rejects this decision (G4S190)".to_owned()
            }
            Resolution::Adaptive => "adaptive_predict".to_owned(),
            Resolution::FullLl => "full LL(*)".to_owned(),
        }
    }

    fn walk_text(&self, walk: &TableWalk) -> String {
        let kind = match walk.kind {
            TableKind::Exact => "an exact table accept",
            TableKind::Fallback => "the table fallback",
            TableKind::MinAlt => "MinAlt (minimum conflicting alternative)",
            TableKind::MinViableAlt => "MinViableAlt (lookahead budget cutoff)",
            TableKind::GuardTaken => "a stack guard that takes its alternative",
            TableKind::GuardDeferred => "a stack guard that defers",
            TableKind::SyntaxError => "a predicated dead end",
            TableKind::Escape => "an escape to adaptive_predict",
            TableKind::Unknown => "the table",
        };
        let class = walk
            .class
            .map_or_else(String::new, |class| format!(" (precedence class {class})"));
        let lookahead = if walk.lookahead.is_empty() {
            String::new()
        } else {
            let quoted = |indices: &[isize]| {
                indices
                    .iter()
                    .map(|index| self.token_quoted(*index))
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            let count = walk.lookahead.len();
            if count > LOOKAHEAD_TOKENS {
                let head = quoted(&walk.lookahead[..LOOKAHEAD_TOKENS / 2]);
                let tail = quoted(&walk.lookahead[count - LOOKAHEAD_TOKENS / 2..]);
                format!(", lookahead {head} ... {tail} ({count} tokens)")
            } else {
                format!(", lookahead {}", quoted(&walk.lookahead))
            }
        };
        format!("{kind} at s{}{class}{lookahead}", walk.state)
    }

    fn error_section(&self, out: &mut String, label: &str, error: &SyntaxError) {
        let kind = match error.kind {
            ErrorKind::NoViableAlt => "no viable alternative",
            ErrorKind::Mismatch => "mismatched input",
            ErrorKind::FailedPredicate => "failed predicate",
            ErrorKind::Other => "error",
        };
        let _ = writeln!(
            out,
            "[SYNTAX ERROR ({label}) @ token {}]",
            self.token_text(error.token_index)
        );
        if error.message.is_empty() {
            let _ = writeln!(out, "  {kind}");
        } else {
            let _ = writeln!(out, "  {}", error.message);
        }
        if let Some(token) = self.token(error.token_index) {
            let width = if token.token_type == TOKEN_EOF {
                1
            } else {
                token
                    .text
                    .lines()
                    .next()
                    .map_or(0, |line| line.chars().count())
                    .max(1)
            };
            source_excerpt(out, self.input, token.line, token.column, width);
        }
        if !error.expected.is_empty() {
            let mut names = error
                .expected
                .iter()
                .take(EXPECTED_TOKENS)
                .map(|token_type| self.token_type_name(*token_type))
                .collect::<Vec<_>>();
            if error.expected.len() > EXPECTED_TOKENS {
                names.push(format!(
                    "and {} more",
                    error.expected.len() - EXPECTED_TOKENS
                ));
            }
            let _ = writeln!(out, "  expected: {}", names.join(", "));
        }
        if self.verbose {
            let _ = writeln!(out, "  rule stack:");
            for frame in &error.stack {
                let _ = writeln!(
                    out,
                    "    {} (from token {})",
                    self.rule_name(frame.rule),
                    self.token_text(frame.start)
                );
            }
        }
    }

    fn guesses_section(&self, out: &mut String) {
        let error_stack = match &self.static_trace.outcome {
            Outcome::Failed(error) => Some(error.stack.as_slice()),
            Outcome::Accepted { .. } => None,
        };
        let divergence = match self.verdict {
            Verdict::Divergence { step } => Some(step),
            _ => None,
        };
        let guesses = self
            .static_trace
            .events
            .iter()
            .enumerate()
            .filter(|(_, event)| event.is_guess())
            .collect::<Vec<_>>();
        let _ = writeln!(out);
        let title = if error_stack.is_some() {
            "STATIC GUESSES BEFORE THE ERROR"
        } else {
            "STATIC GUESSES"
        };
        let on_stack =
            |event: &DecisionEvent| error_stack.is_some_and(|stack| is_prefix(&event.stack, stack));
        match error_stack {
            Some(_) => {
                let active = guesses.iter().filter(|(_, event)| on_stack(event)).count();
                let _ = writeln!(
                    out,
                    "[{title}: {} ({active} on the rule stack, {} in completed subtrees)]",
                    guesses.len(),
                    guesses.len() - active
                );
            }
            None => {
                let _ = writeln!(out, "[{title}: {}]", guesses.len());
            }
        }
        let skipped = guesses.len().saturating_sub(LISTED_GUESSES);
        if skipped > 0 {
            let _ = writeln!(out, "  ({skipped} earlier guesses not shown)");
        }
        for (step, event) in guesses.iter().skip(skipped) {
            let mut tags = String::new();
            if error_stack.is_some() {
                tags.push_str(if on_stack(event) {
                    " [rule stack]"
                } else {
                    " [completed subtree]"
                });
            }
            if divergence == Some(*step) {
                tags.push_str(" [DIVERGENCE]");
            }
            let state = event
                .walk()
                .map_or_else(String::new, |walk| format!(" s{}", walk.state));
            let kind = match &event.resolution {
                Resolution::DescentFallback { .. } => "descent fallback",
                _ => match event.walk().map(|walk| walk.kind) {
                    Some(TableKind::MinAlt) => "MinAlt",
                    Some(TableKind::MinViableAlt) => "MinViableAlt",

                    Some(TableKind::GuardDeferred) => "guard defers",
                    _ => "guess",
                },
            };
            let alt = event.alt.unwrap_or_default();
            let _ = writeln!(
                out,
                "  step {step}, token {}: decision {} in rule '{}'{state} -> alt {alt} via {kind}{tags}",
                self.token_text(event.token_index),
                event.decision,
                self.rule_name(event.rule),
            );
        }
    }

    fn default_channel_tokens(&self) -> usize {
        self.lexed
            .tokens
            .iter()
            .filter(|token| token.channel == TOKEN_DEFAULT_CHANNEL && token.token_type != TOKEN_EOF)
            .count()
    }

    /// The number of default-channel tokens after `from`, up to and including
    /// `to`.
    fn distance(&self, from: isize, to: isize) -> usize {
        let (Ok(from), Ok(to)) = (usize::try_from(from), usize::try_from(to)) else {
            return 0;
        };
        self.lexed
            .tokens
            .iter()
            .enumerate()
            .filter(|(index, token)| {
                *index > from && *index <= to && token.channel == TOKEN_DEFAULT_CHANNEL
            })
            .count()
    }

    fn token(&self, index: isize) -> Option<&super::lex::TokenView> {
        usize::try_from(index)
            .ok()
            .and_then(|index| self.lexed.tokens.get(index))
    }

    fn token_type(&self, index: isize) -> i32 {
        self.token(index)
            .map_or(TOKEN_EOF, |token| token.token_type)
    }

    fn token_quoted(&self, index: isize) -> String {
        match self.token(index) {
            Some(token) if token.token_type == TOKEN_EOF => "<EOF>".to_owned(),
            Some(token) => format!("'{}'", escape(&token.text)),
            None => "<EOF>".to_owned(),
        }
    }

    /// A token as `index 'text' (line L:C)`, with a 1-based column.
    fn token_text(&self, index: isize) -> String {
        match self.token(index) {
            Some(token) => format!(
                "{index} {} (line {}:{})",
                self.token_quoted(index),
                token.line,
                token.column + 1
            ),
            None => format!("{index} <EOF>"),
        }
    }

    fn token_type_name(&self, token_type: i32) -> String {
        if token_type == TOKEN_EOF {
            return "<EOF>".to_owned();
        }
        usize::try_from(token_type - 1)
            .ok()
            .and_then(|index| self.report.vocabulary.get(index))
            .map_or_else(|| token_type.to_string(), ToOwned::to_owned)
    }

    fn rule_name(&self, rule: usize) -> &str {
        self.report.rule_names.get(rule).map_or("?", String::as_str)
    }

    fn alt_name(&self, decision: usize, alt: i32) -> String {
        self.report.decisions.get(decision).map_or_else(
            || format!("alt {alt}"),
            |decision| decision.alternatives.name(alt),
        )
    }

    fn alt_list(&self, decision: usize, alts: &[i32]) -> String {
        self.report.decisions.get(decision).map_or_else(
            || {
                alts.iter()
                    .map(|alt| format!("alt {alt}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            },
            |decision| decision.alternatives.list(alts),
        )
    }
}

struct Diagnosis {
    cause: String,
    remediations: Vec<String>,
}

/// Whether the parse stack `inner` starts with the frames of `outer`.
fn is_prefix(outer: &[Frame], inner: &[Frame]) -> bool {
    outer.len() <= inner.len()
        && outer
            .iter()
            .zip(inner)
            .all(|(a, b)| a.rule == b.rule && a.start == b.start)
}

/// Writes the source line `line` of `input` with a caret under `width`
/// characters from the 0-based `column`.
fn source_excerpt(out: &mut String, input: &str, line: u32, column: i32, width: usize) {
    let Some(index) = usize::try_from(line)
        .ok()
        .and_then(|line| line.checked_sub(1))
    else {
        return;
    };
    let lines = input.lines().collect::<Vec<_>>();
    // `<EOF>` after a final line break is on the empty line after the last.
    let text = match lines.get(index) {
        Some(text) => *text,
        None if index == lines.len() => "",
        None => return,
    };
    let column = usize::try_from(column).unwrap_or(0);
    let number = line.to_string();
    let _ = writeln!(out, "  {number} | {}", text.replace('\t', " "));
    let _ = writeln!(
        out,
        "  {} | {}{}",
        " ".repeat(number.len()),
        " ".repeat(column),
        "^".repeat(width)
    );
}

fn escape(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
