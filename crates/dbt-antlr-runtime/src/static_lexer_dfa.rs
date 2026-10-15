//! Runtime side of the static lexer DFA: the table representation that the
//! walker in [`crate::lexer_atn_simulator::LexerATNSimulator`] tokenizes
//! from, and the word-stream codec that carries the tables from the
//! compile-time builder to generated code.
//!
//! The builder (the `static_dfa_build` module, feature `static-dfa-build`)
//! runs the subset construction and commits each mode to the walker's own
//! [`StaticLexerDFA`]. [`StaticLexerTables::to_words`] (same feature)
//! encodes the tables as the word stream that generated code embeds;
//! [`StaticLexerTables::from_words`] decodes the words, with full
//! validation at load: after a successful load, the walker indexes the
//! tables without bounds checks. `to_words` is the exact inverse of
//! `from_words`.
//!
//! The runtime form expands the serialized low rows into dense per-state
//! rows (column 0 is the EOF edge, columns 1..=128 are symbols 0..=127),
//! derives the direct symbol-to-class map from the class starts, expands
//! the deduplicated class rows, and decodes the continuation pool into an
//! owned form ready for the resume path.
//!
//! # Word stream format (version 1)
//!
//! One `i32` value per `u32` word, except where noted. Sizes and indices
//! are `u32`. Symbols are `i32` values in `0..=0x10FFFF`. All multi-word
//! sections use the order of this list:
//!
//! - Preamble: `MAGIC`, `STATIC_DFA_FORMAT_VERSION`, the total word count
//!   of the stream, `num_modes`.
//! - Per mode: `present` flag (0 or 1). A mode with `present == 0` has no
//!   more words.
//! - `num_states`, `start_state`.
//! - `accept_type[num_states]`: token type per state, `-1` when the state
//!   is not an accept state.
//! - `accept_actions[num_states]`: word offset of the accept action list
//!   of the state in the action-list section, `-1` when empty.
//! - `eof[num_states]`: the EOF edge entry of each state.
//! - `low[num_states * 128]`: the edge entries of each state for the
//!   symbols `0..=127`, in symbol order per state.
//! - Hi-class section: `num_classes`, then, when `num_classes > 0`,
//!   `class_starts[num_classes]`, `num_rows`, `num_row_triples`,
//!   `row_offsets[num_rows + 1]` (in triples), `num_row_triples` times
//!   `(lo_class, hi_class, target_entry)`, and `row_of[num_states]`.
//!   Class `i` covers the symbols in `class_starts[i]..class_starts[i + 1]`,
//!   or up to `0x10FFFF` for the last class. `class_starts[0]` is 128, and
//!   the bounds are the union of the hi-edge bounds of the mode. The target
//!   of a state is constant within a class. Rows are deduplicated by
//!   content. The deserializer derives the direct symbol-to-class map from
//!   `class_starts`; the stream carries no expanded map.
//! - Action-list section: `num_action_list_ints`, then the concatenated
//!   lists. A list is `len`, then `len` times the ATN encoding
//!   `(type, data1, data2)` of one lexer action. Lists are deduplicated by
//!   content.
//! - Accel section: `num_accel_ints`, then the run-acceleration entries.
//!   An entry is `(state, ACCEL_POSITIVE, w0, w1, w2, w3)` for a 128-bit
//!   bitmap of self-looping ASCII symbols, or `(state, ACCEL_EXCLUSION,
//!   nl_in_set, n_needles, needle0, needle1, needle2)` with zero-padded
//!   needles. The bitmap never contains `'\n'`. A `\n` that loops is an
//!   exclusion needle with `nl_in_set = 1`, so the walker keeps line and
//!   column exact.
//! - Escape rows: per state, `eof_escape`, `num_ranges`, then `num_ranges`
//!   times `(lo_symbol, hi_symbol, continuation_index)`. The ranges are
//!   the sorted, disjoint symbol intervals whose edge escapes. Edges with a
//!   plain target and edges that do not exist have no range. `eof_escape`
//!   is meaningful only when the EOF edge entry is `ESCAPE`. A
//!   continuation index of `NO_CONTINUATION` marks a budget escape.
//! - Continuation pool: `num_continuations`, then per continuation `kind`,
//!   `nctx`, `nctx` times `(ctx_kind, parent_local, return_state)`,
//!   `nconfigs`, and `nconfigs` times `(state, flags, alt, ctx_local,
//!   nactions)`, with `nactions` times `(action_index, behind)` after each
//!   config.
//!
//! Edge entries are `target | ACCEPT_BIT` when the target state is an
//! accept state, `target` otherwise, `NO_EDGE` when the edge does not
//! exist, or `ESCAPE` for an escape edge. Escape edges never carry
//! `ACCEPT_BIT`. State ids stay below `0x7FFE`, so `ACCEPT_BIT | state`
//! peaks at `0xFFFD`.
//!
//! In a continuation, `kind` is `CONTINUATION_MOVED_CONFIGS` (0) for the
//! post-move, pre-closure configs of the escape edge, or
//! `CONTINUATION_SOURCE_SET` (1) for the source configs of the state. The
//! context table is topological: local id 0 is the empty context, and
//! entry `i` has local id `i + 1` with parents that refer to smaller ids.
//! `ctx_kind` is 0 (singleton); lexer contexts are always empty or
//! singleton chains. In `flags`, bit 0 is `passed_through_non_greedy`,
//! bit 1 is `alt_reached_accept` (meaningful for kind 0 only; the resume
//! recomputes the flag for kind 1), and bit 2 is reserved. `behind` is the
//! count of consumed symbols between the action and the escape edge, or
//! `BEHIND_POSITION_INDEPENDENT` for a position-independent action. The
//! builder only emits `BEHIND_POSITION_INDEPENDENT`.

use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use crate::atn::ATN;
use crate::int_stream::EOF;
use crate::lexer_action::{LexerAction, LEXER_ACTION_TYPE_CUSTOM};

/// First word of the word stream: ASCII "SDFA".
pub const STATIC_DFA_MAGIC: u32 = 0x5344_4641;
/// Version of the word stream format.
pub const STATIC_DFA_FORMAT_VERSION: u32 = 1;
/// Edge entry for an edge that does not exist.
pub const NO_EDGE: u16 = 0xFFFF;
/// Edge entry for an escape edge.
pub const ESCAPE: u16 = 0xFFFE;
/// Bit in an edge entry that marks the target state as an accept state.
pub const ACCEPT_BIT: u16 = 0x8000;
/// Mask that extracts the target state from an edge entry.
pub const STATE_MASK: u16 = 0x7FFF;
/// Continuation index of a budget escape. The walker restarts the token
/// lazily.
pub const NO_CONTINUATION: u32 = u32::MAX;
/// Continuation kind: the post-move, pre-closure configs of the escape edge.
pub const CONTINUATION_MOVED_CONFIGS: u32 = 0;
/// Continuation kind: the source configs of the state. The resume re-runs
/// the full move step.
pub const CONTINUATION_SOURCE_SET: u32 = 1;
/// Accel tag: run over the ASCII symbols of a 128-bit bitmap.
pub const ACCEL_POSITIVE: u32 = 1;
/// Accel tag: run over all input except up to three needle bytes.
pub const ACCEL_EXCLUSION: u32 = 2;
/// `behind` value of a position-independent action.
pub const BEHIND_POSITION_INDEPENDENT: u32 = 0xFFFF;

/// Dense-row width for the symbols 0..=127, which are direct-indexed.
pub(crate) const ASCII_ROW: usize = 128;
/// Low-row stride: column 0 is the EOF edge, then `ASCII_ROW` symbol
/// columns.
pub(crate) const LOW_ROW: usize = ASCII_ROW + 1;
/// Maximum input symbol.
const MAX_SYMBOL: u32 = 0x10FFFF;
/// Table limit of the state count per mode: state ids stay below
/// `ACCEPT_BIT`, so `ACCEPT_BIT | state` never collides with the sentinels.
const MAX_STATE_COUNT: usize = 0x7FFE;

/// All static lexer DFA tables of a grammar, one entry per lexer mode. A
/// mode that the builder could not compile has no table; the walker runs
/// the lazy path for it.
#[derive(Debug)]
#[cfg_attr(feature = "static-dfa-build", derive(PartialEq))]
pub struct StaticLexerTables {
    pub(crate) modes: Vec<Option<StaticLexerDFA>>,
}

