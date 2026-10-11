// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Build-script API ([`Config`]) coverage of the grammar transform
//! diagnostics surface, ported from the vendored `multi_recognizer.rs`
//! builder tests.
#![allow(clippy::disallowed_methods)] // insta assertion macros unwrap internal I/O.
#[allow(clippy::wildcard_imports)]
use super::support::*;

#[allow(clippy::disallowed_methods)] // `insta` assertion macros unwrap internal I/O.
#[test]
fn generation_exposes_structured_non_fatal_diagnostics_with_exact_spans() {
    let temp = temporary_directory("generation-structured-diagnostics");
    let grammar = fixture("unreachable-rules/Reachability.g4");
    let source = fs::read_to_string(&grammar).expect("fixture grammar should be readable");
    let mut config = dbt_antlr_codegen::Config::new(&grammar);
    config
        .out_dir(temp.path().join("generated"))
        .cargo_directives(false);
    let generation = config
        .generate_detailed()
        .expect("unreachable rules should remain non-fatal");

    let unreachable = generation
        .diagnostics()
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S078")
        .map(|diagnostic| {
            assert_eq!(diagnostic.path(), grammar);
            let span = diagnostic
                .byte_span()
                .expect("authored diagnostics should retain their source span");
            let subject = source
                .get(span.clone())
                .expect("diagnostic span should be a valid UTF-8 source range");
            (
                diagnostic.code(),
                diagnostic.severity(),
                diagnostic.line(),
                diagnostic.column(),
                span,
                subject,
            )
        })
        .collect::<Vec<_>>();

    insta::assert_debug_snapshot!(
        "generation_structured_non_fatal_diagnostics_with_exact_spans",
        unreachable
    );
}
