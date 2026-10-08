// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
#![allow(clippy::print_stderr, clippy::print_stdout)]

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

const DESCRIPTOR_PATH: &str = "resources/org/antlr/v4/test/runtime/descriptors";
const TEMPLATE_PATH: &str = "resources/org/antlr/v4/test/runtime/templates/Rust.test.stg";
const ANTLR_JAR_ENV: &str = "ANTLR4_JAR";
const DESCRIPTORS_ENV: &str = "ANTLR4_RUNTIME_TESTSUITE";
const RENDER_DRIVER: &str = "tests/runtime-testsuite/java/RenderGrammar.java";
const RUNTIME_PACKAGE: &str = "dbt-antlr-runtime";
/// Guard against a single pathological case stalling the whole sweep; the
/// upstream JVM harness bounds execution the same way.
const RUN_TIMEOUT: Duration = Duration::from_secs(300);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse()?;
    let descriptor_root = resolve_descriptor_root(&args.descriptors)?;
    let descriptors = load_descriptors(&descriptor_root, &args)?;
    ensure_descriptors_loaded(&descriptors, &descriptor_root)?;
    let mut summary = Summary::default();

    if args.work_dir.exists() && !args.keep {
        fs::remove_dir_all(&args.work_dir)?;
    }
    fs::create_dir_all(&args.work_dir)?;

    // Skips are classified up front (in descriptor order) so workers only see
    // runnable cases; `--limit` counts runnable cases and stops classification
    // there.
    let mut runnable = Vec::new();
    for descriptor in descriptors {
        if let Some(reason) = skip_reason(&descriptor) {
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
/// independent: each renders, generates, compiles (against the read-only
/// prebuilt runtime rlib), and executes in its own case directory.
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
    /// `dbt-antlr` exited non-zero or reported diagnostics.
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
/// precompiled `StringTemplate` render driver (the Java single-file source
/// launcher would otherwise re-compile `RenderGrammar.java` on every render),
/// the prebuilt `dbt-antlr` executable, and the prebuilt `dbt-antlr-runtime`
/// runtime rlib that every case compiles against.
struct SweepContext<'a> {
    args: &'a Args,
    render_classes: PathBuf,
    generator: PathBuf,
    runtime_rlib: PathBuf,
}

impl<'a> SweepContext<'a> {
    fn prepare(args: &'a Args) -> io::Result<Self> {
        let render_classes = args.work_dir.join("stg-render-classes");
        fs::create_dir_all(&render_classes)?;
        run_checked(
            Command::new("javac")
                .arg("-cp")
                .arg(&args.antlr_jar)
                .arg("-d")
                .arg(&render_classes)
                .arg(args.workspace_root.join(RENDER_DRIVER)),
            "StringTemplate render driver compile",
        )?;
        let generator = prebuild_generator(args)?;
        let runtime_rlib = prebuild_runtime(args)?;
        Ok(Self {
            args,
            render_classes,
            generator,
            runtime_rlib,
        })
    }

    /// `java -cp` separator-joined ANTLR jar + render driver classes.
    fn render_classpath(&self) -> String {
        let separator = if cfg!(windows) { ";" } else { ":" };
        format!(
            "{}{separator}{}",
            self.args.antlr_jar.display(),
            self.render_classes.display()
        )
    }

    /// Directory of `target/debug/deps` holding the runtime rlib and its
    /// transitive dependencies, passed to rustc as `-L dependency=`.
    fn deps_dir(&self) -> &Path {
        self.runtime_rlib
            .parent()
            .expect("runtime rlib lives in a deps directory")
    }
}

/// Builds `dbt-antlr` once and resolves its executable path from
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
            .arg("dbt-antlr")
            .arg("--message-format=json"),
    )?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "dbt-antlr build failed\nstderr:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .filter(|line| line.contains("\"reason\":\"compiler-artifact\""))
        .filter(|line| json_string_value(line, "name").as_deref() == Some("dbt-antlr"))
        .filter_map(|line| json_string_value(line, "executable"))
        .map(PathBuf::from)
        .next_back()
        .ok_or_else(|| io::Error::other("cargo did not report the dbt-antlr executable"))
}

