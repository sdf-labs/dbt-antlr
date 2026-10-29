// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeMap;

use super::super::diagnostic::CompilationError;
use super::super::model::{
    Alternative, Block, Element, ElementKind, ModelNodeId, Quantifier, Rule, RuleId,
    SemanticGrammar, SetElement, Terminal,
};
use super::super::provenance::{ProvenanceIndex, SyntheticReason};
use super::analysis::{ParserAnalysis, ParserCheckSites, analyze_parser};
use super::build::{BuildGraph, BuildTransitionKind, BuildTransitionSpec, FinalizedAtnGraph};
use super::optimize::remove_tail_epsilons;
use super::runtime_build::{self, add_rule_return_edges, usize_to_i32};
use super::state_kind::AtnStateKind;
use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_state::{ATNStateType, RuleStartState, StarLoopEntryState};
use dbt_antlr_runtime::atn_type::ATNType;

const EOF_TOKEN_TYPE: i32 = -1;

#[derive(Debug)]
pub(crate) struct CompiledParser {
    pub(crate) semantic: SemanticGrammar,
    pub(crate) graph: FinalizedAtnGraph,
    pub(crate) atn: &'static ATN,
    pub(crate) analysis: ParserAnalysis,
    pub(crate) provenance: ProvenanceIndex,
}

pub(crate) fn compile_parser(
    grammar: SemanticGrammar,
    mut provenance: ProvenanceIndex,
) -> Result<CompiledParser, CompilationError> {
    let mut factory = ParserFactory::new(&grammar, &mut provenance);
    factory.build();
    let epsilon_closures = std::mem::take(&mut factory.epsilon_closures);
    let epsilon_optionals = std::mem::take(&mut factory.epsilon_optionals);
    let graph = factory.graph.finalize();
    let check_sites = ParserCheckSites::compact(&graph, epsilon_closures, epsilon_optionals);
    let analysis = analyze_parser(&grammar, &graph, &check_sites)?;
    let atn = build_atn(&graph);
    Ok(CompiledParser {
        semantic: grammar,
        graph,
        atn: atn.into_static(),
        analysis,
        provenance,
    })
}

#[cfg(test)]
pub(super) fn build_graph_for_test(
    grammar: &SemanticGrammar,
    mut provenance: ProvenanceIndex,
) -> (FinalizedAtnGraph, ProvenanceIndex) {
    let mut factory = ParserFactory::new(grammar, &mut provenance);
    factory.build();
    (factory.graph.finalize(), provenance)
}

#[derive(Clone, Copy, Debug)]
struct StatePair {
    left: super::super::model::BuildStateId,
    right: super::super::model::BuildStateId,
}

struct ParserFactory<'a> {
    grammar: &'a SemanticGrammar,
    graph: BuildGraph,
    provenance: &'a mut ProvenanceIndex,
    current_rule: Option<(&'a Rule, usize)>,
    epsilon_closures: Vec<(
        RuleId,
        super::super::model::BuildStateId,
        super::super::model::BuildStateId,
    )>,
    epsilon_optionals: Vec<(
        RuleId,
        super::super::model::BuildStateId,
        super::super::model::BuildStateId,
    )>,
}

impl<'a> ParserFactory<'a> {
    fn new(grammar: &'a SemanticGrammar, provenance: &'a mut ProvenanceIndex) -> Self {
        Self {
            grammar,
            graph: BuildGraph::new(grammar.recognizer.vocabulary.max_token_type()),
            provenance,
            current_rule: None,
            epsilon_closures: Vec::new(),
            epsilon_optionals: Vec::new(),
        }
    }

    fn build(&mut self) {
        self.create_rule_boundaries();
        for (rule_index, rule) in self.grammar.unit.rules.iter().enumerate() {
            self.current_rule = Some((rule, rule_index));
            let pair = self.block(&rule.block, Quantifier::One, ModelNodeId::Rule(rule.id));
            let start = self.graph.rule_starts[rule_index];
            let stop = self.graph.rule_stops[rule_index];
            self.synthetic_epsilon(
                start,
                pair.left,
                SyntheticReason::RuleBoundary,
                ModelNodeId::Rule(rule.id),
                false,
            );
            self.synthetic_epsilon(
                pair.right,
                stop,
                SyntheticReason::RuleBoundary,
                ModelNodeId::Rule(rule.id),
                false,
            );
        }
        self.add_rule_follow_links();
        self.add_eof_transitions();
        self.current_rule = None;
    }

