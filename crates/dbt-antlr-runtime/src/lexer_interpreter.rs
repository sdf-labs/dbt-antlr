//! A lexer simulator that mimics what ANTLR's generated lexer code does.
//!
//! Rust port of Java's `org.antlr.v4.runtime.LexerInterpreter`: it drives a
//! deserialized lexer ATN through the standard [BaseLexer] framework and
//! [crate::lexer_atn_simulator::LexerATNSimulator].
use std::ops::{Deref, DerefMut};

use std::fmt::{Debug, Formatter};

use crate::arena::Arena;
use crate::atn::ATN;
use crate::atn_simulator::LexerATNSimulatorManager;
use crate::atn_type::ATNType;
use crate::char_stream::CharStream;
use crate::errors::ANTLRError;
use crate::lexer::{BaseLexer, LexerRecog};
use crate::recognizer::Actions;
use crate::token_factory::TokenFactory;
use crate::vocabulary::Vocabulary;

/// Base lexer type the lexer interpreter is built on.
pub type LexerInterpreterBase<'input, 'arena, Input, TF> =
    BaseLexer<'input, 'arena, LexerInterpreterExt, Input, TF>;

/// Recognizer data of a [LexerInterpreter].
///
/// Unlike generated lexers, which compile this data into statics, the
/// interpreter receives it at construction time. The ATN and the ATN
/// simulator manager must live for `'static` because of the runtime's
/// simulator manager design, so they are leaked on construction; each
/// interpreter instance gets its own (not shared) decision DFAs, matching the
/// Java runtime.
pub struct LexerInterpreterExt {
    grammar_file_name: &'static str,
    rule_names: &'static [&'static str],
    channel_names: &'static [&'static str],
    mode_names: &'static [&'static str],
    vocabulary: Box<dyn Vocabulary>,
    atn_manager: &'static LexerATNSimulatorManager,
}

impl Debug for LexerInterpreterExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LexerInterpreterExt")
            .field("grammar_file_name", &self.grammar_file_name)
            .field("rule_names", &self.rule_names)
            .field("channel_names", &self.channel_names)
            .field("mode_names", &self.mode_names)
            .field("vocabulary", &self.vocabulary)
            .finish_non_exhaustive()
    }
}

impl<'input, 'arena, Input, TF>
    Actions<'input, 'arena, LexerInterpreterBase<'input, 'arena, Input, TF>, TF::Tok>
    for LexerInterpreterExt
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    // `sempred` and `action` intentionally keep the default implementations
    // (return true / do nothing): the interpreter cannot run target code,
    // exactly like Java's LexerInterpreter which inherits the no-op
    // `Recognizer.sempred`/`Recognizer.action`.

    fn get_rule_names(&self) -> &[&str] {
        self.rule_names
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &*self.vocabulary
    }

    fn get_grammar_file_name(&self) -> &str {
        self.grammar_file_name
    }
}

impl<'input, 'arena, Input, TF>
    LexerRecog<'input, 'arena, TF, LexerInterpreterBase<'input, 'arena, Input, TF>>
    for LexerInterpreterExt
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] {
        self.rule_names
    }

    fn get_literal_names(&self) -> &[Option<&str>] {
        &[]
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &[]
    }

    fn get_grammar_file_name(&self) -> &'static str {
        self.grammar_file_name
    }

    fn get_atn_simulator_man(&self) -> &'static LexerATNSimulatorManager {
        self.atn_manager
    }
}

/// A lexer that interprets a deserialized ATN at runtime instead of executing
/// generated code.
///
/// See Java's `LexerInterpreter` for the reference semantics.
pub struct LexerInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    base: LexerInterpreterBase<'input, 'arena, Input, TF>,
}

impl<'input, 'arena, Input, TF> Debug for LexerInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LexerInterpreter").finish_non_exhaustive()
    }
}
impl<'input, 'arena, Input, TF> LexerInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    /// Creates a new lexer interpreter.
    ///
    /// `atn` is a ready (already deserialized) lexer ATN; it is moved onto
    /// the heap and leaked because the runtime's ATN simulator machinery
    /// requires `&'static ATN`. Metadata (`grammar_file_name`, `rule_names`,
    /// `channel_names`, `mode_names`) must be `&'static` for the same reason;
    /// leak dynamically built strings with [Box::leak] if needed.
    ///
    /// Returns an error if `atn` is not a lexer ATN, matching the
    /// `IllegalArgumentException` thrown by the Java constructor.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        arena: &'arena Arena,
        grammar_file_name: &'static str,
        vocabulary: Box<dyn Vocabulary>,
        rule_names: &'static [&'static str],
        channel_names: &'static [&'static str],
        mode_names: &'static [&'static str],
        atn: ATN,
        input: Input,
    ) -> Result<Self, ANTLRError> {
        if atn.grammar_type != ATNType::Lexer {
            return Err(ANTLRError::illegal_state(
                "The ATN must be a lexer ATN.".to_owned(),
            ));
        }
        let atn: &'static ATN = Box::leak(Box::new(atn));
        let atn_manager: &'static LexerATNSimulatorManager =
            Box::leak(Box::new(LexerATNSimulatorManager::new(atn)));
        Ok(Self {
            base: BaseLexer::new_base_lexer(
                input,
                LexerInterpreterExt {
                    grammar_file_name,
                    rule_names,
                    channel_names,
                    mode_names,
                    vocabulary,
                    atn_manager,
                },
                arena,
            ),
        })
    }

    /// Channel names passed at construction.
    pub fn get_channel_names(&self) -> &'static [&'static str] {
        self.base.channel_names
    }

    /// Mode names passed at construction.
    pub fn get_mode_names(&self) -> &'static [&'static str] {
        self.base.mode_names
    }
}

impl<'input, 'arena, Input, TF> Deref for LexerInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    type Target = LexerInterpreterBase<'input, 'arena, Input, TF>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, 'arena, Input, TF> DerefMut for LexerInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, 'arena, Input, TF> crate::TokenSource<'input, 'arena, TF>
    for LexerInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    fn next_token(&mut self) -> &'arena mut TF::Tok {
        self.base.next_token()
    }

    fn get_line(&self) -> u32 {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> i32 {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn crate::int_stream::IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &TF {
        self.base.get_token_factory()
    }

    fn get_dfa_string(&self) -> String {
        self.base.get_dfa_string()
    }
}
