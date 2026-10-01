// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Shared helpers for the dbt emission golden-diff tests.

#![allow(dead_code)]
#![allow(clippy::print_stderr)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

pub(crate) fn grammars_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime/grammars")
        .canonicalize()
        .expect("grammars directory exists")
}

pub(crate) fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime/tests/gen")
        .canonicalize()
        .expect("golden directory exists")
}

pub(crate) fn read_golden(name: &str) -> String {
    std::fs::read_to_string(golden_dir().join(name)).expect("golden readable")
}

/// Byte index where the embedded-ATN section starts (its definition, not
/// the earlier `ATN_SIMULATOR_MANAGER` references).
pub(crate) fn atn_section_start(content: &str) -> usize {
    content
        .match_indices("static ATN_SIMULATOR_MANAGER")
        .map(|(i, _)| i)
        .last()
        .expect("ATN section exists")
}

/// Prints the first differing line pair with context.
pub(crate) fn report_first_diff(expected: &str, actual: &str) {
    let expected_lines = expected.split('\n').collect::<Vec<_>>();
    let actual_lines = actual.split('\n').collect::<Vec<_>>();
    let common = expected_lines.len().min(actual_lines.len());
    let first = (0..common).find(|&i| expected_lines[i] != actual_lines[i]);
    match first {
        Some(i) => {
            let start = i.saturating_sub(3);
            eprintln!("first line difference at line {}:", i + 1);
            for j in start..(i + 3).min(common) {
                eprintln!("  expected[{}]: {:?}", j + 1, expected_lines[j]);
                eprintln!("  actual  [{}]: {:?}", j + 1, actual_lines[j]);
            }
        }
        None => eprintln!(
            "shared prefix; lengths differ: expected {}, actual {}",
            expected_lines.len(),
            actual_lines.len()
        ),
    }
}

/// Extracts the bracketed integer word list following `marker`.
pub(crate) fn extract_words(content: &str, marker: &str) -> Vec<i64> {
    let start = content.find(marker).expect("marker exists");
    let rest = &content[start..];
    let open = rest.find('[').expect("word list opens");
    let close = rest.find("])").expect("word list closes");
    rest[open + 1..close]
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().expect("integer word"))
        .collect()
}

/// Masks the intentional ATN-encoding difference between the golden and the
/// emitted parser: everything from the embedded-ATN section on, plus the
/// deserializer import line.
pub(crate) fn mask_parser_atn_diff(content: &str) -> String {
    mask_deserializer_import(&content[..atn_section_start(content)])
}

fn mask_deserializer_import(content: &str) -> String {
    content.replace(
        "use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;",
        "use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;",
    )
}

/// Compile check: writes `files` plus a `lib.rs` declaring `modules` into a
/// scratch crate under `/tmp/pi/` and runs `cargo check` against the
/// `dbt-antlr-runtime` runtime.
pub(crate) fn compile_check(crate_name: &str, files: &[(String, String)], modules: &[&str]) {
    let scratch = Path::new("/tmp/pi").join(crate_name);
    let src = scratch.join("src");
    std::fs::create_dir_all(&src).expect("create scratch crate");

    let runtime_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../dbt-antlr-runtime")
        .canonicalize()
        .expect("dbt-antlr-runtime crate exists");
    std::fs::write(
        scratch.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{crate_name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n\
             [dependencies]\ndbt-antlr-runtime = {{ path = {:?} }}\n",
            runtime_path.display().to_string()
        ),
    )
    .expect("write scratch Cargo.toml");

    for (name, content) in files {
        std::fs::write(src.join(name), content).expect("write source file");
    }
    let lib = modules.iter().fold(String::new(), |mut lib, module| {
        let _ = writeln!(lib, "#[allow(clippy::all)]\npub mod {module};");
        lib
    });
    std::fs::write(src.join("lib.rs"), lib).expect("write scratch lib.rs");

    let output = std::process::Command::new("cargo")
        .args(["check", "--offline"])
        .current_dir(&scratch)
        .env("CARGO_TARGET_DIR", scratch.join("target"))
        .output()
        .expect("run cargo check");
    assert!(
        output.status.success(),
        "cargo check failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// Asserts the ATN embedded in `actual` (always packed u32 words) builds the
/// same graph as the ATN embedded in `golden`, which may be in either
/// encoding: Java i32 words (pre-swap goldens) or packed u32 words
/// (post-swap goldens, where this still proves emission determinism).
pub(crate) fn assert_atn_graphs_equivalent(golden: &str, actual: &str, context: &str) {
    use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
    use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;

    let packed = |content: &str| -> Vec<u32> {
        extract_words(content, "_serializedATN")
            .into_iter()
            .map(|word| u32::try_from(word).expect("packed ATN word fits u32"))
            .collect()
    };
    let packed_dump = |words: &[u32]| {
        dbt_antlr_runtime::atn_dump::dump_atn(
            &PackedATNDeserializer::new()
                .deserialize(words)
                .expect("packed ATN deserializes"),
        )
    };

    let golden_dump = if golden.contains("LazyLock<Vec<i32>>") {
        let words: Vec<i32> = extract_words(golden, "_serializedATN")
            .into_iter()
            .map(|word| word as i32)
            .collect();
        dbt_antlr_runtime::atn_dump::dump_atn(
            &ATNDeserializer::new(None).deserialize(&mut words.iter()),
        )
    } else {
        packed_dump(&packed(golden))
    };
    let actual_dump = packed_dump(&packed(actual));
    assert_eq!(golden_dump, actual_dump, "{context}: ATN graphs differ");
}
