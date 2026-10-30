// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-lexer-dfa` optimization pass (lexer scope).
//!
//! When the flag is on, the pass reads the runtime ATN that the lexer compiler
//! built and runs the runtime static lexer DFA builder (`static-dfa-build`
//! feature). The builder drives the reference `LexerATNSimulator` move and
//! closure code exhaustively. As a result, the tables match the lazy path by
//! construction, and the build is deterministic: one grammar produces one word
//! stream.
//!
//! The build result becomes the `StaticLexerTables` of the lexer output
//! model. Modes that the builder cannot compile, and edges that escape to the
//! lazy simulator, produce compiler warnings (`G4S188` and `G4S189`). Under
//! `--static-dfa-only`, these diagnostics are errors. When every mode compiles
//! and no edge escapes, the generated lexer runs on the tables alone: its ATN
//! statics are elided and its word stream is never serialized. The
//! `StaticLexerDfaReport` of each lexer is appended to the
//! [`OptimizationReport`](super::report::OptimizationReport), which renders the
//! `static-lexer-dfa` manifest entry.

use dbt_antlr_runtime::dfa::static_dfa_build::{
    DEFAULT_MAX_STATES_PER_MODE, EscapeCounts, EscapeReason, EscapedEdge, ModeBuildOutcome,
    ModeFailure, StaticLexerDfaBuild, StaticLexerDfaBuilder,
};
use dbt_antlr_runtime::dfa::static_lexer_dfa::{Accel, CONTINUATION_MOVED_CONFIGS, StaticLexerDFA};
use dbt_antlr_runtime::int_stream::EOF;
use minijinja::Value;

use super::config::AtnFallback;
use super::descriptor::PassDescriptor;
use super::manifest::{object, option, source_value};
use crate::grammar::atn::{CompiledLexer, FinalizedTransitionKind};
use crate::grammar::diagnostic::Diagnostic;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::{
    Block, BuildTransitionId, ElementId, ElementKind, GrammarUnit, ModelNodeId, Rule, RuleId,
};
use crate::grammar::provenance::{Origin, ProvenanceIndex};
use crate::grammar::source::SourceSet;
use crate::grammar::transform::SafetyClass;

pub(crate) struct StaticLexerDfa;

impl StaticLexerDfa {
    pub(crate) const NAME: &'static str = "static-lexer-dfa";
    pub(crate) const SAFETY: SafetyClass = SafetyClass::TreeAndApiPreserving;

    /// Builds the static lexer DFA tables of one compiled lexer unit.
    ///
    /// With [`AtnFallback::Forbidden`], every mode failure and escaped edge is
    /// an error instead of a warning.
    pub(crate) fn build(compiled: &CompiledLexer, fallback: AtnFallback) -> StaticLexerDfaArtifact {
        let build = StaticLexerDfaBuilder::new().build(compiled.atn);

        let unit = &compiled.semantic.unit;
        let grammar_name = unit.name.as_str();
        let mode_name = |index: usize| {
            compiled
                .semantic
                .recognizer
                .mode_names
                .get(index)
                .map_or("DEFAULT_MODE", String::as_str)
        };

        let mut diagnostics = Vec::new();
        let mut all_present = true;
        let mut has_escapes = false;
        for (index, mode) in build.modes.iter().enumerate() {
            let name = mode_name(index);
            if let Some(failure) = mode.failure {
                all_present = false;
                diagnostics.push(mode_failure_diagnostic(
                    grammar_name,
                    name,
                    failure,
                    compiled,
                    fallback,
                ));
            }
            if !mode.escaped_edges.is_empty() {
                has_escapes = true;
                for edge in &mode.escaped_edges {
                    diagnostics.push(escaped_edge_diagnostic(
                        grammar_name,
                        name,
                        edge,
                        compiled,
                        fallback,
                    ));
                }
            }
        }

        let omit_atn = all_present && !has_escapes;
        let report = build_lexer_report(compiled, &build, omit_atn);
        StaticLexerDfaArtifact {
            tables: StaticLexerTables {
                words: build.words,
                omit_atn,
            },
            diagnostics,
            report,
        }
    }
}

/// The static lexer DFA tables of one lexer build.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerTables {
    /// Word stream of the `static_dfa_build` format (version 1).
    pub(crate) words: Vec<u32>,
    /// Whether the generated lexer needs no ATN at runtime.
    pub(crate) omit_atn: bool,
}

