// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--static-parser-dfa`: the `StaticDFA.g4` static
//! tables, `LrDfa.g4` precedence dispatch, the `FuzzExpr.g4` static-vs-adaptive
//! differential, the `SDFA` table image, the D5 escape handoff, and the
//! `--static-dfa-only` resolution of the `GuardExpr.g4` and `InlineAlias.g4`
//! escape and guarded states.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod betweenexprlexer;
    pub mod betweenexprparser;
    pub mod betweenexprparser_static;
    pub mod fuzzexprlexer;
    pub mod fuzzexprparser;
    pub mod fuzzexprparser_static;
    pub mod guardexprlexer;
    pub mod guardexprparser;
    pub mod guardexprparser_static;
    pub mod guardexprparser_static_only;
    pub mod inlinealiaslexer;
    pub mod inlinealiasparser;
    pub mod inlinealiasparser_static_only;
    pub mod loopfrontierlexer;
    pub mod loopfrontierparser;
    pub mod loopfrontierparser_static;
    pub mod loopobligationlexer;
    pub mod loopobligationparser;
    pub mod loopobligationparser_static;
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
        let listener = CollectingErrorListener::default();
        let messages = listener.messages.clone();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener));
        let tree = parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))?;
        let errors = messages.borrow().clone();
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        Ok(tree)
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
        let listener = CollectingErrorListener::default();
        let messages = listener.messages.clone();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener));
        let tree = parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|e| format!("{e:?}"))?;
        let errors = messages.borrow().clone();
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        Ok(tree)
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

/// One `s` input: mostly items, sometimes a `let`, `pick`, or `alias`
/// statement.
fn gen_input(rng: &mut Rng) -> String {
    let mut out = String::new();
    match rng.below(10) {
        0 | 1 => {}
        2 => {
            out.push_str("pick ");
            let depth = 1 + rng.below(4);
            gen_expr(rng, depth, &mut out);
            if rng.below(2) == 0 {
                out.push_str(" as y");
            }
            return out;
        }
        3 => {
            out.push_str("alias ");
            let depth = 1 + rng.below(4);
            gen_expr(rng, depth, &mut out);
            if rng.below(2) == 0 {
                out.push_str(" as y");
            }
            out.push_str(" as z");
            return out;
        }
        _ => return gen_item_input(rng),
    }
    out.push_str("let ");
    let depth = 1 + rng.below(4);
    gen_expr(rng, depth, &mut out);
    if rng.below(2) == 0 {
        out.push_str(" = ");
        let depth = 1 + rng.below(3);
        gen_expr(rng, depth, &mut out);
    }
    out
}

/// One `s` input of the form `item, item, ... ;`.
fn gen_item_input(rng: &mut Rng) -> String {
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
        assert!(
            parse_fuzz_adaptive(&input).is_ok(),
            "valid input had syntax error: {input:?}"
        );
        assert_fuzz_equivalent(&input);
    }
}

