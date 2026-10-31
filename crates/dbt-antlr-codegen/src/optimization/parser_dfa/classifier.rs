// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The SLL DFA classifier: subset construction, budgets, and trust rules.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

use dbt_antlr_runtime::analysis::DFA;
use dbt_antlr_runtime::analysis::{
    ATN, ATNConfigSet, ATNStateRef, ATNStateType, Arena, BOUNDARY_TAINT, BitSet, Bump, DFAState,
    EOF, FOREIGN_CONSUME_TAINT, IATNSimulator, INTERIOR_TAINT, PRECPRED_TAINT, PRED_TAINT,
    ParserATNSimulator, ParserATNSimulatorManager, PrecedencePredicateTransition,
    PredictionContext, PredictionContextRef, PredictionMode, ProposedDFAState, ROOTED_TAINT,
    RuleStartState, WIDENED_TAINT, all_subsets_conflict, all_subsets_equal,
    conflicting_alt_subsets, get_alts, has_sll_conflict_terminating_prediction,
};
use dbt_antlr_runtime::dfa::static_parser_dfa::{
    ContinuationConfigEntry, ContinuationContextNode, ParserContinuation,
};

use super::closure::{ClosedSet, Closure, ClosureFacts, PrecedenceClass, Taint, taint_of};
use super::loop_block::{ITERATE_ALT, ITERATE_REAL, LoopBlock, is_iterate_exit_pair};
use super::postfix::{PostfixShape, bottom_frame_is, compute_guard_data, reach_eps};
use super::table::{StaticDfaEdge, StaticDfaState, StaticDfaTable};
use super::{CONFLICT_ESCAPE, Category, ESCAPE, EscapeResolutions, GUARDED};
use crate::optimization::AtnFallback;

/// The state budget of one decision table.
pub(crate) const MAX_DFA_STATES: usize = 1000;
/// Depth at which a hybrid table stops expanding and escapes.
pub(crate) const HYBRID_DEPTH_CAP: usize = 4;
/// State count at which a hybrid table stops expanding beyond depth 2.
pub(crate) const HYBRID_STATE_CAP: usize = 64;
/// [`HYBRID_DEPTH_CAP`] under `--static-dfa-only`. Without the adaptive
/// fallback, a frontier state can only guess, so the table looks further.
pub(crate) const STATIC_ONLY_HYBRID_DEPTH_CAP: usize = 16;
/// [`HYBRID_STATE_CAP`] under `--static-dfa-only`. See
/// [`DecisionBuild::at_hybrid_frontier`] for the room it keeps below
/// [`MAX_DFA_STATES`].
pub(crate) const STATIC_ONLY_HYBRID_STATE_CAP: usize = MAX_DFA_STATES;
/// Below this share of useful start edges a hybrid table is not emitted,
/// unless `--static-dfa-only` removes the adaptive fallback.
pub(crate) const HYBRID_MIN_COVERAGE_PERCENT: u32 = 10;
/// Largest continuation a hybrid table emits, counted in wire words.
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

/// Taint bits that invalidate a trusted verdict when a second path adds them
/// to an already interned state.
const TRUST_TAINT: i32 =
    WIDENED_TAINT | PRECPRED_TAINT | BOUNDARY_TAINT | FOREIGN_CONSUME_TAINT | INTERIOR_TAINT;

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

const VIRTUAL_ALT_BASE: usize = 65_536;
const MAX_TRACKED_DEPTH: usize = 30;
const VARIABLE_DEPTH: usize = 31;

/// The verdict of one DFA state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Verdict {
    /// The state predicts this alternative.
    Accept(i32),
    /// The state escapes to the adaptive simulation, with the table marker it
    /// carries ([`ESCAPE`] or [`CONFLICT_ESCAPE`]).
    Escape(i32),
    /// The state is a decision point: it needs outgoing edges.
    Decide,
}

/// What the classifier remembers about one interned DFA state.
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
/// an already known config set must not count its conflicts twice, and must
/// not abort a build whose state set is already complete.
#[derive(Debug, Default)]
struct ClassifyEffects {
    exact_ambiguity: Option<BitSet>,
    utrust: bool,
    approximation: Option<BitSet>,
    context_sensitive: Option<BitSet>,
    conflict: Option<Vec<i32>>,
    /// A take-vetoed conflict state that may resolve as a guarded take. The
    /// payload is the conflicting alternative set, or `None` for a budget
    /// escape that was never conflict-analyzed.
    guard_candidate: Option<Option<BitSet>>,
    /// The decision state of the attempt is behind a predicate, so the whole
    /// decision depends on parser state and no table can decide it.
    fail_predicated: bool,
    abort: bool,
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

/// The policy of one build attempt.
#[derive(Clone, Copy, Debug)]
struct AttemptPolicy {
    /// Widening bound: `i32::MAX` for the unbounded attempt, `0` for the fully
    /// widened one.
    depth_bound: i32,
    /// Bound the construction and escape at the frontier instead of failing.
    hybrid: bool,
    /// Depth at which a hybrid construction stops expanding.
    hybrid_depth_cap: usize,
    /// State count at which a hybrid construction stops expanding beyond
    /// depth 2.
    hybrid_state_cap: usize,
    /// Stop at the first conflict the construction does not trust, keeping the
    /// work small for decisions that end up adaptive anyway.
    abort_on_untrusted_conflict: bool,
    /// Trust an exact-ambiguity conflict without the witness checks.
    trust_exact_ambiguity: bool,
}

impl AttemptPolicy {
    const fn unbounded() -> Self {
        Self {
            depth_bound: i32::MAX,
            hybrid: false,
            hybrid_depth_cap: HYBRID_DEPTH_CAP,
            hybrid_state_cap: HYBRID_STATE_CAP,
            abort_on_untrusted_conflict: false,
            trust_exact_ambiguity: true,
        }
    }

    const fn widened() -> Self {
        Self {
            depth_bound: 0,
            ..Self::unbounded()
        }
    }

    const fn hybrid() -> Self {
        Self {
            hybrid: true,
            ..Self::unbounded()
        }
    }

    const fn static_only_hybrid() -> Self {
        Self {
            hybrid_depth_cap: STATIC_ONLY_HYBRID_DEPTH_CAP,
            hybrid_state_cap: STATIC_ONLY_HYBRID_STATE_CAP,
            ..Self::hybrid()
        }
    }

