// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Optional-postfix decisions (`X Y?`) and their take/skip conflict.
//!
//! A two-alternative block where alternative 1 enters a postfix element and
//! alternative 2 skips straight to the block end is a dangling-else shape. Its
//! take/skip conflict resolves to *take* when the skip reading cannot win on
//! its own. In that case every token the skip reading consumes is also
//! consumed by the take structure, so both readings meet at the same block
//! end.
//!
//! The rule needs three facts about a conflict set:
//!
//! - **Mirror**: every skip configuration that descended into another rule has
//!   its deepest context frame return into the decision's block end. Such a
//!   skip re-entered through this decision's own take path. A skip that
//!   bottoms out anywhere else consumed tokens outside the discipline.
//! - **Foreign consumption**: no skip configuration is marked
//!   [`FOREIGN_CONSUME_TAINT`]. A marked skip consumed at a position that is
//!   not the decision rule's own. The alias family is the canonical shape: its
//!   clause continuations (`LIMIT`, `FROM`, ...) let skip genuinely win.
//! - **Real take**: a take configuration that is neither widened nor foreign
//!   attests that taking is viable.
//!
//! The analyzer also rejects shapes where a mandatory element follows the
//! optional one and its FIRST set intersects the optional's. There the skipped
//! input can serve that element instead (`TRIM(x)`, `IN foo`), so skip
//! genuinely wins and the rule must not apply.

use std::collections::HashSet;

use dbt_antlr_runtime::analysis::{
    ATN, ATNStateRef, ATNStateType, BasicBlockStartState, BitSet, IntervalSetBuf, NotSetTransition,
    PlusBlockStartState, PredictionContextRef, RuleTransition, StarBlockStartState,
    TOKEN_MIN_USER_TOKEN_TYPE, TransitionType,
};

/// The optional-postfix shape of one decision.
#[derive(Clone, Copy, Debug)]
pub(super) struct PostfixShape {
    /// State number of the decision block's end state. The mirror check
    /// requires every descended skip configuration's deepest frame to return
    /// into this state.
    pub(super) block_end_state: ATNStateRef,
}

impl PostfixShape {
    /// Detects the optional-postfix shape of a decision, or returns `None`.
    pub(super) fn analyze(atn: &ATN, decision_state: ATNStateRef) -> Option<Self> {
        let block = block_end_state(atn, decision_state)?;
        let transitions = decision_state.get_transitions();
        if transitions.len() != 2 {
            return None;
        }
        // Alternative 2 skips: an epsilon-only path to the block end.
        if !epsilon_path_to(transitions[1].get_target(), block) {
            return None;
        }
        // Alternative 1 heads into a postfix element.
        let first = first_element(transitions[0].get_target())?;
        let postfix_rule = first
            .get_transitions()
            .iter()
            .find_map(|transition| transition.try_as::<RuleTransition>())
            .map_or(-1, |rule| rule.get_target().get_rule_index());
        let compound = postfix_rule < 0;
        if compound && !is_block_start(first.state_type()) {
            return None;
        }

        // Starvation check: a mandatory element after the optional whose FIRST
        // set intersects the optional's can be served by the skipped input, so
        // skip genuinely wins there.
        let rule_stop = atn.rule_to_stop_state[decision_state.get_rule_index() as usize];
        let continuation = look(atn, block, rule_stop);
        if !continuation.epsilon {
            let take = look(atn, transitions[0].get_target(), block);
            if continuation
                .tokens
                .intersection(&take.tokens)
                .next()
                .is_some()
            {
                return None;
            }
        }

        Some(Self {
            block_end_state: block,
        })
    }
}

/// Whether every context frame adjacent to the empty context (the deepest
/// frame of each context branch) is a return into `end_state`.
///
/// A skip configuration mirrors the take reading only when it re-entered the
/// decision through this decision's own take path. A branch that reaches the
/// empty context without such a frame consumed tokens fully outside the
/// decision's discipline.
pub(super) fn bottom_frame_is(
    context: PredictionContextRef<'_>,
    end_state: ATNStateRef,
    visited: &mut HashSet<*const ()>,
) -> bool {
    if context.is_empty() {
        return false;
    }
    if !visited.insert(context.as_ref() as *const _ as *const ()) {
        return true;
    }
    for index in 0..context.length() {
        let return_state = context.get_return_state(index);
        match context.get_parent(index) {
            Some(parent) if !parent.is_empty() => {
                if !bottom_frame_is(parent, end_state, visited) {
                    return false;
                }
            }
            _ => {
                if return_state != end_state {
                    return false;
                }
            }
        }
    }
    true
}

