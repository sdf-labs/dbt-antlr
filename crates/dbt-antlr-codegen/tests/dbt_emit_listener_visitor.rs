// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Golden-diff test for the dbt listener/visitor emission layer
//! (milestone 4.5).
//!
//! Byte-exact gates against the Java tool output in `dbt-antlr-runtime/tests/gen/`:
//!
//! - `csvlistener.rs`, `csvvisitor.rs` (CSV, `-visitor`),
//! - `simplelrlistener.rs` (`SimpleLR`, no `-visitor` — no visitor files),
//! - `visitorbasiclistener.rs`, `visitorbasicvisitor.rs` (`VisitorBasic`),
//! - `visitorcalclistener.rs`, `visitorcalcvisitor.rs` (`VisitorCalc`, whose
//!   left-recursive `expr` rule has alt labels, so the traits reference the
//!   alt-label context types in the Java `HashMap` label order).
//!
//! The base files (`<g>baselistener.rs`, `<g>basevisitor.rs`) are gated
//! against the committed goldens as well: a fresh reference run of the Java
//! tool (pinned `dbt-antlr4-2.0.0` release) with its source-tree `tool/resources` on the
//! classpath reproduces them byte-for-byte, so they are not stale relative
//! to the current `Rust.stg`. They do not compile (the `BaseListenerFile`
//! template emits an undeclared `'input` lifetime; `BaseVisitorFile` emits
//! only the header comment), so the compile checks exclude them, matching
//! the dbt test suite, which never includes any base file.

mod common;

use common::{compile_check, grammars_dir, report_first_diff};
use dbt_antlr_codegen::dbt::{
    EmittedFile, emit_files, emit_files_with_flags, emit_listener_visitor_files,
};

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
    let golden = common::read_golden(golden_name);
    if golden != actual.content {
        report_first_diff(&golden, &actual.content);
        panic!("{golden_name}: not byte-identical to golden");
    }
}

/// CSV with `-visitor`: lexer, parser, listener, base listener, visitor,
/// base visitor (the Java `CodeGenPipeline` file set).
#[test]
fn csv_emits_the_full_file_set() {
    let files = emit("CSV.g4", true, true);
    let names: Vec<&str> = files.iter().map(|file| file.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "csvlexer.rs",
            "csvparser.rs",
            "csvlistener.rs",
            "csvbaselistener.rs",
            "csvvisitor.rs",
            "csvbasevisitor.rs",
        ]
    );
    for name in [
        "csvlistener.rs",
        "csvvisitor.rs",
        "csvbaselistener.rs",
        "csvbasevisitor.rs",
    ] {
        assert_byte_identical(name, emitted(&files, name));
    }
}

/// `emit_files` (the CSV-style default) fixes both flags on.
#[test]
fn csv_listener_visitor_via_emit_files() {
    let grammar = grammars_dir().join("CSV.g4");
    let files = emit_files(&grammar, &[grammars_dir()]).expect("emission succeeds");
    assert_byte_identical("csvlistener.rs", emitted(&files, "csvlistener.rs"));
    assert_byte_identical("csvvisitor.rs", emitted(&files, "csvvisitor.rs"));
}

/// `SimpleLR` without `-visitor`: listener and base listener only.
#[test]
fn simplelr_emits_listener_only() {
    let files = emit("SimpleLR.g4", true, false);
    let names: Vec<&str> = files.iter().map(|file| file.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "simplelrlexer.rs",
            "simplelrparser.rs",
            "simplelrlistener.rs",
            "simplelrbaselistener.rs",
        ]
    );
    for name in ["simplelrlistener.rs", "simplelrbaselistener.rs"] {
        assert_byte_identical(name, emitted(&files, name));
    }
}

/// `VisitorBasic` with `-visitor`.
#[test]
fn visitorbasic_listener_visitor_match_goldens() {
    let files = emit("VisitorBasic.g4", true, true);
    for name in [
        "visitorbasiclistener.rs",
        "visitorbasicvisitor.rs",
        "visitorbasicbaselistener.rs",
        "visitorbasicbasevisitor.rs",
    ] {
        assert_byte_identical(name, emitted(&files, name));
    }
}

/// `VisitorCalc` with `-visitor`: alt labels of the left-recursive `expr`
/// rule appear in the Java `HashMap` label order (`add`, `number`,
/// `multiply`). The parser file itself needs alt-label context structs
/// (milestone 4.6), so only the listener/visitor files are emitted, through
/// the dedicated entry point.
#[test]
fn visitorcalc_listener_visitor_match_goldens() {
    let grammar = grammars_dir().join("VisitorCalc.g4");
    let files = emit_listener_visitor_files(&grammar, &[grammars_dir()], true, true)
        .expect("emission succeeds");
    let names: Vec<&str> = files.iter().map(|file| file.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "visitorcalclistener.rs",
            "visitorcalcbaselistener.rs",
            "visitorcalcvisitor.rs",
            "visitorcalcbasevisitor.rs",
        ]
    );
    for name in [
        "visitorcalclistener.rs",
        "visitorcalcvisitor.rs",
        "visitorcalcbaselistener.rs",
        "visitorcalcbasevisitor.rs",
    ] {
        assert_byte_identical(name, emitted(&files, name));
    }
}

/// Compile check: the emitted `VisitorBasic` listener/visitor must compile
/// against the `dbt-antlr-runtime` runtime together with the emitted lexer/parser.
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn emitted_visitorbasic_compiles_against_dbt_antlr_runtime() {
    let modules = [
        "visitorbasiclistener",
        "visitorbasicvisitor",
        "visitorbasiclexer",
        "visitorbasicparser",
    ];
    let files: Vec<(String, String)> = emit("VisitorBasic.g4", true, true)
        .into_iter()
        .filter(|file| {
            modules
                .iter()
                .any(|module| file.name == format!("{module}.rs"))
        })
        .map(|file| (file.name, file.content))
        .collect();
    compile_check("dbt-emit-check-visitorbasic", &files, &modules);
}

/// Compile check: the emitted `VisitorCalc` listener/visitor must compile
/// against the `dbt-antlr-runtime` runtime. The parser sibling is the GOLDEN
/// `visitorcalcparser.rs` (its alt-label context structs are milestone 4.6).
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn emitted_visitorcalc_listener_visitor_compile_against_dbt_antlr_runtime() {
    let modules = [
        "visitorcalclistener",
        "visitorcalcvisitor",
        "visitorcalcparser",
    ];
    let grammar = grammars_dir().join("VisitorCalc.g4");
    let mut files: Vec<(String, String)> =
        emit_listener_visitor_files(&grammar, &[grammars_dir()], true, true)
            .expect("emission succeeds")
            .into_iter()
            .filter(|file| {
                modules
                    .iter()
                    .any(|module| file.name == format!("{module}.rs"))
            })
            .map(|file| (file.name, file.content))
            .collect();
    files.push((
        "visitorcalcparser.rs".to_owned(),
        common::read_golden("visitorcalcparser.rs"),
    ));
    compile_check("dbt-emit-check-visitorcalc", &files, &modules);
}