    fn create_rule_boundaries(&mut self) {
        for (rule_index, rule) in self.grammar.unit.rules.iter().enumerate() {
            let owner = ModelNodeId::Rule(rule.id);
            let start = self.graph.add_synthetic_state(
                AtnStateKind::RuleStart,
                Some((rule.id, rule_index)),
                SyntheticReason::RuleBoundary,
                owner,
                self.provenance,
            );
            self.graph.state_mut(start).left_recursive_rule = rule.left_recursion.is_some();
            let stop = self.graph.add_synthetic_state(
                AtnStateKind::RuleStop,
                Some((rule.id, rule_index)),
                SyntheticReason::RuleBoundary,
                owner,
                self.provenance,
            );
            self.graph.rule_starts.push(start);
            self.graph.rule_stops.push(stop);
        }
    }

    fn block(&mut self, block: &Block, quantifier: Quantifier, owner: ModelNodeId) -> StatePair {
        let alternatives = block
            .alternatives
            .iter()
            .map(|alternative| self.alternative(alternative))
            .collect::<Vec<_>>();
        if quantifier == Quantifier::One && alternatives.len() == 1 {
            return alternatives[0];
        }

        let start_kind = match quantifier {
            Quantifier::One | Quantifier::Optional { .. } => AtnStateKind::BlockStart,
            Quantifier::ZeroOrMore { .. } => AtnStateKind::StarBlockStart,
            Quantifier::OneOrMore { .. } => AtnStateKind::PlusBlockStart,
        };
        let start = self.synthetic_state(start_kind, SyntheticReason::BlockBoundary, owner);
        if alternatives.len() > 1 || matches!(quantifier, Quantifier::Optional { .. }) {
            self.graph.add_decision(start);
        }
        let pair = self.make_block(start, alternatives, owner);
        match quantifier {
            Quantifier::One => pair,
            Quantifier::Optional { greedy } => self.optional(pair, greedy, owner),
            Quantifier::ZeroOrMore { greedy } => self.star(pair, greedy, owner),
            Quantifier::OneOrMore { greedy } => self.plus(pair, greedy, owner),
        }
    }

    fn alternative(&mut self, alternative: &Alternative) -> StatePair {
        if alternative.elements.is_empty() {
            return self.epsilon_pair(ModelNodeId::Alternative(alternative.id));
        }
        let elements = alternative
            .elements
            .iter()
            .map(|element| self.element(element))
            .collect::<Vec<_>>();
        self.element_list(&elements, ModelNodeId::Alternative(alternative.id))
    }

    fn element(&mut self, element: &Element) -> StatePair {
        let owner = ModelNodeId::Element(element.id);
        let base = match &element.kind {
            ElementKind::Terminal(terminal) => self.terminal(element, terminal),
            ElementKind::RuleCall(_) => self.rule_call(element),
            ElementKind::Range(start, _, _) => {
                let token_type = self
                    .grammar
                    .recognizer
                    .vocabulary
                    .by_literal
                    .get(&start.value)
                    .copied()
                    .unwrap_or(EOF_TOKEN_TYPE);
                self.atom_pair(owner, token_type)
            }
            ElementKind::Set { inverted, elements } => self.set_pair(owner, *inverted, elements),
            ElementKind::Block(block) => {
                return self.block(block, element.quantifier, owner);
            }
            ElementKind::Action { id, .. } => {
                self.grammar
                    .bindings
                    .actions
                    .get(id)
                    .expect("semantic action binding exists");
                let pair = self.basic_pair(owner);
                self.authored_transition(
                    pair.left,
                    pair.right,
                    BuildTransitionKind::Action {
                        rule_index: self.current_rule_index(),
                        action_index: None,
                        context_dependent: false,
                    },
                    owner,
                );
                pair
            }
            ElementKind::Predicate { id, precedence, .. } => {
                let binding = self
                    .grammar
                    .bindings
                    .predicates
                    .get(id)
                    .expect("semantic predicate binding exists");
                let pair = self.basic_pair(owner);
                let kind = precedence.or(binding.precedence).map_or_else(
                    || BuildTransitionKind::Predicate {
                        rule_index: self.current_rule_index(),
                        predicate_index: binding.index,
                        context_dependent: binding.context_dependent,
                    },
                    |precedence| {
                        BuildTransitionKind::Precedence(
                            i32::try_from(precedence).expect("precedence exceeds i32"),
                        )
                    },
                );
                self.authored_transition(pair.left, pair.right, kind, owner);
                pair
            }
            ElementKind::Epsilon => self.epsilon_pair(owner),
        };

        if element.quantifier == Quantifier::One {
            base
        } else {
            self.quantified_atom(base, element.quantifier, owner)
        }
    }

