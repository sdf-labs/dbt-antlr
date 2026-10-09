//! A parser simulator that mimics what ANTLR's generated parser code does.
//!
//! Rust port of Java's `org.antlr.v4.runtime.ParserInterpreter`.
//!
//! A [`ParserATNSimulator`](crate::parser_atn_simulator::ParserATNSimulator)
//! is used to make predictions via `adaptive_predict`, but this type moves a
//! pointer through the ATN to simulate parsing. The simulator just makes us
//! efficient rather than having to backtrack, for example.
//!
//! This properly creates parse trees even for left recursive rules.
//!
//! We rely on the left recursive rule invocation and special predicate
//! transitions to make left recursive rules work.
use std::ops::{Deref, DerefMut};

use std::fmt::{Debug, Formatter};

use crate::arena::Arena;
use crate::atn::ATN;
use crate::atn_simulator::ParserATNSimulatorManager;
use crate::atn_state::{
    ATNState, ATNStateType, RuleStartState, StarLoopEntryState, ATNSTATE_INVALID_STATE_NUMBER,
};
use crate::char_stream::CharStream;
use crate::error_strategy::{DefaultErrorStrategy, ErrorStrategy, ErrorStrategyDelegate};
use crate::errors::{ANTLRError, ANTLRErrorKind};
use crate::interpreter_rule_context::{
    InterpreterNode, InterpreterNodeKind, InterpreterRuleContextExt,
};
use crate::parser::{BaseParser, Parser, ParserRecog};
use crate::parser_rule_context::BaseParserRuleContext;
use crate::recognizer::{Actions, Recognizer};
use crate::rule_context::RuleContext as _;
use crate::token::{Token, TOKEN_DEFAULT_CHANNEL, TOKEN_INVALID_TYPE, TOKEN_MIN_USER_TOKEN_TYPE};
use crate::token_factory::TokenFactory;
use crate::token_stream::TokenStream;
use crate::transition::{
    ActionTransition, AtomTransition, PrecedencePredicateTransition, PredicateTransition,
    RuleTransition, TransitionType,
};
use crate::tree::TreeNode;
use crate::vocabulary::Vocabulary;

/// Base parser type the parser interpreter is built on.
pub type ParserInterpreterBase<'input, 'arena, Input, TF> =
    BaseParser<'input, 'arena, ParserInterpreterExt, InterpreterNodeKind, Input, TF>;

/// Recognizer data of a [ParserInterpreter].
///
/// Unlike generated parsers, which compile this data into statics, the
/// interpreter receives it at construction time. The ATN and the ATN
/// simulator manager must live for `'static` because of the runtime's
/// simulator manager design, so they are leaked on construction; each
/// interpreter instance gets its own (not shared) decision DFAs, matching the
/// Java runtime.
pub struct ParserInterpreterExt {
    grammar_file_name: &'static str,
    rule_names: &'static [&'static str],
    vocabulary: Box<dyn Vocabulary>,
    atn: &'static ATN,
    atn_manager: &'static ParserATNSimulatorManager,
}

impl Debug for ParserInterpreterExt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserInterpreterExt")
            .field("grammar_file_name", &self.grammar_file_name)
            .field("rule_names", &self.rule_names)
            .field("vocabulary", &self.vocabulary)
            .field("atn", &self.atn)
            .finish_non_exhaustive()
    }
}

impl<'input, 'arena, Input, TF>
    Actions<'input, 'arena, ParserInterpreterBase<'input, 'arena, Input, TF>, TF::Tok>
    for ParserInterpreterExt
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    // `sempred` and `action` intentionally keep the default implementations
    // (return true / do nothing): the interpreter cannot run target code,
    // exactly like Java's ParserInterpreter which inherits the no-op
    // `Recognizer.sempred`/`Recognizer.action`.

    fn get_grammar_file_name(&self) -> &str {
        self.grammar_file_name
    }

    fn get_rule_names(&self) -> &[&str] {
        self.rule_names
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &*self.vocabulary
    }
}

impl<'input, 'arena, Input, TF>
    ParserRecog<'input, 'arena, ParserInterpreterBase<'input, 'arena, Input, TF>, TF::Tok>
    for ParserInterpreterExt
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn get_atn_simulator_man(&self) -> &'static ParserATNSimulatorManager {
        self.atn_manager
    }
}

