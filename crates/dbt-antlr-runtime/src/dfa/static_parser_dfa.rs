// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Static parser SLL prediction tables and their `SDFA` wire encoding.
//!
//! A table is a precomputed SLL DFA of one prediction decision: rows of token
//! edges over a deduplicated state graph, the accept alternative of every
//! accept state, and the fallback of every state that a lookahead can leave.
//! States the construction could not decide carry [`ESCAPE`] and the runtime
//! hands the decision back to the adaptive simulation; states where the
//! construction found an SLL conflict it trusts carry [`CONFLICT_ESCAPE`] plus
//! the conflicting alternatives, so the runtime can skip Stage 1 SLL entirely.
//!
//! # Escape handoff
//!
//! Prediction is a two-stage algorithm: Stage 1 is context-free SLL over the
//! cached dynamic DFA, Stage 2 is full-context LL and only runs when Stage 1
//! conflicts. An escape has to name which stage continues:
//!
//! - A [`CONFLICT_ESCAPE`] state is an SLL conflict, so Stage 1 has *already*
//!   terminated in conflict at this lookahead depth. Restarting SLL from the
//!   decision start would only re-discover the conflict and rewind, so the
//!   table also stores the conflicting alternatives and the walker resumes at
//!   Stage 2 directly.
//! - A frontier [`ESCAPE`] state stopped because the construction ran out of
//!   budget, *not* because SLL conflicted. SLL must keep scanning from the
//!   frontier, so the state carries a [`ParserContinuation`]: the closed,
//!   input-independent config set the construction had reached. The runtime
//!   materializes that config set once per escape state into the dynamic DFA
//!   and resumes the SLL simulation at `la(k+1)`, which removes the
//!   "double-walk" penalty (k static plus k dynamic steps) of restarting from
//!   the decision start.
//!
//! # Wire format
//!
//! The tables ride in a self-locating `SDFA` section appended to the packed
//! parser ATN word stream (design decision D1). The section starts after the
//! last packed ATN section and repeats its own magic, version, and length, so
//! the packed ATN decoder stays unchanged for streams without tables.
//!
//! Edges are stored per *distinct row*: every state names one row of
//! `edge_width` columns (token `t` is column `t + 1`, EOF is column 0) and a
//! row is a list of inclusive token ranges. Most states of a decision share a
//! row with their neighbours, so the row table is much smaller than the state
//! table, and the walker reads one row per state instead of a binary search.

use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter};

/// Magic of an `SDFA` section.
pub const STATIC_PARSER_DFA_MAGIC: u32 = 0x5344_4641;

/// Version of the `SDFA` wire format.
///
/// Version 2 added the guarded-take section (guarded alternatives, guard danger
/// and pass sets).
pub const FORMAT_VERSION: u32 = 2;

/// Accept marker of a state whose decision must be recomputed by the adaptive
/// simulation: the static construction stopped here.
pub const ESCAPE: i32 = -1;

/// Accept marker of a state where the static construction found an SLL conflict
/// it trusts. The walker resumes at Stage 2 (full context) instead of Stage 1.
pub const CONFLICT_ESCAPE: i32 = -2;

/// Accept marker of a state whose alternative is decided by a runtime stack
/// guard: the tree is resolved to the guarded alternative unless the real parse
/// stack can reach one of the decision's guard roots, in which case the walker
/// defers to the adaptive simulation.
pub const GUARDED: i32 = -3;

/// The empty prediction context in a continuation context DAG.
pub(crate) const EMPTY_CONTEXT: u32 = u32::MAX;

/// Largest precedence value the dispatch resolves through its dense table.
const MAX_DENSE_PRECEDENCE: i32 = 1024;

/// Rejects a record count that cannot fit in the remaining stream.
///
/// Every count the decoder pre-allocates from comes from the stream, so an
/// unchecked count would let a short stream claim a huge allocation.
fn checked_count(
    count: usize,
    words_per_record: usize,
    remaining: usize,
) -> Result<usize, StaticParserDfaError> {
    if count.saturating_mul(words_per_record) > remaining {
        return Err(StaticParserDfaError::TruncatedStream);
    }
    Ok(count)
}

/// Word index of the total-length word in a packed ATN header.
const PACKED_ATN_TOTAL_LEN: usize = 25;

/// Decoder cap on the states of one table. The classifier stops at 1000.
const MAX_TABLE_STATES: usize = 1 << 16;
/// Decoder cap on the distinct rows of one table.
const MAX_TABLE_ROWS: usize = 1 << 16;
/// Decoder cap on the token columns of one table.
const MAX_TABLE_COLUMNS: usize = 1 << 16;
/// Decoder cap on the expanded cells of one table (64 MiB of `i32`).
const MAX_TABLE_CELLS: usize = 1 << 24;

/// Failure while decoding an `SDFA` word stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticParserDfaError {
    /// The stream ends before the declared data.
    TruncatedStream,
    /// Word 0 is not the `SDFA` magic.
    BadMagic {
        /// The value found in word 0.
        found: u32,
    },
    /// The format version is not [`FORMAT_VERSION`].
    UnsupportedVersion {
        /// The value found in word 1.
        found: u32,
    },
    /// The declared section length does not match the stream.
    LengthMismatch {
        /// The declared word count.
        declared: usize,
        /// The actual word count.
        actual: usize,
    },
    /// A guarded state record names a state twice.
    DuplicateGuard {
        /// The state index found twice.
        state: usize,
    },
    /// A guard set names a negative invoking state.
    InvalidGuardState {
        /// The value found.
        state: i32,
    },
    /// A state row index is out of bounds.
    InvalidRowIndex {
        /// The row index found.
        row: i32,
        /// The number of rows of the table.
        num_rows: usize,
    },
    /// The row edge offsets are not a monotone sequence of range starts.
    InvalidEdgeOffsets,
    /// An edge token range is empty or outside the row width.
    InvalidEdgeBounds {
        /// The first token of the range.
        lo: i32,
        /// The last token of the range.
        hi: i32,
        /// The number of columns of the table.
        edge_width: usize,
    },
    /// An edge target is not a state of the table.
    InvalidEdgeTarget {
        /// The target state found.
        target: i32,
        /// The number of states of the table.
        num_states: usize,
    },
    /// A record names a state that is not part of the table.
    InvalidStateIndex {
        /// The state index found.
        state: usize,
        /// The number of states of the table.
        num_states: usize,
    },
    /// A table names a decision slot that does not exist.
    InvalidDecision {
        /// The decision index found.
        decision: usize,
        /// The number of decision slots.
        num_slots: usize,
    },
    /// A precedence dispatch names a table that does not exist.
    InvalidDispatchTable {
        /// The table index found.
        table_index: i32,
        /// The number of tables of the section.
        num_tables: usize,
    },
    /// Precedence dispatch cutoffs are not strictly ascending.
    UnsortedCutoffs,
    /// A decision carries two precedence dispatches.
    DuplicateDispatch {
        /// The decision index.
        decision: usize,
    },
    /// A continuation context node refers to a node that is not its ancestor.
    InvalidContinuationContext {
        /// The parent index found.
        index: u32,
        /// The number of nodes decoded before it.
        decoded: usize,
    },
    /// A continuation context node has no parents.
    EmptyContinuationContext {
        /// The index of the node in its continuation.
        index: usize,
    },
    /// Two continuations are registered for the same escape state.
    DuplicateContinuation {
        /// The escape state.
        state: u32,
    },
    /// A conflict escape carries no positive minimum alternative.
    InvalidConflictAlt {
        /// The alternative found.
        alt: i32,
    },
    /// A continuation config carries an out-of-range state, alternative, or
    /// outer-context depth.
    InvalidContinuationConfig {
        /// The config's ATN state number.
        state: i32,
        /// The config's alternative.
        alt: i32,
        /// The config's outer-context depth.
        outer_depth: i32,
    },
    /// A continuation context names a negative return state.
    InvalidReturnState {
        /// The return state found.
        return_state: i32,
    },
    /// A table or decision slot points outside the ATN it is loaded into.
    OutsideAtn {
        /// What the index addresses.
        what: &'static str,
        /// The index found.
        index: i64,
        /// The number of valid entries.
        count: usize,
    },
    /// The declared table shape is empty or too large to decode.
    InvalidTableSize {
        /// The declared number of states.
        num_states: usize,
        /// The declared number of rows.
        num_rows: usize,
        /// The declared number of token columns.
        edge_width: usize,
    },
}

