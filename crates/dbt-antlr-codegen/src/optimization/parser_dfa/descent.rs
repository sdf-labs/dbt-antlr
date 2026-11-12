// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Shared-descent groups: alternatives that all start with the same rule.
//!
//! When every member alternative of a group consumes the same number of leading
//! tokens (`0..=2`, the *prefix*) and then calls the same *shared rule* `R`
//! directly, a table state whose live alternatives are all members does not
//! have to escape. It accepts with the group's virtual alternative instead. The
//! generated block parses `R` once with muted parse listeners, reads the token
//! after it, and dispatches to the member alternative whose tail starts with
//! that token. The member then re-descends through its prefix, and its call of
//! `R` takes the subtree of the first parse.
//!
//! A group is generated only when the runtime neutral parse **never needs to be
//! reverted**:
//!
//! 1. **The shared prefix is the only path forward at the state and
//!    lookaheads.** A member is one straight path from its start to its `R`
//!    call inside the decision rule: the prefix tokens, and no other rule call,
//!    predicate, decision, or loop. Every member calls `R` with the same
//!    precedence. At a group-accept state, the table has read at least the
//!    prefix, every live alternative is a member of the group, and every live
//!    configuration is inside a member's call of `R` or right at its exit
//!    ([`DescentPlan::group_for`]).
//! 2. **The member follow sets disambiguate the members at the shared call
//!    site, and the neutral parse of `R` has no conflict or ambiguity with
//!    them.** At most one member can complete the decision rule without a
//!    token, the members' follow sets at the `R` call site are pairwise
//!    disjoint, and no derivation of `R` can both exit `R` and continue inside
//!    `R` on any token of those follow sets (see [`DescentAnalyzer::dispatch`]
//!    and [`DescentAnalyzer::exit_continue_tokens`]).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::OnceLock;

use dbt_antlr_runtime::analysis::{
    ATN, ATNConfig, ATNStateRef, ATNStateType, BOUNDARY_TAINT, BasicBlockStartState, BitSet, EOF,
    IntervalSet, IntervalSetBuf, NotSetTransition, PlusBlockStartState, PredictionContextRef,
    RuleTransition, StarBlockStartState, TOKEN_MIN_USER_TOKEN_TYPE, Transition, TransitionType,
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
    /// The call-site states of the members' calls of the shared rule, ascending.
    pub(crate) call_sites: Vec<i32>,
    /// The precedence argument of every member's call of the shared rule.
    pub(crate) precedence: i32,
    /// The tail dispatch.
    pub(crate) arms: Vec<TailArm>,
    /// The nullable member that every token without an explicit arm selects, or
    /// `0` when every member has a mandatory tail token.
    pub(crate) default_alt: i32,
    /// The follow states of the members' calls of the shared rule.
    boundary: BTreeSet<i32>,
    /// The states a member reaches right after the shared rule, before its
    /// tail consumes a token, and never after that.
    ended: BTreeSet<i32>,
}

impl DescentGroup {
    /// The lowest-numbered member alternative of the group.
    pub(crate) fn min_alt(&self) -> i32 {
        self.alts.first().copied().unwrap_or(1)
    }

