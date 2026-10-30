// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Assembly of the runtime table set from the classified decisions.
//!
//! The classifier produces one [`StaticDfaTable`] per decision (or per
//! precedence class) as an owned snapshot. This module turns those snapshots
//! into the [`StaticParserTables`] the runtime decodes: it normalizes the
//! escape markers, collects the continuations, indexes the decisions, and
//! deduplicates repeated precedence class tables.

use dbt_antlr_runtime::dfa::static_parser_dfa::{
    DecisionTables, ParserContinuation, PrecedenceDispatch, StaticParserTable, StaticParserTables,
};

use super::{CONFLICT_ESCAPE, DecisionResult, ESCAPE, StaticDfaTable};

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
    let mut tables: Vec<StaticParserTable> = Vec::new();
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
fn runtime_table(decision: i32, table: &StaticDfaTable) -> StaticParserTable {
    let mut accepts = Vec::with_capacity(table.states.len());
    let mut fallbacks = Vec::with_capacity(table.states.len());
    let mut conflict_min_alts = Vec::with_capacity(table.states.len());
    let mut guarded_alts = Vec::with_capacity(table.states.len());
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
        guarded_alts.push(entry.guarded_alt);
        fallbacks.push(entry.fallback);
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
    StaticParserTable::from_state_edges(
        decision,
        accepts,
        fallbacks,
        conflict_min_alts,
        continuations,
        &edges,
    )
    .with_guards(
        guarded_alts,
        table.guard_danger.clone(),
        table.guard_pass.clone(),
    )
}
