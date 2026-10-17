// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Compile-time static parser SLL DFA classifier and builder (feature `static-dfa-build`).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::{self, Display, Formatter, Write as _};
use std::hash::{Hash, Hasher};

pub use bit_set::BitSet;
use fxhash::FxHasher64;

use crate::atn::ATN;
use crate::atn_config::ATNConfig;
use crate::atn_config_set::MutableATNConfigSet;
use crate::atn_state::{ATNStateRef, ATNStateType, RuleStartState};
use crate::int_stream::EOF;
use crate::interval_set::IntervalSetBuf;
use crate::parser_atn_simulator::MergeCache;
use crate::prediction_context::{PredictionContext, PredictionContextRef};
use crate::prediction_mode::{
    all_subsets_conflict, all_subsets_equal, get_alts, get_conflicting_alt_subsets,
    has_sll_conflict_terminating_prediction, PredictionMode,
};
use crate::token::TOKEN_MIN_USER_TOKEN_TYPE;
use crate::transition::{
    NotSetTransition, PrecedencePredicateTransition, PredicateTransition, RuleTransition,
    TransitionType,
};

pub const ESCAPE: i32 = -1;
pub const CONFLICT_ESCAPE: i32 = -2;
pub const GUARDED: i32 = -3;
pub const VIRTUAL_ALT_BASE: i32 = 65536;

pub const BOUNDARY_TAINT: i32 = 1;
pub const PRECPRED_TAINT: i32 = 2;
pub const WIDENED_TAINT: i32 = 4;
pub const FOREIGN_CONSUME_TAINT: i32 = 8;
pub const ROOTED_TAINT: i32 = 16;
pub const PRED_TAINT: i32 = 32;
pub const HARD_TAINT: i32 = WIDENED_TAINT | PRECPRED_TAINT;

pub const DEFAULT_MAX_DFA_STATES: usize = 1000;
pub const DEFAULT_HYBRID_DEPTH_CAP: usize = 4;
pub const DEFAULT_HYBRID_STATE_CAP: usize = 64;
pub const DEFAULT_HYBRID_MIN_COVERAGE: f64 = 0.1;

const MAX_TRACKED_DEPTH: usize = 30;
const VARIABLE_DEPTH: usize = 31;

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Category {
    LL1,
    LLK,
    LLSTAR,
    EXACT_AMBIG,
    CONTEXT_SENSITIVE,
    PREDICATED,
    NON_GREEDY,
    LR_PRECEDENCE,
    OVERFLOW,
    HYBRID,
}

impl Category {
    pub const ALL: [Self; 10] = [
        Self::LL1,
        Self::LLK,
        Self::LLSTAR,
        Self::EXACT_AMBIG,
        Self::CONTEXT_SENSITIVE,
        Self::PREDICATED,
        Self::NON_GREEDY,
        Self::LR_PRECEDENCE,
        Self::OVERFLOW,
        Self::HYBRID,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LL1 => "LL1",
            Self::LLK => "LLK",
            Self::LLSTAR => "LLSTAR",
            Self::EXACT_AMBIG => "EXACT_AMBIG",
            Self::CONTEXT_SENSITIVE => "CONTEXT_SENSITIVE",
            Self::PREDICATED => "PREDICATED",
            Self::NON_GREEDY => "NON_GREEDY",
            Self::LR_PRECEDENCE => "LR_PRECEDENCE",
            Self::OVERFLOW => "OVERFLOW",
            Self::HYBRID => "HYBRID",
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticParserDFA {
    pub decision: usize,
    pub num_states: usize,
    pub accepts: Vec<i32>,
    pub fallbacks: Vec<i32>,
    pub edge_offsets: Vec<usize>,
    pub edges: Vec<i32>,
    pub cyclic: bool,
    pub max_k: i32,
    pub guarded_alts: Vec<i32>,
    pub guard_danger: Vec<i32>,
    pub guard_pass: Vec<i32>,
}

impl StaticParserDFA {
    pub const fn is_virtual_alt(alt: i32) -> bool {
        alt >= VIRTUAL_ALT_BASE
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrecedenceStaticParserDFA {
    pub decision: usize,
    pub cutoffs: Vec<i32>,
    pub tables: Vec<Option<StaticParserDFA>>,
}

#[derive(Clone, Debug)]
pub struct DecisionResult {
    pub decision: usize,
    pub rule_index: usize,
    pub state_number: i32,
    pub num_transitions: usize,
    pub category: Category,
    pub k: i32,
    pub num_dfa_states: usize,
    pub used_widening: bool,
    pub widening_depth: i32,
    pub saw_predicate: bool,
    pub saw_prec_predicate: bool,
    pub first_predicate: Option<(i32, i32)>,
    pub conflict_witness: Option<Vec<i32>>,
    pub budget_witness: Option<Vec<i32>>,
    pub witness_class: Option<String>,
    pub exact_ambig_conflicts: Vec<BitSet>,
    pub context_sensitive_conflicts: Vec<BitSet>,
    pub approx_conflicts: Vec<BitSet>,
    pub dfa: Option<StaticParserDFA>,
    pub prec_dfa: Option<PrecedenceStaticParserDFA>,
    pub prec_note: Option<String>,
    pub escapes: usize,
    pub adaptive_classes: usize,
    pub coverage: f64,
    pub factor_group_states: usize,
    pub factor_escapes_cured: usize,
    pub take_rule_fires: usize,
    pub guarded_takes: usize,
    pub guarded_iterates: usize,
    pub guarded_obligations: usize,
    pub obligation_accepts: usize,
    pub iterate_accepts: usize,
    pub frontier_iterates: usize,
    pub utrust_resolutions: usize,
    pub descent_group_states: usize,
    pub has_postfix_shape: bool,
}

impl DecisionResult {
    fn new(decision_state: ATNStateRef) -> Self {
        let decision = decision_state.get_decision().unwrap_or(0) as usize;
        let rule_index = decision_state.get_rule_index() as usize;
        let state_number = decision_state.get_state_number();
        let num_transitions = decision_state.get_transitions().len();
        Self {
            decision,
            rule_index,
            state_number,
            num_transitions,
            category: Category::LLK,
            k: -1,
            num_dfa_states: 0,
            used_widening: false,
            widening_depth: i32::MAX,
            saw_predicate: false,
            saw_prec_predicate: false,
            first_predicate: None,
            conflict_witness: None,
            budget_witness: None,
            witness_class: None,
            exact_ambig_conflicts: Vec::new(),
            context_sensitive_conflicts: Vec::new(),
            approx_conflicts: Vec::new(),
            dfa: None,
            prec_dfa: None,
            prec_note: None,
            escapes: 0,
            adaptive_classes: 0,
            coverage: 0.0,
            factor_group_states: 0,
            factor_escapes_cured: 0,
            take_rule_fires: 0,
            guarded_takes: 0,
            guarded_iterates: 0,
            guarded_obligations: 0,
            obligation_accepts: 0,
            iterate_accepts: 0,
            frontier_iterates: 0,
            utrust_resolutions: 0,
            descent_group_states: 0,
            has_postfix_shape: false,
        }
    }

    pub fn fb_string(&self) -> String {
        if self.category == Category::LR_PRECEDENCE {
            if self.prec_dfa.is_none() {
                return "all".to_owned();
            }
            if self.escapes == 0 && self.adaptive_classes == 0 {
                return "0".to_owned();
            }
            let mut s = String::new();
            if self.escapes > 0 {
                let _ = write!(s, "{}e", self.escapes);
            }
            if self.adaptive_classes > 0 {
                if !s.is_empty() {
                    s.push('+');
                }
                let _ = write!(s, "{}c", self.adaptive_classes);
            }
            return s;
        }
        if self.category == Category::LL1 {
            return "0".to_owned();
        }
        if self.dfa.is_none() {
            return "all".to_owned();
        }
        if self.escapes > 0 {
            self.escapes.to_string()
        } else {
            "0".to_owned()
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct CallInfo {
    target_rule_index: i32,
    precedence: i32,
}

#[derive(Clone)]
struct CanonicalConfigSet<'a> {
    configs: Vec<ATNConfig<'a>>,
    hash: u64,
}

impl<'a> CanonicalConfigSet<'a> {
    fn new(configs: Vec<ATNConfig<'a>>) -> Self {
        let mut hash = 0u64;
        for c in &configs {
            let mut hasher = FxHasher64::default();
            c.hash(&mut hasher);
            hash = hash.wrapping_add(hasher.finish());
        }
        Self { configs, hash }
    }
}

impl PartialEq for CanonicalConfigSet<'_> {
    fn eq(&self, other: &Self) -> bool {
        if self.hash != other.hash || self.configs.len() != other.configs.len() {
            return false;
        }
        if self.configs.iter().zip(&other.configs).all(|(a, b)| a == b) {
            return true;
        }
        self.configs.iter().all(|c| other.configs.contains(c))
    }
}

impl Eq for CanonicalConfigSet<'_> {}

impl Hash for CanonicalConfigSet<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash);
    }
}

struct OrderedConfigSet<'a> {
    items: Vec<ATNConfig<'a>>,
    seen: HashSet<ATNConfig<'a>>,
}

impl<'a> OrderedConfigSet<'a> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            seen: HashSet::new(),
        }
    }

    fn insert(&mut self, config: ATNConfig<'a>) {
        if self.seen.insert(config.clone()) {
            self.items.push(config);
        }
    }
}

