#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--explain`, `--explain-file`, and `--explain-batch`: the differential
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
fn a_budget_guess_on_the_rule_stack_verbose() {
    let report = explain(&[
        "-v",
        "--static-dfa-only",
        "--explain",
        "a b = c ;",
        "Budget.g4",
    ]);
    insta::assert_snapshot!("budget_guess_on_the_rule_stack_verbose", report);
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
fn explain_shows_warnings_by_default_and_quiet_hides_them() {
    let args = ["--static-dfa-only", "--explain", "a b = c ;", "Budget.g4"];
    let default = run(&args);
    let quiet = run(&[&["--quiet"], &args[..]].concat());
    let verbose = run(&[&["-v"], &args[..]].concat());
    assert!(default.status.success() && quiet.status.success() && verbose.status.success());

    let default_stderr = String::from_utf8_lossy(&default.stderr);
    assert!(
        default_stderr.contains("warning[G4S192]"),
        "default mode shows analysis warnings:\n{default_stderr}"
    );
    assert!(
        default_stderr.ends_with("summary: Budget.g4: 0 errors, 1 warning (1 G4S192)\n"),
        "the summary line ends stderr:\n{default_stderr}"
    );
    assert_eq!(verbose.stderr, default.stderr);

    assert_eq!(
        String::from_utf8_lossy(&quiet.stderr),
        "summary: Budget.g4: 0 errors, 1 warning (1 G4S192); --quiet hid the warning\n"
    );
    assert_eq!(quiet.stdout, default.stdout);

    let default_stdout = String::from_utf8_lossy(&default.stdout);
    let verbose_stdout = String::from_utf8_lossy(&verbose.stdout);
    assert!(!default_stdout.contains("[SYNTAX ERROR"));
    assert!(!default_stdout.contains("[STATIC GUESSES"));
    assert!(verbose_stdout.contains("[SYNTAX ERROR"));
    assert!(verbose_stdout.contains("[STATIC GUESSES"));
}

#[test]
fn explain_always_prints_the_summary_line_on_stderr() {
    let output = run(&["--static-dfa-only", "--explain", "((a)) + b", "Descent.g4"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "summary: Descent.g4: 0 errors, 0 warnings\n"
    );
}

#[test]
fn quiet_and_verbose_work_together() {
    let output = run(&[
        "-q",
        "-v",
        "--static-dfa-only",
        "--explain",
        "a b = c ;",
        "Budget.g4",
    ]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "summary: Budget.g4: 0 errors, 1 warning (1 G4S192); --quiet hid the warning\n"
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("[SYNTAX ERROR"), "{stdout}");
    assert!(stdout.contains("[STATIC GUESSES"), "{stdout}");
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

#[allow(clippy::disallowed_methods)]
#[test]
fn multiple_explain_inputs_collapse_agree_and_summarize() {
    let report = explain(&[
        "--static-dfa-only",
        "--explain",
        "a b c ;",
        "--explain",
        "d e f ;",
        "--explain",
        "a b = c ;",
        "--explain-file",
        "two-statements.txt",
        "--explain",
        "a b ;",
        "--explain",
        "a # b ;",
        "Budget.g4",
    ]);
    insta::assert_snapshot!("multi_explain_batch", report);
}

#[test]
fn multiple_explain_inputs_verbose_expands_agree() {
    let report = explain(&[
        "-v",
        "--static-dfa-only",
        "--explain",
        "a b c ;",
        "--explain",
        "a b = c ;",
        "Budget.g4",
    ]);
    assert!(report.contains("--- [1/2] <inline #1>"), "{report}");
    assert!(report.contains("[VERDICT: AGREE]"), "{report}");
    assert!(report.contains("[STATIC GUESSES"), "{report}");
    assert!(report.contains("=== batch summary:"), "{report}");
}

#[test]
fn duplicate_stdin_explain_file_is_an_error() {
    let stderr = fail(&[
        "--static-dfa-only",
        "--explain-file",
        "-",
        "--explain-file",
        "-",
        "Budget.g4",
    ]);
    assert!(stderr.contains("at most one --explain-file"), "{stderr}");
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
    assert!(stdout.contains("[SUGGESTED REMEDIATION]"), "{stdout}");
    assert!(stdout.contains("'column' (line 8)"), "{stdout}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains(
            "[stack-hijack hazard: when called from 'column' (line 8), defers 'DAY' '+' INT \
             even when followed by '+']"
        ),
        "{stderr}"
    );

    let quiet = run(&[&["-q"], &args[..]].concat());
    assert!(quiet.status.success());
    let quiet_stderr = String::from_utf8_lossy(&quiet.stderr);
    assert!(!quiet_stderr.contains("warning["), "{quiet_stderr}");
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

#[allow(clippy::disallowed_methods)]
#[test]
fn explain_batch_file_with_start_rules_and_summary() {
    let report = explain(&[
        "--static-dfa-only",
        "--explain-batch",
        "batch-cases.txt",
        "Budget.g4",
    ]);
    insta::assert_snapshot!("explain_batch_file", report);
}

#[test]
fn explain_batch_stdin_with_custom_delimiter() {
    use std::io::Write as _;
    use std::process::Stdio;

    let mut child = Command::new(env!("CARGO_BIN_EXE_dbt-antlr-codegen"))
        .args([
            "--static-dfa-only",
            "--explain-batch",
            "-",
            "--explain-delimiter",
            "===",
            "Budget.g4",
        ])
        .current_dir(fixtures())
        .env("NO_COLOR", "1")
        .env("COLUMNS", "4096")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("dbt-antlr-codegen spawns");
    child
        .stdin
        .as_mut()
        .expect("stdin is piped")
        .write_all(b"=== ok ===\na b c ;\n=== stat-div [start_rule=stat] ===\na b = c ;\n")
        .expect("writes batch stdin");
    let output = child.wait_with_output().expect("waits for child");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is UTF-8");
    assert!(
        stdout.contains("[1/2] ok (start rule: prog, 4 tokens): AGREE"),
        "{stdout}"
    );
    assert!(
        stdout.contains("--- [2/2] stat-div (start rule: stat, 5 tokens) ---"),
        "{stdout}"
    );
    assert!(
        stdout.contains("[VERDICT: STATIC-DFA DIVERGENCE]"),
        "{stdout}"
    );
}

#[test]
fn explain_batch_unknown_per_case_start_rule_is_an_error() {
    use std::io::Write as _;
    use std::process::Stdio;

    let mut child = Command::new(env!("CARGO_BIN_EXE_dbt-antlr-codegen"))
        .args(["--static-dfa-only", "--explain-batch", "-", "Budget.g4"])
        .current_dir(fixtures())
        .env("NO_COLOR", "1")
        .env("COLUMNS", "4096")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("dbt-antlr-codegen spawns");
    child
        .stdin
        .as_mut()
        .expect("stdin is piped")
        .write_all(b"--- bad [start_rule=missing_rule] ---\na b c ;\n")
        .expect("writes batch stdin");
    let output = child.wait_with_output().expect("waits for child");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("has no rule"), "{stderr}");
    assert!(stderr.contains("missing_rule"), "{stderr}");
}

#[test]
fn missing_batch_file_is_an_error() {
    let stderr = fail(&[
        "--static-dfa-only",
        "--explain-batch",
        "missing-batch.txt",
        "Budget.g4",
    ]);
    assert!(
        stderr.contains("cannot read --explain-batch missing-batch.txt"),
        "{stderr}"
    );
}