    /// The first pass of the report: bail out of an untrusted conflict instead
    /// of exploring the whole lookahead, so only the decisions that stay
    /// adaptive pay for the deep pass.
    const fn report_pass() -> Self {
        Self {
            abort_on_untrusted_conflict: true,
            ..Self::unbounded()
        }
    }
}

/// The result of one build attempt.
#[derive(Debug, Default)]
struct Attempt {
    decision: usize,
    state_count: usize,
    table: Option<StaticDfaTable>,
    overflow: bool,
    /// Set when the attempt stopped at an untrusted conflict.
    stopped: bool,
    facts: ClosureFacts,
    utrust_resolutions: usize,
    exact_ambiguities: Vec<BitSet>,
    context_sensitive: Vec<BitSet>,
    approximations: Vec<BitSet>,
    /// Alternative sets of the conflicts the construction did not trust.
    conflicts: Vec<Vec<i32>>,
    /// DFA state number of the first conflict, for the report witness.
    first_conflict_state: Option<i32>,
    /// The same config set was reached by two paths with different trust
    /// taint, so no verdict derived from the first path can be trusted.
    taint_conflict: bool,
    /// The attempt ran under the hybrid policy.
    hybrid: bool,
    /// The decision state is behind a predicate: the decision is predicated.
    predicated: bool,
    /// The decision is an optional-postfix (`X Y?`) shape.
    has_postfix_shape: bool,
    /// Conflict states the optional-postfix take rule resolved to take.
    take_rule_fires: usize,
    /// Conflict states a guarded take resolved.
    guarded_takes: usize,
}

/// The precedence classes of one precedence-loop decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrecedenceTables {
    /// The precedence cut points of the loop, ascending.
    pub(crate) cutoffs: Vec<i32>,
    /// One table per class: `p<=cutoffs[0]`, `p=cutoffs[i-1]+1..=cutoffs[i]`,
    /// and `p>cutoffs[last]`.
    pub(crate) tables: Vec<Option<StaticDfaTable>>,
    pub(crate) adaptive_classes: usize,
    pub(crate) escapes: usize,
}

/// What the classifier decided about one prediction decision.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DecisionResult {
    pub(crate) decision: usize,
    pub(crate) rule_index: usize,
    pub(crate) state_number: i32,
    pub(crate) category: Category,
    /// Longest lookahead of a finite table, `-1` when unbounded or unknown.
    pub(crate) k: i32,
    pub(crate) num_dfa_states: usize,
    pub(crate) cyclic: bool,
    pub(crate) used_widening: bool,
    pub(crate) saw_predicate: bool,
    /// The decision state itself sits behind a predicate, so no construction
    /// can decide it and a hybrid retry cannot help.
    pub(crate) predicated_start: bool,
    pub(crate) first_predicate: Option<(i32, i32)>,
    pub(crate) saw_precedence_predicate: bool,
    /// The decision is an optional-postfix (`X Y?`) shape.
    pub(crate) has_postfix_shape: bool,
    /// Conflict states the optional-postfix take rule resolved to take.
    pub(crate) take_rule_fires: usize,
    /// Conflict states a guarded take resolved (Phase 5.2b).
    pub(crate) guarded_takes: usize,
    pub(crate) utrust_resolutions: usize,
    pub(crate) conflict_witness: Option<Vec<i32>>,
    pub(crate) budget_witness: Option<Vec<i32>>,
    pub(crate) conflicts: Vec<Vec<i32>>,
    pub(crate) approximations: Vec<Vec<i32>>,
    pub(crate) escapes: usize,
    /// Share of the start alphabet that reaches an accept state, in percent.
    pub(crate) coverage_percent: u32,
    pub(crate) table: Option<StaticDfaTable>,
    pub(crate) precedence: Option<PrecedenceTables>,
    /// The escape states that `--static-dfa-only` resolved, set once every
    /// table of the decision predicts without `adaptivePredict`.
    pub(crate) static_only: Option<EscapeResolutions>,
}

impl DecisionResult {
    fn new(decision: usize, state: ATNStateRef) -> Self {
        Self {
            decision,
            rule_index: usize::try_from(state.get_rule_index()).unwrap_or(0),
            state_number: state.get_state_number(),
            category: Category::LLK,
            k: -1,
            num_dfa_states: 0,
            cyclic: false,
            used_widening: false,
            saw_predicate: false,
            predicated_start: false,
            first_predicate: None,
            saw_precedence_predicate: false,
            has_postfix_shape: false,
            take_rule_fires: 0,
            guarded_takes: 0,
            utrust_resolutions: 0,
            conflict_witness: None,
            budget_witness: None,
            conflicts: Vec::new(),
            approximations: Vec::new(),
            escapes: 0,
            coverage_percent: 0,
            table: None,
            precedence: None,
            static_only: None,
        }
    }

    /// A short description of how the decision falls back at runtime, for
    /// `--report-decisions`.
    ///
    /// Under `--static-dfa-only`, the text counts the resolved escape states
    /// instead: `m` states predict the minimum conflicting alternative, `v`
    /// states predict the minimum viable alternative, and `x` states report a
    /// syntax error.
    pub(crate) fn fallback_text(&self) -> String {
        if let Some(resolved) = &self.static_only {
            let mut parts = Vec::new();
            if resolved.min_alt > 0 {
                parts.push(format!("{}m", resolved.min_alt));
            }
            if resolved.min_viable_alt > 0 {
                parts.push(format!("{}v", resolved.min_viable_alt));
            }
            if resolved.syntax_error > 0 {
                parts.push(format!("{}x", resolved.syntax_error));
            }
            if parts.is_empty() {
                return "0".to_owned();
            }
            return parts.join("+");
        }
        if self.category == Category::LR_PRECEDENCE {
            let Some(precedence) = &self.precedence else {
                return "all".to_owned();
            };
            if self.escapes == 0 && precedence.adaptive_classes == 0 {
                return "0".to_owned();
            }
            let mut text = String::new();
            if self.escapes > 0 {
                text.push_str(&format!("{}e", self.escapes));
            }
            if precedence.adaptive_classes > 0 {
                if !text.is_empty() {
                    text.push('+');
                }
                text.push_str(&format!("{}c", precedence.adaptive_classes));
            }
            return text;
        }
        if self.category == Category::LL1 {
            return "0".to_owned();
        }
        if self.table.is_none() {
            return "all".to_owned();
        }
        if self.escapes > 0 {
            self.escapes.to_string()
        } else if self.guarded_takes > 0 {
            format!("{}g", self.guarded_takes)
        } else {
            "0".to_owned()
        }
    }

    /// `true` when the decision never falls back to the adaptive simulation.
    ///
    /// `LL(1)` and non-greedy decisions need no table at all, and a table
    /// without escapes decides on its own; everything else keeps at least one
    /// adaptive path. A decision that `--static-dfa-only` resolved never falls
    /// back.
    pub(crate) fn is_fully_static(&self) -> bool {
        if self.static_only.is_some() {
            return true;
        }
        if self.guarded_takes > 0 {
            // A guarded take defers to the adaptive simulation when the parse
            // stack reaches a guard root.
            return false;
        }
        match self.category {
            Category::LL1
            | Category::NON_GREEDY
            | Category::LLK
            | Category::LLSTAR
            | Category::EXACT_AMBIG => true,
            Category::LR_PRECEDENCE => {
                self.escapes == 0
                    && self
                        .precedence
                        .as_ref()
                        .is_some_and(|precedence| precedence.adaptive_classes == 0)
            }
            Category::HYBRID => false,
            Category::CONTEXT_SENSITIVE | Category::PREDICATED | Category::OVERFLOW => false,
        }
    }
}

