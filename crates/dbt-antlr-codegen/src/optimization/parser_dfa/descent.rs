// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Shared-descent groups: alternatives that all start with the same rule.
//!
//! When every member alternative of a group consumes the same number of leading
//! tokens (`0..=2`, the *prefix*) and then calls the same *shared rule* `R`
//! directly, a table state whose live alternatives are all members does not
//! have to escape. It accepts with the group's virtual alternative instead. The
//! generated block parses `R` once with muted listeners, reads the token after
//! it, and dispatches to the member alternative whose tail starts with that
//! token. The member then re-descends through its prefix, and its call of `R`
//! takes the subtree of the first parse.
//!
//! The group accept must be a proof, never a guess:
//!
//! - A member is one straight path from its start to its `R` call, inside the
//!   decision rule: the prefix tokens, and no other rule call, predicate,
//!   decision, or loop. Then skipping the prefix lands on the start of `R` for
//!   every input of every member, and the re-descent reaches `R` at that same
//!   position without a prediction of its own.
//! - Every member calls `R` with the same precedence, so the one parse of `R`
//!   serves all of them. `R` takes no arguments, and no rule reachable from
//!   `R` runs an action: a fallback after the first parse parses `R` again.
//! - The tail dispatch picks a member only when no other member can continue
//!   with the token (see [`Dispatch`]).
//! - At the state, the table has read at least the prefix, and every live
//!   configuration is inside a member's call of `R` or right after it
//!   ([`DescentPlan::group_for`]). So no member has read `R` with another
//!   extent than the others.
//! - The neutral parse runs with the call site of one member. A prediction in
//!   it that reads the parse stack (full-context prediction, a stack guard)
//!   could parse `R` differently for another member, so the runtime discards
//!   such a neutral parse and predicts adaptively.
//!
//! The reference analyzer (`SharedDescentAnalyzer` of the k3 tool) checks the
//! first point only on the first state the prefix reaches and follows nullable
//! rule calls before the prefix; both let a member reach `R` after another
//! number of tokens. It also takes one level of `LOOK` as the FOLLOW of the
//! decision rule, accepts configurations past `R`, and trusts every neutral
//! parse. This port is stricter in all of these places.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use dbt_antlr_runtime::analysis::{
    ATN, ATNConfig, ATNStateRef, ATNStateType, BOUNDARY_TAINT, BasicBlockStartState, BitSet, EOF,
    PlusBlockStartState, PredictionContextRef, RuleTransition, StarBlockStartState, TransitionType,
    WIDENED_TAINT,
};

use super::closure::{Taint, taint_of};
use dbt_antlr_runtime::dfa::static_parser_dfa::VIRTUAL_ALT_BASE;
use dbt_antlr_runtime::token::{CommonToken, TOKEN_EPSILON};

/// Most leading tokens a group may share before its shared rule.
const MAX_PREFIX_TOKENS: usize = 2;

/// Most token types of one prefix transition the path search enumerates.
const MAX_PREFIX_LABEL: usize = 32;

/// One tail arm of a group: the member that the token after the shared rule
/// selects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TailArm {
    pub(crate) alt: i32,
    /// Sorted token types; `-1` is EOF.
    pub(crate) tokens: Vec<i32>,
    /// The arm commits only when the token cannot follow the decision rule on
    /// the real parse stack (a nullable member could take it there).
    pub(crate) guarded: bool,
}

/// One shared-descent group of a decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DescentGroup {
    /// What a table state accepts with for this group.
    pub(crate) virtual_alt: i32,
    /// The shared rule.
    pub(crate) rule: usize,
    /// The member alternatives, ascending.
    pub(crate) alts: Vec<i32>,
    /// The leading tokens of the first member path that formed the group.
    pub(crate) prefix: Vec<i32>,
    /// A member's call-site state of the shared rule: the neutral parse runs
    /// with it as the invoking state.
    pub(crate) call_site: i32,
    /// The precedence argument of every member's call of the shared rule.
    pub(crate) precedence: i32,
    /// The tail dispatch.
    pub(crate) arms: Vec<TailArm>,
    /// The member that every token without an arm selects, `0` when such a
    /// token falls back to full prediction.
    pub(crate) default_alt: i32,
    /// The follow states of the members' calls of the shared rule.
    boundary: BTreeSet<i32>,
    /// The states a member reaches right after the shared rule, before its
    /// tail consumes a token, and never after that.
    ended: BTreeSet<i32>,
    /// Whether no member's tail can reach the shared rule again. Only then is
    /// a widened configuration with its bottom segment in the shared rule
    /// inside the first call.
    tails_avoid_shared: bool,
}

