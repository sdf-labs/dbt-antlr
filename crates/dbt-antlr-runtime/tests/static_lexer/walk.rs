//! Differential tests for the static lexer DFA walker: the table-driven
//! static table walker (plus its escape/resume/restart handling) against the
//! pure lazy `LexerATNSimulator`, token for token.
//!
//! Every grammar under `grammars/` is compiled to a lexer ATN with
//! `dbt-antlr-codegen`'s `atn-export`, the tables are built with the
//! `static-dfa-build` builder, and both sides run through `BaseLexer` with
//! the same test predicates and custom actions. Token type, text, channel,
//! line, column, start, stop, the error messages, and the custom-action log
//! must match exactly. A deterministic fragment fuzzer makes the escape
//! edges fire often; per-grammar assertions on the escape and restart
//! counters keep the differential honest.
#![cfg(feature = "static-dfa-build")]

use std::any::Any;
use std::borrow::Cow;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager;
use dbt_antlr_runtime::char_stream::CharStream;
use dbt_antlr_runtime::dfa::static_dfa_build::{StaticLexerDfaBuildOutput, StaticLexerDfaBuilder};
use dbt_antlr_runtime::dfa::static_lexer_dfa::{StaticLexerError, StaticLexerTables};
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::input_stream::InputStream;
use dbt_antlr_runtime::int_stream::IntStream;
use dbt_antlr_runtime::lexer::{BaseLexer, LexerRecog};
use dbt_antlr_runtime::recognizer::{Actions, Recognizer};
use dbt_antlr_runtime::rule_context::EmptyRuleNode;
use dbt_antlr_runtime::token::{CommonToken, Token, TOKEN_EOF};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::vocabulary::{Vocabulary, VocabularyImpl};
use dbt_antlr_runtime::{Arena, Lexer, TokenSource};

// ======= Grammar fixtures =======

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GrammarKind {
    Csv,
    Spike,
    MiniModes,
    CustomAction,
    EofLexer,
    Xml,
    StartPred,
    StartAction,
    StartRecurse,
    EofEscape,
    EofChain,
    RecursiveRule,
    SharedComments,
}

impl GrammarKind {
    fn file(self) -> &'static str {
        match self {
            GrammarKind::Csv => "CSV.g4",
            GrammarKind::Spike => "lexer/Spike.g4",
            GrammarKind::MiniModes => "lexer/MiniModes.g4",
            GrammarKind::CustomAction => "lexer/CustomAction.g4",
            GrammarKind::EofLexer => "lexer/EofLexer.g4",
            GrammarKind::Xml => "XMLLexer.g4",
            GrammarKind::StartPred => "lexer/StartPred.g4",
            GrammarKind::StartAction => "lexer/StartAction.g4",
            GrammarKind::StartRecurse => "lexer/StartRecurse.g4",
            GrammarKind::EofEscape => "lexer/EofEscape.g4",
            GrammarKind::EofChain => "lexer/EofChain.g4",
            GrammarKind::RecursiveRule => "lexer/RecursiveRule.g4",
            GrammarKind::SharedComments => "lexer/SharedComments.g4",
        }
    }
}

/// The compiled inputs of one grammar: the ATN, the tables, and two ATN
/// simulator managers (one per side), so a state interned by the resume
/// path of the static side can never leak into the ground-truth side.
struct Fixture {
    atn_manager_truth: &'static LexerATNSimulatorManager,
    atn_manager_static: &'static LexerATNSimulatorManager,
    tables: &'static StaticLexerTables,
    build: StaticLexerDfaBuildOutput,
}

fn compile(kind: GrammarKind, builder: &StaticLexerDfaBuilder) -> Fixture {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("grammars")
        .join(kind.file());
    let units =
        dbt_antlr_codegen::atn_export::compile_atn_data(&path, &[]).expect("grammar compiles");
    let words = units
        .iter()
        .find_map(|u| u.lexer_atn_words.clone())
        .expect("lexer unit");
    let atn: &'static ATN = ATNDeserializer::new(None)
        .deserialize(&mut words.iter())
        .into_static();
    let build = builder.build(atn);
    let image: &'static [u32] = Box::leak(build.words.clone().into_boxed_slice());
    let tables = StaticLexerTables::from_image(image).expect("tables load");
    Fixture {
        atn_manager_truth: Box::leak(Box::new(LexerATNSimulatorManager::new(atn))),
        atn_manager_static: Box::leak(Box::new(LexerATNSimulatorManager::new(atn))),
        tables: Box::leak(Box::new(tables)),
        build,
    }
}