/// The emitted tables, pass diagnostics, and optimization report of one
/// static lexer DFA build.
#[derive(Clone, Debug)]
pub(crate) struct StaticLexerDfaArtifact {
    pub(crate) tables: StaticLexerTables,
    pub(crate) diagnostics: Vec<Diagnostic>,
    pub(crate) report: StaticLexerDfaReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerDfaReport {
    pub(crate) grammar: String,
    pub(crate) source_span: SourceSpan,
    pub(crate) omit_atn: bool,
    pub(crate) table_words: usize,
    pub(crate) modes: Vec<StaticLexerModeReport>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum StaticLexerModeStatus {
    Compiled,
    Partial,
    Failed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerModeFailureReport {
    pub(crate) code: &'static str,
    pub(crate) kind: &'static str,
    pub(crate) reason: String,
    pub(crate) rules: Vec<String>,
    pub(crate) hint: &'static str,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct StaticLexerStateStats {
    pub(crate) total: usize,
    pub(crate) accept: usize,
    pub(crate) non_accept: usize,
    pub(crate) accelerated: usize,
    pub(crate) with_escapes: usize,
    pub(crate) max_configs: usize,
    pub(crate) total_configs: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct StaticLexerEdgeStats {
    pub(crate) total: usize,
    pub(crate) target: usize,
    pub(crate) escaped: usize,
    pub(crate) eof: usize,
    pub(crate) self_loop_intervals: usize,
    pub(crate) ascii_populated_cells: usize,
    pub(crate) hi_symbol_classes: usize,
    pub(crate) hi_distinct_rows: usize,
    pub(crate) escape_counts: EscapeCounts,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct StaticLexerContinuationStats {
    pub(crate) total: usize,
    pub(crate) moved_configs: usize,
    pub(crate) source_set: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerSimpleRecursiveRuleReport {
    pub(crate) rule: String,
    pub(crate) source_span: SourceSpan,
    pub(crate) opener: String,
    pub(crate) closer: String,
    pub(crate) delimiter_bytes: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum StaticLexerAccelKindReport {
    Positive {
        ascii_count: u32,
        ascii_intervals: Vec<String>,
        bitmap_hex: String,
        hi_self_loop: bool,
        excluded_ascii: Vec<String>,
        hint: Option<String>,
    },
    Exclusion {
        needle_count: usize,
        needles: Vec<String>,
        needle_bytes: Vec<u8>,
    },
    CountedExclusion {
        opener: String,
        closer: String,
        delimiter_bytes: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerAcceleratedStateReport {
    pub(crate) state: usize,
    pub(crate) accept_rule: Option<String>,
    pub(crate) accept_token_type: Option<i32>,
    pub(crate) rules: Vec<String>,
    pub(crate) self_loop_intervals: Vec<String>,
    pub(crate) accel: StaticLexerAccelKindReport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerContinuationReport {
    pub(crate) id: u32,
    pub(crate) kind: &'static str,
    pub(crate) config_count: usize,
    pub(crate) context_count: usize,
    pub(crate) non_greedy_configs: usize,
    pub(crate) rules: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerEscapedEdgeReport {
    pub(crate) state: usize,
    pub(crate) state_rules: Vec<String>,
    pub(crate) symbols: String,
    pub(crate) symbol_start: i32,
    pub(crate) symbol_end: i32,
    pub(crate) reason: &'static str,
    pub(crate) message: &'static str,
    pub(crate) rule: Option<String>,
    pub(crate) source_span: SourceSpan,
    pub(crate) hint: &'static str,
    pub(crate) continuation: Option<StaticLexerContinuationReport>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerRuleFootprintReport {
    pub(crate) rule: String,
    pub(crate) source_span: SourceSpan,
    pub(crate) states: usize,
    pub(crate) accept_states: usize,
    pub(crate) accelerated_states: usize,
    pub(crate) escaped_edges: usize,
    pub(crate) counted_exclusion: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StaticLexerModeReport {
    pub(crate) index: usize,
    pub(crate) mode: String,
    pub(crate) source_span: SourceSpan,
    pub(crate) status: StaticLexerModeStatus,
    pub(crate) failure: Option<StaticLexerModeFailureReport>,
    pub(crate) states: StaticLexerStateStats,
    pub(crate) edges: StaticLexerEdgeStats,
    pub(crate) continuations: StaticLexerContinuationStats,
    pub(crate) simple_recursive_rules: Vec<StaticLexerSimpleRecursiveRuleReport>,
    pub(crate) accelerated_states: Vec<StaticLexerAcceleratedStateReport>,
    pub(crate) escaped_edges: Vec<StaticLexerEscapedEdgeReport>,
    pub(crate) rule_footprint: Vec<StaticLexerRuleFootprintReport>,
}

fn build_lexer_report(
    compiled: &CompiledLexer,
    build: &StaticLexerDfaBuild,
    omit_atn: bool,
) -> StaticLexerDfaReport {
    let unit = &compiled.semantic.unit;
    let modes = build
        .modes
        .iter()
        .enumerate()
        .map(|(index, mode)| {
            let name = compiled
                .semantic
                .recognizer
                .mode_names
                .get(index)
                .map_or("DEFAULT_MODE", String::as_str);
            build_mode_report(compiled, index, name, mode)
        })
        .collect();
    StaticLexerDfaReport {
        grammar: unit.name.clone(),
        source_span: unit.span.clone(),
        omit_atn,
        table_words: build.words.len(),
        modes,
    }
}

fn build_mode_report(
    compiled: &CompiledLexer,
    index: usize,
    mode_name: &str,
    mode: &ModeBuildOutcome,
) -> StaticLexerModeReport {
    let unit = &compiled.semantic.unit;
    let status = if mode.failure.is_some() {
        StaticLexerModeStatus::Failed
    } else if mode.escaped_edges.is_empty() {
        StaticLexerModeStatus::Compiled
    } else {
        StaticLexerModeStatus::Partial
    };
    let failure = mode
        .failure
        .map(|failure| build_mode_failure_report(compiled, mode_name, failure));

    let accept_states = mode
        .states
        .iter()
        .filter(|state| state.accept_type >= 0)
        .count();
    let accelerated_count = mode
        .states
        .iter()
        .filter(|state| state.accel.is_some())
        .count();
    let with_escapes = mode
        .states
        .iter()
        .filter(|state| state.escaped_intervals > 0)
        .count();
    let max_configs = mode
        .states
        .iter()
        .map(|state| state.num_configs)
        .max()
        .unwrap_or(0);
    let total_configs = mode.states.iter().map(|state| state.num_configs).sum();
    let states = StaticLexerStateStats {
        total: mode.num_states,
        accept: accept_states,
        non_accept: mode.num_states.saturating_sub(accept_states),
        accelerated: accelerated_count,
        with_escapes,
        max_configs,
        total_configs,
    };

    let target_edges: usize = mode.states.iter().map(|state| state.target_intervals).sum();
    let escaped_edges_count: usize = mode
        .states
        .iter()
        .map(|state| state.escaped_intervals)
        .sum();
    let eof_edges = mode
        .states
        .iter()
        .filter(|state| state.has_eof_edge)
        .count();
    let self_loop_intervals = mode
        .states
        .iter()
        .map(|state| state.self_loop_intervals.len())
        .sum();
    let edges = StaticLexerEdgeStats {
        total: target_edges + escaped_edges_count,
        target: target_edges,
        escaped: escaped_edges_count,
        eof: eof_edges,
        self_loop_intervals,
        ascii_populated_cells: mode
            .table
            .as_ref()
            .map_or(0, StaticLexerDFA::ascii_populated_edges),
        hi_symbol_classes: mode.table.as_ref().map_or(0, StaticLexerDFA::num_classes),
        hi_distinct_rows: mode
            .table
            .as_ref()
            .map_or(0, StaticLexerDFA::num_distinct_hi_rows),
        escape_counts: mode.escape_counts,
    };

    let moved_configs = mode.table.as_ref().map_or(0, |table| {
        (0..mode.num_continuations as u32)
            .filter(|&id| table.continuation_kind(id) == CONTINUATION_MOVED_CONFIGS)
            .count()
    });
    let continuations = StaticLexerContinuationStats {
        total: mode.num_continuations,
        moved_configs,
        source_set: mode.num_continuations.saturating_sub(moved_configs),
    };

    let simple_recursive_rules = mode
        .simple_recursive_rules
        .iter()
        .filter_map(|compiled_rule| {
            let rule = unit.rules.get(compiled_rule.rule_index)?;
            let Accel::CountedExclusion { opener, closer, n } = compiled_rule.accel else {
                return None;
            };
            let len = usize::from(n);
            Some(StaticLexerSimpleRecursiveRuleReport {
                rule: rule.name.clone(),
                source_span: rule.span.clone(),
                opener: format_ascii_bytes(&opener[..len]),
                closer: format_ascii_bytes(&closer[..len]),
                delimiter_bytes: n,
            })
        })
        .collect();

    let accelerated_states = mode
        .states
        .iter()
        .enumerate()
        .filter_map(|(state_index, state_summary)| {
            let accel = state_summary.accel?;
            let accept_rule = state_summary
                .accept_rule_index
                .and_then(|rule_index| unit.rules.get(rule_index))
                .map(|rule| rule.name.clone());
            let accept_token_type =
                (state_summary.accept_type >= 0).then_some(state_summary.accept_type);
            let rules = state_summary
                .rule_indices
                .iter()
                .filter_map(|&rule_index| unit.rules.get(rule_index).map(|rule| rule.name.clone()))
                .collect();
            let self_loop_intervals = state_summary
                .self_loop_intervals
                .iter()
                .map(|&(lo, hi)| format_edge_label(lo, hi))
                .collect();
            Some(StaticLexerAcceleratedStateReport {
                state: state_index,
                accept_rule,
                accept_token_type,
                rules,
                self_loop_intervals,
                accel: build_accel_kind_report(accel, state_summary.hi_all_self_loop),
            })
        })
        .collect();

    let escaped_edges: Vec<StaticLexerEscapedEdgeReport> = mode
        .escaped_edges
        .iter()
        .map(|edge| build_escaped_edge_report(compiled, mode, edge))
        .collect();

    let rule_footprint = unit
        .rules
        .iter()
        .enumerate()
        .filter_map(|(rule_index, rule)| {
            let state_count = mode
                .states
                .iter()
                .filter(|state| state.rule_indices.contains(&rule_index))
                .count();
            let accept_states = mode
                .states
                .iter()
                .filter(|state| state.accept_rule_index == Some(rule_index))
                .count();
            let accelerated_states = mode
                .states
                .iter()
                .filter(|state| state.accel.is_some() && state.rule_indices.contains(&rule_index))
                .count();
            let escaped_count = escaped_edges
                .iter()
                .filter(|edge| edge.rule.as_deref() == Some(rule.name.as_str()))
                .count();
            let counted_exclusion = mode
                .simple_recursive_rules
                .iter()
                .any(|rec| rec.rule_index == rule_index);
            (state_count > 0 || escaped_count > 0 || counted_exclusion).then(|| {
                StaticLexerRuleFootprintReport {
                    rule: rule.name.clone(),
                    source_span: rule.span.clone(),
                    states: state_count,
                    accept_states,
                    accelerated_states,
                    escaped_edges: escaped_count,
                    counted_exclusion,
                }
            })
        })
        .collect();

    StaticLexerModeReport {
        index,
        mode: mode_name.to_owned(),
        source_span: mode_span(unit, mode_name),
        status,
        failure,
        states,
        edges,
        continuations,
        simple_recursive_rules,
        accelerated_states,
        escaped_edges,
        rule_footprint,
    }
}

fn build_mode_failure_report(
    compiled: &CompiledLexer,
    mode_name: &str,
    failure: ModeFailure,
) -> StaticLexerModeFailureReport {
    match failure {
        ModeFailure::StartClosurePredicate => {
            let rules = rules_with(compiled, mode_name, Construct::Predicate);
            StaticLexerModeFailureReport {
                code: "G4S188",
                kind: "start-closure-predicate",
                reason: format!(
                    "start-state closure crosses a predicate{}",
                    rule_names_note(rules.clone()),
                ),
                rules: rules.into_iter().map(str::to_owned).collect(),
                hint: "move the predicate after at least one matched character or replace it with explicit lexical patterns or lexer modes",
            }
        }
        ModeFailure::StartClosureCustomAction => {
            let rules = rules_with(compiled, mode_name, Construct::Action);
            StaticLexerModeFailureReport {
                code: "G4S188",
                kind: "start-closure-custom-action",
                reason: format!(
                    "start-state closure runs a position-dependent action{}",
                    rule_names_note(rules.clone()),
                ),
                rules: rules.into_iter().map(str::to_owned).collect(),
                hint: "move the embedded action after the rule prefix or replace it with a position-independent lexer command (-> pushMode, -> popMode, -> mode, -> skip, -> more, -> channel, -> type)",
            }
        }
        ModeFailure::StartClosureRecursiveContext => {
            let rules = rules_with(compiled, mode_name, Construct::Recursive);
            StaticLexerModeFailureReport {
                code: "G4S188",
                kind: "start-closure-recursive-context",
                reason: format!(
                    "start-state closure reaches a recursive rule context{}",
                    rule_names_note(rules.clone()),
                ),
                rules: rules.into_iter().map(str::to_owned).collect(),
                hint: "ensure recursive rule calls occur only after a non-empty literal prefix",
            }
        }
        ModeFailure::StartStateBudgetExceeded => StaticLexerModeFailureReport {
            code: "G4S189",
            kind: "start-state-budget-exceeded",
            reason: format!(
                "static DFA expansion exceeded the cap of {DEFAULT_MAX_STATES_PER_MODE} states"
            ),
            rules: Vec::new(),
            hint: "simplify overlapping character ranges or split rules across lexer modes",
        },
    }
}

fn build_accel_kind_report(accel: Accel, hi_all_self_loop: bool) -> StaticLexerAccelKindReport {
    match accel {
        Accel::Positive(bitmap) => {
            let ascii_count = bitmap.count_ones();
            let ascii_intervals = bitmap_intervals(bitmap, true);
            let excluded_ascii = if hi_all_self_loop {
                bitmap_intervals(bitmap, false)
            } else {
                Vec::new()
            };
            let excluded_count = 128 - ascii_count;
            let hint = (hi_all_self_loop && excluded_count > 3).then(|| {
                format!(
                    "all non-ASCII symbols self-loop, but {excluded_count} ASCII bytes are excluded; \
                     exclusion (memchr) acceleration supports at most 3 excluded ASCII bytes, so this \
                     state uses positive ASCII bitmap acceleration"
                )
            });
            StaticLexerAccelKindReport::Positive {
                ascii_count,
                ascii_intervals,
                bitmap_hex: format!("0x{bitmap:032x}"),
                hi_self_loop: hi_all_self_loop,
                excluded_ascii,
                hint,
            }
        }
        Accel::Exclude(n0) => StaticLexerAccelKindReport::Exclusion {
            needle_count: 1,
            needles: vec![format_symbol(i32::from(n0))],
            needle_bytes: vec![n0],
        },
        Accel::Exclude2(n0, n1) => StaticLexerAccelKindReport::Exclusion {
            needle_count: 2,
            needles: vec![format_symbol(i32::from(n0)), format_symbol(i32::from(n1))],
            needle_bytes: vec![n0, n1],
        },
        Accel::Exclude3(n0, n1, n2) => StaticLexerAccelKindReport::Exclusion {
            needle_count: 3,
            needles: vec![
                format_symbol(i32::from(n0)),
                format_symbol(i32::from(n1)),
                format_symbol(i32::from(n2)),
            ],
            needle_bytes: vec![n0, n1, n2],
        },
        Accel::CountedExclusion { opener, closer, n } => {
            let len = usize::from(n);
            StaticLexerAccelKindReport::CountedExclusion {
                opener: format_ascii_bytes(&opener[..len]),
                closer: format_ascii_bytes(&closer[..len]),
                delimiter_bytes: n,
            }
        }
    }
}

fn build_escaped_edge_report(
    compiled: &CompiledLexer,
    mode: &ModeBuildOutcome,
    edge: &EscapedEdge,
) -> StaticLexerEscapedEdgeReport {
    let unit = &compiled.semantic.unit;
    let (source_span, rule_name, message) = resolve_escape_site(compiled, edge.reason);
    let (reason, hint) = match edge.reason {
        EscapeReason::Predicate { .. } => (
            "predicate",
            "semantic predicates {...}? require runtime evaluation; replace with explicit lexical patterns or lexer modes to keep the edge static",
        ),
        EscapeReason::CustomAction { .. } => (
            "custom-action",
            "replace the embedded action {...} with a position-independent lexer command (-> pushMode, -> popMode, -> mode, -> skip, -> more, -> channel, -> type) to keep the edge static",
        ),
        EscapeReason::RecursiveContext { .. } => (
            "recursive-context",
            "general recursive lexer rules require runtime context stacks; rewrite as a non-recursive loop or a direct self-recursive delimiter rule ('opener' (RULE | .)*? 'closer') to enable static CountedExclusion acceleration",
        ),
        EscapeReason::Budget { .. } => (
            "state-budget",
            "mode exceeded the static DFA state budget; simplify overlapping character ranges, factor shared prefixes, or split rules across lexer modes",
        ),
    };
    let state_rules = mode
        .states
        .get(edge.state)
        .map(|state| {
            state
                .rule_indices
                .iter()
                .filter_map(|&rule_index| unit.rules.get(rule_index).map(|rule| rule.name.clone()))
                .collect()
        })
        .unwrap_or_default();
    let continuation = edge.continuation.and_then(|id| {
        let table = mode.table.as_ref()?;
        let kind = if table.continuation_kind(id) == CONTINUATION_MOVED_CONFIGS {
            "moved-configs"
        } else {
            "source-set"
        };
        let mut rules = Vec::new();
        for atn_state in table.continuation_config_states(id) {
            if let Some(rule) = compiled
                .graph
                .states
                .get(atn_state)
                .and_then(|state| state.rule)
                .and_then(|rule_id| rule_by_id(unit, rule_id))
                && !rules.contains(&rule.name)
            {
                rules.push(rule.name.clone());
            }
        }
        Some(StaticLexerContinuationReport {
            id,
            kind,
            config_count: table.continuation_num_configs(id),
            context_count: table.continuation_num_contexts(id),
            non_greedy_configs: table.continuation_num_non_greedy_configs(id),
            rules,
        })
    });
    StaticLexerEscapedEdgeReport {
        state: edge.state,
        state_rules,
        symbols: format_edge_label(edge.lo, edge.hi),
        symbol_start: edge.lo,
        symbol_end: edge.hi,
        reason,
        message,
        rule: rule_name.map(str::to_owned),
        source_span,
        hint,
        continuation,
    }
}

fn bitmap_intervals(bitmap: u128, set: bool) -> Vec<String> {
    let mut intervals = Vec::new();
    let mut start: Option<i32> = None;
    for bit in 0..128i32 {
        let active = ((bitmap >> bit) & 1 == 1) == set;
        match (start, active) {
            (None, true) => start = Some(bit),
            (Some(lo), false) => {
                intervals.push(format_edge_label(lo, bit - 1));
                start = None;
            }
            _ => {}
        }
    }
    if let Some(lo) = start {
        intervals.push(format_edge_label(lo, 127));
    }
    intervals
}

fn format_ascii_bytes(bytes: &[u8]) -> String {
    let mut out = String::new();
    for &byte in bytes {
        match byte {
            b'\t' => out.push_str("\\t"),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            0x20..=0x7E => out.push(char::from(byte)),
            _ => out.push_str(&format!("\\x{byte:02X}")),
        }
    }
    out
}

/// Per-mode failure diagnostic when a whole mode has no static table.
fn mode_failure_diagnostic(
    grammar_name: &str,
    mode_name: &str,
    failure: ModeFailure,
    compiled: &CompiledLexer,
    fallback: AtnFallback,
) -> Diagnostic {
    let unit = &compiled.semantic.unit;
    let span = mode_span(unit, mode_name);
    let detail = edge_attribution_note(grammar_name, mode_name, None);

    let reason = match failure {
        ModeFailure::StartClosurePredicate => Some(format!(
            "start-state closure crosses a predicate{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Predicate)),
        )),
        ModeFailure::StartClosureCustomAction => Some(format!(
            "start-state closure runs a position-dependent action{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Action)),
        )),
        ModeFailure::StartClosureRecursiveContext => Some(format!(
            "start-state closure reaches a recursive rule context{}",
            rule_names_note(rules_with(compiled, mode_name, Construct::Recursive)),
        )),
        ModeFailure::StartStateBudgetExceeded => None,
    };
    let (code, message) = match reason {
        Some(reason) => (
            "G4S188",
            format!("{detail}: static DFA expansion failed: {reason}"),
        ),
        None => (
            "G4S189",
            format!(
                "{detail}: static DFA expansion exceeded the cap of {DEFAULT_MAX_STATES_PER_MODE} states"
            ),
        ),
    };
    match fallback {
        AtnFallback::Allowed => Diagnostic::warning(
            code,
            span,
            format!("{message}; the whole mode is deferred to runtime expansion"),
        ),
        AtnFallback::Forbidden => static_only_error(code, span, &message),
    }
}

/// Per-edge diagnostic for one escaped DFA edge, with a source span in the
/// `.g4` file.
fn escaped_edge_diagnostic(
    grammar_name: &str,
    mode_name: &str,
    edge: &EscapedEdge,
    compiled: &CompiledLexer,
    fallback: AtnFallback,
) -> Diagnostic {
    let (span, rule_name, reason) = resolve_escape_site(compiled, edge.reason);
    let detail = edge_attribution_note(grammar_name, mode_name, rule_name);
    let symbol = format_edge_label(edge.lo, edge.hi);
    let message = format!(
        "{detail}: static-lexer-dfa failed to expand edge on {symbol} (state {}) \
         due to: {reason}",
        edge.state,
    );
    match fallback {
        AtnFallback::Allowed => Diagnostic::warning("G4S188", span, message),
        AtnFallback::Forbidden => static_only_error("G4S188", span, &message),
    }
}

/// The error that replaces a static lexer DFA warning under
/// `--static-dfa-only`.
fn static_only_error(code: &'static str, span: SourceSpan, message: &str) -> Diagnostic {
    Diagnostic::error(
        code,
        span,
        format!("{message}; --static-dfa-only requires a fully static lexer"),
    )
}

fn resolve_escape_site(
    compiled: &CompiledLexer,
    reason: EscapeReason,
) -> (SourceSpan, Option<&str>, &'static str) {
    let unit = &compiled.semantic.unit;
    match reason {
        EscapeReason::Predicate {
            rule_index,
            pred_index,
        } => {
            let binding = compiled
                .semantic
                .bindings
                .predicates
                .values()
                .find(|binding| binding.index == pred_index);
            let rule = binding
                .and_then(|binding| rule_by_id(unit, binding.rule))
                .or_else(|| unit.rules.get(rule_index));
            let span = binding
                .and_then(|binding| element_span(compiled, binding.rule, binding.element))
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "cross a predicate",
            )
        }
        EscapeReason::CustomAction {
            rule_index,
            action_index,
        } => {
            let binding = compiled
                .semantic
                .bindings
                .actions
                .values()
                .find(|binding| binding.index == action_index);
            let rule = binding
                .and_then(|binding| rule_by_id(unit, binding.rule))
                .or_else(|| unit.rules.get(rule_index));
            let span = binding
                .and_then(|binding| element_span(compiled, binding.rule, binding.element))
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "run a position-dependent action",
            )
        }
        EscapeReason::RecursiveContext {
            rule_index,
            state_number,
            return_state,
        } => {
            let rule_transition = compiled.graph.transitions.iter().find(|transition| {
                matches!(
                    transition.kind,
                    FinalizedTransitionKind::Rule { follow, .. } if follow == return_state
                )
            });
            let rule = rule_transition
                .and_then(|transition| compiled.graph.states.get(transition.source))
                .and_then(|state| state.rule)
                .and_then(|id| rule_by_id(unit, id))
                .or_else(|| {
                    compiled
                        .graph
                        .states
                        .get(return_state)
                        .and_then(|state| state.rule)
                        .and_then(|id| rule_by_id(unit, id))
                })
                .or_else(|| unit.rules.get(rule_index));
            let span = rule_transition
                .and_then(|transition| transition_span(compiled, transition.original))
                .or_else(|| state_span(compiled, return_state))
                .or_else(|| state_span(compiled, state_number))
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "reach a recursive rule context",
            )
        }
        EscapeReason::Budget {
            rule_index,
            state_number,
        } => {
            let rule = compiled
                .graph
                .states
                .get(state_number)
                .and_then(|state| state.rule)
                .and_then(|id| rule_by_id(unit, id))
                .or_else(|| unit.rules.get(rule_index));
            let span = state_span(compiled, state_number)
                .or_else(|| rule.map(|rule| rule.span.clone()))
                .unwrap_or_else(|| unit.span.clone());
            (
                span,
                rule.map(|rule| rule.name.as_str()),
                "exceed the state budget",
            )
        }
    }
}

fn mode_span(unit: &GrammarUnit, mode_name: &str) -> SourceSpan {
    if mode_name != "DEFAULT_MODE"
        && let Some(mode) = unit.modes.iter().find(|mode| mode.name == mode_name)
    {
        return mode.span.clone();
    }
    unit.span.clone()
}

fn rule_by_id(unit: &GrammarUnit, id: RuleId) -> Option<&Rule> {
    unit.rules.iter().find(|rule| rule.id == id)
}

fn element_span(
    compiled: &CompiledLexer,
    rule_id: RuleId,
    element_id: ElementId,
) -> Option<SourceSpan> {
    if let Some(rule) = rule_by_id(&compiled.semantic.unit, rule_id)
        && let Some(span) = find_element_span(&rule.block, element_id)
    {
        return Some(span);
    }
    origins_span(
        &compiled.provenance,
        compiled
            .provenance
            .origins(ModelNodeId::Element(element_id)),
    )
}

fn find_element_span(block: &Block, target: ElementId) -> Option<SourceSpan> {
    for alternative in &block.alternatives {
        for element in &alternative.elements {
            if element.id == target {
                return Some(element.span.clone());
            }
            if let ElementKind::Block(nested) = &element.kind
                && let Some(span) = find_element_span(nested, target)
            {
                return Some(span);
            }
        }
    }
    None
}

fn state_span(compiled: &CompiledLexer, state_number: usize) -> Option<SourceSpan> {
    let state = compiled.graph.states.get(state_number)?;
    origins_span(
        &compiled.provenance,
        compiled.provenance.state_origins(state.original),
    )
}

fn transition_span(compiled: &CompiledLexer, transition: BuildTransitionId) -> Option<SourceSpan> {
    origins_span(
        &compiled.provenance,
        compiled.provenance.transition_origins(transition),
    )
}

fn origins_span(provenance: &ProvenanceIndex, origins: &[Origin]) -> Option<SourceSpan> {
    for origin in origins {
        match origin {
            Origin::Authored { span, .. } => return Some(span.clone()),
            Origin::Synthetic { owner, .. } => {
                if let Some(span) = origins_span(provenance, provenance.origins(*owner)) {
                    return Some(span);
                }
            }
            _ => {}
        }
    }
    None
}

fn format_edge_label(lo: i32, hi: i32) -> String {
    if lo == hi {
        format_symbol(lo)
    } else {
        format!("{}..={}", format_symbol(lo), format_symbol(hi))
    }
}

fn format_symbol(symbol: i32) -> String {
    if symbol == EOF {
        return "EOF".to_owned();
    }
    match symbol {
        0x09 => "'\\t'".to_owned(),
        0x0A => "'\\n'".to_owned(),
        0x0D => "'\\r'".to_owned(),
        0x20..=0x7E => format!("{:?}", char::from_u32(symbol as u32).unwrap()),
        _ => format!("U+{symbol:04X}"),
    }
}

fn edge_attribution_note(grammar_name: &str, mode_name: &str, rule_name: Option<&str>) -> String {
    match rule_name {
        Some(rule) => format!(" ({grammar_name}, mode '{mode_name}', rule '{rule}')"),
        None => format!(" ({grammar_name}, mode '{mode_name}')"),
    }
}

fn rule_names_note(rules: Vec<&str>) -> String {
    if rules.is_empty() {
        return String::new();
    }
    let label = if rules.len() == 1 { "rule" } else { "rules" };
    format!(" ({label} {})", quote_join(rules.iter().copied()))
}

fn quote_join(names: impl Iterator<Item = impl AsRef<str>>) -> String {
    names
        .map(|name| format!("'{}'", name.as_ref()))
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Clone, Copy)]
enum Construct {
    Predicate,
    Action,
    Recursive,
}

fn rules_with<'u>(
    compiled: &'u CompiledLexer,
    mode_name: &str,
    construct: Construct,
) -> Vec<&'u str> {
    let unit = &compiled.semantic.unit;
    mode_rules(unit, mode_name)
        .into_iter()
        .filter(|rule| match construct {
            Construct::Predicate => block_has(&rule.block, &ElementDiscriminator::Predicate),
            Construct::Action => block_has(&rule.block, &ElementDiscriminator::Action),
            Construct::Recursive => is_recursive(compiled, rule.id),
        })
        .map(|rule| rule.name.as_str())
        .collect()
}

fn mode_rules<'u>(unit: &'u GrammarUnit, mode_name: &str) -> Vec<&'u Rule> {
    if mode_name == "DEFAULT_MODE" {
        return unit
            .rules
            .iter()
            .filter(|rule| rule.mode.is_none())
            .collect();
    }
    unit.modes
        .iter()
        .find(|mode| mode.name == mode_name)
        .map(|mode| {
            mode.rules
                .iter()
                .filter_map(|id| unit.rules.iter().find(|rule| rule.id == *id))
                .collect()
        })
        .unwrap_or_default()
}

enum ElementDiscriminator {
    Predicate,
    Action,
}

fn block_has(block: &Block, kind: &ElementDiscriminator) -> bool {
    block.alternatives.iter().any(|alternative| {
        alternative
            .elements
            .iter()
            .any(|element| match (&element.kind, kind) {
                (ElementKind::Predicate { .. }, ElementDiscriminator::Predicate)
                | (ElementKind::Action { .. }, ElementDiscriminator::Action) => true,
                (ElementKind::Block(nested), _) => block_has(nested, kind),
                _ => false,
            })
    })
}

fn is_recursive(compiled: &CompiledLexer, rule: RuleId) -> bool {
    let call_graph = &compiled.semantic.call_graph;
    let mut visited = std::collections::BTreeSet::new();
    let mut stack = call_graph.get(&rule).cloned().unwrap_or_default();
    while let Some(target) = stack.pop() {
        if target == rule {
            return true;
        }
        if visited.insert(target)
            && let Some(next) = call_graph.get(&target)
        {
            stack.extend(next.iter().copied());
        }
    }
    false
}

impl StaticLexerDfa {
    /// Renders the `static-lexer-dfa` entry of the optimization manifest.
    pub(super) fn manifest_pass_value(
        id: usize,
        descriptor: &PassDescriptor,
        lexers: &[StaticLexerDfaReport],
        sources: &SourceSet,
    ) -> Value {
        let changed = lexers.iter().any(|lexer| {
            lexer
                .modes
                .iter()
                .any(|mode| mode.status != StaticLexerModeStatus::Failed)
        });
        let fully_static_lexers = lexers.iter().filter(|lexer| lexer.omit_atn).count();
        let total_modes: usize = lexers.iter().map(|lexer| lexer.modes.len()).sum();
        let compiled_modes: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .filter(|mode| mode.status != StaticLexerModeStatus::Failed)
            .count();
        let failed_modes = total_modes.saturating_sub(compiled_modes);
        let total_states: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.states.total)
            .sum();
        let accept_states: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.states.accept)
            .sum();
        let accelerated_states: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.states.accelerated)
            .sum();
        let total_edges: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.edges.total)
            .sum();
        let target_edges: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.edges.target)
            .sum();
        let escaped_edges: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.edges.escaped)
            .sum();
        let continuations: usize = lexers
            .iter()
            .flat_map(|lexer| &lexer.modes)
            .map(|mode| mode.continuations.total)
            .sum();
        let table_words: usize = lexers.iter().map(|lexer| lexer.table_words).sum();