impl DescentGroup {
    /// The member alternative a fallback predicts when the adaptive simulation
    /// is not available.
    pub(crate) fn min_alt(&self) -> i32 {
        self.alts.first().copied().unwrap_or(1)
    }

    fn covers(&self, live: &BitSet) -> bool {
        live.iter()
            .all(|alt| self.alts.binary_search(&(alt as i32)).is_ok())
    }

    /// Whether every stack path of a configuration whose state is in rule
    /// `rule` is inside a member's call of the shared rule: some segment of the
    /// path runs in the shared rule and the frame below it returns into a
    /// member's call site (the boundary).
    ///
    /// A widening joint drops the frames below a recursive invocation. For a
    /// `widened` configuration, a path whose bottom segment runs in the shared
    /// rule also passes when no member's tail can reach the shared rule: the
    /// closure then reaches the recursion only through a member's call.
    fn first_consumer_paths(
        &self,
        rule: i32,
        context: &PredictionContextRef<'_>,
        widened: bool,
        memo: &mut HashMap<(*const (), bool, bool), bool>,
    ) -> bool {
        let in_shared = usize::try_from(rule).ok() == Some(self.rule);
        if context.is_empty() {
            return in_shared && widened && self.tails_avoid_shared;
        }
        let key = (
            context.as_ref() as *const _ as *const (),
            in_shared,
            widened,
        );
        if let Some(known) = memo.get(&key) {
            return *known;
        }
        let passes = (0..context.length()).all(|index| {
            let return_state = context.get_return_state(index);
            if return_state == ATNStateRef::invalid() {
                return in_shared && widened && self.tails_avoid_shared;
            }
            if in_shared && self.boundary.contains(&return_state.get_state_number()) {
                return true;
            }
            let parent = context
                .get_parent(index)
                .unwrap_or_else(PredictionContextRef::new_empty);
            self.first_consumer_paths(return_state.get_rule_index(), &parent, widened, memo)
        });
        memo.insert(key, passes);
        passes
    }
}

/// The shared-descent groups of one decision.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct DescentPlan {
    pub(crate) groups: Vec<DescentGroup>,
}

impl DescentPlan {
    /// The group a state accepts with, if any.
    ///
    /// The group must cover the live alternatives, the table must have read
    /// the prefix, and every configuration must be inside a member's call of
    /// the shared rule (see [`DescentGroup::first_consumer_paths`]) or right
    /// after it. A configuration that went on into its member's tail, or left
    /// the decision, read the shared rule with another extent than the
    /// configurations still inside it, and the one parse of the shared rule
    /// cannot serve both readings.
    pub(super) fn group_for(
        &self,
        live: &BitSet,
        depth: usize,
        configs: &[ATNConfig<'_>],
        taint: &Taint,
    ) -> Option<&DescentGroup> {
        self.groups.iter().find(|group| {
            if depth < group.prefix.len() || !group.covers(live) {
                return false;
            }
            let mut memo = HashMap::new();
            configs.iter().all(|config| {
                let taint = taint_of(taint, config);
                taint & BOUNDARY_TAINT == 0
                    && config.get_context().is_some_and(|context| {
                        if context.is_empty() {
                            return group.ended.contains(&config.get_state().get_state_number());
                        }
                        group.first_consumer_paths(
                            config.get_state().get_rule_index(),
                            &context,
                            taint & WIDENED_TAINT != 0,
                            &mut memo,
                        )
                    })
            })
        })
    }
}

/// What the descent analysis knows about each rule of the grammar.
#[derive(Clone, Debug, Default)]
pub(crate) struct DescentRules {
    /// Rules that take arguments: the generated block cannot call them.
    pub(crate) has_arguments: Vec<bool>,
    /// Rules that run an action of their own: `@init`, `@after`, or an
    /// embedded action.
    pub(crate) has_actions: Vec<bool>,
}

/// Builds and caches the shared-descent plans of one ATN.
#[derive(Debug)]
pub(super) struct DescentAnalyzer {
    atn: &'static ATN,
    rules: Option<DescentRules>,
    /// Per rule, whether it can serve as a shared rule.
    shareable: Option<Vec<bool>>,
    /// Per rule, the rules it calls.
    callees: Option<Vec<BTreeSet<usize>>>,
    /// The grammar-wide FOLLOW set of every rule, EOF included.
    follow: Option<Vec<BTreeSet<i32>>>,
    plans: HashMap<usize, Option<DescentPlan>>,
}

impl DescentAnalyzer {
    pub(super) fn new(atn: &'static ATN) -> Self {
        Self {
            atn,
            rules: None,
            shareable: None,
            callees: None,
            follow: None,
            plans: HashMap::new(),
        }
    }

