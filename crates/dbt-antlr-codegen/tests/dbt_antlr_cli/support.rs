// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Shared helpers for the grammar transform command-line tests: running the
//! `dbt-antlr-codegen` binary, compiling and running its output against the
//! `dbt-antlr-runtime` runtime in scratch crates, and snapshot
//! normalization.
#![allow(dead_code)]

pub(super) use std::collections::BTreeSet;
pub(super) use std::ffi::OsStr;
pub(super) use std::fs;
pub(super) use std::path::{Path, PathBuf};
pub(super) use std::process::{Command, Output};
pub(super) use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn run_dbt_antlr(args: &[impl AsRef<OsStr>]) -> Output {
    cli_command(env!("CARGO_BIN_EXE_dbt-antlr-codegen"))
        .args(args)
        .output()
        .expect("dbt-antlr-codegen should run")
}

pub(super) fn cli_command(binary: &str) -> Command {
    let mut command = Command::new(binary);
    command
        .env("NO_COLOR", "1")
        .env("CLICOLOR", "0")
        .env("CLICOLOR_FORCE", "0")
        .env("NO_GRAPHICS", "0")
        .env("COLUMNS", "4096");
    command
}

pub(super) fn utf8(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).expect("process output should be UTF-8")
}

pub(super) fn normalize_cli_snapshot(value: &str) -> String {
    let mut normalized = value
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    if value.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}

pub(super) fn replace_miette_path(value: &str, path: &str, replacement: &str) -> String {
    let Some(first) = path.chars().next() else {
        return value.to_owned();
    };
    let mut first_buffer = [0; 4];
    let first: &str = first.encode_utf8(&mut first_buffer);
    let mut normalized = value.to_owned();
    let mut search_from = 0;
    while let Some(relative_start) = normalized[search_from..].find(first) {
        let start = search_from + relative_start;
        let Some(end) = wrapped_path_end(&normalized, start, path) else {
            search_from = start + first.len();
            continue;
        };
        normalized.replace_range(start..end, replacement);
        search_from = start + replacement.len();
    }
    normalized
}

fn wrapped_path_end(value: &str, start: usize, path: &str) -> Option<usize> {
    // miette wraps long lines onto continuation lines whose indentation
    // (`\n  │ `, `\n      `, ...) depends on the report layout, so between
    // path characters allow any line break followed by whitespace and an
    // optional box-drawing gutter.
    let mut cursor = start;
    for expected in path.chars() {
        while let Some(after_break) = value.get(cursor..).and_then(|rest| rest.strip_prefix('\n')) {
            let skipped = after_break
                .chars()
                .take_while(|character| character.is_whitespace() || *character == '│')
                .count();
            if skipped == 0 {
                break;
            }
            cursor += 1 + after_break
                .chars()
                .take(skipped)
                .map(char::len_utf8)
                .sum::<usize>();
        }
        let actual = value.get(cursor..)?.chars().next()?;
        if actual != expected {
            return None;
        }
        cursor += actual.len_utf8();
    }
    Some(cursor)
}

#[test]
fn miette_path_normalization_handles_message_wrapping() {
    let path = "/tmp/dbt-antlr-codegen-long/Example.g4";
    let rendered = "  × /tmp/dbt-antlr-codegen-\n  │ long/Example.g4:4:6: invalid input";
    assert_eq!(
        replace_miette_path(rendered, path, "$GRAMMAR"),
        "  × $GRAMMAR:4:6: invalid input"
    );
}

pub(super) fn temporary_directory(label: &str) -> TempDirectory {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should follow the Unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "dbt-antlr-codegen-{label}-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&path).expect("temporary directory should be writable");
    TempDirectory(path)
}

pub(super) struct TempDirectory(PathBuf);

