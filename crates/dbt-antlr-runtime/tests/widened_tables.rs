// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Static parser tables against the adaptive parser where the classifier
//! falls back to the widened attempt.
//!
//! The widened attempt gives every rule call an empty context. Two configs
//! can then meet at the same state with the same context, but have different
//! real stacks. `WidenedTrust.g4` is a conflict that only widening makes: the
//! table must not resolve it to the lowest alternative. `WidenedPrecedence.g4`
//! is a precedence-class start state: the precedence filter must not drop the
//! exit config that widening made equal to an iterate config.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod widenedprecedencelexer;
    pub mod widenedprecedenceparser;
    pub mod widenedprecedenceparser_static;
    pub mod widenedtrustlexer;
    pub mod widenedtrustparser;
    pub mod widenedtrustparser_static;
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
    trust_adaptive,
    gen::widenedtrustlexer::WidenedTrustLexer<'_, '_, _, CommonTokenFactory>,
    gen::widenedtrustparser::WidenedTrustParser<'_, '_, _, _>,
    top
);
parse_fn!(
    trust_static,
    gen::widenedtrustlexer::WidenedTrustLexer<'_, '_, _, CommonTokenFactory>,
    gen::widenedtrustparser_static::WidenedTrustParser<'_, '_, _, _>,
    top
);
parse_fn!(
    precedence_adaptive,
    gen::widenedprecedencelexer::WidenedPrecedenceLexer<'_, '_, _, CommonTokenFactory>,
    gen::widenedprecedenceparser::WidenedPrecedenceParser<'_, '_, _, _>,
    s
);
parse_fn!(
    precedence_static,
    gen::widenedprecedencelexer::WidenedPrecedenceLexer<'_, '_, _, CommonTokenFactory>,
    gen::widenedprecedenceparser_static::WidenedPrecedenceParser<'_, '_, _, _>,
    s
);

/// After `c b b b b a`, only the nested `a` of `B B B B a E` can continue with
/// `d e`. The widened attempt sees a conflict there, and the static table must
/// not resolve it to `B B B B A`.
#[test]
fn a_conflict_that_only_widening_makes_is_not_resolved() {
    for input in [
        "c b b b b a d e d x",
        "c b b b b a d y",
        "c b b b b c b b b b a d e d e d x",
        "a d x",
        "c z q q p q q q q p q q w n d y",
    ] {
        let adaptive = trust_adaptive(input);
        assert!(adaptive.is_valid(), "{input}: {adaptive:?}");
        assert_eq!(trust_static(input), adaptive, "{input}");
    }
}

/// In `x*x+qr`, the right operand of `*` exits, and the outer loop reads
/// `+ q r` through `e op`. The static table of that operand's precedence class
/// must keep the exit reading.
#[test]
fn the_precedence_filter_keeps_a_widened_exit() {
    for input in [
        "x*x+qr",
        "x*x+qra",
        "x*x+qr+qr",
        "x*x+qra*x+qr",
        "x+qr*x+qr",
        "x u u u u u u u u u u u end t10a t9a t8a t7a t6a t5a t4a t3a t2a t1a t0a m1",
    ] {
        let adaptive = precedence_adaptive(input);
        assert!(adaptive.is_valid(), "{input}: {adaptive:?}");
        assert_eq!(precedence_static(input), adaptive, "{input}");
    }
}
