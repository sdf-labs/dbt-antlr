// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Static parser tables against the adaptive parser where the static analysis
//! must know who consumes the tokens after a decision rule returns.
//!
//! `LoopSibling.g4` continues after the exit of an operator loop with tokens
//! that the loop does not own: a new invocation of the rule reads the next
//! `'-'`, or an enclosing alternative still owes it. The tables of the loop
//! must leave the iterate/exit conflict to adaptive prediction. `LoopSubst.g4`
//! is the conflict that the tables do decide: an operator after a lambda body,
//! which an enclosing invocation of the loop can also consume.
//!
//! `PostfixSibling.g4` does the same after a skipped optional postfix: a new
//! invocation of the decision rule reads the tokens that the postfix would
//! take, so the take must defer on the stacks where that invocation follows.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod loopsiblinglexer;
    pub mod loopsiblingparser;
    pub mod loopsiblingparser_static;
    pub mod loopsubstlexer;
    pub mod loopsubstparser;
    pub mod loopsubstparser_static;
    pub mod postfixsiblinglexer;
    pub mod postfixsiblingparser;
    pub mod postfixsiblingparser_static;
}

use std::cell::RefCell;
use std::rc::Rc;

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::Parser;
use dbt_antlr_runtime::{Arena, InputStream};

/// Collects `syntax_error` notifications.
#[derive(Clone, Default)]
struct CollectingErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
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

/// The tree and the syntax errors of one parse.
#[derive(Debug, PartialEq)]
struct Outcome {
    tree: Result<String, String>,
    errors: Vec<String>,
}

impl Outcome {
    fn is_valid(&self) -> bool {
        self.tree.is_ok() && self.errors.is_empty()
    }
}

macro_rules! parse_fn {
    ($name:ident, $lexer:ty, $parser:ty, $rule:ident) => {
        fn $name(input: &str) -> Outcome {
            Arena::with(|arena| {
                let lexer = <$lexer>::new(arena, InputStream::new(input));
                let mut parser = <$parser>::new(arena, CommonTokenStream::new(lexer));
                let listener = CollectingErrorListener::default();
                let messages = listener.messages.clone();
                parser.remove_error_listeners();
                parser.add_error_listener(Box::new(listener));
                let tree = parser
                    .$rule()
                    .map(|tree| string_tree(tree, parser.get_rule_names()))
                    .map_err(|error| format!("{error:?}"));
                let errors = messages.borrow().clone();
                Outcome { tree, errors }
            })
        }
    };
}

parse_fn!(
    sibling_adaptive,
    gen::loopsiblinglexer::LoopSiblingLexer<'_, '_, _, CommonTokenFactory>,
    gen::loopsiblingparser::LoopSiblingParser<'_, '_, _, _>,
    start
);
parse_fn!(
    sibling_static,
    gen::loopsiblinglexer::LoopSiblingLexer<'_, '_, _, CommonTokenFactory>,
    gen::loopsiblingparser_static::LoopSiblingParser<'_, '_, _, _>,
    start
);
parse_fn!(
    subst_adaptive,
    gen::loopsubstlexer::LoopSubstLexer<'_, '_, _, CommonTokenFactory>,
    gen::loopsubstparser::LoopSubstParser<'_, '_, _, _>,
    start
);
parse_fn!(
    subst_static,
    gen::loopsubstlexer::LoopSubstLexer<'_, '_, _, CommonTokenFactory>,
    gen::loopsubstparser_static::LoopSubstParser<'_, '_, _, _>,
    start
);

parse_fn!(
    postfix_adaptive,
    gen::postfixsiblinglexer::PostfixSiblingLexer<'_, '_, _, CommonTokenFactory>,
    gen::postfixsiblingparser::PostfixSiblingParser<'_, '_, _, _>,
    start
);
parse_fn!(
    postfix_static,
    gen::postfixsiblinglexer::PostfixSiblingLexer<'_, '_, _, CommonTokenFactory>,
    gen::postfixsiblingparser_static::PostfixSiblingParser<'_, '_, _, _>,
    start
);

/// Every sentence of `tokens` with `1..=max_len` tokens.
fn sentences(tokens: &[&str], max_len: usize) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Vec<Vec<&str>> = vec![Vec::new()];
    for _ in 0..max_len {
        let mut next = Vec::new();
        for prefix in &current {
            for token in tokens {
                let mut sentence = prefix.clone();
                sentence.push(token);
                out.push(sentence.join(" "));
                next.push(sentence);
            }
        }
        current = next;
    }
    out
}

#[test]
fn new_invocations_after_the_exit_parse_like_the_adaptive_parser() {
    let cases = [
        ("a - - a ;", "(start (expr a) (expr - (expr - (expr a))) ; <EOF>)"),
        (
            "- a - - a ;",
            "(start (expr - (expr a)) (expr - (expr - (expr a))) ; <EOF>)",
        ),
        (
            "a - - a - - ;",
            "(start (expr (expr a) - (expr - (expr a))) - - ; <EOF>)",
        ),
        (
            "with pair a - pair - a a",
            "(start with (term pair (term a) (term - (term pair (term - (term a)) (term a)))) <EOF>)",
        ),
        (
            "if a ? a - a",
            "(start if (cond (cond a) ? (cond a) - a) <EOF>)",
        ),
        (
            "if a ? a - a - a",
            "(start if (cond (cond a) ? (cond (cond a) - (cond a)) - a) <EOF>)",
        ),
        (
            "opt a ? a - a - a !",
            "(start opt (tail (tail a) ? (tail a) - a - a !) <EOF>)",
        ),
        (
            "prec a * a + a : a",
            "(start prec (prec (prec (prec a) * (prec a)) + (prec a) : a) <EOF>)",
        ),
    ];
    for (input, expected) in cases {
        let adaptive = sibling_adaptive(input);
        assert_eq!(adaptive.tree.as_deref(), Ok(expected), "{input:?}");
        assert!(adaptive.errors.is_empty(), "{input:?}: {adaptive:?}");
        assert_eq!(sibling_static(input), adaptive, "{input:?}");
    }
}

