// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The operator-loop block of a left-recursive rule, and the token
//! obligations of its positions.
//!
//! A precedence-class table predicts the iterate/exit conflict of a
//! left-recursive rule's loop. The classifier resolves that conflict to
//! *iterate* without exactness. The argument: every exit derivation of a
//! sentence maps onto an iterate derivation of the same sentence. In the exit
//! reading, an enclosing invocation of this same loop consumes the next
//! operator. If the inner loop iterates instead, it consumes that operator and
//! then leaves through the free loop exit into the same caller continuation.
//!
//! That argument only holds while the exit lineage's tokens are the loop's own
//! tokens. Four facts decide it:
//!
//! - **Operator loop** ([`LoopBlock::is_loop_consumption`]): the loop
//!   decision, the operator alternatives, and the follow states of their
//!   operands. A boundary-popped configuration is in the frame that it popped
//!   into, or in an invocation that it started from that frame. In the popped
//!   frame, consumption at an operator-loop position belongs to an enclosing
//!   invocation of the loop. In an invocation that an operator-loop position
//!   started (an operand descent), consumption also belongs to the loop. All
//!   other consumption is *foreign*, and the substitution does not apply.
//!   - A new invocation of the rule after the exit is foreign: in
//!     `s : e e ';'` the second `e` can read the `'-'` that the first `e` reads
//!     as an operator.
//!   - A base alternative of an enclosing invocation is foreign too: in
//!     `e : ... | 'pair' e e` the second `e` of `pair` can read that `'-'`.
//!
//!   [`LoopBlock::is_foreign_call`] marks the call that starts such an
//!   invocation with [`FOREIGN_LINEAGE_TAINT`], because widening and merged
//!   contexts can remove its frame later.
//! - **Pending obligations** ([`LoopBlock::has_obligated_return`]): positions
//!   that cannot reach the loop decision by epsilon alone. Such a position owes
//!   a token of the current alternative. An enclosing frame that sits there
//!   therefore owes something the iterate reading does not owe. Example: the
//!   list element of `expr (',' expr)* '>>' expr` returns into a pending
//!   `','`/`'>>'`. Such an obligation vetoes the substitution. A frame that can
//!   skip to the decision owes only the next operator, which the iterate
//!   reading owes too, and is benign.
//! - **Midway returns**: the boundary chase can return a configuration into
//!   the middle of an alternative of an enclosing invocation, with no frame
//!   left. In `e '?' e '-' ID` the middle operand returns before `'-'`, and in
//!   `e '?' e ('-' ID)?` before an optional suffix. That alternative owns the
//!   next tokens. The chase marks the configuration with [`MIDWAY_TAINT`],
//!   and the mark stays until the configuration reaches the loop decision
//!   again. A return that leads straight back to the loop decision, as after
//!   the right operand of `e '-' e`, gets no mark.
//! - **Simple guarded alternatives** ([`LoopBlock::guards_are_simple`]): at
//!   most one rule invocation, and no internal iteration, between the
//!   precedence predicate and the loop end. The substitution maps "operator
//!   consumed, operand in progress" onto the inner loop's iterate. That map is
//!   valid only while the operand completes the recursive structure of the
//!   alternative. A compound alternative breaks it: `expr (',' expr)* '>>'
//!   expr` would complete an inner construct as a list element and then fail.
//!
//! Obligations of *phantom-born* frames will become guard roots when the
//! stack-guard machinery lands (Phase 5.3). A phantom-born frame is a frame
//! value that only a wildcard lineage produced. Until that machinery lands,
//! every obligation is a hard veto. This module therefore resolves strictly
//! fewer conflicts than the reference classifier, and never more.

use std::collections::HashSet;

use dbt_antlr_runtime::analysis::{
    ATN, ATNConfig, ATNStateRef, ATNStateType, BitSet, FOREIGN_CONSUME_TAINT,
    FOREIGN_LINEAGE_TAINT, MIDWAY_TAINT, PrecedencePredicateTransition, PredictionContextRef,
    RuleTransition, WIDENED_TAINT,
};

use super::closure::{Taint, in_popped_frame, positions_owing_input, rule_positions, taint_of};

/// The alternative of the loop's iterate branch.
pub(super) const ITERATE_ALT: i32 = 1;
/// The alternative of the loop's exit branch.
pub(super) const EXIT_ALT: i32 = 2;

/// The exit lineage consumed a token outside the operator loop.
pub(super) const EXIT_FOREIGN: i32 = 1;
/// The exit lineage holds a frame that owes tokens the iterate reading does
/// not also owe.
pub(super) const EXIT_OBLIGATED: i32 = 2;
/// Iterate viability is attested by a configuration that is neither widened
/// nor foreign.
pub(super) const ITERATE_REAL: i32 = 4;

