#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Golden-diff test for the dbt parser emission layer.
//!
//! Renders the CSV combined grammar and compares the emitted `csvparser.rs`
//! against the Java tool output in `dbt-antlr-runtime/tests/gen/csvparser.rs`.
//!
//! The serialized-ATN section differs BY DESIGN: the golden embeds Java
//! i32 words consumed by `ATNDeserializer`, while the emitted file embeds
//! packed u32 words consumed by `PackedATNDeserializer`. The test therefore
//!
//! 1. asserts byte equality of everything before the ATN section,
//! 2. asserts the two ATN encodings build identical graphs (via
//!    `dbt_antlr_runtime::atn_dump`).

mod common;

use common::{compile_check, grammars_dir, mask_parser_atn_diff, report_first_diff};
use dbt_antlr_codegen::dbt::{EmittedFile, emit_files};

fn emit_csv() -> Vec<EmittedFile> {
    let grammar = grammars_dir().join("CSV.g4");
    emit_files(&grammar, &[grammars_dir()]).expect("emission succeeds")
}

#[test]
fn csv_parser_matches_golden_modulo_atn() {
    let files = emit_csv();
    let parser = files
        .iter()
        .find(|f| f.name == "csvparser.rs")
        .expect("parser file emitted");
    let golden = common::read_golden("csvparser.rs");

    // Byte equality of everything before the embedded ATN section (the
    // deserializer import line is masked as part of the intentional
    // ATN-encoding difference).
    let golden_prefix = mask_parser_atn_diff(&golden);
    let emitted_prefix = mask_parser_atn_diff(&parser.content);
    if golden_prefix != emitted_prefix {
        report_first_diff(&golden_prefix, &emitted_prefix);
        panic!("csvparser.rs: content before the ATN section differs from golden");
    }

    common::assert_atn_graphs_equivalent(&golden, &parser.content, "csvparser.rs");
}

/// The combined grammar still emits the lexer byte-identically.
#[test]
fn csv_lexer_still_matches_golden() {
    let files = emit_csv();
    let lexer = files
        .iter()
        .find(|f| f.name == "csvlexer.rs")
        .expect("lexer file emitted");
    let golden = common::read_golden("csvlexer.rs");
    if golden != lexer.content {
        report_first_diff(&golden, &lexer.content);
        panic!("csvlexer.rs: not byte-identical to golden");
    }
}

/// Compile check: the emitted lexer, parser, listener, and visitor must
/// compile against the `dbt-antlr-runtime` runtime. (The emitted
/// `csvbaselistener.rs`/`csvbasevisitor.rs` are skipped: like the goldens,
/// they do not compile — see `dbt_emit_listener_visitor.rs`.)
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn emitted_parser_compiles_against_dbt_antlr_runtime() {
    let modules = ["csvlistener", "csvvisitor", "csvlexer", "csvparser"];
    let files: Vec<(String, String)> = emit_csv()
        .into_iter()
        .filter(|file| {
            modules
                .iter()
                .any(|module| file.name == format!("{module}.rs"))
        })
        .map(|file| (file.name, file.content))
        .collect();
    compile_check("dbt-emit-check-parser", &files, &modules);
}