/// The block start state's end state, when `state` starts a block.
fn block_end_state(atn: &ATN, state: ATNStateRef) -> Option<ATNStateRef> {
    let number = state.get_state_number();
    if number < 0 {
        return None;
    }
    let state = atn.get_state(number);
    match state.state_type() {
        ATNStateType::BasicBlockStart => state
            .try_as::<BasicBlockStartState>()
            .map(|start| start.end_state),
        ATNStateType::StarBlockStart => state
            .try_as::<StarBlockStartState>()
            .map(|start| start.end_state),
        ATNStateType::PlusBlockStart => state
            .try_as::<PlusBlockStartState>()
            .map(|start| start.end_state),
        _ => None,
    }
}

fn is_block_start(kind: ATNStateType) -> bool {
    matches!(
        kind,
        ATNStateType::BasicBlockStart | ATNStateType::StarBlockStart | ATNStateType::PlusBlockStart
    )
}

/// Whether an epsilon-only path leads from `from` to `to`.
fn epsilon_path_to(from: ATNStateRef, to: ATNStateRef) -> bool {
    let mut visited: BitSet = BitSet::new();
    let mut work = vec![from];
    while let Some(state) = work.pop() {
        if state == to {
            return true;
        }
        if state.get_state_number() < 0 || !visited.insert(state.get_state_number() as usize) {
            continue;
        }
        for transition in state.get_transitions() {
            if transition.is_epsilon() {
                work.push(transition.get_target());
            }
        }
    }
    false
}

/// The first state with a non-epsilon outgoing transition reachable over
/// epsilon links. A rule invocation is an element, not an epsilon link.
fn first_element(start: ATNStateRef) -> Option<ATNStateRef> {
    let mut visited: BitSet = BitSet::new();
    let mut work = vec![start];
    while let Some(state) = work.pop() {
        if state.get_state_number() < 0 || !visited.insert(state.get_state_number() as usize) {
            continue;
        }
        for transition in state.get_transitions() {
            if transition.try_as::<RuleTransition>().is_some() || !transition.is_epsilon() {
                return Some(state);
            }
            work.push(transition.get_target());
        }
    }
    None
}

/// The lookahead of the states reachable from `start` up to the end of its
/// rule: the token types they can start with, and whether they can reach the
/// rule end without consuming a token.
struct Look {
    tokens: BitSet,
    epsilon: bool,
}

/// `LOOK(start)` until `stop` (inclusive) or the end of the start state's rule,
/// predicates transparent.
fn look(atn: &ATN, start: ATNStateRef, stop: ATNStateRef) -> Look {
    let mut result = Look {
        tokens: BitSet::new(),
        epsilon: false,
    };
    let mut visited: HashSet<(i32, usize)> = HashSet::new();
    let mut work: Vec<(ATNStateRef, Vec<ATNStateRef>)> = vec![(start, Vec::new())];
    while let Some((state, returns)) = work.pop() {
        let number = state.get_state_number();
        if number < 0 {
            continue;
        }
        if !visited.insert((number, returns.len())) {
            continue;
        }
        if state == stop {
            result.epsilon = true;
            continue;
        }
        if state.state_type() == ATNStateType::RuleStop {
            match returns.last() {
                Some(follow) => {
                    let mut next = returns.clone();
                    next.pop();
                    work.push((*follow, next));
                }
                None => result.epsilon = true,
            }
            continue;
        }
        for transition in state.get_transitions() {
            if let Some(rule) = transition.try_as::<RuleTransition>() {
                let mut next = returns.clone();
                next.push(rule.follow_state);
                work.push((transition.get_target(), next));
                continue;
            }
            if transition.is_epsilon() {
                work.push((transition.get_target(), returns.clone()));
                continue;
            }
            let label: IntervalSetBuf = match transition.transition_type() {
                TransitionType::NotSet => transition
                    .try_as::<NotSetTransition>()
                    .expect("not-set transition")
                    .get_label()
                    .expect("not-set label")
                    .complement(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type),
                TransitionType::Wildcard => {
                    let mut all = IntervalSetBuf::new();
                    if atn.max_token_type >= TOKEN_MIN_USER_TOKEN_TYPE {
                        all.add_range(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type);
                    }
                    all
                }
                _ => match transition.get_label() {
                    Some(label) => label.to_interval_set_buf(),
                    None => continue,
                },
            };
            add_intervals(&mut result.tokens, &label);
        }
    }
    result
}

fn add_intervals(tokens: &mut BitSet, label: &IntervalSetBuf) {
    for interval in label.as_interval_set().as_slice() {
        for token in interval.a..=interval.b {
            if token >= 0 {
                tokens.insert(token as usize);
            }
        }
    }
}
