// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Shared construction helpers for the runtime ATNs the compiler builds.
//!
//! Both grammar scopes build a [`dbt_antlr_runtime::atn::ATN`] from a
//! `FinalizedAtnGraph`: that ATN is the single ATN representation of a
//! compiled unit, and the tool-side serializers turn it into the word streams
//! that generated recognizers embed. The helpers here keep both constructions
//! identical where the graph is scoped the same way.

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_state::{
    ATNStateRef, BaseATNState, BasicBlockStartState, BasicState, BlockEndState, LoopEndState,
    PlusBlockStartState, PlusLoopBackState, RuleStartState, RuleStopState, StarBlockStartState,
    StarLoopEntryState, StarLoopbackState, TokenStartState,
};
use dbt_antlr_runtime::interval_set::{IntervalSet, IntervalSetBuf};
use dbt_antlr_runtime::transition::{
    ActionTransition, AtomTransition, EpsilonTransition, NotSetTransition,
    PrecedencePredicateTransition, PredicateTransition, RangeTransition, RuleTransition,
    SetTransition, Transition, WildcardTransition,
};

use super::build::{FinalizedAtnGraph, FinalizedTransition, FinalizedTransitionKind};
use super::state_kind::AtnStateKind;

fn state_ref(atn: &ATN, number: Option<usize>) -> ATNStateRef {
    number.map_or(ATNStateRef::invalid(), |number| {
        atn.make_state_ref(usize_to_i32(number))
    })
}

fn state_refs(atn: &ATN, numbers: &[usize]) -> Vec<ATNStateRef> {
    numbers
        .iter()
        .map(|&number| atn.make_state_ref(usize_to_i32(number)))
        .collect()
}

/// Materializes a transition set as a runtime set.
///
/// Runtime set transitions borrow their set for the whole ATN lifetime, so the
/// set is leaked once per set transition. The runtime deserializers leak once
/// per distinct set in the same way.
fn runtime_set(ranges: &[(i32, i32)]) -> &'static IntervalSet {
    let mut set = IntervalSetBuf::new();
    for &(start, stop) in ranges {
        debug_assert!(start <= stop, "transition set range is reversed");
        set.add_range(start, stop);
    }
    set.into_static()
}

/// Adds the rule return edges the deserializers synthesize.
///
/// A return edge runs from the called rule's stop state to the follow state of
/// its call site. Its outermost precedence return is the rule index when the
/// called rule is left recursive and the call carries no precedence, and `-1`
/// otherwise.
pub(crate) fn add_rule_return_edges(atn: &mut ATN) {
    let mut return_edges = Vec::new();
    for state in atn.iter_states() {
        for transition in state.get_transitions() {
            let Some(rule) = transition.try_as::<RuleTransition>() else {
                continue;
            };
            let rule_index = rule.get_target().get_rule_index();
            let index = usize::try_from(rule_index).expect("rule index is not negative");
            let left_recursive = atn.rule_to_start_state[index]
                .try_as::<RuleStartState>()
                .is_some_and(|start| start.is_left_recursive);
            let precedence_return = if left_recursive && rule.precedence() == 0 {
                rule_index
            } else {
                -1
            };
            return_edges.push((
                atn.rule_to_stop_state[index],
                rule.follow_state,
                precedence_return,
            ));
        }
    }
    for (stop, follow, precedence_return) in return_edges {
        atn.get_state_mut(stop.get_state_number())
            .add_transition(EpsilonTransition::create(follow, precedence_return));
    }
}