#[test]
fn fuzz_token_soup() {
    const TOKENS: &[&str] = &[
        "x", "1", "as", "+", "-", "*", "/", "^", "(", ")", "[", "]", ",", ";", ".", "f", "let", "=",
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
fn sdfa_image_reloads_in_place() {
    Arena::with(|arena| {
        let manager = gen::fuzzexprparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");
        assert!(!tables.is_empty());

        let words: &'static [u32] = Box::leak(tables.words().to_vec().into_boxed_slice());
        let reloaded = StaticParserTables::from_image(words).expect("the image reloads");
        assert_eq!(reloaded.words(), tables.words());
        assert_eq!(reloaded.tables().len(), tables.tables().len());
        for (reloaded, table) in reloaded.tables().iter().zip(tables.tables()) {
            assert_eq!(reloaded.words(), table.words());
            for state in 0..table.num_states() as i32 {
                assert_eq!(reloaded.accept(state), table.accept(state));
            }
        }
    });
}

/// The `FuzzExpr.g4` decision of `binding : target '=' e | e`.
const BINDING: i32 = 5;

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
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");

        // `binding : target '=' e | e` is the hybrid decision: its table stops
        // at the depth-4 frontier and carries the frontier config sets.
        let binding = tables
            .table(BINDING, 0)
            .expect("binding has a hybrid table");
        let escape_state =
            frontier_escape_state(binding, "a + b.c = y").expect("the walk leaves the table");
        let (slot, _) = binding
            .continuation(escape_state)
            .expect("the frontier escape carries a continuation");
        let input = "let a + b.c = y";

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
        let decision = simulator
            .decision_to_dfa(BINDING as usize)
            .expect("the binding decision exists");
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
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");
        let binding = tables
            .table(BINDING, 0)
            .expect("binding has a hybrid table");
        let escape_state =
            frontier_escape_state(binding, "a + b.c = y").expect("the walk leaves the table");
        let (slot, _) = binding
            .continuation(escape_state)
            .expect("the frontier escape carries a continuation");
        let input = "let a + b.c = y";

        let mut numbers = Vec::new();
        for _ in 0..3 {
            let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new(input),
            );
            let tokens = CommonTokenStream::new(lexer);
            let mut parser = gen::fuzzexprparser_static::FuzzExprParser::new(arena, tokens);
            parser.s().expect("parse succeeds");

            let decision = simulator
                .decision_to_dfa(BINDING as usize)
                .expect("the binding decision exists");
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

/// The value after `=` follows the 16-token witness of the lookahead budget
/// of the `pred?` table.
const FRONTIER_VALUE: &str = "( b AND NOT ( NOT c < ( d AND NOT e IN ( f ) ) ) )";

/// The guarded states of the static-only table keep their stack guard: where
/// the guard defers, they skip without adaptivePredict. So the static-only
/// parser matches the adaptive one inside and outside an assignment target,
/// also at the lookahead budget.
#[test]
fn static_dfa_only_matches_the_adaptive_engine() {
    let frontier_eval = format!("a = {FRONTIER_VALUE} ;");
    let frontier_assign = format!("set a = {FRONTIER_VALUE} ;");
    for input in [
        // The skip wins: the assignment's own '=' must come through.
        "set a = b ;",
        "set x = ( a = b ) ;",
        "set a = NOT b = c ;",
        frontier_assign.as_str(),
        // The take wins.
        "a = b ;",
        "a = b AND c = d ;",
        "a IN ( b ) ;",
        "( a = b ) ;",
        "NOT a = b ;",
        "a < b OR c > d ;",
        "a ;",
        frontier_eval.as_str(),
    ] {
        let [adaptive, static_only] = parse_guard_both(input);
        assert!(adaptive.1.is_empty(), "{input:?}: {adaptive:?}");
        assert_eq!(static_only, adaptive, "{input:?}");
    }
    let [_, static_only] = parse_guard_both("set a = b ;");
    assert!(
        static_only
            .0
            .contains("set (expr (value a)) = (expr (value b)) ;"),
        "the guarded skip lets the assignment through: {static_only:?}"
    );
}

/// `set a = b = c ;` is ambiguous: both `set (a = b) = c ;` and
/// `set a = (b = c) ;` parse. Full-context prediction takes the minimum
/// alternative (the take). The static-only guard sees the assignment on the
/// stack and skips. Both parses are free of errors.
#[test]
fn static_dfa_only_skips_in_an_ambiguous_assignment_target() {
    let [adaptive, static_only] = parse_guard_both("set a = b = c ;");
    assert!(adaptive.1.is_empty(), "{adaptive:?}");
    assert!(static_only.1.is_empty(), "{static_only:?}");
    assert!(
        adaptive.0.contains("(pred = (value b))"),
        "the take wins: {adaptive:?}"
    );
    assert!(
        static_only
            .0
            .starts_with("(prog (stmt set (expr (value a)) ="),
        "the skip wins: {static_only:?}"
    );
}

/// The guard sees the assignment on the stack, but not the input after the
/// lookahead of the table. In `set a = ( b ) = c = d ;`, the token that
/// rules out the skip is the second `=` after `c`. So the guard skips, and the
/// static-only parser reports a syntax error where the adaptive one takes.
#[test]
fn static_dfa_only_reports_a_syntax_error_after_a_wrong_skip() {
    let [adaptive, static_only] = parse_guard_both("set a = ( b ) = c = d ;");
    assert!(adaptive.1.is_empty(), "{adaptive:?}");
    assert!(
        adaptive
            .0
            .starts_with("(prog (stmt set (expr (value a) (pred = (value ("),
        "the take wins: {adaptive:?}"
    );
    assert!(
        static_only
            .0
            .starts_with("(prog (stmt set (expr (value a)) ="),
        "the skip wins: {static_only:?}"
    );
    assert_eq!(
        static_only.1,
        ["line 1:18 mismatched input '=' expecting {';', 'AND', 'OR'}"]
    );
}

/// Parses `input` with one generated `InlineAlias` parser module, and returns
/// the tree and the syntax errors.
macro_rules! parse_alias_with {
    ($parser:ident, $input:expr) => {
        Arena::with(|arena| {
            let lexer = gen::inlinealiaslexer::InlineAliasLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new($input),
            );
            let tokens = CommonTokenStream::new(lexer);
            let mut parser = gen::$parser::InlineAliasParser::new(arena, tokens);
            let listener = CollectingErrorListener::default();
            let messages = listener.messages.clone();
            parser.remove_error_listeners();
            parser.add_error_listener(Box::new(listener));
            let tree = match parser.stmt() {
                Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                Err(error) => format!("ERR {error}"),
            };
            let messages = messages.borrow().clone();
            (tree, messages)
        })
    };
}

/// Parses `input` with the adaptive and the `--static-dfa-only` `InlineAlias`
/// parsers, and returns the tree and the syntax errors of each.
fn parse_alias_both(input: &str) -> [(String, Vec<String>); 2] {
    [
        parse_alias_with!(inlinealiasparser, input),
        parse_alias_with!(inlinealiasparser_static_only, input),
    ]
}

#[test]
fn inline_alias_static_dfa_only_parser_never_calls_adaptive_predict() {
    let src = include_str!("gen/inlinealiasparser_static_only.rs");
    assert!(src.contains("dfa_predict("), "expected table prediction");
    assert_eq!(src.matches("adaptive_predict(").count(), 0);
}

/// The alias is a postfix operator of the recursive expression, and `AS` also
/// follows the operand of `CAST`. The static-only guard of the operator loop
/// exits where the `CAST` operand is on the stack, and keeps the alias
/// everywhere else. The `AND` states have their own guard group, so the
/// `CAST` operand does not make an `AND` exit.
#[test]
fn inline_alias_static_dfa_only_matches_the_adaptive_engine() {
    for input in [
        "SELECT x AS a , y = z AS b FROM t",
        "SELECT x AS INT64",
        "SELECT ( x AS INT64 )",
        "SELECT CAST ( x AS INT64 )",
        "SELECT CAST ( x AS y AS INT64 )",
        "SELECT CAST ( ( x AS inner ) AS INT64 ) AS outer",
        "SELECT CAST ( CAST ( x AS t ) AS INT64 )",
        "SELECT CAST ( a AND b AS INT64 )",
        "SELECT ( CAST ( a AS t ) AND b ) AS c",
        "SELECT a ROWS BETWEEN x AND y",
        "SELECT a ROWS BETWEEN CAST ( x AS INT64 ) AND y",
        "SELECT a ROWS BETWEEN x AS b AND y",
        "CREATE VIEW v ORDER BY d AS SELECT x FROM t",
        "CREATE VIEW v ORDER BY d AS a , e AS SELECT x",
        "CREATE VIEW v ORDER BY CAST ( d AS t ) AS SELECT x",
    ] {
        let [adaptive, static_only] = parse_alias_both(input);
        assert!(adaptive.1.is_empty(), "{input:?}: {adaptive:?}");
        assert_eq!(static_only, adaptive, "{input:?}");
    }
    let [_, cast] = parse_alias_both("SELECT CAST ( x AS INT64 )");
    assert!(
        cast.0.contains("(expr CAST ( (expr x) AS (type INT64) ))"),
        "the cast closes: {cast:?}"
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
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");
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
        // The budget-frontier guard: inside `SUBSTRING` without `FROM`, `1`
        // exits so `- 2` and `- ( 2 )` become the second argument; with `FROM`
        // or inside parentheses, `1` iterates to absorb the `-`.
        "SUBSTRING ( 1 - 2 )",
        "SUBSTRING ( 1 - ( 2 ) )",
        "SUBSTRING ( 1 - - 2 )",
        "SUBSTRING ( 1 - 2 FROM 3 )",
        "SUBSTRING ( 1 - ( 2 ) FROM 3 )",
        "( 1 - ( 2 ) )",
        "1 - - 2",
    ];
    for input in corpus {
        assert_eq!(
            parse_frontier_static(input),
            parse_frontier_adaptive(input),
            "tree divergence on {input:?}"
        );
    }
    // Both guard directions are load-bearing for conflict and frontier states.
    // Inside the foreign callers the exit reading wins.
    let deferring = parse_frontier_static("set 1 || 2");
    assert!(
        deferring.contains("set (expr (operand 1)) || 2"),
        "the guarded exit must let the `set` tail through: {deferring}"
    );
    let frontier_deferring = parse_frontier_static("SUBSTRING ( 1 - ( 2 ) )");
    assert!(
        frontier_deferring.contains(
            "SUBSTRING ( (expr (operand 1)) (expr - (expr (operand ( (expr (operand 2)) )))) )"
        ),
        "the frontier guard must defer inside SUBSTRING: {frontier_deferring}"
    );
    // Everywhere else the iterate reading wins.
    let resolving = parse_frontier_static("1 || 2");
    assert!(
        resolving.contains("(expr (expr (operand 1)) || (expr (operand 2)))"),
        "the guarded iterate must absorb the operator: {resolving}"
    );
    let frontier_resolving = parse_frontier_static("( 1 - ( 2 ) )");
    assert!(
        frontier_resolving.contains(
            "(operand ( (expr (expr (operand 1)) - (expr (operand ( (expr (operand 2)) )))) ))"
        ),
        "the frontier guard must iterate outside SUBSTRING: {frontier_resolving}"
    );
}

fn parse_between_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::betweenexprlexer::BetweenExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::betweenexprparser_static::BetweenExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|error| format!("{error:?}"))
    })
}