/// Classifies the decision states of one compiled parser unit.
pub(crate) struct DecisionClassifier {
    atn: &'static ATN,
    ll1_disjoint: Vec<bool>,
    completion_depths: HashMap<i32, BitSet>,
    /// Configs one build attempt may visit before it is treated as overflow.
    visit_budget: u64,
    /// The operator-loop analysis of each precedence-loop decision, built at
    /// most once and shared by every class attempt.
    loop_blocks: HashMap<i32, LoopBlock>,
    /// The optional-postfix shape of each decision, built at most once.
    postfix_shapes: HashMap<i32, Option<PostfixShape>>,
    /// Whether the generated parser can defer to the adaptive simulation. When
    /// it cannot (`--static-dfa-only`), a partial table is kept regardless of
    /// its coverage, because the alternative is no table at all.
    atn_fallback: AtnFallback,
}

impl DecisionClassifier {
    pub(crate) fn new(atn: &'static ATN, ll1_disjoint: Vec<bool>) -> Self {
        Self {
            atn,
            ll1_disjoint,
            completion_depths: HashMap::new(),
            visit_budget: super::closure::DEFAULT_VISIT_BUDGET,
            loop_blocks: HashMap::new(),
            postfix_shapes: HashMap::new(),
            atn_fallback: AtnFallback::Allowed,
        }
    }

    /// The loop-block analysis of one precedence-loop decision, built on first
    /// use and shared by every class attempt of that decision.
    fn loop_block(&mut self, decision_state: ATNStateRef) -> LoopBlock {
        if let Some(cached) = self.loop_blocks.get(&decision_state.get_state_number()) {
            return cached.clone();
        }
        let block = LoopBlock::of(self.atn, decision_state);
        self.loop_blocks
            .insert(decision_state.get_state_number(), block.clone());
        block
    }

    /// The optional-postfix shape of one decision, detected on first use.
    fn postfix_shape(&mut self, decision_state: ATNStateRef) -> Option<PostfixShape> {
        if let Some(cached) = self.postfix_shapes.get(&decision_state.get_state_number()) {
            return *cached;
        }
        let shape = PostfixShape::analyze(self.atn, decision_state);
        self.postfix_shapes
            .insert(decision_state.get_state_number(), shape);
        shape
    }

    /// Sets the per-attempt visit budget, for tests that classify a whole
    /// corpus and only care that no attempt runs away.
    #[cfg(test)]
    pub(crate) fn with_visit_budget(mut self, budget: u64) -> Self {
        self.visit_budget = budget;
        self
    }

    /// Sets whether the generated parser can defer to the adaptive simulation.
    pub(crate) const fn with_atn_fallback(mut self, atn_fallback: AtnFallback) -> Self {
        self.atn_fallback = atn_fallback;
        self
    }

    /// Classifies every decision.
    ///
    /// The first pass keeps the work small by stopping at the first conflict it
    /// does not trust; decisions that end up adaptive are then classified again
    /// without that shortcut, because their report wants the real witness.
    pub(crate) fn classify_all(&mut self) -> Vec<DecisionResult> {
        let decisions = self.atn.decision_to_state.len();
        let mut results: Vec<DecisionResult> = (0..decisions)
            .map(|decision| self.classify(decision, AttemptPolicy::report_pass()))
            .collect();
        for (decision, result) in results.iter_mut().enumerate() {
            let adaptive_fallback = if result.category == Category::LR_PRECEDENCE {
                result
                    .precedence
                    .as_ref()
                    .is_none_or(|precedence| precedence.adaptive_classes > 0)
            } else {
                result.category != Category::LL1 && result.table.is_none()
            };
            if adaptive_fallback {
                *result = self.classify(decision, AttemptPolicy::unbounded());
            }
        }
        results
    }

    /// Classifies one decision under `base` policy.
    fn classify(&mut self, decision: usize, base: AttemptPolicy) -> DecisionResult {
        let state = self.atn.decision_to_state[decision];
        let mut result = DecisionResult::new(decision, state);

        if state.is_nongreedy_decision() == Some(true) {
            result.category = Category::NON_GREEDY;
            return result;
        }
        if self.is_precedence_loop_decision(state) {
            return self.classify_precedence(result, base);
        }
        if self.ll1_disjoint.get(decision).copied() == Some(true) {
            result.category = Category::LL1;
            result.k = 1;
            return result;
        }
        if state.get_transitions().len() >= VIRTUAL_ALT_BASE {
            result.category = Category::OVERFLOW;
            return result;
        }

        let mut outcome = self.attempt(decision, base, None);
        if outcome.overflow {
            // The retry keeps the abort policy: it belongs to the pass, not to
            // one depth bound, so the cheap first pass stays cheap.
            let mut widened = AttemptPolicy::widened();
            widened.abort_on_untrusted_conflict = base.abort_on_untrusted_conflict;
            outcome = self.attempt(decision, widened, None);
        }
        let mut result = self.finalize(outcome);
        if result.table.is_none()
            && !result.predicated_start
            && matches!(
                result.category,
                Category::OVERFLOW | Category::CONTEXT_SENSITIVE | Category::PREDICATED
            )
        {
            for &policy in self.hybrid_policies() {
                let hybrid_attempt = self.attempt(decision, policy, None);
                let hybrid = self.finalize(hybrid_attempt);
                if hybrid.table.is_some() {
                    result = hybrid;
                    break;
                }
            }
        }
        result
    }

