// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::rc::Rc;

use petgraph::algo::tarjan_scc;
use petgraph::graph::DiGraph;

use super::super::diagnostic::{CompilationError, Diagnostic, Severity};
use super::super::model::{BuildStateId, RuleId, SemanticGrammar};
use super::build::{FinalizedAtnGraph, FinalizedTransition, FinalizedTransitionKind};
use super::state_kind::AtnStateKind;

const EOF_TOKEN_TYPE: i32 = -1;
const MIN_USER_TOKEN_TYPE: i32 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct DecisionLookahead {
    pub(crate) state: usize,
    pub(crate) alternatives: Vec<Option<Vec<(i32, i32)>>>,
    pub(crate) disjoint: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ParserAnalysis {
    pub(crate) decision_lookahead: Vec<DecisionLookahead>,
    pub(crate) nullable_rules: BTreeSet<RuleId>,
    pub(crate) recursive_components: Vec<Vec<RuleId>>,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ParserCheckSite {
    pub(super) rule: RuleId,
    pub(super) start: usize,
    pub(super) stop: usize,
}

#[derive(Clone, Debug, Default)]
pub(super) struct ParserCheckSites {
    pub(super) closures: Vec<ParserCheckSite>,
    pub(super) optionals: Vec<ParserCheckSite>,
}

impl ParserCheckSites {
    pub(super) fn compact(
        graph: &FinalizedAtnGraph,
        closures: impl IntoIterator<Item = (RuleId, BuildStateId, BuildStateId)>,
        optionals: impl IntoIterator<Item = (RuleId, BuildStateId, BuildStateId)>,
    ) -> Self {
        Self {
            closures: compact_sites(graph, closures),
            optionals: compact_sites(graph, optionals),
        }
    }
}

fn compact_sites(
    graph: &FinalizedAtnGraph,
    sites: impl IntoIterator<Item = (RuleId, BuildStateId, BuildStateId)>,
) -> Vec<ParserCheckSite> {
    sites
        .into_iter()
        .filter_map(|(rule, start, stop)| {
            Some(ParserCheckSite {
                rule,
                start: *graph.state_map.get(&start)?,
                stop: *graph.state_map.get(&stop)?,
            })
        })
        .collect()
}

pub(crate) fn analyze_parser(
    grammar: &SemanticGrammar,
    graph: &FinalizedAtnGraph,
    check_sites: &ParserCheckSites,
) -> Result<ParserAnalysis, CompilationError> {
    let analyzer = LookAnalyzer::new(graph);
    let mut diagnostics = epsilon_diagnostics(grammar, check_sites, &analyzer);
    if has_errors(&diagnostics) {
        return Err(CompilationError::new(diagnostics));
    }

    let nullable_rule_indices = nullable_rule_indices(graph);
    let recursive_components = left_recursive_components(graph, &nullable_rule_indices);
    diagnostics.extend(recursion_diagnostics(grammar, &recursive_components));
    if has_errors(&diagnostics) {
        return Err(CompilationError::new(diagnostics));
    }

    let nullable_rules = nullable_rule_indices
        .iter()
        .filter_map(|index| {
            graph
                .rule_starts
                .get(*index)
                .and_then(|state| graph.states[*state].rule)
        })
        .collect();
    let decision_lookahead = decision_lookahead(graph, &analyzer);
    Ok(ParserAnalysis {
        decision_lookahead,
        nullable_rules,
        recursive_components,
        diagnostics,
    })
}

fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics
        .iter()
        .any(|diagnostic| diagnostic.severity == Severity::Error)
}

fn epsilon_diagnostics(
    grammar: &SemanticGrammar,
    check_sites: &ParserCheckSites,
    analyzer: &LookAnalyzer<'_>,
) -> Vec<Diagnostic> {
    let rules = grammar
        .unit
        .rules
        .iter()
        .map(|rule| (rule.id, rule))
        .collect::<BTreeMap<_, _>>();
    let mut diagnostics = Vec::new();
    for site in &check_sites.closures {
        let look = analyzer.look(site.start, Some(site.stop), true, true);
        let rule = rules[&site.rule];
        if look.epsilon {
            let (code, message) = if rule.left_recursion.is_some() {
                (
                    "G4A002",
                    format!(
                        "left recursive rule {} contains a left recursive alternative which can be followed by the empty string",
                        rule.name
                    ),
                )
            } else {
                (
                    "G4A001",
                    format!(
                        "rule {} contains a closure whose body can match an empty string",
                        rule.name
                    ),
                )
            };
            diagnostics.push(Diagnostic::error(code, rule.span.clone(), message));
        }
        if look.contains(EOF_TOKEN_TYPE) {
            diagnostics.push(Diagnostic::error(
                "G4A003",
                rule.span.clone(),
                format!(
                    "rule {} contains a closure whose body can match EOF",
                    rule.name
                ),
            ));
        }
    }

    for site in &check_sites.optionals {
        let mut bypasses = 0;
        let mut warned = false;
        for transition in analyzer.transitions_from(site.start) {
            if transition.target == site.stop {
                bypasses += 1;
                continue;
            }
            let look = analyzer.look(transition.target, Some(site.stop), true, true);
            if look.epsilon {
                let rule = rules[&site.rule];
                diagnostics.push(Diagnostic::warning(
                    "G4A004",
                    rule.span.clone(),
                    format!(
                        "rule {} contains an optional block with an alternative that can match an empty string",
                        rule.name
                    ),
                ));
                warned = true;
                break;
            }
        }
        if !warned {
            debug_assert_eq!(
                bypasses,
                1,
                "optional block must have one bypass: site={site:?}, targets={:?}",
                analyzer
                    .transitions_from(site.start)
                    .map(|transition| transition.target)
                    .collect::<Vec<_>>()
            );
        }
    }
    diagnostics
}

fn nullable_rule_indices(graph: &FinalizedAtnGraph) -> BTreeSet<usize> {
    let transitions = transitions_by_id(graph);
    let mut nullable = BTreeSet::new();
    loop {
        let previous = nullable.len();
        for (rule, (&start, &stop)) in graph.rule_starts.iter().zip(&graph.rule_stops).enumerate() {
            if epsilon_reaches(graph, &transitions, start, stop, &nullable) {
                nullable.insert(rule);
            }
        }
        if nullable.len() == previous {
            return nullable;
        }
    }
}

fn epsilon_reaches(
    graph: &FinalizedAtnGraph,
    transitions: &BTreeMap<super::super::model::BuildTransitionId, &FinalizedTransition>,
    start: usize,
    stop: usize,
    nullable_rules: &BTreeSet<usize>,
) -> bool {
    let mut pending = vec![start];
    let mut visited = BTreeSet::new();
    while let Some(state) = pending.pop() {
        if state == stop {
            return true;
        }
        if !visited.insert(state) {
            continue;
        }
        for transition in graph.states[state]
            .transitions
            .iter()
            .filter_map(|transition| transitions.get(transition).copied())
        {
            match &transition.kind {
                FinalizedTransitionKind::Rule {
                    rule_index, follow, ..
                } if nullable_rules.contains(rule_index) => pending.push(*follow),
                kind if kind.is_epsilon() => pending.push(transition.target),
                _ => {}
            }
        }
    }
    false
}

/// The rules that each rule can call before it reads a token, by rule index.
///
/// `nullable_rules` holds the indices of the rules that can match the empty
/// string. Thus the walk also continues after a call of such a rule.
pub(crate) fn left_corner_rules(
    graph: &FinalizedAtnGraph,
    nullable_rules: &BTreeSet<usize>,
) -> Vec<BTreeSet<usize>> {
    let transitions = transitions_by_id(graph);
    let mut left_corners = vec![BTreeSet::new(); graph.rule_starts.len()];
    for (rule, start) in graph.rule_starts.iter().copied().enumerate() {
        collect_left_corners(
            graph,
            &transitions,
            start,
            nullable_rules,
            &mut left_corners[rule],
        );
    }
    left_corners
}

pub(super) fn left_recursive_components(
    graph: &FinalizedAtnGraph,
    nullable_rules: &BTreeSet<usize>,
) -> Vec<Vec<RuleId>> {
    let left_corners = left_corner_rules(graph, nullable_rules);

    let mut petgraph = DiGraph::<RuleId, ()>::new();
    let nodes = graph
        .rule_starts
        .iter()
        .map(|state| {
            let rule = graph.states[*state]
                .rule
                .expect("rule-start state has a rule");
            (rule, petgraph.add_node(rule))
        })
        .collect::<Vec<_>>();
    for (source, targets) in left_corners.iter().enumerate() {
        for target in targets {
            petgraph.add_edge(nodes[source].1, nodes[*target].1, ());
        }
    }

    let mut components = tarjan_scc(&petgraph)
        .into_iter()
        .filter_map(|component| {
            let cyclic = component.len() > 1
                || component
                    .first()
                    .is_some_and(|node| petgraph.find_edge(*node, *node).is_some());
            cyclic.then(|| {
                let mut rules = component
                    .into_iter()
                    .map(|node| petgraph[node])
                    .collect::<Vec<_>>();
                let first = rules
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, rule)| **rule)
                    .map_or(0, |(index, _)| index);
                rules.rotate_left(first);
                rules
            })
        })
        .collect::<Vec<_>>();
    components.sort();
    components
}

