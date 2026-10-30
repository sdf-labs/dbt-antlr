// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--static-parser-dfa`: the `StaticDFA.g4` static
//! tables, `LrDfa.g4` precedence dispatch, the `FuzzExpr.g4` static-vs-adaptive
//! differential, the `SDFA` word codec, the D5 escape handoff, and the
//! `--static-dfa-only` resolution of the `GuardExpr.g4` escape states.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod fuzzexprlexer;
    pub mod fuzzexprparser;
    pub mod fuzzexprparser_static;
    pub mod guardexprlexer;
    pub mod guardexprparser;
    pub mod guardexprparser_static;
    pub mod guardexprparser_static_only;
    pub mod loopfrontierlexer;
    pub mod loopfrontierparser;
    pub mod loopfrontierparser_static;
    pub mod lrdfalexer;
    pub mod lrdfaparser;
    pub mod staticdfalexer;
    pub mod staticdfaparser;
}

use dbt_antlr_runtime::atn_simulator::IATNSimulator;
use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::int_stream::IntStream;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::Parser;
use dbt_antlr_runtime::{Arena, InputStream};

use crate::gen::lrdfalexer::LrDfaLexer;
use crate::gen::lrdfaparser::LrDfaParser;
use crate::gen::staticdfalexer::StaticDFALexer;
use crate::gen::staticdfaparser::{DupContextAll, StaticDFAParser};

fn parse_static_dfa(input: &'static str, rule: &str) -> Result<String, ANTLRError> {
    Arena::with(|arena| {
        let lexer = StaticDFALexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = StaticDFAParser::new(arena, tokens);
        match rule {
            "stat" => parser
                .stat()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "expr" => parser
                .expr()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "starLoop" => parser
                .starLoop()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "plusLoop" => parser
                .plusLoop()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "opt" => parser
                .opt()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            "dup" => parser
                .dup()
                .map(|ctx| string_tree(ctx, parser.get_rule_names())),
            _ => unreachable!(),
        }
    })
}

#[test]
fn llk_alt_block() {
    assert_eq!(
        parse_static_dfa("create table foo", "stat").unwrap(),
        "(stat create table foo)"
    );
    assert_eq!(
        parse_static_dfa("create view foo", "stat").unwrap(),
        "(stat create view foo)"
    );
    assert_eq!(
        parse_static_dfa("drop table foo", "stat").unwrap(),
        "(stat drop table foo)"
    );
}

#[test]
fn llstar_dotted_name() {
    assert_eq!(
        parse_static_dfa("aa.bb.cc.dd()", "expr").unwrap(),
        "(expr (name aa . bb . cc . dd) ( ))"
    );
    assert_eq!(
        parse_static_dfa("aa.bb.cc.dd", "expr").unwrap(),
        "(expr (name aa . bb . cc . dd))"
    );
    assert_eq!(
        parse_static_dfa("aa()", "expr").unwrap(),
        "(expr (name aa) ( ))"
    );
    assert_eq!(parse_static_dfa("aa", "expr").unwrap(), "(expr (name aa))");
}

#[test]
fn dfa_star_loop() {
    assert_eq!(
        parse_static_dfa("a c", "starLoop").unwrap(),
        "(starLoop a c)"
    );
    assert_eq!(
        parse_static_dfa("a b a c", "starLoop").unwrap(),
        "(starLoop a b a c)"
    );
    assert_eq!(
        parse_static_dfa("a b a b a c", "starLoop").unwrap(),
        "(starLoop a b a b a c)"
    );
}

#[test]
fn dfa_plus_loop() {
    assert_eq!(
        parse_static_dfa("a b a c", "plusLoop").unwrap(),
        "(plusLoop a b a c)"
    );
    assert_eq!(
        parse_static_dfa("a b a b a c", "plusLoop").unwrap(),
        "(plusLoop a b a b a c)"
    );
}

