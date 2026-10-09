// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Lexer Abstract Transition Network construction containers.
//!
//! This module is the tool-side construction copy of the vendored runtime's
//! `atn` lexer containers. Codegen builds the graph here, serializes it to
//! the Java word stream itself (`lexer::encode_lexer_atn`), and never
//! simulates it, so the simulator-only read API of the vendored types
//! (transition matching, epsilon/tail-call queries) is not reproduced. The
//! word format is identical, so streams built here deserialize in the
//! runtime unchanged.

use super::interval_set::IntervalSet;
use super::state_kind::AtnStateKind;

#[derive(Clone, Copy)]
struct TailCallSite {
    start: usize,
    stop: usize,
    rule_index: usize,
    state_count: usize,
}

#[derive(Default)]
struct TailCallScratch {
    marks: Vec<u8>,
    work: Vec<(usize, bool)>,
    successors: Vec<usize>,
}

fn plain_epsilon_tail_call<StateKind, StateRule, PushSuccessors>(
    site: TailCallSite,
    scratch: &mut TailCallScratch,
    state_kind: StateKind,
    state_rule_index: StateRule,
    push_successors: PushSuccessors,
) -> bool
where
    StateKind: Fn(usize) -> AtnStateKind,
    StateRule: Fn(usize) -> Option<usize>,
    PushSuccessors: Fn(usize, &mut Vec<usize>) -> bool,
{
    let TailCallSite {
        start,
        stop,
        rule_index,
        state_count,
    } = site;
    if start >= state_count
        || stop >= state_count
        || state_kind(stop) != AtnStateKind::RuleStop
        || state_rule_index(stop) != Some(rule_index)
    {
        return false;
    }

    // Reject cycles as well as semantic, consuming, nested-rule, and dead-end
    // paths. Every continuation must finish the enclosing rule without
    // observable work.
    let TailCallScratch {
        marks,
        work,
        successors,
    } = scratch;
    marks.clear();
    marks.resize(state_count, 0);
    work.clear();
    work.push((start, false));
    successors.clear();
    while let Some((state, exiting)) = work.pop() {
        if state == stop {
            continue;
        }
        if state >= state_count {
            return false;
        }
        if exiting {
            marks[state] = 2;
            continue;
        }
        match marks[state] {
            1 => return false,
            2 => continue,
            _ => {}
        }
        if state_kind(state) == AtnStateKind::RuleStop
            || state_rule_index(state) != Some(rule_index)
        {
            return false;
        }
        successors.clear();
        if !push_successors(state, successors) || successors.is_empty() {
            return false;
        }
        marks[state] = 1;
        work.push((state, true));
        work.extend(successors.iter().copied().map(|target| (target, false)));
    }
    true
}

/// Lexer Abstract Transition Network under construction.
///
/// The structure keeps the state graph plus ANTLR side tables such as
/// rule-to-start, rule-to-token, mode-to-start, decisions, and actions. Parser
/// ATNs never use this object-graph representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LexerAtn {
    max_token_type: i32,
    states: Vec<LexerAtnState>,
    rule_to_start_state: Vec<usize>,
    rule_to_stop_state: Vec<usize>,
    rule_to_token_type: Vec<i32>,
    mode_to_start_state: Vec<usize>,
    decision_to_state: Vec<usize>,
    lexer_actions: Vec<LexerAction>,
}

impl LexerAtn {
    /// Creates an empty lexer ATN with the maximum token type of the grammar.
    pub(crate) const fn new(max_token_type: i32) -> Self {
        Self {
            max_token_type,
            states: Vec::new(),
            rule_to_start_state: Vec::new(),
            rule_to_stop_state: Vec::new(),
            rule_to_token_type: Vec::new(),
            mode_to_start_state: Vec::new(),
            decision_to_state: Vec::new(),
            lexer_actions: Vec::new(),
        }
    }

    pub(crate) const fn max_token_type(&self) -> i32 {
        self.max_token_type
    }

    pub(crate) fn states(&self) -> &[LexerAtnState] {
        &self.states
    }

    pub(crate) fn state(&self, state_number: usize) -> Option<&LexerAtnState> {
        self.states.get(state_number)
    }

    pub(crate) fn state_mut(&mut self, state_number: usize) -> Option<&mut LexerAtnState> {
        self.states.get_mut(state_number)
    }

    /// Appends a state and returns the state number assigned by insertion
    /// order.
    pub(crate) fn add_state(&mut self, state: LexerAtnState) -> usize {
        let index = self.states.len();
        self.states.push(state);
        index
    }

    pub(crate) fn decision_to_state(&self) -> &[usize] {
        &self.decision_to_state
    }

    pub(crate) fn add_decision_state(&mut self, state_number: usize) {
        self.decision_to_state.push(state_number);
    }

    pub(crate) fn rule_to_start_state(&self) -> &[usize] {
        &self.rule_to_start_state
    }

    pub(crate) fn set_rule_to_start_state(&mut self, rule_to_start_state: Vec<usize>) {
        self.rule_to_start_state = rule_to_start_state;
    }

    pub(crate) fn rule_to_stop_state(&self) -> &[usize] {
        &self.rule_to_stop_state
    }

    pub(crate) fn set_rule_to_stop_state(&mut self, rule_to_stop_state: Vec<usize>) {
        self.rule_to_stop_state = rule_to_stop_state;
    }

    pub(crate) fn rule_to_token_type(&self) -> &[i32] {
        &self.rule_to_token_type
    }

    pub(crate) fn set_rule_to_token_type(&mut self, rule_to_token_type: Vec<i32>) {
        self.rule_to_token_type = rule_to_token_type;
    }