/// A parser that interprets a deserialized ATN at runtime instead of executing
/// generated code.
///
/// See Java's `ParserInterpreter` for the reference semantics; this port
/// follows them 1:1, including left-recursion handling, decision overrides and
/// the "all predicates pass" convention (the interpreter cannot run target
/// code, so `sempred` always returns true while `precpred` checks the
/// precedence stack for real).
pub struct ParserInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    base: ParserInterpreterBase<'input, 'arena, Input, TF>,
    err_handler:
        ErrorStrategyDelegate<'input, 'arena, TF, ParserInterpreterBase<'input, 'arena, Input, TF>>,

    /// This stack corresponds to the `_parentctx`, `_parentState` pair of
    /// locals that would exist on call stack frames with a recursive descent
    /// parser; in the generated function for a left-recursive rule you'd see:
    ///
    /// ```ignore
    /// fn e(&mut self, _p: i32) {
    ///     let _parentctx = self.take_ctx();   // pair.0
    ///     let _parentState = self.get_state(); // pair.1
    ///     ...
    /// }
    /// ```
    ///
    /// Those values are used to create new recursive rule invocation contexts
    /// associated with the left operand of an alt like `expr '*' expr`.
    parent_context_stack: Vec<(
        Option<&'arena InterpreterNode<'input, 'arena, TF::Tok>>,
        i32,
    )>,

    /// We need a map from (decision, inputIndex) -> forced alt for computing
    /// ambiguous parse trees. For now, we allow exactly one override.
    override_decision: i32,
    override_decision_input_index: isize,
    override_decision_alt: i32,
    /// latch and only override once; error might trigger infinite loop
    override_decision_reached: bool,

    /// What is the current context when we override a decision? This tells
    /// us what the root of the parse tree is when using override
    /// for an ambiguity/lookahead check.
    override_decision_root: Option<&'arena InterpreterNode<'input, 'arena, TF::Tok>>,

    root_context: Option<&'arena InterpreterNode<'input, 'arena, TF::Tok>>,

    /// Opt-in outer alternative number tracking, mirroring the ANTLR tool's
    /// `GrammarParserInterpreter`: when enabled, [ParserInterpreter::visit_decision_state]
    /// records the predicted (rewritten) alternative number on the current
    /// rule context at each state of [ParserInterpreter::outer_alt_decision_states].
    /// Java's runtime `ParserInterpreter` never sets alt numbers, so this is
    /// off by default.
    track_alt_numbers: bool,

    /// State numbers where [GrammarParserInterpreter](https://github.com/antlr/antlr4/blob/master/tool/src/org/antlr/v4/tool/GrammarParserInterpreter.java)'s
    /// `findOuterMostDecisionStates` would record the outer alternative:
    /// the outermost block of every rule and the star block holding the
    /// recursive alternatives of a left-recursive rule. Computed lazily when
    /// [ParserInterpreter::set_track_alt_numbers] is enabled.
    outer_alt_decision_states: Option<Vec<bool>>,
}

impl<'input, 'arena, Input, TF> Debug for ParserInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParserInterpreter").finish_non_exhaustive()
    }
}