/// Every short sentence over the tokens of `LoopSibling.g4`: the static parser
/// accepts exactly the sentences that the adaptive parser accepts, with the
/// same trees.
#[test]
fn short_sentences_parse_like_the_adaptive_parser() {
    let tokens = [";", "-", "!", "with", "pair", "a"];
    let mut valid = 0;
    for input in sentences(&tokens, 6) {
        let adaptive = sibling_adaptive(&input);
        let parsed = sibling_static(&input);
        assert_eq!(parsed.is_valid(), adaptive.is_valid(), "{input:?}");
        if adaptive.is_valid() {
            valid += 1;
            assert_eq!(parsed, adaptive, "{input:?}");
        }
    }
    assert!(valid > 0, "the enumeration reaches the valid sentences");

    let mut valid = 0;
    let bodies = sentences(&["-", "?", "a"], 7)
        .into_iter()
        .map(|body| format!("if {body}"))
        .chain(
            sentences(&["-", "?", "!", "a"], 7)
                .into_iter()
                .map(|body| format!("opt {body}")),
        )
        .chain(
            sentences(&["+", "*", ":", "a"], 7)
                .into_iter()
                .map(|body| format!("prec {body}")),
        );
    for input in bodies {
        let adaptive = sibling_adaptive(&input);
        let parsed = sibling_static(&input);
        assert_eq!(parsed.is_valid(), adaptive.is_valid(), "{input:?}");
        if adaptive.is_valid() {
            valid += 1;
            assert_eq!(parsed, adaptive, "{input:?}");
        }
    }
    assert!(valid > 0, "the enumeration reaches the valid sentences");
}

#[test]
fn operators_after_a_lambda_body_extend_the_body() {
    let cases = [
        "x -> a + b",
        "x -> a * b + c",
        "x -> y -> a - b",
        "( x -> a ) + b",
        "f ( x -> a + b , c ) * d",
        "- x -> a || b << c",
    ];
    for input in cases {
        let adaptive = subst_adaptive(input);
        assert!(adaptive.is_valid(), "{input:?}: {adaptive:?}");
        assert_eq!(subst_static(input), adaptive, "{input:?}");
    }
    let tree = subst_static("x -> a + b").tree.expect("parses");
    assert!(
        tree.contains("-> (expr (expr (operand a)) + (expr (operand b)))"),
        "the body takes the operator: {tree}"
    );
}

#[test]
fn new_invocations_after_the_skip_parse_like_the_adaptive_parser() {
    let cases = [
        ("A - B ;", "(start (r A) (r - B) ; <EOF>)"),
        ("A - B - B ;", "(start (r A (y - B)) (r - B) ; <EOF>)"),
        ("( A - B - B )", "(start ( (r A (y - B)) (r - B) ) <EOF>)"),
        (
            "with [ A - B ] X",
            "(start with (q [ (q A) (q - B) ] X) <EOF>)",
        ),
        (
            "with [ A - B A ] X",
            "(start with (q [ (q A (z - B)) (q A) ] X) <EOF>)",
        ),
        ("cont A - B C !", "(start cont (c A (c - B) C !) <EOF>)"),
        ("cont A - B", "(start cont (c A (w - B)) <EOF>)"),
        (
            "mid A - B A - B A - B !",
            "(start mid (m A (v - B) (m A) - B (m A (v - B)) !) <EOF>)",
        ),
        (
            "mid A - B A - B !",
            "(start mid (m A (v - B) (m A) - B !) <EOF>)",
        ),
    ];
    for (input, expected) in cases {
        let adaptive = postfix_adaptive(input);
        assert_eq!(adaptive.tree.as_deref(), Ok(expected), "{input:?}");
        assert!(adaptive.errors.is_empty(), "{input:?}: {adaptive:?}");
        assert_eq!(postfix_static(input), adaptive, "{input:?}");
    }
}

/// Every short sentence over the tokens of `r`, and every `with`, `cont` and
/// `mid` sentence over the tokens of `q`, `c` and `m`: the static parser
/// accepts exactly the sentences that the adaptive parser accepts, with the
/// same trees.
#[test]
fn short_postfix_sentences_parse_like_the_adaptive_parser() {
    let mut inputs = sentences(&[";", "(", ")", "A", "-", "B"], 5);
    for (keyword, tokens, len) in [
        ("with", &["[", "]", "X", "A", "-", "B"][..], 5),
        ("cont", &["A", "-", "B", "C", "!"][..], 6),
        ("mid", &["A", "-", "B", "!"][..], 7),
    ] {
        inputs.extend(
            sentences(tokens, len)
                .into_iter()
                .map(|body| format!("{keyword} {body}")),
        );
    }
    let mut valid = 0;
    for input in inputs {
        let adaptive = postfix_adaptive(&input);
        let parsed = postfix_static(&input);
        assert_eq!(parsed.is_valid(), adaptive.is_valid(), "{input:?}");
        if adaptive.is_valid() {
            valid += 1;
            assert_eq!(parsed, adaptive, "{input:?}");
        }
    }
    assert!(valid > 0, "the enumeration reaches the valid sentences");
}
