#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Tests for `--ffi` (`Config::ffi`): the `<parser>_ffi.rs` grammar
//! description for `dbt_antlr_runtime::ffi` bundles.

mod common;

use std::path::{Path, PathBuf};

use common::grammars_dir;
use dbt_antlr_codegen::emission::{EmitError, EmittedOutput, emit_files_with_ffi};
use dbt_antlr_codegen::{Config, OptimizationOptions};

fn emit(grammar: &Path, gen_ffi: bool) -> Result<EmittedOutput, EmitError> {
    let library = grammar
        .parent()
        .map(Path::to_path_buf)
        .into_iter()
        .collect::<Vec<_>>();
    emit_files_with_ffi(
        grammar,
        &library,
        true,
        false,
        &OptimizationOptions::default(),
        gen_ffi,
    )
}

fn file<'o>(files: &'o [dbt_antlr_codegen::emission::EmittedFile], name: &str) -> &'o str {
    &files
        .iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} was emitted"))
        .content
}

/// A scratch directory with `grammar` written to `file_name`.
struct ScratchGrammar {
    dir: PathBuf,
    path: PathBuf,
}

impl ScratchGrammar {
    fn new(test: &str, file_name: &str, grammar: &str) -> Self {
        let dir =
            std::env::temp_dir().join(format!("dbt-ffi-codegen-{test}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch dir");
        let path = dir.join(file_name);
        std::fs::write(&path, grammar).expect("grammar written");
        Self { dir, path }
    }
}

impl Drop for ScratchGrammar {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn labels_ffi_descriptor() {
    let output = emit(&grammars_dir().join("Labels.g4"), true).expect("emission succeeds");
    insta::assert_snapshot!("labels_ffi", file(&output.files, "labelsparser_ffi.rs"));
}

#[test]
fn split_grammar_ffi_uses_the_token_vocab_lexer() {
    let grammar =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/split-grammar/SplitParser.g4");
    let output = emit(&grammar, true).expect("emission succeeds");
    insta::assert_snapshot!("split_ffi", file(&output.files, "splitparser_ffi.rs"));
}

#[test]
fn rust_keywords_and_rules_with_arguments() {
    let scratch = ScratchGrammar::new(
        "keywords",
        "Keywords.g4",
        "grammar Keywords;\n\
         match : fn type EOF ;\n\
         fn : True | type ;\n\
         type : with[1] ;\n\
         with[int depth] : ID ;\n\
         True : 'true' ;\n\
         ID : [a-z]+ ;\n\
         WS : ' ' -> skip ;\n",
    );
    let output = emit(&scratch.path, true).expect("emission succeeds");
    insta::assert_snapshot!("keywords_ffi", file(&output.files, "keywordsparser_ffi.rs"));
}

#[test]
fn ffi_is_off_by_default() {
    let output = emit(&grammars_dir().join("Labels.g4"), false).expect("emission succeeds");
    assert!(
        output
            .files
            .iter()
            .all(|file| !file.name.ends_with("_ffi.rs"))
    );
}

#[test]
fn parser_grammar_without_token_vocab_has_no_ffi_descriptor() {
    let scratch = ScratchGrammar::new(
        "no-vocab",
        "Bare.g4",
        "parser grammar Bare;\ntokens { A }\nroot : A ;\n",
    );
    let error = emit(&scratch.path, true).expect_err("emission fails");
    assert!(
        matches!(&error, EmitError::Unsupported(message) if message.contains("tokenVocab")),
        "{error:?}"
    );
}

#[test]
fn config_ffi_writes_ffi_descriptor_into_out_dir() {
    let scratch = ScratchGrammar::new("config", "unused.txt", "");
    let out_dir = scratch.dir.join("rust");
    let written = Config::new(grammars_dir().join("CSV.g4"))
        .out_dir(&out_dir)
        .ffi(true)
        .cargo_directives(false)
        .generate()
        .expect("generation succeeds");
    assert!(written.contains(&out_dir.join("csvparser_ffi.rs")));
}