    /// Turns what one attempt observed into a decision result: the category,
    /// and the table when the attempt earns one.
    fn finalize(&mut self, outcome: Attempt) -> DecisionResult {
        let decision = outcome.decision;
        let mut result = DecisionResult::new(decision, self.atn.decision_to_state[decision]);
        apply_facts(&mut result, &outcome);

        if outcome.overflow {
            result.category = Category::OVERFLOW;
            result.num_dfa_states = outcome.state_count;
            return result;
        }

        if outcome.predicated || (outcome.facts.saw_predicate && !outcome.hybrid) {
            result.predicated_start = outcome.predicated;
            // A predicate in front of the decision, or anywhere in a
            // construction that cannot escape, makes viability depend on the
            // parse state, so no table can decide it.
            result.category = Category::PREDICATED;
            result.num_dfa_states = outcome.state_count;
            return result;
        }

        let Some(table) = outcome.table else {
            result.category = if outcome.facts.saw_predicate {
                Category::PREDICATED
            } else if !outcome.conflicts.is_empty() {
                Category::CONTEXT_SENSITIVE
            } else {
                Category::OVERFLOW
            };
            return result;
        };

        result.num_dfa_states = table.states.len();
        result.cyclic = table.cyclic;
        result.k = table.max_k;
        result.escapes = table.escapes();
        result.coverage_percent = (table.start_coverage() * 100.0) as u32;
        result.conflict_witness = table
            .first_escape(CONFLICT_ESCAPE)
            .or_else(|| {
                outcome.first_conflict_state.and_then(|state| {
                    usize::try_from(state)
                        .ok()
                        .filter(|state| *state < table.states.len())
                })
            })
            .and_then(|state| table.witness(state));
        if let Some(state) = table.first_escape(ESCAPE) {
            result.budget_witness = table.witness(state);
        }

        if outcome.taint_conflict {
            // Two paths disagreed on the taint of one config set, so no accept
            // derived from it is trustworthy.
            result.category = Category::CONTEXT_SENSITIVE;
            return result;
        }
        if result.escapes > 0 || table.guarded() > 0 {
            result.category = Category::HYBRID;
            // A table whose start state escapes answers nothing at all. Below
            // the coverage threshold, the adaptive simulation decides almost
            // every lookahead anyway, unless there is no adaptive simulation.
            let start_decides = table.states.first().is_some_and(|state| !state.is_escape());
            let covers = result.coverage_percent >= HYBRID_MIN_COVERAGE_PERCENT
                || self.atn_fallback == AtnFallback::Forbidden;
            if start_decides && covers {
                result.table = Some(table);
            }
            return result;
        }

        result.category =
            if !outcome.context_sensitive.is_empty() || !outcome.approximations.is_empty() {
                Category::CONTEXT_SENSITIVE
            } else if !outcome.exact_ambiguities.is_empty() {
                Category::EXACT_AMBIG
            } else if table.cyclic {
                Category::LLSTAR
            } else {
                Category::LLK
            };
        // Only a table that decides every reachable lookahead is worth
        // emitting: a conflict-resolving state would have to escape anyway.
        if matches!(
            result.category,
            Category::LLK | Category::LLSTAR | Category::EXACT_AMBIG
        ) {
            result.table = Some(table);
        }
        result
    }

    /// The hybrid attempt policies, in the order to try them.
    ///
    /// Without the adaptive fallback, every frontier state of a hybrid table
    /// is a guess, so a larger budget comes first. The default budget stays as
    /// the second choice, for a decision whose larger construction overflows.
    fn hybrid_policies(&self) -> &'static [AttemptPolicy] {
        const ALLOWED: &[AttemptPolicy] = &[AttemptPolicy::hybrid()];
        const FORBIDDEN: &[AttemptPolicy] =
            &[AttemptPolicy::static_only_hybrid(), AttemptPolicy::hybrid()];
        match self.atn_fallback {
            AtnFallback::Allowed => ALLOWED,
            AtnFallback::Forbidden => FORBIDDEN,
        }
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

    fn classify_precedence(
        &mut self,
        mut result: DecisionResult,
        base: AttemptPolicy,
    ) -> DecisionResult {
        result.category = Category::LR_PRECEDENCE;
        let state = self.atn.decision_to_state[result.decision];
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
            return result;
        }

        let cutoffs: Vec<i32> = cutoffs.into_iter().collect();
        let mut representatives = Vec::with_capacity(cutoffs.len() + 1);
        representatives.push(0);
        for cutoff in &cutoffs {
            representatives.push(cutoff.saturating_add(1));
        }

        let mut tables: Vec<Option<StaticDfaTable>> = vec![None; representatives.len()];
        let mut any_table = false;
        let mut max_k = 0_i32;
        let mut total_states = 0_usize;
        let mut adaptive_classes = 0_usize;
        let mut escapes = 0_usize;

        for class_index in 0..representatives.len() {
            let class = PrecedenceClass {
                rule_index: target_rule,
                eval_value: representatives[class_index],
                lo: representatives[class_index],
                hi: cutoffs.get(class_index).copied().unwrap_or(i32::MAX),
            };
            let mut policy = AttemptPolicy::unbounded();
            policy.trust_exact_ambiguity = false;
            policy.abort_on_untrusted_conflict = base.abort_on_untrusted_conflict;
            let attempt = self.attempt(result.decision, policy, Some(class));
            let mut outcome = self.finalize(attempt);
            if outcome.category == Category::OVERFLOW {
                let mut widened = AttemptPolicy::widened();
                widened.trust_exact_ambiguity = false;
                widened.abort_on_untrusted_conflict = base.abort_on_untrusted_conflict;
                let attempt = self.attempt(result.decision, widened, Some(class));
                outcome = self.finalize(attempt);
            }
            if outcome.table.is_none() {
                for &policy in self.hybrid_policies() {
                    let mut hybrid = policy;
                    hybrid.trust_exact_ambiguity = false;
                    let attempt = self.attempt(result.decision, hybrid, Some(class));
                    let candidate = self.finalize(attempt);
                    if candidate.table.is_some() {
                        outcome = candidate;
                        break;
                    }
                }
            }

            result.used_widening |= outcome.used_widening;
            result.saw_predicate |= outcome.saw_predicate;
            if result.first_predicate.is_none() {
                result.first_predicate = outcome.first_predicate;
            }
            result.saw_precedence_predicate |= outcome.saw_precedence_predicate;
            result.utrust_resolutions += outcome.utrust_resolutions;
            if result.conflict_witness.is_none() {
                result.conflict_witness = outcome.conflict_witness.clone();
            }
            if result.budget_witness.is_none() {
                result.budget_witness = outcome.budget_witness.clone();
            }

            match outcome.table {
                Some(table) => {
                    any_table = true;
                    max_k = max_k.max(table.max_k);
                    total_states += table.states.len();
                    escapes += table.escapes();
                    tables[class_index] = Some(table);
                }
                None => {
                    adaptive_classes += 1;
                }
            }
        }

