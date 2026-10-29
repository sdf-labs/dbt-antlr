#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--static-lexer-dfa` (lexer scope): CLI plumbing, the
//! table statics of the flag-on output, ATN elision for a fully static
//! lexer, and the escape-reason warnings of the diagnostic channel.
//!
//! Byte-identity of the flag-off output is covered by the golden sweep
//! (`dbt_emit_sweep.rs`) and is not repeated here.

mod common;

use common::grammars_dir;
use dbt_antlr_codegen::cli;
use dbt_antlr_codegen::emission::{EmittedOutput, emit_files_with_options};
use dbt_antlr_codegen::{OptimizationOptions, Severity};

fn emit(grammar_file: &str, static_lexer_dfa: bool) -> EmittedOutput {
    let grammar = grammars_dir().join(grammar_file);
    let mut optimization = OptimizationOptions::new();
    optimization.static_lexer_dfa(static_lexer_dfa);
    emit_files_with_options(&grammar, &[grammars_dir()], true, false, &optimization)
        .expect("emission succeeds")
}
fn lexer_content<'o>(output: &'o EmittedOutput, name: &str) -> &'o str {
    &output
        .files
        .iter()
        .find(|file| file.name == name)
        .unwrap_or_else(|| panic!("{name} emitted"))
        .content
}

