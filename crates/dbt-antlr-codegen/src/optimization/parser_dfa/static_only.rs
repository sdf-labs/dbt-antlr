// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-dfa-only` resolution of the escape states of decision tables.
//!
//! A hybrid table escapes to the adaptive simulation where the static
//! construction stopped. Without that fallback, each escape state gets a
//! static verdict:
//!
//! - A guarded state keeps its runtime stack guard. When the guard defers, the
//!   state returns the skip of its optional postfix or the exit of its
//!   precedence loop instead of the adaptive simulation. The guard walks only
//!   the parse stack, so this needs no ATN simulation. This is also a guess:
//!   the guard sees the guard roots on the stack, but not the input after the
//!   lookahead of the table.
//! - A state with an SLL conflict (a conflict escape) accepts its minimum
//!   conflicting alternative. This is the choice of the SLL prediction mode.
//! - A state where the lookahead budget stopped the construction accepts its
//!   minimum viable alternative. This is a guess: if the input continues in
//!   a different alternative, the parse reports a syntax error later.
//!   Grammars usually put the specific alternatives before the general ones,
//!   so the guess is usually right.
//! - A predicated state becomes a dead end without edges or fallback, so the
//!   walk reports a syntax error. The table cannot evaluate the predicate.

use super::{CONFLICT_ESCAPE, Category, DecisionResult, ESCAPE, GUARDED, StaticDfaTable};

/// How `--static-dfa-only` replaced one escape state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum EscapeResolution {
    /// The state keeps its stack guard, and returns its defer alternative
    /// when the guard defers.
    Guarded,
    /// The state accepts its minimum conflicting alternative.
    MinAlt,
    /// The lookahead budget stopped the construction at the state. The state
    /// accepts its minimum viable alternative.
    MinViableAlt,
    /// A predicate made the state undecidable. The state reports a syntax
    /// error.
    SyntaxError,
}

/// The escape states that `--static-dfa-only` resolved in the tables of one
/// decision.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct EscapeResolutions {
    /// The number of guarded states that return their defer alternative when
    /// the guard defers.
    pub(crate) guarded: usize,
    /// The number of states that accept their minimum conflicting alternative.
    pub(crate) min_alt: usize,
    /// The number of states that accept their minimum viable alternative.
    pub(crate) min_viable_alt: usize,
    /// The number of states that report a syntax error.
    pub(crate) syntax_error: usize,
    /// A shortest lookahead path to a [`EscapeResolution::Guarded`] state.
    pub(crate) guarded_witness: Option<Vec<i32>>,
    /// A shortest lookahead path to a [`EscapeResolution::MinAlt`] state.
    pub(crate) min_alt_witness: Option<Vec<i32>>,
    /// A shortest lookahead path to a [`EscapeResolution::MinViableAlt`]
    /// state.
    pub(crate) min_viable_alt_witness: Option<Vec<i32>>,
    /// A shortest lookahead path to a [`EscapeResolution::SyntaxError`] state.
    pub(crate) syntax_error_witness: Option<Vec<i32>>,
    /// The states that resolved to one alternative over others, grouped by
    /// the chosen alternative and the dropped alternatives. The groups with
    /// the most states come first.
    pub(crate) tie_breaks: Vec<TieBreak>,
}

/// The states of one decision where `--static-dfa-only` chose one
/// alternative over other viable or conflicting alternatives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TieBreak {
    /// The alternative that the states predict.
    pub(crate) chosen: i32,
    /// The sorted alternatives that the states drop.
    pub(crate) dropped: Vec<i32>,
    /// The number of states.
    pub(crate) states: usize,
    /// A shortest lookahead path to one of the states.
    pub(crate) witness: Option<Vec<i32>>,
}

impl TieBreak {
    fn key(&self) -> (i32, &[i32]) {
        (self.chosen, &self.dropped)
    }

    /// Adds `other` to the group: the states, and the shorter witness.
    fn absorb(&mut self, other: Self) {
        self.states += other.states;
        let shorter = match (&self.witness, &other.witness) {
            (None, Some(_)) => true,
            (Some(known), Some(new)) => new.len() < known.len(),
            _ => false,
        };
        if shorter {
            self.witness = other.witness;
        }
    }
}

/// Adds `group` to `groups`, merged into the group with the same key.
fn add_tie_break(groups: &mut Vec<TieBreak>, group: TieBreak) {
    match groups.iter_mut().find(|known| known.key() == group.key()) {
        Some(known) => known.absorb(group),
        None => groups.push(group),
    }
}

/// Orders tie-break groups: the most states first, then by alternatives.
fn sort_tie_breaks(groups: &mut [TieBreak]) {
    groups.sort_by(|left, right| {
        right
            .states
            .cmp(&left.states)
            .then_with(|| left.chosen.cmp(&right.chosen))
            .then_with(|| left.dropped.cmp(&right.dropped))
    });
}

impl EscapeResolutions {
    fn merge(&mut self, other: Self) {
        self.guarded += other.guarded;
        self.min_alt += other.min_alt;
        self.min_viable_alt += other.min_viable_alt;
        self.syntax_error += other.syntax_error;
        if self.guarded_witness.is_none() {
            self.guarded_witness = other.guarded_witness;
        }
        if self.min_alt_witness.is_none() {
            self.min_alt_witness = other.min_alt_witness;
        }
        if self.min_viable_alt_witness.is_none() {
            self.min_viable_alt_witness = other.min_viable_alt_witness;
        }
        if self.syntax_error_witness.is_none() {
            self.syntax_error_witness = other.syntax_error_witness;
        }
        for group in other.tie_breaks {
            add_tie_break(&mut self.tie_breaks, group);
        }
        sort_tie_breaks(&mut self.tie_breaks);
    }
}