        let metrics = object(vec![
            ("lexers", Value::from(lexers.len())),
            ("fullyStaticLexers", Value::from(fully_static_lexers)),
            ("modes", Value::from(total_modes)),
            ("compiledModes", Value::from(compiled_modes)),
            ("failedModes", Value::from(failed_modes)),
            ("states", Value::from(total_states)),
            ("acceptStates", Value::from(accept_states)),
            ("acceleratedStates", Value::from(accelerated_states)),
            ("edges", Value::from(total_edges)),
            ("targetEdges", Value::from(target_edges)),
            ("escapedEdges", Value::from(escaped_edges)),
            ("continuations", Value::from(continuations)),
            ("tableWords", Value::from(table_words)),
            ("tableBytes", Value::from(table_words * 4)),
        ]);
        let rendered_lexers = Value::from_iter(
            lexers
                .iter()
                .map(|lexer| static_lexer_report_value(lexer, sources)),
        );
        super::manifest::pass_value(
            id,
            descriptor,
            changed,
            metrics,
            vec![("lexers", rendered_lexers)],
        )
    }
}

fn static_lexer_report_value(lexer: &StaticLexerDfaReport, sources: &SourceSet) -> Value {
    let compiled_modes = lexer
        .modes
        .iter()
        .filter(|mode| mode.status != StaticLexerModeStatus::Failed)
        .count();
    let failed_modes = lexer.modes.len().saturating_sub(compiled_modes);
    let total_states: usize = lexer.modes.iter().map(|mode| mode.states.total).sum();
    let accept_states: usize = lexer.modes.iter().map(|mode| mode.states.accept).sum();
    let accelerated_states: usize = lexer.modes.iter().map(|mode| mode.states.accelerated).sum();
    let total_edges: usize = lexer.modes.iter().map(|mode| mode.edges.total).sum();
    let target_edges: usize = lexer.modes.iter().map(|mode| mode.edges.target).sum();
    let escaped_edges: usize = lexer.modes.iter().map(|mode| mode.edges.escaped).sum();
    let continuations: usize = lexer
        .modes
        .iter()
        .map(|mode| mode.continuations.total)
        .sum();

    object(vec![
        ("grammar", Value::from(lexer.grammar.as_str())),
        ("source", source_value(&lexer.source_span, sources)),
        ("omitAtn", Value::from(lexer.omit_atn)),
        ("tableWords", Value::from(lexer.table_words)),
        ("tableBytes", Value::from(lexer.table_words * 4)),
        ("numModes", Value::from(lexer.modes.len())),
        ("compiledModes", Value::from(compiled_modes)),
        ("failedModes", Value::from(failed_modes)),
        ("totalStates", Value::from(total_states)),
        ("acceptStates", Value::from(accept_states)),
        ("acceleratedStates", Value::from(accelerated_states)),
        ("totalEdges", Value::from(total_edges)),
        ("targetEdges", Value::from(target_edges)),
        ("escapedEdges", Value::from(escaped_edges)),
        ("continuations", Value::from(continuations)),
        (
            "modes",
            Value::from_iter(
                lexer
                    .modes
                    .iter()
                    .map(|mode| static_lexer_mode_value(mode, sources)),
            ),
        ),
    ])
}