    pub(crate) fn mode_to_start_state(&self) -> &[usize] {
        &self.mode_to_start_state
    }

    pub(crate) fn add_mode_start_state(&mut self, state_number: usize) {
        self.mode_to_start_state.push(state_number);
    }

    pub(crate) fn lexer_actions(&self) -> &[LexerAction] {
        &self.lexer_actions
    }

    pub(crate) fn set_lexer_actions(&mut self, lexer_actions: Vec<LexerAction>) {
        self.lexer_actions = lexer_actions;
    }

    /// Recomputes conservative tail-call markers after the graph is complete.
    ///
    /// Call this after every transition and derived rule-return edge has been
    /// added. Any later mutation through [`Self::add_state`],
    /// [`Self::state_mut`], or [`LexerAtnState::add_transition`] invalidates the
    /// stored markers.
    pub(crate) fn identify_tail_calls(&mut self) {
        let mut tail_calls = Vec::new();
        let mut scratch = TailCallScratch::default();
        for source in 0..self.states.len() {
            for index in 0..self.states[source].transitions.len() {
                let follow_state = match &self.states[source].transitions[index] {
                    LexerTransition::Rule { follow_state, .. } => *follow_state,
                    _ => continue,
                };
                tail_calls.push((
                    source,
                    index,
                    self.tail_call_follow_is_safe(source, follow_state, &mut scratch),
                ));
            }
        }
        for (source, index, tail_call) in tail_calls {
            if let Some(LexerTransition::Rule {
                tail_call: marker, ..
            }) = self
                .states
                .get_mut(source)
                .and_then(|state| state.transitions.get_mut(index))
            {
                *marker = tail_call;
            }
        }
    }

    fn tail_call_follow_is_safe(
        &self,
        source: usize,
        start: usize,
        scratch: &mut TailCallScratch,
    ) -> bool {
        let Some(rule_index) = self.states.get(source).and_then(|state| state.rule_index) else {
            return false;
        };
        let Some(&stop) = self.rule_to_stop_state.get(rule_index) else {
            return false;
        };
        plain_epsilon_tail_call(
            TailCallSite {
                start,
                stop,
                rule_index,
                state_count: self.states.len(),
            },
            scratch,
            |state| self.states[state].kind,
            |state| self.states[state].rule_index,
            |state, successors| {
                for transition in &self.states[state].transitions {
                    let LexerTransition::Epsilon { target } = transition else {
                        return false;
                    };
                    successors.push(*target);
                }
                true
            },
        )
    }
}

/// A node in the lexer ATN graph.
///
/// Some ANTLR state subclasses carry references to paired states, such as a
/// block-start state's end state or a loop-end state's loop-back state. This
/// representation stores those links as state numbers so the graph remains easy
/// to clone and serialize.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LexerAtnState {
    pub(crate) state_number: usize,
    pub(crate) rule_index: Option<usize>,
    pub(crate) kind: AtnStateKind,
    pub(crate) end_state: Option<usize>,
    pub(crate) loop_back_state: Option<usize>,
    pub(crate) non_greedy: bool,
    pub(crate) left_recursive_rule: bool,
    pub(crate) transitions: Vec<LexerTransition>,
}

impl LexerAtnState {
    /// Creates an ATN state with no rule index and no outgoing transitions.
    pub(crate) const fn new(state_number: usize, kind: AtnStateKind) -> Self {
        Self {
            state_number,
            rule_index: None,
            kind,
            end_state: None,
            loop_back_state: None,
            non_greedy: false,
            left_recursive_rule: false,
            transitions: Vec::new(),
        }
    }

    #[must_use]
    pub(crate) const fn with_rule_index(mut self, rule_index: usize) -> Self {
        self.rule_index = Some(rule_index);
        self
    }

    /// Adds an outgoing transition in serialized order.
    ///
    /// Transition order matters for alternatives and lexer priority, so the
    /// builder preserves the order in which transitions are added.
    pub(crate) fn add_transition(&mut self, transition: LexerTransition) {
        self.transitions.push(transition);
    }
}

/// Edge between two lexer ATN states.
///
/// Epsilon-like transitions do not consume input. Matching transitions compare
/// the current input symbol against an atom, range, set, negated set, or
/// wildcard.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LexerTransition {
    Epsilon {
        target: usize,
    },
    Atom {
        target: usize,
        label: i32,
    },
    Range {
        target: usize,
        start: i32,
        stop: i32,
    },
    Set {
        target: usize,
        set: IntervalSet,
    },
    NotSet {
        target: usize,
        set: IntervalSet,
    },
    Wildcard {
        target: usize,
    },
    Rule {
        target: usize,
        rule_index: usize,
        follow_state: usize,
        precedence: i32,
        tail_call: bool,
    },
    Predicate {
        target: usize,
        rule_index: usize,
        pred_index: usize,
        context_dependent: bool,
    },
    Action {
        target: usize,
        rule_index: usize,
        action_index: Option<usize>,
        context_dependent: bool,
    },
    Precedence {
        target: usize,
        precedence: i32,
    },
}

/// Lexer action attached to an action transition.
///
/// These actions are grammar-independent operations generated by ANTLR's lexer
/// commands (`skip`, `more`, `type`, `channel`, `pushMode`, `popMode`, and
/// `mode`). Custom embedded actions are represented but intentionally inert
/// until a generated semantic-action hook exists.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LexerAction {
    Channel(i32),
    Custom { rule_index: i32, action_index: i32 },
    Mode(i32),
    More,
    PopMode,
    PushMode(i32),
    Skip,
    Type(i32),
}
