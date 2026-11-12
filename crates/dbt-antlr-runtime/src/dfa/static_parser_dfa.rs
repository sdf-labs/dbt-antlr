// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Static parser SLL prediction tables: a zero-copy view over their packed
//! table image.
//!
//! A table is a precomputed SLL DFA of one prediction decision: rows of token
//! edges over a deduplicated state graph, the terminal outcome of every state
//! where the walk stops, and the fallback of every state that a lookahead can
//! leave. States the construction could not decide escape and the runtime
//! hands the decision back to the adaptive simulation; states where the
//! construction found an SLL conflict it trusts are conflict escapes that
//! carry the lowest conflicting alternative, so the runtime can skip Stage 1
//! SLL entirely.
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
//!   table also stores the lowest conflicting alternative and the walker
//!   resumes at Stage 2 directly.
//! - A frontier [`ESCAPE`] state stopped because the construction ran out of
//!   budget, *not* because SLL conflicted. SLL must keep scanning from the
//!   frontier, so the state carries a continuation: the closed,
//!   input-independent config set the construction had reached. The runtime
//!   materializes that config set once per escape state into the dynamic DFA
//!   and resumes the SLL simulation at `la(k+1)`, which removes the
//!   "double-walk" penalty (k static plus k dynamic steps) of restarting from
//!   the decision start.
//!
//! # Image container
//!
//! Generated parsers embed one container as a `static [u32; N]` beside the
//! packed parser ATN, and [`StaticParserTables::from_image`] overlays it in
//! place. All offsets of the container are in words:
//!
//! - word 0: [`STATIC_PARSER_DFA_MAGIC`].
//! - word 1: [`FORMAT_VERSION`].
//! - word 2: the word count of the whole container.
//! - word 3: the decision slot count.
//! - word 4: the table count.
//! - word 5: the precedence dispatch count.
//! - word 6: the word count of the dispatch data.
//! - the decision slots, one word per decision: `NO_TABLE` for an adaptive
//!   decision, a table index, or `DISPATCH_BIT | d` for the precedence
//!   dispatch `d`.
//! - the table directory, two words per table: the `(offset, len)` span of
//!   the table image in the table data, back to back and covering it.
//! - the precedence dispatches, five words each (`DispatchRecord`).
//! - the dispatch data: the cutoffs, class tables, and dense precedence
//!   lookups of every dispatch, as `i32` words.
//! - the table data: the table images.
//!
//! # Table image
//!
//! A table image starts with its `Header` and continues with the sections in
//! the order of the header fields. Section offsets are byte offsets from the
//! start of the image and are multiples of 4. A `u16` section packs two
//! entries per word, the entry with the lower index in the low half of the
//! word, and pads an odd entry count with one zero entry.
//!
//! - `outcomes`: `num_states` `i32` entries: `0` for a state that continues
//!   the walk, the predicted alternative of an accept state, or a tagged
//!   terminal (escape, continuation slot, conflict escape with its lowest
//!   conflicting alternative, or guarded state with its [`Guard`]).
//! - `fallbacks`: `fallbacks_len` `i32` entries, either none or one per state:
//!   the lowest alternative that already left the decision, `0` for none.
//! - `row_of`: `num_states` `u16` entries: the row of the state, `NO_ROW` for
//!   a state without edges or one where the walk stops. The row of the start
//!   state is row 0 or `NO_ROW`.
//! - `rows`: `num_rows * row_width` `u16` entries: the distinct rows. Column
//!   `c` is the token `min_token + c`. An entry is `NO_EDGE` or the target
//!   state, with [`TERMINAL_BIT`] set when the target's outcome is not `0`.
//! - `guard_danger`: the danger sets of all guard groups, back to back. Each
//!   set is a sorted list of distinct invoking states.
//! - `guard_pass`: the sorted, distinct invoking states that the guard walk of
//!   every group continues through.
//! - `guard_groups`: `0` or `G + 1` `u32` entries for `G` guard groups. Group
//!   `g` is the span `guard_groups[g]..guard_groups[g + 1]` of `guard_danger`.
//! - `continuations`: `ContinuationRecord`s, one per frontier escape state.
//! - `continuation_ctxs`: the context DAG nodes of all continuations, as
//!   spans of `continuation_ctx_edges`.
//! - `continuation_ctx_edges`: the `(parent, return state)` pairs of all
//!   context nodes.
//! - `continuation_configs`: the configs of all continuations.
//!
//! Loading validates the container, the sections of every table, and every
//! entry the walker dereferences unchecked.

use std::fmt::{self, Debug, Display, Formatter};
use std::mem::{align_of, size_of};

/// First word of an image container: ASCII "SDFA".
pub const STATIC_PARSER_DFA_MAGIC: u32 = 0x5344_4641;

/// Version of the table image format.
///
/// Version 4 adds the defer alternative and the guard group of a guarded
/// state. Version 3 is the packed zero-copy image. Version 2 was a word stream
/// that decoded into owned tables.
pub const FORMAT_VERSION: u32 = 4;

/// Accept marker of a state whose decision must be recomputed by the adaptive
/// simulation: the static construction stopped here.
pub const ESCAPE: i32 = -1;

/// Accept marker of a state where the static construction found an SLL conflict
/// it trusts. The walker resumes at Stage 2 (full context) instead of Stage 1.
pub const CONFLICT_ESCAPE: i32 = -2;

/// Accept marker of a state whose alternative is decided by a runtime stack
/// guard: the walk resolves to the guarded alternative unless the real parse
/// stack can reach one of the guard roots of the state. If the stack can
/// reach a root, the walk returns the defer alternative of the state. A defer
/// alternative of [`crate::atn::INVALID_ALT`] hands the decision to the
/// adaptive simulation.
pub const GUARDED: i32 = -3;

/// First virtual alternative of a decision.
///
/// A state whose live alternatives all start with the same shared rule accepts
/// with the virtual alternative `VIRTUAL_ALT_BASE + group` of that
/// shared-descent group instead of escaping. A generated block with an arm for
/// the group parses the shared rule once and picks the member alternative from
/// the token after it; a block without one defers to the adaptive simulation.
/// No decision has this many alternatives.
pub const VIRTUAL_ALT_BASE: i32 = 65_536;

/// The empty prediction context in a continuation context DAG.
pub const EMPTY_CONTEXT: u32 = u32::MAX;

/// Row entry of a token without an edge.
pub(crate) const NO_EDGE: u16 = 0xFFFF;
/// Row entry bit of an edge into a state where the walk stops.
pub(crate) const TERMINAL_BIT: u16 = 0x8000;
/// Row entry bits of the target state.
pub(crate) const STATE_MASK: u16 = 0x7FFF;
/// `row_of` entry of a state without edges.
const NO_ROW: u16 = 0xFFFF;

/// Decision slot of an adaptive decision.
const NO_TABLE: u32 = u32::MAX;
/// Decision slot bit of a precedence dispatch.
const DISPATCH_BIT: u32 = 0x8000_0000;

/// Tag bits of a terminal outcome.
const OUTCOME_TAG: u32 = 0xE000_0000;
/// Payload bits of a terminal outcome.
const OUTCOME_PAYLOAD: u32 = 0x1FFF_FFFF;
/// Outcome tag of a frontier escape with a continuation slot.
const OUTCOME_CONTINUATION: u32 = 0x8000_0000;
/// Outcome tag of a conflict escape with its lowest conflicting alternative.
const OUTCOME_CONFLICT: u32 = 0xA000_0000;
/// Outcome tag of a guarded state with its [`Guard`].
const OUTCOME_GUARDED: u32 = 0xC000_0000;
/// Bits of the guarded alternative and of the defer alternative in the payload
/// of a guarded outcome.
const GUARD_ALT_BITS: u32 = 8;
/// Largest guarded or defer alternative.
pub const MAX_GUARD_ALT: i32 = (1 << GUARD_ALT_BITS) - 1;
/// Largest number of guard groups of one table.
pub const MAX_GUARD_GROUPS: usize = 1 << (29 - 2 * GUARD_ALT_BITS);
/// Outcome of a plain escape.
const OUTCOME_ESCAPE: i32 = -1;

/// Cap on the states of one table: a row entry holds a 15-bit state.
const MAX_TABLE_STATES: usize = STATE_MASK as usize;
/// Cap on the token columns of one table.
const MAX_TABLE_COLUMNS: usize = 1 << 16;
/// Cap on the row cells of one table (32 MiB of `u16`).
const MAX_TABLE_CELLS: usize = 1 << 24;
/// Largest precedence value a dispatch resolves through its dense table.
#[cfg(feature = "static-dfa-build")]
const MAX_DENSE_PRECEDENCE: i32 = 1024;

/// Words of the container preamble.
const PREAMBLE_WORDS: usize = 7;
/// Words of a table directory entry.
const TABLE_DIR_WORDS: usize = 2;
/// Words of a precedence dispatch record.
const DISPATCH_WORDS: usize = size_of::<DispatchRecord>() / 4;

/// Failure while loading a static parser table image.
///
/// The tables are a compile-time product that generated code carries as
/// constants, so these are format errors of the image itself, not conditions
/// a correct build can produce.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticParserDfaError {
    /// The image ends before the data its preamble or header declares.
    TruncatedImage {
        /// What the image is too short for.
        what: &'static str,
    },
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
    /// A declared word count does not match the image.
    LengthMismatch {
        /// What declares the length.
        what: &'static str,
        /// The declared word count.
        declared: usize,
        /// The actual word count.
        actual: usize,
    },
    /// A section is unaligned or leaves the table image.
    OutOfBounds {
        /// The table index.
        table: usize,
        /// The section.
        what: &'static str,
    },
    /// The declared table shape is empty or too large.
    InvalidTableSize {
        /// The table index.
        table: usize,
        /// The declared number of states.
        num_states: usize,
        /// The declared number of rows.
        num_rows: usize,
        /// The declared number of token columns.
        row_width: usize,
    },
    /// An entry points outside the table it addresses.
    InvalidReference {
        /// The table index, or the decision or dispatch index for their
        /// entries.
        table: usize,
        /// What the entry addresses.
        what: &'static str,
        /// The value of the entry.
        value: i64,
        /// The number of valid entries.
        count: usize,
    },
    /// A table entry breaks an invariant of the format.
    Malformed {
        /// The table index, or the decision or dispatch index for their
        /// entries.
        table: usize,
        /// What is malformed.
        what: &'static str,
    },
    /// A continuation context node refers to a node that is not its ancestor.
    InvalidContinuationContext {
        /// The parent index found.
        index: u32,
        /// The number of nodes before it.
        decoded: usize,
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
}

impl Display for StaticParserDfaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedImage { what } => {
                write!(f, "static parser DFA image too short for {what}")
            }
            Self::BadMagic { found } => write!(
                f,
                "bad static parser DFA magic 0x{found:08x}; expected \
                 0x{STATIC_PARSER_DFA_MAGIC:08x}"
            ),
            Self::UnsupportedVersion { found } => write!(
                f,
                "unsupported static parser DFA format version {found}; expected \
                 {FORMAT_VERSION}"
            ),
            Self::LengthMismatch {
                what,
                declared,
                actual,
            } => write!(f, "{what} declares {declared} words but holds {actual}"),
            Self::OutOfBounds { table, what } => {
                write!(f, "table {table}: section {what} is out of bounds")
            }
            Self::InvalidTableSize {
                table,
                num_states,
                num_rows,
                row_width,
            } => write!(
                f,
                "table {table}: invalid shape of {num_states} states, {num_rows} rows, \
                 {row_width} columns"
            ),
            Self::InvalidReference {
                table,
                what,
                value,
                count,
            } => write!(f, "table {table}: {what} {value} outside 0..{count}"),
            Self::Malformed { table, what } => write!(f, "table {table}: {what} is malformed"),
            Self::InvalidContinuationContext { index, decoded } => write!(
                f,
                "continuation context parent {index} is not one of the {decoded} preceding nodes"
            ),
            Self::OutsideAtn { what, index, count } => {
                write!(f, "{what} {index} out of bounds for {count} ATN entries")
            }
        }
    }
}

impl std::error::Error for StaticParserDfaError {}

