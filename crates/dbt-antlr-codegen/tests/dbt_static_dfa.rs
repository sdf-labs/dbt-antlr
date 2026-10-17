#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--static-lexer-dfa` (lexer scope): CLI plumbing, the
//! table statics of the flag-on output, ATN elision for a fully static
//! lexer, and the escape-reason warnings of the diagnostic channel.
//!
//! Byte-identity of the flag-off output is covered by the golden sweep
//! (`dbt_emit_sweep.rs`) and is not repeated here.

mod common;

use common::grammars_dir;
use dbt_antlr_codegen::cli;
use dbt_antlr_codegen::emission::{EmittedOutput, emit_files_with_options};
use dbt_antlr_codegen::{OptimizationOptions, Severity};

fn emit(grammar_file: &str, static_lexer_dfa: bool) -> EmittedOutput {
    let grammar = grammars_dir().join(grammar_file);
    emit_files_with_options(
        &grammar,
        &[grammars_dir()],
        true,
        false,
        &OptimizationOptions::default(),
        static_lexer_dfa,
    )
    .expect("emission succeeds")
}

fn lexer_content<'o>(output: &'o EmittedOutput, name: &str) -> &'o str {
    &output
        .files
        .iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} emitted"))
        .content
}

#[test]
fn cli_accepts_the_static_lexer_dfa_flag() {
    let grammar = grammars_dir().join("lexer/Spike.g4");
    let out_dir =
        std::env::temp_dir().join(format!("dbt-static-lexer-dfa-cli-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let args = vec![
        "-o".to_owned(),
        out_dir.to_string_lossy().into_owned(),
        "--static-lexer-dfa".to_owned(),
        grammar.to_string_lossy().into_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    cli::run(&args, &mut stdout, &mut stderr).expect("CLI run succeeds");

    let written = std::fs::read_to_string(out_dir.join("spike.rs")).expect("lexer written");
    assert!(
        written.contains("_STATIC_LEXER_TABLE_WORDS"),
        "the flag puts the table statics into the emitted lexer"
    );

    let warnings = String::from_utf8(stderr).expect("stderr is utf-8");
    assert!(
        warnings.contains("warning[G4S188]"),
        "the escape warning reaches the CLI diagnostic channel: {warnings}"
    );

    std::fs::remove_dir_all(&out_dir).expect("output dir removed");
}

#[test]
fn help_text_lists_the_static_lexer_dfa_flag() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let error = cli::run(&["--help".to_owned()], &mut stdout, &mut stderr)
        .expect_err("--help prints the usage and stops");
    assert!(
        error.to_string().contains("--static-lexer-dfa"),
        "usage lists --static-lexer-dfa: {error}"
    );
}

#[test]
fn xml_lexer_flag_on_keeps_the_atn_next_to_the_tables() {
    // XMLLexer has escape edges (a predicate in COMMENT, a position-dependent
    // action in CLOSE), so the ATN stays for the lazy-simulator hand-off.
    let output = emit("XMLLexer.g4", true);
    let content = lexer_content(&output, "xmllexer.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_with_static_lexer_tables"));
    assert!(content.contains("_serializedATN"));
}

#[test]
fn csv_lexer_flag_on_is_fully_static() {
    // CSV has no predicates, position-dependent actions, or recursion: the
    // tables cover every edge and the ATN is elided entirely.
    let output = emit("CSV.g4", true);
    let content = lexer_content(&output, "csvlexer.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_static_lexer"));
    assert!(!content.contains("_serializedATN"));
}

#[test]
fn spike_lexer_flag_on_keeps_the_atn_next_to_the_tables() {
    // The escape-edge witness: recursion and a predicate escape, so the
    // tables come with the retained ATN.
    let output = emit("lexer/Spike.g4", true);
    let content = lexer_content(&output, "spike.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_with_static_lexer_tables"));
    assert!(content.contains("_serializedATN"));
}

#[test]
fn flag_off_emits_no_tables() {
    for (grammar, name) in [
        ("CSV.g4", "csvlexer.rs"),
        ("XMLLexer.g4", "xmllexer.rs"),
        ("lexer/Spike.g4", "spike.rs"),
    ] {
        let output = emit(grammar, false);
        let content = lexer_content(&output, name);
        assert!(
            !content.contains("_STATIC_LEXER_TABLE"),
            "{name} without the flag has no table statics"
        );
    }
}

