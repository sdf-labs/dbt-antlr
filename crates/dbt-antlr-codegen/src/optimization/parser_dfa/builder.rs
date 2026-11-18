// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The static DFA table builder: subset construction, frontiers, budgets, and
//! trust rules.
//!
//! The builder makes one table per decision, or per precedence class. A state
//! that the construction cannot decide soundly escapes on its own: a
//! predicate, a conflict that no static resolution decides, a taint that a
//! later path adds, or a budget. The other states stay static. A decision gets
//! no table when its start state cannot decide, when the table decides too
//! little of the start lookahead, or when the generated parser cannot use a
//! table for it (see [`NoTableReason`]).
//!
//! Until the first state escapes, the table can still be static, so the
//! construction expands up to the state cap of the decision. After the first
//! escape, the table defers anyway, and the construction stops at [`Frontier`].
//! When an escaping table expanded past [`Frontier`] before its first escape,
//! a bounded pass retries within [`Frontier`]. A conflict-free budget cutoff
//! or an exhausted start state retries with full widening. A `max_k` option
//! stops the construction at its depth from the start.

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::num::NonZeroUsize;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use dbt_antlr_runtime::analysis::{
    ATN, ATNConfigSet, ATNStateRef, ATNStateType, Arena, BOUNDARY_TAINT, BitSet, Bump, EOF,
    FOREIGN_CONSUME_TAINT, FOREIGN_LINEAGE_TAINT, IATNSimulator, INTERIOR_TAINT, MIDWAY_TAINT,
    PRECPRED_TAINT, PRED_TAINT, ParserATNSimulator, ParserATNSimulatorManager, ParserDFAState,
    PrecedencePredicateTransition, PredictionContext, PredictionContextRef, PredictionMode,
    ProposedDFAState, ROOTED_TAINT, RuleStartState, WIDENED_TAINT, all_subsets_conflict,
    all_subsets_equal, conflicting_alt_subsets, get_alts, has_sll_conflict_terminating_prediction,
};
use dbt_antlr_runtime::dfa::ParserDFA;
use dbt_antlr_runtime::dfa::static_parser_dfa::{
    ContinuationConfigEntry, ContinuationContextNode, ParserContinuation, VIRTUAL_ALT_BASE,
};

use super::closure::{
    CallSites, ClosedSet, Closure, ClosureFacts, GuardRoots, PrecedenceClass, Taint, taint_of,
};
use super::cost::{AttemptCost, AttemptKind, AttemptOutcome, CostLog};
use super::descent::{DescentAnalyzer, DescentPlan, DescentRules};
use super::hazard::{self, StackHijackVerdict, StateHazard};
use super::loop_block::{
    EXIT_ALT, EXIT_FOREIGN, ITERATE_ALT, ITERATE_REAL, LoopBlock, is_iterate_exit_pair,
};
use super::outcome::{
    BudgetCutoff, DecisionKind, DecisionOutcome, NoTableReason, PrecedenceTables,
};
use super::postfix::{GuardRegions, PostfixShape, epsilon_closure};
use super::table::{StaticDfaEdge, StaticDfaState, StaticDfaTable};
use super::{CONFLICT_ESCAPE, DecisionBudget, ESCAPE, GUARDED};
use crate::optimization::AtnFallback;

/// The depth at which a partial table stops expanding.
pub(crate) const PARTIAL_DEPTH_CAP: usize = 4;
/// The state count at which a partial table stops expanding beyond depth 2.
pub(crate) const PARTIAL_STATE_CAP: usize = 64;
/// [`PARTIAL_DEPTH_CAP`] under `--static-dfa-only`. Without the adaptive
/// fallback, a frontier state can only guess, so the table looks further. Under
/// `--static-dfa-only`, the state cap of a partial table is the state cap of
/// the decision.
pub(crate) const STATIC_ONLY_DEPTH_CAP: usize = 16;
/// Below this share of useful start edges a partial table is not emitted,
/// unless `--static-dfa-only` removes the adaptive fallback.
pub(crate) const PARTIAL_MIN_COVERAGE_PERCENT: u32 = 10;
/// Largest continuation a partial table emits, counted in wire words.
///
/// A continuation saves the `k` steps of a Stage 1 restart, but replaying its
/// config set costs a closure over that set, and storing it costs
/// `continuation_words` words of the emitted table. Both grow with the set, so
/// only small frontier sets are worth serializing: on `CPP14.g4` the median
/// continuation is 112 words and the pool is 61% of the whole section, while
/// the measured runtime difference is a fraction of a percent. 64 words is the
/// largest continuation that still beats a restart on our corpora
/// (`FuzzExpr.g4`'s `item` frontier).
pub(crate) const MAX_CONTINUATION_WORDS: usize = 64;

/// Maximum number of threads that build the tables of one parser.
///
/// One build attempt can use several hundred megabytes of scratch memory, and
/// each thread can run an attempt at the same time. On our SQL benchmark
/// grammars, 16 threads give most of the speedup at less than half the memory
/// of 60 threads. The slowest decisions set the wall time.
const MAX_BUILD_THREADS: usize = 16;

/// The escape states of one table that the prefix-hijack analysis analyzes
/// at most, the shallowest first.
const HAZARD_STATES: usize = 48;
/// The escape states of one tie-break group that the prefix-hijack analysis
/// analyzes at most. The analysis of a group stops at its first hijack.
const HAZARD_STATES_PER_GROUP: usize = 4;
/// The guarded states of one table whose stack-hijack hazard the builder
/// analyzes at most.
const GUARD_HAZARD_STATES: usize = 48;
/// The analyzed guarded states of one guard group at most.
const GUARD_HAZARD_STATES_PER_GROUP: usize = 8;

/// Taint bits that invalidate a trusted verdict when a second path adds them
/// to an already interned state. A decision state that gets one of them is
/// expanded again, so its successors get the bit too.
const TRUST_TAINT: i32 = WIDENED_TAINT
    | PRECPRED_TAINT
    | BOUNDARY_TAINT
    | FOREIGN_CONSUME_TAINT
    | FOREIGN_LINEAGE_TAINT
    | MIDWAY_TAINT
    | ROOTED_TAINT
    | INTERIOR_TAINT;

/// The alternative of an optional-postfix decision that takes the postfix
/// element.
const POSTFIX_TAKE_ALT: i32 = 1;
/// The alternative of an optional-postfix decision that skips it.
const POSTFIX_SKIP_ALT: i32 = 2;

/// Whether a set of live alternatives is exactly the take/skip pair of an
/// optional-postfix decision.
fn is_take_skip_pair(alts: &BitSet) -> bool {
    alts.len() == 2
        && alts.contains(POSTFIX_TAKE_ALT as usize)
        && alts.contains(POSTFIX_SKIP_ALT as usize)
}

const MAX_TRACKED_DEPTH: usize = 30;
const VARIABLE_DEPTH: usize = 31;

/// The verdict of one DFA state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Verdict {
    /// The state predicts this alternative.
    Accept(i32),
    /// The state escapes to the adaptive simulation, with the table marker it
    /// carries ([`ESCAPE`], [`CONFLICT_ESCAPE`], or [`GUARDED`]).
    Escape(i32),
    /// The state is a decision point: it needs outgoing edges.
    Decide,
}

/// What the builder remembers about one interned DFA state.
#[derive(Clone, Debug)]
struct StateFacts {
    depth: usize,
    verdict: Verdict,
    taint: Taint,
    fallback: i32,
    conflicts: Vec<i32>,
    /// The accept was reached by resolving a conflict (exact ambiguity or
    /// uniform trust), not by finding a single viable alternative. Only this
    /// kind of accept depends on the taint of the paths that reached the
    /// state.
    resolved_conflict: bool,
    /// The state's successors were computed at least once.
    expanded: bool,
    /// The lowest conflicting alternative of a conflict escape state, `0` when
    /// the runtime cannot resolve the conflict directly.
    conflict_min_alt: i32,
    /// The alternative a guarded state resolves to, `0` when the state is not
    /// guarded.
    guarded_alt: i32,
    /// The other alternative of a guarded state: the skip or the exit.
    guard_defer_alt: i32,
    /// The guard group of a guarded state.
    guard_group: u32,
    /// The guard roots of the configs of the state.
    roots: GuardRoots,
    /// The closed config set of a frontier escape state.
    continuation: Option<ParserContinuation>,
    /// A predicate in the config set made the state escape.
    predicated: bool,
}

/// The attempt-level bookkeeping one classification commits.
///
/// A classification is computed before the state is interned, because the
/// runtime DFA needs the accept flag and the prediction up front. Only a
/// *newly* interned state commits these effects: a lookahead path that reaches
/// an already known config set must not count its conflicts twice.
#[derive(Debug, Default)]
struct ClassifyEffects {
    conflict: Option<Vec<i32>>,
    /// A take-vetoed conflict state that may resolve as a guarded take.
    guard_candidate: bool,
    /// An iterate/exit conflict state vetoed only on foreign consumption: a
    /// candidate for the guarded iterate.
    guard_iterate_candidate: bool,
    /// The decision state of the attempt is behind a predicate, so the whole
    /// decision depends on parser state and no table can decide it.
    fail_predicated: bool,
}

impl StateFacts {
    fn accept_value(&self) -> i32 {
        match self.verdict {
            Verdict::Accept(alt) => alt,
            Verdict::Escape(marker) => marker,
            Verdict::Decide => 0,
        }
    }
}

/// The soft limits of a table that has an escape state.
///
/// A table with an escape state defers to the adaptive simulation anyway, so
/// expanding it further only buys coverage. Since the construction runs
/// breadth first, the states that it already expanded stay in the table.
#[derive(Clone, Copy, Debug)]
struct Frontier {
    /// The depth at which the table stops expanding.
    depth_cap: usize,
    /// The state count at which the table stops expanding beyond depth 2.
    state_cap: usize,
}

impl Frontier {
    /// The frontier of a decision with `budget`. A depth cap or a state cap of
    /// the budget replaces the default of the partial table.
    fn of(budget: &DecisionBudget, atn_fallback: AtnFallback) -> Self {
        match atn_fallback {
            AtnFallback::Allowed => Self {
                depth_cap: budget.max_k.unwrap_or(PARTIAL_DEPTH_CAP),
                state_cap: if budget.explicit_max_states {
                    budget.max_states
                } else {
                    PARTIAL_STATE_CAP
                },
            },
            AtnFallback::Forbidden => Self {
                depth_cap: budget.max_k.unwrap_or(STATIC_ONLY_DEPTH_CAP),
                state_cap: budget.max_states,
            },
        }
    }
}

/// The result of one build attempt.
#[derive(Debug, Default)]
struct Attempt {
    decision: usize,
    state_count: usize,
    table: Option<StaticDfaTable>,
    /// Set when the attempt has no table because the start state spent a budget.
    failed: Option<BudgetCutoff>,
    /// Set when the closure or the expansion of the start state spent the
    /// visit budget.
    start_exhausted: bool,
    /// Set when the closures of the attempt spent the visit budget.
    exhausted: bool,
    /// Set when the attempt spent a budget and escaped the states it cannot
    /// expand.
    budget_cutoff: Option<BudgetCutoff>,
    /// Set when a conflict, predicate, or taint-increase escape occurred
    /// before `cut_off` fired.
    non_budget_escape: bool,
    /// Set when pre-escape expansion expanded a state at [`Frontier`].
    exceeded_frontier: bool,
    /// Configs the closures of the attempt visited.
    visits: u64,
    /// The visited configs by rule index.
    rule_visits: Vec<u64>,
    facts: ClosureFacts,
    /// Alternative sets of the conflicts the construction did not resolve.
    conflicts: Vec<Vec<i32>>,
    /// DFA state number of the first conflict, for the report witness.
    first_conflict_state: Option<i32>,
    /// The decision state is behind a predicate: the decision is predicated.
    predicated: bool,
    /// The shared-descent plan of the attempt, when it had one.
    descent: Option<DescentPlan>,
}

