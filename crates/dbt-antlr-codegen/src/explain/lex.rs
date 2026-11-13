// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The tokenization of the `--explain` input.
//!
//! The input is tokenized with the lexer ATN. A static lexer DFA produces the
//! same tokens, so the explanation does not need the lexer tables.

use std::cell::RefCell;
use std::rc::Rc;

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::input_stream::InputStream;
use dbt_antlr_runtime::lexer_interpreter::LexerInterpreter;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token::{TOKEN_EOF, Token};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::vocabulary::{Vocabulary, VocabularyImpl};
use dbt_antlr_runtime::{Arena, TokenSource};

use crate::grammar::atn::CompiledLexer;

pub(super) type ExplainLexer<'input, 'arena> =
    LexerInterpreter<'input, 'arena, InputStream<'input>, CommonTokenFactory<'input, 'arena>>;

/// The grammar-level data of the lexer.
pub(super) struct LexerSpec {
    grammar_file_name: &'static str,
    rule_names: &'static [&'static str],
    channel_names: &'static [&'static str],
    mode_names: &'static [&'static str],
    literal_names: Vec<Option<String>>,
    symbolic_names: Vec<Option<String>>,
    atn: &'static ATN,
}

/// One token of the input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TokenView {
    pub(super) token_type: i32,
    pub(super) text: String,
    pub(super) line: u32,
    pub(super) column: i32,
    pub(super) channel: i32,
}

/// One lexer error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LexError {
    pub(super) line: u32,
    pub(super) column: i32,
    pub(super) message: String,
}

/// The tokens of the input, indexed like the parser's token stream.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct Lexed {
    pub(super) tokens: Vec<TokenView>,
    pub(super) errors: Vec<LexError>,
}

#[derive(Clone, Default)]
struct CollectingListener {
    errors: Rc<RefCell<Vec<LexError>>>,
}

impl<'input, 'arena, R, Tok> ErrorListener<'input, 'arena, R, Tok> for CollectingListener
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
        message: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.errors.borrow_mut().push(LexError {
            line,
            column,
            message: message.to_owned(),
        });
    }
}

impl LexerSpec {
    pub(super) fn new(compiled: &CompiledLexer) -> Self {
        let recognizer = &compiled.semantic.recognizer;
        let channel_names = recognizer
            .channel_names
            .iter()
            .map(|name| name.clone().unwrap_or_default())
            .collect::<Vec<_>>();
        Self {
            grammar_file_name: leak_str(&recognizer.name),
            rule_names: leak_strings(&recognizer.rule_names),
            channel_names: leak_strings(&channel_names),
            mode_names: leak_strings(&recognizer.mode_names),
            literal_names: recognizer.literal_names.clone(),
            symbolic_names: recognizer.symbolic_names.clone(),
            atn: compiled.atn,
        }
    }

    fn vocabulary(&self) -> Box<dyn Vocabulary> {
        Box::new(VocabularyImpl::new(
            self.literal_names.iter(),
            self.symbolic_names.iter(),
            None,
        ))
    }

    /// A lexer over `input` without error listeners.
    pub(super) fn lexer<'input, 'arena>(
        &self,
        arena: &'arena Arena,
        input: &'input str,
    ) -> ExplainLexer<'input, 'arena> {
        let mut lexer = LexerInterpreter::new(
            arena,
            self.grammar_file_name,
            self.vocabulary(),
            self.rule_names,
            self.channel_names,
            self.mode_names,
            self.atn,
            InputStream::new(input),
        )
        .expect("a compiled lexer ATN is a lexer ATN");
        lexer.remove_error_listeners();
        lexer
    }

    /// Tokenizes `input`, with every token on every channel.
    pub(super) fn tokenize(&self, input: &str) -> Lexed {
        Arena::with(|arena| {
            let listener = CollectingListener::default();
            let mut lexer = self.lexer(arena, input);
            lexer.add_error_listener(Box::new(listener.clone()));
            let mut tokens = Vec::new();
            loop {
                let token = lexer.next_token();
                let token_type = token.get_token_type();
                tokens.push(TokenView {
                    token_type,
                    text: token.get_text().to_owned(),
                    line: token.get_line(),
                    column: token.get_char_position_in_line(),
                    channel: token.get_channel(),
                });
                if token_type == TOKEN_EOF {
                    break;
                }
            }
            let errors = listener.errors.borrow().clone();
            Lexed { tokens, errors }
        })
    }
}

pub(super) fn leak_str(text: &str) -> &'static str {
    Box::leak(text.to_owned().into_boxed_str())
}

pub(super) fn leak_strings(names: &[String]) -> &'static [&'static str] {
    let leaked = names.iter().map(|name| leak_str(name)).collect::<Vec<_>>();
    Box::leak(leaked.into_boxed_slice())
}