    /// Enables the analysis with the rule facts of the grammar. Without them
    /// no decision has a plan.
    pub(super) fn with_rules(mut self, rules: DescentRules) -> Self {
        self.rules = Some(rules);
        self
    }

    /// The plan of a decision, `None` when it has no codegenable group.
    pub(super) fn plan(&mut self, decision: usize) -> Option<DescentPlan> {
        if let Some(cached) = self.plans.get(&decision) {
            return cached.clone();
        }
        let plan = self.build(decision);
        self.plans.insert(decision, plan.clone());
        plan
    }

    fn build(&mut self, decision: usize) -> Option<DescentPlan> {
        self.rules.as_ref()?;
        let decision_state = *self.atn.decision_to_state.get(decision)?;
        let end_state = block_end_state(self.atn, decision_state)?;
        let transitions = decision_state.get_transitions();
        if transitions.len() < 2 || transitions.len() >= VIRTUAL_ALT_BASE as usize {
            return None;
        }
        // An optional block's skip alternative is an edge straight to the
        // block end; such a block is emitted as an optional block, which has
        // no group arms.
        if transitions
            .iter()
            .any(|transition| transition.get_target() == end_state)
        {
            return None;
        }
        let decision_rule = decision_state.get_rule_index();

        let mut members: BTreeMap<(usize, Vec<i32>), Vec<Member>> = BTreeMap::new();
        for (index, transition) in transitions.iter().enumerate() {
            let alt = index as i32 + 1;
            for (rule, prefix) in candidate_paths(transition.get_target(), decision_rule) {
                if !self.shareable(rule) {
                    continue;
                }
                let Some(path) = uniform_path(transition.get_target(), rule, prefix.len()) else {
                    continue;
                };
                members.entry((rule, prefix)).or_default().push((alt, path));
            }
        }

        let mut groups = Vec::new();
        for ((rule, prefix), members) in members {
            if members.len() < 2 {
                continue;
            }
            let precedence = members[0].1.precedence;
            if members
                .iter()
                .any(|(_, path)| path.precedence != precedence)
            {
                continue;
            }
            let Some(dispatch) = self.dispatch(decision_rule, &members) else {
                continue;
            };
            groups.push(DescentGroup {
                virtual_alt: 0,
                rule,
                alts: members.iter().map(|(alt, _)| *alt).collect(),
                prefix,
                call_site: members[0].1.call_sites[0],
                precedence,
                arms: dispatch.arms,
                default_alt: dispatch.default_alt,
                tails_avoid_shared: !self.tails_reach(
                    rule,
                    members
                        .iter()
                        .flat_map(|(_, path)| path.follow_states.iter().copied()),
                ),
                ended: ended_states(
                    self.atn,
                    members
                        .iter()
                        .flat_map(|(_, path)| path.follow_states.iter().copied()),
                ),
                boundary: members
                    .iter()
                    .flat_map(|(_, path)| path.follow_states.iter().copied())
                    .collect(),
            });
        }
        if groups.is_empty() {
            return None;
        }
        for (index, group) in groups.iter_mut().enumerate() {
            group.virtual_alt = VIRTUAL_ALT_BASE + index as i32;
        }
        Some(DescentPlan { groups })
    }