#[test]
fn dfa_optional() {
    assert_eq!(parse_static_dfa("a c", "opt").unwrap(), "(opt a c)");
    assert_eq!(parse_static_dfa("a b a c", "opt").unwrap(), "(opt a b a c)");
}

#[test]
fn exact_ambiguity_resolves_to_the_min_alternative() {
    Arena::with(|arena| {
        let lexer = StaticDFALexer::<_, CommonTokenFactory>::new(arena, InputStream::new("d e"));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = StaticDFAParser::new(arena, tokens);
        let result = parser.dup().unwrap();
        assert!(
            matches!(result, DupContextAll::DupFirstContext(_)),
            "an exact ambiguity must resolve to the minimum alternative"
        );
    });
}

/// Collects `syntax_error` notifications in Java's `ConsoleErrorListener`
/// format: `line <line>:<column> <msg>`.
#[derive(Clone, Default)]
struct CollectingErrorListener {
    messages: std::rc::Rc<std::cell::RefCell<Vec<String>>>,
}

impl<'input, 'arena, R, Tok>
    dbt_antlr_runtime::error_listener::ErrorListener<'input, 'arena, R, Tok>
    for CollectingErrorListener
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: dbt_antlr_runtime::token::Token + 'input,
{
    fn syntax_error(
        &self,
        _recognizer: &R,
        _offending_symbol: Option<&dyn dbt_antlr_runtime::token::Token>,
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

fn static_dfa_errors(input: &'static str, rule: &str) -> Vec<String> {
    Arena::with(|arena| {
        let lexer = StaticDFALexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = StaticDFAParser::new(arena, tokens);
        let listener = CollectingErrorListener::default();
        let messages = listener.messages.clone();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener));
        match rule {
            "stat" => {
                let _ = parser.stat();
            }
            "expr" => {
                let _ = parser.expr();
            }
            _ => unreachable!(),
        }
        let messages = messages.borrow().clone();
        messages
    })
}

#[test]
fn no_table_edge_reports_no_viable_alternative() {
    // The table has no edge for `create foo`: the walk has to report the same
    // NoViableAlt error the adaptive simulation would, not silently predict
    // nothing.
    let errors = static_dfa_errors("create foo", "stat");
    assert_eq!(errors.len(), 1, "one syntax error is reported: {errors:?}");
    assert!(
        errors[0].contains("no viable alternative at input 'createfoo'"),
        "unexpected message: {errors:?}"
    );

    let errors = static_dfa_errors("aa.bb.", "expr");
    assert_eq!(errors.len(), 1, "one syntax error is reported: {errors:?}");
}

#[test]
fn static_dfa_parser_has_no_bare_adaptive_predict() {
    let src = include_str!("gen/staticdfaparser.rs");
    let calls = src.matches("adaptive_predict(").count();
    assert_eq!(
        calls, 0,
        "every StaticDFA.g4 decision is fully static, so its parser must not fall back at all"
    );
    assert!(
        src.contains("dfa_predict"),
        "expected static DFA call sites"
    );
}

fn parse_lr_dfa(input: &'static str) -> Result<String, ANTLRError> {
    Arena::with(|arena| {
        let lexer = LrDfaLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = LrDfaParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
    })
}

#[test]
fn lr_precedence_and_left_associativity() {
    assert_eq!(
        parse_lr_dfa("1+2*3").unwrap(),
        "(s (e (e 1) + (e (e 2) * (e 3))) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("1*2+3").unwrap(),
        "(s (e (e (e 1) * (e 2)) + (e 3)) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("1+2+3").unwrap(),
        "(s (e (e (e 1) + (e 2)) + (e 3)) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("1-2-3").unwrap(),
        "(s (e (e (e 1) - (e 2)) - (e 3)) <EOF>)"
    );
}

#[test]
fn lr_right_associativity() {
    assert_eq!(
        parse_lr_dfa("2^3^4").unwrap(),
        "(s (e (e 2) ^ (e (e 3) ^ (e 4))) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("2^3*4").unwrap(),
        "(s (e (e (e 2) ^ (e 3)) * (e 4)) <EOF>)"
    );
}

