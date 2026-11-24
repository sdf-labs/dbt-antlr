// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Differential tests for deferred error synchronization under `--static-dfa-only`.
//!
//! Compares parse trees and syntax error messages between the adaptive parser
//! and the `--static-dfa-only` parser for bounded-loop resynchronization,
//! subrule-entry single-token deletion, skipped-optional expected sets, and
//! `BailErrorStrategy`.

#[rustfmt::skip]
mod gen {
    #![allow(clippy::all, warnings)]
    pub mod deferredsynclexer;
    pub mod deferredsyncparser;
    pub mod deferredsyncparser_static_only;
    pub mod guardexprlexer;
    pub mod guardexprparser;
    pub mod guardexprparser_static_only;
    pub mod parenexprlexer;
    pub mod parenexprparser;
    pub mod parenexprparser_static_only;
}

use std::cell::RefCell;
use std::rc::Rc;

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::error_strategy::BailErrorStrategy;
use dbt_antlr_runtime::errors::{ANTLRError, ANTLRErrorKind};
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token::Token;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::{Arena, InputStream, Parser};

#[derive(Clone, Default)]
struct CollectingErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'input, 'arena, R, Tok> ErrorListener<'input, 'arena, R, Tok> for CollectingErrorListener
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: Token + 'input,
{
    fn syntax_error(
        &self,
        _recognizer: &R,
        _offending_symbol: Option<&dyn Token>,
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

macro_rules! parse_deferred_with {
    ($parser:ident, $rule:ident, $input:expr) => {
        Arena::with(|arena| {
            let lexer = gen::deferredsynclexer::DeferredSyncLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new($input),
            );
            let tokens = CommonTokenStream::new(lexer);
            let mut parser = gen::$parser::DeferredSyncParser::new(arena, tokens);
            let listener = CollectingErrorListener::default();
            let messages = listener.messages.clone();
            parser.remove_error_listeners();
            parser.add_error_listener(Box::new(listener));
            let tree = match parser.$rule() {
                Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                Err(error) => format!("ERR {error}"),
            };
            let messages = messages.borrow().clone();
            (tree, messages)
        })
    };
}

macro_rules! assert_deferred_eq {
    ($rule:ident, $inputs:expr) => {
        for input in $inputs {
            let adaptive = parse_deferred_with!(deferredsyncparser, $rule, input);
            let static_only = parse_deferred_with!(deferredsyncparser_static_only, $rule, input);
            assert_eq!(
                static_only,
                adaptive,
                "{}({input:?}): static-dfa-only must match adaptive",
                stringify!($rule)
            );
        }
    };
}

#[test]
fn bounded_ll1_star_loop_matches_adaptive_recovery() {
    assert_deferred_eq!(
        ll1Star,
        [
            "( )",
            "( foo bar )",
            "( foo + bar )",
            "( + foo )",
            "( + )",
            "( + + )",
            "( foo + + bar )",
            "( foo + bar + baz )",
            "( foo +",
        ]
    );
}

#[test]
fn bounded_ll1_plus_loop_matches_adaptive_recovery() {
    assert_deferred_eq!(
        ll1Plus,
        [
            "( foo )",
            "( foo bar )",
            "( foo + bar )",
            "( + foo )",
            "( + + )",
            "( foo + )",
            "( foo + + bar )",
            "( foo + bar + baz )",
            "( foo +",
        ]
    );
}

#[test]
fn rule_trailing_ll1_loops_match_adaptive_recovery() {
    assert_deferred_eq!(trailStar, ["", "foo bar", "+ foo", "foo + bar"]);
    assert_deferred_eq!(trailPlus, ["foo", "foo bar", "+ foo", "+ +", "foo + bar"]);
}

#[test]
fn bounded_dfa_star_loop_matches_adaptive_recovery() {
    assert_deferred_eq!(
        dfaStar,
        [
            "( a c )",
            "( a b a c )",
            "( a b + a b a c )",
            "( a b + a c )",
            "( + a b a c )",
            "( + + a c )",
            "( a b a + )",
            "( a b +",
        ]
    );
}

#[test]
fn bounded_dfa_plus_loop_matches_adaptive_recovery() {
    assert_deferred_eq!(
        dfaPlus,
        [
            "( a b a c )",
            "( a b a b a c )",
            "( a b + a b a c )",
            "( a b + a c )",
            "( + a b a c )",
            "( + + a c )",
            "( a b a + )",
            "( a b +",
        ]
    );
}

#[test]
fn non_nullable_alt_blocks_match_adaptive_recovery() {
    assert_deferred_eq!(
        ll1Alt,
        ["a foo ;", "b foo ;", "+ a foo ;", "+ b foo ;", "+ + ;"]
    );
    assert_deferred_eq!(
        dfaAlt,
        ["a b ;", "a c ;", "d e ;", "+ a b ;", "+ d e ;", "+ + ;", "a + ;", "d + ;",]
    );
    assert_deferred_eq!(
        dfaOpt,
        [
            "( a c )",
            "( a b a c )",
            "( + a b a c )",
            "( + a c )",
            "( + + )",
            "( a + )",
        ]
    );
}

#[test]
fn skipped_optional_expected_sets_match_adaptive() {
    assert_deferred_eq!(termOptTop, ["a ;", "a b ;", "a +", "a b +"]);
    assert_deferred_eq!(
        nestedOptTop,
        ["a ;", "a c ;", "a b ;", "a c b ;", "a +", "a c +", "a b +"]
    );
    assert_deferred_eq!(
        zeroSibTop,
        ["a ;", "a b ;", "a c ;", "a b c ;", "a +", "a b +", "a c +"]
    );
    assert_deferred_eq!(startEmptyTop, [";", "c ;", "+"]);
    assert_deferred_eq!(nonAdjTop, ["a d ;", "a b d ;", "a b d +"]);
    assert_deferred_eq!(intraOpt, ["a ;", "a b ;", "a +", "a b +"]);

    let (tree, errs) = parse_deferred_with!(deferredsyncparser_static_only, termOptTop, "a +");
    assert_eq!(
        errs,
        ["line 1:2 mismatched input '+' expecting {'b', ';'}"],
        "{tree}"
    );

    let (_, errs) = parse_deferred_with!(deferredsyncparser_static_only, nestedOptTop, "a +");
    assert_eq!(
        errs,
        ["line 1:2 mismatched input '+' expecting {'b', 'c', ';'}"]
    );

    let (_, errs) = parse_deferred_with!(deferredsyncparser_static_only, zeroSibTop, "a +");
    assert_eq!(
        errs,
        ["line 1:2 mismatched input '+' expecting {'b', 'c', ';'}"]
    );

    let (_, errs) = parse_deferred_with!(deferredsyncparser_static_only, startEmptyTop, "+");
    assert_eq!(errs, ["line 1:0 mismatched input '+' expecting {'c', ';'}"]);

    let (_, errs) = parse_deferred_with!(deferredsyncparser_static_only, nonAdjTop, "a d +");
    assert_eq!(errs, ["line 1:4 mismatched input '+' expecting ';'"]);

    let (_, errs) = parse_deferred_with!(deferredsyncparser_static_only, ambigTermTop, "d a +");
    assert_eq!(errs, ["line 1:4 mismatched input '+' expecting ';'"]);

    assert_deferred_eq!(samePostTermTop, ["d a ;", "e a b ;", "d a +", "e a +"]);
    let (_, errs) = parse_deferred_with!(deferredsyncparser_static_only, samePostTermTop, "d a +");
    assert_eq!(errs, ["line 1:4 mismatched input '+' expecting {'b', ';'}"]);

    assert_deferred_eq!(conjuredTermTop, ["a +", "+ d a +"]);
}

#[test]
fn guard_expr_skipped_optional_and_loop_expected_sets_match_adaptive() {
    for input in ["set a = b + ;", "set a = b AND c + ;", "a ; + b ;", "+ a ;"] {
        let parse_one = |static_only: bool| {
            Arena::with(|arena| {
                let lexer = gen::guardexprlexer::GuardExprLexer::<_, CommonTokenFactory>::new(
                    arena,
                    InputStream::new(input),
                );
                let tokens = CommonTokenStream::new(lexer);
                let listener = CollectingErrorListener::default();
                let messages = listener.messages.clone();
                let tree = if static_only {
                    let mut parser =
                        gen::guardexprparser_static_only::GuardExprParser::new(arena, tokens);
                    parser.remove_error_listeners();
                    parser.add_error_listener(Box::new(listener));
                    match parser.prog() {
                        Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                        Err(e) => format!("ERR {e}"),
                    }
                } else {
                    let mut parser = gen::guardexprparser::GuardExprParser::new(arena, tokens);
                    parser.remove_error_listeners();
                    parser.add_error_listener(Box::new(listener));
                    match parser.prog() {
                        Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                        Err(e) => format!("ERR {e}"),
                    }
                };
                let messages = messages.borrow().clone();
                (tree, messages)
            })
        };
        assert_eq!(parse_one(true), parse_one(false), "input: {input:?}");
    }
}

#[test]
fn descent_alt_block_single_token_deletion_matches_adaptive() {
    for input in [", a", ", ( a , b )", ", ,", "( a , b * , c )"] {
        let parse_one = |static_only: bool| {
            Arena::with(|arena| {
                let lexer = gen::parenexprlexer::ParenExprLexer::<_, CommonTokenFactory>::new(
                    arena,
                    InputStream::new(input),
                );
                let tokens = CommonTokenStream::new(lexer);
                let listener = CollectingErrorListener::default();
                let messages = listener.messages.clone();
                let tree = if static_only {
                    let mut parser =
                        gen::parenexprparser_static_only::ParenExprParser::new(arena, tokens);
                    parser.remove_error_listeners();
                    parser.add_error_listener(Box::new(listener));
                    match parser.s() {
                        Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                        Err(e) => format!("ERR {e}"),
                    }
                } else {
                    let mut parser = gen::parenexprparser::ParenExprParser::new(arena, tokens);
                    parser.remove_error_listeners();
                    parser.add_error_listener(Box::new(listener));
                    match parser.s() {
                        Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                        Err(e) => format!("ERR {e}"),
                    }
                };
                let messages = messages.borrow().clone();
                (tree, messages)
            })
        };
        assert_eq!(parse_one(true), parse_one(false), "input: {input:?}");
    }
}

#[test]
fn bail_error_strategy_cancels_without_deferred_sync_recovery() {
    macro_rules! assert_bails {
        ($rule:ident, $input:expr) => {
            Arena::with(|arena| {
                let lexer = gen::deferredsynclexer::DeferredSyncLexer::<_, CommonTokenFactory>::new(
                    arena,
                    InputStream::new($input),
                );
                let tokens = CommonTokenStream::new(lexer);
                let mut parser =
                    gen::deferredsyncparser_static_only::DeferredSyncParser::with_strategy(
                        arena,
                        tokens,
                        Box::new(BailErrorStrategy::new()),
                    );
                parser.remove_error_listeners();
                let err = parser
                    .$rule()
                    .expect_err("BailErrorStrategy must cancel without recovery");
                assert!(
                    matches!(&*err, ANTLRErrorKind::FallThrough(_)),
                    "{}({:?}): expected FallThrough, got {err:?}",
                    stringify!($rule),
                    $input
                );
            });
        };
    }

    assert_bails!(ll1Alt, "+ a foo ;");
    assert_bails!(dfaAlt, "+ a b ;");
    assert_bails!(ll1Star, "( foo + bar )");
    assert_bails!(ll1Plus, "( + foo )");
    assert_bails!(dfaStar, "( a b + a c )");
    assert_bails!(dfaPlus, "( + a b a c )");
}