/// The table of one lexer mode.
#[derive(Debug)]
#[cfg_attr(feature = "static-dfa-build", derive(PartialEq))]
pub struct StaticLexerDFA {
    pub(crate) start_state: u16,
    /// `num_states * LOW_ROW` dense targets: column 0 is the EOF edge,
    /// columns 1..=128 are the symbols 0..=127.
    pub(crate) low: Vec<u16>,
    /// Direct symbol-to-class map for the symbols `ASCII_ROW..=MAX_SYMBOL`
    /// (empty when the mode has no edges past 127).
    pub(crate) class_of: Vec<u16>,
    /// Deduplicated dense class rows: the hi-edge row of state `s` is
    /// `hi_rows[row_of[s] * num_classes..][..num_classes]`.
    pub(crate) hi_rows: Vec<u16>,
    /// State-to-row map into `hi_rows`.
    pub(crate) row_of: Vec<u32>,
    /// Row width of `hi_rows`: the symbol-class count of the mode.
    pub(crate) num_classes: usize,
    /// Class bounds of the mode; class `i` covers
    /// `class_starts[i]..class_starts[i + 1]`. Used only by load-time
    /// validation; the walker goes through `class_of`.
    pub(crate) class_starts: Vec<u32>,
    /// Token type per accept state; -1 when the state is not an accept
    /// state. 0 is a real accept type: rules with a `more`/`type(...)`
    /// command get no token type assigned by the tool.
    pub(crate) accept_type: Vec<i32>,
    /// `(offset, len)` of the accept action list of each state in
    /// `actions`; `(0, 0)` when empty.
    pub(crate) accept_actions: Vec<(u32, u32)>,
    /// The accept action lists of all states, concatenated and
    /// deduplicated by content. Only position-independent actions appear
    /// here, so the walker can run them with plain `LexerAction::execute`
    /// calls.
    pub(crate) actions: Vec<LexerAction<'static>>,
    /// Run-acceleration flavor tag per state: 0 = none, else the index of
    /// the payload in `accel_data` + 1.
    pub(crate) accel_tag: Vec<u8>,
    /// Run-acceleration payloads (see [`Accel`]), accelerated states only.
    pub(crate) accel_data: Vec<Accel>,
    /// Continuation index of the EOF escape edge of each state.
    /// `NO_CONTINUATION` marks a budget escape, or no EOF escape at all.
    pub(crate) eof_escape: Vec<u32>,
    /// Start offsets of the per-state escape ranges in `escape_ranges`;
    /// `num_states + 1` entries.
    pub(crate) escape_off: Vec<u32>,
    /// The escape ranges of all states, concatenated: the sorted, disjoint
    /// symbol intervals whose edge escapes.
    pub(crate) escape_ranges: Vec<EscapeRange>,
    /// The continuation pool of the mode, decoded for the resume path.
    pub(crate) continuations: Vec<LexerContinuation>,
}

/// One escape range: the edges on the symbols `lo..=hi` escape to the
/// continuation `continuation` (`NO_CONTINUATION` for a budget escape).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EscapeRange {
    pub(crate) lo: i32,
    pub(crate) hi: i32,
    pub(crate) continuation: u32,
}

/// One continuation of the pool: everything the resume path needs to
/// rebuild the configs of an escape edge.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct LexerContinuation {
    /// `CONTINUATION_MOVED_CONFIGS` or `CONTINUATION_SOURCE_SET`.
    pub(crate) kind: u32,
    /// Topological singleton context table: `(parent_local, return_state)`.
    /// Entry `i` has local id `i + 1`; local id 0 is the empty context.
    pub(crate) ctxs: Vec<(u32, i32)>,
    pub(crate) configs: Vec<ContinuationConfig>,
}

/// One config of a continuation.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ContinuationConfig {
    /// ATN state number.
    pub(crate) state: i32,
    pub(crate) alt: i32,
    /// Context as a local id into [`LexerContinuation::ctxs`].
    pub(crate) ctx: u32,
    /// `passed_through_non_greedy` flag bit.
    pub(crate) non_greedy: bool,
    /// The `current_alt_reached_accept_state` flag the closure of the
    /// config ran with. Meaningful for `CONTINUATION_MOVED_CONFIGS` only;
    /// the resume recomputes it for `CONTINUATION_SOURCE_SET`.
    pub(crate) alt_reached_accept: bool,
    /// Indices into `ATN::lexer_actions`: the position-independent action
    /// trace of the config.
    pub(crate) actions: Vec<u32>,
}

/// Per-state run-acceleration descriptor. When a run of consecutive input
/// bytes all loops back to the same state, the walker consumes the whole
/// run with a single scan instead of the per-char table chase. Both
/// flavors re-express the per-char walk exactly: a run ends on the first
/// byte that does not loop, which the normal table step then handles.
/// `\n` is never inside a positive bitmap and is always a needle in the
/// exclusion flavor, so line/column tracking stays exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Accel {
    /// Run over the ASCII bytes set in the bitmap (`\n` never set).
    Positive(u128),
    /// Run over all input except the first `n` of `needles` (every symbol
    /// past 127 loops). `nl_in_set`: `\n` is among the needles but loops; a
    /// `\n` hit means consume-newline-and-continue, not exit-run.
    Exclusion {
        needles: [u8; 3],
        n: u8,
        nl_in_set: bool,
    },
}

impl Accel {
    /// The serialized accel entry of this descriptor for state `s`:
    /// `(state, tag, payload...)`, in the section layout of the word-stream
    /// documentation.
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn to_serialized(self, s: u16) -> Vec<u32> {
        match self {
            Accel::Positive(bitmap) => {
                vec![
                    u32::from(s),
                    ACCEL_POSITIVE,
                    bitmap as u32,
                    (bitmap >> 32) as u32,
                    (bitmap >> 64) as u32,
                    (bitmap >> 96) as u32,
                ]
            }
            Accel::Exclusion {
                needles,
                n,
                nl_in_set,
            } => vec![
                u32::from(s),
                ACCEL_EXCLUSION,
                u32::from(nl_in_set),
                u32::from(n),
                u32::from(needles[0]),
                u32::from(needles[1]),
                u32::from(needles[2]),
            ],
        }
    }
}

