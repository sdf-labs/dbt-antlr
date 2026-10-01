// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Builds the dbt parser output model from a compiled parser grammar unit.
//!
//! This is a port of the Java tool's parser-path model construction:
//! `OutputModelController.buildParserOutputModel` /
//! `buildNormalRuleFunction`, `ParserFactory`, the `SourceGenTriggers.g`
//! rule walk, the `codegen.model` constructors (`RuleFunction`, the
//! `Choice` hierarchy, `TestSetInline`, `StructDecl`, the context getter
//! decls, `ElementFrequenciesVisitor`), built on the vendored `grammar`
//! analysis layer, which also provides the `LL1Analyzer` decision lookahead
//! (`grammar::atn::analysis`), plus the
//! left-recursive rule handling of `OutputModelController
//! .buildLeftRecursiveRuleFunction` (the `recRule*` op injection) and
//! `LeftRecursiveRule.getUnlabeledAltASTs`.
//!
//! Later milestones: rule args/locals, throws/catch/finally, list (`+=`)
//! labels, labels on sets and wildcards, non-local attribute references,
//! and user-authored sempreds are rejected with [`EmitError::Unsupported`]
//! for now.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::dbt::action_translator::{ActionScope, LabelDef, split_action, translate_action};
use crate::dbt::emit::EmitError;
use crate::dbt::lexer_factory::{
    ANTLR_VERSION, capitalize, collect_named_actions, escape_if_needed, rule_context_struct_name,
    target_string_literal_from_string, token_constants, translate_literal_names,
    translate_symbolic_names,
};
use crate::dbt::listener_factory::alt_labels;
use crate::dbt::model::{
    ActionChunkModel, BitsetModel, ChoiceModel, ChunkContextModel, DeclModel, IndexedActionModel,
    OpLabelModel, PackedAtnModel, ParserFileContext, ParserFileModel, ParserModel,
    RuleActionFunctionModel, RuleFunctionModel, RuleNamedActionsModel, RuleRefModel, SrcOpModel,
    StructDeclModel, TokenInfoModel,
};
use crate::grammar::atn::{
    AtnStateKind, CompiledParser, DecisionLookahead, FinalizedAtnGraph, FinalizedTransitionKind,
};
use crate::grammar::model::{
    Alternative, AlternativeId, Element, ElementKind, GrammarUnit, Label, LabelKind, ModelNodeId,
    PredicateId, Quantifier, RecognizerModel, Rule, RuleAttributes, RuleId, SetElement, Terminal,
};
use crate::grammar::provenance::Origin;
use crate::grammar::source::SourceSet;

/// `Token.EOF`.
const TOKEN_EOF: i32 = -1;
/// `RustTarget.getInlineTestSetWordSize`.
const INLINE_TEST_SET_WORD_SIZE: i32 = 32;

/// Builds the full `ParserFile` render context for one compiled parser unit.
///
/// `grammar_name` is the authored grammar name (`CSV`); `grammar_file_name`
/// is the name that lands in the `fileHeader` comment.
pub(crate) fn build_parser_file_context(
    compiled: &CompiledParser,
    sources: &SourceSet,
    grammar_name: &str,
    grammar_file_name: &str,
    gen_listener: bool,
    gen_visitor: bool,
) -> Result<ParserFileContext, EmitError> {
    let unit = &compiled.semantic.unit;
    let recognizer = &compiled.semantic.recognizer;
    let walk = WalkContext::new(
        &compiled.semantic.unit,
        recognizer,
        &compiled.graph,
        compiled,
        sources,
    );

    let mut funcs = Vec::new();
    let mut sempred_funcs = Vec::new();
    for (rule_index, rule) in unit.rules.iter().enumerate() {
        let (func, sempred_func) = build_rule_function(&walk, unit, rule, rule_index)?;
        funcs.push(func);
        if let Some(sempred_func) = sempred_func {
            sempred_funcs.push(sempred_func);
        }
    }

    let recognizer_name = if grammar_name.ends_with("Parser") {
        grammar_name.to_owned()
    } else {
        format!("{grammar_name}Parser")
    };

    let parser = ParserModel {
        name: recognizer_name,
        grammar_name: grammar_name.to_owned(),
        grammar_file_name: format!("{grammar_name}.g4"),
        tokens: token_constants(recognizer),
        rules: recognizer
            .rule_names
            .iter()
            .enumerate()
            .map(|(index, name)| RuleRefModel {
                name: name.clone(),
                index,
            })
            .collect(),
        rule_names: recognizer.rule_names.clone(),
        literal_names: translate_literal_names(&recognizer.literal_names),
        symbolic_names: translate_symbolic_names(&recognizer.symbolic_names),
        funcs,
        sempred_funcs,
        atn: PackedAtnModel {
            serialized: compiled.packed.packed_words().to_vec(),
        },
    };

    Ok(ParserFileContext {
        file: ParserFileModel {
            grammar_file_name: grammar_file_name.to_owned(),
            antlr_version: ANTLR_VERSION.to_owned(),
            grammar_name: grammar_name.to_owned(),
            gen_listener,
            gen_visitor,
        },
        parser,
        named_actions: collect_named_actions(unit, "parser"),
        context_super_class: None,
    })
}

/// Data the rule walk needs from the compiled grammar unit.
struct WalkContext<'a> {
    unit: &'a GrammarUnit,
    recognizer: &'a RecognizerModel,
    graph: &'a FinalizedAtnGraph,
    /// Synthetic block/loop states by owning model node and state kind.
    synthetic_states: BTreeMap<ModelNodeId, Vec<(AtnStateKind, usize)>>,
    /// Decision number of each decision state.
    decision_by_state: BTreeMap<usize, usize>,
    /// Token type to token name (`T__0`, `TEXT`, `EOF`), falling back to
    /// the numeric string, like `Grammar.getTokenName`.
    token_names: BTreeMap<i32, String>,
    /// `LL1Analyzer.getDecisionLookahead` results by decision state,
    /// precomputed by the grammar analysis.
    decision_looks: BTreeMap<usize, &'a DecisionLookahead>,
    /// Rule attributes (`args`/`returns`/`locals`) by rule.
    attributes: &'a BTreeMap<RuleId, RuleAttributes>,
    /// Return-value names of every rule, for `$x.y` resolution on rule
    /// references and rule labels.
    rule_retvals: BTreeMap<String, BTreeSet<String>>,
    /// The grammar source files (with token streams), for the
    /// `ImplicitSetLabel` token index.
    sources: &'a SourceSet,
}

impl<'a> WalkContext<'a> {
    fn new(
        unit: &'a GrammarUnit,
        recognizer: &'a RecognizerModel,
        graph: &'a FinalizedAtnGraph,
        compiled: &'a CompiledParser,
        sources: &'a SourceSet,
    ) -> Self {
        let mut synthetic_states: BTreeMap<ModelNodeId, Vec<(AtnStateKind, usize)>> =
            BTreeMap::new();
        for (state_number, state) in graph.states.iter().enumerate() {
            for origin in compiled.provenance.state_origins(state.original) {
                if let Origin::Synthetic { owner, .. } = origin {
                    synthetic_states
                        .entry(*owner)
                        .or_default()
                        .push((state.kind, state_number));
                }
            }
        }
        let decision_by_state = graph
            .decisions
            .iter()
            .enumerate()
            .map(|(decision, state)| (*state, decision))
            .collect();
        let mut token_names = BTreeMap::new();
        token_names.insert(TOKEN_EOF, "EOF".to_owned());
        for token in &recognizer.vocabulary.tokens {
            let name = token
                .name
                .clone()
                .unwrap_or_else(|| token.number.to_string());
            token_names.insert(token.number, name);
        }
        let decision_looks = compiled
            .analysis
            .decision_lookahead
            .iter()
            .map(|look| (look.state, look))
            .collect();
        let rule_retvals = unit
            .rules
            .iter()
            .map(|rule| {
                let retvals = compiled
                    .semantic
                    .bindings
                    .attributes
                    .get(&rule.id)
                    .map(|attributes| {
                        attributes
                            .returns
                            .iter()
                            .map(|attribute| attribute.name.clone())
                            .collect()
                    })
                    .unwrap_or_default();
                (rule.name.clone(), retvals)
            })
            .collect();
        Self {
            unit,
            recognizer,
            graph,
            synthetic_states,
            decision_by_state,
            token_names,
            decision_looks,
            attributes: &compiled.semantic.bindings.attributes,
            rule_retvals,
            sources,
        }
    }

    /// The state of `kind` created for `owner`, e.g. the `StarLoopEntry`
    /// state of a `(...)*` element.
    fn synthetic_state(&self, owner: ModelNodeId, kind: AtnStateKind) -> Result<usize, EmitError> {
        // `StarBlockStart`/`PlusBlockStart` are `BlockStart` subclasses in
        // the Java ATN; a multi-alternative `(...)*`/`(...)+` block takes
        // its alternative decision on the loop-flavored start state.
        let accepted: &[AtnStateKind] = match kind {
            AtnStateKind::BlockStart => &[
                AtnStateKind::BlockStart,
                AtnStateKind::StarBlockStart,
                AtnStateKind::PlusBlockStart,
            ],
            _ => &[kind],
        };
        self.synthetic_states
            .get(&owner)
            .and_then(|states| {
                accepted.iter().find_map(|wanted| {
                    states
                        .iter()
                        .find(|(state_kind, _)| state_kind == wanted)
                        .map(|(_, state)| *state)
                })
            })
            .ok_or_else(|| unsupported(format!("no {kind:?} state for {owner:?}")))
    }

    /// The ATN state of an atom element (the source state of its authored
    /// transition), the Java `ast.atnState.stateNumber`.
    fn element_state(&self, element: &Element) -> Result<i32, EmitError> {
        self.graph
            .transitions_for_model(ModelNodeId::Element(element.id))
            .next()
            .map(|transition| to_i32(transition.source))
            .ok_or_else(|| unsupported(format!("no ATN transition for element {:?}", element.id)))
    }

    fn decision(&self, state: usize) -> Result<usize, EmitError> {
        self.decision_by_state
            .get(&state)
            .copied()
            .ok_or_else(|| unsupported(format!("state {state} is not a decision state")))
    }

    fn token_name(&self, ttype: i32) -> String {
        self.token_names
            .get(&ttype)
            .cloned()
            .unwrap_or_else(|| ttype.to_string())
    }

    /// `LL1Analyzer.getDecisionLookahead`, precomputed by the grammar
    /// analysis (`analyze_parser`): per alternative of `decision_state`, in
    /// transition order, the LL(1) look set — `None` when the look is empty
    /// or hits a predicate.
    fn decision_look(&self, decision_state: usize) -> Result<&'a DecisionLookahead, EmitError> {
        self.decision_looks
            .get(&decision_state)
            .copied()
            .ok_or_else(|| unsupported(format!("state {decision_state} is not a decision state")))
    }

    /// `Target.getImplicitSetLabel`'s `<id>`: the index of the set's block
    /// token in the grammar's token stream (hidden-channel tokens included).
    /// The reduced `Set` element keeps the block's span, which starts at the
    /// `(` token; for a `~(...)` set the `~` token comes first and the block
    /// token follows.
    fn set_token_index(&self, element: &Element) -> Result<usize, EmitError> {
        let file = self
            .sources
            .get(element.span.source)
            .ok_or_else(|| unsupported("set element without a source file".to_owned()))?;
        let tokens = file.tokens();
        let mut index =
            tokens.partition_point(|token| token.span.bytes.start < element.span.bytes.start);
        if index >= tokens.len() || tokens[index].span.bytes.start != element.span.bytes.start {
            return Err(unsupported(format!(
                "no grammar token at the set element span of {:?}",
                element.id
            )));
        }
        if &file.text()
            [tokens[index].span.bytes.start as usize..tokens[index].span.bytes.end as usize]
            == "~"
        {
            index += 1;
        }
        Ok(index)
    }
}