/// The runtime stack guard of a guarded state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Guard {
    /// The alternative of the state when the guard does not defer: the take
    /// of an optional postfix, or the iterate of a precedence loop.
    pub alt: i32,
    /// The alternative of the state when the guard defers: the skip or the
    /// exit, or [`crate::atn::INVALID_ALT`] to hand the decision to the
    /// adaptive simulation.
    pub defer_alt: i32,
    /// The guard group that holds the danger set of the state.
    pub group: u32,
}

/// The terminal outcome of a table state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Outcome {
    /// The walk continues.
    Continue,
    /// The state predicts an alternative.
    Alt(i32),
    /// The walk escapes to the adaptive simulation, resuming from the
    /// continuation slot when the state has one.
    Escape(Option<u32>),
    /// A trusted SLL conflict with its lowest conflicting alternative.
    Conflict(i32),
    /// A state that a runtime stack guard decides.
    Guarded(Guard),
}

impl Outcome {
    /// Decodes an `outcomes` entry.
    #[inline]
    pub(crate) fn decode(raw: i32) -> Option<Self> {
        if raw >= 0 {
            return Some(if raw == 0 {
                Self::Continue
            } else {
                Self::Alt(raw)
            });
        }
        if raw == OUTCOME_ESCAPE {
            return Some(Self::Escape(None));
        }
        let raw = raw as u32;
        let payload = raw & OUTCOME_PAYLOAD;
        match raw & OUTCOME_TAG {
            OUTCOME_CONTINUATION => Some(Self::Escape(Some(payload))),
            OUTCOME_CONFLICT if payload > 0 => Some(Self::Conflict(payload as i32)),
            OUTCOME_GUARDED => {
                let alt = (payload & MAX_GUARD_ALT as u32) as i32;
                let defer_alt = ((payload >> GUARD_ALT_BITS) & MAX_GUARD_ALT as u32) as i32;
                let group = payload >> (2 * GUARD_ALT_BITS);
                (alt > 0 && defer_alt != alt).then_some(Self::Guarded(Guard {
                    alt,
                    defer_alt,
                    group,
                }))
            }
            _ => None,
        }
    }

    /// The `outcomes` entry of the outcome.
    #[cfg(feature = "static-dfa-build")]
    fn encode(self) -> i32 {
        match self {
            Self::Continue => 0,
            Self::Alt(alt) => alt,
            Self::Escape(None) => OUTCOME_ESCAPE,
            Self::Escape(Some(slot)) => (OUTCOME_CONTINUATION | slot) as i32,
            Self::Conflict(alt) => (OUTCOME_CONFLICT | alt as u32) as i32,
            Self::Guarded(guard) => {
                (OUTCOME_GUARDED
                    | guard.alt as u32
                    | (guard.defer_alt as u32) << GUARD_ALT_BITS
                    | guard.group << (2 * GUARD_ALT_BITS)) as i32
            }
        }
    }

    /// The accept marker of the outcome: `0`, the alternative, or one of
    /// [`ESCAPE`], [`CONFLICT_ESCAPE`], and [`GUARDED`].
    fn accept(self) -> i32 {
        match self {
            Self::Continue => 0,
            Self::Alt(alt) => alt,
            Self::Escape(_) => ESCAPE,
            Self::Conflict(_) => CONFLICT_ESCAPE,
            Self::Guarded(_) => GUARDED,
        }
    }
}

/// The words of an image: borrowed from a `static` array, or owned by a table
/// built at generation time.
#[derive(Clone, Debug)]
enum ImageWords {
    Static(&'static [u32]),
    #[cfg(feature = "static-dfa-build")]
    Owned {
        words: std::sync::Arc<[u32]>,
        start: usize,
        len: usize,
    },
}

impl ImageWords {
    #[inline]
    fn as_slice(&self) -> &[u32] {
        match self {
            Self::Static(words) => words,
            #[cfg(feature = "static-dfa-build")]
            Self::Owned { words, start, len } => &words[*start..*start + *len],
        }
    }

    /// The words `start..start + len` of the image.
    fn sub(&self, start: usize, len: usize) -> Self {
        match self {
            Self::Static(words) => Self::Static(&words[start..start + len]),
            #[cfg(feature = "static-dfa-build")]
            Self::Owned {
                words, start: base, ..
            } => Self::Owned {
                words: words.clone(),
                start: base + start,
                len,
            },
        }
    }
}

/// The header of a table image: a table of words with no padding and no
/// invalid bit pattern.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Header {
    /// Words of the whole table image, header included.
    total_words: u32,
    /// Decision index, or `-1` for a table only reachable through a dispatch.
    decision: i32,
    /// States of the table.
    num_states: u32,
    /// Distinct rows of the table.
    num_rows: u32,
    /// Token of row column 0.
    min_token: i32,
    /// Token columns of a row, `0` when the table has no edge.
    row_width: u32,
    /// Byte offset of the outcomes, `num_states` entries.
    outcomes: u32,
    /// Byte offset of the fallbacks.
    fallbacks: u32,
    /// Number of fallbacks: `0` or `num_states`.
    fallbacks_len: u32,
    /// Byte offset of the `u16` row index per state.
    row_of: u32,
    /// Byte offset of the `u16` rows.
    rows: u32,
    /// Byte offset of the guard danger set.
    guard_danger: u32,
    /// Number of invoking states in the guard danger set.
    guard_danger_len: u32,
    /// Byte offset of the guard pass set.
    guard_pass: u32,
    /// Number of invoking states in the guard pass set.
    guard_pass_len: u32,
    /// Byte offset of the guard group spans.
    guard_groups: u32,
    /// Number of guard group span entries: `0`, or the group count plus one.
    guard_groups_len: u32,
    /// Byte offset of the continuation records.
    continuations: u32,
    /// Number of continuation records.
    continuations_len: u32,
    /// Byte offset of the context nodes of all continuations.
    continuation_ctxs: u32,
    /// Number of context nodes.
    continuation_ctxs_len: u32,
    /// Byte offset of the context edges of all context nodes.
    continuation_ctx_edges: u32,
    /// Number of context edges.
    continuation_ctx_edges_len: u32,
    /// Byte offset of the configs of all continuations.
    continuation_configs: u32,
    /// Number of configs.
    continuation_configs_len: u32,
}

/// Words of a table image header.
const HEADER_WORDS: usize = size_of::<Header>() / 4;

/// A precedence dispatch: word offsets and counts into the dispatch data.
///
/// `class_tables[i]` is the class table for `cutoffs[i-1] < p <= cutoffs[i]`
/// (the first class covers `p <= cutoffs[0]`, the last one `p > cutoffs[last]`)
/// or `-1` when the class stays adaptive. `by_precedence[p]` is the class
/// table of `p` for the small precedence values.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct DispatchRecord {
    cutoffs: u32,
    num_cutoffs: u32,
    class_tables: u32,
    by_precedence: u32,
    by_precedence_len: u32,
}

/// The continuation of one frontier escape state: spans of the context and
/// config pools of its table.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct ContinuationRecord {
    state: u32,
    ctxs_off: u32,
    ctxs_len: u32,
    configs_off: u32,
    configs_len: u32,
}

/// One context DAG node: a span of the context edge pool. A node with one
/// edge is a `Singleton` context, a node with more is an `Array`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct ContinuationCtxSpan {
    edges_off: u32,
    edges_len: u32,
}

/// One `(parent, return state)` pair of a context DAG node.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ContinuationCtxEdge {
    /// Parent node index within the continuation, or [`EMPTY_CONTEXT`].
    pub parent: u32,
    /// ATN state number of the return state, or `-1` for none.
    pub return_state: i32,
}

/// One config of a continuation.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

/// The closed SLL config set a frontier escape stopped at, borrowed from the
/// table image.
///
/// The set is post-closure and input-independent, so the runtime materializes
/// it at most once per escape state and resumes the SLL simulation from it.
#[derive(Clone, Copy, Debug)]
pub struct ContinuationView<'a> {
    state: u32,
    ctxs: &'a [ContinuationCtxSpan],
    edges: &'a [ContinuationCtxEdge],
    configs: &'a [ContinuationConfigEntry],
}

impl<'a> ContinuationView<'a> {
    /// DFA state number of the escape state.
    pub fn state(&self) -> u32 {
        self.state
    }

    /// The number of context DAG nodes.
    pub fn num_contexts(&self) -> usize {
        self.ctxs.len()
    }

    /// The context DAG nodes in topological order, each as its
    /// `(parent, return state)` pairs: a node refers only to nodes before it.
    pub fn contexts(&self) -> impl ExactSizeIterator<Item = &'a [ContinuationCtxEdge]> + 'a {
        let edges = self.edges;
        self.ctxs.iter().map(move |span| {
            let start = span.edges_off as usize;
            &edges[start..start + span.edges_len as usize]
        })
    }

    /// The configs of the closed set.
    pub fn configs(&self) -> &'a [ContinuationConfigEntry] {
        self.configs
    }
}

/// The table of one decision, or of one precedence class: a zero-copy view
/// over its packed image with pre-resolved pointers for the walker.
#[repr(C)]
#[derive(Clone)]
pub struct StaticParserTable {
    /// Pointer to the `num_rows * row_width` `u16` row entries.
    rows_ptr: *const u16,
    /// Pointer to the `num_states` `u16` row indices.
    row_of_ptr: *const u16,
    /// Pointer to the `num_states` `i32` outcomes.
    outcomes_ptr: *const i32,
    /// Pointer to the `num_states` `i32` fallbacks, null when the table has
    /// none.
    fallbacks_ptr: *const i32,
    min_token: i32,
    row_width: u32,
    num_states: u32,
    /// `row_width` when the walk starts on row 0 of the image, `0` when the
    /// start state stops the walk or has no edge.
    start_width: u32,
    data: ImageWords,
    #[cfg(target_endian = "big")]
    big_endian_u16: BigEndianU16Tables,
}

// The walker header of a table embedded in `.rodata` is one cache line.
#[cfg(all(
    not(feature = "static-dfa-build"),
    target_pointer_width = "64",
    target_endian = "little"
))]
const _: () = assert!(size_of::<StaticParserTable>() == 64);

/// Owns the unpacked `u16` tables of a big-endian image: the raw pointers in
/// [`StaticParserTable`] point into these buffers.
#[cfg(target_endian = "big")]
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct BigEndianU16Tables {
    row_of: std::sync::Arc<[u16]>,
    rows: std::sync::Arc<[u16]>,
}

// SAFETY: Every raw pointer in `StaticParserTable` points into immutable
// memory backed by `self.data` (or `self.big_endian_u16` on big-endian
// targets) that is never mutated or reallocated while the table lives.
unsafe impl Send for StaticParserTable {}
unsafe impl Sync for StaticParserTable {}

impl Debug for StaticParserTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = self.header();
        f.debug_struct("StaticParserTable")
            .field("decision", &header.decision)
            .field("num_states", &header.num_states)
            .field("num_rows", &header.num_rows)
            .field("row_width", &header.row_width)
            .field("words", &self.data.as_slice().len())
            .finish()
    }
}

#[cfg(feature = "static-dfa-build")]
impl PartialEq for StaticParserTable {
    fn eq(&self, other: &Self) -> bool {
        self.words() == other.words()
    }
}

#[cfg(feature = "static-dfa-build")]
impl Eq for StaticParserTable {}