    /// The tail dispatch of a group, `None` when no token discriminates.
    ///
    /// A member's tail is the FIRST set of its continuation after the shared
    /// rule up to the end of the decision rule; a *nullable* member can reach
    /// the end, so the decision rule's FOLLOW joins its mandatory tail. With
    /// at most one nullable member and mandatory tails that are pairwise
    /// disjoint, each non-nullable member gets its tail as an arm and the
    /// nullable member takes every other token. Otherwise each member's arm is
    /// its tail minus every other member's, and an arm commits only when the
    /// token cannot follow the decision rule on the real stack; every other
    /// token falls back.
    fn dispatch(&mut self, decision_rule: i32, members: &[Member]) -> Option<Dispatch> {
        let mut tails: Vec<(i32, BTreeSet<i32>, bool)> = Vec::with_capacity(members.len());
        for (alt, path) in members {
            let mut tail = BTreeSet::new();
            let mut nullable = false;
            for follow_state in &path.follow_states {
                let (first, reaches_end) = first_set(self.atn, *follow_state);
                tail.extend(first);
                nullable |= reaches_end;
            }
            tails.push((*alt, tail, nullable));
        }
        let nullable: Vec<i32> = tails
            .iter()
            .filter(|(_, tail, nullable)| *nullable || tail.is_empty())
            .map(|(alt, _, _)| *alt)
            .collect();

        if nullable.len() <= 1 {
            let follow = self.follow(decision_rule);
            let default_set: BTreeSet<i32> = match nullable.first() {
                Some(default_alt) => tails
                    .iter()
                    .find(|(alt, _, _)| alt == default_alt)
                    .map(|(_, tail, _)| tail.union(&follow).copied().collect())
                    .unwrap_or_default(),
                None => BTreeSet::new(),
            };
            let mut claimed: BTreeSet<i32> = BTreeSet::new();
            let mut disjoint = true;
            for (alt, tail, _) in &tails {
                if nullable.contains(alt) {
                    continue;
                }
                if tail
                    .iter()
                    .any(|token| claimed.contains(token) || default_set.contains(token))
                {
                    disjoint = false;
                    break;
                }
                claimed.extend(tail.iter().copied());
            }
            if disjoint {
                let arms: Vec<TailArm> = tails
                    .iter()
                    .filter(|(alt, tail, _)| !nullable.contains(alt) && !tail.is_empty())
                    .map(|(alt, tail, _)| TailArm {
                        alt: *alt,
                        tokens: tail.iter().copied().collect(),
                        guarded: false,
                    })
                    .collect();
                let default_alt = nullable.first().copied().unwrap_or(0);
                return (default_alt != 0 || !arms.is_empty())
                    .then_some(Dispatch { arms, default_alt });
            }
        }

        // Guarded dispatch: in-decision tails only, minus every other member. An
        // arm needs the guard when another member is nullable: that member can
        // take the token after the decision rule.
        let mut arms = Vec::new();
        for (alt, tail, _) in &tails {
            let guarded = nullable.iter().any(|other| other != alt);
            let tokens: Vec<i32> = tail
                .iter()
                .copied()
                .filter(|token| {
                    tails
                        .iter()
                        .all(|(other, other_tail, _)| other == alt || !other_tail.contains(token))
                })
                .collect();
            if !tokens.is_empty() {
                arms.push(TailArm {
                    alt: *alt,
                    tokens,
                    guarded,
                });
            }
        }
        (!arms.is_empty()).then_some(Dispatch {
            arms,
            default_alt: 0,
        })
    }

    /// Whether `rule` can be the shared rule of a group.
    fn shareable(&mut self, rule: usize) -> bool {
        if self.shareable.is_none() {
            self.shareable = Some(self.compute_shareable());
        }
        self.shareable
            .as_ref()
            .is_some_and(|shareable| shareable.get(rule).copied().unwrap_or(false))
    }