const fn unsupported(message: String) -> EmitError {
    EmitError::Unsupported(message)
}

/// Builds one `RuleFunction` model, the Java
/// `OutputModelController.buildRuleFunction` (with
/// `buildLeftRecursiveRuleFunction` for left-recursive rules).
fn build_rule_function(
    walk: &WalkContext<'_>,
    unit: &GrammarUnit,
    rule: &Rule,
    rule_index: usize,
) -> Result<(RuleFunctionModel, Option<RuleActionFunctionModel>), EmitError> {
    let left_recursion = rule.left_recursion.is_some();
    if !rule.throws.is_empty() || !rule.catches.is_empty() || rule.finally_action.is_some() {
        return Err(unsupported(format!(
            "rule {} with throws/catch/finally (milestone 4.7)",
            rule.name
        )));
    }
    let mut ctxs = build_rule_contexts(walk, rule)?;
    let mut rule_walk = RuleWalk {
        walk,
        unit,
        rule,
        has_lookahead_block: false,
        locals: Vec::new(),
        local_names: BTreeSet::new(),
        ctxs: &mut ctxs,
        current_alt_label: None,
        scope: ActionScope::default(),
        sempred_actions: Vec::new(),
    };
    let mut code = rule_walk.rule_block(rule)?;
    if left_recursion {
        inject_left_recursion_ops(&mut code, rule)?;
    }
    // `RuleFunction.fillNamedActions`: the `@init`/`@after` actions are
    // translated in the scope left over from the rule walk (the last
    // outermost alternative), like the Java `Action` model.
    let mut named_actions = RuleNamedActionsModel::default();
    for action in &rule.actions {
        let slot = match action.name.as_str() {
            "init" => &mut named_actions.init,
            "after" => &mut named_actions.after,
            _ => continue,
        };
        let node_ctx = rule_walk
            .ctxs
            .ctx_escaped_name(rule_walk.current_alt_label.as_deref());
        let rule_ctx = rule_walk.ctxs.rule_ctx.escaped_name.clone();
        *slot = Some(translate_action(
            &action.body,
            &rule_walk.scope,
            &node_ctx,
            &rule_ctx,
        )?);
    }
    // `RuleSempredFunction`: one action per predicate of the rule, keyed
    // by the ATN predicate number (`g.sempreds`).
    let sempred_func = if rule_walk.sempred_actions.is_empty() {
        None
    } else {
        let mut actions = std::mem::take(&mut rule_walk.sempred_actions)
            .into_iter()
            .map(|(id, chunks)| {
                let index = walk
                    .recognizer
                    .predicate_numbers
                    .get(&id)
                    .copied()
                    .ok_or_else(|| unsupported(format!("no sempred index for predicate {id:?}")))?;
                Ok(IndexedActionModel { index, chunks })
            })
            .collect::<Result<Vec<_>, EmitError>>()?;
        actions.sort_by_key(|action| action.index);
        Some(RuleActionFunctionModel {
            name: rule.name.clone(),
            escaped_name: escape_if_needed(&rule.name),
            ctx_type: rule_context_struct_name(&rule.name),
            rule_index,
            is_lexer: false,
            actions,
        })
    };
    let has_lookahead_block = rule_walk.has_lookahead_block;
    let locals = std::mem::take(&mut rule_walk.locals);
    drop(rule_walk);
    Ok((
        RuleFunctionModel {
            name: rule.name.clone(),
            escaped_name: escape_if_needed(&rule.name),
            modifiers: rule
                .modifiers
                .iter()
                .map(|modifier| modifier.value.clone())
                .collect(),
            ctx_type: rule_context_struct_name(&rule.name),
            index: rule_index,
            start_state: to_i32(walk.graph.rule_starts[rule_index]),
            has_lookahead_block,
            left_recursive: left_recursion,
            args: ctxs.args,
            locals,
            code,
            rule_ctx: ctxs.rule_ctx,
            alt_label_ctxs: ctxs.alt_label_ctxs,
            named_actions,
            finally_action: None,
            postamble: rule_postamble(rule),
            exceptions: Vec::new(),
        },
        sempred_func,
    ))
}

/// `DefaultOutputModelFactory.rulePostamble`: the stop-token setter for
/// rules with an `@after` or `@finally` action.
fn rule_postamble(rule: &Rule) -> Vec<SrcOpModel> {
    let has_after = rule
        .actions
        .iter()
        .any(|action| action.name == "after" || action.name == "finally");
    if has_after || rule.finally_action.is_some() {
        vec![SrcOpModel::RecRuleSetStopToken]
    } else {
        Vec::new()
    }
}

/// Whether any nested alternative of the (possibly rewritten) rule block
/// carries an alt label.
fn alternative_has_label(alternative: &Alternative) -> bool {
    alternative.elements.iter().any(|element| {
        if let ElementKind::Block(block) = &element.kind {
            block.alternatives.iter().any(|alternative| {
                alternative.label.is_some() || alternative_has_label(alternative)
            })
        } else {
            false
        }
    })
}

/// The `recRuleAltPredicate` template text of `Rust.stg`:
/// `recog.precpred(None, <opPrec>)`.
fn rec_rule_alt_predicate_text(precedence: u32) -> String {
    format!("recog.precpred(None, {precedence})")
}

/// `OutputModelController.buildLeftRecursiveRuleFunction`: injects the
/// recursion-context ops into the walked code of a rewritten rule — the
/// `recRuleReplaceContext` at the start of each labeled primary
/// alternative, the stop-token setter after the primary alternatives, the
/// `recRuleSetPrevCtx` iteration op, and the `recRuleAltStartAction` /
/// `recRuleLabeledAltStartAction` at the start of each operator
/// alternative.
fn inject_left_recursion_ops(code: &mut [SrcOpModel], rule: &Rule) -> Result<(), EmitError> {
    let unexpected = || {
        unsupported(format!(
            "left-recursive rule {} with an unexpected rewritten shape",
            rule.name
        ))
    };
    let (primary_labels, op_alt_infos) = left_recursion_alt_infos(rule)?;
    let Some(SrcOpModel::CodeBlockForOuterMostAlt { ops, .. }) = code.first_mut() else {
        return Err(unexpected());
    };
    if ops.len() != 2 {
        return Err(unexpected());
    }
    // The primary alternatives: one code block per labeled primary.
    let mut primary_alts_code = match &mut ops[0] {
        SrcOpModel::LL1AltBlock { alts, .. } | SrcOpModel::AltBlock { alts, .. } => {
            alts.iter_mut().collect::<Vec<_>>()
        }
        single @ SrcOpModel::CodeBlockForAlt { .. } => vec![single],
        _ => return Err(unexpected()),
    };
    for (alt, label) in primary_alts_code.iter_mut().zip(primary_labels) {
        let Some(label) = label else {
            continue;
        };
        let SrcOpModel::CodeBlockForAlt { ops, .. } = alt else {
            return Err(unexpected());
        };
        ops.insert(
            0,
            SrcOpModel::RecRuleReplaceContext {
                ctx_name: capitalize(&label),
            },
        );
    }
    // After the primary alternatives, before the operator loop.
    ops.insert(1, SrcOpModel::RecRuleSetStopToken);
    let SrcOpModel::StarBlock {
        alts, iteration, ..
    } = &mut ops[2]
    else {
        return Err(unexpected());
    };
    iteration.push(SrcOpModel::RecRuleSetPrevCtx);
    let Some(SrcOpModel::CodeBlockForAlt { ops: wrapper, .. }) = alts.first_mut() else {
        return Err(unexpected());
    };
    let ctx_name = capitalize(&rule.name);
    let insert_start_action = |ops: &mut Vec<SrcOpModel>,
                               alt_info: &(Option<String>, Option<Label>)|
     -> Result<(), EmitError> {
        let (alt_label, deleted_label) = alt_info;
        let op = alt_label.as_ref().map_or_else(
            || SrcOpModel::RecRuleAltStartAction {
                rule_name: rule.name.clone(),
                ctx_name: ctx_name.clone(),
                label: deleted_label.as_ref().map(|label| label.name.clone()),
                is_list_label: deleted_label
                    .as_ref()
                    .is_some_and(|label| label.kind == LabelKind::List),
            },
            |alt_label| SrcOpModel::RecRuleLabeledAltStartAction {
                rule_name: rule.name.clone(),
                current_alt_label: alt_label.clone(),
                label: deleted_label.as_ref().map(|label| label.name.clone()),
                is_list_label: deleted_label
                    .as_ref()
                    .is_some_and(|label| label.kind == LabelKind::List),
            },
        );
        ops.insert(0, op);
        Ok(())
    };
    let mut op_alt_infos = op_alt_infos.iter();
    match wrapper.first_mut() {
        // Multiple operator alternatives: one code block per alternative.
        Some(SrcOpModel::AltBlock { alts, .. }) => {
            for alt in alts {
                let SrcOpModel::CodeBlockForAlt { ops, .. } = alt else {
                    return Err(unexpected());
                };
                let Some(alt_info) = op_alt_infos.next() else {
                    return Err(unexpected());
                };
                insert_start_action(ops, alt_info)?;
            }
        }
        // A single operator alternative: no AltBlock wrapper.
        Some(SrcOpModel::CodeBlockForAlt { ops, .. }) => {
            let Some(alt_info) = op_alt_infos.next() else {
                return Err(unexpected());
            };
            insert_start_action(ops, alt_info)?;
        }
        _ => return Err(unexpected()),
    }
    Ok(())
}

/// The alt-label metadata of a rewritten left-recursive rule: the alt labels
/// of the primary alternatives in rewritten order, and per operator
/// alternative its alt label plus the label of the deleted leading recursive
/// reference.
type LeftRecursionAltInfos = (Vec<Option<String>>, Vec<(Option<String>, Option<Label>)>);

/// The label metadata of a rewritten left-recursive rule: the alt labels of
/// the primary alternatives in rewritten order, and per operator
/// alternative its alt label plus the label the original alternative put on
/// the deleted leading recursive reference, the Java
/// `LeftRecursiveRuleAltInfo.altLabel` / `leftRecursiveRuleRefLabel` /
/// `isListLabel`.
fn left_recursion_alt_infos(rule: &Rule) -> Result<LeftRecursionAltInfos, EmitError> {
    let unexpected = || {
        unsupported(format!(
            "left-recursive rule {} with an unexpected rewritten shape",
            rule.name
        ))
    };
    let Some(info) = &rule.left_recursion else {
        return Err(unexpected());
    };
    let Some(outer) = rule.block.alternatives.first() else {
        return Err(unexpected());
    };
    let [primary_element, operator_element] = &outer.elements[..] else {
        return Err(unexpected());
    };
    let ElementKind::Block(primary_block) = &primary_element.kind else {
        return Err(unexpected());
    };
    let ElementKind::Block(operator_block) = &operator_element.kind else {
        return Err(unexpected());
    };
    let rewritten_by_original: BTreeMap<AlternativeId, AlternativeId> = info
        .original_to_rewritten
        .iter()
        .map(|(original, rewritten)| (*rewritten, *original))
        .collect();
    let deleted_label_of = |alternative_id: AlternativeId| -> Option<Label> {
        let original = rewritten_by_original.get(&alternative_id)?;
        info.deleted_labels
            .values()
            .find(|removed| removed.original_alternative == *original)
            .map(|removed| removed.label.clone())
    };
    let primary_labels = primary_block
        .alternatives
        .iter()
        .map(|alternative| alternative.label.as_ref().map(|label| label.value.clone()))
        .collect();
    let op_alt_infos = operator_block
        .alternatives
        .iter()
        .map(|alternative| {
            (
                alternative.label.as_ref().map(|label| label.value.clone()),
                deleted_label_of(alternative.id),
            )
        })
        .collect();
    Ok((primary_labels, op_alt_infos))
}