impl Attempt {
    fn finalize(self, decision_state: ATNStateRef, atn_fallback: AtnFallback) -> DecisionOutcome {
        let mut outcome = DecisionOutcome::new(self.decision, decision_state);
        for conflict in &self.conflicts {
            if !outcome.conflicts.contains(conflict) {
                outcome.conflicts.push(conflict.clone());
            }
        }

        if self.predicated {
            outcome.no_table = Some(NoTableReason::PredicatedStart(self.facts.first_predicate));
            return outcome;
        }
        let Some(table) = self.table else {
            outcome.no_table = Some(NoTableReason::StartBudget(
                self.failed.expect("failed attempt records its budget"),
            ));
            return outcome;
        };

        outcome.coverage_percent = (table.start_coverage() * 100.0) as u32;
        outcome.conflict_witness = table
            .first_escape(CONFLICT_ESCAPE)
            .or_else(|| {
                self.first_conflict_state.and_then(|state| {
                    usize::try_from(state)
                        .ok()
                        .filter(|state| *state < table.states.len())
                })
            })
            .and_then(|state| table.witness(state));
        if let Some(state) = table.first_escape(ESCAPE) {
            outcome.budget_witness = table.witness(state);
        }

        // A table whose start state escapes answers nothing at all.
        if table.states.first().is_none_or(StaticDfaState::is_escape) {
            outcome.no_table = Some(NoTableReason::StartConflict);
            return outcome;
        }
        let escapes = table.escapes();
        if escapes > 0 {
            // Below the coverage threshold, the adaptive simulation decides
            // almost every lookahead anyway, unless there is no adaptive
            // simulation.
            if atn_fallback == AtnFallback::Allowed
                && outcome.coverage_percent < PARTIAL_MIN_COVERAGE_PERCENT
            {
                outcome.no_table = Some(NoTableReason::LowCoverage {
                    coverage_percent: outcome.coverage_percent,
                });
                return outcome;
            }
            outcome.kind = DecisionKind::Hybrid;
        } else {
            outcome.kind = DecisionKind::Static;
        }
        outcome.num_dfa_states = table.states.len();
        outcome.k = table.max_k;
        outcome.escapes = escapes;
        outcome.budget_cutoff = self.budget_cutoff;
        outcome.descent = used_groups(self.descent.as_ref(), &table);
        outcome.table = Some(table);
        outcome
    }
}

/// Builds the static DFA tables of the decision states of one compiled parser
/// unit.
pub(crate) struct DecisionTableBuilder {
    atn: &'static ATN,
    ll1_disjoint: Vec<bool>,
    /// The token counts from each alternative start state to the stop state
    /// of its rule, by ATN state number, computed on first use.
    completion_depths: Box<[OnceLock<BitSet>]>,
    /// The budget of a decision without an entry in `budgets`.
    default_budget: DecisionBudget,
    /// The budget of each decision, by decision number.
    budgets: Vec<DecisionBudget>,
    /// The operator-loop analysis of each precedence-loop decision, built at
    /// most once and shared by every class attempt.
    loop_blocks: Box<[OnceLock<LoopBlock>]>,
    /// The optional-postfix shape of each decision, built at most once.
    postfix_shapes: Box<[OnceLock<Option<PostfixShape>>]>,
    /// Whether the generated parser can defer to the adaptive simulation. When
    /// it cannot (`--static-dfa-only`), a partial table is kept regardless of
    /// its coverage, because the alternative is no table at all.
    atn_fallback: AtnFallback,
    /// The shared-descent plans of the decisions.
    descent: DescentAnalyzer,
    /// The rule invocations of the ATN, shared by every build attempt.
    call_sites: CallSites,
    /// The follow regions of the stack guards, computed on first use.
    guard_regions: OnceLock<GuardRegions>,
    /// Measure the wall-clock time of each decision and attempt.
    profile: bool,
    /// Every build runs with full widening.
    #[cfg(test)]
    widened: bool,
}

impl DecisionTableBuilder {
    pub(crate) fn new(atn: &'static ATN, ll1_disjoint: Vec<bool>) -> Self {
        Self {
            atn,
            ll1_disjoint,
            completion_depths: once_locks(atn.states_count()),
            default_budget: DecisionBudget::default(),
            budgets: Vec::new(),
            loop_blocks: once_locks(atn.decision_to_state.len()),
            postfix_shapes: once_locks(atn.decision_to_state.len()),
            atn_fallback: AtnFallback::Allowed,
            descent: DescentAnalyzer::new(atn),
            call_sites: CallSites::of(atn),
            guard_regions: OnceLock::new(),
            profile: false,
            #[cfg(test)]
            widened: false,
        }
    }

    /// The loop-block analysis of one precedence-loop decision, built on first
    /// use and shared by every class attempt of that decision.
    fn loop_block(&self, decision: usize) -> LoopBlock {
        self.loop_blocks[decision]
            .get_or_init(|| LoopBlock::of(self.atn, self.atn.decision_to_state[decision]))
            .clone()
    }

    /// The optional-postfix shape of one decision, detected on first use.
    fn postfix_shape(&self, decision: usize) -> Option<PostfixShape> {
        self.postfix_shapes[decision]
            .get_or_init(|| PostfixShape::analyze(self.atn, self.atn.decision_to_state[decision]))
            .clone()
    }

    /// Sets the per-attempt visit budget of the decisions without an entry
    /// in [`Self::with_budgets`]. The default is
    /// [`DEFAULT_VISIT_BUDGET`](super::DEFAULT_VISIT_BUDGET).
    #[cfg(test)]
    pub(crate) const fn with_visit_budget(mut self, budget: u64) -> Self {
        self.default_budget.max_visits = budget;
        self
    }

    /// Sets the lookahead depth of the decisions without an entry in
    /// [`Self::with_budgets`].
    #[cfg(test)]
    pub(crate) const fn with_max_k(mut self, max_k: usize) -> Self {
        self.default_budget.max_k = Some(max_k);
        self
    }

    /// Runs every build with full widening, like the build after a start
    /// state that spent the visit budget.
    #[cfg(test)]
    pub(crate) const fn with_widening(mut self) -> Self {
        self.widened = true;
        self
    }

    /// Sets the budget of each decision, by decision number.
    pub(crate) fn with_budgets(mut self, budgets: Vec<DecisionBudget>) -> Self {
        self.budgets = budgets;
        self
    }

    /// The budget of decision `decision`.
    pub(crate) fn budget(&self, decision: usize) -> DecisionBudget {
        self.budgets
            .get(decision)
            .copied()
            .unwrap_or(self.default_budget)
    }

    /// Enables shared-descent groups with the rule facts of the grammar.
    pub(crate) fn with_descent_rules(mut self, rules: DescentRules) -> Self {
        self.descent = self.descent.with_rules(rules);
        self
    }

    /// Sets whether the generated parser can defer to the adaptive simulation.
    pub(crate) const fn with_atn_fallback(mut self, atn_fallback: AtnFallback) -> Self {
        self.atn_fallback = atn_fallback;
        self
    }

    /// Measures the wall-clock time of each decision and each build attempt
    /// (`--profile-decisions`). The times differ from run to run, so the
    /// results are no longer reproducible.
    pub(crate) const fn with_profiling(mut self, profile: bool) -> Self {
        self.profile = profile;
        self
    }

    /// The number of threads that [`Self::build_all`] runs.
    pub(crate) fn threads(&self) -> usize {
        let available = std::thread::available_parallelism().map_or(1, NonZeroUsize::get);
        self.workers(available.min(MAX_BUILD_THREADS))
    }

    /// The number of threads that [`Self::build_on`] runs when it can run
    /// `threads` threads: never more than there are decisions.
    fn workers(&self, threads: usize) -> usize {
        threads.clamp(1, self.atn.decision_to_state.len().max(1))
    }

    /// Builds the tables of every decision, in parallel.
    pub(crate) fn build_all(&self) -> Vec<DecisionOutcome> {
        self.build_on(self.threads())
    }

    /// Builds the tables of every decision on at most `threads` threads.
    ///
    /// Each thread takes the next decision until none is left. The tables of
    /// one decision do not depend on the others. The shared caches hold
    /// values that do not depend on the order of the computation. Thus the
    /// results do not depend on the thread count.
    pub(super) fn build_on(&self, threads: usize) -> Vec<DecisionOutcome> {
        let decisions = self.atn.decision_to_state.len();
        let threads = self.workers(threads);
        if threads == 1 {
            return (0..decisions)
                .map(|decision| self.build(decision))
                .collect();
        }
        let next = AtomicUsize::new(0);
        let mut outcomes: Vec<Option<DecisionOutcome>> = vec![None; decisions];
        std::thread::scope(|scope| {
            let workers: Vec<_> = (0..threads)
                .map(|_| {
                    scope.spawn(|| {
                        let mut built = Vec::new();
                        loop {
                            let decision = next.fetch_add(1, Ordering::Relaxed);
                            if decision >= decisions {
                                break built;
                            }
                            built.push((decision, self.build(decision)));
                        }
                    })
                })
                .collect();
            for worker in workers {
                let built = worker
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
                for (decision, outcome) in built {
                    outcomes[decision] = Some(outcome);
                }
            }
        });
        outcomes
            .into_iter()
            .map(|outcome| outcome.expect("every decision is built"))
            .collect()
    }

    /// Builds the tables of one decision and records what they cost.
    fn build(&self, decision: usize) -> DecisionOutcome {
        let started = self.profile.then(Instant::now);
        let mut log = CostLog::default();
        let mut outcome = self.build_decision(decision, &mut log);
        outcome.cost = log.finish(started.map(|started| started.elapsed()));
        outcome.budget = self.budget(decision);
        outcome
    }

    fn build_decision(&self, decision: usize, log: &mut CostLog) -> DecisionOutcome {
        let state = self.atn.decision_to_state[decision];
        let mut outcome = DecisionOutcome::new(decision, state);

        if state.is_nongreedy_decision() == Some(true) {
            outcome.kind = DecisionKind::Adaptive;
            outcome.no_table = Some(NoTableReason::NonGreedy);
            return outcome;
        }
        if self.is_precedence_loop_decision(state) {
            return self.build_precedence(outcome, log);
        }
        if self.ll1_disjoint.get(decision).copied() == Some(true) {
            outcome.kind = DecisionKind::Ll1;
            outcome.k = 1;
            return outcome;
        }
        if state.get_transitions().len() >= VIRTUAL_ALT_BASE as usize {
            outcome.no_table = Some(NoTableReason::TooManyAlternatives);
            return outcome;
        }
        self.build_table(decision, None, true, log)
    }

    /// Builds the table of one decision, or of one precedence class of it.
    ///
    /// If the start state or a conflict-free build exhausts a budget, the build
    /// retries with full widening. If an escaping table expanded past
    /// [`Frontier`], a bounded pass retries within the frontier.
    fn build_table(
        &self,
        decision: usize,
        class: Option<PrecedenceClass>,
        analyze_hazards: bool,
        log: &mut CostLog,
    ) -> DecisionOutcome {
        let decision_state = self.atn.decision_to_state[decision];

        #[cfg(test)]
        if self.widened {
            let widened = self.attempt(decision, 0, class, false, analyze_hazards, log);
            return self.finish_partial(widened, 0, class, analyze_hazards, log);
        }

        let attempt = self.attempt(decision, i32::MAX, class, false, analyze_hazards, log);
        if attempt.start_exhausted {
            let widened = self.attempt(decision, 0, class, false, analyze_hazards, log);
            return self.finish_partial(widened, 0, class, analyze_hazards, log);
        }
        let should_widen = attempt.budget_cutoff.is_some() && !attempt.non_budget_escape;
        let exceeded_frontier = attempt.exceeded_frontier;
        let outcome = attempt.finalize(decision_state, self.atn_fallback);
        if outcome.kind == DecisionKind::Static {
            return outcome;
        }
        if should_widen {
            let widened = self.attempt(decision, 0, class, false, analyze_hazards, log);
            let widened_outcome = widened.finalize(decision_state, self.atn_fallback);
            if widened_outcome.kind == DecisionKind::Static {
                return widened_outcome;
            }
        }
        if self.should_retry_bounded(decision, exceeded_frontier, class, &outcome) {
            let bounded = self.attempt(decision, i32::MAX, class, true, analyze_hazards, log);
            return bounded.finalize(decision_state, self.atn_fallback);
        }
        outcome
    }

    fn finish_partial(
        &self,
        attempt: Attempt,
        depth_bound: i32,
        class: Option<PrecedenceClass>,
        analyze_hazards: bool,
        log: &mut CostLog,
    ) -> DecisionOutcome {
        let decision = attempt.decision;
        let decision_state = self.atn.decision_to_state[decision];
        let exceeded_frontier = attempt.exceeded_frontier;
        let outcome = attempt.finalize(decision_state, self.atn_fallback);
        if outcome.kind == DecisionKind::Static {
            return outcome;
        }
        if self.should_retry_bounded(decision, exceeded_frontier, class, &outcome) {
            let bounded = self.attempt(decision, depth_bound, class, true, analyze_hazards, log);
            return bounded.finalize(decision_state, self.atn_fallback);
        }
        outcome
    }

    fn should_retry_bounded(
        &self,
        decision: usize,
        exceeded_frontier: bool,
        class: Option<PrecedenceClass>,
        outcome: &DecisionOutcome,
    ) -> bool {
        let may_gain_guards = class.is_some() || self.postfix_shape(decision).is_some();
        exceeded_frontier
            && (outcome.table.is_some()
                || (may_gain_guards
                    && matches!(outcome.no_table, Some(NoTableReason::LowCoverage { .. }))))
    }