    /// The rules each rule calls.
    fn callees(&mut self) -> &[BTreeSet<usize>] {
        let atn = self.atn;
        self.callees.get_or_insert_with(|| {
            let num_rules = atn.rule_to_start_state.len();
            let mut callees: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); num_rules];
            for state in atn.iter_states() {
                let Ok(rule) = usize::try_from(state.get_rule_index()) else {
                    continue;
                };
                if rule >= num_rules {
                    continue;
                }
                for transition in state.get_transitions() {
                    if matches!(transition.transition_type(), TransitionType::Rule)
                        && let Ok(callee) =
                            usize::try_from(transition.get_target().get_rule_index())
                    {
                        callees[rule].insert(callee);
                    }
                }
            }
            callees
        })
    }

    /// Whether the tails after `follow_states` can call `shared`, directly or
    /// through other rules.
    fn tails_reach(&mut self, shared: usize, follow_states: impl Iterator<Item = i32>) -> bool {
        let callees = self.callees().to_vec();
        let mut reaches: Vec<bool> = (0..callees.len()).map(|rule| rule == shared).collect();
        let mut changed = true;
        while changed {
            changed = false;
            for rule in 0..callees.len() {
                if !reaches[rule] && callees[rule].iter().any(|callee| reaches[*callee]) {
                    reaches[rule] = true;
                    changed = true;
                }
            }
        }
        let mut seen = BTreeSet::new();
        let mut work: Vec<ATNStateRef> = follow_states
            .map(|state| self.atn.make_state_ref(state))
            .collect();
        while let Some(state) = work.pop() {
            if !seen.insert(state.get_state_number())
                || state.state_type() == ATNStateType::RuleStop
            {
                continue;
            }
            for transition in state.get_transitions() {
                if let Some(call) = transition.try_as::<RuleTransition>() {
                    let callee = usize::try_from(call.get_target().get_rule_index()).ok();
                    // In a tail of the shared rule itself, any call returns
                    // into the shared rule.
                    let caller = usize::try_from(state.get_rule_index()).ok();
                    if caller == Some(shared)
                        || callee
                            .is_some_and(|callee| reaches.get(callee).copied().unwrap_or(false))
                    {
                        return true;
                    }
                    work.push(call.follow_state);
                } else {
                    work.push(transition.get_target());
                }
            }
        }
        false
    }

    /// A rule is shareable when it takes no arguments and no rule reachable
    /// from it (itself included) has an action.
    fn compute_shareable(&mut self) -> Vec<bool> {
        let num_rules = self.atn.rule_to_start_state.len();
        let rules = self.rules.clone().unwrap_or_default();
        let mut acts: Vec<bool> = (0..num_rules)
            .map(|rule| rules.has_actions.get(rule).copied().unwrap_or(false))
            .collect();
        let callees = self.callees().to_vec();
        // A rule acts when any rule it reaches acts: propagate to a fixpoint.
        let mut changed = true;
        while changed {
            changed = false;
            for rule in 0..num_rules {
                if !acts[rule] && callees[rule].iter().any(|callee| acts[*callee]) {
                    acts[rule] = true;
                    changed = true;
                }
            }
        }
        (0..num_rules)
            .map(|rule| !acts[rule] && !rules.has_arguments.get(rule).copied().unwrap_or(false))
            .collect()
    }

    /// The grammar-wide FOLLOW set of `rule`, EOF included: any rule can be
    /// the start rule of a parse.
    fn follow(&mut self, rule: i32) -> BTreeSet<i32> {
        if self.follow.is_none() {
            self.follow = Some(compute_follow(self.atn));
        }
        usize::try_from(rule)
            .ok()
            .and_then(|rule| self.follow.as_ref()?.get(rule).cloned())
            .unwrap_or_else(|| BTreeSet::from([EOF]))
    }
}

/// The tail dispatch of a group.
struct Dispatch {
    arms: Vec<TailArm>,
    default_alt: i32,
}

/// One member alternative of a candidate group and its paths to the shared
/// rule.
type Member = (i32, MemberPath);

/// The calls of the shared rule that the paths of one member reach.
#[derive(Clone, Debug)]
struct MemberPath {
    call_sites: Vec<i32>,
    follow_states: BTreeSet<i32>,
    precedence: i32,
}

