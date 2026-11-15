// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The prefix-hijack analysis of the guesses of `--static-dfa-only`.
//!
//! At an escape state, `--static-dfa-only` predicts one alternative (the
//! chosen alternative) and drops the other viable alternatives. The guess is
//! harmful when a dropped alternative can read a token that the chosen
//! alternative cannot read. On such input, the parser commits to the chosen
//! alternative. Then it reports a syntax error, although the grammar accepts
//! the input. A narrow alternative before a broad alternative with a shared
//! prefix has this problem, for example `'(' selectStmt ')'` before
//! `'(' exprList ')'`.
//!
//! The analysis continues the lookahead after the escape state, one token at
//! a time. It keeps the configs of the chosen alternative and of the dropped
//! alternatives side by side. It stops at a token that only the dropped
//! alternatives read (a hijack). It also stops when the chosen alternative
//! reads each continuation of the dropped alternatives (covered), or at its
//! budget (unknown).
//!
//! The stack-hijack analysis applies the same comparison to the guarded
//! states. A guard defers when a call of its danger set is on the parse
//! stack, without a look at the tokens after the lookahead of the table. The
//! guess is harmful when the danger caller is an outer wrapper of the
//! decision: inside the call of the danger caller, the guarded alternative
//! reads a token that the defer alternative cannot read. For example, in
//! `SELECT 1 SETTINGS a = 1 UNION ALL SELECT 2`, a statement that can end
//! with SETTINGS makes the SETTINGS of the SELECT clause a skip, and the
//! skip cannot read UNION. The analysis builds real parse stacks where the
//! guard defers, reads the lookahead path of the state on each stack, and
//! then continues as the prefix-hijack analysis does.

use std::cell::Cell;
use std::collections::{HashMap, HashSet, VecDeque};

use dbt_antlr_runtime::analysis::{
    ATNConfig, ATNStateRef, ATNStateType, BOUNDARY_TAINT, IntervalSetBuf, PRECPRED_TAINT,
    PRED_TAINT, PredictionContext, PredictionContextRef, Transition, TransitionType, WIDENED_TAINT,
};
use dbt_antlr_runtime::token::TOKEN_EOF;

use super::closure::{
    CallSites, ClosedSet, Closure, GuardRoots, PrecedenceClass, RuleCall, Taint, taint_of,
};

/// The tokens that the analysis reads after an escape state at most.
const HAZARD_DEPTH: usize = 8;
/// The pairs of config sets that the analysis of one state visits at most.
const HAZARD_PAIRS: usize = 64;
/// The configs that the analysis of one state can visit.
const HAZARD_VISITS: u64 = 20_000;

/// What the analysis found about the guess of one escape state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum HijackVerdict {
    /// The chosen alternative reads every continuation of the dropped
    /// alternatives that the analysis explored to its end.
    Covered,
    /// The analysis stopped at its budget.
    Unknown,
    /// After `extension`, the dropped alternative `alt` reads the last token
    /// of `extension`, and the chosen alternative cannot read it.
    Hijack { alt: i32, extension: Vec<i32> },
}

/// The hazard of the guess of one escape state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StateHazard {
    pub(crate) verdict: HijackVerdict,
    /// The guess is the greedy choice of a loop or of an optional element:
    /// it repeats or takes, and drops the exit or the skip.
    pub(crate) greedy: bool,
}

/// How harmful a guess is, from the least to the most harmful.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum HazardLevel {
    /// The chosen alternative covers the dropped ones, or the guess is the
    /// greedy choice of a loop or an optional element without a found
    /// hijack.
    Low,
    /// The analysis did not decide, or a greedy choice can hijack.
    Medium,
    /// A dropped alternative continues where the chosen alternative fails.
    High,
}

impl StateHazard {
    pub(crate) fn level(&self) -> HazardLevel {
        match (&self.verdict, self.greedy) {
            (HijackVerdict::Hijack { .. }, false) => HazardLevel::High,
            (HijackVerdict::Hijack { .. }, true) | (HijackVerdict::Unknown, false) => {
                HazardLevel::Medium
            }
            (HijackVerdict::Unknown, true) | (HijackVerdict::Covered, _) => HazardLevel::Low,
        }
    }
}

