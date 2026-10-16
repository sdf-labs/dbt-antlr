// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Emission layer generating Rust recognizers for the `dbt-antlr-runtime` runtime.
//!
//! Generated text comes from the minijinja template
//! `templates/dbt/Rust.stg.jinja`, which mirrors the Java tool's `Rust.stg`.
//! The output model ([`model`]) mirrors the Java `codegen.model` classes; the
//! factories ([`lexer_factory`], [`parser_factory`]) mirror the Java
//! `OutputModelController` / `LexerFactory` / `ParserFactory` walks on top of
//! the `grammar` analysis layer.
//!
//! Public entry point of the dbt emission layer: generate Rust recognizer
//! sources for the `dbt-antlr-runtime` runtime from `.g4` grammars.

mod action_translator;
mod lexer_factory;
mod listener_factory;
pub mod model;
mod parser_factory;
pub mod template;

use std::path::{Path, PathBuf};

use crate::emission::lexer_factory::build_lexer_file_context;
use crate::emission::listener_factory::build_tree_walk_file_contexts;
use crate::emission::parser_factory::build_parser_file_context;
use crate::emission::template::render_template;
use crate::error::Diagnostic;
use crate::grammar::compiler::{self, Compilation};
use crate::grammar::loader::LoadOptions;
use crate::optimization::static_lexer_dfa::build_static_lexer_tables;
use crate::optimization::{OptimizationOptions, OptimizationPlan};
/// One generated source file.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmittedFile {
    /// File name as the Java tool's `RustTarget` derives it (lowercased
    /// recognizer name plus `.rs`).
    pub name: String,
    /// File content, byte-identical to the Java tool's output modulo the
    /// `grammarFileName` canonicalization documented on
    /// [`emit_lexer_files`] and the lint allow header: the Java tool emits
    /// inner `#![allow(...)]` attributes, which make the files unusable with
    /// `include!`, so generated files carry no lint attributes and consumers
    /// put `#[allow]` on the wrapping module instead.
    pub content: String,
}

pub use crate::error::EmitError;

fn compile_grammar(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
) -> Result<Compilation, EmitError> {
    let roots = vec![grammar_path.to_path_buf()];
    compiler::compile(LoadOptions {
        roots: roots.clone(),
        library_directories: library_dirs.to_vec(),
    })
    .map_err(|error| crate::compile_error::compilation_error(&error, &roots))
    .map_err(EmitError::from)
}

fn compile_grammar_with(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
    plan: &OptimizationPlan,
) -> Result<Compilation, EmitError> {
    let roots = vec![grammar_path.to_path_buf()];
    compiler::compile_with_transforms(
        LoadOptions {
            roots: roots.clone(),
            library_directories: library_dirs.to_vec(),
        },
        plan.transforms(),
        plan.report_only(),
        plan.entry_rules(),
    )
    .map_err(|error| crate::compile_error::compilation_error(&error, &roots))
    .map_err(EmitError::from)
}