impl Display for StaticParserDfaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedStream => f.write_str("truncated static parser DFA stream"),
            Self::BadMagic { found } => write!(f, "bad static parser DFA magic 0x{found:08x}"),
            Self::UnsupportedVersion { found } => {
                write!(f, "unsupported static parser DFA format version {found}")
            }
            Self::LengthMismatch { declared, actual } => write!(
                f,
                "static parser DFA section declares {declared} words but holds {actual}"
            ),
            Self::InvalidRowIndex { row, num_rows } => {
                write!(f, "row index {row} out of bounds for {num_rows} rows")
            }
            Self::InvalidEdgeOffsets => f.write_str("invalid state row edge offsets"),
            Self::DuplicateGuard { state } => {
                write!(f, "state {state} carries two guarded alternatives")
            }
            Self::InvalidGuardState { state } => {
                write!(f, "guard set names a negative invoking state {state}")
            }
            Self::InvalidEdgeBounds {
                lo,
                hi,
                edge_width,
            } => write!(
                f,
                "edge token range {lo}..={hi} out of bounds for width {edge_width}"
            ),
            Self::InvalidEdgeTarget {
                target,
                num_states,
            } => write!(
                f,
                "edge target {target} out of bounds for {num_states} states"
            ),
            Self::InvalidStateIndex { state, num_states } => {
                write!(f, "state index {state} out of bounds for {num_states} states")
            }
            Self::InvalidDecision {
                decision,
                num_slots,
            } => write!(
                f,
                "decision {decision} out of bounds for {num_slots} decision slots"
            ),
            Self::InvalidDispatchTable {
                table_index,
                num_tables,
            } => write!(
                f,
                "precedence class table {table_index} out of bounds for {num_tables} tables"
            ),
            Self::UnsortedCutoffs => {
                f.write_str("precedence dispatch cutoffs are not strictly ascending")
            }
            Self::DuplicateDispatch { decision } => {
                write!(f, "decision {decision} carries two precedence dispatches")
            }
            Self::InvalidContinuationContext { index, decoded } => write!(
                f,
                "continuation context parent {index} is not one of the {decoded} preceding nodes"
            ),
            Self::EmptyContinuationContext { index } => {
                write!(f, "continuation context node {index} has no parents")
            }
            Self::DuplicateContinuation { state } => {
                write!(f, "two continuations are registered for state {state}")
            }
            Self::InvalidConflictAlt { alt } => {
                write!(f, "conflict escape alternative {alt}; expected a positive alt")
            }
            Self::InvalidContinuationConfig {
                state,
                alt,
                outer_depth,
            } => write!(
                f,
                "continuation config (state {state}, alt {alt}, outer depth {outer_depth}) is out of range"
            ),
            Self::InvalidReturnState { return_state } => {
                write!(f, "continuation return state {return_state}; expected -1 or a state number")
            }
            Self::OutsideAtn { what, index, count } => {
                write!(f, "{what} {index} out of bounds for {count} ATN entries")
            }
            Self::InvalidTableSize {
                num_states,
                num_rows,
                edge_width,
            } => write!(
                f,
                "invalid static parser DFA table shape: {num_states} states, {num_rows} rows, {edge_width} columns"
            ),
        }
    }
}

impl std::error::Error for StaticParserDfaError {}

/// One node of a continuation context DAG.
///
/// The nodes are in topological order: a node refers only to nodes that come
/// before it, and [`EMPTY_CONTEXT`] names the empty context. A one-parent node
/// is a `Singleton` context and a node with two or more parents is an `Array`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContinuationContextNode {
    /// A `Singleton` context: one return state on top of one parent context.
    Singleton {
        /// Parent node index, or [`EMPTY_CONTEXT`].
        parent: u32,
        /// ATN state number of the return state, or `-1` for none.
        return_state: i32,
    },
    /// An `Array` context: the merge of two or more contexts.
    Array {
        /// Parent node indexes, or [`EMPTY_CONTEXT`] entries.
        parents: Vec<u32>,
        /// ATN state number per parent, or `-1` for none.
        return_states: Vec<i32>,
    },
}

/// One config of a [`ParserContinuation`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuationConfigEntry {
    /// ATN state number.
    pub state: i32,
    /// Prediction alternative.
    pub alt: i32,
    /// Context node index, or [`EMPTY_CONTEXT`] for the empty context.
    pub context: u32,
    /// How many times the config popped out of the decision's rule.
    pub outer_depth: i32,
}

/// The closed SLL config set a frontier escape stopped at.
///
/// The set is post-closure and input-independent, so the runtime materializes
/// it at most once per escape state and resumes the SLL simulation from it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParserContinuation {
    /// DFA state number of the escape state.
    pub state: u32,
    /// Context DAG nodes, in topological order.
    pub contexts: Vec<ContinuationContextNode>,
    /// Configs of the closed set.
    pub configs: Vec<ContinuationConfigEntry>,
}