// ======= Test lexer extension =======

/// A `CharStream` over an `InputStream` that hides the random-access fast
/// path: `bytes_at()` is `None`, so the static walker takes the
/// cursor-based fallback loop. Everything else delegates.
#[derive(Debug)]
struct CursorStream<'input>(InputStream<'input>);

impl IntStream for CursorStream<'_> {
    fn consume(&mut self) {
        self.0.consume();
    }

    fn la(&mut self, i: isize) -> i32 {
        self.0.la(i)
    }

    fn mark(&mut self) -> isize {
        self.0.mark()
    }

    fn release(&mut self, marker: isize) {
        self.0.release(marker)
    }

    fn index(&self) -> isize {
        self.0.index()
    }

    fn seek(&mut self, index: isize) {
        self.0.seek(index)
    }

    fn size(&self) -> isize {
        self.0.size()
    }

    fn get_source_name(&self) -> String {
        self.0.get_source_name()
    }
}

impl<'input> CharStream<'input> for CursorStream<'input> {
    fn get_text(&self, a: isize, b: isize) -> Cow<'input, str> {
        self.0.get_text(a, b)
    }
}

type TestLexer<'input, 'arena, S> =
    BaseLexer<'input, 'arena, TestExt, S, CommonTokenFactory<'input, 'arena>>;

struct TestExt {
    atn_manager: &'static LexerATNSimulatorManager,
    kind: GrammarKind,
    vocabulary: VocabularyImpl,
    log: Rc<RefCell<Vec<String>>>,
}

impl<'input, 'arena, S> Actions<'input, 'arena, TestLexer<'input, 'arena, S>, CommonToken<'input>>
    for TestExt
where
    'input: 'arena,
    S: CharStream<'input>,
{
    fn sempred(
        _localctx: Option<&'arena EmptyRuleNode<'input, 'arena, CommonToken<'input>>>,
        _rule_index: i32,
        _pred_index: i32,
        recog: &mut TestLexer<'input, 'arena, S>,
    ) -> bool {
        match recog.kind {
            // Spike: `KWLET: 'let' {p}?` — true iff the char after `let`
            // is '='. The simulator consumed the triggering char
            // speculatively before the call, so la(1) is that char.
            GrammarKind::Spike => recog.input().la(1) == '=' as i32,
            // XMLLexer COMMENT: `{true}?`. StartPred/EofEscape: the exact
            // result does not matter, only that both sides see the same.
            GrammarKind::Xml | GrammarKind::StartPred | GrammarKind::EofEscape => true,
            kind => unreachable!("{kind:?} has no predicates"),
        }
    }

    fn action(
        _localctx: Option<&'arena EmptyRuleNode<'input, 'arena, CommonToken<'input>>>,
        rule_index: i32,
        action_index: i32,
        recog: &mut TestLexer<'input, 'arena, S>,
    ) {
        match recog.kind {
            // CustomAction CUST / StartAction A: `{custom();}`.
            GrammarKind::CustomAction | GrammarKind::StartAction => {
                let message = format!("custom@{rule_index}:{action_index}");
                recog.log.borrow_mut().push(message);
            }
            // XMLLexer CLOSE: `'>' {recog.pop_mode();}`.
            GrammarKind::Xml => {
                recog.pop_mode();
            }
            kind => unreachable!("{kind:?} has no custom actions"),
        }
    }

    fn get_rule_names(&self) -> &[&str] {
        &[]
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &self.vocabulary
    }

    fn get_grammar_file_name(&self) -> &str {
        "test.g4"
    }
}

impl<'input, 'arena, S>
    LexerRecog<'input, 'arena, CommonTokenFactory<'input, 'arena>, TestLexer<'input, 'arena, S>>
    for TestExt
where
    'input: 'arena,
    S: CharStream<'input>,
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
        "test.g4"
    }

    fn get_atn_simulator_man(&self) -> &'static LexerATNSimulatorManager {
        self.atn_manager
    }
}