impl StaticParserTable {
    /// Overlays and validates a table image.
    fn load(data: ImageWords, table: usize) -> Result<Self, StaticParserDfaError> {
        let words = data.as_slice();
        if words.len() < HEADER_WORDS {
            return Err(StaticParserDfaError::TruncatedImage {
                what: "a table header",
            });
        }
        // SAFETY: `words` holds at least `HEADER_WORDS` words, and every `u32`
        // bit pattern is a valid `Header`.
        let header = unsafe { *(words.as_ptr() as *const Header) };
        header.validate(table, words.len())?;

        let bytes_ptr = words.as_ptr() as *const u8;
        // SAFETY: `validate` proved that every section is 4-byte aligned and
        // inside `words`.
        let (outcomes_ptr, fallbacks_ptr) = unsafe {
            (
                bytes_ptr.add(header.outcomes as usize) as *const i32,
                if header.fallbacks_len == 0 {
                    std::ptr::null()
                } else {
                    bytes_ptr.add(header.fallbacks as usize) as *const i32
                },
            )
        };
        let num_cells = header.num_rows as usize * header.row_width as usize;

        #[cfg(target_endian = "little")]
        // SAFETY: as above.
        let (row_of_ptr, rows_ptr) = unsafe {
            (
                bytes_ptr.add(header.row_of as usize) as *const u16,
                bytes_ptr.add(header.rows as usize) as *const u16,
            )
        };

        #[cfg(target_endian = "big")]
        let (row_of_ptr, rows_ptr, big_endian_u16) = {
            let unpack = |offset: u32, entries: usize| -> std::sync::Arc<[u16]> {
                // SAFETY: as above; the section holds `entries.div_ceil(2)`
                // words.
                let section = unsafe {
                    std::slice::from_raw_parts(
                        bytes_ptr.add(offset as usize) as *const u32,
                        entries.div_ceil(2),
                    )
                };
                (0..entries)
                    .map(|i| (section[i >> 1] >> ((i & 1) * 16)) as u16)
                    .collect::<Vec<_>>()
                    .into()
            };
            let row_of = unpack(header.row_of, header.num_states as usize);
            let rows = unpack(header.rows, num_cells);
            (
                row_of.as_ptr(),
                rows.as_ptr(),
                BigEndianU16Tables { row_of, rows },
            )
        };

        let mut loaded = Self {
            rows_ptr,
            row_of_ptr,
            outcomes_ptr,
            fallbacks_ptr,
            min_token: header.min_token,
            row_width: header.row_width,
            num_states: header.num_states,
            start_width: 0,
            data,
            #[cfg(target_endian = "big")]
            big_endian_u16,
        };
        loaded.validate_entries(table, num_cells)?;
        // SAFETY: every table has state 0, and `validate_entries` checked
        // that its row is row 0 or none.
        let start_row = unsafe { *loaded.row_of_ptr };
        if start_row == 0 && loaded.outcomes()[0] == 0 {
            loaded.start_width = loaded.row_width;
        }
        Ok(loaded)
    }

    #[inline]
    fn header(&self) -> &Header {
        // SAFETY: a table only ever wraps an image whose first `HEADER_WORDS`
        // words `load` checked.
        unsafe { &*(self.data.as_slice().as_ptr() as *const Header) }
    }

    /// The table of `len` entries of `T` at byte `offset` of the image.
    ///
    /// `offset` and `len` come from the validated header, so the section is
    /// 4-byte aligned and inside the image, and `T` is made of 4-byte words.
    #[inline]
    fn section<T>(&self, offset: u32, len: u32) -> &[T] {
        debug_assert_eq!(size_of::<T>() % 4, 0, "sections hold whole words");
        debug_assert!(align_of::<T>() <= 4, "sections are word aligned");
        let words = self.data.as_slice();
        // SAFETY: see above.
        unsafe {
            std::slice::from_raw_parts(
                (words.as_ptr() as *const u8).add(offset as usize) as *const T,
                len as usize,
            )
        }
    }

    #[inline]
    fn outcomes(&self) -> &[i32] {
        // SAFETY: `outcomes_ptr` points to `num_states` entries.
        unsafe { std::slice::from_raw_parts(self.outcomes_ptr, self.num_states as usize) }
    }

    #[inline]
    fn fallbacks(&self) -> &[i32] {
        if self.fallbacks_ptr.is_null() {
            return &[];
        }
        // SAFETY: a non-null `fallbacks_ptr` points to `num_states` entries.
        unsafe { std::slice::from_raw_parts(self.fallbacks_ptr, self.num_states as usize) }
    }

    #[inline]
    fn row_of(&self) -> &[u16] {
        // SAFETY: `row_of_ptr` points to `num_states` entries.
        unsafe { std::slice::from_raw_parts(self.row_of_ptr, self.num_states as usize) }
    }

    #[inline]
    fn rows(&self) -> &[u16] {
        let header = self.header();
        // SAFETY: `rows_ptr` points to `num_rows * row_width` entries.
        unsafe {
            std::slice::from_raw_parts(
                self.rows_ptr,
                header.num_rows as usize * header.row_width as usize,
            )
        }
    }

    fn guard_danger(&self) -> &[i32] {
        let header = self.header();
        self.section(header.guard_danger, header.guard_danger_len)
    }

    fn guard_groups(&self) -> &[u32] {
        let header = self.header();
        self.section(header.guard_groups, header.guard_groups_len)
    }

    /// The danger set of a guard group.
    ///
    /// Panics when `group` is not a guard group of the table.
    #[inline]
    fn guard_group_danger(&self, group: u32) -> &[i32] {
        let groups = self.guard_groups();
        let group = group as usize;
        &self.guard_danger()[groups[group] as usize..groups[group + 1] as usize]
    }

    fn guard_pass(&self) -> &[i32] {
        let header = self.header();
        self.section(header.guard_pass, header.guard_pass_len)
    }

    fn continuation_records(&self) -> &[ContinuationRecord] {
        let header = self.header();
        self.section(header.continuations, header.continuations_len)
    }

    fn continuation_ctxs(&self) -> &[ContinuationCtxSpan] {
        let header = self.header();
        self.section(header.continuation_ctxs, header.continuation_ctxs_len)
    }

    fn continuation_ctx_edges(&self) -> &[ContinuationCtxEdge] {
        let header = self.header();
        self.section(
            header.continuation_ctx_edges,
            header.continuation_ctx_edges_len,
        )
    }

    fn continuation_configs(&self) -> &[ContinuationConfigEntry] {
        let header = self.header();
        self.section(header.continuation_configs, header.continuation_configs_len)
    }

    /// The packed image words of this table.
    pub fn words(&self) -> &[u32] {
        self.data.as_slice()
    }

    /// The decision index of the table, `-1` for a precedence class table.
    pub fn decision(&self) -> i32 {
        self.header().decision
    }

    /// The number of states of the table.
    pub fn num_states(&self) -> usize {
        self.num_states as usize
    }

    /// The decoded outcome of a state.
    ///
    /// Panics when `state` is not a state of the table.
    #[inline]
    pub(crate) fn outcome(&self, state: i32) -> Outcome {
        Outcome::decode(self.outcomes()[state as usize]).expect("load validated every outcome")
    }

    /// The `outcomes` entry of a state.
    ///
    /// # Safety
    ///
    /// `state` must be below the state count of the table. The start state 0
    /// and every edge target are.
    #[inline(always)]
    pub(crate) unsafe fn outcome_unchecked(&self, state: usize) -> i32 {
        debug_assert!(state < self.num_states as usize);
        unsafe { *self.outcomes_ptr.add(state) }
    }

    /// The row entry of the start state for a token, [`NO_EDGE`] when the
    /// start state stops the walk or has no edge for the token.
    #[inline(always)]
    pub(crate) fn start_entry(&self, token: i32) -> u16 {
        let column = (token as u32).wrapping_sub(self.min_token as u32);
        if column >= self.start_width {
            return NO_EDGE;
        }
        // SAFETY: a nonzero `start_width` means that row 0 exists and has
        // `start_width` columns.
        unsafe { *self.rows_ptr.add(column as usize) }
    }

    /// The row entry of a state for a token: [`NO_EDGE`] or the target state,
    /// with [`TERMINAL_BIT`] when the walk stops at the target.
    ///
    /// # Safety
    ///
    /// `state` must be below the state count of the table. The start state 0
    /// and every edge target are.
    #[inline(always)]
    pub(crate) unsafe fn edge_entry_unchecked(&self, state: usize, token: i32) -> u16 {
        debug_assert!(state < self.num_states as usize);
        let column = (token as u32).wrapping_sub(self.min_token as u32);
        if column >= self.row_width {
            return NO_EDGE;
        }
        // SAFETY: `load` checked `row_of` entries against `num_rows`, and
        // `column < row_width`.
        unsafe {
            let row = *self.row_of_ptr.add(state);
            if row == NO_ROW {
                return NO_EDGE;
            }
            *self
                .rows_ptr
                .add(row as usize * self.row_width as usize + column as usize)
        }
    }

    /// The accept marker or predicted alternative of a state: `0` for a state
    /// that continues the walk, the alternative of an accept state, or one of
    /// [`ESCAPE`], [`CONFLICT_ESCAPE`], and [`GUARDED`]. A value at or above
    /// [`VIRTUAL_ALT_BASE`] is the virtual alternative of a shared-descent
    /// group.
    ///
    /// Panics when `state` is not a state of the table.
    pub fn accept(&self, state: i32) -> i32 {
        self.outcome(state).accept()
    }

    /// The alternative a guarded state resolves to, if the state is guarded.
    ///
    /// Panics when `state` is not a state of the table.
    pub fn guarded_alt(&self, state: i32) -> Option<i32> {
        self.guard(state).map(|guard| guard.alt)
    }

    /// The stack guard of a state, if the state is guarded.
    ///
    /// Panics when `state` is not a state of the table.
    pub fn guard(&self, state: i32) -> Option<Guard> {
        match self.outcome(state) {
            Outcome::Guarded(guard) => Some(guard),
            _ => None,
        }
    }

    /// `true` when the table carries at least one guarded state.
    pub fn has_guards(&self) -> bool {
        self.outcomes()
            .iter()
            .any(|&raw| matches!(Outcome::decode(raw), Some(Outcome::Guarded(_))))
    }

    /// The number of guard groups of the table.
    pub fn num_guard_groups(&self) -> usize {
        self.guard_groups().len().saturating_sub(1)
    }

    /// The guard step of one invoking state for a guard group. `Some(true)`
    /// means that the follow region of the state reaches a guard root of the
    /// group, so the guard defers. `Some(false)` means that a mandatory token
    /// seals the frame off, so the guard does not defer. `None` means that the
    /// frame can pop out of its rule, so the stack walk continues upwards.
    ///
    /// Panics when `group` is not a guard group of the table.
    #[inline]
    pub fn guard_step(&self, group: u32, invoking_state: i32) -> Option<bool> {
        if self
            .guard_group_danger(group)
            .binary_search(&invoking_state)
            .is_ok()
        {
            return Some(true);
        }
        if self.guard_pass().binary_search(&invoking_state).is_err() {
            return Some(false);
        }
        None
    }

    /// The lowest alternative that already left the decision, if any.
    ///
    /// Panics when `state` is not a state of the table.
    #[inline]
    pub fn fallback(&self, state: i32) -> Option<i32> {
        let fallbacks = self.fallbacks();
        if fallbacks.is_empty() {
            assert!((state as u32) < self.num_states, "state out of bounds");
            return None;
        }
        let fallback = fallbacks[state as usize];
        (fallback > 0).then_some(fallback)
    }

    /// The lowest conflicting alternative of a conflict escape state.
    ///
    /// Panics when `state` is not a state of the table.
    pub fn conflict_min_alt(&self, state: i32) -> Option<i32> {
        match self.outcome(state) {
            Outcome::Conflict(alt) => Some(alt),
            _ => None,
        }
    }

    /// The continuation of a frontier escape state and its slot.
    ///
    /// Panics when `state` is not a state of the table.
    pub fn continuation(&self, state: i32) -> Option<(usize, ContinuationView<'_>)> {
        match self.outcome(state) {
            Outcome::Escape(Some(slot)) => Some((slot as usize, self.continuation_at(slot))),
            _ => None,
        }
    }

