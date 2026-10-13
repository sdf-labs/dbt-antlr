// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! ClickHouse-scale stress witness for the static lexer DFA (Phase 1.5 of
//! the static-DFA plan): a big real-world lexer grammar through the whole
//! `--static-lexer-dfa` pipeline — codegen, table build, and a static-vs-adaptive
//! token-stream differential — plus a wall-time smoke measurement.
//!
//! The grammar is not vendored: the test is env-gated on
//! `STATIC_LEXER_STRESS`, the path of a directory holding
//! `ClickHouseLexer.g4` (or of the grammar file itself), for example
//! `STATIC_LEXER_STRESS=~/work/antlr4/ClickHouse/utils/antlr`. Without the
//! variable the test prints a skip note and passes, like the `ATN_SWEEP`
//! tier of `atn_packed_equivalence.rs`.
//!
//! ClickHouseLexer.g4 is a flat lexer grammar (~310 rules, no modes, no
//! predicates, no custom actions; `-> skip` commands only), so it must
//! build fully static with zero escapes; the assertions keep that honest.
#![cfg(feature = "static-dfa-build")]

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Instant;

use dbt_antlr_codegen::dbt::emit_files_with_options;
use dbt_antlr_codegen::OptimizationOptions;
use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager;
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::input_stream::InputStream;
use dbt_antlr_runtime::lexer::{BaseLexer, LexerRecog};
use dbt_antlr_runtime::recognizer::{Actions, Recognizer};
use dbt_antlr_runtime::rule_context::EmptyRuleNode;
use dbt_antlr_runtime::static_dfa_build::{StaticLexerDfaBuild, StaticLexerDfaBuilder};
use dbt_antlr_runtime::static_lexer_dfa::StaticLexerTables;
use dbt_antlr_runtime::token::{CommonToken, Token, TOKEN_EOF};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::vocabulary::{Vocabulary, VocabularyImpl};
use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::TokenSource;

/// Environment variable pointing at the ClickHouse grammar checkout.
const STRESS_ENV: &str = "STATIC_LEXER_STRESS";
const GRAMMAR_FILE: &str = "ClickHouseLexer.g4";
/// Bounds for the sample `.sql` files pulled from the checkout.
const MAX_SAMPLE_BYTES: u64 = 32 * 1024;
const MAX_SAMPLE_FILES: usize = 3;

/// Representative ClickHouse SQL fragments: keywords in both cases,
/// identifiers, every literal shape, comments, operators, and error text.
const EMBEDDED_CORPUS: &[&str] = &[
    "SELECT 1",
    "select * from system.numbers limit 10",
    "SELECT UserID, count() FROM hits GROUP BY UserID ORDER BY count() DESC LIMIT 5",
    "CREATE TABLE t (a UInt32, b String, c Array(Nullable(Float64))) ENGINE = MergeTree ORDER BY a",
    "INSERT INTO t VALUES (1, 'one', [1.0, NULL]), (2, 'two', [])",
    "WITH '2024-01-01' AS d SELECT toDateTime(d), now()",
    "SELECT sum(x * 2) FROM t WHERE x BETWEEN 1 AND 10 AND name ILIKE '%foo%'",
    "SELECT a, b FROM t1 ANY LEFT JOIN t2 USING (a)",
    "SELECT arrayMap(x -> x + 1, [1, 2, 3])",
    "SELECT uniqExact(user) FROM logs WHERE ts >= now() - INTERVAL 1 DAY",
    "SELECT 'it\\'s', \"double\", `back`",
    "SELECT 0x1F, 0b101, 1e10, 2.5e-3, 42, 007",
    "-- line comment\nSELECT 1 -- tail",
    "/* multi\nline */ SELECT /* nested-shaped */ 2",
    "SELECT x::Int32, CAST(y AS String) FROM t",
    "SELECT CASE WHEN a > 0 THEN 'pos' ELSE 'neg' END FROM t",
    "DROP TABLE IF EXISTS t ON CLUSTER 'cluster'",
    "ALTER TABLE t ADD COLUMN c UInt8 DEFAULT 0",
    "SELECT $1, @@version, ?",
    "SELECT * FROM t LIMIT 10 BY name OFFSET 5",
    "SeLeCt MiXeD CaSe FrOm T",
    "unterminated 'string",
    "SELECT @@@, ~~",
    "\u{0B}\u{0C} SELECT\t1\r\n",
];

