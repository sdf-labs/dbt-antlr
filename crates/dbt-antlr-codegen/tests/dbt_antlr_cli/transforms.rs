// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Grammar transform integration tests, ported from the vendored
//! `antlr-rust-codegen` `antlr4_rust_gen_cli/transforms.rs` suite to the
//! `dbt-antlr-codegen` command line and the `dbt-antlr-runtime` runtime.
#[allow(clippy::wildcard_imports)]
use super::support::*;

#[allow(clippy::too_many_lines)] // the differential harness source is one literal
#[test]
fn precedence_ladder_optimization_is_explicit_auditable_and_recognition_preserving() {
    let temp = temporary_directory("precedence-ladder");
    let (baseline, optimized, report) = run_optimization_matrix(
        temp.path(),
        "tests/fixtures/transforms/precedence-ladder/Ladder.g4",
        "--optimize-precedence-ladders",
        "--report-precedence-ladders",
    );

    let manifest = fs::read_to_string(optimized.join("optimizations.json"))
        .expect("applied optimization manifest should be emitted");
    insta::assert_snapshot!("precedence_ladder_optimization_manifest", manifest);
    let dry_run_manifest = fs::read_to_string(report.join("optimizations.json"))
        .expect("dry-run optimization manifest should be emitted");
    assert!(dry_run_manifest.contains("\"reportOnly\": true"));
    assert!(dry_run_manifest.contains("\"status\": \"eligible\""));
    assert!(!dry_run_manifest.contains("\"status\": \"applied\""));

    let baseline_parser = fs::read_to_string(baseline.join("ladderparser.rs"))
        .expect("baseline parser should be emitted");
    let optimized_parser = fs::read_to_string(optimized.join("ladderparser.rs"))
        .expect("optimized parser should be emitted");
    assert!(generated_parser_has_rule(&baseline_parser, "conditionalOr"));
    assert!(!generated_parser_has_rule(
        &optimized_parser,
        "conditionalOr"
    ));
    assert!(generated_parser_has_rule(&optimized_parser, "expr"));
    assert!(generated_parser_has_rule(&optimized_parser, "atom"));

    assert_generated_project(
        temp.path(),
        &baseline,
        &optimized,
        r#"
#[cfg(test)]
mod precedence_ladder_differential {
    use std::cell::Cell;
    use std::rc::Rc;

    use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
    use dbt_antlr_runtime::error_listener::ErrorListener;
    use dbt_antlr_runtime::errors::ANTLRError;
    use dbt_antlr_runtime::int_stream::IntStream;
    use dbt_antlr_runtime::parser::Parser;
    use dbt_antlr_runtime::recognizer::Recognizer;
    use dbt_antlr_runtime::token::Token;
    use dbt_antlr_runtime::token_factory::CommonTokenFactory;
    use dbt_antlr_runtime::trees::string_tree;
    use dbt_antlr_runtime::{Arena, InputStream};

    use crate::{baseline, optimized};

    #[derive(Debug)]
    struct ParseOutcome {
        completed: bool,
        syntax_errors: usize,
        token_index: isize,
        tree: String,
    }

    impl ParseOutcome {
        fn accepted(&self) -> bool {
            self.completed && self.syntax_errors == 0
        }
    }

    struct CountingErrors {
        count: Rc<Cell<usize>>,
    }

    impl<'input: 'arena, 'arena, R> ErrorListener<'input, 'arena, R> for CountingErrors
    where
        R: Recognizer<'input, 'arena>,
    {
        fn syntax_error(
            &self,
            _recognizer: &R,
            _offending_symbol: Option<&dyn Token>,
            _line: u32,
            _column: i32,
            _msg: &str,
            _error: Option<&ANTLRError>,
        ) {
            self.count.set(self.count.get() + 1);
        }
    }

    macro_rules! parse_result {
        ($name:ident, $module:ident, $entry:ident) => {
            fn $name(input: &str) -> ParseOutcome {
                Arena::with(|arena| {
                    let lexer = $module::ladderlexer::LadderLexer::<_, CommonTokenFactory>::new(
                        arena,
                        InputStream::new(input),
                    );
                    let stream = CommonTokenStream::new(lexer);
                    let mut parser = $module::ladderparser::LadderParser::new(arena, stream);
                    let error_count = Rc::new(Cell::new(0));
                    parser.add_error_listener(Box::new(CountingErrors {
                        count: Rc::clone(&error_count),
                    }));
                    match parser.$entry() {
                        Ok(tree) => {
                            let text = string_tree(tree, parser.get_rule_names());
                            let token_index = parser.get_input_stream().index();
                            ParseOutcome {
                                completed: true,
                                syntax_errors: error_count.get(),
                                token_index,
                                tree: text,
                            }
                        }
                        Err(_) => ParseOutcome {
                            completed: false,
                            syntax_errors: usize::MAX,
                            token_index: 0,
                            tree: String::new(),
                        },
                    }
                })
            }
        };
    }

    fn assert_same_recognition(input: &str, baseline: ParseOutcome, optimized: ParseOutcome) {
        assert_eq!(
            optimized.accepted(),
            baseline.accepted(),
            "recognition diverged for {input:?}: baseline={baseline:?}, optimized={optimized:?}"
        );
        if baseline.accepted() {
            assert_eq!(
                optimized.token_index, baseline.token_index,
                "valid-input consumption diverged for {input:?}: \
                 baseline={baseline:?}, optimized={optimized:?}"
            );
        }
    }

    parse_result!(baseline, baseline, start);
    parse_result!(optimized, optimized, start);
    parse_result!(baseline_star, baseline, starStart);
    parse_result!(optimized_star, optimized, starStart);
    parse_result!(baseline_direct, baseline, directStart);
    parse_result!(optimized_direct, optimized, directStart);
    parse_result!(baseline_mixed, baseline, mixedStart);
    parse_result!(optimized_mixed, optimized, mixedStart);

    #[test]
    fn valid_and_invalid_inputs_keep_the_authored_language() {
        for input in [
            "1",
            "1 + 2 * 3",
            "-1 + 2",
            "!!1 || 2 && 3",
            "1 < 2 == 3",
            "1 ? 2 : 3 ? 4 : 5",
            "(1 + 2) * 3",
        ] {
            let baseline = baseline(input);
            assert!(baseline.accepted(), "baseline should accept {input:?}: {baseline:?}");
            assert_same_recognition(input, baseline, optimized(input));
        }
        for input in [
            "+",
            "1 ? 2 ? 3 : 4 : 5",
            "1 +",
            "? 1 : 2",
            "1 ? 2",
            "1 && || 2",
            "((1)",
        ] {
            let baseline = baseline(input);
            assert!(
                !baseline.accepted(),
                "baseline should reject {input:?}: {baseline:?}"
            );
            assert_same_recognition(input, baseline, optimized(input));
        }
    }

    #[test]
    fn precedence_and_right_associativity_survive_the_rewrite() {
        assert_eq!(
            optimized("1 + 2 * 3").tree,
            "(start (expr (expr (atom 1)) + (expr (expr (atom 2)) * (expr (atom 3)))) <EOF>)",
            "multiplication should bind inside the right operand"
        );
        assert_eq!(
            optimized("1 ? 2 : 3 ? 4 : 5").tree,
            "(start (expr (expr (atom 1)) ? (expr (atom 2)) : \
             (expr (expr (atom 3)) ? (expr (atom 4)) : (expr (atom 5)))) <EOF>)",
            "the conditional tail should remain right associative"
        );
    }

    #[test]
    fn lowest_star_rung_remains_repeatable_at_the_ladder_boundary() {
        for input in [
            "1",
            "1 * 2 * 3",
            "1 + 2 * 3 + 4 * 5 * 6",
            "1 *",
            "* 1",
        ] {
            assert_same_recognition(input, baseline_star(input), optimized_star(input));
        }
    }

    #[test]
    fn direct_base_rhs_and_binary_right_tail_match_the_original() {
        for input in [
            "1",
            "1 ^ 2 ^ 3",
            "1 ~ 2 ~ 3",
            "1 ~ 2 ^ 3 ~ 4",
            "1 ^",
            "~ 1",
        ] {
            assert_same_recognition(input, baseline_direct(input), optimized_direct(input));
        }
    }

    #[test]
    fn looser_prefix_stays_out_of_a_tighter_right_tail_operand() {
        for input in ["1", "!1", "!!1 ^ 2", "1 ^ 2 ^ 3"] {
            assert_same_recognition(input, baseline_mixed(input), optimized_mixed(input));
        }

        let invalid = baseline_mixed("1 ^ !1");
        assert!(
            !invalid.accepted(),
            "baseline should reject the loose prefix: {invalid:?}"
        );
        assert_same_recognition("1 ^ !1", invalid, optimized_mixed("1 ^ !1"));
    }
}
"#,
    );
}