struct LabeledSuccessor<'a> {
    label: IntervalSetBuf,
    configs: CanonicalConfigSet<'a>,
}

#[derive(Debug)]
pub struct DecisionClassifier<'atn> {
    atn: &'atn ATN,
    ll1_disjoint: Vec<bool>,
    all_tokens: IntervalSetBuf,
    pub max_dfa_states: usize,
    pub hybrid_depth_cap: usize,
    pub hybrid_state_cap: usize,
    pub hybrid_min_coverage: f64,
    pub widening_depths: Vec<i32>,
    pub abort_on_untrusted_conflict: bool,
    pub abort_inf_as_overflow: bool,
    current_depth_bound: i32,
    prec_start_closure: bool,
    prec_eval_value: i32,
    prec_class_lo: i32,
    prec_class_hi: i32,
    prec_rule_index: i32,
    trust_exact_ambig: bool,
    hybrid_mode: bool,
    call_by_follow_state: Option<HashMap<i32, CallInfo>>,
    completion_depths_cache: HashMap<i32, BitSet>,
}

impl<'atn> DecisionClassifier<'atn> {
    pub fn new(atn: &'atn ATN, ll1_disjoint: Vec<bool>) -> Self {
        let mut all_tokens = IntervalSetBuf::new();
        if atn.max_token_type >= TOKEN_MIN_USER_TOKEN_TYPE {
            all_tokens.add_range(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type);
        }
        Self {
            atn,
            ll1_disjoint,
            all_tokens,
            max_dfa_states: DEFAULT_MAX_DFA_STATES,
            hybrid_depth_cap: DEFAULT_HYBRID_DEPTH_CAP,
            hybrid_state_cap: DEFAULT_HYBRID_STATE_CAP,
            hybrid_min_coverage: DEFAULT_HYBRID_MIN_COVERAGE,
            widening_depths: vec![i32::MAX, 0],
            abort_on_untrusted_conflict: false,
            abort_inf_as_overflow: false,
            current_depth_bound: i32::MAX,
            prec_start_closure: false,
            prec_eval_value: 0,
            prec_class_lo: 0,
            prec_class_hi: i32::MAX,
            prec_rule_index: -1,
            trust_exact_ambig: true,
            hybrid_mode: false,
            call_by_follow_state: None,
            completion_depths_cache: HashMap::new(),
        }
    }

    pub fn classify_all(&mut self) -> Vec<DecisionResult> {
        let states = self.atn.decision_to_state.clone();
        let mut results = Vec::with_capacity(states.len());
        for s in states {
            results.push(self.classify(s));
        }
        results
    }

    pub fn classify(&mut self, s: ATNStateRef) -> DecisionResult {
        let mut res = DecisionResult::new(s);
        if s.is_nongreedy_decision() == Some(true) {
            res.category = Category::NON_GREEDY;
            return res;
        }
        if self.is_precedence_loop_decision(s) {
            return self.classify_precedence(s);
        }
        if self.ll1_disjoint.get(res.decision).copied() == Some(true) {
            res.category = Category::LL1;
            res.k = 1;
            return res;
        }
        if s.get_transitions().len() >= VIRTUAL_ALT_BASE as usize {
            res.category = Category::OVERFLOW;
            return res;
        }

        let depths = self.widening_depths.clone();
        let mut outcome = res;
        let mut all_overflowed = true;
        for depth_bound in depths {
            let mut attempt = DecisionResult::new(s);
            attempt.widening_depth = depth_bound;
            if !self.build_dfa(s, &mut attempt, depth_bound) {
                outcome = attempt;
                all_overflowed = false;
                break;
            }
            outcome = attempt;
        }
        if all_overflowed {
            outcome.category = Category::OVERFLOW;
        }

        if outcome.dfa.is_none()
            && (outcome.category == Category::OVERFLOW
                || outcome.category == Category::CONTEXT_SENSITIVE)
        {
            if let Some(hybrid) = self.classify_hybrid(s) {
                if hybrid.dfa.is_some() {
                    return hybrid;
                }
            }
        }
        outcome
    }

    fn classify_hybrid(&mut self, s: ATNStateRef) -> Option<DecisionResult> {
        let mut hybrid = DecisionResult::new(s);
        hybrid.widening_depth = i32::MAX;
        self.hybrid_mode = true;
        let overflowed = self.build_dfa(s, &mut hybrid, i32::MAX);
        self.hybrid_mode = false;
        if overflowed {
            None
        } else {
            Some(hybrid)
        }
    }

    pub fn is_precedence_loop_decision(&self, s: ATNStateRef) -> bool {
        if s.state_type() != ATNStateType::StarLoopEntry {
            return false;
        }
        let rule_start = self.atn.rule_to_start_state[s.get_rule_index() as usize];
        let is_lr = rule_start
            .try_as::<RuleStartState>()
            .is_some_and(|rs| rs.is_left_recursive);
        if !is_lr {
            return false;
        }
        let transitions = s.get_transitions();
        let Some(last) = transitions.last() else {
            return false;
        };
        let maybe_loop_end = last.get_target();
        if maybe_loop_end.state_type() != ATNStateType::LoopEnd {
            return false;
        }
        maybe_loop_end.has_epsilon_only_transitions()
            && maybe_loop_end
                .get_transitions()
                .first()
                .is_some_and(|t| t.get_target().state_type() == ATNStateType::RuleStop)
    }