/// Continues the lookahead of an escape state with the configs `configs`,
/// and compares the alternative `chosen` with the alternatives `dropped`.
pub(super) fn analyze<'a>(
    closure: &mut Closure<'_, 'a>,
    configs: &[ATNConfig<'a>],
    taint: &Taint,
    chosen: i32,
    dropped: &[i32],
) -> HijackVerdict {
    let subset = |alts: &dyn Fn(i32) -> bool| ClosedSet {
        configs: configs
            .iter()
            .filter(|config| alts(config.get_alt()))
            .cloned()
            .collect(),
        taint: taint.clone(),
        roots: GuardRoots::default(),
    };
    // A config of the escape state that popped the decision rule can have
    // read tokens in its assumed caller already.
    let mut chosen = subset(&|alt| alt == chosen);
    let mut dropped = subset(&|alt| dropped.contains(&alt));
    commit_popped(&mut chosen);
    commit_popped(&mut dropped);
    let start = (chosen, dropped, Vec::new());
    closure
        .sandbox(HAZARD_VISITS, |closure| explore(closure, start, &|_| {}))
        .unwrap_or(HijackVerdict::Unknown)
}

type Pair<'a> = (ClosedSet<'a>, ClosedSet<'a>, Vec<i32>);

/// A taint bit of this analysis only: the path read a token after it popped
/// the decision rule into a caller that the closure assumes. Two such paths
/// can read their tokens in different callers and then meet at one state.
const COMMITTED_TAINT: i32 = 1 << 20;

/// The taint bits of a path that the runtime cannot take for sure: a
/// predicate, a precedence predicate, a widened stack, or a token read in an
/// assumed caller.
const UNSURE_TAINT: i32 = PRED_TAINT | PRECPRED_TAINT | WIDENED_TAINT | COMMITTED_TAINT;

/// A pair of config sets, with the taint of each config that matters, as the
/// key of the explored pairs.
type PairKey<'a> = (Vec<(ATNConfig<'a>, i32)>, Vec<(ATNConfig<'a>, i32)>);

fn pair_key<'a>(chosen: &ClosedSet<'a>, dropped: &ClosedSet<'a>) -> PairKey<'a> {
    let tainted = |set: &ClosedSet<'a>| {
        set.configs
            .iter()
            .map(|config| {
                let taint = set.taint_of(config) & (UNSURE_TAINT | BOUNDARY_TAINT);
                (config.clone(), taint)
            })
            .collect()
    };
    (tainted(chosen), tainted(dropped))
}

