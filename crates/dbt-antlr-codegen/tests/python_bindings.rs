#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Tests for the Python bindings (`--python`): the `<parser>_ffi.rs` grammar
//! description for `dbt_antlr_runtime::ffi` bundles and the `<name>_parser.py` module
//! for the `dbt_antlr` Python package.

mod common;

use std::path::{Path, PathBuf};

use common::grammars_dir;
use dbt_antlr_codegen::emission::{EmitError, EmittedOutput, emit_files_with_bindings};
use dbt_antlr_codegen::{Config, OptimizationOptions};

fn emit(grammar: &Path, python_bindings: bool) -> Result<EmittedOutput, EmitError> {
    let library = grammar
        .parent()
        .map(Path::to_path_buf)
        .into_iter()
        .collect::<Vec<_>>();
    emit_files_with_bindings(
        grammar,
        &library,
        true,
        false,
        &OptimizationOptions::default(),
        python_bindings,
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
            std::env::temp_dir().join(format!("dbt-python-bindings-{test}-{}", std::process::id()));
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
fn labels_bindings() {
    let output = emit(&grammars_dir().join("Labels.g4"), true).expect("emission succeeds");
    insta::assert_snapshot!("labels_ffi", file(&output.files, "labelsparser_ffi.rs"));
    assert_eq!(output.python_files.len(), 1);
    insta::assert_snapshot!(
        "labels_python",
        file(&output.python_files, "labels_parser.py")
    );
}

#[test]
fn split_grammar_bindings_use_the_token_vocab_lexer() {
    let grammar =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/split-grammar/SplitParser.g4");
    let output = emit(&grammar, true).expect("emission succeeds");
    insta::assert_snapshot!("split_ffi", file(&output.files, "splitparser_ffi.rs"));
    insta::assert_snapshot!(
        "split_python",
        file(&output.python_files, "split_parser.py")
    );
}

#[test]
fn python_keywords_and_rules_with_arguments() {
    let scratch = ScratchGrammar::new(
        "keywords",
        "Keywords.g4",
        "grammar Keywords;\n\
         from : class type EOF ;\n\
         class : True | type ;\n\
         type : with[1] ;\n\
         with[int depth] : ID ;\n\
         True : 'true' ;\n\
         ID : [a-z]+ ;\n\
         WS : ' ' -> skip ;\n",
    );
    let output = emit(&scratch.path, true).expect("emission succeeds");
    insta::assert_snapshot!("keywords_ffi", file(&output.files, "keywordsparser_ffi.rs"));
    insta::assert_snapshot!(
        "keywords_python",
        file(&output.python_files, "keywords_parser.py")
    );
}

#[test]
fn duplicate_python_names_are_unsupported() {
    let scratch = ScratchGrammar::new(
        "duplicates",
        "Twice.g4",
        "grammar Twice;\nfrom : from_ ;\nfrom_ : ID ;\nID : [a-z]+ ;\n",
    );
    let error = emit(&scratch.path, true).expect_err("emission fails");
    assert!(
        matches!(&error, EmitError::Unsupported(message) if message == "the Python module of TwiceParser defines from_ twice"),
        "{error:?}"
    );
}

#[test]
fn bindings_are_off_by_default() {
    let output = emit(&grammars_dir().join("Labels.g4"), false).expect("emission succeeds");
    assert!(output.python_files.is_empty());
    assert!(
        output
            .files
            .iter()
            .all(|file| !file.name.ends_with("_ffi.rs"))
    );
}

#[test]
fn parser_grammar_without_token_vocab_has_no_bindings() {
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
fn config_writes_python_modules_into_the_python_out_dir() {
    let scratch = ScratchGrammar::new("config", "unused.txt", "");
    let out_dir = scratch.dir.join("rust");
    let python_dir = scratch.dir.join("python");
    let written = Config::new(grammars_dir().join("CSV.g4"))
        .out_dir(&out_dir)
        .python_out_dir(&python_dir)
        .cargo_directives(false)
        .generate()
        .expect("generation succeeds");
    assert!(written.contains(&out_dir.join("csvparser_ffi.rs")));
    assert!(written.contains(&python_dir.join("csv_parser.py")));
    assert!(!out_dir.join("csv_parser.py").exists());
}