    fn classify_precedence(&mut self, s: ATNStateRef) -> DecisionResult {
        let mut res = DecisionResult::new(s);
        res.category = Category::LR_PRECEDENCE;
        let target_rule = s.get_rule_index();
        let mut cut_set = BTreeSet::new();
        for st in &self.atn.states {
            if st.get_rule_index() != target_rule {
                continue;
            }
            for t in st.get_transitions() {
                if let Some(ppt) = t.try_as::<PrecedencePredicateTransition>() {
                    cut_set.insert(ppt.precedence());
                }
            }
        }
        if cut_set.is_empty() {
            return res;
        }

        let cutoffs: Vec<i32> = cut_set.into_iter().collect();
        let mut reps = Vec::with_capacity(cutoffs.len() + 1);
        reps.push(0);
        for &cut in &cutoffs {
            reps.push(cut + 1);
        }

        self.ensure_call_by_follow_state();

        let mut tables: Vec<Option<StaticParserDFA>> = vec![None; reps.len()];
        let mut note = String::new();
        let mut any_table = false;
        let mut max_k = 0;
        let mut total_states = 0;

        self.prec_rule_index = target_rule;
        self.trust_exact_ambig = false;

        let depths = self.widening_depths.clone();
        for c in 0..reps.len() {
            self.prec_eval_value = reps[c];
            self.prec_class_lo = reps[c];
            self.prec_class_hi = if c < cutoffs.len() {
                cutoffs[c]
            } else {
                i32::MAX
            };
            let mut attempt = DecisionResult::new(s);
            for &depth_bound in &depths {
                let mut a = DecisionResult::new(s);
                a.widening_depth = depth_bound;
                if !self.build_dfa(s, &mut a, depth_bound) {
                    attempt = a;
                    break;
                }
                attempt = a;
            }
            if attempt.dfa.is_none() && !attempt.saw_predicate {
                let mut h = DecisionResult::new(s);
                h.widening_depth = i32::MAX;
                self.hybrid_mode = true;
                let overflowed = self.build_dfa(s, &mut h, i32::MAX);
                self.hybrid_mode = false;
                if !overflowed {
                    attempt = h;
                }
            }
            res.saw_prec_predicate |= attempt.saw_prec_predicate;
            res.used_widening |= attempt.used_widening;
            res.guarded_iterates += attempt.guarded_iterates;
            res.iterate_accepts += attempt.iterate_accepts;
            res.frontier_iterates += attempt.frontier_iterates;
            res.guarded_obligations += attempt.guarded_obligations;
            res.obligation_accepts += attempt.obligation_accepts;

            let class_label = if c == 0 {
                format!("p<={}", cutoffs[0])
            } else if c < cutoffs.len() {
                format!("p={}..{}", cutoffs[c - 1] + 1, cutoffs[c])
            } else {
                format!("p>{}", cutoffs[cutoffs.len() - 1])
            };

            if res.conflict_witness.is_none() && attempt.conflict_witness.is_some() {
                res.conflict_witness = attempt.conflict_witness.clone();
                res.witness_class = Some(class_label.clone());
            }
            if res.budget_witness.is_none() && attempt.budget_witness.is_some() {
                res.budget_witness = attempt.budget_witness.clone();
                if res.conflict_witness.is_none() {
                    res.witness_class = Some(class_label.clone());
                }
            }
            if !note.is_empty() {
                note.push(' ');
            }
            let _ = write!(note, "{class_label}:");
            if let Some(dfa) = attempt.dfa {
                any_table = true;
                max_k = max_k.max(attempt.k);
                total_states += attempt.num_dfa_states;
                if attempt.category == Category::HYBRID {
                    let _ = write!(note, "hyb(cover={:.0}%)", attempt.coverage * 100.0);
                    res.escapes += attempt.escapes;
                } else if dfa.cyclic {
                    note.push_str("LL(*)");
                } else {
                    let _ = write!(note, "k={}", attempt.k);
                }
                tables[c] = Some(dfa);
            } else {
                res.adaptive_classes += 1;
                if attempt.category == Category::HYBRID {
                    let _ = write!(note, "adaptive(cover={:.0}%)", attempt.coverage * 100.0);
                } else {
                    let _ = write!(note, "{}", attempt.category);
                }
                if !attempt.context_sensitive_conflicts.is_empty() {
                    let _ = write!(
                        note,
                        "/ctx{}",
                        format_distinct_bitsets(&attempt.context_sensitive_conflicts)
                    );
                }
                if !attempt.approx_conflicts.is_empty() {
                    let _ = write!(
                        note,
                        "/approx{}",
                        format_distinct_bitsets(&attempt.approx_conflicts)
                    );
                }
                if !attempt.exact_ambig_conflicts.is_empty() {
                    let _ = write!(
                        note,
                        "/exact{}",
                        format_distinct_bitsets(&attempt.exact_ambig_conflicts)
                    );
                }
            }
        }

        self.prec_rule_index = -1;
        self.trust_exact_ambig = true;
        self.prec_start_closure = false;

        res.prec_note = Some(format!("classes={} [{note}]", reps.len()));
        if any_table {
            res.prec_dfa = Some(PrecedenceStaticParserDFA {
                decision: res.decision,
                cutoffs,
                tables,
            });
            res.k = max_k;
            res.num_dfa_states = total_states;
        }
        res
    }