/// Explores the pairs after `start`. `retire` removes the dropped configs
/// that cannot give evidence any more before each token.
fn explore<'a>(
    closure: &mut Closure<'_, 'a>,
    start: Pair<'a>,
    retire: &dyn Fn(&mut ClosedSet<'a>),
) -> Option<HijackVerdict> {
    let base = start.2.len();
    let mut queue: VecDeque<Pair<'a>> = VecDeque::from([start]);
    let mut seen: HashSet<PairKey<'a>> = HashSet::new();
    let mut complete = true;
    while let Some((mut chosen, mut dropped, extension)) = queue.pop_front() {
        retire(&mut dropped);
        // A dropped config at the same state and stack as a chosen config has
        // the same continuations, when both paths are sure.
        let dropped_taint = dropped.taint.clone();
        dropped
            .configs
            .retain(|config| !covered_by(&chosen, &dropped_taint, config));
        if dropped.configs.is_empty() {
            continue;
        }
        if !seen.insert(pair_key(&chosen, &dropped)) {
            continue;
        }
        if seen.len() > HAZARD_PAIRS {
            complete = false;
            continue;
        }
        // A chosen config that leaves the decision to an unknown caller can
        // read any token after the decision.
        let chosen_open = chosen.configs.iter().any(is_open_end);
        // The tokens that a sure chosen path reads. Only these prove that the
        // chosen alternative covers a dropped one.
        let sure_readable = sure_reads(&chosen);
        // The closure shows a dropped config that leaves the decision as a
        // read of EOF. Only a faithful config with a real EOF transition
        // reads EOF.
        let eof_alts: Vec<i32> = dropped
            .configs
            .iter()
            .filter(|config| is_faithful(dropped.taint_of(config)) && reads_eof(config))
            .map(ATNConfig::get_alt)
            .collect();
        commit_popped(&mut chosen);
        commit_popped(&mut dropped);
        let chosen_next = closure.successors(&chosen);
        let dropped_next = closure.successors(&dropped);
        if closure.exhausted {
            return None;
        }
        let mut readable = IntervalSetBuf::new();
        for (label, _) in &chosen_next {
            readable.add_set(label);
        }
        for (label, set) in &dropped_next {
            if first_token_outside(label, &sure_readable, true).is_some() {
                complete = false;
            }
            let faithful_alt = |eof: bool| {
                set.configs
                    .iter()
                    .filter(|config| is_faithful(set.taint_of(config)))
                    .map(ATNConfig::get_alt)
                    .filter(|alt| !eof || eof_alts.contains(alt))
                    .min()
            };
            let hijack = first_token_outside(label, &readable, false)
                .map(|token| (token, faithful_alt(false)))
                .or_else(|| {
                    let eof = label.as_interval_set().contains(TOKEN_EOF)
                        && !readable.as_interval_set().contains(TOKEN_EOF);
                    eof.then(|| (TOKEN_EOF, faithful_alt(true)))
                });
            match hijack {
                Some((token, Some(alt))) if !chosen_open => {
                    let mut extension = extension.clone();
                    extension.push(token);
                    return Some(HijackVerdict::Hijack { alt, extension });
                }
                Some(_) => complete = false,
                None => {}
            }
        }
        if extension.len() - base + 1 >= HAZARD_DEPTH {
            complete &= dropped_next.is_empty();
            continue;
        }
        let chosen_next: Vec<_> = chosen_next
            .into_iter()
            .map(|(label, set)| (label, closure.canonical_set(set)))
            .collect();
        let dropped_next: Vec<_> = dropped_next
            .into_iter()
            .map(|(label, set)| (label, closure.canonical_set(set)))
            .collect();
        for (dropped_label, dropped_set) in &dropped_next {
            for (chosen_label, chosen_set) in &chosen_next {
                if let Some(token) = first_common_token(dropped_label, chosen_label) {
                    let mut extension = extension.clone();
                    extension.push(token);
                    queue.push_back((clone_set(chosen_set), clone_set(dropped_set), extension));
                }
            }
        }
    }
    Some(if complete {
        HijackVerdict::Covered
    } else {
        HijackVerdict::Unknown
    })
}

/// Whether the taint of a config shows a path that the runtime can take. A
/// path across a predicate depends on the parser state. A widened path that
/// popped its rule, or a precedence predicate inside its rule, has
/// continuations that the runtime stack does not have.
fn is_faithful(taint: i32) -> bool {
    let widened_pop = WIDENED_TAINT | BOUNDARY_TAINT;
    taint & PRED_TAINT == 0
        && taint & widened_pop != widened_pop
        && taint & (PRECPRED_TAINT | BOUNDARY_TAINT) != PRECPRED_TAINT
}

/// Marks the configs of `set` that popped the decision rule, so that the
/// configs after their next token carry [`COMMITTED_TAINT`].
fn commit_popped(set: &mut ClosedSet<'_>) {
    for taint in set.taint.values_mut() {
        if *taint & BOUNDARY_TAINT != 0 {
            *taint |= COMMITTED_TAINT;
        }
    }
}

/// Whether a chosen config has the state and the stack of the dropped config
/// `config`, on two sure paths that both stayed in the decision rule or both
/// popped it without a token after the pop.
fn covered_by(chosen: &ClosedSet<'_>, dropped_taint: &Taint, config: &ATNConfig<'_>) -> bool {
    let taint = taint_of(dropped_taint, config);
    taint & UNSURE_TAINT == 0
        && chosen.configs.iter().any(|known| {
            let known_taint = chosen.taint_of(known);
            known.get_state() == config.get_state()
                && known.get_context() == config.get_context()
                && known_taint & UNSURE_TAINT == 0
                && known_taint & BOUNDARY_TAINT == taint & BOUNDARY_TAINT
        })
}

/// The tokens that the configs of `chosen` read next on a sure path inside
/// the decision rule.
fn sure_reads(chosen: &ClosedSet<'_>) -> IntervalSetBuf {
    let mut tokens = IntervalSetBuf::new();
    for config in &chosen.configs {
        if chosen.taint_of(config) & (UNSURE_TAINT | BOUNDARY_TAINT) != 0 {
            continue;
        }
        for transition in config.get_state().get_transitions() {
            if let Some(label) = read_label(transition) {
                tokens.add_set(&label);
            }
        }
    }
    tokens
}

