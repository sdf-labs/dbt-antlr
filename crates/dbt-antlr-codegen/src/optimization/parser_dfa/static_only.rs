// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-dfa-only` resolution of the escape states of decision tables.
//!
//! A hybrid table escapes to the adaptive simulation where the static
//! construction stopped. Without that fallback, each escape state gets a
//! static verdict:
//!
//! - A state with an SLL conflict (a conflict escape or a guarded take)
//!   accepts its minimum conflicting alternative. This is the choice of the
//!   SLL prediction mode.
//! - Every other escape state (a budget frontier or a predicated branch)
//!   becomes a dead end without edges or fallback, so the walk reports a
//!   syntax error.

use super::{CONFLICT_ESCAPE, Category, DecisionResult, ESCAPE, GUARDED, StaticDfaTable};

/// How `--static-dfa-only` replaced one escape state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EscapeResolution {
    /// The state accepts its minimum conflicting alternative.
    MinAlt,
    /// The state reports a syntax error.
    SyntaxError,
}

/// The escape states that `--static-dfa-only` resolved in the tables of one
/// decision.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct EscapeResolutions {
    /// The number of states that accept their minimum conflicting alternative.
    pub(crate) min_alt: usize,
    /// The number of states that report a syntax error.
    pub(crate) syntax_error: usize,
    /// A shortest lookahead path to a [`EscapeResolution::MinAlt`] state.
    pub(crate) min_alt_witness: Option<Vec<i32>>,
    /// A shortest lookahead path to a [`EscapeResolution::SyntaxError`] state.
    pub(crate) syntax_error_witness: Option<Vec<i32>>,
}

impl EscapeResolutions {
    fn merge(&mut self, other: Self) {
        self.min_alt += other.min_alt;
        self.syntax_error += other.syntax_error;
        if self.min_alt_witness.is_none() {
            self.min_alt_witness = other.min_alt_witness;
        }
        if self.syntax_error_witness.is_none() {
            self.syntax_error_witness = other.syntax_error_witness;
        }
    }
}

impl StaticDfaTable {
    /// Gives every escape and guarded state a static verdict, so that the
    /// table never defers to the adaptive simulation.
    pub(crate) fn resolve_escapes(&mut self) -> EscapeResolutions {
        let mut resolutions = EscapeResolutions::default();
        let mut first_min_alt = None;
        let mut first_syntax_error = None;
        for (index, state) in self.states.iter_mut().enumerate() {
            if !matches!(state.accept, ESCAPE | CONFLICT_ESCAPE | GUARDED) {
                continue;
            }
            let min_alt = if state.accept == GUARDED && state.guarded_alt > 0 {
                // The guarded alternative is the take of a take/skip pair,
                // which is the minimum alternative of the pair.
                Some(state.guarded_alt)
            } else {
                state.conflicts.iter().copied().filter(|alt| *alt > 0).min()
            };
            let resolution = if let Some(alt) = min_alt {
                state.accept = alt;
                resolutions.min_alt += 1;
                first_min_alt.get_or_insert(index);
                EscapeResolution::MinAlt
            } else {
                state.accept = 0;
                state.fallback = 0;
                state.edges.clear();
                resolutions.syntax_error += 1;
                first_syntax_error.get_or_insert(index);
                EscapeResolution::SyntaxError
            };
            state.resolution = Some(resolution);
            state.conflict_min_alt = 0;
            state.guarded_alt = 0;
            state.continuation = None;
        }
        self.guard_danger.clear();
        self.guard_pass.clear();
        resolutions.min_alt_witness = first_min_alt.and_then(|state| self.witness(state));
        resolutions.syntax_error_witness = first_syntax_error.and_then(|state| self.witness(state));
        resolutions
    }
}

impl DecisionResult {
    /// Resolves the escape states of every table of the decision for
    /// `--static-dfa-only` and records the result in
    /// [`DecisionResult::static_only`].
    ///
    /// Returns `false` and changes nothing when the generated parser still
    /// needs `adaptivePredict` for the decision. This is the case when the
    /// decision has no table, when a precedence class has no table, and for a
    /// non-greedy decision.
    pub(crate) fn resolve_static_only(&mut self) -> bool {
        let mut resolutions = EscapeResolutions::default();
        match self.category {
            Category::LL1 => {}
            Category::NON_GREEDY => return false,
            _ => {
                if let Some(precedence) = &mut self.precedence {
                    if precedence.tables.is_empty() || precedence.adaptive_classes > 0 {
                        return false;
                    }
                    for table in precedence.tables.iter_mut().flatten() {
                        resolutions.merge(table.resolve_escapes());
                    }
                    precedence.escapes = 0;
                } else if let Some(table) = &mut self.table {
                    resolutions.merge(table.resolve_escapes());
                } else {
                    return false;
                }
            }
        }
        self.escapes = 0;
        self.guarded_takes = 0;
        self.static_only = Some(resolutions);
        true
    }
}