// ======= Snapshot runner =======

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
    log: Vec<String>,
    escapes: usize,
    restarts: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Completed(Snap),
    Panicked(String),
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

fn panic_message(payload: Box<dyn Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = payload.downcast_ref::<&'static str>() {
        (*s).to_owned()
    } else {
        "<unknown panic>".to_owned()
    }
}

fn run<S>(
    fixture: &Fixture,
    kind: GrammarKind,
    text: &str,
    tables: Option<&'static StaticLexerTables>,
    make_stream: fn(&'static str) -> S,
) -> Outcome
where
    S: CharStream<'static>,
{
    // Arena::with imposes 'input: 'static in practice; leak the input copy.
    static EMPTY_NAMES: &[Option<&'static str>] = &[];
    let text: &'static str = Box::leak(text.to_owned().into_boxed_str());
    let log = Rc::new(RefCell::new(Vec::new()));
    let log_out = log.clone();
    let atn_manager = if tables.is_some() {
        fixture.atn_manager_static
    } else {
        fixture.atn_manager_truth
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        Arena::with(|arena| {
            let mut lexer = BaseLexer::new_base_lexer(
                make_stream(text),
                TestExt {
                    atn_manager,
                    kind,
                    vocabulary: VocabularyImpl::new(EMPTY_NAMES.iter(), EMPTY_NAMES.iter(), None),
                    log,
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
                let t = lexer.next_token();
                tokens.push(Tok {
                    ttype: t.get_token_type(),
                    text: t.get_text().to_owned(),
                    channel: t.get_channel(),
                    line: t.get_line(),
                    column: t.get_char_position_in_line(),
                    start: t.get_start_index(),
                    stop: t.get_stop_index(),
                });
                if t.get_token_type() == TOKEN_EOF {
                    break;
                }
            }
            let escapes = lexer.interpreter.escape_count.get();
            let restarts = lexer.interpreter.restart_count.get();
            let errors = messages.borrow().clone();
            let log = log_out.borrow().clone();
            Snap {
                tokens,
                errors,
                log,
                escapes,
                restarts,
            }
        })
    }));
    match result {
        Ok(snap) => Outcome::Completed(snap),
        Err(payload) => Outcome::Panicked(panic_message(payload)),
    }
}

/// Runs one input on both sides and returns the static-side counters. Panics on
/// a mismatch. The static side runs twice per input: once over an `InputStream`
/// (`match_static_random_access`) and once over a `CursorStream`
/// (`match_static_cursor`); both must match the truth side and each other,
/// counters included. The comparison against the truth covers the token stream,
/// the errors, and the custom-action log; the escape and restart counters are
/// static-side instrumentation and are asserted separately by the per-grammar
/// tests.
fn check(fixture: &Fixture, kind: GrammarKind, input: &str) -> (usize, usize) {
    let truth = run(fixture, kind, input, None, InputStream::new);
    let fast = run(fixture, kind, input, Some(fixture.tables), InputStream::new);
    let cursor = run(fixture, kind, input, Some(fixture.tables), |text| {
        CursorStream(InputStream::new(text))
    });
    assert_eq!(
        fast, cursor,
        "STREAM-FLAVOR MISMATCH on {input:?}\n  fast: {fast:?}\n  cursor: {cursor:?}"
    );
    let without_counters = |outcome: &Outcome| match outcome {
        Outcome::Completed(snap) => Outcome::Completed(Snap {
            escapes: 0,
            restarts: 0,
            ..Clone::clone(snap)
        }),
        Outcome::Panicked(message) => Outcome::Panicked(message.clone()),
    };
    assert!(
        without_counters(&truth) == without_counters(&fast),
        "MISMATCH on {input:?}\n  truth: {truth:?}\n  static: {fast:?}"
    );
    match fast {
        Outcome::Completed(snap) => (snap.escapes, snap.restarts),
        Outcome::Panicked(_) => (0, 0),
    }
}

// ======= Fuzzer =======

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn pick<'a>(&mut self, items: &'a [&'a str]) -> &'a str {
        items[(self.next() % items.len() as u64) as usize]
    }
}

/// Grammar-directed fuzz: random concatenations of token-shaped fragments,
/// plus pure token soup. Returns (escape, restart) totals.
fn fuzz(fixture: &Fixture, kind: GrammarKind, fragments: &[&str], seed: u64) -> (usize, usize) {
    let mut escapes = 0;
    let mut restarts = 0;
    let mut rng = Rng(seed);
    for len in 0..=6usize {
        for _ in 0..250 {
            let mut s = String::new();
            for _ in 0..len {
                s.push_str(rng.pick(fragments));
            }
            let (e, r) = check(fixture, kind, &s);
            escapes += e;
            restarts += r;
        }
    }
    // Token soup: arbitrary bytes around the vocabulary of the grammar.
    let soup: &[&str] = &[
        "a",
        "z",
        "A",
        "Z",
        "0",
        "9",
        " ",
        "\t",
        "\n",
        "\r",
        "<",
        ">",
        "/",
        "*",
        "\"",
        "'",
        "=",
        "&",
        ";",
        "#",
        "?",
        "!",
        "@",
        "-",
        "é",
        "中",
        "\u{2070}",
        "\u{10FFFF}",
    ];
    for _ in 0..700 {
        let len = (rng.next() % 12) as usize;
        let mut s = String::new();
        for _ in 0..len {
            s.push_str(rng.pick(soup));
        }
        let (e, r) = check(fixture, kind, &s);
        escapes += e;
        restarts += r;
    }
    (escapes, restarts)
}

fn run_corpus(
    fixture: &Fixture,
    kind: GrammarKind,
    corpus: &[&str],
    fragments: &[&str],
    seed: u64,
) -> (usize, usize) {
    let mut escapes = 0;
    let mut restarts = 0;
    for input in corpus {
        let (e, r) = check(fixture, kind, input);
        escapes += e;
        restarts += r;
    }
    let (e, r) = fuzz(fixture, kind, fragments, seed);
    escapes += e;
    restarts += r;
    (escapes, restarts)
}

// ======= CSV: fully static =======

#[test]
fn csv_static_matches_lazy() {
    let fixture = compile(GrammarKind::Csv, &StaticLexerDfaBuilder::new());
    assert!(fixture.build.modes[0].table.is_some());
    assert_eq!(
        fixture.build.modes[0].escape_counts,
        Default::default(),
        "CSV compiles without escapes"
    );
    let corpus = [
        "a,b,c\n",
        "a,\"b,c\",d\r\ne,f,g\n",
        "\"a\"\"b\",x\n",
        "héllo,中,1\n",
        ",,\n",
        "@\n",
        "a,@,b\n",
        "\"unterminated",
        "",
        " ",
        "a b\nc d\n",
        "\u{10FFFF},\"q\"\n",
    ];
    let fragments = [
        "a", "xy", ",", "\"", "\"\"", "\r", "\n", " ", "é", "中", "@", "\",",
    ];
    let (escapes, restarts) = run_corpus(&fixture, GrammarKind::Csv, &corpus, &fragments, 0xC51);
    assert_eq!(escapes, 0, "CSV is fully static");
    assert_eq!(restarts, 0, "CSV never restarts");
}

// ======= Spike: predicate + recursion escapes =======

#[test]
fn spike_escapes_match_lazy() {
    let fixture = compile(GrammarKind::Spike, &StaticLexerDfaBuilder::new());
    let counts = fixture.build.modes[0].escape_counts;
    assert!(counts.predicate > 0 && counts.recursive_context == 0);
    let corpus = [
        "",
        "abc 123",
        "let=1",
        "let 1",
        "let",
        "let=",
        "letx",
        "x let= y",
        "@",
        "abc @ def",
        "/* hello */",
        "/* a */ /* b */",
        "/* a */ b */",
        "/* a /* b */ c */",
        "/* /* /* deep */ */ */",
        "/* unterminated",
        "/*",
        "/*/",
        "x/* c */y",
        "/* */*/",
        "/*/**/*/",
        "/*/*/**/*/*/",
        "/* a /* b */ c */ 42 let= z9",
        "9let=9",
        "abc\nlet=\r\n/* multi\nline /* nest\n*/ done */ 7",
        "= =",
        "l le let let=",
    ];
    let fragments = [
        "/*", "*/", "*", "/", "let", "le", "=", "==", "abc", "z", "09", " ", "\n", "@", "let=",
        "/* */",
    ];
    let pieces = ["/*", "*/", "/", "*", "a"];
    for k in 1..=6usize {
        let total = pieces.len().pow(k as u32);
        for mut n in 0..total {
            let mut s = String::new();
            for _ in 0..k {
                s.push_str(pieces[n % pieces.len()]);
                n /= pieces.len();
            }
            check(&fixture, GrammarKind::Spike, &s);
        }
    }
    let (escapes, _) = run_corpus(&fixture, GrammarKind::Spike, &corpus, &fragments, 0x591E);
    assert!(escapes > 0, "escape edges crossed");
}

// ======= MiniModes: modes and hi classes, no escapes =======

#[test]
fn mini_modes_match_lazy() {
    let fixture = compile(GrammarKind::MiniModes, &StaticLexerDfaBuilder::new());
    assert!(fixture.build.modes.iter().all(|m| m.table.is_some()));
    let corpus = [
        "text<name>more",
        "<abc>",
        "a<b>c<d>e",
        "<<>>",
        "héllo<中>/x>",
        "<a b c>",
        "1",
        "@",
        "<@>",
        "",
        "text",
        "é中\u{2070}",
        "</>",
    ];
    let fragments = ["<", ">", "/", "abc", "Z", " ", "\n", "é", "1", "@"];
    let (escapes, restarts) = run_corpus(
        &fixture,
        GrammarKind::MiniModes,
        &corpus,
        &fragments,
        0x1111,
    );
    assert_eq!((escapes, restarts), (0, 0), "MiniModes is fully static");
}

// ======= CustomAction: position-dependent action escape =======

#[test]
fn custom_action_escapes_match_lazy() {
    let fixture = compile(GrammarKind::CustomAction, &StaticLexerDfaBuilder::new());
    assert!(fixture.build.modes[0].escape_counts.custom_action > 0);
    let corpus = [
        "custx",
        "cust",
        "custy",
        "cuscustx",
        "custx cust 42",
        "custcust",
        "custx9",
        "c",
        "cu",
        "cus",
        "cust ",
        "42 custx",
        "@",
        "",
    ];
    let fragments = [
        "cust", "custx", "cus", "custo", "x", "abc", "123", " ", "\n", "@",
    ];
    let (escapes, _) = run_corpus(
        &fixture,
        GrammarKind::CustomAction,
        &corpus,
        &fragments,
        0xAC7,
    );
    assert!(escapes > 0, "custom-action escape edges crossed");
}

// ======= EofLexer: explicit static EOF edges =======

#[test]
fn eof_lexer_match_lazy() {
    let fixture = compile(GrammarKind::EofLexer, &StaticLexerDfaBuilder::new());
    assert_eq!(fixture.build.modes[0].escape_counts, Default::default());
    let corpus = [
        "ab", "a", "ab ", "ab\n a", "a b", "abab", "b", "", " ", "a\n", "ab\tab a",
    ];
    let fragments = ["a", "ab", "b", "abc", " ", "\n", "@"];
    let (escapes, restarts) =
        run_corpus(&fixture, GrammarKind::EofLexer, &corpus, &fragments, 0xE0F);
    assert_eq!((escapes, restarts), (0, 0), "EofLexer is fully static");
}

// ======= XMLLexer: modes, more, predicate and custom-action escapes =======

#[test]
fn xml_lexer_match_lazy() {
    let fixture = compile(GrammarKind::Xml, &StaticLexerDfaBuilder::new());
    let counts: Vec<_> = fixture
        .build
        .modes
        .iter()
        .map(|m| m.escape_counts)
        .collect();
    assert!(counts.iter().any(|c| c.predicate > 0));
    assert!(counts.iter().any(|c| c.custom_action > 0));
    let corpus = [
        "hello",
        "<foo>",
        "<foo>bar</foo>",
        "<a b='c'>text</a>",
        "<!-- comment -->",
        "<!-- a <!-- b --> c -->",
        "<!-- unterminated",
        "<![CDATA[x]]>",
        "<![CDATA[ unterminated",
        "<!DOCTYPE html>",
        "&amp; &#65; &#x41;",
        "&unknown",
        "&",
        "<?xml version='1.0'?>",
        "<?pi data?>",
        "<?",
        "<a/>",
        "< a>",
        "<\u{2070}foo>",
        "text with é and 中",
        "é<&",
        "",
        "<foo bar=\"baz\">x&amp;y</foo>",
        "a\nb\r\nc<d\n>",
    ];
    let fragments = [
        "<",
        ">",
        "<?",
        "?>",
        "<!--",
        "-->",
        "<![CDATA[",
        "]]>",
        "&",
        "&#",
        ";",
        "\"",
        "'",
        "=",
        "/",
        "name",
        "N1",
        " ",
        "\n",
        "é",
        "text",
        "@",
    ];
    let (escapes, _) = run_corpus(&fixture, GrammarKind::Xml, &corpus, &fragments, 0xA11);
    assert!(escapes > 0, "escape edges crossed");
}

// ======= RecursiveRule: the recursive rule escapes at its first symbol =======

#[test]
fn recursive_rule_escapes_match_lazy() {
    let fixture = compile(GrammarKind::RecursiveRule, &StaticLexerDfaBuilder::new());
    let counts = fixture.build.modes[0].escape_counts;
    assert_eq!(
        counts.recursive_context, 3,
        "two curly-brace edges into NESTED, one `<` edge into DELEGATED"
    );
    assert_eq!(
        counts.budget, 0,
        "the rules escape on their first recursive entry, so nothing reaches the budget"
    );
    let table = fixture.build.modes[0].table.as_ref().expect("table");
    assert!(
        table.num_states() < 16,
        "a recursive rule costs a handful of states, not the budget: {}",
        table.num_states()
    );

    let mut deep = String::new();
    for _ in 0..40 {
        deep.push('{');
    }
    deep.push_str("\"s\"");
    for _ in 0..40 {
        deep.push('}');
    }
    let corpus = [
        "",
        "abc 123",
        "{}",
        "{a}",
        "{{}}",
        "{{{}}}",
        "{\"q\"}",
        "{\"q\" {\"r\"}}",
        "{ /* c */ }",
        "{ /* { */ }",
        "{",
        "}",
        "}{",
        "x{1}y",
        "x{1}",
        "x{1}y z",
        "x y",
        "/* a /* b */ c */",
        "{unterminated",
        "{a}bc",
        "<>",
        "<1>",
        "<1<2>3>",
        "<1<<2>>3>",
        "<<>>",
        "<",
        "<1",
        "<1<2>",
        "x{1}y<2<3>4>",
    ];
    let fragments = [
        "{", "}", "x", "y", "a", " \"", "/*", "*/", " ", "\n", "@", "1", "<", ">", "2",
    ];
    let (escapes, _) = run_corpus(
        &fixture,
        GrammarKind::RecursiveRule,
        &corpus,
        &fragments,
        0x2EC,
    );
    assert!(escapes > 0, "the recursion escapes are crossed");
    check(&fixture, GrammarKind::RecursiveRule, &deep);

    // `.` matches both ends of the Unicode range: the wildcard transition is
    // inclusive, like the Java reference.
    let top = "/**\0\u{10FFFF}*/";
    let Outcome::Completed(snap) = run(
        &fixture,
        GrammarKind::RecursiveRule,
        top,
        None,
        InputStream::new,
    ) else {
        panic!("the lazy path completes");
    };
    assert!(
        snap.errors.is_empty(),
        "the lazy path recognizes the comment: {:?}",
        snap.errors
    );
    assert_eq!(snap.tokens.len(), 2, "one comment token plus EOF");
    assert_eq!(snap.tokens[0].text, top);
    check(&fixture, GrammarKind::RecursiveRule, top);
}

// ======= SharedComments: the context-level recursion escape =======

#[test]
fn shared_comments_escape_at_the_context_level() {
    let fixture = compile(GrammarKind::SharedComments, &StaticLexerDfaBuilder::new());
    let counts = fixture.build.modes[0].escape_counts;
    assert_eq!(
        counts.recursive_context, 1,
        "the mixed config set blocks the strip, so the recursion escapes"
    );
    assert_eq!(counts.budget, 0);
    let corpus = [
        "",
        "a b",
        "/* x */",
        "/**/",
        "/* /* */ */",
        "/* /* /* */ */ */",
        "/* unterminated",
        "a/*b*/c",
        "/* a */ /* b */",
    ];
    let fragments = ["/*", "*/", "/", "*", "a", " ", "\n", "@"];
    let (escapes, _) = run_corpus(
        &fixture,
        GrammarKind::SharedComments,
        &corpus,
        &fragments,
        0x5CC,
    );
    assert!(escapes > 0, "the recursion escape is crossed");
}

// ======= Start-closure failures: whole mode falls back to lazy =======

#[test]
fn start_closure_failures_fall_back_to_lazy() {
    for (kind, corpus) in [
        (
            GrammarKind::StartPred,
            &["x", "y", "xx", "", " ", "@", "xy"][..],
        ),
        (
            GrammarKind::StartAction,
            &["x", "y", "xx", "", " ", "@"][..],
        ),
        (
            GrammarKind::StartRecurse,
            &["xa", "x", "xaa", "y", "xaaa", "", " ", "@", "xaay"][..],
        ),
    ] {
        let fixture = compile(kind, &StaticLexerDfaBuilder::new());
        assert!(
            fixture.build.modes.iter().all(|m| !m.table.is_some()),
            "{kind:?} has no table"
        );
        let fragments = ["x", "a", "y", " ", "\n", "@"];
        let (escapes, restarts) = run_corpus(&fixture, kind, corpus, &fragments, 0x57A7);
        assert_eq!((escapes, restarts), (0, 0), "{kind:?} never sees a table");
    }
}

// ======= EofEscape: the EOF edge escapes on a guarded EOF =======

#[test]
fn eof_escape_matches_lazy() {
    let fixture = compile(GrammarKind::EofEscape, &StaticLexerDfaBuilder::new());
    assert_eq!(fixture.build.modes[0].escape_counts.predicate, 1);
    // "a" takes the EOF escape edge; the resumed closure evaluates the
    // predicate at EOF, and the speculative consume at EOF panics in the
    // input stream — identically on both sides (a pre-existing runtime
    // quirk of evaluate_predicate, matched here on purpose).
    let (escapes, _) = run_corpus(
        &fixture,
        GrammarKind::EofEscape,
        &["a", "aa", "ba", "b", "ab", "", " "],
        &["a", "b", " ", "@"],
        0xE5C,
    );
    let _ = escapes; // every escape crossing of this grammar panics at EOF
                     // Explicit both-panic check on the escape-crossing input.
    let truth = run(
        &fixture,
        GrammarKind::EofEscape,
        "a",
        None,
        InputStream::new,
    );
    let got = run(
        &fixture,
        GrammarKind::EofEscape,
        "a",
        Some(fixture.tables),
        InputStream::new,
    );
    assert!(matches!(truth, Outcome::Panicked(_)));
    assert_eq!(truth, got);
}

// ======= EofChain: chained static EOF edges accept at the first EOF accept =======

#[test]
fn eof_chain_accepts_match_lazy() {
    let fixture = compile(GrammarKind::EofChain, &StaticLexerDfaBuilder::new());
    assert!(fixture.build.modes[0].table.is_some());
    let corpus = ["a", "b", "ab", "ba", "aa", "", " "];
    let fragments = ["a", "b", " ", "\n"];
    let (escapes, restarts) =
        run_corpus(&fixture, GrammarKind::EofChain, &corpus, &fragments, 0xC41);
    assert_eq!(escapes, 0, "EofChain has no escape edges");
    // The builder computes EOF edges with the lazy `treat_eofas_epsilon`
    // closure semantics: the closure follows the rest of an EOF epsilon
    // chain as epsilon, so every EOF-edge target is an accept state. The
    // walk breaks at the first EOF accept (like the lazy simulator's
    // `capture_sim_state && symbol == EOF` break).
    assert_eq!(
        restarts, 0,
        "every EOF-edge target accepts; the walk breaks at the first EOF accept"
    );
}

// ======= Forced budget: every edge escapes without a continuation =======

#[test]
fn budget_escape_restarts_match_lazy() {
    let fixture = compile(
        GrammarKind::Csv,
        &StaticLexerDfaBuilder::new().with_max_states_per_mode(1),
    );
    assert!(fixture.build.modes[0].table.is_some());
    assert!(fixture.build.modes[0].escape_counts.budget > 0);
    let corpus = ["a,b\n", "\"x\",y\n", "héllo,中\n", "@\n", "", " ", "a b\n"];
    let fragments = ["a", ",", "\"", "\n", " ", "é", "@"];
    let (_, restarts) = run_corpus(&fixture, GrammarKind::Csv, &corpus, &fragments, 0xBD6);
    assert!(restarts > 0, "budget escapes restarted tokens");
}

// ======= Negative tests: corrupted images =======

/// The CSV image, mutable so a test can corrupt one word of it.
fn csv_image() -> &'static mut [u32] {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("grammars")
        .join("CSV.g4");
    let units =
        dbt_antlr_codegen::atn_export::compile_atn_data(&path, &[]).expect("grammar compiles");
    let words = units
        .iter()
        .find_map(|u| u.lexer_atn_words.clone())
        .expect("lexer unit");
    let atn = ATNDeserializer::new(None).deserialize(&mut words.iter());
    let words = StaticLexerDfaBuilder::new().build(&atn).words;
    Box::leak(words.into_boxed_slice())
}