/// Failure while decoding a static lexer DFA word stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticLexerError {
    /// Word 0 is not the static DFA magic value.
    BadMagic {
        /// The value found in word 0.
        found: u32,
    },
    /// The total length word does not match the stream length.
    TotalLengthMismatch {
        /// The declared total word count.
        declared: usize,
        /// The actual stream word count.
        actual: usize,
    },
    /// The stream ends before the declared data.
    TruncatedStream {
        /// The word count the stream must have at least.
        expected_at_least: usize,
        /// The actual stream word count.
        actual: usize,
    },
    /// The mode sections end before the declared stream end.
    TrailingWords {
        /// The number of words the sections consumed.
        consumed: usize,
        /// The declared total word count.
        actual: usize,
    },
    /// A mode `present` flag is not 0 or 1.
    InvalidModeFlag {
        /// The mode index.
        mode: usize,
        /// The flag word value.
        value: u32,
    },
    /// A mode state count is 0 or past the table limit.
    InvalidStateCount {
        /// The mode index.
        mode: usize,
        /// The declared state count.
        count: usize,
    },
    /// An index points outside its table.
    IndexOutOfBounds {
        /// The mode index.
        mode: usize,
        /// What the index addresses.
        what: &'static str,
        /// The index value.
        index: u32,
        /// The number of valid entries.
        count: usize,
    },
    /// An edge entry is neither sentinel nor a valid target state, or the
    /// word does not fit a u16 edge entry.
    InvalidEdgeTarget {
        /// The mode index.
        mode: usize,
        /// The raw word value.
        entry: u32,
    },
    /// The accept bit of a non-sentinel edge entry does not match the
    /// accept type of the target state.
    InconsistentAcceptBit {
        /// The mode index.
        mode: usize,
        /// The raw entry value.
        entry: u16,
    },
    /// An accept token type is below -1.
    InvalidAcceptType {
        /// The mode index.
        mode: usize,
        /// The state index.
        state: usize,
        /// The token type value.
        value: i32,
    },
    /// The class starts do not begin at 128, are not strictly increasing,
    /// or exceed `MAX_SYMBOL`.
    InvalidClassStarts {
        /// The mode index.
        mode: usize,
    },
    /// The row offsets do not start at 0, are not monotonic, or do not end
    /// at the declared triple count.
    InvalidRowOffsets {
        /// The mode index.
        mode: usize,
    },
    /// A class run has `lo > hi`, leaves the class range, or overlaps the
    /// previous run of its row.
    InvalidClassRange {
        /// The mode index.
        mode: usize,
        /// The row index.
        row: usize,
    },
    /// An accept action list offset does not point at a complete list in
    /// the action-list section.
    InvalidActionList {
        /// The mode index.
        mode: usize,
        /// The state index.
        state: usize,
    },
    /// An accept action list holds an unknown or position-dependent action
    /// type.
    InvalidActionType {
        /// The mode index.
        mode: usize,
        /// The state index.
        state: usize,
        /// The action type value.
        value: i32,
    },
    /// A section word count does not match the data read for the section.
    SectionLengthMismatch {
        /// The mode index.
        mode: usize,
        /// The section name.
        section: &'static str,
        /// The declared section word count.
        expected: usize,
        /// The number of words read for the section.
        found: usize,
    },
    /// An accel flavor tag is not a known value.
    InvalidAccelTag {
        /// The mode index.
        mode: usize,
        /// The tag value.
        value: u32,
    },
    /// A positive accel bitmap contains `'\n'`; the walker keeps line and
    /// column exact by scanning newlines per char.
    InvalidAccelBitmap {
        /// The mode index.
        mode: usize,
    },
    /// An exclusion accel entry has a needle count outside `1..=3`, a
    /// needle byte past 127, or no `'\n'` needle (the walker scans runs
    /// vectorized; a `'\n'` outside the needles would pass without a line
    /// update).
    InvalidAccelNeedles {
        /// The mode index.
        mode: usize,
    },
    /// An escape row is not sorted and disjoint, or a range leaves the
    /// symbol range.
    InvalidEscapeRow {
        /// The mode index.
        mode: usize,
        /// The state index.
        state: usize,
    },
    /// An escape range endpoint has no ESCAPE edge entry, or an ESCAPE
    /// entry has no covering escape range.
    InconsistentEscapeRow {
        /// The mode index.
        mode: usize,
        /// The state index.
        state: usize,
        /// The symbol (EOF = -1).
        symbol: i32,
    },
    /// A continuation kind is not a known value.
    UnknownContinuationKind {
        /// The mode index.
        mode: usize,
        /// The continuation index.
        index: usize,
        /// The kind value.
        kind: u32,
    },
    /// A context-table entry kind is not 0 (singleton).
    InvalidContextKind {
        /// The mode index.
        mode: usize,
        /// The continuation index.
        index: usize,
        /// The kind value.
        value: u32,
    },
    /// A context-table parent refers to an entry that is not smaller.
    NonTopologicalContext {
        /// The mode index.
        mode: usize,
        /// The continuation index.
        index: usize,
    },
    /// A continuation config has reserved flag bits set, an alt below 1, or
    /// a context id past the context table.
    InvalidContinuationConfig {
        /// The mode index.
        mode: usize,
        /// The continuation index.
        index: usize,
    },
    /// A serialized action has a `behind` value other than
    /// [`BEHIND_POSITION_INDEPENDENT`].
    InvalidBehind {
        /// The mode index.
        mode: usize,
        /// The continuation index.
        index: usize,
        /// The `behind` value.
        value: u32,
    },
}

impl fmt::Display for StaticLexerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            StaticLexerError::BadMagic { found } => {
                write!(
                    f,
                    "bad magic 0x{found:08x}; expected 0x{STATIC_DFA_MAGIC:08x}"
                )
            }
            StaticLexerError::TotalLengthMismatch { declared, actual } => {
                write!(
                    f,
                    "declared total length {declared} does not match stream length {actual}"
                )
            }
            StaticLexerError::TruncatedStream {
                expected_at_least,
                actual,
            } => {
                write!(
                    f,
                    "stream has {actual} words; expected at least {expected_at_least}"
                )
            }
            StaticLexerError::TrailingWords { consumed, actual } => {
                write!(
                    f,
                    "mode sections consumed {consumed} words of the declared {actual}"
                )
            }
            StaticLexerError::InvalidModeFlag { mode, value } => {
                write!(f, "mode {mode} present flag is {value}; expected 0 or 1")
            }
            StaticLexerError::InvalidStateCount { mode, count } => {
                write!(
                    f,
                    "mode {mode} has {count} states; expected 1..={MAX_STATE_COUNT}"
                )
            }
            StaticLexerError::IndexOutOfBounds {
                mode,
                what,
                index,
                count,
            } => {
                write!(f, "mode {mode}: {what} index {index} outside 0..{count}")
            }
            StaticLexerError::InvalidEdgeTarget { mode, entry } => {
                write!(f, "mode {mode}: invalid edge entry 0x{entry:04x}")
            }
            StaticLexerError::InconsistentAcceptBit { mode, entry } => {
                write!(
                    f,
                    "mode {mode}: edge entry 0x{entry:04x} accept bit does not match \
                     the target state's accept type"
                )
            }
            StaticLexerError::InvalidAcceptType { mode, state, value } => {
                write!(
                    f,
                    "mode {mode} state {state}: accept type {value}; expected -1 or a token type"
                )
            }
            StaticLexerError::InvalidClassStarts { mode } => {
                write!(f, "mode {mode}: invalid class starts")
            }
            StaticLexerError::InvalidRowOffsets { mode } => {
                write!(f, "mode {mode}: invalid class row offsets")
            }
            StaticLexerError::InvalidClassRange { mode, row } => {
                write!(f, "mode {mode} hi row {row}: invalid class range")
            }
            StaticLexerError::InvalidActionList { mode, state } => {
                write!(
                    f,
                    "mode {mode} state {state}: accept action list is not decodable"
                )
            }
            StaticLexerError::InvalidActionType { mode, state, value } => {
                write!(
                    f,
                    "mode {mode} state {state}: invalid or position-dependent action type {value}"
                )
            }
            StaticLexerError::SectionLengthMismatch {
                mode,
                section,
                expected,
                found,
            } => {
                write!(
                    f,
                    "mode {mode}: {section} section declared {expected} words; read {found}"
                )
            }
            StaticLexerError::InvalidAccelTag { mode, value } => {
                write!(f, "mode {mode}: unknown accel tag {value}")
            }
            StaticLexerError::InvalidAccelBitmap { mode } => {
                write!(f, "mode {mode}: positive accel bitmap contains '\\n'")
            }
            StaticLexerError::InvalidAccelNeedles { mode } => {
                write!(f, "mode {mode}: invalid exclusion accel needles")
            }
            StaticLexerError::InvalidEscapeRow { mode, state } => {
                write!(f, "mode {mode} state {state}: invalid escape row")
            }
            StaticLexerError::InconsistentEscapeRow {
                mode,
                state,
                symbol,
            } => {
                write!(
                    f,
                    "mode {mode} state {state}: escape row does not match the edge entry at symbol {symbol}"
                )
            }
            StaticLexerError::UnknownContinuationKind { mode, index, kind } => {
                write!(f, "mode {mode} continuation {index}: unknown kind {kind}")
            }
            StaticLexerError::InvalidContextKind { mode, index, value } => {
                write!(
                    f,
                    "mode {mode} continuation {index}: context kind {value}; expected 0 (singleton)"
                )
            }
            StaticLexerError::NonTopologicalContext { mode, index } => {
                write!(
                    f,
                    "mode {mode} continuation {index}: context table is not topological"
                )
            }
            StaticLexerError::InvalidContinuationConfig { mode, index } => {
                write!(f, "mode {mode} continuation {index}: invalid config")
            }
            StaticLexerError::InvalidBehind { mode, index, value } => {
                write!(
                    f,
                    "mode {mode} continuation {index}: behind value {value}; expected {BEHIND_POSITION_INDEPENDENT}"
                )
            }
        }
    }
}

impl std::error::Error for StaticLexerError {}

impl StaticLexerTables {
    /// Decodes a static lexer DFA word stream, with full validation: magic,
    /// total length, section lengths, every edge target below the state
    /// count or a sentinel, row indices in bounds, continuation indices in
    /// bounds, and decodable action triples. After a successful load the
    /// walker indexes the tables without bounds checks.
    ///
    /// Panics on a format-version mismatch: the words are generated
    /// together with the code that embeds them, so a version mismatch is a
    /// build inconsistency, not a runtime condition. Regenerate the tables
    /// with the matching tool version.
    pub fn from_words(words: &[u32]) -> Result<Self, StaticLexerError> {
        let mut r = Reader { words, pos: 0 };
        if words.len() < 4 {
            return Err(StaticLexerError::TruncatedStream {
                expected_at_least: 4,
                actual: words.len(),
            });
        }
        let magic = r.next()?;
        if magic != STATIC_DFA_MAGIC {
            return Err(StaticLexerError::BadMagic { found: magic });
        }
        let version = r.next()?;
        assert_eq!(
            version, STATIC_DFA_FORMAT_VERSION,
            "static lexer DFA table format version mismatch (found {version}, expected \
             {STATIC_DFA_FORMAT_VERSION}): the table was generated with a different tool \
             version; regenerate the lexer tables"
        );
        let total = r.next()? as usize;
        if total != words.len() {
            return Err(StaticLexerError::TotalLengthMismatch {
                declared: total,
                actual: words.len(),
            });
        }
        let num_modes = r.next()? as usize;
        let mut modes = Vec::with_capacity(num_modes);
        for mode in 0..num_modes {
            modes.push(read_mode(&mut r, mode)?);
        }
        if r.pos != words.len() {
            return Err(StaticLexerError::TrailingWords {
                consumed: r.pos,
                actual: words.len(),
            });
        }
        Ok(StaticLexerTables { modes })
    }