fn collect_left_corners(
    graph: &FinalizedAtnGraph,
    transitions: &BTreeMap<super::super::model::BuildTransitionId, &FinalizedTransition>,
    start: usize,
    nullable_rules: &BTreeSet<usize>,
    result: &mut BTreeSet<usize>,
) {
    let mut pending = vec![start];
    let mut visited = BTreeSet::new();
    while let Some(state) = pending.pop() {
        if !visited.insert(state) || graph.states[state].kind == AtnStateKind::RuleStop {
            continue;
        }
        for transition in graph.states[state]
            .transitions
            .iter()
            .filter_map(|transition| transitions.get(transition).copied())
        {
            match &transition.kind {
                FinalizedTransitionKind::Rule {
                    rule_index, follow, ..
                } => {
                    result.insert(*rule_index);
                    if nullable_rules.contains(rule_index) {
                        pending.push(*follow);
                    }
                }
                kind if kind.is_epsilon() => pending.push(transition.target),
                _ => {}
            }
        }
    }
}

pub(super) fn recursion_diagnostics(
    grammar: &SemanticGrammar,
    components: &[Vec<RuleId>],
) -> Vec<Diagnostic> {
    let rules = grammar
        .unit
        .rules
        .iter()
        .map(|rule| (rule.id, rule))
        .collect::<BTreeMap<_, _>>();
    components
        .iter()
        .filter_map(|component| {
            let first = rules.get(component.first()?)?;
            let names = component
                .iter()
                .filter_map(|rule| rules.get(rule).map(|rule| rule.name.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            let mut diagnostic = Diagnostic::error(
                "G4A005",
                first.span.clone(),
                format!("mutually left-recursive rules: [{names}]"),
            );
            for rule in component.iter().skip(1).filter_map(|rule| rules.get(rule)) {
                diagnostic =
                    diagnostic.with_related(rule.span.clone(), "cycle member is declared here");
            }
            Some(diagnostic)
        })
        .collect()
}

fn decision_lookahead(
    graph: &FinalizedAtnGraph,
    analyzer: &LookAnalyzer<'_>,
) -> Vec<DecisionLookahead> {
    graph
        .decisions
        .iter()
        .copied()
        .map(|state_number| {
            let state = &graph.states[state_number];
            let transitions = analyzer.transitions_from(state_number).collect::<Vec<_>>();
            let alternatives = if state.non_greedy {
                vec![None; transitions.len() + 1]
            } else {
                transitions
                    .iter()
                    .map(|transition| {
                        let look = analyzer.look(transition.target, None, false, false);
                        (!look.hit_predicate && !look.ranges.is_empty()).then_some(look.ranges)
                    })
                    .collect()
            };
            let disjoint = alternatives_are_disjoint(&alternatives);
            DecisionLookahead {
                state: state_number,
                alternatives,
                disjoint,
            }
        })
        .collect()
}

fn alternatives_are_disjoint(alternatives: &[Option<Vec<(i32, i32)>>]) -> bool {
    let mut combined = Vec::new();
    for alternative in alternatives {
        let Some(alternative) = alternative else {
            return false;
        };
        if alternative
            .iter()
            .any(|range| range_overlaps_any(*range, &combined))
        {
            return false;
        }
        combined = normalize_ranges(
            &combined
                .into_iter()
                .chain(alternative.iter().copied())
                .collect::<Vec<_>>(),
        );
    }
    true
}

fn range_overlaps_any(range: (i32, i32), ranges: &[(i32, i32)]) -> bool {
    ranges
        .iter()
        .any(|other| range.0 <= other.1 && other.0 <= range.1)
}

pub(super) struct LookAnalyzer<'a> {
    graph: &'a FinalizedAtnGraph,
    outgoing: Vec<Vec<&'a FinalizedTransition>>,
    rule_looks: RefCell<Vec<Vec<Rc<RuleLook>>>>,
}

impl<'a> LookAnalyzer<'a> {
    pub(super) fn new(graph: &'a FinalizedAtnGraph) -> Self {
        let transitions = transitions_by_id(graph);
        let outgoing = graph
            .states
            .iter()
            .map(|state| {
                state
                    .transitions
                    .iter()
                    .filter_map(|transition| transitions.get(transition).copied())
                    .collect()
            })
            .collect();
        Self {
            graph,
            outgoing,
            rule_looks: RefCell::new(vec![Vec::new(); graph.rule_starts.len()]),
        }
    }

    fn transitions_from(&self, state: usize) -> impl Iterator<Item = &'a FinalizedTransition> + '_ {
        self.outgoing[state].iter().copied()
    }

    pub(super) fn look(
        &self,
        start: usize,
        stop: Option<usize>,
        context_none: bool,
        see_through_predicates: bool,
    ) -> LookResult {
        let mut frame = LookFrame::default();
        let mut active = vec![false; self.graph.rule_starts.len()];
        let mut epsilon = false;
        let mut pending = vec![start];
        let mut visited = HashSet::new();
        while let Some(state) = pending.pop() {
            if !visited.insert(state) {
                continue;
            }
            if stop == Some(state) && context_none {
                epsilon = true;
                continue;
            }
            if self.graph.states[state].kind == AtnStateKind::RuleStop && context_none {
                epsilon = true;
                continue;
            }
            // Java `LL1Analyzer._LOOK` under the `getDecisionLookahead`
            // configuration (`EmptyPredictionContext`, `addEOF=false`):
            // an outermost rule stop falls through to its follow-link
            // transitions.
            for transition in self.transitions_from(state) {
                self.follow_transition(
                    transition,
                    &mut active,
                    see_through_predicates,
                    &mut pending,
                    &mut frame,
                );
            }
        }
        LookResult {
            ranges: normalize_ranges(&frame.ranges),
            epsilon,
            hit_predicate: frame.hit_predicate,
        }
    }

    /// Returns the look of `rule` from its start state to its stop state.
    ///
    /// `active` marks the rules on the call stack. The look does not enter
    /// an active rule again.
    fn rule_look(
        &self,
        rule: usize,
        active: &mut [bool],
        see_through_predicates: bool,
    ) -> Rc<RuleLook> {
        active[rule] = true;
        let known = self.rule_looks.borrow()[rule]
            .iter()
            .find(|look| look.matches(see_through_predicates, active))
            .cloned();
        if let Some(known) = known {
            active[rule] = false;
            return known;
        }

        let mut frame = LookFrame {
            tested: Some(BTreeSet::new()),
            ..LookFrame::default()
        };
        let mut reaches_stop = false;
        let mut pending = vec![self.graph.rule_starts[rule]];
        let mut visited = HashSet::new();
        while let Some(state) = pending.pop() {
            if !visited.insert(state) {
                continue;
            }
            if self.graph.states[state].kind == AtnStateKind::RuleStop {
                reaches_stop = true;
                continue;
            }
            for transition in self.transitions_from(state) {
                self.follow_transition(
                    transition,
                    active,
                    see_through_predicates,
                    &mut pending,
                    &mut frame,
                );
            }
        }
        let tested = frame.tested.unwrap_or_default();
        let look = Rc::new(RuleLook {
            see_through_predicates,
            tested: tested
                .into_iter()
                .map(|tested| (tested, active[tested]))
                .collect(),
            ranges: normalize_ranges(&frame.ranges),
            reaches_stop,
            hit_predicate: frame.hit_predicate,
        });
        active[rule] = false;
        self.rule_looks.borrow_mut()[rule].push(Rc::clone(&look));
        look
    }

    fn follow_transition(
        &self,
        transition: &FinalizedTransition,
        active: &mut [bool],
        see_through_predicates: bool,
        pending: &mut Vec<usize>,
        frame: &mut LookFrame,
    ) {
        match &transition.kind {
            FinalizedTransitionKind::Rule {
                rule_index, follow, ..
            } => {
                if let Some(tested) = &mut frame.tested {
                    tested.insert(*rule_index);
                }
                if active[*rule_index] {
                    return;
                }
                debug_assert_eq!(transition.target, self.graph.rule_starts[*rule_index]);
                let callee = self.rule_look(*rule_index, active, see_through_predicates);
                if let Some(tested) = &mut frame.tested {
                    tested.extend(callee.tested.iter().map(|(rule, _)| *rule));
                }
                frame.ranges.extend(callee.ranges.iter().copied());
                frame.hit_predicate |= callee.hit_predicate;
                if callee.reaches_stop {
                    pending.push(*follow);
                }
            }
            FinalizedTransitionKind::Predicate { .. } | FinalizedTransitionKind::Precedence(_) => {
                if see_through_predicates {
                    pending.push(transition.target);
                } else {
                    frame.hit_predicate = true;
                }
            }
            FinalizedTransitionKind::Epsilon | FinalizedTransitionKind::Action { .. } => {
                pending.push(transition.target);
            }
            FinalizedTransitionKind::Atom(label) => {
                frame.ranges.push((*label, *label));
            }
            FinalizedTransitionKind::Range { start, stop } => {
                frame.ranges.push((*start, *stop));
            }
            FinalizedTransitionKind::Set(ranges) => {
                frame.ranges.extend(ranges.iter().copied());
            }
            FinalizedTransitionKind::NotSet(ranges) => {
                frame.ranges.extend(complement_ranges(
                    ranges,
                    MIN_USER_TOKEN_TYPE,
                    self.graph.max_token_type,
                ));
            }
            FinalizedTransitionKind::Wildcard => {
                frame
                    .ranges
                    .push((MIN_USER_TOKEN_TYPE, self.graph.max_token_type));
            }
        }
    }
}

#[derive(Default)]
struct LookFrame {
    ranges: Vec<(i32, i32)>,
    hit_predicate: bool,
    tested: Option<BTreeSet<usize>>,
}

/// The look of one called rule, from its start state to its stop state.
///
/// A look inside a called rule does not read the frames of its callers. The
/// result depends only on the rule, on the predicate mode, and on the active
/// rules that the look does not enter. `tested` holds each rule that the look
/// tried to call, and whether that rule was active. The result is correct for
/// each call stack that agrees on all of these rules.
struct RuleLook {
    see_through_predicates: bool,
    tested: Vec<(usize, bool)>,
    ranges: Vec<(i32, i32)>,
    reaches_stop: bool,
    hit_predicate: bool,
}

impl RuleLook {
    fn matches(&self, see_through_predicates: bool, active: &[bool]) -> bool {
        self.see_through_predicates == see_through_predicates
            && self
                .tested
                .iter()
                .all(|(rule, was_active)| active[*rule] == *was_active)
    }
}

#[derive(Default)]
pub(super) struct LookResult {
    ranges: Vec<(i32, i32)>,
    pub(super) epsilon: bool,
    hit_predicate: bool,
}

impl LookResult {
    pub(super) fn contains(&self, symbol: i32) -> bool {
        self.ranges
            .iter()
            .any(|(start, stop)| *start <= symbol && symbol <= *stop)
    }
}

fn transitions_by_id(
    graph: &FinalizedAtnGraph,
) -> BTreeMap<super::super::model::BuildTransitionId, &FinalizedTransition> {
    graph
        .transitions
        .iter()
        .map(|transition| (transition.original, transition))
        .collect()
}

fn complement_ranges(ranges: &[(i32, i32)], minimum: i32, maximum: i32) -> Vec<(i32, i32)> {
    if maximum < minimum {
        return Vec::new();
    }
    let ranges = normalize_ranges(ranges);
    let mut complement = Vec::new();
    let mut cursor = minimum;
    for (start, stop) in ranges {
        if stop < minimum || start > maximum {
            continue;
        }
        let start = start.max(minimum);
        let stop = stop.min(maximum);
        if cursor < start {
            complement.push((cursor, start - 1));
        }
        cursor = stop.saturating_add(1);
        if cursor > maximum {
            return complement;
        }
    }
    if cursor <= maximum {
        complement.push((cursor, maximum));
    }
    complement
}

fn normalize_ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut ranges = ranges.to_vec();
    ranges.sort_unstable();
    let mut normalized: Vec<(i32, i32)> = Vec::with_capacity(ranges.len());
    for (start, stop) in ranges {
        let (start, stop) = if start <= stop {
            (start, stop)
        } else {
            (stop, start)
        };
        if let Some((_, previous_stop)) = normalized.last_mut()
            && start <= previous_stop.saturating_add(1)
        {
            *previous_stop = (*previous_stop).max(stop);
        } else {
            normalized.push((start, stop));
        }
    }
    normalized
}

trait EpsilonTransition {
    fn is_epsilon(&self) -> bool;
}

impl EpsilonTransition for FinalizedTransitionKind {
    fn is_epsilon(&self) -> bool {
        matches!(
            self,
            Self::Epsilon | Self::Predicate { .. } | Self::Action { .. } | Self::Precedence(_)
        )
    }
}