impl<'input, 'arena, Input, TF> Deref for ParserInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    type Target = ParserInterpreterBase<'input, 'arena, Input, TF>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, 'arena, Input, TF> DerefMut for ParserInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, 'arena, Input, TF> ParserInterpreter<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    /// Creates a new parser interpreter with the given error strategy.
    ///
    /// `atn` is a ready (already deserialized) parser ATN; it is moved onto
    /// the heap and leaked because the runtime's ATN simulator machinery
    /// requires `&'static ATN`. Metadata (`grammar_file_name`, `rule_names`)
    /// must be `&'static` for the same reason; leak dynamically built strings
    /// with [Box::leak] if needed.
    pub fn with_strategy(
        arena: &'arena Arena,
        grammar_file_name: &'static str,
        vocabulary: Box<dyn Vocabulary>,
        rule_names: &'static [&'static str],
        atn: ATN,
        input: Input,
        strategy: Box<
            dyn ErrorStrategy<'input, 'arena, TF, ParserInterpreterBase<'input, 'arena, Input, TF>>
                + 'arena,
        >,
    ) -> Self {
        let atn: &'static ATN = Box::leak(Box::new(atn));
        let atn_manager: &'static ParserATNSimulatorManager =
            Box::leak(Box::new(ParserATNSimulatorManager::new(atn)));
        Self {
            base: BaseParser::new_base_parser(
                arena,
                input,
                ParserInterpreterExt {
                    grammar_file_name,
                    rule_names,
                    vocabulary,
                    atn,
                    atn_manager,
                },
            ),
            err_handler: unsafe { ErrorStrategyDelegate::new(strategy) },
            parent_context_stack: Vec::new(),
            override_decision: -1,
            override_decision_input_index: -1,
            override_decision_alt: -1,
            override_decision_reached: false,
            override_decision_root: None,
            root_context: None,
            track_alt_numbers: false,
            outer_alt_decision_states: None,
        }
    }

    /// Creates a new parser interpreter with [DefaultErrorStrategy].
    ///
    /// See [ParserInterpreter::with_strategy] for the parameter documentation.
    pub fn new(
        arena: &'arena Arena,
        grammar_file_name: &'static str,
        vocabulary: Box<dyn Vocabulary>,
        rule_names: &'static [&'static str],
        atn: ATN,
        input: Input,
    ) -> Self {
        Self::with_strategy(
            arena,
            grammar_file_name,
            vocabulary,
            rule_names,
            atn,
            input,
            Box::new(DefaultErrorStrategy::new()),
        )
    }

    /// Replaces the error strategy of this interpreter.
    pub fn set_error_strategy(
        &mut self,
        strategy: Box<
            dyn ErrorStrategy<'input, 'arena, TF, ParserInterpreterBase<'input, 'arena, Input, TF>>
                + 'arena,
        >,
    ) {
        self.err_handler = unsafe { ErrorStrategyDelegate::new(strategy) };
    }

    /// Begin parsing at `start_rule_index`
    pub fn parse(
        &mut self,
        start_rule_index: usize,
    ) -> Result<&'arena InterpreterNode<'input, 'arena, TF::Tok>, ANTLRError> {
        let atn = self.base.atn;
        let start_rule_start_state = atn.rule_to_start_state[start_rule_index];
        let is_left_recursive = is_left_recursive_rule(start_rule_start_state.as_ref());

        let root_context = self.create_interpreter_rule_context(
            None,
            ATNSTATE_INVALID_STATE_NUMBER,
            start_rule_index,
        )?;
        // Keep a second reference to the root node; the arena allocation
        // outlives the parser, and the raw pointer read does not lock the
        // `&'arena mut` used below.
        self.root_context = Some(unsafe { &*(root_context as *const _) });
        if is_left_recursive {
            self.enter_recursion_rule(
                root_context,
                start_rule_start_state.get_state_number(),
                start_rule_index,
                0,
            )?;
        } else {
            self.base.enter_rule(
                root_context,
                start_rule_start_state.get_state_number(),
                start_rule_index,
            )?;
        }

        loop {
            let p = atn.get_state(self.base.get_state());
            match p.state_type() {
                ATNStateType::RuleStop => {
                    // pop; return from rule
                    let ctx = self.base.ctx().expect("parser is inside a rule");
                    if ctx.is_empty() {
                        if is_left_recursive {
                            let result = ctx;
                            let (parent_ctx, _) = self
                                .parent_context_stack
                                .pop()
                                .expect("left-recursive start rule has a parent context");
                            self.base.unroll_recursion_context(parent_ctx)?;
                            return Ok(result);
                        } else {
                            self.base.exit_rule()?;
                            return Ok(self.root_context.expect("root context was created"));
                        }
                    }

                    self.visit_rule_stop_state(p)?;
                }
                _ => {
                    match self.visit_state(p) {
                        Ok(()) => {}
                        Err(e) if !e.is_recoverable() => return Err(e),
                        Err(e) => {
                            self.base.set_state(
                                atn.rule_to_stop_state[p.get_rule_index() as usize]
                                    .get_state_number(),
                            );
                            // `ctx.exception = e` is a no-op in this runtime
                            self.err_handler.report_error(&mut self.base, &e);
                            self.recover(&e)?;
                        }
                    }
                }
            }
        }
    }

    /// Interpreting counterpart of [BaseParser::enter_recursion_rule]:
    /// additionally tracks the `_parentctx`/`_parentState` pair on
    /// [ParserInterpreter::parent_context_stack].
    pub fn enter_recursion_rule(
        &mut self,
        localctx: &'arena mut InterpreterNode<'input, 'arena, TF::Tok>,
        state: i32,
        rule_index: usize,
        precedence: i32,
    ) -> Result<(), ANTLRError> {
        let pair = (self.base.ctx(), localctx.get_invoking_state());
        self.parent_context_stack.push(pair);
        self.base
            .enter_recursion_rule(localctx, state, rule_index, precedence)
    }

    fn visit_state(&mut self, p: &'static ATNState) -> Result<(), ANTLRError> {
        let atn = self.base.atn;
        let mut predicted_alt = 1;
        if p.is_decision_state() {
            predicted_alt = self.visit_decision_state(p)?;
        }

        let transition = &p.get_transitions()[(predicted_alt - 1) as usize];
        match transition.transition_type() {
            TransitionType::Epsilon => {
                if p.state_type() == ATNStateType::StarLoopEntry
                    && p.try_as::<StarLoopEntryState>()
                        .is_some_and(|state| state.is_precedence)
                    && transition.get_target().state_type() != ATNStateType::LoopEnd
                {
                    // We are at the start of a left recursive rule's (...)* loop
                    // and we're not taking the exit branch of loop.
                    let (parent_ctx, parent_state) = *self
                        .parent_context_stack
                        .last()
                        .expect("left-recursive rule has a parent context");
                    let rule_index = self
                        .base
                        .ctx()
                        .expect("parser is inside a rule")
                        .get_rule_index();
                    let localctx =
                        self.create_interpreter_rule_context(parent_ctx, parent_state, rule_index)?;
                    self.base.push_new_recursion_context(
                        localctx,
                        atn.rule_to_start_state[p.get_rule_index() as usize].get_state_number(),
                        rule_index,
                    )?;
                }
            }
            TransitionType::Atom => {
                let label = transition
                    .try_as::<AtomTransition>()
                    .expect("atom transition")
                    .label();
                self.base.match_token(label, &mut self.err_handler)?;
            }
            TransitionType::Range | TransitionType::Set | TransitionType::NotSet => {
                if !transition.matches(
                    self.base.get_input_stream_mut().la(1),
                    TOKEN_MIN_USER_TOKEN_TYPE,
                    65535,
                ) {
                    self.recover_inline()?;
                }
                self.base.match_wildcard(&mut self.err_handler)?;
            }
            TransitionType::Wildcard => {
                self.base.match_wildcard(&mut self.err_handler)?;
            }
            TransitionType::Rule => {
                let rule_transition = transition
                    .try_as::<RuleTransition>()
                    .expect("rule transition");
                let rule_start_state = transition.get_target();
                let rule_index = rule_start_state.get_rule_index() as usize;
                let newctx = self.create_interpreter_rule_context(
                    self.base.ctx(),
                    p.get_state_number(),
                    rule_index,
                )?;
                if is_left_recursive_rule(&rule_start_state) {
                    self.enter_recursion_rule(
                        newctx,
                        rule_start_state.get_state_number(),
                        rule_index,
                        rule_transition.precedence(),
                    )?;
                } else {
                    self.base.enter_rule(
                        newctx,
                        rule_start_state.get_state_number(),
                        rule_index,
                    )?;
                }
            }
            TransitionType::Predicate => {
                let predicate_transition = transition
                    .try_as::<PredicateTransition>()
                    .expect("predicate transition");
                let ctx = self.base.ctx();
                if !self.base.sempred(
                    ctx,
                    predicate_transition.rule_index(),
                    predicate_transition.pred_index(),
                ) {
                    return Err(ANTLRError::failed_predicate(&mut self.base, None, None));
                }
            }
            TransitionType::Action => {
                let action_transition = transition
                    .try_as::<ActionTransition>()
                    .expect("action transition");
                let ctx = self.base.ctx();
                self.base.action(
                    ctx,
                    action_transition.rule_index(),
                    action_transition.action_index(),
                );
            }
            TransitionType::PrecedencePredicate => {
                let precedence = transition
                    .try_as::<PrecedencePredicateTransition>()
                    .expect("precedence predicate transition")
                    .precedence();
                let ctx = self.base.ctx();
                if !self.base.precpred(ctx, precedence) {
                    return Err(ANTLRError::failed_predicate(
                        &mut self.base,
                        Some(format!("precpred(_ctx, {})", precedence)),
                        None,
                    ));
                }
            }
        }

        self.base
            .set_state(transition.get_target().get_state_number());
        Ok(())
    }

    /// Called when the interpreter reaches a decision state.
    ///
    /// Mirrors Java's `ParserInterpreter.visitDecisionState`, including the
    /// decision override check.
    fn visit_decision_state(&mut self, p: &'static ATNState) -> Result<i32, ANTLRError> {
        let mut predicted_alt = 1;
        if p.get_transitions().len() > 1 {
            self.err_handler.sync(&mut self.base)?;
            let decision = p.get_decision().expect("decision state has a decision");
            if decision == self.override_decision
                && self.base.get_input_stream().index() == self.override_decision_input_index
                && !self.override_decision_reached
            {
                predicted_alt = self.override_decision_alt;
                self.override_decision_reached = true;
            } else {
                predicted_alt = self
                    .base
                    .get_interpreter()
                    .adaptive_predict(decision, &mut self.base)?;
            }
        }
        let track = self.track_alt_numbers
            && self
                .outer_alt_decision_states
                .as_ref()
                .is_some_and(|states| {
                    states
                        .get(p.get_state_number() as usize)
                        .copied()
                        .unwrap_or(false)
                });
        if track {
            self.base
                .with_mut_ctx(|ctx| ctx.set_alt_number(predicted_alt));
        }
        Ok(predicted_alt)
    }

    /// Provides a simple "factory" for [InterpreterRuleContext](crate::interpreter_rule_context::InterpreterRuleContext)s.
    fn create_interpreter_rule_context(
        &self,
        parent: Option<&'arena InterpreterNode<'input, 'arena, TF::Tok>>,
        invoking_state: i32,
        rule_index: usize,
    ) -> Result<&'arena mut InterpreterNode<'input, 'arena, TF::Tok>, ANTLRError> {
        let node = BaseParserRuleContext::create(
            self.base.arena,
            parent,
            invoking_state,
            InterpreterRuleContextExt::new(rule_index),
        )?;
        // The ANTLR tool's `GrammarInterpreterRuleContext` initializes
        // `outerAltNum` to 1; match that when alt-number tracking is on.
        if self.track_alt_numbers {
            node.set_alt_number(1);
        }
        Ok(node)
    }

    fn visit_rule_stop_state(&mut self, p: &'static ATNState) -> Result<(), ANTLRError> {
        let atn = self.base.atn;
        let rule_start_state = atn.rule_to_start_state[p.get_rule_index() as usize];
        if is_left_recursive_rule(&rule_start_state) {
            let (parent_ctx, parent_state) = self
                .parent_context_stack
                .pop()
                .expect("left-recursive rule has a parent context");
            self.base.unroll_recursion_context(parent_ctx)?;
            self.base.set_state(parent_state);
        } else {
            self.base.exit_rule()?;
        }

        let state = atn.get_state(self.base.get_state());
        let rule_transition = state
            .get_transitions()
            .first()
            .and_then(|transition| transition.try_as::<RuleTransition>())
            .expect("invoking state has a rule transition");
        self.base
            .set_state(rule_transition.follow_state.get_state_number());
        Ok(())
    }

    /// Enables or disables outer alternative number tracking; see
    /// [ParserInterpreter::track_alt_numbers].
    pub fn set_track_alt_numbers(&mut self, track: bool) {
        if track && self.outer_alt_decision_states.is_none() {
            self.outer_alt_decision_states = Some(find_outer_most_decision_states(self.base.atn));
        }
        self.track_alt_numbers = track;
    }

    /// Override this parser interpreter's normal decision-making process at a
    /// particular decision and input token index. Instead of allowing the
    /// adaptive prediction mechanism to choose the first alternative within a
    /// block that leads to a successful parse, force it to take the
    /// alternative, 1..n for n alternatives.
    ///
    /// As an implementation limitation right now, you can only specify one
    /// override. This is sufficient to allow construction of different parse
    /// trees for ambiguous input. It means re-parsing the entire input in
    /// general because you're never sure where an ambiguous sequence would
    /// live in the various parse trees.
    ///
    /// Only parser interpreters can override decisions so as to avoid
    /// inserting override checking code in the critical ALL(*) prediction
    /// execution path.
    pub fn add_decision_override(&mut self, decision: i32, token_index: isize, forced_alt: i32) {
        self.override_decision = decision;
        self.override_decision_input_index = token_index;
        self.override_decision_alt = forced_alt;
    }

    /// Returns the root of the parse when a decision override was used.
    pub fn get_override_decision_root(
        &self,
    ) -> Option<&'arena InterpreterNode<'input, 'arena, TF::Tok>> {
        self.override_decision_root
    }

    /// Rely on the error handler for this parser but, if no tokens are
    /// consumed to recover, add an error node. Otherwise, nothing is seen in
    /// the parse tree.
    fn recover(&mut self, e: &ANTLRError) -> Result<(), ANTLRError> {
        let i = self.base.get_input_stream().index();
        self.err_handler.recover(&mut self.base, e)?;
        if self.base.get_input_stream().index() == i {
            // no input consumed, better add an error node
            let expected_token_type = match e.as_ref() {
                ANTLRErrorKind::InputMismatchError(ime) => ime
                    .base
                    .get_expected_tokens(&self.base)
                    .get_min()
                    .unwrap_or(TOKEN_INVALID_TYPE),
                // NoViableAlt
                _ => TOKEN_INVALID_TYPE,
            };
            let offending_token = e
                .get_offending_token()
                .expect("recognition error has an offending token");
            let text = offending_token.get_text().to_owned();
            let line = offending_token.get_line();
            let column = offending_token.get_char_position_in_line();
            let err_token = self.base.get_token_factory().create(
                None::<&mut dyn CharStream<'input>>,
                expected_token_type,
                Some(text),
                TOKEN_DEFAULT_CHANNEL,
                -1,
                -1, // invalid start/stop
                line,
                column,
            );
            let node = TreeNode::create_error_node(self.base.arena, err_token);
            if node.is_null() {
                return Err(ANTLRError::dfa_cache_limit_exceeded(0, 0, 0));
            }
            let node = unsafe { &*node };
            self.base.with_mut_ctx(|ctx| ctx.add_child(node));
        }
        Ok(())
    }

    fn recover_inline(&mut self) -> Result<&'arena TF::Tok, ANTLRError> {
        self.err_handler.recover_inline(&mut self.base)
    }

    /// Return the root of the parse, which can be useful if the parser
    /// bails out. You still can access the top node. Note that,
    /// because of the way left recursive rules add children, it's possible
    /// that the root will not have any children if the start rule immediately
    /// called and left recursive rule that fails.
    pub fn get_root_context(&self) -> Option<&'arena InterpreterNode<'input, 'arena, TF::Tok>> {
        self.root_context
    }
}