    /// The table for lexer mode `m`, if the builder compiled one.
    #[inline]
    pub fn mode(&self, m: usize) -> Option<&StaticLexerDFA> {
        self.modes.get(m).and_then(Option::as_ref)
    }

    /// Number of lexer modes in the stream, including modes without a
    /// table.
    pub fn num_modes(&self) -> usize {
        self.modes.len()
    }

    /// Assembles the tables from per-mode build results: the builder side
    /// of the type ([`Self::from_words`] is the wire side).
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn from_modes(modes: Vec<Option<StaticLexerDFA>>) -> Self {
        StaticLexerTables { modes }
    }

    /// The inverse of `from_modes`: unpacks into the per-mode tables.
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn into_modes(self) -> Vec<Option<StaticLexerDFA>> {
        self.modes
    }

    /// Encodes the tables as a word stream: the exact inverse of
    /// [`Self::from_words`]. Generated code embeds the words.
    ///
    /// (Feature `static-dfa-build`: only the builder produces tables.)
    #[cfg(feature = "static-dfa-build")]
    pub fn to_words(&self) -> Vec<u32> {
        let mut out = vec![
            STATIC_DFA_MAGIC,
            STATIC_DFA_FORMAT_VERSION,
            0,
            self.modes.len() as u32,
        ];
        for mode in &self.modes {
            match mode {
                None => out.push(0),
                Some(dfa) => {
                    out.push(1);
                    dfa.write_mode(&mut out);
                }
            }
        }
        out[2] = out.len() as u32;
        out
    }

    /// Checks the ATN-relative references of the continuation pool against
    /// `atn`: config states, context return states, and action indices.
    ///
    /// Panics on a mismatch: the tables and the ATN come from the same
    /// generated lexer, so a mismatch means the two are out of sync and the
    /// lexer must be regenerated.
    pub(crate) fn validate_for_atn(&self, atn: &ATN) {
        let states = atn.states_count();
        let actions = atn.lexer_actions.len();
        for (m, mode) in self.modes.iter().enumerate() {
            let Some(mode) = mode else { continue };
            for cont in &mode.continuations {
                for &(_, return_state) in &cont.ctxs {
                    assert!(
                        (return_state as usize) < states,
                        "static lexer DFA table of mode {m} references ATN state {return_state}, \
                         but the ATN has {states} states; regenerate the lexer tables"
                    );
                }
                for config in &cont.configs {
                    assert!(
                        (config.state as usize) < states,
                        "static lexer DFA table of mode {m} references ATN state {}, \
                         but the ATN has {states} states; regenerate the lexer tables",
                        config.state
                    );
                    for &action in &config.actions {
                        assert!(
                            (action as usize) < actions,
                            "static lexer DFA table of mode {m} references lexer action {action}, \
                             but the ATN has {actions} lexer actions; regenerate the lexer tables"
                        );
                    }
                }
            }
        }
    }
}