    fn terminal(&mut self, element: &Element, terminal: &Terminal) -> StatePair {
        let owner = ModelNodeId::Element(element.id);
        match terminal {
            Terminal::Wildcard => {
                let pair = self.basic_pair(owner);
                self.authored_transition(
                    pair.left,
                    pair.right,
                    BuildTransitionKind::Wildcard,
                    owner,
                );
                pair
            }
            Terminal::Eof => self.atom_pair(owner, EOF_TOKEN_TYPE),
            Terminal::Token(_) | Terminal::Literal(_) => {
                let binding = self
                    .grammar
                    .bindings
                    .terminals
                    .get(&element.id)
                    .expect("semantic terminal binding exists");
                self.atom_pair(owner, binding.token_type)
            }
            Terminal::LexerCharSet(_) => {
                unreachable!("lexer character set reached parser factory")
            }
        }
    }

    fn rule_call(&mut self, element: &Element) -> StatePair {
        let owner = ModelNodeId::Element(element.id);
        let binding = self
            .grammar
            .bindings
            .rule_calls
            .get(&element.id)
            .expect("semantic rule-call binding exists");
        let pair = self.basic_pair(owner);
        let target_index = self.grammar.recognizer.rule_numbers[&binding.target];
        self.authored_transition(
            pair.left,
            self.graph.rule_starts[target_index],
            BuildTransitionKind::Rule {
                rule: binding.target,
                rule_index: target_index,
                follow: pair.right,
                precedence: i32::try_from(binding.precedence).expect("precedence exceeds i32"),
            },
            owner,
        );
        pair
    }

    fn set_pair(
        &mut self,
        owner: ModelNodeId,
        inverted: bool,
        elements: &[SetElement],
    ) -> StatePair {
        let mut ranges = Vec::new();
        for element in elements {
            match element {
                SetElement::Terminal { value, .. } => {
                    if let Some(token_type) = terminal_type(value, self.grammar) {
                        ranges.push((token_type, token_type));
                    }
                }
                SetElement::Range { start, stop, .. } => {
                    let start = self
                        .grammar
                        .recognizer
                        .vocabulary
                        .by_literal
                        .get(start)
                        .copied()
                        .unwrap_or(INVALID_TOKEN);
                    let stop = self
                        .grammar
                        .recognizer
                        .vocabulary
                        .by_literal
                        .get(stop)
                        .copied()
                        .unwrap_or(INVALID_TOKEN);
                    ranges.push((start, stop));
                }
            }
        }
        ranges.sort_unstable();
        ranges.dedup();
        let pair = self.basic_pair(owner);
        self.authored_transition(
            pair.left,
            pair.right,
            if inverted {
                BuildTransitionKind::NotSet(ranges)
            } else {
                BuildTransitionKind::Set(ranges)
            },
            owner,
        );
        pair
    }

    fn quantified_atom(
        &mut self,
        base: StatePair,
        quantifier: Quantifier,
        owner: ModelNodeId,
    ) -> StatePair {
        let kind = match quantifier {
            Quantifier::Optional { .. } => AtnStateKind::BlockStart,
            Quantifier::ZeroOrMore { .. } => AtnStateKind::StarBlockStart,
            Quantifier::OneOrMore { .. } => AtnStateKind::PlusBlockStart,
            Quantifier::One => return base,
        };
        let start = self.synthetic_state(kind, SyntheticReason::BlockBoundary, owner);
        if matches!(quantifier, Quantifier::Optional { .. }) {
            self.graph.add_decision(start);
        }
        let pair = self.make_block(start, vec![base], owner);
        match quantifier {
            Quantifier::Optional { greedy } => self.optional(pair, greedy, owner),
            Quantifier::ZeroOrMore { greedy } => self.star(pair, greedy, owner),
            Quantifier::OneOrMore { greedy } => self.plus(pair, greedy, owner),
            Quantifier::One => unreachable!("handled above"),
        }
    }