/// The `SourceGenTriggers` walk state for one rule.
struct RuleWalk<'a, 'b> {
    walk: &'b WalkContext<'a>,
    unit: &'b GrammarUnit,
    rule: &'b Rule,
    has_lookahead_block: bool,
    locals: Vec<DeclModel>,
    local_names: BTreeSet<String>,
    /// The context structs label decls are routed to.
    ctxs: &'b mut RuleContexts,
    /// The alt label of the innermost enclosing labeled alternative
    /// (`GrammarAST.getAltLabel`).
    current_alt_label: Option<String>,
    /// The attribute scope of the current outer-most alternative.
    scope: ActionScope,
    /// Translated sempred chunks by predicate id, in walk order: the
    /// `RuleSempredFunction` actions share the inline op's chunks (the
    /// `SemPred` template's "hack" comment).
    sempred_actions: Vec<(PredicateId, Vec<ActionChunkModel>)>,
}

impl RuleWalk<'_, '_> {
    /// `TokenTypeDecl` local registration (`RuleFunction.addLocalDecl`).
    fn add_token_type_local(&mut self, name: &str) {
        if self.local_names.insert(name.to_owned()) {
            self.locals.push(DeclModel::TokenTypeDecl {
                name: name.to_owned(),
                escaped_name: escape_if_needed(name),
            });
        }
    }

    /// `dummy : block[null, null]` on the rule's block.
    fn rule_block(&mut self, rule: &Rule) -> Result<Vec<SrcOpModel>, EmitError> {
        let owner = ModelNodeId::Rule(rule.id);
        let alts = self.alternatives(&rule.block.alternatives, true)?;
        if alts.len() == 1 {
            return Ok(alts);
        }
        Ok(vec![self.choice_block(owner, alts)?])
    }

    /// `block[label, ebnfRoot]`.
    fn block(
        &mut self,
        block: &crate::grammar::model::Block,
        ebnf_root: Option<&Element>,
        owner: ModelNodeId,
        outermost: bool,
    ) -> Result<Vec<SrcOpModel>, EmitError> {
        let alts = self.alternatives(&block.alternatives, outermost)?;
        if alts.len() == 1 && ebnf_root.is_none() {
            return Ok(alts);
        }
        match ebnf_root {
            None => Ok(vec![self.choice_block(owner, alts)?]),
            Some(root) => {
                let kind = ebnf_kind(root.quantifier);
                let choice = self.ebnf_block(root, kind, alts)?;
                self.has_lookahead_block |= matches!(
                    choice,
                    SrcOpModel::StarBlock { .. } | SrcOpModel::PlusBlock { .. }
                );
                Ok(vec![choice])
            }
        }
    }

    /// `alternative`/`alt`: one code block per alternative. For outer-most
    /// alternatives this (re)builds the attribute scope of the alternative
    /// and resets the enclosing alt label; nested labeled alternatives
    /// shadow it for the duration of their walk.
    fn alternatives(
        &mut self,
        alternatives: &[Alternative],
        outermost: bool,
    ) -> Result<Vec<SrcOpModel>, EmitError> {
        alternatives
            .iter()
            .enumerate()
            .map(|(index, alternative)| {
                let saved_alt_label = self.current_alt_label.take();
                if outermost {
                    self.scope = self.build_action_scope(alternative);
                    self.current_alt_label =
                        alternative.label.as_ref().map(|label| label.value.clone());
                } else if let Some(label) = &alternative.label {
                    self.current_alt_label = Some(label.value.clone());
                } else {
                    self.current_alt_label.clone_from(&saved_alt_label);
                }
                let result = (|| {
                    let mut ops = Vec::new();
                    for element in &alternative.elements {
                        ops.extend(self.element(element)?);
                    }
                    if outermost {
                        Ok(SrcOpModel::CodeBlockForOuterMostAlt {
                            alt_num: index + 1,
                            alt_label: alternative.label.as_ref().map(|label| label.value.clone()),
                            locals: Vec::new(),
                            preamble: Vec::new(),
                            ops,
                        })
                    } else {
                        Ok(SrcOpModel::CodeBlockForAlt {
                            locals: Vec::new(),
                            preamble: Vec::new(),
                            ops,
                        })
                    }
                })();
                self.current_alt_label = saved_alt_label;
                result
            })
            .collect()
    }

    /// `element`: quantified single (non-block) elements are wrapped the way
    /// the Java tool's `^(op ^(BLOCK ^(ALT el)))` tree walk wraps them:
    /// closures get the op's block code block inside the alternative code
    /// block; optionals get the op directly in the alternative code block.
    fn element(&mut self, element: &Element) -> Result<Vec<SrcOpModel>, EmitError> {
        if !matches!(element.quantifier, Quantifier::One)
            && !matches!(element.kind, ElementKind::Block(_))
        {
            let bare = self.bare_element(element)?;
            let inner = SrcOpModel::CodeBlockForAlt {
                locals: Vec::new(),
                preamble: Vec::new(),
                ops: bare,
            };
            let kind = ebnf_kind(element.quantifier);
            let alts = match kind {
                EbnfKind::Optional => vec![inner],
                EbnfKind::Closure | EbnfKind::PositiveClosure => {
                    vec![SrcOpModel::CodeBlockForAlt {
                        locals: Vec::new(),
                        preamble: Vec::new(),
                        ops: vec![inner],
                    }]
                }
            };
            let choice = self.ebnf_block(element, kind, alts)?;
            self.has_lookahead_block |= matches!(
                choice,
                SrcOpModel::StarBlock { .. } | SrcOpModel::PlusBlock { .. }
            );
            return Ok(vec![choice]);
        }
        self.bare_element(element)
    }

    /// `element` of the Java walk, restricted to unquantified elements and
    /// blocks (which carry their own quantifier handling).
    fn bare_element(&mut self, element: &Element) -> Result<Vec<SrcOpModel>, EmitError> {
        match &element.kind {
            ElementKind::RuleCall(_) => self.invoke_rule(element),
            ElementKind::Terminal(terminal) => match terminal {
                Terminal::Token(_) | Terminal::Literal(_) | Terminal::Eof => {
                    self.match_token(element, terminal)
                }
                Terminal::Wildcard => Ok(vec![SrcOpModel::Wildcard {
                    state_number: self.walk.element_state(element)?,
                    labels: Vec::new(),
                }]),
                Terminal::LexerCharSet(_) => Err(unsupported(
                    "lexer character set in a parser rule".to_owned(),
                )),
            },
            ElementKind::Set { inverted, .. } => self.match_set(element, *inverted),
            ElementKind::Block(block) => {
                let owner = ModelNodeId::Element(element.id);
                match element.quantifier {
                    Quantifier::One => self.block(block, None, owner, false),
                    Quantifier::Optional { .. } => self.block(block, Some(element), owner, false),
                    Quantifier::ZeroOrMore { .. } | Quantifier::OneOrMore { .. } => {
                        // `subrule : ^(op=CLOSURE b=block[null,null])`:
                        // the inner block walk, wrapped in a code block.
                        let inner = self.block(block, None, owner, false)?;
                        let mut inner = inner.into_iter();
                        let Some(first) = inner.next() else {
                            return Err(unsupported("empty block under a closure".to_owned()));
                        };
                        let wrapper = SrcOpModel::CodeBlockForAlt {
                            locals: Vec::new(),
                            preamble: Vec::new(),
                            ops: vec![first],
                        };
                        let kind = ebnf_kind(element.quantifier);
                        let choice = self.ebnf_block(element, kind, vec![wrapper])?;
                        self.has_lookahead_block |= matches!(
                            choice,
                            SrcOpModel::StarBlock { .. } | SrcOpModel::PlusBlock { .. }
                        );
                        Ok(vec![choice])
                    }
                }
            }
            ElementKind::Action { body, .. } => {
                // `ParserFactory.action` / `Action`: the action text,
                // translated into chunks. The left-recursion transform
                // prepends an empty action to the first primary
                // alternative; like every action it becomes an `Action` op
                // (its empty chunk list renders as nothing).
                let node_ctx = self
                    .ctxs
                    .ctx_escaped_name(self.current_alt_label.as_deref());
                let rule_ctx = self.ctxs.rule_ctx.escaped_name.clone();
                let chunks = if body.is_empty() {
                    Vec::new()
                } else {
                    translate_action(body, &self.scope, &node_ctx, &rule_ctx)?
                };
                Ok(vec![SrcOpModel::Action { chunks }])
            }
            ElementKind::Predicate {
                id,
                body,
                fail,
                precedence,
            } => {
                // An empty body is a left-recursion precedence predicate;
                // its text is the synthesized `recRuleAltPredicate`.
                let text = if body.is_empty() {
                    let Some(precedence) = precedence else {
                        return Err(unsupported(format!(
                            "sempred without precedence in rule {}",
                            self.rule.name
                        )));
                    };
                    rec_rule_alt_predicate_text(*precedence)
                } else {
                    body.clone()
                };
                let node_ctx = self
                    .ctxs
                    .ctx_escaped_name(self.current_alt_label.as_deref());
                let rule_ctx = self.ctxs.rule_ctx.escaped_name.clone();
                let chunks = translate_action(&text, &self.scope, &node_ctx, &rule_ctx)?;
                self.sempred_actions.push((*id, chunks.clone()));
                // `SemPred`: the `fail` option is either an action
                // (`fail={...}`) or a string literal (`fail='...'`).
                let (fail_chunks, msg) = match fail {
                    Some(fail) if fail.starts_with('{') => (
                        Some(translate_action(
                            fail.trim_start_matches('{').trim_end_matches('}'),
                            &self.scope,
                            &node_ctx,
                            &rule_ctx,
                        )?),
                        None,
                    ),
                    // The `fail` text arrives without the literal's quotes.
                    Some(fail) => (None, Some(target_string_literal_from_string(fail))),
                    None => (None, None),
                };
                Ok(vec![SrcOpModel::SemPred {
                    state_number: self.walk.element_state(element)?,
                    predicate: target_string_literal_from_string(&text),
                    chunks,
                    fail_chunks,
                    msg,
                }])
            }
            ElementKind::Range(..) => Err(unsupported("token ranges in parser rules".to_owned())),
            ElementKind::Epsilon => Ok(Vec::new()),
        }
    }