    fn is_precedence_loop_decision(&self, state: ATNStateRef) -> bool {
        if state.state_type() != ATNStateType::StarLoopEntry {
            return false;
        }
        let rule_start = self.atn.rule_to_start_state[state.get_rule_index() as usize];
        let is_left_recursive = rule_start
            .try_as::<RuleStartState>()
            .is_some_and(|start| start.is_left_recursive);
        if !is_left_recursive {
            return false;
        }
        let transitions = state.get_transitions();
        let Some(last) = transitions.last() else {
            return false;
        };
        let loop_end = last.get_target();
        if loop_end.state_type() != ATNStateType::LoopEnd {
            return false;
        }
        loop_end.has_epsilon_only_transitions()
            && loop_end
                .get_transitions()
                .first()
                .is_some_and(|transition| {
                    transition.get_target().state_type() == ATNStateType::RuleStop
                })
    }

    /// Builds one table per precedence class of a precedence-loop decision.
    ///
    /// Once a class has no table, `--static-dfa-only` rejects the decision,
    /// so the tables of the later classes skip the hazard analysis.
    fn build_precedence(&self, mut outcome: DecisionOutcome, log: &mut CostLog) -> DecisionOutcome {
        let state = self.atn.decision_to_state[outcome.decision];
        let target_rule = state.get_rule_index();
        let mut cutoffs = BTreeSet::new();
        for atn_state in self.atn.iter_states() {
            if atn_state.get_rule_index() != target_rule {
                continue;
            }
            for transition in atn_state.get_transitions() {
                if let Some(predicate) = transition.try_as::<PrecedencePredicateTransition>() {
                    cutoffs.insert(predicate.precedence());
                }
            }
        }
        if cutoffs.is_empty() {
            outcome.no_table = Some(NoTableReason::EmptyPrecedenceLoop);
            return outcome;
        }

        let cutoffs: Vec<i32> = cutoffs.into_iter().collect();
        let mut representatives = Vec::with_capacity(cutoffs.len() + 1);
        representatives.push(0);
        for cutoff in &cutoffs {
            representatives.push(cutoff.saturating_add(1));
        }

        let mut tables: Vec<Option<StaticDfaTable>> = vec![None; representatives.len()];
        let mut max_k = 0_i32;
        let mut total_states = 0_usize;
        let mut escapes = 0_usize;
        let mut first_failure: Option<NoTableReason> = None;
        let mut failed = 0_usize;

        for class_index in 0..representatives.len() {
            let class = PrecedenceClass {
                rule_index: target_rule,
                eval_value: representatives[class_index],
                lo: representatives[class_index],
                hi: cutoffs.get(class_index).copied().unwrap_or(i32::MAX),
            };
            log.class = Some(class_index);
            let class_outcome =
                self.build_table(outcome.decision, Some(class), first_failure.is_none(), log);

            BudgetCutoff::merge_opt(&mut outcome.budget_cutoff, class_outcome.budget_cutoff);
            for conflict in &class_outcome.conflicts {
                if !outcome.conflicts.contains(conflict) {
                    outcome.conflicts.push(conflict.clone());
                }
            }
            if outcome.conflict_witness.is_none() {
                outcome.conflict_witness = class_outcome.conflict_witness.clone();
            }
            if outcome.budget_witness.is_none() {
                outcome.budget_witness = class_outcome.budget_witness.clone();
            }

            match class_outcome.table {
                Some(table) => {
                    max_k = max_k.max(table.max_k);
                    total_states += table.states.len();
                    escapes += table.escapes();
                    tables[class_index] = Some(table);
                }
                None => {
                    failed += 1;
                    if first_failure.is_none() {
                        first_failure = class_outcome.no_table;
                    }
                }
            }
        }

        let total = tables.len();
        outcome.kind = if failed == total {
            DecisionKind::Adaptive
        } else if failed > 0 || escapes > 0 {
            DecisionKind::Hybrid
        } else {
            DecisionKind::Static
        };
        if failed > 0 {
            outcome.no_table = Some(NoTableReason::PrecedenceClasses {
                failed,
                total,
                first: Box::new(
                    first_failure.expect("a failed precedence class records a no_table reason"),
                ),
            });
        }
        if failed < total {
            outcome.k = max_k;
            outcome.num_dfa_states = total_states;
        }
        outcome.escapes = escapes;
        outcome.precedence = Some(PrecedenceTables {
            cutoffs,
            tables,
            adaptive_classes: failed,
            escapes,
        });
        outcome
    }

    /// Runs one build attempt (see [`Self::run_attempt`]) and records its
    /// cost in `log`.
    fn attempt(
        &self,
        decision: usize,
        depth_bound: i32,
        precedence: Option<PrecedenceClass>,
        enforce_frontier: bool,
        analyze_hazards: bool,
        log: &mut CostLog,
    ) -> Attempt {
        let started = self.profile.then(Instant::now);
        let mut attempt = self.run_attempt(
            decision,
            depth_bound,
            precedence,
            enforce_frontier,
            analyze_hazards,
        );
        let outcome = if attempt.exhausted {
            AttemptOutcome::VisitBudget
        } else if attempt.failed.is_some() || attempt.budget_cutoff.is_some() {
            AttemptOutcome::StateBudget
        } else {
            AttemptOutcome::Finished
        };
        let kind = if depth_bound == 0 {
            AttemptKind::Widened
        } else if self.atn_fallback == AtnFallback::Forbidden {
            AttemptKind::StaticOnly
        } else {
            AttemptKind::Default
        };
        log.record(
            AttemptCost {
                kind,
                class: log.class,
                outcome,
                visits: attempt.visits,
                states: attempt.state_count,
                elapsed: started.map(|started| started.elapsed()),
            },
            &std::mem::take(&mut attempt.rule_visits),
        );
        attempt
    }

    /// One build attempt: fresh simulator, fresh arena, fresh scratch arena.
    ///
    /// A fresh manager per attempt is deliberate: each attempt needs an empty
    /// DFA. The build reads its states back by their numbers, which a DFA
    /// gives out densely in creation order. It also reads back every edge in
    /// the DFA. The states and edges of an earlier attempt break both. A
    /// shared manager also gives all precedence classes of a decision the same
    /// DFA, but one config set can have other successors in another class.
    fn run_attempt(
        &self,
        decision: usize,
        depth_bound: i32,
        precedence: Option<PrecedenceClass>,
        enforce_frontier: bool,
        analyze_hazards: bool,
    ) -> Attempt {
        let atn = self.atn;
        let decision_state = atn.decision_to_state[decision];
        let loop_block = precedence.map(|_| self.loop_block(decision));
        let postfix = self.postfix_shape(decision);
        let budget = self.budget(decision);
        // A precedence class table is selected by the precedence, which a
        // group arm cannot see.
        let descent = if precedence.is_none() {
            self.descent.plan(decision)
        } else {
            None
        };
        let manager = ParserATNSimulatorManager::new(atn);
        Arena::with(|arena| {
            let simulator = ParserATNSimulator::new(manager.get_simulator(arena));
            let scratch = Bump::new();
            let mut build = DecisionBuild {
                atn,
                simulator: &simulator,
                dfa: simulator
                    .decision_to_dfa(decision)
                    .expect("every decision has a DFA"),
                scratch: &scratch,
                closure: Closure::new(
                    atn,
                    &self.call_sites,
                    &scratch,
                    depth_bound,
                    precedence,
                    loop_block,
                    postfix.clone(),
                    budget.max_visits,
                ),
                depth_bound,
                frontier: Frontier::of(&budget, self.atn_fallback),
                enforce_frontier,
                max_states: budget.max_states,
                max_k: budget.max_k,
                has_escape: false,
                budget_spent: false,
                allow_continuations: self.atn_fallback == AtnFallback::Allowed
                    && precedence.is_none(),
                precedence_class: precedence.is_some(),
                postfix,
                descent,
                guard_candidates: BTreeSet::new(),
                guard_iterate_candidates: BTreeMap::new(),
                guard: None,
                guard_regions: &self.guard_regions,
                static_only: self.atn_fallback == AtnFallback::Forbidden,
                analyze_hazards,
                decision_state,
                decision,
                info: HashMap::new(),
                accept_sinks: HashMap::new(),
                queue: BinaryHeap::new(),
                next_queue_seq: 0,
                depth1_budget: None,
                completion_depths: &self.completion_depths,
                attempt: Attempt {
                    decision,
                    ..Attempt::default()
                },
            };
            build.run()
        })
    }
}

/// `len` empty cells.
fn once_locks<T>(len: usize) -> Box<[OnceLock<T>]> {
    (0..len).map(|_| OnceLock::new()).collect()
}

/// The groups of `plan` that some state of `table` accepts with.
fn used_groups(plan: Option<&DescentPlan>, table: &StaticDfaTable) -> Option<DescentPlan> {
    let groups: Vec<_> = plan?
        .groups
        .iter()
        .filter(|group| {
            table
                .states
                .iter()
                .any(|state| state.accept == group.virtual_alt)
        })
        .cloned()
        .collect();
    (!groups.is_empty()).then_some(DescentPlan { groups })
}

/// The stack-guard sets of one decision table.
#[derive(Debug)]
struct GuardSets {
    /// The danger set of every guard group.
    groups: Vec<Vec<i32>>,
    /// The pass set that all guard groups share.
    pass: Vec<i32>,
}

impl GuardSets {
    /// The guard group with the danger set `danger`, added when it is new.
    fn group_of(&mut self, danger: Vec<i32>) -> u32 {
        let index = match self.groups.iter().position(|group| *group == danger) {
            Some(index) => index,
            None => {
                self.groups.push(danger);
                self.groups.len() - 1
            }
        };
        index as u32
    }
}

struct QueuedState<'sim> {
    depth: usize,
    expanded: bool,
    configs: usize,
    seq: usize,
    state: &'sim ParserDFAState<'sim>,
}

impl Eq for QueuedState<'_> {}

impl PartialEq for QueuedState<'_> {
    fn eq(&self, other: &Self) -> bool {
        (self.depth, self.expanded, self.configs, self.seq)
            == (other.depth, other.expanded, other.configs, other.seq)
    }
}

impl Ord for QueuedState<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.depth, other.expanded, other.configs, other.seq).cmp(&(
            self.depth,
            self.expanded,
            self.configs,
            self.seq,
        ))
    }
}

impl PartialOrd for QueuedState<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// One decision build, alive for one attempt.
struct DecisionBuild<'b, 'sim, 'a> {
    atn: &'static ATN,
    simulator: &'b ParserATNSimulator<'sim>,
    dfa: &'sim ParserDFA<'sim>,
    scratch: &'a Bump,
    closure: Closure<'static, 'a>,
    /// Widening bound: `i32::MAX` for the default attempt, `0` for the fully
    /// widened one.
    depth_bound: i32,
    /// The soft limits of the table once it has an escape state.
    frontier: Frontier,
    enforce_frontier: bool,
    /// The state cap of the table.
    max_states: usize,
    /// The lookahead depth that a source set: the table stops expanding at
    /// this depth, even without an escape state.
    max_k: Option<usize>,
    /// A state escapes: the table defers anyway, so the construction stops at
    /// the frontier.
    has_escape: bool,
    /// The attempt spent the state cap or the visit budget: every state that
    /// is not expanded yet becomes a frontier escape.
    budget_spent: bool,
    /// Whether frontier escape states may carry a continuation. Precedence
    /// classes never do: their tables are selected by the parser's precedence,
    /// which the resumed SLL simulation cannot reproduce from a config set.
    /// Under `--static-dfa-only`, no escape state reaches the runtime.
    allow_continuations: bool,
    /// `true` when this attempt builds one precedence class of a precedence
    /// loop decision.
    precedence_class: bool,
    /// The decision's optional-postfix shape, when it has one.
    postfix: Option<PostfixShape>,
    /// The decision's shared-descent groups, when the attempt may accept with
    /// them.
    descent: Option<DescentPlan>,
    /// Take-vetoed conflict states that may resolve as a guarded take. A
    /// sorted set keeps the resolution order deterministic.
    guard_candidates: BTreeSet<i32>,
    /// Iterate/exit states (conflict or budget frontier) vetoed only on foreign
    /// consumption. The value is `true` for a conflict state, and `false` for
    /// a budget escape that was never conflict-analyzed.
    guard_iterate_candidates: BTreeMap<i32, bool>,
    /// The guard sets of the decision, once a state resolved as a guarded
    /// state (an optional-postfix take or a precedence-loop iterate).
    guard: Option<GuardSets>,
    guard_regions: &'a OnceLock<GuardRegions>,
    /// `true` under `--static-dfa-only`: the table has no adaptive fallback.
    static_only: bool,
    /// Whether a `--static-dfa-only` table analyzes the hijack hazards of its
    /// resolved states.
    analyze_hazards: bool,
    decision: usize,
    decision_state: ATNStateRef,
    info: HashMap<i32, StateFacts>,
    /// For each alternative, the shared accept state of the config sets with
    /// only that viable alternative.
    accept_sinks: HashMap<i32, &'sim ParserDFAState<'sim>>,
    queue: BinaryHeap<QueuedState<'sim>>,
    next_queue_seq: usize,
    depth1_budget: Option<(usize, u64)>,
    completion_depths: &'a [OnceLock<BitSet>],
    attempt: Attempt,
}

