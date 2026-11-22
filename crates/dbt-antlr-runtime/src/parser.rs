//! Base parser implementation
use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use crate::arena::Arena;
use crate::atn::ATN;
use crate::atn_simulator::{IATNSimulator, ParserATNSimulatorManager};
use crate::error_listener::{
    ConsoleErrorListener, ErrorListener, ErrorListenerDelegate, ProxyErrorListener,
};
use crate::error_strategy::ErrorStrategy;
use crate::errors::ANTLRError;
use crate::interval_set::IntervalSet;
use crate::parser_atn_simulator::ParserATNSimulator;
use crate::prediction_mode::PredictionMode;
use crate::recognizer::{Actions, Recognizer};
use crate::rule_context::{states_stack, RuleContext as _};
use crate::token::{OwningToken, Token, TOKEN_EOF};
use crate::token_factory::TokenFactory;
use crate::token_stream::TokenStream;
use crate::tree::{
    NodeInner as _, NodeKindType, ParseTreeListener, ParserRuleContextCommon, Tree as _, TreeNode,
};
use crate::utils::cell_update;
use crate::vocabulary::Vocabulary;

#[cfg(feature = "recursion-limit")]
const DEFAULT_RECURSION_LIMIT: u32 = 2000;

/// parser functionality required for `ParserATNSimulator` to work
#[allow(missing_docs)]
pub trait Parser<'input, 'arena, TF>: Recognizer<'input, 'arena, TF::Tok>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_arena(&self) -> &'arena Arena;

    fn get_interpreter(&self) -> &'arena ParserATNSimulator<'arena>;

    fn get_token_factory(&self) -> &TF;

    /// Current rule context, or `None` when the parser is not inside a rule.
    ///
    /// Java has no equivalent accessor: `ParserATNSimulator.adaptivePredict`
    /// takes the outer context as an explicit parameter, and the tool's
    /// interpreter tests pass `ParserRuleContext.EMPTY`. This accessor lets
    /// [crate::parser_atn_simulator::ParserATNSimulator::adaptive_predict]
    /// treat a missing context as Java's `ParserRuleContext.EMPTY`.
    fn get_current_context(&self) -> Option<&'arena TreeNode<'input, 'arena, Self::Node, TF::Tok>>;

    fn consume(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<(), ANTLRError>
    where
        Self: Sized;

    fn precpred(
        &self,
        localctx: Option<&TreeNode<'input, 'arena, Self::Node, TF::Tok>>,
        precedence: i32,
    ) -> bool;

    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream<'input, 'arena, TF>;
    fn get_input_stream(&self) -> &dyn TokenStream<'input, 'arena, TF>;
    fn get_current_token(&self) -> &'arena TF::Tok;
    fn get_expected_tokens<'a>(&'a self) -> Cow<'a, IntervalSet>;

    fn add_error_listener(
        &mut self,
        listener: Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'input>,
    ) where
        Self: Sized;

    fn remove_error_listeners(&mut self);

    fn notify_error_listeners(
        &self,
        msg: String,
        offending_token: Option<isize>,
        err: Option<&ANTLRError>,
    );
    fn get_error_lister_dispatch<'a>(
        &'a self,
    ) -> Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'a>
    where
        Self: Sized;

    fn is_expected_token(&self, symbol: i32) -> bool;

    /// The `(parent_ctx_ptr, call_sites)` pairs of the active neutral parses,
    /// outermost first.
    #[doc(hidden)]
    fn neutral_call_sites(&self) -> &[(*const (), &'static [i32])] {
        &[]
    }
    fn get_precedence(&self) -> i32;

    fn get_state(&self) -> i32;
    fn set_state(&mut self, v: i32);
    fn get_rule_invocation_stack(&self) -> Vec<String>;

    #[cfg(feature = "recursion-limit")]
    fn get_recursion_limit(&self) -> u32;
    #[cfg(feature = "recursion-limit")]
    fn set_recursion_limit(&mut self, v: u32);
}

/// The parser state that the neutral parse of a shared-descent group saves and
/// restores (see [`BaseParser::begin_neutral_parse`]).
#[doc(hidden)]
#[derive(Debug)]
pub struct NeutralParse {
    start_index: isize,
    mark: isize,
    /// The stream index where the shared rule starts, after the prefix.
    shared_index: isize,
    state: i32,
    children: usize,
    matched_eof: bool,
}

/// A shared rule parsed by a neutral parse, waiting for the re-descent to call
/// it (see [`BaseParser::resume_take`]).
struct Resume<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    node: &'arena TreeNode<'input, 'arena, Node, Tok>,
    rule: usize,
    shared_index: isize,
    tail_index: isize,
    /// `matched_eof` after the shared rule.
    matched_eof: bool,
    mark: isize,
}

const INLINE_PRECEDENCE_CAP: usize = 16;

struct PrecedenceStack {
    top: i32,
    len: usize,
    inline: [i32; INLINE_PRECEDENCE_CAP],
    overflow: Vec<i32>,
}

impl PrecedenceStack {
    #[inline]
    fn new() -> Self {
        Self {
            top: 0,
            len: 1,
            inline: [0; INLINE_PRECEDENCE_CAP],
            overflow: Vec::new(),
        }
    }

    #[inline(always)]
    fn top(&self) -> i32 {
        self.top
    }

    #[inline(always)]
    fn push(&mut self, prec: i32) {
        let len = self.len;
        if len < INLINE_PRECEDENCE_CAP {
            self.inline[len] = prec;
        } else {
            self.push_overflow(prec);
        }
        self.len = len + 1;
        self.top = prec;
    }

    #[inline(always)]
    fn pop(&mut self) {
        let len = self.len;
        if len == 0 {
            return;
        }
        let new_len = len - 1;
        self.len = new_len;
        if new_len < INLINE_PRECEDENCE_CAP {
            self.top = if new_len == 0 {
                -1
            } else {
                self.inline[new_len - 1]
            };
        } else {
            self.top = self.pop_overflow(new_len);
        }
    }

    #[cold]
    #[inline(never)]
    fn push_overflow(&mut self, prec: i32) {
        self.overflow.push(prec);
    }

    #[cold]
    #[inline(never)]
    fn pop_overflow(&mut self, new_len: usize) -> i32 {
        self.overflow.pop();
        if new_len == INLINE_PRECEDENCE_CAP {
            self.inline[INLINE_PRECEDENCE_CAP - 1]
        } else {
            *self.overflow.last().unwrap()
        }
    }
}

