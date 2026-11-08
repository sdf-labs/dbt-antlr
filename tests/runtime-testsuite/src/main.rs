// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
#![allow(clippy::print_stderr, clippy::print_stdout)]

use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

mod custom_descriptors;
mod upstream;

/// Vendored case tree (`<Group>/<Name>.txt`) inside the testsuite package.
const VENDORED_CORPUS: &str = "tests/runtime-testsuite/corpus";
/// Tool jar (a `StringTemplate` 4 carrier) used only by `--export-corpus`.
const ANTLR_JAR_ENV: &str = "ANTLR4_JAR";
/// Upstream case tree, an `--export-corpus` input only.
const EXPORT_SOURCE_ENV: &str = "ANTLR4_RUNTIME_TESTSUITE";
/// Template group, an `--export-corpus` input only.
const TEMPLATE_PATH: &str = "resources/org/antlr/v4/test/runtime/templates/Rust.test.stg";
const DESCRIPTOR_PATH: &str = "resources/org/antlr/v4/test/runtime/descriptors";
const RENDER_DRIVER: &str = "tests/runtime-testsuite/java/RenderGrammar.java";
const RUNTIME_PACKAGE: &str = "dbt-antlr-runtime";
/// Guard against a single pathological case stalling the whole sweep; the
/// upstream JVM harness bounds execution the same way.
const RUN_TIMEOUT: Duration = Duration::from_secs(300);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse()?;
    if args.work_dir.exists() && !args.keep {
        fs::remove_dir_all(&args.work_dir)?;
    }
    fs::create_dir_all(&args.work_dir)?;

    if let Some(export) = &args.export {
        let descriptors = load_descriptors(&export.source_root, &args)?;
        ensure_descriptors_loaded(&descriptors, &export.source_root)?;
        let render = RenderContext::prepare(&args.work_dir, &args.workspace_root, export)?;
        let exported = export_corpus(
            &args,
            &render,
            &export.source_root,
            &descriptors,
            &export.out_dir,
        )
        .map_err(|error| format!("corpus export: {error}"))?;
        println!(
            "exported {exported} descriptor(s) to {}",
            export.out_dir.display()
        );
        return Ok(());
    }

    let descriptors = load_descriptors(&args.corpus, &args)?;
    ensure_descriptors_loaded(&descriptors, &args.corpus)?;
    let mut summary = Summary::default();
    if args.static_lexer_dfa {
        println!("mode: static-lexer-dfa (generating every case with --static-lexer-dfa)");
    }
    if args.static_parser_dfa {
        println!("mode: static-parser-dfa (generating every case with --static-parser-dfa)");
    }
    // Skips are classified up front (in descriptor order) so workers only see
    // runnable cases; `--limit` counts runnable cases and stops classification
    // there.
    let mut runnable = Vec::new();
    for descriptor in descriptors {
        if let Some(reason) =
            skip_reason(&descriptor, args.static_lexer_dfa, args.static_parser_dfa)
        {
            summary.skipped += 1;
            println!("skip {}: {reason}", descriptor.id());
            continue;
        }
        runnable.push(descriptor);
        if args.limit.is_some_and(|limit| runnable.len() >= limit) {
            break;
        }
    }
    summary.ran = runnable.len();

    let context = SweepContext::prepare(&args)?;
    let (passed, failures) = run_cases(&args, &context, &runnable);
    summary.passed = passed;
    summary.failed = failures.total();

    println!(
        "summary: {} passed, {} failed, {} skipped, {} run",
        summary.passed, summary.failed, summary.skipped, summary.ran
    );
    if summary.failed > 0 {
        println!("failures: {failures}");
    }

    if summary.failed == 0 {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "{} runtime-testsuite case(s) failed",
            summary.failed
        ))
        .into())
    }
}

/// Runs the runnable descriptors on a worker pool. Cases are fully
/// independent: each generates, compiles (against the read-only prebuilt
/// runtime rlib), and executes in its own case directory.
fn run_cases(
    args: &Args,
    context: &SweepContext<'_>,
    runnable: &[Descriptor],
) -> (usize, Failures) {
    let jobs = args.jobs.clamp(1, runnable.len().max(1));
    let next = AtomicUsize::new(0);
    let tally = Mutex::new((0_usize, Failures::default()));
    std::thread::scope(|scope| {
        for _ in 0..jobs {
            let next = &next;
            let tally = &tally;
            scope.spawn(move || {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(descriptor) = runnable.get(index) else {
                        break;
                    };
                    let failure = run_case(args, context, descriptor);
                    let mut tally = tally.lock().expect("result tally lock");
                    match failure {
                        None => tally.0 += 1,
                        Some(category) => tally.1.add(category),
                    }
                }
            });
        }
    });
    let tally = tally.lock().expect("result tally lock");
    (tally.0, tally.1.clone())
}

/// Runs one descriptor and reports its outcome; `println!`/`eprintln!` are
/// line-atomic, so per-case lines from parallel workers never interleave.
fn run_case(args: &Args, context: &SweepContext<'_>, descriptor: &Descriptor) -> Option<Category> {
    match run_descriptor(args, context, descriptor) {
        Ok(()) => {
            println!("PASS {}", descriptor.id());
            if let Err(error) = remove_descriptor_work_dir(args, descriptor) {
                eprintln!("warning: could not clean {}: {error}", descriptor.id());
            }
            None
        }
        Err(failure) => {
            // The full detail stays in the (preserved) case directory; the
            // console gets the category and the first lines of the detail.
            println!(
                "{} {}: {}",
                failure.category,
                descriptor.id(),
                failure.headline()
            );
            let detail = failure.detail_lines(20);
            if !detail.is_empty() {
                println!("{detail}");
            }
            if let Err(error) = failure.persist(args, descriptor) {
                eprintln!("warning: could not record {}: {error}", descriptor.id());
            }
            Some(failure.category)
        }
    }
}

/// Failure buckets reported per case, mirroring the pipeline stage that broke.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Category {
    /// `RenderGrammar.java` / `StringTemplate` rendering of the descriptor
    /// grammar failed.
    RenderFail,
    /// `dbt-antlr-codegen` exited non-zero or reported diagnostics.
    ToolFail,
    /// `rustc` rejected the generated recognizer plus harness `main.rs`.
    CompileFail,
    /// The binary ran but stdout differed from the descriptor `[output]`.
    RunDiffOutput,
    /// The binary ran, stdout matched, but stderr differed from `[errors]`.
    RunDiffErrors,
    /// The binary exited non-zero (panic, abort) or hit the run timeout.
    RunCrash,
}

impl Category {
    const fn as_str(self) -> &'static str {
        match self {
            Self::RenderFail => "RENDER-FAIL",
            Self::ToolFail => "TOOL-FAIL",
            Self::CompileFail => "COMPILE-FAIL",
            Self::RunDiffOutput => "RUN-DIFF-OUTPUT",
            Self::RunDiffErrors => "RUN-DIFF-ERRORS",
            Self::RunCrash => "RUN-CRASH",
        }
    }

    const ALL: [Self; 6] = [
        Self::RenderFail,
        Self::ToolFail,
        Self::CompileFail,
        Self::RunDiffOutput,
        Self::RunDiffErrors,
        Self::RunCrash,
    ];
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// One failed case: the category plus a multi-line human-readable detail.
#[derive(Debug)]
struct Failure {
    category: Category,
    detail: String,
}

impl Failure {
    fn new(category: Category, detail: impl Into<String>) -> Self {
        Self {
            category,
            detail: detail.into(),
        }
    }

    fn headline(&self) -> String {
        self.detail
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or("(no detail)")
            .trim()
            .chars()
            .take(200)
            .collect()
    }

    fn detail_lines(&self, limit: usize) -> String {
        self.detail
            .lines()
            .take(limit)
            .map(|line| format!("  {line}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Records the failure detail next to the preserved case artifacts.
    fn persist(&self, args: &Args, descriptor: &Descriptor) -> io::Result<()> {
        fs::write(
            descriptor_work_dir(args, descriptor).join("failure.txt"),
            format!("{} {}\n{}", self.category, descriptor.id(), self.detail),
        )
    }
}

/// Per-category failure counts for the sweep summary.
#[derive(Clone, Debug, Default)]
struct Failures {
    counts: [usize; 6],
}

impl Failures {
    fn add(&mut self, category: Category) {
        let index = Category::ALL
            .iter()
            .position(|candidate| *candidate == category)
            .expect("every category has a slot");
        self.counts[index] += 1;
    }

    fn total(&self) -> usize {
        self.counts.iter().sum()
    }
}

impl std::fmt::Display for Failures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        for (index, category) in Category::ALL.iter().enumerate() {
            if self.counts[index] > 0 {
                parts.push(format!("{}={}", category.as_str(), self.counts[index]));
            }
        }
        f.write_str(&parts.join(" "))
    }
}

/// Per-sweep artifacts prepared once so per-case work stays minimal: the
/// prebuilt `dbt-antlr` executable and the prebuilt `dbt-antlr-runtime` runtime
/// rlib that every case compiles against.
struct SweepContext<'a> {
    args: &'a Args,
    generator: PathBuf,
    runtime_rlib: PathBuf,
}

