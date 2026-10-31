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
use crate::token::{OwningToken, Token, TOKEN_EOF, TOKEN_EPSILON};
use crate::token_factory::TokenFactory;
use crate::token_stream::TokenStream;
use crate::transition::RuleTransition;
use crate::tree::{NodeKindType, ParseTreeListener, Tree as _, TreeNode};
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

    fn get_current_context(&self) -> &'arena TreeNode<'input, 'arena, Self::Node, TF::Tok>;

    /// Current rule context, or `None` when the parser is not inside a rule.
    ///
    /// Java has no equivalent accessor: `ParserATNSimulator.adaptivePredict`
    /// takes the outer context as an explicit parameter, and the tool's
    /// interpreter tests pass `ParserRuleContext.EMPTY`. This accessor lets
    /// [crate::parser_atn_simulator::ParserATNSimulator::adaptive_predict]
    /// treat a missing context as Java's `ParserRuleContext.EMPTY`.
    fn get_current_context_opt(
        &self,
    ) -> Option<&'arena TreeNode<'input, 'arena, Self::Node, TF::Tok>> {
        Some(self.get_current_context())
    }

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

    /// Called when a prediction reads the rule invocation stack: a
    /// full-context simulation or a stack guard. A shared-descent group
    /// discards a neutral parse that did, because the stack of the neutral
    /// parse is the one of a single member.
    fn note_stack_dependent_prediction(&self) {}
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
pub struct NeutralParse<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    start_index: isize,
    mark: isize,
    /// The stream index where the shared rule starts, after the prefix.
    shared_index: isize,
    state: i32,
    ctx: Option<&'arena TreeNode<'input, 'arena, Node, Tok>>,
    children: usize,
    precedence_depth: usize,
    syntax_errors: i32,
    /// [`BaseParser::stack_dependent`] of the enclosing neutral parse.
    outer_stack_dependent: bool,
    matched_eof: bool,
    #[cfg(feature = "recursion-limit")]
    recursion_depth: u32,
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

    /// Rule context parser is currently processing
    ctx: *mut (),

    /// Track the {@link ParserRuleContext} objects during the parse and hook
    /// them up using the {@link ParserRuleContext#children} list so that it
    /// forms a parse tree. The {@link ParserRuleContext} returned from the start
    /// rule represents the root of the parse tree.
    ///
    /// <p>Note that if we are not building parse trees, rule contexts only point
    /// upwards. When a rule exits, it returns the context bute that gets garbage
    /// collected if nobody holds a reference. It points upwards but nobody
    /// points at it. </p>
    ///
    /// <p>When we build parse trees, we are adding all of these contexts to
    /// {@link ParserRuleContext#children} list. Contexts are then not candidates
    /// for garbage collection.</p>
    ///
    /// Returns {@code true} if a complete parse tree will be constructed while
    /// parsing, otherwise {@code false}
    pub build_parse_trees: bool,

    /// true if parser reached EOF
    pub matched_eof: bool,

    state: i32,
    /// Token stream that is currently used by this parser
    pub input: Input,
    precedence_stack: Vec<i32>,
    #[cfg(feature = "recursion-limit")]
    pub recursion_limit: u32,
    #[cfg(feature = "recursion-limit")]
    current_recursion_depth: u32,

    parse_listeners: Vec<Box<Node::Listener>>,
    _syntax_errors: Cell<i32>,
    error_listeners: Vec<ErrorListenerDelegate<'input, 'arena, Self, TF::Tok>>,
    /// Mutes the parse and error listeners while it is above zero. A counter,
    /// because the neutral parses of nested shared-descent groups nest (see
    /// [`Self::begin_neutral_parse`]).
    muted: u32,
    /// The pending re-descent of a shared-descent group (see
    /// [`Self::resume_neutral_parse`]).
    resume: Option<Resume<'input, 'arena, Node, TF::Tok>>,
    /// Whether a prediction of the current neutral parse read the rule
    /// invocation stack (see [`Parser::note_stack_dependent_prediction`]).
    stack_dependent: Cell<bool>,

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
            .field("build_parse_trees", &self.build_parse_trees)
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
    fn get_current_context(&self) -> &'arena TreeNode<'input, 'arena, Node, TF::Tok> {
        self.ctx().unwrap()
    }

    #[inline(always)]
    fn get_current_context_opt(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        self.ctx()
    }

    fn consume(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<(), ANTLRError> {
        let o = self.get_current_token();
        if o.borrow().get_token_type() != TOKEN_EOF {
            self.input.consume();
        }
        if self.build_parse_trees || !self.parse_listeners.is_empty() {
            if err_handler.in_error_recovery_mode(self) {
                // todo report ructc inference issue
                let node = self.create_error_node(o)?;
                self.add_child_to_ctx(node);
                if self.muted == 0 {
                    for listener in &mut self.parse_listeners {
                        listener.visit_error_node(
                            node.as_error_node()
                                .expect("node was created as error node"),
                        )?
                    }
                }
            } else {
                let node = self.create_token_node(o)?;
                self.add_child_to_ctx(node);
                if self.muted == 0 {
                    for listener in &mut self.parse_listeners {
                        listener.visit_terminal(
                            node.as_terminal_node()
                                .expect("node was created as terminal node"),
                        )?
                    }
                }
            }
        }
        Ok(())
    }

    fn precpred(
        &self,
        _localctx: Option<&TreeNode<'input, 'arena, Node, TF::Tok>>,
        precedence: i32,
    ) -> bool {
        //        localctx.map(|it|println!("check at{}",it.to_string_tree(self)));
        //        println!("{}",self.get_precedence());
        precedence >= self.get_precedence()
    }

    fn get_input_stream_mut(&mut self) -> &mut dyn TokenStream<'input, 'arena, TF> {
        &mut self.input //.as_mut()
    }

    fn get_input_stream(&self) -> &dyn TokenStream<'input, 'arena, TF> {
        &self.input
    }

    #[inline]
    fn get_current_token(&self) -> &'arena TF::Tok {
        self.input.get(self.input.index())
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
        self.error_listeners
            .push(ErrorListenerDelegate::new(listener))
    }

    fn remove_error_listeners(&mut self) {
        self.error_listeners.clear();
    }

    fn notify_error_listeners(
        &self,
        msg: String,
        offending_token: Option<isize>,
        err: Option<&ANTLRError>,
    ) {
        cell_update(&self._syntax_errors, |it| it + 1);
        if self.muted > 0 {
            return;
        }
        let offending_token: Option<&_> = match offending_token {
            None => Some(self.get_current_token().borrow()),
            Some(x) => Some(self.input.get(x).borrow()),
        };
        let line = offending_token.map_or(0, |x| x.get_line());
        let column = offending_token.map_or(-1, |x| x.get_char_position_in_line());

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
        // The ambiguity reports of a muted neutral parse are muted too.
        let delegates: &[_] = if self.muted > 0 {
            &[]
        } else {
            self.error_listeners.borrow()
        };
        Box::new(ProxyErrorListener { delegates })
    }

    fn is_expected_token(&self, _symbol: i32) -> bool {
        unimplemented!()
    }

    fn note_stack_dependent_prediction(&self) {
        if self.muted > 0 {
            self.stack_dependent.set(true);
        }
    }

    fn get_precedence(&self) -> i32 {
        *self.precedence_stack.last().unwrap_or(&-1)
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
        let mut ctx = self.get_current_context();
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

    //    fn get_rule_invocation_stack(&self, c: _) -> Vec<String> {
    //        unimplemented!()
    //    }
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
            build_parse_trees: true,
            matched_eof: false,
            state: -1,
            input,
            precedence_stack: vec![0],
            #[cfg(feature = "recursion-limit")]
            recursion_limit: DEFAULT_RECURSION_LIMIT,
            #[cfg(feature = "recursion-limit")]
            current_recursion_depth: 0,
            parse_listeners: vec![],
            _syntax_errors: Cell::new(0),
            error_listeners: vec![ErrorListenerDelegate::new(
                Box::new(ConsoleErrorListener {})
                    as Box<dyn ErrorListener<'input, 'arena, Self, TF::Tok> + 'input>,
            )],
            muted: 0,
            resume: None,
            stack_dependent: Cell::new(false),
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
    /// The walk consumes tokens while it reads them and rewinds afterwards, so
    /// every step costs one `la(1)` and one `consume`. It must not read
    /// `la(k)`: a channel-filtering token stream resolves `la(k)` by scanning
    /// from the current token, which would make a k-token lookahead O(k^2).
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
    fn dfa_walk(&mut self, decision: i32) -> Result<i32, ANTLRError> {
        use crate::dfa::static_parser_dfa::{CONFLICT_ESCAPE, ESCAPE, GUARDED};

        let tables = self.get_interpreter().atn().static_dfas();
        if tables.is_empty() {
            return Ok(crate::atn::INVALID_ALT);
        }
        let Some(table) = tables.table(decision, self.get_precedence()) else {
            return Ok(crate::atn::INVALID_ALT);
        };
        let start_index = self.input.index();
        let mark = self.input.mark();
        let mut state = 0_i32;
        let result = loop {
            let accept = table.accept(state);
            if accept > 0 {
                self.input.seek(start_index);
                break Ok(accept);
            }
            if accept == CONFLICT_ESCAPE {
                self.input.seek(start_index);
                break match table.conflict_min_alt(state) {
                    Some(min_alt) => self
                        .get_interpreter()
                        .adaptive_predict_conflict(decision, min_alt, self),
                    None => Ok(crate::atn::INVALID_ALT),
                };
            }
            if accept == GUARDED {
                self.note_stack_dependent_prediction();
                // Guarded take of an optional-postfix decision: resolve to the
                // guarded alternative unless the real parse stack's epsilon-pop
                // chase from the decision's block end can reach a guard root.
                // The walk stops at the first invoking state whose follow
                // region cannot reach its own rule's stop.
                let mut defers = false;
                let mut context = self.ctx();
                while let Some(node) = context {
                    let invoking_state = node.get_invoking_state();
                    if invoking_state < 0 {
                        break;
                    }
                    match table.guard_step(invoking_state) {
                        Some(true) => {
                            defers = true;
                            break;
                        }
                        Some(false) => break,
                        None => context = node.get_parent(),
                    }
                }
                self.input.seek(start_index);
                break if defers {
                    Ok(crate::atn::INVALID_ALT)
                } else {
                    Ok(table.guarded_alt(state).unwrap_or(crate::atn::INVALID_ALT))
                };
            }
            if accept == ESCAPE {
                // A frontier continuation resumes from the position the walk
                // reached, so it must not rewind first.
                break match table.continuation(state) {
                    Some((slot, continuation)) => self.get_interpreter().resume_from_continuation(
                        decision,
                        slot,
                        table.num_continuations(),
                        continuation,
                        start_index,
                        self,
                    ),
                    None => {
                        self.input.seek(start_index);
                        Ok(crate::atn::INVALID_ALT)
                    }
                };
            }
            let token = self.input.la(1);
            match table.edge(state, token) {
                Some(target) => {
                    state = target;
                    if token != TOKEN_EOF {
                        self.input.consume();
                    }
                }
                None => {
                    if let Some(fallback) = table.fallback(state) {
                        self.input.seek(start_index);
                        break Ok(fallback);
                    }
                    let start = OwningToken::from(self.input.get(start_index) as &dyn Token);
                    let error = match self.input.lt(1) {
                        Some(offending) => ANTLRError::no_alt_full(
                            self,
                            start,
                            OwningToken::from(offending as &dyn Token),
                        ),
                        None => ANTLRError::no_alt(self),
                    };
                    self.input.seek(start_index);
                    break Err(error);
                }
            }
        };
        self.input.release(mark);
        result
    }

    /// Whether `token` can follow the current rule on the real parse stack.
    ///
    /// Walks the context frames upward. At each frame, the token follows when
    /// it starts the caller's continuation after the frame's call site; the
    /// walk goes on to the next frame only while that continuation can
    /// complete without a token. Above the outermost frame only EOF follows.
    /// The guarded tail arm of a shared-descent group commits only when this
    /// is `false`, because the group's nullable member is then dead.
    pub fn follow_contains(&self, token: i32) -> bool {
        self.follow_contains_at(self.ctx(), token)
    }

    /// [`Self::follow_contains`] for the stack whose innermost frame is
    /// `context`.
    #[doc(hidden)]
    pub fn follow_contains_at(
        &self,
        mut context: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
        token: i32,
    ) -> bool {
        self.note_stack_dependent_prediction();
        let atn = self.get_interpreter().atn();
        while let Some(node) = context {
            let invoking_state = node.get_invoking_state();
            if invoking_state < 0 {
                break;
            }
            let Some(call) = atn
                .get_state(invoking_state)
                .get_transitions()
                .first()
                .and_then(|transition| transition.try_as::<RuleTransition>())
            else {
                // Not a call site: assume the token can follow.
                return true;
            };
            let continuation = atn.next_tokens::<TF::Tok>(&call.follow_state);
            if continuation.contains(token) {
                return true;
            }
            if !continuation.contains(TOKEN_EPSILON) {
                return false;
            }
            context = node.get_parent();
        }
        token == TOKEN_EOF
    }

    /// Whether a shared-descent group can start a neutral parse here.
    ///
    /// A group cannot start while the re-descent of another group is pending,
    /// and not while parse listeners are attached without a parse tree: the
    /// muted listener events of the shared rule are replayed from its subtree.
    ///
    /// A group also cannot start in the SLL prediction mode. For an ambiguous
    /// input, SLL resolves the conflict of the decision to its minimum
    /// alternative, but the tail dispatch after the shared rule can select
    /// another member.
    #[doc(hidden)]
    pub fn descent_ready(&mut self) -> bool {
        self.drop_stale_resume();
        self.resume.is_none()
            && (self.build_parse_trees || self.parse_listeners.is_empty())
            && self.get_interpreter().get_prediction_mode() != PredictionMode::SLL
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
    /// calls the shared rule once, with every listener muted, to read the
    /// token after it.
    ///
    /// Skips the `prefix_len` leading tokens of the group (the table walk read
    /// them) and sets the call-site state, so the shared rule's context has a
    /// real invoking state. Finish with [`Self::end_neutral_parse`] to discard
    /// the parse, or with [`Self::resume_neutral_parse`] to keep its subtree
    /// for the re-descent.
    #[doc(hidden)]
    pub fn begin_neutral_parse(
        &mut self,
        call_site: i32,
        prefix_len: usize,
    ) -> NeutralParse<'input, 'arena, Node, TF::Tok> {
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
            ctx,
            children: ctx.map_or(0, |ctx| ctx.get_child_count()),
            precedence_depth: self.precedence_stack.len(),
            syntax_errors: self._syntax_errors.get(),
            outer_stack_dependent: self.muted > 0 && self.stack_dependent.get(),
            matched_eof: self.matched_eof,
            #[cfg(feature = "recursion-limit")]
            recursion_depth: self.current_recursion_depth,
        };
        self.stack_dependent.set(false);
        self.muted += 1;
        self.state = call_site;
        neutral
    }

    /// Whether the subtree of the neutral parse is the one every member would
    /// parse: the parse reported no syntax error, and no prediction in it read
    /// the rule invocation stack, which holds the call site of one member only.
    #[doc(hidden)]
    pub fn neutral_parse_usable(
        &self,
        neutral: &NeutralParse<'input, 'arena, Node, TF::Tok>,
    ) -> bool {
        self._syntax_errors.get() == neutral.syntax_errors && !self.stack_dependent.get()
    }

    /// Discards a neutral parse: rewinds the stream to the decision start and
    /// restores the parser state of [`Self::begin_neutral_parse`].
    #[doc(hidden)]
    pub fn end_neutral_parse(&mut self, neutral: NeutralParse<'input, 'arena, Node, TF::Tok>) {
        self.restore_neutral_state(&neutral);
        self.input.release(neutral.mark);
    }

    /// Finishes a neutral parse and keeps the subtree `node` of the shared rule
    /// `rule` for the re-descent.
    ///
    /// Rewinds the stream to the decision start like
    /// [`Self::end_neutral_parse`]. The member alternative then re-descends
    /// through its prefix to its call of the shared rule, where
    /// [`Self::resume_take`] returns `node` instead of parsing it again.
    #[doc(hidden)]
    pub fn resume_neutral_parse(
        &mut self,
        neutral: NeutralParse<'input, 'arena, Node, TF::Tok>,
        node: &'arena TreeNode<'input, 'arena, Node, TF::Tok>,
        rule: usize,
    ) {
        // The mark stays until the re-descent seeks forward to the tail.
        self.resume = Some(Resume {
            node,
            rule,
            shared_index: neutral.shared_index,
            tail_index: self.input.index(),
            matched_eof: self.matched_eof,
            mark: neutral.mark,
        });
        self.restore_neutral_state(&neutral);
    }

    fn restore_neutral_state(&mut self, neutral: &NeutralParse<'input, 'arena, Node, TF::Tok>) {
        debug_assert!(self.muted > 0, "no neutral parse is active");
        self.muted -= 1;
        self.input.seek(neutral.start_index);
        self.set_current_ctx(neutral.ctx);
        // Truncate rather than pop: a left-recursive shared rule adds its
        // context to the parent only when it unrolls, so an error can leave
        // no child behind.
        if let Some(ctx) = unsafe { self.ctx_mut() } {
            ctx.truncate_children(neutral.children);
        }
        self.precedence_stack.truncate(neutral.precedence_depth);
        self.state = neutral.state;
        self.matched_eof = neutral.matched_eof;
        // The predictions of this neutral parse belong to the enclosing one.
        self.stack_dependent
            .set(neutral.outer_stack_dependent || self.stack_dependent.get());
        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth = neutral.recursion_depth;
        }
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
        if self.build_parse_trees {
            self.add_child_to_ctx(node);
        }
        // The neutral parse created the context under the decision's context
        // at the time; the re-descent's context and call site may differ.
        self.set_current_ctx(Some(node));
        self.with_mut_ctx(|ctx| {
            ctx.set_parent(parent);
            ctx.set_invoking_state(state);
        });
        self.set_current_ctx(parent);
        if self.muted == 0 && !self.parse_listeners.is_empty() {
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

    #[inline]
    fn parent_ctx(&self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        self.ctx().and_then(|it| it.get_parent())
    }

    #[inline]
    fn set_current_ctx(&mut self, ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>) {
        if let Some(ctx) = ctx {
            self.ctx = ctx as *const TreeNode<'input, 'arena, Node, TF::Tok>
                as *mut TreeNode<'input, 'arena, Node, TF::Tok> as *mut ();
        } else {
            self.ctx = std::ptr::null_mut();
        }
    }

    pub fn take_ctx(&mut self) -> Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>> {
        if self.ctx.is_null() {
            None
        } else {
            let ret = unsafe { &*(self.ctx as *const TreeNode<'input, 'arena, Node, TF::Tok>) };
            self.ctx = std::ptr::null_mut();
            Some(ret)
        }
    }

    #[inline]
    fn add_child_to_ctx(&mut self, child: &'arena TreeNode<'input, 'arena, Node, TF::Tok>) {
        if !self.ctx.is_null() {
            unsafe {
                (*(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)).add_child(child);
            }
        }
    }

    #[inline]
    pub fn with_mut_ctx<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut TreeNode<'input, 'arena, Node, TF::Tok>) -> R,
    {
        assert!(!self.ctx.is_null());
        unsafe { f(&mut *(self.ctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)) }
    }

    #[inline]
    pub fn match_token(
        &mut self,
        ttype: i32,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        let mut token = self.get_current_token();
        if token.get_token_type() == ttype {
            if ttype == TOKEN_EOF {
                self.matched_eof = true;
            }

            err_handler.report_match(self);
            self.consume(err_handler)?;
        } else {
            token = err_handler.recover_inline(self)?;
            if self.build_parse_trees && token.get_token_index() == -1 {
                self.add_child_to_ctx(self.create_error_node(token)?);
            }
        }
        Ok(token)
    }

    #[inline]
    pub fn match_wildcard(
        &mut self,
        err_handler: &mut impl ErrorStrategy<'input, 'arena, TF, Self>,
    ) -> Result<&'arena TF::Tok, ANTLRError> {
        let mut token = self.get_current_token();
        if token.get_token_type() > 0 {
            err_handler.report_match(self);
            self.consume(err_handler)?;
        } else {
            token = err_handler.recover_inline(self)?;
            if self.build_parse_trees && token.get_token_index() == -1 {
                self.add_child_to_ctx(self.create_error_node(token)?);
            }
        }
        Ok(token)
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_dyn_parse_listener(&mut self, listener: Box<Node::Listener>) {
        self.parse_listeners.push(listener);
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
        unsafe { listener_id.into_listener(self.parse_listeners.remove(index)) }
    }

    /// Removes all added parse listeners without returning them
    pub fn remove_parse_listeners(&mut self) {
        self.parse_listeners.clear()
    }

    pub fn trigger_enter_rule_event(&mut self) -> Result<(), ANTLRError> {
        if self.muted > 0 {
            return Ok(());
        }
        let ctx = self.ctx().unwrap();
        for listener in self.parse_listeners.iter_mut() {
            listener.enter_every_rule(ctx)?;
            ctx.enter_rule(listener)?;
        }
        Ok(())
    }

    pub fn trigger_exit_rule_event(&mut self) -> Result<(), ANTLRError> {
        if self.muted > 0 {
            return Ok(());
        }
        let ctx = self.ctx().unwrap();
        for listener in self.parse_listeners.iter_mut().rev() {
            ctx.exit_rule(listener)?;
            listener.exit_every_rule(ctx)?;
        }
        Ok(())
    }

    //    fn get_atn_with_bypass_alts(&self) { unimplemented!() }
    //
    //    fn compile_parse_tree_pattern(&self, pattern, patternRuleIndex: Lexer, lexer: Lexer) { unimplemented!() }

    #[inline]
    pub fn enter_rule(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
    ) -> Result<(), ANTLRError> {
        let child = localctx;
        if self.build_parse_trees {
            self.set_current_ctx(child.get_parent());
            self.add_child_to_ctx(child);
        }

        self.set_state(state);
        self.set_current_ctx(Some(child));
        let start = self.input.lt(1);
        self.with_mut_ctx(|ctx| {
            ctx.set_start(start);
        });

        if !self.parse_listeners.is_empty() {
            self.trigger_enter_rule_event()?;
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
        assert!(self.ctx().is_some());

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth -= 1;
        }
        if self.matched_eof {
            // if we have matched EOF, it cannot consume past EOF so we use LT(1) here
            let stop = self.input.lt(1);
            self.with_mut_ctx(|ctx| {
                ctx.set_stop(stop);
            });
        } else {
            // stop node is what we just matched
            let stop = self.input.lt(-1);
            self.with_mut_ctx(|ctx| {
                ctx.set_stop(stop);
            });
        }
        if !self.parse_listeners.is_empty() {
            self.trigger_exit_rule_event()?;
        }

        self.set_state(self.ctx().unwrap().get_invoking_state());
        let child = self.ctx().unwrap();
        self.set_current_ctx(child.get_parent());

        Ok(child)
    }

    pub fn enter_recursion_rule(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
        precedence: i32,
    ) -> Result<(), ANTLRError> {
        self.set_state(state);
        self.precedence_stack.push(precedence);
        self.set_current_ctx(Some(localctx));
        let start = self.input.lt(1);
        self.with_mut_ctx(|ctx| {
            ctx.set_start(start);
        });
        if !self.parse_listeners.is_empty() {
            self.trigger_enter_rule_event()?;
        }
        //println!("{}",self.input.lt(1).map(Token::to_owned).unwrap());

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth += 1;
            if self.current_recursion_depth > self.recursion_limit {
                return Err(ANTLRError::recursion_limit_exceeded(self.recursion_limit));
            }
        }
        Ok(())
    }

    pub fn push_new_recursion_context(
        &mut self,
        localctx: &'arena mut TreeNode<'input, 'arena, Node, TF::Tok>,
        state: i32,
        _rule_index: usize,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        let stop = self.input.lt(-1);
        self.with_mut_ctx(|ctx| {
            ctx.set_parent(Some(localctx));
            ctx.set_invoking_state(state);
            ctx.set_stop(stop);
        });

        let prev = self.take_ctx().unwrap();
        self.set_current_ctx(Some(localctx));
        let start = prev.get_start_token();
        self.with_mut_ctx(|ctx| {
            ctx.set_start(start);
        });
        if self.build_parse_trees {
            self.add_child_to_ctx(prev);
        }
        if !self.parse_listeners.is_empty() {
            self.trigger_enter_rule_event()?;
        }
        Ok(prev)
    }

    pub fn unroll_recursion_context(
        &mut self,
        parent_ctx: Option<&'arena TreeNode<'input, 'arena, Node, TF::Tok>>,
    ) -> Result<&'arena TreeNode<'input, 'arena, Node, TF::Tok>, ANTLRError> {
        assert!(self.ctx().is_some());

        #[cfg(feature = "recursion-limit")]
        {
            self.current_recursion_depth -= 1;
        }
        self.precedence_stack.pop();
        let stop = self.input.lt(-1);
        self.with_mut_ctx(|ctx| {
            ctx.set_stop(stop);
        });
        let retctx = self.ctx;

        // unroll so _ctx is as it was before call to recursive method
        if !self.parse_listeners.is_empty() {
            while !self.ctx_is(parent_ctx) {
                self.trigger_exit_rule_event()?;
                self.set_current_ctx(self.parent_ctx());
            }
        } else {
            self.set_current_ctx(parent_ctx);
        }

        // hook into tree
        unsafe {
            (*(retctx as *mut TreeNode<'input, 'arena, Node, TF::Tok>)).set_parent(parent_ctx);
        }

        //        println!("{:?}",self.ctx.as_ref().map(|it|it.to_string_tree(self)));
        if self.build_parse_trees && parent_ctx.is_some() {
            self.add_child_to_ctx(unsafe {
                &*(retctx as *const TreeNode<'input, 'arena, Node, TF::Tok>)
            });
        }
        Ok(unsafe { &*(retctx as *const TreeNode<'input, 'arena, Node, TF::Tok>) })
    }

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
            Err(ANTLRError::dfa_cache_limit_exceeded(0, 0, 0))
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