fn parse_between_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::betweenexprlexer::BetweenExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::betweenexprparser::BetweenExprParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|error| format!("{error:?}"))
    })
}

#[test]
fn between_and_collision_matches_the_adaptive_engine() {
    let corners = [
        "a BETWEEN 1 AND 2",
        "a BETWEEN 1 + 1 AND 2 * 3",
        "a BETWEEN 1 AND 2 AND b",
        "a BETWEEN 1 AND 2 AND b AND c",
        "a AND b BETWEEN 1 AND 2",
        "a = 1 AND b BETWEEN 2 AND 3",
        "a BETWEEN 1 AND 2 AND b BETWEEN 3 AND 4",
        "a BETWEEN 1 AND 2 OR b BETWEEN 3 AND 4 AND c",
        "x BETWEEN a AND b AND c AND d AND e",
        "a AND b BETWEEN 1 AND 2 AND c AND d BETWEEN 3 AND 4 AND e",
        "a BETWEEN 1 + 2 AND 3 - 4 AND b BETWEEN 5 * 6 AND 7 / 8",
        "NOT a BETWEEN 1 AND 2",
        "a NOT BETWEEN 1 AND 2",
        "a NOT BETWEEN 1 AND 2 AND b NOT BETWEEN 3 AND 4",
        "NOT a AND b NOT BETWEEN 1 AND 2",
        "a = b AND c BETWEEN 1 AND 2",
        "a < b OR c > d BETWEEN 1 AND 2",
        "( a BETWEEN 1 AND 2 ) AND b",
        "a BETWEEN ( x AND y ) AND z",
        "a BETWEEN ( x ) AND ( y )",
        "a BETWEEN 1 AND ( 2 ) AND b",
        "( a AND b ) BETWEEN 1 AND 2",
        "a BETWEEN ( x AND y ) AND ( p OR q ) AND r",
        "x BETWEEN y -> a AND b AND c",
        "x BETWEEN y -> a AND y -> b AND c",
        "x BETWEEN a AND y -> b AND c",
        "f -> a BETWEEN 1 AND 2",
        "f -> a AND b",
        "f -> a AND b BETWEEN 1 AND 2",
        "x BETWEEN f -> a AND b",
        "x BETWEEN f -> a AND b AND c",
        "( x BETWEEN f -> a AND b ) AND c",
        "f -> g -> a AND b BETWEEN 1 AND 2 AND c",
    ];
    for input in corners {
        let adaptive = parse_between_adaptive(input).expect(input);
        let static_tree = parse_between_static(input).expect(input);
        assert_eq!(static_tree, adaptive, "tree divergence on {input:?}");
    }
    assert_eq!(
        parse_between_static("x BETWEEN y -> a AND b AND c").unwrap(),
        "(s (b (v (atom x)) (pred BETWEEN (v (atom y -> (b (b (v (atom a))) AND (b (v (atom b)))))) AND (v (atom c)))) <EOF>)"
    );
    assert_eq!(
        parse_between_static("x BETWEEN f -> a AND b").unwrap(),
        "(s (b (v (atom x)) (pred BETWEEN (v (atom f -> (b (v (atom a))))) AND (v (atom b)))) <EOF>)"
    );
}

