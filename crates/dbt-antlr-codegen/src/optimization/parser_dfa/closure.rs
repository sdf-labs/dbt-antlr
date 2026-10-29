// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Build-time SLL closure over the ATN.
//!
//! The lazy simulator's closure assumes there is an input stream: recursion is
//! cut off by the tokens running out, predicates are evaluated against the
//! live parser, and a rule stop pops the caller frame that pushed it. None of
//! that holds at build time, so this module runs the same closure *shape* over
//! a different policy:
//!
//! - **Widening.** A rule call whose returned context would exceed the depth
//!   bound, or would re-enter a return state already on the context path, is
//!   widened: the callee starts from the empty context and the config is
//!   marked [`WIDENED_TAINT`]. That is what makes recursive grammars
//!   terminate. The joints that were widened are remembered, because a
//!   conflict whose witness passes through one is not faithful to the runtime
//!   context and must not be trusted.
//! - **Boundary chase.** A config that reaches a rule stop with an empty
//!   context has left the decision; the runtime would pop the real caller.
//!   The build has no caller, so the config is marked
//!   [`BOUNDARY_TAINT`] and the closure continues through the grammar-wide
//!   FOLLOW links of that rule stop ("some caller"). Chased configs are
//!   tracked with a negative depth so their pop bookkeeping stays separate.
//! - **Symbolic predicates.** Predicate transitions pass through and mark
//!   the config with [`PRED_TAINT`]; the decision reports the first predicate
//!   it crossed. Precedence predicates are resolved from the *statically
//!   known* precedence class currently being built, or, when the class is not
//!   known, from the call sites recorded on the context path.
//!
//! Taints are kept in a side map keyed by `(state number, alternative)`
//! instead of on the configs themselves: [`ATNConfig`] identity ignores
//! `reaches_into_outer_context`, so a tainted and an untainted path to the
//! same config merge in the set, and OR-ing the side map is what preserves
//! the union of both paths' taints.

use std::collections::{HashMap, HashSet};

use dbt_antlr_runtime::analysis::{
    ATN, ATNConfig, ATNStateRef, ATNStateType, BOUNDARY_TAINT, BitSet, Bump, EOF, INTERIOR_TAINT,
    IntervalSetBuf, MergeCache, MutableATNConfigSet, NotSetTransition, PRECPRED_TAINT, PRED_TAINT,
    PrecedencePredicateTransition, PredicateTransition, PredictionContext, PredictionContextRef,
    RuleTransition, TOKEN_MIN_USER_TOKEN_TYPE, TransitionType, WIDENED_TAINT,
};

/// Union of the taints of every path that reached a closed config, keyed by
/// `(ATN state number, alternative)`.
pub(super) type Taint = HashMap<(i32, i32), i32>;

/// A closed config set with the taints of the paths that produced it.
///
/// The configs are *not* merged here: like the reference classifier, the
/// closure keeps every distinct `(state, alt, context)` it produced, and the
/// contexts are merged only once the set is finished (see
/// [`Closure::config_set`]). Merging during the closure would feed
/// analysis-only merged contexts back into the closure, and the successor sets
/// would stop corresponding to the states the runtime actually visits.
pub(super) struct ClosedSet<'a> {
    pub(super) configs: Vec<ATNConfig<'a>>,
    pub(super) taint: Taint,
}

impl ClosedSet<'_> {
    /// The taint recorded for one config of the set.
    pub(super) fn taint_of(&self, config: &ATNConfig<'_>) -> i32 {
        self.taint
            .get(&(config.get_state().get_state_number(), config.get_alt()))
            .copied()
            .unwrap_or(0)
    }
}

/// The precedence class a precedence-loop decision is being built for.
///
/// `eval_value` stands in for the precedence the parser will have when the
/// decision runs: the class is `lo..=hi` and the closure may only take
/// `PrecedencePredicateTransition`s whose predicate holds for it.
#[derive(Clone, Copy, Debug)]
pub(super) struct PrecedenceClass {
    pub(super) rule_index: i32,
    pub(super) eval_value: i32,
    pub(super) lo: i32,
    pub(super) hi: i32,
}

/// What one build attempt observed while closing config sets.
#[derive(Debug, Default)]
pub(super) struct ClosureFacts {
    pub(super) saw_predicate: bool,
    pub(super) first_predicate: Option<(i32, i32)>,
    pub(super) saw_precedence_predicate: bool,
    pub(super) used_widening: bool,
}