/// Abstract base parser implementation
///
/// Only meant to be instantiated by generated parsers
pub struct BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    // Grammar-specific implementation of parser functionality required for
    // `ParserATNSimulator` to work
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    // Token factory used by the input stream
    TF: TokenFactory<'input, 'arena> + 'arena,
    // Token stream (lexer)
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    interp: Option<ParserATNSimulator<'arena>>,
    atn_manager: &'static ParserATNSimulatorManager,
    global_cache_threshold: usize,

    /// Points to the current rule context node, or is null outside a rule.
    ///
    /// A non-null `ctx` always points to a rule context [`TreeNode`] in
    /// `arena`. Code mutates `ctx` only through `with_mut_ctx`,
    /// `with_mut_ctx_common`, or `add_child_to_ctx`. Code updates the `ctx`
    /// pointer only through `set_current_ctx`. Callers never dereference `ctx`
    /// directly. As a result, `ctx` and a `&mut TreeNode` parameter (for
    /// example, `localctx`) are never both `&mut` in the same stack frame.
    ctx: *mut (),

    /// true if parser reached EOF
    pub matched_eof: bool,

    state: i32,
    /// Token stream that is currently used by this parser
    pub input: Input,
    precedence_stack: PrecedenceStack,
    #[cfg(feature = "recursion-limit")]
    pub recursion_limit: u32,
    #[cfg(feature = "recursion-limit")]
    current_recursion_depth: u32,

    parse_listeners: Vec<Box<Node::Listener>>,
    listeners_active: bool,
    _syntax_errors: Cell<i32>,
    has_default_console_listener: bool,
    primary_error_listener: Option<ErrorListenerDelegate<'input, 'arena, Self, TF::Tok>>,
    error_listeners: Vec<ErrorListenerDelegate<'input, 'arena, Self, TF::Tok>>,
    /// Mutes the parse listeners while it is above zero. A counter, because
    /// the neutral parses of nested shared-descent groups nest (see
    /// [`Self::begin_neutral_parse`]).
    muted: u32,
    /// The pending re-descent of a shared-descent group (see
    /// [`Self::resume_neutral_parse`]).
    resume: Option<Resume<'input, 'arena, Node, TF::Tok>>,
    /// The `(parent_ctx_ptr, call_sites)` pairs of the active neutral parses,
    /// outermost first.
    neutral_call_sites: Vec<(*const (), &'static [i32])>,

    pub arena: &'arena Arena,
    ext: Ext,
    pd: PhantomData<(
        &'input (),
        &'arena TF,
        //        &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
    )>,
}

impl<'input, 'arena, Ext, Node, Input, TF> Debug
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BaseParser")
            .field("state", &self.state)
            .field("matched_eof", &self.matched_eof)
            .finish_non_exhaustive()
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> Deref
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    type Target = Ext;

    fn deref(&self) -> &Self::Target {
        &self.ext
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> DerefMut
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ext
    }
}

pub trait ParserRecog<'input, 'arena, R, Tok>: Actions<'input, 'arena, R, Tok>
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: Token + 'input,
{
    fn get_atn_simulator_man(&self) -> &'static ParserATNSimulatorManager;
}

impl<'input, 'arena, Ext, Node, Input, TF> Recognizer<'input, 'arena, TF::Tok>
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    type Node = Node;

    fn sempred(
        &mut self,
        localctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
        rule_index: i32,
        action_index: i32,
    ) -> bool {
        Ext::sempred(localctx, rule_index, action_index, self)
    }

    fn get_rule_names(&self) -> &[&str] {
        self.ext.get_rule_names()
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        self.ext.get_vocabulary()
    }

    fn get_grammar_file_name(&self) -> &str {
        self.ext.get_grammar_file_name()
    }

    fn get_atn(&self) -> &ATN {
        self.interp.as_ref().unwrap().atn()
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> Parser<'input, 'arena, TF>
    for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn get_arena(&self) -> &'arena Arena {
        self.arena
    }

    #[inline(always)]
    fn get_interpreter(&self) -> &'arena ParserATNSimulator<'arena> {
        unsafe {
            std::mem::transmute::<&ParserATNSimulator<'arena>, &'arena ParserATNSimulator<'arena>>(
                self.interp.as_ref().unwrap(),
            )
        }
    }

    fn get_token_factory(&self) -> &TF {
        self.input.get_token_source().get_token_factory()
    }

    #[inline(always)]
    fn get_current_context(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        self.ctx()
    }

    #[inline]
    fn consume(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<(), ANTLRError> {
        let o = self.get_current_token();
        let is_eof = self.input.la(1) == TOKEN_EOF;
        self.consume_current(o, is_eof, err_handler)
    }

    fn precpred(
        &self,
        _localctx: Option<&TreeNode<'input, 'arena, Node, TF::Tok>>,
        precedence: i32,
    ) -> bool {
        precedence >= self.get_precedence()
    }

    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream<'input, 'arena, TF> {
        &mut self.input //.as_mut()
    }

    fn get_input_stream(&self) -> &dyn TokenStream<'input, 'arena, TF> {
        &self.input
    }

    #[inline(always)]
    fn get_current_token(&self) -> &'arena TF::Tok {
        self.input.current_token()
    }

    fn get_expected_tokens<'a>(&'a self) -> Cow<'a, IntervalSet> {
        let states_stack = states_stack(self.ctx().unwrap());
        self.interp
            .as_ref()
            .unwrap()
            .atn()
            .get_expected_tokens::<TF::Tok>(self.state, states_stack)
    }

    fn add_error_listener(
        &mut self,
        listener: Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'input>,
    ) {
        let delegate = ErrorListenerDelegate::new(listener);
        if self.primary_error_listener.is_none() {
            self.primary_error_listener = Some(delegate);
        } else {
            self.error_listeners.push(delegate);
        }
    }

    fn remove_error_listeners(&mut self) {
        self.has_default_console_listener = false;
        self.primary_error_listener = None;
        self.error_listeners.clear();
    }

    fn notify_error_listeners(
        &self,
        msg: String,
        offending_token: Option<isize>,
        err: Option<&ANTLRError>,
    ) {
        cell_update(&self._syntax_errors, |it| it + 1);
        if !self.has_default_console_listener
            && self.primary_error_listener.is_none()
            && self.error_listeners.is_empty()
        {
            return;
        }
        let offending_token: Option<&_> = match offending_token {
            None => Some(self.get_current_token().borrow()),
            Some(x) => Some(self.input.get(x).borrow()),
        };
        let line = offending_token.map_or(0, |x| x.get_line());
        let column = offending_token.map_or(-1, |x| x.get_char_position_in_line());

        if self.has_default_console_listener {
            ConsoleErrorListener.syntax_error(
                self,
                offending_token.map(|x| x as _),
                line,
                column,
                &msg,
                err,
            );
        }
        if let Some(ref listener) = self.primary_error_listener {
            listener.syntax_error(
                self,
                offending_token.map(|x| x as _),
                line,
                column,
                &msg,
                err,
            );
        }
        for listener in self.error_listeners.iter() {
            listener.syntax_error(
                self,
                offending_token.map(|x| x as _),
                line,
                column,
                &msg,
                err,
            )
        }
    }

    fn get_error_lister_dispatch<'a>(
        &'a self,
    ) -> Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'a> {
        Box::new(ProxyErrorListener {
            has_default_console_listener: self.has_default_console_listener,
            primary: self.primary_error_listener.as_ref(),
            delegates: self.error_listeners.borrow(),
        })
    }

    fn is_expected_token(&self, _symbol: i32) -> bool {
        unimplemented!()
    }

    #[inline(always)]
    fn get_precedence(&self) -> i32 {
        self.precedence_stack.top()
    }

    #[inline(always)]
    fn neutral_call_sites(&self) -> &[(*const (), &'static [i32])] {
        &self.neutral_call_sites
    }

    #[inline(always)]
    fn get_state(&self) -> i32 {
        self.state
    }

    #[inline(always)]
    fn set_state(&mut self, v: i32) {
        self.state = v;
    }

    fn get_rule_invocation_stack(&self) -> Vec<String> {
        let mut vec = Vec::new();
        let rule_names = self.get_rule_names();
        let mut ctx = self.get_current_context().unwrap();
        loop {
            let rule_index = ctx.get_rule_index();
            vec.push(rule_names.get(rule_index).unwrap_or(&"n/a").to_string());
            ctx = if let Some(parent) = ctx.get_parent() {
                parent
            } else {
                break;
            }
        }
        vec
    }

    #[cfg(feature = "recursion-limit")]
    fn get_recursion_limit(&self) -> u32 {
        self.recursion_limit
    }

    #[cfg(feature = "recursion-limit")]
    fn set_recursion_limit(&mut self, v: u32) {
        self.recursion_limit = v;
    }
}

