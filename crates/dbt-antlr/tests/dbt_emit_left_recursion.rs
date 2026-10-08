// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Golden-diff test for the dbt parser emission layer: left-recursive rules
//! (milestone 4.4).
//!
//! Renders the `SimpleLR` combined grammar (direct left recursion in rule `a`,
//! plus an `@after` named action on rule `s`) and compares the emitted
//! `simplelrparser.rs` against the Java tool output in
//! `dbt-antlr-runtime/tests/gen/simplelrparser.rs`.
//!
//! The `SimpleLR` goldens were generated without `-visitor`, so the parser is
//! emitted with `gen_visitor` off. The serialized-ATN section differs BY
//! DESIGN (packed u32 words + `PackedATNDeserializer` vs Java i32 words +
//! `ATNDeserializer`), as in `dbt_emit_parser.rs`.

mod common;

use common::{compile_check, grammars_dir, mask_parser_atn_diff, report_first_diff};
use dbt_antlr::dbt::{EmittedFile, emit_files_with_flags};

fn emit_simplelr() -> Vec<EmittedFile> {
    let grammar = grammars_dir().join("SimpleLR.g4");
    emit_files_with_flags(&grammar, &[grammars_dir()], true, false).expect("emission succeeds")
}

#[test]
fn simplelr_parser_matches_golden_modulo_atn() {
    let files = emit_simplelr();
    let parser = files
        .iter()
        .find(|f| f.name == "simplelrparser.rs")
        .expect("parser file emitted");
    let golden = common::read_golden("simplelrparser.rs");

    // Byte equality of everything before the embedded ATN section (the
    // deserializer import line is masked as part of the intentional
    // ATN-encoding difference).
    let golden_prefix = mask_parser_atn_diff(&golden);
    let emitted_prefix = mask_parser_atn_diff(&parser.content);
    if golden_prefix != emitted_prefix {
        report_first_diff(&golden_prefix, &emitted_prefix);
        panic!("simplelrparser.rs: content before the ATN section differs from golden");
    }

    common::assert_atn_graphs_equivalent(&golden, &parser.content, "simplelrparser.rs");
}

/// The combined grammar still emits the lexer byte-identically.
#[test]
fn simplelr_lexer_matches_golden() {
    let files = emit_simplelr();
    let lexer = files
        .iter()
        .find(|f| f.name == "simplelrlexer.rs")
        .expect("lexer file emitted");
    let golden = common::read_golden("simplelrlexer.rs");
    if golden != lexer.content {
        report_first_diff(&golden, &lexer.content);
        panic!("simplelrlexer.rs: not byte-identical to golden");
    }
}

/// Compile check: the emitted lexer, parser, and listener must compile
/// against the `dbt-antlr-runtime` runtime. (The emitted `simplelrbaselistener.rs`
/// is skipped: like the golden, it does not compile — see
/// `dbt_emit_listener_visitor.rs`.)
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn emitted_simplelr_compiles_against_dbt_antlr_runtime() {
    let modules = ["simplelrlistener", "simplelrlexer", "simplelrparser"];
    let files: Vec<(String, String)> = emit_simplelr()
        .into_iter()
        .filter(|file| {
            modules
                .iter()
                .any(|module| file.name == format!("{module}.rs"))
        })
        .map(|file| (file.name, file.content))
        .collect();
    compile_check("dbt-emit-check-left-recursion", &files, &modules);
}