    /// `getChoiceBlock` for a plain `(A | B | C)` block.
    fn choice_block(
        &self,
        owner: ModelNodeId,
        alts: Vec<SrcOpModel>,
    ) -> Result<SrcOpModel, EmitError> {
        let state = self.walk.synthetic_state(owner, AtnStateKind::BlockStart)?;
        let decision = self.walk.decision(state)?;
        let look = self.walk.decision_look(state)?;
        if look.disjoint {
            let mut choice = choice_model(state, decision);
            choice.alt_look = self.alt_look_token_lists(&look.alternatives);
            Ok(SrcOpModel::LL1AltBlock {
                choice,
                preamble: Vec::new(),
                alts,
                error: Box::new(SrcOpModel::ThrowNoViableAlt),
            })
        } else {
            Ok(SrcOpModel::AltBlock {
                choice: choice_model(state, decision),
                preamble: Vec::new(),
                alts,
            })
        }
    }

    /// `getEBNFBlock` / `getLL1EBNFBlock` / `getComplexEBNFBlock`.
    ///
    /// A non-greedy decision is never LL(1): Java's `AnalysisPipeline`
    /// gives it an all-null lookahead, so it always takes the complex path.
    fn ebnf_block(
        &mut self,
        root: &Element,
        kind: EbnfKind,
        alts: Vec<SrcOpModel>,
    ) -> Result<SrcOpModel, EmitError> {
        let owner = ModelNodeId::Element(root.id);
        let greedy = ebnf_greedy(root.quantifier);
        match kind {
            EbnfKind::Optional => {
                let state = self.walk.synthetic_state(owner, AtnStateKind::BlockStart)?;
                let decision = self.walk.decision(state)?;
                let look = self.walk.decision_look(state)?;
                if greedy && look.disjoint {
                    if alts.len() == 1 {
                        let Some(enter_look) = &look.alternatives[0] else {
                            return Err(unsupported(
                                "optional block with empty enter look".to_owned(),
                            ));
                        };
                        let follow_look = look
                            .alternatives
                            .get(1)
                            .cloned()
                            .flatten()
                            .unwrap_or_default();
                        self.add_token_type_local("_la");
                        Ok(SrcOpModel::LL1OptionalBlockSingleAlt {
                            choice: choice_model(state, decision),
                            expr: Box::new(self.test_set_inline(enter_look)),
                            alts,
                            preamble: vec![capture_next_token_type()],
                            error: Box::new(SrcOpModel::ThrowNoViableAlt),
                            follow_expr: Box::new(self.test_set_inline(&follow_look)),
                        })
                    } else {
                        let mut choice = choice_model(state, decision);
                        choice.alt_look = self.alt_look_token_lists(&look.alternatives);
                        Ok(SrcOpModel::LL1OptionalBlock {
                            choice,
                            alts,
                            error: Box::new(SrcOpModel::ThrowNoViableAlt),
                        })
                    }
                } else {
                    let mut choice = choice_model(state, decision);
                    choice.greedy = greedy;
                    Ok(SrcOpModel::OptionalBlock { choice, alts })
                }
            }
            EbnfKind::Closure => {
                let entry = self
                    .walk
                    .synthetic_state(owner, AtnStateKind::StarLoopEntry)?;
                let loop_back = self
                    .walk
                    .synthetic_state(owner, AtnStateKind::StarLoopBack)?;
                let decision = self.walk.decision(entry)?;
                let look = self.walk.decision_look(entry)?;
                let mut choice = choice_model(entry, decision);
                choice.loop_back_state_number = to_i32(loop_back);
                choice.exit_alt = if greedy { to_i32(alts.len()) + 1 } else { 1 };
                choice.greedy = greedy;
                if greedy && look.disjoint && alts.len() == 1 {
                    let Some(enter_look) = &look.alternatives[0] else {
                        return Err(unsupported("star block with empty enter look".to_owned()));
                    };
                    self.add_token_type_local("_la");
                    Ok(SrcOpModel::LL1StarBlockSingleAlt {
                        choice,
                        loop_expr: Box::new(self.test_set_inline(enter_look)),
                        alts,
                        preamble: vec![capture_next_token_type()],
                        iteration: vec![capture_next_token_type()],
                    })
                } else {
                    Ok(SrcOpModel::StarBlock {
                        choice,
                        alts,
                        iteration: Vec::new(),
                    })
                }
            }
            EbnfKind::PositiveClosure => {
                let block_start = self
                    .walk
                    .synthetic_state(owner, AtnStateKind::PlusBlockStart)?;
                let loop_back = self
                    .walk
                    .synthetic_state(owner, AtnStateKind::PlusLoopBack)?;
                let decision = self.walk.decision(loop_back)?;
                let look = self.walk.decision_look(loop_back)?;
                let mut choice = choice_model(loop_back, decision);
                choice.block_start_state_number = to_i32(block_start);
                choice.loop_back_state_number = to_i32(loop_back);
                choice.exit_alt = if greedy { to_i32(alts.len()) + 1 } else { 1 };
                choice.greedy = greedy;
                if greedy && look.disjoint && alts.len() == 1 {
                    let Some(loop_back_look) = &look.alternatives[0] else {
                        return Err(unsupported(
                            "plus block with empty loopback look".to_owned(),
                        ));
                    };
                    self.add_token_type_local("_la");
                    Ok(SrcOpModel::LL1PlusBlockSingleAlt {
                        choice,
                        loop_expr: Box::new(self.test_set_inline(loop_back_look)),
                        alts,
                        preamble: vec![capture_next_token_type()],
                        iteration: vec![capture_next_token_type()],
                    })
                } else {
                    Ok(SrcOpModel::PlusBlock {
                        choice,
                        alts,
                        error: Box::new(SrcOpModel::ThrowNoViableAlt),
                    })
                }
            }
        }
    }

    /// `InvokeRule` (without argument handling for now). The precedence
    /// option (`r.ast.options.p` in the Java walk) selects the `_rec`
    /// entry point of a left-recursive rule. Explicit `x=r` labels and
    /// implicit labels (`$r` referenced from an action of the alternative)
    /// add a `RuleContextDecl` to the op and to the owning context struct.
    fn invoke_rule(&mut self, element: &Element) -> Result<Vec<SrcOpModel>, EmitError> {
        let ElementKind::RuleCall(call) = &element.kind else {
            unreachable!()
        };
        // `InvokeRule.argExprsChunks`: the call arguments, translated in
        // the enclosing alternative's scope.
        let arg_exprs_chunks = match &call.arguments {
            Some(arguments) => {
                let node_ctx = self
                    .ctxs
                    .ctx_escaped_name(self.current_alt_label.as_deref());
                let rule_ctx = self.ctxs.rule_ctx.escaped_name.clone();
                translate_action(arguments, &self.scope, &node_ctx, &rule_ctx)?
            }
            None => Vec::new(),
        };
        let ctx_name = rule_context_struct_name(&call.name);
        let mut labels = Vec::new();
        let mut list_label = None;
        if let Some(label) = &element.label {
            if label.kind == LabelKind::List {
                // `PLUS_ASSIGN`: the op assigns an implicit label named by
                // the rule (`defineImplicitLabel`); the list decl itself
                // goes to the context struct.
                let decl = DeclModel::RuleContextDecl {
                    name: call.name.clone(),
                    escaped_name: escape_if_needed(&call.name),
                    ctx_name: ctx_name.clone(),
                };
                self.add_op_label(&mut labels, &decl);
                list_label = Some(label.name.clone());
            } else {
                let decl = DeclModel::RuleContextDecl {
                    name: label.name.clone(),
                    escaped_name: escape_if_needed(&label.name),
                    ctx_name: ctx_name.clone(),
                };
                self.add_op_label(&mut labels, &decl);
            }
        }
        // `InvokeRule`'s constructor adds an implicit label whenever an
        // action references the rule by name, with or without a manual
        // label; `OrderedHashSet` dedups a name collision with the manual
        // label. The `needsImplicitLabel` fallback (`tokenRefsInActions`)
        // applies only when the op has no labels at all.
        if self.scope.rule_refs_in_actions.contains(&call.name)
            && !labels.iter().any(|label| label.name == call.name)
            || labels.is_empty() && self.scope.token_refs_in_actions.contains(&call.name)
        {
            // `defineImplicitLabel` for `$r` references in actions.
            let decl = DeclModel::RuleContextDecl {
                name: call.name.clone(),
                escaped_name: escape_if_needed(&call.name),
                ctx_name: ctx_name.clone(),
            };
            self.add_op_label(&mut labels, &decl);
        }
        let mut ops = vec![SrcOpModel::InvokeRule {
            name: call.name.clone(),
            escaped_name: escape_if_needed(&call.name),
            state_number: self.walk.element_state(element)?,
            ctx_name: ctx_name.clone(),
            precedence: call.precedence,
            labels: labels.clone(),
            arg_exprs_chunks,
        }];
        if let Some(list_name) = list_label {
            // `getAddToListOpIfListLabelPresent`: the list decl lives on the
            // context struct only; the op pushes the first label's value.
            let list_decl = DeclModel::RuleContextListDecl {
                name: list_name.clone(),
                escaped_name: escape_if_needed(&list_name),
                ctx_name,
            };
            self.ctxs
                .add_context_decl(self.current_alt_label.as_deref(), &list_decl);
            ops.push(SrcOpModel::AddToLabelList {
                label: labels[0].clone(),
                list_name: escape_if_needed(&list_name),
            });
        }
        Ok(ops)
    }

    /// Routes a label decl to the owning context struct
    /// (`RuleFunction.addContextDecl`) and records it on the op.
    fn add_op_label(&mut self, labels: &mut Vec<OpLabelModel>, decl: &DeclModel) {
        let (name, escaped_name) = match decl {
            DeclModel::TokenDecl {
                name, escaped_name, ..
            }
            | DeclModel::RuleContextDecl {
                name, escaped_name, ..
            } => (name.clone(), escaped_name.clone()),
            _ => unreachable!("label decl"),
        };
        let ctx = self
            .ctxs
            .add_context_decl(self.current_alt_label.as_deref(), decl);
        labels.push(OpLabelModel {
            name,
            escaped_name,
            ctx: ChunkContextModel::new(ctx),
        });
    }

