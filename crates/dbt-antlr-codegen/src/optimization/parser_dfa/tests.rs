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

use dbt_antlr_runtime::analysis::{BOUNDARY_TAINT, FOREIGN_LINEAGE_TAINT};

use crate::grammar::compiler::compile;
use crate::grammar::loader::LoadOptions;
use crate::optimization::AtnFallback;

use super::loop_block::LoopBlock;
use super::table::{StaticDfaState, StaticDfaTable};
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
        let ll1 = crate::optimization::static_parser_dfa::ll1_disjointness(parser);
        results.extend(
            DecisionClassifier::new(parser.atn, ll1)
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

/// The compiled ATN of one grammar's parser root.
fn parser_atn(path: &Path) -> &'static dbt_antlr_runtime::analysis::ATN {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let root = compilation.roots.first().expect("one root");
    compilation
        .parser(root.parser.expect("parser root"))
        .expect("parser artifact")
        .atn
}

/// The precedence-loop decision of the first left-recursive rule: a
/// `StarLoopEntry` whose exit branch ends the rule (the transformed operator
/// loop), not the `( ... )*` loop of a guarded alternative.
fn precedence_decision(
    atn: &dbt_antlr_runtime::analysis::ATN,
) -> dbt_antlr_runtime::analysis::ATNStateRef {
    use dbt_antlr_runtime::analysis::{ATNStateType, RuleStartState};
    atn.decision_to_state
        .iter()
        .copied()
        .find(|state| {
            state.state_type() == ATNStateType::StarLoopEntry
                && atn.rule_to_start_state[state.get_rule_index() as usize]
                    .try_as::<RuleStartState>()
                    .is_some_and(|start| start.is_left_recursive)
                && state.get_transitions().last().is_some_and(|transition| {
                    let loop_end = transition.get_target();
                    loop_end.state_type() == ATNStateType::LoopEnd
                        && loop_end.has_epsilon_only_transitions()
                        && loop_end.get_transitions().first().is_some_and(|epsilon| {
                            epsilon.get_target().state_type() == ATNStateType::RuleStop
                        })
                })
        })
        .expect("a left-recursive rule has a precedence loop")
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
        AtnFallback::Allowed,
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
        let build = crate::optimization::StaticParserDfa::build(parser, AtnFallback::Allowed);
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

/// The operator loop of a left-recursive rule covers the loop decision and the
/// operator alternatives, not the base alternatives and not other rules.
#[test]
fn operator_loop_covers_the_loop_positions_only() {
    let atn = parser_atn(&grammars_dir().join("parser").join("LoopSubst.g4"));
    let decision = precedence_decision(atn);
    let block = LoopBlock::of(atn, decision);
    let rule_index = decision.get_rule_index();

    assert!(block.guards_are_simple(), "the arithmetic loop is simple");
    assert!(
        block.is_loop_consumption(decision, None, BOUNDARY_TAINT),
        "the loop decision is part of the loop"
    );
    // An enclosing invocation that continues a base alternative consumes for
    // that alternative.
    let rule_start = atn.rule_to_start_state[rule_index as usize];
    assert!(!block.is_loop_consumption(rule_start, None, BOUNDARY_TAINT));
    assert!(block.is_foreign_call(rule_start, None));
    // An operand body runs with an operator-loop frame in its context, so its
    // own positions are not loop positions.
    let operand_state = atn.make_state_ref(
        atn.iter_states()
            .find(|state| state.get_rule_index() != rule_index)
            .expect("the grammar has another rule")
            .get_state_number(),
    );
    assert!(!block.is_loop_consumption(operand_state, None, BOUNDARY_TAINT));
    assert!(block.is_foreign_call(operand_state, None));
    assert!(!block.is_foreign_call(decision, None));
    // A new invocation that a foreign call started owns its tokens, even at the
    // positions of the loop.
    assert!(!block.is_loop_consumption(decision, None, BOUNDARY_TAINT | FOREIGN_LINEAGE_TAINT));
    // The decision reaches itself by epsilon, so it owes no token.
    assert!(!block.is_pending(decision));
}

/// A guarded alternative that iterates internally and then owes another token
/// breaks the enter/exit mapping, so the substitution is disabled for the
/// whole rule.
#[test]
fn compound_guarded_alternatives_are_not_substitutable() {
    let atn = parser_atn(&grammars_dir().join("parser").join("LoopVeto.g4"));
    let decision = precedence_decision(atn);
    let block = LoopBlock::of(atn, decision);
    assert!(
        !block.guards_are_simple(),
        "`(',' send)* '>>' send` is not a simple guarded alternative"
    );

    let results = classify(&grammars_dir().join("parser").join("LoopVeto.g4"));
    let result = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the loop decision");
    assert_eq!(result.escapes, 2, "the vetoed conflict escapes");
    assert!(!result.is_fully_static());
    let table = &result
        .precedence
        .as_ref()
        .expect("precedence tables")
        .tables[0]
        .as_ref()
        .expect("the first class has a table");
    assert!(
        table
            .states
            .iter()
            .any(|state| state.is_escape() && state.conflicts == [1, 2]),
        "the escape is the iterate/exit conflict"
    );
}

/// The same shape without the compound alternative stays fully static: the
/// iterate/exit conflicts after a lambda body (`x -> a + b`) resolve by
/// substitution. Without it this decision escapes 4 times.
#[test]
fn simple_guarded_alternatives_need_no_escape() {
    let results = classify(&grammars_dir().join("parser").join("LoopSubst.g4"));
    let result = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the arithmetic loop decision");
    assert_eq!(result.escapes, 0);
    assert!(result.is_fully_static());
    assert_eq!(result.fallback_text(), "0");
    let classes = result.precedence.as_ref().expect("precedence tables");
    assert_eq!(classes.adaptive_classes, 0);
    assert!(classes.tables.iter().all(Option::is_some));
    let tables: Vec<_> = classes.tables.iter().flatten().collect();
    for table in &tables {
        assert!(
            table
                .states
                .iter()
                .all(|state| !(state.is_escape() && state.conflicts == [1, 2])),
            "no iterate/exit state escapes"
        );
    }
    assert!(
        tables
            .iter()
            .flat_map(|table| table.states.iter())
            .any(|state| state.accept == 1 && state.conflicts == [1, 2]),
        "the substitution resolves an iterate/exit conflict to iterate"
    );
}

/// A follow that starts a new invocation of the loop's rule (`expr+`), or that
/// consumes an operator outside the loop (`expr '||' INT EOF`), is foreign to
/// the loop. The substitution then resolves none of the iterate/exit states,
/// neither the conflicts nor the states at the budget frontier.
#[test]
fn foreign_consumption_vetoes_the_substitution_at_the_frontier() {
    let results = classify(&grammars_dir().join("parser").join("LoopFrontier.g4"));
    let result = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the arithmetic loop decision");
    assert!(result.escapes > 0);
    assert!(!result.is_fully_static());
    let classes = result.precedence.as_ref().expect("precedence tables");
    assert_eq!(classes.adaptive_classes, 0);
    let states: Vec<_> = classes
        .tables
        .iter()
        .flatten()
        .flat_map(|table| table.states.iter())
        .collect();
    assert!(
        states
            .iter()
            .all(|state| !(state.accept == 1 && state.conflicts == [1, 2])),
        "no iterate/exit conflict resolves to iterate"
    );
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

/// After the exit, the loop does not own the tokens of a new invocation of the
/// rule, or the tokens that an enclosing alternative still owes.
/// `LoopSibling.g4` reads `a - - a ;` as `a` and `- - a`,
/// `with pair a - pair - a a` as `pair a (- pair (- a) a)`, `if a ? a - a` as
/// `a ? a - a`, and `opt a ? a - a - a !` as `a ? a - a - a !`. No loop can
/// resolve the iterate/exit conflict on these paths to iterate.
#[test]
fn tokens_that_the_loop_does_not_own_veto_the_substitution() {
    let path = grammars_dir().join("parser").join("LoopSibling.g4");
    let results = classify(&path);
    let names = rule_names(&path, &results);
    let types = token_types(&path, &["'-'", "'pair'", "ID"]);
    let (minus, pair, id) = (types[0], types[1], types[2]);
    for (rule, tokens) in [
        ("expr", vec![minus, minus, id]),
        ("term", vec![minus, pair, minus, id]),
        ("cond", vec![minus, id]),
        ("tail", vec![minus, id, minus, id]),
    ] {
        let (result, _) = results
            .iter()
            .zip(&names)
            .find(|(result, name)| result.category == Category::LR_PRECEDENCE && *name == rule)
            .unwrap_or_else(|| panic!("the loop decision of {rule}"));
        let ends: Vec<&StaticDfaState> = result
            .precedence
            .as_ref()
            .expect("precedence tables")
            .tables
            .iter()
            .flatten()
            .filter_map(|table| walk(table, &tokens))
            .collect();
        assert!(
            ends.iter().any(|state| state.is_escape()),
            "{rule}: the iterate/exit conflict escapes"
        );
        assert!(
            ends.iter().all(|state| state.accept != 1),
            "{rule}: no class table iterates on the path"
        );
    }
}

/// The call site of the first of two side-by-side calls of `rule_index`
/// (`r r`): the call whose follow state calls the rule again.
fn first_of_two_calls(atn: &dbt_antlr_runtime::analysis::ATN, rule_index: usize) -> i32 {
    use dbt_antlr_runtime::analysis::RuleTransition;
    let calls = |state: dbt_antlr_runtime::analysis::ATNStateRef| {
        state.get_transitions().iter().any(|transition| {
            transition
                .try_as::<RuleTransition>()
                .is_some_and(|call| call.get_target().get_rule_index() as usize == rule_index)
        })
    };
    atn.iter_states()
        .map(|state| atn.make_state_ref(state.get_state_number()))
        .find(|state| {
            state.get_transitions().iter().any(|transition| {
                transition.try_as::<RuleTransition>().is_some_and(|call| {
                    call.get_target().get_rule_index() as usize == rule_index
                        && calls(call.follow_state)
                })
            })
        })
        .expect("two side-by-side calls")
        .get_state_number()
}

/// After the skip, a new invocation of the decision rule owns the tokens that
/// it reads. `PostfixSibling.g4` reads `A - B ;` as `A` and `- B`, and
/// `with [ A - B ] X` as `[ A (- B) ] X`. On these paths the tables of `r` and
/// `q` must not take, and a guard must defer at the first of the two
/// side-by-side calls. In `mid A - B A - B A - B !` the skip of the second `m`
/// returns into the middle of the take path of the first, so `m` must not take
/// on that path either.
#[test]
fn new_invocations_after_the_skip_are_out_of_discipline() {
    let path = grammars_dir().join("parser").join("PostfixSibling.g4");
    let atn = parser_atn(&path);
    let results = classify(&path);
    let names = rule_names(&path, &results);
    let types = token_types(&path, &["'-'", "'B'", "';'", "']'", "'A'"]);
    let (minus, b, semi, close, a) = (types[0], types[1], types[2], types[3], types[4]);
    for (rule, tokens) in [
        ("r", vec![minus, b, semi]),
        ("q", vec![minus, b, close]),
        ("m", vec![minus, b, a, minus]),
    ] {
        let (result, _) = results
            .iter()
            .zip(&names)
            .find(|(result, name)| result.has_postfix_shape && *name == rule)
            .unwrap_or_else(|| panic!("the postfix decision of {rule}"));
        let table = result.table.as_ref().expect("a hybrid table");
        let state = walk(table, &tokens).expect("the path has edges");
        assert_ne!(state.accept, 1, "{rule}: the table does not take outright");
        assert!(
            state.is_escape() || state.is_guarded(),
            "{rule}: the conflict defers"
        );
        if state.is_guarded() && rule != "m" {
            let first_call = first_of_two_calls(atn, result.rule_index);
            assert!(
                table.guard_danger.contains(&first_call),
                "{rule}: the guard defers at the first of the two calls"
            );
        }
    }
}

/// A new invocation of the decision rule in an optional element after the
/// postfix can read what the postfix takes (`c : 'A' w? (c 'C' '!')?`), so the
/// shape check rejects it.
#[test]
fn optional_elements_after_the_postfix_starve_it() {
    let path = grammars_dir().join("parser").join("PostfixSibling.g4");
    let results = classify(&path);
    let names = rule_names(&path, &results);
    let shaped: Vec<&str> = results
        .iter()
        .zip(&names)
        .filter(|(result, _)| result.has_postfix_shape)
        .map(|(_, name)| name.as_str())
        .collect();
    assert_eq!(shaped, ["r", "q", "m"]);
}

/// The mirror test of a skip configuration, with no context: the frame that it
/// popped into.
#[test]
fn skip_mirrors_need_the_take_path_of_the_popped_frame() {
    use dbt_antlr_runtime::analysis::{MIDWAY_TAINT, ROOTED_TAINT};

    let atn = parser_atn(&grammars_dir().join("parser").join("DanglingElse.g4"));
    let (decision, shape) = atn
        .decision_to_state
        .iter()
        .find_map(|state| {
            super::postfix::PostfixShape::analyze(atn, *state).map(|shape| (*state, shape))
        })
        .expect("the `e?` shape");
    let take = decision.get_transitions()[0].get_target();
    let rule_start = atn.rule_to_start_state[decision.get_rule_index() as usize];

    assert!(
        shape.skip_mirrors(rule_start, None, 0),
        "inside the decision invocation"
    );
    assert!(
        shape.skip_mirrors(take, None, BOUNDARY_TAINT),
        "on the take path"
    );
    assert!(
        !shape.skip_mirrors(rule_start, None, BOUNDARY_TAINT),
        "off the take path"
    );
    for taint in [MIDWAY_TAINT, FOREIGN_LINEAGE_TAINT, ROOTED_TAINT] {
        assert!(!shape.skip_mirrors(take, None, BOUNDARY_TAINT | taint));
    }
    assert!(
        !shape.is_foreign_call(take, None),
        "the take path calls `e`"
    );
    assert!(shape.is_foreign_call(rule_start, None));
}

/// A skip that consumes on the take path of an enclosing invocation mirrors
/// the take reading: the dangling `else` still resolves to take.
#[test]
fn the_take_path_of_an_enclosing_invocation_mirrors_the_take() {
    let results = classify(&grammars_dir().join("parser").join("DanglingElse.g4"));
    let fires: usize = results.iter().map(|result| result.take_rule_fires).sum();
    assert_eq!(fires, 1);
    assert!(results.iter().all(DecisionResult::is_fully_static));
}

/// `pred=predicate?` is the canonical optional-postfix shape, and the
/// `set expr '=' expr` caller is where skip genuinely wins. The reference
/// classifier reports one postfix decision for this grammar with no take-rule
/// fire and 8 guarded takes; ours matches.
#[test]
fn optional_postfix_shapes_are_guarded() {
    let results = classify(&grammars_dir().join("parser").join("GuardExpr.g4"));
    let shaped: Vec<&DecisionResult> = results
        .iter()
        .filter(|result| result.has_postfix_shape)
        .collect();
    assert_eq!(shaped.len(), 1, "one `pred=predicate?` decision");
    assert_eq!(shaped[0].rule_index, 2, "the `expr` rule owns the shape");
    let fires: usize = results.iter().map(|result| result.take_rule_fires).sum();
    assert_eq!(fires, 0);
    let guarded: usize = results.iter().map(|result| result.guarded_takes).sum();
    assert_eq!(guarded, 8, "matching the reference classifier");
    assert!(!shaped[0].is_fully_static(), "a guard may defer");
    let table = shaped[0].table.as_ref().expect("the hybrid table is kept");
    assert!(!table.guard_danger.is_empty(), "the guard can trip");
    assert!(!table.guard_pass.is_empty(), "the walk can continue");
    assert!(
        table.states.iter().any(|state| state.is_guarded()),
        "the table carries guarded states"
    );

    // No shape means no report line either: the grammar corpus that has no
    // `X Y?` block must stay silent.
    let plain = classify(&grammars_dir().join("CSV.g4"));
    assert!(plain.iter().all(|result| !result.has_postfix_shape));
}

/// Classifies every decision of one grammar and resolves it for
/// `--static-dfa-only`.
fn resolve_static_only(path: &Path) -> Vec<DecisionResult> {
    let mut results = classify_with(path, AtnFallback::Forbidden);
    for result in &mut results {
        result.resolve_static_only();
    }
    results
}

/// The runtime decoding of the tables of `results`.
fn decoded_tables(
    results: &[DecisionResult],
) -> dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables {
    let words = super::build_tables(results).to_words();
    dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables::from_words(&words)
        .expect("the emitted words decode")
}

/// Every state of every runtime table either accepts an alternative or
/// continues the walk: no state defers to the adaptive simulation.
fn assert_no_runtime_escapes(results: &[DecisionResult]) {
    use dbt_antlr_runtime::dfa::static_parser_dfa::{CONFLICT_ESCAPE, ESCAPE, GUARDED};

    let decoded = decoded_tables(results);
    for result in results {
        let precedences = result.precedence.as_ref().map_or_else(
            || vec![0],
            |precedence| {
                std::iter::once(0)
                    .chain(precedence.cutoffs.iter().map(|cutoff| cutoff + 1))
                    .collect()
            },
        );
        for precedence in precedences {
            let Some(table) = decoded.table(result.decision as i32, precedence) else {
                continue;
            };
            for state in 0..table.num_states() as i32 {
                let accept = table.accept(state);
                assert!(
                    ![ESCAPE, CONFLICT_ESCAPE, GUARDED].contains(&accept),
                    "decision {} state {state} still escapes ({accept})",
                    result.decision
                );
            }
        }
    }
}

#[test]
fn static_only_resolves_conflict_escapes_to_the_minimum_alternative() {
    let results = resolve_static_only(&grammars_dir().join("ReferenceToATN.g4"));
    let decision = &results[0];
    assert_eq!(decision.category, Category::HYBRID);
    assert_eq!(decision.escapes, 0);
    assert!(decision.is_fully_static());
    assert_eq!(decision.fallback_text(), "1m");
    let resolved = decision.static_only.as_ref().expect("resolved");
    assert_eq!((resolved.min_alt, resolved.syntax_error), (1, 0));
    assert_eq!(resolved.min_alt_witness.as_deref(), Some(&[2, -1][..]));

    let table = decision.table.as_ref().expect("the table is kept");
    let state = &table.states[4];
    assert_eq!(
        state.accept, 1,
        "the ATN/ID conflict predicts alternative 1"
    );
    assert_eq!(state.resolution, Some(super::EscapeResolution::MinAlt));
    assert_eq!(table.escapes(), 0);
    assert_no_runtime_escapes(&results);
}

#[test]
fn static_only_turns_predicated_states_into_syntax_errors() {
    let results = resolve_static_only(&fixture("parser-shapes").join("ParserShapes.g4"));
    let plus = &results[1];
    let resolved = plus.static_only.as_ref().expect("the loop is resolved");
    assert_eq!(resolved.min_alt, 0);
    assert!(resolved.syntax_error > 0, "the predicated branches stop");
    assert!(resolved.syntax_error_witness.is_some());
    assert_eq!(plus.fallback_text(), format!("{}x", resolved.syntax_error));

    let table = plus.table.as_ref().expect("the hybrid table is kept");
    let dead_ends: Vec<_> = table
        .states
        .iter()
        .filter(|state| state.resolution == Some(super::EscapeResolution::SyntaxError))
        .collect();
    assert_eq!(dead_ends.len(), resolved.syntax_error);
    for state in dead_ends {
        assert!(state.predicated, "only a predicate stops the guess");
        assert_eq!(state.accept, 0);
        assert_eq!(
            state.fallback, 0,
            "no error-recovery alternative hides the error"
        );
        assert!(state.edges.is_empty());
        assert!(state.continuation.is_none());
    }

    let decoded = decoded_tables(&results);
    let runtime = decoded.table(1, 0).expect("decision 1 has a table");
    let dead_end = table
        .states
        .iter()
        .position(|state| state.resolution.is_some())
        .expect("a resolved state") as i32;
    assert_eq!(runtime.accept(dead_end), 0);
    assert_eq!(runtime.fallback(dead_end), None);
    for token in -1..=5 {
        assert_eq!(runtime.edge(dead_end, token), None, "token {token}");
    }
}

#[test]
fn static_only_resolves_guarded_takes_to_the_take() {
    let results = resolve_static_only(&grammars_dir().join("parser").join("GuardExpr.g4"));
    let shaped = results
        .iter()
        .find(|result| result.has_postfix_shape)
        .expect("the `pred?` decision");
    assert_eq!(shaped.guarded_takes, 0);
    assert!(shaped.is_fully_static());
    let table = shaped.table.as_ref().expect("the hybrid table is kept");
    assert_eq!(table.guarded(), 0);
    assert!(table.guard_danger.is_empty());
    assert!(table.guard_pass.is_empty());
    let resolved = shaped.static_only.as_ref().expect("resolved");
    assert!(
        resolved.min_alt >= 8,
        "the 8 guarded takes predict the take: {resolved:?}"
    );
    assert_no_runtime_escapes(&results);
}

/// Under `--static-dfa-only`, a hybrid table looks further than with the
/// adaptive fallback. Where the larger budget still stops, a state predicts its
/// minimum viable alternative.
#[test]
fn static_only_looks_further_and_guesses_the_minimum_viable_alternative() {
    let path = grammars_dir().join("parser").join("GuardExpr.g4");
    let adaptive = &classify(&path)[2];
    assert_eq!(adaptive.category, Category::HYBRID);
    let default_table = adaptive.table.as_ref().expect("the hybrid table");
    assert!(default_table.escapes() > 0);

    let results = resolve_static_only(&path);
    let decision = &results[2];
    let table = decision.table.as_ref().expect("the hybrid table is kept");
    assert!(
        table.states.len() > 4 * default_table.states.len(),
        "{} states with the larger budget, {} without",
        table.states.len(),
        default_table.states.len()
    );
    let resolved = decision.static_only.as_ref().expect("resolved");
    assert!(resolved.min_viable_alt > 0, "{resolved:?}");
    assert_eq!(resolved.syntax_error, 0);
    assert!(resolved.min_viable_alt_witness.is_some());
    assert!(
        decision
            .fallback_text()
            .ends_with(&format!("+{}v", resolved.min_viable_alt)),
        "{}",
        decision.fallback_text()
    );

    let guesses: Vec<_> = table
        .states
        .iter()
        .filter(|state| state.resolution == Some(super::EscapeResolution::MinViableAlt))
        .collect();
    assert_eq!(guesses.len(), resolved.min_viable_alt);
    for state in guesses {
        assert!(!state.predicated);
        assert!(state.conflicts.is_empty(), "a guess is not a conflict");
        assert!(state.viable_alts.len() >= 2, "{:?}", state.viable_alts);
        assert_eq!(state.accept, state.viable_alts[0]);
        assert!(state.continuation.is_none());
    }
    assert_no_runtime_escapes(&results);
}

#[test]
fn static_only_keeps_precedence_classes_static() {
    let results = resolve_static_only(&grammars_dir().join("parser").join("LrDfa.g4"));
    assert!(results.iter().any(|result| result.precedence.is_some()));
    for result in &results {
        assert!(
            result.static_only.is_some(),
            "decision {} ({}) resolves",
            result.decision,
            result.category
        );
        if let Some(precedence) = &result.precedence {
            assert_eq!(precedence.escapes, 0);
            assert!(
                precedence
                    .tables
                    .iter()
                    .flatten()
                    .all(|table| table.escapes() == 0)
            );
        }
    }
    assert_no_runtime_escapes(&results);
}

#[test]
fn static_only_rejects_decisions_without_a_static_prediction() {
    let mut results = classify_with(
        &fixture("parser-shapes").join("ParserShapes.g4"),
        AtnFallback::Forbidden,
    );
    let nongreedy = &mut results[3];
    assert_eq!(nongreedy.category, Category::NON_GREEDY);
    assert!(!nongreedy.resolve_static_only());
    assert!(nongreedy.static_only.is_none());

    let optional = &mut results[0];
    assert_eq!(optional.category, Category::LL1);
    assert!(optional.resolve_static_only(), "LL(1) needs no table");
    assert_eq!(optional.fallback_text(), "0");
}

/// Without the adaptive fallback, a partial table below the coverage threshold
/// is still better than no table: the classifier keeps it, and the resolution
/// turns its escapes into syntax errors.
#[test]
fn static_only_keeps_partial_tables_below_the_coverage_threshold() {
    let path = grammars_dir().join("parser").join("LowCoverage.g4");
    let adaptive = &classify(&path)[0];
    assert_eq!(adaptive.category, Category::PREDICATED);
    assert!(adaptive.table.is_none());

    let results = resolve_static_only(&path);
    let decision = &results[0];
    assert_eq!(decision.category, Category::HYBRID);
    assert!(
        decision.coverage_percent < super::classifier::HYBRID_MIN_COVERAGE_PERCENT,
        "coverage {}%",
        decision.coverage_percent
    );
    assert!(decision.table.is_some());
    let resolved = decision.static_only.as_ref().expect("resolved");
    assert_eq!((resolved.min_alt, resolved.syntax_error), (0, 10));
    assert_eq!(decision.fallback_text(), "10x");
    assert_no_runtime_escapes(&results);
}