#[derive(Clone, Copy, Debug)]
struct CallInfo {
    target_rule_index: i32,
    precedence: i32,
}

/// The configs produced by one closure root, in insertion order.
struct SetBuilder<'a> {
    configs: Vec<ATNConfig<'a>>,
    seen: HashSet<ATNConfig<'a>>,
    taint: Taint,
    busy: HashSet<(ATNConfig<'a>, i32, bool)>,
}

impl<'a> SetBuilder<'a> {
    fn new() -> Self {
        Self {
            configs: Vec::new(),
            seen: HashSet::new(),
            taint: Taint::new(),
            busy: HashSet::new(),
        }
    }

    fn record(&mut self, config: &ATNConfig<'a>, taint: i32) {
        let key = (config.get_state().get_state_number(), config.get_alt());
        let slot = self.taint.entry(key).or_insert(0);
        *slot |= taint;
        if taint & BOUNDARY_TAINT == 0 {
            // Remember that this config is reachable without popping the
            // decision boundary: a later boundary-pop path must not turn the
            // usage into "all configs crossed the boundary".
            *slot |= INTERIOR_TAINT;
        }
    }

    fn insert(&mut self, config: ATNConfig<'a>, taint: i32) {
        self.record(&config, taint);
        if self.seen.insert(config.clone()) {
            self.configs.push(config);
        }
    }

    fn into_closed_set(self) -> ClosedSet<'a> {
        ClosedSet {
            configs: self.configs,
            taint: self.taint,
        }
    }
}

/// The default number of configs one attempt may visit.
///
/// The DFA state budget alone does not bound the analysis time: a single state
/// of a large grammar can close over tens of thousands of configs, because the
/// build-time closure explores every path the runtime would only explore on
/// demand. An attempt that spends this budget is treated like one that spends
/// the state budget - the decision falls back to a wider widening or a hybrid
/// table - so classification time stays proportional to the grammar rather
/// than to its recursion depth.
pub(crate) const DEFAULT_VISIT_BUDGET: u64 = 2_000_000;

/// The build-time closure of one ATN.
pub(super) struct Closure<'atn, 'a> {
    atn: &'atn ATN,
    scratch: &'a Bump,
    merge: MergeCache<'a>,
    pub(super) facts: ClosureFacts,
    precedence: Option<PrecedenceClass>,
    depth_cache: HashMap<usize, i32>,
    call_by_follow_state: Option<HashMap<i32, CallInfo>>,
    /// Widening bound of the current attempt: `i32::MAX` for the unbounded
    /// attempt, `0` for the fully widened one.
    depth_bound: i32,
    /// `true` while the start closure of a precedence class runs: precedence
    /// predicates there are evaluated against the class, not the context.
    precedence_start: bool,
    /// Configs visited by this closure, across all roots of the attempt.
    visits: u64,
    visit_budget: u64,
    /// Set when the visit budget ran out: the config sets built from here on
    /// are incomplete, so the attempt has to be discarded as an overflow.
    pub(super) exhausted: bool,
}

impl<'atn, 'a> Closure<'atn, 'a> {
    pub(super) fn new(
        atn: &'atn ATN,
        scratch: &'a Bump,
        depth_bound: i32,
        precedence: Option<PrecedenceClass>,
        visit_budget: u64,
    ) -> Self {
        Self {
            atn,
            scratch,
            merge: MergeCache::new(scratch),
            facts: ClosureFacts::default(),
            precedence,
            depth_cache: HashMap::new(),
            call_by_follow_state: None,
            depth_bound,
            precedence_start: false,
            visits: 0,
            visit_budget,
            exhausted: false,
        }
    }