// The accessors the walker's hot path shares with builders, tests, and
// codegen come in cfg pairs: `pub` under the `static-dfa-build` feature,
// `pub(crate)` without it. (A macro cannot share the body: `self` does not
// cross macro boundaries.)
impl StaticLexerDFA {
    /// The state the walk of a token starts in.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn start_state(&self) -> u16 {
        self.start_state
    }

    /// The state the walk of a token starts in.
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn start_state(&self) -> u16 {
        self.start_state
    }

    /// Raw edge entry of state `s` on input symbol `t` (`-1` = EOF):
    /// `NO_EDGE` when there is no edge, `ESCAPE` for an escape edge, else
    /// `target | ACCEPT_BIT?` (the bit marks accept states, so the per-char
    /// walk loop needs only this one table load per input symbol).
    ///
    /// No bounds checks apart from the class-map length: every `s` the
    /// walker produces is a valid state id (the start state and all
    /// serialized targets are validated at load time), `row_of` entries are
    /// validated in range at load time, and class ids are smaller than
    /// `num_classes` by construction.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn edge_entry(&self, s: u16, t: i32) -> u16 {
        // Column 0 is EOF (`t = -1` wraps to 0), columns 1..=128 are symbols
        // 0..=127; everything else takes the class-map path.
        let col = (t as u32).wrapping_add(1) as usize;
        if col < LOW_ROW {
            return unsafe { *self.low.get_unchecked(s as usize * LOW_ROW + col) };
        }
        // Two direct loads: symbol to class, class to target. The length
        // check also covers modes without hi edges (an empty class map) and
        // symbols beyond MAX_SYMBOL.
        let i = t as usize - ASCII_ROW;
        if i >= self.class_of.len() {
            return NO_EDGE;
        }
        unsafe {
            let class = *self.class_of.get_unchecked(i) as usize;
            let row = *self.row_of.get_unchecked(s as usize) as usize;
            *self.hi_rows.get_unchecked(row * self.num_classes + class)
        }
    }

    /// Raw edge entry of state `s` on input symbol `t` (`-1` = EOF):
    /// `NO_EDGE` when there is no edge, `ESCAPE` for an escape edge, else
    /// `target | ACCEPT_BIT?` (the bit marks accept states, so the per-char
    /// walk loop needs only this one table load per input symbol).
    ///
    /// No bounds checks apart from the class-map length: every `s` the
    /// walker produces is a valid state id (the start state and all
    /// serialized targets are validated at load time), `row_of` entries are
    /// validated in range at load time, and class ids are smaller than
    /// `num_classes` by construction.
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn edge_entry(&self, s: u16, t: i32) -> u16 {
        // Column 0 is EOF (`t = -1` wraps to 0), columns 1..=128 are symbols
        // 0..=127; everything else takes the class-map path.
        let col = (t as u32).wrapping_add(1) as usize;
        if col < LOW_ROW {
            return unsafe { *self.low.get_unchecked(s as usize * LOW_ROW + col) };
        }
        // Two direct loads: symbol to class, class to target. The length
        // check also covers modes without hi edges (an empty class map) and
        // symbols beyond MAX_SYMBOL.
        let i = t as usize - ASCII_ROW;
        if i >= self.class_of.len() {
            return NO_EDGE;
        }
        unsafe {
            let class = *self.class_of.get_unchecked(i) as usize;
            let row = *self.row_of.get_unchecked(s as usize) as usize;
            *self.hi_rows.get_unchecked(row * self.num_classes + class)
        }
    }

    /// Token type produced when accepting in state `s`; -1 = not an accept
    /// state.
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn accept_type(&self, s: u16) -> i32 {
        unsafe { *self.accept_type.get_unchecked(s as usize) }
    }

    /// Token type produced when accepting in state `s`; -1 = not an accept
    /// state.
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn accept_type(&self, s: u16) -> i32 {
        unsafe { *self.accept_type.get_unchecked(s as usize) }
    }

    /// The position-independent lexer actions to execute when accepting in
    /// state `s` (skip, mode, channel, type, ...), in order; empty slice
    /// when none.
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[inline]
    pub(crate) fn accept_actions(&self, s: u16) -> &[LexerAction<'static>] {
        let (off, len) = unsafe { *self.accept_actions.get_unchecked(s as usize) };
        &self.actions[off as usize..(off + len) as usize]
    }

    /// Run-acceleration flavor tag of state `s`: 0 = none, else an index
    /// + 1 into the payload table (see [`StaticLexerDFA::accel_payload`]).
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[inline]
    pub(crate) fn accel_tag(&self, s: u16) -> u8 {
        unsafe { *self.accel_tag.get_unchecked(s as usize) }
    }

    /// Run-acceleration payload for a nonzero [`StaticLexerDFA::accel_tag`].
    ///
    /// Unchecked-index safety: tags are written only at load, bounded by
    /// the payload table length.
    #[inline]
    pub(crate) fn accel_payload(&self, tag: u8) -> Accel {
        unsafe { *self.accel_data.get_unchecked(tag as usize - 1) }
    }

    /// The continuation index of the escape edge of state `s` on `symbol`
    /// (EOF = -1 takes the EOF escape). `NO_CONTINUATION` when the edge
    /// does not escape or the escape is a budget escape.
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[cfg(feature = "static-dfa-build")]
    pub fn escape_continuation(&self, s: u16, symbol: i32) -> u32 {
        if symbol == EOF {
            return unsafe { *self.eof_escape.get_unchecked(s as usize) };
        }
        // Valid tables have a range for every ESCAPE entry (checked at
        // load); the restart-on-budget-escape path treats a miss as a
        // budget escape, which stays correct.
        self.escape_range(s, symbol)
            .map_or(NO_CONTINUATION, |r| r.continuation)
    }

    /// The continuation index of the escape edge of state `s` on `symbol`
    /// (EOF = -1 takes the EOF escape). `NO_CONTINUATION` when the edge
    /// does not escape or the escape is a budget escape.
    ///
    /// Same unchecked-index safety argument as [`StaticLexerDFA::edge_entry`].
    #[cfg(not(feature = "static-dfa-build"))]
    pub(crate) fn escape_continuation(&self, s: u16, symbol: i32) -> u32 {
        if symbol == EOF {
            return unsafe { *self.eof_escape.get_unchecked(s as usize) };
        }
        // Valid tables have a range for every ESCAPE entry (checked at
        // load); the restart-on-budget-escape path treats a miss as a
        // budget escape, which stays correct.
        self.escape_range(s, symbol)
            .map_or(NO_CONTINUATION, |r| r.continuation)
    }

    /// The escape range of state `s` covering `symbol`, if any. The
    /// continuation of the range can be `NO_CONTINUATION` (a budget
    /// escape): coverage and continuation value are separate facts.
    fn escape_range(&self, s: u16, symbol: i32) -> Option<&EscapeRange> {
        let row = unsafe {
            let lo = *self.escape_off.get_unchecked(s as usize) as usize;
            let hi = *self.escape_off.get_unchecked(s as usize + 1) as usize;
            self.escape_ranges.get_unchecked(lo..hi)
        };
        let found = row.binary_search_by(|r| {
            if symbol < r.lo {
                std::cmp::Ordering::Greater
            } else if symbol > r.hi {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        found.ok().map(|i| &row[i])
    }

    /// The continuation at `index` in the pool.
    ///
    /// Unchecked-index safety: continuation indices in escape rows are
    /// validated against the pool length at load time.
    pub(crate) fn continuation(&self, index: u32) -> &LexerContinuation {
        unsafe { self.continuations.get_unchecked(index as usize) }
    }
}

/// Structural inspection of a committed table, for tests and future
/// codegen. The walker uses the hot-path accessors above; nothing here is
/// on it.
#[cfg(feature = "static-dfa-build")]
impl StaticLexerDFA {
    /// Number of table states of this mode.
    pub fn num_states(&self) -> usize {
        self.accept_type.len()
    }

    /// Number of hi-symbol classes of this mode (0 = no edges past 127).
    pub fn num_classes(&self) -> usize {
        self.num_classes
    }

    /// Number of continuations of this mode.
    pub fn num_continuations(&self) -> usize {
        self.continuations.len()
    }

    /// The kind of continuation `index`: `CONTINUATION_MOVED_CONFIGS` or
    /// `CONTINUATION_SOURCE_SET`.
    pub fn continuation_kind(&self, index: u32) -> u32 {
        self.continuations[index as usize].kind
    }

    /// The number of configs of continuation `index`.
    pub fn continuation_num_configs(&self, index: u32) -> usize {
        self.continuations[index as usize].configs.len()
    }

    /// The accept action list of state `s` as ATN `(type, data1, data2)`
    /// triples, in order.
    pub fn accept_action_triples(&self, s: u16) -> Vec<(i32, i32, i32)> {
        self.accept_actions(s)
            .iter()
            .map(LexerAction::to_serialized)
            .collect()
    }

    /// The escape ranges of state `s` as `(lo, hi, continuation)` triples,
    /// sorted and disjoint (`NO_CONTINUATION` marks a budget escape).
    pub fn escape_ranges(&self, s: u16) -> Vec<(i32, i32, u32)> {
        let row = &self.escape_ranges
            [self.escape_off[s as usize] as usize..self.escape_off[s as usize + 1] as usize];
        row.iter().map(|r| (r.lo, r.hi, r.continuation)).collect()
    }
}

/// The encode side of the word-stream codec: [`StaticLexerTables::to_words`]
/// and its per-section writers. The section order is the one of the module
/// documentation; the output is the exact inverse of [`StaticLexerTables::from_words`].
#[cfg(feature = "static-dfa-build")]
impl StaticLexerDFA {
    /// Writes one present mode section.
    fn write_mode(&self, out: &mut Vec<u32>) {
        let num_states = self.accept_type.len();
        out.push(num_states as u32);
        out.push(u32::from(self.start_state));
        out.extend(self.accept_type.iter().map(|&t| t as u32));

        // Accept action lists, deduplicated by content in state order.
        let mut action_lists: Vec<u32> = Vec::new();
        let mut list_ids: HashMap<&[LexerAction], u32> = HashMap::new();
        let mut offsets = Vec::with_capacity(num_states);
        for &(off, len) in &self.accept_actions {
            if len == 0 {
                offsets.push(-1i32);
                continue;
            }
            let list = &self.actions[off as usize..(off + len) as usize];
            let offset = match list_ids.get(list) {
                Some(&offset) => offset,
                None => {
                    let offset = action_lists.len() as u32;
                    action_lists.push(len);
                    for action in list {
                        let (ty, d1, d2) = action.to_serialized();
                        action_lists.extend([ty as u32, d1 as u32, d2 as u32]);
                    }
                    list_ids.insert(list, offset);
                    offset
                }
            };
            offsets.push(offset as i32);
        }
        out.extend(offsets.iter().map(|&o| o as u32));

        for s in 0..num_states {
            out.push(u32::from(self.low[s * LOW_ROW]));
        }
        for s in 0..num_states {
            out.extend(
                self.low[s * LOW_ROW + 1..(s + 1) * LOW_ROW]
                    .iter()
                    .map(|&entry| u32::from(entry)),
            );
        }

        self.write_hi_classes(out);

        out.push(action_lists.len() as u32);
        out.extend(action_lists);

        let mut accel = Vec::new();
        for (s, &tag) in self.accel_tag.iter().enumerate() {
            if tag != 0 {
                accel.extend(self.accel_data[tag as usize - 1].to_serialized(s as u16));
            }
        }
        out.push(accel.len() as u32);
        out.extend(accel);

        for s in 0..num_states {
            out.push(self.eof_escape[s]);
            let ranges =
                &self.escape_ranges[self.escape_off[s] as usize..self.escape_off[s + 1] as usize];
            out.push(ranges.len() as u32);
            for range in ranges {
                out.extend([range.lo as u32, range.hi as u32, range.continuation]);
            }
        }

        out.push(self.continuations.len() as u32);
        for cont in &self.continuations {
            out.push(cont.kind);
            out.push(cont.ctxs.len() as u32);
            for &(parent, return_state) in &cont.ctxs {
                out.extend([0, parent, return_state as u32]);
            }
            out.push(cont.configs.len() as u32);
            for config in &cont.configs {
                let flags =
                    u32::from(config.non_greedy) | (u32::from(config.alt_reached_accept) << 1);
                out.extend([
                    config.state as u32,
                    flags,
                    config.alt as u32,
                    config.ctx,
                    config.actions.len() as u32,
                ]);
                for &action in &config.actions {
                    out.extend([action, BEHIND_POSITION_INDEPENDENT]);
                }
            }
        }
    }

    /// Writes the class-compressed hi-edge section: the deduplicated dense
    /// rows are re-expressed as `(lo_class, hi_class, target)` runs.
    fn write_hi_classes(&self, out: &mut Vec<u32>) {
        out.push(self.num_classes as u32);
        if self.num_classes == 0 {
            return;
        }
        out.extend(self.class_starts.iter().copied());
        let num_rows = self.hi_rows.len() / self.num_classes;
        let row_runs: Vec<Vec<(u32, u32, u16)>> = (0..num_rows)
            .map(|row| {
                dense_row_runs(&self.hi_rows[row * self.num_classes..(row + 1) * self.num_classes])
            })
            .collect();
        out.push(num_rows as u32);
        out.push(row_runs.iter().map(Vec::len).sum::<usize>() as u32);
        let mut offset = 0;
        for runs in &row_runs {
            out.push(offset);
            offset += runs.len() as u32;
        }
        out.push(offset);
        for runs in &row_runs {
            for &(lo, hi, entry) in runs {
                out.extend([lo, hi, u32::from(entry)]);
            }
        }
        out.extend(self.row_of.iter().copied());
    }
}

/// The `(lo_class, hi_class, entry)` runs of a dense class row: maximal
/// spans of equal entries; `NO_EDGE` gaps have no run.
#[cfg(feature = "static-dfa-build")]
fn dense_row_runs(dense: &[u16]) -> Vec<(u32, u32, u16)> {
    let mut runs = Vec::new();
    let mut c = 0;
    while c < dense.len() {
        let entry = dense[c];
        if entry == NO_EDGE {
            c += 1;
            continue;
        }
        let start = c;
        c += 1;
        while c < dense.len() && dense[c] == entry {
            c += 1;
        }
        runs.push((start as u32, (c - 1) as u32, entry));
    }
    runs
}

/// The direct symbol-to-class map for the symbols `ASCII_ROW..=MAX_SYMBOL`:
/// class `i` covers `class_starts[i]..class_starts[i + 1]`, or up to
/// `MAX_SYMBOL` for the last class. Both producers of [`StaticLexerDFA`]
/// (the loader and the build-time commit) derive the map from the class
/// starts with this one function.
pub(crate) fn class_map(class_starts: &[u32]) -> Vec<u16> {
    let mut class_of = vec![0u16; (MAX_SYMBOL - ASCII_ROW as u32 + 1) as usize];
    for (c, &start) in class_starts.iter().enumerate() {
        let from = (start - ASCII_ROW as u32) as usize;
        let to = if c + 1 < class_starts.len() {
            (class_starts[c + 1] - ASCII_ROW as u32) as usize
        } else {
            class_of.len()
        };
        class_of[from..to].fill(c as u16);
    }
    class_of
}

/// Expands per-row `(lo_class, hi_class, target)` runs into dense
/// `num_classes`-wide rows, `NO_EDGE`-filled. Both producers of
/// [`StaticLexerDFA`] (the loader and the build-time commit) expand rows
/// with this one function.
pub(crate) fn dense_hi_rows(row_runs: &[Vec<(u32, u32, u16)>], num_classes: usize) -> Vec<u16> {
    let mut hi_rows = vec![NO_EDGE; row_runs.len() * num_classes];
    for (row, runs) in row_runs.iter().enumerate() {
        for &(lo, hi, target) in runs {
            hi_rows[row * num_classes + lo as usize..=row * num_classes + hi as usize].fill(target);
        }
    }
    hi_rows
}

/// Cursor over the word stream with bounds-checked reads.
struct Reader<'a> {
    words: &'a [u32],
    pos: usize,
}

impl Reader<'_> {
    fn next(&mut self) -> Result<u32, StaticLexerError> {
        let w = self
            .words
            .get(self.pos)
            .ok_or(StaticLexerError::TruncatedStream {
                expected_at_least: self.pos + 1,
                actual: self.words.len(),
            })?;
        self.pos += 1;
        Ok(*w)
    }
}