#[allow(missing_docs)] // todo docs
impl<'input, 'arena, Ext, Node, Input, TF> BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    /// Replaces the input token stream, mirroring Java's
    /// `Parser.setTokenStream`.
    ///
    /// Unlike Java, this does not seek the old stream back to 0 or reset
    /// parser state; it is intended for swapping in a fresh stream before any
    /// rule invocation, as the ANTLR tool's interpreter tests do when they
    /// reuse one parser (and its decision DFAs) across several inputs.
    pub fn set_input_stream(&mut self, input: Input) {
        self.input = input;
    }

    pub fn new_base_parser(arena: &'arena Arena, input: Input, ext: Ext) -> Self {
        let atn_manager = ext.get_atn_simulator_man();
        let interp = ParserATNSimulator::new(atn_manager.get_simulator(arena));
        Self {
            atn_manager,
            interp: Some(interp),
            global_cache_threshold: 0,
            ctx: std::ptr::null_mut(),
            matched_eof: false,
            state: -1,
            input,
            precedence_stack: PrecedenceStack::new(),
            #[cfg(feature = "recursion-limit")]
            recursion_limit: DEFAULT_RECURSION_LIMIT,
            #[cfg(feature = "recursion-limit")]
            current_recursion_depth: 0,
            parse_listeners: vec![],
            listeners_active: false,
            _syntax_errors: Cell::new(0),
            has_default_console_listener: true,
            primary_error_listener: None,
            error_listeners: Vec::new(),
            muted: 0,
            resume: None,
            neutral_call_sites: Vec::new(),
            arena,
            ext,
            pd: PhantomData,
        }
    }

    pub fn set_global_cache_threshold(&mut self, threshold: usize) {
        self.global_cache_threshold = threshold;
    }

    pub fn get_interpreter_mut(&mut self) -> &mut ParserATNSimulator<'arena> {
        self.interp.as_mut().unwrap()
    }

    /// Predicts the alternative of a decision with its static SLL DFA table.
    ///
    /// Returns [`INVALID_ALT`] when the decision has no table and when the walk
    /// leaves the table at an escape state the generated code must handle with
    /// [`ParserATNSimulator::adaptive_predict`]. The virtual alternative of a
    /// shared-descent group is an escape too: this block has no arm for it (see
    /// [`Self::dfa_predict_virtual`]). A lookahead that has no table edge and no
    /// fallback alternative fails like the adaptive simulation does.
    ///
    /// The walk advances a lookahead cursor through [`TokenStream::next_lookahead`]
    /// without a stream position change or rewind.
    /// This keeps a k-token lookahead O(k) on a channel-filtering token stream.
    ///
    /// [`INVALID_ALT`]: crate::atn::INVALID_ALT
    pub fn dfa_predict(&mut self, decision: i32) -> Result<i32, ANTLRError> {
        let alt = self.dfa_walk(decision)?;
        if alt >= crate::dfa::static_parser_dfa::VIRTUAL_ALT_BASE {
            return Ok(crate::atn::INVALID_ALT);
        }
        Ok(alt)
    }

    /// Predicts like [`Self::dfa_predict`], but returns the virtual alternative
    /// of a shared-descent group (at or above
    /// [`VIRTUAL_ALT_BASE`](crate::dfa::static_parser_dfa::VIRTUAL_ALT_BASE))
    /// instead of deferring. A generated `DescentAltBlock` resolves the group to
    /// one of its member alternatives.
    pub fn dfa_predict_virtual(&mut self, decision: i32) -> Result<i32, ANTLRError> {
        self.dfa_walk(decision)
    }
    /// The table walk of [`Self::dfa_predict`] and
    /// [`Self::dfa_predict_virtual`].
    ///
    /// An edge entry flags a target state where the walk stops, so the walk
    /// resolves an accept without consuming the token that leads into it. A
    /// one-token decision that accepts returns before it marks the input.
    #[inline]
    fn dfa_walk(&mut self, decision: i32) -> Result<i32, ANTLRError> {
        use crate::dfa::static_parser_dfa::{NO_EDGE, STATE_MASK, TERMINAL_BIT};

        let Some(tables) = self.get_interpreter().static_parser_tables() else {
            return Ok(crate::atn::INVALID_ALT);
        };
        let Some(table) = tables.table(decision, self.get_precedence()) else {
            return Ok(crate::atn::INVALID_ALT);
        };
        let token = self.input.la(1);
        let entry = table.start_entry(token);
        if entry >= TERMINAL_BIT && entry != NO_EDGE {
            // SAFETY: the loader checked that edge targets are states.
            let outcome = unsafe { table.outcome_unchecked(usize::from(entry & STATE_MASK)) };
            if outcome > 0 {
                return Ok(outcome);
            }
        }
        self.dfa_walk_steps(table, decision, token, entry)
    }

    /// Continues [`Self::dfa_walk`] from the start state, whose entry for
    /// `token` is `entry`.
    fn dfa_walk_steps(
        &mut self,
        table: &crate::dfa::static_parser_dfa::StaticParserTable,
        decision: i32,
        mut token: i32,
        mut entry: u16,
    ) -> Result<i32, ANTLRError> {
        use crate::dfa::static_parser_dfa::{Outcome, NO_EDGE, STATE_MASK, TERMINAL_BIT};

        let start_index = self.input.index();
        let mark = self.input.mark();
        let mut cursor = start_index;
        let mut state = 0_usize;
        let result = loop {
            if entry == NO_EDGE {
                break self.dfa_no_edge(table, decision, state, start_index, cursor);
            }
            let target = usize::from(entry & STATE_MASK);
            if entry & TERMINAL_BIT != 0 {
                // SAFETY: the loader checked that edge targets are states.
                let outcome = unsafe { table.outcome_unchecked(target) };
                if outcome > 0 {
                    break Ok(outcome);
                }
                let outcome = Outcome::decode(outcome);
                if matches!(outcome, Some(Outcome::Escape(Some(_)))) {
                    self.input.seek(cursor);
                    if token != TOKEN_EOF {
                        // A frontier continuation resumes after the token that
                        // led into the escape state.
                        self.input.consume();
                    }
                }
                break self.dfa_terminal(table, decision, outcome, start_index);
            }
            if token != TOKEN_EOF {
                (cursor, token) = self.input.next_lookahead(cursor);
            }
            state = target;
            // SAFETY: `state` is an edge target.
            entry = unsafe { table.edge_entry_unchecked(state, token) };
        };
        self.input.release(mark);
        result
    }

    /// Resolves a table walk without an edge for the current token: the start
    /// state may stop the walk itself, and otherwise the state falls back to
    /// an alternative that already left the decision or fails.
    #[cold]
    fn dfa_no_edge(
        &mut self,
        table: &crate::dfa::static_parser_dfa::StaticParserTable,
        decision: i32,
        state: usize,
        start_index: isize,
        cursor: isize,
    ) -> Result<i32, ANTLRError> {
        use crate::dfa::static_parser_dfa::Outcome;

        // SAFETY: `state` is 0 or an edge target.
        let outcome = unsafe { table.outcome_unchecked(state) };
        if outcome > 0 {
            return Ok(outcome);
        }
        if outcome != 0 {
            return self.dfa_terminal(table, decision, Outcome::decode(outcome), start_index);
        }
        if let Some(fallback) = table.fallback(state as i32) {
            return Ok(fallback);
        }
        let start = OwningToken::from(self.input.get(start_index) as &dyn Token);
        self.input.seek(cursor);
        let error = match self.input.lt(1) {
            Some(offending) => {
                ANTLRError::no_alt_full(self, start, OwningToken::from(offending as &dyn Token))
            }
            None => ANTLRError::no_alt(self),
        };
        self.input.seek(start_index);
        Err(error)
    }

    /// Resolves a table walk that stopped at an escape, conflict escape, or
    /// guarded state. A frontier continuation resumes from the current
    /// position. Every other outcome expects the stream at `start_index`.
    #[cold]
    fn dfa_terminal(
        &mut self,
        table: &crate::dfa::static_parser_dfa::StaticParserTable,
        decision: i32,
        outcome: Option<crate::dfa::static_parser_dfa::Outcome>,
        start_index: isize,
    ) -> Result<i32, ANTLRError> {
        use crate::dfa::static_parser_dfa::Outcome;

        match outcome {
            Some(Outcome::Escape(Some(slot))) => self.get_interpreter().resume_from_continuation(
                decision,
                slot as usize,
                table.num_continuations(),
                table.continuation_at(slot),
                start_index,
                self,
            ),
            Some(Outcome::Conflict(min_alt)) => {
                debug_assert_eq!(self.input.index(), start_index);
                self.get_interpreter()
                    .adaptive_predict_conflict(decision, min_alt, self)
            }
            Some(Outcome::Guarded(guard)) => {
                debug_assert_eq!(self.input.index(), start_index);
                let defers = self.guard_defers(table, guard.group);
                Ok(if defers { guard.defer_alt } else { guard.alt })
            }
            _ => {
                debug_assert_eq!(self.input.index(), start_index);
                Ok(crate::atn::INVALID_ALT)
            }
        }
    }

    /// Whether the stack guard of a guarded state defers.
    ///
    /// A guarded conflict (an optional-postfix take or a precedence loop's
    /// iterate) resolves to the guarded alternative unless the real parse
    /// stack's epsilon-pop chase can reach a guard root of the state's guard
    /// group. The walk stops at the first invoking state whose follow region
    /// cannot reach its own rule's stop.
    fn guard_defers(
        &self,
        table: &crate::dfa::static_parser_dfa::StaticParserTable,
        group: u32,
    ) -> bool {
        let mut context = self.ctx();
        while let Some(node) = context {
            let invoking_state = node.get_invoking_state();
            if invoking_state < 0 {
                return false;
            }
            let parent = node.get_parent();
            let parent_ptr = parent.map_or(std::ptr::null(), |p| p as *const _ as *const ());
            if let Some((_, call_sites)) = self
                .neutral_call_sites
                .iter()
                .find(|&&(ptr, sites)| ptr == parent_ptr && sites.first() == Some(&invoking_state))
            {
                if call_sites
                    .iter()
                    .any(|&site| table.guard_step(group, site) == Some(true))
                {
                    return true;
                }
                if call_sites
                    .iter()
                    .all(|&site| table.guard_step(group, site) == Some(false))
                {
                    return false;
                }
                context = parent;
                continue;
            }
            match table.guard_step(group, invoking_state) {
                Some(true) => return true,
                Some(false) => return false,
                None => context = parent,
            }
        }
        false
    }
    /// Whether a shared-descent group can start a neutral parse here.
    /// A group cannot start while the re-descent of another group is pending.
    ///
    /// A group also cannot start in the SLL prediction mode. For an ambiguous
    /// input, SLL resolves the conflict of the decision to its minimum
    /// alternative, but the tail dispatch after the shared rule can select
    /// another member.
    #[doc(hidden)]
    pub fn descent_ready(&mut self) -> bool {
        self.drop_stale_resume();
        self.resume.is_none() && self.get_interpreter().get_prediction_mode() != PredictionMode::SLL
    }

    /// Ends a re-descent that moved past the start of its shared rule without
    /// calling it. An error recovery in the prefix can do that; the kept
    /// subtree is then useless.
    fn drop_stale_resume(&mut self) {
        let index = self.input.index();
        if let Some(resume) = self.resume.take_if(|resume| index > resume.shared_index) {
            self.input.release(resume.mark);
        }
    }

    /// Starts the neutral parse of a shared-descent group: the generated block
    /// calls the shared rule once, with parse listeners muted, to read the
    /// token after it.
    ///
    /// Skips the `prefix_len` leading tokens of the group (the table walk read
    /// them) and records the group's member `call_sites` so the shared rule's
    /// context has a real invoking state and full-context prediction inside it
    /// sees every member's follow state. Finish with
    /// [`Self::resume_neutral_parse`] to keep its subtree for the re-descent,
    /// or [`Self::abort_neutral_parse`] when the shared rule exits with an
    /// unrecoverable error.
    #[doc(hidden)]
    pub fn begin_neutral_parse(
        &mut self,
        call_sites: &'static [i32],
        prefix_len: usize,
    ) -> NeutralParse {
        let ctx = self.ctx();
        let start_index = self.input.index();
        let mark = self.input.mark();
        // Skip by consuming, not by index arithmetic: a channel-filtering
        // stream has off-channel tokens between the prefix tokens.
        for _ in 0..prefix_len {
            if self.input.la(1) != TOKEN_EOF {
                self.input.consume();
            }
        }
        let neutral = NeutralParse {
            start_index,
            mark,
            shared_index: self.input.index(),
            state: self.state,
            children: ctx.map_or(0, |ctx| ctx.get_child_count()),
            matched_eof: self.matched_eof,
        };
        self.neutral_call_sites.push((self.ctx, call_sites));
        self.muted += 1;
        self.update_listeners_active();
        self.state = call_sites.first().copied().unwrap_or(-1);
        neutral
    }

    /// Releases a neutral parse when the shared rule returned an
    /// unrecoverable error.
    #[doc(hidden)]
    pub fn abort_neutral_parse(&mut self, neutral: NeutralParse) {
        debug_assert!(self.muted > 0, "no neutral parse is active");
        self.muted -= 1;
        self.update_listeners_active();
        self.neutral_call_sites.pop();
        self.input.release(neutral.mark);
    }

    /// Finishes a neutral parse and keeps the subtree `node` of the shared rule
    /// `rule` for the re-descent.
    ///
    /// Rewinds the stream to the decision start and detaches `node` from the
    /// current context so the member alternative can re-descend through its
    /// prefix to its call of the shared rule, where [`Self::resume_take`]
    /// grafts `node` back and seeks to the tail.
    #[doc(hidden)]
    pub fn resume_neutral_parse(
        &mut self,
        neutral: NeutralParse,
        node: &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
        rule: usize,
    ) {
        debug_assert!(self.muted > 0, "no neutral parse is active");
        self.muted -= 1;
        self.update_listeners_active();
        self.neutral_call_sites.pop();
        // The mark stays until the re-descent seeks forward to the tail.
        self.resume = Some(Resume {
            node,
            rule,
            shared_index: neutral.shared_index,
            tail_index: self.input.index(),
            matched_eof: self.matched_eof,
            mark: neutral.mark,
        });
        self.input.seek(neutral.start_index);
        if let Some(ctx) = unsafe { self.ctx_mut() } {
            ctx.truncate_children(neutral.children);
        }
        self.state = neutral.state;
        self.matched_eof = neutral.matched_eof;
    }

    /// Returns the subtree that a neutral parse kept for the shared rule
    /// `rule`, when the re-descent calls it at the position where the neutral
    /// parse started it. The subtree is grafted under the current context, the
    /// muted listener events are replayed, and the stream moves to the token
    /// after the subtree.
    ///
    /// Returns `None` when no re-descent is pending or `rule` is another rule.
    /// A call of `rule` at another position ends the re-descent: the kept
    /// subtree does not belong there, so the rule parses normally.
    #[doc(hidden)]
    pub fn resume_take(
        &mut self,
        rule: usize,
    ) -> Result<Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>, ANTLRError> {
        let Some(resume) = self.resume.take_if(|resume| resume.rule == rule) else {
            return Ok(None);
        };
        if self.input.index() != resume.shared_index {
            self.input.release(resume.mark);
            return Ok(None);
        }
        let node = resume.node;
        let state = self.state;
        let parent = self.ctx();
        self.add_child_to_ctx(node);
        // The neutral parse created the context under the decision's context
        // at the time; the re-descent's context and call site may differ.
        self.set_current_ctx(Some(node));
        self.with_mut_ctx(|ctx| {
            ctx.set_parent(parent);
            ctx.set_invoking_state(state);
        });
        self.set_current_ctx(parent);
        if self.listeners_active {
            self.replay_listener_events(node)?;
        }
        self.matched_eof = resume.matched_eof;
        self.input.seek(resume.tail_index);
        self.input.release(resume.mark);
        Ok(Some(node))
    }

    /// Fires the parse listener events of the subtree `root` in the order of
    /// a live parse: enter events in listener order, exit events in reverse
    /// listener order.
    ///
    /// A live parse of a left-recursive rule exits the previous context of
    /// the operator loop before it enters the new recursion context, and that
    /// previous context is the first child of the new one. So the replay
    /// visits such a first child before it enters its parent.
    fn replay_listener_events(
        &mut self,
        root: &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
    ) -> Result<(), ANTLRError> {
        struct Frame<'a, T> {
            node: &'a T,
            next_child: usize,
            entered: bool,
        }
        // An explicit stack: the subtree can be as deep as the input.
        let mut stack = vec![Frame {
            node: root,
            next_child: 0,
            entered: false,
        }];
        while let Some(Frame {
            node,
            mut next_child,
            mut entered,
        }) = stack.pop()
        {
            if let Some(terminal) = node.as_terminal_node() {
                for listener in &mut self.parse_listeners {
                    listener.visit_terminal(terminal)?;
                }
                continue;
            }
            if let Some(error) = node.as_error_node() {
                for listener in &mut self.parse_listeners {
                    listener.visit_error_node(error)?;
                }
                continue;
            }
            if !entered {
                let previous = if next_child == 0 {
                    self.recursion_predecessor(node)
                } else {
                    None
                };
                if let Some(previous) = previous {
                    stack.push(Frame {
                        node,
                        next_child: 1,
                        entered: false,
                    });
                    stack.push(Frame {
                        node: previous,
                        next_child: 0,
                        entered: false,
                    });
                    continue;
                }
                for listener in self.parse_listeners.iter_mut() {
                    listener.enter_every_rule(node)?;
                    node.enter_rule(listener)?;
                }
                entered = true;
            }
            if let Some(child) = node.get_child(next_child) {
                next_child += 1;
                stack.push(Frame {
                    node,
                    next_child,
                    entered,
                });
                stack.push(Frame {
                    node: child,
                    next_child: 0,
                    entered: false,
                });
            } else {
                for listener in self.parse_listeners.iter_mut().rev() {
                    node.exit_rule(listener)?;
                    listener.exit_every_rule(node)?;
                }
            }
        }
        Ok(())
    }

    /// The previous context of the left-recursive operator loop that created
    /// the recursion context `node`: its first child, a context of the same
    /// rule that `push_new_recursion_context` re-parented with the rule start
    /// state as its invoking state.
    fn recursion_predecessor(
        &self,
        node: &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
    ) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        let first = node.get_child(0)?;
        if first.as_terminal_node().is_some() || first.as_error_node().is_some() {
            return None;
        }
        let rule = node.get_rule_index();
        let start = self.get_interpreter().atn().rule_to_start_state.get(rule)?;
        (first.get_rule_index() == rule && first.get_invoking_state() == start.get_state_number())
            .then_some(first)
    }

    /// If current context is same as the given one by pointer comparison
    #[inline]
    pub fn ctx_is(&self, other: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>) -> bool {
        if self.ctx.is_null() && other.is_none() {
            true
        } else if self.ctx.is_null() || other.is_none() {
            false
        } else {
            std::ptr::eq(self.ctx, other.unwrap() as *const _ as *const ())
        }
    }

    /// Gets a reference to current context.
    #[inline]
    pub fn ctx(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        if self.ctx.is_null() {
            None
        } else {
            unsafe { Some(&*(self.ctx as *const TreeNode<'input, 'arena, Node, TF::Tok>)) }
        }
    }

    /// Gets a mutable reference to current context.
    ///
    /// # Safety
    /// Follows the same safety rules as dereferencing *mut to &mut
    #[inline]
    pub unsafe fn ctx_mut(
        &mut self,
    ) -> Option<&'arena mut TreeNode<'input, 'arena, Node, TF::Tok>> {
        if self.ctx.is_null() {
            None
        } else {
            Some(&mut *(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>))
        }
    }

    #[inline(always)]
    fn ctx_common(&self) -> Option<&'arena ParserRuleContextCommon<'input, 'arena, Node, TF::Tok>> {
        self.ctx().map(|node| {
            // SAFETY: `self.ctx` always points to a rule context node in `self.arena`.
            unsafe { ParserRuleContextCommon::cast_from_unchecked(node) }
        })
    }

    #[inline]
    fn parent_ctx(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        self.ctx_common().and_then(|ctx| ctx.parent)
    }

    #[inline(always)]
    fn set_current_ctx(&mut self, ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>) {
        self.ctx = match ctx {
            Some(ctx) => {
                debug_assert!(ctx.node_tag().is_context());
                ctx as *const TreeNode<'input, 'arena, Node, TF::Tok> as *mut ()
            }
            None => std::ptr::null_mut(),
        };
    }

    #[inline(always)]
    fn update_listeners_active(&mut self) {
        self.listeners_active = self.muted == 0 && !self.parse_listeners.is_empty();
    }

    #[inline(always)]
    fn add_child_to_ctx(&mut self, child: &'arena TreeNode<'input, 'arena, Node, TF::Tok>) {
        if !self.ctx.is_null() {
            let arena = self.arena;
            self.with_mut_ctx_common(|common| common.children.push(child, arena));
        }
    }

    #[inline(always)]
    pub fn with_mut_ctx<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut TreeNode<'input, 'arena, Node, TF::Tok>) -> R,
    {
        debug_assert!(!self.ctx.is_null());
        unsafe { f(&mut *(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)) }
    }

    #[inline(always)]
    fn with_mut_ctx_common<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut ParserRuleContextCommon<'input, 'arena, Node, TF::Tok>) -> R,
    {
        self.with_mut_ctx(|ctx| {
            // SAFETY: `self.ctx` always points to a rule context node in `self.arena`.
            f(unsafe { ctx.as_rule_context_common_mut_unchecked() })
        })
    }

    #[cold]
    #[inline(never)]
    fn visit_terminal_listeners(
        &mut self,
        node: &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
    ) -> Result<(), ANTLRError> {
        let terminal = node
            .as_terminal_node()
            .expect("node was created as terminal node");
        for listener in &mut self.parse_listeners {
            listener.visit_terminal(terminal)?;
        }
        Ok(())
    }

    #[cold]
    #[inline(never)]
    fn consume_error_node(&mut self, o: &'arena TF::Tok) -> Result<(), ANTLRError> {
        let node = self.create_error_node(o)?;
        self.add_child_to_ctx(node);
        if self.listeners_active {
            let error = node
                .as_error_node()
                .expect("node was created as error node");
            for listener in &mut self.parse_listeners {
                listener.visit_error_node(error)?;
            }
        }
        Ok(())
    }

    #[inline]
    fn consume_current(
        &mut self,
        o: &'arena TF::Tok,
        is_eof: bool,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<(), ANTLRError> {
        if !is_eof {
            self.input.consume();
        }
        if err_handler.in_error_recovery_mode(self) {
            return self.consume_error_node(o);
        }
        let node = self.create_token_node(o)?;
        self.add_child_to_ctx(node);
        if self.listeners_active {
            self.visit_terminal_listeners(node)?;
        }
        Ok(())
    }

    #[cold]
    #[inline(never)]
    fn match_recover_inline(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        let token = err_handler.recover_inline(self)?;
        if token.get_token_index() == -1 {
            let err_node = self.create_error_node(token)?;
            self.add_child_to_ctx(err_node);
        }
        Ok(token)
    }

    #[inline]
    pub fn match_token(
        &mut self,
        ttype: i32,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        if self.input.la(1) == ttype {
            let token = self.get_current_token();
            let is_eof = ttype == TOKEN_EOF;
            if is_eof {
                self.matched_eof = true;
            }

            err_handler.report_match(self);
            self.consume_current(token, is_eof, err_handler)?;
            Ok(token)
        } else {
            self.match_recover_inline(err_handler)
        }
    }

    #[inline]
    pub fn match_wildcard(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        if self.input.la(1) > 0 {
            let token = self.get_current_token();
            err_handler.report_match(self);
            self.consume_current(token, false, err_handler)?;
            Ok(token)
        } else {
            self.match_recover_inline(err_handler)
        }
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_dyn_parse_listener(&mut self, listener: Box<Node::Listener>) {
        self.parse_listeners.push(listener);
        self.update_listeners_active();
    }

    /// Removes parse listener with corresponding `listener_id`, casts it back to user type and returns it to the caller.
    /// `listener_id` is returned when listener is added via `add_parse_listener`
    pub fn remove_parse_listener<L>(&mut self, listener_id: ListenerId<L>) -> Box<L>
    where
        L: ParseTreeListener<'arena, Node, TF::Tok>,
    {
        let index = self
            .parse_listeners
            .iter()
            .position(|it| ListenerId::new(it).actual_id == listener_id.actual_id)
            .expect("listener not found");
        let removed = self.parse_listeners.remove(index);
        self.update_listeners_active();
        unsafe { listener_id.into_listener(removed) }
    }

    /// Removes all added parse listeners without returning them
    pub fn remove_parse_listeners(&mut self) {
        self.parse_listeners.clear();
        self.update_listeners_active();
    }

    #[inline]
    pub fn trigger_enter_rule_event(&mut self) -> Result<(), ANTLRError> {
        if !self.listeners_active {
            return Ok(());
        }
        self.trigger_enter_rule_event_cold()
    }

    #[cold]
    #[inline(never)]
    fn trigger_enter_rule_event_cold(&mut self) -> Result<(), ANTLRError> {
        let ctx = self.ctx().unwrap();
        for listener in self.parse_listeners.iter_mut() {
            listener.enter_every_rule(ctx)?;
            ctx.enter_rule(listener)?;
        }
        Ok(())
    }

    #[inline]
    pub fn trigger_exit_rule_event(&mut self) -> Result<(), ANTLRError> {
        if !self.listeners_active {
            return Ok(());
        }
        self.trigger_exit_rule_event_cold()
    }

    #[cold]
    #[inline(never)]
    fn trigger_exit_rule_event_cold(&mut self) -> Result<(), ANTLRError> {
        let ctx = self.ctx().unwrap();
        for listener in self.parse_listeners.iter_mut().rev() {
            ctx.exit_rule(listener)?;
            listener.exit_every_rule(ctx)?;
        }
        Ok(())
    }

    #[inline]
    pub fn enter_rule(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
    ) -> Result<(), ANTLRError> {
        debug_assert!(localctx.node_tag().is_context());

        let start = self.input.lt(1);
        let parent = {
            // SAFETY: `localctx` is a rule context node allocated in `self.arena`.
            let common = unsafe { localctx.as_rule_context_common_mut_unchecked() };
            common.start = start;
            common.parent
        };
        let child: &'arena TreeNode<'input, 'arena, Node, TF::Tok> = localctx;

        self.set_current_ctx(parent);
        self.add_child_to_ctx(child);
        self.set_state(state);
        self.set_current_ctx(Some(child));

        if self.listeners_active {
            self.trigger_enter_rule_event_cold()?;
        }

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth += 1;
            if self.current_recursion_depth > self.recursion_limit {
                return Err(ANTLRError::recursion_limit_exceeded(self.recursion_limit));
            }
        }
        Ok(())
    }

    #[inline]
    pub fn exit_rule(
        &mut self,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth -= 1;
        }
        let stop = if self.matched_eof {
            // if we have matched EOF, it cannot consume past EOF so we use LT(1) here
            self.input.lt(1)
        } else {
            // stop node is what we just matched
            self.input.lt(-1)
        };
        self.with_mut_ctx_common(|common| common.stop = stop);
        if self.listeners_active {
            self.trigger_exit_rule_event_cold()?;
        }

        let ctx = unsafe { self.ctx_common().unwrap_unchecked() };
        let child = ctx.as_node();
        self.set_state(child.get_invoking_state());
        self.set_current_ctx(ctx.parent);

        Ok(child)
    }

    #[inline]
    pub fn enter_recursion_rule(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
        precedence: i32,
    ) -> Result<(), ANTLRError> {
        debug_assert!(localctx.node_tag().is_context());

        self.set_state(state);
        self.precedence_stack.push(precedence);
        self.set_current_ctx(Some(localctx));
        let start = self.input.lt(1);
        self.with_mut_ctx_common(|ctx| ctx.start = start);
        if self.listeners_active {
            self.trigger_enter_rule_event_cold()?;
        }

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth += 1;
            if self.current_recursion_depth > self.recursion_limit {
                return Err(ANTLRError::recursion_limit_exceeded(self.recursion_limit));
            }
        }
        Ok(())
    }

    #[inline]
    pub fn push_new_recursion_context(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        debug_assert!(localctx.node_tag().is_context());

        let stop = self.input.lt(-1);
        let next: &'arena TreeNode<'input, 'arena, Node, TF::Tok> = localctx;
        let start = self.with_mut_ctx_common(|prev| {
            prev.as_node_mut().set_invoking_state(state);
            prev.parent = Some(next);
            prev.stop = stop;
            prev.start
        });
        let prev = self.ctx().unwrap();
        self.set_current_ctx(Some(next));
        let arena = self.arena;
        self.with_mut_ctx_common(|next_ctx| {
            next_ctx.start = start;
            next_ctx.children.push(prev, arena);
        });
        if self.listeners_active {
            self.trigger_enter_rule_event_cold()?;
        }
        Ok(prev)
    }

    #[cold]
    #[inline(never)]
    fn unroll_recursion_exit_events_cold(
        &mut self,
        parent_ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
    ) -> Result<(), ANTLRError> {
        while !self.ctx_is(parent_ctx) {
            self.trigger_exit_rule_event_cold()?;
            self.set_current_ctx(self.parent_ctx());
        }
        Ok(())
    }

    #[inline]
    pub fn unroll_recursion_context(
        &mut self,
        parent_ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth -= 1;
        }
        self.precedence_stack.pop();
        let stop = self.input.lt(-1);
        if self.listeners_active {
            self.with_mut_ctx_common(|common| common.stop = stop);
            let retctx = self.ctx().unwrap();
            self.unroll_recursion_exit_events_cold(parent_ctx)?;
            self.set_current_ctx(Some(retctx));
            self.with_mut_ctx_common(|common| common.parent = parent_ctx);
            self.set_current_ctx(parent_ctx);
            self.add_child_to_ctx(retctx);
            Ok(retctx)
        } else {
            self.with_mut_ctx_common(|common| {
                common.stop = stop;
                common.parent = parent_ctx;
            });
            let retctx = self.ctx().unwrap();
            self.set_current_ctx(parent_ctx);
            self.add_child_to_ctx(retctx);
            Ok(retctx)
        }
    }

    #[inline]
    #[allow(clippy::mut_from_ref)] // &mut is from the arena allocation
    fn create_token_node(
        &self,
        token: &'arena TF::Tok,
    ) -> Result<&'arena mut TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        let ptr = TreeNode::create_token_node(self.arena, token);
        if ptr.is_null() {
            #[cfg(feature = "arena-allocation-limit")]
            {
                Err(ANTLRError::arena_allocation_limit_exceeded(
                    self.arena.allocation_limit_bytes(),
                    self.arena.total_allocated_bytes(),
                ))
            }
            #[cfg(not(feature = "arena-allocation-limit"))]
            {
                std::alloc::handle_alloc_error(std::alloc::Layout::new::<
                    TreeNode<'input, 'arena, Node, TF::Tok>,
                >())
            }
        } else {
            Ok(unsafe { &mut *ptr })
        }
    }

    #[allow(clippy::mut_from_ref)] // &mut is from the arena allocation
    fn create_error_node(
        &self,
        token: &'arena TF::Tok,
    ) -> Result<&'arena mut TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        let ptr = TreeNode::create_error_node(self.arena, token);
        if ptr.is_null() {
            #[cfg(feature = "arena-allocation-limit")]
            {
                Err(ANTLRError::arena_allocation_limit_exceeded(
                    self.arena.allocation_limit_bytes(),
                    self.arena.total_allocated_bytes(),
                ))
            }
            #[cfg(not(feature = "arena-allocation-limit"))]
            {
                std::alloc::handle_alloc_error(std::alloc::Layout::new::<
                    TreeNode<'input, 'arena, Node, TF::Tok>,
                >())
            }
        } else {
            Ok(unsafe { &mut *ptr })
        }
    }

    /// Text representation of generated DFA for debugging purposes
    pub fn dump_dfa(&self) {
        let mut seen_one = false;
        for i in 0..self.get_atn().decision_to_state.len() {
            let dfa = self
                .get_interpreter()
                .decision_to_dfa(i)
                .expect("dfa should exist for each decision");
            if !dfa.is_empty() {
                if seen_one {
                    println!()
                }
                println!("Decision {}:", dfa.decision);
                print!("{}", dfa.to_string(self.get_vocabulary()));
                seen_one = true;
            }
        }
    }
}

