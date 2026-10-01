// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Builds the dbt listener/visitor output models from a compiled parser
//! grammar unit.
//!
//! This is a port of the Java tool's `ListenerFile`/`VisitorFile`
//! constructors (`BaseListenerFile`/`BaseVisitorFile` reuse them): the
//! `listenerNames`/`visitorNames` sets and the `*LabelRuleNames` maps,
//! including the `Rule.getAltLabels`/`LeftRecursiveRule.getAltLabels`
//! label enumeration. The listener/visitor models do not depend on the
//! parser walk, so grammars whose parser emission is not supported yet
//! (alt-label context structs, milestone 4.6) still emit these files.

use std::collections::BTreeMap;

use crate::dbt::lexer_factory::ANTLR_VERSION;
use crate::dbt::model::{
    ListenerFileContext, ListenerFileModel, NamedActionsModel, VisitorFileContext, VisitorFileModel,
};
use crate::grammar::model::{ElementKind, GrammarUnit, Rule};

/// The four tree-walker file contexts of one parser grammar unit.
pub(crate) struct TreeWalkFileContexts {
    pub(crate) listener: ListenerFileContext,
    pub(crate) base_listener: ListenerFileContext,
    pub(crate) visitor: VisitorFileContext,
    pub(crate) base_visitor: VisitorFileContext,
}

/// Builds the listener/visitor/base-listener/base-visitor render contexts
/// for one parser unit, the Java `buildListenerFile`/`buildVisitorFile` and
/// base variants.
///
/// `grammar_name` is the authored grammar name (`CSV`); for a combined
/// grammar the parser unit is named after the recognizer (`CSVParser`), so
/// the name comes from the grammar file stem, as in the parser path.
pub(crate) fn build_tree_walk_file_contexts(
    unit: &GrammarUnit,
    grammar_name: &str,
    grammar_file_name: &str,
) -> TreeWalkFileContexts {
    let (listener_names, label_rule_names) = listener_names(unit);
    let header = unit
        .actions
        .iter()
        .find(|action| action.scope.is_none() && action.name == "header")
        .map(|action| action.body.clone());
    let named_actions = collect_unscoped_named_actions(unit);
    let listener_file = ListenerFileModel {
        grammar_file_name: grammar_file_name.to_owned(),
        antlr_version: ANTLR_VERSION.to_owned(),
        grammar_name: grammar_name.to_owned(),
        parser_name: recognizer_name(grammar_name),
        listener_names: listener_names.clone(),
        listener_label_rule_names: label_rule_names.clone(),
    };
    let visitor_file = VisitorFileModel {
        grammar_file_name: grammar_file_name.to_owned(),
        antlr_version: ANTLR_VERSION.to_owned(),
        grammar_name: grammar_name.to_owned(),
        parser_name: recognizer_name(grammar_name),
        visitor_names: listener_names,
        visitor_label_rule_names: label_rule_names,
    };
    TreeWalkFileContexts {
        listener: ListenerFileContext {
            kind: "ListenerFile",
            file: listener_file.clone(),
            header: header.clone(),
            named_actions: named_actions.clone(),
        },
        base_listener: ListenerFileContext {
            kind: "BaseListenerFile",
            file: listener_file,
            header: header.clone(),
            named_actions: named_actions.clone(),
        },
        visitor: VisitorFileContext {
            kind: "VisitorFile",
            file: visitor_file.clone(),
            header: header.clone(),
            named_actions: named_actions.clone(),
        },
        base_visitor: VisitorFileContext {
            kind: "BaseVisitorFile",
            file: visitor_file,
            header,
            named_actions,
        },
    }
}

/// `Grammar.getRecognizerName`: the grammar name plus a `Parser` suffix.
fn recognizer_name(grammar_name: &str) -> String {
    if grammar_name.ends_with("Parser") {
        grammar_name.to_owned()
    } else {
        format!("{grammar_name}Parser")
    }
}