fn static_lexer_mode_value(mode: &StaticLexerModeReport, sources: &SourceSet) -> Value {
    let counts = mode.edges.escape_counts;
    let escape_total =
        counts.predicate + counts.custom_action + counts.recursive_context + counts.budget;
    object(vec![
        ("index", Value::from(mode.index)),
        ("mode", Value::from(mode.mode.as_str())),
        ("source", source_value(&mode.source_span, sources)),
        ("status", Value::from(mode_status_name(mode.status))),
        (
            "failure",
            option(mode.failure.as_ref(), static_lexer_mode_failure_value),
        ),
        (
            "statistics",
            object(vec![
                (
                    "states",
                    object(vec![
                        ("total", Value::from(mode.states.total)),
                        ("accept", Value::from(mode.states.accept)),
                        ("nonAccept", Value::from(mode.states.non_accept)),
                        ("accelerated", Value::from(mode.states.accelerated)),
                        ("withEscapes", Value::from(mode.states.with_escapes)),
                        ("maxConfigs", Value::from(mode.states.max_configs)),
                        ("totalConfigs", Value::from(mode.states.total_configs)),
                    ]),
                ),
                (
                    "edges",
                    object(vec![
                        ("total", Value::from(mode.edges.total)),
                        ("target", Value::from(mode.edges.target)),
                        ("escaped", Value::from(mode.edges.escaped)),
                        ("eof", Value::from(mode.edges.eof)),
                        (
                            "selfLoopIntervals",
                            Value::from(mode.edges.self_loop_intervals),
                        ),
                        (
                            "asciiPopulatedCells",
                            Value::from(mode.edges.ascii_populated_cells),
                        ),
                        ("hiSymbolClasses", Value::from(mode.edges.hi_symbol_classes)),
                        ("hiDistinctRows", Value::from(mode.edges.hi_distinct_rows)),
                        (
                            "escapeCounts",
                            object(vec![
                                ("budget", Value::from(counts.budget)),
                                ("customAction", Value::from(counts.custom_action)),
                                ("predicate", Value::from(counts.predicate)),
                                ("recursiveContext", Value::from(counts.recursive_context)),
                                ("total", Value::from(escape_total)),
                            ]),
                        ),
                    ]),
                ),
                (
                    "continuations",
                    object(vec![
                        ("total", Value::from(mode.continuations.total)),
                        (
                            "movedConfigs",
                            Value::from(mode.continuations.moved_configs),
                        ),
                        ("sourceSet", Value::from(mode.continuations.source_set)),
                    ]),
                ),
            ]),
        ),
        (
            "simpleRecursiveRules",
            Value::from_iter(
                mode.simple_recursive_rules
                    .iter()
                    .map(|rule| simple_recursive_rule_value(rule, sources)),
            ),
        ),
        (
            "acceleratedStates",
            Value::from_iter(mode.accelerated_states.iter().map(accelerated_state_value)),
        ),
        (
            "escapedEdges",
            Value::from_iter(
                mode.escaped_edges
                    .iter()
                    .map(|edge| escaped_edge_value(edge, sources)),
            ),
        ),
        (
            "ruleFootprint",
            Value::from_iter(
                mode.rule_footprint
                    .iter()
                    .map(|footprint| rule_footprint_value(footprint, sources)),
            ),
        ),
    ])
}

