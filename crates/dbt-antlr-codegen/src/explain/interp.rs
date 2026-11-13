// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The parser simulation of `--explain`.
//!
//! [`Simulation`] walks the parser ATN like the runtime `ParserInterpreter`,
//! but predicts each decision the way the parser under test does:
//!
//! - [`Oracle::Static`] predicts like the generated `--static-parser-dfa` or
//!   `--static-dfa-only` parser: the runtime walk of the static tables, the
//!   inline token switch of an `LL(1)` decision, and the neutral parse of a
//!   shared-descent group.
//! - [`Oracle::FullLl`] predicts every decision with `adaptivePredict` in the
//!   `LL` prediction mode.
//!
//! Both simulations stop at the first syntax error and record every
//! prediction in a [`Trace`].

use std::collections::HashMap;

use dbt_antlr_runtime::atn::{ATN, INVALID_ALT};
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager;
use dbt_antlr_runtime::atn_state::{ATNState, ATNStateType, RuleStartState, StarLoopEntryState};
use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::dfa::static_parser_dfa::StaticParserTables as RuntimeTables;
use dbt_antlr_runtime::error_strategy::ErrorStrategy;
use dbt_antlr_runtime::errors::{ANTLRError, ANTLRErrorKind};
use dbt_antlr_runtime::interpreter_rule_context::{
    InterpreterNode, InterpreterNodeKind, InterpreterRuleContextExt,
};
use dbt_antlr_runtime::parser::{BaseParser, Parser, ParserRecog};
use dbt_antlr_runtime::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use dbt_antlr_runtime::recognizer::Actions;
use dbt_antlr_runtime::rule_context::RuleContext;
use dbt_antlr_runtime::token::{TOKEN_EOF, TOKEN_MIN_USER_TOKEN_TYPE, Token};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::transition::{
    PrecedencePredicateTransition, RuleTransition, TransitionType,
};
use dbt_antlr_runtime::tree::Tree;
use dbt_antlr_runtime::vocabulary::Vocabulary;
use dbt_antlr_runtime::{Arena, PredictionMode};

use super::lex::{ExplainLexer, LexerSpec};
use crate::grammar::atn::DecisionLookahead;
use crate::optimization::parser_dfa::{DecisionResult, EscapeResolution, StaticDfaTable};
use crate::optimization::{DescentGroup, StaticParserTables};

type ExplainTokens<'input, 'arena> = CommonTokenStream<
    'input,
    'arena,
    ExplainLexer<'input, 'arena>,
    CommonTokenFactory<'input, 'arena>,
>;
type ExplainBase<'input, 'arena> = BaseParser<
    'input,
    'arena,
    ExplainParserExt,
    InterpreterNodeKind,
    ExplainTokens<'input, 'arena>,
    CommonTokenFactory<'input, 'arena>,
>;
type ExplainNode<'input, 'arena> =
    InterpreterNode<'input, 'arena, dbt_antlr_runtime::token::CommonToken<'input>>;

/// The recognizer data of the simulated parser.
pub(super) struct ExplainParserExt {
    grammar_file_name: &'static str,
    rule_names: &'static [&'static str],
    vocabulary: Box<dyn Vocabulary>,
    atn_manager: &'static ParserATNSimulatorManager,
}

impl<'input, 'arena> Actions<'input, 'arena, ExplainBase<'input, 'arena>, ExplainTok<'input>>
    for ExplainParserExt
{
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

impl<'input, 'arena> ParserRecog<'input, 'arena, ExplainBase<'input, 'arena>, ExplainTok<'input>>
    for ExplainParserExt
{
    fn get_atn_simulator_man(&self) -> &'static ParserATNSimulatorManager {
        self.atn_manager
    }
}

type ExplainTok<'input> = dbt_antlr_runtime::token::CommonToken<'input>;

/// An error strategy that never recovers: the first syntax error ends the
/// simulation.
struct FailFast;

impl<'input, 'arena>
    ErrorStrategy<'input, 'arena, CommonTokenFactory<'input, 'arena>, ExplainBase<'input, 'arena>>
    for FailFast
{
    fn reset(&mut self, _recognizer: &mut ExplainBase<'input, 'arena>) {}

    fn recover_inline(
        &mut self,
        recognizer: &mut ExplainBase<'input, 'arena>,
    ) -> Result<&'arena ExplainTok<'input>, ANTLRError> {
        Err(ANTLRError::input_mismatch(recognizer))
    }

    fn recover(
        &mut self,
        _recognizer: &mut ExplainBase<'input, 'arena>,
        e: &ANTLRError,
    ) -> Result<(), ANTLRError> {
        Err(e.clone())
    }

    fn sync(&mut self, _recognizer: &mut ExplainBase<'input, 'arena>) -> Result<(), ANTLRError> {
        Ok(())
    }

    fn in_error_recovery_mode(&mut self, _recognizer: &mut ExplainBase<'input, 'arena>) -> bool {
        false
    }

    fn report_error(&mut self, _recognizer: &mut ExplainBase<'input, 'arena>, _e: &ANTLRError) {}

    fn report_match(&mut self, _recognizer: &mut ExplainBase<'input, 'arena>) {}
}