        result.precedence = Some(PrecedenceTables {
            cutoffs,
            tables,
            adaptive_classes,
            escapes,
        });
        if any_table {
            result.k = max_k;
            result.num_dfa_states = total_states;
        }
        result.escapes = escapes;
        result
    }

    /// One build attempt: fresh simulator, fresh arena, fresh scratch arena.
    ///
    /// A fresh manager per attempt is deliberate. The manager owns the vector
    /// of DFAs, and a DFA keeps the states interned so far; the next attempt
    /// (a different widening bound, or a hybrid bound) must start with an
    /// empty state set, or it would reuse verdicts computed under the previous
    /// policy.
    fn attempt(
        &mut self,
        decision: usize,
        policy: AttemptPolicy,
        precedence: Option<PrecedenceClass>,
    ) -> Attempt {
        let atn = self.atn;
        let decision_state = atn.decision_to_state[decision];
        let loop_block = precedence.map(|_| self.loop_block(decision_state));
        let postfix = self.postfix_shape(decision_state);
        let mut completion_depths = std::mem::take(&mut self.completion_depths);
        let attempt = Arena::with(|arena| {
            let manager = ParserATNSimulatorManager::new(atn);
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
                    &scratch,
                    policy.depth_bound,
                    precedence,
                    loop_block,
                    postfix,
                    self.visit_budget,
                ),
                policy,
                allow_continuations: policy.hybrid && precedence.is_none(),
                precedence_class: precedence.is_some(),
                postfix,
                own_roots: HashMap::new(),
                guard_candidates: BTreeMap::new(),
                guard_data: None,
                decision_state,
                decision,
                info: HashMap::new(),
                states: Vec::new(),
                queue: VecDeque::new(),
                completion_depths: &mut completion_depths,
                attempt: Attempt {
                    decision,
                    hybrid: policy.hybrid,
                    has_postfix_shape: postfix.is_some(),
                    ..Attempt::default()
                },
            };
            build.run()
        });
        self.completion_depths = completion_depths;
        attempt
    }
}

/// Merges the facts of one attempt into the decision result.
fn apply_facts(result: &mut DecisionResult, attempt: &Attempt) {
    result.used_widening |= attempt.facts.used_widening;
    result.saw_predicate |= attempt.facts.saw_predicate;
    result.saw_precedence_predicate |= attempt.facts.saw_precedence_predicate;
    if result.first_predicate.is_none() {
        result.first_predicate = attempt.facts.first_predicate;
    }
    result.utrust_resolutions += attempt.utrust_resolutions;
    result.has_postfix_shape |= attempt.has_postfix_shape;
    result.take_rule_fires += attempt.take_rule_fires;
    result.guarded_takes += attempt.guarded_takes;
    for conflict in &attempt.conflicts {
        if !result.conflicts.contains(conflict) {
            result.conflicts.push(conflict.clone());
        }
    }
    for conflict in &attempt.approximations {
        let alts = conflict.iter().map(|alt| alt as i32).collect::<Vec<_>>();
        if !result.approximations.contains(&alts) {
            result.approximations.push(alts);
        }
    }
}

/// One decision build, alive for one attempt.
struct DecisionBuild<'b, 'sim, 'a> {
    atn: &'static ATN,
    simulator: &'b ParserATNSimulator<'sim>,
    dfa: &'sim DFA<'sim, ATNConfigSet<'sim>>,
    scratch: &'a Bump,
    closure: Closure<'static, 'a>,
    policy: AttemptPolicy,
    /// Whether frontier escape states may carry a continuation. Precedence
    /// classes never do: their tables are selected by the parser's precedence,
    /// which the resumed SLL simulation cannot reproduce from a config set.
    allow_continuations: bool,
    /// `true` when this attempt builds one precedence class of a precedence
    /// loop decision.
    precedence_class: bool,
    /// The decision's optional-postfix shape, when it has one.
    postfix: Option<PostfixShape>,
    /// Guard roots entering each state, keyed by DFA state number.
    own_roots: HashMap<i32, BTreeSet<i32>>,
    /// Take-vetoed conflict states that may resolve as a guarded take, with
    /// their conflicting alternatives (`None` for a budget escape that was
    /// never conflict-analyzed). A sorted map keeps the resolution order
    /// deterministic.
    guard_candidates: BTreeMap<i32, Option<BitSet>>,
    /// The decision's guard danger and pass sets, once a guarded take resolved.
    guard_data: Option<(Vec<i32>, Vec<i32>)>,
    decision: usize,
    decision_state: ATNStateRef,
    info: HashMap<i32, StateFacts>,
    /// DFA states in creation order; the index is the state number.
    states: Vec<&'sim DFAState<'sim, ATNConfigSet<'sim>>>,
    queue: VecDeque<i32>,
    completion_depths: &'a mut HashMap<i32, BitSet>,
    attempt: Attempt,
}

