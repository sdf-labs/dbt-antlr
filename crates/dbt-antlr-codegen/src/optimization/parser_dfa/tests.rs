// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Classifier tests over the golden grammar corpus.
//!
//! The expected categories are the ones the reference `antlr4.base2` classifier
//! reports for the same grammars (`-Xdecision-report`), with one deliberate
//! divergence on the small corpus: `Perf.g4`'s `stat` decision is
//! `EXACT_AMBIG` upstream only because the upstream classifier has
//! shared-descent groups, which this tree does not implement yet, so it
//! classifies conservatively as `HYBRID`.
//!
//! The same conservatism shows on large grammars, where the reference
//! classifier resolves a conflict from the *first* path that reached a config
//! set, while this classifier keeps the union of every path's taint
//! (`INTERIOR_TAINT` marks the paths that never popped the decision boundary).
//! Mixed usage is never trusted here; on `fixtures/vscode-cpp14/CPP14.g4` that
//! costs six `EXACT_AMBIG` decisions, which land on hybrid tables instead.

use std::path::{Path, PathBuf};

use crate::grammar::compiler::compile;
use crate::grammar::loader::LoadOptions;

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

/// Classifies every decision of every root parser of one grammar file.
fn classify(path: &Path) -> Vec<DecisionResult> {
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
        let ll1 = crate::optimization::static_parser_dfa::ll1_disjointness(parser);
        results.extend(DecisionClassifier::new(parser.atn, ll1).classify_all());
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
        // Upstream reports `EXACT_AMBIG` for the first decision via its
        // shared-descent groups; without them the conflict stays untrusted.
        (
            "Perf.g4",
            &[
                Category::HYBRID,
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

#[test]
fn ll1_decisions_need_no_table() {
    let results = classify(&grammars_dir().join("CSV.g4"));
    for result in &results {
        assert_eq!(result.category, Category::LL1);
        assert_eq!(result.k, 1);
        assert!(result.table.is_none());
        assert!(result.is_fully_static());
        assert_eq!(result.fallback_text(), "0");
    }
}

#[test]
fn hybrid_tables_escape_at_the_conflict_state() {
    let results = classify(&grammars_dir().join("ReferenceToATN.g4"));
    let decision = &results[0];
    assert_eq!(decision.category, Category::HYBRID);
    assert_eq!(decision.k, 2);
    assert_eq!(decision.escapes, 1);
    assert!(!decision.is_fully_static());
    assert_eq!(decision.fallback_text(), "1");
    let table = decision.table.as_ref().expect("hybrid table is kept");
    assert_eq!(table.states.len(), 5);
    assert_eq!(table.states[0].edges.len(), 3);
    assert_eq!(decision.conflict_witness.as_deref(), Some(&[2, -1][..]));
}

#[test]
fn precedence_decisions_are_classified_per_class() {
    let results = classify(&grammars_dir().join("SimpleLR.g4"));
    let decision = &results[0];
    assert_eq!(decision.category, Category::LR_PRECEDENCE);
    let precedence = decision.precedence.as_ref().expect("precedence tables");
    assert_eq!(precedence.cutoffs, [2]);
    assert_eq!(precedence.tables.len(), 2);
    assert_eq!(precedence.adaptive_classes, 0);
    assert!(precedence.tables.iter().all(Option::is_some));
    assert!(decision.is_fully_static());
    assert_eq!(decision.fallback_text(), "0");
}

#[test]
fn predicated_branches_escape_while_the_loop_stays_static() {
    let shapes = fixture("parser-shapes").join("ParserShapes.g4");
    let results = classify(&shapes);
    let plus = &results[1];
    // The `(B | C)+` loop decision itself is static; only the configs the
    // FOLLOW chase reached through `{ready()}?` depend on the parse state, so
    // they escape and the table stays (Tier 3, per-config predicate taint).
    assert_eq!(plus.category, Category::HYBRID);
    assert!(plus.saw_predicate || plus.first_predicate.is_some());
    let table = plus.table.as_ref().expect("the loop keeps a hybrid table");
    assert!(table.escapes() > 0, "the predicated branch escapes");
    assert!(!plus.is_fully_static());
}

#[test]
fn non_greedy_decisions_are_flagged_without_a_table() {
    let shapes = fixture("parser-shapes").join("ParserShapes.g4");
    let results = classify(&shapes);
    let nongreedy = &results[3];
    assert_eq!(nongreedy.category, Category::NON_GREEDY);
    assert!(nongreedy.table.is_none());
    assert!(nongreedy.is_fully_static());
}

#[test]
fn classification_is_deterministic() {
    for file in ["Labels.g4", "Perf.g4", "ReferenceToATN.g4", "SimpleLR.g4"] {
        let path = grammars_dir().join(file);
        let first = classify(&path);
        let second = classify(&path);
        assert_eq!(first.len(), second.len(), "{file}");
        for (first, second) in first.iter().zip(&second) {
            assert_eq!(first.category, second.category, "{file}");
            assert_eq!(first.k, second.k, "{file}");
            assert_eq!(first.num_dfa_states, second.num_dfa_states, "{file}");
            assert_eq!(first.table, second.table, "{file}");
            assert_eq!(first.precedence, second.precedence, "{file}");
        }
    }
}

/// A table must never consume EOF in a loop: the runtime's walker does not
/// advance on EOF, so an EOF edge back to the same state would spin forever
/// instead of predicting.
#[test]
fn no_table_has_an_eof_self_loop() {
    let grammars = [
        "CSV.g4",
        "Labels.g4",
        "Perf.g4",
        "ReferenceToATN.g4",
        "SimpleLR.g4",
        "VisitorBasic.g4",
        "VisitorCalc.g4",
    ];
    let mut tables = Vec::new();
    for file in grammars {
        for result in classify(&grammars_dir().join(file)) {
            if let Some(table) = &result.table {
                tables.push((file, result.decision, table.clone()));
            }
            if let Some(precedence) = &result.precedence {
                for table in precedence.tables.iter().flatten() {
                    tables.push((file, result.decision, table.clone()));
                }
            }
        }
    }
    assert!(!tables.is_empty(), "the corpus produces tables");
    for (file, decision, table) in tables {
        for (index, state) in table.states.iter().enumerate() {
            for edge in &state.edges {
                assert!(
                    !(edge.lo == -1 && edge.target == index),
                    "{file} decision {decision}: state {index} loops on EOF"
                );
            }
        }
    }
}

/// Classification must survive a broad slice of the fixture corpus: no panic,
/// no hang, and every table well formed.
///
/// The corpus is sampled (every fourth grammar) because compiling each grammar
/// fronts the ANTLRv4 frontend, which dominates the runtime of a debug build;
/// the sample still spans ambiguous EBNF, left recursion, predicates, and
/// imported grammars. The visit budget is exercised by the largest grammars of
/// the sample.
#[test]
fn fixture_corpus_classifies_without_panicking() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("codegen-direct")
        .join("fixtures");
    let mut all = Vec::new();
    collect_grammars(&fixtures, &mut all);
    assert!(all.len() > 100, "fixture corpus found: {}", all.len());
    all.sort();
    let grammars = all
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 4 == 0)
        .map(|(_, path)| path)
        .collect::<Vec<_>>();

    let mut classified = 0_usize;
    for path in grammars {
        let Ok(compilation) = compile(LoadOptions {
            roots: vec![path.clone()],
            library_directories: Vec::new(),
        }) else {
            continue;
        };
        for root in &compilation.roots {
            let Some(parser_id) = root.parser else {
                continue;
            };
            let parser = compilation.parser(parser_id).expect("parser artifact");
            let ll1 = crate::optimization::static_parser_dfa::ll1_disjointness(parser);
            let results = DecisionClassifier::new(parser.atn, ll1).classify_all();
            for result in results {
                classified += 1;
                for table in result.table.iter().chain(
                    result
                        .precedence
                        .iter()
                        .flat_map(|p| p.tables.iter().flatten()),
                ) {
                    assert!(
                        !table.states.is_empty(),
                        "{}: decision {} has an empty table",
                        path.display(),
                        result.decision
                    );
                    for state in &table.states {
                        for edge in &state.edges {
                            assert!(
                                edge.target < table.states.len(),
                                "{}: decision {} edge leaves the table",
                                path.display(),
                                result.decision
                            );
                            assert!(
                                edge.lo <= edge.hi,
                                "{}: empty edge interval",
                                path.display()
                            );
                        }
                    }
                }
            }
        }
    }
    assert!(classified > 10, "classified {classified} decisions");
}

fn collect_grammars(directory: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_grammars(&path, out);
        } else if path.extension().is_some_and(|extension| extension == "g4") {
            out.push(path);
        }
    }
}