/// The table of one decision, or of one precedence class.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticParserTable {
    /// Decision index, or `-1` for a table only reachable through a dispatch.
    decision: i32,
    /// Accept marker or predicted alternative per state.
    accepts: Vec<i32>,
    /// Lowest alternative that already left the decision per state.
    fallbacks: Vec<i32>,
    /// Lowest conflicting alternative per state, `0` when the state is not a
    /// conflict escape.
    conflict_min_alts: Vec<i32>,
    /// Continuations of the frontier escape states.
    continuations: Vec<ParserContinuation>,
    /// Continuation index per state, [`EMPTY_CONTEXT`] when the state has none.
    continuation_of: Vec<u32>,
    /// The alternative a guarded state resolves to, `0` for the other states.
    guarded_alts: Vec<i32>,
    /// Sorted invoking states whose follow region reaches a guard root: the
    /// runtime guard defers when the parse stack carries one.
    guard_danger: Vec<i32>,
    /// Sorted invoking states whose follow region can pop out of its rule
    /// without reaching a guard root: the stack walk continues past them.
    guard_pass: Vec<i32>,
    /// Number of token columns of a row (`max_token + 2`).
    edge_width: usize,
    /// Row index per state.
    row_of: Vec<u32>,
    /// The distinct rows, `edge_width` columns each, `-1` for "no edge".
    rows: Vec<i32>,
}

/// How the tables of one decision are selected.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecisionTables {
    /// The decision has no table and always runs the adaptive simulation.
    None,
    /// The decision has exactly one table.
    Single(usize),
    /// The decision is a precedence loop with one table per precedence class.
    Dispatch(PrecedenceDispatch),
}

/// Precedence classes of a left-recursive loop decision.
///
/// `class_tables[i]` is the class table for `cutoffs[i-1] < p <= cutoffs[i]`
/// (the first class covers `p <= cutoffs[0]`, the last one `p > cutoffs[last]`)
/// or `-1` when the class stays adaptive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecedenceDispatch {
    /// The precedence cut points, ascending.
    pub cutoffs: Vec<i32>,
    /// One table index per class, `-1` for an adaptive class.
    pub class_tables: Vec<i32>,
    /// Table index per precedence value, indexed by `max(p, 0)`.
    by_precedence: Box<[i32]>,
}

impl PrecedenceDispatch {
    /// Builds the dispatch and its dense precedence lookup table.
    pub fn new(cutoffs: Vec<i32>, class_tables: Vec<i32>) -> Self {
        let last = cutoffs.last().copied().unwrap_or(-1);
        // `p > cutoffs[last]` is the last class, so the dense table has one
        // entry past the last cutoff. Huge precedence values stay out of it
        // and take the counted fallback instead of a large allocation.
        let dense_end = last.saturating_add(1).clamp(0, MAX_DENSE_PRECEDENCE);
        let mut by_precedence = Vec::with_capacity(dense_end as usize + 1);
        for precedence in 0..=dense_end {
            by_precedence.push(class_tables[class_of(&cutoffs, precedence)]);
        }
        Self {
            cutoffs,
            class_tables,
            by_precedence: by_precedence.into_boxed_slice(),
        }
    }

    /// The table index of a precedence value, `-1` for an adaptive class.
    #[inline]
    pub fn table_of(&self, precedence: i32) -> i32 {
        let precedence = precedence.max(0);
        match usize::try_from(precedence)
            .ok()
            .and_then(|index| self.by_precedence.get(index))
        {
            Some(&table) => table,
            None => self.class_tables[class_of(&self.cutoffs, precedence)],
        }
    }
}

/// The precedence class of a value: the number of cutoffs below it.
///
/// Class `i` covers `cutoffs[i-1] < p <= cutoffs[i]`, the first class covers
/// `p <= cutoffs[0]`, and the class after the last cutoff covers `p > last`.
fn class_of(cutoffs: &[i32], precedence: i32) -> usize {
    cutoffs
        .iter()
        .take_while(|&&cutoff| cutoff < precedence)
        .count()
}

/// The static parser tables of one parser unit.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct StaticParserTables {
    decisions: Vec<DecisionTables>,
    tables: Vec<StaticParserTable>,
}

impl StaticParserTables {
    /// The empty table set of a parser unit without static tables.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Builds a table set from per-decision slots and the table pool.
    pub fn new(decisions: Vec<DecisionTables>, tables: Vec<StaticParserTable>) -> Self {
        Self { decisions, tables }
    }

    /// `true` when the unit has no static table at all.
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    /// The table pool, for the table-emission tests and the ATN dump.
    pub fn tables(&self) -> &[StaticParserTable] {
        &self.tables
    }

    /// The decision slots.
    pub fn decisions(&self) -> &[DecisionTables] {
        &self.decisions
    }

    /// `true` when the decision selects its table by precedence.
    #[inline]
    pub fn is_precedence_dispatched(&self, decision: i32) -> bool {
        matches!(
            usize::try_from(decision)
                .ok()
                .and_then(|d| self.decisions.get(d)),
            Some(DecisionTables::Dispatch(_))
        )
    }

    /// The table of a decision at a precedence, or `None` when the decision is
    /// adaptive at that precedence.
    #[inline]
    pub fn table(&self, decision: i32, precedence: i32) -> Option<&StaticParserTable> {
        let index = match usize::try_from(decision)
            .ok()
            .and_then(|decision| self.decisions.get(decision))?
        {
            DecisionTables::None => return None,
            DecisionTables::Single(index) => *index,
            DecisionTables::Dispatch(dispatch) => {
                let index = dispatch.table_of(precedence);
                if index < 0 {
                    return None;
                }
                index as usize
            }
        };
        self.tables.get(index)
    }

    /// Encodes the tables as an `SDFA` word stream.
    pub fn to_words(&self) -> Vec<u32> {
        if self.tables.is_empty() {
            return Vec::new();
        }
        let mut words = vec![
            STATIC_PARSER_DFA_MAGIC,
            FORMAT_VERSION,
            0,
            self.tables.len() as u32,
            self.decisions.len() as u32,
        ];
        for table in &self.tables {
            table.encode_into(&mut words);
        }
        let dispatches = self
            .decisions
            .iter()
            .enumerate()
            .filter_map(|(decision, slot)| match slot {
                DecisionTables::Dispatch(dispatch) => Some((decision, dispatch)),
                _ => None,
            })
            .collect::<Vec<_>>();
        words.push(dispatches.len() as u32);
        for (decision, dispatch) in dispatches {
            words.push(decision as u32);
            words.push(dispatch.cutoffs.len() as u32);
            words.extend(dispatch.cutoffs.iter().map(|&cutoff| cutoff as u32));
            words.extend(dispatch.class_tables.iter().map(|&table| table as u32));
        }
        words[2] = words.len() as u32;
        words
    }

