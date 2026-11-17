// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Classifier tests over the golden grammar corpus.
//!
//! The expected categories are the ones the reference `antlr4.base2` classifier
//! reports for the same grammars (`-Xdecision-report`).
//!
//! These are the tests of the classifier itself: its categories, its witness
//! pass, and its attempt sequence. They moved here with the classifier. The
//! tests of the table construction went on with `DecisionTableBuilder` in
//! `optimization::parser_dfa`.

use std::path::{Path, PathBuf};

use dbt_antlr_runtime::analysis::EOF;

use crate::grammar::compiler::compile;
use crate::grammar::loader::LoadOptions;
use crate::optimization::AtnFallback;
use crate::optimization::parser_dfa::cost::{AttemptKind, AttemptOutcome};
use crate::optimization::parser_dfa::table::{StaticDfaState, StaticDfaTable};

use super::{Category, DecisionClassifier, DecisionResult};

fn grammars_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate directory")
        .join("dbt-antlr-runtime")
        .join("grammars")
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("codegen-direct")
        .join("fixtures")
        .join(name)
}

/// The classifier of one compiled parser, set up as the static parser DFA
/// pass sets it up.
fn classifier(parser: &crate::grammar::atn::CompiledParser) -> DecisionClassifier {
    let ll1 = crate::optimization::static_parser_dfa::ll1_disjointness(parser);
    DecisionClassifier::new(parser.atn, ll1).with_descent_rules(
        crate::optimization::static_parser_dfa::descent_rules(parser),
    )
}

/// Classifies every decision of every root parser of one grammar file.
fn classify(path: &Path) -> Vec<DecisionResult> {
    classify_with(path, AtnFallback::Allowed)
}

/// [`classify`] with an explicit adaptive fallback policy.
fn classify_with(path: &Path, atn_fallback: AtnFallback) -> Vec<DecisionResult> {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let mut results = Vec::new();
    for root in &compilation.roots {
        let Some(parser_id) = root.parser else {
            continue;
        };
        let parser = compilation.parser(parser_id).expect("parser artifact");
        results.extend(
            classifier(parser)
                .with_atn_fallback(atn_fallback)
                .classify_all(),
        );
    }
    results
}

fn rule_names(path: &Path, results: &[DecisionResult]) -> Vec<String> {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let Some(root) = compilation.roots.first() else {
        return Vec::new();
    };
    let Some(parser) = compilation.parser(root.parser.expect("parser root")) else {
        return Vec::new();
    };
    results
        .iter()
        .map(|result| parser.semantic.recognizer.rule_names[result.rule_index].clone())
        .collect()
}

fn categories(path: &Path) -> Vec<Category> {
    classify(path)
        .into_iter()
        .map(|result| result.category)
        .collect()
}

#[test]
fn golden_corpus_categories_match_the_reference_classifier() {
    let cases: [(&str, &[Category]); 7] = [
        ("CSV.g4", &[Category::LL1; 4]),
        (
            "Labels.g4",
            &[
                Category::LL1,
                Category::LL1,
                Category::LLK,
                Category::LR_PRECEDENCE,
            ],
        ),
        ("SimpleLR.g4", &[Category::LR_PRECEDENCE]),
        ("VisitorBasic.g4", &[]),
        ("VisitorCalc.g4", &[Category::LLK, Category::LR_PRECEDENCE]),
        ("ReferenceToATN.g4", &[Category::HYBRID, Category::LL1]),
        // The shared-descent group of `expr ';' | expr '.'` cures every
        // conflict of the first decision, as upstream.
        (
            "Perf.g4",
            &[
                Category::EXACT_AMBIG,
                Category::LL1,
                Category::LLK,
                Category::LR_PRECEDENCE,
            ],
        ),
    ];
    for (file, expected) in cases {
        let path = grammars_dir().join(file);
        assert_eq!(categories(&path), expected, "{file}");
    }

    let shapes = fixture("parser-shapes").join("ParserShapes.g4");
    assert_eq!(
        categories(&shapes),
        [
            Category::LL1,
            Category::HYBRID,
            Category::LL1,
            Category::NON_GREEDY,
        ]
    );
    let results = classify(&shapes);
    let rules = rule_names(&shapes, &results);
    assert_eq!(rules, ["optional", "plus", "star", "nongreedy"]);
}