/// The static prediction data of the parser under test.
pub(super) struct StaticOracle<'r> {
    pub(super) results: &'r [DecisionResult],
    pub(super) tables: &'r StaticParserTables,
    pub(super) runtime: &'static RuntimeTables,
    /// The `LL(1)` lookahead of each disjoint decision, by decision number.
    pub(super) ll1: HashMap<usize, &'r DecisionLookahead>,
    pub(super) static_only: bool,
}

/// How a simulation predicts its decisions.
pub(super) enum Oracle<'r> {
    Static(StaticOracle<'r>),
    FullLl {
        /// The step at which the simulation also predicts with SLL and stops.
        probe: Option<usize>,
    },
}

/// The grammar-level data a simulation needs.
pub(super) struct ParserSpec {
    pub(super) grammar_file_name: &'static str,
    pub(super) rule_names: &'static [&'static str],
    pub(super) literal_names: Vec<Option<String>>,
    pub(super) symbolic_names: Vec<Option<String>>,
    pub(super) atn: &'static ATN,
}

impl ParserSpec {
    fn vocabulary(&self) -> Box<dyn Vocabulary> {
        Box::new(dbt_antlr_runtime::vocabulary::VocabularyImpl::new(
            self.literal_names.iter(),
            self.symbolic_names.iter(),
            None,
        ))
    }
}

/// One rule invocation on the parse stack: the rule and the index of its
/// first token.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Frame {
    pub(super) rule: usize,
    pub(super) start: isize,
}

/// How the static parser chose the alternative of a table decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TableKind {
    /// An accept state that the analysis proved.
    Exact,
    /// No edge matched: the lowest alternative that already left the decision.
    Fallback,
    /// `--static-dfa-only` took the minimum alternative of an SLL conflict.
    MinAlt,
    /// The lookahead budget stopped the analysis; `--static-dfa-only` took the
    /// minimum viable alternative.
    MinViableAlt,
    /// A guarded state took its alternative.
    GuardTaken,
    /// A guarded state deferred to its other alternative.
    GuardDeferred,
    /// A predicated state that `--static-dfa-only` turned into a syntax error.
    SyntaxError,
    /// An escape state: the table handed the decision to `adaptivePredict`.
    Escape,
    /// The walk ended in a state that the explanation cannot classify.
    Unknown,
}

impl TableKind {
    /// Whether the static table guessed here instead of deciding.
    pub(super) const fn is_guess(self) -> bool {
        matches!(
            self,
            Self::MinAlt | Self::MinViableAlt | Self::GuardDeferred
        )
    }
}

/// Where a static table walk stopped.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct TableWalk {
    /// The precedence class of the table, for a precedence decision.
    pub(super) class: Option<usize>,
    /// The generation-time state, as `--dump-dfas` numbers it.
    pub(super) state: usize,
    /// The token indices that the walk read.
    pub(super) lookahead: Vec<isize>,
    pub(super) kind: TableKind,
    /// The alternatives that the guess dropped.
    pub(super) dropped: Vec<i32>,
    /// The alternative the state predicts, when the table alone decides it.
    expected: Option<i32>,
    /// The alternative and the deferred alternative of a guarded state.
    guard: Option<(i32, i32)>,
}

/// How the simulation predicted one decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Resolution {
    /// Not predicted yet, or the prediction failed before it was classified.
    Pending,
    /// The inline token switch of an `LL(1)` decision.
    Ll1 { matched: bool },
    /// A static table walk.
    Table(TableWalk),
    /// A shared-descent group: a neutral parse of the shared rule, then a
    /// dispatch on the token after it.
    Descent {
        group: i32,
        rule: usize,
        /// The token index after the shared rule.
        tail: isize,
        walk: Option<TableWalk>,
        /// The member alternatives of the group.
        alts: Vec<i32>,
        /// The step after the last decision of the neutral parse.
        neutral_end: usize,
    },
    /// A shared-descent group that could not start its neutral parse.
    DescentFallback { group: i32 },
    /// A shared-descent group whose neutral parse of the shared rule failed
    /// before the tail dispatch chose a member alternative.
    DescentAborted {
        group: i32,
        /// The member alternatives of the group.
        alts: Vec<i32>,
        walk: Option<TableWalk>,
    },
    /// A decision without a static table, which `--static-dfa-only` cannot
    /// predict (the generator reports `G4S190`).
    NoStaticTable,
    /// `adaptivePredict` of a decision without a static table.
    Adaptive,
    /// Full `LL` prediction (the oracle).
    FullLl,
}