/// The `listenerNames` set and `listenerLabelRuleNames` map: for each rule
/// in declaration order, either the rule name (rules without alt labels) or
/// the alt labels (rules with alt-specific contexts). Duplicated labels
/// across rules are deduplicated like the Java `LinkedHashSet`.
fn listener_names(unit: &GrammarUnit) -> (Vec<String>, BTreeMap<String, String>) {
    let mut names: Vec<String> = Vec::new();
    let mut label_rule_names = BTreeMap::new();
    for rule in &unit.rules {
        let labels = alt_labels(rule);
        if labels.is_empty() {
            if !names.contains(&rule.name) {
                names.push(rule.name.clone());
            }
        } else {
            for label in labels {
                if !names.contains(&label) {
                    names.push(label.clone());
                }
                label_rule_names.insert(label, rule.name.clone());
            }
        }
    }
    (names, label_rule_names)
}

/// `Rule.getAltLabels` / `LeftRecursiveRule.getAltLabels`: the labels of the
/// rule's outer alternatives in the Java iteration order. Empty when the
/// rule has no labeled alternatives (the Java methods return `null` then).
pub(crate) fn alt_labels(rule: &Rule) -> Vec<String> {
    if rule.left_recursion.is_some() {
        // `LeftRecursiveRule.getAltLabels` collects the labels of the
        // deleted primary and operator alternatives (the rewritten rule
        // itself is unlabeled) into a `HashMap`: primary alternatives in
        // source order, then operator alternatives in source order, then
        // iterated in `HashMap` bucket order. A `put` with an existing key
        // keeps the first insertion position, so repeated labels appear
        // once.
        let mut insertion_order = Vec::new();
        for alternative in &rule.block.alternatives {
            for element in &alternative.elements {
                if let ElementKind::Block(block) = &element.kind {
                    for inner in &block.alternatives {
                        if let Some(label) = &inner.label
                            && !insertion_order.contains(&label.value)
                        {
                            insertion_order.push(label.value.clone());
                        }
                    }
                }
            }
        }
        java_hash_map_iteration_order(insertion_order)
    } else {
        // `Rule.getAltLabels`: a `LinkedHashMap` in alternative order, so
        // repeated labels appear once at their first position.
        let mut labels = Vec::new();
        for alternative in &rule.block.alternatives {
            if let Some(label) = &alternative.label
                && !labels.contains(&label.value)
            {
                labels.push(label.value.clone());
            }
        }
        labels
    }
}

/// The iteration order of a Java 8+ `HashMap` with the given keys inserted
/// in order: buckets ordered by `spread(hashCode) & (capacity - 1)`, keys
/// within one bucket in insertion order. The table starts at capacity 16
/// and doubles when the size exceeds 3/4 of the capacity.
fn java_hash_map_iteration_order(keys: Vec<String>) -> Vec<String> {
    let mut capacity = 16_usize;
    while keys.len() > capacity * 3 / 4 {
        capacity *= 2;
    }
    let mut keyed: Vec<(usize, String)> = keys
        .into_iter()
        .map(|key| (java_hash_map_bucket(&key, capacity), key))
        .collect();
    keyed.sort_by_key(|(bucket, _)| *bucket);
    keyed.into_iter().map(|(_, key)| key).collect()
}

/// `HashMap.hash(key) & (capacity - 1)` for a `String` key.
fn java_hash_map_bucket(key: &str, capacity: usize) -> usize {
    // `String.hashCode` works on UTF-16 code units with i32 overflow.
    let mut hash: u32 = 0;
    for unit in key.encode_utf16() {
        hash = hash.wrapping_mul(31).wrapping_add(u32::from(unit));
    }
    let spread = hash ^ (hash >> 16);
    spread as usize & (capacity - 1)
}

/// `ListenerFile`'s `namedActions`: the scope-less named actions of the
/// grammar. The `Rust.stg` listener/visitor templates do not read them; the
/// model carries them for template-signature fidelity.
fn collect_unscoped_named_actions(unit: &GrammarUnit) -> NamedActionsModel {
    let mut model = NamedActionsModel::default();
    for action in &unit.actions {
        if action.scope.is_some() {
            continue;
        }
        let slot = match action.name.as_str() {
            "header" => &mut model.header,
            "init" => &mut model.init,
            "fields" => &mut model.fields,
            "members" => &mut model.members,
            "definitions" => &mut model.definitions,
            "extend" => &mut model.extend,
            _ => continue,
        };
        *slot = Some(action.body.clone());
    }
    model
}