/// The fixture sample, classified under a small visit budget: every attempt is
/// bounded, so this checks the budget path on grammars large enough to hit it
/// without paying the production budget's time.
#[test]
fn whole_fixture_corpus_classifies_under_a_small_budget() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("codegen-direct")
        .join("fixtures");
    let mut all = Vec::new();
    collect_grammars(&fixtures, &mut all);
    all.sort();
    let grammars = all
        .into_iter()
        .enumerate()
        .filter(|(index, _)| index % 4 == 0)
        .map(|(_, path)| path)
        .collect::<Vec<_>>();
    let mut tables = 0_usize;
    for path in grammars {
        let Ok(compilation) = compile(LoadOptions {
            roots: vec![path.clone()],
            library_directories: Vec::new(),
        }) else {
            continue;
        };
        for root in &compilation.roots {
            let Some(parser_id) = root.parser else {
                continue;
            };
            let parser = compilation.parser(parser_id).expect("parser artifact");
            let ll1 = crate::optimization::static_parser_dfa::ll1_disjointness(parser);
            let results = DecisionClassifier::new(parser.atn, ll1)
                .with_visit_budget(20_000)
                .classify_all();
            for result in results {
                for table in result.table.iter().chain(
                    result
                        .precedence
                        .iter()
                        .flat_map(|p| p.tables.iter().flatten()),
                ) {
                    tables += 1;
                    assert!(!table.states.is_empty(), "{}", path.display());
                    for state in &table.states {
                        for edge in &state.edges {
                            assert!(edge.target < table.states.len(), "{}", path.display());
                        }
                    }
                }
            }
        }
    }
    assert!(tables > 0, "the corpus produces tables");
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
    let ll1 = crate::optimization::static_parser_dfa::ll1_disjointness(parser);
    DecisionClassifier::new(parser.atn, ll1)
        .with_visit_budget(budget)
        .classify_all()
}

