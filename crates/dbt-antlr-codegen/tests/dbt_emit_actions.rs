// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Golden-diff test for the dbt action/alt-label emission layer
//! (milestone 4.6).
//!
//! Byte-exact gates (modulo the intentional packed-ATN section) against the
//! Java tool output in `dbt-antlr-runtime/tests/gen/`:
//!
//! - `Labels.g4` — left-recursive rule with alt labels, `returns [String v]`,
//!   and `$v`/`$a.v`/`$INT.text`/`$x.v` action references (no `-visitor`).
//! - `ReferenceToATN.g4` — the `$text` reference and breadth-first context
//!   getter ordering (no `-visitor`).
//! - `VisitorCalc.g4` — left-recursive rule with alt labels (`-visitor`).

mod common;

use common::{compile_check, grammars_dir, mask_parser_atn_diff, read_golden, report_first_diff};
use dbt_antlr_codegen::dbt::{EmittedFile, emit_files_with_flags};

fn emit(grammar: &str, gen_listener: bool, gen_visitor: bool) -> Vec<EmittedFile> {
    let grammar = grammars_dir().join(grammar);
    emit_files_with_flags(&grammar, &[grammars_dir()], gen_listener, gen_visitor)
        .expect("emission succeeds")
}

fn emitted<'a>(files: &'a [EmittedFile], name: &str) -> &'a EmittedFile {
    files
        .iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} emitted"))
}

fn assert_byte_identical(golden_name: &str, actual: &EmittedFile) {
    assert_eq!(golden_name, actual.name);
    let golden = read_golden(golden_name);
    if golden != actual.content {
        report_first_diff(&golden, &actual.content);
        panic!("{golden_name}: not byte-identical to golden");
    }
}

fn assert_parser_identical_modulo_atn(golden_name: &str, actual: &EmittedFile) {
    assert_eq!(golden_name, actual.name);
    let golden = read_golden(golden_name);
    let golden_masked = mask_parser_atn_diff(&golden);
    let actual_masked = mask_parser_atn_diff(&actual.content);
    if golden_masked != actual_masked {
        report_first_diff(&golden_masked, &actual_masked);
        panic!("{golden_name}: content before the ATN section differs from golden");
    }

    common::assert_atn_graphs_equivalent(&golden, &actual.content, golden_name);
}

#[test]
fn labels_matches_golden() {
    let files = emit("Labels.g4", true, false);
    assert_parser_identical_modulo_atn("labelsparser.rs", emitted(&files, "labelsparser.rs"));
    assert_byte_identical("labelslexer.rs", emitted(&files, "labelslexer.rs"));
    assert_byte_identical("labelslistener.rs", emitted(&files, "labelslistener.rs"));
    assert_byte_identical(
        "labelsbaselistener.rs",
        emitted(&files, "labelsbaselistener.rs"),
    );
}

#[test]
fn reference_to_atn_matches_golden() {
    let files = emit("ReferenceToATN.g4", true, false);
    assert_parser_identical_modulo_atn(
        "referencetoatnparser.rs",
        emitted(&files, "referencetoatnparser.rs"),
    );
    assert_byte_identical(
        "referencetoatnlexer.rs",
        emitted(&files, "referencetoatnlexer.rs"),
    );
    assert_byte_identical(
        "referencetoatnlistener.rs",
        emitted(&files, "referencetoatnlistener.rs"),
    );
    assert_byte_identical(
        "referencetoatnbaselistener.rs",
        emitted(&files, "referencetoatnbaselistener.rs"),
    );
}

#[test]
fn visitor_calc_matches_golden() {
    let files = emit("VisitorCalc.g4", true, true);
    assert_parser_identical_modulo_atn(
        "visitorcalcparser.rs",
        emitted(&files, "visitorcalcparser.rs"),
    );
    assert_byte_identical(
        "visitorcalclexer.rs",
        emitted(&files, "visitorcalclexer.rs"),
    );
    assert_byte_identical(
        "visitorcalclistener.rs",
        emitted(&files, "visitorcalclistener.rs"),
    );
    assert_byte_identical(
        "visitorcalcvisitor.rs",
        emitted(&files, "visitorcalcvisitor.rs"),
    );
    assert_byte_identical(
        "visitorcalcbaselistener.rs",
        emitted(&files, "visitorcalcbaselistener.rs"),
    );
    assert_byte_identical(
        "visitorcalcbasevisitor.rs",
        emitted(&files, "visitorcalcbasevisitor.rs"),
    );
}

/// Compile check: the emitted files must compile against the `dbt-antlr-runtime`
/// runtime. Base listener/visitor files are excluded (the current Rust.stg
/// emits them broken — see the 4.5 milestone notes).
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn emitted_action_grammars_compile_against_dbt_antlr_runtime() {
    for (grammar, visitor, prefix) in [
        ("Labels.g4", false, "labels"),
        ("ReferenceToATN.g4", false, "referencetoatn"),
        ("VisitorCalc.g4", true, "visitorcalc"),
    ] {
        let files = emit(grammar, true, visitor);
        let mut sources: Vec<(String, String)> = Vec::new();
        let mut module_names: Vec<String> = Vec::new();
        for suffix in ["lexer", "parser", "listener"] {
            let name = format!("{prefix}{suffix}.rs");
            let file = emitted(&files, &name);
            sources.push((file.name.clone(), file.content.clone()));
            module_names.push(format!("{prefix}{suffix}"));
        }
        if visitor {
            let name = format!("{prefix}visitor.rs");
            let file = emitted(&files, &name);
            sources.push((file.name.clone(), file.content.clone()));
            module_names.push(format!("{prefix}visitor"));
        }
        let modules: Vec<&str> = module_names.iter().map(String::as_str).collect();
        compile_check(&format!("dbt-emit-check-{prefix}"), &sources, &modules);
    }
}