    fn make_block(
        &mut self,
        start: super::super::model::BuildStateId,
        alternatives: Vec<StatePair>,
        owner: ModelNodeId,
    ) -> StatePair {
        let end = self.synthetic_state(
            AtnStateKind::BlockEnd,
            SyntheticReason::BlockBoundary,
            owner,
        );
        self.graph.state_mut(start).end_state = Some(end);
        for alternative in alternatives {
            self.synthetic_epsilon(
                start,
                alternative.left,
                SyntheticReason::BlockBoundary,
                owner,
                false,
            );
            self.synthetic_epsilon(
                alternative.right,
                end,
                SyntheticReason::BlockBoundary,
                owner,
                false,
            );
            remove_tail_epsilons(&mut self.graph, alternative.left);
        }
        StatePair {
            left: start,
            right: end,
        }
    }

    fn optional(&mut self, pair: StatePair, greedy: bool, owner: ModelNodeId) -> StatePair {
        let (rule, _) = self.current_rule.expect("building a rule");
        self.epsilon_optionals
            .push((rule.id, pair.left, pair.right));
        self.graph.state_mut(pair.left).non_greedy = !greedy;
        self.synthetic_epsilon(
            pair.left,
            pair.right,
            SyntheticReason::LoopBoundary,
            owner,
            !greedy,
        );
        pair
    }

    fn plus(&mut self, pair: StatePair, greedy: bool, owner: ModelNodeId) -> StatePair {
        let (rule, _) = self.current_rule.expect("building a rule");
        self.epsilon_closures.push((rule.id, pair.left, pair.right));
        let loop_state = self.synthetic_state(
            AtnStateKind::PlusLoopBack,
            SyntheticReason::LoopBoundary,
            owner,
        );
        self.graph.state_mut(loop_state).non_greedy = !greedy;
        self.graph.add_decision(loop_state);
        let end = self.synthetic_state(AtnStateKind::LoopEnd, SyntheticReason::LoopBoundary, owner);
        self.graph.state_mut(end).loop_back_state = Some(loop_state);
        self.synthetic_epsilon(
            pair.right,
            loop_state,
            SyntheticReason::LoopBoundary,
            owner,
            false,
        );
        let ordered = if greedy {
            [(pair.left, false), (end, false)]
        } else {
            [(end, false), (pair.left, false)]
        };
        for (target, prepend) in ordered {
            self.synthetic_epsilon(
                loop_state,
                target,
                SyntheticReason::LoopBoundary,
                owner,
                prepend,
            );
        }
        StatePair {
            left: pair.left,
            right: end,
        }
    }

    fn star(&mut self, pair: StatePair, greedy: bool, owner: ModelNodeId) -> StatePair {
        let (rule, _) = self.current_rule.expect("building a rule");
        self.epsilon_closures.push((rule.id, pair.left, pair.right));
        let entry = self.synthetic_state(
            AtnStateKind::StarLoopEntry,
            SyntheticReason::LoopBoundary,
            owner,
        );
        self.graph.state_mut(entry).non_greedy = !greedy;
        self.graph.add_decision(entry);
        let end = self.synthetic_state(AtnStateKind::LoopEnd, SyntheticReason::LoopBoundary, owner);
        let loop_state = self.synthetic_state(
            AtnStateKind::StarLoopBack,
            SyntheticReason::LoopBoundary,
            owner,
        );
        self.graph.state_mut(end).loop_back_state = Some(loop_state);
        for target in if greedy {
            [pair.left, end]
        } else {
            [end, pair.left]
        } {
            self.synthetic_epsilon(entry, target, SyntheticReason::LoopBoundary, owner, false);
        }
        self.synthetic_epsilon(
            pair.right,
            loop_state,
            SyntheticReason::LoopBoundary,
            owner,
            false,
        );
        self.synthetic_epsilon(
            loop_state,
            entry,
            SyntheticReason::LoopBoundary,
            owner,
            false,
        );
        StatePair {
            left: entry,
            right: end,
        }
    }