/// The operator loop of one left-recursive rule, and the token obligations of
/// its positions.
#[derive(Clone, Debug)]
pub(super) struct LoopBlock {
    rule_index: i32,
    /// Positions of the operator loop: the loop decision, the operator
    /// alternatives, and the follow states of their operands.
    operator_states: BitSet,
    /// Rule-local positions that cannot epsilon-reach the loop decision, and
    /// so owe a token of the current alternative.
    pending_states: BitSet,
    /// Rule-local positions from which the current alternative can consume or
    /// call before an epsilon path reaches the loop decision.
    midway_states: BitSet,
    /// Whether every guarded (precedence-bearing) alternative is simple: at
    /// most one rule invocation and no internal iteration.
    simple: bool,
}

impl LoopBlock {
    /// Analyses the loop block of the rule owning `decision_state`.
    pub(super) fn of(atn: &ATN, decision_state: ATNStateRef) -> Self {
        let rule_index = decision_state.get_rule_index();
        let operator_states = operator_states(decision_state);
        let pending_states = pending_states(atn, decision_state, rule_index);
        let midway_states = positions_owing_input(atn, rule_index, decision_state);
        let simple = guards_are_simple(atn, decision_state, rule_index);
        Self {
            rule_index,
            operator_states,
            pending_states,
            midway_states,
            simple,
        }
    }