#[test]
fn spike_lexer_warnings_name_the_escape_reasons() {
    let output = emit("lexer/Spike.g4", true);
    let escape = output
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "G4S188")
        .expect("escape warning reported");
    assert_eq!(escape.severity(), Severity::Warning);
    assert!(
        escape.message().contains("cross a predicate"),
        "predicate escape reason: {}",
        escape.message()
    );
    assert!(
        !escape.message().contains("reach a recursive rule context"),
        "nested comment compiles without recursion escape: {}",
        escape.message()
    );
    assert!(
        escape.path().ends_with("Spike.g4"),
        "warning path is the grammar: {}",
        escape.path().display()
    );

    let rec_grammar = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/codegen-direct/fixtures/lexer-recursion/LexerRecursion.g4");
    let rec_output = emit_files_with_options(
        &rec_grammar,
        &[],
        true,
        false,
        &OptimizationOptions::default(),
        true,
    )
    .expect("emission succeeds");
    let rec_escape = rec_output
        .diagnostics
        .iter()
        .find(|diagnostic| diagnostic.code() == "G4S188")
        .expect("recursion escape warning reported");
    assert!(
        rec_escape
            .message()
            .contains("reach a recursive rule context"),
        "recursion escape reason: {}",
        rec_escape.message()
    );
}

#[test]
fn flag_off_reports_no_escape_warnings() {
    let output = emit("lexer/Spike.g4", false);
    assert!(
        output
            .diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.code().starts_with("G4S")),
        "no static-DFA diagnostics without the flag: {:?}",
        output.diagnostics
    );
}

#[test]
fn fully_static_csv_reports_no_escape_warnings() {
    let output = emit("CSV.g4", true);
    assert!(
        output
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code() != "G4S188"),
        "a fully static lexer has no escape warning: {:?}",
        output.diagnostics
    );
}