fn static_lexer_mode_failure_value(failure: &StaticLexerModeFailureReport) -> Value {
    object(vec![
        ("code", Value::from(failure.code)),
        ("kind", Value::from(failure.kind)),
        ("reason", Value::from(failure.reason.as_str())),
        (
            "rules",
            Value::from_iter(failure.rules.iter().map(|rule| Value::from(rule.as_str()))),
        ),
        ("hint", Value::from(failure.hint)),
    ])
}

fn simple_recursive_rule_value(
    rule: &StaticLexerSimpleRecursiveRuleReport,
    sources: &SourceSet,
) -> Value {
    object(vec![
        ("rule", Value::from(rule.rule.as_str())),
        ("source", source_value(&rule.source_span, sources)),
        ("opener", Value::from(rule.opener.as_str())),
        ("closer", Value::from(rule.closer.as_str())),
        ("delimiterBytes", Value::from(rule.delimiter_bytes)),
    ])
}

fn accelerated_state_value(state: &StaticLexerAcceleratedStateReport) -> Value {
    object(vec![
        ("state", Value::from(state.state)),
        (
            "acceptRule",
            option(state.accept_rule.as_deref(), Value::from),
        ),
        (
            "acceptTokenType",
            option(state.accept_token_type, Value::from),
        ),
        (
            "rules",
            Value::from_iter(state.rules.iter().map(|rule| Value::from(rule.as_str()))),
        ),
        (
            "selfLoopIntervals",
            Value::from_iter(
                state
                    .self_loop_intervals
                    .iter()
                    .map(|interval| Value::from(interval.as_str())),
            ),
        ),
        ("accel", accel_kind_value(&state.accel)),
    ])
}

