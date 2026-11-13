// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! The names of the alternatives of a parser decision, for diagnostics.
//!
//! A decision numbers its alternatives from 1, in the order of the
//! transitions of its decision state. This module finds the grammar block,
//! loop, or optional element behind each decision. It names each alternative
//! by its `#Label`, or else by its first elements, with the line where it
//! starts. For example: `alt 9 (#ColumnExprWinFunction, line 981)`.

use dbt_antlr_runtime::analysis::ATNStateType;
use dbt_antlr_runtime::dfa::static_parser_dfa::VIRTUAL_ALT_BASE;

use crate::grammar::atn::CompiledParser;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::{
    Alternative, Block, Element, ElementId, ElementKind, ModelNodeId, Quantifier, SetElement,
    Terminal,
};
use crate::grammar::provenance::Origin;
use crate::grammar::source::SourceSet;

/// The elements of an alternative that its name shows at most.
const NAME_ELEMENTS: usize = 3;
/// The alternatives that a list in a diagnostic shows at most.
const LIST_ALTERNATIVES: usize = 4;

/// The names of the alternatives of one decision. Index `alt - 1` holds the
/// name of alternative `alt`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct AlternativeNames {
    names: Vec<String>,
}

impl AlternativeNames {
    /// The name of alternative `alt`, `alt N` when the decision has no grammar
    /// alternative for it.
    pub(crate) fn name(&self, alt: i32) -> String {
        if alt >= VIRTUAL_ALT_BASE {
            return format!("shared-descent group {}", alt - VIRTUAL_ALT_BASE);
        }
        usize::try_from(alt)
            .ok()
            .and_then(|alt| alt.checked_sub(1))
            .and_then(|index| self.names.get(index))
            .cloned()
            .unwrap_or_else(|| format!("alt {alt}"))
    }

    /// The names of `alts`, joined with commas. After
    /// [`LIST_ALTERNATIVES`] names, the list gives only the number of the
    /// other alternatives.
    pub(crate) fn list(&self, alts: &[i32]) -> String {
        let mut names: Vec<String> = alts
            .iter()
            .take(LIST_ALTERNATIVES)
            .map(|alt| self.name(*alt))
            .collect();
        match alts.len().saturating_sub(LIST_ALTERNATIVES) {
            0 => {}
            1 => names.push("and 1 more alternative".to_owned()),
            more => names.push(format!("and {more} more alternatives")),
        }
        names.join(", ")
    }

    /// Every name, in alternative order.
    pub(crate) fn all(&self) -> &[String] {
        &self.names
    }
}