#[test]
fn bad_magic_is_a_clean_error() {
    let words = csv_image();
    words[0] = 0xDEAD_BEEF;
    let err = StaticLexerTables::from_image(words).err().unwrap();
    assert_eq!(err, StaticLexerError::BadMagic { found: 0xDEAD_BEEF });
}

#[test]
fn length_mismatch_is_a_clean_error() {
    let words = csv_image();
    let len = words.len() as u32;
    words[2] = len + 1;
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::TotalLengthMismatch { .. })
    ));
}

#[test]
fn truncated_image_is_a_clean_error() {
    let words = csv_image();
    assert!(matches!(
        StaticLexerTables::from_image(&words[..2]),
        Err(StaticLexerError::TruncatedImage { .. })
    ));
}

#[test]
fn mode_table_outside_the_image_is_a_clean_error() {
    let words = csv_image();
    let mode_count = words[3] as usize;
    words[3] = (mode_count + 1024) as u32;
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::OutOfBounds { .. })
    ));
}

#[test]
fn mode_image_outside_the_image_is_a_clean_error() {
    let words = csv_image();
    words[5] = u32::MAX - 3; // the length of mode 0
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::OutOfBounds { .. })
    ));
}

#[test]
fn non_contiguous_or_half_absent_mode_entry_is_a_clean_error() {
    let words = csv_image();
    words[4] = 4; // mode 0 offset != 0
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::Malformed { .. })
    ));

    let words = csv_image();
    words[4] = u32::MAX; // absent offset, but len is still non-MAX
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::Malformed { .. })
    ));
}