/// An exhausted visit budget is reported as an overflow, never as a table: the
/// config sets built after the budget ran out are incomplete.
#[test]
fn exhausted_visit_budget_yields_overflow_without_a_table() {
    let reference = classify_with_budget(&grammars_dir().join("ReferenceToATN.g4"), 1);
    let decision = &reference[0];
    assert_eq!(decision.category, Category::OVERFLOW);
    assert!(decision.table.is_none());
    assert!(!decision.is_fully_static());

    // A precedence decision keeps its category - the classes are what degrade.
    let precedence = classify_with_budget(&grammars_dir().join("SimpleLR.g4"), 1);
    let decision = &precedence[0];
    assert_eq!(decision.category, Category::LR_PRECEDENCE);
    let classes = decision.precedence.as_ref().expect("precedence classes");
    assert!(classes.adaptive_classes > 0);
    assert!(classes.tables.iter().all(Option::is_none));
    assert!(!decision.is_fully_static());
}

fn classify_with_budget(path: &Path, budget: u64) -> Vec<DecisionResult> {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let root = compilation.roots.first().expect("one root");
    let parser = compilation
        .parser(root.parser.expect("parser root"))
        .expect("parser artifact");
    classifier(parser).with_visit_budget(budget).classify_all()
}

/// The token types of `names`, each a literal (`'-'`) or a symbolic token name
/// of one grammar's parser root.
fn token_types(path: &Path, names: &[&str]) -> Vec<i32> {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let root = compilation.roots.first().expect("one root");
    let parser = compilation
        .parser(root.parser.expect("parser root"))
        .expect("parser artifact");
    let recognizer = &parser.semantic.recognizer;
    names
        .iter()
        .map(|name| {
            let position = |names: &[Option<String>]| {
                names
                    .iter()
                    .position(|candidate| candidate.as_deref() == Some(*name))
            };
            position(&recognizer.literal_names)
                .or_else(|| position(&recognizer.symbolic_names))
                .unwrap_or_else(|| panic!("unknown token {name}")) as i32
        })
        .collect()
}

/// The state that `tokens` lead to from the start state, or `None` when a token
/// has no edge.
fn walk<'t>(table: &'t StaticDfaTable, tokens: &[i32]) -> Option<&'t StaticDfaState> {
    let mut state = &table.states[0];
    for token in tokens {
        let edge = state
            .edges
            .iter()
            .find(|edge| edge.lo <= *token && *token <= edge.hi)?;
        state = &table.states[edge.target];
    }
    Some(state)
}

/// The build attempts of one decision, in order.
fn attempts_of(results: &[DecisionResult], decision: usize) -> Vec<AttemptKind> {
    results[decision]
        .cost
        .attempts
        .iter()
        .map(|attempt| attempt.kind)
        .collect()
}

/// A first pass that stops at an untrusted conflict does not try the widened
/// construction. When the decision stays adaptive, the witness pass runs the
/// unbounded construction again, but not the hybrid constructions, which do
/// not depend on the stop.
#[test]
fn witness_pass_repeats_only_the_stopped_attempt() {
    let path = fixture("vscode-split").join("TParser.g4");
    let results = classify_with_budget(&path, super::DEFAULT_VISIT_BUDGET);
    assert_eq!(results[5].category, Category::PREDICATED);
    assert!(results[5].table.is_none());
    assert_eq!(
        attempts_of(&results, 5),
        [
            AttemptKind::Unbounded { abort: true },
            AttemptKind::Hybrid,
            AttemptKind::Unbounded { abort: false }
        ]
    );
}

/// A decision whose first pass did not stop keeps its first-pass result, even
/// without a table.
#[test]
fn unstopped_first_pass_needs_no_witness_pass() {
    let path = grammars_dir().join("parser").join("LowCoverage.g4");
    let results = classify_with_budget(&path, super::DEFAULT_VISIT_BUDGET);
    assert_eq!(results[0].category, Category::PREDICATED);
    assert!(results[0].table.is_none());
    assert_eq!(
        attempts_of(&results, 0),
        [AttemptKind::Unbounded { abort: true }, AttemptKind::Hybrid]
    );
}

/// A start state that spends the visit budget spends it in every unbounded
/// construction, so no hybrid construction runs.
#[test]
fn exhausted_start_state_skips_the_hybrid_attempts() {
    let path = grammars_dir().join("ReferenceToATN.g4");
    let results = classify_with_budget(&path, 1);
    assert_eq!(results[0].category, Category::OVERFLOW);
    assert_eq!(
        attempts_of(&results, 0),
        [
            AttemptKind::Unbounded { abort: true },
            AttemptKind::Widened { abort: true }
        ]
    );
}