#[test]
fn configured_entry_survives_precedence_ladder_optimization() {
    let temp = temporary_directory("configured-precedence-entry");
    let grammar = fixture("precedence-ladder/Ladder.g4");
    let out = temp.path().join("generated");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--entry-rule"),
        OsStr::new("conditionalOr"),
        OsStr::new("-visitor"),
        OsStr::new("--optimize-precedence-ladders"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    let parser = fs::read_to_string(out.join("ladderparser.rs")).expect("parser should be emitted");
    let api = generated_parser_api(&parser);
    assert!(
        api.iter().any(|symbol| symbol == "fn conditionalOr"),
        "configured entry conditionalOr should survive optimization"
    );
    insta::assert_debug_snapshot!("configured_precedence_entry_generated_api", api);
    assert_generated_modules_compile(temp.path(), &out);
}

#[test]
fn precedence_ladder_modes_reject_authored_semantic_errors_before_transforming() {
    let temp = temporary_directory("precedence-ladder-authored-errors");
    let cases = [
        (
            "DuplicateRule",
            r#"grammar DuplicateRule;
start : high EOF ;
high : low ('+' low)* ;
low : atom ('*' atom)* ;
low : atom ;
atom : INT ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
"#,
            "G4S002",
            "rule low is redefined",
        ),
        (
            "DuplicateLabel",
            r#"grammar DuplicateLabel;
start : high EOF ;
high : low ('+' low)* # Shared ;
low : atom ('*' atom)* # Shared ;
atom : INT ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
"#,
            "G4S013",
            "alternative label Shared is redefined",
        ),
    ];
    let mut diagnostics = Vec::new();

    for (grammar_name, source, code, message) in cases {
        let grammar_dir = temp.path().join(grammar_name);
        fs::create_dir_all(&grammar_dir).expect("grammar directory should be writable");
        fs::write(grammar_dir.join(format!("{grammar_name}.g4")), source)
            .expect("invalid grammar should be writable");
        // Invoke with paths relative to the temporary directory: miette wraps
        // its rendered output at a fixed width, so absolute paths would make
        // the wrap points (and this snapshot) depend on the machine.
        let grammar = format!("{grammar_name}/{grammar_name}.g4");
        for flag in [
            "--optimize-precedence-ladders",
            "--report-precedence-ladders",
        ] {
            let output = run_dbt_antlr_in(
                temp.path(),
                &[
                    OsStr::new(&grammar),
                    OsStr::new(flag),
                    OsStr::new("-o"),
                    OsStr::new("generated"),
                ],
            );
            assert!(
                !output.status.success(),
                "{grammar_name} unexpectedly succeeded with {flag}"
            );
            let stderr = utf8(&output.stderr);
            assert!(stderr.contains(code), "{stderr}");
            assert!(stderr.contains(message), "{stderr}");
            diagnostics.push((grammar_name, flag, stderr.to_owned()));
        }
    }

    insta::assert_debug_snapshot!("precedence_ladder_authored_semantic_errors", diagnostics);
}

#[test]
fn unoptimized_regeneration_removes_stale_precedence_manifest() {
    let temp = temporary_directory("precedence-ladder-stale-manifest");
    let grammar = fixture("precedence-ladder/Ladder.g4");
    let out = temp.path().join("generated");

    let optimized = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--optimize-precedence-ladders"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        optimized.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&optimized.stdout),
        utf8(&optimized.stderr)
    );
    assert!(out.join("optimizations.json").is_file());

    let baseline = run_dbt_antlr(&[grammar.as_os_str(), OsStr::new("-o"), out.as_os_str()]);
    assert!(
        baseline.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&baseline.stdout),
        utf8(&baseline.stderr)
    );
    assert!(!out.join("optimizations.json").exists());
    let parser = fs::read_to_string(out.join("ladderparser.rs"))
        .expect("unoptimized parser should be emitted");
    assert!(generated_parser_has_rule(&parser, "conditionalOr"));
}

