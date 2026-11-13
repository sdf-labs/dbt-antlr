// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Assembly of the runtime table set from the classified decisions.
//!
//! The classifier produces one [`StaticDfaTable`] per decision (or per
//! precedence class) as an owned snapshot. This module turns those snapshots
//! into the [`StaticParserTables`] image the runtime overlays: it normalizes the
//! escape markers, merges the states that the runtime cannot distinguish,
//! collects the continuations, indexes the decisions, and deduplicates
//! repeated precedence class tables.

use dbt_antlr_runtime::atn::INVALID_ALT;
use dbt_antlr_runtime::dfa::static_parser_dfa::{
    DecisionTables, Guard, ParserContinuation, PrecedenceDispatch, StaticParserTableBuilder,
    StaticParserTables,
};

use std::collections::HashMap;

use super::table::StaticDfaEdge;
use super::{CONFLICT_ESCAPE, DecisionResult, ESCAPE, EscapeResolution, StaticDfaTable};

/// Builds the table set of one parser unit.
pub(crate) fn build_tables(results: &[DecisionResult]) -> StaticParserTables {
    let num_slots = results
        .iter()
        .filter(|result| result.table.is_some() || result.precedence.is_some())
        .map(|result| result.decision + 1)
        .max()
        .unwrap_or(0);
    if num_slots == 0 {
        return StaticParserTables::empty();
    }

    let mut decisions = vec![DecisionTables::None; num_slots];
    let mut tables: Vec<StaticParserTableBuilder> = Vec::new();
    for result in results {
        if let Some(precedence) = result.precedence.as_ref() {
            if precedence.tables.iter().all(Option::is_none) {
                // Every class stays adaptive: no slot, so the emitter keeps
                // the plain adaptive block for this decision.
                continue;
            }
            let mut class_tables = Vec::with_capacity(precedence.tables.len());
            for class_table in &precedence.tables {
                let Some(table) = class_table else {
                    class_tables.push(-1);
                    continue;
                };
                let built = runtime_table(-1, table);
                let index = tables
                    .iter()
                    .position(|existing| existing == &built)
                    .unwrap_or_else(|| {
                        tables.push(built);
                        tables.len() - 1
                    });
                class_tables.push(index as i32);
            }
            decisions[result.decision] = DecisionTables::Dispatch(PrecedenceDispatch::new(
                precedence.cutoffs.clone(),
                class_tables,
            ));
            continue;
        }
        let Some(table) = &result.table else {
            continue;
        };
        decisions[result.decision] = DecisionTables::Single(tables.len());
        tables.push(runtime_table(result.decision as i32, table));
    }
    StaticParserTables::new(decisions, tables)
}

/// Converts one snapshot table into the runtime table.
fn runtime_table(decision: i32, table: &StaticDfaTable) -> StaticParserTableBuilder {
    let table = &minimized(table);
    let mut accepts = Vec::with_capacity(table.states.len());
    let mut fallbacks = Vec::with_capacity(table.states.len());
    let mut conflict_min_alts = Vec::with_capacity(table.states.len());
    let mut guards = Vec::with_capacity(table.states.len());
    let mut continuations: Vec<ParserContinuation> = Vec::new();
    let mut edges = Vec::with_capacity(table.states.len());
    for (state, entry) in table.states.iter().enumerate() {
        // A conflict escape without a usable minimum alternative is a plain
        // escape: the runtime has to defer instead of resolving the conflict.
        let accept = if entry.accept == CONFLICT_ESCAPE && entry.conflict_min_alt <= 0 {
            ESCAPE
        } else {
            entry.accept
        };
        accepts.push(accept);
        guards.push((entry.guarded_alt > 0).then(|| Guard {
            alt: entry.guarded_alt,
            // A table that keeps the adaptive fallback defers to it.
            defer_alt: if entry.resolution == Some(EscapeResolution::Guarded) {
                entry.guard_defer_alt
            } else {
                INVALID_ALT
            },
            group: entry.guard_group,
        }));
        fallbacks.push(if accept == 0 { entry.fallback } else { 0 });
        conflict_min_alts.push(if accept == CONFLICT_ESCAPE {
            entry.conflict_min_alt
        } else {
            0
        });
        if accept == ESCAPE
            && let Some(mut continuation) = entry.continuation.clone()
        {
            continuation.state = state as u32;
            continuations.push(continuation);
        }
        edges.push(
            entry
                .edges
                .iter()
                .map(|edge| (edge.lo, edge.hi, edge.target as u32))
                .collect::<Vec<_>>(),
        );
    }
    StaticParserTableBuilder::from_state_edges(
        decision,
        accepts,
        fallbacks,
        conflict_min_alts,
        continuations,
        &edges,
    )
    .with_guards(guards, table.guard_groups.clone(), table.guard_pass.clone())
}