/// One prediction of a simulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DecisionEvent {
    pub(super) decision: usize,
    pub(super) rule: usize,
    pub(super) token_index: isize,
    pub(super) stack: Vec<Frame>,
    /// The predicted alternative, `None` when the prediction failed.
    pub(super) alt: Option<i32>,
    pub(super) resolution: Resolution,
}

impl DecisionEvent {
    /// Whether the static parser guessed at this decision.
    pub(super) fn is_guess(&self) -> bool {
        match &self.resolution {
            Resolution::Table(walk) => walk.kind.is_guess(),
            Resolution::Descent {
                walk: Some(walk), ..
            } => walk.kind.is_guess(),
            Resolution::DescentFallback { .. } => true,
            _ => false,
        }
    }

    /// The table walk of a static prediction, if it had one.
    pub(super) fn walk(&self) -> Option<&TableWalk> {
        match &self.resolution {
            Resolution::Table(walk)
            | Resolution::Descent {
                walk: Some(walk), ..
            }
            | Resolution::DescentAborted {
                walk: Some(walk), ..
            } => Some(walk),
            _ => None,
        }
    }
}

/// The kind of a syntax error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ErrorKind {
    NoViableAlt,
    Mismatch,
    FailedPredicate,
    Other,
}

/// The first syntax error of a simulation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SyntaxError {
    pub(super) kind: ErrorKind,
    pub(super) token_index: isize,
    /// The token types the parser expected at the error.
    pub(super) expected: Vec<i32>,
    pub(super) stack: Vec<Frame>,
    /// The step of the decision whose prediction failed.
    pub(super) step: Option<usize>,
    pub(super) message: String,
}

/// How a simulation ended.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) enum Outcome {
    /// The start rule matched. `stop` is the index of the next token.
    Accepted {
        stop: isize,
    },
    Failed(SyntaxError),
}

/// What a simulation recorded.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Trace {
    pub(super) events: Vec<DecisionEvent>,
    pub(super) outcome: Outcome,
    /// The SLL prediction at the probe step, when the simulation probed.
    pub(super) probe: Option<Option<i32>>,
}

/// Simulates one parse of `tokens` from `start_rule`.
pub(super) fn simulate(
    spec: &ParserSpec,
    lexer: &LexerSpec,
    input: &str,
    oracle: &Oracle<'_>,
    start_rule: usize,
) -> Trace {
    let atn_manager: &'static ParserATNSimulatorManager = match oracle {
        Oracle::Static(static_oracle) => Box::leak(Box::new(
            ParserATNSimulatorManager::new_with_static_parser_tables(
                spec.atn,
                static_oracle.runtime,
            ),
        )),
        Oracle::FullLl { .. } => Box::leak(Box::new(ParserATNSimulatorManager::new(spec.atn))),
    };
    Arena::with(|arena| {
        let tokens = CommonTokenStream::new(lexer.lexer(arena, input));
        let mut base = BaseParser::new_base_parser(
            arena,
            tokens,
            ExplainParserExt {
                grammar_file_name: spec.grammar_file_name,
                rule_names: spec.rule_names,
                vocabulary: spec.vocabulary(),
                atn_manager,
            },
        );
        base.remove_error_listeners();
        base.get_interpreter()
            .set_prediction_mode(PredictionMode::LL);
        let mut simulation = Simulation {
            base,
            atn: spec.atn,
            oracle,
            parent_context_stack: Vec::new(),
            depth: 0,
            events: Vec::new(),
            probe: None,
            call_sites: HashMap::new(),
        };
        let outcome = match simulation.parse(start_rule) {
            Ok(()) => Outcome::Accepted {
                stop: simulation.base.get_input_stream().index(),
            },
            Err(error) => Outcome::Failed(simulation.syntax_error(&error)),
        };
        Trace {
            events: simulation.events,
            outcome,
            probe: simulation.probe,
        }
    })
}

/// The error a probe raises to end the simulation once the probe ran.
const PROBE_DONE: &str = "explain: probe done";