#[test]
fn embedded_actions_keep_potentially_referenced_ladder_rules() {
    let temp = temporary_directory("precedence-ladder-embedded-actions");
    let grammar = temp.path().join("ActionLadder.g4");
    let out = temp.path().join("generated");
    fs::write(
        &grammar,
        "grammar ActionLadder;\n\
         start: high { let _ = recog.low(); } EOF;\n\
         high: low ('+' low)*;\n\
         low: atom ('*' atom)*;\n\
         atom: INT;\n\
         INT: [0-9]+;\n\
         WS: [ \\t\\r\\n]+ -> skip;\n",
    )
    .expect("embedded-action grammar should be writable");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--optimize-precedence-ladders"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    let parser =
        fs::read_to_string(out.join("actionladderparser.rs")).expect("parser should be emitted");
    assert!(generated_parser_has_rule(&parser, "low"));
    let manifest = fs::read_to_string(out.join("optimizations.json"))
        .expect("optimization manifest should be emitted");
    assert!(manifest.contains("\"changed\": false"), "{manifest}");
    assert_generated_modules_compile(temp.path(), &out);
}

#[test]
fn embedded_rule_attributes_keep_potentially_referenced_ladder_contexts() {
    let temp = temporary_directory("precedence-ladder-embedded-rule-attributes");
    let grammar = temp.path().join("AttributeLadder.g4");
    let out = temp.path().join("generated");
    fs::write(
        &grammar,
        "grammar AttributeLadder;\n\
         start returns [std::marker::PhantomData<fn() -> LowContextAll<'static, 'static, dbt_antlr_runtime::token::CommonToken<'static>>> saved]\n\
             : high EOF;\n\
         high: low ('+' low)*;\n\
         low: atom ('*' atom)*;\n\
         atom: INT;\n\
         INT: [0-9]+;\n\
         WS: [ \\t\\r\\n]+ -> skip;\n",
    )
    .expect("embedded-rule-attribute grammar should be writable");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--optimize-precedence-ladders"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    let parser =
        fs::read_to_string(out.join("attributeladderparser.rs")).expect("parser should be emitted");
    assert!(generated_parser_has_rule(&parser, "low"));
    let manifest = fs::read_to_string(out.join("optimizations.json"))
        .expect("optimization manifest should be emitted");
    assert!(manifest.contains("\"changed\": false"), "{manifest}");
    // No compile check here: the `returns` clause type is outside the
    // template's `rustTypeInitMap`, so the emitted context initializer is
    // the Java tool's `null` placeholder (faithful to the upstream
    // `Rust.stg`); such attribute types never compiled in this emission
    // layer. The transform-level guarantees above are what this test pins.
}