    /// Decodes an `SDFA` word stream.
    pub fn from_words(words: &[u32]) -> Result<Self, StaticParserDfaError> {
        if words.is_empty() {
            return Ok(Self::empty());
        }
        let mut reader = Reader::new(words);
        let magic = reader.next()?;
        if magic != STATIC_PARSER_DFA_MAGIC {
            return Err(StaticParserDfaError::BadMagic { found: magic });
        }
        let version = reader.next()?;
        if version != FORMAT_VERSION {
            return Err(StaticParserDfaError::UnsupportedVersion { found: version });
        }
        let declared = reader.next()? as usize;
        if declared != words.len() {
            return Err(StaticParserDfaError::LengthMismatch {
                declared,
                actual: words.len(),
            });
        }
        let num_tables = checked_count(reader.next()? as usize, 5, reader.remaining())?;
        let num_slots = checked_count(reader.next()? as usize, 1, reader.remaining())?;
        let mut decisions = vec![DecisionTables::None; num_slots];
        let mut tables = Vec::with_capacity(num_tables);
        for _ in 0..num_tables {
            let (decision, table) = StaticParserTable::decode(&mut reader)?;
            if decision >= 0 {
                let slot = decisions.get_mut(decision as usize).ok_or(
                    StaticParserDfaError::InvalidDecision {
                        decision: decision as usize,
                        num_slots,
                    },
                )?;
                *slot = DecisionTables::Single(tables.len());
            }
            tables.push(table);
        }
        let num_dispatches = reader.next()? as usize;
        for _ in 0..num_dispatches {
            let decision = reader.next()? as usize;
            let num_cutoffs = checked_count(reader.next()? as usize, 2, reader.remaining())?;
            if decisions
                .get(decision)
                .is_some_and(|slot| matches!(slot, DecisionTables::Dispatch(_)))
            {
                return Err(StaticParserDfaError::DuplicateDispatch { decision });
            }
            let mut cutoffs = Vec::with_capacity(num_cutoffs);
            for _ in 0..num_cutoffs {
                cutoffs.push(reader.next()? as i32);
            }
            if cutoffs.windows(2).any(|window| window[0] >= window[1]) {
                return Err(StaticParserDfaError::UnsortedCutoffs);
            }
            let mut class_tables = Vec::with_capacity(num_cutoffs + 1);
            for _ in 0..=num_cutoffs {
                let table = reader.next()? as i32;
                if table < -1 || (table >= 0 && table as usize >= num_tables) {
                    return Err(StaticParserDfaError::InvalidDispatchTable {
                        table_index: table,
                        num_tables,
                    });
                }
                class_tables.push(table);
            }
            let slot =
                decisions
                    .get_mut(decision)
                    .ok_or(StaticParserDfaError::InvalidDecision {
                        decision,
                        num_slots,
                    })?;
            *slot = DecisionTables::Dispatch(PrecedenceDispatch::new(cutoffs, class_tables));
        }
        if reader.position != words.len() {
            return Err(StaticParserDfaError::LengthMismatch {
                declared: words.len(),
                actual: reader.position,
            });
        }
        Ok(Self { decisions, tables })
    }
}

impl StaticParserTable {
    /// Builds a table from per-state edge ranges.
    ///
    /// `edges[state]` lists `(lo, hi, target)` inclusive token ranges; the
    /// constructor expands them into rows and deduplicates identical rows.
    pub fn from_state_edges(
        decision: i32,
        accepts: Vec<i32>,
        fallbacks: Vec<i32>,
        conflict_min_alts: Vec<i32>,
        continuations: Vec<ParserContinuation>,
        edges: &[Vec<(i32, i32, u32)>],
    ) -> Self {
        let num_states = accepts.len();
        let edge_width = edges
            .iter()
            .flatten()
            .map(|&(_, hi, _)| hi.saturating_add(2).max(1) as usize)
            .max()
            .unwrap_or(1)
            .max(1);
        assert!(
            edge_width <= MAX_TABLE_COLUMNS,
            "a table row must not be wider than {MAX_TABLE_COLUMNS} columns"
        );
        let mut state_rows = vec![-1_i32; num_states * edge_width];
        for (state, ranges) in edges.iter().enumerate() {
            let row = &mut state_rows[state * edge_width..(state + 1) * edge_width];
            for &(lo, hi, target) in ranges {
                for token in lo.max(-1)..=hi {
                    row[(token + 1) as usize] = target as i32;
                }
            }
        }
        let mut rows: Vec<i32> = Vec::new();
        let mut row_of = Vec::with_capacity(num_states);
        let mut distinct: HashMap<&[i32], u32> = HashMap::new();
        for state in 0..num_states {
            let key = &state_rows[state * edge_width..(state + 1) * edge_width];
            let index = match distinct.get(key) {
                Some(&index) => index,
                None => {
                    let index = rows.len() as u32 / edge_width as u32;
                    rows.extend_from_slice(key);
                    distinct.insert(key, index);
                    index
                }
            };
            row_of.push(index);
        }
        let continuation_of = Self::continuation_slots(&continuations, num_states);
        Self {
            decision,
            accepts,
            fallbacks,
            conflict_min_alts,
            continuations,
            continuation_of,
            guarded_alts: vec![0; num_states],
            guard_danger: Vec::new(),
            guard_pass: Vec::new(),
            edge_width,
            row_of,
            rows,
        }
    }

    /// Attaches the guard section: the alternative of every guarded state,
    /// plus the decision's guard danger and pass sets (sorted invoking state
    /// numbers).
    ///
    /// The caller sets `accepts[state] = GUARDED` for every state named here.
    #[must_use]
    pub fn with_guards(
        mut self,
        guarded_alts: Vec<i32>,
        guard_danger: Vec<i32>,
        guard_pass: Vec<i32>,
    ) -> Self {
        assert_eq!(
            guarded_alts.len(),
            self.accepts.len(),
            "one guarded alternative per state"
        );
        self.guarded_alts = guarded_alts;
        self.guard_danger = guard_danger;
        self.guard_danger.sort_unstable();
        self.guard_pass = guard_pass;
        self.guard_pass.sort_unstable();
        self
    }