    fn ensure_call_by_follow_state(&mut self) {
        if self.call_by_follow_state.is_some() {
            return;
        }
        let mut map = HashMap::new();
        for st in &self.atn.states {
            for t in st.get_transitions() {
                if let Some(rt) = t.try_as::<RuleTransition>() {
                    map.insert(
                        rt.follow_state.get_state_number(),
                        CallInfo {
                            target_rule_index: rt.get_target().get_rule_index(),
                            precedence: rt.precedence(),
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
            .and_then(|m| m.get(&follow_state_number).copied())
    }

    fn eval_precpred_from_context(
        &mut self,
        ctx: PredictionContextRef<'_>,
        rule: i32,
        n: i32,
    ) -> Option<bool> {
        if ctx.is_empty() {
            return None;
        }
        let mut agreed = None;
        for i in 0..ctx.length() {
            let return_state = ctx.get_return_state(i);
            if return_state == ATNStateRef::invalid() {
                return None;
            }
            let call = self.call_info(return_state.get_state_number())?;
            if call.target_rule_index != rule {
                return None;
            }
            let outcome = n >= call.precedence;
            match agreed {
                None => agreed = Some(outcome),
                Some(prev) if prev != outcome => return None,
                Some(_) => {}
            }
        }
        agreed
    }

    fn apply_precedence_filter<'a>(
        raw: &[ATNConfig<'a>],
        arena: &'a bumpalo::Bump,
        merge_cache: &mut MergeCache<'a>,
    ) -> Vec<ATNConfig<'a>> {
        let mut taint_by_state_alt: HashMap<(i32, i32), i32> = HashMap::new();
        for c in raw {
            let key = (c.get_state().get_state_number(), c.get_alt());
            let taint = c.reaches_into_outer_context
                & (BOUNDARY_TAINT | PRECPRED_TAINT | WIDENED_TAINT | ROOTED_TAINT);
            *taint_by_state_alt.entry(key).or_insert(0) |= taint;
        }

        let merged = canonical_vec(raw, arena, merge_cache);

        let mut states_from_alt1: HashMap<i32, PredictionContextRef<'a>> = HashMap::new();
        for c in &merged {
            if c.get_alt() == 1 {
                states_from_alt1.insert(c.get_state().get_state_number(), c.get_context().unwrap());
            }
        }

        let mut result = OrderedConfigSet::new();
        for c in merged {
            if c.get_alt() != 1 && !c.is_precedence_filter_suppressed() {
                if let Some(&ctx1) = states_from_alt1.get(&c.get_state().get_state_number()) {
                    if ctx1 == c.get_context().unwrap() {
                        continue;
                    }
                }
            }
            let mut kept = c.clone();
            kept.set_precedence_filter_suppressed(false);
            let key = (kept.get_state().get_state_number(), kept.get_alt());
            if let Some(&taint) = taint_by_state_alt.get(&key) {
                kept.reaches_into_outer_context |= taint;
            }
            result.insert(kept);
        }
        result.items
    }

    #[allow(clippy::too_many_lines)]
    fn build_dfa(&mut self, s: ATNStateRef, res: &mut DecisionResult, depth_bound: i32) -> bool {
        self.current_depth_bound = depth_bound;
        let arena = bumpalo::Bump::new();
        let mut merge_cache = MergeCache::new(&arena);
        let mut depth_cache: HashMap<usize, i32> = HashMap::new();
        let mut faith_cache: HashMap<usize, bool> = HashMap::new();
        let mut widened_joints: HashSet<usize> = HashSet::new();

        let mut state_ids: HashMap<CanonicalConfigSet<'_>, usize> = HashMap::new();
        let mut states: Vec<CanonicalConfigSet<'_>> = Vec::new();
        let mut edges: Vec<Vec<usize>> = Vec::new();
        let mut edge_labels: Vec<Vec<IntervalSetBuf>> = Vec::new();
        let mut accept_alts: Vec<i32> = Vec::new();
        let mut fallback_alts: Vec<i32> = Vec::new();
        let mut state_depth: Vec<usize> = Vec::new();

        let mut first_conflict_esc: Option<usize> = None;
        let mut first_budget_esc: Option<usize> = None;

        let mut start = OrderedConfigSet::new();
        let mut start_busy: HashSet<(ATNConfig<'_>, bool)> = HashSet::with_capacity(1024);
        self.prec_start_closure = self.prec_rule_index >= 0;
        for (i, t) in s.get_transitions().iter().enumerate() {
            let cfg = ATNConfig::new(
                t.get_target(),
                (i + 1) as i32,
                Some(PredictionContextRef::new_empty()),
            );
            self.closure(
                cfg,
                &mut start,
                &mut start_busy,
                res,
                0,
                &arena,
                &mut depth_cache,
                &mut widened_joints,
            );
        }
        self.prec_start_closure = false;

        let start_items = if self.prec_rule_index >= 0 {
            Self::apply_precedence_filter(&start.items, &arena, &mut merge_cache)
        } else {
            start.items
        };
        let start_canonical =
            CanonicalConfigSet::new(canonical_vec(&start_items, &arena, &mut merge_cache));

        state_ids.insert(start_canonical.clone(), 0);
        states.push(start_canonical);
        edges.push(Vec::new());
        edge_labels.push(Vec::new());
        accept_alts.push(0);
        fallback_alts.push(0);
        state_depth.push(0);

        let mut work = VecDeque::new();
        work.push_back(0usize);
        let mut overflow = false;

        while let Some(d) = work.pop_front() {
            if overflow {
                break;
            }
            let configs = states[d].configs.clone();
            let cs = to_config_set(&configs, &arena, &mut merge_cache);

            let mut alts = BitSet::new();
            for c in cs.get_items() {
                alts.insert(c.get_alt() as usize);
            }
            if alts.len() <= 1 {
                accept_alts[d] = alts.iter().next().map_or(0, |a| a as i32);
                continue;
            }

            if has_sll_conflict_terminating_prediction(&arena, PredictionMode::SLL, &cs) {
                let alt_subsets = get_conflicting_alt_subsets(&arena, &cs);
                let exact = all_subsets_conflict(alt_subsets.values())
                    && all_subsets_equal(alt_subsets.values());
                let mut widened_tainted = false;
                let mut prec_tainted = false;
                let mut any_boundary = false;
                let mut all_boundary = true;
                for c in cs.get_items() {
                    if (c.reaches_into_outer_context & WIDENED_TAINT) != 0 {
                        widened_tainted = true;
                    }
                    if (c.reaches_into_outer_context & PRECPRED_TAINT) != 0 {
                        prec_tainted = true;
                    }
                    if (c.reaches_into_outer_context & BOUNDARY_TAINT) != 0 {
                        any_boundary = true;
                    } else {
                        all_boundary = false;
                    }
                }
                let conflicting = get_alts(alt_subsets.values().cloned());

                let untrusted = prec_tainted
                    || widened_tainted
                    || (any_boundary && !all_boundary)
                    || !self.uniform_pop_depths(s, &conflicting, any_boundary, all_boundary)
                    || !self.trust_exact_ambig;

                if !untrusted && exact {
                    res.exact_ambig_conflicts.push(conflicting.clone());
                    accept_alts[d] = conflicting.iter().next().unwrap() as i32;
                } else {
                    let um = if state_depth[d] > 0 {
                        self.utrust_min_alt(
                            &cs,
                            &conflicting,
                            any_boundary,
                            all_boundary,
                            &arena,
                            &widened_joints,
                            &mut faith_cache,
                        )
                    } else {
                        0
                    };
                    if um > 0 {
                        res.utrust_resolutions += 1;
                        res.exact_ambig_conflicts.push(conflicting);
                        accept_alts[d] = um;
                        continue;
                    }
                    if untrusted {
                        res.approx_conflicts.push(conflicting);
                    } else {
                        res.context_sensitive_conflicts.push(conflicting);
                    }
                    if self.hybrid_mode {
                        accept_alts[d] = ESCAPE;
                        if first_conflict_esc.is_none() {
                            first_conflict_esc = Some(d);
                        }
                    } else if self.abort_on_untrusted_conflict {
                        res.num_dfa_states = states.len();
                        res.category = Category::CONTEXT_SENSITIVE;
                        res.conflict_witness = witness_path(&edge_labels, &edges, d);
                        return false;
                    } else if self.abort_inf_as_overflow {
                        overflow = true;
                        break;
                    }
                }
                continue;
            }

            if self.hybrid_mode
                && (state_depth[d] >= self.hybrid_depth_cap
                    || (states.len() >= self.hybrid_state_cap && state_depth[d] >= 2))
            {
                accept_alts[d] = ESCAPE;
                if first_budget_esc.is_none() {
                    first_budget_esc = Some(d);
                }
                continue;
            }

            let mut fallback = 0;
            for c in cs.get_items() {
                if (c.reaches_into_outer_context & BOUNDARY_TAINT) != 0
                    && (fallback == 0 || c.get_alt() < fallback)
                {
                    fallback = c.get_alt();
                }
            }
            fallback_alts[d] = fallback;

            let succs = self.successors(
                &configs,
                res,
                &arena,
                &mut merge_cache,
                &mut depth_cache,
                &mut widened_joints,
            );
            for ls in succs {
                let id = if let Some(&existing_id) = state_ids.get(&ls.configs) {
                    existing_id
                } else {
                    if states.len() >= self.max_dfa_states {
                        overflow = true;
                        break;
                    }
                    let new_id = states.len();
                    state_ids.insert(ls.configs.clone(), new_id);
                    states.push(ls.configs);
                    edges.push(Vec::new());
                    edge_labels.push(Vec::new());
                    accept_alts.push(0);
                    fallback_alts.push(0);
                    state_depth.push(state_depth[d] + 1);
                    work.push_back(new_id);
                    new_id
                };
                edges[d].push(id);
                edge_labels[d].push(ls.label);
            }
        }

        res.num_dfa_states = states.len();
        if overflow {
            res.category = Category::OVERFLOW;
            return true;
        }
        if res.conflict_witness.is_none() {
            if let Some(esc) = first_conflict_esc {
                res.conflict_witness = witness_path(&edge_labels, &edges, esc);
            }
        }
        if res.budget_witness.is_none() {
            if let Some(esc) = first_budget_esc {
                res.budget_witness = witness_path(&edge_labels, &edges, esc);
            }
        }
        if res.saw_predicate {
            res.category = Category::PREDICATED;
            return false;
        }

        let cyclic = is_cyclic(&edges);
        if !cyclic {
            res.k = longest_path(&edges);
        }

        let mut escapes = 0;
        let mut guarded_count = 0;
        for &a in &accept_alts {
            if a == ESCAPE {
                escapes += 1;
            } else if a == GUARDED {
                guarded_count += 1;
            }
        }
        if escapes > 0 || guarded_count > 0 {
            res.category = Category::HYBRID;
            res.escapes = escapes;
            res.coverage = start_coverage(&edges, &edge_labels, &accept_alts);
            if accept_alts[0] != ESCAPE && res.coverage >= self.hybrid_min_coverage {
                res.dfa = Some(to_static_dfa(
                    res.decision,
                    &edge_labels,
                    &edges,
                    &accept_alts,
                    &fallback_alts,
                    cyclic,
                    res.k,
                    None,
                    Vec::new(),
                    Vec::new(),
                ));
            }
            return false;
        }

        if !res.context_sensitive_conflicts.is_empty() || !res.approx_conflicts.is_empty() {
            res.category = Category::CONTEXT_SENSITIVE;
        } else if !res.exact_ambig_conflicts.is_empty() {
            res.category = Category::EXACT_AMBIG;
        } else if cyclic {
            res.category = Category::LLSTAR;
        } else {
            res.category = Category::LLK;
        }

        if matches!(
            res.category,
            Category::LLK | Category::LLSTAR | Category::EXACT_AMBIG
        ) {
            res.dfa = Some(to_static_dfa(
                res.decision,
                &edge_labels,
                &edges,
                &accept_alts,
                &fallback_alts,
                cyclic,
                res.k,
                None,
                Vec::new(),
                Vec::new(),
            ));
        }
        false
    }

    fn successors<'a>(
        &mut self,
        configs: &[ATNConfig<'a>],
        res: &mut DecisionResult,
        arena: &'a bumpalo::Bump,
        merge_cache: &mut MergeCache<'a>,
        depth_cache: &mut HashMap<usize, i32>,
        widened_joints: &mut HashSet<usize>,
    ) -> Vec<LabeledSuccessor<'a>> {
        let mut labels: Vec<IntervalSetBuf> = Vec::new();
        let mut move_configs: Vec<ATNConfig<'a>> = Vec::new();
        let mut move_targets: Vec<ATNStateRef> = Vec::new();
        let mut move_is_stop: Vec<bool> = Vec::new();

        for c in configs {
            let state = c.get_state();
            if state.state_type() == ATNStateType::RuleStop {
                let mut eof_set = IntervalSetBuf::new();
                eof_set.add_one(EOF);
                labels.push(eof_set);
                move_configs.push(c.clone());
                move_targets.push(state);
                move_is_stop.push(true);
                continue;
            }
            for t in state.get_transitions() {
                if t.is_epsilon() {
                    continue;
                }
                let label = match t.transition_type() {
                    TransitionType::NotSet => {
                        let ns = t.try_as::<NotSetTransition>().unwrap();
                        ns.get_label()
                            .unwrap()
                            .complement(TOKEN_MIN_USER_TOKEN_TYPE, self.atn.max_token_type)
                    }
                    TransitionType::Wildcard => self.all_tokens.clone(),
                    _ => match t.get_label() {
                        Some(set) => set.to_interval_set_buf(),
                        None => continue,
                    },
                };
                if label.as_slice().is_empty() {
                    continue;
                }
                labels.push(label);
                move_configs.push(c.clone());
                move_targets.push(t.get_target());
                move_is_stop.push(false);
            }
        }

        let mut boundary_set = BTreeSet::new();
        for label in &labels {
            for iv in label.as_slice() {
                boundary_set.insert(iv.a);
                boundary_set.insert(iv.b + 1);
            }
        }
        let boundaries: Vec<i32> = boundary_set.into_iter().collect();

        let mut class_order: Vec<BitSet> = Vec::new();
        let mut class_labels: HashMap<BitSet, IntervalSetBuf> = HashMap::new();
        for b in 0..boundaries.len().saturating_sub(1) {
            let lo = boundaries[b];
            let hi = boundaries[b + 1] - 1;
            let mut key = BitSet::with_capacity(labels.len());
            for (m, label) in labels.iter().enumerate() {
                if label.contains(lo) {
                    key.insert(m);
                }
            }
            if key.is_empty() {
                continue;
            }
            if let Some(existing) = class_labels.get_mut(&key) {
                existing.add_range(lo, hi);
            } else {
                let mut set = IntervalSetBuf::new();
                set.add_range(lo, hi);
                class_order.push(key.clone());
                class_labels.insert(key, set);
            }
        }

        let mut succ_order: Vec<CanonicalConfigSet<'a>> = Vec::new();
        let mut by_successor: HashMap<CanonicalConfigSet<'a>, IntervalSetBuf> = HashMap::new();

        for key in class_order {
            let class_label = class_labels.remove(&key).unwrap();
            let mut succ = OrderedConfigSet::new();
            let mut busy: HashSet<(ATNConfig<'a>, bool)> = HashSet::with_capacity(1024);
            for m in &key {
                let c = &move_configs[m];
                if move_is_stop[m] {
                    succ.insert(c.clone());
                } else {
                    let advanced = c.clone().with_state(move_targets[m]);
                    self.closure(
                        advanced,
                        &mut succ,
                        &mut busy,
                        res,
                        0,
                        arena,
                        depth_cache,
                        widened_joints,
                    );
                }
            }
            let succ_canonical =
                CanonicalConfigSet::new(canonical_vec(&succ.items, arena, merge_cache));
            if let Some(merged) = by_successor.get_mut(&succ_canonical) {
                merged.add_set(&class_label);
            } else {
                succ_order.push(succ_canonical.clone());
                by_successor.insert(succ_canonical, class_label);
            }
        }

        succ_order
            .into_iter()
            .map(|succ_key| {
                let label = by_successor.remove(&succ_key).unwrap();
                LabeledSuccessor {
                    label,
                    configs: succ_key,
                }
            })
            .collect()
    }

    #[allow(clippy::too_many_arguments, clippy::too_many_lines)]
    fn closure<'a>(
        &mut self,
        mut config: ATNConfig<'a>,
        configs: &mut OrderedConfigSet<'a>,
        busy: &mut HashSet<(ATNConfig<'a>, bool)>,
        res: &mut DecisionResult,
        depth: i32,
        arena: &'a bumpalo::Bump,
        depth_cache: &mut HashMap<usize, i32>,
        widened_joints: &mut HashSet<usize>,
    ) {
        if !busy.insert((config.clone(), depth < 0)) {
            return;
        }
        let p = config.get_state();

        if p.state_type() == ATNStateType::RuleStop {
            let ctx = config.get_context().unwrap();
            if !ctx.is_empty() {
                for i in 0..ctx.length() {
                    if ctx.get_return_state(i) == ATNStateRef::invalid() {
                        let c = config
                            .clone()
                            .with_state(p)
                            .with_prediction_context(Some(PredictionContextRef::new_empty()));
                        self.closure(
                            c,
                            configs,
                            busy,
                            res,
                            depth,
                            arena,
                            depth_cache,
                            widened_joints,
                        );
                    } else {
                        let return_state = ctx.get_return_state(i);
                        let parent = ctx
                            .get_parent(i)
                            .unwrap_or_else(PredictionContextRef::new_empty);
                        let c = config
                            .clone()
                            .with_state(return_state)
                            .with_prediction_context(Some(parent));
                        self.closure(
                            c,
                            configs,
                            busy,
                            res,
                            depth - 1,
                            arena,
                            depth_cache,
                            widened_joints,
                        );
                    }
                }
                return;
            }
            config.reaches_into_outer_context |= BOUNDARY_TAINT;
            configs.insert(config.clone());
        }

        if !p.has_epsilon_only_transitions() {
            configs.insert(config.clone());
        }

        let boundary_chase = p.state_type() == ATNStateType::RuleStop;

        for t in p.get_transitions() {
            match t.transition_type() {
                TransitionType::Rule => {
                    let rt = t.try_as::<RuleTransition>().unwrap();
                    let follow_state = rt.follow_state;
                    let follow_state_number = follow_state.get_state_number();
                    let ctx = config.get_context().unwrap();
                    let (new_ctx, widened) = if self.current_depth_bound == 0 {
                        (PredictionContextRef::new_empty(), true)
                    } else if Self::context_depth(ctx, depth_cache) >= self.current_depth_bound
                        || has_return_state(ctx, follow_state_number)
                    {
                        let joint: PredictionContextRef<'a> = arena
                            .alloc(PredictionContext::new_singleton(
                                Some(PredictionContextRef::new_empty()),
                                follow_state,
                            ))
                            .into();
                        widened_joints
                            .insert(joint.as_ref() as *const PredictionContext<'a> as usize);
                        (joint, true)
                    } else {
                        let pushed: PredictionContextRef<'a> = arena
                            .alloc(PredictionContext::new_singleton(Some(ctx), follow_state))
                            .into();
                        (pushed, false)
                    };
                    let mut callee = config
                        .clone()
                        .with_state(rt.get_target())
                        .with_prediction_context(Some(new_ctx));
                    if widened {
                        res.used_widening = true;
                        callee.reaches_into_outer_context |= WIDENED_TAINT;
                    }
                    let next_depth = if depth >= 0 { depth + 1 } else { depth };
                    self.closure(
                        callee,
                        configs,
                        busy,
                        res,
                        next_depth,
                        arena,
                        depth_cache,
                        widened_joints,
                    );
                }
                TransitionType::PrecedencePredicate => {
                    let ppt = t.try_as::<PrecedencePredicateTransition>().unwrap();
                    let n = ppt.precedence();
                    if self.prec_start_closure && depth == 0 {
                        if n >= self.prec_eval_value {
                            let c2 = config.clone().with_state(t.get_target());
                            self.closure(
                                c2,
                                configs,
                                busy,
                                res,
                                depth,
                                arena,
                                depth_cache,
                                widened_joints,
                            );
                        }
                    } else {
                        let ctx = config.get_context().unwrap();
                        if let Some(outcome) =
                            self.eval_precpred_from_context(ctx, p.get_rule_index(), n)
                        {
                            if outcome {
                                let c2 = config.clone().with_state(t.get_target());
                                self.closure(
                                    c2,
                                    configs,
                                    busy,
                                    res,
                                    depth,
                                    arena,
                                    depth_cache,
                                    widened_joints,
                                );
                            }
                        } else {
                            res.saw_prec_predicate = true;
                            let mut c2 = config.clone().with_state(t.get_target());
                            c2.reaches_into_outer_context |= PRECPRED_TAINT;
                            self.closure(
                                c2,
                                configs,
                                busy,
                                res,
                                depth,
                                arena,
                                depth_cache,
                                widened_joints,
                            );
                        }
                    }
                }
                TransitionType::Predicate => {
                    let pt = t.try_as::<PredicateTransition>().unwrap();
                    res.saw_predicate = true;
                    if res.first_predicate.is_none() {
                        res.first_predicate = Some((pt.rule_index(), pt.pred_index()));
                    }
                    let mut c2 = config.clone().with_state(t.get_target());
                    c2.reaches_into_outer_context |= PRED_TAINT;
                    self.closure(
                        c2,
                        configs,
                        busy,
                        res,
                        depth,
                        arena,
                        depth_cache,
                        widened_joints,
                    );
                }
                _ if t.is_epsilon() => {
                    if boundary_chase
                        && self.prec_rule_index >= 0
                        && p.get_rule_index() == self.prec_rule_index
                        && depth >= 0
                    {
                        if let Some(call) = self.call_info(t.get_target().get_state_number()) {
                            if call.target_rule_index == self.prec_rule_index
                                && (call.precedence < self.prec_class_lo
                                    || call.precedence > self.prec_class_hi)
                            {
                                continue;
                            }
                        }
                    }
                    let mut c2 = config.clone().with_state(t.get_target());
                    let mut new_depth = depth;
                    if boundary_chase {
                        new_depth = (depth - 1).min(-1);
                        if self.prec_start_closure && p.get_rule_index() == self.prec_rule_index {
                            if let Some(call) = self.call_info(t.get_target().get_state_number()) {
                                if call.precedence == 0
                                    && call.target_rule_index == self.prec_rule_index
                                {
                                    c2.set_precedence_filter_suppressed(true);
                                }
                            }
                        }
                    }
                    self.closure(
                        c2,
                        configs,
                        busy,
                        res,
                        new_depth,
                        arena,
                        depth_cache,
                        widened_joints,
                    );
                }
                _ => {}
            }
        }
    }

    fn context_depth(ctx: PredictionContextRef<'_>, depth_cache: &mut HashMap<usize, i32>) -> i32 {
        if ctx.is_empty() {
            return 0;
        }
        let ptr = ctx.as_ref() as *const PredictionContext<'_> as usize;
        if let Some(&cached) = depth_cache.get(&ptr) {
            return cached;
        }
        let mut max_depth = 0;
        for i in 0..ctx.length() {
            if ctx.get_return_state(i) == ATNStateRef::invalid() {
                continue;
            }
            if let Some(parent) = ctx.get_parent(i) {
                max_depth = max_depth.max(1 + Self::context_depth(parent, depth_cache));
            } else {
                max_depth = max_depth.max(1);
            }
        }
        depth_cache.insert(ptr, max_depth);
        max_depth
    }

    fn uniform_pop_depths(
        &mut self,
        s: ATNStateRef,
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
        if self.current_depth_bound == 0 {
            return false;
        }
        let mut reference: Option<usize> = None;
        let transitions = s.get_transitions();
        for a in conflicting {
            if a == 0 || a > transitions.len() {
                return false;
            }
            let depths = self.completion_depths(transitions[a - 1].get_target());
            if depths.len() != 1 || depths.contains(VARIABLE_DEPTH) {
                return false;
            }
            let d1 = depths.iter().next().unwrap();
            match reference {
                None => reference = Some(d1),
                Some(r) if r != d1 => return false,
                Some(_) => {}
            }
        }
        true
    }

    fn completion_depths(&mut self, alt_start: ATNStateRef) -> BitSet {
        let state_num = alt_start.get_state_number();
        if let Some(cached) = self.completion_depths_cache.get(&state_num) {
            return cached.clone();
        }
        let mut visiting = HashSet::new();
        let mut budget = 100_000i32;
        let depths = Self::token_counts_to_stop(alt_start, &mut visiting, &mut budget);
        self.completion_depths_cache
            .insert(state_num, depths.clone());
        depths
    }

    fn token_counts_to_stop(
        s: ATNStateRef,
        visiting: &mut HashSet<i32>,
        budget: &mut i32,
    ) -> BitSet {
        let mut out = BitSet::with_capacity(VARIABLE_DEPTH + 1);
        if s.state_type() == ATNStateType::RuleStop {
            out.insert(0);
            return out;
        }
        let prev_budget = *budget;
        *budget -= 1;
        if prev_budget <= 0 || !visiting.insert(s.get_state_number()) {
            out.insert(VARIABLE_DEPTH);
            return out;
        }
        for t in s.get_transitions() {
            let sub = if let Some(rt) = t.try_as::<RuleTransition>() {
                let inner = Self::token_counts_to_stop(rt.get_target(), visiting, budget);
                let rest = Self::token_counts_to_stop(rt.follow_state, visiting, budget);
                sum_depths(&inner, &rest)
            } else if t.is_epsilon() {
                Self::token_counts_to_stop(t.get_target(), visiting, budget)
            } else {
                shift_depths(&Self::token_counts_to_stop(
                    t.get_target(),
                    visiting,
                    budget,
                ))
            };
            out.union_with(&sub);
        }
        visiting.remove(&s.get_state_number());
        out
    }

    #[allow(clippy::too_many_arguments)]
    fn utrust_min_alt<'a>(
        &self,
        cs: &MutableATNConfigSet<'a>,
        conflicting: &BitSet,
        any_boundary: bool,
        all_boundary: bool,
        arena: &'a bumpalo::Bump,
        widened_joints: &HashSet<usize>,
        faith_cache: &mut HashMap<usize, bool>,
    ) -> i32 {
        if any_boundary && !all_boundary {
            return 0;
        }
        let Some(m) = conflicting.iter().next() else {
            return 0;
        };
        let m_i32 = m as i32;
        let mut clean_m_by_state: HashMap<i32, Vec<ATNConfig<'a>>> = HashMap::new();
        for c in cs.get_items() {
            if c.get_alt() == m_i32
                && (c.reaches_into_outer_context & (BOUNDARY_TAINT | FOREIGN_CONSUME_TAINT)) == 0
                && runtime_faithful(c, widened_joints, faith_cache)
            {
                clean_m_by_state
                    .entry(c.get_state().get_state_number())
                    .or_default()
                    .push(c.clone());
            }
        }
        if clean_m_by_state.is_empty() {
            return 0;
        }

        let mut by_state_ctx: HashMap<i32, Vec<PredictionContextRef<'a>>> = HashMap::new();
        for c in cs.get_items() {
            let state_num = c.get_state().get_state_number();
            let ctx = c.get_context().unwrap();
            let entries = by_state_ctx.entry(state_num).or_default();
            if !entries.contains(&ctx) {
                entries.push(ctx);
            }
        }

        for (state_num, entry_ctxs) in by_state_ctx {
            let Some(attestations) = clean_m_by_state.get(&state_num) else {
                return 0;
            };
            for entry_ctx in entry_ctxs {
                let assured = attestations
                    .iter()
                    .any(|c| top_prefix_wildcard_match(c.get_context().unwrap(), entry_ctx, arena));
                if !assured {
                    return 0;
                }
            }
        }
        m_i32
    }
}

fn shift_depths(input: &BitSet) -> BitSet {
    let mut out = BitSet::with_capacity(VARIABLE_DEPTH + 1);
    if input.contains(VARIABLE_DEPTH) {
        out.insert(VARIABLE_DEPTH);
        return out;
    }
    for i in input {
        if i + 1 > MAX_TRACKED_DEPTH {
            out.insert(VARIABLE_DEPTH);
            return out;
        }
        out.insert(i + 1);
    }
    out
}

fn sum_depths(a: &BitSet, b: &BitSet) -> BitSet {
    let mut out = BitSet::with_capacity(VARIABLE_DEPTH + 1);
    if a.contains(VARIABLE_DEPTH) || b.contains(VARIABLE_DEPTH) {
        out.insert(VARIABLE_DEPTH);
        return out;
    }
    for i in a {
        for j in b {
            if i + j > MAX_TRACKED_DEPTH {
                out.insert(VARIABLE_DEPTH);
                return out;
            }
            out.insert(i + j);
        }
    }
    out
}

fn top_prefix_wildcard_match<'a>(
    shadow: PredictionContextRef<'a>,
    exact: PredictionContextRef<'a>,
    arena: &'a bumpalo::Bump,
) -> bool {
    if exact.length() > 1 {
        for j in 0..exact.length() {
            if !top_prefix_wildcard_match(shadow, ctx_branch(exact, j, arena), arena) {
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
            if top_prefix_wildcard_match(ctx_branch(shadow, i, arena), exact, arena) {
                return true;
            }
        }
        return false;
    }
    if exact.is_empty() {
        return false;
    }
    let rs = shadow.get_return_state(0);
    if rs == ATNStateRef::invalid() {
        return true;
    }
    if exact.get_return_state(0) == ATNStateRef::invalid() {
        return false;
    }
    rs == exact.get_return_state(0)
        && top_prefix_wildcard_match(
            shadow
                .get_parent(0)
                .unwrap_or_else(PredictionContextRef::new_empty),
            exact
                .get_parent(0)
                .unwrap_or_else(PredictionContextRef::new_empty),
            arena,
        )
}

fn ctx_branch<'a>(
    ctx: PredictionContextRef<'a>,
    i: usize,
    arena: &'a bumpalo::Bump,
) -> PredictionContextRef<'a> {
    let p = ctx
        .get_parent(i)
        .unwrap_or_else(PredictionContextRef::new_empty);
    let rs = ctx.get_return_state(i);
    if rs == ATNStateRef::invalid() && p.is_empty() {
        return PredictionContextRef::new_empty();
    }
    arena
        .alloc(PredictionContext::new_singleton(Some(p), rs))
        .into()
}

fn runtime_faithful(
    c: &ATNConfig<'_>,
    widened_joints: &HashSet<usize>,
    faith_cache: &mut HashMap<usize, bool>,
) -> bool {
    let Some(root_ctx) = c.get_context() else {
        return true;
    };
    if widened_joints.is_empty() || root_ctx.is_empty() {
        return true;
    }
    let root_ptr = root_ctx.as_ref() as *const PredictionContext<'_> as usize;
    if let Some(&cached) = faith_cache.get(&root_ptr) {
        return cached;
    }
    let mut visited = HashSet::new();
    let mut work = VecDeque::new();
    work.push_back(root_ctx);
    let mut faithful = true;
    while let Some(ctx) = work.pop_front() {
        let ptr = ctx.as_ref() as *const PredictionContext<'_> as usize;
        if !visited.insert(ptr) {
            continue;
        }
        if widened_joints.contains(&ptr) {
            faithful = false;
            break;
        }
        for i in 0..ctx.length() {
            if let Some(p) = ctx.get_parent(i) {
                work.push_back(p);
            }
        }
    }
    faith_cache.insert(root_ptr, faithful);
    faithful
}

fn has_return_state(mut ctx: PredictionContextRef<'_>, return_state_number: i32) -> bool {
    while !ctx.is_empty() {
        if ctx.length() != 1 {
            let mut visited = HashSet::new();
            return has_return_state_dag(ctx, return_state_number, &mut visited);
        }
        let rs = ctx.get_return_state(0);
        if rs.get_state_number() == return_state_number {
            return true;
        }
        if rs == ATNStateRef::invalid() {
            return false;
        }
        match ctx.get_parent(0) {
            Some(parent) => ctx = parent,
            None => return false,
        }
    }
    false
}

fn has_return_state_dag(
    ctx: PredictionContextRef<'_>,
    return_state_number: i32,
    visited: &mut HashSet<usize>,
) -> bool {
    if ctx.is_empty() {
        return false;
    }
    let ptr = ctx.as_ref() as *const PredictionContext<'_> as usize;
    if !visited.insert(ptr) {
        return false;
    }
    for i in 0..ctx.length() {
        let rs = ctx.get_return_state(i);
        if rs.get_state_number() == return_state_number {
            return true;
        }
        if rs != ATNStateRef::invalid() {
            if let Some(parent) = ctx.get_parent(i) {
                if has_return_state_dag(parent, return_state_number, visited) {
                    return true;
                }
            }
        }
    }
    false
}

fn to_config_set<'a>(
    configs: &[ATNConfig<'a>],
    arena: &'a bumpalo::Bump,
    merge_cache: &mut MergeCache<'a>,
) -> MutableATNConfigSet<'a> {
    let mut cs = MutableATNConfigSet::new(arena, false);
    for c in configs {
        cs.add_cached(c.clone(), merge_cache);
    }
    cs
}

fn canonical_vec<'a>(
    configs: &[ATNConfig<'a>],
    arena: &'a bumpalo::Bump,
    merge_cache: &mut MergeCache<'a>,
) -> Vec<ATNConfig<'a>> {
    to_config_set(configs, arena, merge_cache)
        .into_iter()
        .collect()
}

fn is_cyclic(edges: &[Vec<usize>]) -> bool {
    let n = edges.len();
    let mut color = vec![0u8; n];
    let mut stack: Vec<(usize, usize)> = Vec::new();
    for root in 0..n {
        if color[root] != 0 {
            continue;
        }
        stack.push((root, 0));
        color[root] = 1;
        while let Some((state, edge_idx)) = stack.last_mut() {
            let s = *state;
            let idx = *edge_idx;
            if idx < edges[s].len() {
                *edge_idx += 1;
                let t = edges[s][idx];
                if color[t] == 1 {
                    return true;
                }
                if color[t] == 0 {
                    color[t] = 1;
                    stack.push((t, 0));
                }
            } else {
                color[s] = 2;
                stack.pop();
            }
        }
    }
    false
}

fn longest_path(edges: &[Vec<usize>]) -> i32 {
    if edges.is_empty() {
        return 0;
    }
    let mut memo = vec![-1i32; edges.len()];
    longest_path_from(0, edges, &mut memo)
}

fn longest_path_from(s: usize, edges: &[Vec<usize>], memo: &mut [i32]) -> i32 {
    if memo[s] >= 0 {
        return memo[s];
    }
    memo[s] = 0;
    let mut max_len = 0;
    for &t in &edges[s] {
        max_len = max_len.max(1 + longest_path_from(t, edges, memo));
    }
    memo[s] = max_len;
    max_len
}

fn witness_path(
    edge_labels: &[Vec<IntervalSetBuf>],
    edges: &[Vec<usize>],
    target: usize,
) -> Option<Vec<i32>> {
    let mut parent: Vec<Option<usize>> = vec![None; edges.len()];
    let mut parent_tok = vec![0i32; edges.len()];
    let mut bfs = VecDeque::new();
    parent[0] = Some(0);
    bfs.push_back(0usize);
    while let Some(d) = bfs.pop_front() {
        if d == target {
            break;
        }
        for (i, &t) in edges[d].iter().enumerate() {
            if parent[t].is_none() {
                parent[t] = Some(d);
                parent_tok[t] = edge_labels[d][i].get_min().unwrap_or(EOF);
                bfs.push_back(t);
            }
        }
    }
    parent[target]?;
    let mut path = Vec::new();
    let mut x = target;
    while x != 0 {
        path.push(parent_tok[x]);
        x = parent[x].unwrap();
    }
    path.reverse();
    Some(path)
}

fn start_coverage(
    edges: &[Vec<usize>],
    edge_labels: &[Vec<IntervalSetBuf>],
    accept_alts: &[i32],
) -> f64 {
    let n = edges.len();
    let mut reverse = vec![Vec::new(); n];
    for (s_idx, out) in edges.iter().enumerate() {
        for &target in out {
            reverse[target].push(s_idx);
        }
    }
    let mut reaches_accept = vec![false; n];
    let mut work = VecDeque::new();
    for (i, &alt) in accept_alts.iter().enumerate() {
        if alt > 0 || alt == GUARDED {
            reaches_accept[i] = true;
            work.push_back(i);
        }
    }
    while let Some(curr) = work.pop_front() {
        for &prev in &reverse[curr] {
            if !reaches_accept[prev] {
                reaches_accept[prev] = true;
                work.push_back(prev);
            }
        }
    }
    let mut total = 0i64;
    let mut useful = 0i64;
    for (e, &target) in edges[0].iter().enumerate() {
        let width = i64::from(edge_labels[0][e].length());
        total += width;
        if reaches_accept[target] {
            useful += width;
        }
    }
    if total == 0 {
        0.0
    } else {
        (useful as f64) / (total as f64)
    }
}

#[allow(clippy::too_many_arguments)]
fn to_static_dfa(
    decision: usize,
    edge_labels: &[Vec<IntervalSetBuf>],
    edge_targets: &[Vec<usize>],
    accept_alts: &[i32],
    fallback_alts: &[i32],
    cyclic: bool,
    max_k: i32,
    guard_alts: Option<&[i32]>,
    guard_danger: Vec<i32>,
    guard_pass: Vec<i32>,
) -> StaticParserDFA {
    let n = edge_targets.len();
    let accepts = accept_alts.to_vec();
    let fallbacks = fallback_alts.to_vec();
    let guarded_alts = guard_alts.map_or_else(|| vec![0; n], <[i32]>::to_vec);
    let mut edge_offsets = Vec::with_capacity(n + 1);
    let mut edges = Vec::new();

    for s in 0..n {
        edge_offsets.push(edges.len());
        let mut state_triples: Vec<(i32, i32, i32)> = Vec::new();
        for (e, &target) in edge_targets[s].iter().enumerate() {
            for iv in edge_labels[s][e].as_slice() {
                state_triples.push((iv.a, iv.b, target as i32));
            }
        }
        state_triples.sort_by_key(|&(lo, _, _)| lo);
        for (lo, hi, target) in state_triples {
            edges.push(lo);
            edges.push(hi);
            edges.push(target);
        }
    }
    edge_offsets.push(edges.len());

    StaticParserDFA {
        decision,
        num_states: n,
        accepts,
        fallbacks,
        edge_offsets,
        edges,
        cyclic,
        max_k,
        guarded_alts,
        guard_danger,
        guard_pass,
    }
}

pub fn format_bitset(bs: &BitSet) -> String {
    let mut out = String::from("{");
    for (idx, val) in bs.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{val}");
    }
    out.push('}');
    out
}

pub fn distinct_bitsets(sets: &[BitSet]) -> Vec<BitSet> {
    let mut out = Vec::new();
    for s in sets {
        if !out.contains(s) {
            out.push(s.clone());
        }
    }
    out
}

pub fn format_distinct_bitsets(sets: &[BitSet]) -> String {
    let distinct = distinct_bitsets(sets);
    let mut out = String::from("[");
    for (idx, bs) in distinct.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        out.push_str(&format_bitset(bs));
    }
    out.push(']');
    out
}

pub fn format_histogram(map: &BTreeMap<i32, usize>) -> String {
    let mut out = String::from("{");
    for (idx, (k, v)) in map.iter().enumerate() {
        if idx > 0 {
            out.push_str(", ");
        }
        let _ = write!(out, "{k}={v}");
    }
    out.push('}');
    out
}