    /// Closes the start configurations of `decision_state`: one config per
    /// alternative, each starting from the empty prediction context.
    pub(super) fn start(&mut self, decision_state: ATNStateRef) -> ClosedSet<'a> {
        let mut out = SetBuilder::new();
        self.precedence_start = self.precedence.is_some();
        for (index, transition) in decision_state.get_transitions().iter().enumerate() {
            let config = ATNConfig::new(
                transition.get_target(),
                (index + 1) as i32,
                Some(PredictionContextRef::new_empty()),
            );
            self.closure(config, 0, 0, &mut out);
        }
        self.precedence_start = false;
        let closed = out.into_closed_set();
        match self.precedence {
            Some(_) => self.apply_precedence_filter(closed),
            None => closed,
        }
    }

    /// The labeled successors of one closed set: the token intervals that can
    /// be read next, each with the set of configs that survives them.
    ///
    /// The alphabet is partitioned into equivalence classes: every interval in
    /// one class enables the same set of configs, so one successor set serves
    /// the whole class.
    pub(super) fn successors(
        &mut self,
        closed: &ClosedSet<'a>,
    ) -> Vec<(IntervalSetBuf, ClosedSet<'a>)> {
        let all_tokens = self.all_tokens();
        let mut labels: Vec<IntervalSetBuf> = Vec::new();
        let mut sources: Vec<ATNConfig<'a>> = Vec::new();
        let mut targets: Vec<ATNStateRef> = Vec::new();
        let mut is_stop: Vec<bool> = Vec::new();
        let mut taints: Vec<i32> = Vec::new();

        for config in &closed.configs {
            let mut taint = closed.taint_of(config);
            if taint & BOUNDARY_TAINT != 0 {
                // A path that popped the boundary stays a boundary path; only
                // the interior paths mark the config as mixed usage.
                taint &= !INTERIOR_TAINT;
            }
            let state = config.get_state();
            if state.state_type() == ATNStateType::RuleStop {
                let mut eof = IntervalSetBuf::new();
                eof.add_one(EOF);
                labels.push(eof);
                sources.push(config.clone());
                targets.push(state);
                is_stop.push(true);
                taints.push(taint);
                continue;
            }
            for transition in state.get_transitions() {
                if transition.is_epsilon() {
                    continue;
                }
                let label = match transition.transition_type() {
                    TransitionType::NotSet => {
                        let not_set = transition.try_as::<NotSetTransition>().unwrap();
                        not_set
                            .get_label()
                            .unwrap()
                            .complement(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type)
                    }
                    TransitionType::Wildcard => all_tokens.clone(),
                    _ => match transition.get_label() {
                        Some(set) => set.to_interval_set_buf(),
                        None => continue,
                    },
                };
                if label.as_interval_set().as_slice().is_empty() {
                    continue;
                }
                labels.push(label);
                sources.push(config.clone());
                targets.push(transition.get_target());
                is_stop.push(false);
                taints.push(taint);
            }
        }

        let mut boundaries: Vec<i32> = Vec::new();
        for label in &labels {
            for interval in label.as_interval_set().as_slice() {
                boundaries.push(interval.a);
                boundaries.push(interval.b.saturating_add(1));
            }
        }
        boundaries.sort_unstable();
        boundaries.dedup();

        let mut classes: Vec<(BitSet, IntervalSetBuf)> = Vec::new();
        for window in boundaries.windows(2) {
            let (lo, hi) = (window[0], window[1] - 1);
            let mut key = BitSet::new();
            for (index, label) in labels.iter().enumerate() {
                if label.as_interval_set().contains(lo) {
                    key.insert(index);
                }
            }
            if key.is_empty() {
                continue;
            }
            match classes.last_mut() {
                Some((known, label)) if *known == key => label.add_range(lo, hi),
                _ => {
                    let mut label = IntervalSetBuf::new();
                    label.add_range(lo, hi);
                    classes.push((key, label));
                }
            }
        }

        let mut successors = Vec::with_capacity(classes.len());
        for (class, label) in classes {
            if self.exhausted {
                break;
            }
            let mut out = SetBuilder::new();
            for index in class.iter() {
                let source = &sources[index];
                if is_stop[index] {
                    out.insert(source.clone(), taints[index]);
                } else {
                    let advanced = source.clone().with_state(targets[index]);
                    self.closure(advanced, taints[index], 0, &mut out);
                }
            }
            successors.push((label, out.into_closed_set()));
        }
        successors
    }

    /// Rebuilds a closed set in canonical order (state number, then
    /// alternative) so equal sets compare equal regardless of the order the
    /// closure happened to produce them in.
    ///
    /// The runtime DFA identifies a state by its config set, and that
    /// comparison walks both sets in insertion order. The closure reaches the
    /// same set through many paths in different insertion orders, so without
    /// this step every path would intern a fresh DFA state and the subset
    /// construction would never terminate on grammars with loops.
    pub(super) fn canonical_set(&mut self, closed: ClosedSet<'a>) -> ClosedSet<'a> {
        let set = self.config_set(&closed.configs);
        let mut configs: Vec<ATNConfig<'a>> = set.into_iter().collect();
        configs.sort_by_key(|config| (config.get_state().get_state_number(), config.get_alt()));
        ClosedSet {
            configs,
            taint: closed.taint,
        }
    }

    /// The merged, runtime-shaped config set of a closed set, for the conflict
    /// predicates and for interning.
    pub(super) fn merged_config_set(&mut self, closed: &ClosedSet<'a>) -> MutableATNConfigSet<'a> {
        self.config_set(&closed.configs)
    }

    fn all_tokens(&self) -> IntervalSetBuf {
        let mut set = IntervalSetBuf::new();
        if self.atn.max_token_type >= TOKEN_MIN_USER_TOKEN_TYPE {
            set.add_range(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type);
        }
        set
    }

    /// Merges a finished config list into a runtime config set: contexts of
    /// configs that share `(state, alt)` are merged here, once, exactly as the
    /// runtime's `ATNConfigSet::add` would.
    pub(super) fn config_set(&mut self, configs: &[ATNConfig<'a>]) -> MutableATNConfigSet<'a> {
        let mut set = MutableATNConfigSet::new(self.scratch, false);
        for config in configs {
            // The analysis taint stays in the side map: writing it into the
            // config would make `ATNConfigSet::dips_into_outer_context` part of
            // the DFA state identity, so the same config set reached with
            // different taint would intern as two states and the classifier
            // would never see the second path.
            set.add_cached(config.clone(), &mut self.merge);
        }
        set
    }

    fn closure(
        &mut self,
        config: ATNConfig<'a>,
        mut taint: i32,
        depth: i32,
        out: &mut SetBuilder<'a>,
    ) {
        if self.exhausted {
            return;
        }
        self.visits = self.visits.saturating_add(1);
        if self.visits > self.visit_budget {
            self.exhausted = true;
            return;
        }
        let chasing = depth < 0;
        if !out.busy.insert((config.clone(), taint, chasing)) {
            // Already explored with this taint; the taint of the config is
            // already recorded (or the config never joins the set).
            return;
        }
        let state = config.get_state();

        if state.state_type() == ATNStateType::RuleStop {
            let context = config.get_context().expect("config has a context");
            if !context.is_empty() {
                for i in 0..context.length() {
                    if context.get_return_state(i) == ATNStateRef::invalid() {
                        let popped = config
                            .clone()
                            .with_prediction_context(Some(PredictionContextRef::new_empty()));
                        self.closure(popped, taint, depth, out);
                    } else {
                        let return_state = context.get_return_state(i);
                        let parent = context
                            .get_parent(i)
                            .unwrap_or_else(PredictionContextRef::new_empty);
                        let popped = config
                            .clone()
                            .with_state(return_state)
                            .with_prediction_context(Some(parent));
                        self.closure(popped, taint, depth - 1, out);
                    }
                }
                return;
            }
            // The config just left the decision: from here on its path crossed
            // the boundary, and it is no longer "inside". The FOLLOW chase
            // below inherits the taint, exactly like the runtime closure, where
            // the marker lives on the config that keeps being passed down.
            taint = (taint & !INTERIOR_TAINT) | BOUNDARY_TAINT;
            out.insert(config.clone(), taint);
        } else if !state.has_epsilon_only_transitions() {
            out.insert(config.clone(), taint);
        }

        let boundary_chase = state.state_type() == ATNStateType::RuleStop;
        for transition in state.get_transitions() {
            match transition.transition_type() {
                TransitionType::Rule => {
                    let rule_transition = transition.try_as::<RuleTransition>().unwrap();
                    self.enter_rule(&config, rule_transition, taint, depth, out);
                }
                TransitionType::PrecedencePredicate => {
                    let predicate = transition
                        .try_as::<PrecedencePredicateTransition>()
                        .unwrap();
                    self.cross_precedence_predicate(&config, predicate, taint, depth, out);
                }
                TransitionType::Predicate => {
                    let predicate = transition.try_as::<PredicateTransition>().unwrap();
                    self.facts.saw_predicate = true;
                    if self.facts.first_predicate.is_none() {
                        self.facts.first_predicate =
                            Some((predicate.rule_index(), predicate.pred_index()));
                    }
                    let next = config.clone().with_state(predicate.get_target());
                    self.closure(next, taint | PRED_TAINT, depth, out);
                }
                _ if transition.is_epsilon() => {
                    if boundary_chase
                        && transition.get_target().get_rule_index() == state.get_rule_index()
                        && self.precedence_suppresses(transition.get_target(), depth)
                    {
                        continue;
                    }
                    let mut next = config.clone().with_state(transition.get_target());
                    let mut next_depth = depth;
                    if boundary_chase {
                        next_depth = (depth - 1).min(-1);
                        if self.precedence_start
                            && self
                                .precedence
                                .is_some_and(|class| state.get_rule_index() == class.rule_index)
                            && self.precedence_starts_self_call(transition.get_target())
                        {
                            next.set_precedence_filter_suppressed(true);
                        }
                    }
                    self.closure(next, taint, next_depth, out);
                }
                _ => {}
            }
        }
    }

    fn enter_rule(
        &mut self,
        config: &ATNConfig<'a>,
        transition: &RuleTransition,
        taint: i32,
        depth: i32,
        out: &mut SetBuilder<'a>,
    ) {
        let follow_state = transition.follow_state;
        let follow_state_number = follow_state.get_state_number();
        let context = config.get_context().expect("config has a context");
        let (new_context, widened) = if self.depth_bound == 0 {
            (PredictionContextRef::new_empty(), true)
        } else if self.context_depth(context) >= self.depth_bound
            || has_return_state(context, follow_state_number)
        {
            let joint =
                PredictionContextRef::from(self.scratch.alloc(PredictionContext::new_singleton(
                    Some(PredictionContextRef::new_empty()),
                    follow_state,
                )));
            (joint, true)
        } else {
            let pushed = PredictionContextRef::from(self.scratch.alloc(
                PredictionContext::new_singleton(Some(context), follow_state),
            ));
            (pushed, false)
        };

        let callee = config
            .clone()
            .with_state(transition.get_target())
            .with_prediction_context(Some(new_context));
        let mut callee_taint = taint;
        if widened {
            self.facts.used_widening = true;
            callee_taint |= WIDENED_TAINT;
        }
        let next_depth = if depth >= 0 { depth + 1 } else { depth };
        self.closure(callee, callee_taint, next_depth, out);
    }

    fn cross_precedence_predicate(
        &mut self,
        config: &ATNConfig<'a>,
        predicate: &PrecedencePredicateTransition,
        taint: i32,
        depth: i32,
        out: &mut SetBuilder<'a>,
    ) {
        let value = predicate.precedence();
        if let Some(class) = self.precedence
            && self.precedence_start
            && depth == 0
        {
            if value >= class.eval_value {
                let next = config.clone().with_state(predicate.get_target());
                self.closure(next, taint, depth, out);
            }
            return;
        }
        let context = config.get_context().expect("config has a context");
        let rule_index = config.get_state().get_rule_index();
        if let Some(passes) = self.evaluate_precedence_from_context(context, rule_index, value) {
            if passes {
                let next = config.clone().with_state(predicate.get_target());
                self.closure(next, taint, depth, out);
            }
            return;
        }
        self.facts.saw_precedence_predicate = true;
        let next = config.clone().with_state(predicate.get_target());
        self.closure(next, taint | PRECPRED_TAINT, depth, out);
    }

    fn precedence_suppresses(&mut self, target: ATNStateRef, depth: i32) -> bool {
        let Some(class) = self.precedence else {
            return false;
        };
        if depth < 0 {
            return false;
        }
        self.call_info(target.get_state_number())
            .is_some_and(|call| {
                call.target_rule_index == class.rule_index
                    && (call.precedence < class.lo || call.precedence > class.hi)
            })
    }

    fn precedence_starts_self_call(&mut self, target: ATNStateRef) -> bool {
        let Some(class) = self.precedence else {
            return false;
        };
        self.call_info(target.get_state_number())
            .is_some_and(|call| call.precedence == 0 && call.target_rule_index == class.rule_index)
    }

    fn ensure_call_by_follow_state(&mut self) {
        if self.call_by_follow_state.is_some() {
            return;
        }
        let mut map = HashMap::new();
        for state in self.atn.iter_states() {
            for transition in state.get_transitions() {
                if let Some(rule_transition) = transition.try_as::<RuleTransition>() {
                    map.insert(
                        rule_transition.follow_state.get_state_number(),
                        CallInfo {
                            target_rule_index: rule_transition.get_target().get_rule_index(),
                            precedence: rule_transition.precedence(),
                        },
                    );
                }
            }
        }
        self.call_by_follow_state = Some(map);
    }

    fn call_info(&mut self, follow_state_number: i32) -> Option<CallInfo> {
        self.ensure_call_by_follow_state();
        self.call_by_follow_state
            .as_ref()
            .and_then(|map| map.get(&follow_state_number).copied())
    }

    /// Evaluates a precedence predicate from the call sites recorded on the
    /// context path: `None` when the path does not pin it down.
    fn evaluate_precedence_from_context(
        &mut self,
        context: PredictionContextRef<'_>,
        rule: i32,
        value: i32,
    ) -> Option<bool> {
        if context.is_empty() {
            return None;
        }
        let mut agreed = None;
        for i in 0..context.length() {
            let return_state = context.get_return_state(i);
            if return_state == ATNStateRef::invalid() {
                return None;
            }
            let call = self.call_info(return_state.get_state_number())?;
            if call.target_rule_index != rule {
                return None;
            }
            let outcome = value >= call.precedence;
            match agreed {
                None => agreed = Some(outcome),
                Some(previous) if previous != outcome => return None,
                Some(_) => {}
            }
        }
        agreed
    }

    fn context_depth(&mut self, context: PredictionContextRef<'a>) -> i32 {
        if context.is_empty() {
            return 0;
        }
        let pointer = context.as_ref() as *const PredictionContext<'_> as usize;
        if let Some(&cached) = self.depth_cache.get(&pointer) {
            return cached;
        }
        let mut max_depth = 0;
        for i in 0..context.length() {
            if context.get_return_state(i) == ATNStateRef::invalid() {
                continue;
            }
            let depth = match context.get_parent(i) {
                Some(parent) => 1 + self.context_depth(parent),
                None => 1,
            };
            max_depth = max_depth.max(depth);
        }
        self.depth_cache.insert(pointer, max_depth);
        max_depth
    }

    /// The precedence filter of a precedence-loop start state: an alternative
    /// that would consume the loop while a higher-precedence alternative
    /// already reached the same state with the same context is dropped.
    fn apply_precedence_filter(&mut self, closed: ClosedSet<'a>) -> ClosedSet<'a> {
        // Merge the contexts first (as the reference classifier does): the
        // filter compares alternative 1's context against every other
        // alternative's, so it has to see the same merged contexts the runtime
        // start state would hold.
        let closed = self.canonical_set(closed);
        let mut states_from_alt1: HashMap<i32, PredictionContextRef<'a>> = HashMap::new();
        for config in &closed.configs {
            if config.get_alt() == 1 {
                states_from_alt1.insert(
                    config.get_state().get_state_number(),
                    config.get_context().expect("config has a context"),
                );
            }
        }

        let mut out = SetBuilder::new();
        for config in closed.configs {
            if config.get_alt() != 1
                && !config.is_precedence_filter_suppressed()
                && states_from_alt1.get(&config.get_state().get_state_number())
                    == config.get_context().as_ref()
            {
                continue;
            }
            let mut kept = config;
            kept.set_precedence_filter_suppressed(false);
            let taint = closed
                .taint
                .get(&(kept.get_state().get_state_number(), kept.get_alt()))
                .copied()
                .unwrap_or(0);
            out.insert(kept, taint);
        }
        out.into_closed_set()
    }
}

/// `true` when `return_state_number` appears on the context path.
fn has_return_state(mut context: PredictionContextRef<'_>, return_state_number: i32) -> bool {
    while !context.is_empty() {
        if context.length() != 1 {
            let mut visited = HashSet::new();
            return has_return_state_in_dag(context, return_state_number, &mut visited);
        }
        let return_state = context.get_return_state(0);
        if return_state.get_state_number() == return_state_number {
            return true;
        }
        if return_state == ATNStateRef::invalid() {
            return false;
        }
        match context.get_parent(0) {
            Some(parent) => context = parent,
            None => return false,
        }
    }
    false
}

fn has_return_state_in_dag(
    context: PredictionContextRef<'_>,
    return_state_number: i32,
    visited: &mut HashSet<usize>,
) -> bool {
    if context.is_empty() {
        return false;
    }
    let pointer = context.as_ref() as *const PredictionContext<'_> as usize;
    if !visited.insert(pointer) {
        return false;
    }
    for i in 0..context.length() {
        let return_state = context.get_return_state(i);
        if return_state.get_state_number() == return_state_number {
            return true;
        }
        if return_state != ATNStateRef::invalid()
            && context
                .get_parent(i)
                .is_some_and(|parent| has_return_state_in_dag(parent, return_state_number, visited))
        {
            return true;
        }
    }
    false
}