    /// The continuation in a slot.
    ///
    /// Panics when `slot` is not a continuation slot of the table.
    pub(crate) fn continuation_at(&self, slot: u32) -> ContinuationView<'_> {
        let record = self.continuation_records()[slot as usize];
        let ctxs = self.continuation_ctxs();
        let configs = self.continuation_configs();
        let ctxs_off = record.ctxs_off as usize;
        let configs_off = record.configs_off as usize;
        ContinuationView {
            state: record.state,
            ctxs: &ctxs[ctxs_off..ctxs_off + record.ctxs_len as usize],
            edges: self.continuation_ctx_edges(),
            configs: &configs[configs_off..configs_off + record.configs_len as usize],
        }
    }

    /// The number of continuations, i.e. of cache slots per DFA.
    pub fn num_continuations(&self) -> usize {
        self.header().continuations_len as usize
    }

    /// The target state of a token edge, or `None` when the table has no edge.
    ///
    /// Panics when `state` is not a state of the table.
    #[inline]
    pub fn edge(&self, state: i32, token: i32) -> Option<i32> {
        assert!((state as u32) < self.num_states, "state out of bounds");
        // SAFETY: checked above.
        let entry = unsafe { self.edge_entry_unchecked(state as usize, token) };
        (entry != NO_EDGE).then_some(i32::from(entry & STATE_MASK))
    }

    /// Checks the table entries that the walker dereferences unchecked.
    fn validate_entries(&self, table: usize, num_cells: usize) -> Result<(), StaticParserDfaError> {
        let header = self.header();
        let num_states = self.num_states as usize;
        let num_rows = header.num_rows as usize;
        let reference = |what, value: i64, count| StaticParserDfaError::InvalidReference {
            table,
            what,
            value,
            count,
        };
        let malformed = |what| StaticParserDfaError::Malformed { table, what };

        let outcomes = self.outcomes();
        let mut referenced_slots = 0usize;
        for (state, &raw) in outcomes.iter().enumerate() {
            match Outcome::decode(raw) {
                None => return Err(malformed("a state outcome")),
                Some(Outcome::Escape(Some(slot))) => {
                    let record =
                        self.continuation_records()
                            .get(slot as usize)
                            .ok_or(reference(
                                "continuation slot",
                                i64::from(slot),
                                header.continuations_len as usize,
                            ))?;
                    if record.state as usize != state {
                        return Err(malformed("a continuation state"));
                    }
                    referenced_slots += 1;
                }
                Some(_) => {}
            }
        }
        // Every record names a distinct escape state that names it back, so
        // the records and the escape states are in one-to-one correspondence.
        if referenced_slots != header.continuations_len as usize {
            return Err(malformed("the continuation slots"));
        }

        for &row in self.row_of() {
            if row != NO_ROW && row as usize >= num_rows {
                return Err(reference("row index", i64::from(row), num_rows));
            }
        }
        if !matches!(self.row_of()[0], 0 | NO_ROW) {
            return Err(malformed("the row of the start state"));
        }
        for &entry in self.rows() {
            if entry == NO_EDGE {
                continue;
            }
            let target = (entry & STATE_MASK) as usize;
            if target >= num_states {
                return Err(reference("edge target", target as i64, num_states));
            }
            let terminal = outcomes[target] != 0;
            if (entry & TERMINAL_BIT != 0) != terminal {
                return Err(malformed("the terminal bit of an edge"));
            }
        }
        debug_assert_eq!(self.rows().len(), num_cells);

        let groups = self.guard_groups();
        let danger_len = self.guard_danger().len();
        if !groups.is_empty()
            && (groups.len() < 2
                || groups.len() - 1 > MAX_GUARD_GROUPS
                || groups[0] != 0
                || groups[groups.len() - 1] as usize != danger_len
                || groups.windows(2).any(|pair| pair[0] > pair[1]))
        {
            return Err(malformed("the guard groups"));
        }
        if groups.is_empty() && danger_len != 0 {
            return Err(malformed("the guard groups"));
        }
        let danger_sets = groups
            .windows(2)
            .map(|pair| &self.guard_danger()[pair[0] as usize..pair[1] as usize]);
        for set in danger_sets.chain([self.guard_pass()]) {
            if set.first().is_some_and(|&state| state < 0)
                || set.windows(2).any(|pair| pair[0] >= pair[1])
            {
                return Err(malformed("a guard set"));
            }
        }
        let num_groups = groups.len().saturating_sub(1);
        for &raw in outcomes {
            let Some(Outcome::Guarded(guard)) = Outcome::decode(raw) else {
                continue;
            };
            if guard.group as usize >= num_groups {
                return Err(reference("guard group", i64::from(guard.group), num_groups));
            }
        }

        self.validate_continuations(table)
    }

    /// Checks that the continuation records cover their pools back to back and
    /// that every context DAG is topological.
    fn validate_continuations(&self, table: usize) -> Result<(), StaticParserDfaError> {
        let malformed = |what| StaticParserDfaError::Malformed { table, what };
        let ctxs = self.continuation_ctxs();
        let edges = self.continuation_ctx_edges();
        let configs = self.continuation_configs();
        let mut next_ctx = 0usize;
        let mut next_config = 0usize;
        for record in self.continuation_records() {
            if record.ctxs_off as usize != next_ctx || record.configs_off as usize != next_config {
                return Err(malformed("a continuation span"));
            }
            let nodes = span_of(ctxs, next_ctx, record.ctxs_len)
                .ok_or(malformed("a continuation context span"))?;
            let own_configs = span_of(configs, next_config, record.configs_len)
                .ok_or(malformed("a continuation config span"))?;
            next_ctx += nodes.len();
            next_config += own_configs.len();
            for config in own_configs {
                if config.context != EMPTY_CONTEXT && config.context as usize >= nodes.len() {
                    return Err(StaticParserDfaError::InvalidContinuationContext {
                        index: config.context,
                        decoded: nodes.len(),
                    });
                }
                if config.state < 0 || config.alt <= 0 || config.outer_depth < 0 {
                    return Err(malformed("a continuation config"));
                }
            }
        }
        if next_ctx != ctxs.len() || next_config != configs.len() {
            return Err(malformed("the continuation pools"));
        }

        // Each continuation's nodes index from 0, so a node's position in its
        // continuation is its local index.
        let mut next_edge = 0usize;
        for record in self.continuation_records() {
            let start = record.ctxs_off as usize;
            let nodes = &ctxs[start..start + record.ctxs_len as usize];
            for (index, span) in nodes.iter().enumerate() {
                if span.edges_off as usize != next_edge || span.edges_len == 0 {
                    return Err(malformed("a context node span"));
                }
                let node_edges = span_of(edges, next_edge, span.edges_len)
                    .ok_or(malformed("a context node span"))?;
                next_edge += node_edges.len();
                for edge in node_edges {
                    if edge.parent != EMPTY_CONTEXT && edge.parent as usize >= index {
                        return Err(StaticParserDfaError::InvalidContinuationContext {
                            index: edge.parent,
                            decoded: index,
                        });
                    }
                    if edge.return_state < -1 {
                        return Err(malformed("a continuation return state"));
                    }
                }
            }
        }
        if next_edge != edges.len() {
            return Err(malformed("the context edge pool"));
        }
        Ok(())
    }
}

impl Header {
    /// Checks the declared length and shape, and that every section is inside
    /// the image and word aligned.
    fn validate(&self, table: usize, words: usize) -> Result<(), StaticParserDfaError> {
        if self.total_words as usize != words {
            return Err(StaticParserDfaError::LengthMismatch {
                what: "a table header",
                declared: self.total_words as usize,
                actual: words,
            });
        }
        let num_states = self.num_states as usize;
        let num_rows = self.num_rows as usize;
        let row_width = self.row_width as usize;
        let bad_shape = num_states == 0
            || num_states > MAX_TABLE_STATES
            || num_rows > num_states
            || row_width > MAX_TABLE_COLUMNS
            || num_rows * row_width > MAX_TABLE_CELLS
            || (num_rows == 0) != (row_width == 0)
            || self.min_token < -1
            || i64::from(self.min_token) + row_width as i64 > i64::from(i32::MAX)
            || (self.fallbacks_len != 0 && self.fallbacks_len != self.num_states);
        if bad_shape {
            return Err(StaticParserDfaError::InvalidTableSize {
                table,
                num_states,
                num_rows,
                row_width,
            });
        }

        let total = words as u64 * 4;
        let records = |len: u32, size: usize| u64::from(len) * size as u64;
        let packed = |entries: u64| entries.div_ceil(2) * 4;
        let sections: [(&'static str, u32, u64); 12] = [
            ("outcomes", self.outcomes, records(self.num_states, 4)),
            ("fallbacks", self.fallbacks, records(self.fallbacks_len, 4)),
            ("row_of", self.row_of, packed(num_states as u64)),
            ("rows", self.rows, packed((num_rows * row_width) as u64)),
            (
                "guard_danger",
                self.guard_danger,
                records(self.guard_danger_len, 4),
            ),
            (
                "guard_pass",
                self.guard_pass,
                records(self.guard_pass_len, 4),
            ),
            (
                "guard_groups",
                self.guard_groups,
                records(self.guard_groups_len, 4),
            ),
            (
                "continuations",
                self.continuations,
                records(self.continuations_len, size_of::<ContinuationRecord>()),
            ),
            (
                "continuation_ctxs",
                self.continuation_ctxs,
                records(self.continuation_ctxs_len, size_of::<ContinuationCtxSpan>()),
            ),
            (
                "continuation_ctx_edges",
                self.continuation_ctx_edges,
                records(
                    self.continuation_ctx_edges_len,
                    size_of::<ContinuationCtxEdge>(),
                ),
            ),
            (
                "continuation_configs",
                self.continuation_configs,
                records(
                    self.continuation_configs_len,
                    size_of::<ContinuationConfigEntry>(),
                ),
            ),
            ("header", 0, (HEADER_WORDS * 4) as u64),
        ];
        for (what, offset, len) in sections {
            let start = u64::from(offset);
            if start % 4 != 0 || start + len > total {
                return Err(StaticParserDfaError::OutOfBounds { table, what });
            }
        }
        Ok(())
    }
}

/// The static parser tables of one parser unit: a zero-copy view over the
/// packed image container.
#[derive(Clone)]
pub struct StaticParserTables {
    slots_ptr: *const u32,
    num_slots: u32,
    num_dispatches: u32,
    dispatches_ptr: *const DispatchRecord,
    dispatch_data_ptr: *const i32,
    tables: Box<[StaticParserTable]>,
    data: ImageWords,
}

// SAFETY: Every raw pointer in `StaticParserTables` points into immutable
// memory backed by `self.data` that is never mutated or reallocated while the
// tables live.
unsafe impl Send for StaticParserTables {}
unsafe impl Sync for StaticParserTables {}

impl Debug for StaticParserTables {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("StaticParserTables")
            .field("num_decisions", &self.num_slots)
            .field("num_dispatches", &self.num_dispatches)
            .field("tables", &self.tables)
            .finish()
    }
}

impl Default for StaticParserTables {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(feature = "static-dfa-build")]
impl PartialEq for StaticParserTables {
    fn eq(&self, other: &Self) -> bool {
        self.words() == other.words()
    }
}

#[cfg(feature = "static-dfa-build")]
impl Eq for StaticParserTables {}

impl StaticParserTables {
    /// The empty table set of a parser unit without static tables.
    pub fn empty() -> Self {
        Self {
            slots_ptr: std::ptr::NonNull::dangling().as_ptr(),
            num_slots: 0,
            num_dispatches: 0,
            dispatches_ptr: std::ptr::NonNull::dangling().as_ptr(),
            dispatch_data_ptr: std::ptr::NonNull::dangling().as_ptr(),
            tables: Box::new([]),
            data: ImageWords::Static(&[]),
        }
    }

    /// Overlays an embedded packed image container.
    ///
    /// The container, the sections of every table, and every entry the walker
    /// dereferences unchecked are validated; the image stays borrowed.
    pub fn from_image(image: &'static [u32]) -> Result<Self, StaticParserDfaError> {
        Self::load(ImageWords::Static(image))
    }

    /// Loads a container the builder produced.
    #[cfg(feature = "static-dfa-build")]
    pub fn from_owned_image(image: Vec<u32>) -> Result<Self, StaticParserDfaError> {
        let len = image.len();
        Self::load(ImageWords::Owned {
            words: image.into(),
            start: 0,
            len,
        })
    }

