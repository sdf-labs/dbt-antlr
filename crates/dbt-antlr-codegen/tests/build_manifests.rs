#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration test for the optimization manifests that the build-script API
//! ([`Config`]) writes: one manifest per grammar, so the grammars of a split
//! grammar do not overwrite the manifests of the others.

use std::path::Path;

use dbt_antlr_codegen::Config;

#[test]
fn split_grammars_write_one_manifest_each() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/split-grammar");
    let out_dir =
        std::env::temp_dir().join(format!("dbt-build-api-manifests-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);
    std::fs::create_dir_all(&out_dir).expect("output directory");
    let shared = out_dir.join("optimizations.txt");
    std::fs::write(&shared, "{}").expect("shared manifest of an earlier release");

    let mut config = Config::new(fixtures.join("SplitLexer.g4"));
    config
        .grammar(fixtures.join("SplitParser.g4"))
        .out_dir(&out_dir)
        .static_dfa_only(true)
        .cargo_directives(false);
    let written = config.generate().expect("generation succeeds");

    let lexer_manifest = out_dir.join("splitlexer.optimizations.txt");
    let parser_manifest = out_dir.join("splitparser.optimizations.txt");
    assert!(written.contains(&lexer_manifest), "{written:?}");
    assert!(written.contains(&parser_manifest), "{written:?}");
    assert!(
        !shared.exists(),
        "the shared manifest of earlier releases is removed"
    );

    let lexer = std::fs::read_to_string(&lexer_manifest).expect("lexer manifest");
    assert!(lexer.contains("\"grammar\": \"SplitLexer\""), "{lexer}");
    assert!(lexer.contains("\"fullyStaticLexers\": 1"), "{lexer}");
    let parser = std::fs::read_to_string(&parser_manifest).expect("parser manifest");
    assert!(parser.contains("\"grammar\": \"SplitParser\""), "{parser}");
    assert!(parser.contains("\"decisionInventory\": ["), "{parser}");
    assert!(parser.contains("\"rule\": \"statement\""), "{parser}");

    std::fs::remove_dir_all(&out_dir).expect("cleanup");
}