/// What the runtime reads of one state, without its edges. Two states with
/// the same signature and edges into equal states are equal for the runtime.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct RuntimeSignature {
    accept: i32,
    /// The runtime reads the fallback only in a state without a verdict.
    fallback: i32,
    conflict_min_alt: i32,
    guard: Option<(i32, i32, u32)>,
    /// The index of the continuation in the distinct continuations of the
    /// table.
    continuation: Option<usize>,
}

/// The equivalent table with the fewest states. This function merges the
/// states that the runtime cannot distinguish (Moore partition refinement).
/// The report keeps the built table.
pub(super) fn minimized(table: &StaticDfaTable) -> StaticDfaTable {
    let states = &table.states;
    if states.is_empty() {
        return table.clone();
    }
    let mut continuations: Vec<&ParserContinuation> = Vec::new();
    let signatures: Vec<RuntimeSignature> = states
        .iter()
        .map(|state| {
            let accept = if state.accept == CONFLICT_ESCAPE && state.conflict_min_alt <= 0 {
                ESCAPE
            } else {
                state.accept
            };
            let continuation = (accept == ESCAPE)
                .then_some(state.continuation.as_ref())
                .flatten()
                .map(|continuation| {
                    continuations
                        .iter()
                        .position(|known| {
                            known.contexts == continuation.contexts
                                && known.configs == continuation.configs
                        })
                        .unwrap_or_else(|| {
                            continuations.push(continuation);
                            continuations.len() - 1
                        })
                });
            RuntimeSignature {
                accept,
                fallback: if accept == 0 { state.fallback } else { 0 },
                conflict_min_alt: if accept == CONFLICT_ESCAPE {
                    state.conflict_min_alt
                } else {
                    0
                },
                guard: (state.guarded_alt > 0).then_some((
                    state.guarded_alt,
                    if state.resolution == Some(EscapeResolution::Guarded) {
                        state.guard_defer_alt
                    } else {
                        INVALID_ALT
                    },
                    state.guard_group,
                )),
                continuation,
            }
        })
        .collect();

    let mut class = number_classes(signatures.iter());
    loop {
        // The runtime reads the edges of a state without a verdict only.
        let keys: Vec<(usize, Vec<ClassEdge>)> = states
            .iter()
            .enumerate()
            .map(|(index, state)| {
                let edges = if signatures[index].accept == 0 {
                    class_edges(&state.edges, &class)
                } else {
                    Vec::new()
                };
                (class[index], edges)
            })
            .collect();
        let refined = number_classes(keys.iter());
        let count = |classes: &[usize]| classes.iter().max().map_or(0, |max| max + 1);
        let done = count(&refined) == count(&class);
        class = refined;
        if done {
            break;
        }
    }

    // The start state is in class 0, so it stays state 0. The other classes
    // keep the order of their first state.
    let mut index_of = vec![usize::MAX; states.len()];
    let mut representatives = Vec::new();
    for (state, &state_class) in class.iter().enumerate() {
        if index_of[state_class] == usize::MAX {
            index_of[state_class] = representatives.len();
            representatives.push(state);
        }
    }
    if representatives.len() == states.len() {
        return table.clone();
    }
    let merged = representatives
        .iter()
        .enumerate()
        .map(|(index, &representative)| {
            let mut state = states[representative].clone();
            if signatures[representative].accept != 0 {
                state.fallback = 0;
            }
            if let Some(continuation) = state.continuation.as_mut() {
                continuation.state = index as u32;
            }
            state.edges = class_edges(&state.edges, &class)
                .into_iter()
                .map(|(lo, hi, target)| StaticDfaEdge {
                    lo,
                    hi,
                    target: index_of[target],
                })
                .collect();
            state
        })
        .collect();
    let mut minimized = StaticDfaTable::finish(table.decision, merged);
    minimized.guard_groups.clone_from(&table.guard_groups);
    minimized.guard_pass.clone_from(&table.guard_pass);
    minimized
}

/// The number of states of the table that the generated parser embeds.
pub(crate) fn minimized_states(table: &StaticDfaTable) -> usize {
    minimized(table).states.len()
}

/// Numbers the distinct keys in order of their first occurrence.
fn number_classes<'k, K: Clone + Eq + std::hash::Hash + 'k>(
    keys: impl Iterator<Item = &'k K>,
) -> Vec<usize> {
    let mut numbers: HashMap<K, usize> = HashMap::new();
    keys.map(|key| {
        let next = numbers.len();
        *numbers.entry(key.clone()).or_insert(next)
    })
    .collect()
}

/// An edge interval `(lo, hi)` into a class of states.
type ClassEdge = (i32, i32, usize);

/// Replaces the target of each edge with its class, and merges the adjacent
/// intervals into one class.
fn class_edges(edges: &[StaticDfaEdge], class: &[usize]) -> Vec<ClassEdge> {
    let mut merged: Vec<ClassEdge> = Vec::with_capacity(edges.len());
    for edge in edges {
        let target = class[edge.target];
        match merged.last_mut() {
            Some((_, hi, last)) if *last == target && *hi + 1 == edge.lo => *hi = edge.hi,
            _ => merged.push((edge.lo, edge.hi, target)),
        }
    }
    merged
}