#[test]
fn lr_prefix_and_postfix_operators() {
    assert_eq!(
        parse_lr_dfa("-1+2").unwrap(),
        "(s (e (e - (e 1)) + (e 2)) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("2^-3").unwrap(),
        "(s (e (e 2) ^ (e - (e 3))) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("1+2!").unwrap(),
        "(s (e (e 1) + (e (e 2) !)) <EOF>)"
    );
    assert_eq!(parse_lr_dfa("-a!").unwrap(), "(s (e - (e (e a) !)) <EOF>)");
}

#[test]
fn lr_index_suppression_and_ternary() {
    assert_eq!(
        parse_lr_dfa("a[1]").unwrap(),
        "(s (e (e a) [ (e 1) ]) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("a[1][2]").unwrap(),
        "(s (e (e (e a) [ (e 1) ]) [ (e 2) ]) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("a[b?c:d]").unwrap(),
        "(s (e (e a) [ (e (e b) ? (e c) : (e d)) ]) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("a[1+2]*3").unwrap(),
        "(s (e (e (e a) [ (e (e 1) + (e 2)) ]) * (e 3)) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("a?b:c?d:e").unwrap(),
        "(s (e (e a) ? (e b) : (e (e c) ? (e d) : (e e))) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("1+a?b:c+2").unwrap(),
        "(s (e (e (e 1) + (e a)) ? (e b) : (e (e c) + (e 2))) <EOF>)"
    );
    assert_eq!(
        parse_lr_dfa("1*-2^a[3]!+b?c[1]:-d").unwrap(),
        "(s (e (e (e (e 1) * (e (e - (e 2)) ^ (e (e (e a) [ (e 3) ]) !))) + (e b)) ? (e (e c) [ (e 1) ]) : (e - (e d))) <EOF>)"
    );
}

#[test]
fn lr_errors_are_still_reported() {
    assert_eq!(parse_lr_dfa("1+").unwrap(), "(s (e (e 1) + e) <EOF>)");
    assert_eq!(
        parse_lr_dfa("a[").unwrap(),
        "(s (e (e a) [ e <missing ']'>) <EOF>)"
    );
    assert_eq!(parse_lr_dfa("?a").unwrap(), "(s (e ? a) <EOF>)");
    assert_eq!(parse_lr_dfa("").unwrap(), "(s e <EOF>)");
    assert_eq!(parse_lr_dfa("1]2").unwrap(), "(s (e 1) ] 2)");
}

#[test]
fn lr_dfa_parser_defers_only_from_its_tables() {
    let src = include_str!("gen/lrdfaparser.rs");
    assert_eq!(
        src.matches("adaptive_predict(").count(),
        src.matches("if _sdp == INVALID_ALT").count(),
        "LrDfa.g4 must generate no bare adaptive_predict call sites"
    );
}

fn parse_fuzz_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::fuzzexprparser_static::FuzzExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn parse_fuzz_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::fuzzexprparser::FuzzExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))
    })
}