    /// The member alternative that the `_` arm of the runtime tail match
    /// selects: the nullable member when the group has one, or the minimum
    /// member when a syntax error follows the shared rule.
    pub(crate) fn default_or_min_alt(&self) -> i32 {
        if self.default_alt > 0 {
            self.default_alt
        } else {
            self.min_alt()
        }
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
    /// A widening joint drops the frames below a recursive invocation. Because
    /// the members' follow sets at the shared call site are pairwise disjoint
    /// and the shared rule cannot continue on any of them where it can exit, no
    /// two members can both survive a token after the shared rule; a widened
    /// configuration whose bottom segment runs in the shared rule is therefore
    /// still inside the first call.
    fn first_consumer_paths(
        &self,
        rule: i32,
        context: &PredictionContextRef<'_>,
        widened: bool,
        memo: &mut HashMap<(*const (), bool, bool), bool>,
    ) -> bool {
        let in_shared = usize::try_from(rule).ok() == Some(self.rule);
        if context.is_empty() {
            return in_shared && widened;
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
            if return_state == ATNStateRef::empty_return() {
                return in_shared && widened;
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
    shareable: OnceLock<Vec<bool>>,
    /// The grammar-wide FOLLOW set of every rule, EOF included.
    follow: OnceLock<Vec<BTreeSet<i32>>>,
    /// Per rule `R`, the tokens that a derivation of `R` can consume at a
    /// point where `R` can also exit.
    exit_continue: Box<[OnceLock<BTreeSet<i32>>]>,
    /// Per decision, its plan.
    plans: Box<[OnceLock<Option<DescentPlan>>]>,
}

impl DescentAnalyzer {
    pub(super) fn new(atn: &'static ATN) -> Self {
        Self {
            atn,
            rules: None,
            shareable: OnceLock::new(),
            follow: OnceLock::new(),
            exit_continue: (0..atn.rule_to_start_state.len())
                .map(|_| OnceLock::new())
                .collect(),
            plans: (0..atn.decision_to_state.len())
                .map(|_| OnceLock::new())
                .collect(),
        }
    }

    /// Enables the analysis with the rule facts of the grammar. Without them
    /// no decision has a plan.
    pub(super) fn with_rules(mut self, rules: DescentRules) -> Self {
        self.rules = Some(rules);
        self
    }

    /// The plan of a decision, `None` when it has no codegenable group.
    pub(super) fn plan(&self, decision: usize) -> Option<DescentPlan> {
        self.plans
            .get(decision)?
            .get_or_init(|| self.build(decision))
            .clone()
    }

    fn build(&self, decision: usize) -> Option<DescentPlan> {
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
                let Some(path) = uniform_path(transition.get_target(), rule, &prefix) else {
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
            let Some(dispatch) = self.dispatch(decision_rule, rule, &members) else {
                continue;
            };
            groups.push(DescentGroup {
                virtual_alt: 0,
                rule,
                alts: members.iter().map(|(alt, _)| *alt).collect(),
                prefix,
                call_sites: members
                    .iter()
                    .flat_map(|(_, path)| path.call_sites.iter().copied())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                precedence,
                arms: dispatch.arms,
                default_alt: dispatch.default_alt,
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

    /// The tail dispatch of a group, `None` when the member follow sets do not
    /// disambiguate the members or when the shared rule can continue on a token
    /// of those follow sets at a point where it can exit.
    fn dispatch(
        &self,
        decision_rule: i32,
        shared_rule: usize,
        members: &[Member],
    ) -> Option<Dispatch> {
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
        if nullable.len() > 1 {
            return None;
        }

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
        for (alt, tail, _) in &tails {
            if nullable.contains(alt) {
                continue;
            }
            if tail
                .iter()
                .any(|token| claimed.contains(token) || default_set.contains(token))
            {
                return None;
            }
            claimed.extend(tail.iter().copied());
        }

        let mut call_site_follow = claimed;
        call_site_follow.extend(default_set);
        if !self
            .exit_continue_tokens(shared_rule)
            .is_disjoint(&call_site_follow)
        {
            return None;
        }

        let arms: Vec<TailArm> = tails
            .iter()
            .filter(|(alt, tail, _)| !nullable.contains(alt) && !tail.is_empty())
            .map(|(alt, tail, _)| TailArm {
                alt: *alt,
                tokens: tail.iter().copied().collect(),
            })
            .collect();
        let default_alt = nullable.first().copied().unwrap_or(0);
        (default_alt != 0 || !arms.is_empty()).then_some(Dispatch { arms, default_alt })
    }

    /// The tokens that a derivation of `shared_rule` can consume at a point
    /// where `shared_rule` can also exit.
    fn exit_continue_tokens(&self, shared_rule: usize) -> &BTreeSet<i32> {
        self.exit_continue[shared_rule].get_or_init(|| compute_exit_continue(self.atn, shared_rule))
    }

    /// Whether `rule` can be the shared rule of a group.
    fn shareable(&self, rule: usize) -> bool {
        self.shareable
            .get_or_init(|| self.compute_shareable())
            .get(rule)
            .copied()
            .unwrap_or(false)
    }

    /// The rules each rule calls.
    fn callees(&self) -> Vec<BTreeSet<usize>> {
        let num_rules = self.atn.rule_to_start_state.len();
        let mut callees: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); num_rules];
        for state in self.atn.iter_states() {
            let Ok(rule) = usize::try_from(state.get_rule_index()) else {
                continue;
            };
            if rule >= num_rules {
                continue;
            }
            for transition in state.get_transitions() {
                if matches!(transition.transition_type(), TransitionType::Rule)
                    && let Ok(callee) = usize::try_from(transition.get_target().get_rule_index())
                {
                    callees[rule].insert(callee);
                }
            }
        }
        callees
    }

    /// A rule is shareable when it takes no arguments and no rule reachable
    /// from it (itself included) has an action or a semantic predicate.
    fn compute_shareable(&self) -> Vec<bool> {
        let num_rules = self.atn.rule_to_start_state.len();
        let rules = self.rules.clone().unwrap_or_default();
        let mut tainted: Vec<bool> = (0..num_rules)
            .map(|rule| rules.has_actions.get(rule).copied().unwrap_or(false))
            .collect();
        for state in self.atn.iter_states() {
            let Ok(rule) = usize::try_from(state.get_rule_index()) else {
                continue;
            };
            if rule < num_rules
                && state.get_transitions().iter().any(|transition| {
                    matches!(transition.transition_type(), TransitionType::Predicate)
                })
            {
                tainted[rule] = true;
            }
        }
        let callees = self.callees();
        let mut changed = true;
        while changed {
            changed = false;
            for rule in 0..num_rules {
                if !tainted[rule] && callees[rule].iter().any(|callee| tainted[*callee]) {
                    tainted[rule] = true;
                    changed = true;
                }
            }
        }
        (0..num_rules)
            .map(|rule| !tainted[rule] && !rules.has_arguments.get(rule).copied().unwrap_or(false))
            .collect()
    }

    /// The grammar-wide FOLLOW set of `rule`, EOF included: any rule can be
    /// the start rule of a parse.
    fn follow(&self, rule: i32) -> BTreeSet<i32> {
        usize::try_from(rule)
            .ok()
            .and_then(|rule| {
                self.follow
                    .get_or_init(|| compute_follow(self.atn))
                    .get(rule)
                    .cloned()
            })
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

/// The call of `rule` that the path from `start` reaches after the exact
/// single-token `prefix`, or `None` when the path consumes other tokens,
/// calls another rule, crosses a predicate, a decision, or a loop, or leaves
/// the alternative without calling `rule`.
fn uniform_path(start: ATNStateRef, rule: usize, prefix: &[i32]) -> Option<MemberPath> {
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
                    if consumed != prefix.len() || callee != Some(rule) {
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
                TransitionType::Atom | TransitionType::Range | TransitionType::Set => {
                    let expected = *prefix.get(consumed)?;
                    let label = transition.get_label()?;
                    if label.length() != 1 || !label.contains(expected) {
                        return None;
                    }
                    work.push((transition.get_target(), consumed + 1));
                }
                _ => return None,
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

/// Whether `state` can reach the stop state of its rule without consuming a
/// token.
fn nullable_to_stop(atn: &ATN, state: ATNStateRef) -> bool {
    state.state_type() == ATNStateType::RuleStop
        || atn
            .next_tokens::<CommonToken<'static>>(&state)
            .contains(TOKEN_EPSILON)
}
/// Whether `left` and `right` share any token type other than
/// [`TOKEN_EPSILON`].
fn intervals_intersect(left: &IntervalSet, right: &IntervalSet) -> bool {
    let left = left.as_slice();
    let right = right.as_slice();
    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        let a = left[i].a.max(right[j].a);
        let b = left[i].b.min(right[j].b);
        if a <= b && (a != TOKEN_EPSILON || b != TOKEN_EPSILON) {
            return true;
        }
        if left[i].b < right[j].b {
            i += 1;
        } else {
            j += 1;
        }
    }
    false
}

/// Adds the token types of a non-epsilon, non-rule transition into `out`.
fn add_transition_tokens(atn: &ATN, transition: &Transition, out: &mut IntervalSetBuf) {
    match transition.transition_type() {
        TransitionType::NotSet => {
            if let Some(not_set) = transition.try_as::<NotSetTransition>()
                && let Some(set) = not_set.get_label()
            {
                let comp = set.complement(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type);
                out.add_set(comp.as_interval_set());
            }
        }
        TransitionType::Wildcard => {
            out.add_range(TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type);
        }
        TransitionType::Atom | TransitionType::Range | TransitionType::Set => {
            if let Some(label) = transition.get_label() {
                out.add_set(label);
            }
        }
        _ => {}
    }
}

/// The consuming moves reachable from a state across epsilon transitions,
/// nullable rule calls, and tail-position rule entries.
struct StepClosure {
    reaches_stop: bool,
    /// Target state number -> union of token labels reaching it.
    token_moves: BTreeMap<i32, IntervalSetBuf>,
    /// `(callee_start, follow_state)` of reachable rule transitions.
    rule_moves: Vec<(ATNStateRef, ATNStateRef)>,
}

fn step_closure<'a>(
    atn: &ATN,
    start: ATNStateRef,
    cache: &'a mut HashMap<i32, StepClosure>,
) -> &'a StepClosure {
    let number = start.get_state_number();
    cache.entry(number).or_insert_with(|| {
        let mut token_moves: BTreeMap<i32, IntervalSetBuf> = BTreeMap::new();
        let mut rule_moves = Vec::new();
        let mut seen_rules = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut work = vec![start];
        while let Some(state) = work.pop() {
            if !seen.insert(state.get_state_number())
                || state.state_type() == ATNStateType::RuleStop
            {
                continue;
            }
            for transition in state.get_transitions() {
                if let Some(call) = transition.try_as::<RuleTransition>() {
                    let key = (
                        call.get_target().get_state_number(),
                        call.follow_state.get_state_number(),
                    );
                    if seen_rules.insert(key) {
                        rule_moves.push((call.get_target(), call.follow_state));
                    }
                    if nullable_to_stop(atn, call.get_target()) {
                        work.push(call.follow_state);
                    }
                } else if transition.is_epsilon() {
                    work.push(transition.get_target());
                } else {
                    add_transition_tokens(
                        atn,
                        transition,
                        token_moves
                            .entry(transition.get_target().get_state_number())
                            .or_default(),
                    );
                }
            }
        }
        StepClosure {
            reaches_stop: nullable_to_stop(atn, start),
            token_moves,
            rule_moves,
        }
    })
}

/// Computes the tokens that a derivation of `shared_rule` can consume at a
/// point where `shared_rule` can also exit.
///
/// Two patterns produce such a token:
///
/// 1. A state in `shared_rule` (or in a rule reachable through tail calls of
///    `shared_rule`) can reach its rule's stop state without a token, and can
///    also consume a token (loops `( ... )*`, `( ... )+`, left-recursive
///    operator loops, and trailing optional blocks `( ... )?`).
/// 2. Two alternatives of a decision in `shared_rule` (or a tail-called rule)
///    match a common prefix after which one alternative can exit the rule while
///    the other continues inside it (`ID '+' ID | ID`, `e 'as' ID | e`).
fn compute_exit_continue(atn: &ATN, shared_rule: usize) -> BTreeSet<i32> {
    let mut tail_rules = BTreeSet::from([shared_rule]);
    let mut work = vec![shared_rule];
    while let Some(rule) = work.pop() {
        for state in atn.iter_states() {
            if usize::try_from(state.get_rule_index()).ok() != Some(rule) {
                continue;
            }
            for transition in state.get_transitions() {
                let Some(call) = transition.try_as::<RuleTransition>() else {
                    continue;
                };
                if !nullable_to_stop(atn, call.follow_state) {
                    continue;
                }
                if let Ok(callee) = usize::try_from(call.get_target().get_rule_index())
                    && tail_rules.insert(callee)
                {
                    work.push(callee);
                }
            }
        }
    }

    let mut out = BTreeSet::new();
    for state in atn.iter_states() {
        let Ok(rule) = usize::try_from(state.get_rule_index()) else {
            continue;
        };
        if !tail_rules.contains(&rule) {
            continue;
        }
        let state_ref = atn.make_state_ref(state.get_state_number());
        let tokens = atn.next_tokens::<CommonToken<'static>>(&state_ref);
        if tokens.contains(TOKEN_EPSILON) {
            for interval in tokens.as_slice() {
                out.extend((interval.a..=interval.b).filter(|token| *token != TOKEN_EPSILON));
            }
        }
    }

    let mut closures: HashMap<i32, StepClosure> = HashMap::new();
    let mut memo: HashMap<(i32, i32), BTreeSet<i32>> = HashMap::new();
    let mut visiting: BTreeSet<(i32, i32)> = BTreeSet::new();
    for decision_state in &atn.decision_to_state {
        let Ok(rule) = usize::try_from(decision_state.get_rule_index()) else {
            continue;
        };
        if !tail_rules.contains(&rule) {
            continue;
        }
        let transitions = decision_state.get_transitions();
        let nexts: Vec<&IntervalSet> = transitions
            .iter()
            .map(|transition| {
                let target = transition.get_target();
                atn.next_tokens::<CommonToken<'static>>(&target)
            })
            .collect();
        for (i, left) in transitions.iter().enumerate() {
            for (j, right) in transitions.iter().enumerate() {
                if i == j || !intervals_intersect(nexts[i], nexts[j]) {
                    continue;
                }
                for state in co_reachable_on_exit(
                    atn,
                    left.get_target(),
                    right.get_target(),
                    &mut closures,
                    &mut memo,
                    &mut visiting,
                ) {
                    let state_ref = atn.make_state_ref(state);
                    for interval in atn
                        .next_tokens::<CommonToken<'static>>(&state_ref)
                        .as_slice()
                    {
                        out.extend(
                            (interval.a..=interval.b).filter(|token| *token != TOKEN_EPSILON),
                        );
                    }
                }
            }
        }
    }
    out
}

/// The states that `start2` can reach over a common prefix on which `start1`
/// reaches a tail-nullable state of its rule.
fn co_reachable_on_exit(
    atn: &ATN,
    start1: ATNStateRef,
    start2: ATNStateRef,
    closures: &mut HashMap<i32, StepClosure>,
    memo: &mut HashMap<(i32, i32), BTreeSet<i32>>,
    visiting: &mut BTreeSet<(i32, i32)>,
) -> BTreeSet<i32> {
    let key = (start1.get_state_number(), start2.get_state_number());
    if let Some(cached) = memo.get(&key) {
        return cached.clone();
    }
    if !visiting.insert(key) {
        return BTreeSet::new();
    }
    let mut out = BTreeSet::new();
    let mut seen: BTreeSet<(i32, i32)> = BTreeSet::new();
    let mut work = vec![(start1, start2)];
    while let Some((s1, s2)) = work.pop() {
        if s1 == s2 || !seen.insert((s1.get_state_number(), s2.get_state_number())) {
            continue;
        }
        let _ = step_closure(atn, s1, closures);
        let _ = step_closure(atn, s2, closures);
        let c1 = &closures[&s1.get_state_number()];
        let c2 = &closures[&s2.get_state_number()];
        if c1.reaches_stop {
            out.insert(s2.get_state_number());
        }

        let next1 = atn.next_tokens::<CommonToken<'static>>(&s1);
        let next2 = atn.next_tokens::<CommonToken<'static>>(&s2);
        for (&target1, label1) in &c1.token_moves {
            for (&target2, label2) in &c2.token_moves {
                if target1 != target2
                    && intervals_intersect(label1.as_interval_set(), label2.as_interval_set())
                {
                    work.push((atn.make_state_ref(target1), atn.make_state_ref(target2)));
                }
            }
        }
        let mut non_tail1 = Vec::new();
        for &(callee1, follow1) in &c1.rule_moves {
            for &(callee2, follow2) in &c2.rule_moves {
                if callee1 == callee2 && follow1 != follow2 {
                    work.push((follow1, follow2));
                }
            }
            let first1 = atn.next_tokens::<CommonToken<'static>>(&callee1);
            if !intervals_intersect(first1, next2) {
                continue;
            }
            if nullable_to_stop(atn, follow1) {
                work.push((callee1, s2));
            } else {
                non_tail1.push((callee1, follow1));
            }
        }
        let mut non_tail2 = Vec::new();
        for &(callee2, follow2) in &c2.rule_moves {
            let first2 = atn.next_tokens::<CommonToken<'static>>(&callee2);
            if !intervals_intersect(first2, next1) {
                continue;
            }
            if nullable_to_stop(atn, follow2) {
                work.push((s1, callee2));
            } else {
                non_tail2.push((callee2, follow2));
            }
        }
        for (callee1, follow1) in non_tail1 {
            for after2 in co_reachable_on_exit(atn, callee1, s2, closures, memo, visiting) {
                work.push((follow1, atn.make_state_ref(after2)));
            }
        }
        for (callee2, follow2) in non_tail2 {
            for after1 in co_reachable_on_exit(atn, callee2, s1, closures, memo, visiting) {
                work.push((atn.make_state_ref(after1), follow2));
            }
        }
    }
    visiting.remove(&key);
    memo.insert(key, out.clone());
    out
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