/// Installs the states, the rule tables, and the decisions of `graph`.
///
/// State numbers are the graph indices, so the states must be installed in
/// graph order. State links that the word formats do not carry keep the
/// placeholder a deserialized ATN holds.
pub(crate) fn build_states(atn: &mut ATN, graph: &FinalizedAtnGraph) {
    atn.alloc_states(graph.states.len());
    for (state_number, state) in graph.states.iter().enumerate() {
        let base = BaseATNState::new(
            usize_to_i32(state_number),
            state.rule_index.map_or(-1, usize_to_i32),
            state_type(state.kind),
        );
        let runtime = match state.kind {
            AtnStateKind::Invalid => continue,
            AtnStateKind::Basic => BasicState::create(base),
            AtnStateKind::RuleStart => {
                let stop = state
                    .rule_index
                    .and_then(|rule_index| graph.rule_stops.get(rule_index).copied());
                RuleStartState::create(base, state_ref(atn, stop), state.left_recursive_rule)
            }
            AtnStateKind::BlockStart => BasicBlockStartState::create(
                base,
                -1,
                state.non_greedy,
                state_ref(atn, state.end_state),
            ),
            // Only loop ends store a loop-back link in the word formats.
            AtnStateKind::PlusBlockStart => PlusBlockStartState::create(
                base,
                -1,
                state.non_greedy,
                state_ref(atn, state.end_state),
                ATNStateRef::invalid(),
            ),
            AtnStateKind::StarBlockStart => StarBlockStartState::create(
                base,
                -1,
                state.non_greedy,
                state_ref(atn, state.end_state),
            ),
            AtnStateKind::TokenStart => TokenStartState::create(base, -1, state.non_greedy),
            AtnStateKind::RuleStop => RuleStopState::create(base),
            AtnStateKind::BlockEnd => BlockEndState::create(base, ATNStateRef::invalid()),
            AtnStateKind::StarLoopBack => StarLoopbackState::create(base),
            AtnStateKind::StarLoopEntry => StarLoopEntryState::create(
                base,
                -1,
                state.non_greedy,
                ATNStateRef::invalid(),
                false,
            ),
            AtnStateKind::PlusLoopBack => PlusLoopBackState::create(base, -1, state.non_greedy),
            AtnStateKind::LoopEnd => {
                LoopEndState::create(base, state_ref(atn, state.loop_back_state))
            }
        };
        atn.add_state(runtime);
    }

    atn.rule_to_start_state = state_refs(atn, &graph.rule_starts);
    atn.rule_to_stop_state = state_refs(atn, &graph.rule_stops);
    for (decision_number, &decision) in graph.decisions.iter().enumerate() {
        let reference = atn.make_state_ref(usize_to_i32(decision));
        atn.decision_to_state.push(reference);
        if let Some(field) = atn.get_state_mut(usize_to_i32(decision)).get_decision_mut() {
            *field = usize_to_i32(decision_number);
        }
    }
}

/// Numeric state type id of a graph state kind, as the word formats encode it.
pub(crate) const fn state_type(kind: AtnStateKind) -> i32 {
    match kind {
        AtnStateKind::Invalid => 0,
        AtnStateKind::Basic => 1,
        AtnStateKind::RuleStart => 2,
        AtnStateKind::BlockStart => 3,
        AtnStateKind::PlusBlockStart => 4,
        AtnStateKind::StarBlockStart => 5,
        AtnStateKind::TokenStart => 6,
        AtnStateKind::RuleStop => 7,
        AtnStateKind::BlockEnd => 8,
        AtnStateKind::StarLoopBack => 9,
        AtnStateKind::StarLoopEntry => 10,
        AtnStateKind::PlusLoopBack => 11,
        AtnStateKind::LoopEnd => 12,
    }
}

pub(crate) fn usize_to_i32(value: usize) -> i32 {
    i32::try_from(value).expect("ATN value exceeds i32")
}

/// Builds the runtime transition of one finalized graph edge.
pub(crate) fn transition(
    atn: &ATN,
    transition: &FinalizedTransition,
    target: ATNStateRef,
) -> Transition {
    match &transition.kind {
        FinalizedTransitionKind::Epsilon => EpsilonTransition::create(target, 0),
        FinalizedTransitionKind::Atom(label) => AtomTransition::create(target, *label),
        FinalizedTransitionKind::Range { start, stop } => {
            RangeTransition::create(target, *start, *stop)
        }
        FinalizedTransitionKind::Set(ranges) => SetTransition::create(target, runtime_set(ranges)),
        FinalizedTransitionKind::NotSet(ranges) => {
            NotSetTransition::create(target, runtime_set(ranges))
        }
        FinalizedTransitionKind::Wildcard => WildcardTransition::create(target),
        FinalizedTransitionKind::Rule {
            rule_index,
            follow,
            precedence,
            ..
        } => RuleTransition::create(
            target,
            atn.make_state_ref(usize_to_i32(*follow)),
            usize_to_i32(*rule_index),
            *precedence,
        ),
        FinalizedTransitionKind::Predicate {
            rule_index,
            predicate_index,
            context_dependent,
        } => PredicateTransition::create(
            target,
            *context_dependent,
            usize_to_i32(*rule_index),
            usize_to_i32(*predicate_index),
        ),
        FinalizedTransitionKind::Action {
            rule_index,
            action_index,
            context_dependent,
        } => ActionTransition::create(
            target,
            *context_dependent,
            usize_to_i32(*rule_index),
            action_index.map_or(-1, usize_to_i32),
            0,
        ),
        FinalizedTransitionKind::Precedence(precedence) => {
            PrecedencePredicateTransition::create(target, *precedence)
        }
    }
}