struct Simulation<'input, 'arena, 'o, 'r> {
    base: ExplainBase<'input, 'arena>,
    atn: &'static ATN,
    oracle: &'o Oracle<'r>,
    /// The `_parentctx`, `_parentState` pairs of the active left-recursive
    /// rule invocations (see `ParserInterpreter`).
    parent_context_stack: Vec<(Option<&'arena ExplainNode<'input, 'arena>>, i32)>,
    /// The number of active rule invocations.
    depth: usize,
    events: Vec<DecisionEvent>,
    probe: Option<Option<i32>>,
    /// The leaked call sites of each shared-descent group, by virtual
    /// alternative and decision.
    call_sites: HashMap<(usize, i32), &'static [i32]>,
}

impl<'input, 'arena> Simulation<'input, 'arena, '_, '_> {
    fn parse(&mut self, start_rule: usize) -> Result<(), ANTLRError> {
        let start_state = self.atn.rule_to_start_state[start_rule];
        let left_recursive = is_left_recursive_rule(start_state.as_ref());
        let root = self.create_context(None, -1, start_rule)?;
        if left_recursive {
            self.enter_recursion_rule(root, start_state.get_state_number(), start_rule, 0)?;
        } else {
            self.base
                .enter_rule(root, start_state.get_state_number(), start_rule)?;
        }
        self.depth = 1;
        self.run_until(1)?;
        if left_recursive {
            let (parent, _) = self
                .parent_context_stack
                .pop()
                .expect("left-recursive start rule has a parent context");
            self.base.unroll_recursion_context(parent)?;
        } else {
            self.base.exit_rule()?;
        }
        Ok(())
    }

    /// Runs until the innermost rule invocation at `depth` reaches its stop
    /// state.
    fn run_until(&mut self, depth: usize) -> Result<(), ANTLRError> {
        loop {
            let state = self.atn.get_state(self.base.get_state());
            if state.state_type() == ATNStateType::RuleStop {
                if self.depth == depth {
                    return Ok(());
                }
                self.visit_rule_stop_state(state)?;
            } else {
                self.visit_state(state)?;
            }
        }
    }

    fn visit_state(&mut self, state: &'static ATNState) -> Result<(), ANTLRError> {
        let mut alt = 1;
        if state.is_decision_state() && state.get_transitions().len() > 1 {
            alt = self.predict(state)?;
        }
        let transition = usize::try_from(alt - 1)
            .ok()
            .and_then(|index| state.get_transitions().get(index))
            .ok_or_else(|| ANTLRError::no_alt(&mut self.base))?;
        match transition.transition_type() {
            TransitionType::Epsilon => {
                if state.state_type() == ATNStateType::StarLoopEntry
                    && state
                        .try_as::<StarLoopEntryState>()
                        .is_some_and(|entry| entry.is_precedence)
                    && transition.get_target().state_type() != ATNStateType::LoopEnd
                {
                    let (parent, parent_state) = *self
                        .parent_context_stack
                        .last()
                        .expect("left-recursive rule has a parent context");
                    let rule = self
                        .base
                        .ctx()
                        .expect("parser is inside a rule")
                        .get_rule_index();
                    let context = self.create_context(parent, parent_state, rule)?;
                    self.base.push_new_recursion_context(
                        context,
                        self.atn.rule_to_start_state[rule].get_state_number(),
                        rule,
                    )?;
                }
            }
            TransitionType::Atom => {
                let label = transition
                    .try_as::<dbt_antlr_runtime::transition::AtomTransition>()
                    .expect("atom transition")
                    .label();
                self.base.match_token(label, &mut FailFast)?;
            }
            TransitionType::Range | TransitionType::Set | TransitionType::NotSet => {
                let token = self.base.get_input_stream_mut().la(1);
                if !transition.matches(token, TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type) {
                    return Err(ANTLRError::input_mismatch(&mut self.base));
                }
                self.base.match_wildcard(&mut FailFast)?;
            }
            TransitionType::Wildcard => {
                self.base.match_wildcard(&mut FailFast)?;
            }
            TransitionType::Rule => {
                let rule_transition = transition
                    .try_as::<RuleTransition>()
                    .expect("rule transition");
                let rule_start = transition.get_target();
                let rule = usize::try_from(rule_start.get_rule_index()).unwrap_or(0);
                if matches!(self.oracle, Oracle::Static(_))
                    && self.base.resume_take(rule)?.is_some()
                {
                    // The re-descent of a shared-descent member takes the
                    // subtree of the neutral parse.
                    self.base
                        .set_state(rule_transition.follow_state.get_state_number());
                    return Ok(());
                }
                self.enter_rule_call(state.get_state_number(), rule, rule_transition.precedence())?;
                return Ok(());
            }
            // The simulation cannot run target code: every semantic predicate
            // passes and every action does nothing.
            TransitionType::Predicate | TransitionType::Action => {}
            TransitionType::PrecedencePredicate => {
                let precedence = transition
                    .try_as::<PrecedencePredicateTransition>()
                    .expect("precedence predicate transition")
                    .precedence();
                let context = self.base.ctx();
                if !self.base.precpred(context, precedence) {
                    return Err(ANTLRError::failed_predicate(
                        &mut self.base,
                        Some(format!("precpred(_ctx, {precedence})")),
                        None,
                    ));
                }
            }
        }
        self.base
            .set_state(transition.get_target().get_state_number());
        Ok(())
    }