fn accel_kind_value(accel: &StaticLexerAccelKindReport) -> Value {
    match accel {
        StaticLexerAccelKindReport::Positive {
            ascii_count,
            ascii_intervals,
            bitmap_hex,
            hi_self_loop,
            excluded_ascii,
            hint,
        } => object(vec![
            ("kind", Value::from("positive")),
            ("asciiCount", Value::from(*ascii_count)),
            (
                "asciiIntervals",
                Value::from_iter(
                    ascii_intervals
                        .iter()
                        .map(|interval| Value::from(interval.as_str())),
                ),
            ),
            ("bitmapHex", Value::from(bitmap_hex.as_str())),
            ("hiSelfLoop", Value::from(*hi_self_loop)),
            (
                "excludedAscii",
                Value::from_iter(
                    excluded_ascii
                        .iter()
                        .map(|interval| Value::from(interval.as_str())),
                ),
            ),
            ("hint", option(hint.as_deref(), Value::from)),
        ]),
        StaticLexerAccelKindReport::Exclusion {
            needle_count,
            needles,
            needle_bytes,
        } => object(vec![
            ("kind", Value::from("exclusion")),
            ("needleCount", Value::from(*needle_count)),
            (
                "needles",
                Value::from_iter(needles.iter().map(|needle| Value::from(needle.as_str()))),
            ),
            (
                "needleBytes",
                Value::from_iter(needle_bytes.iter().copied().map(Value::from)),
            ),
        ]),
        StaticLexerAccelKindReport::CountedExclusion {
            opener,
            closer,
            delimiter_bytes,
        } => object(vec![
            ("kind", Value::from("counted-exclusion")),
            ("opener", Value::from(opener.as_str())),
            ("closer", Value::from(closer.as_str())),
            ("delimiterBytes", Value::from(*delimiter_bytes)),
        ]),
    }
}

