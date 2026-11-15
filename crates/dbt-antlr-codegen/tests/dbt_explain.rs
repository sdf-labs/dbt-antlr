#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--explain` and `--explain-file`: the differential
//! fault localizer of static parser DFAs.
//!
//! The commands run in `tests/fixtures/explain` with relative paths, so the
//! reports name no absolute path.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/explain")
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_dbt-antlr-codegen"))
        .args(args)
        .current_dir(fixtures())
        .env("NO_COLOR", "1")
        .env("COLUMNS", "4096")
        .output()
        .expect("dbt-antlr-codegen runs")
}

/// Runs a successful explanation and returns its report.
fn explain(args: &[&str]) -> String {
    let output = run(args);
    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(
        output.status.success(),
        "stdout: {stdout}\nstderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    stdout
}

/// Runs a failing command and returns its stderr.
fn fail(args: &[&str]) -> String {
    let output = run(args);
    assert!(
        !output.status.success(),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    String::from_utf8(output.stderr).expect("stderr is UTF-8")
}

#[allow(clippy::disallowed_methods)] // `insta` assertion macros unwrap internal I/O.
#[test]
fn a_budget_guess_on_the_rule_stack() {
    let report = explain(&["--static-dfa-only", "--explain", "a b = c ;", "Budget.g4"]);
    insta::assert_snapshot!("budget_guess_on_the_rule_stack", report);
}

#[allow(clippy::disallowed_methods)]
#[test]
fn a_budget_guess_in_a_completed_subtree() {
    let report = explain(&["--static-dfa-only", "--explain", "a b = c ;", "Subtree.g4"]);
    insta::assert_snapshot!("budget_guess_in_a_completed_subtree", report);
}

#[allow(clippy::disallowed_methods)]
#[test]
fn a_context_sensitive_conflict() {
    let report = explain(&["--static-dfa-only", "--explain", "x a y", "Context.g4"]);
    insta::assert_snapshot!("context_sensitive_conflict", report);
}

#[allow(clippy::disallowed_methods)]
#[test]
fn the_grammar_rejects_the_input() {
    let report = explain(&["--static-dfa-only", "--explain", "a b ;", "Budget.g4"]);
    insta::assert_snapshot!("grammar_rejects_the_input", report);
}

#[allow(clippy::disallowed_methods)]
#[test]
fn a_lexer_error_stops_before_the_parsers() {
    let report = explain(&["--static-dfa-only", "--explain", "a # b ;", "Budget.g4"]);
    insta::assert_snapshot!("lexer_error", report);
}

#[allow(clippy::disallowed_methods)]
#[test]
fn an_input_file_with_two_statements() {
    let report = explain(&[
        "--static-dfa-only",
        "--explain-file",
        "two-statements.txt",
        "Budget.g4",
    ]);
    insta::assert_snapshot!("input_file", report);
}

#[test]
fn a_shared_descent_group_agrees_with_full_ll() {
    let report = explain(&["--static-dfa-only", "--explain", "((a)) + b", "Descent.g4"]);
    assert!(report.contains("[VERDICT: AGREE]"), "{report}");
}

#[test]
fn the_hybrid_parser_agrees_with_full_ll() {
    let report = explain(&["--static-parser-dfa", "--explain", "a b = c ;", "Budget.g4"]);
    assert!(report.contains("mode: --static-parser-dfa"), "{report}");
    assert!(report.contains("[VERDICT: AGREE]"), "{report}");
}

#[test]
fn a_split_grammar_finds_its_token_vocabulary_lexer() {
    let report = explain(&[
        "--static-dfa-only",
        "--explain",
        "a b = c ;",
        "SplitParser.g4",
    ]);
    assert!(
        report.contains("[VERDICT: STATIC-DFA DIVERGENCE]"),
        "{report}"
    );
}

#[test]
fn the_lexer_option_names_the_lexer_grammar() {
    let report = explain(&[
        "--static-dfa-only",
        "--lexer",
        "lexers/OtherLexer.g4",
        "--start-rule",
        "stat",
        "--explain",
        "a b c ;",
        "OrphanParser.g4",
    ]);
    assert!(report.contains("start rule: stat"), "{report}");
    assert!(report.contains("[VERDICT: AGREE]"), "{report}");
}

#[test]
fn quiet_keeps_the_report_and_drops_the_warnings() {
    let args = ["--static-dfa-only", "--explain", "a b = c ;", "Budget.g4"];
    let loud = run(&args);
    let quiet = run(&[&["--quiet"], &args[..]].concat());
    assert!(loud.status.success() && quiet.status.success());
    assert!(
        String::from_utf8_lossy(&loud.stderr).contains("warning[G4S192]"),
        "the budget cutoff is a warning without --quiet"
    );
    assert!(
        String::from_utf8_lossy(&loud.stderr)
            .ends_with("summary: Budget.g4: 0 errors, 1 warning (1 G4S192)\n"),
        "the summary line ends stderr"
    );
    assert_eq!(
        String::from_utf8_lossy(&quiet.stderr),
        "summary: Budget.g4: 0 errors, 1 warning (1 G4S192); --quiet hid the warning\n"
    );
    assert_eq!(quiet.stdout, loud.stdout);
}

#[test]
fn quiet_keeps_the_errors() {
    let output = run(&[
        "-q",
        "--static-dfa-only",
        "--explain",
        "a ; x y z ;",
        "Predicated.g4",
    ]);
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[VERDICT: STATIC-DFA SYNTAX ERROR]"),
        "{stdout}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("error[G4S190]: Predicated.g4:3:4:"),
        "{stderr}"
    );
    assert!(!stderr.contains("warning["), "{stderr}");
    assert!(
        stderr.contains(
            "summary: Predicated.g4: 1 error (1 G4S190), 1 warning (1 G4S192); \
             --quiet hid the warning\n"
        ),
        "{stderr}"
    );
}

#[test]
fn explain_needs_a_static_parser_mode() {
    let stderr = fail(&["--explain", "a b c ;", "Budget.g4"]);
    assert!(
        stderr.contains("add --static-dfa-only or --static-parser-dfa"),
        "{stderr}"
    );
}

#[test]
fn explain_takes_one_input() {
    let stderr = fail(&[
        "--static-dfa-only",
        "--explain",
        "a b c ;",
        "--explain-file",
        "two-statements.txt",
        "Budget.g4",
    ]);
    assert!(stderr.contains("at most one --explain"), "{stderr}");
}

#[test]
fn the_start_rule_needs_explain() {
    let stderr = fail(&["--static-dfa-only", "--start-rule", "stat", "Budget.g4"]);
    assert!(stderr.contains("need --explain"), "{stderr}");
}

#[test]
fn an_unknown_start_rule_is_an_error() {
    let stderr = fail(&[
        "--static-dfa-only",
        "--start-rule",
        "nothing",
        "--explain",
        "a b c ;",
        "Budget.g4",
    ]);
    assert!(stderr.contains("has no rule"), "{stderr}");
    assert!(stderr.contains("nothing"), "{stderr}");
}

#[test]
fn a_missing_input_file_is_an_error() {
    let stderr = fail(&[
        "--static-dfa-only",
        "--explain-file",
        "missing.txt",
        "Budget.g4",
    ]);
    assert!(
        stderr.contains("cannot read --explain-file missing.txt"),
        "{stderr}"
    );
}

#[test]
fn a_parser_grammar_without_its_lexer_is_an_error() {
    let stderr = fail(&["--static-dfa-only", "--explain", "a", "OrphanParser.g4"]);
    assert!(stderr.contains("OtherLexer"), "{stderr}");
}

#[test]
fn a_stack_hijack_defers_the_guard() {
    let args = [
        "--static-dfa-only",
        "--explain",
        "SELECT INTERVAL '1' DAY + 1 + 2",
        "Interval.g4",
    ];
    let output = run(&args);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("[VERDICT: STATIC-DFA DIVERGENCE]"),
        "{stdout}"
    );
    assert!(stdout.contains("via a stack guard that defers"), "{stdout}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[stack-hijack hazard: when called from 'column' (line 8), defers 'DAY' '+' INT \
             even when followed by '+']"
        ),
        "{stderr}"
    );
}

#[test]
fn a_deferred_guard_can_build_another_tree() {
    let report = explain(&[
        "--static-dfa-only",
        "--explain",
        "SELECT INTERVAL '1' DAY + 1",
        "Interval.g4",
    ]);
    assert!(report.contains("[VERDICT: SILENT DIVERGENCE]"), "{report}");
    assert!(report.contains("via a stack guard that defers"), "{report}");
}