fn assert_fuzz_equivalent(input: &str) {
    let static_result = parse_fuzz_static(input);
    let adaptive_result = parse_fuzz_adaptive(input);
    match (&static_result, &adaptive_result) {
        (Ok(static_tree), Ok(adaptive_tree)) => {
            assert_eq!(static_tree, adaptive_tree, "tree divergence on {input:?}");
        }
        (Err(_), Err(_)) => {}
        _ => panic!(
            "outcome divergence on {input:?}:\n  static:   {static_result:?}\n  adaptive: {adaptive_result:?}"
        ),
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

fn gen_expr(rng: &mut Rng, depth: usize, out: &mut String) {
    if depth == 0 || rng.below(4) == 0 {
        match rng.below(6) {
            0 => out.push('x'),
            1 => out.push('1'),
            2 => out.push_str("as"),
            3 => out.push_str("a.b"),
            4 => {
                out.push_str("f(");
                gen_expr(rng, depth.saturating_sub(1), out);
                out.push(')');
            }
            _ => {
                out.push('(');
                gen_expr(rng, depth.saturating_sub(1), out);
                out.push(')');
            }
        }
        return;
    }
    match rng.below(7) {
        0 => {
            out.push_str("- ");
            gen_expr(rng, depth - 1, out);
        }
        1 => {
            gen_expr(rng, depth - 1, out);
            out.push('[');
            gen_expr(rng, depth - 1, out);
            out.push(']');
        }
        op => {
            gen_expr(rng, depth - 1, out);
            out.push_str([" ^ ", " * ", " / ", " + ", " - "][op - 2]);
            gen_expr(rng, depth - 1, out);
        }
    }
}

fn gen_input(rng: &mut Rng) -> String {
    let mut out = String::new();
    let items = 1 + rng.below(3);
    for item in 0..items {
        if item > 0 {
            out.push_str(", ");
        }
        let depth = 1 + rng.below(4);
        gen_expr(rng, depth, &mut out);
        match rng.below(3) {
            0 => out.push_str(" as y"),
            1 => out.push_str(" as as"),
            _ => {}
        }
    }
    if rng.below(2) == 0 {
        out.push(';');
    }
    out
}

#[test]
fn fuzz_valid_expressions() {
    let mut rng = Rng(0x5EED_0001);
    for _ in 0..400 {
        let input = gen_input(&mut rng);
        assert_fuzz_equivalent(&input);
    }
}

#[test]
fn fuzz_token_soup() {
    const TOKENS: &[&str] = &[
        "x", "1", "as", "+", "-", "*", "/", "^", "(", ")", "[", "]", ",", ";", ".", "f",
    ];
    let mut rng = Rng(0x5EED_0002);
    for _ in 0..400 {
        let len = rng.below(12);
        let mut input = String::new();
        for i in 0..len {
            if i > 0 {
                input.push(' ');
            }
            input.push_str(TOKENS[rng.below(TOKENS.len())]);
        }
        assert_fuzz_equivalent(&input);
    }
}

#[test]
fn fuzz_escape_corners() {
    for input in [
        "as",
        "as as as",
        "as.as as as",
        "a as as, as as a;",
        "x + as as y",
        "f(as) as as",
        "a[as] as x",
        "- as as as",
    ] {
        assert_fuzz_equivalent(input);
    }
}

#[test]
fn sdfa_words_round_trip() {
    Arena::with(|arena| {
        let manager = gen::fuzzexprparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator.atn().static_dfas();
        assert!(!tables.is_empty());

        let words = tables.to_words();
        let decoded = StaticParserTables::from_words(&words).expect("round-trip decode");
        assert_eq!(&decoded, tables);
        assert_eq!(decoded.to_words(), words);
    });
}

/// Walks a decision's static table over the tokens of `input` and returns the
/// escape state the walk leaves at, mirroring `BaseParser::dfa_predict`.
fn frontier_escape_state(
    table: &dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTable,
    input: &str,
) -> Option<i32> {
    let tokens = Arena::with(|arena| {
        let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let mut tokens = CommonTokenStream::new(lexer);
        let mut types = Vec::new();
        loop {
            let token_type = tokens.la(1);
            types.push(token_type);
            if token_type == dbt_antlr_runtime::int_stream::EOF {
                break;
            }
            tokens.consume();
        }
        types
    });
    let mut state = 0_i32;
    for token in tokens {
        match table.accept(state) {
            accept if accept > 0 => return None,
            accept if accept < 0 => return Some(state),
            _ => {}
        }
        state = table.edge(state, token)?;
    }
    Some(state)
}

#[test]
fn frontier_escape_resumes_from_the_continuation() {
    Arena::with(|arena| {
        let manager = gen::fuzzexprparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator.atn().static_dfas();

        // `item : e 'as' name | e` is the hybrid decision: its table stops at
        // the depth-4 frontier and carries the frontier config sets.
        let item = tables.table(2, 0).expect("item has a hybrid table");
        let input = "a + b.c as y";
        let escape_state = frontier_escape_state(item, input).expect("the walk leaves the table");
        let (slot, _) = item
            .continuation(escape_state)
            .expect("the frontier escape carries a continuation");

        let tree = {
            let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new(input),
            );
            let tokens = CommonTokenStream::new(lexer);
            let mut parser = gen::fuzzexprparser_static::FuzzExprParser::new(arena, tokens);
            parser
                .s()
                .map(|ctx| string_tree(ctx, parser.get_rule_names()))
                .expect("parse succeeds")
        };
        assert_eq!(tree, parse_fuzz_adaptive(input).unwrap());

        // The escape was resumed from the continuation: the frontier config
        // set is materialized in the dynamic DFA, keyed by its slot.
        let decision = simulator.decision_to_dfa(2).expect("decision 2 exists");
        let state = decision
            .get_continuation_state(slot)
            .expect("the escape materialized its continuation state");
        assert!(
            state.state_number() >= 0,
            "the continuation state is a dynamic DFA state"
        );
    });
}

#[test]
fn continuation_state_is_reused_across_parses() {
    Arena::with(|arena| {
        let manager = gen::fuzzexprparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator.atn().static_dfas();
        let item = tables.table(2, 0).expect("item has a hybrid table");
        let input = "a + b.c as y";
        let escape_state = frontier_escape_state(item, input).expect("the walk leaves the table");
        let (slot, _) = item
            .continuation(escape_state)
            .expect("the frontier escape carries a continuation");

        let mut numbers = Vec::new();
        for _ in 0..3 {
            let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new(input),
            );
            let tokens = CommonTokenStream::new(lexer);
            let mut parser = gen::fuzzexprparser_static::FuzzExprParser::new(arena, tokens);
            parser.s().expect("parse succeeds");

            let decision = simulator.decision_to_dfa(2).expect("decision 2 exists");
            numbers.push(
                decision
                    .get_continuation_state(slot)
                    .expect("the continuation state is cached")
                    .state_number(),
            );
        }
        numbers.dedup();
        assert_eq!(
            numbers.len(),
            1,
            "every escape of the same state must reuse one continuation state, got {numbers:?}"
        );
    });
}