/// Builds the `dbt-antlr-runtime` runtime once and locates its rlib from cargo's
/// JSON artifact messages (mirrors `RustRunner.initRuntime`), honoring any
/// `CARGO_TARGET_DIR` redirection.
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

#[derive(Debug)]
struct Args {
    antlr_jar: PathBuf,
    descriptors: PathBuf,
    workspace_root: PathBuf,
    work_dir: PathBuf,
    group: Option<String>,
    case_name: Option<String>,
    limit: Option<usize>,
    keep: bool,
    /// Parallel case workers.
    jobs: usize,
    /// Template group used to render descriptor grammars (real
    /// `StringTemplate` via the ANTLR jar) before generation.
    stg: PathBuf,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut antlr_jar = None;
        let mut descriptors = None;
        let mut workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("testsuite package should live below the workspace root")
            .to_path_buf();
        let mut work_dir = env::temp_dir().join("dbt-antlr-runtime-testsuite");
        let mut group = None;
        let mut case_name = None;
        let mut limit = None;
        let mut keep = false;
        let mut jobs = None;
        let mut stg = None;

        let mut iter = env::args().skip(1);
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--antlr-jar" => {
                    antlr_jar = Some(PathBuf::from(next_arg(&mut iter, "--antlr-jar")?));
                }
                "--descriptors" => {
                    descriptors = Some(PathBuf::from(next_arg(&mut iter, "--descriptors")?));
                }
                "--workspace-root" => {
                    workspace_root = PathBuf::from(next_arg(&mut iter, &arg)?);
                }
                "--work-dir" => work_dir = PathBuf::from(next_arg(&mut iter, "--work-dir")?),
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
                "--help" | "-h" => return Err(usage()),
                other => return Err(format!("unknown argument {other}\n\n{}", usage())),
            }
        }

        let current_dir = env::current_dir().map_err(|error| error.to_string())?;
        let workspace_root = absolutize(&current_dir, workspace_root);
        // Explicit flags and environment overrides win; otherwise fall back
        // to the pinned upstream release artifacts (downloaded and
        // checksum-verified into the workspace target/ cache).
        let antlr_jar = match explicit_or_env(antlr_jar, ANTLR_JAR_ENV) {
            Some(path) => absolutize(&current_dir, path),
            None => upstream::ensure_tool_jar(&workspace_root)?,
        };
        let descriptors = match explicit_or_env(descriptors, DESCRIPTORS_ENV) {
            Some(path) => absolutize(&current_dir, path),
            None => upstream::ensure_runtime_testsuite(&workspace_root)?,
        };

        let work_dir = absolutize(&current_dir, work_dir);
        let stg = stg.map_or_else(
            || default_stg_path(&descriptors),
            |stg| absolutize(&current_dir, stg),
        );
        if !stg.is_file() {
            return Err(format!(
                "Rust action template group not found at {}; pass --stg\n\n{}",
                stg.display(),
                usage()
            ));
        }
        // Every worker drives a JVM and rustc of its own; past ~8 the extra
        // workers mostly fight each other for cores.
        let jobs = jobs.unwrap_or_else(|| {
            std::thread::available_parallelism().map_or(1, |cores| cores.get().min(8))
        });
        Ok(Self {
            antlr_jar,
            descriptors,
            workspace_root,
            work_dir,
            group,
            case_name,
            limit,
            keep,
            jobs,
            stg,
        })
    }
}

