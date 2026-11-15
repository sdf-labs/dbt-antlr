// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The `--report-decisions` decision report of the parked grammar analyzer.
//!
//! The report rendered one line per decision with its [`Category`], the
//! category histogram, and the guard, shared-descent, and lookahead-depth
//! summaries. It moved here from `optimization::static_parser_dfa` when the
//! optimizer stopped classifying decisions. It still renders the report types
//! of that module, and its `render_blocks` and `finish_block`.

use std::collections::BTreeMap;

use super::{Category, DecisionResult};
use crate::optimization::static_parser_dfa::{StaticParserDecisionReport, StaticParserDfaReport};

/// Renders the `--report-decisions` report of every parser unit.
pub(crate) fn render_decision_report(reports: &[StaticParserDfaReport]) -> String {
    render_blocks(&reports.iter().map(decision_report_text).collect::<Vec<_>>())
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