fn check_index(
    mode: usize,
    what: &'static str,
    index: u32,
    count: usize,
) -> Result<(), StaticLexerError> {
    if index as usize >= count {
        return Err(StaticLexerError::IndexOutOfBounds {
            mode,
            what,
            index,
            count,
        });
    }
    Ok(())
}

/// Validates one raw edge entry word and passes the entry through: the
/// word must fit a u16, a non-sentinel target must be a valid state id,
/// and the entry's accept bit must match the target's accept type.
fn check_edge(
    mode: usize,
    word: u32,
    num_states: usize,
    accept_type: &[i32],
) -> Result<u16, StaticLexerError> {
    if word > u16::MAX as u32 {
        return Err(StaticLexerError::InvalidEdgeTarget { mode, entry: word });
    }
    let entry = word as u16;
    if entry == NO_EDGE || entry == ESCAPE {
        return Ok(entry);
    }
    let target = (entry & STATE_MASK) as usize;
    if target >= num_states {
        return Err(StaticLexerError::InvalidEdgeTarget { mode, entry: word });
    }
    if (entry & ACCEPT_BIT != 0) != (accept_type[target] >= 0) {
        return Err(StaticLexerError::InconsistentAcceptBit { mode, entry });
    }
    Ok(entry)
}

/// Reads and validates one mode section; `None` when the mode has no table.
fn read_mode(r: &mut Reader, mode: usize) -> Result<Option<StaticLexerDFA>, StaticLexerError> {
    let present = r.next()?;
    if present == 0 {
        return Ok(None);
    }
    if present != 1 {
        return Err(StaticLexerError::InvalidModeFlag {
            mode,
            value: present,
        });
    }
    let num_states = r.next()? as usize;
    if num_states == 0 || num_states > MAX_STATE_COUNT {
        return Err(StaticLexerError::InvalidStateCount {
            mode,
            count: num_states,
        });
    }
    let start_state = r.next()?;
    check_index(mode, "start state", start_state, num_states)?;
    let mut accept_type = Vec::with_capacity(num_states);
    for s in 0..num_states {
        let value = r.next()? as i32;
        if value < -1 {
            return Err(StaticLexerError::InvalidAcceptType {
                mode,
                state: s,
                value,
            });
        }
        accept_type.push(value);
    }
    let mut accept_action_offsets = Vec::with_capacity(num_states);
    for _ in 0..num_states {
        accept_action_offsets.push(r.next()? as i32);
    }
    let mut low = vec![NO_EDGE; num_states * LOW_ROW];
    for s in 0..num_states {
        low[s * LOW_ROW] = check_edge(mode, r.next()?, num_states, &accept_type)?;
    }
    for s in 0..num_states {
        for c in 1..LOW_ROW {
            low[s * LOW_ROW + c] = check_edge(mode, r.next()?, num_states, &accept_type)?;
        }
    }
    let hi = read_hi_classes(r, mode, num_states, &accept_type)?;
    let action_lists = read_action_lists(r, mode, &accept_action_offsets)?;
    let (accel_tag, accel_data) = read_accel(r, mode, num_states)?;
    let (eof_escape, escape_off, escape_ranges) = read_escape_rows(r, mode, num_states)?;
    let continuations = read_continuations(r, mode)?;

    let table = StaticLexerDFA {
        start_state: start_state as u16,
        low,
        class_of: hi.class_of,
        hi_rows: hi.hi_rows,
        row_of: hi.row_of,
        num_classes: hi.num_classes,
        class_starts: hi.class_starts,
        accept_type,
        accept_actions: action_lists.accept_actions,
        actions: action_lists.actions,
        accel_tag,
        accel_data,
        eof_escape,
        escape_off,
        escape_ranges,
        continuations,
    };
    table.validate_escapes(mode)?;
    Ok(Some(table))
}

/// The expanded hi-edge data of one mode.
struct HiClasses {
    class_of: Vec<u16>,
    class_starts: Vec<u32>,
    hi_rows: Vec<u16>,
    row_of: Vec<u32>,
    num_classes: usize,
}

/// The decoded action-list section of one mode.
struct ActionLists {
    actions: Vec<LexerAction<'static>>,
    accept_actions: Vec<(u32, u32)>,
}