impl<'b, 'sim, 'a> DecisionBuild<'b, 'sim, 'a>
where
    'sim: 'a,
{
    fn push_queue(&mut self, state: &'sim ParserDFAState<'sim>, depth: usize, expanded: bool) {
        let seq = self.next_queue_seq;
        self.next_queue_seq += 1;
        self.queue.push(QueuedState {
            depth,
            expanded,
            configs: state.configs().length(),
            seq,
            state,
        });
    }

    fn remaining_depth1_states(&self, current_state: i32) -> usize {
        let mut seen = HashSet::new();
        seen.insert(current_state);
        for item in &self.queue {
            if item.depth != 1 {
                continue;
            }
            let number = item.state.state_number();
            if self
                .info
                .get(&number)
                .is_some_and(|facts| !facts.expanded && facts.verdict == Verdict::Decide)
            {
                seen.insert(number);
            }
        }
        seen.len()
    }

    fn run(&mut self) -> Attempt {
        let start = self.closure.start(self.decision_state);
        if self.closure.exhausted {
            self.attempt.failed = Some(BudgetCutoff::Visits);
            self.attempt.start_exhausted = true;
            return self.take_attempt();
        }
        let Some(start_state) = self.intern(start, 0) else {
            self.attempt.failed = Some(BudgetCutoff::States);
            return self.take_attempt();
        };
        self.dfa.set_s0(start_state);
        if self.attempt.predicated {
            return self.take_attempt();
        }

        while let Some(QueuedState { state, .. }) = self.queue.pop() {
            let state_number = state.state_number();
            let Some(facts) = self.info.get(&state_number).cloned() else {
                continue;
            };
            if facts.verdict != Verdict::Decide {
                continue;
            }
            if facts.expanded && self.attempt.exhausted {
                if facts.depth > 0 {
                    self.escape_frontier(state_number, &facts);
                    continue;
                }
                self.attempt.failed = Some(BudgetCutoff::Visits);
                self.attempt.start_exhausted = true;
                break;
            }
            // The frontier is a *creation* budget: a state that was already
            // expanded keeps its successors, even though re-expanding it
            // after a taint increase may now run past the frontier.
            if !facts.expanded {
                let at_frontier = self.at_frontier(facts.depth);
                let at_max_k = self.max_k.is_some_and(|max_k| facts.depth >= max_k);
                if self.budget_spent
                    || (at_frontier && (self.enforce_frontier || self.has_escape || at_max_k))
                {
                    self.escape_frontier(state_number, &facts);
                    continue;
                }
                // Before the first escape, the table can still be fully
                // static. A frontier state that a static resolution decides
                // keeps it static, while its expansion can find conflicts
                // that no resolution decides.
                if at_frontier && self.resolve_frontier(state_number, &facts, false) {
                    continue;
                }
                if at_frontier && !self.enforce_frontier {
                    self.attempt.exceeded_frontier = true;
                }
            }
            if let Some(facts) = self.info.get_mut(&state_number) {
                facts.expanded = true;
            }

            let closed = ClosedSet {
                configs: state.configs().get_items().cloned().collect(),
                taint: facts.taint.clone(),
                roots: facts.roots.clone(),
            };
            let (successors, visit_cutoff) = if facts.depth == 1 && !facts.expanded {
                let remaining_depth1 = self.remaining_depth1_states(state_number);
                let remaining_visits = self.closure.remaining_visits();
                let (initial_depth1, initial_remaining) = *self
                    .depth1_budget
                    .get_or_insert((remaining_depth1, remaining_visits));
                let avg = (initial_remaining / initial_depth1 as u64).max(1);
                let fair_share = (remaining_visits / remaining_depth1 as u64).max(1);
                let reserve_per_state = (avg / 2).min(fair_share);
                let above_reserve = remaining_visits.saturating_sub(
                    (remaining_depth1 as u64 - 1).saturating_mul(reserve_per_state),
                );
                let per_state_cap = avg.saturating_mul(5).max(fair_share);
                let slice = per_state_cap.min(above_reserve).max(fair_share);
                self.closure.successors_capped(&closed, slice)
            } else {
                let successors = self.closure.successors(&closed);
                (successors, self.closure.exhausted)
            };
            let global_visit_exhausted = self.closure.exhausted;
            // Past the start state, the table escapes the states that it has
            // no budget to expand instead of failing. The check before the
            // interning assumes that every successor is new, so no state is
            // interned without an edge to it. A state that is expanded again
            // keeps its old edges in the DFA, but an escape state has no
            // edges in the table (see `Self::snapshot`).
            let new_states = if facts.expanded { 0 } else { successors.len() };
            let state_cutoff = facts.depth > 0 && self.dfa.len() + new_states > self.max_states;
            let cut_off = facts.depth > 0 && (visit_cutoff || state_cutoff);
            if cut_off {
                let cutoff = match (state_cutoff, visit_cutoff) {
                    (true, true) => BudgetCutoff::Both,
                    (true, false) => BudgetCutoff::States,
                    (false, true) => BudgetCutoff::Visits,
                    (false, false) => unreachable!(),
                };
                self.attempt.exhausted |= global_visit_exhausted;
                self.closure.exhausted = false;
                if state_cutoff || global_visit_exhausted {
                    self.budget_spent = true;
                }
                BudgetCutoff::merge_opt(&mut self.attempt.budget_cutoff, Some(cutoff));
                self.escape_frontier(state_number, &facts);
                continue;
            }
            if self.closure.exhausted {
                // Only the expansion of the start state gets here, and no
                // table can do without it.
                self.attempt.failed = Some(BudgetCutoff::Visits);
                self.attempt.start_exhausted = true;
                break;
            }
            let mut targets = Vec::with_capacity(successors.len());
            for (label, successor) in successors {
                // Only the successors of the start state can run past the
                // state cap here.
                let Some(target) = self.intern(successor, facts.depth + 1) else {
                    self.attempt.failed = Some(BudgetCutoff::States);
                    break;
                };
                targets.push((label, target));
            }
            if self.attempt.failed.is_some() {
                break;
            }
            for (label, target) in targets {
                for interval in label.as_interval_set().as_slice() {
                    for token in interval.a..=interval.b {
                        if token < EOF || token > self.atn.max_token_type {
                            continue;
                        }
                        self.dfa.set_edge(state, (token + 1) as usize, target);
                    }
                }
            }
        }

        if self.attempt.failed.is_none() {
            self.prune_unreachable();
            self.finalize_guards();
            // `finalize` drops a table with an escaping start state, so its
            // continuations and hazards are not needed.
            let start_decides = self.info.get(&0).is_some_and(|facts| {
                !matches!(facts.verdict, Verdict::Escape(ESCAPE | CONFLICT_ESCAPE))
            });
            let has_escape = self
                .info
                .values()
                .any(|facts| matches!(facts.verdict, Verdict::Escape(ESCAPE | CONFLICT_ESCAPE)));
            let will_retry_partial = self.attempt.exceeded_frontier && start_decides && has_escape;
            if !will_retry_partial {
                self.attach_continuations();
            }
            let analyze =
                self.static_only && self.analyze_hazards && start_decides && !will_retry_partial;
            let hazards = if analyze {
                self.state_hazards()
            } else {
                HashMap::new()
            };
            let mut table = self.snapshot(&hazards);
            if analyze {
                self.compute_guard_hazards(&mut table);
            }
            self.attempt.table = Some(table);
        }
        self.take_attempt()
    }

    /// Ends the expansion at a state that the construction does not expand: a
    /// static resolution decides it (see [`Self::resolve_frontier`]), or else
    /// it escapes.
    fn escape_frontier(&mut self, number: i32, facts: &StateFacts) {
        if self.resolve_frontier(number, facts, true) {
            return;
        }
        self.set_escape(number, ESCAPE);
        if self.postfix.is_some() && self.state_has_take_skip_pair(number) {
            // A budget escape of a take/skip state: the guarded take can
            // still resolve it (eligibility is evaluated once the
            // construction is complete).
            self.guard_candidates.insert(number);
        }
    }

    /// Decides a frontier state statically: the shared-descent group that
    /// covers it accepts it, or the iterate of a precedence loop does. When
    /// `escaping` is set, the state escapes otherwise, and an iterate/exit
    /// state becomes a candidate for the guarded iterate.
    fn resolve_frontier(&mut self, number: i32, facts: &StateFacts, escaping: bool) -> bool {
        if self.accept_descent_group(number, facts) {
            return true;
        }
        // A budget frontier that sits on the iterate/exit pair of a
        // precedence loop stays decidable by substitution. No conflict
        // materialized before the cutoff. The attestations are still a
        // lineage property, independent of any collision. So the argument
        // below applies, and iterate is the answer the runtime would reach.
        // Before depth 2, a multi-token operator of another precedence class
        // can still separate the alternatives, so the substitution does not
        // apply there.
        self.precedence_class
            && facts.depth >= 2
            && self.resolve_frontier_iterate(number, facts, escaping)
    }

    /// Whether a state at `depth` is at the frontier of a partial table.
    ///
    /// The state cap of the frontier applies from depth 2. When an expansion
    /// exceeds the state cap of the table or spends the visit budget, its
    /// state escapes (see [`Self::run`]).
    fn at_frontier(&self, depth: usize) -> bool {
        let state_cap = self.frontier.state_cap.min(self.max_states);
        depth >= self.frontier.depth_cap || (self.dfa.len() >= state_cap && depth >= 2)
    }

    /// Moves the attempt out of the build, recording what it observed.
    fn take_attempt(&mut self) -> Attempt {
        self.attempt.decision = self.decision;
        self.attempt.state_count = self.dfa.len();
        self.attempt.facts = std::mem::take(&mut self.closure.facts);
        self.attempt.exhausted |= self.closure.exhausted;
        self.attempt.visits = self.closure.visits();
        self.attempt.rule_visits = self.closure.take_rule_visits();
        self.attempt.descent = self.descent.take();
        std::mem::take(&mut self.attempt)
    }

    /// Second chance for take-vetoed conflicts of an optional-postfix decision.
    ///
    /// A vetoed conflict means some skip configuration left the dangling-else
    /// discipline: it consumed tokens outside the decision's own take path. Such
    /// a configuration is realizable at run time only on stacks whose
    /// epsilon-pop chase from the block end reaches the guard root its first
    /// out-of-discipline consumption passed through. So the conflict resolves to
    /// take under a runtime stack test: defer when the chase can reach a root.
    ///
    /// The roots ride on the skip configurations, so each guarded state tests
    /// only the roots of the skip readings that are still alive in it. States
    /// with equal danger sets share one guard group.
    ///
    /// The eligibility rules are conservative. A widening-born skip
    /// configuration is an analysis artifact unless it is rooted, a clean
    /// out-of-discipline skip has no root for the guard to catch, a root that the
    /// block end reaches without popping a frame makes the guard fire on every
    /// stack, and a block end that cannot pop out of its rule means no root is
    /// ever reachable (that case resolves to plain take accepts instead). A
    /// state whose danger set is empty resolves to a plain take accept too,
    /// when its take is real. The attempt bookkeeping is applied only once
    /// every bail has passed.
    fn finalize_guards(&mut self) {
        self.finalize_guarded_takes();
        if self.guard.is_none() {
            self.finalize_guarded_iterates();
        }
    }

    /// The guard roots of the configs of `alt` in a state.
    fn state_roots(&self, state: i32, alt: i32) -> BTreeSet<i32> {
        self.info.get(&state).map_or_else(BTreeSet::new, |facts| {
            facts
                .roots
                .iter()
                .filter(|((_, config_alt), _)| *config_alt == alt)
                .flat_map(|(_, roots)| roots.iter().copied())
                .collect()
        })
    }

    /// Empty guard sets with the pass set of the grammar.
    fn new_guard_sets(&self) -> GuardSets {
        let regions = self
            .guard_regions
            .get_or_init(|| GuardRegions::of(self.atn));
        GuardSets {
            groups: Vec::new(),
            pass: regions.pass().to_vec(),
        }
    }

    /// The danger set of a guard-root set.
    fn guard_danger(&self, roots: &BTreeSet<i32>) -> Vec<i32> {
        self.guard_regions
            .get_or_init(|| GuardRegions::of(self.atn))
            .danger(roots)
    }

    /// The take pass of [`Self::finalize_guards`].
    fn finalize_guarded_takes(&mut self) {
        let Some(postfix) = self.postfix.clone() else {
            return;
        };
        if self.guard_candidates.is_empty() {
            return;
        }
        let candidates = std::mem::take(&mut self.guard_candidates);
        // A root in the epsilon closure of the block end trips the guard on
        // every stack, so it guards nothing.
        let reachable_from_block = epsilon_closure(postfix.block_end_state);
        let mut accepted: Vec<(i32, BTreeSet<i32>, bool)> = Vec::new();
        for state in candidates {
            let Some(facts) = self.info.get(&state) else {
                continue;
            };
            if facts.predicated
                || !matches!(
                    facts.verdict,
                    Verdict::Escape(ESCAPE) | Verdict::Escape(CONFLICT_ESCAPE)
                )
            {
                continue;
            }
            let roots = self.state_roots(state, POSTFIX_SKIP_ALT);
            if roots.is_empty()
                || roots
                    .iter()
                    .any(|root| reachable_from_block.contains(*root as usize))
            {
                continue;
            }
            let mut eligible = true;
            let mut real_take = false;
            let Some(dfa_state) = self.dfa.get_state(state) else {
                continue;
            };
            for config in dfa_state.configs().get_items() {
                let taint = taint_of(&facts.taint, config);
                if config.get_alt() == POSTFIX_TAKE_ALT && taint & WIDENED_TAINT == 0 {
                    real_take = true;
                }
                if config.get_alt() != POSTFIX_SKIP_ALT {
                    continue;
                }
                // A widening-born skip is an analysis artifact unless it is
                // rooted: then its bottom frame is a real follow state and the
                // guard covers its realizability.
                if taint & (WIDENED_TAINT | ROOTED_TAINT) == WIDENED_TAINT {
                    eligible = false;
                    break;
                }
                if taint & ROOTED_TAINT != 0
                    || postfix.skip_mirrors(config.get_state(), config.get_context(), taint)
                {
                    continue;
                }
                eligible = false;
                break;
            }
            // Under `--static-dfa-only`, the state predicts the take on every
            // stack without the guard, so the guard needs no proof that the
            // take is real.
            if !eligible || !(real_take || self.static_only) {
                continue;
            }
            accepted.push((state, roots, real_take));
        }
        if accepted.is_empty() {
            return;
        }

        let chase_leaves_the_rule = {
            let rule_stop =
                self.atn.rule_to_stop_state[self.decision_state.get_rule_index() as usize];
            rule_stop.get_state_number() >= 0
                && reachable_from_block.contains(rule_stop.get_state_number() as usize)
        };
        let mut guard = self.new_guard_sets();
        for (state, roots, real_take) in accepted {
            let danger = if chase_leaves_the_rule {
                self.guard_danger(&roots)
            } else {
                Vec::new()
            };
            if danger.is_empty() && !real_take {
                // Without a guard, only a real take resolves the conflict.
                continue;
            }
            if danger.is_empty() {
                // No guard can ever trip: resolve to a plain take accept.
                if let Some(facts) = self.info.get_mut(&state) {
                    facts.verdict = Verdict::Accept(POSTFIX_TAKE_ALT);
                    facts.resolved_conflict = true;
                }
                continue;
            }
            let group = guard.group_of(danger);
            if let Some(facts) = self.info.get_mut(&state) {
                facts.verdict = Verdict::Escape(GUARDED);
                facts.guarded_alt = POSTFIX_TAKE_ALT;
                facts.guard_defer_alt = POSTFIX_SKIP_ALT;
                facts.guard_group = group;
            }
        }
        if !guard.groups.is_empty() {
            self.guard = Some(guard);
        }
    }

    /// Second chance for iterate/exit conflicts that only foreign consumption
    /// vetoed.
    ///
    /// The canonical shape is Trino's `BETWEEN ... AND ...`: the exit lineage
    /// consumes an operator token in a frame that is not an enclosing invocation
    /// of the same loop. Such a conflict resolves to iterate (alt 1) under a
    /// stack guard, or outright when no invoking state in the grammar can trip
    /// it, because then the foreign consumption is unreachable on every real
    /// stack and exit can never uniquely win. Like the guarded take, each state
    /// tests only the roots of its own exit readings.
    fn finalize_guarded_iterates(&mut self) {
        if self.guard_iterate_candidates.is_empty() {
            return;
        }
        let candidates = std::mem::take(&mut self.guard_iterate_candidates);
        // A root in the epsilon closure of the rule stop trips the guard on
        // every stack, so it guards nothing.
        let rule_stop = self.atn.rule_to_stop_state[self.decision_state.get_rule_index() as usize];
        let reachable_from_stop =
            (rule_stop.get_state_number() >= 0).then(|| epsilon_closure(rule_stop));
        let mut accepted: Vec<(i32, bool, BTreeSet<i32>, bool)> = Vec::new();
        for (state, conflict) in candidates {
            let Some(facts) = self.info.get(&state) else {
                continue;
            };
            if facts.predicated
                || !matches!(
                    facts.verdict,
                    Verdict::Escape(ESCAPE) | Verdict::Escape(CONFLICT_ESCAPE)
                )
            {
                continue;
            }
            let roots = self.state_roots(state, EXIT_ALT);
            if roots.is_empty()
                || reachable_from_stop.as_ref().is_some_and(|reachable| {
                    roots.iter().any(|root| reachable.contains(*root as usize))
                })
            {
                continue;
            }
            let mut eligible = true;
            let mut real_iterate = false;
            let Some(dfa_state) = self.dfa.get_state(state) else {
                continue;
            };
            for config in dfa_state.configs().get_items() {
                let taint = taint_of(&facts.taint, config);
                if config.get_alt() == ITERATE_ALT
                    && taint & (WIDENED_TAINT | FOREIGN_CONSUME_TAINT) == 0
                {
                    real_iterate = true;
                }
                if config.get_alt() != EXIT_ALT {
                    continue;
                }
                // Foreign consumption must be rooted: then the stack guard
                // covers its realizability. A widening-born exit is an analysis
                // artifact unless it is rooted too.
                if taint & FOREIGN_CONSUME_TAINT != 0 && taint & ROOTED_TAINT == 0 {
                    eligible = false;
                    break;
                }
                if taint & (WIDENED_TAINT | ROOTED_TAINT) == WIDENED_TAINT {
                    eligible = false;
                    break;
                }
            }
            // Under `--static-dfa-only`, the state predicts the iterate on
            // every stack without the guard, so the guard needs no proof
            // that the iterate is real.
            if !eligible || !(real_iterate || self.static_only) {
                continue;
            }
            accepted.push((state, conflict, roots, real_iterate));
        }
        if accepted.is_empty() {
            return;
        }

        let mut guard = self.new_guard_sets();
        for (state, conflict, roots, real_iterate) in accepted {
            let danger = self.guard_danger(&roots);
            if danger.is_empty() && !real_iterate {
                // Without a guard, only a real iterate resolves the conflict.
                continue;
            }
            if danger.is_empty() && !conflict && self.static_only {
                // A frontier state without a guard stays an escape, so that
                // the `--static-dfa-only` report counts its guess.
                continue;
            }
            let Some(facts) = self.info.get_mut(&state) else {
                continue;
            };
            facts.resolved_conflict = true;
            if danger.is_empty() {
                facts.verdict = Verdict::Accept(ITERATE_ALT);
                continue;
            }
            facts.verdict = Verdict::Escape(GUARDED);
            facts.guarded_alt = ITERATE_ALT;
            facts.guard_defer_alt = EXIT_ALT;
            facts.guard_group = guard.group_of(danger);
        }
        if !guard.groups.is_empty() {
            self.guard = Some(guard);
        }
    }

    /// Attaches a [`ParserContinuation`] to every frontier escape state whose
    /// config set the resumed SLL simulation can rebuild faithfully.
    ///
    /// A continuation replays a config set without the input, so it must not
    /// carry any approximation: a widened context would erase the caller that
    /// only exists at run time, and a precedence-filtered set would need the
    /// parser's precedence. Decisions that reach a left-recursive loop are
    /// excluded for the same reason - a resumed simulation cannot reapply the
    /// precedence filter the static construction applied.
    fn attach_continuations(&mut self) {
        if !self.allow_continuations {
            return;
        }
        for state in self.dfa.get_states() {
            let number = state.state_number();
            let Some(facts) = self.info.get(&number) else {
                continue;
            };
            if facts.depth == 0 || facts.verdict != Verdict::Escape(ESCAPE) {
                continue;
            }
            let configs = state.configs();
            if !continuation_is_faithful(configs, &facts.taint) {
                continue;
            }
            let continuation = serialize_continuation(number as u32, configs, &facts.taint);
            if continuation_words(&continuation) > MAX_CONTINUATION_WORDS {
                continue;
            }
            if let Some(facts) = self.info.get_mut(&number) {
                facts.continuation = Some(continuation);
            }
        }
    }

    /// Classifies a closed set and interns it as a DFA state.
    fn intern(
        &mut self,
        closed: ClosedSet<'a>,
        depth: usize,
    ) -> Option<&'sim ParserDFAState<'sim>> {
        let closed = self.closure.canonical_set(closed);
        let (facts, effects) = self.classify(&closed, depth);
        // An accept state with one viable alternative has no edges, and the
        // runtime reads only its alternative. All such accept states of one
        // alternative share one state, so they use one state of the state cap.
        if let Verdict::Accept(alt) = facts.verdict
            && alt > 0
            && depth > 0
            && !facts.resolved_conflict
            && !facts.predicated
        {
            return self.accept_sink(alt, facts, effects);
        }
        let interned = self.closure.merged_config_set(&closed);
        let mut proposed = ProposedDFAState::new(interned);
        proposed.is_accept_state = matches!(facts.verdict, Verdict::Accept(_));
        proposed.prediction = facts.accept_value();
        let state = self.dfa.add_state(proposed, self.simulator);
        let number = state.state_number();
        let newly_interned = !self.info.contains_key(&number);
        if !newly_interned {
            // The same config set is reachable through paths whose taint
            // differs, and a DFA state's verdict is decided once. Widening,
            // precedence-guard or boundary taint on *any* path invalidates a
            // verdict that trusted the config set, so the state escapes
            // rather than carry a verdict the runtime could disagree with.
            let state_configs = state.configs();
            let mut new_trust_taint = 0;
            let mut verdict = Verdict::Decide;
            let mut resolved_conflict = false;
            let mut roots_grew = false;
            let mut expanded = false;
            if let Some(existing) = self.info.get_mut(&number) {
                verdict = existing.verdict;
                resolved_conflict = existing.resolved_conflict;
                expanded = existing.expanded;
                for (key, taint) in &facts.taint {
                    let entry = existing.taint.entry(*key).or_insert(0);
                    new_trust_taint |= *taint & !*entry & TRUST_TAINT;
                    *entry |= *taint;
                }
                for (key, roots) in &facts.roots {
                    let entry = existing.roots.entry(*key).or_default();
                    let before = entry.len();
                    entry.extend(roots.iter().copied());
                    roots_grew |= entry.len() != before;
                }
                // The fallback is a property of the whole set's taint, so a
                // later boundary path can add one.
                existing.fallback = fallback_of(state_configs, &existing.taint);
                // A predicate on any path makes a later escape of the state a
                // syntax error under `--static-dfa-only`, not a guess.
                existing.predicated |= facts.predicated;
            }
            if new_trust_taint != 0 {
                self.on_taint_increase(state, verdict, resolved_conflict);
            } else if roots_grew && expanded && verdict == Verdict::Decide {
                // The roots ride on the configs, so the successors of an
                // expanded state must see the new roots too.
                let existing_depth = self.info.get(&number).map_or(0, |existing| existing.depth);
                self.push_queue(state, existing_depth, true);
            }
            return Some(state);
        }
        if number as usize >= self.max_states {
            return None;
        }
        if effects.conflict.is_some() {
            self.attempt.first_conflict_state.get_or_insert(number);
        }
        self.commit(number, effects);
        let decide = facts.verdict == Verdict::Decide;
        let is_escape = matches!(facts.verdict, Verdict::Escape(_));
        self.has_escape |= is_escape;
        if is_escape && !self.budget_spent {
            self.attempt.non_budget_escape = true;
        }
        self.info.insert(number, facts);
        if decide && self.attempt.failed.is_none() {
            self.push_queue(state, depth, false);
        }
        Some(state)
    }

    /// The shared accept state of alternative `alt`, interned on first use.
    ///
    /// The shared state has its own config set: one config at the decision
    /// state. No closure output contains the decision state, because it has
    /// only epsilon transitions. So no other config set of the build becomes
    /// this state, and the state keeps no fallback, taint, or roots of one
    /// path.
    fn accept_sink(
        &mut self,
        alt: i32,
        facts: StateFacts,
        effects: ClassifyEffects,
    ) -> Option<&'sim ParserDFAState<'sim>> {
        if let Some(state) = self.accept_sinks.get(&alt) {
            return Some(*state);
        }
        let config = dbt_antlr_runtime::analysis::ATNConfig::new(
            self.decision_state,
            alt,
            Some(PredictionContextRef::new_empty()),
        );
        let sink = ClosedSet {
            configs: vec![config],
            taint: Taint::default(),
            roots: GuardRoots::default(),
        };
        let mut proposed = ProposedDFAState::new(self.closure.merged_config_set(&sink));
        proposed.is_accept_state = true;
        proposed.prediction = alt;
        let state = self.dfa.add_state(proposed, self.simulator);
        let number = state.state_number();
        if number as usize >= self.max_states {
            return None;
        }
        self.commit(number, effects);
        self.accept_sinks.insert(alt, state);
        self.info.insert(
            number,
            StateFacts {
                fallback: 0,
                taint: Taint::default(),
                roots: GuardRoots::default(),
                ..facts
            },
        );
        Some(state)
    }

    /// Accepts a budget-frontier state with the virtual alternative of the
    /// shared-descent group that covers it, when there is one (see
    /// [`DescentPlan::group_for`]).
    fn accept_descent_group(&mut self, number: i32, facts: &StateFacts) -> bool {
        let Some(plan) = &self.descent else {
            return false;
        };
        let Some(dfa_state) = self.dfa.get_state(number) else {
            return false;
        };
        let configs: Vec<_> = dfa_state.configs().get_items().cloned().collect();
        let mut live = BitSet::new();
        for config in &configs {
            live.insert(config.get_alt() as usize);
        }
        let Some(alt) = plan
            .group_for(&live, facts.depth, &configs, &facts.taint)
            .map(|group| group.virtual_alt)
        else {
            return false;
        };
        if let Some(facts) = self.info.get_mut(&number) {
            facts.verdict = Verdict::Accept(alt);
            // The group accept depends on the taint of the paths.
            facts.resolved_conflict = true;
        }
        true
    }

    /// Whether the interned state's live alternatives are exactly the
    /// take/skip pair of an optional-postfix decision.
    fn state_has_take_skip_pair(&self, number: i32) -> bool {
        let mut alts = BitSet::new();
        let Some(dfa_state) = self.dfa.get_state(number) else {
            return false;
        };
        for config in dfa_state.configs().get_items() {
            alts.insert(config.get_alt() as usize);
        }
        is_take_skip_pair(&alts)
    }

    /// Whether an iterate/exit attestation makes a state a candidate for the
    /// guarded iterate: only foreign consumption of the exit lineage vetoes
    /// the iterate.
    ///
    /// With the adaptive fallback, the iterate lineage must also be real: a
    /// guard that does not defer predicts the iterate without the adaptive
    /// simulation. Under `--static-dfa-only`, an iterate lineage that consumes
    /// foreign tokens is a candidate too (as in `CAST ( x AS INT64 )`, where
    /// the alias reading reaches the `)` of a parenthesis). Without the guard,
    /// such a state predicts the iterate on every stack. The guard predicts the
    /// exit where the stack reaches a root of the exit lineage, and keeps the
    /// iterate everywhere else.
    fn is_iterate_guard_candidate(&self, attestation: i32) -> bool {
        attestation == (ITERATE_REAL | EXIT_FOREIGN)
            || (self.static_only && attestation == EXIT_FOREIGN)
    }

    /// Resolves a budget-frontier state of a precedence class to iterate when
    /// the iterate/exit substitution attestations permit it.
    ///
    /// Returns `false` when the state is not the loop's iterate/exit pair, when
    /// the loop's guarded alternatives are not simple, or when the attestation
    /// is anything but a clean iterate reading - the caller then escapes the
    /// state as usual, and with `escaping`, the state can become a candidate
    /// for the guarded iterate.
    fn resolve_frontier_iterate(
        &mut self,
        number: i32,
        facts: &StateFacts,
        escaping: bool,
    ) -> bool {
        let Some(dfa_state) = self.dfa.get_state(number) else {
            return false;
        };
        let configs: Vec<dbt_antlr_runtime::analysis::ATNConfig<'_>> =
            dfa_state.configs().get_items().cloned().collect();
        let mut alts = BitSet::new();
        for config in &configs {
            alts.insert(config.get_alt() as usize);
        }
        if !is_iterate_exit_pair(&alts) {
            return false;
        }
        let Some(block) = self.closure.loop_block() else {
            return false;
        };
        if !block.guards_are_simple() {
            return false;
        }
        // Under `--static-dfa-only`, an escape of the iterate/exit pair
        // predicts the iterate on every stack, so a guard is never worse than
        // the escape. The state stays an escape when no guard applies, so
        // that the report counts it.
        let covered = block.iterate_covers_exit(&configs, &facts.taint);
        if !covered && !self.static_only {
            return false;
        }
        let attestation = block.iterate_exit_attestation(&configs, &facts.taint);
        if covered && attestation == ITERATE_REAL && self.keeps_caller_frames() {
            if let Some(facts) = self.info.get_mut(&number) {
                facts.verdict = Verdict::Accept(ITERATE_ALT);
                facts.resolved_conflict = true;
            }
            return true;
        }
        if escaping && self.postfix.is_none() && self.is_iterate_guard_candidate(attestation) {
            self.guard_iterate_candidates.insert(number, false);
        }
        false
    }

    /// Reacts to a second path adding trust taint to an already interned
    /// state.
    ///
    /// The state's verdict was decided once, from the first path's taint, and a
    /// state cannot be both trusted and untrusted: a table cannot carry a
    /// verdict that any path contradicts. A trusted accept therefore becomes an
    /// escape, while a decision state is queued again so its successors see
    /// the union of both paths' taint.
    fn on_taint_increase(
        &mut self,
        state: &'sim ParserDFAState<'sim>,
        verdict: Verdict,
        resolved_conflict: bool,
    ) {
        let number = state.state_number();
        match verdict {
            // A single-viable-alternative accept does not depend on taint: no
            // other alternative survives the lookahead, so there is nothing a
            // different path could add. A *conflict-resolved* accept does: the
            // min-alt argument only holds while every path that reaches the
            // state is taint-free.
            Verdict::Accept(_) if resolved_conflict => {
                // A frontier state resolved by substitution carries no
                // conflict, so it degrades to a plain escape: the runtime
                // restarts the adaptive simulation, and the report keeps the
                // budget witness.
                let conflicted = self
                    .info
                    .get(&number)
                    .is_some_and(|facts| !facts.conflicts.is_empty());
                if conflicted {
                    self.attempt.first_conflict_state.get_or_insert(number);
                }
                self.has_escape = true;
                if !self.budget_spent {
                    self.attempt.non_budget_escape = true;
                }
                if let Some(facts) = self.info.get_mut(&number) {
                    // The taint of every path, the first one included.
                    let approximated = facts
                        .taint
                        .values()
                        .any(|taint| taint & (WIDENED_TAINT | PRECPRED_TAINT) != 0);
                    facts.verdict = if conflicted {
                        Verdict::Escape(CONFLICT_ESCAPE)
                    } else {
                        Verdict::Escape(ESCAPE)
                    };
                    // A precedence class table never takes the conflict
                    // short cut: the runtime's full-context restart would
                    // have to reapply the precedence filter, which the
                    // table cannot carry.
                    if !approximated && !self.precedence_class {
                        facts.conflict_min_alt = facts.conflicts.iter().min().copied().unwrap_or(0);
                    }
                }
            }
            Verdict::Accept(_) | Verdict::Escape(_) => {}
            Verdict::Decide => {
                if let Some(facts) = self.info.get(&number)
                    && facts.expanded
                {
                    self.push_queue(state, facts.depth, true);
                }
            }
        }
    }

    /// Records the attempt-level bookkeeping of a newly interned state.
    fn commit(&mut self, number: i32, effects: ClassifyEffects) {
        if effects.guard_candidate {
            self.guard_candidates.insert(number);
        }
        if effects.guard_iterate_candidate {
            self.guard_iterate_candidates.insert(number, true);
        }
        if let Some(alts) = effects.conflict {
            self.attempt.conflicts.push(alts);
        }
        if effects.fail_predicated {
            self.attempt.predicated = true;
        }
    }

    /// Decides what a closed config set means: an accept, an escape, or a
    /// point that needs successors.
    fn classify(&mut self, closed: &ClosedSet<'a>, depth: usize) -> (StateFacts, ClassifyEffects) {
        let mut effects = ClassifyEffects::default();
        let mut fallback = 0;
        let mut alts = BitSet::new();
        let mut predicated = false;
        for config in &closed.configs {
            alts.insert(config.get_alt() as usize);
            let taint = closed.taint_of(config);
            if taint & BOUNDARY_TAINT != 0 && (fallback == 0 || config.get_alt() < fallback) {
                fallback = config.get_alt();
            }
            if taint & PRED_TAINT != 0 {
                predicated = true;
            }
        }
        let mut facts = StateFacts {
            depth,
            verdict: Verdict::Decide,
            taint: closed.taint.clone(),
            fallback,
            conflicts: Vec::new(),
            resolved_conflict: false,
            expanded: false,
            conflict_min_alt: 0,
            guarded_alt: 0,
            guard_defer_alt: 0,
            guard_group: 0,
            roots: closed.roots.clone(),
            continuation: None,
            predicated: false,
        };
        if predicated {
            // A config behind a predicate is only viable when that predicate
            // holds for the current parse, so this state can never decide:
            // predicated branches escape while the unpredicated ones stay
            // static. A predicate in front of the decision itself makes the
            // whole decision predicated.
            if depth == 0 {
                effects.fail_predicated = true;
            } else {
                facts.verdict = Verdict::Escape(ESCAPE);
                facts.predicated = true;
            }
            return (facts, effects);
        }
        if alts.len() <= 1 {
            facts.verdict = Verdict::Accept(alts.iter().next().map_or(0, |alt| alt as i32));
            return (facts, effects);
        }
        let all_stops = !closed.configs.is_empty()
            && closed
                .configs
                .iter()
                .all(|config| config.get_state().state_type() == ATNStateType::RuleStop);
        let merged = self.closure.merged_config_set(closed);
        // A state where every config sits at a rule stop is a conflict for the
        // runtime (`compute_target_state` checks it before the SLL predicate):
        // all alternatives already left the decision and only full-context
        // prediction can tell which one the caller frame allows.
        if !all_stops
            && !has_sll_conflict_terminating_prediction(self.scratch, PredictionMode::SLL, &merged)
        {
            return (facts, effects);
        }

        let subsets = conflicting_alt_subsets(self.scratch, &merged);
        let exact = all_subsets_conflict(subsets.iter()) && all_subsets_equal(subsets.iter());
        let mut widened_tainted = false;
        let mut precedence_tainted = false;
        let mut any_boundary = false;
        let mut all_boundary = true;
        for config in &closed.configs {
            let taint = closed.taint_of(config);
            if taint & WIDENED_TAINT != 0 {
                widened_tainted = true;
            }
            if taint & PRECPRED_TAINT != 0 {
                precedence_tainted = true;
            }
            if taint & BOUNDARY_TAINT != 0 {
                any_boundary = true;
            }
            // "All configs crossed the boundary" only holds when no path
            // reached this set from inside the decision.
            if taint & BOUNDARY_TAINT == 0 || taint & INTERIOR_TAINT != 0 {
                all_boundary = false;
            }
        }
        let conflicting = get_alts(subsets.into_iter());
        facts.conflicts = conflicting.iter().map(|alt| alt as i32).collect();

        // Enter/exit ambiguity of a precedence loop, resolved to iterate by
        // substitution. When the exit lineage consumed every scanned token
        // inside the operator loop, no foreign consumption remains. Then every
        // exit derivation maps onto an iterate derivation of the same
        // sentence.
        //
        // The mapping is precedence-safe. The outer reading consumes the
        // operator through the same loop alternative, with the same guard and
        // the same operand precedence. The iterate lineage can always take the
        // free loop exit and rejoin the continuation the exit lineage heads
        // for. So exit never becomes uniquely viable, and the runtime answers
        // iterate whenever it terminates.
        //
        // Exactness is not required. Inexact subsets are the common case:
        // they diverge because exit carries caller continuations that iterate
        // only rejoins after its free exit.
        //
        // Three vetoes decide the state: foreign consumption outside the loop
        // block, pending token obligations the iterate reading does not owe,
        // and a missing iterate attestation. A compound guarded alternative
        // breaks the mapping altogether (see `LoopBlock`).
        //
        // The vetoes read the frames of the contexts. The widened attempt
        // drops every caller frame, so it does not resolve the conflict.
        if let Some(block) = self.closure.loop_block()
            && block.guards_are_simple()
            && is_iterate_exit_pair(&conflicting)
        {
            let attestation = block.iterate_exit_attestation(&closed.configs, &closed.taint);
            if attestation == ITERATE_REAL && self.keeps_caller_frames() {
                facts.resolved_conflict = true;
                facts.verdict = Verdict::Accept(ITERATE_ALT);
                return (facts, effects);
            }
            if self.postfix.is_none() && self.is_iterate_guard_candidate(attestation) {
                // Vetoed only by foreign consumption of the exit lineage: a
                // candidate for the guarded iterate, resolved once the
                // construction is complete.
                effects.guard_iterate_candidate = true;
            }
        }

        // Optional-postfix take rule: resolve a take/skip conflict to take when
        // every skip configuration mirrors the take reading and a non-widened
        // take configuration attests take viability. Like the iterate/exit
        // vetoes, the mirror check reads the frames of the contexts, so the
        // widened attempt does not apply the rule.
        if let Some(postfix) = self.postfix.clone()
            && is_take_skip_pair(&conflicting)
        {
            let mut mirror_ok = true;
            let mut real_take = false;
            for config in &closed.configs {
                let taint = closed.taint_of(config);
                if config.get_alt() == POSTFIX_SKIP_ALT
                    && !postfix.skip_mirrors(config.get_state(), config.get_context(), taint)
                {
                    mirror_ok = false;
                }
                if config.get_alt() == POSTFIX_TAKE_ALT && taint & WIDENED_TAINT == 0 {
                    real_take = true;
                }
            }
            if mirror_ok && real_take && self.keeps_caller_frames() {
                facts.resolved_conflict = true;
                facts.verdict = Verdict::Accept(POSTFIX_TAKE_ALT);
                return (facts, effects);
            }
            if real_take || self.static_only {
                // Vetoed only by the mirror check: a candidate for the guarded
                // take, resolved once the construction is complete.
                effects.guard_candidate = true;
            }
        }

        let untrusted = precedence_tainted
            || widened_tainted
            || (any_boundary && !all_boundary)
            || !self.uniform_pop_depths(&conflicting, any_boundary, all_boundary)
            || self.precedence_class;

        if !untrusted && exact {
            facts.resolved_conflict = true;
            facts.verdict = Verdict::Accept(conflicting.iter().next().map_or(0, |alt| alt as i32));
            return (facts, effects);
        }

        if depth > 0 {
            let min_alt = self.utrust_min_alt(closed, &conflicting, any_boundary, all_boundary);
            if min_alt > 0 {
                facts.resolved_conflict = true;
                facts.verdict = Verdict::Accept(min_alt);
                return (facts, effects);
            }
        }
        // A conflict among the members of a shared-descent group is resolved
        // by the group's tail dispatch after one parse of the shared rule.
        if let Some(alt) = self
            .descent
            .as_ref()
            .and_then(|plan| plan.group_for(&alts, depth, &closed.configs, &closed.taint))
            .map(|group| group.virtual_alt)
        {
            effects.guard_candidate = false;
            facts.resolved_conflict = true;
            facts.verdict = Verdict::Accept(alt);
            return (facts, effects);
        }
        effects.conflict = Some(facts.conflicts.clone());
        facts.verdict = Verdict::Escape(CONFLICT_ESCAPE);
        // The conflict already ended Stage 1, so the runtime can resume at
        // Stage 2 without re-walking, but only while the set is a faithful
        // SLL set: a widened or precedence-filtered set is an approximation
        // whose conflicting alternatives the runtime may not reproduce.
        // A precedence class table never takes the conflict short cut: the
        // runtime's full-context restart would have to reapply the
        // precedence filter, which the table cannot carry.
        if depth > 0 && !widened_tainted && !precedence_tainted && !self.precedence_class {
            facts.conflict_min_alt = conflicting.iter().next().map_or(0, |alt| alt as i32);
        }
        (facts, effects)
    }

    /// Whether a rule call keeps the frame of its caller, at least up to the
    /// widening bound. The widened attempt gives every call an empty context.
    const fn keeps_caller_frames(&self) -> bool {
        self.depth_bound > 0
    }

    fn set_escape(&mut self, number: i32, marker: i32) {
        self.has_escape = true;
        if let Some(facts) = self.info.get_mut(&number) {
            if matches!(facts.verdict, Verdict::Escape(_)) {
                return;
            }
            facts.verdict = Verdict::Escape(marker);
        }
    }

    /// The prefix-hijack hazards of the escape states that
    /// `--static-dfa-only` resolves to an alternative, for at most
    /// [`HAZARD_STATES`] of the shallowest of them.
    fn state_hazards(&mut self) -> HashMap<i32, StateHazard> {
        let mut candidates: Vec<(usize, i32)> = self
            .info
            .iter()
            .filter(|(_, facts)| {
                matches!(facts.verdict, Verdict::Escape(ESCAPE | CONFLICT_ESCAPE))
                    && !facts.predicated
            })
            .map(|(number, facts)| (facts.depth, *number))
            .collect();
        candidates.sort_unstable();
        let mut hazards = HashMap::new();
        // The analyzed states of each tie-break group, and whether one of
        // them is a hijack.
        let mut groups: HashMap<(i32, Vec<i32>), (usize, bool)> = HashMap::new();
        let mut analyzed = 0;
        for (_, number) in candidates {
            if analyzed == HAZARD_STATES {
                break;
            }
            let Some(facts) = self.info.get(&number).cloned() else {
                continue;
            };
            let Some(dfa_state) = self.dfa.get_state(number) else {
                continue;
            };
            let configs: Vec<_> = dfa_state.configs().get_items().cloned().collect();
            let alts: BTreeSet<i32> = configs
                .iter()
                .map(|config| config.get_alt())
                .filter(|alt| *alt > 0)
                .collect();
            let conflicts: BTreeSet<i32> = facts
                .conflicts
                .iter()
                .copied()
                .filter(|alt| *alt > 0)
                .collect();
            // The same choice as `StaticDfaTable::resolve_escapes`.
            let candidates = if conflicts.is_empty() {
                &alts
            } else {
                &conflicts
            };
            let Some(&chosen) = candidates.first() else {
                continue;
            };
            let dropped: Vec<i32> = candidates.iter().copied().skip(1).collect();
            if dropped.is_empty() {
                continue;
            }
            let group = groups.entry((chosen, dropped.clone())).or_default();
            if group.1 || group.0 == HAZARD_STATES_PER_GROUP {
                continue;
            }
            group.0 += 1;
            analyzed += 1;
            let verdict =
                hazard::analyze(&mut self.closure, &configs, &facts.taint, chosen, &dropped);
            group.1 = matches!(verdict, hazard::HijackVerdict::Hijack { .. });
            let greedy = self.is_greedy_choice(&dropped);
            hazards.insert(number, StateHazard { verdict, greedy });
        }
        hazards
    }

    /// The stack-hijack hazards of the guarded states of `table`, for at most
    /// [`GUARD_HAZARD_STATES`] of them and [`GUARD_HAZARD_STATES_PER_GROUP`]
    /// of each guard group. The frontier states come first, then the
    /// shallowest states: a frontier state stops where the lookahead budget
    /// cut the guarded alternative off, and so can hide a hijack that a
    /// conflict state shows the end of.
    fn compute_guard_hazards(&mut self, table: &mut StaticDfaTable) {
        let mut candidates: Vec<(bool, usize, usize)> = table
            .states
            .iter()
            .enumerate()
            .filter(|(_, state)| {
                state.is_guarded()
                    && state.guarded_alt > 0
                    && state.guard_defer_alt > 0
                    && !state.predicated
            })
            .map(|(index, state)| (!state.conflicts.is_empty(), state.depth, index))
            .collect();
        candidates.sort_unstable();
        let decision_rule = self.decision_state.get_rule_index();
        // The stacks of each guard group, the analyzed states of the group,
        // and whether one of them is a hijack.
        let mut groups: HashMap<u32, (hazard::GuardChains, usize, bool)> = HashMap::new();
        let mut analyzed = 0;
        for (_, _, index) in candidates {
            if analyzed == GUARD_HAZARD_STATES {
                break;
            }
            let state = &table.states[index];
            let (group_index, guarded_alt, defer_alt) =
                (state.guard_group, state.guarded_alt, state.guard_defer_alt);
            let Some(witness) = table.witness(index) else {
                continue;
            };
            let group = groups.entry(group_index).or_insert_with(|| {
                let danger = table
                    .guard_groups
                    .get(group_index as usize)
                    .map_or(&[][..], Vec::as_slice);
                let chains = hazard::guard_chains(
                    self.closure.call_sites(),
                    decision_rule,
                    self.closure.precedence(),
                    danger,
                    &table.guard_pass,
                );
                (chains, 0, false)
            });
            if group.2 || group.1 == GUARD_HAZARD_STATES_PER_GROUP {
                continue;
            }
            group.1 += 1;
            analyzed += 1;
            let verdict = hazard::analyze_guard(
                &mut self.closure,
                self.decision_state,
                &group.0,
                &witness,
                (guarded_alt, defer_alt),
            );
            group.2 = matches!(verdict, StackHijackVerdict::Hijack { .. });
            table.states[index].stack_hazard = Some(verdict);
        }
    }

    /// Whether a guess that drops `dropped` is the greedy choice of the
    /// decision: it drops the exit of a loop, or the skip of an optional block
    /// or element.
    fn is_greedy_choice(&self, dropped: &[i32]) -> bool {
        let transitions = self.decision_state.get_transitions();
        match self.decision_state.state_type() {
            ATNStateType::StarLoopEntry | ATNStateType::PlusLoopBack => dropped == [2],
            _ => {
                let last = transitions.len();
                dropped == [last as i32]
                    && transitions.last().is_some_and(|transition| {
                        transition.get_target().state_type() == ATNStateType::BlockEnd
                    })
            }
        }
    }

    fn prune_unreachable(&mut self) {
        let state_count = self.dfa.len();
        let mut edges: Vec<Vec<i32>> = vec![Vec::new(); state_count];
        for (from, _, to) in self.dfa.enumerate_edges() {
            let from = from.state_number();
            if self
                .info
                .get(&from)
                .is_some_and(|facts| facts.verdict == Verdict::Decide)
            {
                edges[from as usize].push(to.state_number());
            }
        }
        let mut reachable = vec![false; state_count];
        let mut queue = VecDeque::new();
        if self.info.contains_key(&0) {
            reachable[0] = true;
            queue.push_back(0_i32);
        }
        let mut reached = 0;
        while let Some(number) = queue.pop_front() {
            reached += 1;
            for &target in &edges[number as usize] {
                if let Some(slot) = reachable.get_mut(target as usize)
                    && !*slot
                    && self.info.contains_key(&target)
                {
                    *slot = true;
                    queue.push_back(target);
                }
            }
        }
        if reached == state_count {
            return;
        }
        self.info
            .retain(|number, _| reachable.get(*number as usize).copied().unwrap_or(false));
        self.guard_candidates
            .retain(|number| reachable.get(*number as usize).copied().unwrap_or(false));
        self.guard_iterate_candidates
            .retain(|number, _| reachable.get(*number as usize).copied().unwrap_or(false));
        if let Some(first) = self.attempt.first_conflict_state
            && !reachable.get(first as usize).copied().unwrap_or(false)
        {
            self.attempt.first_conflict_state = None;
        }
    }

    fn snapshot(&mut self, hazards: &HashMap<i32, StateHazard>) -> StaticDfaTable {
        let mut remap = vec![usize::MAX; self.dfa.len()];
        let mut states: Vec<StaticDfaState> = Vec::with_capacity(self.info.len());
        for state in self.dfa.get_states() {
            let number = state.state_number();
            let Some(facts) = self.info.get(&number) else {
                continue;
            };
            remap[number as usize] = states.len();
            let escape = matches!(facts.verdict, Verdict::Escape(_));
            let viable_alts = if escape {
                let alts: BTreeSet<i32> = state
                    .configs()
                    .get_items()
                    .map(|config| config.get_alt())
                    .filter(|alt| *alt > 0)
                    .collect();
                alts.into_iter().collect()
            } else {
                Vec::new()
            };
            states.push(StaticDfaState {
                accept: facts.accept_value(),
                fallback: facts.fallback,
                depth: facts.depth,
                conflicts: facts.conflicts.clone(),
                conflict_min_alt: facts.conflict_min_alt,
                guarded_alt: facts.guarded_alt,
                guard_defer_alt: facts.guard_defer_alt,
                guard_group: facts.guard_group,
                continuation: facts.continuation.clone(),
                viable_alts,
                predicated: facts.predicated,
                resolution: None,
                hazard: hazards.get(&number).cloned(),
                stack_hazard: None,
                edges: Vec::new(),
            });
        }
        if let Some(first) = self.attempt.first_conflict_state {
            self.attempt.first_conflict_state = usize::try_from(first)
                .ok()
                .and_then(|old| remap.get(old).copied())
                .filter(|&new| new != usize::MAX)
                .and_then(|new| i32::try_from(new).ok());
        }

        // Only a decision state keeps its edges: a state that the construction
        // expanded and then escaped, when a taint increase made it expand
        // again past the budget, answers before the runtime reads them.
        let mut per_state: Vec<Vec<(i32, usize)>> = vec![Vec::new(); states.len()];
        for (from, symbol, to) in self.dfa.enumerate_edges() {
            let from = from.state_number();
            let decides = self
                .info
                .get(&from)
                .is_some_and(|facts| facts.verdict == Verdict::Decide);
            let from = remap.get(from as usize).copied().unwrap_or(usize::MAX);
            let to = remap
                .get(to.state_number() as usize)
                .copied()
                .unwrap_or(usize::MAX);
            if decides && from < per_state.len() && to < states.len() {
                per_state[from].push((symbol as i32 - 1, to));
            }
        }
        for (index, mut edges) in per_state.into_iter().enumerate() {
            edges.sort_unstable();
            let mut merged: Vec<StaticDfaEdge> = Vec::new();
            for (token, target) in edges {
                match merged.last_mut() {
                    Some(edge) if edge.target == target && edge.hi + 1 == token => edge.hi = token,
                    _ => merged.push(StaticDfaEdge {
                        lo: token,
                        hi: token,
                        target,
                    }),
                }
            }
            states[index].edges = merged;
        }

        let mut table = StaticDfaTable::finish(self.decision, states);
        if let Some(guard) = &self.guard {
            table.guard_groups.clone_from(&guard.groups);
            table.guard_pass.clone_from(&guard.pass);
        }
        table
    }

    /// `true` when the conflicting alternatives leave the decision after the
    /// same number of tokens: the ambiguity is then faithful to the runtime.
    fn uniform_pop_depths(
        &self,
        conflicting: &BitSet,
        any_boundary: bool,
        all_boundary: bool,
    ) -> bool {
        if !any_boundary {
            return true;
        }
        if !all_boundary {
            return false;
        }
        if self.depth_bound == 0 {
            return false;
        }
        let alt_starts: Vec<ATNStateRef> = self
            .decision_state
            .get_transitions()
            .iter()
            .map(|transition| transition.get_target())
            .collect();
        let mut reference: Option<usize> = None;
        for alt in conflicting {
            if alt == 0 || alt > alt_starts.len() {
                return false;
            }
            let depths = self.completion_depths(alt_starts[alt - 1]);
            if depths.len() != 1 || depths.contains(VARIABLE_DEPTH) {
                return false;
            }
            let depth = depths.iter().next().unwrap();
            match reference {
                None => reference = Some(depth),
                Some(known) if known != depth => return false,
                Some(_) => {}
            }
        }
        true
    }

    fn completion_depths(&self, alt_start: ATNStateRef) -> &'a BitSet {
        let state_number = usize::try_from(alt_start.get_state_number())
            .expect("an alternative start is a real ATN state");
        self.completion_depths[state_number].get_or_init(|| {
            let mut visiting = BTreeSet::new();
            let mut budget = 100_000_i32;
            token_counts_to_stop(alt_start, &mut visiting, &mut budget)
        })
    }

    /// The "untrusted-but-uniform" resolution: when every conflicting state is
    /// attested by a faithful config of the lowest alternative, predict it.
    fn utrust_min_alt(
        &mut self,
        closed: &ClosedSet<'a>,
        conflicting: &BitSet,
        any_boundary: bool,
        all_boundary: bool,
    ) -> i32 {
        if any_boundary && !all_boundary {
            return 0;
        }
        let Some(min_alt) = conflicting.iter().next() else {
            return 0;
        };
        let min_alt = min_alt as i32;

        let mut clean_by_state: HashMap<i32, Vec<dbt_antlr_runtime::analysis::ATNConfig<'a>>> =
            HashMap::new();
        for config in &closed.configs {
            if config.get_alt() != min_alt {
                continue;
            }
            // Only a config whose every path stayed inside the decision and
            // never had its context widened or its precedence guard assumed can
            // attest for the lowest alternative.
            if closed.taint_of(config) & (BOUNDARY_TAINT | WIDENED_TAINT | PRECPRED_TAINT) != 0 {
                continue;
            }
            clean_by_state
                .entry(config.get_state().get_state_number())
                .or_default()
                .push(config.clone());
        }
        if clean_by_state.is_empty() {
            return 0;
        }

        let mut by_state_contexts: HashMap<i32, Vec<PredictionContextRef<'a>>> = HashMap::new();
        for config in &closed.configs {
            // Widening removes frames from the context of a competing config,
            // so its context does not show the continuation of its real
            // stacks. No attestation can match such a context, and equal
            // contexts after widening can make a conflict that the runtime
            // never sees.
            if config.get_alt() != min_alt && closed.taint_of(config) & WIDENED_TAINT != 0 {
                return 0;
            }
            let state_number = config.get_state().get_state_number();
            let context = config.get_context().expect("config has a context");
            let entries = by_state_contexts.entry(state_number).or_default();
            if !entries.contains(&context) {
                entries.push(context);
            }
        }

        for (state_number, contexts) in by_state_contexts {
            let Some(attestations) = clean_by_state.get(&state_number) else {
                return 0;
            };
            for context in contexts {
                let assured = attestations.iter().any(|config| {
                    top_prefix_wildcard_match(
                        config.get_context().expect("config has a context"),
                        context,
                        self.scratch,
                    )
                });
                if !assured {
                    return 0;
                }
            }
        }
        min_alt
    }
}