/// Builds the runtime tables of one grammar through the pass, exactly as the
/// compiler stage does.
fn build_tables(path: &Path) -> (Vec<DecisionResult>, super::super::StaticParserTables) {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let build = crate::optimization::StaticParserDfa::build(
        compilation
            .parser(compilation.roots[0].parser.expect("parser root"))
            .expect("parser artifact"),
    );
    let results = classify(path);
    (results, build.tables)
}

#[test]
fn table_words_round_trip_through_the_runtime_decoder() {
    let (results, tables) = build_tables(&grammars_dir().join("ReferenceToATN.g4"));
    assert!(tables.has_table(0), "decision 0 has a hybrid table");
    assert!(tables.defers(0), "the hybrid table keeps an adaptive path");

    let words = tables.words();
    let decoded = dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables::from_words(&words)
        .expect("the emitted words decode");
    assert_eq!(decoded.to_words(), words, "the codec is stable");
    assert_eq!(
        decoded.table(0, 0).expect("decoded table").num_states(),
        results[0].num_dfa_states
    );
}

#[test]
fn fully_static_decisions_never_defer() {
    let (results, tables) = build_tables(&grammars_dir().join("parser/StaticDFA.g4"));
    assert!(!results.is_empty());
    for result in &results {
        assert!(
            result.is_fully_static(),
            "decision {} of rule index {} should be fully static, got {:?}",
            result.decision,
            result.rule_index,
            result.category
        );
        if matches!(result.category, Category::LL1 | Category::NON_GREEDY) {
            // These categories need no table at all.
            continue;
        }
        assert!(tables.has_table(result.decision));
        assert!(!tables.defers(result.decision));
    }
}