/// What a decision chooses between.
enum Shape<'m> {
    /// The alternatives of a block. An optional block has one more
    /// alternative that skips the block.
    Block {
        alternatives: &'m [Alternative],
        skip: bool,
    },
    /// The entry or the back edge of a loop: repeat or exit. The element is
    /// the loop body, when the decision has one in the grammar.
    Loop(Option<&'m Element>),
    /// An optional element that is not a block: take or skip.
    Optional(&'m Element),
}

/// The names of the alternatives of decision `decision`.
pub(crate) fn alternative_names(
    compiled: &CompiledParser,
    sources: &SourceSet,
    decision: usize,
) -> AlternativeNames {
    let Some(state) = compiled.atn.decision_to_state.get(decision) else {
        return AlternativeNames::default();
    };
    let count = state.get_transitions().len();
    let state_type = state.state_type();
    // A non-greedy decision puts its exit or skip first.
    let non_greedy = usize::try_from(state.get_state_number())
        .ok()
        .and_then(|number| compiled.graph.states.get(number))
        .is_some_and(|state| state.non_greedy);
    let looping = matches!(
        state_type,
        ATNStateType::StarLoopEntry | ATNStateType::PlusLoopBack
    );
    let owner = usize::try_from(state.get_state_number())
        .ok()
        .and_then(|number| decision_owner(compiled, number));
    let shape = if looping {
        Some(Shape::Loop(owner.and_then(|owner| match owner {
            Owner::Element(element) => Some(element),
            Owner::Rule(_) => None,
        })))
    } else {
        owner.and_then(|owner| decision_shape(owner, count))
    };
    let names = (1..=count)
        .map(|alt| {
            // The position of the alternative in the greedy order.
            let greedy_alt = match (&shape, non_greedy) {
                (Some(Shape::Loop(_) | Shape::Optional(_)), true) => 3 - alt.min(2),
                (Some(Shape::Block { skip: true, .. }), true) if alt == 1 => count,
                (Some(Shape::Block { skip: true, .. }), true) => alt - 1,
                _ => alt,
            };
            let detail = match &shape {
                Some(Shape::Block { alternatives, .. }) if greedy_alt <= alternatives.len() => {
                    alternative_detail(&alternatives[greedy_alt - 1], sources)
                }
                Some(Shape::Block { skip: true, .. }) => Some("skip the block".to_owned()),
                Some(Shape::Loop(Some(body))) if greedy_alt == 1 => Some(located(
                    &format!("repeat {}", loop_body_text(body)),
                    &body.span,
                    sources,
                )),
                Some(Shape::Loop(None)) if greedy_alt == 1 => Some("repeat the loop".to_owned()),
                Some(Shape::Loop(_)) if greedy_alt == 2 => Some("exit the loop".to_owned()),
                Some(Shape::Optional(element)) if greedy_alt == 1 => Some(located(
                    &element_text(&Element {
                        quantifier: Quantifier::One,
                        ..(*element).clone()
                    }),
                    &element.span,
                    sources,
                )),
                Some(Shape::Optional(_)) if greedy_alt == 2 => Some("skip".to_owned()),
                _ => None,
            };
            match detail {
                Some(detail) => format!("alt {alt} ({detail})"),
                None => format!("alt {alt}"),
            }
        })
        .collect();
    AlternativeNames { names }
}

/// The grammar node that owns a decision state.
#[derive(Clone, Copy)]
enum Owner<'m> {
    Rule(&'m Block),
    Element(&'m Element),
}

/// The rule block or the element that owns the decision at ATN state
/// `state`.
fn decision_owner(compiled: &CompiledParser, state: usize) -> Option<Owner<'_>> {
    let state = compiled.graph.states.get(state)?;
    let rules = &compiled.semantic.unit.rules;
    let rule = state.rule_index.and_then(|index| rules.get(index));
    // The immediate owner of a state is its last synthetic origin.
    let owner = compiled
        .provenance
        .state_origins(state.original)
        .iter()
        .rev()
        .find_map(|origin| match origin {
            Origin::Synthetic { owner, .. } => Some(*owner),
            _ => None,
        })?;
    match owner {
        ModelNodeId::Rule(id) => rule
            .filter(|rule| rule.id == id)
            .map(|rule| Owner::Rule(&rule.block)),
        ModelNodeId::Element(element) => rule
            .and_then(|rule| find_element(&rule.block, element))
            .map(Owner::Element),
        _ => None,
    }
}

/// The block or optional element that a decision with `count` alternatives
/// chooses in.
fn decision_shape(owner: Owner<'_>, count: usize) -> Option<Shape<'_>> {
    match owner {
        Owner::Rule(block) => Some(block_shape(block, count)),
        Owner::Element(element) => match &element.kind {
            ElementKind::Block(block) => Some(block_shape(block, count)),
            _ if count == 2 && matches!(element.quantifier, Quantifier::Optional { .. }) => {
                Some(Shape::Optional(element))
            }
            _ => None,
        },
    }
}

/// A short text of a loop body: the elements of a single alternative, or the
/// alternatives of a block.
fn loop_body_text(body: &Element) -> String {
    let ElementKind::Block(block) = &body.kind else {
        return element_text(&Element {
            quantifier: Quantifier::One,
            ..body.clone()
        });
    };
    let alternatives: Vec<String> = block
        .alternatives
        .iter()
        .map(|alternative| match &alternative.label {
            Some(label) => format!("#{}", label.value),
            None => elements_text(&alternative.elements),
        })
        .collect();
    match alternatives.as_slice() {
        [single] => single.clone(),
        many if many.len() <= NAME_ELEMENTS => format!("( {} )", many.join(" | ")),
        many => format!("( {} | ... )", many[..NAME_ELEMENTS].join(" | ")),
    }
}

/// The shape of a decision over `block` with `count` alternatives.
fn block_shape(block: &Block, count: usize) -> Shape<'_> {
    Shape::Block {
        alternatives: &block.alternatives,
        skip: count == block.alternatives.len() + 1,
    }
}

fn find_element(block: &Block, target: ElementId) -> Option<&Element> {
    block
        .alternatives
        .iter()
        .flat_map(|alternative| &alternative.elements)
        .find_map(|element| {
            if element.id == target {
                return Some(element);
            }
            match &element.kind {
                ElementKind::Block(inner) => find_element(inner, target),
                _ => None,
            }
        })
}

/// The label of an alternative, or else its first elements, with its line.
fn alternative_detail(alternative: &Alternative, sources: &SourceSet) -> Option<String> {
    if let Some(label) = &alternative.label {
        return Some(located(
            &format!("#{}", label.value),
            &alternative.span,
            sources,
        ));
    }
    Some(located(
        &elements_text(&alternative.elements),
        &alternative.span,
        sources,
    ))
}

/// The first elements of a sequence, `empty` for a sequence without text.
fn elements_text(elements: &[Element]) -> String {
    let texts: Vec<String> = elements
        .iter()
        .map(element_text)
        .filter(|text| !text.is_empty())
        .collect();
    if texts.is_empty() {
        return "empty".to_owned();
    }
    let mut text = texts
        .iter()
        .take(NAME_ELEMENTS)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if texts.len() > NAME_ELEMENTS {
        text.push_str(" ...");
    }
    text
}

/// `text` with the line where `span` starts, when the source is known.
fn located(text: &str, span: &SourceSpan, sources: &SourceSet) -> String {
    match sources.line_column(span.source, span.bytes.start) {
        Some((line, _)) => format!("{text}, line {line}"),
        None => text.to_owned(),
    }
}

/// A short text of one element: the rule or token it names, or a mark for a
/// block, a set, an action, or a predicate.
fn element_text(element: &Element) -> String {
    let base = match &element.kind {
        ElementKind::Terminal(terminal) => terminal_text(terminal),
        ElementKind::RuleCall(call) => call.name.clone(),
        ElementKind::Range(start, stop, _) => format!("{}..{}", start.value, stop.value),
        ElementKind::Set { inverted, elements } => {
            let members = elements
                .iter()
                .map(|member| match member {
                    SetElement::Terminal { value, .. } => terminal_text(value),
                    SetElement::Range { start, stop, .. } => format!("{start}..{stop}"),
                })
                .collect::<Vec<_>>();
            let set = if members.len() <= 2 {
                format!("({})", members.join(" | "))
            } else {
                format!("({} | ...)", members[..2].join(" | "))
            };
            if *inverted { format!("~{set}") } else { set }
        }
        ElementKind::Block(_) => "( ... )".to_owned(),
        // The left-recursion rewrite adds the precedence predicates.
        ElementKind::Predicate {
            precedence: Some(_),
            ..
        } => String::new(),
        ElementKind::Predicate { .. } => "{...}?".to_owned(),
        ElementKind::Action { .. } | ElementKind::Epsilon => String::new(),
    };
    let suffix = match element.quantifier {
        Quantifier::One => "",
        Quantifier::Optional { greedy: true } => "?",
        Quantifier::Optional { greedy: false } => "??",
        Quantifier::ZeroOrMore { greedy: true } => "*",
        Quantifier::ZeroOrMore { greedy: false } => "*?",
        Quantifier::OneOrMore { greedy: true } => "+",
        Quantifier::OneOrMore { greedy: false } => "+?",
    };
    if base.is_empty() {
        base
    } else {
        format!("{base}{suffix}")
    }
}

fn terminal_text(terminal: &Terminal) -> String {
    match terminal {
        Terminal::Token(name) => name.clone(),
        Terminal::Literal(literal) | Terminal::LexerCharSet(literal) => literal.clone(),
        Terminal::Wildcard => ".".to_owned(),
        Terminal::Eof => "EOF".to_owned(),
    }
}