impl<'input, 'arena, Ext, Node, Input, TF> Drop for BaseParser<'input, 'arena, Ext, Node, Input, TF>
where
    'input: 'arena,
    Ext: ParserRecog<'input, 'arena, Self, TF::Tok>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF>,
    Node: NodeKindType<'arena, TF::Tok>,
{
    fn drop(&mut self) {
        if self.global_cache_threshold == 0 {
            return;
        }

        let Some(interp) = self.interp.take() else {
            return;
        };
        let cache_bytes = interp.total_allocated_bytes();
        drop(interp);

        if cache_bytes > self.global_cache_threshold {
            self.atn_manager.reset_all();
        }
    }
}

/// Allows to safely cast listener back to user type
#[derive(Debug)]
pub struct ListenerId<T: ?Sized> {
    pub(crate) actual_id: usize,
    phantom: PhantomData<fn() -> T>,
}

impl<T: ?Sized> ListenerId<T> {
    #[allow(clippy::borrowed_box)]
    pub fn new(listener: &Box<T>) -> ListenerId<T> {
        ListenerId {
            actual_id: listener.as_ref() as *const T as *const () as usize,
            phantom: Default::default(),
        }
    }
}

impl<T> ListenerId<T> {
    unsafe fn into_listener<U: ?Sized>(self, boxed: Box<U>) -> Box<T> {
        Box::from_raw(Box::into_raw(boxed) as *mut T)
    }
}