#[allow(clippy::disallowed_methods)] // `insta` assertion macros unwrap internal I/O.
#[test]
fn trivial_rule_inlining_is_explicit_auditable_and_recognition_preserving() {
    let temp = temporary_directory("trivial-inline");
    let (baseline, optimized, report) = run_optimization_matrix(
        temp.path(),
        "tests/fixtures/transforms/trivial-inline/Inline.g4",
        "--inline-trivial-rules",
        "--report-trivial-rules",
    );

    let manifest = fs::read_to_string(optimized.join("optimizations.json"))
        .expect("applied optimization manifest should be emitted");
    insta::assert_snapshot!("trivial_inline_optimization_manifest", manifest);
    let dry_run_manifest = fs::read_to_string(report.join("optimizations.json"))
        .expect("dry-run optimization manifest should be emitted");
    assert!(dry_run_manifest.contains("\"reportOnly\": true"));
    assert!(dry_run_manifest.contains("\"status\": \"eligible\""));
    assert!(dry_run_manifest.contains("\"status\": \"declined\""));
    assert!(!dry_run_manifest.contains("\"status\": \"applied\""));

    let baseline_parser = fs::read_to_string(baseline.join("inlineparser.rs"))
        .expect("baseline parser should be emitted");
    let optimized_parser = fs::read_to_string(optimized.join("inlineparser.rs"))
        .expect("optimized parser should be emitted");
    for dissolved in ["kw", "qualifier", "group"] {
        assert!(
            generated_parser_has_rule(&baseline_parser, dissolved),
            "baseline lost {dissolved}"
        );
        assert!(
            !generated_parser_has_rule(&optimized_parser, dissolved),
            "optimized parser should inline away {dissolved}"
        );
    }
    for kept in ["target", "marked", "badge", "opt"] {
        assert!(
            generated_parser_has_rule(&optimized_parser, kept),
            "declined or shared rule {kept} must keep its API"
        );
    }

    assert_generated_project(
        temp.path(),
        &baseline,
        &optimized,
        r#"
#[cfg(test)]
mod trivial_inline_differential {
    use std::cell::Cell;
    use std::rc::Rc;

    use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
    use dbt_antlr_runtime::error_listener::ErrorListener;
    use dbt_antlr_runtime::errors::ANTLRError;
    use dbt_antlr_runtime::int_stream::IntStream;
    use dbt_antlr_runtime::parser::Parser;
    use dbt_antlr_runtime::recognizer::Recognizer;
    use dbt_antlr_runtime::token::Token;
    use dbt_antlr_runtime::token_factory::CommonTokenFactory;
    use dbt_antlr_runtime::{Arena, InputStream};

    use crate::{baseline, optimized};

    #[derive(Debug)]
    struct ParseOutcome {
        completed: bool,
        syntax_errors: usize,
        token_index: isize,
    }

    impl ParseOutcome {
        fn accepted(&self) -> bool {
            self.completed && self.syntax_errors == 0
        }
    }

    struct CountingErrors {
        count: Rc<Cell<usize>>,
    }

    impl<'input: 'arena, 'arena, R> ErrorListener<'input, 'arena, R> for CountingErrors
    where
        R: Recognizer<'input, 'arena>,
    {
        fn syntax_error(
            &self,
            _recognizer: &R,
            _offending_symbol: Option<&dyn Token>,
            _line: u32,
            _column: i32,
            _msg: &str,
            _error: Option<&ANTLRError>,
        ) {
            self.count.set(self.count.get() + 1);
        }
    }

    macro_rules! parse_result {
        ($name:ident, $module:ident, $entry:ident) => {
            fn $name(input: &str) -> ParseOutcome {
                Arena::with(|arena| {
                    let lexer = $module::inlinelexer::InlineLexer::<_, CommonTokenFactory>::new(
                        arena,
                        InputStream::new(input),
                    );
                    let stream = CommonTokenStream::new(lexer);
                    let mut parser = $module::inlineparser::InlineParser::new(arena, stream);
                    let error_count = Rc::new(Cell::new(0));
                    parser.add_error_listener(Box::new(CountingErrors {
                        count: Rc::clone(&error_count),
                    }));
                    match parser.$entry() {
                        Ok(_tree) => {
                            let token_index = parser.get_input_stream().index();
                            ParseOutcome {
                                completed: true,
                                syntax_errors: error_count.get(),
                                token_index,
                            }
                        }
                        Err(_) => ParseOutcome {
                            completed: false,
                            syntax_errors: usize::MAX,
                            token_index: 0,
                        },
                    }
                })
            }
        };
    }

    fn assert_same_recognition(input: &str, baseline: ParseOutcome, optimized: ParseOutcome) {
        assert_eq!(
            optimized.accepted(),
            baseline.accepted(),
            "recognition diverged for {input:?}: baseline={baseline:?}, optimized={optimized:?}"
        );
        if baseline.accepted() {
            assert_eq!(
                optimized.token_index, baseline.token_index,
                "valid-input consumption diverged for {input:?}: \
                 baseline={baseline:?}, optimized={optimized:?}"
            );
        }
    }

    parse_result!(baseline, baseline, start);
    parse_result!(optimized, optimized, start);

    #[test]
    fn valid_and_invalid_inputs_keep_the_authored_language() {
        for input in [
            "select @x;",
            "from @@abc;",
            "@x select where from;",
            "@@y;",
            "*z; +q;",
            "(@x,);",
            "(@@deep);",
            "select @a; @b from; (@c,); *d;",
        ] {
            let baseline = baseline(input);
            assert!(
                baseline.accepted(),
                "baseline should accept {input:?}: {baseline:?}"
            );
            assert_same_recognition(input, baseline, optimized(input));
        }
        for input in [
            "select;",
            "@x",
            "select from @x;",
            "* ;",
            "(@x;",
            "(@x,,);",
            ",;",
            "x;",
            "select @x",
        ] {
            let baseline = baseline(input);
            assert!(
                !baseline.accepted(),
                "baseline should reject {input:?}: {baseline:?}"
            );
            assert_same_recognition(input, baseline, optimized(input));
        }
    }

    #[test]
    fn inlined_token_sets_and_blocks_survive_direct_reparse() {
        let outcome = optimized("select @x; @y select from where; (@z,);");
        assert!(
            outcome.accepted(),
            "optimized parser should accept the mixed statement input: {outcome:?}"
        );
    }
}
"#,
    );
}

