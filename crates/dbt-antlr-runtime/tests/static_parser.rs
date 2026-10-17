// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for `--static-parser-dfa`: `StaticDFA.g4`, `LrDfa.g4`,
//! `FuzzExpr.g4` differential fuzzing, `SDFA` round-trip codec, and D5
//! `CONFLICT_ESCAPE` / `ParserContinuation` handoff.

#[rustfmt::skip]
#[allow(dead_code)]
mod gen {
    #![allow(non_snake_case)]
    #![allow(clippy::all, warnings)]
    pub mod fuzzexprlexer;
    pub mod fuzzexprparser;
    pub mod fuzzexprparser_static;
    pub mod lrdfalexer;
    pub mod lrdfaparser;
    pub mod staticdfalexer;
    pub mod staticdfaparser;
}

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::dfa::static_parser_dfa::StaticDFATables;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::{Arena, InputStream};

use crate::gen::lrdfalexer::LrDfaLexer;
use crate::gen::lrdfaparser::LrDfaParser;
use crate::gen::staticdfalexer::StaticDFALexer;
use crate::gen::staticdfaparser::{DupContextAll, StaticDFAParser};

fn parse_static_dfa(
    input: &'static str,
    rule: &str,
) -> Result<String, dbt_antlr_runtime::errors::ANTLRError> {
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
fn test_llk_alt_block() {
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
fn test_llstar_dotted_name() {
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
fn test_dfa_star_loop() {
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
fn test_dfa_plus_loop() {
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
fn test_dfa_optional() {
    assert_eq!(parse_static_dfa("a c", "opt").unwrap(), "(opt a c)");
    assert_eq!(parse_static_dfa("a b a c", "opt").unwrap(), "(opt a b a c)");
}

#[test]
fn test_exact_ambiguity_min_alt() {
    Arena::with(|arena| {
        let lexer = StaticDFALexer::<_, CommonTokenFactory>::new(arena, InputStream::new("d e"));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = StaticDFAParser::new(arena, tokens);
        let result = parser.dup().unwrap();
        assert!(
            matches!(result, DupContextAll::DupFirstContext(_)),
            "min-alt resolution must pick #dupFirst"
        );
    });
}

#[test]
fn test_no_viable_alt_handling() {
    let r = parse_static_dfa("create foo", "stat");
    if let Ok(tree) = r {
        assert!(tree.contains("create"));
    }

    let r = parse_static_dfa("aa.bb.", "expr");
    if let Ok(tree) = r {
        assert!(tree.contains("aa"));
    }
}

#[test]
fn test_no_adaptive_predict_in_static_dfa_generated_code() {
    let src = include_str!("gen/staticdfaparser.rs");
    assert_eq!(
        src.matches("adaptive_predict(").count(),
        src.matches("if _sdp == INVALID_ALT").count(),
        "StaticDFA parser should be fully table-driven"
    );
    assert_eq!(
        src.matches("adaptive_predict(").count(),
        0,
        "escape-free StaticDFA parser must emit zero adaptive_predict fallbacks"
    );
    assert!(
        src.contains("dfa_predict"),
        "expected static DFA call sites"
    );
}

fn parse_lr_dfa(input: &'static str) -> Result<String, dbt_antlr_runtime::errors::ANTLRError> {
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
fn test_lr_precedence_and_left_associativity() {
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
fn test_lr_right_associativity() {
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
fn test_lr_prefix_and_postfix_operators() {
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
fn test_lr_index_suppression_and_ternary() {
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
fn test_lr_errors_still_reported() {
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
fn test_no_bare_adaptive_predict_in_lr_dfa_generated_code() {
    let src = include_str!("gen/lrdfaparser.rs");
    assert_eq!(
        src.matches("adaptive_predict(").count(),
        src.matches("if _sdp == INVALID_ALT").count(),
        "LrDfa.g4 must generate zero bare adaptive_predict call sites"
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
    let s = parse_fuzz_static(input);
    let a = parse_fuzz_adaptive(input);
    match (&s, &a) {
        (Ok(st), Ok(at)) => assert_eq!(st, at, "tree divergence on input: {input:?}"),
        (Err(_), Err(_)) => {}
        _ => panic!("outcome divergence on input {input:?}:\n  static:   {s:?}\n  adaptive: {a:?}"),
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
    for i in 0..items {
        if i > 0 {
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
fn test_fuzz_valid_expressions() {
    let mut rng = Rng(0x5EED_0001);
    for _ in 0..400 {
        let input = gen_input(&mut rng);
        assert_fuzz_equivalent(&input);
    }
}

#[test]
fn test_fuzz_token_soup() {
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
fn test_fuzz_escape_corners() {
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
fn test_sdfa_round_trip_codec_and_continuation_handoff() {
    use dbt_antlr_runtime::atn_simulator::IATNSimulator;

    Arena::with(|arena| {
        let man = gen::fuzzexprparser_static::parser_simulator_manager();
        let sim = man.get_simulator(arena);
        let tables = &sim.atn().static_dfas;
        assert!(!tables.is_empty());

        let words = tables.to_words();
        let decoded = StaticDFATables::from_words(&words).expect("round-trip decode succeeds");
        assert_eq!(&decoded, tables);
        assert_eq!(decoded.to_words(), words);

        // Decision 2 (`item : e 'as' name | e`) is a hybrid table whose depth-4 frontier
        // states carry post-closure SLL ParserContinuation entries.
        let item_table = tables.table(2, 0).expect("item has hybrid table");
        assert!(
            !item_table.continuations.is_empty(),
            "hybrid frontier states carry ParserContinuation entries"
        );

        // Parsing `"a + b.c as y"` crosses the depth-4 frontier at state s28
        // (`a` `+` `b` `.`) of decision 2 and resumes Stage 1 SLL via
        // ParserContinuation without populating s0 in the dynamic ParserDFA.
        let lexer = gen::fuzzexprlexer::FuzzExprLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new("a + b.c as y"),
        );
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = gen::fuzzexprparser_static::FuzzExprParser::new(arena, tokens);
        let tree = parser
            .s()
            .map(|ctx| string_tree(ctx, parser.get_rule_names()))
            .expect("parse succeeds");
        assert_eq!(tree, parse_fuzz_adaptive("a + b.c as y").unwrap());
        let dfa_d2 = sim.decision_to_dfa(2).expect("decision 2 exists");
        assert!(
            dfa_d2.s0().is_none(),
            "frontier escape must resume from ParserContinuation without allocating s0"
        );
        assert!(
            !dfa_d2.is_empty(),
            "continuation state and tail transitions are cached in ParserDFA"
        );
    });
}