fn bare_file_name(grammar_path: &Path) -> String {
    grammar_path.file_name().map_or_else(
        || grammar_path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// Emits the lexer source files for the grammars rooted at `grammar_path`.
///
/// A combined grammar yields one file for its implicit lexer (for example
/// `csvlexer.rs`); a lexer grammar yields one file for itself
/// (`xmllexer.rs`). Imported grammar units are compiled but not emitted,
/// matching the Java tool's single-input behavior.
///
/// Deviation from the Java tool: the `fileHeader` comment uses the bare file
/// name of `grammar_path` instead of the path verbatim as passed on the
/// command line, so the output does not depend on the caller's working
/// directory.
///
/// # Errors
///
/// Returns [`EmitError::Compile`] when the grammar fails to compile and
/// [`EmitError::Template`] when the template cannot be read or rendered.
pub fn emit_lexer_files(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
) -> Result<Vec<EmittedFile>, EmitError> {
    let compilation = compile_grammar(grammar_path, library_dirs)?;
    let grammar_file_name = bare_file_name(grammar_path);

    let mut files = Vec::new();
    for root in &compilation.roots {
        let Some(lexer_id) = root.lexer else {
            continue;
        };
        let compiled = &compilation.lexers[&lexer_id];
        let context = build_lexer_file_context(compiled, &grammar_file_name, None);
        let content = render_template(&minijinja::Value::from_serialize(&context))?;
        let name = format!("{}.rs", compiled.semantic.unit.name.to_lowercase());
        files.push(EmittedFile { name, content });
    }
    Ok(files)
}

/// Emits all recognizer source files for the grammars rooted at
/// `grammar_path`.
///
/// A combined grammar yields both its implicit lexer file and its parser
/// file (`csvlexer.rs`, `csvparser.rs`), matching the Java tool's
/// single-input behavior.
///
/// The listener/visitor flags are fixed to `true` here, matching how the
/// CSV golden files were generated; use [`emit_files_with_flags`] for
/// grammars generated without the visitor (for example `SimpleLR`).
///
/// The same `grammarFileName` canonicalization deviation as
/// [`emit_lexer_files`] applies.
///
/// # Errors
///
/// See [`emit_lexer_files`].
pub fn emit_files(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
) -> Result<Vec<EmittedFile>, EmitError> {
    emit_files_with_flags(grammar_path, library_dirs, true, true)
}

/// [`emit_files`] with explicit listener/visitor generation flags.
///
/// The file set matches the Java tool's `CodeGenPipeline`: the listener
/// flags gates `<g>listener.rs` plus `<g>baselistener.rs`, the visitor flag
/// gates `<g>visitor.rs` plus `<g>basevisitor.rs` (`wantsBaseListener`/
/// `wantsBaseVisitor` are true for the Rust target).
///
/// # Errors
///
/// See [`emit_lexer_files`].
pub fn emit_files_with_flags(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
    gen_listener: bool,
    gen_visitor: bool,
) -> Result<Vec<EmittedFile>, EmitError> {
    let compilation = compile_grammar(grammar_path, library_dirs)?;
    Ok(render_recognizer_files(&compilation, grammar_path, gen_listener, gen_visitor, false)?.0)
}

/// Output of [`emit_files_with_options`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmittedOutput {
    /// Generated recognizer source files. Empty when the selected transforms
    /// run in report-only mode.
    pub files: Vec<EmittedFile>,
    /// Non-fatal compiler diagnostics (for example unreachable-rule
    /// warnings).
    pub diagnostics: Vec<Diagnostic>,
    /// Transform notes for the user (for example `pruned: ...` lines).
    pub optimization_messages: Vec<String>,
    /// Rendered `optimizations.txt` manifest, present when the selected
    /// transforms emit one.
    pub optimization_manifest: Option<String>,
}

/// [`emit_files_with_flags`] with an explicit grammar transform selection.
///
/// Applies the configured [`OptimizationOptions`] to the grammar model
/// before ATN construction. When the selection runs in report-only mode (a
/// `report_*` flag), no recognizer files are produced and
/// [`EmittedOutput::optimization_manifest`] describes what the transform
/// *would* change.
///
/// `static_lexer_dfa` is the command line's `--static-lexer-dfa`: the
/// lexer units get
/// static DFA tables (plus `G4S188`/`G4S189` warnings for the modes and
/// edges that stay on the lazy simulator). With `false` the output is
/// byte-identical to [`emit_files_with_flags`].
///
/// # Errors
///
/// See [`emit_lexer_files`]. Also returns [`EmitError::Config`] when the
/// transform selection is internally inconsistent (for example apply and
/// report modes of one transform combined).
pub fn emit_files_with_options(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
    gen_listener: bool,
    gen_visitor: bool,
    optimization: &OptimizationOptions,
    static_lexer_dfa: bool,
) -> Result<EmittedOutput, EmitError> {
    let plan = OptimizationPlan::from_options(optimization)?;
    let compilation = compile_grammar_with(grammar_path, library_dirs, &plan)?;
    let mut diagnostics = crate::compile_error::non_fatal_diagnostics(&compilation);
    let optimization_messages = plan.diagnostic_messages(&compilation);
    let optimization_manifest = plan.render_manifest(&compilation);
    let files = if plan.report_only() {
        Vec::new()
    } else {
        let (files, static_diagnostics) = render_recognizer_files(
            &compilation,
            grammar_path,
            gen_listener,
            gen_visitor,
            static_lexer_dfa,
        )?;
        diagnostics.extend(static_diagnostics);
        files
    };
    Ok(EmittedOutput {
        files,
        diagnostics,
        optimization_messages,
        optimization_manifest,
    })
}

fn render_recognizer_files(
    compilation: &Compilation,
    grammar_path: &Path,
    gen_listener: bool,
    gen_visitor: bool,
    static_lexer_dfa: bool,
) -> Result<(Vec<EmittedFile>, Vec<Diagnostic>), EmitError> {
    let grammar_file_name = bare_file_name(grammar_path);
    let grammar_name = grammar_path.file_stem().map_or_else(
        || grammar_file_name.clone(),
        |stem| stem.to_string_lossy().into_owned(),
    );

    let mut files = Vec::new();
    let mut diagnostics = Vec::new();
    for root in &compilation.roots {
        if let Some(lexer_id) = root.lexer {
            let compiled = &compilation.lexers[&lexer_id];
            let static_build =
                static_lexer_dfa.then(|| build_static_lexer_tables(compiled, grammar_path));
            let context = build_lexer_file_context(
                compiled,
                &grammar_file_name,
                static_build.as_ref().map(|build| build.model.clone()),
            );
            if let Some(build) = static_build {
                diagnostics.extend(build.diagnostics);
            }
            let content = render_template(&minijinja::Value::from_serialize(&context))?;
            let name = format!("{}.rs", compiled.semantic.unit.name.to_lowercase());
            files.push(EmittedFile { name, content });
        }
        if let Some(parser_id) = root.parser {
            let compiled = &compilation.parsers[&parser_id];
            let context = build_parser_file_context(
                compiled,
                &compilation.sources,
                &grammar_name,
                &grammar_file_name,
                gen_listener,
                gen_visitor,
            )?;
            let content = render_template(&minijinja::Value::from_serialize(&context))?;
            let name = format!("{}.rs", context.parser.name.to_lowercase());
            files.push(EmittedFile { name, content });
            files.extend(emit_tree_walk_files(
                &compiled.semantic.unit,
                &grammar_name,
                &grammar_file_name,
                gen_listener,
                gen_visitor,
            )?);
        }
    }
    Ok((files, diagnostics))
}

/// Emits only the listener/visitor source files for the grammars rooted at
/// `grammar_path`.
///
/// These are the Java `generateListener`/`generateBaseListener`/
/// `generateVisitor`/`generateBaseVisitor` entry points without the
/// recognizer files. Unlike [`emit_files_with_flags`] this does not build
/// the parser output model, so grammars whose parser emission is not
/// supported yet (alt labels, milestone 4.6) still emit these files.
///
/// # Errors
///
/// See [`emit_lexer_files`].
pub fn emit_listener_visitor_files(
    grammar_path: &Path,
    library_dirs: &[PathBuf],
    gen_listener: bool,
    gen_visitor: bool,
) -> Result<Vec<EmittedFile>, EmitError> {
    let compilation = compile_grammar(grammar_path, library_dirs)?;
    let grammar_file_name = bare_file_name(grammar_path);
    let grammar_name = grammar_path.file_stem().map_or_else(
        || grammar_file_name.clone(),
        |stem| stem.to_string_lossy().into_owned(),
    );

    let mut files = Vec::new();
    for root in &compilation.roots {
        if let Some(parser_id) = root.parser {
            let compiled = &compilation.parsers[&parser_id];
            files.extend(emit_tree_walk_files(
                &compiled.semantic.unit,
                &grammar_name,
                &grammar_file_name,
                gen_listener,
                gen_visitor,
            )?);
        }
    }
    Ok(files)
}

/// Renders the listener/visitor files of one parser unit, in the Java
/// `CodeGenPipeline` order: listener, base listener, visitor, base visitor.
fn emit_tree_walk_files(
    unit: &crate::grammar::model::GrammarUnit,
    grammar_name: &str,
    grammar_file_name: &str,
    gen_listener: bool,
    gen_visitor: bool,
) -> Result<Vec<EmittedFile>, EmitError> {
    let contexts = build_tree_walk_file_contexts(unit, grammar_name, grammar_file_name);
    let base_name = grammar_name.to_lowercase();
    let mut files = Vec::new();
    if gen_listener {
        files.push(render_tree_walk_file(
            &contexts.listener,
            &base_name,
            "listener",
        )?);
        files.push(render_tree_walk_file(
            &contexts.base_listener,
            &base_name,
            "baselistener",
        )?);
    }
    if gen_visitor {
        files.push(render_tree_walk_file(
            &contexts.visitor,
            &base_name,
            "visitor",
        )?);
        files.push(render_tree_walk_file(
            &contexts.base_visitor,
            &base_name,
            "basevisitor",
        )?);
    }
    Ok(files)
}

fn render_tree_walk_file<T: serde::Serialize>(
    context: &T,
    base_name: &str,
    suffix: &str,
) -> Result<EmittedFile, EmitError> {
    let content = render_template(&minijinja::Value::from_serialize(context))?;
    Ok(EmittedFile {
        name: format!("{base_name}{suffix}.rs"),
        content,
    })
}