/// Whether `config` leaves the decision to an unknown caller: it is at the
/// stop state of a rule, with an empty stack path, and no return edge.
fn is_open_end(config: &ATNConfig<'_>) -> bool {
    config.get_state().state_type() == ATNStateType::RuleStop
        && config.get_state().get_transitions().is_empty()
        && config
            .get_context()
            .is_none_or(|context| context.has_empty_path())
}

/// Whether `config` is not at a rule stop and has an EOF transition.
fn reads_eof(config: &ATNConfig<'_>) -> bool {
    let state = config.get_state();
    state.state_type() != ATNStateType::RuleStop
        && state.get_transitions().iter().any(|transition| {
            read_label(transition).is_some_and(|label| label.as_interval_set().contains(TOKEN_EOF))
        })
}

/// The tokens that a transition reads, for an atom, a range, or a set. The
/// label of a negated set holds the tokens that it does not read, so a negated
/// set and a wildcard give `None`, like an epsilon transition.
fn read_label(transition: &Transition) -> Option<IntervalSetBuf> {
    match transition.transition_type() {
        TransitionType::Atom | TransitionType::Range | TransitionType::Set => transition
            .get_label()
            .map(|label| label.to_interval_set_buf()),
        _ => None,
    }
}

fn clone_set<'a>(set: &ClosedSet<'a>) -> ClosedSet<'a> {
    ClosedSet {
        configs: set.configs.clone(),
        taint: set.taint.clone(),
        roots: set.roots.clone(),
    }
}

/// The lowest token of `label` that `readable` does not hold. The token is
/// not EOF unless `with_eof`.
fn first_token_outside(
    label: &IntervalSetBuf,
    readable: &IntervalSetBuf,
    with_eof: bool,
) -> Option<i32> {
    label
        .as_interval_set()
        .as_slice()
        .iter()
        .flat_map(|interval| interval.a..=interval.b)
        .filter(|token| with_eof || *token != TOKEN_EOF)
        .find(|token| !readable.as_interval_set().contains(*token))
}

/// The lowest token that `left` and `right` both hold.
fn first_common_token(left: &IntervalSetBuf, right: &IntervalSetBuf) -> Option<i32> {
    left.as_interval_set()
        .as_slice()
        .iter()
        .flat_map(|interval| interval.a..=interval.b)
        .find(|token| right.as_interval_set().contains(*token))
}

/// The danger callers that the analysis of one guard group explores at most.
const GUARD_CHAINS: usize = 16;
/// The pass frames between the decision rule and a danger caller at most.
const GUARD_PASS_DEPTH: usize = 16;
/// The frames above a danger caller, up to a rule without callers, at most.
const OUTER_DEPTH: usize = 32;

/// What the stack-hijack analysis found about the stack guard of one guarded
/// state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum StackHijackVerdict {
    /// On each explored parse stack where the guard defers, the defer
    /// alternative reads every continuation of the guarded alternative inside
    /// the call of the danger caller.
    Covered,
    /// The analysis stopped at its budget, or a path was not sure.
    Unknown,
    /// When the danger caller `caller` is on the parse stack, the guarded
    /// alternative reads the last token of `extension` inside the call of
    /// `caller`, and the defer alternative cannot read it. `extension` is
    /// the whole lookahead path from the start of the decision.
    Hijack { caller: i32, extension: Vec<i32> },
}

impl StackHijackVerdict {
    pub(crate) fn level(&self) -> HazardLevel {
        match self {
            Self::Hijack { .. } => HazardLevel::High,
            Self::Unknown => HazardLevel::Medium,
            Self::Covered => HazardLevel::Low,
        }
    }
}

/// One parse stack where the guard of a group defers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct GuardChain {
    /// The invoking state of the danger set where the guard defers.
    caller: i32,
    /// The follow states of the stack, the innermost frame first. The first
    /// `inner` frames are the calls from `caller` down to the decision rule.
    /// The other frames are calls from a rule without callers down to the
    /// rule of `caller`.
    follows: Vec<i32>,
    inner: usize,
}

/// The parse stacks where the guard of one group defers.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct GuardChains {
    chains: Vec<GuardChain>,
    /// `false` when the chains do not include each danger caller.
    complete: bool,
}

