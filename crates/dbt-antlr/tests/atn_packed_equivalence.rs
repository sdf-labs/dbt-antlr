//! Differential ATN equivalence tests.
//!
//! Tier 1 (always runs) compares the packed-ATN deserializer against the
//! proven Java-word deserializer on the dbt-antlr-runtime golden grammars, using the
//! `.interp` files produced by the Java ANTLR tool as oracles.
//!
//! Tier 2 (runs only with `ATN_SWEEP=1`) sweeps the codegen fixture corpus
//! the same way.

// Sweep progress goes to stdout/stderr like other long-running tests.
#![allow(clippy::print_stdout, clippy::print_stderr)]
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use dbt_antlr::atn_export::{GrammarAtnData, compile_atn_data};
use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::atn_dump::dump_atn;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;

const COMBINED_GRAMMARS: [&str; 6] = [
    "CSV",
    "Labels",
    "ReferenceToATN",
    "SimpleLR",
    "VisitorBasic",
    "VisitorCalc",
];

/// Fixture directories the sweep skips, with the reason for each skip.
/// Every entry names a negative fixture: the grammar is intentionally
/// invalid and the codegen pipeline rejects it, so there is no compiled ATN
/// to compare against the Java `.interp` oracle. The sweep re-checks that
/// compilation of a skipped fixture still fails.
const SKIP: [(&str, &str); 15] = [
    (
        "testbasicsemanticerrors-testargumentretvallocalconflicts-fd702fec44",
        "intentional parameter/return/local name conflicts (G4S056..G4S064)",
    ),
    (
        "testbasicsemanticerrors-testillegalnonsetlabel-5c18487902",
        "intentional label on a non-set block (G4S055)",
    ),
    (
        "testleftrecursiontoolissues-testcheckforleftrecursiveemptyfollow-558283d55a",
        "intentional left-recursive alternative with empty follow (G4A002)",
    ),
    (
        "testleftrecursiontoolissues-testcheckfornonleftrecursiverule-477f42142e",
        "intentional left-recursive rule without a non-recursive alternative (G4R002)",
    ),
    (
        "testleftrecursiontoolissues-testisolatedleftrecursiveruleref-43f8252e7d",
        "intentional unhandled left-recursion pattern (G4R001)",
    ),
    (
        "testleftrecursiontoolissues-testleftrecursiverulerefwitharg-40cd52608d",
        "intentional unhandled left-recursion pattern (G4R001)",
    ),
    (
        "testleftrecursiontoolissues-testleftrecursiverulerefwitharg2-7332bdbd4f",
        "intentional unhandled left-recursion pattern (G4R001)",
    ),
    (
        "testleftrecursiontoolissues-testleftrecursiverulerefwitharg3-719e121a92",
        "intentional unhandled left-recursion pattern (G4R001)",
    ),
    (
        "testsymbolissues-testlabelsfortokenswithmixedtypes-0a6a086afc",
        "intentional conflicting label types (G4S041)",
    ),
    (
        "testsymbolissues-testlabelsfortokenswithmixedtypeslrwithoutlabels-15b35eab8e",
        "intentional conflicting label types (G4S041)",
    ),
    (
        "testsymbolissues-testundefinedlabel-d2fa215436",
        "intentional undefined rule parameter reference (G4S042)",
    ),
    (
        "testtoolsyntaxerrors-testepsilonclosureanalysis-fbecc8c0c7",
        "intentional closure over an empty-matching body (G4A001)",
    ),
    (
        "testtoolsyntaxerrors-testepsilonnestedclosureanalysis-6379007236",
        "intentional closure over an empty-matching body (G4A001)",
    ),
    (
        "testtoolsyntaxerrors-testruleredefinition-9db9b586df",
        "intentional rule redefinition (G4S002)",
    ),
    (
        "vscode-indirect-left-recursion",
        "mutual left recursion (G4A005); the runtime ATN graph cannot represent it",
    ),
];

fn grammars_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../dbt-antlr-runtime/grammars")
}

fn gen_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../dbt-antlr-runtime/tests/gen")
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/codegen-direct/fixtures")
}

/// Parses the word list of the `atn:` section of a `.interp` file.
///
/// The section is a `atn:` line followed by a bracketed, comma-separated
/// integer list that can span more than one line.
fn parse_interp_atn(path: &Path) -> Vec<i32> {
    let text =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let mut lines = text.lines();
    for line in &mut lines {
        if line.trim_end() == "atn:" {
            break;
        }
    }
    let mut body = String::new();
    for line in &mut lines {
        body.push_str(line);
        if line.contains(']') {
            break;
        }
    }
    let start = body
        .find('[')
        .unwrap_or_else(|| panic!("no atn word list in {}", path.display()));
    let end = body
        .find(']')
        .unwrap_or_else(|| panic!("unterminated atn word list in {}", path.display()));
    body[start + 1..end]
        .split(',')
        .map(|word| {
            word.trim()
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("bad atn word {word:?} in {}: {e}", path.display()))
        })
        .collect()
}