    fn load(data: ImageWords) -> Result<Self, StaticParserDfaError> {
        let image = data.as_slice();
        if image.len() < PREAMBLE_WORDS {
            return Err(StaticParserDfaError::TruncatedImage {
                what: "the container preamble",
            });
        }
        if image[0] != STATIC_PARSER_DFA_MAGIC {
            return Err(StaticParserDfaError::BadMagic { found: image[0] });
        }
        if image[1] != FORMAT_VERSION {
            return Err(StaticParserDfaError::UnsupportedVersion { found: image[1] });
        }
        if image[2] as usize != image.len() {
            return Err(StaticParserDfaError::LengthMismatch {
                what: "the container",
                declared: image[2] as usize,
                actual: image.len(),
            });
        }
        let num_slots = image[3] as usize;
        let num_tables = image[4] as usize;
        let num_dispatches = image[5] as usize;
        let dispatch_data_len = image[6] as usize;
        let slots_start = PREAMBLE_WORDS;
        let dir_start = slots_start.checked_add(num_slots);
        let dispatches_start = dir_start.and_then(|start| {
            num_tables
                .checked_mul(TABLE_DIR_WORDS)
                .and_then(|words| start.checked_add(words))
        });
        let dispatch_data_start = dispatches_start.and_then(|start| {
            num_dispatches
                .checked_mul(DISPATCH_WORDS)
                .and_then(|words| start.checked_add(words))
        });
        let tables_start = dispatch_data_start
            .and_then(|start| start.checked_add(dispatch_data_len))
            .filter(|&end| end <= image.len());
        let (
            Some(dir_start),
            Some(dispatches_start),
            Some(dispatch_data_start),
            Some(tables_start),
        ) = (
            dir_start,
            dispatches_start,
            dispatch_data_start,
            tables_start,
        )
        else {
            return Err(StaticParserDfaError::TruncatedImage {
                what: "the container directory",
            });
        };

        let directory = image[dir_start..dispatches_start]
            .as_chunks::<TABLE_DIR_WORDS>()
            .0;
        let mut next_offset = 0usize;
        let mut tables = Vec::with_capacity(num_tables);
        let table_words = image.len() - tables_start;
        for (table, &[offset, len]) in directory.iter().enumerate() {
            if offset as usize != next_offset || len as usize > table_words - next_offset {
                return Err(StaticParserDfaError::Malformed {
                    table,
                    what: "the table directory entry",
                });
            }
            tables.push(StaticParserTable::load(
                data.sub(tables_start + next_offset, len as usize),
                table,
            )?);
            next_offset += len as usize;
        }
        if next_offset != table_words {
            return Err(StaticParserDfaError::LengthMismatch {
                what: "the table directory",
                declared: tables_start + next_offset,
                actual: image.len(),
            });
        }

        let base = image.as_ptr();
        // SAFETY: the section starts are inside `image`, and `DispatchRecord`
        // is made of `u32` words.
        let loaded = unsafe {
            Self {
                slots_ptr: base.add(slots_start),
                num_slots: num_slots as u32,
                num_dispatches: num_dispatches as u32,
                dispatches_ptr: base.add(dispatches_start) as *const DispatchRecord,
                dispatch_data_ptr: base.add(dispatch_data_start) as *const i32,
                tables: tables.into_boxed_slice(),
                data,
            }
        };
        loaded.validate_dispatches(dispatch_data_len)?;
        loaded.validate_slots()?;
        Ok(loaded)
    }

    fn slots(&self) -> &[u32] {
        // SAFETY: `slots_ptr` points to `num_slots` words of the image.
        unsafe { std::slice::from_raw_parts(self.slots_ptr, self.num_slots as usize) }
    }

    fn dispatches(&self) -> &[DispatchRecord] {
        // SAFETY: `dispatches_ptr` points to `num_dispatches` records.
        unsafe { std::slice::from_raw_parts(self.dispatches_ptr, self.num_dispatches as usize) }
    }

    /// `len` words of the dispatch data at `offset`.
    ///
    /// # Safety
    ///
    /// The range must be one `validate_dispatches` checked.
    #[inline]
    unsafe fn dispatch_data(&self, offset: u32, len: u32) -> &[i32] {
        unsafe {
            std::slice::from_raw_parts(self.dispatch_data_ptr.add(offset as usize), len as usize)
        }
    }

    /// Checks that every decision slot names a table or dispatch, and that
    /// the decision tables and their slots name each other.
    fn validate_slots(&self) -> Result<(), StaticParserDfaError> {
        let malformed = |decision, what| StaticParserDfaError::Malformed {
            table: decision,
            what,
        };
        for (decision, &slot) in self.slots().iter().enumerate() {
            let (what, value, count) = if slot == NO_TABLE {
                continue;
            } else if slot & DISPATCH_BIT != 0 {
                (
                    "precedence dispatch",
                    slot & !DISPATCH_BIT,
                    self.num_dispatches as usize,
                )
            } else {
                ("decision table", slot, self.tables.len())
            };
            if value as usize >= count {
                return Err(StaticParserDfaError::InvalidReference {
                    table: decision,
                    what,
                    value: i64::from(value),
                    count,
                });
            }
            if slot & DISPATCH_BIT == 0 && self.tables[slot as usize].decision() != decision as i32
            {
                return Err(malformed(decision, "the table of a decision slot"));
            }
        }
        for (index, table) in self.tables.iter().enumerate() {
            if table.decision() == -1 {
                continue;
            }
            let named = usize::try_from(table.decision())
                .ok()
                .and_then(|decision| self.slots().get(decision));
            if named != Some(&(index as u32)) {
                return Err(malformed(index, "the decision of a table"));
            }
        }
        Ok(())
    }

    fn validate_dispatches(&self, dispatch_data_len: usize) -> Result<(), StaticParserDfaError> {
        for (index, record) in self.dispatches().iter().enumerate() {
            let malformed = |what| StaticParserDfaError::Malformed { table: index, what };
            let in_bounds = |offset: u32, len: u32| {
                (offset as usize)
                    .checked_add(len as usize)
                    .is_some_and(|end| end <= dispatch_data_len)
            };
            if !in_bounds(record.cutoffs, record.num_cutoffs)
                || record.num_cutoffs == u32::MAX
                || !in_bounds(record.class_tables, record.num_cutoffs + 1)
                || !in_bounds(record.by_precedence, record.by_precedence_len)
            {
                return Err(malformed("a precedence dispatch span"));
            }
            // SAFETY: the spans are inside the dispatch data.
            let (cutoffs, class_tables, by_precedence) = unsafe {
                (
                    self.dispatch_data(record.cutoffs, record.num_cutoffs),
                    self.dispatch_data(record.class_tables, record.num_cutoffs + 1),
                    self.dispatch_data(record.by_precedence, record.by_precedence_len),
                )
            };
            if cutoffs.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err(malformed("the precedence cutoffs"));
            }
            for &table in class_tables {
                if table < -1 || (table >= 0 && table as usize >= self.tables.len()) {
                    return Err(StaticParserDfaError::InvalidReference {
                        table: index,
                        what: "precedence class table",
                        value: i64::from(table),
                        count: self.tables.len(),
                    });
                }
                if table >= 0 && self.tables[table as usize].decision() != -1 {
                    return Err(malformed("the decision of a precedence class table"));
                }
            }
            for (precedence, &table) in by_precedence.iter().enumerate() {
                if table != class_tables[class_of(cutoffs, precedence as i32)] {
                    return Err(malformed("the dense precedence table"));
                }
            }
        }
        Ok(())
    }

    /// The packed image words of the container, empty for [`Self::empty`].
    pub fn words(&self) -> &[u32] {
        self.data.as_slice()
    }

    /// `true` when the unit has no static table at all.
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    /// The table pool.
    pub fn tables(&self) -> &[StaticParserTable] {
        &self.tables
    }

    /// The number of decision slots.
    pub fn num_decisions(&self) -> usize {
        self.num_slots as usize
    }

    /// `true` when the decision selects its table by precedence.
    pub fn is_precedence_dispatched(&self, decision: i32) -> bool {
        self.slots()
            .get(decision as u32 as usize)
            .is_some_and(|&slot| slot != NO_TABLE && slot & DISPATCH_BIT != 0)
    }

    /// The table of a decision at a precedence, or `None` when the decision is
    /// adaptive at that precedence.
    #[inline]
    pub fn table(&self, decision: i32, precedence: i32) -> Option<&StaticParserTable> {
        let slot = *self.slots().get(decision as u32 as usize)?;
        let index = if slot & DISPATCH_BIT == 0 {
            slot as usize
        } else if slot == NO_TABLE {
            return None;
        } else {
            let table = self.dispatch_table_of(slot & !DISPATCH_BIT, precedence);
            if table < 0 {
                return None;
            }
            table as usize
        };
        // SAFETY: `load` checked every slot and class table index against the
        // table count.
        Some(unsafe { self.tables.get_unchecked(index) })
    }

    /// The class table index of a precedence in a dispatch, `-1` for an
    /// adaptive class.
    #[inline]
    fn dispatch_table_of(&self, dispatch: u32, precedence: i32) -> i32 {
        // SAFETY: `validate_slots` checked the dispatch index, and
        // `validate_dispatches` its spans.
        unsafe {
            let record = &*self.dispatches_ptr.add(dispatch as usize);
            let precedence = precedence.max(0);
            if (precedence as u32) < record.by_precedence_len {
                return *self
                    .dispatch_data_ptr
                    .add(record.by_precedence as usize + precedence as usize);
            }
            let cutoffs = self.dispatch_data(record.cutoffs, record.num_cutoffs);
            let class_tables = self.dispatch_data(record.class_tables, record.num_cutoffs + 1);
            class_tables[class_of(cutoffs, precedence)]
        }
    }

    /// Checks the tables against the ATN they are loaded with.
    ///
    /// The loader can only check the image against itself; the ATN-relative
    /// indexes (decision slots, guard states, config states, continuation
    /// return states) are only meaningful once the ATN is known, and a wrong
    /// index would panic when a continuation is materialized.
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
        if self.num_decisions() > num_decisions {
            return Err(StaticParserDfaError::OutsideAtn {
                what: "decision slot",
                index: num_decisions as i64,
                count: num_decisions,
            });
        }
        for table in self.tables() {
            let decision = table.decision();
            if decision < -1 || (decision >= 0 && decision as usize >= num_decisions) {
                return Err(StaticParserDfaError::OutsideAtn {
                    what: "table decision",
                    index: i64::from(decision),
                    count: num_decisions,
                });
            }
            for &state in table.guard_danger().iter().chain(table.guard_pass()) {
                check_state(i64::from(state), "guard invoking state")?;
            }
            for edge in table.continuation_ctx_edges() {
                if edge.return_state >= 0 {
                    check_state(i64::from(edge.return_state), "return state")?;
                }
            }
            for config in table.continuation_configs() {
                check_state(i64::from(config.state), "continuation config state")?;
            }
        }
        Ok(())
    }
}