/// `true` when the config set can be replayed without the input.
///
/// Widening, an unevaluated precedence guard, or a predicate each stand for
/// information only the run time has, so a continuation never carries them.
fn continuation_is_faithful(configs: &ATNConfigSet<'_>, taint: &Taint) -> bool {
    // `INTERIOR_TAINT` and `BOUNDARY_TAINT` are facts about where the config
    // came from, not approximations: the replay reproduces them.
    const APPROXIMATION: i32 = WIDENED_TAINT | PRECPRED_TAINT | PRED_TAINT;
    configs.get_items().all(|config| {
        let taint = taint
            .get(&(config.get_state().get_state_number(), config.get_alt()))
            .copied()
            .unwrap_or(0);
        taint & APPROXIMATION == 0
    })
}

fn continuation_words(continuation: &ParserContinuation) -> usize {
    let contexts = continuation
        .contexts
        .iter()
        .map(|node| match node {
            ContinuationContextNode::Singleton { .. } => 3,
            ContinuationContextNode::Array { parents, .. } => 1 + parents.len() * 2,
        })
        .sum::<usize>();
    // state + node count + config count, plus one config entry each.
    3 + contexts + continuation.configs.len() * 4
}

/// Serializes the closed config set of a frontier escape state.
///
/// The contexts become a topological node table: parents are always written
/// before the nodes that name them, so the run time rebuilds the DAG in one
/// pass.
fn serialize_continuation(
    state: u32,
    configs: &ATNConfigSet<'_>,
    taint: &Taint,
) -> ParserContinuation {
    fn return_state_number(return_state: ATNStateRef) -> i32 {
        if return_state == ATNStateRef::empty_return() {
            -1
        } else {
            return_state.get_state_number()
        }
    }

    fn intern_context<'a>(
        context: PredictionContextRef<'a>,
        nodes: &mut Vec<ContinuationContextNode>,
        seen: &mut HashMap<usize, u32>,
    ) -> u32 {
        const EMPTY_CONTEXT: u32 = u32::MAX;
        if context.is_empty() {
            return EMPTY_CONTEXT;
        }
        let key = context.as_ref() as *const PredictionContext<'a> as usize;
        if let Some(&index) = seen.get(&key) {
            return index;
        }
        let node = if context.length() == 1 {
            ContinuationContextNode::Singleton {
                parent: context
                    .get_parent(0)
                    .map_or(EMPTY_CONTEXT, |parent| intern_context(parent, nodes, seen)),
                return_state: return_state_number(context.get_return_state(0)),
            }
        } else {
            let mut parents = Vec::with_capacity(context.length());
            let mut return_states = Vec::with_capacity(context.length());
            for index in 0..context.length() {
                parents.push(
                    context
                        .get_parent(index)
                        .map_or(EMPTY_CONTEXT, |parent| intern_context(parent, nodes, seen)),
                );
                return_states.push(return_state_number(context.get_return_state(index)));
            }
            ContinuationContextNode::Array {
                parents,
                return_states,
            }
        };
        let index = nodes.len() as u32;
        nodes.push(node);
        seen.insert(key, index);
        index
    }

    let mut contexts = Vec::new();
    let mut seen = HashMap::new();
    let mut entries = Vec::with_capacity(configs.length());
    for config in configs.get_items() {
        let context = config.get_context().map_or(u32::MAX, |context| {
            intern_context(context, &mut contexts, &mut seen)
        });
        let taint = taint
            .get(&(config.get_state().get_state_number(), config.get_alt()))
            .copied()
            .unwrap_or(0);
        entries.push(ContinuationConfigEntry {
            state: config.get_state().get_state_number(),
            alt: config.get_alt(),
            context,
            outer_depth: i32::from(taint & BOUNDARY_TAINT != 0),
        });
    }
    ParserContinuation {
        state,
        contexts,
        configs: entries,
    }
}