#[test]
fn cli_accepts_the_static_lexer_dfa_flag() {
    let grammar = grammars_dir().join("lexer/Spike.g4");
    let out_dir =
        std::env::temp_dir().join(format!("dbt-static-lexer-dfa-cli-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    let args = vec![
        "-o".to_owned(),
        out_dir.to_string_lossy().into_owned(),
        "--static-lexer-dfa".to_owned(),
        grammar.to_string_lossy().into_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    cli::run(&args, &mut stdout, &mut stderr).expect("CLI run succeeds");

    let written = std::fs::read_to_string(out_dir.join("spike.rs")).expect("lexer written");
    assert!(
        written.contains("_STATIC_LEXER_TABLE_WORDS"),
        "the flag puts the table statics into the emitted lexer"
    );
    let manifest =
        std::fs::read_to_string(out_dir.join("optimizations.txt")).expect("manifest written");
    assert!(
        manifest.contains("\"name\": \"static-lexer-dfa\""),
        "the flag writes the static-lexer-dfa entry into optimizations.txt"
    );

    let warnings = String::from_utf8(stderr).expect("stderr is utf-8");
    assert!(
        warnings.contains("warning[G4S188]"),
        "the escape warning reaches the CLI diagnostic channel: {warnings}"
    );

    std::fs::remove_dir_all(&out_dir).expect("output dir removed");
}

#[test]
fn cli_report_only_runs_static_lexer_dfa_without_writing_files() {
    let grammar = grammars_dir().join("lexer/Spike.g4");
    for flag in ["-n", "--dry-run", "--report-only"] {
        let out_dir = std::env::temp_dir().join(format!(
            "dbt-static-lexer-dfa-dry-run-{}-{}",
            flag.trim_start_matches('-'),
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&out_dir);

        let args = vec![
            "-o".to_owned(),
            out_dir.to_string_lossy().into_owned(),
            flag.to_owned(),
            "--static-lexer-dfa".to_owned(),
            grammar.to_string_lossy().into_owned(),
        ];
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        cli::run(&args, &mut stdout, &mut stderr).expect("CLI dry-run succeeds");

        assert!(stdout.is_empty(), "{flag} prints no written files");
        assert!(!out_dir.exists(), "{flag} writes no output files");
        let warnings = String::from_utf8(stderr).expect("stderr is utf-8");
        assert!(
            warnings.contains("Spike.g4:4:16:"),
            "{flag} prints the span-annotated static lexer DFA warning: {warnings}"
        );
    }
}

#[test]
fn help_text_lists_the_static_lexer_dfa_flag() {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let error = cli::run(&["--help".to_owned()], &mut stdout, &mut stderr)
        .expect_err("--help prints the usage and stops");
    for flag in [
        "--static-lexer-dfa",
        "--static-parser-dfa",
        "--report-decisions",
        "--dump-dfas",
    ] {
        assert!(
            error.to_string().contains(flag),
            "usage lists {flag}: {error}"
        );
    }
}

#[test]
fn xml_lexer_flag_on_keeps_the_atn_next_to_the_tables() {
    // XMLLexer has escape edges (a predicate in COMMENT, a position-dependent
    // action in CLOSE), so the ATN stays for the lazy-simulator hand-off.
    let output = emit("XMLLexer.g4", true);
    let content = lexer_content(&output, "xmllexer.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_with_static_lexer_tables"));
    assert!(content.contains("_serializedATN"));
}

#[test]
fn csv_lexer_flag_on_is_fully_static() {
    // CSV has no predicates, position-dependent actions, or recursion: the
    // tables cover every edge and the ATN is elided entirely.
    let output = emit("CSV.g4", true);
    let content = lexer_content(&output, "csvlexer.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_static_lexer"));
    assert!(!content.contains("_serializedATN"));
}

#[test]
fn spike_lexer_flag_on_keeps_the_atn_next_to_the_tables() {
    // The escape-edge witness: recursion and a predicate escape, so the
    // tables come with the retained ATN.
    let output = emit("lexer/Spike.g4", true);
    let content = lexer_content(&output, "spike.rs");
    assert!(content.contains("_STATIC_LEXER_TABLE_WORDS"));
    assert!(content.contains("new_with_static_lexer_tables"));
    assert!(content.contains("_serializedATN"));
}

#[test]
fn flag_off_emits_no_tables() {
    for (grammar, name) in [
        ("CSV.g4", "csvlexer.rs"),
        ("XMLLexer.g4", "xmllexer.rs"),
        ("lexer/Spike.g4", "spike.rs"),
    ] {
        let output = emit(grammar, false);
        let content = lexer_content(&output, name);
        assert!(
            !content.contains("_STATIC_LEXER_TABLE"),
            "{name} without the flag has no table statics"
        );
    }
}

#[test]
fn spike_lexer_warnings_name_the_escape_reasons() {
    let output = emit("lexer/Spike.g4", true);
    let escapes: Vec<_> = output
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S188")
        .collect();
    assert_eq!(escapes.len(), 1, "one escaped edge in Spike.g4");
    let escape = escapes[0];
    assert_eq!(escape.severity(), Severity::Warning);
    assert!(
        escape.message().contains("cross a predicate"),
        "predicate escape reason: {}",
        escape.message()
    );
    assert!(
        escape.message().contains("rule 'KWLET'"),
        "predicate escape attributes rule KWLET: {}",
        escape.message()
    );
    assert!(
        escape.path().ends_with("Spike.g4"),
        "warning path is the grammar: {}",
        escape.path().display()
    );
    assert_eq!(escape.line(), Some(4), "points to KWLET predicate line");
    assert_eq!(
        escape.column(),
        Some(16),
        "points to KWLET predicate column"
    );

    let xml_output = emit("XMLLexer.g4", true);
    let xml_escapes: Vec<_> = xml_output
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S188")
        .collect();
    assert_eq!(
        xml_escapes.len(),
        3,
        "separate diagnostic per escaped edge in XMLLexer.g4: {xml_escapes:?}"
    );
    for comment_escape in &xml_escapes[..2] {
        assert_eq!(comment_escape.line(), Some(4));
        assert_eq!(comment_escape.column(), Some(33));
        assert!(comment_escape.message().contains("cross a predicate"));
    }
    assert_eq!(xml_escapes[2].line(), Some(25));
    assert_eq!(xml_escapes[2].column(), Some(40));
    assert!(
        xml_escapes[2]
            .message()
            .contains("run a position-dependent action")
    );

    let rec_grammar = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/codegen-direct/fixtures/lexer-recursion/LexerRecursion.g4");
    let mut rec_optimization = OptimizationOptions::new();
    rec_optimization.static_lexer_dfa(true).report_only(true);
    let rec_output = emit_files_with_options(&rec_grammar, &[], true, false, &rec_optimization)
        .expect("emission succeeds");
    assert!(rec_output.files.is_empty(), "report_only emits no files");
    let rec_escapes: Vec<_> = rec_output
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S188")
        .collect();
    assert_eq!(
        rec_escapes.len(),
        2,
        "one diagnostic per escaped edge in LexerRecursion.g4: {rec_escapes:?}"
    );
    for rec_escape in rec_escapes {
        assert!(
            rec_escape
                .message()
                .contains("reach a recursive rule context"),
            "recursion escape reason: {}",
            rec_escape.message()
        );
        assert_eq!(
            rec_escape.line(),
            Some(5),
            "points to LEVEL_B recursion site"
        );
        assert_eq!(
            rec_escape.column(),
            Some(19),
            "points to LEVEL_A call in LEVEL_B"
        );
    }
}

#[test]
fn flag_off_reports_no_escape_warnings() {
    let output = emit("lexer/Spike.g4", false);
    assert!(
        output
            .diagnostics
            .iter()
            .all(|diagnostic| !diagnostic.code().starts_with("G4S")),
        "no static-DFA diagnostics without the flag: {:?}",
        output.diagnostics
    );
}

#[test]
fn fully_static_csv_reports_no_escape_warnings() {
    let output = emit("CSV.g4", true);
    assert!(
        output
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code() != "G4S188"),
        "a fully static lexer has no escape warning: {:?}",
        output.diagnostics
    );
}