/// The parse stacks where a guard with the danger set `danger` and the pass
/// set `pass` defers, at most one for each danger caller.
///
/// The guard walks up the stack from the decision rule. It continues through
/// a pass frame, and defers at the first frame of the danger set. A stack
/// with a call between two rules continues to the first rule without
/// callers, so that the frames above the danger caller are real frames. The
/// stacks with pass frames come first: there the danger caller is an outer
/// wrapper of the decision.
pub(super) fn guard_chains(
    calls: &CallSites,
    decision_rule: i32,
    precedence: Option<PrecedenceClass>,
    danger: &[i32],
    pass: &[i32],
) -> GuardChains {
    let mut chains = Vec::new();
    let mut complete = true;
    let mut expanded = HashSet::from([decision_rule]);
    let mut queue: VecDeque<(i32, Vec<RuleCall>)> = VecDeque::from([(decision_rule, Vec::new())]);
    while let Some((rule, path)) = queue.pop_front() {
        for call in calls.calls_to(rule) {
            let outside_class = path.is_empty()
                && precedence.is_some_and(|class| {
                    class.rule_index == rule
                        && (call.precedence < class.lo || call.precedence > class.hi)
                });
            if outside_class {
                continue;
            }
            let mut inner = path.clone();
            inner.push(*call);
            if danger.binary_search(&call.site).is_ok() {
                let mut follows: Vec<i32> = inner.iter().map(|call| call.follow).collect();
                follows.extend(outer_follows(calls, call.caller_rule));
                chains.push(GuardChain {
                    caller: call.site,
                    inner: inner.len(),
                    follows,
                });
            } else if pass.binary_search(&call.site).is_ok() {
                if inner.len() == GUARD_PASS_DEPTH {
                    complete = false;
                } else if expanded.insert(call.caller_rule) {
                    queue.push_back((call.caller_rule, inner));
                }
            }
        }
    }
    chains.sort_by_key(|chain| (chain.inner == 1, chain.inner));
    complete &= chains.len() <= GUARD_CHAINS;
    chains.truncate(GUARD_CHAINS);
    GuardChains { chains, complete }
}

/// The follow states of a shortest chain of calls from a rule without
/// callers down to `rule`, the innermost frame first. The chain is empty
/// when no such rule is near.
fn outer_follows(calls: &CallSites, rule: i32) -> Vec<i32> {
    let mut reached = HashSet::from([rule]);
    let mut queue: VecDeque<(i32, Vec<i32>)> = VecDeque::from([(rule, Vec::new())]);
    while let Some((rule, follows)) = queue.pop_front() {
        let callers = calls.calls_to(rule);
        if callers.is_empty() {
            return follows;
        }
        if follows.len() == OUTER_DEPTH {
            continue;
        }
        for call in callers {
            if reached.insert(call.caller_rule) {
                let mut follows = follows.clone();
                follows.push(call.follow);
                queue.push_back((call.caller_rule, follows));
            }
        }
    }
    Vec::new()
}

/// The stack-hijack verdict of a guarded state with the lookahead path
/// `witness`, the guarded alternative `guarded_alt`, and the defer
/// alternative `defer_alt`, on the stacks `chains` where its guard defers.
///
/// On each stack, the analysis starts the decision again and reads
/// `witness`. Then it continues the lookahead with the configs of the defer
/// alternative as the chosen side and the configs of the guarded alternative
/// as the dropped side. A guarded config gives evidence only while the call
/// of the danger caller is on its stack: after the pop, the danger caller
/// reads its own continuation.
pub(super) fn analyze_guard<'a>(
    closure: &mut Closure<'_, 'a>,
    decision_state: ATNStateRef,
    chains: &GuardChains,
    witness: &[i32],
    (guarded_alt, defer_alt): (i32, i32),
) -> StackHijackVerdict {
    // The stacks are real, so the analysis does not widen them.
    closure.with_depth_bound(i32::MAX, |closure| {
        let mut complete = chains.complete;
        for chain in &chains.chains {
            let verdict = closure.sandbox(HAZARD_VISITS, |closure| {
                analyze_chain(
                    closure,
                    decision_state,
                    chain,
                    witness,
                    (guarded_alt, defer_alt),
                )
            });
            match verdict {
                Some(HijackVerdict::Hijack { extension, .. }) => {
                    return StackHijackVerdict::Hijack {
                        caller: chain.caller,
                        extension,
                    };
                }
                Some(HijackVerdict::Covered) => {}
                Some(HijackVerdict::Unknown) | None => complete = false,
            }
        }
        if complete {
            StackHijackVerdict::Covered
        } else {
            StackHijackVerdict::Unknown
        }
    })
}