fn fallback_of(configs: &ATNConfigSet<'_>, taint: &Taint) -> i32 {
    let mut fallback = 0;
    for config in configs.get_items() {
        let key = (config.get_state().get_state_number(), config.get_alt());
        if taint.get(&key).copied().unwrap_or(0) & BOUNDARY_TAINT != 0
            && (fallback == 0 || config.get_alt() < fallback)
        {
            fallback = config.get_alt();
        }
    }
    fallback
}

fn token_counts_to_stop(
    state: ATNStateRef,
    visiting: &mut BTreeSet<i32>,
    budget: &mut i32,
) -> BitSet {
    let mut out = BitSet::with_capacity(VARIABLE_DEPTH + 1);
    if state.state_type() == ATNStateType::RuleStop {
        out.insert(0);
        return out;
    }
    let previous_budget = *budget;
    *budget -= 1;
    if previous_budget <= 0 || !visiting.insert(state.get_state_number()) {
        out.insert(VARIABLE_DEPTH);
        return out;
    }
    for transition in state.get_transitions() {
        let sub = if let Some(rule) =
            transition.try_as::<dbt_antlr_runtime::analysis::RuleTransition>()
        {
            let inner = token_counts_to_stop(rule.get_target(), visiting, budget);
            let rest = token_counts_to_stop(rule.follow_state, visiting, budget);
            sum_depths(&inner, &rest)
        } else if transition.is_epsilon() {
            token_counts_to_stop(transition.get_target(), visiting, budget)
        } else {
            shift_depths(&token_counts_to_stop(
                transition.get_target(),
                visiting,
                budget,
            ))
        };
        out.union_with(&sub);
    }
    visiting.remove(&state.get_state_number());
    out
}