/// Resolves the env-gated grammar location; `None` disables the test.
/// A set-but-bad value is a configuration error, not a skip: panic naming
/// the offending path.
fn stress_grammar() -> Option<PathBuf> {
    let value = std::env::var_os(STRESS_ENV)?;
    let path = PathBuf::from(value);
    let path = if path.is_dir() {
        path.join(GRAMMAR_FILE)
    } else {
        path
    };
    assert!(
        path.is_file(),
        "{STRESS_ENV} is set but no grammar file exists at {}",
        path.display()
    );
    Some(path)
}

/// A few real query files from the checkout, bounded in count and size.
fn sample_sql_inputs(grammar: &Path) -> Vec<(String, String)> {
    // <checkout>/utils/antlr/ClickHouseLexer.g4 -> <checkout>/tests/queries/0_stateless.
    let Some(root) = grammar
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent)
    else {
        return Vec::new();
    };
    let queries = root.join("tests/queries/0_stateless");
    let Ok(entries) = std::fs::read_dir(&queries) else {
        return Vec::new();
    };
    let mut candidates: Vec<(u64, PathBuf)> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "sql"))
        .filter_map(|path| {
            let len = std::fs::metadata(&path).ok()?.len();
            (len <= MAX_SAMPLE_BYTES).then_some((len, path))
        })
        .collect();
    // The largest in-bounds files give the differential and the perf smoke
    // a real workload; ties break by name for determinism.
    candidates.sort_by(|(a_len, a), (b_len, b)| b_len.cmp(a_len).then_with(|| a.cmp(b)));
    let mut inputs = Vec::new();
    for (_, path) in candidates.into_iter().take(MAX_SAMPLE_FILES) {
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        inputs.push((name, text));
    }
    inputs
}

// ======= Test lexer over a deserialized ATN (no generated code) =======

/// The compiled grammar: the ATN, the built tables, and two ATN simulator
/// managers (one per side), so a state interned by the resume path of the
/// static side can never leak into the ground-truth side. The static side
/// uses the sentinel-ATN manager of `new_static_lexer`: the grammar builds
/// with zero escapes (asserted below), so the lazy path is unreachable and
/// any walker bypass fails loudly on the empty ATN instead of silently
/// falling back to simulation.
struct Fixture {
    atn_manager_truth: &'static LexerATNSimulatorManager,
    atn_manager_static: &'static LexerATNSimulatorManager,
    tables: &'static StaticLexerTables,
}

fn compile_fixture(grammar: &Path) -> (StaticLexerDfaBuild, Fixture) {
    let library = [grammar.parent().expect("grammar has a dir").to_path_buf()];
    let units = dbt_antlr_codegen::atn_export::compile_atn_data(grammar, &library)
        .expect("grammar compiles");
    let words = units
        .iter()
        .find_map(|unit| unit.lexer_atn_words.clone())
        .expect("lexer unit");
    let atn: &'static ATN = Box::leak(Box::new(
        ATNDeserializer::new(None).deserialize(&mut words.iter()),
    ));
    let build = StaticLexerDfaBuilder::new().build(atn);
    let tables: &'static StaticLexerTables = Box::leak(Box::new(
        StaticLexerTables::from_words(&build.words).expect("tables decode"),
    ));
    let fixture = Fixture {
        atn_manager_truth: Box::leak(Box::new(LexerATNSimulatorManager::new(atn))),
        atn_manager_static: Box::leak(Box::new(LexerATNSimulatorManager::new_static_lexer(tables))),
        tables,
    };
    (build, fixture)
}

type TestLexer<'input, 'arena> =
    BaseLexer<'input, 'arena, StressExt, InputStream<'input>, CommonTokenFactory<'input, 'arena>>;

struct StressExt {
    atn_manager: &'static LexerATNSimulatorManager,
    vocabulary: VocabularyImpl,
}

impl<'input, 'arena> Actions<'input, 'arena, TestLexer<'input, 'arena>, CommonToken<'input>>
    for StressExt
where
    'input: 'arena,
{
    fn sempred(
        _localctx: Option<&'arena EmptyRuleNode<'input, 'arena, CommonToken<'input>>>,
        _rule_index: i32,
        _pred_index: i32,
        _recog: &mut TestLexer<'input, 'arena>,
    ) -> bool {
        unreachable!("ClickHouseLexer has no predicates")
    }

    fn action(
        _localctx: Option<&'arena EmptyRuleNode<'input, 'arena, CommonToken<'input>>>,
        _rule_index: i32,
        _action_index: i32,
        _recog: &mut TestLexer<'input, 'arena>,
    ) {
        unreachable!("ClickHouseLexer has no custom actions")
    }

    fn get_rule_names(&self) -> &[&str] {
        &[]
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &self.vocabulary
    }

    fn get_grammar_file_name(&self) -> &str {
        GRAMMAR_FILE
    }
}