    fn continuation_slots(continuations: &[ParserContinuation], num_states: usize) -> Vec<u32> {
        let mut slots = vec![EMPTY_CONTEXT; num_states];
        for (index, continuation) in continuations.iter().enumerate() {
            if let Some(slot) = slots.get_mut(continuation.state as usize) {
                *slot = index as u32;
            }
        }
        slots
    }

    /// The decision index of the table, `-1` for a precedence class table.
    pub fn decision(&self) -> i32 {
        self.decision
    }

    /// The number of states of the table.
    pub fn num_states(&self) -> usize {
        self.accepts.len()
    }

    /// The accept marker or predicted alternative of a state.
    #[inline]
    pub fn accept(&self, state: i32) -> i32 {
        self.accepts[state as usize]
    }

    /// The alternative a guarded state resolves to, if the state is guarded.
    #[inline]
    pub fn guarded_alt(&self, state: i32) -> Option<i32> {
        let alt = *self.guarded_alts.get(state as usize)?;
        if alt > 0 {
            Some(alt)
        } else {
            None
        }
    }

    /// `true` when the table carries at least one guarded state.
    pub fn has_guards(&self) -> bool {
        self.guarded_alts.iter().any(|&alt| alt > 0)
    }

    /// The guard step of one invoking state. `Some(true)` means the follow
    /// region of the state reaches a guard root, so the take defers.
    /// `Some(false)` means a mandatory token seals the frame off, so the take is
    /// resolved. `None` means the frame can pop out of its rule, so the stack
    /// walk continues upwards.
    #[inline]
    pub fn guard_step(&self, invoking_state: i32) -> Option<bool> {
        if self.guard_danger.binary_search(&invoking_state).is_ok() {
            return Some(true);
        }
        if self.guard_pass.binary_search(&invoking_state).is_err() {
            return Some(false);
        }
        None
    }

    /// The lowest alternative that already left the decision, if any.
    #[inline]
    pub fn fallback(&self, state: i32) -> Option<i32> {
        let fallback = self.fallbacks[state as usize];
        if fallback > 0 {
            Some(fallback)
        } else {
            None
        }
    }

    /// The lowest conflicting alternative of a conflict escape state.
    #[inline]
    pub fn conflict_min_alt(&self, state: i32) -> Option<i32> {
        let alt = self.conflict_min_alts[state as usize];
        if alt > 0 {
            Some(alt)
        } else {
            None
        }
    }

    /// The continuation of a frontier escape state and its slot.
    #[inline]
    pub fn continuation(&self, state: i32) -> Option<(usize, &ParserContinuation)> {
        let slot = *self.continuation_of.get(state as usize)?;
        if slot == EMPTY_CONTEXT {
            None
        } else {
            Some((slot as usize, &self.continuations[slot as usize]))
        }
    }

    /// The number of continuations, i.e. of cache slots per DFA.
    pub fn num_continuations(&self) -> usize {
        self.continuations.len()
    }

    /// The target state of a token edge, or `None` when the table has no edge.
    #[inline]
    pub fn edge(&self, state: i32, token: i32) -> Option<i32> {
        let column = token.wrapping_add(1) as usize;
        if column >= self.edge_width {
            return None;
        }
        let row = unsafe { *self.row_of.get_unchecked(state as usize) } as usize;
        let target = unsafe { *self.rows.get_unchecked(row * self.edge_width + column) };
        if target < 0 {
            None
        } else {
            Some(target)
        }
    }

    fn encode_into(&self, words: &mut Vec<u32>) {
        let num_states = self.accepts.len();
        words.push(self.decision as u32);
        words.push(num_states as u32);
        words.push(self.edge_width as u32);
        words.push((self.rows.len() / self.edge_width) as u32);
        let (offsets, triples) = self.encode_rows();
        words.push(triples.len() as u32);
        for &accept in &self.accepts {
            let accept = if accept == CONFLICT_ESCAPE {
                ESCAPE
            } else {
                accept
            };
            words.push(accept as u32);
        }
        words.extend(self.fallbacks.iter().map(|&fallback| fallback as u32));
        words.extend(self.row_of.iter().copied());
        words.extend(offsets.iter().map(|&offset| offset as u32));
        words.extend(triples.iter().map(|&triple| triple as u32));

        let conflicts = self
            .conflict_min_alts
            .iter()
            .enumerate()
            .filter(|(_, &alt)| alt > 0)
            .collect::<Vec<_>>();
        words.push(conflicts.len() as u32);
        for (state, &alt) in conflicts {
            words.push(state as u32);
            words.push(alt as u32);
        }

        words.push(self.continuations.len() as u32);
        for continuation in &self.continuations {
            words.push(continuation.state);
            words.push(continuation.contexts.len() as u32);
            for node in &continuation.contexts {
                match node {
                    ContinuationContextNode::Singleton {
                        parent,
                        return_state,
                    } => {
                        words.push(1);
                        words.push(*parent);
                        words.push(*return_state as u32);
                    }
                    ContinuationContextNode::Array {
                        parents,
                        return_states,
                    } => {
                        words.push(parents.len() as u32);
                        for (&parent, &return_state) in parents.iter().zip(return_states) {
                            words.push(parent);
                            words.push(return_state as u32);
                        }
                    }
                }
            }
            words.push(continuation.configs.len() as u32);
            for config in &continuation.configs {
                words.push(config.state as u32);
                words.push(config.alt as u32);
                words.push(config.context);
                words.push(config.outer_depth as u32);
            }
        }

        let guarded = self
            .guarded_alts
            .iter()
            .enumerate()
            .filter(|(_, &alt)| alt > 0)
            .collect::<Vec<_>>();
        words.push(guarded.len() as u32);
        for (state, &alt) in guarded {
            words.push(state as u32);
            words.push(alt as u32);
        }
        words.push(self.guard_danger.len() as u32);
        words.extend(self.guard_danger.iter().map(|&state| state as u32));
        words.push(self.guard_pass.len() as u32);
        words.extend(self.guard_pass.iter().map(|&state| state as u32));
    }

    /// The per-row range lists (`offsets` in ints, not in ranges).
    fn encode_rows(&self) -> (Vec<usize>, Vec<i32>) {
        let num_rows = self.rows.len() / self.edge_width;
        let mut offsets = Vec::with_capacity(num_rows + 1);
        let mut triples = Vec::with_capacity(num_rows);
        for row_index in 0..num_rows {
            offsets.push(triples.len());
            let row = &self.rows[row_index * self.edge_width..(row_index + 1) * self.edge_width];
            let mut column = 0;
            while column < self.edge_width {
                let target = row[column];
                if target < 0 {
                    column += 1;
                    continue;
                }
                let start = column;
                while column + 1 < self.edge_width && row[column + 1] == target {
                    column += 1;
                }
                triples.push(start as i32 - 1);
                triples.push(column as i32 - 1);
                triples.push(target);
                column += 1;
            }
        }
        offsets.push(triples.len());
        (offsets, triples)
    }