fn shift_depths(input: &BitSet) -> BitSet {
    let mut out = BitSet::with_capacity(VARIABLE_DEPTH + 1);
    if input.contains(VARIABLE_DEPTH) {
        out.insert(VARIABLE_DEPTH);
        return out;
    }
    for depth in input {
        if depth + 1 > MAX_TRACKED_DEPTH {
            out.insert(VARIABLE_DEPTH);
            return out;
        }
        out.insert(depth + 1);
    }
    out
}

fn sum_depths(left: &BitSet, right: &BitSet) -> BitSet {
    let mut out = BitSet::with_capacity(VARIABLE_DEPTH + 1);
    if left.contains(VARIABLE_DEPTH) || right.contains(VARIABLE_DEPTH) {
        out.insert(VARIABLE_DEPTH);
        return out;
    }
    for a in left {
        for b in right {
            if a + b > MAX_TRACKED_DEPTH {
                out.insert(VARIABLE_DEPTH);
                return out;
            }
            out.insert(a + b);
        }
    }
    out
}

/// `true` when every branch of `shadow` reaches `exact`, treating a widened
/// joint (`$` under a return state) as matching anything.
fn top_prefix_wildcard_match<'a>(
    shadow: PredictionContextRef<'a>,
    exact: PredictionContextRef<'a>,
    scratch: &'a Bump,
) -> bool {
    if exact.length() > 1 {
        for i in 0..exact.length() {
            if !top_prefix_wildcard_match(shadow, branch(exact, i, scratch), scratch) {
                return false;
            }
        }
        return true;
    }
    if shadow.is_empty() {
        return true;
    }
    if shadow.length() > 1 {
        for i in 0..shadow.length() {
            if top_prefix_wildcard_match(branch(shadow, i, scratch), exact, scratch) {
                return true;
            }
        }
        return false;
    }
    if exact.is_empty() {
        return false;
    }
    let return_state = shadow.get_return_state(0);
    if return_state == ATNStateRef::empty_return() {
        return true;
    }
    if exact.get_return_state(0) == ATNStateRef::empty_return() {
        return false;
    }
    return_state == exact.get_return_state(0)
        && top_prefix_wildcard_match(
            shadow
                .get_parent(0)
                .unwrap_or_else(PredictionContextRef::new_empty),
            exact
                .get_parent(0)
                .unwrap_or_else(PredictionContextRef::new_empty),
            scratch,
        )
}

fn branch<'a>(
    context: PredictionContextRef<'a>,
    index: usize,
    scratch: &'a Bump,
) -> PredictionContextRef<'a> {
    let parent = context
        .get_parent(index)
        .unwrap_or_else(PredictionContextRef::new_empty);
    let return_state = context.get_return_state(index);
    if return_state == ATNStateRef::empty_return() && parent.is_empty() {
        return PredictionContextRef::new_empty();
    }
    PredictionContextRef::from(
        scratch.alloc(PredictionContext::new_singleton(Some(parent), return_state)),
    )
}