fn parse_guard_static(input: &str) -> String {
    Arena::with(|arena| {
        let lexer = gen::guardexprlexer::GuardExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::guardexprparser_static::GuardExprParser::new(arena, tokens);
        match parser.prog() {
            Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
            Err(error) => format!("ERR {error}"),
        }
    })
}

fn parse_guard_adaptive(input: &str) -> String {
    Arena::with(|arena| {
        let lexer = gen::guardexprlexer::GuardExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::guardexprparser::GuardExprParser::new(arena, tokens);
        match parser.prog() {
            Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
            Err(error) => format!("ERR {error}"),
        }
    })
}

/// The guarded-take differential: a take/skip conflict that only the parse
/// stack can settle. Inside an assignment target (`set a = b ;`) skip wins,
/// everywhere else take does, and the static table's stack guard defers to the
/// adaptive engine in exactly the first case.
#[test]
fn guarded_takes_match_the_adaptive_engine() {
    let corpus = [
        // Skip genuinely wins: the assignment's own '=' must come through.
        "set a = b ;",
        "set x = ( a = b ) ;",
        "set a = NOT b = c ;",
        "set a = b = c ;",
        // Take genuinely wins.
        "a = b ;",
        "a = b AND c = d ;",
        "a IN ( b ) ;",
        "( a = b ) ;",
        "NOT a = b ;",
        "a < b OR c > d ;",
        "set = a ;",
        "set a ;",
        "a ;",
    ];
    for input in corpus {
        assert_eq!(
            parse_guard_static(input),
            parse_guard_adaptive(input),
            "tree divergence on {input:?}"
        );
    }
    // The deferring case proves the guard is load-bearing: the assignment's own
    // `=` reaches the statement only when the `pred?` decision skips.
    let hint = parse_guard_static("set a = b ;");
    assert!(
        hint.contains("set (expr (value a)) = (expr (value b)) ;"),
        "the guarded skip must let the assignment through: {hint}"
    );
    let inline = parse_guard_static("a = b ;");
    assert!(
        inline.contains("(pred = (value b))"),
        "outside an assignment target the take must win: {inline}"
    );
}