    /// Enters a rule from the rule transition at `invoking_state`.
    fn enter_rule_call(
        &mut self,
        invoking_state: i32,
        rule: usize,
        precedence: i32,
    ) -> Result<&'arena ExplainNode<'input, 'arena>, ANTLRError> {
        let rule_start = self.atn.rule_to_start_state[rule];
        let context = self.create_context(self.base.ctx(), invoking_state, rule)?;
        // SAFETY: the context is allocated in the arena, which outlives the
        // simulation; the raw read does not hold the `&mut` borrow.
        let node = unsafe { &*std::ptr::from_ref(&*context) };
        if is_left_recursive_rule(rule_start.as_ref()) {
            self.enter_recursion_rule(context, rule_start.get_state_number(), rule, precedence)?;
        } else {
            self.base
                .enter_rule(context, rule_start.get_state_number(), rule)?;
        }
        self.depth += 1;
        Ok(node)
    }

    fn enter_recursion_rule(
        &mut self,
        context: &'arena mut ExplainNode<'input, 'arena>,
        state: i32,
        rule: usize,
        precedence: i32,
    ) -> Result<(), ANTLRError> {
        let pair = (self.base.ctx(), context.get_invoking_state());
        self.parent_context_stack.push(pair);
        self.base
            .enter_recursion_rule(context, state, rule, precedence)
    }

    fn visit_rule_stop_state(&mut self, state: &'static ATNState) -> Result<(), ANTLRError> {
        let rule_start =
            self.atn.rule_to_start_state[usize::try_from(state.get_rule_index()).unwrap_or(0)];
        if is_left_recursive_rule(rule_start.as_ref()) {
            let (parent, parent_state) = self
                .parent_context_stack
                .pop()
                .expect("left-recursive rule has a parent context");
            self.base.unroll_recursion_context(parent)?;
            self.base.set_state(parent_state);
        } else {
            self.base.exit_rule()?;
        }
        self.depth -= 1;
        let invoking = self.atn.get_state(self.base.get_state());
        let rule_transition = invoking
            .get_transitions()
            .first()
            .and_then(|transition| transition.try_as::<RuleTransition>())
            .expect("invoking state has a rule transition");
        self.base
            .set_state(rule_transition.follow_state.get_state_number());
        Ok(())
    }

