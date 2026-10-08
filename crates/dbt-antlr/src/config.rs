// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Build-script entry point: generate recognizers during `cargo build`, in
//! the style of `prost-build` or `lalrpop`.

use std::path::PathBuf;

use crate::dbt::{EmitError, emit_files_with_flags};

/// Generator configuration, primarily for use from a `build.rs` build script.
///
/// With the generator as a build dependency, no separate tool installation is
/// needed and the generator version is pinned by `Cargo.lock` like any other
/// dependency:
///
/// ```toml
/// [dependencies]
/// dbt-antlr-runtime = "0.1"
///
/// [build-dependencies]
/// dbt-antlr = "0.1"
/// ```
///
/// ```rust,no_run
/// // build.rs
/// dbt_antlr::Config::new("grammars/Expr.g4")
///     .generate()
///     .unwrap_or_else(|error| panic!("{error:?}"));
/// ```
///
/// Generated files land in the [output directory](Config::out_dir), which
/// defaults to the `OUT_DIR` environment variable cargo sets for build
/// scripts; pull them into the crate with `include!`:
///
/// ```rust,ignore
/// mod expr {
///     #![allow(clippy::all)]
///     include!(concat!(env!("OUT_DIR"), "/exprparser.rs"));
/// }
/// ```
///
/// [`generate`](Config::generate) prints `cargo:rerun-if-changed` directives
/// for the configured grammars and library directories (including each root
/// grammar's own directory, which the loader searches for imported grammars),
/// so the build script reruns exactly when grammar inputs change.
#[derive(Clone, Debug)]
pub struct Config {
    grammars: Vec<PathBuf>,
    out_dir: Option<PathBuf>,
    lib_dirs: Vec<PathBuf>,
    gen_listener: bool,
    gen_visitor: bool,
    cargo_directives: bool,
}

impl Config {
    /// Starts a configuration generating from `grammar`.
    pub fn new(grammar: impl Into<PathBuf>) -> Self {
        Self {
            grammars: vec![grammar.into()],
            out_dir: None,
            lib_dirs: Vec::new(),
            gen_listener: true,
            gen_visitor: false,
            cargo_directives: true,
        }
    }

    /// Adds another root grammar to generate from.
    pub fn grammar(&mut self, grammar: impl Into<PathBuf>) -> &mut Self {
        self.grammars.push(grammar.into());
        self
    }

    /// Sets the output directory. Defaults to the `OUT_DIR` environment
    /// variable at [`generate`](Config::generate) time.
    pub fn out_dir(&mut self, out_dir: impl Into<PathBuf>) -> &mut Self {
        self.out_dir = Some(out_dir.into());
        self
    }

    /// Adds a grammar library search directory (the command line's `-lib`):
    /// imported grammars and `tokenVocab` `.tokens` files are looked up here
    /// after the importing grammar's own directory.
    pub fn lib_dir(&mut self, dir: impl Into<PathBuf>) -> &mut Self {
        self.lib_dirs.push(dir.into());
        self
    }

    /// Toggles listener (+ base listener) generation. Default: `true`.
    pub const fn listener(&mut self, generate: bool) -> &mut Self {
        self.gen_listener = generate;
        self
    }

    /// Toggles visitor (+ base visitor) generation. Default: `false`.
    pub const fn visitor(&mut self, generate: bool) -> &mut Self {
        self.gen_visitor = generate;
        self
    }

    /// Toggles printing of `cargo:rerun-if-changed` directives.
    /// Default: `true`.
    pub const fn cargo_directives(&mut self, print: bool) -> &mut Self {
        self.cargo_directives = print;
        self
    }

    /// Generates all configured grammars into the output directory, creating
    /// it when needed, and returns the paths written.
    ///
    /// # Errors
    ///
    /// Returns [`EmitError::Config`] when no grammars are configured or no
    /// output directory is available, [`EmitError::Compile`] or
    /// [`EmitError::Template`] when a grammar fails to generate, and
    /// [`EmitError::Write`] when an output file cannot be written.
    // Cargo's build-script protocol reads directives from stdout.
    #[allow(clippy::print_stdout)]
    pub fn generate(&self) -> Result<Vec<PathBuf>, EmitError> {
        if self.grammars.is_empty() {
            return Err(EmitError::Config("no grammars configured".to_owned()));
        }
        let out_dir = match &self.out_dir {
            Some(dir) => dir.clone(),
            None => PathBuf::from(std::env::var_os("OUT_DIR").ok_or_else(|| {
                EmitError::Config(
                    "no output directory: call Config::out_dir or run from a build \
                     script (OUT_DIR is not set)"
                        .to_owned(),
                )
            })?),
        };
        if self.cargo_directives {
            for grammar in &self.grammars {
                println!("cargo:rerun-if-changed={}", grammar.display());
                if let Some(parent) = grammar.parent().filter(|dir| !dir.as_os_str().is_empty()) {
                    println!("cargo:rerun-if-changed={}", parent.display());
                }
            }
            for dir in &self.lib_dirs {
                println!("cargo:rerun-if-changed={}", dir.display());
            }
        }
        std::fs::create_dir_all(&out_dir).map_err(|source| EmitError::Write {
            path: out_dir.clone(),
            source,
        })?;
        let mut written = Vec::new();
        for grammar in &self.grammars {
            let files = emit_files_with_flags(
                grammar,
                &self.lib_dirs,
                self.gen_listener,
                self.gen_visitor,
            )?;
            for file in &files {
                let path = out_dir.join(&file.name);
                std::fs::write(&path, &file.content).map_err(|source| EmitError::Write {
                    path: path.clone(),
                    source,
                })?;
                written.push(path);
            }
        }
        Ok(written)
    }
}
