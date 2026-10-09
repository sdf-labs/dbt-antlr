#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration test for the build-script API ([`Config`]): it must write the
//! same files the emission API produces, and fail cleanly without an output
//! directory.

mod common;

use common::grammars_dir;
use dbt_antlr_codegen::Config;
use dbt_antlr_codegen::dbt::{EmitError, emit_files_with_flags};

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
