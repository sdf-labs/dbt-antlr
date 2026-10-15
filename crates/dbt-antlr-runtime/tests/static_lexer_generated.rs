// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Differential tests for `--static-lexer-dfa` generated lexers (Phase 1.4 of the
//! static-DFA plan): the checked-in `tests/gen/*_static.rs` lexers against
//! their adaptive twins, token for token.
//!
//! Three witness grammars cover the three emission shapes:
//!
//! - `XMLLexer` (modes, a `{true}?` predicate, a custom action): tables with
//!   escape edges, ATN retained.
//! - `SpikeLexer` (a recursive nested-comment rule, a predicated keyword):
//!   tables with escape edges, ATN retained.
//! - `CSV` (no predicates, actions, or recursion): fully static tables, ATN
//!   elided (`ATNSimulatorManager::new_static_lexer`).
//!
//! Token type, text, channel, line, column, start, stop, and the error
//! messages must match exactly. Per-grammar assertions on the escape and
//! restart counters keep the differential honest: the escape grammars must
//! fire escapes, the fully static one must not.

use std::cell::RefCell;
use std::rc::Rc;

use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::input_stream::InputStream;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token::{Token, TOKEN_EOF};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::{Arena, Lexer, TokenSource};

#[rustfmt::skip]
mod gen {
    // Generated lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod csvlexer;
    pub mod csvlexer_static;
    pub mod spikelexer;
    pub mod spikelexer_static;
    pub mod xmllexer;
    pub mod xmllexer_static;
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
    escapes: usize,
    restarts: usize,
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

/// Drains one lexer over `text`: the full token stream, the error listener
/// messages, and the static walker's escape/restart counters (0 for the
/// adaptive side, which has no tables). `$make` builds the lexer wrapper
/// (`XMLLexer::new` and friends) from the arena and the input stream.
macro_rules! run_lexer {
    ($text:expr, $make:expr) => {{
        let text: &'static str = Box::leak(format!("{}", $text).into_boxed_str());
        Arena::with(|arena| {
            let mut lexer = $make(arena, InputStream::new(text));
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
            let escapes = lexer.get_interpreter().expect("interpreter").escape_count;
            let restarts = lexer.get_interpreter().expect("interpreter").restart_count;
            let errors = messages.borrow().clone();
            let snap = Snap {
                tokens,
                errors,
                escapes,
                restarts,
            };
            snap
        })
    }};
}

const XML_CORPUS: &[&str] = &[
    "<?xml version=\"1.0\"?>",
    "<!--comment-->",
    "<!-- multi\nline\ncomment -->",
    "<!--a--><!--b-->",
    "<!-- <!-- -->",
    "<!-- unterminated",
    "<!--",
    "<![CDATA[function f(x) { if (x < x) duh } ]]>",
    "<![CDATA[]]>",
    "<!DOCTYPE html>",
    "<!ENTITY foo 'bar'>",
    "<a href='x'>text</a>",
    "<a b=\"c\" d='e'>x y</a>",
    "<a/>",
    "<a />",
    "<ä>unicode</ä>",
    "&amp; &#65; &#x41;",
    "<?php echo 'x'; ?>",
    "<script>\n<![CDATA[\nif (a > 0) then\n]]>\n</script>",
    "plain text only",
    "<a>mixed <b>nested</b> tags</a>",
    "<",
    "&",
    "<a",
    ">",
    "<a b=c>", // unquoted attribute value: error inside the tag
    "text & more text",
    "\r\n<a>\r\n</a>\r\n",
    "<a\tb = \"c\" >x</a >",
];

const SPIKE_CORPUS: &[&str] = &[
    "let x = 1",
    "let=5",
    "let =",
    "let",
    "letlet",
    "let0",
    "/* comment */",
    "/* outer /* inner */ tail */",
    "/*/*/*/**/*/",
    "/* unterminated",
    "/*",
    "/*/",
    "let/* c */x",
    "/*a*/let=let/*b*/",
    "123 abc let= x y",
    "1.5",
    "let\t\r\n42",
    "@",
    "/* a /* b /* c */ d */ e */ zz",
    "let==x",
];

const CSV_CORPUS: &[&str] = &[
    "a,b,c\n1,2,3\n",
    "name,\"quoted \"\"escaped\"\"\",tail\n",
    "x,,y\n",
    "\n",
    "\"only\"\n",
    "field with spaces,next\n",
    "\"unterminated",
    "a\r\nb\r\n",
    ",,\n,,\n",
    "trailing,comma,",
];

fn assert_same(adaptive: &Snap, r#static: &Snap, grammar: &str, input: &str) {
    assert_eq!(
        adaptive.tokens, r#static.tokens,
        "{grammar}: token streams differ on {input:?}"
    );
    assert_eq!(
        adaptive.errors, r#static.errors,
        "{grammar}: error streams differ on {input:?}"
    );
}

#[test]
fn xml_lexer_static_matches_adaptive() {
    let mut total_escapes = 0;
    for input in XML_CORPUS {
        let adaptive = run_lexer!(input, |arena, stream| {
            gen::xmllexer::XMLLexer::<_, CommonTokenFactory>::new(arena, stream)
        });
        let r#static = run_lexer!(input, |arena, stream| {
            gen::xmllexer_static::XMLLexer::<_, CommonTokenFactory>::new(arena, stream)
        });
        assert_same(&adaptive, &r#static, "XMLLexer", input);
        total_escapes += r#static.escapes;
    }
    assert!(
        total_escapes > 0,
        "the predicate and custom-action escapes of XMLLexer must fire"
    );
}

#[test]
fn spike_lexer_static_matches_adaptive() {
    let mut total_escapes = 0;
    for input in SPIKE_CORPUS {
        let adaptive = run_lexer!(input, |arena, stream| {
            gen::spikelexer::SpikeLexer::<_, CommonTokenFactory>::new(arena, stream)
        });
        let r#static = run_lexer!(input, |arena, stream| {
            gen::spikelexer_static::SpikeLexer::<_, CommonTokenFactory>::new(arena, stream)
        });
        assert_same(&adaptive, &r#static, "SpikeLexer", input);
        total_escapes += r#static.escapes;
    }
    assert!(
        total_escapes > 0,
        "the predicate and recursion escapes of SpikeLexer must fire"
    );
}

#[test]
fn csv_lexer_static_get_dfa_string_is_empty() {
    // The fully static lexer is ATN-elided: its decision-to-DFA vec is
    // empty, and rendering the DFA must not panic.
    Arena::with(|arena| {
        let lexer = gen::csvlexer_static::CSVLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new("a,b\n"),
        );
        assert_eq!(lexer.get_dfa_string(), "");
    });
}

#[test]
fn csv_lexer_static_matches_adaptive() {
    for input in CSV_CORPUS {
        let adaptive = run_lexer!(input, |arena, stream| {
            gen::csvlexer::CSVLexer::<_, CommonTokenFactory>::new(arena, stream)
        });
        let r#static = run_lexer!(input, |arena, stream| {
            gen::csvlexer_static::CSVLexer::<_, CommonTokenFactory>::new(arena, stream)
        });
        assert_same(&adaptive, &r#static, "CSV", input);
        assert_eq!(
            r#static.escapes, 0,
            "CSV is fully static: no escapes on {input:?}"
        );
        assert_eq!(r#static.restarts, 0, "CSV is fully static: no restarts");
    }
}