impl<'input, 'arena>
    LexerRecog<'input, 'arena, CommonTokenFactory<'input, 'arena>, TestLexer<'input, 'arena>>
    for StressExt
where
    'input: 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] {
        &[]
    }

    fn get_literal_names(&self) -> &[Option<&str>] {
        &[]
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &[]
    }

    fn get_grammar_file_name(&self) -> &'static str {
        GRAMMAR_FILE
    }

    fn get_atn_simulator_man(&self) -> &'static LexerATNSimulatorManager {
        self.atn_manager
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Tok {
    ttype: i32,
    text: String,
    channel: i32,
    line: u32,
    column: i32,
    start: isize,
    stop: isize,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Snap {
    tokens: Vec<Tok>,
    errors: Vec<String>,
}

#[derive(Clone, Default)]
struct Collecting {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'input, 'arena, R, Tok> ErrorListener<'input, 'arena, R, Tok> for Collecting
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: Token + 'input,
{
    fn syntax_error(
        &self,
        _recognizer: &R,
        _offending: Option<&dyn Token>,
        line: u32,
        column: i32,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.messages
            .borrow_mut()
            .push(format!("line {line}:{column} {msg}"));
    }
}

/// How much of each token `drain` records: the correctness runs compare
/// the full text; the timed runs skip the per-token text copy and compare
/// type, channel, line, column, and span (start/stop) only, which together
/// with the shared input still pins the text down.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DrainMode {
    Correctness,
    Timed,
}

/// Drains one lexer over `text`. With `tables`, the walker runs the static
/// tables; without, it is the plain adaptive `LexerATNSimulator` path.
/// `text` is pre-leaked by the caller: `Arena::with` grounds 'input at
/// 'static, and the timed runs must not pay the copy inside the measured
/// region.
fn drain(
    fixture: &Fixture,
    text: &'static str,
    tables: Option<&'static StaticLexerTables>,
    mode: DrainMode,
) -> Snap {
    static EMPTY_NAMES: &[Option<&'static str>] = &[];
    Arena::with(|arena| {
        let atn_manager = if tables.is_some() {
            fixture.atn_manager_static
        } else {
            fixture.atn_manager_truth
        };
        let mut lexer = BaseLexer::new_base_lexer(
            InputStream::new(text),
            StressExt {
                atn_manager,
                vocabulary: VocabularyImpl::new(EMPTY_NAMES.iter(), EMPTY_NAMES.iter(), None),
            },
            arena,
        );
        if let Some(tables) = tables {
            lexer.get_interpreter_mut().set_static_tables(tables);
        }
        lexer.remove_error_listeners();
        let collecting = Collecting::default();
        let messages = collecting.messages.clone();
        lexer.add_error_listener(Box::new(collecting));
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            tokens.push(Tok {
                ttype: token.get_token_type(),
                text: match mode {
                    DrainMode::Correctness => token.get_text().to_owned(),
                    DrainMode::Timed => String::new(),
                },
                channel: token.get_channel(),
                line: token.get_line(),
                column: token.get_char_position_in_line(),
                start: token.get_start_index(),
                stop: token.get_stop_index(),
            });
            if token.get_token_type() == TOKEN_EOF {
                break;
            }
        }
        if tables.is_some() {
            assert_eq!(
                lexer.interpreter.as_ref().unwrap().escape_count,
                0,
                "ClickHouseLexer is fully static; an escape means the walker fell back \
                 to the lazy path"
            );
        }
        let errors = messages.borrow().clone();
        Snap { tokens, errors }
    })
}

// ======= The stress test =======

#[test]
fn clickhouse_static_lexer_stress() {
    let Some(grammar) = stress_grammar() else {
        eprintln!(
            "ClickHouse stress disabled; set {STRESS_ENV}=<dir with {GRAMMAR_FILE}> to enable"
        );
        return;
    };
    println!("grammar: {}", grammar.display());

    // 1. Codegen with --static-lexer-dfa through the library API: the
    // build must complete and the emitted lexer must carry the table
    // statics.
    let library = [grammar.parent().expect("grammar has a dir").to_path_buf()];
    let output = emit_files_with_options(
        &grammar,
        &library,
        false,
        false,
        &OptimizationOptions::default(),
        true,
    )
    .expect("--static-lexer-dfa emission succeeds");
    let lexer_file = output
        .files
        .iter()
        .find(|file| file.name == "clickhouselexer.rs")
        .expect("lexer file emitted");
    assert!(
        lexer_file.content.contains("_STATIC_LEXER_TABLE_WORDS"),
        "the emitted lexer carries the static tables"
    );
    println!("emitted lexer: {} bytes", lexer_file.content.len());
    assert!(
        lexer_file.content.contains("new_static_lexer"),
        "a fully static lexer elides the serialized ATN"
    );

    // 2. Table build + stats.
    let (build, fixture) = compile_fixture(&grammar);
    println!("table words: {}", build.words.len());
    for (index, mode) in build.modes.iter().enumerate() {
        println!(
            "mode {index}: present={} states={} escapes={:?} continuations={}",
            mode.present, mode.num_states, mode.escape_counts, mode.num_continuations
        );
        assert!(
            mode.present,
            "mode {index} builds a table: {:?}",
            mode.failure
        );
        assert_eq!(
            mode.escape_counts,
            Default::default(),
            "ClickHouseLexer has no predicates, position-dependent actions, or recursion"
        );
    }

    // 3. Differential token-stream comparison, static vs adaptive.
    let samples = sample_sql_inputs(&grammar);
    for (name, _) in &samples {
        println!("sample input: {name}");
    }
    // Leak one copy of each input up front: `drain` takes `&'static str`,
    // and the perf smoke below must not pay the copy in its timed loop.
    let inputs: Vec<(&str, &'static str)> = EMBEDDED_CORPUS
        .iter()
        .map(|text| ("embedded", *text))
        .chain(
            samples
                .iter()
                .map(|(name, text)| (name.as_str(), &*Box::leak(text.clone().into_boxed_str()))),
        )
        .collect();
    let mut total_tokens = 0;
    for (label, input) in &inputs {
        let truth = drain(&fixture, input, None, DrainMode::Correctness);
        let r#static = drain(
            &fixture,
            input,
            Some(fixture.tables),
            DrainMode::Correctness,
        );
        assert_eq!(
            truth,
            r#static,
            "token streams differ on {label} input ({} chars)",
            input.len()
        );
        total_tokens += r#static.tokens.len();
    }
    println!(
        "differential: {} inputs, {total_tokens} tokens each side, identical",
        inputs.len()
    );

    // 4. Perf smoke: wall time of static vs adaptive lexing on the largest
    // input, plus an amplified buffer for a steadier signal. Printed, never
    // asserted. The adaptive side reuses its simulator manager across
    // iterations, so its lazy DFA cache is warm — the steady-state
    // comparison, which is what the static tables target.
    let &(_, largest) = inputs
        .iter()
        .max_by_key(|(_, text)| text.len())
        .expect("corpus is not empty");
    let amplified: &'static str = Box::leak(largest.repeat(8).into_boxed_str());
    for (label, input) in [("largest", largest), ("largest x8", amplified)] {
        println!(
            "perf[{label}]: {} chars, 3 iterations (adaptive DFA warm)",
            input.len()
        );
        // Warm both sides once.
        drain(&fixture, input, None, DrainMode::Timed);
        drain(&fixture, input, Some(fixture.tables), DrainMode::Timed);
        let mut adaptive_best = f64::MAX;
        let mut static_best = f64::MAX;
        for iteration in 0..3 {
            let start = Instant::now();
            let truth = drain(&fixture, input, None, DrainMode::Timed);
            let adaptive = start.elapsed();
            let start = Instant::now();
            let r#static = drain(&fixture, input, Some(fixture.tables), DrainMode::Timed);
            let r#static_t = start.elapsed();
            assert_eq!(truth, r#static, "perf input {label} differs");
            adaptive_best = adaptive_best.min(adaptive.as_secs_f64());
            static_best = static_best.min(r#static_t.as_secs_f64());
            println!(
                "  iter {iteration}: adaptive {:7.3} ms, static {:7.3} ms",
                adaptive.as_secs_f64() * 1e3,
                r#static_t.as_secs_f64() * 1e3
            );
        }
        println!(
            "  best: adaptive {:7.3} ms, static {:7.3} ms, speedup {:.2}x",
            adaptive_best * 1e3,
            static_best * 1e3,
            adaptive_best / static_best
        );
    }
}
