#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration test for the build-script API ([`Config`]): it must write the
//! same files the emission API produces, fail cleanly without an output
//! directory, report the implicit parser tokens that `strict` rejects, and
//! keep the diagnostics of a `quiet` run.

mod common;

use std::path::Path;

use common::grammars_dir;
use dbt_antlr_codegen::emission::{EmitError, emit_files_with_flags};
use dbt_antlr_codegen::{Config, Diagnostic, Severity};

#[test]
fn generates_the_same_files_as_the_emission_api() {
    let grammar = grammars_dir().join("CSV.g4");
    let out_dir = std::env::temp_dir().join(format!("dbt-build-api-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let mut config = Config::new(&grammar);
    config
        .out_dir(&out_dir)
        .lib_dir(grammars_dir())
        .visitor(true)
        .cargo_directives(false);
    let written = config.generate().expect("generation succeeds");

    let expected =
        emit_files_with_flags(&grammar, &[grammars_dir()], true, true).expect("emission succeeds");
    assert_eq!(written.len(), expected.len());
    for file in &expected {
        let path = out_dir.join(&file.name);
        assert!(written.contains(&path), "{} was written", path.display());
        let content = std::fs::read_to_string(&path).expect("written file is readable");
        assert_eq!(
            content, file.content,
            "{} matches the emission API",
            file.name
        );
    }

    std::fs::remove_dir_all(&out_dir).expect("cleanup");
}

#[test]
fn missing_out_dir_is_a_config_error() {
    // `cargo test` does not set OUT_DIR, so generation without an explicit
    // output directory must fail with the configuration error here.
    assert!(std::env::var_os("OUT_DIR").is_none());
    let grammar = grammars_dir().join("CSV.g4");
    let result = Config::new(&grammar).generate();
    assert!(matches!(result, Err(EmitError::Config(_))));
}

#[test]
fn strict_rejects_parser_tokens_that_the_lexer_never_emits() {
    let grammar = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/grammar-semantics/implicit-parser-tokens/SplitParser.g4");
    let out_dir = std::env::temp_dir().join(format!("dbt-build-api-strict-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let mut config = Config::new(&grammar);
    config.out_dir(&out_dir).cargo_directives(false);
    let generation = config
        .generate_detailed()
        .expect("implicit tokens are warnings by default");
    let warnings = generation
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S030")
        .map(|diagnostic| (diagnostic.severity(), diagnostic.to_string()))
        .collect::<Vec<_>>();
    let location = grammar.display();
    assert_eq!(
        warnings,
        [
            (
                Severity::Warning,
                format!(
                    "warning[G4S030]: {location}:5:22: implicit definition of token ENABLE in \
                     parser: the token vocabulary SplitLexer does not define ENABLE, so its lexer \
                     never emits it"
                )
            ),
            (
                Severity::Warning,
                format!(
                    "warning[G4S030]: {location}:5:30: implicit definition of token CACHE in \
                     parser: the token vocabulary SplitLexer does not define CACHE, so its lexer \
                     never emits it"
                )
            ),
        ]
    );

    let error = config
        .strict(true)
        .generate()
        .expect_err("--strict rejects the implicit tokens");
    let EmitError::Compile(error) = error else {
        panic!("expected a compilation error, got {error:?}");
    };
    let errors = error
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S030")
        .map(Diagnostic::severity)
        .collect::<Vec<_>>();
    assert_eq!(errors, [Severity::Error, Severity::Error]);

    std::fs::remove_dir_all(&out_dir).expect("cleanup");
}

#[test]
fn quiet_keeps_the_diagnostics_of_the_generation() {
    let grammar = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/grammar-semantics/implicit-parser-tokens/SplitParser.g4");
    let generation = Config::new(&grammar)
        .report_only(true)
        .cargo_directives(false)
        .quiet(true)
        .generate_detailed()
        .expect("implicit tokens are warnings by default");
    assert!(
        generation
            .diagnostics()
            .iter()
            .any(|diagnostic| diagnostic.code() == "G4S030"),
        "{:?}",
        generation.diagnostics()
    );
}