    fn create_context(
        &self,
        parent: Option<&'arena ExplainNode<'input, 'arena>>,
        invoking_state: i32,
        rule: usize,
    ) -> Result<&'arena mut ExplainNode<'input, 'arena>, ANTLRError> {
        BaseParserRuleContext::create(
            self.base.arena,
            parent,
            invoking_state,
            InterpreterRuleContextExt::new(rule),
        )
    }

    /// The parse stack, outermost rule first.
    fn stack(&self) -> Vec<Frame> {
        let mut frames = Vec::new();
        let mut context = self.base.ctx();
        while let Some(node) = context {
            frames.push(Frame {
                rule: node.get_rule_index(),
                start: ParserRuleContext::start(node).get_token_index(),
            });
            context = node.get_parent();
        }
        frames.reverse();
        frames
    }

    fn predict(&mut self, state: &'static ATNState) -> Result<i32, ANTLRError> {
        let decision = usize::try_from(state.get_decision().unwrap_or(-1)).unwrap_or(usize::MAX);
        let step = self.events.len();
        if let Oracle::FullLl { probe: Some(probe) } = self.oracle
            && *probe == step
        {
            return self.run_probe(decision);
        }
        self.events.push(DecisionEvent {
            decision,
            rule: usize::try_from(state.get_rule_index()).unwrap_or(0),
            token_index: self.base.get_input_stream().index(),
            stack: self.stack(),
            alt: None,
            resolution: Resolution::Pending,
        });
        let predicted = match self.oracle {
            Oracle::Static(oracle) => self.predict_static(oracle, state, decision),
            Oracle::FullLl { .. } => self.adaptive(decision).map(|alt| (alt, Resolution::FullLl)),
        };
        match predicted {
            Ok((alt, resolution)) => {
                let event = &mut self.events[step];
                event.alt = Some(alt);
                event.resolution = resolution;
                Ok(alt)
            }
            Err(error) => Err(error),
        }
    }

    fn adaptive(&mut self, decision: usize) -> Result<i32, ANTLRError> {
        let decision = i32::try_from(decision).unwrap_or(-1);
        self.base
            .get_interpreter()
            .adaptive_predict(decision, &mut self.base)
    }

    /// Predicts the decision at the probe step with SLL and ends the
    /// simulation.
    fn run_probe(&mut self, decision: usize) -> Result<i32, ANTLRError> {
        let interpreter = self.base.get_interpreter();
        interpreter.set_prediction_mode(PredictionMode::SLL);
        let sll = self.adaptive(decision).ok();
        interpreter.set_prediction_mode(PredictionMode::LL);
        self.probe = Some(sll);
        Err(ANTLRError::custom_error(PROBE_DONE.to_owned()))
    }

    fn predict_static(
        &mut self,
        oracle: &StaticOracle<'_>,
        state: &'static ATNState,
        decision: usize,
    ) -> Result<(i32, Resolution), ANTLRError> {
        let decision_number = i32::try_from(decision).unwrap_or(-1);
        if let Some(lookahead) = oracle.ll1.get(&decision) {
            return self.predict_ll1(state, lookahead);
        }
        if oracle.tables.has_table(decision) {
            let walk = oracle
                .results
                .get(decision)
                .and_then(|result| self.walk_table(result));
            // A failed table walk keeps the state it stopped in.
            self.set_resolution(Resolution::Table(walk.clone().unwrap_or_else(unknown_walk)));
            if let Some(plan) = oracle.tables.descent(decision) {
                let virtual_alt = self.base.dfa_predict_virtual(decision_number)?;
                if let Some(group) = plan
                    .groups
                    .iter()
                    .find(|group| group.virtual_alt == virtual_alt)
                {
                    return self.shared_descent(oracle, decision, group, walk);
                }
                return self.table_alt(oracle, decision, virtual_alt, walk);
            }
            let alt = self.base.dfa_predict(decision_number)?;
            return self.table_alt(oracle, decision, alt, walk);
        }
        if oracle.static_only {
            return Err(self.no_static_table(decision));
        }
        self.adaptive(decision)
            .map(|alt| (alt, Resolution::Adaptive))
    }

    /// Sets the resolution of the prediction in progress, the last event.
    fn set_resolution(&mut self, resolution: Resolution) {
        if let Some(event) = self.events.last_mut() {
            event.resolution = resolution;
        }
    }

    /// The error of a decision that `--static-dfa-only` cannot predict.
    fn no_static_table(&mut self, decision: usize) -> ANTLRError {
        self.set_resolution(Resolution::NoStaticTable);
        ANTLRError::custom_error(format!(
            "decision {decision} has no static table: --static-dfa-only rejects it (G4S190)"
        ))
    }

    /// Finishes the prediction of a table decision.
    fn table_alt(
        &mut self,
        oracle: &StaticOracle<'_>,
        decision: usize,
        alt: i32,
        walk: Option<TableWalk>,
    ) -> Result<(i32, Resolution), ANTLRError> {
        if alt == INVALID_ALT {
            if oracle.tables.defers(decision) {
                if oracle.static_only {
                    return Err(self.no_static_table(decision));
                }
                let alt = self.adaptive(decision)?;
                let mut walk = walk.unwrap_or_else(unknown_walk);
                walk.kind = TableKind::Escape;
                return Ok((alt, Resolution::Table(walk)));
            }
            return Err(ANTLRError::no_alt(&mut self.base));
        }
        let walk = walk.map_or_else(unknown_walk, |walk| checked_walk(walk, alt));
        Ok((alt, Resolution::Table(walk)))
    }

    /// The `shared_descent!` protocol of a generated `DescentAltBlock`.
    fn shared_descent(
        &mut self,
        oracle: &StaticOracle<'_>,
        decision: usize,
        group: &DescentGroup,
        walk: Option<TableWalk>,
    ) -> Result<(i32, Resolution), ANTLRError> {
        let group_number =
            group.virtual_alt - dbt_antlr_runtime::dfa::static_parser_dfa::VIRTUAL_ALT_BASE;
        if !self.base.descent_ready() {
            let alt = if oracle.static_only {
                group.min_alt()
            } else {
                self.adaptive(decision)?
            };
            return Ok((
                alt,
                Resolution::DescentFallback {
                    group: group_number,
                },
            ));
        }
        let call_sites = *self
            .call_sites
            .entry((decision, group.virtual_alt))
            .or_insert_with(|| Box::leak(group.call_sites.clone().into_boxed_slice()));
        let step = self.events.len() - 1;
        let neutral = self
            .base
            .begin_neutral_parse(call_sites, group.prefix.len());
        let depth = self.depth;
        let invoked = self
            .enter_rule_call(self.base.get_state(), group.rule, group.precedence)
            .and_then(|_| self.run_until(depth + 1))
            .and_then(|()| self.finish_rule_call());
        match invoked {
            Ok(node) => {
                let token = self.base.get_input_stream_mut().la(1);
                let tail = self.base.get_input_stream().index();
                let alt = group
                    .arms
                    .iter()
                    .find(|arm| arm.tokens.contains(&token))
                    .map_or_else(|| group.default_or_min_alt(), |arm| arm.alt);
                self.base.resume_neutral_parse(neutral, node, group.rule);
                Ok((
                    alt,
                    Resolution::Descent {
                        group: group_number,
                        rule: group.rule,
                        tail,
                        walk,
                        alts: group.alts.clone(),
                        neutral_end: self.events.len(),
                    },
                ))
            }
            Err(error) => {
                self.base.abort_neutral_parse(neutral);
                self.events[step].resolution = Resolution::DescentAborted {
                    group: group_number,
                    alts: group.alts.clone(),
                    walk,
                };
                Err(error)
            }
        }
    }

    /// Returns from the rule invocation that [`Self::run_until`] stopped in,
    /// like the generated rule function does.
    fn finish_rule_call(&mut self) -> Result<&'arena ExplainNode<'input, 'arena>, ANTLRError> {
        let state = self.atn.get_state(self.base.get_state());
        let rule_start =
            self.atn.rule_to_start_state[usize::try_from(state.get_rule_index()).unwrap_or(0)];
        self.depth -= 1;
        if is_left_recursive_rule(rule_start.as_ref()) {
            let node = self.base.ctx().expect("parser is inside a rule");
            let (parent, _) = self
                .parent_context_stack
                .pop()
                .expect("left-recursive rule has a parent context");
            self.base.unroll_recursion_context(parent)?;
            Ok(node)
        } else {
            self.base.exit_rule()
        }
    }

    /// The inline token switch of a generated `LL(1)` block.
    fn predict_ll1(
        &mut self,
        state: &'static ATNState,
        lookahead: &DecisionLookahead,
    ) -> Result<(i32, Resolution), ANTLRError> {
        let token = self.base.get_input_stream_mut().la(1);
        // The generated block tests only the alternatives that enter it: a
        // token that matches none skips the block or exits the loop.
        let exit = has_exit_alternative(state);
        let entries = if exit {
            lookahead.alternatives.len().saturating_sub(1)
        } else {
            lookahead.alternatives.len()
        };
        let matched = lookahead.alternatives[..entries]
            .iter()
            .position(|alternative| {
                alternative.as_ref().is_some_and(|ranges| {
                    ranges.iter().any(|(lo, hi)| (*lo..=*hi).contains(&token))
                })
            });
        if let Some(index) = matched {
            let alt = i32::try_from(index + 1).unwrap_or(INVALID_ALT);
            return Ok((alt, Resolution::Ll1 { matched: true }));
        }
        self.set_resolution(Resolution::Ll1 { matched: false });
        if exit {
            let alt = i32::try_from(state.get_transitions().len()).unwrap_or(INVALID_ALT);
            return Ok((alt, Resolution::Ll1 { matched: false }));
        }
        Err(ANTLRError::no_alt(&mut self.base))
    }

    /// Walks the generation-time table of a decision over the lookahead to
    /// find the state where the runtime walk stops.
    fn walk_table(&mut self, result: &DecisionResult) -> Option<TableWalk> {
        let precedence = self.base.get_precedence().max(0);
        let (table, class) = match (&result.table, &result.precedence) {
            (Some(table), _) => (table, None),
            (None, Some(classes)) => {
                let class = classes
                    .cutoffs
                    .iter()
                    .take_while(|cutoff| **cutoff < precedence)
                    .count();
                (classes.tables.get(class)?.as_ref()?, Some(class))
            }
            (None, None) => return None,
        };
        let mut state = 0_usize;
        let mut lookahead = Vec::new();
        let mut stopped_on_edge = false;
        if table.states.first()?.accept == 0 {
            // Step one token at a time like the runtime walk: `la(k)` over a
            // channel-filtering stream costs O(k).
            let stream = self.base.get_input_stream_mut();
            let start = stream.index();
            let mark = stream.mark();
            loop {
                let token = stream.la(1);
                let Some(edge) = table.states.get(state).and_then(|entry| {
                    entry
                        .edges
                        .iter()
                        .find(|edge| edge.lo <= token && token <= edge.hi)
                }) else {
                    break;
                };
                lookahead.push(stream.lt(1).map_or(-1, |token| token.get_token_index()));
                state = edge.target;
                if table
                    .states
                    .get(state)
                    .is_none_or(|entry| entry.accept != 0)
                {
                    stopped_on_edge = true;
                    break;
                }
                if token == TOKEN_EOF {
                    break;
                }
                stream.consume();
            }
            stream.seek(start);
            stream.release(mark);
        }
        table
            .states
            .get(state)
            .map(|_| classify_walk(table, class, state, lookahead, stopped_on_edge))
    }

    /// Describes the error that ended the simulation.
    fn syntax_error(&mut self, error: &ANTLRError) -> SyntaxError {
        let (kind, base) = match error.as_ref() {
            ANTLRErrorKind::NoAltError(error) => (ErrorKind::NoViableAlt, Some(&error.base)),
            ANTLRErrorKind::InputMismatchError(error) => (ErrorKind::Mismatch, Some(&error.base)),
            ANTLRErrorKind::PredicateError(error) => {
                (ErrorKind::FailedPredicate, Some(&error.base))
            }
            _ => (ErrorKind::Other, None),
        };
        let token_index = error.get_offending_token().map_or_else(
            || self.base.get_input_stream().index(),
            |token| token.get_token_index(),
        );
        let expected = base.map_or_else(Vec::new, |base| {
            let set = base.get_expected_tokens(&self.base);
            set.as_slice()
                .iter()
                .flat_map(|interval| interval.a..=interval.b)
                .collect()
        });
        let step = self
            .events
            .last()
            .filter(|event| event.alt.is_none())
            .map(|_| self.events.len() - 1);
        SyntaxError {
            kind,
            token_index,
            expected,
            stack: self.stack(),
            step,
            message: match error.as_ref() {
                ANTLRErrorKind::CustomError(message)
                | ANTLRErrorKind::IllegalStateError(message) => message.clone(),
                _ if kind == ErrorKind::Other => error.to_string(),
                _ => String::new(),
            },
        }
    }
}