/// The `len` entries of `pool` at `start`, `None` when they leave the pool.
fn span_of<T>(pool: &[T], start: usize, len: u32) -> Option<&[T]> {
    pool.get(start..start.checked_add(len as usize)?)
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

#[cfg(feature = "static-dfa-build")]
pub use build::{
    ContinuationContextNode, DecisionTables, ParserContinuation, PrecedenceDispatch,
    StaticParserTableBuilder,
};

/// The generation-time builder of the table images.
#[cfg(feature = "static-dfa-build")]
mod build {
    use std::collections::HashMap;

    use super::*;

    /// One node of a continuation context DAG.
    ///
    /// The nodes are in topological order: a node refers only to nodes that
    /// come before it, and [`EMPTY_CONTEXT`] names the empty context. A
    /// one-parent node is a `Singleton` context and a node with two or more
    /// parents is an `Array`.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum ContinuationContextNode {
        /// A `Singleton` context: one return state on top of one parent
        /// context.
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

    /// The closed SLL config set a frontier escape stopped at.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ParserContinuation {
        /// DFA state number of the escape state.
        pub state: u32,
        /// Context DAG nodes, in topological order.
        pub contexts: Vec<ContinuationContextNode>,
        /// Configs of the closed set.
        pub configs: Vec<ContinuationConfigEntry>,
    }

    /// How the tables of one decision are selected.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum DecisionTables {
        /// The decision has no table and always runs the adaptive simulation.
        None,
        /// The decision has exactly one table.
        Single(usize),
        /// The decision is a precedence loop with one table per precedence
        /// class.
        Dispatch(PrecedenceDispatch),
    }

    /// Precedence classes of a left-recursive loop decision.
    ///
    /// `class_tables[i]` is the class table for `cutoffs[i-1] < p <=
    /// cutoffs[i]` (the first class covers `p <= cutoffs[0]`, the last one `p
    /// > cutoffs[last]`) or `-1` when the class stays adaptive.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct PrecedenceDispatch {
        /// The precedence cut points, ascending.
        pub cutoffs: Vec<i32>,
        /// One table index per class, `-1` for an adaptive class.
        pub class_tables: Vec<i32>,
    }

    impl PrecedenceDispatch {
        /// Builds the dispatch of the classes.
        pub fn new(cutoffs: Vec<i32>, class_tables: Vec<i32>) -> Self {
            assert_eq!(
                class_tables.len(),
                cutoffs.len() + 1,
                "one class table per class"
            );
            Self {
                cutoffs,
                class_tables,
            }
        }

        /// The table index of a precedence value, `-1` for an adaptive class.
        pub fn table_of(&self, precedence: i32) -> i32 {
            self.class_tables[class_of(&self.cutoffs, precedence.max(0))]
        }

        /// The dense lookup of the small precedence values: `p > cutoffs[last]`
        /// is the last class, so the table ends one entry past the last
        /// cutoff. Huge precedence values stay out of it and take the counted
        /// fallback instead.
        fn by_precedence(&self) -> Vec<i32> {
            let last = self.cutoffs.last().copied().unwrap_or(-1);
            let dense_end = last.saturating_add(1).clamp(0, MAX_DENSE_PRECEDENCE);
            (0..=dense_end).map(|p| self.table_of(p)).collect()
        }
    }

    /// The table of one decision, or of one precedence class, before it is
    /// frozen into its image.
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StaticParserTableBuilder {
        decision: i32,
        outcomes: Vec<Outcome>,
        fallbacks: Vec<i32>,
        /// Per state, the canonical sorted disjoint `(lo, hi, target)` runs.
        edges: Vec<Vec<(i32, i32, u32)>>,
        continuations: Vec<ParserContinuation>,
        /// The danger sets of all guard groups, back to back.
        guard_danger: Vec<i32>,
        /// The span bounds of the guard groups in `guard_danger`: empty, or
        /// `0` and then the end of every group.
        guard_group_ends: Vec<u32>,
        guard_pass: Vec<i32>,
    }

    impl StaticParserTableBuilder {
        /// Builds a table from per-state markers and edge ranges.
        ///
        /// `accepts[state]` is `0`, an alternative, or one of [`ESCAPE`],
        /// [`CONFLICT_ESCAPE`], and [`GUARDED`]. A positive
        /// `conflict_min_alts[state]` makes the state a conflict escape; a
        /// conflict escape without one is a plain escape. `edges[state]`
        /// lists `(lo, hi, target)` inclusive token ranges; a later range
        /// overrides an earlier one.
        pub fn from_state_edges(
            decision: i32,
            accepts: Vec<i32>,
            fallbacks: Vec<i32>,
            conflict_min_alts: Vec<i32>,
            continuations: Vec<ParserContinuation>,
            edges: &[Vec<(i32, i32, u32)>],
        ) -> Self {
            let num_states = accepts.len();
            assert!(
                (1..=MAX_TABLE_STATES).contains(&num_states),
                "a table has 1..={MAX_TABLE_STATES} states"
            );
            assert_eq!(fallbacks.len(), num_states, "one fallback per state");
            assert_eq!(
                conflict_min_alts.len(),
                num_states,
                "one conflict alternative per state"
            );
            assert_eq!(edges.len(), num_states, "one edge list per state");
            let outcomes = accepts
                .iter()
                .zip(&conflict_min_alts)
                .map(|(&accept, &min_alt)| {
                    assert!(accept >= GUARDED, "unknown accept marker {accept}");
                    if min_alt > 0 {
                        assert!(min_alt as u32 <= OUTCOME_PAYLOAD);
                        Outcome::Conflict(min_alt)
                    } else if accept > 0 {
                        Outcome::Alt(accept)
                    } else if accept == 0 {
                        Outcome::Continue
                    } else {
                        Outcome::Escape(None)
                    }
                })
                .collect();
            let mut builder = Self {
                decision,
                outcomes,
                fallbacks,
                edges: edges.iter().map(|ranges| canonical_runs(ranges)).collect(),
                continuations: Vec::new(),
                guard_danger: Vec::new(),
                guard_group_ends: Vec::new(),
                guard_pass: Vec::new(),
            };
            for continuation in continuations {
                builder.attach_continuation(continuation);
            }
            builder
        }

        /// Attaches the continuation of a frontier escape state. A
        /// continuation of a state that is not a plain escape is dropped: the
        /// walker never resumes from it.
        fn attach_continuation(&mut self, continuation: ParserContinuation) {
            let state = continuation.state as usize;
            let outcome = self
                .outcomes
                .get_mut(state)
                .expect("a continuation names a state of the table");
            match outcome {
                Outcome::Escape(None) => {
                    *outcome = Outcome::Escape(Some(self.continuations.len() as u32));
                    self.continuations.push(continuation);
                }
                Outcome::Escape(Some(_)) => panic!("state {state} carries two continuations"),
                _ => {}
            }
        }

        /// Attaches the guard section: the guard of every guarded state, the
        /// danger set of every guard group, and the pass set of the table
        /// (invoking state numbers).
        #[must_use]
        pub fn with_guards(
            mut self,
            guards: Vec<Option<Guard>>,
            guard_groups: Vec<Vec<i32>>,
            mut guard_pass: Vec<i32>,
        ) -> Self {
            assert_eq!(guards.len(), self.outcomes.len(), "one guard per state");
            assert!(
                guard_groups.len() <= MAX_GUARD_GROUPS,
                "a table has at most {MAX_GUARD_GROUPS} guard groups"
            );
            for (state, guard) in guards.into_iter().enumerate() {
                let Some(guard) = guard else { continue };
                assert!(
                    (1..=MAX_GUARD_ALT).contains(&guard.alt)
                        && (0..=MAX_GUARD_ALT).contains(&guard.defer_alt)
                        && guard.defer_alt != guard.alt,
                    "invalid guard {guard:?} of state {state}"
                );
                assert!(
                    (guard.group as usize) < guard_groups.len(),
                    "guard group {} of state {state} is not a group of the table",
                    guard.group
                );
                if let Outcome::Escape(Some(_)) = self.outcomes[state] {
                    panic!("guarded state {state} carries a continuation");
                }
                self.outcomes[state] = Outcome::Guarded(guard);
            }
            self.guard_danger.clear();
            self.guard_group_ends.clear();
            if !guard_groups.is_empty() {
                self.guard_group_ends.push(0);
            }
            for mut danger in guard_groups {
                danger.sort_unstable();
                danger.dedup();
                self.guard_danger.extend(danger);
                self.guard_group_ends.push(self.guard_danger.len() as u32);
            }
            guard_pass.sort_unstable();
            guard_pass.dedup();
            self.guard_pass = guard_pass;
            self
        }

        /// Freezes the table into its image words.
        pub fn to_image(&self) -> Vec<u32> {
            let num_states = self.outcomes.len();
            let outcomes: Vec<i32> = self.outcomes.iter().map(|o| o.encode()).collect();
            // The walk stops at a terminal state, so its edges are dead.
            let live_edges: Vec<&[(i32, i32, u32)]> = self
                .edges
                .iter()
                .zip(&outcomes)
                .map(|(runs, &outcome)| if outcome == 0 { &runs[..] } else { &[] })
                .collect();

            let min_token = live_edges
                .iter()
                .copied()
                .flatten()
                .map(|&(lo, _, _)| lo)
                .min()
                .unwrap_or(0);
            let max_token = live_edges
                .iter()
                .copied()
                .flatten()
                .map(|&(_, hi, _)| hi)
                .max()
                .unwrap_or(-1);
            let row_width = if max_token >= min_token {
                (i64::from(max_token) - i64::from(min_token) + 1) as usize
            } else {
                0
            };
            let min_token = if row_width == 0 { 0 } else { min_token };
            assert!(
                row_width <= MAX_TABLE_COLUMNS,
                "a table row must not be wider than {MAX_TABLE_COLUMNS} columns"
            );
            let mut rows: Vec<u16> = Vec::new();
            let mut row_of = Vec::with_capacity(num_states);
            let mut distinct: HashMap<Vec<u16>, u16> = HashMap::new();
            for runs in live_edges {
                if runs.is_empty() {
                    row_of.push(NO_ROW);
                    continue;
                }
                let mut row = vec![NO_EDGE; row_width];
                for &(lo, hi, target) in runs {
                    let entry = if outcomes[target as usize] != 0 {
                        target as u16 | TERMINAL_BIT
                    } else {
                        target as u16
                    };
                    let start = (lo - min_token) as usize;
                    let end = (hi - min_token) as usize;
                    row[start..=end].fill(entry);
                }
                let next = (rows.len() / row_width) as u16;
                let index = *distinct.entry(row).or_insert_with_key(|row| {
                    rows.extend_from_slice(row);
                    next
                });
                row_of.push(index);
            }
            let num_rows = rows.len().checked_div(row_width).unwrap_or(0);
            assert!(
                num_rows * row_width <= MAX_TABLE_CELLS,
                "a table must not have more than {MAX_TABLE_CELLS} row cells"
            );

            let has_fallbacks = self.fallbacks.iter().any(|&fallback| fallback > 0);
            let mut ctx_spans = Vec::new();
            let mut ctx_edges = Vec::new();
            let mut configs = Vec::new();
            let mut records = Vec::new();
            for continuation in &self.continuations {
                let ctxs_off = ctx_spans.len() as u32;
                let configs_off = configs.len() as u32;
                for node in &continuation.contexts {
                    let edges_off = ctx_edges.len() as u32;
                    match node {
                        ContinuationContextNode::Singleton {
                            parent,
                            return_state,
                        } => ctx_edges.push(ContinuationCtxEdge {
                            parent: *parent,
                            return_state: *return_state,
                        }),
                        ContinuationContextNode::Array {
                            parents,
                            return_states,
                        } => {
                            assert_eq!(parents.len(), return_states.len());
                            ctx_edges.extend(parents.iter().zip(return_states).map(
                                |(&parent, &return_state)| ContinuationCtxEdge {
                                    parent,
                                    return_state,
                                },
                            ));
                        }
                    }
                    ctx_spans.push(ContinuationCtxSpan {
                        edges_off,
                        edges_len: ctx_edges.len() as u32 - edges_off,
                    });
                }
                configs.extend_from_slice(&continuation.configs);
                records.push(ContinuationRecord {
                    state: continuation.state,
                    ctxs_off,
                    ctxs_len: ctx_spans.len() as u32 - ctxs_off,
                    configs_off,
                    configs_len: configs.len() as u32 - configs_off,
                });
            }

            let mut words = vec![0u32; HEADER_WORDS];
            let mut header = Header {
                total_words: 0,
                decision: self.decision,
                num_states: num_states as u32,
                num_rows: num_rows as u32,
                min_token,
                row_width: row_width as u32,
                outcomes: 0,
                fallbacks: 0,
                fallbacks_len: if has_fallbacks { num_states as u32 } else { 0 },
                row_of: 0,
                rows: 0,
                guard_danger: 0,
                guard_danger_len: self.guard_danger.len() as u32,
                guard_pass: 0,
                guard_pass_len: self.guard_pass.len() as u32,
                guard_groups: 0,
                guard_groups_len: self.guard_group_ends.len() as u32,
                continuations: 0,
                continuations_len: records.len() as u32,
                continuation_ctxs: 0,
                continuation_ctxs_len: ctx_spans.len() as u32,
                continuation_ctx_edges: 0,
                continuation_ctx_edges_len: ctx_edges.len() as u32,
                continuation_configs: 0,
                continuation_configs_len: configs.len() as u32,
            };
            header.outcomes = push_words(&mut words, outcomes.iter().map(|&o| o as u32));
            header.fallbacks = push_words(
                &mut words,
                self.fallbacks
                    .iter()
                    .filter(|_| has_fallbacks)
                    .map(|&fallback| fallback as u32),
            );
            header.row_of = push_packed_u16(&mut words, &row_of);
            header.rows = push_packed_u16(&mut words, &rows);
            header.guard_danger =
                push_words(&mut words, self.guard_danger.iter().map(|&s| s as u32));
            header.guard_pass = push_words(&mut words, self.guard_pass.iter().map(|&s| s as u32));
            header.guard_groups = push_words(&mut words, self.guard_group_ends.iter().copied());
            header.continuations = push_words(
                &mut words,
                records.iter().flat_map(|record| {
                    [
                        record.state,
                        record.ctxs_off,
                        record.ctxs_len,
                        record.configs_off,
                        record.configs_len,
                    ]
                }),
            );
            header.continuation_ctxs = push_words(
                &mut words,
                ctx_spans
                    .iter()
                    .flat_map(|span| [span.edges_off, span.edges_len]),
            );
            header.continuation_ctx_edges = push_words(
                &mut words,
                ctx_edges
                    .iter()
                    .flat_map(|edge| [edge.parent, edge.return_state as u32]),
            );
            header.continuation_configs = push_words(
                &mut words,
                configs.iter().flat_map(|config| {
                    [
                        config.state as u32,
                        config.alt as u32,
                        config.context,
                        config.outer_depth as u32,
                    ]
                }),
            );
            header.total_words = words.len() as u32;
            header.write(&mut words);
            words
        }
    }

    /// The canonical runs of a state's edge ranges: sorted, disjoint, with
    /// equal adjacent targets merged, and a later range overriding an earlier
    /// one. Tokens below EOF are dropped.
    fn canonical_runs(ranges: &[(i32, i32, u32)]) -> Vec<(i32, i32, u32)> {
        let ranges: Vec<_> = ranges
            .iter()
            .map(|&(lo, hi, target)| (lo.max(-1), hi, target))
            .filter(|&(lo, hi, _)| lo <= hi)
            .collect();
        let Some(lo) = ranges.iter().map(|&(lo, _, _)| lo).min() else {
            return Vec::new();
        };
        let hi = ranges.iter().map(|&(_, hi, _)| hi).max().unwrap_or(lo);
        let mut dense = vec![None; (i64::from(hi) - i64::from(lo) + 1) as usize];
        for &(start, end, target) in &ranges {
            dense[(start - lo) as usize..=(end - lo) as usize].fill(Some(target));
        }
        let mut runs: Vec<(i32, i32, u32)> = Vec::new();
        for (offset, target) in dense.into_iter().enumerate() {
            let Some(target) = target else { continue };
            let token = lo + offset as i32;
            match runs.last_mut() {
                Some((_, end, last)) if *last == target && *end + 1 == token => *end = token,
                _ => runs.push((token, token, target)),
            }
        }
        runs
    }

    impl Header {
        /// Writes the header into the first words of `words`.
        fn write(&self, words: &mut [u32]) {
            assert!(words.len() >= HEADER_WORDS, "image holds the header");
            // SAFETY: the header is `HEADER_WORDS` words with no padding, and
            // `words` is at least that long and 4-byte aligned.
            unsafe { std::ptr::write(words.as_mut_ptr() as *mut Header, *self) };
        }
    }

    /// Appends a section of words and returns its byte offset.
    fn push_words(words: &mut Vec<u32>, values: impl IntoIterator<Item = u32>) -> u32 {
        let offset = (words.len() * 4) as u32;
        words.extend(values);
        offset
    }

    /// Appends a `u16` section, two entries per word, and returns its byte
    /// offset.
    fn push_packed_u16(words: &mut Vec<u32>, values: &[u16]) -> u32 {
        let offset = (words.len() * 4) as u32;
        words.extend(
            values.chunks(2).map(|pair| {
                u32::from(pair[0]) | (u32::from(pair.get(1).copied().unwrap_or(0)) << 16)
            }),
        );
        offset
    }

    impl StaticParserTables {
        /// Freezes per-decision slots and the table pool into one image
        /// container and loads it.
        ///
        /// Panics when the tables do not form a valid image.
        pub fn new(decisions: Vec<DecisionTables>, tables: Vec<StaticParserTableBuilder>) -> Self {
            if tables.is_empty() {
                return Self::empty();
            }
            Self::from_owned_image(Self::image(&decisions, &tables))
                .unwrap_or_else(|error| panic!("the builder tables form an invalid image: {error}"))
        }

        /// The image container of per-decision slots and the table pool.
        pub fn image(
            decisions: &[DecisionTables],
            tables: &[StaticParserTableBuilder],
        ) -> Vec<u32> {
            let mut slots = Vec::with_capacity(decisions.len());
            let mut dispatches = Vec::new();
            let mut dispatch_data: Vec<i32> = Vec::new();
            for slot in decisions {
                slots.push(match slot {
                    DecisionTables::None => NO_TABLE,
                    DecisionTables::Single(index) => *index as u32,
                    DecisionTables::Dispatch(dispatch) => {
                        let by_precedence = dispatch.by_precedence();
                        let record = DispatchRecord {
                            cutoffs: dispatch_data.len() as u32,
                            num_cutoffs: dispatch.cutoffs.len() as u32,
                            class_tables: (dispatch_data.len() + dispatch.cutoffs.len()) as u32,
                            by_precedence: (dispatch_data.len() + 2 * dispatch.cutoffs.len() + 1)
                                as u32,
                            by_precedence_len: by_precedence.len() as u32,
                        };
                        dispatch_data.extend_from_slice(&dispatch.cutoffs);
                        dispatch_data.extend_from_slice(&dispatch.class_tables);
                        dispatch_data.extend_from_slice(&by_precedence);
                        dispatches.push(record);
                        DISPATCH_BIT | (dispatches.len() - 1) as u32
                    }
                });
            }
            let images: Vec<Vec<u32>> = tables
                .iter()
                .map(StaticParserTableBuilder::to_image)
                .collect();
            let mut words = vec![
                STATIC_PARSER_DFA_MAGIC,
                FORMAT_VERSION,
                0,
                slots.len() as u32,
                images.len() as u32,
                dispatches.len() as u32,
                dispatch_data.len() as u32,
            ];
            words.extend_from_slice(&slots);
            let mut offset = 0u32;
            for image in &images {
                words.push(offset);
                words.push(image.len() as u32);
                offset += image.len() as u32;
            }
            for record in &dispatches {
                words.extend_from_slice(&[
                    record.cutoffs,
                    record.num_cutoffs,
                    record.class_tables,
                    record.by_precedence,
                    record.by_precedence_len,
                ]);
            }
            words.extend(dispatch_data.iter().map(|&value| value as u32));
            for image in &images {
                words.extend_from_slice(image);
            }
            words[2] = words.len() as u32;
            words
        }
    }
}