    #[allow(clippy::too_many_lines)]
    fn decode(reader: &mut Reader<'_>) -> Result<(i32, Self), StaticParserDfaError> {
        let decision = reader.next()? as i32;
        let num_states = reader.next()? as usize;
        let edge_width = reader.next()? as usize;
        let num_rows = reader.next()? as usize;
        let num_row_edge_ints = reader.next()? as usize;
        if !num_row_edge_ints.is_multiple_of(3) {
            return Err(StaticParserDfaError::InvalidEdgeOffsets);
        }
        let too_large = num_states == 0
            || edge_width == 0
            || num_states > MAX_TABLE_STATES
            || num_rows > MAX_TABLE_ROWS
            || edge_width > MAX_TABLE_COLUMNS
            || num_rows.saturating_mul(edge_width) > MAX_TABLE_CELLS;
        if too_large {
            return Err(StaticParserDfaError::InvalidTableSize {
                num_states,
                num_rows,
                edge_width,
            });
        }

        checked_count(num_states, 3, reader.remaining())?;
        let mut accepts = Vec::with_capacity(num_states);
        for _ in 0..num_states {
            accepts.push(reader.next()? as i32);
        }
        let mut fallbacks = Vec::with_capacity(num_states);
        for _ in 0..num_states {
            fallbacks.push(reader.next()? as i32);
        }
        let mut row_of = Vec::with_capacity(num_states);
        for _ in 0..num_states {
            let row = reader.next()? as i32;
            if row < 0 || row as usize >= num_rows {
                return Err(StaticParserDfaError::InvalidRowIndex { row, num_rows });
            }
            row_of.push(row as u32);
        }
        let mut offsets = Vec::with_capacity(num_rows + 1);
        for _ in 0..=num_rows {
            offsets.push(reader.next()? as usize);
        }
        if offsets.first().copied() != Some(0)
            || offsets.last().copied() != Some(num_row_edge_ints)
            || offsets
                .windows(2)
                .any(|window| window[0] > window[1] || !window[0].is_multiple_of(3))
        {
            return Err(StaticParserDfaError::InvalidEdgeOffsets);
        }
        let mut triples = Vec::with_capacity(num_row_edge_ints);
        for _ in 0..num_row_edge_ints {
            triples.push(reader.next()? as i32);
        }
        let mut rows = vec![-1_i32; num_rows.saturating_mul(edge_width)];
        for row_index in 0..num_rows {
            let row = &mut rows[row_index * edge_width..(row_index + 1) * edge_width];
            for range in offsets[row_index] / 3..offsets[row_index + 1] / 3 {
                let lo = triples[range * 3];
                let hi = triples[range * 3 + 1];
                let target = triples[range * 3 + 2];
                if lo < -1 || hi < lo || hi >= edge_width as i32 - 1 {
                    return Err(StaticParserDfaError::InvalidEdgeBounds { lo, hi, edge_width });
                }
                if target < 0 || target as usize >= num_states {
                    return Err(StaticParserDfaError::InvalidEdgeTarget { target, num_states });
                }
                for token in lo..=hi {
                    row[(token + 1) as usize] = target;
                }
            }
        }

        let mut conflict_min_alts = vec![0_i32; num_states];
        let num_conflicts = checked_count(reader.next()? as usize, 2, reader.remaining())?;
        for _ in 0..num_conflicts {
            let state = reader.next()? as usize;
            let alt = reader.next()? as i32;
            if state >= num_states {
                return Err(StaticParserDfaError::InvalidStateIndex { state, num_states });
            }
            if alt <= 0 {
                return Err(StaticParserDfaError::InvalidConflictAlt { alt });
            }
            accepts[state] = CONFLICT_ESCAPE;
            conflict_min_alts[state] = alt;
        }

        let mut continuations = Vec::new();
        let num_continuations = checked_count(reader.next()? as usize, 3, reader.remaining())?;
        for _ in 0..num_continuations {
            let state = reader.next()?;
            if state as usize >= num_states {
                return Err(StaticParserDfaError::InvalidStateIndex {
                    state: state as usize,
                    num_states,
                });
            }
            let num_nodes = checked_count(reader.next()? as usize, 1, reader.remaining())?;
            let mut contexts = Vec::with_capacity(num_nodes);
            for node in 0..num_nodes {
                let arity = checked_count(reader.next()? as usize, 2, reader.remaining())?;
                if arity == 0 {
                    return Err(StaticParserDfaError::EmptyContinuationContext { index: node });
                }
                let mut parents = Vec::with_capacity(arity);
                let mut return_states = Vec::with_capacity(arity);
                for _ in 0..arity {
                    let parent = reader.next()?;
                    if parent != EMPTY_CONTEXT && parent as usize >= node {
                        return Err(StaticParserDfaError::InvalidContinuationContext {
                            index: parent,
                            decoded: node,
                        });
                    }
                    parents.push(parent);
                    let return_state = reader.next()? as i32;
                    if return_state < -1 {
                        return Err(StaticParserDfaError::InvalidReturnState { return_state });
                    }
                    return_states.push(return_state);
                }
                if arity == 1 {
                    contexts.push(ContinuationContextNode::Singleton {
                        parent: parents[0],
                        return_state: return_states[0],
                    });
                } else {
                    contexts.push(ContinuationContextNode::Array {
                        parents,
                        return_states,
                    });
                }
            }
            let num_configs = checked_count(reader.next()? as usize, 4, reader.remaining())?;
            let mut configs = Vec::with_capacity(num_configs);
            for _ in 0..num_configs {
                let config_state = reader.next()? as i32;
                let alt = reader.next()? as i32;
                let context = reader.next()?;
                if context != EMPTY_CONTEXT && context as usize >= num_nodes {
                    return Err(StaticParserDfaError::InvalidContinuationContext {
                        index: context,
                        decoded: num_nodes,
                    });
                }
                let outer_depth = reader.next()? as i32;
                if config_state < 0 || alt <= 0 || outer_depth < 0 {
                    return Err(StaticParserDfaError::InvalidContinuationConfig {
                        state: config_state,
                        alt,
                        outer_depth,
                    });
                }
                configs.push(ContinuationConfigEntry {
                    state: config_state,
                    alt,
                    context,
                    outer_depth,
                });
            }
            if continuations
                .iter()
                .any(|existing: &ParserContinuation| existing.state == state)
            {
                return Err(StaticParserDfaError::DuplicateContinuation { state });
            }
            continuations.push(ParserContinuation {
                state,
                contexts,
                configs,
            });
        }

        let mut guarded_alts = vec![0_i32; num_states];
        let num_guarded = checked_count(reader.next()? as usize, 2, reader.remaining())?;
        for _ in 0..num_guarded {
            let state = reader.next()? as usize;
            let alt = reader.next()? as i32;
            if state >= num_states {
                return Err(StaticParserDfaError::InvalidStateIndex { state, num_states });
            }
            if alt <= 0 {
                return Err(StaticParserDfaError::InvalidConflictAlt { alt });
            }
            if guarded_alts[state] != 0 {
                return Err(StaticParserDfaError::DuplicateGuard { state });
            }
            accepts[state] = GUARDED;
            guarded_alts[state] = alt;
        }
        let num_danger = checked_count(reader.next()? as usize, 1, reader.remaining())?;
        let mut guard_danger = Vec::with_capacity(num_danger);
        for _ in 0..num_danger {
            let state = reader.next()? as i32;
            if state < 0 {
                return Err(StaticParserDfaError::InvalidGuardState { state });
            }
            guard_danger.push(state);
        }
        guard_danger.sort_unstable();
        let num_pass = checked_count(reader.next()? as usize, 1, reader.remaining())?;
        let mut guard_pass = Vec::with_capacity(num_pass);
        for _ in 0..num_pass {
            let state = reader.next()? as i32;
            if state < 0 {
                return Err(StaticParserDfaError::InvalidGuardState { state });
            }
            guard_pass.push(state);
        }
        guard_pass.sort_unstable();

        let continuation_of = Self::continuation_slots(&continuations, num_states);
        Ok((
            decision,
            Self {
                decision,
                accepts,
                fallbacks,
                conflict_min_alts,
                continuations,
                continuation_of,
                guarded_alts,
                guard_danger,
                guard_pass,
                edge_width,
                row_of,
                rows,
            },
        ))
    }
}

