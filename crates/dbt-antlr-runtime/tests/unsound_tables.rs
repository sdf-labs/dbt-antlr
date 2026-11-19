// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Static parser tables against the adaptive parser on the witness of the
//! entry-context attestation.
//!
//! A config that reached a conflict on the entry stack of the decision has an
//! empty context. That empty context is not a wildcard: it continues with the
//! caller of the decision. `UnsoundSimple.g4` is a conflict between a config
//! on the entry stack and one inside the nested `a`, which still owes `E`:
//! only the nested alternative can read `c b p a d e d x`. The table must not
//! resolve the conflict to the alternative of the entry stack.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod unsoundsimplelexer;
    pub mod unsoundsimpleparser;
    pub mod unsoundsimpleparser_static;
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
    adaptive,
    gen::unsoundsimplelexer::UnsoundSimpleLexer<'_, '_, _, CommonTokenFactory>,
    gen::unsoundsimpleparser::UnsoundSimpleParser<'_, '_, _, _>,
    top
);
parse_fn!(
    static_dfa,
    gen::unsoundsimplelexer::UnsoundSimpleLexer<'_, '_, _, CommonTokenFactory>,
    gen::unsoundsimpleparser_static::UnsoundSimpleParser<'_, '_, _, _>,
    top
);

/// After `c b p a`, the alternatives `B (P | Q) A` and `B P A` are on the
/// entry stack of decision 1, and `B P a E` is inside the nested `a`, which
/// still owes `e`. The static table must not resolve that conflict to an
/// alternative of the entry stack.
#[test]
fn an_entry_context_does_not_choose_the_shorter_alternative() {
    for input in [
        "c b p a d e d x",
        "c b p a d e d y",
        "c b p c b p a d e d e d x",
        "c b p a d e m d x",
        "a d x",
        "c b q a d x",
        "c b p a d x",
    ] {
        let adaptive = adaptive(input);
        assert!(adaptive.is_valid(), "{input}: {adaptive:?}");
        assert_eq!(static_dfa(input), adaptive, "{input}");
    }
}