impl<'b, 'sim, 'a> DecisionBuild<'b, 'sim, 'a>
where
    'sim: 'a,
{
    fn run(&mut self) -> Attempt {
        let start = self.closure.start(self.decision_state);
        if self.closure.exhausted {
            self.attempt.overflow = true;
            return self.take_attempt();
        }
        let Some(start_state) = self.intern(start, 0) else {
            self.attempt.overflow = true;
            return self.take_attempt();
        };
        self.dfa.set_s0(start_state);
        if self.attempt.overflow || self.closure.exhausted || self.attempt.predicated {
            return self.take_attempt();
        }

        while let Some(number) = self.queue.pop_front() {
            let Some(facts) = self.info.get(&number).cloned() else {
                continue;
            };
            if facts.verdict != Verdict::Decide {
                continue;
            }
            // The hybrid frontier is a *creation* budget: a state that was
            // already expanded keeps its successors, even though re-expanding
            // it after a taint increase may now run past the cap.
            if !facts.expanded && self.policy.hybrid && self.at_hybrid_frontier(facts.depth) {
                // A budget frontier that sits on the iterate/exit pair of a
                // precedence loop stays decidable by substitution. No conflict
                // materialized before the cutoff. The attestations are still a
                // lineage property, independent of any collision. So the
                // argument below applies, and iterate is the answer the
                // runtime would reach.
                if !(self.precedence_class && self.resolve_frontier_iterate(number, &facts)) {
                    self.set_escape(number, ESCAPE);
                    if self.postfix.is_some() && self.state_has_take_skip_pair(number) {
                        // A budget escape of a take/skip state: the guarded
                        // take can still resolve it (eligibility is evaluated
                        // once the roots have propagated).
                        self.guard_candidates.insert(number, None);
                    }
                }
                continue;
            }
            if let Some(facts) = self.info.get_mut(&number) {
                facts.expanded = true;
            }

            let state = self.states[number as usize];
            let closed = ClosedSet {
                configs: state.configs().get_items().cloned().collect(),
                taint: facts.taint.clone(),
            };
            let successors = self.closure.successors(&closed);
            if self.closure.exhausted {
                self.attempt.overflow = true;
                break;
            }
            for (label, successor, roots) in successors {
                let Some(target) = self.intern(successor, facts.depth + 1) else {
                    self.attempt.overflow = true;
                    break;
                };
                if !roots.is_empty() {
                    self.own_roots
                        .entry(target.state_number())
                        .or_default()
                        .extend(roots);
                }
                for interval in label.as_interval_set().as_slice() {
                    for token in interval.a..=interval.b {
                        if token < EOF || token > self.atn.max_token_type {
                            continue;
                        }
                        self.dfa.set_edge(state, (token + 1) as usize, target);
                    }
                }
            }
            if self.attempt.overflow {
                break;
            }
        }

        if !self.attempt.overflow {
            self.finalize_guarded_takes();
            self.attach_continuations();
            self.attempt.table = Some(self.snapshot());
        }
        self.take_attempt()
    }

    /// Whether a hybrid construction stops expanding a state at `depth`.
    ///
    /// A state cap above [`HYBRID_STATE_CAP`] leaves room for one more
    /// expansion, which adds at most one state per token type and EOF. The
    /// cap applies from depth 2, so the expansions at depth 0 and 1 can still
    /// overflow [`MAX_DFA_STATES`], as can the closure visit budget. In these
    /// cases, the classifier retries with the default budget.
    fn at_hybrid_frontier(&self, depth: usize) -> bool {
        let alphabet = usize::try_from(self.atn.max_token_type).unwrap_or(0) + 2;
        let headroom = MAX_DFA_STATES.saturating_sub(alphabet);
        let state_cap = self
            .policy
            .hybrid_state_cap
            .min(headroom.max(HYBRID_STATE_CAP));
        depth >= self.policy.hybrid_depth_cap || (self.states.len() >= state_cap && depth >= 2)
    }

    /// Moves the attempt out of the build, recording what it observed.
    fn take_attempt(&mut self) -> Attempt {
        self.attempt.decision = self.decision;
        self.attempt.state_count = self.states.len();
        self.attempt.facts = std::mem::take(&mut self.closure.facts);
        std::mem::take(&mut self.attempt)
    }

    /// Second chance for take-vetoed conflicts of an optional-postfix decision.
    ///
    /// A vetoed conflict means some skip configuration left the dangling-else
    /// discipline: it consumed tokens outside the decision's own take path. Such
    /// a configuration is realizable at run time only on stacks whose
    /// epsilon-pop chase from the block end reaches the guard root its first
    /// out-of-discipline consumption passed through. So the conflict resolves to
    /// take under a runtime stack test: defer to the adaptive simulation when the
    /// chase can reach a root.
    ///
    /// The eligibility rules are conservative. A widening-born skip
    /// configuration is an analysis artifact unless it is rooted, a clean
    /// out-of-discipline skip has no root for the guard to catch, a root that the
    /// block end reaches without popping a frame makes the guard fire on every
    /// stack, and a block end that cannot pop out of its rule means no root is
    /// ever reachable (that case resolves to plain take accepts instead). The
    /// attempt bookkeeping is applied only once every bail has passed.
    fn finalize_guarded_takes(&mut self) {
        let Some(postfix) = self.postfix else {
            return;
        };
        if self.guard_candidates.is_empty() {
            return;
        }
        let through = self.propagated_guard_roots();
        let candidates = std::mem::take(&mut self.guard_candidates);
        let mut accepted: Vec<(i32, Option<BitSet>, BTreeSet<i32>)> = Vec::new();
        let mut roots_all: BTreeSet<i32> = BTreeSet::new();
        for (state, conflicting) in candidates {
            let Some(facts) = self.info.get(&state) else {
                continue;
            };
            if !matches!(
                facts.verdict,
                Verdict::Escape(ESCAPE) | Verdict::Escape(CONFLICT_ESCAPE)
            ) {
                continue;
            }
            let Some(roots) = through.get(&state).filter(|roots| !roots.is_empty()) else {
                continue;
            };
            let mut eligible = true;
            let mut real_take = false;
            for config in self.states[state as usize].configs().get_items() {
                let taint = taint_of(&facts.taint, config);
                if config.get_alt() == POSTFIX_TAKE_ALT
                    && taint & (WIDENED_TAINT | FOREIGN_CONSUME_TAINT) == 0
                {
                    real_take = true;
                }
                if config.get_alt() != POSTFIX_SKIP_ALT {
                    continue;
                }
                if taint & FOREIGN_CONSUME_TAINT != 0 {
                    eligible = false;
                    break;
                }
                // A widening-born skip is an analysis artifact unless it is
                // rooted: then its bottom frame is a real follow state and the
                // guard covers its realizability.
                if taint & (WIDENED_TAINT | ROOTED_TAINT) == WIDENED_TAINT {
                    eligible = false;
                    break;
                }
                if config.get_state().get_rule_index() == self.decision_state.get_rule_index() {
                    continue;
                }
                if taint & ROOTED_TAINT != 0 {
                    continue;
                }
                if config.get_context().is_some_and(|context| {
                    bottom_frame_is(context, postfix.block_end_state, &mut HashSet::new())
                }) {
                    continue;
                }
                eligible = false;
                break;
            }
            if !eligible || !real_take {
                continue;
            }
            roots_all.extend(roots.iter().copied());
            accepted.push((state, conflicting, roots.clone()));
        }
        if accepted.is_empty() {
            return;
        }

        // A root that the block end reaches without popping any frame would trip
        // the guard on every stack: useless.
        let reachable_from_block = reach_eps(postfix.block_end_state);
        if roots_all
            .iter()
            .any(|root| reachable_from_block.contains(*root as usize))
        {
            return;
        }
        let chase_leaves_the_rule = {
            let rule_stop =
                self.atn.rule_to_stop_state[self.decision_state.get_rule_index() as usize];
            rule_stop.get_state_number() >= 0
                && reachable_from_block.contains(rule_stop.get_state_number() as usize)
        };
        let guard_data = chase_leaves_the_rule.then(|| compute_guard_data(self.atn, &roots_all));
        let guarded = guard_data
            .as_ref()
            .is_some_and(|(danger, _)| !danger.is_empty());
        for (state, conflicting, _) in accepted {
            if let Some(chosen) = conflicting {
                self.attempt
                    .approximations
                    .retain(|conflict| conflict != &chosen);
                self.attempt
                    .context_sensitive
                    .retain(|conflict| conflict != &chosen);
                self.attempt.exact_ambiguities.push(chosen);
            }
            if guarded {
                if let Some(facts) = self.info.get_mut(&state) {
                    facts.verdict = Verdict::Escape(GUARDED);
                    facts.guarded_alt = POSTFIX_TAKE_ALT;
                }
                self.attempt.guarded_takes += 1;
            } else {
                // No guard can ever trip: resolve to a plain take accept.
                if let Some(facts) = self.info.get_mut(&state) {
                    facts.verdict = Verdict::Accept(POSTFIX_TAKE_ALT);
                    facts.resolved_conflict = true;
                }
                self.attempt.take_rule_fires += 1;
            }
        }
        if guarded {
            self.guard_data = guard_data;
        }
    }

    /// Propagates the guard roots of every state along the edge graph to a
    /// fixpoint: `through[s]` is the union of the roots of the paths that reach
    /// `s`.
    fn propagated_guard_roots(&self) -> HashMap<i32, BTreeSet<i32>> {
        let mut successors: HashMap<i32, Vec<i32>> = HashMap::new();
        for (from, _, to) in self.dfa.enumerate_edges() {
            let targets = successors.entry(from.state_number()).or_default();
            if !targets.contains(&to.state_number()) {
                targets.push(to.state_number());
            }
        }
        let mut through: HashMap<i32, BTreeSet<i32>> = self.own_roots.clone();
        let mut queue: VecDeque<i32> = self.own_roots.keys().copied().collect();
        while let Some(source) = queue.pop_front() {
            let roots = through.get(&source).cloned().unwrap_or_default();
            let Some(targets) = successors.get(&source) else {
                continue;
            };
            for &target in targets {
                let entry = through.entry(target).or_default();
                let before = entry.len();
                entry.extend(roots.iter().copied());
                if entry.len() != before {
                    queue.push_back(target);
                }
            }
        }
        through
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
        for number in 0..self.states.len() as i32 {
            let Some(facts) = self.info.get(&number) else {
                continue;
            };
            if facts.depth == 0 || facts.verdict != Verdict::Escape(ESCAPE) {
                continue;
            }
            let configs = self.states[number as usize].configs();
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
    ) -> Option<&'sim DFAState<'sim, ATNConfigSet<'sim>>> {
        let closed = self.closure.canonical_set(closed);
        let (facts, effects) = self.classify(&closed, depth);
        let interned = self.closure.merged_config_set(&closed);
        let mut proposed = ProposedDFAState::new(interned);
        proposed.is_accept_state = matches!(facts.verdict, Verdict::Accept(_));
        proposed.prediction = facts.accept_value();
        let state = self.dfa.add_state(proposed, self.simulator);
        let number = state.state_number();
        let newly_interned = !self.info.contains_key(&number);
        if number as usize >= self.states.len() {
            self.states.push(state);
        }
        if !newly_interned {
            // The same config set is reachable through paths whose taint
            // differs, and a DFA state's verdict is decided once. Widening,
            // precedence-guard or boundary taint on *any* path invalidates a
            // verdict that trusted the config set, so the attempt gives up
            // rather than emit a table the runtime could disagree with.
            let state_configs = self.states[number as usize].configs();
            let mut new_trust_taint = 0;
            let mut verdict = Verdict::Decide;
            let mut resolved_conflict = false;
            if let Some(existing) = self.info.get_mut(&number) {
                verdict = existing.verdict;
                resolved_conflict = existing.resolved_conflict;
                for (key, taint) in &facts.taint {
                    let entry = existing.taint.entry(*key).or_insert(0);
                    new_trust_taint |= *taint & !*entry & TRUST_TAINT;
                    *entry |= *taint;
                }
                // The fallback is a property of the whole set's taint, so a
                // later boundary path can add one.
                existing.fallback = fallback_of(state_configs, &existing.taint);
                // A predicate on any path makes a later escape of the state a
                // syntax error under `--static-dfa-only`, not a guess.
                existing.predicated |= facts.predicated;
            }
            if new_trust_taint != 0 {
                self.on_taint_increase(number, verdict, resolved_conflict, new_trust_taint);
            }
            return Some(state);
        }
        if number as usize >= MAX_DFA_STATES {
            self.attempt.overflow = true;
            return None;
        }
        if effects.conflict.is_some() {
            self.attempt.first_conflict_state.get_or_insert(number);
        }
        self.commit(number, effects);
        let decide = facts.verdict == Verdict::Decide;
        self.info.insert(number, facts);
        if decide && !self.attempt.overflow {
            self.queue.push_back(number);
        }
        Some(state)
    }

    /// Whether the interned state's live alternatives are exactly the
    /// take/skip pair of an optional-postfix decision.
    fn state_has_take_skip_pair(&self, number: i32) -> bool {
        let mut alts = BitSet::new();
        for config in self.states[number as usize].configs().get_items() {
            alts.insert(config.get_alt() as usize);
        }
        is_take_skip_pair(&alts)
    }

    /// Resolves a budget-frontier state of a precedence class to iterate when
    /// the iterate/exit substitution attestations permit it.
    ///
    /// Returns `false` when the state is not the loop's iterate/exit pair, when
    /// the loop's guarded alternatives are not simple, or when the attestation
    /// is anything but a clean iterate reading - the caller then escapes the
    /// state as usual.
    fn resolve_frontier_iterate(&mut self, number: i32, facts: &StateFacts) -> bool {
        let configs: Vec<dbt_antlr_runtime::analysis::ATNConfig<'_>> = self.states[number as usize]
            .configs()
            .get_items()
            .cloned()
            .collect();
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
        if !block.guards_are_simple()
            || block.iterate_exit_attestation(&configs, &facts.taint) != ITERATE_REAL
        {
            return false;
        }
        if let Some(facts) = self.info.get_mut(&number) {
            facts.verdict = Verdict::Accept(ITERATE_ALT);
            facts.resolved_conflict = true;
        }
        self.attempt.exact_ambiguities.push(alts.iter().collect());
        true
    }

    /// Reacts to a second path adding trust taint to an already interned
    /// state.
    ///
    /// The state's verdict was decided once, from the first path's taint, and a
    /// state cannot be both trusted and untrusted: a table cannot carry a
    /// verdict that any path contradicts. A trusted accept therefore becomes an
    /// escape (hybrid) or invalidates the whole attempt (a non-hybrid table has
    /// no escapes), while a decision state is queued again so its successors
    /// see the union of both paths' taint.
    fn on_taint_increase(
        &mut self,
        number: i32,
        verdict: Verdict,
        resolved_conflict: bool,
        added: i32,
    ) {
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
                if self.policy.hybrid {
                    let approximated = added & (WIDENED_TAINT | PRECPRED_TAINT) != 0;
                    if let Some(facts) = self.info.get_mut(&number) {
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
                            facts.conflict_min_alt =
                                facts.conflicts.iter().min().copied().unwrap_or(0);
                        }
                    }
                } else {
                    self.attempt.taint_conflict = true;
                }
            }
            Verdict::Accept(_) | Verdict::Escape(_) => {}
            Verdict::Decide => self.queue.push_back(number),
        }
    }

    /// Records the attempt-level bookkeeping of a newly interned state.
    fn commit(&mut self, number: i32, effects: ClassifyEffects) {
        if let Some(candidate) = effects.guard_candidate {
            self.guard_candidates.insert(number, candidate);
        }
        if effects.utrust {
            self.attempt.utrust_resolutions += 1;
        }
        if let Some(alts) = effects.exact_ambiguity {
            self.attempt.exact_ambiguities.push(alts);
        }
        if let Some(alts) = effects.approximation {
            self.attempt.approximations.push(alts);
        }
        if let Some(alts) = effects.context_sensitive {
            self.attempt.context_sensitive.push(alts);
        }
        if let Some(alts) = effects.conflict {
            self.attempt.conflicts.push(alts);
        }
        if effects.fail_predicated {
            self.attempt.predicated = true;
        }
        if effects.abort {
            self.attempt.stopped = true;
            self.attempt.overflow = true;
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
            continuation: None,
            predicated: false,
        };
        if self.policy.hybrid && predicated {
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
        if let Some(block) = self.closure.loop_block()
            && block.guards_are_simple()
            && is_iterate_exit_pair(&conflicting)
            && block.iterate_exit_attestation(&closed.configs, &closed.taint) == ITERATE_REAL
        {
            facts.resolved_conflict = true;
            effects.exact_ambiguity = Some(conflicting.clone());
            facts.verdict = Verdict::Accept(ITERATE_ALT);
            return (facts, effects);
        }

        // Optional-postfix take rule: the take/skip conflict of an `X Y?`
        // decision is a dangling-else shape. Skip is never uniquely viable
        // when every token it consumes is mirrored into the take structure.
        // Three facts attest that:
        //
        // - no skip configuration consumed outside the decision rule,
        // - every descended skip configuration bottoms out at the decision's
        //   block end (it re-entered through this decision's own take path),
        // - a clean take configuration attests take viability.
        //
        // Take's continuation after the optional is then the same block end as
        // skip's, and the mirrored re-descents consume the same tokens. So the
        // runtime answers take whenever it terminates: unique take, or an
        // ambiguity resolved to the minimum alternative.
        if let Some(postfix) = self.postfix
            && is_take_skip_pair(&conflicting)
        {
            let mut skip_foreign = false;
            let mut mirror_ok = true;
            let mut real_take = false;
            for config in &closed.configs {
                let taint = closed.taint_of(config);
                if config.get_alt() == POSTFIX_SKIP_ALT {
                    if taint & FOREIGN_CONSUME_TAINT != 0 {
                        skip_foreign = true;
                    }
                    if config.get_state().get_rule_index() != self.decision_state.get_rule_index()
                        && !bottom_frame_is(
                            config.get_context().expect("config has a context"),
                            postfix.block_end_state,
                            &mut HashSet::new(),
                        )
                    {
                        mirror_ok = false;
                    }
                }
                if config.get_alt() == POSTFIX_TAKE_ALT
                    && taint & (WIDENED_TAINT | FOREIGN_CONSUME_TAINT) == 0
                {
                    real_take = true;
                }
            }
            if mirror_ok && !skip_foreign && real_take {
                self.attempt.take_rule_fires += 1;
                facts.resolved_conflict = true;
                effects.exact_ambiguity = Some(conflicting.clone());
                facts.verdict = Verdict::Accept(POSTFIX_TAKE_ALT);
                return (facts, effects);
            }
            if self.policy.hybrid && !skip_foreign && real_take {
                // Vetoed only by the mirror check: a candidate for the guarded
                // take, resolved after construction once the guard roots have
                // propagated.
                effects.guard_candidate = Some(Some(conflicting.clone()));
            }
        }

        let untrusted = precedence_tainted
            || widened_tainted
            || (any_boundary && !all_boundary)
            || !self.uniform_pop_depths(&conflicting, any_boundary, all_boundary)
            || !self.policy.trust_exact_ambiguity;

        if !untrusted && exact {
            facts.resolved_conflict = true;
            effects.exact_ambiguity = Some(conflicting.clone());
            facts.verdict = Verdict::Accept(conflicting.iter().next().map_or(0, |alt| alt as i32));
            return (facts, effects);
        }

        if depth > 0 {
            let min_alt = self.utrust_min_alt(closed, &conflicting, any_boundary, all_boundary);
            if min_alt > 0 {
                facts.resolved_conflict = true;
                effects.utrust = true;
                effects.exact_ambiguity = Some(conflicting);
                facts.verdict = Verdict::Accept(min_alt);
                return (facts, effects);
            }
        }
        if untrusted {
            effects.approximation = Some(conflicting.clone());
        } else {
            effects.context_sensitive = Some(conflicting.clone());
        }
        effects.conflict = Some(facts.conflicts.clone());

        if self.policy.hybrid {
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
        } else if self.policy.abort_on_untrusted_conflict {
            effects.abort = true;
        }
        (facts, effects)
    }

    fn set_escape(&mut self, number: i32, marker: i32) {
        if let Some(facts) = self.info.get_mut(&number) {
            if matches!(facts.verdict, Verdict::Escape(_)) {
                return;
            }
            facts.verdict = Verdict::Escape(marker);
        }
    }

    fn snapshot(&self) -> StaticDfaTable {
        let mut states: Vec<StaticDfaState> = self
            .states
            .iter()
            .map(|state| {
                let number = state.state_number();
                let facts = self.info.get(&number);
                let escape = facts.is_some_and(|facts| matches!(facts.verdict, Verdict::Escape(_)));
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
                StaticDfaState {
                    accept: facts.map_or(0, StateFacts::accept_value),
                    fallback: facts.map_or(0, |facts| facts.fallback),
                    depth: facts.map_or(0, |facts| facts.depth),
                    conflicts: facts
                        .map(|facts| facts.conflicts.clone())
                        .unwrap_or_default(),
                    conflict_min_alt: facts.map_or(0, |facts| facts.conflict_min_alt),
                    guarded_alt: facts.map_or(0, |facts| facts.guarded_alt),
                    continuation: facts.and_then(|facts| facts.continuation.clone()),
                    viable_alts,
                    predicated: facts.is_some_and(|facts| facts.predicated),
                    resolution: None,
                    edges: Vec::new(),
                }
            })
            .collect();

        let mut per_state: Vec<Vec<(i32, usize)>> = vec![Vec::new(); states.len()];
        for (from, symbol, to) in self.dfa.enumerate_edges() {
            let from = from.state_number() as usize;
            let to = to.state_number() as usize;
            if from < per_state.len() && to < states.len() {
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
        if let Some((danger, pass)) = &self.guard_data {
            table.guard_danger.clone_from(danger);
            table.guard_pass.clone_from(pass);
        }
        table
    }

    /// `true` when the conflicting alternatives leave the decision after the
    /// same number of tokens: the ambiguity is then faithful to the runtime.
    fn uniform_pop_depths(
        &mut self,
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
        if self.policy.depth_bound == 0 {
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

    fn completion_depths(&mut self, alt_start: ATNStateRef) -> BitSet {
        let state_number = alt_start.get_state_number();
        if let Some(cached) = self.completion_depths.get(&state_number) {
            return cached.clone();
        }
        let mut visiting = BTreeSet::new();
        let mut budget = 100_000_i32;
        let depths = token_counts_to_stop(alt_start, &mut visiting, &mut budget);
        self.completion_depths.insert(state_number, depths.clone());
        depths
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

/// The lowest alternative that already left the decision, over the union of
/// every path's taint.
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

/// The wire size of a serialized continuation, in `u32` words.
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