/// A walk that names no state, for a prediction the table walk could not
/// follow.
fn unknown_walk() -> TableWalk {
    TableWalk {
        class: None,
        state: 0,
        lookahead: Vec::new(),
        kind: TableKind::Unknown,
        dropped: Vec::new(),
        expected: None,
        guard: None,
    }
}

/// Classifies the state where a generation-time walk stopped.
fn classify_walk(
    table: &StaticDfaTable,
    class: Option<usize>,
    state: usize,
    lookahead: Vec<isize>,
    stopped_on_edge: bool,
) -> TableWalk {
    let entry = &table.states[state];
    let without = |alts: &[i32]| -> Vec<i32> {
        alts.iter()
            .copied()
            .filter(|alt| *alt != entry.accept)
            .collect()
    };
    let guard = entry
        .is_guarded()
        .then_some((entry.guarded_alt, entry.guard_defer_alt));
    let (kind, dropped) = match entry.resolution {
        Some(EscapeResolution::MinAlt) => (TableKind::MinAlt, without(&entry.conflicts)),
        Some(EscapeResolution::MinViableAlt) => {
            (TableKind::MinViableAlt, without(&entry.viable_alts))
        }
        Some(EscapeResolution::SyntaxError) => (TableKind::SyntaxError, Vec::new()),
        Some(EscapeResolution::Guarded) => (TableKind::GuardTaken, Vec::new()),
        None if entry.is_guarded() => (TableKind::GuardTaken, Vec::new()),
        None if entry.is_accept() => (TableKind::Exact, Vec::new()),
        None if entry.is_escape() => (TableKind::Escape, Vec::new()),
        None if !stopped_on_edge && entry.fallback > 0 => (TableKind::Fallback, Vec::new()),
        None => (TableKind::Unknown, Vec::new()),
    };
    let expected = match kind {
        TableKind::Exact | TableKind::MinAlt | TableKind::MinViableAlt => Some(entry.accept),
        TableKind::Fallback => Some(entry.fallback),
        _ => None,
    };
    TableWalk {
        class,
        state,
        lookahead,
        kind,
        dropped,
        expected,
        guard,
    }
}

/// Checks a classified walk against the alternative that the runtime walk
/// returned, and resolves the side a guard took.
fn checked_walk(mut walk: TableWalk, alt: i32) -> TableWalk {
    if let Some((taken, deferred)) = walk.guard {
        walk.kind = if alt == deferred && alt != taken {
            TableKind::GuardDeferred
        } else {
            TableKind::GuardTaken
        };
    } else if walk.expected.is_some_and(|expected| expected != alt) {
        walk.kind = TableKind::Unknown;
        walk.dropped.clear();
    }
    walk
}

/// Whether the decision has an alternative that leaves the block without
/// entering it: the exit of a loop or the skip of an optional block.
fn has_exit_alternative(state: &ATNState) -> bool {
    match state.state_type() {
        ATNStateType::StarLoopEntry | ATNStateType::PlusLoopBack => true,
        _ => state.get_transitions().last().is_some_and(|transition| {
            transition.get_target().state_type() == ATNStateType::BlockEnd
        }),
    }
}

fn is_left_recursive_rule(rule_start: &ATNState) -> bool {
    rule_start
        .try_as::<RuleStartState>()
        .is_some_and(|state| state.is_left_recursive)
}