impl StaticDfaTable {
    /// Gives every escape state a static verdict and every guarded state a
    /// defer alternative, so that the table never defers to the adaptive
    /// simulation.
    pub(crate) fn resolve_escapes(&mut self) -> EscapeResolutions {
        let mut resolutions = EscapeResolutions::default();
        let mut first_guarded = None;
        let mut first_min_alt = None;
        let mut first_min_viable_alt = None;
        let mut first_syntax_error = None;
        let depths = self.depths();
        // The tie-break groups, each with the shallowest state that is in it.
        let mut groups: Vec<(TieBreak, usize)> = Vec::new();
        for (index, state) in self.states.iter_mut().enumerate() {
            if !matches!(state.accept, ESCAPE | CONFLICT_ESCAPE | GUARDED) {
                continue;
            }
            if state.accept == GUARDED && state.guarded_alt > 0 && state.guard_defer_alt > 0 {
                state.resolution = Some(EscapeResolution::Guarded);
                resolutions.guarded += 1;
                first_guarded.get_or_insert(index);
                continue;
            }
            let conflict_alt = if state.accept == GUARDED && state.guarded_alt > 0 {
                // The guarded alternative is the take or iterate branch,
                // which is the minimum alternative of the pair.
                Some(state.guarded_alt)
            } else {
                state.conflicts.iter().copied().filter(|alt| *alt > 0).min()
            };
            let viable_alt = if state.predicated {
                None
            } else {
                state.viable_alts.first().copied()
            };
            let mut dropped: Vec<i32> = if conflict_alt.is_some() {
                state.conflicts.clone()
            } else {
                state.viable_alts.clone()
            };
            let resolution = if let Some(alt) = conflict_alt {
                state.accept = alt;
                resolutions.min_alt += 1;
                first_min_alt.get_or_insert(index);
                EscapeResolution::MinAlt
            } else if let Some(alt) = viable_alt {
                state.accept = alt;
                resolutions.min_viable_alt += 1;
                first_min_viable_alt.get_or_insert(index);
                EscapeResolution::MinViableAlt
            } else {
                state.accept = 0;
                state.fallback = 0;
                state.edges.clear();
                resolutions.syntax_error += 1;
                first_syntax_error.get_or_insert(index);
                EscapeResolution::SyntaxError
            };
            state.resolution = Some(resolution);
            dropped.retain(|alt| *alt > 0 && *alt != state.accept);
            dropped.sort_unstable();
            dropped.dedup();
            if resolution != EscapeResolution::SyntaxError && !dropped.is_empty() {
                let group = TieBreak {
                    chosen: state.accept,
                    dropped,
                    states: 1,
                    witness: None,
                };
                match groups
                    .iter_mut()
                    .find(|(known, _)| known.key() == group.key())
                {
                    Some((known, shallowest)) => {
                        known.states += 1;
                        if depths[index] < depths[*shallowest] {
                            *shallowest = index;
                        }
                    }
                    None => groups.push((group, index)),
                }
            }
            state.conflict_min_alt = 0;
            state.guarded_alt = 0;
            state.guard_defer_alt = 0;
            state.guard_group = 0;
            state.continuation = None;
        }
        if resolutions.guarded == 0 {
            self.guard_groups.clear();
            self.guard_pass.clear();
        }
        resolutions.guarded_witness = first_guarded.and_then(|state| self.witness(state));
        resolutions.min_alt_witness = first_min_alt.and_then(|state| self.witness(state));
        resolutions.min_viable_alt_witness =
            first_min_viable_alt.and_then(|state| self.witness(state));
        resolutions.syntax_error_witness = first_syntax_error.and_then(|state| self.witness(state));
        resolutions.tie_breaks = groups
            .into_iter()
            .map(|(mut group, shallowest)| {
                group.witness = self.witness(shallowest);
                group
            })
            .collect();
        sort_tie_breaks(&mut resolutions.tie_breaks);
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
        self.static_only = Some(resolutions);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{EscapeResolutions, TieBreak};

    fn group(chosen: i32, dropped: &[i32], states: usize, witness: &[i32]) -> TieBreak {
        TieBreak {
            chosen,
            dropped: dropped.to_vec(),
            states,
            witness: Some(witness.to_vec()),
        }
    }

    /// The tie-break groups of several tables, such as the precedence
    /// classes of one decision, merge by alternatives. A merged group keeps
    /// the shorter witness, and the largest group comes first.
    #[test]
    fn tie_break_groups_merge_over_tables() {
        let mut resolutions = EscapeResolutions {
            tie_breaks: vec![group(1, &[2], 2, &[5, 6, 7]), group(3, &[4], 1, &[8])],
            ..EscapeResolutions::default()
        };
        resolutions.merge(EscapeResolutions {
            tie_breaks: vec![group(1, &[2], 3, &[5]), group(1, &[3], 4, &[9])],
            ..EscapeResolutions::default()
        });
        assert_eq!(
            resolutions.tie_breaks,
            [
                group(1, &[2], 5, &[5]),
                group(1, &[3], 4, &[9]),
                group(3, &[4], 1, &[8]),
            ]
        );
    }
}