/// Finds the target `Rust.test.stg` next to the descriptor corpus, whether
/// `--descriptors` names the runtime-testsuite root or the descriptor
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
    format!(
        "usage: runtime-testsuite [--workspace-root PATH] [--antlr-jar ANTLR.jar] [--descriptors PATH] [--case Group/Name] [--group Group] [--limit N] [--jobs N] [--keep] [--stg PATH] [--work-dir PATH]\n\nDefaults: workspace root inferred from the testsuite package; work dir $TMPDIR/dbt-antlr-runtime-testsuite (mirrors upstream, which stages every test under java.io.tmpdir); tool jar and descriptors fetched from the pinned dbt-antlr4 2.0.0 GitHub release (checksum-verified, cached under <workspace>/target/dbt-antlr-runtime-testsuite-cache); ANTLR4_JAR and ANTLR4_RUNTIME_TESTSUITE override; --stg <descriptors>/{TEMPLATE_PATH}; --jobs min(cores, 8)"
    )
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
    /// Raw grammar section text (ST escapes intact) for the template render.
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
        } else {
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
                descriptor.grammar_name = grammar_name(&render_st_backslash_escapes(&value))?;
                descriptor.grammar_template = value;
            }
            "slaveGrammar" => {
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

/// Applies the `StringTemplate` backslash collapse used by the upstream Java
/// harness when descriptor grammars are rendered as templates.
fn render_st_backslash_escapes(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.peek() {
                Some('\\') => {
                    chars.next();
                    out.push('\\');
                }
                Some('<' | '>') => {}
                _ => out.push(ch),
            }
        } else {
            out.push(ch);
        }
    }
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
/// target via the `[skip]` section (`RuntimeTestDescriptor.ignore`).
fn skip_reason(descriptor: &Descriptor) -> Option<String> {
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
    None
}

/// Renders one grammar template through the target `.test.stg` group using
/// the `StringTemplate` engine bundled in the ANTLR jar (driver precompiled
/// once per sweep), mirroring upstream `RuntimeTests.prepareGrammars`.
fn render_grammar_through_stg(
    context: &SweepContext<'_>,
    case_dir: &Path,
    tag: &str,
    grammar_template: &str,
) -> Result<String, Failure> {
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
        .arg(context.render_classpath())
        .arg("RenderGrammar")
        .arg(&context.args.stg)
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
    // Descriptor templates in the fork's runtime-testsuite still reference the
    // runtime crate by its historical name (`dbt_antlr4`) inside embedded Rust
    // actions; retarget them to the renamed `dbt_antlr_runtime` crate.
    Ok(rendered.replace("dbt_antlr4", "dbt_antlr_runtime"))
}

/// Runs one rendered descriptor through the full pipeline: `StringTemplate`
/// render, `dbt-antlr` codegen, harness `main.rs`, `rustc` against the
/// prebuilt runtime rlib, execution, and output comparison.
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

    // Render the descriptor grammar (and slaves) through the target
    // `.test.stg` with the real StringTemplate engine, exactly like the
    // upstream harness. The rendered source graph then feeds the generator.
    let rendered =
        render_grammar_through_stg(context, &case_dir, "main", &descriptor.grammar_template)?;
    let grammar_path = case_dir.join(format!("{}.g4", descriptor.grammar_name));
    fs::write(&grammar_path, &rendered).map_err(harness_io("grammar write"))?;
    for (index, slave) in descriptor.slave_grammar_templates.iter().enumerate() {
        let rendered_slave =
            render_grammar_through_stg(context, &case_dir, &format!("slave{index}"), slave)?;
        let slave_name = grammar_name(&rendered_slave).map_err(|error| {
            Failure::new(Category::RenderFail, format!("slave{index}: {error}"))
        })?;
        let slave_path = case_dir.join(format!("{slave_name}.g4"));
        fs::write(&slave_path, &rendered_slave).map_err(harness_io("slave grammar write"))?;
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
    use std::io;
    use std::path::Path;

    use super::{
        Descriptor, ensure_descriptors_loaded, json_first_array_string, json_string_value,
        parse_descriptor, render_main_rs, skip_reason,
    };

    #[test]
    fn json_string_value_unescapes_windows_paths() {
        let line = r#"{"reason":"compiler-artifact","target":{"name":"dbt-antlr"},"executable":"C:\\target\\debug\\dbt-antlr.exe"}"#;
        assert_eq!(
            json_string_value(line, "executable").as_deref(),
            Some(r"C:\target\debug\dbt-antlr.exe")
        );
        assert_eq!(
            json_string_value(line, "name").as_deref(),
            Some("dbt-antlr")
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
        assert!(skip_reason(&descriptor).is_some());
    }

    #[test]
    fn absent_skip_section_keeps_case_runnable() {
        let descriptor = parse(
            "[type]\nLexer\n\n[grammar]\nlexer grammar L;\nA:'a';\n\n[input]\na\n\n[output]\nx\n",
        );
        assert_eq!(descriptor.skip_targets, Vec::<String>::new());
        assert!(skip_reason(&descriptor).is_none());
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
}