/// Deserializes Java-format ATN words with the proven deserializer.
fn java_word_atn(words: &[i32]) -> ATN {
    ATNDeserializer::new(None).deserialize(&mut words.iter())
}

/// Deserializes packed ATN words with the candidate deserializer.
fn packed_atn(words: &[u32], context: &str) -> ATN {
    PackedATNDeserializer::new()
        .deserialize(words)
        .unwrap_or_else(|e| panic!("packed deserialization failed for {context}: {e}"))
}

/// Returns the direct subdirectories of a fixture directory. Fixture
/// grammars can import grammars stored in a subdirectory (for example
/// `sub/S.g4`), and the compiler finds them through library directories.
fn library_dirs(dir: &Path) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect()
}

/// Renders the first differing line of two dumps with one line of context.
fn first_diff(oracle: &str, candidate: &str) -> String {
    let oracle_lines: Vec<&str> = oracle.lines().collect();
    let candidate_lines: Vec<&str> = candidate.lines().collect();
    for i in 0..oracle_lines.len().max(candidate_lines.len()) {
        let a = oracle_lines.get(i).copied().unwrap_or("<missing>");
        let b = candidate_lines.get(i).copied().unwrap_or("<missing>");
        if a != b {
            let before = i.saturating_sub(1);
            let context = oracle_lines[before..i]
                .iter()
                .fold(String::new(), |mut acc, line| {
                    acc.push_str("  context:   ");
                    acc.push_str(line);
                    acc.push('\n');
                    acc
                });
            return format!(
                "first difference at line {}:\n{context}  oracle:    {a}\n  candidate: {b}",
                i + 1,
            );
        }
    }
    "dumps differ in trailing newline only".to_owned()
}

fn assert_dumps_equal(context: &str, oracle: &str, candidate: &str) {
    assert!(
        oracle == candidate,
        "ATN dump mismatch for {context}:\n{}",
        first_diff(oracle, candidate),
    );
}

/// Finds the parser unit and the lexer unit of a compiled grammar.
fn split_units(units: &[GrammarAtnData]) -> (Option<&GrammarAtnData>, Option<&GrammarAtnData>) {
    let parser = units.iter().find(|unit| unit.parser_packed_words.is_some());
    let lexer = units.iter().find(|unit| unit.lexer_atn_words.is_some());
    (parser, lexer)
}

/// Checks one grammar: `.interp` parser oracle vs packed deserializer, and
/// (for combined grammars) `.interp` lexer oracle vs the codegen lexer word
/// stream. `grammar_stem` is the `.g4` file name without extension.
fn check_grammar(grammar_path: &Path, parser_interp: &Path, lexer_interp: Option<&Path>) {
    let name = grammar_path.display().to_string();
    let units = compile_atn_data(grammar_path, &[])
        .unwrap_or_else(|e| panic!("compilation failed for {name}: {e}"));
    let (parser, lexer) = split_units(&units);

    let parser = parser.unwrap_or_else(|| panic!("no parser unit for {name}"));
    let oracle_words = parse_interp_atn(parser_interp);
    let oracle = dump_atn(&java_word_atn(&oracle_words));
    let candidate = dump_atn(&packed_atn(
        parser.parser_packed_words.as_ref().unwrap(),
        &name,
    ));
    assert_dumps_equal(&name, &oracle, &candidate);

    if let Some(lexer_interp) = lexer_interp {
        let lexer = lexer.unwrap_or_else(|| panic!("no lexer unit for {name}"));
        let oracle_words = parse_interp_atn(lexer_interp);
        let candidate_words = lexer.lexer_atn_words.as_ref().unwrap();
        assert_eq!(
            &oracle_words, candidate_words,
            "lexer ATN words differ for {name}",
        );
        assert_dumps_equal(
            &name,
            &dump_atn(&java_word_atn(&oracle_words)),
            &dump_atn(&java_word_atn(candidate_words)),
        );
    }
}

#[test]
fn golden_combined_grammars_match_java_oracle() {
    for grammar in COMBINED_GRAMMARS {
        check_grammar(
            &grammars_dir().join(format!("{grammar}.g4")),
            &gen_dir().join(format!("{grammar}.interp")),
            None,
        );
    }
}