#[test]
fn mode_image_with_a_bad_length_is_a_clean_error() {
    let words = csv_image();
    let offset = 6 + words[4] as usize;
    words[offset] = 0xDEAD_BEEF; // the declared word count of mode 0
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::TotalLengthMismatch { .. })
    ));
}

#[test]
fn mode_image_shorter_than_its_header_is_a_clean_error() {
    let words = csv_image();
    words[5] = 8; // the length of mode 0
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::OutOfBounds { .. })
    ));
}

#[test]
fn section_outside_the_image_is_a_clean_error() {
    let words = csv_image();
    let offset = 6 + words[4] as usize;
    words[offset + 5] = u32::MAX - 3; // the byte offset of the eof section
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::OutOfBounds { .. })
    ));
}

#[test]
fn misaligned_section_is_a_clean_error() {
    let words = csv_image();
    let offset = 6 + words[4] as usize;
    words[offset + 5] += 2; // the byte offset of the eof section
    assert!(matches!(
        StaticLexerTables::from_image(words),
        Err(StaticLexerError::OutOfBounds { .. })
    ));
}

#[test]
fn trailing_words_are_a_clean_error() {
    let words = csv_image();
    let len = words.len() as u32;
    let mut extended = words.to_vec();
    extended.push(0);
    extended[2] = len + 1;
    let image: &'static [u32] = Box::leak(extended.into_boxed_slice());
    assert!(matches!(
        StaticLexerTables::from_image(image),
        Err(StaticLexerError::TotalLengthMismatch { .. })
    ));
}

#[test]
#[should_panic(expected = "regenerate")]
fn version_mismatch_panics_with_regenerate_message() {
    let words = csv_image();
    words[1] = 99;
    let _ = StaticLexerTables::from_image(words);
}