/// Port of the ANTLR tool's
/// `GrammarParserInterpreter.findOuterMostDecisionStates`: identifies the ATN
/// states where the outer alternative number of the current rule context is
/// determined. For regular rules that is the block at the target of the rule
/// start state; for left-recursive rules it is also the star block holding
/// the recursive alternatives. Returns a set indexed by state number.
fn find_outer_most_decision_states(atn: &ATN) -> Vec<bool> {
    let mut track = vec![false; atn.states_count()];
    for decision in 0..atn.decision_to_state.len() {
        let decision_state = atn.get_decision_state(decision as i32);
        let start_state = atn.rule_to_start_state[decision_state.get_rule_index() as usize];
        if decision_state.state_type() == ATNStateType::StarLoopEntry {
            if decision_state
                .try_as::<StarLoopEntryState>()
                .is_some_and(|state| state.is_precedence)
            {
                let block_start = decision_state.get_transitions()[0].get_target();
                track[block_start.get_state_number() as usize] = true;
            }
        } else if start_state
            .get_transitions()
            .first()
            .is_some_and(|transition| {
                transition.get_target().get_state_number() == decision_state.get_state_number()
            })
        {
            track[decision_state.get_state_number() as usize] = true;
        }
    }
    track
}

fn is_left_recursive_rule(rule_start_state: &ATNState) -> bool {
    rule_start_state
        .try_as::<RuleStartState>()
        .is_some_and(|state| state.is_left_recursive)
}