#[test]
fn static_lexer_dfa_emits_optimization_manifest_with_stats_accel_and_continuations() {
    let off = emit("lexer/Spike.g4", false);
    assert!(
        off.optimization_manifest.is_none(),
        "no manifest when --static-lexer-dfa is off"
    );

    let csv_manifest = emit("CSV.g4", true)
        .optimization_manifest
        .expect("CSV manifest emitted");
    assert!(csv_manifest.contains("\"name\": \"static-lexer-dfa\""));
    assert!(csv_manifest.contains("\"safetyClass\": \"tree-and-api-preserving\""));
    assert!(csv_manifest.contains("\"omitAtn\": True"));
    assert!(
        csv_manifest.contains("\"atnWords\": 0"),
        "a fully static lexer embeds no ATN words"
    );
    assert!(csv_manifest.contains("\"status\": \"compiled\""));
    assert!(csv_manifest.contains("\"kind\": \"exclusion\""));
    assert!(csv_manifest.contains("\"kind\": \"positive\""));

    let spike_manifest = emit("lexer/Spike.g4", true)
        .optimization_manifest
        .expect("Spike manifest emitted");
    assert!(spike_manifest.contains("\"grammar\": \"Spike\""));
    assert!(spike_manifest.contains("\"omitAtn\": False"));
    assert!(spike_manifest.contains("\"status\": \"partial\""));
    assert!(spike_manifest.contains("\"kind\": \"counted-exclusion\""));
    assert!(spike_manifest.contains("\"opener\": \"/*\""));
    assert!(spike_manifest.contains("\"closer\": \"*/\""));
    assert!(spike_manifest.contains("\"reason\": \"predicate\""));
    assert!(spike_manifest.contains("\"symbols\": \"'t'\""));
    assert!(spike_manifest.contains("\"kind\": \"source-set\""));

    let xml_manifest = emit("XMLLexer.g4", true)
        .optimization_manifest
        .expect("XMLLexer manifest emitted");
    assert!(xml_manifest.contains("\"numModes\": 3"));
    assert!(xml_manifest.contains("\"mode\": \"DEFAULT_MODE\""));
    assert!(xml_manifest.contains("\"mode\": \"INSIDE\""));
    assert!(xml_manifest.contains("\"mode\": \"PROC_INSTR\""));
    assert!(xml_manifest.contains("\"reason\": \"custom-action\""));
    assert!(
        xml_manifest.contains(
            "replace the embedded action {...} with a position-independent lexer command"
        )
    );
}

fn emit_with(configure: impl FnOnce(&mut OptimizationOptions)) -> EmittedOutput {
    let grammar = grammars_dir().join("CSV.g4");
    let mut optimization = OptimizationOptions::new();
    configure(&mut optimization);
    emit_files_with_options(&grammar, &[grammars_dir()], true, false, &optimization)
        .expect("emission succeeds")
}

#[test]
fn static_parser_dfa_writes_the_manifest_entry() {
    let output = emit_with(|options| {
        options.static_parser_dfa(true);
    });
    let manifest = output
        .optimization_manifest
        .expect("the flag writes optimizations.txt");
    assert!(manifest.contains("\"name\": \"static-parser-dfa\""));
    assert!(manifest.contains("\"safetyClass\": \"tree-and-api-preserving\""));
    assert!(manifest.contains("\"decisions\": 4"));
    assert!(manifest.contains("\"staticDecisions\": 0"));
    assert!(manifest.contains("\"grammar\": \"CSVParser\""));
    assert!(
        output.reports.is_empty(),
        "the tables flag alone prints nothing"
    );
    assert!(
        !output.files.is_empty(),
        "the tables flag still emits files"
    );
}

#[test]
fn report_decisions_prints_the_decision_inventory() {
    let output = emit_with(|options| {
        options.report_decisions(true);
    });
    assert!(output.files.is_empty(), "report mode emits no files");
    let manifest = output
        .optimization_manifest
        .expect("manifest is still rendered");
    assert!(manifest.contains("\"reportOnly\": True"), "{manifest}");
    let [report] = output.reports.as_slice() else {
        panic!("one report per parser unit: {:?}", output.reports);
    };
    assert_eq!(
        report.lines().next(),
        Some("=== parser decision report: CSVParser (4 decisions) ==="),
        "{report}"
    );
    for row in [
        "  0  csvFile  12",
        "  1  row      21",
        "  2  row      25",
        "  3  field    32",
    ] {
        assert!(report.contains(row), "missing {row:?} in:\n{report}");
    }
}