impl<'a> SweepContext<'a> {
    fn prepare(args: &'a Args) -> io::Result<Self> {
        let generator = prebuild_generator(args)?;
        let runtime_rlib = prebuild_runtime(args)?;
        Ok(Self {
            args,
            generator,
            runtime_rlib,
        })
    }

    /// Directory of `target/debug/deps` holding the runtime rlib and its
    /// transitive dependencies, passed to rustc as `-L dependency=`.
    fn deps_dir(&self) -> &Path {
        self.runtime_rlib
            .parent()
            .expect("runtime rlib lives in a deps directory")
    }
}

/// Render inputs for `--export-corpus`: the pinned tool jar (a carrier of
/// `StringTemplate` 4), the template group, and the precompiled render driver.
/// The Java single-file source launcher would recompile `RenderGrammar.java`
/// for every render.
struct RenderContext<'a> {
    antlr_jar: &'a Path,
    stg: &'a Path,
    classes: PathBuf,
}

impl<'a> RenderContext<'a> {
    fn prepare(
        work_dir: &Path,
        workspace_root: &Path,
        export: &'a ExportCorpus,
    ) -> io::Result<Self> {
        let classes = work_dir.join("stg-render-classes");
        fs::create_dir_all(&classes)?;
        run_checked(
            Command::new("javac")
                .arg("-cp")
                .arg(&export.antlr_jar)
                .arg("-d")
                .arg(&classes)
                .arg(workspace_root.join(RENDER_DRIVER)),
            "StringTemplate render driver compile",
        )?;
        Ok(Self {
            antlr_jar: &export.antlr_jar,
            stg: &export.stg,
            classes,
        })
    }

    /// `java -cp` separator-joined tool jar plus render driver classes.
    fn classpath(&self) -> String {
        let separator = if cfg!(windows) { ";" } else { ":" };
        format!(
            "{}{separator}{}",
            self.antlr_jar.display(),
            self.classes.display()
        )
    }
}

/// Builds `dbt-antlr-codegen` once and resolves its executable path from
/// cargo's JSON messages, honoring any `CARGO_TARGET_DIR` redirection.
fn prebuild_generator(args: &Args) -> io::Result<PathBuf> {
    let output = run_output(
        Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(args.workspace_root.join("Cargo.toml"))
            .arg("-p")
            .arg("dbt-antlr-codegen")
            .arg("--bin")
            .arg("dbt-antlr-codegen")
            .arg("--message-format=json"),
    )?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "dbt-antlr-codegen build failed\nstderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .filter(|line| line.contains("\"reason\":\"compiler-artifact\""))
        .filter(|line| json_string_value(line, "name").as_deref() == Some("dbt-antlr-codegen"))
        .filter_map(|line| json_string_value(line, "executable"))
        .map(PathBuf::from)
        .next_back()
        .ok_or_else(|| io::Error::other("cargo did not report the dbt-antlr executable"))
}

/// Builds the `dbt-antlr-runtime` runtime once and locates its rlib from cargo's
/// JSON artifact messages (mirrors `RustRunner.initRuntime`), honoring any
/// `CARGO_TARGET_DIR` redirection.
///
/// Both sweep modes use this same build: neither static walker is
/// feature-gated, so generated `--static-lexer-dfa`/`--static-parser-dfa` code
/// compiles against the plain runtime rlib (the lexer side:
/// `static_lexer_dfa`, `LexerATNSimulator::set_static_tables`,
/// `LexerATNSimulatorManager::new_static_lexer`; the parser side:
/// `dfa::static_parser_dfa` and `BaseParser::dfa_predict`). The
/// generation-time builders behind `static-dfa-build` live inside the
/// generator binary, not in the rlib the cases link.
fn prebuild_runtime(args: &Args) -> io::Result<PathBuf> {
    let output = run_output(
        Command::new("cargo")
            .arg("build")
            .arg("--manifest-path")
            .arg(args.workspace_root.join("Cargo.toml"))
            .arg("-p")
            .arg(RUNTIME_PACKAGE)
            .arg("--message-format=json"),
    )?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "{RUNTIME_PACKAGE} build failed\nstderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().rev() {
        if !line.contains("\"reason\":\"compiler-artifact\"")
            || json_string_value(line, "name").as_deref() != Some(RUNTIME_PACKAGE)
        {
            continue;
        }
        if let Some(rlib) = json_first_array_string(line, "filenames")
            && Path::new(&rlib)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rlib"))
        {
            return Ok(PathBuf::from(rlib));
        }
    }
    // Fallback for cargo versions that do not list filenames: pick the
    // newest runtime rlib from the target dir, like `findLibName`.
    let target_dir = env::var_os("CARGO_TARGET_DIR").map_or_else(
        || args.workspace_root.join("target"),
        |dir| absolutize(&args.workspace_root, PathBuf::from(dir)),
    );
    let deps = target_dir.join("debug/deps");
    let mut newest: Option<(PathBuf, std::time::SystemTime)> = None;
    for entry in fs::read_dir(&deps)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(OsStr::to_str) else {
            continue;
        };
        if !(name.starts_with("libdbt_antlr_runtime-")
            && Path::new(name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rlib")))
        {
            continue;
        }
        let modified = fs::metadata(&path).and_then(|m| m.modified()).ok();
        if let Some(modified) = modified
            && newest.as_ref().is_none_or(|(_, time)| modified > *time)
        {
            newest = Some((path, modified));
        }
    }
    newest.map(|(path, _)| path).ok_or_else(|| {
        io::Error::other(format!(
            "no libdbt_antlr_runtime-*.rlib under {}",
            deps.display()
        ))
    })
}

/// Decodes the JSON string value of `key` from one cargo message line.
///
/// A raw byte slice between quotes would keep JSON escapes — Windows path
/// separators arrive as `\\` — so the value is properly unescaped here. A
/// hand-rolled decoder is deliberate: the harness stays dependency-light
/// rather than carrying `serde_json` for one build message.
/// Decodes the first string element of the JSON array value of `key` from
/// one cargo message line (artifact `filenames` lists the rlib first).
fn json_first_array_string(line: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\":[\"");
    let start = line.find(&marker)? + marker.len();
    decode_json_string(&line[start..])
}

fn json_string_value(line: &str, key: &str) -> Option<String> {
    let marker = format!("\"{key}\":\"");
    let start = line.find(&marker)? + marker.len();
    decode_json_string(&line[start..])
}

fn decode_json_string(text: &str) -> Option<String> {
    let mut value = String::new();
    let mut chars = text.chars();
    while let Some(ch) = chars.next() {
        match ch {
            '"' => return Some(value),
            '\\' => match chars.next()? {
                'u' => {
                    let code: String = (&mut chars).take(4).collect();
                    value.push(char::from_u32(u32::from_str_radix(&code, 16).ok()?)?);
                }
                'n' => value.push('\n'),
                't' => value.push('\t'),
                'r' => value.push('\r'),
                'b' => value.push('\u{0008}'),
                'f' => value.push('\u{000C}'),
                other => value.push(other),
            },
            other => value.push(other),
        }
    }
    None
}

/// `--export-corpus <DIR>` plus the pinned artifacts the import renders from.
///
/// A re-import needs a JVM, the tool jar (a `StringTemplate` carrier), the
/// upstream case tree, and the template group; the sweep itself needs none of
/// them.
#[derive(Debug)]
struct ExportCorpus {
    out_dir: PathBuf,
    antlr_jar: PathBuf,
    /// Upstream descriptor directory (the `<Group>/<Name>.txt` tree).
    source_root: PathBuf,
    stg: PathBuf,
}

#[derive(Debug)]
struct Args {
    workspace_root: PathBuf,
    work_dir: PathBuf,
    /// Vendored case tree (`<Group>/<Name>.txt`) the sweep reads.
    corpus: PathBuf,
    group: Option<String>,
    case_name: Option<String>,
    limit: Option<usize>,
    keep: bool,
    /// Parallel case workers.
    jobs: usize,
    /// Generate every case with `--static-lexer-dfa` (static lexer DFA tables).
    static_lexer_dfa: bool,
    /// Generate every case with `--static-parser-dfa` (static parser SLL DFA
    /// tables).
    static_parser_dfa: bool,
    /// Writes the post-rendered descriptor corpus instead of running the sweep.
    export: Option<ExportCorpus>,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut antlr_jar = None;
        let mut export_source = None;
        let mut workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("testsuite package should live below the workspace root")
            .to_path_buf();
        let mut work_dir = None;
        let mut corpus = None;
        let mut group = None;
        let mut case_name = None;
        let mut limit = None;
        let mut keep = false;
        let mut jobs = None;
        let mut stg = None;
        let mut export_corpus = None;
        let mut static_lexer_dfa = false;
        let mut static_parser_dfa = false;