#[test]
fn hybrid_frontier_escapes_carry_continuations() {
    let (results, tables) = build_tables(&grammars_dir().join("parser/FuzzExpr.g4"));
    let hybrid = results
        .iter()
        .find(|result| result.category == Category::HYBRID && result.escapes > 0)
        .expect("FuzzExpr has a hybrid decision");
    let words = tables.words();
    let decoded = dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables::from_words(&words)
        .expect("the emitted words decode");
    assert_eq!(decoded.to_words(), words, "the codec is stable");

    let table = decoded
        .table(hybrid.decision as i32, 0)
        .expect("the hybrid decision has a table");
    let continuations = (0..table.num_states() as i32)
        .filter_map(|state| table.continuation(state))
        .count();
    assert!(
        continuations > 0,
        "decision {} has {} escapes, so its frontier must carry continuations",
        hybrid.decision,
        hybrid.escapes
    );
}

#[test]
fn precedence_class_tables_are_deduplicated() {
    let (results, tables) = build_tables(&grammars_dir().join("parser/LrDfa.g4"));
    let precedence = results
        .iter()
        .find(|result| result.precedence.is_some())
        .expect("LrDfa has a precedence decision");
    let classes = precedence.precedence.as_ref().unwrap();
    assert!(classes.tables.iter().flatten().count() > 0);

    let words = tables.words();
    let decoded = dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables::from_words(&words)
        .expect("the emitted words decode");
    assert!(
        decoded.is_precedence_dispatched(precedence.decision as i32),
        "the precedence decision dispatches on its precedence"
    );
    let first = decoded.table(
        precedence.decision as i32,
        classes.cutoffs.last().copied().unwrap_or(0) + 1,
    );
    assert!(first.is_some(), "the highest class resolves to a table");
}

#[test]
fn table_build_is_deterministic() {
    let path = grammars_dir().join("parser/FuzzExpr.g4");
    let (_, first) = build_tables(&path);
    let (_, second) = build_tables(&path);
    assert_eq!(first.words(), second.words());
}

#[test]
fn packed_atn_tail_decodes_back_into_the_same_tables() {
    use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
    use dbt_antlr_runtime::dfa::static_parser_dfa::append_sdfa_section;

    for grammar in [
        "ReferenceToATN.g4",
        "parser/StaticDFA.g4",
        "parser/LrDfa.g4",
    ] {
        let path = grammars_dir().join(grammar);
        let compilation = compile(LoadOptions {
            roots: vec![path.clone()],
            library_directories: Vec::new(),
        })
        .expect("grammar compiles");
        let parser = compilation
            .parser(compilation.roots[0].parser.expect("parser root"))
            .expect("parser artifact");
        let build = crate::optimization::StaticParserDfa::build(parser);
        let packed = crate::grammar::atn::encode_packed_atn(parser.atn);
        let with_tables = append_sdfa_section(&packed, &build.tables.words());
        assert!(
            with_tables.len() > packed.len(),
            "{grammar}: tables appended"
        );

        let atn = PackedATNDeserializer::new()
            .deserialize(&with_tables)
            .expect("the extended stream deserializes");
        assert_eq!(
            atn.static_dfas().to_words(),
            build.tables.words(),
            "{grammar}: the decoded tables round-trip"
        );

        let plain = PackedATNDeserializer::new()
            .deserialize(&packed)
            .expect("a stream without tables deserializes");
        assert!(
            plain.static_dfas().is_empty(),
            "{grammar}: no tail, no tables"
        );
    }
}