/// Reads the class-compressed hi-edge section of one mode: the class
/// starts, then the deduplicated class rows (as `(lo_class, hi_class,
/// target)` runs) and the per-state row index. Expands the direct
/// symbol-to-class map and the dense class rows.
fn read_hi_classes(
    r: &mut Reader,
    mode: usize,
    num_states: usize,
    accept_type: &[i32],
) -> Result<HiClasses, StaticLexerError> {
    let num_classes = r.next()? as usize;
    if num_classes == 0 {
        return Ok(HiClasses {
            class_of: Vec::new(),
            class_starts: Vec::new(),
            hi_rows: Vec::new(),
            row_of: Vec::new(),
            num_classes: 0,
        });
    }
    // Class ids are u16 in the expanded map.
    if num_classes > u16::MAX as usize + 1 {
        return Err(StaticLexerError::InvalidClassStarts { mode });
    }
    let mut class_starts = Vec::with_capacity(num_classes);
    for _ in 0..num_classes {
        class_starts.push(r.next()?);
    }
    if class_starts[0] != ASCII_ROW as u32
        || !class_starts.windows(2).all(|w| w[0] < w[1])
        || *class_starts.last().unwrap() > MAX_SYMBOL
    {
        return Err(StaticLexerError::InvalidClassStarts { mode });
    }

    // Class i covers [starts[i], starts[i+1] or MAX_SYMBOL + 1).
    let class_of = class_map(&class_starts);

    let num_rows = r.next()? as usize;
    let num_row_triples = r.next()? as usize;
    let mut row_offsets = Vec::with_capacity(num_rows + 1);
    for _ in 0..=num_rows {
        row_offsets.push(r.next()? as usize);
    }
    if row_offsets[0] != 0
        || row_offsets[num_rows] != num_row_triples
        || !row_offsets.windows(2).all(|w| w[0] <= w[1])
    {
        return Err(StaticLexerError::InvalidRowOffsets { mode });
    }
    let mut row_runs = Vec::with_capacity(num_rows);
    for row in 0..num_rows {
        let mut runs = Vec::with_capacity(row_offsets[row + 1] - row_offsets[row]);
        let mut prev_hi = None;
        for _ in row_offsets[row]..row_offsets[row + 1] {
            let lo = r.next()?;
            let hi = r.next()?;
            let target = r.next()?;
            if lo > hi || hi as usize >= num_classes || prev_hi.is_some_and(|p: u32| lo <= p) {
                return Err(StaticLexerError::InvalidClassRange { mode, row });
            }
            prev_hi = Some(hi);
            runs.push((lo, hi, check_edge(mode, target, num_states, accept_type)?));
        }
        row_runs.push(runs);
    }
    let hi_rows = dense_hi_rows(&row_runs, num_classes);
    let mut row_of = Vec::with_capacity(num_states);
    for _ in 0..num_states {
        let row = r.next()?;
        check_index(mode, "hi row", row, num_rows)?;
        row_of.push(row);
    }
    Ok(HiClasses {
        class_of,
        class_starts,
        hi_rows,
        row_of,
        num_classes,
    })
}

/// Reads the action-list section of one mode and decodes the accept action
/// list of every state into the flat `actions` table, deduplicated by
/// content (the same invariant the build-time commit produces). Position-
/// dependent actions cannot appear in accept lists (the accept execution
/// has no match offset for them), so a custom action is a format error
/// here.
fn read_action_lists(
    r: &mut Reader,
    mode: usize,
    accept_action_offsets: &[i32],
) -> Result<ActionLists, StaticLexerError> {
    let num_ints = r.next()? as usize;
    let mut words = Vec::with_capacity(num_ints);
    for _ in 0..num_ints {
        words.push(r.next()?);
    }
    let mut actions = Vec::new();
    let mut list_ids: HashMap<&[u32], (u32, u32)> = HashMap::new();
    let mut lists = Vec::with_capacity(accept_action_offsets.len());
    for (s, &offset) in accept_action_offsets.iter().enumerate() {
        if offset < 0 {
            lists.push((0, 0));
            continue;
        }
        let offset = offset as usize;
        let Some(&len) = words.get(offset) else {
            return Err(StaticLexerError::InvalidActionList { mode, state: s });
        };
        let len = len as usize;
        let triples = len
            .checked_mul(3)
            .and_then(|n| words.get(offset + 1..offset + 1 + n))
            .ok_or(StaticLexerError::InvalidActionList { mode, state: s })?;
        if let Some(&list) = list_ids.get(triples) {
            lists.push(list);
            continue;
        }
        let start = actions.len() as u32;
        for &[ty, d1, d2] in triples.as_chunks::<3>().0 {
            let (ty, d1, d2) = (ty as i32, d1 as i32, d2 as i32);
            if !(0..=7).contains(&ty) || ty == LEXER_ACTION_TYPE_CUSTOM {
                return Err(StaticLexerError::InvalidActionType {
                    mode,
                    state: s,
                    value: ty,
                });
            }
            actions.push(LexerAction::from_serialized(ty, d1, d2));
        }
        let list = (start, len as u32);
        list_ids.insert(triples, list);
        lists.push(list);
    }
    Ok(ActionLists {
        actions,
        accept_actions: lists,
    })
}

/// Reads the run-acceleration section of one mode: a flattened list of
/// fixed-stride entries, `(state, ACCEL_POSITIVE, w0..w3)` or `(state,
/// ACCEL_EXCLUSION, nl_in_set, n_needles, n0..n2)`, expanded into one
/// descriptor per state.
fn read_accel(
    r: &mut Reader,
    mode: usize,
    num_states: usize,
) -> Result<(Vec<u8>, Vec<Accel>), StaticLexerError> {
    let num_ints = r.next()? as usize;
    let mut accel_tag = vec![0u8; num_states];
    let mut accel_data = Vec::new();
    let mut read = 0;
    while read < num_ints {
        let s = r.next()?;
        check_index(mode, "accel state", s, num_states)?;
        let payload = match r.next()? {
            ACCEL_POSITIVE => {
                let mut bitmap = 0u128;
                for limb in 0..4 {
                    bitmap |= (r.next()? as u128) << (limb * 32);
                }
                if bitmap & (1 << b'\n') != 0 {
                    return Err(StaticLexerError::InvalidAccelBitmap { mode });
                }
                read += 6;
                Accel::Positive(bitmap)
            }
            ACCEL_EXCLUSION => {
                let nl_in_set = match r.next()? {
                    0 => false,
                    1 => true,
                    value => {
                        return Err(StaticLexerError::InvalidAccelTag { mode, value });
                    }
                };
                let n = r.next()?;
                let mut needle_words = [0; 3];
                for slot in needle_words.iter_mut() {
                    *slot = r.next()?;
                }
                if !(1..=3).contains(&n)
                    || needle_words[..n as usize].iter().any(|&w| w >= 0x80)
                    || !needle_words[..n as usize].contains(&(b'\n' as u32))
                {
                    return Err(StaticLexerError::InvalidAccelNeedles { mode });
                }
                read += 7;
                Accel::Exclusion {
                    needles: [
                        needle_words[0] as u8,
                        needle_words[1] as u8,
                        needle_words[2] as u8,
                    ],
                    n: n as u8,
                    nl_in_set,
                }
            }
            value => {
                return Err(StaticLexerError::InvalidAccelTag { mode, value });
            }
        };
        accel_data.push(payload);
        // The tag is the payload index + 1 in a u8.
        check_index(
            mode,
            "accel payload",
            accel_data.len() as u32,
            u8::MAX as usize + 1,
        )?;
        accel_tag[s as usize] = accel_data.len() as u8;
    }
    if read != num_ints {
        return Err(StaticLexerError::SectionLengthMismatch {
            mode,
            section: "accel",
            expected: num_ints,
            found: read,
        });
    }
    Ok((accel_tag, accel_data))
}

/// Reads the escape rows of one mode: per state, the EOF escape index,
/// then the sorted, disjoint symbol ranges whose edges escape.
#[allow(clippy::type_complexity)]
fn read_escape_rows(
    r: &mut Reader,
    mode: usize,
    num_states: usize,
) -> Result<(Vec<u32>, Vec<u32>, Vec<EscapeRange>), StaticLexerError> {
    let mut eof_escape = Vec::with_capacity(num_states);
    let mut escape_off = Vec::with_capacity(num_states + 1);
    let mut escape_ranges = Vec::new();
    escape_off.push(0u32);
    for s in 0..num_states {
        eof_escape.push(r.next()?);
        let num_ranges = r.next()? as usize;
        let mut prev_hi: i32 = -1;
        for _ in 0..num_ranges {
            let lo = r.next()? as i32;
            let hi = r.next()? as i32;
            let continuation = r.next()?;
            if lo < 0 || lo > hi || hi > MAX_SYMBOL as i32 || lo <= prev_hi {
                return Err(StaticLexerError::InvalidEscapeRow { mode, state: s });
            }
            prev_hi = hi;
            escape_ranges.push(EscapeRange {
                lo,
                hi,
                continuation,
            });
        }
        escape_off.push(escape_ranges.len() as u32);
    }
    Ok((eof_escape, escape_off, escape_ranges))
}

