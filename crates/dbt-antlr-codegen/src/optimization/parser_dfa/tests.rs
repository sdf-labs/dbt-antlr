// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Classifier tests over the golden grammar corpus.
//!
//! The expected categories are the ones the reference `antlr4.base2` classifier
//! reports for the same grammars (`-Xdecision-report`).
//!
//! Large grammars diverge in a conservative direction: the reference
//! classifier resolves a conflict from the *first* path that reached a config
//! set, while this classifier keeps the union of every path's taint
//! (`INTERIOR_TAINT` marks the paths that never popped the decision boundary).
//! Mixed usage is never trusted here; on `fixtures/vscode-cpp14/CPP14.g4` that
//! costs six `EXACT_AMBIG` decisions, which land on hybrid tables instead.

use std::path::{Path, PathBuf};

use dbt_antlr_runtime::analysis::{BOUNDARY_TAINT, FOREIGN_LINEAGE_TAINT};

use crate::grammar::compiler::compile;
use crate::grammar::diagnostic::Severity;
use crate::grammar::loader::LoadOptions;
use crate::optimization::AtnFallback;

use super::cost::{AttemptKind, AttemptOutcome};
use super::loop_block::LoopBlock;
use super::table::{StaticDfaState, StaticDfaTable};
use super::{BudgetLimits, Category, DecisionClassifier, DecisionResult, ParserDfaBudgets};

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
            let results = classifier(parser).classify_all();
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
            let results = classifier(parser).with_visit_budget(20_000).classify_all();
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
    classifier(parser).with_visit_budget(budget).classify_all()
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
        &compilation.sources,
        AtnFallback::Allowed,
        false,
        &ParserDfaBudgets::default(),
    );
    let results = classify(path);
    (results, build.tables)
}

