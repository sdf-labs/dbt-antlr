// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Build-script entry point: generate recognizers during `cargo build`, in
//! the style of `prost-build` or `lalrpop`.

use std::path::PathBuf;

use crate::OptimizationOptions;
#[cfg(feature = "generator")]
use crate::emission::emit_files_with_options;
use crate::error::{Diagnostic, EmitError};

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
/// # default-features = false skips the `fancy` diagnostics rendering the
/// # command-line tool uses; it shrinks the build-dependency tree by ~40%.
/// dbt-antlr-codegen = { version = "0.1", default-features = false, features = ["generator"] }
/// ```
///
/// ```rust,no_run
/// // build.rs
/// dbt_antlr_codegen::Config::new("grammars/Expr.g4")
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
    optimization: OptimizationOptions,
    pinned_release_version: Option<String>,
    pinned_release_sha256: Option<String>,
    pinned_release_base_url: Option<String>,
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
            optimization: OptimizationOptions::default(),
            pinned_release_version: None,
            pinned_release_sha256: None,
            pinned_release_base_url: None,
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

    /// Precomputes static lexer DFA tables (`--static-lexer-dfa`, lexer scope
    /// only). Default: `false`.
    /// When the flag is off, the generated output is byte-identical to a run
    /// without it.
    pub const fn static_lexer_dfa(&mut self, enabled: bool) -> &mut Self {
        self.optimization.static_lexer_dfa(enabled);
        self
    }

    /// Precomputes static parser DFA tables (`--static-parser-dfa`, parser
    /// scope only). Default: `false`.
    pub const fn static_parser_dfa(&mut self, enabled: bool) -> &mut Self {
        self.optimization.static_parser_dfa(enabled);
        self
    }

    /// Generates only fully static lexers and parsers (`--static-dfa-only`),
    /// which never fall back to the adaptive ATN simulation. Default: `false`.
    /// See [`OptimizationOptions::static_dfa_only`].
    pub const fn static_dfa_only(&mut self, enabled: bool) -> &mut Self {
        self.optimization.static_dfa_only(enabled);
        self
    }

    /// Renders the parser decision report of the static parser DFA pass into
    /// [`Generation::reports`] (`--report-decisions`, printed to stdout by the
    /// CLI). Implies report-only mode.
    pub const fn report_decisions(&mut self, enabled: bool) -> &mut Self {
        self.optimization.report_decisions(enabled);
        self
    }

    /// Renders the static parser DFA tables matching `spec` into
    /// [`Generation::reports`] (`--dump-dfas <SPEC>`, printed to stdout by the
    /// CLI). Implies report-only mode.
    pub fn dump_dfas(&mut self, spec: impl Into<String>) -> &mut Self {
        self.optimization.dump_dfas(spec);
        self
    }

    /// Returns the optimization selection for in-place configuration.
    /// See [`OptimizationOptions`] for the available options. The
    /// convenience setters on [`Config`] delegate to it.
    pub const fn optimization(&mut self) -> &mut OptimizationOptions {
        &mut self.optimization
    }

    /// Marks `name` as an explicit parser entry rule (the command line's
    /// `--entry-rule`). Repeatable.
    pub fn entry_rule(&mut self, name: impl Into<String>) -> &mut Self {
        self.optimization.entry_rule(name);
        self
    }

    /// Deletes parser rules that are unreachable from the entry rules (the
    /// command line's `--prune-unreachable-rules`). Default: `false`.
    pub const fn prune_unreachable_rules(&mut self, enabled: bool) -> &mut Self {
        self.optimization.prune_unreachable_rules(enabled);
        self
    }

    /// Inlines trivial pure parser rules into their call sites (the command
    /// line's `--inline-trivial-rules`). Default: `false`.
    pub const fn inline_trivial_rules(&mut self, enabled: bool) -> &mut Self {
        self.optimization.inline_trivial_rules(enabled);
        self
    }

    /// Collapses precedence-ladder rule chains into left-recursive rules
    /// (`--optimize-precedence-ladders`). Default: `false`.
    pub const fn optimize_precedence_ladders(&mut self, enabled: bool) -> &mut Self {
        self.optimization.optimize_precedence_ladders(enabled);
        self
    }

    /// Compiles, analyzes, and optimizes the input grammars without writing
    /// output files (`--report-only`, `--dry-run`, `-n`). Default: `false`.
    pub const fn report_only(&mut self, enabled: bool) -> &mut Self {
        self.optimization.report_only(enabled);
        self
    }

    /// Switches generation to a pinned prebuilt `dbt-antlr-codegen` release
    /// binary (requires the `download` feature).
    ///
    /// Instead of compiling and running the generator in-process, the
    /// release archive for the build host's target triple is downloaded from
    /// the GitHub release `dbt-antlr-codegen-v{version}`, verified against a
    /// SHA-256 checksum, cached under the output directory, and invoked with
    /// the configured arguments. Use this in build environments where the
    /// generator's dependency tree is too expensive to compile:
    ///
    /// ```toml
    /// [build-dependencies]
    /// dbt-antlr-codegen = { version = "0.1", default-features = false, features = ["download"] }
    /// ```
    ///
    /// The pinned `version` is independent of this crate's own version.
    /// Requires `curl` on `PATH`.
    #[cfg(feature = "download")]
    pub fn pinned_release(&mut self, version: impl Into<String>) -> &mut Self {
        self.pinned_release_version = Some(version.into());
        self
    }

    /// Pins the expected SHA-256 checksum of the release archive (requires
    /// the `download` feature and [`Config::pinned_release`]). When unset,
    /// the checksum is read from the `.sha256` file published next to the
    /// archive.
    #[cfg(feature = "download")]
    pub fn pinned_release_sha256(&mut self, sha256: impl Into<String>) -> &mut Self {
        self.pinned_release_sha256 = Some(sha256.into());
        self
    }

    /// Overrides the release download base URL (requires the `download`
    /// feature), for mirrors or GitHub Enterprise hosts. Defaults to the
    /// `sdf-labs/dbt-antlr` GitHub releases.
    #[cfg(feature = "download")]
    pub fn pinned_release_base_url(&mut self, base_url: impl Into<String>) -> &mut Self {
        self.pinned_release_base_url = Some(base_url.into());
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
        Ok(self.generate_detailed()?.into_written_files())
    }

    /// [`generate`](Config::generate) with the rest of the run output:
    /// compiler diagnostics (for example unreachable parser rules), transform
    /// notes (for example `pruned: ...` lines), and the reports of the report
    /// modes (`report_decisions`, `dump_dfas`).
    ///
    /// # Errors
    ///
    /// See [`generate`](Config::generate).
    // Cargo's build-script protocol reads directives from stdout.
    #[allow(clippy::print_stdout)]
    pub fn generate_detailed(&self) -> Result<Generation, EmitError> {
        if self.grammars.is_empty() {
            return Err(EmitError::Config("no grammars configured".to_owned()));
        }
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
        if let Some(version) = &self.pinned_release_version {
            let out_dir = self.output_dir()?;
            create_out_dir(&out_dir)?;
            return self.generate_pinned(version, &out_dir);
        }
        // Report-only runs render their reports without touching the output
        // directory, so it stays optional here.
        let out_dir = if self.optimization.is_report_only() {
            None
        } else {
            let out_dir = self.output_dir()?;
            create_out_dir(&out_dir)?;
            Some(out_dir)
        };
        self.generate_in_process(out_dir.as_deref())
    }

    /// The configured output directory, defaulting to Cargo's `OUT_DIR`.
    fn output_dir(&self) -> Result<PathBuf, EmitError> {
        if let Some(dir) = &self.out_dir {
            return Ok(dir.clone());
        }
        let out_dir = std::env::var_os("OUT_DIR").ok_or_else(|| {
            EmitError::Config(
                "no output directory: call Config::out_dir or run from a build \
                 script (OUT_DIR is not set)"
                    .to_owned(),
            )
        })?;
        Ok(PathBuf::from(out_dir))
    }

    #[cfg(feature = "download")]
    fn generate_pinned(
        &self,
        version: &str,
        out_dir: &std::path::Path,
    ) -> Result<Generation, EmitError> {
        if self.optimization.has_parser_reports() {
            return Err(EmitError::Config(
                "report_decisions and dump_dfas need the in-process generator; \
                 pinned-release generation cannot render their reports"
                    .to_owned(),
            ));
        }
        let release = crate::download::PinnedRelease {
            version: version.to_owned(),
            expected_sha256: self.pinned_release_sha256.clone(),
            base_url: self.pinned_release_base_url.clone(),
        };
        let written = crate::download::generate_with_pinned_binary(
            &release,
            &self.grammars,
            &self.lib_dirs,
            self.gen_listener,
            self.gen_visitor,
            &self.optimization,
            out_dir,
        )?;
        Ok(Generation {
            written,
            diagnostics: Vec::new(),
            optimization_messages: Vec::new(),
            reports: Vec::new(),
        })
    }

    #[cfg(not(feature = "download"))]
    fn generate_pinned(
        &self,
        version: &str,
        out_dir: &std::path::Path,
    ) -> Result<Generation, EmitError> {
        let _ = (version, out_dir);
        let _ = (&self.pinned_release_sha256, &self.pinned_release_base_url);
        Err(EmitError::Config(
            "pinned_release requires the `download` feature of dbt-antlr-codegen".to_owned(),
        ))
    }

    /// Generates in process. `out_dir` is `None` in report-only mode, where
    /// the run renders reports instead of writing files.
    #[cfg(feature = "generator")]
    fn generate_in_process(
        &self,
        out_dir: Option<&std::path::Path>,
    ) -> Result<Generation, EmitError> {
        let mut written = Vec::new();
        let mut diagnostics = Vec::new();
        let mut optimization_messages = Vec::new();
        let mut reports = Vec::new();
        for grammar in &self.grammars {
            let output = emit_files_with_options(
                grammar,
                &self.lib_dirs,
                self.gen_listener,
                self.gen_visitor,
                &self.optimization,
            )?;
            if let Some(out_dir) = out_dir {
                for file in &output.files {
                    let path = out_dir.join(&file.name);
                    std::fs::write(&path, &file.content).map_err(|source| EmitError::Write {
                        path: path.clone(),
                        source,
                    })?;
                    written.push(path);
                }
                write_optimization_manifest(out_dir, output.optimization_manifest, &mut written)?;
            }
            diagnostics.extend(output.diagnostics);
            optimization_messages.extend(output.optimization_messages);
            reports.extend(output.reports);
        }
        Ok(Generation {
            written,
            diagnostics,
            optimization_messages,
            reports,
        })
    }

    #[cfg(not(feature = "generator"))]
    fn generate_in_process(
        &self,
        out_dir: Option<&std::path::Path>,
    ) -> Result<Generation, EmitError> {
        let _ = out_dir;
        Err(EmitError::Config(
            "this build of dbt-antlr-codegen was compiled without the `generator` feature; \
             enable it or use Config::pinned_release"
                .to_owned(),
        ))
    }
}