/// Reads the continuation pool of one mode.
fn read_continuations(
    r: &mut Reader,
    mode: usize,
) -> Result<Vec<LexerContinuation>, StaticLexerError> {
    let num_continuations = r.next()? as usize;
    let mut continuations = Vec::with_capacity(num_continuations);
    for index in 0..num_continuations {
        let kind = r.next()?;
        if kind != CONTINUATION_MOVED_CONFIGS && kind != CONTINUATION_SOURCE_SET {
            return Err(StaticLexerError::UnknownContinuationKind { mode, index, kind });
        }
        let nctx = r.next()? as usize;
        let mut ctxs = Vec::with_capacity(nctx);
        for i in 0..nctx {
            let ctx_kind = r.next()?;
            if ctx_kind != 0 {
                return Err(StaticLexerError::InvalidContextKind {
                    mode,
                    index,
                    value: ctx_kind,
                });
            }
            let parent = r.next()?;
            // Entry i has local id i + 1; parents refer to smaller ids.
            if parent as usize > i {
                return Err(StaticLexerError::NonTopologicalContext { mode, index });
            }
            let return_state = r.next()?;
            if return_state > i32::MAX as u32 {
                return Err(StaticLexerError::NonTopologicalContext { mode, index });
            }
            ctxs.push((parent, return_state as i32));
        }
        let nconfigs = r.next()? as usize;
        let mut configs = Vec::with_capacity(nconfigs);
        for _ in 0..nconfigs {
            let state = r.next()?;
            let flags = r.next()?;
            let alt = r.next()?;
            let ctx_local = r.next()?;
            if state > i32::MAX as u32
                || flags & !0b11 != 0
                || alt == 0
                || alt > i32::MAX as u32
                || ctx_local as usize > nctx
            {
                return Err(StaticLexerError::InvalidContinuationConfig { mode, index });
            }
            let nactions = r.next()? as usize;
            let mut actions = Vec::with_capacity(nactions);
            for _ in 0..nactions {
                let action = r.next()?;
                if action > i32::MAX as u32 {
                    return Err(StaticLexerError::InvalidContinuationConfig { mode, index });
                }
                let behind = r.next()?;
                if behind != BEHIND_POSITION_INDEPENDENT {
                    return Err(StaticLexerError::InvalidBehind {
                        mode,
                        index,
                        value: behind,
                    });
                }
                actions.push(action);
            }
            configs.push(ContinuationConfig {
                state: state as i32,
                alt: alt as i32,
                ctx: ctx_local,
                non_greedy: flags & 1 != 0,
                alt_reached_accept: flags & 2 != 0,
                actions,
            });
        }
        continuations.push(LexerContinuation {
            kind,
            ctxs,
            configs,
        });
    }
    Ok(continuations)
}

impl StaticLexerDFA {
    /// Cross-checks the escape rows against the edge entries: continuation
    /// indices in bounds, range endpoints on ESCAPE entries, every ESCAPE
    /// entry covered by a range (the EOF escape index covers the EOF edge),
    /// and the EOF escape index meaningful only on an ESCAPE EOF edge.
    fn validate_escapes(&self, mode: usize) -> Result<(), StaticLexerError> {
        let num_states = self.accept_type.len();
        let num_continuations = self.continuations.len();
        let check_continuation = |index: u32| {
            if index != NO_CONTINUATION {
                check_index(mode, "continuation", index, num_continuations)?;
            }
            Ok(())
        };
        for (s, &eof_escape) in self.eof_escape.iter().enumerate() {
            check_continuation(eof_escape)?;
            if eof_escape != NO_CONTINUATION && self.edge_entry(s as u16, EOF) != ESCAPE {
                return Err(StaticLexerError::InconsistentEscapeRow {
                    mode,
                    state: s,
                    symbol: EOF,
                });
            }
        }
        for range in &self.escape_ranges {
            check_continuation(range.continuation)?;
        }
        for s in 0..num_states {
            let lo = self.escape_off[s] as usize;
            let hi = self.escape_off[s + 1] as usize;
            for range in &self.escape_ranges[lo..hi] {
                for bound in [range.lo, range.hi] {
                    if self.edge_entry(s as u16, bound) != ESCAPE {
                        return Err(StaticLexerError::InconsistentEscapeRow {
                            mode,
                            state: s,
                            symbol: bound,
                        });
                    }
                }
            }
            // Every ESCAPE entry needs a covering range. The range's
            // continuation can be NO_CONTINUATION (a budget escape), so
            // check coverage, not the continuation value.
            for col in 1..LOW_ROW {
                let symbol = col as i32 - 1;
                if self.low[s * LOW_ROW + col] == ESCAPE
                    && self.escape_range(s as u16, symbol).is_none()
                {
                    return Err(StaticLexerError::InconsistentEscapeRow {
                        mode,
                        state: s,
                        symbol,
                    });
                }
            }
            if self.num_classes > 0 {
                let row = self.row_of[s] as usize;
                for (class, &start) in self.class_starts.iter().enumerate() {
                    if self.hi_rows[row * self.num_classes + class] == ESCAPE
                        && self.escape_range(s as u16, start as i32).is_none()
                    {
                        return Err(StaticLexerError::InconsistentEscapeRow {
                            mode,
                            state: s,
                            symbol: start as i32,
                        });
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "static-dfa-build"))]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;
    use crate::atn_deserializer::ATNDeserializer;
    use crate::static_dfa_build::StaticLexerDfaBuilder;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/static_lexer")
            .join(name)
    }

    fn gen_grammar(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("grammars")
            .join(name)
    }

    fn compile_tables(path: &Path) -> ATN {
        let units = dbt_antlr_codegen::atn_export::compile_atn_data(path, &[]).expect("compile");
        let words = units
            .iter()
            .find_map(|u| u.lexer_atn_words.clone())
            .expect("lexer unit");
        ATNDeserializer::new(None).deserialize(&mut words.iter())
    }

    /// The word-stream codec round-trips over every fixture grammar plus
    /// the generated-code witnesses (CSV, XMLLexer, SpikeLexer): decoding
    /// the encoded tables gives the tables back, and re-encoding a decoded
    /// stream reproduces it word for word.
    #[test]
    fn word_stream_round_trip() {
        let mut grammars: Vec<PathBuf> = [
            "CSV.g4",
            "CustomAction.g4",
            "EofChain.g4",
            "EofEscape.g4",
            "EofLexer.g4",
            "MiniModes.g4",
            "Spike.g4",
            "StartAction.g4",
            "StartPred.g4",
            "StartRecurse.g4",
            "XMLLexer.g4",
        ]
        .into_iter()
        .map(fixture)
        .collect();
        grammars.extend(
            ["CSV.g4", "XMLLexer.g4", "SpikeLexer.g4"]
                .into_iter()
                .map(gen_grammar),
        );

        for path in grammars {
            let atn = compile_tables(&path);
            for builder in [
                StaticLexerDfaBuilder::new(),
                // Budget escapes (NO_CONTINUATION) round-trip too.
                StaticLexerDfaBuilder::new().with_max_states_per_mode(3),
            ] {
                let build = builder.build(&atn);
                let name = path.display();
                let decoded = StaticLexerTables::from_words(&build.words)
                    .unwrap_or_else(|e| panic!("{name}: the builder's words decode: {e}"));
                assert_eq!(decoded.num_modes(), build.modes.len(), "{name}: mode count");
                for m in 0..build.modes.len() {
                    assert_eq!(
                        decoded.mode(m),
                        build.table(m),
                        "{name}: from_words(to_words(tables)) == tables (mode {m})"
                    );
                }
                assert_eq!(
                    decoded.to_words(),
                    build.words,
                    "{name}: to_words(from_words(words)) == words"
                );
            }
        }
    }

    /// The two producers of [`StaticLexerDFA`] agree: the committed tables
    /// and the decoded tables resolve every edge, accept, and escape the
    /// same way.
    #[test]
    fn commit_and_decode_agree() {
        for name in ["Spike.g4", "XMLLexer.g4", "EofEscape.g4"] {
            let atn = compile_tables(&fixture(name));
            let build = StaticLexerDfaBuilder::new().build(&atn);
            let decoded = StaticLexerTables::from_words(&build.words).expect("decode");
            for m in 0..build.modes.len() {
                let (Some(committed), Some(decoded)) = (build.table(m), decoded.mode(m)) else {
                    continue;
                };
                for s in 0..committed.num_states() as u16 {
                    for symbol in [EOF, 0, b'a' as i32, 127, 128, 0x1000, 0x10FFFF] {
                        assert_eq!(
                            committed.edge_entry(s, symbol),
                            decoded.edge_entry(s, symbol),
                            "{name} mode {m} state {s} symbol {symbol}"
                        );
                        assert_eq!(
                            committed.escape_continuation(s, symbol),
                            decoded.escape_continuation(s, symbol),
                            "{name} mode {m} state {s} symbol {symbol}"
                        );
                    }
                    assert_eq!(committed.accept_type(s), decoded.accept_type(s));
                    assert_eq!(
                        committed.accept_action_triples(s),
                        decoded.accept_action_triples(s)
                    );
                }
            }
        }
    }
}
