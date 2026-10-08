// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration test for the dbt emission command line (`src/dbt/cli.rs`):
//! the files it writes into the output directory must equal the emission
//! API output byte for byte.

mod common;

use common::grammars_dir;
use dbt_antlr_codegen::dbt::cli;
use dbt_antlr_codegen::dbt::emit_files_with_flags;

#[test]
fn cli_writes_the_emitted_files() {
    let grammar = grammars_dir().join("CSV.g4");
    let out_dir = std::env::temp_dir().join(format!("dbt-cli-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let args = vec![
        "-o".to_owned(),
        out_dir.to_string_lossy().into_owned(),
        "-lib".to_owned(),
        grammars_dir().to_string_lossy().into_owned(),
        "-visitor".to_owned(),
        grammar.to_string_lossy().into_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    cli::run(&args, &mut stdout, &mut stderr).expect("CLI run succeeds");

    let expected =
        emit_files_with_flags(&grammar, &[grammars_dir()], true, true).expect("emission succeeds");
    assert_ne!(expected.len(), 0);
    for file in &expected {
        let written = std::fs::read_to_string(out_dir.join(&file.name))
            .unwrap_or_else(|error| panic!("{} written: {error}", file.name));
        assert_eq!(
            written, file.content,
            "{} matches the API output",
            file.name
        );
    }

    let printed = String::from_utf8(stdout).expect("stdout is utf-8");
    for file in &expected {
        assert!(
            printed.lines().any(|line| line == file.name),
            "stdout lists {}",
            file.name
        );
    }

    std::fs::remove_dir_all(&out_dir).expect("output dir removed");
}

#[test]
fn no_listener_flag_skips_the_listener_files() {
    let grammar = grammars_dir().join("CSV.g4");
    let out_dir = std::env::temp_dir().join(format!("dbt-cli-test-nl-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let args = vec![
        "-o".to_owned(),
        out_dir.to_string_lossy().into_owned(),
        "-no-listener".to_owned(),
        grammar.to_string_lossy().into_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    cli::run(&args, &mut stdout, &mut stderr).expect("CLI run succeeds");

    let mut names = std::fs::read_dir(&out_dir)
        .expect("output dir readable")
        .map(|entry| {
            entry
                .expect("entry readable")
                .file_name()
                .into_string()
                .expect("utf-8 name")
        })
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(names, ["csvlexer.rs", "csvparser.rs"]);

    std::fs::remove_dir_all(&out_dir).expect("output dir removed");
}