#[cfg(all(test, feature = "static-dfa-build"))]
mod tests {
    use super::*;

    fn dispatch(cutoffs: &[i32], classes: &[i32]) -> PrecedenceDispatch {
        PrecedenceDispatch::new(cutoffs.to_vec(), classes.to_vec())
    }

    fn table(
        decision: i32,
        accepts: Vec<i32>,
        edges: &[Vec<(i32, i32, u32)>],
    ) -> StaticParserTableBuilder {
        let states = accepts.len();
        StaticParserTableBuilder::from_state_edges(
            decision,
            accepts,
            vec![0; states],
            vec![0; states],
            Vec::new(),
            edges,
        )
    }

    fn single(table: StaticParserTableBuilder) -> StaticParserTables {
        StaticParserTables::new(vec![DecisionTables::Single(0)], vec![table])
    }

    fn reload(tables: &StaticParserTables) -> StaticParserTables {
        StaticParserTables::from_image(Box::leak(tables.words().to_vec().into_boxed_slice()))
            .expect("the image loads")
    }

    #[test]
    fn precedence_dispatch_covers_every_class() {
        // `class_tables[i]` covers `cutoffs[i-1] < p <= cutoffs[i]`, and the
        // class after the last cutoff covers every larger precedence.
        let pool = (1..=3)
            .map(|alt| table(-1, vec![alt], &[Vec::new()]))
            .collect();
        let tables = StaticParserTables::new(
            vec![DecisionTables::Dispatch(dispatch(&[2, 5], &[0, 1, 2]))],
            pool,
        );
        let accept = |precedence| tables.table(0, precedence).expect("a class").accept(0);
        assert!(tables.is_precedence_dispatched(0));
        assert_eq!(accept(-1), 1);
        assert_eq!(accept(0), 1);
        assert_eq!(accept(2), 1);
        assert_eq!(accept(3), 2);
        assert_eq!(accept(5), 2);
        assert_eq!(accept(6), 3);
        assert_eq!(accept(1_000_000), 3);
        assert_eq!(accept(i32::MAX), 3);

        let builder = dispatch(&[2, 5], &[10, 11, 12]);
        assert_eq!(builder.table_of(-1), 10);
        assert_eq!(builder.table_of(3), 11);
        assert_eq!(builder.table_of(i32::MAX), 12);
    }

    #[test]
    fn precedence_dispatch_beyond_the_dense_table_counts_cutoffs() {
        let pool = (1..=3)
            .map(|alt| table(-1, vec![alt], &[Vec::new()]))
            .collect();
        let tables = StaticParserTables::new(
            vec![DecisionTables::Dispatch(dispatch(
                &[1500, 3000],
                &[0, -1, 2],
            ))],
            pool,
        );
        assert_eq!(tables.table(0, 1500).map(|t| t.accept(0)), Some(1));
        assert!(tables.table(0, 2000).is_none(), "an adaptive class");
        assert_eq!(tables.table(0, 3001).map(|t| t.accept(0)), Some(3));
        assert!(tables.table(1, 0).is_none(), "no such decision");
        assert!(tables.table(-1, 0).is_none(), "no such decision");
    }

    #[test]
    fn rows_are_deduplicated_and_trimmed() {
        let built = single(table(
            0,
            vec![0, 0, 1],
            &[
                vec![(1, 2, 1), (4, 4, 2)],
                vec![(1, 2, 1), (4, 4, 2)],
                vec![],
            ],
        ));
        let table = built.table(0, 0).expect("decision 0 has a table");
        assert_eq!(table.edge(0, 1), Some(1));
        assert_eq!(table.edge(0, 3), None);
        assert_eq!(table.edge(0, -1), None);
        assert_eq!(table.edge(0, 5), None);
        assert_eq!(table.edge(2, 1), None);
        assert_eq!(table.header().num_rows, 1, "states 0 and 1 share a row");
        assert_eq!(table.header().row_width, 4, "tokens 1..=4");
        // SAFETY: state 0 and 1 are states of the table.
        unsafe {
            assert_eq!(table.edge_entry_unchecked(0, 1), 1, "1 continues the walk");
            assert_eq!(
                table.edge_entry_unchecked(1, 4),
                2 | TERMINAL_BIT,
                "2 accepts"
            );
        }
        assert_eq!(reload(&built), built);
    }

    #[test]
    fn terminal_states_keep_no_row() {
        // State 1 accepts, so its edges are dead and do not widen the rows.
        let built = single(table(0, vec![0, 1], &[vec![(3, 4, 1)], vec![(90, 99, 0)]]));
        let accepting = built.table(0, 0).expect("the table");
        assert_eq!(accepting.header().num_rows, 1);
        assert_eq!(accepting.header().row_width, 2, "tokens 3..=4");
        assert_eq!(accepting.edge(1, 90), None);
        assert_eq!(accepting.start_entry(3), 1 | TERMINAL_BIT);
        assert_eq!(accepting.start_entry(5), NO_EDGE);

        let start_accepts = single(table(0, vec![2], &[vec![(1, 1, 0)]]));
        let start = start_accepts.table(0, 0).expect("the table");
        assert_eq!(
            start.start_entry(1),
            NO_EDGE,
            "the start state stops the walk"
        );
        assert_eq!(start.accept(0), 2);
    }

    #[test]
    fn later_ranges_override_earlier_ones() {
        let built = single(table(
            0,
            vec![0, 1, 2],
            &[vec![(-5, 9, 1), (3, 4, 2)], Vec::new(), Vec::new()],
        ));
        let table = built.table(0, 0).expect("the table");
        assert_eq!(table.edge(0, -1), Some(1), "tokens below EOF are dropped");
        assert_eq!(table.edge(0, 2), Some(1));
        assert_eq!(table.edge(0, 3), Some(2));
        assert_eq!(table.edge(0, 5), Some(1));
        assert_eq!(table.header().min_token, -1);
    }

