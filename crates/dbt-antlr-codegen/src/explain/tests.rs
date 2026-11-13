// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Tests of the `--explain` simulations and their comparison.

use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::diff::Verdict;
use super::interp::{Outcome, Resolution, TableKind, Trace};
use super::{ExplainRequest, explain_units, units};
use crate::emission::compile_grammar_with;
use crate::optimization::{OptimizationOptions, OptimizationPlan};

struct Run {
    report: String,
    static_trace: Trace,
    ll_trace: Trace,
    verdict: Verdict,
}

/// Explains `input` against the combined grammar `source`.
fn run(name: &str, source: &str, input: &str, static_only: bool) -> Run {
    static RUNS: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "dbt-antlr-explain-{}-{name}-{}",
        std::process::id(),
        RUNS.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("temporary directory");
    let grammar = dir.join(format!("{name}.g4"));
    std::fs::write(&grammar, source).expect("grammar written");
    let mut options = OptimizationOptions::new();
    options.report_only(true);
    if static_only {
        options.static_dfa_only(true);
    } else {
        options.static_parser_dfa(true);
    }
    let plan = OptimizationPlan::from_options(&options).expect("valid options");
    let compilation = compile_grammar_with(&grammar, &[], &plan).expect("the grammar compiles");
    let (parser, lexer) = units(&compilation, None).expect("units found");
    let request = ExplainRequest {
        input: input.to_owned(),
        input_name: "<test>".to_owned(),
        start_rule: None,
        lexer: None::<PathBuf>,
    };
    let explained = explain_units(&compilation, parser, lexer, plan.atn_fallback(), &request)
        .expect("the explanation runs");
    let _ = std::fs::remove_dir_all(&dir);
    let (static_trace, ll_trace, verdict) = explained.traces.expect("the lexer accepts the input");
    Run {
        report: explained.report,
        static_trace,
        ll_trace,
        verdict,
    }
}

const BUDGET: &str = "grammar Budget;
prog : stat+ EOF ;
stat options { max_dfa_states = 3; } : ID ID ID ';' | ID ID '=' ID ';' ;
ID : [a-z]+ ;
WS : [ \\t\\r\\n]+ -> skip ;
";

const CONTEXT: &str = "grammar Context;
s : 'x' e 'y' EOF | 'w' e EOF ;
e : 'a' 'y' | 'a' ;
WS : [ \\t\\r\\n]+ -> skip ;
";

const DESCENT: &str = "grammar Descent;
s : x EOF ;
x : y '+' ID | y ;
y : '(' y ')' | ID ;
ID : [a-z]+ ;
WS : [ \\t]+ -> skip ;
";

const EXPR: &str = "grammar Expr;
prog : e (';' e)* EOF ;
e : e '*' e | e '+' e | '-' e | '(' e ')' | ID | e '?' e ':' e ;
ID : [a-z]+ ;
WS : [ \\t]+ -> skip ;
";

#[test]
fn a_budget_guess_is_the_first_divergence() {
    let run = run("Budget", BUDGET, "a b = c ;", true);
    assert_eq!(run.verdict, Verdict::Divergence { step: 0 });
    let Resolution::Table(walk) = &run.static_trace.events[0].resolution else {
        panic!("a table predicts `stat`: {:?}", run.static_trace.events[0]);
    };
    assert_eq!(walk.kind, TableKind::MinViableAlt);
    assert_eq!(walk.dropped, vec![2]);
    assert_eq!(walk.lookahead, vec![0, 1]);
    assert_eq!(run.static_trace.events[0].alt, Some(1));
    assert_eq!(run.ll_trace.events[0].alt, Some(2));
    assert!(
        matches!(run.static_trace.outcome, Outcome::Failed(ref error) if error.token_index == 2)
    );
    assert!(matches!(run.ll_trace.outcome, Outcome::Accepted { .. }));
    assert!(
        run.report.contains("root cause: lookahead budget"),
        "{}",
        run.report
    );
}

#[test]
fn an_input_that_the_guess_fits_agrees() {
    let run = run("Budget", BUDGET, "a b c ; d e f ;", true);
    assert_eq!(run.verdict, Verdict::Agree);
    assert_eq!(run.static_trace.events.len(), run.ll_trace.events.len());
}

#[test]
fn the_hybrid_parser_keeps_adaptive_prediction() {
    let run = run("Budget", BUDGET, "a b = c ;", false);
    assert_eq!(run.verdict, Verdict::Agree);
    let escaped = run
        .static_trace
        .events
        .iter()
        .any(|event| match &event.resolution {
            Resolution::Table(walk) => walk.kind == TableKind::Escape,
            Resolution::Adaptive => true,
            _ => false,
        });
    assert!(escaped, "{:?}", run.static_trace.events);
}