    /// `MatchToken` for token, literal, and EOF references. Explicit `x=T`
    /// labels and implicit labels (`$T`/`$T.text` referenced from an action
    /// of the alternative) add a `TokenDecl` to the op and to the owning
    /// context struct.
    fn match_token(
        &mut self,
        element: &Element,
        terminal: &Terminal,
    ) -> Result<Vec<SrcOpModel>, EmitError> {
        let authored_text = match terminal {
            Terminal::Token(name) => name.clone(),
            Terminal::Literal(literal) => literal.clone(),
            Terminal::Eof => "EOF".to_owned(),
            Terminal::Wildcard | Terminal::LexerCharSet(_) => unreachable!(),
        };
        let ttype = match terminal {
            Terminal::Eof => TOKEN_EOF,
            Terminal::Token(_) | Terminal::Literal(_) => {
                self.walk.semantic_terminal_type(element).ok_or_else(|| {
                    unsupported(format!("no terminal binding for element {:?}", element.id))
                })?
            }
            Terminal::Wildcard | Terminal::LexerCharSet(_) => unreachable!(),
        };
        let mut labels = Vec::new();
        let mut list_label = None;
        if let Some(label) = &element.label {
            if label.kind == LabelKind::List {
                // `PLUS_ASSIGN`: the op assigns the implicit token label
                // (`getImplicitTokenLabel`: `s<ttype>` for literals, the
                // token name otherwise); the list decl goes to the context
                // struct.
                let implicit = match terminal {
                    Terminal::Literal(_) => format!("s{ttype}"),
                    _ => self.walk.token_name(ttype),
                };
                let decl = DeclModel::TokenDecl {
                    escaped_name: escape_if_needed(&implicit),
                    name: implicit,
                };
                self.add_op_label(&mut labels, &decl);
                list_label = Some(label.name.clone());
            } else {
                let decl = DeclModel::TokenDecl {
                    name: label.name.clone(),
                    escaped_name: escape_if_needed(&label.name),
                };
                self.add_op_label(&mut labels, &decl);
            }
        }
        // `needsImplicitLabel`: both action-reference maps qualify.
        if labels.is_empty()
            && (self.scope.token_refs_in_actions.contains(&authored_text)
                || self.scope.rule_refs_in_actions.contains(&authored_text))
        {
            // `defineImplicitLabel` for `$T` references in actions.
            let decl = DeclModel::TokenDecl {
                name: authored_text.clone(),
                escaped_name: escape_if_needed(&authored_text),
            };
            self.add_op_label(&mut labels, &decl);
        }
        let name = self.walk.token_name(ttype);
        let mut ops = vec![SrcOpModel::MatchToken {
            state_number: self.walk.element_state(element)?,
            escaped_name: escape_if_needed(&name),
            name,
            labels: labels.clone(),
        }];
        if let Some(list_name) = list_label {
            // `getAddToListOpIfListLabelPresent`: the list decl lives on the
            // context struct only; the op pushes the first label's value.
            let list_decl = DeclModel::TokenListDecl {
                name: list_name.clone(),
                escaped_name: escape_if_needed(&list_name),
            };
            self.ctxs
                .add_context_decl(self.current_alt_label.as_deref(), &list_decl);
            ops.push(SrcOpModel::AddToLabelList {
                label: labels[0].clone(),
                list_name: escape_if_needed(&list_name),
            });
        }
        Ok(ops)
    }

    /// `MatchSet` / `MatchNotSet`.
    fn match_set(
        &mut self,
        element: &Element,
        inverted: bool,
    ) -> Result<Vec<SrcOpModel>, EmitError> {
        // `SourceGenTriggers.blockSet` (`^(SET atom[label,invert]+)`): the
        // set members are walked as atoms carrying the SAME label; their
        // ops are discarded but the context decls stay, so e.g.
        // `val+=(INT|FLOAT)*` gets the member fields `INT`, `val`, `FLOAT`
        // ahead of the set's own `_tset26`.
        let ElementKind::Set {
            elements: members, ..
        } = &element.kind
        else {
            unreachable!()
        };
        for member in members {
            let SetElement::Terminal {
                value: terminal, ..
            } = member
            else {
                continue;
            };
            let (authored_text, ttype) = match terminal {
                Terminal::Token(name) => (
                    name.clone(),
                    self.walk.recognizer.vocabulary.by_name.get(name).copied(),
                ),
                Terminal::Literal(literal) => (
                    literal.clone(),
                    self.walk
                        .recognizer
                        .vocabulary
                        .by_literal
                        .get(literal)
                        .copied(),
                ),
                _ => continue,
            };
            let Some(ttype) = ttype else {
                continue;
            };
            let implicit = match terminal {
                Terminal::Literal(_) => format!("s{ttype}"),
                _ => self.walk.token_name(ttype),
            };
            match &element.label {
                Some(label) if label.kind == LabelKind::List => {
                    // The member's discarded `MatchToken` gets the implicit
                    // token label; the list decl is per label, deduped.
                    self.ctxs.add_context_decl(
                        self.current_alt_label.as_deref(),
                        &DeclModel::TokenDecl {
                            escaped_name: escape_if_needed(&implicit),
                            name: implicit,
                        },
                    );
                    self.ctxs.add_context_decl(
                        self.current_alt_label.as_deref(),
                        &DeclModel::TokenListDecl {
                            name: label.name.clone(),
                            escaped_name: escape_if_needed(&label.name),
                        },
                    );
                }
                Some(label) => {
                    self.ctxs.add_context_decl(
                        self.current_alt_label.as_deref(),
                        &DeclModel::TokenDecl {
                            name: label.name.clone(),
                            escaped_name: escape_if_needed(&label.name),
                        },
                    );
                }
                None => {
                    // `needsImplicitLabel` on the discarded member op.
                    if self.scope.token_refs_in_actions.contains(&authored_text)
                        || self.scope.rule_refs_in_actions.contains(&authored_text)
                    {
                        self.ctxs.add_context_decl(
                            self.current_alt_label.as_deref(),
                            &DeclModel::TokenDecl {
                                escaped_name: escape_if_needed(&implicit),
                                name: implicit,
                            },
                        );
                    }
                }
            }
        }
        let mut labels = Vec::new();
        let mut list_label = None;
        if let Some(label) = &element.label {
            if label.kind == LabelKind::List {
                // `PLUS_ASSIGN`: the op assigns the implicit set label
                // (`getImplicitSetLabel`: `_tset<block token index>`); the
                // list decl goes to the context struct.
                let implicit = format!("_tset{}", self.walk.set_token_index(element)?);
                let decl = DeclModel::TokenDecl {
                    escaped_name: escape_if_needed(&implicit),
                    name: implicit,
                };
                self.add_op_label(&mut labels, &decl);
                list_label = Some(label.name.clone());
            } else {
                let decl = DeclModel::TokenDecl {
                    name: label.name.clone(),
                    escaped_name: escape_if_needed(&label.name),
                };
                self.add_op_label(&mut labels, &decl);
            }
        }
        let transition = self
            .walk
            .graph
            .transitions_for_model(ModelNodeId::Element(element.id))
            .next()
            .ok_or_else(|| {
                unsupported(format!("no set transition for element {:?}", element.id))
            })?;
        let ranges = match &transition.kind {
            FinalizedTransitionKind::Set(ranges) | FinalizedTransitionKind::NotSet(ranges) => {
                ranges.clone()
            }
            other => return Err(unsupported(format!("unexpected set transition {other:?}"))),
        };
        self.add_token_type_local("_la");
        let expr = Box::new(self.test_set_inline(&ranges));
        let capture = Box::new(capture_next_token_type());
        let op = if inverted {
            SrcOpModel::MatchNotSet {
                state_number: to_i32(transition.source),
                var_name: "_la".to_owned(),
                labels: labels.clone(),
                expr,
                capture,
            }
        } else {
            SrcOpModel::MatchSet {
                state_number: to_i32(transition.source),
                var_name: "_la".to_owned(),
                labels: labels.clone(),
                expr,
                capture,
            }
        };
        let mut ops = vec![op];
        if let Some(list_name) = list_label {
            // `getAddToListOpIfListLabelPresent`: the list decl lives on the
            // context struct only; the op pushes the first label's value.
            let list_decl = DeclModel::TokenListDecl {
                name: list_name.clone(),
                escaped_name: escape_if_needed(&list_name),
            };
            self.ctxs
                .add_context_decl(self.current_alt_label.as_deref(), &list_decl);
            ops.push(SrcOpModel::AddToLabelList {
                label: labels[0].clone(),
                list_name: escape_if_needed(&list_name),
            });
        }
        Ok(ops)
    }

    /// `TestSetInline` over a sorted, normalized range list.
    fn test_set_inline(&self, ranges: &[(i32, i32)]) -> SrcOpModel {
        let tokens = ranges
            .iter()
            .flat_map(|(start, stop)| *start..=*stop)
            .collect::<Vec<_>>();
        let with_zero = self.create_bitsets(&tokens, true);
        let without_zero = self.create_bitsets(&tokens, false);
        let bitsets = if with_zero.len() <= without_zero.len() {
            with_zero
        } else {
            without_zero
        };
        SrcOpModel::TestSetInline {
            var_name: "_la".to_owned(),
            bitsets,
        }
    }

    /// `TestSetInline.createBitsets`: groups tokens into shift windows of
    /// `INLINE_TEST_SET_WORD_SIZE`.
    fn create_bitsets(&self, tokens: &[i32], use_zero_offset: bool) -> Vec<BitsetModel> {
        let mut bitsets: Vec<BitsetModel> = Vec::new();
        for &ttype in tokens {
            let needs_new = bitsets
                .last()
                .is_none_or(|current| ttype > current.shift + INLINE_TEST_SET_WORD_SIZE - 1);
            if needs_new {
                let shift =
                    if use_zero_offset && (0..INLINE_TEST_SET_WORD_SIZE - 1).contains(&ttype) {
                        0
                    } else {
                        ttype
                    };
                bitsets.push(BitsetModel {
                    shift,
                    tokens: Vec::new(),
                    calculated: 0,
                });
            }
            let current = bitsets.last_mut().expect("just pushed");
            current.tokens.push(TokenInfoModel {
                ttype,
                name: escape_if_needed(&self.walk.token_name(ttype)),
            });
            current.calculated |= 1_i64 << (ttype - current.shift);
        }
        bitsets
    }

    /// The attribute scope of one outer-most alternative: the Java
    /// `Alternative` symbol tables (`tokenRefs`, `ruleRefs`, `labelDefs`)
    /// plus the `ActionSniffer` output (`tokenRefsInActions`,
    /// `ruleRefsInActions`), completed with the rule- and grammar-level
    /// dictionaries.
    fn build_action_scope(&self, alternative: &Alternative) -> ActionScope {
        let mut symbols = AltSymbols::default();
        for element in &alternative.elements {
            collect_alt_symbols(element, &mut symbols);
        }
        // Left-recursive rules: the labels of the deleted leading recursive
        // references stay resolvable in actions, as they are in the Java
        // tool (the sniffer walks the original alternative text).
        if let Some(info) = &self.rule.left_recursion {
            for removed in info.deleted_labels.values() {
                let def = if removed.label.kind == LabelKind::List {
                    LabelDef::RuleList(self.rule.name.clone())
                } else {
                    LabelDef::Rule(self.rule.name.clone())
                };
                symbols.label_defs.insert(removed.label.name.clone(), def);
            }
        }
        let mut in_actions = AltActionRefs::default();
        for element in &alternative.elements {
            sniff_action_refs(
                element,
                &symbols.token_refs,
                &symbols.rule_refs,
                &mut in_actions,
            );
        }
        let attributes = self.walk.attributes.get(&self.rule.id);
        ActionScope {
            token_refs: symbols.token_refs,
            rule_refs: symbols.rule_refs,
            label_defs: symbols.label_defs,
            args: attributes
                .map(|attributes| {
                    attributes
                        .arguments
                        .iter()
                        .map(|attribute| attribute.name.clone())
                        .collect()
                })
                .unwrap_or_default(),
            retvals: attributes
                .map(|attributes| {
                    attributes
                        .returns
                        .iter()
                        .map(|attribute| attribute.name.clone())
                        .collect()
                })
                .unwrap_or_default(),
            locals: attributes
                .map(|attributes| {
                    attributes
                        .locals
                        .iter()
                        .map(|attribute| attribute.name.clone())
                        .collect()
                })
                .unwrap_or_default(),
            rule_retvals: self.walk.rule_retvals.clone(),
            rule_names: self
                .unit
                .rules
                .iter()
                .map(|rule| rule.name.clone())
                .collect(),
            token_refs_in_actions: in_actions.token_refs,
            rule_refs_in_actions: in_actions.rule_refs,
        }
    }