        let mut iter = env::args().skip(1);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--antlr-jar" => {
                    antlr_jar = Some(PathBuf::from(next_arg(&mut iter, "--antlr-jar")?));
                }
                "--export-source" => {
                    export_source = Some(PathBuf::from(next_arg(&mut iter, "--export-source")?));
                }
                "--workspace-root" => {
                    workspace_root = PathBuf::from(next_arg(&mut iter, &arg)?);
                }
                "--work-dir" => {
                    work_dir = Some(PathBuf::from(next_arg(&mut iter, "--work-dir")?));
                }
                "--corpus" => corpus = Some(PathBuf::from(next_arg(&mut iter, "--corpus")?)),
                "--group" => group = Some(next_arg(&mut iter, "--group")?),
                "--case" => {
                    let value = next_arg(&mut iter, "--case")?;
                    if let Some((case_group, name)) = value.split_once('/') {
                        group = Some(case_group.to_owned());
                        case_name = Some(name.to_owned());
                    } else {
                        case_name = Some(value);
                    }
                }
                "--limit" => {
                    let value = next_arg(&mut iter, "--limit")?;
                    limit = Some(
                        value
                            .parse::<usize>()
                            .map_err(|error| format!("invalid --limit {value:?}: {error}"))?,
                    );
                }
                "--keep" => keep = true,
                "--jobs" => {
                    let value = next_arg(&mut iter, "--jobs")?;
                    jobs = Some(
                        value
                            .parse::<usize>()
                            .ok()
                            .filter(|jobs| *jobs > 0)
                            .ok_or_else(|| format!("invalid --jobs {value:?}"))?,
                    );
                }
                "--stg" => stg = Some(PathBuf::from(next_arg(&mut iter, "--stg")?)),
                "--export-corpus" => {
                    export_corpus = Some(PathBuf::from(next_arg(&mut iter, "--export-corpus")?));
                }
                "--static-lexer-dfa" => static_lexer_dfa = true,
                "--static-parser-dfa" => static_parser_dfa = true,
                "--help" | "-h" => return Err(usage()),
                other => return Err(format!("unknown argument {other}\n\n{}", usage())),
            }
        }

        let current_dir = env::current_dir().map_err(|error| error.to_string())?;
        let workspace_root = absolutize(&current_dir, workspace_root);
        if export_corpus.is_some() && (corpus.is_some() || static_lexer_dfa || static_parser_dfa) {
            return Err(format!(
                "--corpus and the static-DFA modes do not apply to --export-corpus; \
                 use --group, --case or --limit to select cases\n\n{}",
                usage()
            ));
        }
        let corpus = absolutize(
            &current_dir,
            corpus.unwrap_or_else(|| workspace_root.join(VENDORED_CORPUS)),
        );
        // A re-import renders from the pinned upstream artifacts, downloaded
        // and checksum-verified into the workspace target/ cache. Explicit
        // flags and environment overrides win. The sweep never needs them.
        let export_flags_only = antlr_jar.is_some() || stg.is_some() || export_source.is_some();
        if export_corpus.is_none() && export_flags_only {
            return Err(format!(
                "--antlr-jar, --stg and --export-source only apply to --export-corpus\n\n{}",
                usage()
            ));
        }
        let export = match export_corpus {
            None => None,
            Some(out_dir) => {
                let antlr_jar = match explicit_or_env(antlr_jar, ANTLR_JAR_ENV) {
                    Some(path) => absolutize(&current_dir, path),
                    None => upstream::ensure_tool_jar(&workspace_root)?,
                };
                let source_root = match explicit_or_env(export_source, EXPORT_SOURCE_ENV) {
                    Some(path) => absolutize(&current_dir, path),
                    None => upstream::ensure_runtime_testsuite(&workspace_root)?,
                };
                let source_root =
                    resolve_descriptor_root(&source_root).map_err(|error| error.to_string())?;
                let stg = stg.map_or_else(
                    || default_stg_path(&source_root),
                    |stg| absolutize(&current_dir, stg),
                );
                if !stg.is_file() {
                    return Err(format!(
                        "Rust action template group not found at {}; pass --stg\n\n{}",
                        stg.display(),
                        usage()
                    ));
                }
                Some(ExportCorpus {
                    out_dir: absolutize(&current_dir, out_dir),
                    antlr_jar,
                    source_root,
                    stg,
                })
            }
        };

        // The default work dir is keyed on the generation mode so a
        // static-DFA sweep and an adaptive sweep (or two concurrent runs)
        // never wipe each other's case artifacts; the runtime rlib needs no
        // mode keying because the static walker is feature-ungated (both
        // modes compile cases against the same rlib — see prebuild_runtime).
        let work_dir = absolutize(
            &current_dir,
            work_dir.unwrap_or_else(|| {
                env::temp_dir().join(match (static_lexer_dfa, static_parser_dfa) {
                    (true, true) => "dbt-antlr-runtime-testsuite-static-both-dfa",
                    (true, false) => "dbt-antlr-runtime-testsuite-static-lexer-dfa",
                    (false, true) => "dbt-antlr-runtime-testsuite-static-parser-dfa",
                    (false, false) => "dbt-antlr-runtime-testsuite",
                })
            }),
        );
        // Every worker drives rustc of its own; past ~8 the extra workers
        // mostly fight each other for cores.
        let jobs = jobs.unwrap_or_else(|| {
            std::thread::available_parallelism().map_or(1, |cores| cores.get().min(8))
        });
        Ok(Self {
            workspace_root,
            work_dir,
            corpus,
            group,
            case_name,
            limit,
            keep,
            jobs,
            static_lexer_dfa,
            static_parser_dfa,
            export,
        })
    }
}

/// Finds the target `Rust.test.stg` next to the upstream case tree, whether
/// `--export-source` names the runtime-testsuite root or the descriptor
/// directory itself.
fn default_stg_path(descriptors: &Path) -> PathBuf {
    for ancestor in descriptors.ancestors() {
        let candidate = ancestor.join(TEMPLATE_PATH);
        if candidate.is_file() {
            return candidate;
        }
    }
    descriptors.join(TEMPLATE_PATH)
}

/// Resolves an optional CLI path from, in order, the explicit flag and an
/// environment override; `None` means fall back to the pinned upstream
/// release artifact.
fn explicit_or_env(explicit: Option<PathBuf>, env_key: &str) -> Option<PathBuf> {
    explicit.or_else(|| {
        env::var(env_key)
            .ok()
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
    })
}

fn next_arg(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value\n\n{}", usage()))
}

fn absolutize(base: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        base.join(path)
    }
}

fn usage() -> String {
    "usage: runtime-testsuite [--corpus PATH] [--case Group/Name] [--group Group] [--limit N] [--jobs N] [--keep] [--work-dir PATH] [--static-lexer-dfa] [--static-parser-dfa] [--export-corpus DIR [--export-source PATH] [--antlr-jar ANTLR.jar] [--stg PATH]] [--workspace-root PATH]\n\nDefaults: corpus tests/runtime-testsuite/corpus (vendored post-rendered cases); work dir $TMPDIR/dbt-antlr-runtime-testsuite (mirrors upstream, which stages every test under java.io.tmpdir), keyed -static-lexer-dfa in --static-lexer-dfa mode; --jobs min(cores, 8)\n\n--static-lexer-dfa generates every case with --static-lexer-dfa (static lexer DFA tables) and runs the same expected outputs, so the whole corpus doubles as a static-vs-adaptive lexer differential. Descriptors flagged showDFA are skipped in this mode: they print the lazily interned lexer DFA cache, which the static walker does not populate — they introspect the adaptive engine, not lexer behavior (the lexer analogue of upstream's parser static-DFA skips). --static-parser-dfa generates every case with --static-parser-dfa (static parser SLL DFA tables and the D5 escape handoff) and runs the same expected outputs, so the corpus doubles as a parser differential; showDiagnosticErrors/traceATN descriptors are skipped because they print the adaptive ATN simulation the tables bypass.\n\n--export-corpus re-imports the corpus: it renders the upstream descriptor grammars through the template group and writes <DIR>/<Group>/<Name>.txt. It needs a JVM; --export-source (ANTLR4_RUNTIME_TESTSUITE), --antlr-jar (ANTLR4_JAR) and --stg default to the pinned dbt-antlr4 2.0.0 artifacts, downloaded and checksum-verified into <workspace>/target/dbt-antlr-runtime-testsuite-cache. An unfiltered export replaces the group directories under DIR and keeps other files; a --group/--case/--limit filtered export rewrites only the selected cases."
        .to_owned()
}

#[derive(Debug, Default)]
struct Summary {
    ran: usize,
    passed: usize,
    failed: usize,
    skipped: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct Descriptor {
    pub(crate) group: String,
    pub(crate) name: String,
    pub(crate) test_type: String,
    pub(crate) grammar_name: String,
    /// Final grammar text for generation (`[grammar]` section).
    pub(crate) grammar_template: String,
    pub(crate) start_rule: String,
    pub(crate) input: String,
    pub(crate) output: String,
    pub(crate) errors: String,
    pub(crate) flags: String,
    pub(crate) skip_targets: Vec<String>,
    pub(crate) slave_grammar_templates: Vec<String>,
}

impl Descriptor {
    fn id(&self) -> String {
        format!("{}/{}", self.group, self.name)
    }

    fn is_parser(&self) -> bool {
        matches!(self.test_type.as_str(), "Parser" | "CompositeParser")
    }

    fn flag_lines(&self) -> impl Iterator<Item = &str> {
        self.flags
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
    }

    fn show_dfa(&self) -> bool {
        self.flag_lines().any(|flag| flag == "showDFA")
    }

    fn show_diagnostic_errors(&self) -> bool {
        self.flag_lines().any(|flag| flag == "showDiagnosticErrors")
    }