#[test]
fn report_decisions_and_dump_dfas_run_in_report_only_mode() {
    let grammar = grammars_dir().join("CSV.g4");
    let out_dir =
        std::env::temp_dir().join(format!("dbt-static-parser-report-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let args = vec![
        "-o".to_owned(),
        out_dir.to_string_lossy().into_owned(),
        "--report-decisions".to_owned(),
        "--dump-dfas=s:99,zzz,row".to_owned(),
        grammar.to_string_lossy().into_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    cli::run(&args, &mut stdout, &mut stderr).expect("CLI run succeeds");

    let out = String::from_utf8(stdout).expect("stdout is utf-8");
    assert!(
        out.contains("=== decision report: grammar CSV (4 decisions) ==="),
        "{out}"
    );
    assert!(
        out.contains("=== static DFA dump: grammar CSV ==="),
        "{out}"
    );
    assert!(out.contains("--- decision 1: rule 'row'"), "{out}");
    assert!(out.contains("(2 of 4 decisions dumped)"), "{out}");

    let err = String::from_utf8(stderr).expect("stderr is utf-8");
    assert!(err.contains("error[G4S191]"));
    assert!(err.contains("no parser decision of grammar CSV matches 's:99'"));
    assert!(err.contains("matches 'zzz'"));

    let entries = std::fs::read_dir(&out_dir)
        .expect("output dir exists")
        .count();
    assert_eq!(entries, 0, "report-only mode writes no files");
    std::fs::remove_dir_all(&out_dir).expect("output dir removed");
}

#[test]
fn report_decisions_matches_upstream_oracle_on_golden_grammars() {
    let cases = [
        (
            "CSV.g4",
            "=== decision report: grammar CSV (4 decisions) ===\n\
             summary: LL1=4\n\
             fallbacks: static-only=4/4; escape-states=0 across 0 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions=0\n\
             LLK k histogram: {}\n\
             max k: LLK=0, any acyclic non-LL(1) (incl. conflicted)=0\n",
        ),
        (
            "SimpleLR.g4",
            "=== decision report: grammar SimpleLR (1 decisions) ===\n\
             summary: LR_PRECEDENCE=1(1 static)\n\
             fallbacks: static-only=1/1; escape-states=0 across 0 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions=0\n\
             a                      d=0    LR_PRECEDENCE     classes=2 [p<=2:k=1 p>2:k=0] [static] states=4 fb=0\n\
             LLK k histogram: {}\n\
             max k: LLK=0, any acyclic non-LL(1) (incl. conflicted)=0\n",
        ),
        (
            "Labels.g4",
            "=== decision report: grammar Labels (4 decisions) ===\n\
             summary: LL1=2 LLK=1 LR_PRECEDENCE=1(1 static)\n\
             fallbacks: static-only=4/4; escape-states=0 across 0 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions=0\n\
             e                      d=2    LLK               fb=0 k=1 dfaStates=5 [precpred-in-lookahead]\n\
             e                      d=3    LR_PRECEDENCE     classes=5 [p<=2:k=1 p=3..3:k=1 p=4..6:k=1 p=7..7:k=1 p>7:k=0] [static] states=23 fb=0\n\
             LLK k histogram: {1=1}\n\
             max k: LLK=1, any acyclic non-LL(1) (incl. conflicted)=1\n",
        ),
        (
            "VisitorCalc.g4",
            "=== decision report: grammar VisitorCalc (2 decisions) ===\n\
             summary: LLK=1 LR_PRECEDENCE=1(1 static)\n\
             fallbacks: static-only=2/2; escape-states=0 across 0 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions=0\n\
             expr                   d=0    LLK               fb=0 k=1 dfaStates=3 [precpred-in-lookahead]\n\
             expr                   d=1    LR_PRECEDENCE     classes=3 [p<=1:k=1 p=2..2:k=1 p>2:k=0] [static] states=8 fb=0\n\
             LLK k histogram: {1=1}\n\
             max k: LLK=1, any acyclic non-LL(1) (incl. conflicted)=1\n",
        ),
        (
            "VisitorBasic.g4",
            "=== decision report: grammar VisitorBasic (0 decisions) ===\n\
             summary:\n\
             fallbacks: static-only=0/0; escape-states=0 across 0 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions=0\n\
             LLK k histogram: {}\n\
             max k: LLK=0, any acyclic non-LL(1) (incl. conflicted)=0\n",
        ),
        (
            "ReferenceToATN.g4",
            "=== decision report: grammar ReferenceToATN (2 decisions) ===\n\
             summary: LL1=1 HYBRID=1\n\
             fallbacks: static-only=1/2; escape-states=1 across 1 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions=0\n\
             a                      d=0    HYBRID            fb=1 k=2 dfaStates=5 escapes=1 cover=100% approxConflictAlts=[{1, 2}]\n\
             LLK k histogram: {}\n\
             max k: LLK=0, any acyclic non-LL(1) (incl. conflicted)=2\n",
        ),
    ];

    for (grammar_file, expected) in cases {
        let mut opts = OptimizationOptions::default();
        opts.report_decisions(true);
        let output = emit_files_with_options(
            &grammars_dir().join(grammar_file),
            &[grammars_dir()],
            true,
            false,
            &opts,
            false,
        )
        .expect("report succeeds");
        assert!(output.files.is_empty());
        assert_eq!(output.reports, vec![expected.to_owned()], "{grammar_file}");
    }
}

#[test]
fn dump_dfas_selectors_and_paths_match_upstream_oracle() {
    let scratch =
        std::env::temp_dir().join(format!("dbt-static-parser-dump-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).expect("scratch created");
    let grammar_path = scratch.join("T4.g4");
    std::fs::write(
        &grammar_path,
        "grammar T4;\ns: 'if' e s ('else' s)? | 'x';\ne: 'y' 'q' | 'z';\n",
    )
    .expect("grammar written");

    let mut opts = OptimizationOptions::default();
    opts.dump_dfas("paths,0");
    let output = emit_files_with_options(&grammar_path, &[], true, false, &opts, false)
        .expect("dump succeeds");
    assert!(output.diagnostics.is_empty());
    let dump = &output.reports[0];
    assert!(
        dump.contains("=== static DFA dump: grammar T4 ==="),
        "{dump}"
    );
    assert!(dump.contains("vocabulary: 'if'=1"), "{dump}");
    assert!(
        dump.contains("s                      d=0    HYBRID            fb=1 k=1 dfaStates=3"),
        "{dump}"
    );
    assert!(dump.contains("path: (start)"), "{dump}");
    assert!(dump.contains("s1: accept alt=2; path: EOF"), "{dump}");
    assert!(dump.contains("s2: ESCAPE; path: 'else'"), "{dump}");
    assert!(dump.contains("(1 of 3 decisions dumped)"), "{dump}");

    std::fs::remove_dir_all(&scratch).expect("scratch removed");
}

#[test]
fn static_parser_dfa_warnings_report_not_fully_static_reasons_and_group() {
    let scratch =
        std::env::temp_dir().join(format!("dbt-static-parser-warn-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).expect("scratch created");
    let grammar_path = scratch.join("T.g4");
    std::fs::write(
        &grammar_path,
        "grammar T;\ns: '<' (.)*? '>' EOF;\nt: '[' (.)*? ']' EOF;\n",
    )
    .expect("grammar written");

    let mut opts = OptimizationOptions::default();
    opts.static_parser_dfa(true);
    let output = emit_files_with_options(&grammar_path, &[], true, false, &opts, false)
        .expect("emission succeeds");
    let warnings: Vec<_> = output
        .diagnostics
        .iter()
        .filter(|d| d.code() == "G4S190")
        .collect();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].line(), Some(2));
    assert_eq!(warnings[0].column(), Some(10));
    assert_eq!(
        warnings[0].message(),
        "--static-parser-dfa: decision 0 of rule 's' is not fully static: non-greedy loops ((...)*? / (...)+?) inherently need runtime input; to make fully static: rewrite the loop greedily with explicit terminators, or match the delimited region in a lexer rule [same issue at 1 more decision(s): rule 't' decision 1 (3:10)]"
    );

    std::fs::remove_dir_all(&scratch).expect("scratch removed");
}