#[test]
fn an_sll_conflict_is_context_sensitivity() {
    let run = run("Context", CONTEXT, "x a y", true);
    assert_eq!(run.verdict, Verdict::Divergence { step: 1 });
    let walk = run.static_trace.events[1]
        .walk()
        .expect("a table predicts `e`");
    assert_eq!(walk.kind, TableKind::MinAlt);
    assert!(
        run.report.contains("root cause: context sensitivity"),
        "{}",
        run.report
    );
    assert!(
        run.report.contains("[completed subtree] [DIVERGENCE]"),
        "{}",
        run.report
    );
}

#[test]
fn the_same_failure_means_the_grammar_rejects_the_input() {
    let run = run("Budget", BUDGET, "a b ;", true);
    assert_eq!(run.verdict, Verdict::GrammarRejects);
}

#[test]
fn a_shared_descent_group_dispatches_on_the_tail() {
    for (input, member) in [("((a)) + b", 1), ("((a))", 2)] {
        let run = run("Descent", DESCENT, input, true);
        assert_eq!(run.verdict, Verdict::Agree, "{}", run.report);
        let event = run
            .static_trace
            .events
            .iter()
            .find(|event| matches!(event.resolution, Resolution::Descent { .. }))
            .unwrap_or_else(|| panic!("a group predicts `x`: {:?}", run.static_trace.events));
        assert_eq!(event.alt, Some(member), "{input}");
    }
}

#[test]
fn left_recursive_rules_agree() {
    let run = run("Expr", EXPR, "a + b * -c ? (d) : e + f ; g", true);
    assert_eq!(run.verdict, Verdict::Agree, "{}", run.report);
    assert!(
        run.static_trace
            .events
            .iter()
            .any(|event| matches!(event.resolution, Resolution::Ll1 { .. })),
        "{:?}",
        run.static_trace.events
    );
    assert_eq!(run.static_trace.events.len(), run.ll_trace.events.len());
}

const INNER: &str = "grammar Inner;
s : x EOF ;
x : y '+' ID | y '-' ID ;
y : '(' y ')' | z ;
z options { max_dfa_states = 3; } : ID ID ID ';' | ID ID '=' ID ';' ;
ID : [a-z]+ ;
WS : [ \\t]+ -> skip ;
";

const CYCLIC: &str = "grammar Cyclic;
s : 'w' r 'z' EOF | 'v' r EOF ;
r : 'a'+ 'x' 'z' | 'a'+ 'x' ;
WS : [ \\t]+ -> skip ;
";

#[test]
fn a_divergence_inside_a_neutral_parse_is_found() {
    let run = run("Inner", INNER, "((( p q = r ; ))) + m", true);
    let Verdict::Divergence { step } = run.verdict else {
        panic!("{}", run.report);
    };
    assert!(
        run.static_trace.events[..step]
            .iter()
            .any(|event| matches!(event.resolution, Resolution::DescentAborted { .. })),
        "{:?}",
        run.static_trace.events
    );
    let walk = run.static_trace.events[step]
        .walk()
        .expect("a table predicts `z`");
    assert_eq!(walk.kind, TableKind::MinViableAlt);
}

#[test]
fn a_cyclic_table_walk_reads_the_whole_lookahead() {
    let run = run("Cyclic", CYCLIC, "w a a a a a a a a a a a a x z", true);
    assert_eq!(run.verdict, Verdict::Divergence { step: 1 });
    let walk = run.static_trace.events[1]
        .walk()
        .expect("a table predicts `r`");
    assert_eq!(walk.kind, TableKind::MinAlt);
    assert_eq!(walk.lookahead.len(), 15);
    assert!(
        run.report.contains("'a' 'x' 'z' <EOF> (15 tokens)"),
        "{}",
        run.report
    );
}

const INNER_EARLY: &str = "grammar InnerEarly;
s : x EOF ;
x : y '+' ID | y '-' ID ;
y : '(' y | z ;
z options { max_dfa_states = 3; } : ID ID | ID ID '=' ID ;
ID : [a-z]+ ;
WS : [ \\t]+ -> skip ;
";

const PREDICATED: &str = "grammar Predicated;
s : ({true}? 'a' | 'b') ';' EOF ;
WS : [ \\t]+ -> skip ;
";

#[test]
fn a_guess_that_ends_the_shared_rule_early_is_the_root_cause() {
    let run = run("InnerEarly", INNER_EARLY, "((( a b = c - d", true);
    let Verdict::Divergence { step } = run.verdict else {
        panic!("{}", run.report);
    };
    let walk = run.static_trace.events[step]
        .walk()
        .expect("a table predicts `z`");
    assert_eq!(walk.kind, TableKind::MinViableAlt, "{}", run.report);
    assert!(
        run.static_trace.events[..step]
            .iter()
            .any(|event| matches!(event.resolution, Resolution::Descent { .. })),
        "{:?}",
        run.static_trace.events
    );
}

#[test]
fn a_decision_without_a_static_table_fails_under_static_dfa_only() {
    let run = run("Predicated", PREDICATED, "a ;", true);
    assert_eq!(run.verdict, Verdict::StaticFailure { step: Some(0) });
    assert_eq!(
        run.static_trace.events[0].resolution,
        Resolution::NoStaticTable
    );
    assert!(run.report.contains("(G4S190)"), "{}", run.report);
}