/// Parses `input` with one generated `GuardExpr` parser module, and returns the
/// tree and the syntax errors.
macro_rules! parse_guard_with {
    ($parser:ident, $input:expr) => {
        Arena::with(|arena| {
            let lexer = gen::guardexprlexer::GuardExprLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new($input),
            );
            let tokens = CommonTokenStream::new(lexer);
            let mut parser = gen::$parser::GuardExprParser::new(arena, tokens);
            let listener = CollectingErrorListener::default();
            let messages = listener.messages.clone();
            parser.remove_error_listeners();
            parser.add_error_listener(Box::new(listener));
            let tree = match parser.prog() {
                Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                Err(error) => format!("ERR {error}"),
            };
            let messages = messages.borrow().clone();
            (tree, messages)
        })
    };
}

/// Parses `input` with the adaptive and the `--static-dfa-only` `GuardExpr`
/// parsers, and returns the tree and the syntax errors of each.
fn parse_guard_both(input: &str) -> [(String, Vec<String>); 2] {
    [
        parse_guard_with!(guardexprparser, input),
        parse_guard_with!(guardexprparser_static_only, input),
    ]
}

#[test]
fn static_dfa_only_parser_never_calls_adaptive_predict() {
    let src = include_str!("gen/guardexprparser_static_only.rs");
    assert!(src.contains("dfa_predict("), "expected table prediction");
    assert_eq!(src.matches("adaptive_predict(").count(), 0);
}

/// Outside an assignment target, the take of `pred?` is the right reading, so
/// the static-only parser matches the adaptive one.
#[test]
fn static_dfa_only_matches_the_adaptive_engine_where_take_wins() {
    for input in [
        "a = b ;",
        "a = b AND c = d ;",
        "a IN ( b ) ;",
        "( a = b ) ;",
        "NOT a = b ;",
        "a < b OR c > d ;",
        "a ;",
    ] {
        let [adaptive, static_only] = parse_guard_both(input);
        assert!(adaptive.1.is_empty(), "{input:?}: {adaptive:?}");
        assert_eq!(static_only, adaptive, "{input:?}");
    }
}

/// The guarded take predicts the take without the stack guard, so an
/// assignment target reads `a = b` as a comparison and the assignment misses
/// its own `=`.
#[test]
fn static_dfa_only_resolves_the_guarded_take_to_the_take() {
    let [adaptive, static_only] = parse_guard_both("set a = b ;");
    assert!(adaptive.1.is_empty(), "{adaptive:?}");
    assert!(
        static_only.0.contains("(pred = (value b))"),
        "the take wins: {static_only:?}"
    );
    assert_eq!(
        static_only.1,
        ["line 1:10 mismatched input ';' expecting {'=', 'AND', 'OR'}"]
    );
}

/// The value after `=` follows the 16-token witness of the lookahead budget
/// of the `pred?` table. At the budget, the static-only table predicts the
/// minimum viable alternative, the take.
const FRONTIER_VALUE: &str = "( b AND NOT ( NOT c < ( d AND NOT e IN ( f ) ) ) )";

/// Outside an assignment target, the take is the right guess at the budget,
/// so the static-only parser matches the adaptive one.
#[test]
fn static_dfa_only_guesses_right_at_the_lookahead_budget() {
    let input = format!("a = {FRONTIER_VALUE} ;");
    let [adaptive, static_only] = parse_guard_both(&input);
    assert!(adaptive.1.is_empty(), "{adaptive:?}");
    assert_eq!(static_only, adaptive);
}