    fn trace_atn(&self) -> bool {
        self.flag_lines().any(|flag| flag == "traceATN")
    }

    /// Upstream defaults to building parse trees unless the descriptor opts
    /// out with the `notBuildParseTree` flag.
    fn build_parse_tree(&self) -> bool {
        !self.flag_lines().any(|flag| flag == "notBuildParseTree")
    }

    /// Upstream defaults to `LL` unless a `predictionMode=...` flag says
    /// otherwise (`RuntimeTestDescriptorParser`).
    fn prediction_mode(&self) -> String {
        self.flag_lines()
            .find_map(|flag| flag.strip_prefix("predictionMode="))
            .unwrap_or("LL")
            .to_owned()
    }
}

/// Resolves either the upstream `runtime-testsuite` root or the descriptor
/// directory itself to the concrete descriptor directory.
fn resolve_descriptor_root(path: &Path) -> io::Result<PathBuf> {
    let direct = path.join(DESCRIPTOR_PATH);
    if direct.is_dir() {
        return Ok(direct);
    }
    if path.ends_with("descriptors") && path.is_dir() {
        return Ok(path.to_path_buf());
    }
    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "descriptor root not found under {}; pass runtime-testsuite root or descriptors directory",
            path.display()
        ),
    ))
}

/// Loads descriptor files in stable order and applies the CLI group/case
/// filters before parsing.
fn load_descriptors(root: &Path, args: &Args) -> io::Result<Vec<Descriptor>> {
    let mut descriptors = Vec::new();
    let mut group_dirs = sorted_children(root)?;
    group_dirs.retain(|entry| entry.path.is_dir());
    for group_dir in group_dirs {
        let group = group_dir.name;
        if args.group.as_ref().is_some_and(|wanted| wanted != &group) {
            continue;
        }

        let mut files = sorted_children(&group_dir.path)?;
        files.retain(|entry| entry.path.extension() == Some(OsStr::new("txt")));
        for file in files {
            let name = file.name.trim_end_matches(".txt").to_owned();
            if args
                .case_name
                .as_ref()
                .is_some_and(|wanted| wanted != &name)
            {
                continue;
            }
            let text = fs::read_to_string(&file.path)?;
            descriptors.push(parse_descriptor(group.clone(), name, &text)?);
        }
    }
    descriptors.extend(
        custom_descriptors::custom_descriptors()
            .into_iter()
            .filter(|descriptor| {
                args.group
                    .as_ref()
                    .is_none_or(|wanted| wanted == &descriptor.group)
                    && args
                        .case_name
                        .as_ref()
                        .is_none_or(|wanted| wanted == &descriptor.name)
            }),
    );
    Ok(descriptors)
}

fn ensure_descriptors_loaded(descriptors: &[Descriptor], root: &Path) -> io::Result<()> {
    if descriptors.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "no runtime-testsuite descriptors selected under {}; \
                 the checkout may be incomplete or the requested filters matched no cases",
                root.display()
            ),
        ));
    }
    Ok(())
}

#[derive(Debug)]
struct DirEntryInfo {
    name: String,
    path: PathBuf,
}

fn sorted_children(path: &Path) -> io::Result<Vec<DirEntryInfo>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        entries.push(DirEntryInfo {
            name,
            path: entry.path(),
        });
    }
    entries.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(entries)
}

/// Parses ANTLR runtime-testsuite descriptor text into the subset this harness
/// needs for execution and output comparison.
///
/// Text before the first `[section]` line is provenance and is ignored.
fn parse_descriptor(group: String, name: String, text: &str) -> io::Result<Descriptor> {
    let mut current_section: Option<String> = None;
    let mut current_value = String::new();
    let mut sections = Vec::new();

    for line in text.lines() {
        if let Some(section) = section_name(line) {
            if let Some(field) = current_section.replace(section.to_owned()) {
                sections.push((field, current_value.clone()));
                current_value.clear();
            }
        } else if current_section.is_some() {
            current_value.push_str(line);
            current_value.push('\n');
        }
    }
    if let Some(field) = current_section {
        sections.push((field, current_value));
    }

    let mut descriptor = Descriptor {
        group,
        name,
        test_type: "Lexer".to_owned(),
        grammar_name: String::new(),
        grammar_template: String::new(),
        start_rule: String::new(),
        input: String::new(),
        output: String::new(),
        errors: String::new(),
        flags: String::new(),
        skip_targets: Vec::new(),
        slave_grammar_templates: Vec::new(),
    };

    for (section, value) in sections {
        let value = normalize_section_value(&value);
        match section.as_str() {
            "type" => descriptor.test_type = value,
            "grammar" => {
                descriptor.grammar_name = grammar_name(&value)?;
                descriptor.grammar_template = value;
            }
            "slaveGrammar" => {
                grammar_name(&value)?;
                descriptor.slave_grammar_templates.push(value);
            }
            "input" => descriptor.input = value,
            "output" => descriptor.output = value,
            "errors" => descriptor.errors = value,
            "flags" => descriptor.flags = value,
            "start" => descriptor.start_rule = value,
            "skip" => {
                descriptor.skip_targets = value.split_whitespace().map(str::to_owned).collect();
            }
            "notes" => {}
            other => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("unknown descriptor section {other:?}"),
                ));
            }
        }
    }

    Ok(descriptor)
}

/// Returns a descriptor section name, deliberately excluding token display
/// output such as `[@0,...]`.
fn section_name(line: &str) -> Option<&str> {
    if line.starts_with('[') && line.ends_with(']') && line.len() > 2 {
        let name = &line[1..line.len() - 1];
        match name {
            "notes" | "type" | "grammar" | "slaveGrammar" | "start" | "input" | "output"
            | "errors" | "flags" | "skip" => Some(name),
            _ => None,
        }
    } else {
        None
    }
}

/// Mirrors the upstream descriptor parser's section trimming and triple-quote
/// handling so expected stdout/stderr bytes compare correctly.
fn normalize_section_value(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with("\"\"\"") {
        remove_marker(trimmed, "\"\"\"")
    } else if trimmed.contains('\n') {
        let mut out = trimmed.to_owned();
        out.push('\n');
        out
    } else {
        trimmed.to_owned()
    }
}

fn remove_marker(value: &str, marker: &str) -> String {
    let mut out = String::new();
    let mut rest = value;
    while let Some(index) = rest.find(marker) {
        out.push_str(&rest[..index]);
        rest = &rest[index + marker.len()..];
    }
    out.push_str(rest);
    out
}

fn grammar_name(grammar: &str) -> io::Result<String> {
    let first_line = grammar.lines().next().unwrap_or_default();
    let Some(start) = first_line.find("grammar ") else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing grammar declaration in {first_line:?}"),
        ));
    };
    let Some(stop) = first_line.find(';') else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("missing grammar declaration semicolon in {first_line:?}"),
        ));
    };
    Ok(first_line[start + "grammar ".len()..stop].to_owned())
}

/// Classifies descriptors the upstream harness itself skips for the Rust
/// target via the `[skip]` section (`RuntimeTestDescriptor.ignore`). In
/// `static_lexer_dfa` mode, `showDFA` lexer descriptors are additionally skipped:
/// their expected output includes a dump of the DFA cache the adaptive
/// lexer simulator interns on demand, and the static walker never interns
/// those states (a fully static lexer elides the ATN entirely). The dump
/// inspects adaptive-engine internals, so the descriptor has no
/// static-mode meaning; token output of those grammars still gets
/// differential coverage from their non-`showDFA` siblings.
fn skip_reason(
    descriptor: &Descriptor,
    static_lexer_dfa: bool,
    static_parser_dfa: bool,
) -> Option<String> {
    if descriptor
        .skip_targets
        .iter()
        .any(|target| target == "Rust")
    {
        return Some("[skip] section lists the Rust target".to_owned());
    }
    if !(descriptor.is_parser()
        || matches!(descriptor.test_type.as_str(), "Lexer" | "CompositeLexer"))
    {
        return Some(format!(
            "descriptor type {:?} is unknown",
            descriptor.test_type
        ));
    }
    if static_lexer_dfa && descriptor.show_dfa() {
        return Some(
            "showDFA prints the adaptive lexer DFA cache, which the static walker does not populate"
                .to_owned(),
        );
    }
    if static_parser_dfa && (descriptor.show_diagnostic_errors() || descriptor.trace_atn()) {
        return Some(
            "showDiagnosticErrors/traceATN print the adaptive ATN simulation, which the static parser tables bypass"
                .to_owned(),
        );
    }
    None
}