    fn element_list(&mut self, elements: &[StatePair], owner: ModelNodeId) -> StatePair {
        for index in 0..elements.len().saturating_sub(1) {
            let element = elements[index];
            let next = elements[index + 1];
            let state = self.graph.state(element.left);
            let transition = (state.kind == AtnStateKind::Basic
                && self.graph.state(element.right).kind == AtnStateKind::Basic
                && state.transitions.len() == 1)
                .then(|| state.transitions[0]);
            let can_inline = transition.is_some_and(|transition| {
                let transition = self.graph.transition(transition);
                match &transition.kind {
                    BuildTransitionKind::Rule { follow, .. } => *follow == element.right,
                    _ => transition.target == element.right,
                }
            });
            if can_inline {
                let transition = transition.expect("checked above");
                match &mut self.graph.transition_mut(transition).kind {
                    BuildTransitionKind::Rule { follow, .. } => {
                        *follow = next.left;
                    }
                    _ => self.graph.transition_mut(transition).target = next.left,
                }
                self.graph.remove_state(element.right);
            } else {
                self.synthetic_epsilon(
                    element.right,
                    next.left,
                    SyntheticReason::BlockBoundary,
                    owner,
                    false,
                );
            }
        }
        StatePair {
            left: elements[0].left,
            right: elements[elements.len() - 1].right,
        }
    }

    fn add_rule_follow_links(&mut self) {
        let calls = self
            .graph
            .states
            .iter()
            .filter(|state| {
                state.active && state.kind == AtnStateKind::Basic && state.transitions.len() == 1
            })
            .filter_map(|state| {
                let transition = self.graph.transition(state.transitions[0]);
                let BuildTransitionKind::Rule {
                    rule,
                    rule_index,
                    follow,
                    ..
                } = transition.kind
                else {
                    return None;
                };
                Some((rule, rule_index, follow))
            })
            .collect::<Vec<_>>();
        for (rule, rule_index, follow) in calls {
            self.synthetic_epsilon(
                self.graph.rule_stops[rule_index],
                follow,
                SyntheticReason::RuleFollow,
                ModelNodeId::Rule(rule),
                false,
            );
        }
    }

    fn add_eof_transitions(&mut self) {
        let owner = ModelNodeId::Grammar(self.grammar.unit.id);
        let eof = self.graph.add_synthetic_state(
            AtnStateKind::Basic,
            self.current_rule.map(|(rule, index)| (rule.id, index)),
            SyntheticReason::EntryEof,
            owner,
            self.provenance,
        );
        let rules = self
            .grammar
            .unit
            .rules
            .iter()
            .enumerate()
            .map(|(index, rule)| (index, rule.id))
            .collect::<Vec<_>>();
        for (index, rule) in rules {
            let stop = self.graph.rule_stops[index];
            if self.graph.state(stop).transitions.is_empty() {
                self.graph.add_synthetic_transition(
                    BuildTransitionSpec {
                        source: stop,
                        target: eof,
                        kind: BuildTransitionKind::Atom(EOF_TOKEN_TYPE),
                        prepend: false,
                    },
                    SyntheticReason::EntryEof,
                    ModelNodeId::Rule(rule),
                    self.provenance,
                );
            }
        }
    }

    const fn current_rule_index(&self) -> usize {
        self.current_rule.expect("building a rule").1
    }

    fn basic_pair(&mut self, owner: ModelNodeId) -> StatePair {
        StatePair {
            left: self.authored_state(AtnStateKind::Basic, owner),
            right: self.authored_state(AtnStateKind::Basic, owner),
        }
    }

    fn epsilon_pair(&mut self, owner: ModelNodeId) -> StatePair {
        let pair = self.basic_pair(owner);
        self.authored_transition(pair.left, pair.right, BuildTransitionKind::Epsilon, owner);
        pair
    }

    fn atom_pair(&mut self, owner: ModelNodeId, label: i32) -> StatePair {
        let pair = self.basic_pair(owner);
        self.authored_transition(
            pair.left,
            pair.right,
            BuildTransitionKind::Atom(label),
            owner,
        );
        pair
    }

    fn authored_state(
        &mut self,
        kind: AtnStateKind,
        owner: ModelNodeId,
    ) -> super::super::model::BuildStateId {
        let origins = self.provenance.origins(owner).to_vec();
        self.graph.add_state(
            kind,
            self.current_rule.map(|(rule, index)| (rule.id, index)),
            origins,
            self.provenance,
        )
    }

    fn synthetic_state(
        &mut self,
        kind: AtnStateKind,
        reason: SyntheticReason,
        owner: ModelNodeId,
    ) -> super::super::model::BuildStateId {
        self.graph.add_synthetic_state(
            kind,
            self.current_rule.map(|(rule, index)| (rule.id, index)),
            reason,
            owner,
            self.provenance,
        )
    }