#[test]
fn dump_dfas_prints_the_selected_decisions() {
    let output = emit_with(|options| {
        options.dump_dfas("row:1");
    });
    assert!(output.files.is_empty(), "report mode emits no files");
    assert!(
        output.diagnostics.is_empty(),
        "a matched selector reports nothing: {:?}",
        output.diagnostics
    );
    let [dump] = output.reports.as_slice() else {
        panic!("one dump per parser unit: {:?}", output.reports);
    };
    assert!(dump.contains("=== static DFA dump: grammar CSVParser ==="));
    assert!(
        dump.contains("vocabulary: EOF=-1 ','=1 '\\r'=2 '\\n'=3 WS=4 TEXT=5 STRING=6"),
        "{dump}"
    );
    assert!(dump.contains("--- decision 1: rule 'row' ---"), "{dump}");
    assert!(!dump.contains("--- decision 0:"), "{dump}");
    assert!(!dump.contains("--- decision 2:"), "{dump}");
    assert!(dump.contains("(1 of 4 decisions dumped)"), "{dump}");
}

#[test]
fn dump_dfas_reports_every_unmatched_selector() {
    let output = emit_with(|options| {
        options.dump_dfas("row,zzz,row:99");
    });
    let messages = output
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S191")
        .map(|diagnostic| {
            assert_eq!(diagnostic.severity(), Severity::Error);
            diagnostic.message()
        })
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        [
            "--dump-dfas: no parser decision of grammar CSVParser matches 'zzz'",
            "--dump-dfas: no parser decision of grammar CSVParser matches 'row:99'",
        ]
    );
}

#[test]
fn cli_report_modes_print_reports_and_write_no_files() {
    let grammar = grammars_dir().join("CSV.g4");
    let out_dir =
        std::env::temp_dir().join(format!("dbt-parser-dfa-report-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&out_dir);

    for (flags, header) in [
        (
            vec!["--report-decisions".to_owned()],
            "=== parser decision report: CSVParser (4 decisions) ===",
        ),
        (
            vec!["--dump-dfas".to_owned(), "row".to_owned()],
            "=== static DFA dump: grammar CSVParser ===",
        ),
    ] {
        let mut args = vec![
            "-o".to_owned(),
            out_dir.to_string_lossy().into_owned(),
            "-no-listener".to_owned(),
        ];
        args.extend(flags.clone());
        args.push(grammar.to_string_lossy().into_owned());
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        cli::run(&args, &mut stdout, &mut stderr).expect("CLI run succeeds");

        let stdout = String::from_utf8(stdout).expect("stdout is utf-8");
        assert!(stdout.contains(header), "{flags:?}: {stdout}");
        assert!(
            String::from_utf8(stderr)
                .expect("stderr is utf-8")
                .is_empty(),
            "{flags:?} prints no diagnostics"
        );
        assert!(!out_dir.exists(), "{flags:?} writes no output files");
    }
}

#[test]
fn cli_dump_dfas_fails_the_run_on_an_unmatched_selector() {
    let grammar = grammars_dir().join("CSV.g4");
    let args = vec![
        "--dump-dfas=zzz".to_owned(),
        grammar.to_string_lossy().into_owned(),
    ];
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let error =
        cli::run(&args, &mut stdout, &mut stderr).expect_err("an unmatched selector fails the run");

    assert!(
        error.to_string().contains("failed with 1 error(s)"),
        "{error}"
    );
    let stderr = String::from_utf8(stderr).expect("stderr is utf-8");
    assert!(stderr.contains("error[G4S191]"), "{stderr}");
    assert!(
        stderr.contains("no parser decision of grammar CSVParser matches 'zzz'"),
        "{stderr}"
    );
}

#[test]
fn dump_dfas_deduplicates_repeated_selectors() {
    let output = emit_with(|options| {
        options.dump_dfas("zzz,zzz,row:99,:1,row:99");
    });
    let messages = output
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.code() == "G4S191")
        .map(|diagnostic| diagnostic.message())
        .collect::<Vec<_>>();
    assert_eq!(
        messages,
        [
            "--dump-dfas: no parser decision of grammar CSVParser matches ':1'",
            "--dump-dfas: no parser decision of grammar CSVParser matches 'zzz'",
            "--dump-dfas: no parser decision of grammar CSVParser matches 'row:99'",
        ]
    );
}