/// Renders one grammar template through the target `.test.stg` group using
/// the `StringTemplate` engine bundled in the tool jar, mirroring upstream
/// `RuntimeTests.prepareGrammars`, and retargets the result to the current
/// runtime API.
fn render_descriptor_grammar(
    render: &RenderContext<'_>,
    case_dir: &Path,
    tag: &str,
    grammar_template: &str,
) -> Result<RenderedGrammar, Failure> {
    let template_path = case_dir.join(format!("{tag}.template.g4"));
    let rendered_path = case_dir.join(format!("{tag}.rendered.g4"));
    let io = |stage: &'static str| {
        move |error: io::Error| {
            Failure::new(Category::RenderFail, format!("{tag} {stage}: {error}"))
        }
    };
    fs::write(&template_path, grammar_template).map_err(io("template write"))?;
    let output = Command::new("java")
        .arg("-cp")
        .arg(render.classpath())
        .arg("RenderGrammar")
        .arg(render.stg)
        .arg(&template_path)
        .arg(&rendered_path)
        .output()
        .map_err(io("render spawn"))?;
    if !output.status.success() {
        return Err(Failure::new(
            Category::RenderFail,
            format!(
                "{tag}: StringTemplate grammar render failed\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }
    let rendered = fs::read_to_string(&rendered_path).map_err(io("rendered read"))?;
    let (text, shims) = apply_descriptor_shims(&rendered);
    Ok(RenderedGrammar { text, shims })
}

/// One rendered grammar template: the text fed to the generator plus the ids
/// of the [`DESCRIPTOR_SHIMS`] rules it needed.
struct RenderedGrammar {
    text: String,
    shims: Vec<&'static str>,
}

/// One retarget of upstream's rendered action text to the current runtime API.
///
/// `id` names the rule in the provenance header of the vendored corpus file
/// (`tests/runtime-testsuite/corpus`), where the rewrite is baked in.
struct DescriptorShim {
    id: &'static str,
    from: &'static str,
    to: &'static str,
}

/// Rewrites the fork's descriptor action text, which still targets the old
/// runtime crate name, eager lexer position fields, and `Option`-shaped
/// `BaseLexer::input`/`interpreter` fields.
const DESCRIPTOR_SHIMS: &[DescriptorShim] = &[
    DescriptorShim {
        id: "runtime-crate-name",
        from: "dbt_antlr4",
        to: "dbt_antlr_runtime",
    },
    DescriptorShim {
        id: "token-start-column-getter",
        from: "recog.token_start_column",
        to: "recog.get_token_start_column()",
    },
    DescriptorShim {
        id: "lexer-token-start-column",
        from: "this.token_start_column",
        to: "0",
    },
    DescriptorShim {
        id: "lexer-token-start-line",
        from: "this.token_start_line",
        to: "0",
    },
    DescriptorShim {
        id: "simulator-set-line",
        from: "self.set_line(line);",
        to: "",
    },
    DescriptorShim {
        id: "simulator-set-char-position",
        from: "self.set_char_position_in_line(charPositionInLine);",
        to: "",
    },
    DescriptorShim {
        id: "lexer-input-field",
        from: "let mut input = this.input.take().unwrap();",
        to: "let input = &mut this.input;",
    },
    DescriptorShim {
        id: "lexer-interpreter-field",
        from: "this.interpreter.as_mut().unwrap().resetAcceptPosition(&mut input,",
        to: "this.interpreter.resetAcceptPosition(input,",
    },
    DescriptorShim {
        id: "lexer-input-restore",
        from: "this.input = Some(input);",
        to: "",
    },
    DescriptorShim {
        id: "simulator-reset-accept-position",
        from: "resetAcceptPosition(&mut self",
        to: "resetAcceptPosition(&self",
    },
];

/// Applies [`DESCRIPTOR_SHIMS`] and returns the rewritten text plus the ids of
/// the rules that matched, in table order.
fn apply_descriptor_shims(rendered: &str) -> (String, Vec<&'static str>) {
    let mut text = rendered.to_owned();
    let mut applied = Vec::new();
    for shim in DESCRIPTOR_SHIMS {
        if text.contains(shim.from) {
            text = text.replace(shim.from, shim.to);
            applied.push(shim.id);
        }
    }
    (text, applied)
}

/// Renders every upstream descriptor grammar and writes the post-rendered
/// corpus tree `<out>/<Group>/<Name>.txt`.
///
/// An unfiltered export replaces the group directories under `out` and keeps
/// other files (`README.md`) in place. A `--group`/`--case`/`--limit` filtered
/// export rewrites only the selected files.
fn export_corpus(
    args: &Args,
    render: &RenderContext<'_>,
    root: &Path,
    descriptors: &[Descriptor],
    out_dir: &Path,
) -> io::Result<usize> {
    let filtered = args.group.is_some() || args.case_name.is_some() || args.limit.is_some();
    if out_dir.exists() && !filtered {
        for group_dir in sorted_children(out_dir)? {
            if group_dir.path.is_dir() {
                fs::remove_dir_all(&group_dir.path)?;
            }
        }
    }
    let scratch = args.work_dir.join("corpus-export");
    if scratch.exists() {
        fs::remove_dir_all(&scratch)?;
    }
    fs::create_dir_all(&scratch)?;

    let by_id: HashMap<String, &Descriptor> = descriptors
        .iter()
        .map(|descriptor| (descriptor.id(), descriptor))
        .collect();

    let mut exported = 0;
    'cases: for group_dir in sorted_children(root)? {
        if !group_dir.path.is_dir() {
            continue;
        }
        let mut files = sorted_children(&group_dir.path)?;
        files.retain(|entry| entry.path.extension() == Some(OsStr::new("txt")));
        for file in files {
            if args.limit.is_some_and(|limit| exported >= limit) {
                break 'cases;
            }
            let name = file.name.trim_end_matches(".txt");
            let id = format!("{}/{}", group_dir.name, name);
            let Some(descriptor) = by_id.get(&id) else {
                continue;
            };
            let case_scratch = scratch.join(id.replace('/', "_"));
            fs::create_dir_all(&case_scratch)?;
            let main = render_descriptor_grammar(
                render,
                &case_scratch,
                "main",
                &descriptor.grammar_template,
            )
            .map_err(render_error)?;
            let mut shims = main.shims;
            let mut slaves = Vec::with_capacity(descriptor.slave_grammar_templates.len());
            for (index, template) in descriptor.slave_grammar_templates.iter().enumerate() {
                let slave = render_descriptor_grammar(
                    render,
                    &case_scratch,
                    &format!("slave{index}"),
                    template,
                )
                .map_err(render_error)?;
                shims.extend(slave.shims);
                slaves.push(slave.text);
            }
            let raw = fs::read_to_string(&file.path)?;
            let body = rewrite_grammar_sections(&raw, &main.text, &slaves);
            let target_dir = out_dir.join(&group_dir.name);
            fs::create_dir_all(&target_dir)?;
            let header = corpus_header(&id, &shims);
            fs::write(
                target_dir.join(format!("{name}.txt")),
                format!("{header}{body}"),
            )?;
            exported += 1;
        }
    }
    Ok(exported)
}

/// Converts a render failure to the export path's error type.
fn render_error(failure: Failure) -> io::Error {
    io::Error::other(format!("{}: {}", failure.category, failure.detail))
}

/// Rewrites the `[grammar]` and `[slaveGrammar]` bodies of one descriptor file
/// with their rendered text; every other byte of the file is preserved.
fn rewrite_grammar_sections(raw: &str, main: &str, slaves: &[String]) -> String {
    let mut preamble: Vec<&str> = Vec::new();
    let mut sections: Vec<(&str, Vec<&str>)> = Vec::new();
    for line in raw.split('\n') {
        if let Some(name) = section_name(line) {
            sections.push((name, Vec::new()));
        } else if let Some((_, body)) = sections.last_mut() {
            body.push(line);
        } else {
            preamble.push(line);
        }
    }
    let slave_sections = sections
        .iter()
        .filter(|(name, _)| *name == "slaveGrammar")
        .count();
    assert_eq!(
        slave_sections,
        slaves.len(),
        "descriptor has {slave_sections} [slaveGrammar] section(s) but {} rendered slave grammar(s)",
        slaves.len()
    );

    let mut out: Vec<String> = Vec::with_capacity(raw.len() + main.len());
    out.extend(preamble.into_iter().map(str::to_owned));
    let mut slaves = slaves.iter();
    for (name, body) in sections {
        out.push(format!("[{name}]"));
        match name {
            "grammar" => push_rendered_section(&mut out, main),
            "slaveGrammar" => {
                let rendered = slaves
                    .next()
                    .expect("slave grammar count was checked above");
                push_rendered_section(&mut out, rendered);
            }
            _ => out.extend(body.into_iter().map(str::to_owned)),
        }
    }
    out.join("\n")
}

/// Writes one rendered grammar body followed by the blank line that separates
/// sections in the descriptor format.
fn push_rendered_section(out: &mut Vec<String>, rendered: &str) {
    out.extend(
        rendered
            .trim_end_matches('\n')
            .split('\n')
            .map(str::to_owned),
    );
    out.push(String::new());
}

/// Provenance preamble for one corpus file. It names the source descriptor and
/// the [`DESCRIPTOR_SHIMS`] rules the import applied; the descriptor parser
/// ignores text before the first section, so it never reaches the sweep.
fn corpus_header(id: &str, shims: &[&'static str]) -> String {
    let mut header = format!(
        "# Vendored conformance corpus file; see corpus/README.md.\n\
         # Source: {} descriptors/{id}.txt\n\
         # Grammar sections are rendered Rust: edit them here.\n",
        upstream::RELEASE_TAG,
    );
    let mut unique: Vec<&'static str> = Vec::new();
    for &shim in shims {
        if !unique.contains(&shim) {
            unique.push(shim);
        }
    }
    if !unique.is_empty() {
        header.push_str("# Local deltas: ");
        header.push_str(&unique.join(", "));
        header.push('\n');
    }
    header
}

/// Runs one descriptor through the full pipeline: `dbt-antlr` codegen, harness
/// `main.rs`, `rustc` against the prebuilt runtime rlib, execution, and output
/// comparison.
fn run_descriptor(
    args: &Args,
    context: &SweepContext<'_>,
    descriptor: &Descriptor,
) -> Result<(), Failure> {
    let case_dir = descriptor_work_dir(args, descriptor);
    if case_dir.exists() {
        fs::remove_dir_all(&case_dir).map_err(harness_io("case dir reset"))?;
    }
    fs::create_dir_all(&case_dir).map_err(harness_io("case dir create"))?;

    // Case grammars are post-rendered in the vendored corpus, so they reach
    // the generator as-is.
    let grammar_path = case_dir.join(format!("{}.g4", descriptor.grammar_name));
    fs::write(&grammar_path, &descriptor.grammar_template).map_err(harness_io("grammar write"))?;
    for (index, slave) in descriptor.slave_grammar_templates.iter().enumerate() {
        let slave_name = grammar_name(slave)
            .map_err(|error| Failure::new(Category::ToolFail, format!("slave{index}: {error}")))?;
        let slave_path = case_dir.join(format!("{slave_name}.g4"));
        fs::write(&slave_path, slave).map_err(harness_io("slave grammar write"))?;
    }

    generate_rust(context, &case_dir, &grammar_path, descriptor)?;
    write_main_rs(&case_dir, descriptor)?;
    compile_case(context, &case_dir)?;
    execute_case(&case_dir, descriptor)
}

fn descriptor_work_dir(args: &Args, descriptor: &Descriptor) -> PathBuf {
    args.work_dir.join(safe_case_dir(&descriptor.id()))
}

/// Deletes successful descriptor output unless the caller asked to keep cases
/// around for inspection.
fn remove_descriptor_work_dir(args: &Args, descriptor: &Descriptor) -> io::Result<()> {
    if args.keep {
        return Ok(());
    }
    fs::remove_dir_all(descriptor_work_dir(args, descriptor))
}

/// Maps harness-local I/O failures (never a codegen/runtime verdict) to a
/// `COMPILE-FAIL`-adjacent bucket; they surface with their own message.
fn harness_io(stage: &'static str) -> impl Fn(io::Error) -> Failure {
    move |error| Failure::new(Category::CompileFail, format!("harness {stage}: {error}"))
}

/// Runs the prebuilt generator on the rendered root grammar and lets its
/// loader resolve delegate grammars from the case directory. Mirrors the
/// upstream tool invocation: `-visitor` for parser tests, listener on by
/// default, output into `src/` next to the harness `main.rs`.
fn generate_rust(
    context: &SweepContext<'_>,
    case_dir: &Path,
    grammar_path: &Path,
    descriptor: &Descriptor,
) -> Result<(), Failure> {
    let mut command = Command::new(&context.generator);
    command
        .arg(grammar_path)
        .arg("-o")
        .arg(case_dir.join("src"))
        .arg("-lib")
        .arg(case_dir);
    if descriptor.is_parser() {
        command.arg("-visitor");
    }
    if context.args.static_lexer_dfa {
        command.arg("--static-lexer-dfa");
    }
    if context.args.static_parser_dfa {
        command.arg("--static-parser-dfa");
    }
    let output = command
        .output()
        .map_err(|error| Failure::new(Category::ToolFail, format!("generator spawn: {error}")))?;
    if output.status.success() {
        return Ok(());
    }
    Err(Failure::new(
        Category::ToolFail,
        format!(
            "dbt-antlr exited with {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    ))
}

/// Writes the harness `src/main.rs` for the case (see [`render_main_rs`]).
fn write_main_rs(case_dir: &Path, descriptor: &Descriptor) -> Result<(), Failure> {
    let src_dir = case_dir.join("src");
    fs::create_dir_all(&src_dir).map_err(harness_io("src dir create"))?;
    fs::write(src_dir.join("main.rs"), render_main_rs(descriptor))
        .map_err(harness_io("main.rs write"))
}

/// Renders the per-case `src/main.rs`, a hand port of the upstream
/// `runtime-testsuite/resources/org/antlr/v4/test/runtime/helpers/src/main.rs.stg`
/// with the attributes set the way `RuntimeTests.test` and
/// `RuntimeRunner.writeRecognizerFile` set them: parser tests get
/// `<G>Lexer`/`<G>Parser` plus listener/visitor modules, `useListener =
/// useVisitor = true`, `traceATN = profile = false`, prediction mode `LL`
/// unless the descriptor flags override it, and parse trees on unless
/// `notBuildParseTree` is flagged.
fn render_main_rs(descriptor: &Descriptor) -> String {
    let grammar_name = descriptor.grammar_name.as_str();
    let is_parser = descriptor.is_parser();
    let lexer_name = if is_parser {
        format!("{grammar_name}Lexer")
    } else {
        grammar_name.to_owned()
    };
    let parser_name = is_parser.then(|| format!("{grammar_name}Parser"));
    let lexer_mod = lexer_name.to_lowercase();
    let grammar_mod = grammar_name.to_lowercase();

    let mut out = String::new();
    out.push_str(
        "#![allow(unused_imports)]\n\
         use std::env;\n\
         use dbt_antlr_runtime::Arena;\n\
         use dbt_antlr_runtime::{InputStream, Lexer, Parser};\n\
         use dbt_antlr_runtime::common_token_stream::CommonTokenStream;\n\
         use dbt_antlr_runtime::error_listener::DiagnosticErrorListener;\n\
         use dbt_antlr_runtime::int_stream::IntStream;\n\
         use dbt_antlr_runtime::lexer::LEXER_DEFAULT_MODE;\n\
         use dbt_antlr_runtime::PredictionMode;\n\
         use dbt_antlr_runtime::token::Token;\n\
         use dbt_antlr_runtime::token_stream::TokenStream;\n\
         use dbt_antlr_runtime::tree::{IsError, NoError, ParseTreeListener, ParseTreeWalker, TerminalNode};\n\
         use dbt_antlr_runtime::trees::string_tree;\n",
    );
    let _ = write!(out, "mod {lexer_mod};\nuse {lexer_mod}::{lexer_name};\n");
    if let Some(parser_name) = &parser_name {
        let parser_mod = parser_name.to_lowercase();
        let _ = write!(
            out,
            "mod {parser_mod};\nuse {parser_mod}::{parser_name};\nuse {parser_mod}::{grammar_name}ParserNodeKind;\nuse dbt_antlr_runtime::errors::ANTLRError;\n"
        );
        let _ = write!(
            out,
            "mod {grammar_mod}listener;\nuse {grammar_mod}listener::{grammar_name}Listener;\n"
        );
        let _ = write!(
            out,
            "mod {grammar_mod}visitor;\nuse {grammar_mod}visitor::{grammar_name}Visitor;\n"
        );
    }
    out.push_str(
        "\nfn main() {\n\
         \tlet args: Vec<String> = env::args().collect();\n\
         \tlet string = &args[1];\n\
         \tlet result = std::fs::read_to_string(string);\n\
         \tlet data = result.unwrap();\n\
         \tlet input = InputStream::new(data.as_str());\n\n\
         \x20   Arena::with(|arena| {\n\n",
    );
    let _ = write!(
        out,
        "    let lex = {lexer_name}::<_>::new(arena, input);\n  \tlet mut tokens = CommonTokenStream::new(lex);\n"
    );
    if let Some(parser_name) = &parser_name {
        let _ = writeln!(out, "\tlet mut parser = {parser_name}::new(arena, tokens);");
        if descriptor.show_diagnostic_errors() {
            out.push_str(
                "  \tparser.add_error_listener(Box::new(DiagnosticErrorListener::new(true)));\n",
            );
        }
        let _ = writeln!(
            out,
            "\tparser.get_interpreter().set_prediction_mode(PredictionMode::{});",
            descriptor.prediction_mode()
        );
        if !descriptor.build_parse_tree() {
            out.push_str("  \tparser.build_parse_trees = false;\n");
        }
        let _ = write!(
            out,
            "  \tlet tree = parser.{}();\n  \t//ParseTreeWalker::walk(Box::new(TreeShapeListener::new()), &tree);\n",
            descriptor.start_rule
        );
    } else {
        out.push_str(
            "  \t//tokens.fill();\n  \tfor t in tokens.iter() {\n\t}\n\tfor idx in 0..tokens.size() {\n\t\tlet mut x1 = tokens.get(idx);\n\t\tprintln!(\"{}\", x1);\n\t}\n",
        );
        if descriptor.show_dfa() {
            out.push_str("  \tprint!(\"{}\", tokens.get_dfa_string());\n");
        }
    }
    out.push_str("    });\n}\n");
    if let Some(_parser_name) = &parser_name {
        let _ = write!(
            out,
            "\nstruct TreeShapeListener {{\n}}\n\nimpl<'arena, Tok: Token + 'arena> ParseTreeListener<'arena, {grammar_name}ParserNodeKind, Tok> for TreeShapeListener {{\n}}\n"
        );
    }
    out
}

/// Compiles the case crate exactly like the fork's `RustRunner.compile`:
/// `rustc` over `src/main.rs` (the generated modules sit next to it) with the
/// prebuilt runtime rlib and its deps directory on the search path.
fn compile_case(context: &SweepContext<'_>, case_dir: &Path) -> Result<(), Failure> {
    let output = Command::new("rustc")
        .arg("--crate-name")
        .arg("Rust")
        .arg("--edition=2024")
        .arg("src/main.rs")
        .arg("--out-dir")
        .arg("target/debug")
        .arg("-L")
        .arg(format!("dependency={}", context.deps_dir().display()))
        .arg("--extern")
        .arg(format!(
            "dbt_antlr_runtime={}",
            context.runtime_rlib.display()
        ))
        .current_dir(case_dir)
        .output()
        .map_err(|error| Failure::new(Category::CompileFail, format!("rustc spawn: {error}")))?;
    if output.status.success() {
        return Ok(());
    }
    Err(Failure::new(
        Category::CompileFail,
        format!(
            "rustc exited with {}\n{}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ),
    ))
}

/// Writes `[input]` to `input.txt`, runs the case binary with that path as
/// argv[1] (like upstream `RuntimeRunner.execute`), and compares stdout and
/// stderr exactly against `[output]` and `[errors]`.
fn execute_case(case_dir: &Path, descriptor: &Descriptor) -> Result<(), Failure> {
    let input_path = case_dir.join("input.txt");
    fs::write(&input_path, &descriptor.input).map_err(harness_io("input write"))?;
    let binary = case_dir.join(format!("target/debug/Rust{}", env::consts::EXE_SUFFIX));
    let (status, stdout, stderr) = run_binary(&binary, &input_path)?;
    let output = String::from_utf8_lossy(&stdout).into_owned();
    let errors = String::from_utf8_lossy(&stderr).into_owned();
    match status {
        None => {
            return Err(Failure::new(
                Category::RunCrash,
                format!(
                    "case binary exceeded the {}s run timeout and was killed\nstdout:\n{output}\nstderr:\n{errors}",
                    RUN_TIMEOUT.as_secs()
                ),
            ));
        }
        Some(status) if !status.success() => {
            return Err(Failure::new(
                Category::RunCrash,
                format!("case binary exited with {status}\nstdout:\n{output}\nstderr:\n{errors}"),
            ));
        }
        _ => {}
    }
    if output != descriptor.output {
        return Err(Failure::new(
            Category::RunDiffOutput,
            format!(
                "stdout mismatch\nexpected stdout:\n{}\nactual stdout:\n{output}",
                descriptor.output
            ),
        ));
    }
    if errors != descriptor.errors {
        return Err(Failure::new(
            Category::RunDiffErrors,
            format!(
                "stderr mismatch\nexpected stderr:\n{}\nactual stderr:\n{errors}",
                descriptor.errors
            ),
        ));
    }
    Ok(())
}

/// The captured result of a case binary run: exit status (`None` on
/// timeout), stdout, and stderr.
type RunOutput = (Option<std::process::ExitStatus>, Vec<u8>, Vec<u8>);

/// Runs the case binary, enforcing a wall-clock timeout so one pathological
/// case cannot stall the sweep. Returns `None` for the status on timeout.
///
/// stdout and stderr stream to files instead of pipes: a child that writes
/// more than the OS pipe buffer while the parent polls `try_wait` would
/// block in `write` forever and deadlock until the timeout (the
/// antlr/antlr4#1863 regression case emits ~100KB of token dump).
fn run_binary(binary: &Path, input: &Path) -> Result<RunOutput, Failure> {
    let case_dir = input.parent().unwrap_or_else(|| Path::new("."));
    let stdout_path = case_dir.join("stdout.txt");
    let stderr_path = case_dir.join("stderr.txt");
    let capture = |path: &Path| {
        fs::File::create(path).map_err(|error| {
            Failure::new(Category::RunCrash, format!("{}: {error}", path.display()))
        })
    };
    let mut child = Command::new(binary)
        .arg(input)
        .stdin(Stdio::null())
        .stdout(Stdio::from(capture(&stdout_path)?))
        .stderr(Stdio::from(capture(&stderr_path)?))
        .spawn()
        .map_err(|error| Failure::new(Category::RunCrash, format!("case binary spawn: {error}")))?;
    let deadline = Instant::now() + RUN_TIMEOUT;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) if Instant::now() < deadline => std::thread::sleep(Duration::from_millis(50)),
            Ok(None) => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            Err(error) => {
                return Err(Failure::new(
                    Category::RunCrash,
                    format!("case binary wait: {error}"),
                ));
            }
        }
    };
    let drain = |path: &Path| fs::read(path).unwrap_or_default();
    Ok((status, drain(&stdout_path), drain(&stderr_path)))
}

fn run_checked(command: &mut Command, context: &str) -> io::Result<()> {
    let output = run_output(command)?;
    if output.status.success() {
        return Ok(());
    }
    Err(io::Error::other(format!(
        "{context} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )))
}

fn run_output(command: &mut Command) -> io::Result<Output> {
    command.output()
}

fn safe_case_dir(id: &str) -> String {
    id.chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::ffi::OsStr;
    use std::fs;
    use std::io;
    use std::path::Path;

    use super::{
        Descriptor, apply_descriptor_shims, corpus_header, ensure_descriptors_loaded,
        json_first_array_string, json_string_value, parse_descriptor, render_main_rs,
        rewrite_grammar_sections, skip_reason, sorted_children,
    };

    #[test]
    fn json_string_value_unescapes_windows_paths() {
        let line = r#"{"reason":"compiler-artifact","target":{"name":"dbt-antlr-codegen"},"executable":"C:\\target\\debug\\dbt-antlr-codegen.exe"}"#;
        assert_eq!(
            json_string_value(line, "executable").as_deref(),
            Some(r"C:\target\debug\dbt-antlr-codegen.exe")
        );
        assert_eq!(
            json_string_value(line, "name").as_deref(),
            Some("dbt-antlr-codegen")
        );
    }

    #[test]
    fn json_string_value_decodes_escapes_and_rejects_unterminated() {
        assert_eq!(
            json_string_value(r#"{"executable":"a\"b c"}"#, "executable").as_deref(),
            Some("a\"b c")
        );
        assert_eq!(
            json_string_value(r#"{"executable":"broken"#, "executable"),
            None
        );
        assert_eq!(json_string_value("{}", "executable"), None);
    }

    #[test]
    fn json_string_value_reads_first_artifact_filename() {
        let line = r#"{"reason":"compiler-artifact","target":{"name":"dbt-antlr-runtime"},"filenames":["/target/debug/deps/libdbt_antlr_runtime-a4d76dfb3e9898fa.rlib"],"executable":null}"#;
        assert_eq!(
            json_first_array_string(line, "filenames").as_deref(),
            Some("/target/debug/deps/libdbt_antlr_runtime-a4d76dfb3e9898fa.rlib")
        );
    }

    #[test]
    fn empty_descriptor_selection_is_rejected() {
        let error = ensure_descriptors_loaded(&[], Path::new("/tmp/descriptors"))
            .expect_err("an empty runtime-suite selection must not pass");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("/tmp/descriptors"));
    }

    fn parse(text: &str) -> Descriptor {
        parse_descriptor("Group".to_owned(), "Case".to_owned(), text)
            .expect("test descriptor parses")
    }

    #[test]
    fn skip_section_marks_rust_target() {
        let descriptor = parse(
            "[type]\nLexer\n\n[grammar]\nlexer grammar L;\nA:'a';\n\n[input]\na\n\n[output]\nx\n\n[skip]\nGo\nRust\n",
        );
        assert_eq!(descriptor.skip_targets, ["Go", "Rust"]);
        assert!(skip_reason(&descriptor, false, false).is_some());
        assert!(skip_reason(&descriptor, true, false).is_some());
        assert!(skip_reason(&descriptor, false, true).is_some());
    }

    #[test]
    fn absent_skip_section_keeps_case_runnable() {
        let descriptor = parse(
            "[type]\nLexer\n\n[grammar]\nlexer grammar L;\nA:'a';\n\n[input]\na\n\n[output]\nx\n",
        );
        assert_eq!(descriptor.skip_targets, Vec::<String>::new());
        assert!(skip_reason(&descriptor, false, false).is_none());
        assert!(skip_reason(&descriptor, true, false).is_none());
        assert!(skip_reason(&descriptor, false, true).is_none());
    }

    #[test]
    fn show_dfa_skips_only_in_static_lexer_dfa_mode() {
        let descriptor = parse(
            "[type]\nLexer\n\n[grammar]\nlexer grammar L;\nA:'a';\n\n[input]\na\n\n[output]\nx\n\n[flags]\nshowDFA\n",
        );
        assert!(skip_reason(&descriptor, false, false).is_none());
        assert!(skip_reason(&descriptor, false, true).is_none());
        let reason = skip_reason(&descriptor, true, false).expect("showDFA skips in static mode");
        assert!(reason.contains("showDFA"));
    }

    #[test]
    fn show_diagnostic_errors_skips_only_in_static_parser_dfa_mode() {
        let descriptor = parse(
            "[type]\nParser\n\n[grammar]\ngrammar T;\ns:'a';\n\n[input]\na\n\n[output]\nx\n\n[flags]\nshowDiagnosticErrors\n",
        );
        assert!(skip_reason(&descriptor, false, false).is_none());
        assert!(skip_reason(&descriptor, true, false).is_none());
        let reason = skip_reason(&descriptor, false, true)
            .expect("showDiagnosticErrors skips in static parser mode");
        assert!(reason.contains("showDiagnosticErrors"));
    }

    #[test]
    fn lexer_main_rs_mirrors_stg_shape_and_show_dfa() {
        let descriptor = parse(
            "[type]\nLexer\n\n[grammar]\nlexer grammar L;\nA:'a';\n\n[input]\na\n\n[output]\nx\n\n[flags]\nshowDFA\n",
        );
        let main_rs = render_main_rs(&descriptor);
        assert!(main_rs.contains("mod l;\nuse l::L;\n"));
        // No parser modules, start-rule call, or tree-shape listener.
        assert!(!main_rs.contains("LParser"));
        assert!(!main_rs.contains("let mut parser"));
        assert!(main_rs.contains("for idx in 0..tokens.size() {"));
        assert!(main_rs.contains("print!(\"{}\", tokens.get_dfa_string());"));
        assert!(!main_rs.contains("TreeShapeListener"));
    }

    #[test]
    fn parser_main_rs_mirrors_stg_attributes() {
        let descriptor = parse(
            "[type]\nParser\n\n[grammar]\ngrammar T;\ns : 'a';\n\n[start]\ns\n\n[input]\na\n\n[output]\nx\n\n[flags]\nshowDiagnosticErrors\npredictionMode=SLL\nnotBuildParseTree\n",
        );
        assert!(!descriptor.build_parse_tree());
        assert!(descriptor.show_diagnostic_errors());
        assert_eq!(descriptor.prediction_mode(), "SLL");
        let main_rs = render_main_rs(&descriptor);
        assert!(main_rs.contains("mod tlexer;\nuse tlexer::TLexer;\n"));
        assert!(
            main_rs
                .contains("mod tparser;\nuse tparser::TParser;\nuse tparser::TParserNodeKind;\n")
        );
        assert!(main_rs.contains("mod tlistener;\nuse tlistener::TListener;\n"));
        assert!(main_rs.contains("mod tvisitor;\nuse tvisitor::TVisitor;\n"));
        assert!(
            main_rs.contains(
                "parser.add_error_listener(Box::new(DiagnosticErrorListener::new(true)));"
            )
        );
        assert!(main_rs.contains("set_prediction_mode(PredictionMode::SLL);"));
        assert!(main_rs.contains("parser.build_parse_trees = false;"));
        assert!(main_rs.contains("let tree = parser.s();"));
        assert!(main_rs.contains("struct TreeShapeListener {"));
    }

    #[test]
    fn parser_main_rs_defaults_to_ll_and_parse_trees() {
        let descriptor = parse(
            "[type]\nParser\n\n[grammar]\ngrammar T;\ns : 'a';\n\n[start]\ns\n\n[input]\na\n\n[output]\nx\n",
        );
        assert!(descriptor.build_parse_tree());
        assert_eq!(descriptor.prediction_mode(), "LL");
        let main_rs = render_main_rs(&descriptor);
        assert!(main_rs.contains("set_prediction_mode(PredictionMode::LL);"));
        assert!(!main_rs.contains("build_parse_trees = false"));
        assert!(!main_rs.contains("DiagnosticErrorListener::new"));
    }

    /// `(Group, Name, file text)` for every vendored corpus file, in the order
    /// the sweep reads them.
    fn corpus_cases() -> Vec<(String, String, String)> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus");
        let mut cases = Vec::new();
        for group_dir in sorted_children(&root).expect("corpus group directories") {
            if !group_dir.path.is_dir() {
                continue;
            }
            for file in sorted_children(&group_dir.path).expect("corpus case files") {
                if file.path.extension() != Some(OsStr::new("txt")) {
                    continue;
                }
                let name = file.name.trim_end_matches(".txt").to_owned();
                let text = fs::read_to_string(&file.path).expect("corpus file read");
                cases.push((group_dir.name.clone(), name, text));
            }
        }
        assert!(
            !cases.is_empty(),
            "no corpus cases under {}",
            root.display()
        );
        cases
    }

    /// Raw `[grammar]`/`[slaveGrammar]` bodies of one corpus file, in order,
    /// with the blank line that separates sections kept.
    fn stored_grammars(text: &str) -> Vec<String> {
        let mut current: Option<&str> = None;
        let mut bodies: Vec<Vec<&str>> = Vec::new();
        for line in text.split('\n') {
            if let Some(name) = super::section_name(line) {
                current = Some(name);
                if matches!(name, "grammar" | "slaveGrammar") {
                    bodies.push(Vec::new());
                }
            } else if matches!(current, Some("grammar" | "slaveGrammar"))
                && let Some(body) = bodies.last_mut()
            {
                body.push(line);
            }
        }
        bodies.iter().map(|body| body.join("\n")).collect()
    }

    #[test]
    fn vendored_corpus_parses_to_the_stored_grammar_text() {
        let cases = corpus_cases();
        for (group, name, text) in &cases {
            let descriptor = parse_descriptor(group.clone(), name.clone(), text)
                .unwrap_or_else(|error| panic!("{group}/{name}: {error}"));
            let grammars = stored_grammars(text);
            assert!(
                matches!(
                    descriptor.test_type.as_str(),
                    "Lexer" | "CompositeLexer" | "Parser" | "CompositeParser"
                ),
                "{group}/{name} [type] is {:?}",
                descriptor.test_type
            );
            assert!(
                !descriptor.input.contains("Vendored conformance corpus"),
                "{group}/{name} [input] took the provenance preamble"
            );
            for (index, slave) in descriptor.slave_grammar_templates.iter().enumerate() {
                assert!(
                    super::grammar_name(slave).is_ok(),
                    "{group}/{name} slave {index} has no grammar declaration"
                );
            }
            assert_eq!(
                descriptor.grammar_template,
                format!("{}\n", grammars[0].trim()),
                "{group}/{name} [grammar]"
            );
            let slaves: Vec<String> = grammars[1..]
                .iter()
                .map(|body| format!("{}\n", body.trim()))
                .collect();
            assert_eq!(
                descriptor.slave_grammar_templates, slaves,
                "{group}/{name} [slaveGrammar]"
            );
        }
    }

    #[test]
    fn rewrite_grammar_sections_reproduces_each_corpus_file() {
        for (group, name, text) in corpus_cases() {
            let grammars = stored_grammars(&text);
            let rewritten = rewrite_grammar_sections(&text, &grammars[0], &grammars[1..]);
            assert_eq!(rewritten, text, "{group}/{name}");
        }
    }

    #[test]
    fn apply_descriptor_shims_reports_matched_rules_in_table_order() {
        // The three matches appear in reverse table order, so the reported ids
        // prove table order rather than input order.
        let (text, applied) = apply_descriptor_shims(
            "recog.token_start_column dbt_antlr4 let mut input = this.input.take().unwrap();",
        );
        assert_eq!(
            applied,
            vec![
                "runtime-crate-name",
                "token-start-column-getter",
                "lexer-input-field"
            ]
        );
        assert_eq!(
            text,
            "recog.get_token_start_column() dbt_antlr_runtime let input = &mut this.input;"
        );
        assert!(apply_descriptor_shims("lexer grammar L;").1.is_empty());
    }

    #[test]
    fn rewrite_grammar_sections_replaces_only_grammar_bodies() {
        let raw = "# provenance\n[type]\nLexer\n\n[grammar]\nlexer grammar L;\n\n\
                   [input]\nkeep me\n\n[slaveGrammar]\nlexer grammar S;\n\n[output]\nkeep me too\n";
        let rewritten = rewrite_grammar_sections(
            raw,
            "lexer grammar Main;",
            &["lexer grammar Slave;".to_owned()],
        );
        assert_eq!(
            rewritten,
            "# provenance\n[type]\nLexer\n\n[grammar]\nlexer grammar Main;\n\n\
             [input]\nkeep me\n\n[slaveGrammar]\nlexer grammar Slave;\n\n[output]\nkeep me too\n"
        );
    }

    #[test]
    #[should_panic(expected = "descriptor has 1 [slaveGrammar] section(s)")]
    fn rewrite_grammar_sections_rejects_extra_rendered_slaves() {
        let raw = "[grammar]\nlexer grammar L;\n\n[slaveGrammar]\nlexer grammar S;\n";
        rewrite_grammar_sections(raw, "lexer grammar L;", &[]);
    }

    #[test]
    fn corpus_header_lists_each_delta_once() {
        let single = corpus_header("LexerExec/Case", &["lexer-input-field"]);
        assert_eq!(
            single,
            "# Vendored conformance corpus file; see corpus/README.md.\n\
             # Source: dbt-antlr4-2.0.0 descriptors/LexerExec/Case.txt\n\
             # Grammar sections are rendered Rust: edit them here.\n\
             # Local deltas: lexer-input-field\n"
        );
        let repeated = corpus_header(
            "CompositeLexers/Case",
            &["runtime-crate-name", "runtime-crate-name"],
        );
        assert!(repeated.ends_with("# Local deltas: runtime-crate-name\n"));
        assert!(!corpus_header("LexerExec/Case", &[]).contains("Local deltas"));
    }
}