impl TempDirectory {
    pub(super) fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

pub(super) fn fixture(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/transforms")
        .join(relative)
}

/// Generates one grammar three ways — unoptimized baseline, applied
/// optimization, and report-only — asserting each run succeeds, that the
/// baseline emits no optimization manifest, and that report mode emits only
/// `optimizations.json`. Returns the three output directories.
pub(super) fn run_optimization_matrix(
    root: &Path,
    grammar: &Path,
    apply_flag: &str,
    report_flag: &str,
) -> (PathBuf, PathBuf, PathBuf) {
    let baseline = root.join("baseline");
    let optimized = root.join("optimized");
    let report = root.join("report");
    for (out, extra) in [
        (&baseline, None),
        (&optimized, Some(apply_flag)),
        (&report, Some(report_flag)),
    ] {
        let mut args = vec![grammar.as_os_str(), OsStr::new("-o"), out.as_os_str()];
        if let Some(flag) = extra {
            args.push(OsStr::new(flag));
        }
        let output = run_dbt_antlr(&args);
        assert!(
            output.status.success(),
            "{extra:?} failed\nstdout: {}\nstderr: {}",
            utf8(&output.stdout),
            utf8(&output.stderr)
        );
    }
    assert!(!baseline.join("optimizations.json").exists());
    let report_files = fs::read_dir(&report)
        .expect("report directory should exist")
        .map(|entry| {
            entry
                .expect("report entry should be readable")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect::<Vec<_>>();
    assert_eq!(report_files, ["optimizations.json"]);
    (baseline, optimized, report)
}

/// The observable API surface of one generated parser source: token and
/// rule constants, rule entry points, context types, the parser struct, and
/// the listener callbacks the parser dispatches.
pub(super) fn generated_parser_api(source: &str) -> Vec<String> {
    let mut api = BTreeSet::new();
    for raw in source.lines() {
        let line = raw.trim();
        // Rule entry points are the tab-indented `pub fn`s of the parser
        // `impl` block; inherent methods such as `new` are space-indented.
        if let Some(rest) = raw.strip_prefix("\tpub fn ") {
            let name = identifier_prefix(rest.trim_start());
            if !name.is_empty() {
                api.insert(format!("fn {name}"));
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix("pub const RULE_") {
            api.insert(format!("const RULE_{}", identifier_prefix(rest)));
            continue;
        }
        if let Some(rest) = line.strip_prefix("pub const ") {
            if rest.contains(":i32") {
                api.insert(format!("const {}", identifier_prefix(rest)));
            }
            continue;
        }
        for prefix in ["pub type ", "pub enum "] {
            if let Some(rest) = line.strip_prefix(prefix) {
                let name = identifier_prefix(rest);
                if name.ends_with("ContextAll") {
                    api.insert(format!("type {name}"));
                }
            }
        }
        if let Some(rest) = line.strip_prefix("pub struct ") {
            let name = identifier_prefix(rest);
            if name.ends_with("Parser") {
                api.insert(format!("struct {name}"));
            }
        }
        if line.starts_with("dbt_antlr_runtime::impl_listener_dispatch!") {
            for token in line
                .split(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            {
                if token.starts_with("enter_") || token.starts_with("exit_") {
                    api.insert(format!("callback {token}"));
                }
            }
        }
    }
    api.into_iter().collect()
}

fn identifier_prefix(text: &str) -> &str {
    let end = text
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(text.len());
    &text[..end]
}

/// True when the generated parser exposes the rule entry point `rule`
/// (left-recursive rules render with an extra space after `fn`).
pub(super) fn generated_parser_has_rule(source: &str, rule: &str) -> bool {
    source.lines().any(|raw| {
        raw.strip_prefix("\tpub fn ")
            .is_some_and(|rest| identifier_prefix(rest.trim_start()) == rule)
    })
}

fn runtime_crate_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime")
        .canonicalize()
        .expect("dbt-antlr-runtime crate exists")
}

fn scratch_crate(name: &str, target: &Path) -> PathBuf {
    let project = target.join("compile-generated").join(name);
    let _ = fs::remove_dir_all(&project);
    fs::create_dir_all(project.join("src")).expect("scratch crate should be writable");
    fs::write(
        project.join("Cargo.toml"),
        format!(
            "[package]\n\
             name = \"{name}\"\n\
             version = \"0.0.0\"\n\
             edition = \"2021\"\n\
             \n\
             [dependencies]\n\
             dbt-antlr-runtime = {{ path = {:?} }}\n",
            runtime_crate_root(),
        ),
    )
    .expect("scratch manifest should be writable");
    project
}

fn run_cargo(project: &Path, command: &str) -> Output {
    cli_command(env!("CARGO"))
        .args([
            command,
            "--quiet",
            "--offline",
            "--manifest-path",
            project
                .join("Cargo.toml")
                .to_str()
                .expect("scratch path should be UTF-8"),
        ])
        // One shared target directory keeps the runtime build cache warm
        // across the scratch crates of this test suite.
        .env(
            "CARGO_TARGET_DIR",
            Path::new("/tmp/pi/dbt-antlr-cli-tests/target"),
        )
        .env("CARGO_TERM_COLOR", "never")
        .output()
        .expect("cargo should run")
}

/// Compile check: copies every `.rs` file of `generated` into a scratch
/// crate next to a `lib.rs` declaring them, and runs `cargo check` against
/// the `dbt-antlr-runtime` runtime.
pub(super) fn assert_generated_modules_compile(temp_dir: &Path, generated: &Path) {
    let project = scratch_crate("compile-check", temp_dir);
    let mut declarations = String::new();
    for entry in fs::read_dir(generated).expect("generated directory should exist") {
        let entry = entry.expect("generated entry should be readable");
        let file_name = entry.file_name().to_string_lossy().into_owned();
        if !file_name.ends_with(".rs") {
            continue;
        }
        fs::copy(entry.path(), project.join("src").join(&file_name))
            .expect("generated module should be copied into the scratch crate");
        let module = file_name.strip_suffix(".rs").expect("rs suffix");
        declarations.push_str(&format!("pub mod {module};\n"));
    }
    fs::write(
        project.join("src").join("lib.rs"),
        format!("#![allow(clippy::all, warnings, non_snake_case)]\n{declarations}"),
    )
    .expect("scratch crate root should be writable");
    let output = run_cargo(&project, "check");
    assert!(
        output.status.success(),
        "generated project failed\nstdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
}

/// Builds the differential crate pairing a baseline and an optimized
/// generation of the same grammar: each output directory becomes one module
/// tree (`baseline` / `optimized`) so the generated `use super::<module>`
/// imports resolve unmodified, and `test_source` (appended to the crate
/// root) runs under `cargo test`.
pub(super) fn assert_generated_project(
    temp_dir: &Path,
    baseline: &Path,
    optimized: &Path,
    test_source: &str,
) {
    let project = scratch_crate("differential", temp_dir);
    for (group, out_dir) in [("baseline", baseline), ("optimized", optimized)] {
        let directory = project.join("src").join(group);
        fs::create_dir_all(&directory).expect("group directory should be writable");
        let mut declarations = String::new();
        for entry in fs::read_dir(out_dir).expect("generated directory should exist") {
            let entry = entry.expect("generated entry should be readable");
            let file_name = entry.file_name().to_string_lossy().into_owned();
            if !file_name.ends_with(".rs") {
                continue;
            }
            fs::copy(entry.path(), directory.join(&file_name))
                .expect("generated module should be copied into the scratch crate");
            let module = file_name.strip_suffix(".rs").expect("rs suffix");
            declarations.push_str(&format!("pub mod {module};\n"));
        }
        fs::write(
            project.join("src").join(group).with_extension("rs"),
            declarations,
        )
        .expect("group module should be writable");
    }
    fs::write(
        project.join("src").join("lib.rs"),
        format!(
            "#![allow(clippy::all, warnings, non_snake_case)]\n\
             pub mod baseline;\n\
             pub mod optimized;\n\
             {test_source}"
        ),
    )
    .expect("scratch crate root should be writable");
    let output = run_cargo(&project, "test");
    assert!(
        output.status.success(),
        "differential project failed\nstdout: {}\nstderr: {}",
        utf8(&output.stdout),
        utf8(&output.stderr)
    );
}