    /// `Choice.getAltLookaheadAsStringLists` for LL(1) choices (all
    /// alternatives have a look set by disjointness).
    fn alt_look_token_lists(&self, look: &[Option<Vec<(i32, i32)>>]) -> Vec<Vec<TokenInfoModel>> {
        look.iter()
            .map(|alternative| {
                alternative
                    .as_ref()
                    .expect("disjoint alternatives have look sets")
                    .iter()
                    .flat_map(|(start, stop)| *start..=*stop)
                    .map(|ttype| TokenInfoModel {
                        ttype,
                        name: escape_if_needed(&self.walk.token_name(ttype)),
                    })
                    .collect()
            })
            .collect()
    }
}

impl WalkContext<'_> {
    /// The token type of a `Token`/`Literal` terminal element.
    fn semantic_terminal_type(&self, element: &Element) -> Option<i32> {
        // The semantic binding carries the resolved token type.
        match &element.kind {
            ElementKind::Terminal(Terminal::Token(name)) => {
                self.recognizer.vocabulary.by_name.get(name).copied()
            }
            ElementKind::Terminal(Terminal::Literal(literal)) => {
                self.recognizer.vocabulary.by_literal.get(literal).copied()
            }
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum EbnfKind {
    Optional,
    Closure,
    PositiveClosure,
}

fn ebnf_kind(quantifier: Quantifier) -> EbnfKind {
    match quantifier {
        Quantifier::Optional { .. } => EbnfKind::Optional,
        Quantifier::ZeroOrMore { .. } => EbnfKind::Closure,
        Quantifier::OneOrMore { .. } => EbnfKind::PositiveClosure,
        Quantifier::One => unreachable!("not an EBNF quantifier"),
    }
}

const fn ebnf_greedy(quantifier: Quantifier) -> bool {
    match quantifier {
        Quantifier::Optional { greedy }
        | Quantifier::ZeroOrMore { greedy }
        | Quantifier::OneOrMore { greedy } => greedy,
        Quantifier::One => true,
    }
}

/// ANTLR numbers states and alternatives with Java `int`; grammar sizes stay
/// far below the `i32` limit.
fn to_i32(value: usize) -> i32 {
    i32::try_from(value).expect("grammar size exceeds i32")
}

fn choice_model(state_number: usize, decision: usize) -> ChoiceModel {
    ChoiceModel {
        state_number: to_i32(state_number),
        decision: to_i32(decision),
        exit_alt: 0,
        block_start_state_number: 0,
        loop_back_state_number: 0,
        greedy: true,
        alt_look: Vec::new(),
    }
}

fn capture_next_token_type() -> SrcOpModel {
    SrcOpModel::CaptureNextTokenType {
        var_name: "_la".to_owned(),
    }
}

// ---------------------------------------------------------------------
// Context structs: `RuleFunction`/`StructDecl` construction
// ---------------------------------------------------------------------

/// The context structs of one rule function: `ruleCtx` and the
/// `altLabelCtxs` (in the Java `HashMap` label order, shared with the
/// listener/visitor label order).
struct RuleContexts {
    rule_ctx: StructDeclModel,
    alt_label_ctxs: Vec<StructDeclModel>,
    /// `rf.args`: the rule's argument decls (`ruleCtx.ctorAttrs`).
    args: Vec<DeclModel>,
}

impl RuleContexts {
    /// `RuleFunction.addContextDecl`: routes a decl to the alt-label
    /// context struct when the enclosing alternative's label has one, else
    /// to the rule's context struct. Returns the escaped name of the
    /// receiving struct (the `Decl.ctx` of the added decl).
    fn add_context_decl(&mut self, alt_label: Option<&str>, decl: &DeclModel) -> String {
        if let Some(label) = alt_label
            && let Some(ctx) = self
                .alt_label_ctxs
                .iter_mut()
                .find(|ctx| ctx.derived_from_name == label)
        {
            let escaped = ctx.escaped_name.clone();
            struct_decl_add_decl(ctx, decl);
            return escaped;
        }
        let escaped = self.rule_ctx.escaped_name.clone();
        struct_decl_add_decl(&mut self.rule_ctx, decl);
        escaped
    }

    /// The escaped name of the context struct of an alt label (the
    /// `rf.altLabelCtxs.get(altLabel)` of the action translator), falling
    /// back to the rule context struct.
    fn ctx_escaped_name(&self, alt_label: Option<&str>) -> String {
        alt_label
            .and_then(|label| {
                self.alt_label_ctxs
                    .iter()
                    .find(|ctx| ctx.derived_from_name == label)
            })
            .map_or_else(
                || self.rule_ctx.escaped_name.clone(),
                |ctx| ctx.escaped_name.clone(),
            )
    }
}

/// `StructDecl.addDecl` with the `OrderedHashSet` dedup (`Decl.equals`):
/// getters go to `getters` (and their no-body variant to `signatures`),
/// everything else to `attrs`, plus the type-specific lists.
fn struct_decl_add_decl(struct_decl: &mut StructDeclModel, decl: &DeclModel) {
    fn push_unique(list: &mut Vec<DeclModel>, decl: &DeclModel) {
        if !list.iter().any(|existing| decl_equals(existing, decl)) {
            list.push(decl.clone());
        }
    }
    if is_getter_decl(decl) {
        push_unique(&mut struct_decl.getters, decl);
        push_unique(&mut struct_decl.signatures, &getter_signature(decl));
    } else {
        push_unique(&mut struct_decl.attrs, decl);
    }
    match decl {
        DeclModel::TokenDecl { .. } => push_unique(&mut struct_decl.token_decls, decl),
        DeclModel::TokenTypeDecl { .. } => push_unique(&mut struct_decl.token_type_decls, decl),
        DeclModel::TokenListDecl { .. } => push_unique(&mut struct_decl.token_list_decls, decl),
        DeclModel::RuleContextDecl { .. } => {
            push_unique(&mut struct_decl.rule_context_decls, decl);
        }
        DeclModel::RuleContextListDecl { .. } => {
            push_unique(&mut struct_decl.rule_context_list_decls, decl);
        }
        DeclModel::AttributeDecl { .. } => {
            push_unique(&mut struct_decl.attribute_decls, decl);
        }
        _ => {}
    }
}

const fn is_getter_decl(decl: &DeclModel) -> bool {
    matches!(
        decl,
        DeclModel::ContextTokenGetterDecl { .. }
            | DeclModel::ContextTokenListGetterDecl { .. }
            | DeclModel::ContextTokenListIndexedGetterDecl { .. }
            | DeclModel::ContextRuleGetterDecl { .. }
            | DeclModel::ContextRuleListGetterDecl { .. }
            | DeclModel::ContextRuleListIndexedGetterDecl { .. }
    )
}

fn empty_struct_decl(
    name: String,
    derived_from_name: String,
    provide_copy_from: bool,
) -> StructDeclModel {
    StructDeclModel {
        escaped_name: escape_if_needed(&name),
        name,
        derived_from_name,
        provide_copy_from,
        attrs: Vec::new(),
        getters: Vec::new(),
        signatures: Vec::new(),
        ctor_attrs: Vec::new(),
        token_decls: Vec::new(),
        token_type_decls: Vec::new(),
        token_list_decls: Vec::new(),
        rule_context_decls: Vec::new(),
        rule_context_list_decls: Vec::new(),
        attribute_decls: Vec::new(),
    }
}

/// `RustTarget.getAltLabelContextStructName`.
fn alt_label_context_struct_name(label: &str) -> String {
    format!("{}Context", capitalize(label))
}

/// `Rule.hasAltSpecificContexts` (with the `LeftRecursiveRule` override):
/// whether any (possibly nested) alternative of the rule carries an alt
/// label.
fn has_alt_specific_contexts(rule: &Rule) -> bool {
    rule.block
        .alternatives
        .iter()
        .any(|alternative| alternative.label.is_some() || alternative_has_label(alternative))
}

/// `RuleFunction` context construction: the `StructDecl` with the context
/// getters derived from element frequencies, the `AltLabelStructDecl`s of
/// the labeled alternatives, the `returns` attribute decls, and the decls
/// for labels the left-recursion rewrite deleted (`LeftRecursiveRuleFunction`).
fn build_rule_contexts(walk: &WalkContext<'_>, rule: &Rule) -> Result<RuleContexts, EmitError> {
    let provide_copy_from = has_alt_specific_contexts(rule);
    let original_alts = if rule.left_recursion.is_some() {
        original_alts_for_decls(rule)?
    } else {
        rule.block.alternatives.clone()
    };

    let mut ctxs = RuleContexts {
        rule_ctx: empty_struct_decl(
            rule_context_struct_name(&rule.name),
            rule.name.clone(),
            provide_copy_from,
        ),
        alt_label_ctxs: Vec::new(),
        args: Vec::new(),
    };

    // `addContextGetters`: decls of the unlabeled alternatives go to the
    // rule ctx; decls of the labeled ones seed the alt-label ctxs.
    let unlabeled: Vec<&Alternative> = original_alts
        .iter()
        .filter(|alternative| alternative.label.is_none())
        .collect();
    for decl in decls_for_all_elements(walk, &unlabeled) {
        struct_decl_add_decl(&mut ctxs.rule_ctx, &decl);
    }
    for label in alt_labels(rule) {
        let labeled: Vec<&Alternative> = original_alts
            .iter()
            .filter(|alternative| {
                alternative
                    .label
                    .as_ref()
                    .is_some_and(|alt_label| alt_label.value == label)
            })
            .collect();
        let mut ctx = empty_struct_decl(
            alt_label_context_struct_name(&label),
            label,
            provide_copy_from,
        );
        for decl in decls_for_all_elements(walk, &labeled) {
            struct_decl_add_decl(&mut ctx, &decl);
        }
        ctxs.alt_label_ctxs.push(ctx);
    }

    // `r.args` (`a[int i]`): attribute decls of the rule ctx, ahead of
    // `retvals`/`locals`, and the constructor attrs (`rf.args` /
    // `ruleCtx.ctorAttrs`).
    if let Some(attributes) = walk.attributes.get(&rule.id) {
        for attribute in &attributes.arguments {
            let decl = DeclModel::AttributeDecl {
                name: attribute.name.clone(),
                escaped_name: escape_if_needed(&attribute.name),
                ty: attribute.ty.clone(),
            };
            struct_decl_add_decl(&mut ctxs.rule_ctx, &decl);
            ctxs.args.push(decl);
        }
        ctxs.rule_ctx.ctor_attrs.clone_from(&ctxs.args);
    }

    // `r.retvals` (`returns [...]`): attribute decls of the rule ctx.
    if let Some(attributes) = walk.attributes.get(&rule.id) {
        for attribute in &attributes.returns {
            struct_decl_add_decl(
                &mut ctxs.rule_ctx,
                &DeclModel::AttributeDecl {
                    name: attribute.name.clone(),
                    escaped_name: escape_if_needed(&attribute.name),
                    ty: attribute.ty.clone(),
                },
            );
        }
        // `r.locals` (`locals [...]`): attribute decls of the rule ctx,
        // after `retvals`.
        for attribute in &attributes.locals {
            struct_decl_add_decl(
                &mut ctxs.rule_ctx,
                &DeclModel::AttributeDecl {
                    name: attribute.name.clone(),
                    escaped_name: escape_if_needed(&attribute.name),
                    ty: attribute.ty.clone(),
                },
            );
        }
    }

    // `LeftRecursiveRuleFunction`: since the rewrite deletes the labeled
    // leading recursive references (`a=e` in `a=e op='*' b=e`), their decls
    // are added manually to the context struct of the alternative's label.
    if let Some(info) = &rule.left_recursion {
        let rewritten_label_by_original = rewritten_label_by_original(rule, info);
        for removed in info.deleted_labels.values() {
            let ctx_name = rule_context_struct_name(&removed.target);
            let decl = match removed.label.kind {
                LabelKind::Single => DeclModel::RuleContextDecl {
                    name: removed.label.name.clone(),
                    escaped_name: escape_if_needed(&removed.label.name),
                    ctx_name,
                },
                LabelKind::List => DeclModel::RuleContextListDecl {
                    name: removed.label.name.clone(),
                    escaped_name: escape_if_needed(&removed.label.name),
                    ctx_name,
                },
            };
            let alt_label = rewritten_label_by_original
                .get(&removed.original_alternative)
                .map(String::as_str);
            ctxs.add_context_decl(alt_label, &decl);
        }
    }
    Ok(ctxs)
}

/// Maps each original (pre-rewrite) alternative of a left-recursive rule
/// to the alt label of its rewritten alternative (the rewrite keeps the
/// labels), for the deleted-label decl routing.
fn rewritten_label_by_original(
    rule: &Rule,
    info: &crate::grammar::model::LeftRecursionInfo,
) -> BTreeMap<AlternativeId, String> {
    let mut labels_by_rewritten = BTreeMap::new();
    for alternative in &rule.block.alternatives {
        for element in &alternative.elements {
            if let ElementKind::Block(block) = &element.kind {
                for inner in &block.alternatives {
                    if let Some(label) = &inner.label {
                        labels_by_rewritten.insert(inner.id, label.value.clone());
                    }
                }
            }
        }
    }
    info.original_to_rewritten
        .iter()
        .filter_map(|(original, rewritten)| {
            labels_by_rewritten
                .get(rewritten)
                .map(|label| (*original, label.clone()))
        })
        .collect()
}

/// `LeftRecursiveRule.getUnlabeledAltASTs`: the original alternatives of a
/// rewritten left-recursive rule in the Java getter-decl order — primary
/// (and prefix) alternatives first, then operator alternatives, both in
/// source order. The transform deleted the leading recursive call of each
/// operator alternative; it is restored so the getter decls see the
/// original references.
fn original_alts_for_decls(rule: &Rule) -> Result<Vec<Alternative>, EmitError> {
    let unexpected = || {
        unsupported(format!(
            "left-recursive rule {} with an unexpected rewritten shape",
            rule.name
        ))
    };
    let Some(outer) = rule.block.alternatives.first() else {
        return Err(unexpected());
    };
    let [primary_element, operator_element] = &outer.elements[..] else {
        return Err(unexpected());
    };
    let ElementKind::Block(primary_block) = &primary_element.kind else {
        return Err(unexpected());
    };
    let ElementKind::Block(operator_block) = &operator_element.kind else {
        return Err(unexpected());
    };
    let mut alts = primary_block.alternatives.clone();
    let (_, op_alt_infos) = left_recursion_alt_infos(rule)?;
    for (operator, (_, deleted_label)) in operator_block.alternatives.iter().zip(op_alt_infos) {
        let mut alternative = operator.clone();
        let Some(donor) = alternative
            .elements
            .iter()
            .find(|element| !matches!(element.kind, ElementKind::Predicate { .. }))
            .cloned()
        else {
            return Err(unexpected());
        };
        // The restored recursive reference carries the original deleted
        // label: `getNodesWithType` visits it one level below the
        // alternative's direct elements, the `(ASSIGN left e)` depth.
        alternative.elements.insert(
            1,
            Element {
                kind: ElementKind::RuleCall(crate::grammar::model::RuleCall {
                    name: rule.name.clone(),
                    arguments: None,
                    precedence: None,
                }),
                quantifier: Quantifier::One,
                label: deleted_label,
                ..donor
            },
        );
        alts.push(alternative);
    }
    Ok(alts)
}

/// The signature (no-body) variant of a context getter decl.
fn getter_signature(decl: &DeclModel) -> DeclModel {
    match decl {
        DeclModel::ContextTokenGetterDecl { name, .. } => DeclModel::ContextTokenGetterDecl {
            name: name.clone(),
            signature: true,
        },
        DeclModel::ContextTokenListGetterDecl { name, .. } => {
            DeclModel::ContextTokenListGetterDecl {
                name: name.clone(),
                signature: true,
            }
        }
        DeclModel::ContextTokenListIndexedGetterDecl { name, .. } => {
            DeclModel::ContextTokenListIndexedGetterDecl {
                name: name.clone(),
                signature: true,
            }
        }
        DeclModel::ContextRuleGetterDecl {
            name,
            escaped_name,
            ctx_name,
            ..
        } => DeclModel::ContextRuleGetterDecl {
            name: name.clone(),
            escaped_name: escaped_name.clone(),
            ctx_name: ctx_name.clone(),
            signature: true,
        },
        DeclModel::ContextRuleListGetterDecl { name, ctx_name, .. } => {
            DeclModel::ContextRuleListGetterDecl {
                name: name.clone(),
                ctx_name: ctx_name.clone(),
                signature: true,
            }
        }
        DeclModel::ContextRuleListIndexedGetterDecl {
            name,
            escaped_name,
            ctx_name,
            ..
        } => DeclModel::ContextRuleListIndexedGetterDecl {
            name: name.clone(),
            escaped_name: escaped_name.clone(),
            ctx_name: ctx_name.clone(),
            signature: true,
        },
        other => other.clone(),
    }
}

/// `RuleFunction.getDeclsForAllElements`: the context getter decls for all
/// token and rule references of the given alternatives, in first-reference
/// order.
fn decls_for_all_elements(walk: &WalkContext<'_>, alts: &[&Alternative]) -> Vec<DeclModel> {
    let mut all_refs = Vec::new();
    let mut needs_list = BTreeSet::new();
    let mut non_optional: BTreeSet<String> = BTreeSet::new();
    for (alt_index, alt) in alts.iter().enumerate() {
        let refs = collect_ref_names_bfs(walk, alt);
        let (alt_freq, min_freq) = alt_frequencies(walk, alt);
        for name in &refs {
            if alt_freq.get(name).copied().unwrap_or(0) > 1 {
                needs_list.insert(name.clone());
            }
            if alt_index == 0 && min_freq.get(name).copied().unwrap_or(0) != 0 {
                non_optional.insert(name.clone());
            }
        }
        non_optional.retain(|name| min_freq.get(name).copied().unwrap_or(0) != 0);
        all_refs.extend(refs);
    }

    let mut decls: Vec<DeclModel> = Vec::new();
    for name in all_refs {
        let need_list = needs_list.contains(&name);
        let new_decls = decl_for_alt_element(walk, &name, need_list);
        for decl in new_decls {
            if !decls.iter().any(|existing| decl_equals(existing, &decl)) {
                decls.push(decl);
            }
        }
    }
    decls
}

/// `RuleFunction.getDeclForAltElement`.
fn decl_for_alt_element(walk: &WalkContext<'_>, name: &str, need_list: bool) -> Vec<DeclModel> {
    if walk
        .unit
        .rules
        .iter()
        .any(|rule| rule.name == name && rule.kind == crate::grammar::model::RuleKind::Parser)
    {
        let ctx_name = rule_context_struct_name(name);
        if need_list {
            vec![
                DeclModel::ContextRuleListGetterDecl {
                    name: name.to_owned(),
                    ctx_name: ctx_name.clone(),
                    signature: false,
                },
                DeclModel::ContextRuleListIndexedGetterDecl {
                    name: name.to_owned(),
                    escaped_name: escape_if_needed(name),
                    ctx_name,
                    signature: false,
                },
            ]
        } else {
            vec![DeclModel::ContextRuleGetterDecl {
                name: name.to_owned(),
                escaped_name: escape_if_needed(name),
                ctx_name,
                signature: false,
            }]
        }
    } else if need_list {
        vec![
            DeclModel::ContextTokenListGetterDecl {
                name: name.to_owned(),
                signature: false,
            },
            DeclModel::ContextTokenListIndexedGetterDecl {
                name: name.to_owned(),
                signature: false,
            },
        ]
    } else {
        vec![DeclModel::ContextTokenGetterDecl {
            name: name.to_owned(),
            signature: false,
        }]
    }
}

/// `Decl.equals`: non-getters dedup by name, getters by name and argument
/// type, and a getter never equals a non-getter.
fn decl_equals(a: &DeclModel, b: &DeclModel) -> bool {
    fn getter_key(decl: &DeclModel) -> Option<(String, &'static str)> {
        match decl {
            DeclModel::ContextTokenGetterDecl { name, .. }
            | DeclModel::ContextTokenListGetterDecl { name, .. }
            | DeclModel::ContextRuleGetterDecl { name, .. }
            | DeclModel::ContextRuleListGetterDecl { name, .. } => Some((name.clone(), "")),
            DeclModel::ContextTokenListIndexedGetterDecl { name, .. }
            | DeclModel::ContextRuleListIndexedGetterDecl { name, .. } => {
                Some((name.clone(), "int"))
            }
            _ => None,
        }
    }
    match (getter_key(a), getter_key(b)) {
        (Some(key_a), Some(key_b)) => key_a == key_b,
        (None, None) => decl_name(a) == decl_name(b),
        _ => false,
    }
}

fn decl_name(decl: &DeclModel) -> &str {
    match decl {
        DeclModel::TokenDecl { name, .. }
        | DeclModel::TokenTypeDecl { name, .. }
        | DeclModel::TokenListDecl { name, .. }
        | DeclModel::RuleContextDecl { name, .. }
        | DeclModel::RuleContextListDecl { name, .. }
        | DeclModel::AttributeDecl { name, .. }
        | DeclModel::ContextTokenGetterDecl { name, .. }
        | DeclModel::ContextTokenListGetterDecl { name, .. }
        | DeclModel::ContextTokenListIndexedGetterDecl { name, .. }
        | DeclModel::ContextRuleGetterDecl { name, .. }
        | DeclModel::ContextRuleListGetterDecl { name, .. }
        | DeclModel::ContextRuleListIndexedGetterDecl { name, .. } => name,
    }
}

/// `GrammarAST.getNodesWithType`: the token/rule references of one
/// alternative in breadth-first order over the transformed Java AST, the
/// `getName` filtering of `getDeclsForAllElements` included. This is the
/// Java getter order: in `(ID|ATN)* ATN?` the optional `ATN` (level 4)
/// sorts before the set members (level 5).
fn collect_ref_names_bfs(walk: &WalkContext<'_>, alt: &Alternative) -> Vec<String> {
    /// A node of the alternative's AST as Java's `GrammarAST.getNodesWithType`
    /// visits it after `GrammarTransformPipeline`. A quantified element is
    /// `(Q (BLOCK (ALT e)))`, a labeled element is `(ASSIGN id e)`, a reduced
    /// set is `(SET a b)`, and an inverted set is `(NOT (SET a b))`.
    enum Node<'a> {
        Element(&'a Element),
        QuantifierBlock(&'a Element),
        QuantifierAlt(&'a Element),
        Labeled(&'a Element),
        Bare(&'a Element),
        Alt(&'a Alternative),
        Set(&'a [SetElement]),
        Name(String),
    }
    fn set_names(walk: &WalkContext<'_>, elements: &[SetElement]) -> Vec<Node<'static>> {
        elements
            .iter()
            .filter_map(|member| match member {
                SetElement::Terminal { value, .. } => {
                    terminal_ref_name(walk, value).map(Node::Name)
                }
                SetElement::Range { .. } => None,
            })
            .collect()
    }

    let mut out = Vec::new();
    let mut work: VecDeque<Node<'_>> = VecDeque::from([Node::Alt(alt)]);
    while let Some(node) = work.pop_front() {
        // An unquantified element is its label node; an unlabeled one is the
        // bare element node.
        let node = match node {
            Node::Element(element) if element.quantifier == Quantifier::One => {
                Node::Labeled(element)
            }
            other => other,
        };
        let node = match node {
            Node::Labeled(element) if element.label.is_none() => Node::Bare(element),
            other => other,
        };
        match node {
            Node::Name(name) => out.push(name),
            Node::Alt(alternative) => work.extend(alternative.elements.iter().map(Node::Element)),
            Node::Element(element) => work.push_back(Node::QuantifierBlock(element)),
            Node::QuantifierBlock(element) => match &element.kind {
                ElementKind::Block(block) if element.label.is_none() => {
                    work.extend(block.alternatives.iter().map(Node::Alt));
                }
                _ => work.push_back(Node::QuantifierAlt(element)),
            },
            Node::QuantifierAlt(element) => work.push_back(Node::Labeled(element)),
            Node::Labeled(element) => work.push_back(Node::Bare(element)),
            Node::Set(elements) => work.extend(set_names(walk, elements)),
            Node::Bare(element) => match &element.kind {
                ElementKind::RuleCall(call) => out.push(call.name.clone()),
                ElementKind::Terminal(terminal) => {
                    if let Some(name) = terminal_ref_name(walk, terminal) {
                        out.push(name);
                    }
                }
                ElementKind::Set {
                    inverted: false,
                    elements,
                } => {
                    work.extend(set_names(walk, elements));
                }
                ElementKind::Set {
                    inverted: true,
                    elements,
                } => {
                    work.push_back(Node::Set(elements));
                }
                ElementKind::Block(block) => work.extend(block.alternatives.iter().map(Node::Alt)),
                ElementKind::Range(..)
                | ElementKind::Action { .. }
                | ElementKind::Predicate { .. }
                | ElementKind::Epsilon => {}
            },
        }
    }
    out
}

/// `getName`: the getter name of a terminal reference; auto-generated
/// `T__` names and unresolved literals are excluded.
fn terminal_ref_name(walk: &WalkContext<'_>, terminal: &Terminal) -> Option<String> {
    let name = match terminal {
        Terminal::Token(name) => name.clone(),
        Terminal::Literal(literal) => {
            let ttype = walk
                .recognizer
                .vocabulary
                .by_literal
                .get(literal)
                .copied()?;
            walk.token_name(ttype)
        }
        Terminal::Eof => "EOF".to_owned(),
        Terminal::Wildcard | Terminal::LexerCharSet(_) => return None,
    };
    if name.starts_with("T__") {
        return None;
    }
    Some(name)
}

// ---------------------------------------------------------------------
// `SymbolCollector`/`ActionSniffer` port (outer-most alternative scopes)
// ---------------------------------------------------------------------

/// The symbol tables of an outer-most alternative (`Alternative`).
#[derive(Default)]
struct AltSymbols {
    token_refs: BTreeSet<String>,
    rule_refs: BTreeSet<String>,
    label_defs: BTreeMap<String, LabelDef>,
}

/// The `ActionSniffer` output of an outer-most alternative.
#[derive(Default)]
struct AltActionRefs {
    token_refs: BTreeSet<String>,
    rule_refs: BTreeSet<String>,
}

/// `SymbolCollector.tokenRef`/`ruleRef`/`label` over an element subtree:
/// collects the token references (token names and string literals as
/// authored), rule references, and label definitions, descending into
/// nested blocks and sets like the `GrammarTreeVisitor` walk.
fn collect_alt_symbols(element: &Element, symbols: &mut AltSymbols) {
    match &element.kind {
        ElementKind::Terminal(terminal) => {
            let token_ref = match terminal {
                Terminal::Token(name) => Some(name.clone()),
                Terminal::Literal(literal) => Some(literal.clone()),
                Terminal::Eof | Terminal::Wildcard | Terminal::LexerCharSet(_) => None,
            };
            if let Some(token_ref) = token_ref {
                symbols.token_refs.insert(token_ref);
            }
        }
        ElementKind::RuleCall(call) => {
            symbols.rule_refs.insert(call.name.clone());
        }
        ElementKind::Set { elements, .. } => {
            for member in elements {
                if let SetElement::Terminal { value, .. } = member {
                    match value {
                        Terminal::Token(name) => {
                            symbols.token_refs.insert(name.clone());
                        }
                        Terminal::Literal(literal) => {
                            symbols.token_refs.insert(literal.clone());
                        }
                        Terminal::Eof | Terminal::Wildcard | Terminal::LexerCharSet(_) => {}
                    }
                }
            }
        }
        ElementKind::Block(block) => {
            for alternative in &block.alternatives {
                for element in &alternative.elements {
                    collect_alt_symbols(element, symbols);
                }
            }
        }
        ElementKind::Range(..)
        | ElementKind::Action { .. }
        | ElementKind::Predicate { .. }
        | ElementKind::Epsilon => {}
    }
    if let Some(label) = &element.label {
        let list = label.kind == LabelKind::List;
        let def = match &element.kind {
            ElementKind::RuleCall(call) if list => LabelDef::RuleList(call.name.clone()),
            ElementKind::RuleCall(call) => LabelDef::Rule(call.name.clone()),
            _ if list => LabelDef::TokenList,
            _ => LabelDef::Token,
        };
        symbols.label_defs.insert(label.name.clone(), def);
    }
}

/// `ActionSniffer.examineAction` over an element subtree: for every
/// embedded action, tracks the `$x` references that name a token or rule
/// reference of the alternative (`trackRef`), recursing into `setAttr`
/// right-hand sides (`processNested`).
fn sniff_action_refs(
    element: &Element,
    token_refs: &BTreeSet<String>,
    rule_refs: &BTreeSet<String>,
    in_actions: &mut AltActionRefs,
) {
    match &element.kind {
        ElementKind::Action { body, .. } if !body.is_empty() => {
            sniff_action_body(body, token_refs, rule_refs, in_actions);
        }
        ElementKind::Block(block) => {
            for alternative in &block.alternatives {
                for element in &alternative.elements {
                    sniff_action_refs(element, token_refs, rule_refs, in_actions);
                }
            }
        }
        _ => {}
    }
}

fn sniff_action_body(
    body: &str,
    token_refs: &BTreeSet<String>,
    rule_refs: &BTreeSet<String>,
    in_actions: &mut AltActionRefs,
) {
    use crate::dbt::action_translator::SplitEvent;
    for event in split_action(body) {
        match event {
            SplitEvent::Attr(x) | SplitEvent::QualifiedAttr(x, _) => {
                track_action_ref(&x, token_refs, rule_refs, in_actions);
            }
            SplitEvent::SetAttr(x, rhs) => {
                track_action_ref(&x, token_refs, rule_refs, in_actions);
                sniff_action_body(&rhs, token_refs, rule_refs, in_actions);
            }
            SplitEvent::SetNonLocalAttr(_, _, rhs) => {
                sniff_action_body(&rhs, token_refs, rule_refs, in_actions);
            }
            SplitEvent::Text(_) | SplitEvent::NonLocalAttr(_, _) => {}
        }
    }
}

/// `ActionSniffer.trackRef`.
fn track_action_ref(
    x: &str,
    token_refs: &BTreeSet<String>,
    rule_refs: &BTreeSet<String>,
    in_actions: &mut AltActionRefs,
) {
    if token_refs.contains(x) {
        in_actions.token_refs.insert(x.to_owned());
    }
    if rule_refs.contains(x) {
        in_actions.rule_refs.insert(x.to_owned());
    }
}

// ---------------------------------------------------------------------
// `ElementFrequenciesVisitor` port
// ---------------------------------------------------------------------

type Frequencies = BTreeMap<String, u32>;

fn combine_and_clip(a: &Frequencies, b: &Frequencies, clip: u32) -> Frequencies {
    let mut out = a.clone();
    for (name, count) in b {
        let entry = out.entry(name.clone()).or_insert(0);
        *entry = (*entry + *count).min(clip);
    }
    out
}

fn combine_max(a: &Frequencies, b: &Frequencies) -> Frequencies {
    let mut out = a.clone();
    for (name, count) in b {
        let entry = out.entry(name.clone()).or_insert(0);
        *entry = (*entry).max(*count);
    }
    out
}

fn combine_min(a: &Frequencies, b: &Frequencies) -> Frequencies {
    let mut out = Frequencies::new();
    for name in a.keys().chain(b.keys()) {
        let count = (*a.get(name).unwrap_or(&0)).min(*b.get(name).unwrap_or(&0));
        if count > 0 {
            out.insert(name.clone(), count);
        }
    }
    out
}

/// The per-alternative (max frequency, min frequency) pair of the
/// visitor's `outerAlternative`.
fn alt_frequencies(walk: &WalkContext<'_>, alt: &Alternative) -> (Frequencies, Frequencies) {
    let mut freq = Frequencies::new();
    let mut min = Frequencies::new();
    for element in &alt.elements {
        let (element_freq, element_min) = element_frequencies(walk, element);
        freq = combine_and_clip(&freq, &element_freq, 2);
        min = combine_and_clip(&min, &element_min, 2);
    }
    (freq, min)
}

fn element_frequencies(walk: &WalkContext<'_>, element: &Element) -> (Frequencies, Frequencies) {
    fn single(name: String) -> (Frequencies, Frequencies) {
        let mut freq = Frequencies::new();
        freq.insert(name, 1);
        (freq.clone(), freq)
    }
    let (mut freq, mut min) = match &element.kind {
        ElementKind::RuleCall(call) => single(call.name.clone()),
        ElementKind::Terminal(terminal) => terminal_ref_name(walk, terminal)
            .map_or_else(|| (Frequencies::new(), Frequencies::new()), single),
        ElementKind::Set { elements, .. } => {
            let mut freq = Frequencies::new();
            let mut min = Frequencies::new();
            for member in elements {
                let name = match member {
                    SetElement::Terminal { value, .. } => terminal_ref_name(walk, value),
                    SetElement::Range { .. } => None,
                };
                if let Some(name) = name {
                    *freq.entry(name.clone()).or_insert(0) += 1;
                    *min.entry(name).or_insert(0) += 1;
                }
            }
            for count in freq.values_mut() {
                *count = 1;
            }
            if min.len() > 1 {
                min.clear();
            }
            (freq, min)
        }
        ElementKind::Block(block) => {
            let mut freq = Frequencies::new();
            let mut min = Frequencies::new();
            for alternative in &block.alternatives {
                let (alt_freq, alt_min) = alt_frequencies(walk, alternative);
                freq = combine_max(&freq, &alt_freq);
                min = combine_min(&min, &alt_min);
            }
            (freq, min)
        }
        ElementKind::Range(..)
        | ElementKind::Action { .. }
        | ElementKind::Predicate { .. }
        | ElementKind::Epsilon => (Frequencies::new(), Frequencies::new()),
    };
    match element.quantifier {
        Quantifier::ZeroOrMore { .. } => {
            for count in freq.values_mut() {
                *count = 2;
            }
            min.clear();
        }
        Quantifier::OneOrMore { .. } => {
            for count in freq.values_mut() {
                *count = 2;
            }
        }
        Quantifier::Optional { .. } => min.clear(),
        Quantifier::One => {}
    }
    (freq, min)
}