/// An unbounded attempt that stops at an untrusted conflict does not fall
/// back to full widening. Here the widened construction predicts the first
/// alternative after `'b' 'a'`, but when `a` is not nested, only the second
/// alternative can read `'b' 'a' 'd' 'e'`.
#[test]
fn stopped_unbounded_attempt_does_not_fall_back_to_widening() {
    let results = classify_source(
        "WidenedAfterStop",
        "grammar WidenedAfterStop;\n\
         top : a X | a Y ;\n\
         a : ( A | C ( B A | B a E ) ) D ;\n\
         A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ; X : 'x' ; Y : 'y' ;\n",
    );
    let decision = &results[1];
    assert_eq!(decision.category, Category::HYBRID);
    assert_eq!(decision.conflicts, [vec![1, 2]]);
    assert!(decision.table.is_some());
}

/// When the state budget sends the witness pass to the widened attempt, the
/// widened attempt does not resolve a conflict that only widening makes. In
/// `WidenedTrust.g4`, both alternatives of decision 1 meet after
/// `'b' 'b' 'b' 'b' 'a'` with an empty context, but only the second alternative
/// can read `c b b b b a d e d x`.
#[test]
fn widened_competitors_get_no_uniform_trust() {
    let path = grammars_dir().join("parser").join("WidenedTrust.g4");
    let results = classify(&path);
    let decision = &results[1];
    assert_eq!(
        attempts_of(&results, 1),
        [
            AttemptKind::Unbounded { abort: true },
            AttemptKind::Hybrid,
            AttemptKind::Unbounded { abort: false },
            AttemptKind::Widened { abort: false },
        ]
    );
    assert_eq!(
        decision.cost.attempts[2].outcome,
        AttemptOutcome::StateBudget
    );
    assert_eq!(decision.category, Category::CONTEXT_SENSITIVE);
    assert_eq!(decision.utrust_resolutions, 0);
    assert!(decision.table.is_none());
    assert!(!decision.is_fully_static());

    let static_only = classify_with(&path, AtnFallback::Forbidden);
    let decision = &static_only[1];
    assert_eq!(decision.category, Category::HYBRID);
    let types = token_types(&path, &["'a'", "'b'", "'d'", "'e'"]);
    let (a, b, d, e) = (types[0], types[1], types[2], types[3]);
    let table = decision.table.as_ref().expect("static-only table");
    let conflict = walk(table, &[b, b, b, b, a, d, e]).expect("conflict state");
    assert!(conflict.is_escape(), "the conflict is a reported escape");
    assert!(
        walk(table, &[b, b, b, b, a]).is_some_and(|state| state.accept == 0),
        "the table does not decide after `'b' 'b' 'b' 'b' 'a'`"
    );
}

/// The attempt codes name the precedence class of each attempt, the witness
/// pass, and the static-only hybrid attempts of `--static-dfa-only`.
#[test]
fn attempt_codes_follow_the_classes_and_the_fallback_policy() {
    let results = classify_with_budget(
        &grammars_dir().join("parser").join("LoopFrontier.g4"),
        2_000,
    );
    assert_eq!(
        results[4].cost.attempts_text(),
        "c0: U! H$; c1: U! H$; c2: U! H$; c3: U; c4: U"
    );

    let path = grammars_dir().join("ReferenceToATN.g4");
    assert_eq!(classify(&path)[0].cost.attempts_text(), "U! H");
    assert_eq!(
        classify_with(&path, AtnFallback::Forbidden)[0]
            .cost
            .attempts_text(),
        "U! S"
    );
}

// The integration test of `--report-decisions`, from `tests/dbt_static_dfa.rs`.
// It needs the `report_decisions` option of `OptimizationOptions` again.
#[test]
fn report_decisions_prints_the_decision_inventory() {
    let output = emit_with(|options| {
        options.report_decisions(true);
    });
    assert!(output.files.is_empty(), "report mode emits no files");
    let manifest = output
        .optimization_manifest
        .expect("manifest is still rendered");
    assert!(manifest.contains("\"reportOnly\": True"), "{manifest}");
    let [report] = output.reports.as_slice() else {
        panic!("one report per parser unit: {:?}", output.reports);
    };
    assert_eq!(
        report.lines().next(),
        Some("=== parser decision report: CSVParser (4 decisions) ==="),
        "{report}"
    );
    assert!(report.contains("summary: LL1=4"), "{report}");
    for row in [
        "  0  csvFile     12  LL1       1          0         0",
        "  1  row         21  LL1       1          0         0",
        "  2  row         25  LL1       1          0         0",
        "  3  field       32  LL1       1          0         0",
    ] {
        assert!(report.contains(row), "missing {row:?} in:\n{report}");
    }
}
