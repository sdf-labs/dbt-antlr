// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The text report of `--explain`.
//!
//! The report is plain text with a fixed layout: it has no terminal width
//! dependent wrapping and no file system paths, so snapshot tests of it are
//! the same on every platform.

use std::fmt::Write as _;

use dbt_antlr_runtime::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF};

use super::diff::Verdict;
use super::interp::{
    DecisionEvent, ErrorKind, Frame, Outcome, Resolution, SyntaxError, TableKind, TableWalk, Trace,
};
use super::lex::{LexError, Lexed};
use crate::optimization::StaticParserDfaReport;

/// The expected tokens that a syntax error lists at most.
const EXPECTED_TOKENS: usize = 12;
/// The lookahead tokens of a table walk that the report shows at most.
const LOOKAHEAD_TOKENS: usize = 8;
/// The static guesses that the report lists at most.
const LISTED_GUESSES: usize = 10;

/// Everything the report shows.
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
        source_excerpt(&mut out, input, error.line, error.column, 1);
    }
    out
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
        self.verdict_section(&mut out);
        match self.verdict {
            Verdict::Divergence { step } => self.divergence_section(&mut out, step),
            Verdict::StaticFailure { step: Some(step) } => {
                self.static_failure_section(&mut out, step);
            }
            Verdict::Inconclusive { step } => self.inconclusive_section(&mut out, step),
            Verdict::Agree | Verdict::GrammarRejects | Verdict::StaticFailure { step: None } => {}
        }
        if let Outcome::Failed(error) = &self.static_trace.outcome {
            let _ = writeln!(out);
            self.error_section(&mut out, self.mode, error);
        } else if let Outcome::Failed(error) = &self.ll_trace.outcome {
            let _ = writeln!(out);
            self.error_section(&mut out, "full LL(*)", error);
        }
        self.guesses_section(&mut out);
        out
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
                "Both parsers accept this input, but the static parser builds another parse tree than full LL(*).".to_owned(),
            ),
            Verdict::Divergence { .. } => (
                "STATIC-DFA DIVERGENCE",
                "The static parser chooses another alternative than full LL(*). Full LL(*) also rejects the input later, so the input has another error too.".to_owned(),
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
                    "The {} parser fails without an earlier divergent choice. Full LL(*) also rejects the input later, so the input has another error too.",
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
        if let Some(cause) = self.root_cause(ours, our_alt, oracle_alt) {
            let _ = writeln!(out, "  root cause: {cause}");
        }
        self.dump_hint(out, ours.decision);
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
        self.dump_hint(out, ours.decision);
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

    /// Points to the `--dump-dfas` dump of a decision that has a static table.
    fn dump_hint(&self, out: &mut String, decision: usize) {
        let has_table = self.report.decisions.get(decision).is_some_and(|decision| {
            decision.result.table.is_some() || decision.result.precedence.is_some()
        });
        if has_table {
            let _ = writeln!(
                out,
                "  hint: inspect the table with `--dump-dfas {decision}`"
            );
        }
    }

    fn decision_headline(&self, out: &mut String, event: &DecisionEvent) {
        let category = self
            .report
            .decisions
            .get(event.decision)
            .map_or_else(String::new, |decision| {
                format!(" ({})", decision.result.category)
            });
        let _ = writeln!(
            out,
            "  rule '{}', decision {}{category}:",
            self.rule_name(event.rule),
            event.decision
        );
    }

    fn root_cause(&self, ours: &DecisionEvent, our_alt: i32, oracle_alt: i32) -> Option<String> {
        let rule = self.rule_name(ours.rule);
        if matches!(ours.resolution, Resolution::DescentFallback { .. }) {
            return Some(
                "the shared-descent group could not start its neutral parse and took its minimum member alternative.".to_owned(),
            );
        }
        if let Some(walk) = ours.walk()
            && matches!(walk.kind, TableKind::GuardTaken | TableKind::GuardDeferred)
        {
            return Some(format!(
                "the runtime stack guard of the table chose this side. The guard sees only the parse stack, not the tokens after the lookahead of rule '{rule}'."
            ));
        }
        let sll = self.probe?;
        Some(match sll {
            Some(sll) if sll == oracle_alt => format!(
                "lookahead budget. Unbounded SLL(*) also chooses alt {oracle_alt}, so a longer static lookahead decides this input. Left-factor the alternatives, or raise the max_dfa_states, max_dfa_visits, or max_k option of rule '{rule}' or of its block."
            ),
            Some(sll) if sll == our_alt => format!(
                "context sensitivity. SLL(*) also chooses alt {our_alt}: only full-context LL(*) sees that the tokens after rule '{rule}' select alt {oracle_alt}. A larger lookahead budget does not help. Change the grammar so that the caller decides, for example inline the rule at this call site."
            ),
            Some(sll) => format!("SLL(*) chooses alt {sll} here."),
            None => "SLL(*) prediction fails at this decision.".to_owned(),
        })
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
            Resolution::Adaptive => "adaptivePredict".to_owned(),
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
            TableKind::Escape => "an escape to adaptivePredict",
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

/// Whether the parse stack `inner` starts with the frames of `outer`.
fn is_prefix(outer: &[Frame], inner: &[Frame]) -> bool {
    outer.len() <= inner.len() && inner[..outer.len()] == *outer
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