fn create_out_dir(out_dir: &std::path::Path) -> Result<(), EmitError> {
    std::fs::create_dir_all(out_dir).map_err(|source| EmitError::Write {
        path: out_dir.to_path_buf(),
        source,
    })
}

/// Writes `optimizations.txt` next to the generated files when the
/// selected transforms produced a manifest, and removes a stale manifest
/// from an earlier optimized run when they did not. Legacy
/// `optimizations.json` manifests (written by releases that rendered JSON)
/// are removed as well.
#[cfg(feature = "generator")]
fn write_optimization_manifest(
    out_dir: &std::path::Path,
    manifest: Option<String>,
    written: &mut Vec<PathBuf>,
) -> Result<(), EmitError> {
    let path = out_dir.join("optimizations.txt");
    let legacy_path = out_dir.join("optimizations.json");
    match manifest {
        Some(manifest) => {
            std::fs::write(&path, manifest).map_err(|source| EmitError::Write {
                path: path.clone(),
                source,
            })?;
            written.push(path);
        }
        None => remove_stale_manifest(&path)?,
    }
    remove_stale_manifest(&legacy_path)?;
    Ok(())
}

#[cfg(feature = "generator")]
fn remove_stale_manifest(path: &std::path::Path) -> Result<(), EmitError> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(EmitError::Write {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// Files and non-fatal output of [`Config::generate_detailed`].
#[derive(Clone, Debug)]
pub struct Generation {
    written: Vec<PathBuf>,
    diagnostics: Vec<Diagnostic>,
    optimization_messages: Vec<String>,
    reports: Vec<String>,
}

impl Generation {
    /// Every file written into the output directory, in write order.
    pub fn written_files(&self) -> &[PathBuf] {
        &self.written
    }

    /// Diagnostics of the run that did not abort it: warnings (for example
    /// unreachable parser rule warnings) and errors a caller must surface
    /// (for example an unmatched `--dump-dfas` selector). Empty for
    /// pinned-release generation, where diagnostics stay on the release
    /// binary's stderr.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Transform notes of the run (for example `pruned: ...` lines).
    pub fn optimization_messages(&self) -> &[String] {
        &self.optimization_messages
    }

    /// Rendered reports of the report modes (`report_decisions` and
    /// `dump_dfas`), in selection order.
    pub fn reports(&self) -> &[String] {
        &self.reports
    }

    /// Consumes the generation and returns the written files.
    pub fn into_written_files(self) -> Vec<PathBuf> {
        self.written
    }
}