    fn authored_transition(
        &mut self,
        source: super::super::model::BuildStateId,
        target: super::super::model::BuildStateId,
        kind: BuildTransitionKind,
        owner: ModelNodeId,
    ) {
        let origins = self.provenance.origins(owner).to_vec();
        let transition = self.graph.add_transition(
            BuildTransitionSpec {
                source,
                target,
                kind,
                prepend: false,
            },
            origins,
            self.provenance,
        );
        self.graph.record_model_transition(owner, transition);
    }

    fn synthetic_epsilon(
        &mut self,
        source: super::super::model::BuildStateId,
        target: super::super::model::BuildStateId,
        reason: SyntheticReason,
        owner: ModelNodeId,
        prepend: bool,
    ) {
        self.graph.add_synthetic_transition(
            BuildTransitionSpec {
                source,
                target,
                kind: BuildTransitionKind::Epsilon,
                prepend,
            },
            reason,
            owner,
            self.provenance,
        );
    }
}

const INVALID_TOKEN: i32 = 0;

fn terminal_type(terminal: &Terminal, grammar: &SemanticGrammar) -> Option<i32> {
    match terminal {
        Terminal::Token(name) => grammar.recognizer.vocabulary.by_name.get(name).copied(),
        Terminal::Literal(literal) => grammar
            .recognizer
            .vocabulary
            .by_literal
            .get(literal)
            .copied(),
        Terminal::Eof => Some(EOF_TOKEN_TYPE),
        Terminal::LexerCharSet(_) | Terminal::Wildcard => None,
    }
}

/// Builds the runtime parser [`ATN`] of the finalized graph.
///
/// The result is the single ATN representation of a compiled parser:
/// [`super::packed::encode_packed_atn`] serializes it into the packed word
/// stream that generated parsers embed, and the `--static-parser-dfa` pass
/// reads it directly. Construction follows the packed deserializer, so both
/// forms describe the same graph.
fn build_atn(graph: &FinalizedAtnGraph) -> ATN {
    let mut atn = ATN::new(ATNType::Parser, graph.max_token_type);
    runtime_build::build_states(&mut atn, graph);

    let transitions = graph
        .transitions
        .iter()
        .map(|transition| (transition.original, transition))
        .collect::<BTreeMap<_, _>>();
    for (source, state) in graph.states.iter().enumerate() {
        // Return edges leave a rule stop state towards the follow state of a
        // call site, and only the call site knows the call precedence, so the
        // edges are synthesized from the rule calls instead.
        if state.kind == AtnStateKind::RuleStop {
            continue;
        }
        for transition_id in &state.transitions {
            let Some(edge) = transitions.get(transition_id).copied() else {
                continue;
            };
            let target = atn.make_state_ref(usize_to_i32(edge.target));
            let runtime = runtime_build::transition(&atn, edge, target);
            atn.get_state_mut(usize_to_i32(source))
                .add_transition(runtime);
        }
    }
    add_rule_return_edges(&mut atn);
    mark_precedence_decisions(&mut atn);
    atn
}

/// Marks the star-loop entries that select a precedence rule alternative.
///
/// The mark follows the runtime deserializer's precedence rule. For each
/// star-loop entry of a left recursive rule, the entry is a precedence
/// decision when its loop end is an epsilon-only loop end that leads straight
/// to the rule stop state.
fn mark_precedence_decisions(atn: &mut ATN) {
    let candidates = atn
        .iter_states()
        .filter(|state| state.state_type() == ATNStateType::StarLoopEntry)
        .map(|state| state.get_state_number())
        .collect::<Vec<_>>();
    for state in candidates {
        let Ok(rule_index) = usize::try_from(atn.get_state(state).get_rule_index()) else {
            continue;
        };
        let left_recursive = atn
            .rule_to_start_state
            .get(rule_index)
            .and_then(|start| start.try_as::<RuleStartState>())
            .is_some_and(|start| start.is_left_recursive);
        if !left_recursive {
            continue;
        }
        let Some(loop_end) = atn
            .get_state(state)
            .get_transitions()
            .last()
            .map(|transition| transition.get_target())
        else {
            continue;
        };
        let leads_to_rule_stop = loop_end.state_type() == ATNStateType::LoopEnd
            && loop_end.has_epsilon_only_transitions()
            && loop_end
                .get_transitions()
                .first()
                .is_some_and(|transition| {
                    transition.get_target().state_type() == ATNStateType::RuleStop
                });
        if leads_to_rule_stop
            && let Some(entry) = atn.get_state_mut(state).try_as_mut::<StarLoopEntryState>()
        {
            entry.is_precedence = true;
        }
    }
}
