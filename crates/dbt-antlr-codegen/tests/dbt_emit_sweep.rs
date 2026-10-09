#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Full golden sweep for the dbt emission layer (milestone 4.7).
//!
//! Emits every grammar in `dbt-antlr-runtime/grammars/` and asserts byte-exact
//! output against the goldens in `dbt-antlr-runtime/tests/gen/` (modulo the
//! intentional packed-ATN section of parser files). The goldens were first
//! verified byte-identical against a fresh Java tool (`dbt-antlr4-2.0.0`) run
//! before being committed; the committed files deviate from that oracle only
//! in the lint allow header (generated files carry no inner `#![allow(...)]`
//! attributes so they work with `include!`).

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

/// A sweep row: the grammar file, the listener/visitor flags, the parser
/// files gated modulo the ATN section, and the plain byte-exact files.
type SweepRow = (
    &'static str,
    bool,
    bool,
    &'static [&'static str],
    &'static [&'static str],
);

const SWEEP: &[SweepRow] = &[
    (
        "CSV.g4",
        true,
        true,
        &["csvparser.rs"],
        &[
            "csvlexer.rs",
            "csvlistener.rs",
            "csvvisitor.rs",
            "csvbaselistener.rs",
            "csvbasevisitor.rs",
        ],
    ),
    ("XMLLexer.g4", true, false, &[], &["xmllexer.rs"]),
    (
        "SimpleLR.g4",
        true,
        false,
        &["simplelrparser.rs"],
        &[
            "simplelrlexer.rs",
            "simplelrlistener.rs",
            "simplelrbaselistener.rs",
        ],
    ),
    (
        "Labels.g4",
        true,
        false,
        &["labelsparser.rs"],
        &[
            "labelslexer.rs",
            "labelslistener.rs",
            "labelsbaselistener.rs",
        ],
    ),
    (
        "ReferenceToATN.g4",
        true,
        false,
        &["referencetoatnparser.rs"],
        &[
            "referencetoatnlexer.rs",
            "referencetoatnlistener.rs",
            "referencetoatnbaselistener.rs",
        ],
    ),
    (
        "VisitorBasic.g4",
        true,
        true,
        &["visitorbasicparser.rs"],
        &[
            "visitorbasiclexer.rs",
            "visitorbasiclistener.rs",
            "visitorbasicvisitor.rs",
            "visitorbasicbaselistener.rs",
            "visitorbasicbasevisitor.rs",
        ],
    ),
    (
        "VisitorCalc.g4",
        true,
        true,
        &["visitorcalcparser.rs"],
        &[
            "visitorcalclexer.rs",
            "visitorcalclistener.rs",
            "visitorcalcvisitor.rs",
            "visitorcalcbaselistener.rs",
            "visitorcalcbasevisitor.rs",
        ],
    ),
    (
        "Perf.g4",
        true,
        false,
        &["perfparser.rs"],
        &["perflexer.rs", "perflistener.rs", "perfbaselistener.rs"],
    ),
];

#[test]
fn all_golden_grammars_match() {
    for (grammar, listener, visitor, parsers, plains) in SWEEP {
        let files = emit(grammar, *listener, *visitor);
        for name in *parsers {
            let actual = emitted(&files, name);
            let golden = read_golden(name);
            let golden_masked = mask_parser_atn_diff(&golden);
            let actual_masked = mask_parser_atn_diff(&actual.content);
            if golden_masked != actual_masked {
                report_first_diff(&golden_masked, &actual_masked);
                panic!("{name}: content before the ATN section differs from golden");
            }
            common::assert_atn_graphs_equivalent(&golden, &actual.content, name);
        }
        for name in *plains {
            let actual = emitted(&files, name);
            let golden = read_golden(name);
            if golden != actual.content {
                report_first_diff(&golden, &actual.content);
                panic!("{name}: not byte-identical to golden");
            }
        }
    }
}

/// Compile check: everything the sweep emits must compile against the
/// `dbt-antlr-runtime` runtime (base files excluded; see milestone 4.5 notes).
#[test]
#[ignore = "slow: builds dbt-antlr-runtime in a scratch crate; run with --ignored"]
fn sweep_output_compiles_against_dbt_antlr_runtime() {
    let mut sources: Vec<(String, String)> = Vec::new();
    let mut module_names: Vec<String> = Vec::new();
    for (grammar, listener, visitor, _, _) in SWEEP {
        for file in emit(grammar, *listener, *visitor) {
            if file.name.contains("base") {
                continue;
            }
            module_names.push(file.name.trim_end_matches(".rs").to_owned());
            sources.push((file.name.clone(), file.content));
        }
    }
    module_names.sort();
    module_names.dedup();
    let modules: Vec<&str> = module_names.iter().map(String::as_str).collect();
    compile_check("dbt-emit-check-sweep", &sources, &modules);
}
