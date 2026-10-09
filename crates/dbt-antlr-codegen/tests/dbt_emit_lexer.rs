// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Golden-diff tests for the dbt lexer emission layer.
//!
//! Renders the CSV combined grammar's lexer and the `XMLLexer` lexer grammar
//! and compares against the Java tool output in
//! `dbt-antlr-runtime/tests/gen/{csvlexer,xmllexer}.rs`.
//!
//! Phase A (hard gate): whitespace-normalized equality — both sides are
//! split into whitespace-separated tokens and the token streams must be
//! identical.
//!
//! Phase B (aspirational): exact byte equality. Currently achieved for both
//! goldens and asserted; the diff-class report stays for diagnosis.

#![allow(clippy::print_stderr)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use dbt_antlr_codegen::dbt::{EmittedFile, emit_lexer_files};

fn grammars_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime/grammars")
        .canonicalize()
        .expect("grammars directory exists")
}

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime/tests/gen")
        .canonicalize()
        .expect("golden directory exists")
}

fn emit_one(grammar_file: &str, expected_name: &str) -> EmittedFile {
    let grammar = grammars_dir().join(grammar_file);
    let files = emit_lexer_files(&grammar, &[grammars_dir()]).expect("emission succeeds");
    let mut files = files
        .into_iter()
        .filter(|file| file.name == expected_name)
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 1, "expected exactly one {expected_name}");
    files.remove(0)
}

fn normalized_tokens(content: &str) -> Vec<&str> {
    content.split_whitespace().collect()
}

/// Prints the first difference between the two token streams with context.
fn report_first_token_diff(expected: &[&str], actual: &[&str], context: usize) {
    let common = expected.len().min(actual.len());
    let first = (0..common).find(|&i| expected[i] != actual[i]);
    match first {
        Some(i) => {
            let start = i.saturating_sub(context);
            eprintln!("first token difference at token {i}:");
            eprintln!(
                "  expected: {:?}",
                &expected[start..(i + context).min(expected.len())]
            );
            eprintln!(
                "  actual:   {:?}",
                &actual[start..(i + context).min(actual.len())]
            );
        }
        None => {
            eprintln!(
                "token streams share a prefix; lengths differ: expected {}, actual {}",
                expected.len(),
                actual.len()
            );
            let start = common.saturating_sub(context);
            eprintln!("  expected tail: {:?}", &expected[start..]);
            eprintln!("  actual tail:   {:?}", &actual[start..]);
        }
    }
}

/// Counts exact-diff lines by class and prints the first differing line pair.
fn report_exact_diff(expected: &str, actual: &str) -> (usize, usize, usize) {
    let expected_lines = expected.split('\n').collect::<Vec<_>>();
    let actual_lines = actual.split('\n').collect::<Vec<_>>();
    let common = expected_lines.len().min(actual_lines.len());
    let mut trailing_ws_only = 0;
    let mut blank_line = 0;
    let mut content = 0;
    let mut first_shown = false;
    for i in 0..common {
        if expected_lines[i] == actual_lines[i] {
            continue;
        }
        if expected_lines[i].trim_end() == actual_lines[i].trim_end() {
            trailing_ws_only += 1;
        } else if expected_lines[i].trim().is_empty() || actual_lines[i].trim().is_empty() {
            blank_line += 1;
        } else {
            content += 1;
        }
        if !first_shown {
            first_shown = true;
            let start = i.saturating_sub(2);
            eprintln!("first line difference at line {}:", i + 1);
            for j in start..(i + 2).min(common) {
                eprintln!("  expected[{}]: {:?}", j + 1, expected_lines[j]);
                eprintln!("  actual  [{}]: {:?}", j + 1, actual_lines[j]);
            }
        }
    }
    let extra = expected_lines.len().abs_diff(actual_lines.len());
    if extra > 0 {
        blank_line += extra;
        eprintln!(
            "line counts differ: expected {}, actual {}",
            expected_lines.len(),
            actual_lines.len()
        );
    }
    (content, blank_line, trailing_ws_only)
}

fn check_golden(grammar_file: &str, emitted_name: &str) {
    let emitted = emit_one(grammar_file, emitted_name);
    let golden_path = golden_dir().join(emitted_name);
    let golden = std::fs::read_to_string(&golden_path).expect("golden file readable");

    // Phase A: whitespace-normalized equality (hard gate).
    let expected_tokens = normalized_tokens(&golden);
    let actual_tokens = normalized_tokens(&emitted.content);
    if expected_tokens != actual_tokens {
        report_first_token_diff(&expected_tokens, &actual_tokens, 8);
        panic!("{emitted_name}: whitespace-normalized content differs from golden");
    }

    // Phase B: exact byte equality. Currently achieved for both goldens and
    // asserted; the report stays for diagnosis if a future grammar breaks it.
    if golden != emitted.content {
        let (content, blank_line, trailing_ws) = report_exact_diff(&golden, &emitted.content);
        eprintln!(
            "{emitted_name}: exact diff classes: {content} content, {blank_line} blank-line, {trailing_ws} trailing-whitespace"
        );
        panic!("{emitted_name}: not byte-identical to golden");
    }
}

#[test]
fn csv_lexer_matches_golden() {
    check_golden("CSV.g4", "csvlexer.rs");
}

#[test]
fn xml_lexer_matches_golden() {
    check_golden("XMLLexer.g4", "xmllexer.rs");
}

/// Compile check: the emitted files must compile against the `dbt-antlr-runtime`
/// runtime, like the goldens do. Writes a scratch crate under
/// `/tmp/pi/dbt-emit-check/` and runs `cargo check` on it.
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn emitted_lexers_compile_against_dbt_antlr_runtime() {
    let scratch = Path::new("/tmp/pi/dbt-emit-check");
    let src = scratch.join("src");
    std::fs::create_dir_all(&src).expect("create scratch crate");

    let runtime_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime")
        .canonicalize()
        .expect("dbt-antlr-runtime crate exists");
    std::fs::write(
        scratch.join("Cargo.toml"),
        format!(
            "[package]\nname = \"dbt-emit-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n\
             [dependencies]\ndbt-antlr-runtime = {{ path = {:?} }}\n",
            runtime_path.display().to_string()
        ),
    )
    .expect("write scratch Cargo.toml");

    let mut modules = String::new();
    for (grammar, emitted_name) in [("CSV.g4", "csvlexer.rs"), ("XMLLexer.g4", "xmllexer.rs")] {
        let emitted = emit_one(grammar, emitted_name);
        let module = emitted_name.trim_end_matches(".rs");
        std::fs::write(src.join(emitted_name), &emitted.content).expect("write emitted lexer");
        let _ = writeln!(modules, "#[allow(clippy::all)]\npub mod {module};");
    }
    std::fs::write(src.join("lib.rs"), modules).expect("write scratch lib.rs");

    let output = std::process::Command::new("cargo")
        .args(["check", "--offline"])
        .current_dir(scratch)
        .env("CARGO_TARGET_DIR", scratch.join("target"))
        .output()
        .expect("run cargo check");
    assert!(
        output.status.success(),
        "cargo check failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