/// The end state of a block decision, `None` for other decisions.
fn block_end_state(atn: &ATN, state: ATNStateRef) -> Option<ATNStateRef> {
    let state = atn.get_state(state.get_state_number());
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

/// The `(shared rule, prefix)` pairs the paths of one alternative reach: the
/// rule calls reachable inside the decision rule after at most
/// [`MAX_PREFIX_TOKENS`] tokens, with no rule call and no predicate before
/// them.
fn candidate_paths(start: ATNStateRef, decision_rule: i32) -> BTreeSet<(usize, Vec<i32>)> {
    let mut out = BTreeSet::new();
    let mut seen: BTreeSet<(i32, Vec<i32>)> = BTreeSet::new();
    let mut work: Vec<(ATNStateRef, Vec<i32>)> = vec![(start, Vec::new())];
    while let Some((state, prefix)) = work.pop() {
        if state.get_rule_index() != decision_rule
            || state.state_type() == ATNStateType::RuleStop
            || !seen.insert((state.get_state_number(), prefix.clone()))
        {
            continue;
        }
        for transition in state.get_transitions() {
            match transition.transition_type() {
                TransitionType::Rule => {
                    if let Ok(rule) = usize::try_from(transition.get_target().get_rule_index()) {
                        out.insert((rule, prefix.clone()));
                    }
                }
                TransitionType::Predicate | TransitionType::PrecedencePredicate => {}
                TransitionType::Epsilon | TransitionType::Action => {
                    work.push((transition.get_target(), prefix.clone()));
                }
                TransitionType::Atom | TransitionType::Range | TransitionType::Set => {
                    if prefix.len() >= MAX_PREFIX_TOKENS {
                        continue;
                    }
                    let Some(label) = transition.get_label() else {
                        continue;
                    };
                    let tokens: Vec<i32> = label
                        .as_slice()
                        .iter()
                        .flat_map(|interval| interval.a..=interval.b)
                        .take(MAX_PREFIX_LABEL + 1)
                        .collect();
                    if tokens.len() > MAX_PREFIX_LABEL {
                        continue;
                    }
                    for token in tokens {
                        let mut next = prefix.clone();
                        next.push(token);
                        work.push((transition.get_target(), next));
                    }
                }
                TransitionType::NotSet | TransitionType::Wildcard => {}
            }
        }
    }
    out
}

/// The call of `rule` that the path from `start` reaches after exactly
/// `prefix_len` tokens, or `None` when the path consumes another number of
/// tokens first, calls another rule, crosses a predicate, a decision, or a
/// loop, or leaves the alternative without calling `rule`.
fn uniform_path(start: ATNStateRef, rule: usize, prefix_len: usize) -> Option<MemberPath> {
    let mut call_sites = BTreeSet::new();
    let mut follow_states = BTreeSet::new();
    let mut precedence = None;
    let mut seen: BTreeSet<(i32, usize)> = BTreeSet::new();
    let mut work = vec![(start, 0_usize)];
    while let Some((state, consumed)) = work.pop() {
        if !seen.insert((state.get_state_number(), consumed)) {
            continue;
        }
        // A decision or a loop before the call: the re-descent would predict
        // there, or call the shared rule more than once.
        if state.state_type() == ATNStateType::RuleStop
            || state.get_transitions().len() != 1
            || matches!(
                state.state_type(),
                ATNStateType::PlusBlockStart
                    | ATNStateType::StarBlockStart
                    | ATNStateType::StarLoopEntry
            )
        {
            return None;
        }
        for transition in state.get_transitions() {
            match transition.transition_type() {
                TransitionType::Rule => {
                    let call = transition.try_as::<RuleTransition>()?;
                    let callee = usize::try_from(call.get_target().get_rule_index()).ok();
                    if consumed != prefix_len || callee != Some(rule) {
                        return None;
                    }
                    if *precedence.get_or_insert(call.precedence()) != call.precedence() {
                        return None;
                    }
                    call_sites.insert(state.get_state_number());
                    follow_states.insert(call.follow_state.get_state_number());
                }
                TransitionType::Predicate | TransitionType::PrecedencePredicate => return None,
                TransitionType::Epsilon | TransitionType::Action => {
                    work.push((transition.get_target(), consumed));
                }
                _ => {
                    if consumed >= prefix_len {
                        return None;
                    }
                    work.push((transition.get_target(), consumed + 1));
                }
            }
        }
    }
    Some(MemberPath {
        call_sites: call_sites.into_iter().collect(),
        follow_states,
        precedence: precedence?,
    })
}

/// The states that the follow states of a group's shared-rule calls reach
/// without a token, minus the states their tails also reach after a token.
/// Only a configuration in such a state ended the shared rule at the current
/// lookahead position.
fn ended_states(atn: &ATN, follow_states: impl Iterator<Item = i32>) -> BTreeSet<i32> {
    let starts: Vec<ATNStateRef> = follow_states
        .map(|state| atn.make_state_ref(state))
        .collect();
    let mut without_token = BTreeSet::new();
    let mut after_token = BTreeSet::new();
    let mut work: Vec<(ATNStateRef, bool)> = starts.iter().map(|state| (*state, false)).collect();
    while let Some((state, consumed)) = work.pop() {
        let seen = if consumed {
            &mut after_token
        } else {
            &mut without_token
        };
        if !seen.insert(state.get_state_number()) || state.state_type() == ATNStateType::RuleStop {
            continue;
        }
        for transition in state.get_transitions() {
            match transition.transition_type() {
                // A rule call can consume; its return is the follow state.
                TransitionType::Rule => {
                    if let Some(call) = transition.try_as::<RuleTransition>() {
                        work.push((call.follow_state, true));
                    }
                }
                _ if transition.is_epsilon() => work.push((transition.get_target(), consumed)),
                _ => work.push((transition.get_target(), true)),
            }
        }
    }
    without_token.difference(&after_token).copied().collect()
}

/// The FIRST set of the continuation at `state` up to the end of its rule,
/// and whether the continuation can reach the end without a token.
fn first_set(atn: &ATN, state_number: i32) -> (BTreeSet<i32>, bool) {
    let state = atn.make_state_ref(state_number);
    let tokens = atn.next_tokens::<CommonToken<'static>>(&state);
    let mut first = BTreeSet::new();
    for interval in tokens.as_slice() {
        first.extend((interval.a..=interval.b).filter(|token| *token != TOKEN_EPSILON));
    }
    (first, tokens.contains(TOKEN_EPSILON))
}

/// The grammar-wide FOLLOW set of every rule: what the continuations of its
/// call sites start with, through the callers' FOLLOW where a continuation can
/// complete, plus EOF.
fn compute_follow(atn: &ATN) -> Vec<BTreeSet<i32>> {
    let num_rules = atn.rule_to_start_state.len();
    let mut follow: Vec<BTreeSet<i32>> = vec![BTreeSet::from([EOF]); num_rules];
    // (callee, caller, FIRST of the caller's continuation, it can complete)
    let mut calls: Vec<(usize, usize, BTreeSet<i32>, bool)> = Vec::new();
    for state in atn.iter_states() {
        let Ok(caller) = usize::try_from(state.get_rule_index()) else {
            continue;
        };
        for transition in state.get_transitions() {
            let Some(call) = transition.try_as::<RuleTransition>() else {
                continue;
            };
            let Ok(callee) = usize::try_from(call.get_target().get_rule_index()) else {
                continue;
            };
            let (first, reaches_end) = first_set(atn, call.follow_state.get_state_number());
            calls.push((callee, caller, first, reaches_end));
        }
    }
    let mut changed = true;
    while changed {
        changed = false;
        for (callee, caller, first, reaches_end) in &calls {
            if *callee >= num_rules || *caller >= num_rules {
                continue;
            }
            let mut added: Vec<i32> = first
                .iter()
                .copied()
                .filter(|token| !follow[*callee].contains(token))
                .collect();
            if *reaches_end {
                added.extend(
                    follow[*caller]
                        .iter()
                        .copied()
                        .filter(|token| !follow[*callee].contains(token)),
                );
            }
            if !added.is_empty() {
                follow[*callee].extend(added);
                changed = true;
            }
        }
    }
    follow
}