impl StaticParserTables {
    /// Checks the tables against the ATN they are loaded into.
    ///
    /// The decoders can only check the section against itself; the ATN-relative
    /// indexes (decision slots, config states, continuation return states) are
    /// only meaningful once the ATN is known, and a wrong index would panic
    /// when a continuation is materialized.
    pub fn validate_for_atn(&self, atn: &crate::atn::ATN) -> Result<(), StaticParserDfaError> {
        let num_states = atn.states_count();
        let num_decisions = atn.decision_to_state.len();
        let check_state = |state: i64, what: &'static str| {
            if state < 0 || state as usize >= num_states {
                return Err(StaticParserDfaError::OutsideAtn {
                    what,
                    index: state,
                    count: num_states,
                });
            }
            Ok(())
        };
        if self.decisions.len() > num_decisions {
            return Err(StaticParserDfaError::OutsideAtn {
                what: "decision slot",
                index: num_decisions as i64,
                count: num_decisions,
            });
        }
        for table in &self.tables {
            if table.decision as usize >= num_decisions && table.decision >= 0 {
                return Err(StaticParserDfaError::OutsideAtn {
                    what: "table decision",
                    index: i64::from(table.decision),
                    count: num_decisions,
                });
            }
            for &state in table.guard_danger.iter().chain(&table.guard_pass) {
                check_state(i64::from(state), "guard invoking state")?;
            }
            for continuation in &table.continuations {
                for node in &continuation.contexts {
                    let return_states: &[i32] = match node {
                        ContinuationContextNode::Singleton { return_state, .. } => {
                            std::slice::from_ref(return_state)
                        }
                        ContinuationContextNode::Array { return_states, .. } => return_states,
                    };
                    for &return_state in return_states {
                        if return_state >= 0 {
                            check_state(i64::from(return_state), "return state")?;
                        }
                    }
                }
                for config in &continuation.configs {
                    check_state(i64::from(config.state), "continuation config state")?;
                }
            }
        }
        Ok(())
    }
}

/// Appends an `SDFA` section to a packed ATN word stream.
///
/// The packed ATN header carries the total stream length, so appending also
/// updates that word. An empty section leaves the stream unchanged.
pub fn append_sdfa_section(packed_words: &[u32], sdfa_words: &[u32]) -> Vec<u32> {
    if sdfa_words.is_empty() {
        return packed_words.to_vec();
    }
    let mut words = Vec::with_capacity(packed_words.len() + sdfa_words.len());
    words.extend_from_slice(packed_words);
    words.extend_from_slice(sdfa_words);
    words[PACKED_ATN_TOTAL_LEN] = words.len() as u32;
    words
}

/// A cursor over an `SDFA` word stream.
struct Reader<'a> {
    words: &'a [u32],
    position: usize,
}

impl<'a> Reader<'a> {
    const fn new(words: &'a [u32]) -> Self {
        Self { words, position: 0 }
    }

    /// The number of words left in the stream.
    fn remaining(&self) -> usize {
        self.words.len().saturating_sub(self.position)
    }