fn analyze_chain<'a>(
    closure: &mut Closure<'_, 'a>,
    decision_state: ATNStateRef,
    chain: &GuardChain,
    witness: &[i32],
    (guarded_alt, defer_alt): (i32, i32),
) -> Option<HijackVerdict> {
    let context = closure.call_stack(&chain.follows);
    let start = closure.start_in(decision_state, context);
    if closure.exhausted {
        return None;
    }
    let subset = |alt: i32| ClosedSet {
        configs: start
            .configs
            .iter()
            .filter(|config| config.get_alt() == alt)
            .cloned()
            .collect(),
        taint: start.taint.clone(),
        roots: GuardRoots::default(),
    };
    let outer = chain.follows.len() - chain.inner;
    // A merged config can have a path inside the call of the danger caller
    // and a path outside it. Such a config gives no evidence, and the chain
    // is then not covered.
    let mixed = Cell::new(false);
    let retire = |set: &mut ClosedSet<'a>| {
        let mut depths = HashMap::new();
        set.configs.retain(|config| {
            let (least, most) = frames(config.get_context(), &mut depths);
            if least <= outer && most > outer {
                mixed.set(true);
            }
            least > outer
        });
    };
    let mut chosen = subset(defer_alt);
    let mut dropped = subset(guarded_alt);
    // The token of the witness that the defer alternative cannot read, when
    // it stops before the end of the witness.
    let mut chosen_end = None;
    for (index, &token) in witness.iter().enumerate() {
        retire(&mut dropped);
        if dropped.configs.is_empty() {
            return Some(if mixed.get() {
                HijackVerdict::Unknown
            } else {
                HijackVerdict::Covered
            });
        }
        commit_popped(&mut chosen);
        commit_popped(&mut dropped);
        if chosen_end.is_none() {
            let chosen_open = chosen.configs.iter().any(is_open_end);
            chosen = closure.step(&chosen, token);
            if chosen.configs.is_empty() {
                chosen_end = Some((index, chosen_open));
            }
        }
        dropped = closure.step(&dropped, token);
        if closure.exhausted {
            return None;
        }
        chosen = closure.canonical_set(chosen);
        dropped = closure.canonical_set(dropped);
    }
    // The runtime reaches the guarded state only after the whole witness, so
    // a hijack in the witness needs a guarded path through all of it.
    retire(&mut dropped);
    if dropped.configs.is_empty() {
        return Some(if mixed.get() {
            HijackVerdict::Unknown
        } else {
            HijackVerdict::Covered
        });
    }
    if let Some((index, chosen_open)) = chosen_end {
        let faithful = dropped
            .configs
            .iter()
            .any(|config| is_faithful(dropped.taint_of(config)));
        return Some(if faithful && !chosen_open {
            HijackVerdict::Hijack {
                alt: guarded_alt,
                extension: witness[..=index].to_vec(),
            }
        } else {
            HijackVerdict::Unknown
        });
    }
    let verdict = explore(closure, (chosen, dropped, witness.to_vec()), &retire);
    match verdict {
        Some(HijackVerdict::Covered) if mixed.get() => Some(HijackVerdict::Unknown),
        verdict => verdict,
    }
}

/// The least and the most number of frames on the paths of the stack
/// `context`. `depths` keeps the counts of the shared nodes of the stack.
fn frames(
    context: Option<PredictionContextRef<'_>>,
    depths: &mut HashMap<usize, (usize, usize)>,
) -> (usize, usize) {
    let Some(context) = context else {
        return (0, 0);
    };
    if context.is_empty() {
        return (0, 0);
    }
    let pointer = context.as_ref() as *const PredictionContext<'_> as usize;
    if let Some(&known) = depths.get(&pointer) {
        return known;
    }
    let mut range: Option<(usize, usize)> = None;
    for index in 0..context.length() {
        let (least, most) = if context.get_return_state(index) == ATNStateRef::empty_return() {
            (0, 0)
        } else {
            let (least, most) = frames(context.get_parent(index), depths);
            (least + 1, most + 1)
        };
        range = Some(range.map_or((least, most), |(known_least, known_most)| {
            (known_least.min(least), known_most.max(most))
        }));
    }
    let range = range.unwrap_or((0, 0));
    depths.insert(pointer, range);
    range
}
