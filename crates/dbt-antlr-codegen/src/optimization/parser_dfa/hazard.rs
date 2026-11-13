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

use std::collections::{HashSet, VecDeque};

use dbt_antlr_runtime::analysis::{
    ATNConfig, ATNStateType, BOUNDARY_TAINT, IntervalSetBuf, PRECPRED_TAINT, PRED_TAINT,
    Transition, TransitionType, WIDENED_TAINT,
};
use dbt_antlr_runtime::token::TOKEN_EOF;

use super::closure::{ClosedSet, Closure, GuardRoots, Taint, taint_of};

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
        .sandbox(HAZARD_VISITS, |closure| explore(closure, start))
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

fn explore<'a>(closure: &mut Closure<'_, 'a>, start: Pair<'a>) -> Option<HijackVerdict> {
    let mut queue: VecDeque<Pair<'a>> = VecDeque::from([start]);
    let mut seen: HashSet<PairKey<'a>> = HashSet::new();
    let mut complete = true;
    while let Some((mut chosen, mut dropped, extension)) = queue.pop_front() {
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
        if extension.len() + 1 >= HAZARD_DEPTH {
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