#[test]
fn table_image_loads_in_the_runtime() {
    let (results, tables) = build_tables(&grammars_dir().join("ReferenceToATN.g4"));
    assert!(tables.has_table(0), "decision 0 has a hybrid table");
    assert!(tables.defers(0), "the hybrid table keeps an adaptive path");

    let words = tables.words();
    let decoded = load_image(&words);
    assert_eq!(decoded.words(), words, "the image loads in place");
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
    // `binding`, the decision with the most frontier escapes.
    let hybrid = results
        .iter()
        .filter(|result| result.category == Category::HYBRID)
        .max_by_key(|result| result.escapes)
        .expect("FuzzExpr has a hybrid decision");
    let words = tables.words();
    let decoded = load_image(&words);
    assert_eq!(decoded.words(), words, "the image loads in place");

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
    let decoded = load_image(&words);
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
fn emitted_tables_validate_against_the_packed_atn() {
    use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
    use dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables;

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
        let build = crate::optimization::StaticParserDfa::build(
            parser,
            &compilation.sources,
            AtnFallback::Allowed,
            false,
            &ParserDfaBudgets::default(),
        );
        let packed = crate::grammar::atn::encode_packed_atn(parser.atn);
        let atn = PackedATNDeserializer::new()
            .deserialize(&packed)
            .expect("the packed ATN deserializes");
        let tables =
            StaticParserTables::from_image(Box::leak(build.tables.words().into_boxed_slice()))
                .expect("the emitted image loads");
        assert!(!tables.is_empty(), "{grammar}: the unit has tables");
        tables
            .validate_for_atn(&atn)
            .unwrap_or_else(|error| panic!("{grammar}: {error}"));

        let mut extended = packed.clone();
        extended.push(0);
        extended[25] += 1;
        assert!(
            PackedATNDeserializer::new().deserialize(&extended).is_err(),
            "{grammar}: words after the last packed section are rejected"
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
/// consumes an operator outside the loop (`'set' expr '||' INT EOF`), puts the
/// 32 iterate/exit conflict states under a stack guard. The 3 budget-frontier
/// states escape: their exit configs are widened, so their guard roots can be
/// in a frame that the widening removed.
#[test]
fn foreign_consumption_guards_the_iterate_reading() {
    let results = classify(&grammars_dir().join("parser").join("LoopFrontier.g4"));
    let result = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the arithmetic loop decision");
    assert_eq!(result.k, 4);
    assert_eq!(result.num_dfa_states, 88);
    assert_eq!(result.escapes, 3, "the frontier states escape");
    assert_eq!(result.guarded_iterates, 32);
    assert_eq!(result.iterate_accepts, 0);
    assert!(!result.is_fully_static(), "a guard may defer");
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
        "no iterate/exit conflict resolves to iterate outright"
    );
    let guarded: Vec<_> = states
        .iter()
        .copied()
        .filter(|state| state.is_guarded())
        .collect();
    assert_eq!(guarded.len(), 32);
    let conflict_guards = guarded
        .iter()
        .filter(|state| state.conflicts == [1, 2])
        .count();
    let frontier_guards = guarded
        .iter()
        .filter(|state| state.conflicts.is_empty())
        .count();
    assert_eq!((conflict_guards, frontier_guards), (32, 0));
    for state in guarded {
        assert_eq!(state.guarded_alt, 1, "a guarded iterate takes alt 1");
    }
    let table = classes.tables[0].as_ref().expect("class 0 table");
    assert!(!table.guard_danger().is_empty(), "the guard can trip");

    let between = classify(&grammars_dir().join("parser").join("BetweenExpr.g4"));
    let boolean_loop = &between[3];
    assert_eq!(boolean_loop.category, Category::LR_PRECEDENCE);
    assert_eq!(boolean_loop.escapes, 0);
    assert_eq!(boolean_loop.guarded_iterates, 2);
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
    let types = token_types(&path, &["'-'", "'+'", "'pair'", "ID"]);
    let (minus, plus, pair, id) = (types[0], types[1], types[2], types[3]);
    for (rule, tokens) in [
        ("expr", vec![minus, minus, id]),
        ("term", vec![minus, pair, minus, id]),
        ("cond", vec![minus, id]),
        ("tail", vec![minus, id, minus, id]),
        ("prec", vec![plus, id, plus, id]),
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
            ends.iter()
                .any(|state| state.is_escape() || state.is_guarded()),
            "{rule}: the iterate/exit conflict defers"
        );
        assert!(
            ends.iter().all(|state| state.accept != 1),
            "{rule}: no class table iterates on the path"
        );
    }
}

/// In `LoopObligation.g4`, the exit lineage enters the second `p` of
/// `'trim' '(' p p ')'` and owes `')'` at the lookahead budget. That obligation
/// belongs to a foreign invocation that `FOREIGN_CONSUME_TAINT` already roots,
/// so all 10 iterate/exit states (2 conflict states and 8 budget-frontier
/// states) resolve under the stack guard.
#[test]
fn foreign_rooted_obligations_guard_the_iterate_reading() {
    let path = grammars_dir().join("parser").join("LoopObligation.g4");
    let results = classify(&path);
    let result = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the loop decision of `p`");
    assert_eq!(result.k, 4);
    assert_eq!(result.num_dfa_states, 27);
    assert_eq!(result.escapes, 0);
    assert_eq!(result.guarded_iterates, 10);
    assert_eq!(result.iterate_accepts, 0);
    assert!(!result.is_fully_static());
    assert_eq!(result.fallback_text(), "10g");

    let classes = result.precedence.as_ref().expect("precedence tables");
    assert_eq!(classes.adaptive_classes, 0);
    let types = token_types(&path, &["'['", "':'", "'.'", "'prior'", "'trim'", "ID"]);
    let (open, colon, dot, prior, trim, id) =
        (types[0], types[1], types[2], types[3], types[4], types[5]);

    for table in &classes.tables[..2] {
        let table = table.as_ref().expect("hybrid class table");
        assert_eq!(table.guard_danger(), [9, 15]);
        assert_eq!(table.guard_pass, [40]);

        let conflict = walk(table, &[dot, id]).expect("`.` ID edge");
        assert!(conflict.is_guarded());
        assert_eq!(conflict.guarded_alt, 1);
        assert_eq!(conflict.conflicts, [1, 2]);

        for tail in [open, prior, trim, id] {
            let frontier = walk(table, &[open, id, colon, tail]).expect("frontier edge");
            assert!(frontier.is_guarded());
            assert_eq!(frontier.guarded_alt, 1);
            assert!(frontier.conflicts.is_empty());
        }
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
                table.guard_danger().contains(&first_call),
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
    assert!(!table.guard_danger().is_empty(), "the guard can trip");
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
    load_image(super::build_tables(results).words())
}

/// Overlays an emitted table image the way a generated parser does, from a
/// `'static` word array.
fn load_image(words: &[u32]) -> dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables {
    dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables::from_image(Box::leak(
        words.to_vec().into_boxed_slice(),
    ))
    .expect("the emitted image loads")
}

/// Every state of every runtime table accepts an alternative, continues the
/// walk, or is guarded with a defer alternative: no state defers to the
/// adaptive simulation.
fn assert_no_runtime_escapes(results: &[DecisionResult]) {
    use dbt_antlr_runtime::dfa::static_parser_dfa::{CONFLICT_ESCAPE, ESCAPE};

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
                    ![ESCAPE, CONFLICT_ESCAPE].contains(&accept),
                    "decision {} state {state} still escapes ({accept})",
                    result.decision
                );
                if let Some(guard) = table.guard(state) {
                    assert!(
                        guard.defer_alt > 0,
                        "decision {} state {state} defers to adaptivePredict ({guard:?})",
                        result.decision
                    );
                }
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
    let dead_end = super::tables::minimized(table)
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
fn static_only_keeps_the_guard_of_guarded_takes() {
    let results = resolve_static_only(&grammars_dir().join("parser").join("GuardExpr.g4"));
    let shaped = results
        .iter()
        .find(|result| result.has_postfix_shape)
        .expect("the `pred?` decision");
    assert!(shaped.guarded_takes > 0, "the guarded takes stay guarded");
    assert!(shaped.is_fully_static());
    let table = shaped.table.as_ref().expect("the hybrid table is kept");
    assert_eq!(table.guarded(), shaped.guarded_takes);
    assert!(!table.guard_groups.is_empty(), "the guard can trip");
    assert!(!table.guard_pass.is_empty(), "the walk can continue");
    for state in table.states.iter().filter(|state| state.is_guarded()) {
        assert_eq!(state.resolution, Some(super::EscapeResolution::Guarded));
        assert_eq!(state.guarded_alt, 1, "the take");
        assert_eq!(state.guard_defer_alt, 2, "the skip");
        assert!((state.guard_group as usize) < table.guard_groups.len());
    }
    let resolved = shaped.static_only.as_ref().expect("resolved");
    assert_eq!(resolved.guarded, shaped.guarded_takes);
    assert!(
        shaped
            .fallback_text()
            .starts_with(&format!("{}g", resolved.guarded)),
        "{}",
        shaped.fallback_text()
    );
    assert_no_runtime_escapes(&results);

    let decoded = decoded_tables(&results);
    let runtime = decoded
        .table(shaped.decision as i32, 0)
        .expect("the decision has a table");
    let guarded = table
        .states
        .iter()
        .position(|state| state.is_guarded())
        .expect("a guarded state") as i32;
    let guard = runtime
        .guard(guarded)
        .expect("the runtime state is guarded");
    assert_eq!((guard.alt, guard.defer_alt), (1, 2));
}

/// In `InlineAlias.g4`, the alias reading of `AS INT64 )` also reaches the
/// `)` of a parenthesis, so the iterate is not real. With the adaptive
/// fallback, the state escapes. Under `--static-dfa-only`, the guard keeps the
/// iterate and returns the exit where the `CAST` operand is on the stack. The
/// `AND` states guard only the `ROWS BETWEEN` operand.
#[test]
fn static_only_guards_iterates_that_consume_foreign_tokens() {
    let path = grammars_dir().join("parser").join("InlineAlias.g4");
    let hybrid = classify(&path);
    let hybrid_loop = hybrid
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the operator loop");
    assert!(hybrid_loop.escapes > 0, "the cast state escapes");

    let results = resolve_static_only(&path);
    let alias_loop = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the operator loop");
    let resolved = alias_loop.static_only.as_ref().expect("resolved");
    assert!(resolved.guarded > 0);
    assert_eq!(
        (
            resolved.min_alt,
            resolved.min_viable_alt,
            resolved.syntax_error
        ),
        (0, 0, 0),
        "every state of the loop is guarded: {resolved:?}"
    );
    assert_no_runtime_escapes(&results);

    let precedence = alias_loop.precedence.as_ref().expect("precedence tables");
    let table = precedence
        .tables
        .iter()
        .flatten()
        .max_by_key(|table| table.guarded())
        .expect("a class table");
    assert!(table.guard_groups.len() >= 2, "{:?}", table.guard_groups);
    let smallest = table
        .guard_groups
        .iter()
        .min_by_key(|group| group.len())
        .expect("a group");
    assert!(
        table
            .guard_groups
            .iter()
            .any(|group| group.len() > smallest.len()),
        "the `AND` states do not guard the `CAST` operand: {:?}",
        table.guard_groups
    );
}

/// Each guarded state tests only the roots of its own readings. In
/// `LoopObligation.g4`, the `.` state and the `[` states of the `p` loop
/// guard different invoking states, so they are in different guard groups.
#[test]
fn guarded_states_have_their_own_guard_groups() {
    let results = classify(&grammars_dir().join("parser").join("LoopObligation.g4"));
    let p = results
        .iter()
        .find(|result| result.category == Category::LR_PRECEDENCE)
        .expect("the `p` loop");
    let precedence = p.precedence.as_ref().expect("precedence tables");
    for table in precedence.tables.iter().flatten() {
        if table.guarded() == 0 {
            continue;
        }
        assert_eq!(table.guard_danger(), [9, 15]);
        assert_eq!(table.guard_groups.len(), 2, "{:?}", table.guard_groups);
        for state in table.states.iter().filter(|state| state.is_guarded()) {
            assert_eq!(state.guard_defer_alt, 2, "the exit");
            assert_eq!(
                table.guard_groups[state.guard_group as usize].len(),
                1,
                "a state guards one invoking state"
            );
        }
    }
}

/// Under `--static-dfa-only`, a hybrid table looks further than with the
/// adaptive fallback. Where the larger budget still stops, a state predicts its
/// minimum viable alternative.
#[test]
fn static_only_looks_further_and_guesses_the_minimum_viable_alternative() {
    let path = grammars_dir().join("parser").join("ParenExpr.g4");
    let adaptive = &classify(&path)[1];
    assert_eq!(adaptive.category, Category::HYBRID);
    let default_table = adaptive.table.as_ref().expect("the hybrid table");
    assert!(default_table.escapes() > 0);

    let results = resolve_static_only(&path);
    let decision = &results[1];
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

/// Classifies a grammar written to a scratch directory.
fn classify_source(name: &str, source: &str) -> Vec<DecisionResult> {
    let directory =
        std::env::temp_dir().join(format!("dbt-antlr-descent-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join(format!("{name}.g4"));
    std::fs::write(&path, source).expect("scratch grammar");
    let results = classify(&path);
    let _ = std::fs::remove_dir_all(&directory);
    results
}

/// Whether any decision of rule `rule_index` accepts with a shared-descent
/// group.
fn rule_has_groups(results: &[DecisionResult], rule_index: usize) -> bool {
    results
        .iter()
        .filter(|result| result.rule_index == rule_index)
        .any(|result| result.descent.is_some())
}

#[test]
fn shared_descent_groups_cure_the_escapes_of_the_fuzz_items() {
    let results = classify(&grammars_dir().join("parser/FuzzExpr.g4"));
    let path = grammars_dir().join("parser/FuzzExpr.g4");
    let rules = rule_names(&path, &results);
    let by_rule = |name: &str| {
        results
            .iter()
            .zip(&rules)
            .find(|(_, rule)| *rule == name)
            .map(|(result, _)| result)
            .expect("the rule has a decision")
    };

    // `item : e 'as' name | e`: one parse of `e`, then 'as' or anything else.
    let item = by_rule("item");
    assert_eq!(item.category, Category::EXACT_AMBIG);
    assert_eq!(item.escapes, 0);
    assert_eq!(item.descent_group_states, 15);
    let plan = item.descent.as_ref().expect("item accepts with a group");
    let [group] = &plan.groups[..] else {
        panic!("one group: {plan:?}");
    };
    assert_eq!(group.alts, [1, 2]);
    assert!(group.prefix.is_empty());
    assert_eq!(group.default_alt, 2);
    assert_eq!(group.arms.len(), 1);
    assert_eq!(group.arms[0].alt, 1);
    assert!(
        item.table
            .as_ref()
            .expect("item has a table")
            .states
            .iter()
            .any(|state| state.accept == group.virtual_alt)
    );

    // `alias`: 'as' can follow the rule, so the member follow sets at the
    // shared call site overlap on 'as' and the group is rejected.
    let alias = by_rule("alias");
    assert!(alias.descent.is_none());
    assert!(alias.escapes > 0);
    // `binding`: the members reach `e` through different rules.
    let binding = by_rule("binding");
    assert!(binding.descent.is_none());
    assert!(binding.escapes > 0);
}

#[test]
fn shared_descent_groups_share_a_one_token_prefix() {
    let results = classify(&grammars_dir().join("parser/ParenExpr.g4"));
    let primary = results
        .iter()
        .find(|result| result.descent.is_some())
        .expect("the primary block accepts with a group");
    assert_eq!(primary.descent_group_states, 8);
    assert_eq!(primary.escapes, 8);
    let group = &primary.descent.as_ref().unwrap().groups[0];
    assert_eq!(group.alts, [3, 4]);
    assert_eq!(group.prefix.len(), 1);
    assert_eq!(group.default_alt, 0);
    let arms: Vec<(i32, usize)> = group
        .arms
        .iter()
        .map(|arm| (arm.alt, arm.tokens.len()))
        .collect();
    assert_eq!(arms, [(3, 1), (4, 1)], "',' and ')'");
}

/// A grammar whose `x` decision needs unbounded lookahead between its two
/// alternatives, which share the rule `y` after `prefix`.
fn descent_witness(name: &str, x: &str, y: &str) -> String {
    format!(
        "grammar {name};\ns : x EOF ;\nx : {x} ;\ny : {y} ;\nID : [a-z]+ ;\nWS : [ \\t]+ -> skip ;\n"
    )
}

#[test]
fn shared_descent_accepts_a_plain_group() {
    let results = classify_source(
        "DescentPlain",
        &descent_witness("DescentPlain", "y '+' ID | y", "'(' y ')' | ID"),
    );
    assert!(rule_has_groups(&results, 1), "{results:?}");
    assert!(results.iter().all(|result| result.escapes == 0));
}

#[test]
fn shared_descent_rejects_a_rule_with_arguments() {
    let results = classify_source(
        "DescentArgs",
        &descent_witness("DescentArgs", "y[1] '+' ID | y[1]", "'(' y[1] ')' | ID")
            .replace("y : ", "y[i32 depth] : "),
    );
    assert!(!rule_has_groups(&results, 1));
}

#[test]
fn shared_descent_rejects_a_rule_that_runs_an_action() {
    let results = classify_source(
        "DescentAction",
        &descent_witness(
            "DescentAction",
            "y '+' ID | y",
            "'(' y ')' | ID {}  {let _ = 1;}",
        ),
    );
    assert!(!rule_has_groups(&results, 1));
}

#[test]
fn shared_descent_rejects_members_with_another_prefix_length() {
    // Alternative 1 reaches `y` after one or two tokens, so skipping one token
    // would not land on `y` for every input.
    let results = classify_source(
        "DescentPrefix",
        &descent_witness(
            "DescentPrefix",
            "('[' | '[' '[') y '+' ID | '[' y",
            "'(' y ')' | ID",
        ),
    );
    assert!(!rule_has_groups(&results, 1));
    // A multi-token set transition before `y` is not a single-token prefix.
    let results = classify_source(
        "DescentSetPrefix",
        &descent_witness(
            "DescentSetPrefix",
            "('[' | ']') y '+' ID | '[' y",
            "'(' y ')' | ID",
        ),
    );
    assert!(!rule_has_groups(&results, 1));
}

#[test]
fn shared_descent_skips_optional_blocks() {
    // Both alternatives of the optional block start with `y`, but the block is
    // emitted as an optional block, which has no group arms.
    let results = classify_source(
        "DescentOptional",
        &descent_witness("DescentOptional", "(y '+' ID)? y", "'(' y ')' | ID"),
    );
    assert!(results.iter().all(|result| result.descent.is_none()));
}

#[test]
fn shared_descent_rejects_decisions_and_loops_before_the_shared_rule() {
    // A loop around `y` calls it again after its first call returned.
    let results = classify_source(
        "DescentLoop",
        &descent_witness(
            "DescentLoop",
            "(y '+')+ ':' ID | y ';' ID",
            "'(' y ')' | ID",
        ),
    );
    assert!(!rule_has_groups(&results, 1));
    // The re-descent of alternative 1 would have to predict the optional block
    // before it reaches `y`.
    let results = classify_source(
        "DescentOptionalPrefix",
        &descent_witness(
            "DescentOptionalPrefix",
            "'['? y '+' ID | y ';' ID",
            "'(' y ')' | ID",
        ),
    );
    assert!(!rule_has_groups(&results, 1));
}

#[test]
fn shared_descent_excludes_rules_whose_shared_rule_conflicts_with_member_tails() {
    // In `DescentContext.g4`, `d`, `g`, and `k` call a shared rule (`r` or
    // `h`) that can both exit and continue on a token of the member tails
    // ('+' or 'as'), and `h`'s own members overlap on 'as' in FOLLOW(h).
    // Condition 2 excludes all of them at compile time.
    let results = classify(&grammars_dir().join("parser/DescentContext.g4"));
    assert!(
        results
            .iter()
            .all(|result| result.descent.is_none() && result.descent_group_states == 0),
        "{results:?}"
    );
    // Distinct subrules `a` and `b` inside `r` that match the same input `ID`,
    // after which `a` exits `r` while `b` continues with `'+' ID`.
    let results = classify_source(
        "DescentDistinctSubrules",
        concat!(
            "grammar DescentDistinctSubrules;\n",
            "s : d EOF ;\n",
            "d : r '+' ID ';' ID | r ';' ID INT ;\n",
            "r : '(' r ')' | b '+' ID | a ;\n",
            "a : '[' a ']' | ID ;\n",
            "b : '[' b ']' | ID ;\n",
            "ID : [a-z]+ ;\n",
            "INT : [0-9]+ ;\n",
            "WS : [ \\t\\n]+ -> skip ;\n",
        ),
    );
    assert!(
        results
            .iter()
            .all(|result| result.descent.is_none() && result.descent_group_states == 0),
        "{results:?}"
    );
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

/// A hybrid attempt that spends the visit budget after the start state keeps
/// a table: the states that it cannot expand escape. Only a start state that
/// spends the budget leaves the decision without a table.
#[test]
fn hybrid_attempts_escape_where_the_visit_budget_runs_out() {
    let path =
        fixture("testcompositegrammars-testimportlargegrammar-110006096d").join("NewJava.g4");
    let results = classify_with_budget(&path, 2_000);
    assert_eq!(results[7].category, Category::HYBRID);
    assert!(results[7].budget_cutoff);
    assert!(results[7].escapes > 0);
    assert_eq!(
        attempts_of(&results, 7),
        [
            AttemptKind::Unbounded { abort: true },
            AttemptKind::Widened { abort: true },
            AttemptKind::Hybrid,
        ]
    );

    let results = classify_with_budget(&path, 100);
    assert_eq!(results[7].category, Category::OVERFLOW);
    assert!(results[7].table.is_none());
}

/// Each precedence class keeps a hybrid table when its construction spends
/// the visit budget after the start state.
#[test]
fn precedence_classes_keep_tables_where_the_visit_budget_runs_out() {
    let path = grammars_dir().join("parser").join("LoopFrontier.g4");
    let results = classify_with_budget(&path, 2_000);
    let precedence = results[4].precedence.as_ref().expect("precedence tables");
    assert_eq!(precedence.adaptive_classes, 0);
    assert_eq!(precedence.tables.len(), 5);
    assert!(precedence.tables.iter().all(Option::is_some));
    assert!(results[4].budget_cutoff);
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

/// The results of a parallel classification do not depend on the thread
/// count.
#[test]
fn parallel_classification_matches_serial_classification() {
    let mut grammars = Vec::new();
    collect_grammars(&grammars_dir(), &mut grammars);
    grammars.push(fixture("vscode-cpp14/CPP14.g4"));
    grammars.sort();
    let mut decisions = 0_usize;
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
            for atn_fallback in [AtnFallback::Allowed, AtnFallback::Forbidden] {
                let classify_on = |threads| {
                    classifier(parser)
                        .with_atn_fallback(atn_fallback)
                        .with_visit_budget(5_000)
                        .classify_on(threads)
                };
                // The parallel run comes first, so that it fills the caches of
                // the ATN states from more than one thread.
                let parallel = classify_on(8);
                let serial = classify_on(1);
                decisions += serial.len();
                assert_eq!(serial, parallel, "{}", path.display());
            }
        }
    }
    assert!(decisions > 800, "decisions classified: {decisions}");
}

/// Budget settings with the global visit budget `budget`.
fn visit_budget(budget: u64) -> ParserDfaBudgets {
    ParserDfaBudgets {
        global: BudgetLimits {
            max_visits: Some(budget),
            ..BudgetLimits::default()
        },
    }
}

/// Builds the pass artifact of the first root parser of one grammar under a
/// visit budget.
fn build_with_budget(
    path: &Path,
    budget: u64,
) -> crate::optimization::static_parser_dfa::StaticParserDfaArtifact {
    let compilation = compile(LoadOptions {
        roots: vec![path.to_path_buf()],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let parser = compilation
        .parser(compilation.roots[0].parser.expect("parser root"))
        .expect("parser artifact");
    crate::optimization::StaticParserDfa::build(
        parser,
        &compilation.sources,
        AtnFallback::Allowed,
        false,
        &visit_budget(budget),
    )
}

/// A decision with an attempt that spends the visit budget gets one `G4S193`
/// warning, with the counts of the attempts and the most visited rules.
#[test]
fn spent_visit_budgets_emit_g4s193_warnings() {
    let build = build_with_budget(&grammars_dir().join("Perf.g4"), 100);
    let warnings: Vec<_> = build
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "G4S193")
        .map(|diagnostic| {
            assert_eq!(diagnostic.severity, Severity::Warning);
            diagnostic.message.as_str()
        })
        .collect();
    assert_eq!(
        warnings,
        [
            "the analysis of parser decision 0 of rule 'stat' is slow: 2 of 3 build attempts \
             spent the visit budget (100 configs); 216 configs visited, 11 DFA states built; \
             most visited rules: expr 95%, stat 4%",
            "the analysis of parser decision 3 of rule 'expr' is slow: 6 of 10 build attempts \
             spent the visit budget (100 configs); 848 configs visited, 34 DFA states built; \
             most visited rules: expr 89%, stat 10%",
        ]
    );

    let build = build_with_budget(&grammars_dir().join("Perf.g4"), super::DEFAULT_VISIT_BUDGET);
    assert!(
        build
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "G4S193"),
        "{:?}",
        build.diagnostics
    );
}

/// The cost of a decision adds up: each attempt visits at most the budget,
/// and the visits by rule sum to the visits of the attempts.
#[test]
fn decision_costs_account_for_every_visit() {
    for budget in [100, super::DEFAULT_VISIT_BUDGET] {
        let results = classify_with_budget(&grammars_dir().join("Perf.g4"), budget);
        for result in &results {
            let cost = &result.cost;
            assert!(cost.elapsed.is_none(), "no times without profiling");
            for attempt in &cost.attempts {
                assert!(attempt.visits <= budget, "{attempt:?}");
                assert!(attempt.elapsed.is_none());
                if attempt.outcome == AttemptOutcome::VisitBudget {
                    assert_eq!(attempt.visits, budget, "{attempt:?}");
                }
            }
            let by_rule: u64 = cost.rule_visits.iter().map(|(_, visits)| visits).sum();
            assert_eq!(by_rule, cost.visits(), "decision {}", result.decision);
        }
        assert_eq!(attempts_of(&results, 1), []);
        assert_eq!(
            attempts_of(&results, 2),
            [AttemptKind::Unbounded { abort: true }]
        );
    }
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

/// With profiling, every decision and every attempt carries its time, and the
/// profile lists the 20 slowest decisions.
#[test]
fn profiled_builds_time_every_decision_and_list_the_slowest() {
    let directory = std::env::temp_dir().join(format!("dbt-antlr-profile-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join("Many.g4");
    let mut source = String::from("grammar Many;\n");
    for rule in 0..25 {
        source.push_str(&format!("r{rule} : A B | A C ;\n"));
    }
    source.push_str("A : 'a' ; B : 'b' ; C : 'c' ;\n");
    std::fs::write(&path, source).expect("scratch grammar");
    let compilation = compile(LoadOptions {
        roots: vec![path],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let _ = std::fs::remove_dir_all(&directory);
    let parser = compilation
        .parser(compilation.roots[0].parser.expect("parser root"))
        .expect("parser artifact");
    let build = crate::optimization::StaticParserDfa::build(
        parser,
        &compilation.sources,
        AtnFallback::Allowed,
        true,
        &ParserDfaBudgets::default(),
    );

    assert_eq!(build.report.decisions.len(), 25);
    for decision in &build.report.decisions {
        let cost = &decision.result.cost;
        assert!(cost.elapsed.is_some(), "{cost:?}");
        assert_eq!(cost.attempts.len(), 1, "{cost:?}");
        assert!(
            cost.attempts
                .iter()
                .all(|attempt| attempt.elapsed.is_some())
        );
    }
    let profile = crate::optimization::StaticParserDfa::render_decision_profile(&[build.report]);
    let lines: Vec<&str> = profile.lines().collect();
    let slowest = lines
        .iter()
        .position(|line| *line == "slowest 20 decisions:")
        .unwrap_or_else(|| panic!("{profile}"));
    assert!(
        lines[slowest + 1].trim_start().starts_with("#  rule"),
        "{profile}"
    );
    let rows = lines[slowest + 2..]
        .iter()
        .filter(|line| !line.is_empty())
        .count();
    assert_eq!(rows, 20, "{profile}");
}

/// Labeled single-token alternatives keep one branch per token, and all
/// branches end at the block end. Their token classes share one closure, so
/// each extra token adds only a few visits.
#[test]
fn single_token_alternatives_share_one_successor() {
    fn keywords(count: usize) -> DecisionResult {
        let mut source = format!("grammar Keywords{count};\ns : word X | word Y ;\nword :");
        for keyword in 0..count {
            source.push_str(&format!(" K{keyword} #k{keyword} |"));
        }
        source.push_str(" L K0 R #nested ;\nX : 'x' ; Y : 'y' ; L : '(' ; R : ')' ;\n");
        for keyword in 0..count {
            source.push_str(&format!("K{keyword} : 'k{keyword}' ;\n"));
        }
        classify_source(&format!("Keywords{count}"), &source).remove(0)
    }

    let few = keywords(2);
    let many = keywords(200);
    assert_eq!(few.category, Category::LLK);
    assert_eq!(many.category, few.category);
    assert_eq!(many.k, few.k);
    assert_eq!(many.num_dfa_states, few.num_dfa_states);
    let start = &many.table.as_ref().expect("LL(k) table").states[0];
    let covered: i32 = start.edges.iter().map(|edge| edge.hi - edge.lo + 1).sum();
    assert_eq!(
        covered, 201,
        "each keyword and L lead out of the start state"
    );
    let extra = many.cost.visits() - few.cost.visits();
    assert!(extra <= 3 * 198, "{extra} extra visits");
}

/// Builds the pass artifact of a grammar written to a scratch directory.
fn build_source(
    name: &str,
    source: &str,
) -> crate::optimization::static_parser_dfa::StaticParserDfaArtifact {
    let directory =
        std::env::temp_dir().join(format!("dbt-antlr-build-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join(format!("{name}.g4"));
    std::fs::write(&path, source).expect("scratch grammar");
    let build = build_with_budget(&path, super::DEFAULT_VISIT_BUDGET);
    let _ = std::fs::remove_dir_all(&directory);
    build
}

/// A grammar whose rule `word` has `alternatives` as its block, with the
/// parser rules `rules`, 70 keyword tokens `K0` to `K69`, and `callers` rules
/// that call `word`.
fn keyword_grammar(name: &str, alternatives: &str, rules: &str, callers: usize) -> String {
    let mut source = format!("grammar {name};\nword : {alternatives} ;\n{rules}");
    for caller in 0..callers {
        source.push_str(&format!("r{caller} : word X ;\n"));
    }
    source.push_str("X : 'x' ; L : '(' ; R : ')' ;\n");
    for keyword in 0..70 {
        source.push_str(&format!("K{keyword} : 'k{keyword}' ;\n"));
    }
    source
}

/// The `G4S194` warnings of a keyword grammar, with their spans as
/// `line:column`.
fn token_branch_warnings(
    name: &str,
    alternatives: &str,
    rules: &str,
    callers: usize,
) -> Vec<(String, String)> {
    let source = keyword_grammar(name, alternatives, rules, callers);
    build_source(name, &source)
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "G4S194")
        .map(|diagnostic| {
            assert_eq!(diagnostic.severity, Severity::Warning);
            let offset = diagnostic.primary.bytes.start as usize;
            let line = source[..offset].matches('\n').count() + 1;
            let column = offset
                - source[..offset]
                    .rfind('\n')
                    .map_or(0, |newline| newline + 1);
            (format!("{line}:{column}"), diagnostic.message.clone())
        })
        .collect()
}

/// The causes and the entry count of one `G4S194` message.
fn token_branch_causes(message: &str) -> &str {
    let start = message.find("because ").expect("causes") + "because ".len();
    let end = message.find(" before they read").expect("entries");
    &message[start..end]
}

/// A block of many single-token alternatives that do not fold into one token
/// set gets one `G4S194` warning when many call sites enter its rule. The
/// warning tells why the block did not fold, and it stops when the tokens are
/// in one subrule.
#[test]
fn unfolded_token_branches_emit_g4s194_warnings() {
    let keywords = (0..70)
        .map(|keyword| format!("K{keyword}"))
        .collect::<Vec<_>>();
    let mixed = format!("{} | L word R", keywords.join(" | "));
    assert_eq!(
        token_branch_warnings("Mixed", &mixed, "", 20),
        [(
            "2:7".to_owned(),
            "rule 'word' has a block with 70 single-token alternatives that do not fold into one \
             token set. The block does not fold because 1 alternative is not a single token. 21 \
             call sites enter the rule before they read a token. The analysis of the static \
             parser DFA follows each alternative at each site. Move the single tokens into one \
             unlabeled subrule, for example `( A | B | C )`, or into a token-only rule"
                .to_owned()
        )]
    );

    let causes = |name: &str, alternatives: &str, rules: &str, callers: usize| {
        token_branch_warnings(name, alternatives, rules, callers)
            .iter()
            .map(|(span, message)| format!("{span} {}", token_branch_causes(message)))
            .collect::<Vec<_>>()
    };
    let labeled = keywords
        .iter()
        .map(|keyword| format!("{keyword} #{}", keyword.to_lowercase()))
        .collect::<Vec<_>>()
        .join(" | ");
    assert_eq!(
        causes("Labeled", &labeled, "", 20),
        ["2:7 70 alternatives have a label. 20 call sites enter the rule"]
    );
    assert_eq!(causes("Callers", &labeled, "", 19), [] as [String; 0]);
    let element_labels = keywords
        .iter()
        .map(|keyword| format!("k={keyword}"))
        .collect::<Vec<_>>()
        .join(" | ");
    assert_eq!(
        causes("ElementLabels", &format!("{element_labels} | L"), "", 20),
        ["2:7 70 tokens have an element label. 20 call sites enter the rule"]
    );

    let threshold = format!("{} | L word R", keywords[..64].join(" | "));
    assert_eq!(causes("Threshold", &threshold, "", 20).len(), 1);
    let few = format!("{} | L word R", keywords[..63].join(" | "));
    assert_eq!(causes("Few", &few, "", 20), [] as [String; 0]);
    assert_eq!(
        causes("Folded", &keywords.join(" | "), "", 20),
        [] as [String; 0]
    );
    let grouped = format!("( {} ) | L word R", keywords.join(" | "));
    assert_eq!(causes("Grouped", &grouped, "", 20), [] as [String; 0]);
    let late = format!("L ( {mixed} ) | X");
    assert_eq!(causes("Late", &late, "", 20), [] as [String; 0]);

    // `inner` is a left corner of `word` behind the nullable rule `opt`.
    let rules = format!("opt : L | ;\ninner : {mixed} ;\n");
    assert_eq!(
        causes("Inner", "opt inner X | L", &rules, 20),
        ["4:8 1 alternative is not a single token. 22 call sites enter the rule"]
    );
    // Only the second nested block comes before a token.
    let nested = format!("X ( {mixed} )? | ( {mixed} ) X");
    let column = "word : ".len() + nested.rfind("( ").expect("second block");
    assert_eq!(
        causes("Nested", &nested, "", 20),
        [format!(
            "2:{column} 1 alternative is not a single token. 22 call sites enter the rule"
        )]
    );
    // A nullable subrule or a call of a nullable rule does not read a token.
    for (name, prefix) in [("NullableBlock", "( L | )"), ("NullableCall", "opt")] {
        let behind = format!("{prefix} ( {mixed} ) X");
        let column = "word : ".len() + behind.rfind("( K0").expect("block");
        assert_eq!(
            causes(name, &behind, "opt : L | ;\n", 20),
            [format!(
                "2:{column} 1 alternative is not a single token. 21 call sites enter the rule"
            )]
        );
    }
    let recursive = format!("{} | word L word", keywords.join(" | "));
    assert_eq!(
        causes("Recursive", &recursive, "", 20),
        ["2:7 1 alternative is not a single token. 21 call sites enter the rule"]
    );
}

/// The compiler diagnostics and the pass artifact of a grammar written to a
/// scratch directory, under `budgets`.
fn build_source_with(
    name: &str,
    source: &str,
    atn_fallback: AtnFallback,
    budgets: &ParserDfaBudgets,
) -> (
    Vec<crate::grammar::diagnostic::Diagnostic>,
    crate::optimization::static_parser_dfa::StaticParserDfaArtifact,
) {
    static SCRATCH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let scratch = SCRATCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "dbt-antlr-budget-{}-{scratch}-{name}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join(format!("{name}.g4"));
    std::fs::write(&path, source).expect("scratch grammar");
    let compilation = compile(LoadOptions {
        roots: vec![path],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let _ = std::fs::remove_dir_all(&directory);
    let parser = compilation
        .parser(compilation.roots[0].parser.expect("parser root"))
        .expect("parser artifact");
    let build = crate::optimization::StaticParserDfa::build(
        parser,
        &compilation.sources,
        atn_fallback,
        false,
        budgets,
    );
    (compilation.diagnostics.clone(), build)
}

/// The budgets of the decisions of rule `rule`, in decision order.
fn rule_budgets(
    build: &crate::optimization::static_parser_dfa::StaticParserDfaArtifact,
    rule: &str,
) -> Vec<super::DecisionBudget> {
    build
        .report
        .decisions
        .iter()
        .filter(|decision| decision.rule == rule)
        .map(|decision| decision.result.budget)
        .collect()
}

const BUDGET_GRAMMAR: &str = "grammar Budgets;
options { max_dfa_states = 7; }
s : a b EOF ;
a options { max_k = 3; } : A B | A C ;
b : ( options { max_dfa_visits = 5000; } : X Y | X Z ) | W ;
A : 'a' ; B : 'b' ; C : 'c' ; X : 'x' ; Y : 'y' ; Z : 'z' ; W : 'w' ;
";

/// The grammar, rule, and block options set the budgets of the decisions, a
/// block budget wins over a rule budget, which wins over a grammar budget, and
/// each limit resolves on its own.
#[test]
fn budget_options_set_the_budgets_of_their_decisions() {
    let (diagnostics, build) = build_source_with(
        "Budgets",
        BUDGET_GRAMMAR,
        AtnFallback::Allowed,
        &ParserDfaBudgets::default(),
    );
    assert!(
        diagnostics
            .iter()
            .all(|diagnostic| !matches!(diagnostic.code, "G4S014" | "G4S015")),
        "{diagnostics:?}"
    );
    let budget = |max_states, max_k, max_visits| super::DecisionBudget {
        max_states,
        max_k,
        max_visits,
        explicit_max_states: true,
    };
    assert_eq!(rule_budgets(&build, "a"), [budget(7, Some(3), 2_000_000)]);
    assert_eq!(
        rule_budgets(&build, "b"),
        [budget(7, None, 5_000), budget(7, None, 2_000_000)]
    );
    assert_eq!(build.report.budget, budget(7, None, 2_000_000));
}

/// The base limits apply only where no option sets a limit.
#[test]
fn budget_options_win_over_the_base_limits() {
    let budgets = ParserDfaBudgets {
        global: BudgetLimits {
            max_states: Some(50),
            max_k: Some(2),
            max_visits: Some(9_000),
        },
    };
    let (_, build) = build_source_with("Budgets", BUDGET_GRAMMAR, AtnFallback::Allowed, &budgets);
    let budget = |max_states, max_k, max_visits| super::DecisionBudget {
        max_states,
        max_k,
        max_visits,
        explicit_max_states: true,
    };
    assert_eq!(rule_budgets(&build, "a"), [budget(7, Some(3), 9_000)]);
    assert_eq!(
        rule_budgets(&build, "b"),
        [budget(7, Some(2), 5_000), budget(7, Some(2), 9_000)]
    );
}

/// A budget option with a value that is not a positive integer gets a
/// `G4S015` warning, and the pass ignores it.
#[test]
fn incorrect_budget_option_values_emit_g4s015_warnings() {
    let source = "grammar Wrong;
options { max_dfa_states = 0; }
s options { max_k = many; } : A B | A C ;
A : 'a' ; B : 'b' ; C : 'c' ;
";
    let (diagnostics, build) = build_source_with(
        "Wrong",
        source,
        AtnFallback::Allowed,
        &ParserDfaBudgets::default(),
    );
    let warnings: Vec<_> = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code == "G4S015")
        .map(|diagnostic| diagnostic.message.as_str())
        .collect();
    assert_eq!(
        warnings,
        [
            "unsupported option value max_dfa_states=0: the value must be at least 1",
            "unsupported option value max_k=many: 'many' is not a positive integer",
        ]
    );
    assert_eq!(
        rule_budgets(&build, "s"),
        [super::DecisionBudget::default()]
    );
}

const FANOUT_GRAMMAR: &str = "grammar Fanout;
s : a EOF ;
a : A B X | A C X | A D X | A B Y | E ;
A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ; X : 'x' ; Y : 'y' ;
";

/// A hybrid table escapes the state whose expansion would run past the state
/// cap, also below depth 2, instead of losing the whole table. Under
/// `--static-dfa-only`, the escape is a guess and not an error.
#[test]
fn state_caps_cut_hybrid_tables_instead_of_failing_them() {
    let budgets = ParserDfaBudgets {
        global: BudgetLimits {
            max_states: Some(4),
            ..BudgetLimits::default()
        },
    };
    let (_, build) = build_source_with("Fanout", FANOUT_GRAMMAR, AtnFallback::Allowed, &budgets);
    let decision = &build.report.decisions[0];
    assert_eq!(decision.rule, "a");
    assert_eq!(decision.result.category, Category::HYBRID);
    assert!(decision.result.budget_cutoff);
    assert_eq!(decision.result.num_dfa_states, 3);

    let (_, build) = build_source_with("Fanout", FANOUT_GRAMMAR, AtnFallback::Forbidden, &budgets);
    let messages: Vec<_> = build
        .diagnostics
        .iter()
        .map(|diagnostic| (diagnostic.code, diagnostic.message.as_str()))
        .collect();
    assert_eq!(
        messages,
        [(
            "G4S192",
            "parser decision 0 of rule 'a' resolves its escape states without adaptivePredict: \
             1 undecided state predicts the minimum viable alternative (witness: 'a'); \
             tie-breaks: 1 state chooses alt 1 (A B X, line 3) over alt 2 (A C X, line 3), \
             alt 3 (A D X, line 3), alt 4 (A B Y, line 3); the analysis budget (4 states or \
             2000000 visited configs) stopped the lookahead; raise it with the max_dfa_states \
             and max_dfa_visits options of the grammar, the rule, or the block"
        )]
    );

    let (_, build) = build_source_with(
        "Fanout",
        FANOUT_GRAMMAR,
        AtnFallback::Forbidden,
        &ParserDfaBudgets::default(),
    );
    assert!(build.diagnostics.is_empty(), "{:?}", build.diagnostics);
}

const SPLIT_OPERATOR_GRAMMAR: &str = "grammar Split;
s : e EOF ;
e : e '+' 'q' 'a' | e '*' e | e '+' 'q' 'b' | 'x' ;
";

/// The accept of the state that `tokens` reach from the start of `table`.
fn accept_after(table: &StaticDfaTable, tokens: &[i32]) -> Option<i32> {
    let mut state = 0;
    for token in tokens {
        state = table.states[state]
            .edges
            .iter()
            .find(|edge| edge.lo <= *token && *token <= edge.hi)?
            .target;
    }
    Some(table.states[state].accept)
}

/// A frontier state where a multi-token operator has not yet separated the
/// alternatives does not resolve to iterate. In the class of the right operand
/// of `*`, only the first `+` operator iterates, and the exit lineage enters
/// the second `+` operator again: after `+ q`, the input can still be `b`.
#[test]
fn frontier_iterates_need_every_exit_position_in_the_iterate_lineage() {
    let accepts = split_operator_accepts("Split", SPLIT_OPERATOR_GRAMMAR, 4);
    assert_eq!(accepts.len(), 4, "the cutoffs are 2, 3, and 4");
    // The class `p=4` escapes after `+ q` instead of predicting the iterate.
    assert_eq!(accepts[2], Some(super::ESCAPE), "{accepts:?}");
}

/// The accepts after `+ q` in each precedence class, for a grammar built with
/// the state cap `max_states`.
fn split_operator_accepts(name: &str, source: &str, max_states: usize) -> Vec<Option<i32>> {
    let budgets = ParserDfaBudgets {
        global: BudgetLimits {
            max_states: Some(max_states),
            ..BudgetLimits::default()
        },
    };
    let (_, build) = build_source_with(name, source, AtnFallback::Allowed, &budgets);
    let token = |name: &str| {
        build
            .report
            .vocabulary
            .iter()
            .position(|display| display == name)
            .map(|index| index as i32 + 1)
            .expect("token in vocabulary")
    };
    let path = [token("'+'"), token("'q'")];
    let decision = build
        .report
        .decisions
        .iter()
        .find(|decision| decision.result.category == Category::LR_PRECEDENCE)
        .expect("a precedence loop decision");
    let precedence = decision.result.precedence.as_ref().expect("classes");
    precedence
        .tables
        .iter()
        .map(|table| table.as_ref().and_then(|table| accept_after(table, &path)))
        .collect()
}

/// A foreign exit config does not make the frontier state a guard candidate
/// when another exit config of the state is not in the iterate lineage. The
/// stack guard only sees the roots of the foreign lineage, so it would predict
/// the iterate on `x * x + q b`.
#[test]
fn frontier_iterate_guards_need_every_loop_exit_in_the_iterate_lineage() {
    let source = "grammar SplitGuard;
s : e EOF | 'between' e '+' 'q' 'z' ;
e : e '+' 'q' 'a' | e '*' e | e '+' 'q' 'b' | 'x' ;
";
    let accepts = split_operator_accepts("SplitGuard", source, 4);
    assert_eq!(accepts[2], Some(super::ESCAPE), "{accepts:?}");
}

/// The decisions inside a block get the budget options of the blocks around
/// them, and an inner block wins over an outer block. Only the budget option
/// names set a budget: `k` is not one of them.
#[test]
fn nested_blocks_inherit_the_budget_options_of_outer_blocks() {
    let source = "grammar Nested;
options { k = 2; }
s : ( options { max_k = 5; max_dfa_visits = 7000; } :
      ( options { max_k = 6; } : A B | A C ) D
    | E
    ) EOF ;
A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ;
";
    let (diagnostics, build) = build_source_with(
        "Nested",
        source,
        AtnFallback::Allowed,
        &ParserDfaBudgets::default(),
    );
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "G4S014"
                && diagnostic.message == "unsupported option k"),
        "{diagnostics:?}"
    );
    let limits: Vec<_> = rule_budgets(&build, "s")
        .iter()
        .map(|budget| (budget.max_k, budget.max_visits))
        .collect();
    assert_eq!(limits, [(Some(6), 7_000), (Some(5), 7_000)]);
}

/// A lexer grammar or a lexer rule has no parser decisions, so the budget
/// options are not supported there.
#[test]
fn lexer_grammars_do_not_support_budget_options() {
    let directory =
        std::env::temp_dir().join(format!("dbt-antlr-budget-lexer-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join("Words.g4");
    std::fs::write(
        &path,
        "lexer grammar Words;\noptions { max_k = 2; }\nA options { max_k = 3; } : 'a' | 'b' ;\n",
    )
    .expect("scratch grammar");
    let compilation = compile(LoadOptions {
        roots: vec![path],
        library_directories: Vec::new(),
    })
    .expect("grammar compiles");
    let _ = std::fs::remove_dir_all(&directory);
    let unsupported = compilation
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.message == "unsupported option max_k")
        .count();
    assert_eq!(unsupported, 2, "{:?}", compilation.diagnostics);
}

/// The accepts of one alternative that have a single viable alternative share
/// one state, so they use one state of the state cap.
#[test]
fn single_alternative_accepts_share_one_state() {
    let source = "grammar Sinks;
s : a EOF ;
a : ( A B | C D | E F ) X | A G Y ;
A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ; F : 'f' ; G : 'g' ; X : 'x' ; Y : 'y' ;
";
    let results = classify_source("Sinks", source);
    let decision = results
        .iter()
        .find(|result| result.table.is_some() && result.k == 2)
        .expect("the LL(2) decision of `a`");
    let table = decision.table.as_ref().expect("table");
    let accepts: Vec<_> = table
        .states
        .iter()
        .filter(|state| state.is_accept())
        .map(|state| state.accept)
        .collect();
    assert_eq!(accepts, [1, 2], "{table:?}");
    let first_alt = table
        .states
        .iter()
        .position(|state| state.accept == 1)
        .expect("an accept of alternative 1");
    let into_first_alt = table
        .states
        .iter()
        .flat_map(|state| &state.edges)
        .filter(|edge| edge.target == first_alt)
        .count();
    assert_eq!(into_first_alt, 3, "`a b`, `c`, and `e` lead to one state");
}

/// A predicated path and a path without a predicate can reach the same config
/// set. The shared accept state has its own config set, so neither path takes
/// the verdict of the other, in each order of the token types.
#[test]
fn shared_accept_states_do_not_absorb_predicated_paths() {
    for order in ["Y W X", "X Y W", "X W Y", "W Y X"] {
        let mut source = String::from(
            "grammar Order;\ns : a EOF ;\na : P ( Y | W Q | X {false}? ) B | P Z ;\nP : 'p' ;\n",
        );
        for token in order.split(' ') {
            source.push_str(&format!("{token} : '{}' ;\n", token.to_lowercase()));
        }
        source.push_str("Q : 'q' ; B : 'b' ; Z : 'z' ;\n");
        let results = resolve_static_only_source("Order", &source);
        let decision = results
            .iter()
            .find(|result| result.table.is_some() && result.k >= 2)
            .unwrap_or_else(|| panic!("{order}: the decision of `a`"));
        let table = decision.table.as_ref().expect("table");
        let token = |name: char| {
            2 + order
                .split(' ')
                .position(|token| token.starts_with(name))
                .expect("token") as i32
        };
        let after = |second: i32| accept_after(table, &[1, second]);
        assert_eq!(after(token('Y')), Some(1), "{order}: `p y` predicts 1");
        assert_eq!(after(token('W')), Some(1), "{order}: `p w` predicts 1");
        let x = table.states[0]
            .edges
            .iter()
            .find(|edge| edge.lo <= 1 && 1 <= edge.hi)
            .map(|edge| &table.states[edge.target])
            .and_then(|state| {
                state
                    .edges
                    .iter()
                    .find(|edge| edge.lo <= token('X') && token('X') <= edge.hi)
            })
            .map(|edge| &table.states[edge.target])
            .expect("an edge for `p x`");
        assert_eq!(
            x.resolution,
            Some(super::EscapeResolution::SyntaxError),
            "{order}: `p x` stops at the predicate"
        );
    }
}

/// Classifies and resolves for `--static-dfa-only` a grammar written to a
/// scratch directory.
fn resolve_static_only_source(name: &str, source: &str) -> Vec<DecisionResult> {
    static SCRATCH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let scratch = SCRATCH.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let directory = std::env::temp_dir().join(format!(
        "dbt-antlr-static-only-{}-{scratch}-{name}",
        std::process::id()
    ));
    std::fs::create_dir_all(&directory).expect("scratch directory");
    let path = directory.join(format!("{name}.g4"));
    std::fs::write(&path, source).expect("scratch grammar");
    let results = resolve_static_only(&path);
    let _ = std::fs::remove_dir_all(&directory);
    results
}

/// Walks a table and its runtime minimization in lockstep from their start
/// states, and checks that the runtime cannot distinguish them.
fn assert_runtime_equivalent(table: &StaticDfaTable, minimized: &StaticDfaTable) {
    let outcome = |state: &StaticDfaState| {
        let accept = if state.accept == super::CONFLICT_ESCAPE && state.conflict_min_alt <= 0 {
            super::ESCAPE
        } else {
            state.accept
        };
        (
            accept,
            if accept == 0 { state.fallback } else { 0 },
            if accept == super::CONFLICT_ESCAPE {
                state.conflict_min_alt
            } else {
                0
            },
            state.guarded_alt,
            if state.resolution == Some(super::EscapeResolution::Guarded) {
                state.guard_defer_alt
            } else {
                0
            },
            state.guard_group,
            state
                .continuation
                .as_ref()
                .filter(|_| accept == super::ESCAPE)
                .map(|continuation| (continuation.contexts.clone(), continuation.configs.clone())),
        )
    };
    let target = |state: &StaticDfaState, token: i32| {
        state
            .edges
            .iter()
            .find(|edge| edge.lo <= token && token <= edge.hi)
            .map(|edge| edge.target)
    };
    let mut tokens: Vec<i32> = table
        .states
        .iter()
        .chain(&minimized.states)
        .flat_map(|state| &state.edges)
        .flat_map(|edge| [edge.lo - 1, edge.lo, edge.hi, edge.hi + 1])
        .collect();
    tokens.sort_unstable();
    tokens.dedup();
    let mut seen = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::from([(0_usize, 0_usize)]);
    while let Some((left, right)) = queue.pop_front() {
        if !seen.insert((left, right)) {
            continue;
        }
        let (left_state, right_state) = (&table.states[left], &minimized.states[right]);
        assert_eq!(
            outcome(left_state),
            outcome(right_state),
            "states {left} and {right}"
        );
        if left_state.accept != 0 {
            continue;
        }
        for &token in &tokens {
            match (target(left_state, token), target(right_state, token)) {
                (Some(left), Some(right)) => queue.push_back((left, right)),
                (None, None) => {}
                pair => panic!("token {token} from {left}/{right}: {pair:?}"),
            }
        }
    }
}

/// The runtime table of a decision is its minimization, which the runtime
/// cannot distinguish from the built table. The image of the minimized tables
/// loads and validates.
#[test]
fn runtime_tables_are_minimized_and_equivalent() {
    let mut merged = 0;
    for (grammar, fallback) in [
        ("parser/FuzzExpr.g4", AtnFallback::Allowed),
        ("parser/LoopFrontier.g4", AtnFallback::Allowed),
        ("parser/GuardExpr.g4", AtnFallback::Forbidden),
        ("parser/InlineAlias.g4", AtnFallback::Forbidden),
    ] {
        let path = grammars_dir().join(grammar);
        let results = match fallback {
            AtnFallback::Allowed => classify(&path),
            AtnFallback::Forbidden => resolve_static_only(&path),
        };
        for result in &results {
            let tables = result.table.iter().chain(
                result
                    .precedence
                    .iter()
                    .flat_map(|precedence| precedence.tables.iter().flatten()),
            );
            for table in tables {
                let minimized = super::tables::minimized(table);
                assert!(minimized.states.len() <= table.states.len());
                merged += table.states.len() - minimized.states.len();
                assert_runtime_equivalent(table, &minimized);
            }
        }
        decoded_tables(&results);
    }
    assert!(merged > 0, "some states merge");
}

/// The alternative names of each decision of a grammar written to a scratch
/// directory, by rule, in decision order.
fn names_of(name: &str, source: &str) -> Vec<(String, Vec<String>)> {
    let (_, build) = build_source_with(
        name,
        source,
        AtnFallback::Allowed,
        &ParserDfaBudgets::default(),
    );
    build
        .report
        .decisions
        .iter()
        .map(|decision| (decision.rule.clone(), decision.alternatives.all().to_vec()))
        .collect()
}

/// Each decision names its alternatives by label, or by their first elements,
/// with the line where they start. Loops, optional elements, and optional
/// blocks name their implicit alternatives.
#[test]
fn decisions_name_their_alternatives() {
    let source = "grammar Names;
s : e (COMMA e)* COMMA? EOF ;
e : e STAR e     #mul
  | e PLUS e     #add
  | p            #prim
  ;
p : ID LP e RP OVER LP RP
  | ID LP e RP
  | ( ID | INT ) ( DOT ID )?
  ;
COMMA : ',' ; STAR : '*' ; PLUS : '+' ; ID : [a-z]+ ; INT : [0-9]+ ; LP : '(' ; RP : ')' ;
OVER : 'over' ; DOT : '.' ;
";
    let names = names_of("Names", source);
    let find = |rule: &str, first: &str| {
        names
            .iter()
            .find(|(owner, alternatives)| {
                owner == rule
                    && alternatives
                        .first()
                        .is_some_and(|name| name.starts_with(first))
            })
            .map(|(_, alternatives)| alternatives.clone())
            .unwrap_or_else(|| panic!("no decision of {rule} starts with {first}: {names:?}"))
    };
    assert_eq!(
        find("s", "alt 1 (repeat"),
        ["alt 1 (repeat COMMA e, line 2)", "alt 2 (exit the loop)"]
    );
    assert_eq!(
        find("s", "alt 1 (COMMA,"),
        ["alt 1 (COMMA, line 2)", "alt 2 (skip)"]
    );
    assert_eq!(
        find("e", "alt 1 (#mul"),
        ["alt 1 (#mul, line 3)", "alt 2 (#add, line 4)"]
    );
    assert_eq!(
        find("p", "alt 1 (ID LP e ..."),
        [
            "alt 1 (ID LP e ..., line 7)",
            "alt 2 (ID LP e ..., line 8)",
            "alt 3 ((ID | INT) ( ... )?, line 9)",
        ]
    );
    assert_eq!(
        find("p", "alt 1 (DOT ID"),
        ["alt 1 (DOT ID, line 9)", "alt 2 (skip the block)"]
    );
    assert_eq!(
        find("e", "alt 1 (repeat ( #mul"),
        [
            "alt 1 (repeat ( #mul | #add ), line 3)",
            "alt 2 (exit the loop)"
        ]
    );
}

/// `--static-dfa-only` groups the resolved states of a decision by the chosen
/// and the dropped alternatives, over all precedence classes, with the most
/// states first.
#[test]
fn static_only_groups_its_tie_breaks() {
    let mut results = resolve_static_only(&grammars_dir().join("parser").join("GuardExpr.g4"));
    results.extend(resolve_static_only(
        &grammars_dir().join("ReferenceToATN.g4"),
    ));
    results.extend(resolve_static_only(
        &grammars_dir().join("parser").join("LoopFrontier.g4"),
    ));
    for result in &results {
        let Some(resolved) = &result.static_only else {
            continue;
        };
        let grouped: usize = resolved.tie_breaks.iter().map(|group| group.states).sum();
        assert_eq!(grouped, resolved.min_alt + resolved.min_viable_alt);
        for pair in resolved.tie_breaks.windows(2) {
            assert!(
                pair[0].states >= pair[1].states,
                "{:?}",
                resolved.tie_breaks
            );
            assert_ne!(
                (pair[0].chosen, &pair[0].dropped),
                (pair[1].chosen, &pair[1].dropped)
            );
        }
        for group in &resolved.tie_breaks {
            assert!(!group.dropped.is_empty());
            assert!(!group.dropped.contains(&group.chosen));
            assert!(group.dropped.iter().all(|alt| *alt > 0));
            assert!(group.witness.is_some());
        }
    }
    assert!(
        results.iter().any(|result| result
            .static_only
            .as_ref()
            .is_some_and(|resolved| !resolved.tie_breaks.is_empty())),
        "the grammars have tie-breaks"
    );
}

/// A non-greedy decision puts its exit or skip first, and its names follow.
#[test]
fn non_greedy_decisions_name_the_exit_first() {
    let source = "grammar Lazy;
s : ID*? INT ( ID ID | INT INT )?? EOF ;
ID : [a-z]+ ; INT : [0-9]+ ;
";
    let names = names_of("Lazy", source);
    let alternatives: Vec<_> = names.iter().map(|(_, names)| names.clone()).collect();
    assert!(
        alternatives.contains(&vec![
            "alt 1 (exit the loop)".to_owned(),
            "alt 2 (repeat ID, line 2)".to_owned(),
        ]),
        "{alternatives:?}"
    );
    assert!(
        alternatives.contains(&vec![
            "alt 1 (skip the block)".to_owned(),
            "alt 2 (ID ID, line 2)".to_owned(),
            "alt 3 (INT INT, line 2)".to_owned(),
        ]),
        "{alternatives:?}"
    );
}

const TIE_GRAMMAR: &str = "grammar Ties;
s : a EOF ;
a : X Y P | X Y Q | X Z P | X Z Q | W Y P | W Y Q | W Z P | W Z Q ;
X : 'x' ; Y : 'y' ; Z : 'z' ; W : 'w' ; P : 'p' ; Q : 'q' ;
";

/// A decision with more tie-break groups than a `G4S192` warning shows
/// orders them and counts the others.
#[test]
fn static_only_warnings_show_the_largest_tie_break_groups() {
    let budgets = ParserDfaBudgets {
        global: BudgetLimits {
            max_states: Some(8),
            max_k: Some(2),
            ..BudgetLimits::default()
        },
    };
    let (_, build) = build_source_with("Ties", TIE_GRAMMAR, AtnFallback::Forbidden, &budgets);
    let decision = &build.report.decisions[0];
    let resolved = decision.result.static_only.as_ref().expect("resolved");
    let groups: Vec<_> = resolved
        .tie_breaks
        .iter()
        .map(|group| (group.chosen, group.dropped.clone(), group.states))
        .collect();
    assert_eq!(
        groups,
        [
            (1, vec![2], 1),
            (3, vec![4], 1),
            (5, vec![6], 1),
            (7, vec![8], 1)
        ]
    );
    let message = &build
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "G4S192")
        .expect("a G4S192 warning")
        .message;
    assert!(
        message.contains(
            "tie-breaks: 1 state chooses alt 1 (X Y P, line 3) over alt 2 (X Y Q, line 3); \
             1 state chooses alt 3 (X Z P, line 3) over alt 4 (X Z Q, line 3) (witness: 'x' 'z'); \
             1 state chooses alt 5 (W Y P, line 3) over alt 6 (W Y Q, line 3) (witness: 'w' 'y'); \
             and 1 more tie-break group"
        ),
        "{message}"
    );
}

/// A `G4S190` warning for a frontier escape names the viable alternatives of
/// the escape state when the decision has no conflict.
#[test]
fn not_fully_static_warnings_name_the_viable_alternatives() {
    let budgets = ParserDfaBudgets {
        global: BudgetLimits {
            max_states: Some(12),
            max_k: Some(2),
            ..BudgetLimits::default()
        },
    };
    // The direct accept of `e` gives the hybrid table enough coverage.
    let source = TIE_GRAMMAR
        .replace("W Z Q ;", "W Z Q | E ;")
        .replace("Q : 'q' ;", "Q : 'q' ; E : 'e' ;");
    let (_, build) = build_source_with("Ties", &source, AtnFallback::Allowed, &budgets);
    let message = &build
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code == "G4S190")
        .expect("a G4S190 warning")
        .message;
    assert!(
        message.ends_with("; viable alternatives: alt 1 (X Y P, line 3), alt 2 (X Y Q, line 3)"),
        "{message}"
    );
}

/// A non-greedy optional element names its skip first, and a list of names
/// in a diagnostic stops after four names.
#[test]
fn optional_elements_and_long_lists_have_short_names() {
    let source = "grammar Short;
s : A?? (B | C | D | E | F | G) EOF ;
A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ; F : 'f' ; G : 'g' ;
";
    let (_, build) = build_source_with(
        "Short",
        source,
        AtnFallback::Allowed,
        &ParserDfaBudgets::default(),
    );
    let lazy = build
        .report
        .decisions
        .iter()
        .find(|decision| decision.alternatives.all().len() == 2)
        .expect("the decision of `A??`");
    assert_eq!(
        lazy.alternatives.all(),
        ["alt 1 (skip)", "alt 2 (A, line 2)"]
    );
    assert_eq!(
        lazy.alternatives.list(&[1, 2, 1, 2, 1, 2]),
        "alt 1 (skip), alt 2 (A, line 2), alt 1 (skip), alt 2 (A, line 2), \
         and 2 more alternatives"
    );
    assert_eq!(
        lazy.alternatives.list(&[1, 2, 1, 2, 1]),
        "alt 1 (skip), alt 2 (A, line 2), alt 1 (skip), alt 2 (A, line 2), \
         and 1 more alternative"
    );
}