    /// The iterate/exit substitution attestations of one conflict set (see
    /// [`EXIT_FOREIGN`], [`EXIT_OBLIGATED`] and [`ITERATE_REAL`]).
    ///
    /// The substitution resolves to iterate only when the result is exactly
    /// [`ITERATE_REAL`]: the exit lineage stayed inside the loop and owes
    /// nothing the iterate reading does not owe, and iterate viability is
    /// attested by a configuration that is neither widened nor foreign.
    pub(super) fn iterate_exit_attestation(&self, configs: &[ATNConfig<'_>], taint: &Taint) -> i32 {
        let mut attestation = 0;
        for config in configs {
            let taint = taint_of(taint, config);
            if config.get_alt() == EXIT_ALT {
                if taint & FOREIGN_CONSUME_TAINT != 0 {
                    attestation |= EXIT_FOREIGN;
                }
                if self.has_obligated_return(config.get_context()) {
                    attestation |= EXIT_OBLIGATED;
                }
            }
            if config.get_alt() == ITERATE_ALT
                && taint & (WIDENED_TAINT | FOREIGN_CONSUME_TAINT) == 0
            {
                attestation |= ITERATE_REAL;
            }
        }
        attestation
    }

    /// Whether every guarded alternative of the rule is simple (see the module
    /// documentation).
    pub(super) fn guards_are_simple(&self) -> bool {
        self.simple
    }

    /// Whether a token that a boundary-popped configuration consumes at
    /// `state` belongs to the operator loop (see the module documentation).
    ///
    /// A context with an empty path puts the configuration in the frame that
    /// it popped into, so the position decides. Any other context puts it in
    /// an invocation that it started from that frame. That invocation belongs
    /// to the loop unless `taint` holds [`FOREIGN_LINEAGE_TAINT`]. A
    /// configuration with [`MIDWAY_TAINT`] consumes for an enclosing
    /// alternative.
    pub(super) fn is_loop_consumption(
        &self,
        state: ATNStateRef,
        context: Option<PredictionContextRef<'_>>,
        taint: i32,
    ) -> bool {
        if taint & (FOREIGN_LINEAGE_TAINT | MIDWAY_TAINT) != 0 {
            return false;
        }
        !in_popped_frame(context)
            || self
                .operator_states
                .contains(state.get_state_number() as usize)
    }

    /// Whether a rule call of a boundary-popped configuration at `state`
    /// starts an invocation that is foreign to the loop: the call leaves the
    /// frame that the configuration popped into from a position outside the
    /// operator loop.
    pub(super) fn is_foreign_call(
        &self,
        state: ATNStateRef,
        context: Option<PredictionContextRef<'_>>,
    ) -> bool {
        in_popped_frame(context)
            && !self
                .operator_states
                .contains(state.get_state_number() as usize)
    }

    /// Whether the context holds a same-rule return into a pending position -
    /// a frame that owes tokens the iterate reading does not also owe.
    pub(super) fn has_obligated_return(&self, context: Option<PredictionContextRef<'_>>) -> bool {
        let Some(context) = context else {
            return false;
        };
        let mut visited = HashSet::new();
        self.obligation_in(&context, &mut visited)
    }

    /// Whether a boundary chase that returns into `state` lands in the middle
    /// of an alternative that can still consume or call before it reaches the
    /// loop decision (see the module documentation).
    pub(super) fn is_midway(&self, state: ATNStateRef) -> bool {
        self.midway_states
            .contains(state.get_state_number() as usize)
    }

    /// Whether `state` is a rule-local position that owes a token of the
    /// current alternative.
    pub(super) fn is_pending(&self, state: ATNStateRef) -> bool {
        self.pending_states
            .contains(state.get_state_number() as usize)
    }

    fn obligation_in(
        &self,
        context: &PredictionContextRef<'_>,
        visited: &mut HashSet<*const ()>,
    ) -> bool {
        if context.is_empty() || !visited.insert(pointer_key(context)) {
            return false;
        }
        for index in 0..context.length() {
            let return_state = context.get_return_state(index);
            if return_state == ATNStateRef::empty_return() {
                continue;
            }
            if return_state.get_rule_index() == self.rule_index && self.is_pending(return_state) {
                return true;
            }
            let Some(parent) = context.get_parent(index) else {
                continue;
            };
            if self.obligation_in(&parent, visited) {
                return true;
            }
        }
        false
    }
}

/// Whether a set of live alternatives is exactly the iterate/exit pair of a
/// loop decision.
pub(super) fn is_iterate_exit_pair(alts: &BitSet) -> bool {
    alts.len() == 2 && alts.contains(ITERATE_ALT as usize) && alts.contains(EXIT_ALT as usize)
}

/// A context node's identity, for cycle-safe recursion over a shared DAG.
fn pointer_key(context: &PredictionContextRef<'_>) -> *const () {
    context.as_ref() as *const _ as *const ()
}

/// Every position of the operator loop: the loop decision, the operator
/// alternatives, and the follow states of their operands. Rule references
/// contribute their follow state, never the callee's body - an operand runs
/// with an operator-loop frame in its context, so its own consumption is
/// attributed by [`LoopBlock::is_loop_consumption`]. The base alternatives are
/// not part of the loop: an enclosing invocation that continues a base
/// alternative (`'(' e ')'`, `'pair' e e`) consumes for that alternative, not
/// for the loop.
fn operator_states(decision_state: ATNStateRef) -> BitSet {
    rule_positions(decision_state, None)
}

/// Rule-local positions that cannot reach the loop decision by epsilon alone.
///
/// Such a position owes a token of the current alternative; the complement
/// owes nothing, because it can step onto the decision and let the lookahead
/// decide (an optional trailing token like DuckDB's `'(+)'?` is not an
/// obligation, but `Send`'s `'>>'` is).
fn pending_states(atn: &ATN, decision_state: ATNStateRef, rule_index: i32) -> BitSet {
    let states_count = atn.states_count();
    let mut reverse_epsilon: Vec<Vec<ATNStateRef>> = vec![Vec::new(); states_count];
    for state in atn.iter_states() {
        if state.get_rule_index() != rule_index {
            continue;
        }
        for transition in state.get_transitions() {
            if !transition.is_epsilon()
                || transition.try_as::<RuleTransition>().is_some()
                || transition.get_target().get_rule_index() != rule_index
            {
                continue;
            }
            let target = transition.get_target().get_state_number() as usize;
            reverse_epsilon[target].push(atn.make_state_ref(state.get_state_number()));
        }
    }

    let mut skips: BitSet = BitSet::new();
    let mut work: Vec<ATNStateRef> = vec![decision_state];
    skips.insert(decision_state.get_state_number() as usize);
    while let Some(state) = work.pop() {
        for predecessor in &reverse_epsilon[state.get_state_number() as usize] {
            if skips.insert(predecessor.get_state_number() as usize) {
                work.push(*predecessor);
            }
        }
    }

    let mut pending = BitSet::new();
    for state in atn.iter_states() {
        if state.get_rule_index() != rule_index || state.state_type() == ATNStateType::RuleStop {
            continue;
        }
        if !skips.contains(state.get_state_number() as usize) {
            pending.insert(state.get_state_number() as usize);
        }
    }
    pending
}

/// Whether every guarded alternative is simple: at most one rule invocation
/// and no internal iteration between its precedence predicate and the loop end.
fn guards_are_simple(atn: &ATN, decision_state: ATNStateRef, rule_index: i32) -> bool {
    let mut pending: Vec<ATNStateRef> = Vec::new();
    for state in atn.iter_states() {
        if state.get_rule_index() != rule_index {
            continue;
        }
        for transition in state.get_transitions() {
            if transition
                .try_as::<PrecedencePredicateTransition>()
                .is_some()
            {
                pending.push(transition.get_target());
            }
        }
    }

    while let Some(start) = pending.pop() {
        let mut visited: HashSet<i32> = HashSet::new();
        let mut invocations = 0;
        let mut work: Vec<ATNStateRef> = vec![start];
        while let Some(current) = work.pop() {
            if current == decision_state
                || current.get_rule_index() != rule_index
                || !visited.insert(current.get_state_number())
            {
                continue;
            }
            if matches!(
                current.state_type(),
                ATNStateType::StarBlockStart | ATNStateType::PlusBlockStart
            ) {
                return false;
            }
            for transition in current.get_transitions() {
                match transition.try_as::<RuleTransition>() {
                    Some(rule) => {
                        invocations += 1;
                        if invocations > 1 {
                            return false;
                        }
                        work.push(rule.follow_state);
                    }
                    None => work.push(transition.get_target()),
                }
            }
        }
    }
    true
}