    #[inline]
    fn next(&mut self) -> Result<u32, StaticParserDfaError> {
        let word = *self
            .words
            .get(self.position)
            .ok_or(StaticParserDfaError::TruncatedStream)?;
        self.position += 1;
        Ok(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dispatch(cutoffs: &[i32], classes: &[i32]) -> PrecedenceDispatch {
        PrecedenceDispatch::new(cutoffs.to_vec(), classes.to_vec())
    }

    #[test]
    fn precedence_dispatch_covers_every_class() {
        // `class_tables[i]` covers `cutoffs[i-1] < p <= cutoffs[i]`, and the
        // class after the last cutoff covers every larger precedence.
        let dispatch = dispatch(&[2, 5], &[10, 11, 12]);
        assert_eq!(dispatch.table_of(-1), 10);
        assert_eq!(dispatch.table_of(0), 10);
        assert_eq!(dispatch.table_of(2), 10);
        assert_eq!(dispatch.table_of(3), 11);
        assert_eq!(dispatch.table_of(5), 11);
        assert_eq!(dispatch.table_of(6), 12);
        assert_eq!(dispatch.table_of(1_000_000), 12);
        assert_eq!(dispatch.table_of(i32::MAX), 12);
    }

    #[test]
    fn precedence_dispatch_without_cutoffs_is_one_class() {
        let dispatch = dispatch(&[], &[7]);
        assert_eq!(dispatch.table_of(0), 7);
        assert_eq!(dispatch.table_of(i32::MAX), 7);
    }

    fn table(
        decision: i32,
        accepts: Vec<i32>,
        edges: &[Vec<(i32, i32, u32)>],
    ) -> StaticParserTable {
        let states = accepts.len();
        StaticParserTable::from_state_edges(
            decision,
            accepts,
            vec![0; states],
            vec![0; states],
            Vec::new(),
            edges,
        )
    }

    #[test]
    fn rows_are_deduplicated_and_decoded() {
        let built = table(
            0,
            vec![0, 1, 1],
            &[
                vec![(1, 2, 1), (4, 4, 2)],
                vec![(1, 2, 1), (4, 4, 2)],
                vec![(-1, -1, 0)],
            ],
        );
        assert_eq!(built.edge(0, 1), Some(1));
        assert_eq!(built.edge(0, 3), None);
        assert_eq!(built.edge(0, -1), None);
        assert_eq!(built.edge(2, -1), Some(0));

        let words = StaticParserTables::new(vec![DecisionTables::Single(0)], vec![built.clone()])
            .to_words();
        let decoded = StaticParserTables::from_words(&words).expect("the words decode");
        assert_eq!(decoded.table(0, 0), Some(&built));
        assert_eq!(decoded.to_words(), words);
    }

    #[test]
    fn guarded_takes_round_trip_through_the_guard_section() {
        let built = StaticParserTable::from_state_edges(
            0,
            vec![0, GUARDED],
            vec![0, 0],
            vec![0, 0],
            Vec::new(),
            &[vec![(1, 1, 1)], Vec::new()],
        )
        .with_guards(vec![0, 1], vec![4, 9], vec![2, 7]);
        assert!(built.has_guards());
        // `4` reaches a guard root (defer), `9` too; `2` is a pass state, so
        // the walk continues past it; `5` is sealed (resolve the take).
        assert_eq!(built.guard_step(4), Some(true));
        assert_eq!(built.guard_step(2), None);
        assert_eq!(built.guard_step(5), Some(false));
        assert_eq!(built.guarded_alt(1), Some(1));
        assert_eq!(built.guarded_alt(0), None);
        let words = StaticParserTables::new(vec![DecisionTables::Single(0)], vec![built.clone()])
            .to_words();
        let decoded = StaticParserTables::from_words(&words).expect("the words decode");
        let table = decoded.table(0, 0).expect("the table");
        assert_eq!(table, &built);
        assert_eq!(table.accept(1), GUARDED);
        assert_eq!(table.guard_step(9), Some(true));
        assert_eq!(table.guard_step(7), None);
        assert_eq!(table.guard_step(9), Some(true));
        assert_eq!(decoded.to_words(), words);
    }

    #[test]
    fn conflict_escapes_round_trip_through_the_min_alt_pairs() {
        let built = StaticParserTable::from_state_edges(
            0,
            vec![CONFLICT_ESCAPE, 2],
            vec![1, 0],
            vec![1, 0],
            Vec::new(),
            &[vec![(1, 1, 1)], Vec::new()],
        );
        let words =
            StaticParserTables::new(vec![DecisionTables::Single(0)], vec![built]).to_words();
        let decoded = StaticParserTables::from_words(&words).expect("the words decode");
        let table = decoded.table(0, 0).expect("decision 0 has a table");
        assert_eq!(table.accept(0), CONFLICT_ESCAPE);
        assert_eq!(table.conflict_min_alt(0), Some(1));
        assert_eq!(table.accept(1), 2);
        assert_eq!(table.conflict_min_alt(1), None);
    }

    #[test]
    fn continuations_round_trip_and_validate_their_context_dag() {
        let continuation = ParserContinuation {
            state: 1,
            contexts: vec![
                ContinuationContextNode::Singleton {
                    parent: EMPTY_CONTEXT,
                    return_state: 7,
                },
                ContinuationContextNode::Array {
                    parents: vec![0, EMPTY_CONTEXT],
                    return_states: vec![8, -1],
                },
            ],
            configs: vec![
                ContinuationConfigEntry {
                    state: 3,
                    alt: 1,
                    context: 0,
                    outer_depth: 0,
                },
                ContinuationConfigEntry {
                    state: 4,
                    alt: 2,
                    context: 1,
                    outer_depth: 1,
                },
            ],
        };
        let built = StaticParserTable::from_state_edges(
            0,
            vec![0, ESCAPE],
            vec![0, 0],
            vec![0, 0],
            vec![continuation],
            &[vec![(1, 1, 1)], Vec::new()],
        );
        let words = StaticParserTables::new(vec![DecisionTables::Single(0)], vec![built.clone()])
            .to_words();
        let decoded = StaticParserTables::from_words(&words).expect("the words decode");
        let table = decoded.table(0, 0).expect("decision 0 has a table");
        let (slot, decoded_continuation) = table.continuation(1).expect("the escape has one");
        assert_eq!(slot, 0);
        assert_eq!(decoded_continuation.state, 1);
        assert_eq!(decoded_continuation.contexts.len(), 2);
        assert_eq!(decoded_continuation.configs.len(), 2);
        assert_eq!(decoded.to_words(), words);
    }

    #[test]
    fn malformed_streams_are_rejected() {
        let empty = StaticParserTables::from_words(&[]).expect("an empty stream is empty tables");
        assert!(empty.is_empty());

        assert_eq!(
            StaticParserTables::from_words(&[0, FORMAT_VERSION, 0, 0, 0]),
            Err(StaticParserDfaError::BadMagic { found: 0 })
        );
        assert_eq!(
            StaticParserTables::from_words(&[STATIC_PARSER_DFA_MAGIC, 99, 5, 0, 0]),
            Err(StaticParserDfaError::UnsupportedVersion { found: 99 })
        );
        assert_eq!(
            StaticParserTables::from_words(&[STATIC_PARSER_DFA_MAGIC, FORMAT_VERSION, 9, 0, 0]),
            Err(StaticParserDfaError::LengthMismatch {
                declared: 9,
                actual: 5
            })
        );
        // A continuation whose parent is not an ancestor is a cycle for the
        // replay, so the decoder rejects it.
        let built = StaticParserTable::from_state_edges(
            0,
            vec![ESCAPE],
            vec![0],
            vec![0],
            vec![ParserContinuation {
                state: 0,
                contexts: vec![ContinuationContextNode::Singleton {
                    parent: 0,
                    return_state: 3,
                }],
                configs: Vec::new(),
            }],
            &[Vec::new()],
        );
        let words =
            StaticParserTables::new(vec![DecisionTables::Single(0)], vec![built]).to_words();
        assert!(
            matches!(
                StaticParserTables::from_words(&words),
                Err(StaticParserDfaError::InvalidContinuationContext { .. })
            ),
            "a self-referencing context node is a cycle"
        );
    }
}