    #[test]
    fn guarded_states_round_trip_through_the_guard_section() {
        let guard = |alt, defer_alt, group| {
            Some(Guard {
                alt,
                defer_alt,
                group,
            })
        };
        let built = single(
            StaticParserTableBuilder::from_state_edges(
                0,
                vec![0, GUARDED, GUARDED],
                vec![0, 0, 0],
                vec![0, 0, 0],
                Vec::new(),
                &[vec![(1, 1, 1), (2, 2, 2)], Vec::new(), Vec::new()],
            )
            .with_guards(
                vec![None, guard(1, 0, 0), guard(1, 2, 1)],
                vec![vec![9, 4], vec![11]],
                vec![7, 2],
            ),
        );
        let table = reload(&built);
        let table = table.table(0, 0).expect("the table");
        assert!(table.has_guards());
        assert_eq!(table.num_guard_groups(), 2);
        // `4` and `9` reach a guard root of group 0, so the guard defers. `2`
        // is a pass state, so the walk continues past it. `5` is sealed, so
        // the state resolves to the guarded alternative.
        assert_eq!(table.guard_step(0, 4), Some(true));
        assert_eq!(table.guard_step(0, 9), Some(true));
        assert_eq!(table.guard_step(0, 2), None);
        assert_eq!(table.guard_step(0, 7), None);
        assert_eq!(table.guard_step(0, 5), Some(false));
        assert_eq!(table.guard_step(0, 11), Some(false));
        // Each group has its own danger set.
        assert_eq!(table.guard_step(1, 11), Some(true));
        assert_eq!(table.guard_step(1, 4), Some(false));
        assert_eq!(table.guard_step(1, 2), None);
        assert_eq!(table.guarded_alt(1), Some(1));
        assert_eq!(table.guard(1), guard(1, 0, 0));
        assert_eq!(table.guard(2), guard(1, 2, 1));
        assert_eq!(table.guarded_alt(0), None);
        assert_eq!(table.accept(1), GUARDED);
        assert_eq!(table.accept(2), GUARDED);
    }

    #[test]
    fn malformed_guards_are_rejected() {
        let guarded = |groups: Vec<Vec<i32>>| {
            StaticParserTableBuilder::from_state_edges(
                0,
                vec![GUARDED],
                vec![0],
                vec![0],
                Vec::new(),
                &[Vec::new()],
            )
            .with_guards(
                vec![Some(Guard {
                    alt: 1,
                    defer_alt: 2,
                    group: 0,
                })],
                groups,
                Vec::new(),
            )
        };
        // The word index of a section of the table image in its container.
        let section = |words: &[u32], start: usize, field: fn(&Header) -> u32| {
            // SAFETY: the table image starts with its header.
            let header = unsafe { *(words[start..].as_ptr() as *const Header) };
            start + field(&header) as usize / 4
        };
        let (words, _) = container(&guarded(vec![vec![3, 1]]));
        assert!(reload_owned(words).is_ok());

        // A guarded state names a group that the table does not have.
        let (mut words, start) = container(&guarded(vec![vec![3]]));
        let outcomes = section(&words, start, |h| h.outcomes);
        words[outcomes] |= 1 << 16;
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::InvalidReference {
                what: "guard group",
                ..
            })
        ));

        // The defer alternative equals the guarded alternative.
        let (mut words, start) = container(&guarded(vec![vec![3]]));
        let outcomes = section(&words, start, |h| h.outcomes);
        words[outcomes] = (words[outcomes] & !0xFF00) | (1 << 8);
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed {
                what: "a state outcome",
                ..
            })
        ));

        // The group spans do not cover the danger sets.
        let (mut words, start) = container(&guarded(vec![vec![3, 5]]));
        let groups = section(&words, start, |h| h.guard_groups);
        words[groups + 1] = 1;
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed {
                what: "the guard groups",
                ..
            })
        ));

        // A danger set is not sorted.
        let (mut words, start) = container(&guarded(vec![vec![3, 5]]));
        let danger = section(&words, start, |h| h.guard_danger);
        words.swap(danger, danger + 1);
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed {
                what: "a guard set",
                ..
            })
        ));
    }

    #[test]
    fn virtual_alternatives_round_trip_as_accepts() {
        let built = single(table(
            0,
            vec![0, VIRTUAL_ALT_BASE + 1, 2],
            &[vec![(1, 1, 1), (2, 2, 2)], Vec::new(), Vec::new()],
        ));
        let table = built.table(0, 0).expect("decision 0 has a table");
        assert_eq!(table.accept(1), VIRTUAL_ALT_BASE + 1);
        assert_eq!(table.accept(2), 2);
    }

    #[test]
    fn conflict_escapes_carry_their_min_alt() {
        let built = single(StaticParserTableBuilder::from_state_edges(
            0,
            vec![CONFLICT_ESCAPE, 2, CONFLICT_ESCAPE],
            vec![1, 0, 0],
            vec![1, 0, 0],
            Vec::new(),
            &[vec![(1, 1, 1), (2, 2, 2)], Vec::new(), Vec::new()],
        ));
        let table = built.table(0, 0).expect("decision 0 has a table");
        assert_eq!(table.accept(0), CONFLICT_ESCAPE);
        assert_eq!(table.conflict_min_alt(0), Some(1));
        assert_eq!(table.fallback(0), Some(1));
        assert_eq!(table.accept(1), 2);
        assert_eq!(table.conflict_min_alt(1), None);
        assert_eq!(table.fallback(1), None);
        assert_eq!(table.accept(2), ESCAPE, "no min alt, a plain escape");
    }

    fn continuation(state: u32) -> ParserContinuation {
        ParserContinuation {
            state,
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
        }
    }

    #[test]
    fn continuations_round_trip_as_borrowed_views() {
        let built = single(StaticParserTableBuilder::from_state_edges(
            0,
            vec![0, ESCAPE, ESCAPE],
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![continuation(2), continuation(1)],
            &[vec![(1, 1, 1), (2, 2, 2)], Vec::new(), Vec::new()],
        ));
        let tables = reload(&built);
        let table = tables.table(0, 0).expect("decision 0 has a table");
        assert_eq!(table.num_continuations(), 2);
        let (slot, view) = table.continuation(1).expect("the escape has one");
        assert_eq!(slot, 1);
        assert_eq!(view.state(), 1);
        let contexts: Vec<_> = view.contexts().collect();
        assert_eq!(contexts.len(), 2);
        assert_eq!(
            contexts[0],
            &[ContinuationCtxEdge {
                parent: EMPTY_CONTEXT,
                return_state: 7
            }]
        );
        assert_eq!(contexts[1].len(), 2, "an array node");
        assert_eq!(view.configs(), &continuation(1).configs[..]);
        assert_eq!(table.continuation(2).map(|(slot, _)| slot), Some(0));
        assert!(table.continuation(0).is_none());
    }

    fn reload_owned(words: Vec<u32>) -> Result<StaticParserTables, StaticParserDfaError> {
        StaticParserTables::from_owned_image(words)
    }

    /// The image of one single-decision table, with the word offset of its
    /// table image in the container.
    fn container(builder: &StaticParserTableBuilder) -> (Vec<u32>, usize) {
        let words =
            StaticParserTables::image(&[DecisionTables::Single(0)], std::slice::from_ref(builder));
        let table_start = PREAMBLE_WORDS + 1 + TABLE_DIR_WORDS;
        (words, table_start)
    }

    #[test]
    fn malformed_containers_are_rejected() {
        let builder = table(0, vec![0, 1], &[vec![(1, 1, 1)], Vec::new()]);
        let (words, table_start) = container(&builder);
        assert!(reload_owned(words.clone()).is_ok());

        let mut bad = words.clone();
        bad[0] = 0;
        assert_eq!(
            reload_owned(bad),
            Err(StaticParserDfaError::BadMagic { found: 0 })
        );
        let mut bad = words.clone();
        bad[1] = 2;
        assert_eq!(
            reload_owned(bad),
            Err(StaticParserDfaError::UnsupportedVersion { found: 2 })
        );
        let mut bad = words.clone();
        bad.push(0);
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::LengthMismatch { .. })
        ));
        let mut bad = words.clone();
        bad[PREAMBLE_WORDS] = 1;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::InvalidReference { .. })
        ));
        assert!(matches!(
            reload_owned(words[..3].to_vec()),
            Err(StaticParserDfaError::TruncatedImage { .. })
        ));
        let mut bad = vec![
            STATIC_PARSER_DFA_MAGIC,
            FORMAT_VERSION,
            9,
            u32::MAX - 6,
            0,
            0,
            2,
            0,
            0,
        ];
        assert!(matches!(
            reload_owned(bad.clone()),
            Err(StaticParserDfaError::TruncatedImage { .. })
        ));
        bad[3] = 0;
        bad[5] = u32::MAX;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::TruncatedImage { .. })
        ));

        // Header word 6 is the outcomes offset: unaligned.
        let mut bad = words.clone();
        bad[table_start + 6] += 2;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::OutOfBounds { .. })
        ));
        // Header word 2 is the state count.
        let mut bad = words.clone();
        bad[table_start + 2] = 0;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::InvalidTableSize { .. })
        ));
    }

    #[test]
    fn malformed_entries_are_rejected() {
        let builder = table(0, vec![0, 1], &[vec![(1, 1, 1)], Vec::new()]);
        let (words, table_start) = container(&builder);
        let header_at = |words: &[u32], field: usize| words[table_start + field] as usize / 4;
        let outcomes = table_start + header_at(&words, 6);
        let rows = table_start + header_at(&words, 10);

        // State 1 stops the walk, so the edge into it must carry the bit.
        let mut bad = words.clone();
        bad[rows] &= !u32::from(TERMINAL_BIT);
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::Malformed { .. })
        ));
        let mut bad = words.clone();
        bad[rows] = 0x0005;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::InvalidReference { .. })
        ));
        // A continuation slot without a record.
        let mut bad = words.clone();
        bad[outcomes + 1] = OUTCOME_CONTINUATION;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::InvalidReference { .. })
        ));
        // A conflict escape without its alternative.
        let mut bad = words.clone();
        bad[outcomes + 1] = OUTCOME_CONFLICT;
        assert!(matches!(
            reload_owned(bad),
            Err(StaticParserDfaError::Malformed { .. })
        ));
    }

    #[test]
    fn cyclic_continuation_contexts_are_rejected() {
        // A continuation whose parent is not an ancestor is a cycle for the
        // replay, so the loader rejects it.
        let builder = StaticParserTableBuilder::from_state_edges(
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
        let (words, _) = container(&builder);
        assert!(
            matches!(
                reload_owned(words),
                Err(StaticParserDfaError::InvalidContinuationContext { .. })
            ),
            "a self-referencing context node is a cycle"
        );
    }

    #[test]
    fn decision_slots_and_tables_must_agree() {
        // The table of slot 0 claims decision 1.
        let words = StaticParserTables::image(
            &[DecisionTables::Single(0), DecisionTables::None],
            &[table(1, vec![1], &[Vec::new()])],
        );
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed { table: 0, .. })
        ));
        // An unreferenced table claims an invalid decision.
        let words = StaticParserTables::image(&[], &[table(-2, vec![1], &[Vec::new()])]);
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed { table: 0, .. })
        ));
        // A precedence class table claims a decision.
        let words = StaticParserTables::image(
            &[DecisionTables::Dispatch(dispatch(&[], &[0]))],
            &[table(0, vec![1], &[Vec::new()])],
        );
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed { .. })
        ));
    }

    #[test]
    fn tables_outside_the_atn_are_rejected() {
        use crate::atn::ATN;
        use crate::atn_type::ATNType;

        let atn = ATN::new(ATNType::Parser, 0);
        let tables = single(table(0, vec![1], &[Vec::new()]));
        assert!(matches!(
            tables.validate_for_atn(&atn),
            Err(StaticParserDfaError::OutsideAtn {
                what: "decision slot",
                ..
            })
        ));
        let guarded = StaticParserTables::new(
            Vec::new(),
            vec![table(-1, vec![1], &[Vec::new()]).with_guards(
                vec![None],
                vec![vec![3]],
                Vec::new(),
            )],
        );
        assert!(matches!(
            guarded.validate_for_atn(&atn),
            Err(StaticParserDfaError::OutsideAtn {
                what: "guard invoking state",
                ..
            })
        ));
        let escaping = StaticParserTables::new(
            Vec::new(),
            vec![StaticParserTableBuilder::from_state_edges(
                -1,
                vec![ESCAPE],
                vec![0],
                vec![0],
                vec![continuation(0)],
                &[Vec::new()],
            )],
        );
        assert!(matches!(
            escaping.validate_for_atn(&atn),
            Err(StaticParserDfaError::OutsideAtn {
                what: "return state",
                ..
            })
        ));
        assert_eq!(StaticParserTables::empty().validate_for_atn(&atn), Ok(()));
    }

    #[test]
    fn unsorted_cutoffs_are_rejected() {
        let pool = vec![table(-1, vec![1], &[Vec::new()])];
        let words = StaticParserTables::image(
            &[DecisionTables::Dispatch(dispatch(&[5, 2], &[0, 0, 0]))],
            &pool,
        );
        assert!(matches!(
            reload_owned(words),
            Err(StaticParserDfaError::Malformed { .. })
        ));
    }
}
