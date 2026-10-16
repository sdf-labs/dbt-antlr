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
    let grammar = grammars_dir().join("SpikeLexer.g4");
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

    let written = std::fs::read_to_string(out_dir.join("spikelexer.rs")).expect("lexer written");
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
    let output = emit("SpikeLexer.g4", true);
    let content = lexer_content(&output, "spikelexer.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_with_static_lexer_tables"));
    assert!(content.contains("_serializedATN"));
}

#[test]
fn flag_off_emits_no_tables() {
    for (grammar, name) in [
        ("CSV.g4", "csvlexer.rs"),
        ("XMLLexer.g4", "xmllexer.rs"),
        ("SpikeLexer.g4", "spikelexer.rs"),
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
    let output = emit("SpikeLexer.g4", true);
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
        escape.path().ends_with("SpikeLexer.g4"),
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
    let output = emit("SpikeLexer.g4", false);
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