fn parse_obligation_static(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::loopobligationlexer::LoopObligationLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::loopobligationparser_static::LoopObligationParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|error| format!("{error:?}"))
    })
}

fn parse_obligation_adaptive(input: &str) -> Result<String, String> {
    Arena::with(|arena| {
        let lexer = gen::loopobligationlexer::LoopObligationLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(input),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::loopobligationparser::LoopObligationParser::new(arena, tokens);
        parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .map_err(|error| format!("{error:?}"))
    })
}

/// The obligation-discharge differential (`LoopObligation.g4`):
/// - On `. ID` (conflict state), `'prior' p '.' ID` needs the exit reading,
///   and every other stack takes the iterate reading.
/// - On `[ ID : p ]` (budget-frontier states), the first argument of
///   `'trim' '(' p p ')'` needs the exit reading, and every other stack takes
///   the iterate reading.
/// - The right operand of `p '+' p` is in `guard_pass`, so the stack walk
///   continues past `+` and sees an outer `'trim'` or `'prior'` frame.
/// - The slice-bound `p` of `'[' ID ':' p ']'` is in neither `guard_danger` nor
///   `guard_pass`, so it seals off any outer `'trim'` or `'prior'` frame.
#[test]
fn guarded_obligations_match_the_adaptive_engine() {
    Arena::with(|arena| {
        let manager = gen::loopobligationparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");
        let guarded: Vec<_> = tables
            .tables()
            .iter()
            .filter(|table| table.has_guards())
            .collect();
        assert_eq!(
            guarded.len(),
            2,
            "both hybrid precedence classes carry guards"
        );
        for table in guarded {
            // `prior` (9) and the first `p` of `trim` (15) trip the guard,
            // each in its own guard group.
            let groups = table.num_guard_groups() as u32;
            assert_eq!(groups, 2);
            let trips = |state| {
                (0..groups)
                    .filter(|&group| table.guard_step(group, state) == Some(true))
                    .count()
            };
            assert_eq!(trips(9), 1);
            assert_eq!(trips(15), 1);
            for group in 0..groups {
                // The right operand of `p '+' p` (40) is in `guard_pass`.
                assert_eq!(table.guard_step(group, 40), None);
                // The slice bound of `'[' ID ':' p ']'` (22, 31) is sealed by
                // `']'`.
                assert_eq!(table.guard_step(group, 22), Some(false));
                assert_eq!(table.guard_step(group, 31), Some(false));
            }
        }
    });

    let corpus = [
        // Conflict state (`. ID`): iterate wins outside `prior` and `trim` arg 1.
        "a . b",
        "a . b . c",
        "a . b + c",
        "a + b . c",
        // Conflict state (`. ID`): exit wins inside `prior` and `trim` arg 1.
        "prior a . b",
        "prior a . b + c",
        "prior a . b . c",
        "trim ( a . b c )",
        "trim ( a + b . c d )",
        // Conflict state (`. ID`) inside a sealed slice bound: iterate wins.
        "prior a [ x : b . c ] . d",
        "trim ( a [ x : b . c ] d )",
        // Frontier states (`[ ID : p ]`): iterate wins outside `trim` arg 1.
        "a [ x : y ]",
        "a [ x : y ] [ z : w ]",
        "a [ x : [ y : z ] ]",
        "a [ x : prior b . c ]",
        "a [ x : trim ( b c ) ]",
        "a + b [ x : y ]",
        "prior a [ x : y ] . b",
        // Frontier states (`[ ID : p ]`): exit wins inside `trim` arg 1.
        "trim ( a [ x : y ] )",
        "trim ( a [ x : [ y : z ] ] )",
        "trim ( a [ x : prior b . c ] )",
        "trim ( a [ x : trim ( b c ) ] )",
        "trim ( a + b [ x : y ] )",
        // Frontier states (`[ ID : p ]`) inside a sealed slice bound: iterate wins.
        "trim ( a [ x : b [ y : z ] ] c )",
        "prior a [ x : b [ y : z ] ] . c",
    ];
    for input in corpus {
        let adaptive = parse_obligation_adaptive(input).expect(input);
        let static_tree = parse_obligation_static(input).expect(input);
        assert_eq!(static_tree, adaptive, "tree divergence on {input:?}");
    }

    assert_eq!(
        parse_obligation_static("a . b").unwrap(),
        "(s (p (p a) . b) <EOF>)"
    );
    assert_eq!(
        parse_obligation_static("prior a . b").unwrap(),
        "(s (p prior (p a) . b) <EOF>)"
    );
    assert_eq!(
        parse_obligation_static("a [ x : y ]").unwrap(),
        "(s (p (p a) [ x : (p y) ]) <EOF>)"
    );
    assert_eq!(
        parse_obligation_static("trim ( a [ x : y ] )").unwrap(),
        "(s (p trim ( (p a) (p [ x : (p y) ]) )) <EOF>)"
    );
    assert_eq!(
        parse_obligation_static("trim ( a + b [ x : y ] )").unwrap(),
        "(s (p trim ( (p (p a) + (p b)) (p [ x : (p y) ]) )) <EOF>)"
    );
    assert_eq!(
        parse_obligation_static("prior a [ x : b [ y : z ] ] . c").unwrap(),
        "(s (p prior (p (p a) [ x : (p (p b) [ y : (p z) ]) ]) . c) <EOF>)"
    );
}