#[cfg(test)]
mod tests {
    use super::{PrecedenceStack, INLINE_PRECEDENCE_CAP};

    #[test]
    fn precedence_stack_inline_and_overflow_roundtrip() {
        let mut stack = PrecedenceStack::new();
        assert_eq!(stack.top(), 0);
        assert!(stack.overflow.is_empty());

        for depth in 1..INLINE_PRECEDENCE_CAP {
            stack.push(depth as i32 * 10);
            assert_eq!(stack.top(), depth as i32 * 10);
            assert!(stack.overflow.is_empty());
        }

        // Cross into overflow storage and grow several entries beyond inline capacity.
        let total = INLINE_PRECEDENCE_CAP + 5;
        for depth in INLINE_PRECEDENCE_CAP..total {
            stack.push(depth as i32 * 10);
            assert_eq!(stack.top(), depth as i32 * 10);
        }
        assert_eq!(stack.overflow.len(), 5);

        // Pop back across the overflow/inline boundary down to the initial 0 sentinel.
        for depth in (1..total).rev() {
            assert_eq!(stack.top(), depth as i32 * 10);
            stack.pop();
        }
        assert_eq!(stack.top(), 0);
        assert!(stack.overflow.is_empty());

        // Popping the sentinel yields -1; extra pops stay at -1.
        stack.pop();
        assert_eq!(stack.top(), -1);
        stack.pop();
        assert_eq!(stack.top(), -1);

        // Pushing after empty works cleanly.
        stack.push(42);
        assert_eq!(stack.top(), 42);
        stack.pop();
        assert_eq!(stack.top(), -1);
    }
}