/// In an assignment target, the skip is right. The guess at the budget takes,
/// so the static-only parser reports a syntax error.
#[test]
fn static_dfa_only_reports_a_syntax_error_after_a_wrong_guess() {
    let input = format!("set a = {FRONTIER_VALUE} ;");
    let [adaptive, static_only] = parse_guard_both(&input);
    assert!(adaptive.1.is_empty(), "{adaptive:?}");
    assert!(
        static_only
            .0
            .starts_with("(prog (stmt set (expr (value a) (pred = (value ("),
        "the take wins: {static_only:?}"
    );
    assert_eq!(
        static_only.1,
        ["line 1:59 mismatched input ';' expecting {'=', 'AND', 'OR', '<', '>', 'IN', ')'}"]
    );
}

fn parse_frontier_static(input: &str) -> String {
    Arena::with(|arena| {
        let lexer = gen::loopfrontierlexer::LoopFrontierLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::loopfrontierparser_static::LoopFrontierParser::new(arena, tokens);
        match parser.start() {
            Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
            Err(error) => format!("ERR {error}"),
        }
    })
}

fn parse_frontier_adaptive(input: &str) -> String {
    Arena::with(|arena| {
        let lexer = gen::loopfrontierlexer::LoopFrontierLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::loopfrontierparser::LoopFrontierParser::new(arena, tokens);
        match parser.start() {
            Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
            Err(error) => format!("ERR {error}"),
        }
    })
}

/// The guarded-iterate differential: the loop's exit lineage can consume an
/// operator token in a frame that is not an enclosing invocation of the same
/// loop, so the iterate/exit conflict needs the parse stack to settle.
#[test]
fn guarded_iterates_match_the_adaptive_engine() {
    // The tables must carry the guards, or the differential below would only
    // compare the adaptive engine with itself.
    Arena::with(|arena| {
        let manager = gen::loopfrontierparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator.atn().static_dfas();
        let guarded = tables
            .tables()
            .iter()
            .filter(|table| table.has_guards())
            .count();
        assert_eq!(
            guarded, 3,
            "the three operator precedence classes carry guards"
        );
    });
    let corpus = [
        "1",
        "1 + 2",
        "1 + 2 * 3",
        "( 1 + 2 ) * 3",
        "- 1 + 2",
        "1 || 2",
        "1 << 2",
        "1 + 2 || 3 << 4",
        "1 || 2 + 3",
        "CAST ( 1 AS int ) + 2",
        "EXTRACT ( x FROM 1 ) * 2",
        "CASE WHEN 1 THEN 2 ELSE 3 END + 4",
        "1 + ( 2 )",
        "row ( 1 , 2 )",
        "1 + 2 + 3 + 4",
        "1 * 2 * 3",
        "1 << 2 << 3",
        "1 || 2 || 3",
        // The foreign caller is on the stack: skip (exit) is the right reading.
        "set 1 || 2",
        "set 1 + 2 || 3",
        "set ( 1 || 2 ) || 3",
    ];
    for input in corpus {
        assert_eq!(
            parse_frontier_static(input),
            parse_frontier_adaptive(input),
            "tree divergence on {input:?}"
        );
    }
    // Both guard directions are load-bearing. Inside the foreign caller the
    // exit reading wins, so the assignment's `'||' INT` tail must reach `start`
    // rather than being absorbed by the expression loop.
    let deferring = parse_frontier_static("set 1 || 2");
    assert!(
        deferring.contains("set (expr (operand 1)) || 2"),
        "the guarded exit must let the `set` tail through: {deferring}"
    );
    // Everywhere else the iterate reading wins.
    let resolving = parse_frontier_static("1 || 2");
    assert!(
        resolving.contains("(expr (expr (operand 1)) || (expr (operand 2)))"),
        "the guarded iterate must absorb the operator: {resolving}"
    );
}