#[allow(clippy::disallowed_methods)] // `insta` assertion macros unwrap internal I/O.
#[test]
fn unreachable_parser_rules_warn_without_pruning() {
    let temp = temporary_directory("unreachable-rules-warning");
    let grammar = fixture("unreachable-rules/Reachability.g4");
    let out = temp.path().join("generated");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("-visitor"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    let stderr = replace_miette_path(
        utf8(&output.stderr),
        grammar
            .to_str()
            .expect("fixture path should be valid Unicode"),
        "<grammar>",
    );
    insta::assert_snapshot!("unreachable_rule_warning_diagnostics", stderr);

    let parser = fs::read_to_string(out.join("reachabilityparser.rs"))
        .expect("baseline parser should be emitted");
    insta::assert_debug_snapshot!(
        "unreachable_rule_default_generated_api",
        generated_parser_api(&parser)
    );
}

#[allow(clippy::disallowed_methods)] // `insta` assertion macros unwrap internal I/O.
#[test]
fn prune_unreachable_is_transitive_loud_and_preserves_explicit_entries() {
    let temp = temporary_directory("unreachable-rules-pruned");
    let grammar = fixture("unreachable-rules/Reachability.g4");
    let out = temp.path().join("generated");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--entry-rule"),
        OsStr::new("manual"),
        OsStr::new("-visitor"),
        OsStr::new("--prune-unreachable-rules"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    let stderr = replace_miette_path(
        utf8(&output.stderr),
        grammar
            .to_str()
            .expect("fixture path should be valid Unicode"),
        "<grammar>",
    );
    insta::assert_snapshot!("pruned_unreachable_rule_diagnostics", stderr);

    let parser = fs::read_to_string(out.join("reachabilityparser.rs"))
        .expect("pruned parser should be emitted");
    insta::assert_debug_snapshot!(
        "pruned_unreachable_rule_generated_api",
        generated_parser_api(&parser)
    );

    let lexer =
        fs::read_to_string(out.join("reachabilitylexer.rs")).expect("lexer should be emitted");
    assert!(lexer.contains("\"LETTER\""));
    assert_generated_modules_compile(temp.path(), &out);
}

#[test]
fn independent_top_level_rules_without_eof_are_inferred() {
    let temp = temporary_directory("no-eof-entry-rules");
    let grammar = temp.path().join("NoEof.g4");
    let out = temp.path().join("generated");
    fs::write(
        &grammar,
        "grammar NoEof;\n\
         parseA : expr ;\n\
         parseB : stmt ;\n\
         expr : ID ;\n\
         stmt : ID SEMI ;\n\
         ID : [a-z]+ ;\n\
         SEMI : ';' ;\n\
         WS : [ \\t\\r\\n]+ -> skip ;\n",
    )
    .expect("grammar should be writable");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--prune-unreachable-rules"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    assert_eq!(utf8(&output.stderr), "");
    assert_generated_modules_compile(temp.path(), &out);
}

#[allow(clippy::disallowed_methods)] // `insta` assertion macros unwrap internal I/O.
#[test]
fn eof_reaching_recursive_source_component_is_inferred_as_entries() {
    let temp = temporary_directory("recursive-entry-component");
    let grammar = temp.path().join("RecursiveEntries.g4");
    let out = temp.path().join("generated");
    fs::write(
        &grammar,
        "grammar RecursiveEntries;\n\
         junk : ID ;\n\
         a : 'a' b | EOF ;\n\
         b : 'b' a ;\n\
         ID : [a-z]+ ;\n\
         WS : [ \\t\\r\\n]+ -> skip ;\n",
    )
    .expect("grammar should be writable");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("-visitor"),
        OsStr::new("--prune-unreachable-rules"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    let parser = fs::read_to_string(out.join("recursiveentriesparser.rs"))
        .expect("parser should be emitted");
    let api = generated_parser_api(&parser);
    for entry in ["a", "b", "junk"] {
        assert!(
            api.iter().any(|symbol| symbol == &format!("fn {entry}")),
            "inferred entry {entry} should survive pruning"
        );
    }
    insta::assert_debug_snapshot!("recursive_eof_entry_component_generated_api", api);
    assert_generated_modules_compile(temp.path(), &out);
}

#[test]
fn token_vocab_source_parser_is_not_diagnosed_or_pruned() {
    let temp = temporary_directory("token-vocab-reachability");
    let vocabulary = temp.path().join("Vocab.g4");
    let grammar = temp.path().join("User.g4");
    let out = temp.path().join("generated");
    fs::write(
        &vocabulary,
        "grammar Vocab;\n\
         vstart : A EOF ;\n\
         vcycle : A vhelper ;\n\
         vhelper : B vcycle | B ;\n\
         A : 'a' ;\n\
         B : 'b' ;\n",
    )
    .expect("vocabulary grammar should be writable");
    fs::write(
        &grammar,
        "parser grammar User;\n\
         options { tokenVocab=Vocab; }\n\
         root : A EOF ;\n",
    )
    .expect("root grammar should be writable");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("-lib"),
        temp.path().as_os_str(),
        OsStr::new("--prune-unreachable-rules"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    assert_eq!(utf8(&output.stderr), "");
    let mut output_files = fs::read_dir(&out)
        .expect("output directory should exist")
        .map(|entry| {
            entry
                .expect("output entry should be readable")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    output_files.sort();
    assert_eq!(
        output_files,
        ["userbaselistener.rs", "userlistener.rs", "userparser.rs"]
    );
}

#[test]
fn lexer_fragments_and_mode_rules_are_not_parser_reachability_candidates() {
    let temp = temporary_directory("lexer-reachability-exclusions");
    let grammar = temp.path().join("Modes.g4");
    let out = temp.path().join("generated");
    fs::write(
        &grammar,
        "lexer grammar Modes;\n\
         A : 'a';\n\
         fragment UNUSED : 'u';\n\
         mode OTHER;\n\
         B : 'b';\n\
         fragment MODE_UNUSED : 'x';\n",
    )
    .expect("lexer grammar should be writable");

    let output = run_dbt_antlr(&[
        grammar.as_os_str(),
        OsStr::new("--prune-unreachable-rules"),
        OsStr::new("-o"),
        out.as_os_str(),
    ]);
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
    assert!(!utf8(&output.stderr).contains("G4S078"));
    let lexer = fs::read_to_string(out.join("modes.rs")).expect("lexer should be emitted");
    assert!(lexer.contains("\"UNUSED\""));
    assert!(lexer.contains("\"MODE_UNUSED\""));
}