#[test]
fn golden_xml_lexer_matches_java_oracle() {
    let units =
        compile_atn_data(&grammars_dir().join("XMLLexer.g4"), &[]).expect("XMLLexer.g4 compiles");
    let (_, lexer) = split_units(&units);
    let lexer = lexer.expect("XMLLexer.g4 yields a lexer unit");
    let candidate_words = lexer.lexer_atn_words.as_ref().unwrap();

    let oracle_words = parse_interp_atn(&gen_dir().join("XMLLexer.interp"));
    assert_eq!(
        &oracle_words, candidate_words,
        "lexer ATN words differ for XMLLexer",
    );
    assert_dumps_equal(
        "XMLLexer",
        &dump_atn(&java_word_atn(&oracle_words)),
        &dump_atn(&java_word_atn(candidate_words)),
    );
}

/// Tier 2: sweeps every applicable codegen fixture directory.
#[test]
fn fixture_sweep() {
    if env::var("ATN_SWEEP").as_deref() != Ok("1") {
        eprintln!("fixture sweep disabled; run with ATN_SWEEP=1 to enable");
        return;
    }

    let mut checked = 0;
    let mut not_applicable = 0;
    let mut skipped: Vec<String> = Vec::new();
    let mut failures: Vec<String> = Vec::new();

    let mut dirs: Vec<PathBuf> = fs::read_dir(fixtures_dir())
        .expect("fixture dir readable")
        .map(|entry| entry.expect("fixture entry readable").path())
        .filter(|path| path.is_dir())
        .collect();
    dirs.sort();

    for dir in dirs {
        let dir_name = dir.file_name().unwrap().to_string_lossy().into_owned();
        let g4s: Vec<PathBuf> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "g4"))
            .collect();
        let interps: Vec<PathBuf> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "interp"))
            .collect();
        if g4s.len() != 1 || interps.is_empty() {
            not_applicable += 1;
            continue;
        }
        if let Some((_, reason)) = SKIP.iter().find(|(name, _)| *name == dir_name) {
            assert!(
                compile_atn_data(&g4s[0], &library_dirs(&dir)).is_err(),
                "skipped fixture {dir_name} now compiles; remove it from SKIP ({reason})",
            );
            skipped.push(dir_name.clone());
            continue;
        }

        let result = std::panic::catch_unwind(|| check_fixture(&dir, &g4s[0], &interps));
        match result {
            Ok(()) => checked += 1,
            Err(payload) => {
                let message = payload
                    .downcast_ref::<String>()
                    .cloned()
                    .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                    .unwrap_or_else(|| "<non-string panic>".to_owned());
                failures.push(format!("{dir_name}: {message}"));
            }
        }
    }

    println!(
        "sweep summary: {checked} fixtures checked, {} skipped ({:?}), {} not applicable, {} failures",
        skipped.len(),
        skipped,
        not_applicable,
        failures.len(),
    );
    assert!(
        failures.is_empty(),
        "sweep failures:\n{}",
        failures.join("\n\n"),
    );
}

/// Runs the differential check for one fixture directory.
fn check_fixture(dir: &Path, g4: &Path, interps: &[PathBuf]) {
    let dir_name = dir.file_name().unwrap().to_string_lossy().into_owned();
    let stem = g4.file_stem().unwrap().to_string_lossy().into_owned();

    let units = compile_atn_data(g4, &library_dirs(dir))
        .unwrap_or_else(|e| panic!("compilation failed: {e}"));
    let (parser, lexer) = split_units(&units);

    for interp in interps {
        let interp_stem = interp.file_stem().unwrap().to_string_lossy().into_owned();
        let oracle_words = parse_interp_atn(interp);
        let context = format!("{dir_name}/{interp_stem}");
        if lexer.is_some_and(|unit| unit.name == interp_stem) {
            let candidate_words = lexer.unwrap().lexer_atn_words.as_ref().unwrap();
            assert_eq!(
                &oracle_words, candidate_words,
                "lexer ATN words differ for {context}",
            );
        } else if parser.is_some_and(|unit| interp_stem == stem || unit.name == interp_stem) {
            let oracle = dump_atn(&java_word_atn(&oracle_words));
            let candidate = dump_atn(&packed_atn(
                parser.unwrap().parser_packed_words.as_ref().unwrap(),
                &context,
            ));
            assert_dumps_equal(&context, &oracle, &candidate);
        } else {
            panic!("no compiled unit matches oracle {context}");
        }
    }
}