fn escaped_edge_value(edge: &StaticLexerEscapedEdgeReport, sources: &SourceSet) -> Value {
    object(vec![
        ("state", Value::from(edge.state)),
        (
            "stateRules",
            Value::from_iter(
                edge.state_rules
                    .iter()
                    .map(|rule| Value::from(rule.as_str())),
            ),
        ),
        ("symbols", Value::from(edge.symbols.as_str())),
        ("symbolStart", Value::from(edge.symbol_start)),
        ("symbolEnd", Value::from(edge.symbol_end)),
        ("reason", Value::from(edge.reason)),
        ("message", Value::from(edge.message)),
        ("rule", option(edge.rule.as_deref(), Value::from)),
        ("source", source_value(&edge.source_span, sources)),
        ("hint", Value::from(edge.hint)),
        (
            "continuation",
            option(edge.continuation.as_ref(), continuation_value),
        ),
    ])
}

fn continuation_value(continuation: &StaticLexerContinuationReport) -> Value {
    object(vec![
        ("id", Value::from(continuation.id)),
        ("kind", Value::from(continuation.kind)),
        ("configCount", Value::from(continuation.config_count)),
        ("contextCount", Value::from(continuation.context_count)),
        (
            "nonGreedyConfigs",
            Value::from(continuation.non_greedy_configs),
        ),
        (
            "rules",
            Value::from_iter(
                continuation
                    .rules
                    .iter()
                    .map(|rule| Value::from(rule.as_str())),
            ),
        ),
    ])
}

fn rule_footprint_value(footprint: &StaticLexerRuleFootprintReport, sources: &SourceSet) -> Value {
    object(vec![
        ("rule", Value::from(footprint.rule.as_str())),
        ("source", source_value(&footprint.source_span, sources)),
        ("states", Value::from(footprint.states)),
        ("acceptStates", Value::from(footprint.accept_states)),
        (
            "acceleratedStates",
            Value::from(footprint.accelerated_states),
        ),
        ("escapedEdges", Value::from(footprint.escaped_edges)),
        ("countedExclusion", Value::from(footprint.counted_exclusion)),
    ])
}

const fn mode_status_name(status: StaticLexerModeStatus) -> &'static str {
    match status {
        StaticLexerModeStatus::Compiled => "compiled",
        StaticLexerModeStatus::Partial => "partial",
        StaticLexerModeStatus::Failed => "failed",
    }
}
