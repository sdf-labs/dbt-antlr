//! Runtime side of the static lexer DFA: the packed table image that the
//! walker in [`crate::lexer_atn_simulator::LexerATNSimulator`] tokenizes
//! from, and the word-stream codec that carries the tables from the
//! compile-time builder to generated code.
//!
//! The builder (the `static_dfa_build` module, feature `static-dfa-build`)
//! runs the subset construction and commits each mode to a
//! `StaticLexerDfaBuild`, whose `finalize` freezes those sections into the
//! packed image of a [`StaticLexerDFA`]. Every
//! [`StaticLexerDFA`] accessor is a view over that image: a header of section
//! offsets, then the sections, all addressed by byte offset from the start of
//! the image.
//!
//! The image holds no pointers and no `Vec`s. Every section is a table of
//! `u32` words, a table of `u16` entries packed two per word in little-endian
//! order (overlaid directly as `&[u16]` on little-endian targets and extracted
//! on the fly on big-endian targets), or a `#[repr(C)]` / `#[repr(C, u32)]`
//! record of `u32` words (including [`LexerAction`] and [`Accel`]), so a
//! `static` array of the image words is a valid table on every target that
//! compiles the generated code, and a [`StaticLexerDFA`] over such an array
//! allocates nothing.
//!
//! [`StaticLexerTables::to_words`] (same feature) encodes the tables as the
//! word stream that generated code embeds; [`StaticLexerTables::from_words`]
//! decodes the stream with full validation. After a successful load the
//! walker indexes the tables without bounds checks, and `to_words` is the
//! exact inverse of `from_words`.
//!
//! # Image layout
//!
//! A mode image starts with its `Header` and continues with the sections in
//! the order of the header fields. Offsets are byte offsets from the start of
//! the image and are multiples of 4. A `u16` section packs two entries per
//! word, the entry with the lower index in the low half of the word, and pads
//! an odd entry count with one zero entry. Every offset and count a section
//! needs is in the header.
//!
//! - `low`: `num_states * LOW_ROW` `u16` entries. Column 0 is the EOF edge,
//!   columns 1..=128 are the symbols `0..=127`.
//! - `class_starts`: `num_classes` `u32` entries. Class `i` covers the
//!   symbols `class_starts[i]..class_starts[i + 1]`, the last class up to
//!   `MAX_SYMBOL`. `class_starts[0]` is 128. Empty when the mode has no edges
//!   past 127.
//! - `hi_rows`: `num_hi_rows * num_classes` `u16` entries: the hi-edge entry
//!   of state `s` is the row `row_of[s]`, and rows are deduplicated by
//!   content.
//! - `row_of`: `num_states` `u32` entries (zero for every state when the mode
//!   has no hi edges).
//! - `accept_type`: `num_states` `i32` entries, `-1` for a state that is not
//!   an accept state.
//! - `accept_actions`: `num_states` `ActionSpan` records into `actions`,
//!   `(0, 0)` when the state has no accept actions.
//! - `actions`: `actions_len` [`LexerAction`] records (`#[repr(C, u32)]`, 4
//!   words each), overlaid directly as `&[LexerAction]`. Lists are
//!   deduplicated by content.
//! - `accel_tag`: `num_states` `u16` entries: 0 for a state without run
//!   acceleration, else the payload index in `accel_data` + 1.
//! - `accel_data`: `accel_data_len` [`Accel`] records (`#[repr(C, u32)]`, 8
//!   words each), overlaid directly as `&[Accel]`.
//! - `eof_escape`: `num_states` `u32` entries: the continuation index of the
//!   EOF escape edge of the state, `NO_CONTINUATION` for a budget escape or
//!   no EOF escape.
//! - `escape_off`: `num_states + 1` `u32` entries: the start offsets of the
//!   per-state escape ranges in `escape_ranges`.
//! - `escape_ranges`: the `EscapeRange`s of all states, concatenated: the
//!   sorted, disjoint symbol intervals whose edge escapes.
//! - `continuations`: the `LexerContinuation`s of the mode.
//! - `continuation_ctxs`: the `ContinuationContext` tables of all
//!   continuations, concatenated.
//! - `continuation_configs`: the `ContinuationConfig`s of all
//!   continuations, concatenated.
//! - `continuation_actions`: the action traces of all configs, concatenated.
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
//!   content. The stream carries no expanded symbol-to-class map: the walker
//!   takes the class of a symbol by partitioning `class_starts`.
//! - Action-list section: `num_action_list_ints`, then the concatenated
//!   lists. A list is `len`, then `len` times the ATN encoding
//!   `(type, data1, data2)` of one lexer action. Lists are deduplicated by
//!   content.
//! - Accel section: `num_accel_ints`, then the run-acceleration entries.
//!   An entry is `(state, ACCEL_POSITIVE, w0, w1, w2, w3)` for a 128-bit
//!   bitmap of self-looping ASCII symbols, `(state, ACCEL_EXCLUSION,
//!   n_needles, needle0, needle1, needle2)` with zero-padded needles, or
//!   `(state, ACCEL_COUNTED_EXCLUSION, len, opener_le, closer_le, 0)` for a
//!   counted nested opener/closer scan.
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

use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::mem::{align_of, size_of, size_of_val};

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
/// Accel tag: counted nested run over equal-length 1..=3-byte opener and closer.
pub const ACCEL_COUNTED_EXCLUSION: u32 = 3;
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

/// First word of a packed mode image: ASCII "SDFM".
const IMAGE_MAGIC: u32 = 0x5344_464D;
/// Version of the packed mode image layout.
const IMAGE_VERSION: u32 = 1;

/// All static lexer DFA tables of a grammar, one entry per lexer mode. A
/// mode that the builder could not compile has no table; the walker runs
/// the lazy path for it.
#[derive(Debug)]
#[cfg_attr(feature = "static-dfa-build", derive(PartialEq))]
pub struct StaticLexerTables {
    pub(crate) modes: Vec<Option<StaticLexerDFA>>,
}

/// The table of one lexer mode: a zero-copy view over its packed image.
///
/// The image is the self-contained word array that `StaticLexerDfaBuild::finalize`
/// freezes the sections into: a `Header` of section offsets, then the sections
/// (see the module documentation).
/// Generated code embeds those words as a `static [u32; N]`, so a table that
/// [`StaticLexerTables::from_words`] decodes from such an array borrows it.
pub struct StaticLexerDFA {
    data: Cow<'static, [u32]>,
}

impl fmt::Debug for StaticLexerDFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header = self.header();
        f.debug_struct("StaticLexerDFA")
            .field("start_state", &header.start_state)
            .field("num_states", &header.num_states)
            .field("num_classes", &header.num_classes)
            .field("num_continuations", &header.continuations_len)
            .field("words", &self.data.len())
            .finish()
    }
}

#[cfg(feature = "static-dfa-build")]
impl PartialEq for StaticLexerDFA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[cfg(feature = "static-dfa-build")]
impl Eq for StaticLexerDFA {}

/// Header of the packed image of one mode: the section offsets and the
/// element counts that the offsets do not imply.
///
/// Every field is a `u32` word, so the header is a table of words with no
/// padding and no invalid bit pattern, and the image words of one target are
/// a valid header on every target.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Header {
    /// [`IMAGE_MAGIC`].
    magic: u32,
    /// [`IMAGE_VERSION`].
    version: u32,
    /// Words of the whole image, header included.
    total_words: u32,
    /// Table states of the mode.
    num_states: u32,
    /// Hi-symbol classes of the mode, 0 for a mode with no edges past 127.
    num_classes: u32,
    /// Deduplicated hi-edge rows of the mode.
    num_hi_rows: u32,
    /// State the walk of a token starts in.
    start_state: u32,
    /// Byte offset of the dense low rows, [`LOW_ROW`] columns per state.
    low: u32,
    /// Byte offset of the hi-class bounds, `num_classes` entries.
    class_starts: u32,
    /// Byte offset of the dense hi rows, `num_hi_rows` rows.
    hi_rows: u32,
    /// Byte offset of the state-to-row map, `num_states` entries.
    row_of: u32,
    /// Byte offset of the accept token type per state, `num_states` entries.
    accept_type: u32,
    /// Byte offset of the accept action spans, `num_states` entries.
    accept_actions: u32,
    /// Byte offset of the accept action words.
    actions: u32,
    /// Number of actions in the `actions` section.
    actions_len: u32,
    /// Byte offset of the accel tag per state, `num_states` entries.
    accel_tag: u32,
    /// Byte offset of the [`Accel`] payloads.
    accel_data: u32,
    /// Number of payloads in the `accel_data` section.
    accel_data_len: u32,
    /// Byte offset of the EOF escape index per state, `num_states` entries.
    eof_escape: u32,
    /// Byte offset of the escape range offsets, `num_states + 1` entries.
    escape_off: u32,
    /// Byte offset of the escape ranges.
    escape_ranges: u32,
    /// Number of ranges in the `escape_ranges` section.
    escape_ranges_len: u32,
    /// Byte offset of the continuation pool.
    continuations: u32,
    /// Number of continuations in the pool.
    continuations_len: u32,
    /// Byte offset of the context tables of all continuations.
    continuation_ctxs: u32,
    /// Number of contexts in the `continuation_ctxs` section.
    continuation_ctxs_len: u32,
    /// Byte offset of the configs of all continuations.
    continuation_configs: u32,
    /// Number of configs in the `continuation_configs` section.
    continuation_configs_len: u32,
    /// Byte offset of the action traces of all configs.
    continuation_actions: u32,
    /// Number of action indices in the `continuation_actions` section.
    continuation_actions_len: u32,
}

/// Words of a packed-image header.
const HEADER_WORDS: usize = size_of::<Header>() / 4;

/// `(offset, len)` of the accept action list of one state, in actions.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ActionSpan {
    /// Offset into the `actions` section, in actions.
    pub(crate) off: u32,
    /// Number of actions in the list.
    pub(crate) len: u32,
}

impl ActionSpan {
    /// The span of an empty list.
    pub(crate) const EMPTY: ActionSpan = ActionSpan { off: 0, len: 0 };
}

/// One escape range: the edges on the symbols `lo..=hi` escape to the
/// continuation `continuation` (`NO_CONTINUATION` for a budget escape).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct EscapeRange {
    pub(crate) lo: i32,
    pub(crate) hi: i32,
    pub(crate) continuation: u32,
}

/// One continuation of the pool: the span of its context table and the span
/// of its configs. The action trace of a config lives in the config.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LexerContinuation {
    /// `CONTINUATION_MOVED_CONFIGS` or `CONTINUATION_SOURCE_SET`.
    pub(crate) kind: u32,
    /// Offset into the context table of the mode, in contexts.
    pub(crate) ctxs_off: u32,
    /// Number of contexts of the continuation.
    pub(crate) ctxs_len: u32,
    /// Offset into the config table of the mode, in configs.
    pub(crate) configs_off: u32,
    /// Number of configs of the continuation.
    pub(crate) configs_len: u32,
}

/// One singleton context node of a continuation: the parent node (`0` for the
/// empty context) and the ATN state the context returns to.
///
/// The context table is topological: local id 0 is the empty context, and
/// entry `i` has local id `i + 1` with parents that refer to smaller ids.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContinuationContext {
    pub(crate) parent: u32,
    pub(crate) return_state: i32,
}

/// One config of a continuation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContinuationConfig {
    /// ATN state number.
    pub(crate) state: i32,
    pub(crate) alt: i32,
    /// Context as a local id into the context table of the continuation.
    pub(crate) ctx: u32,
    /// `passed_through_non_greedy` flag bit 0, `alt_reached_accept` bit 1.
    pub(crate) flags: u32,
    /// Offset into the action trace of the mode, in action indices.
    pub(crate) actions_off: u32,
    /// Number of actions of the config.
    pub(crate) actions_len: u32,
}

impl ContinuationConfig {
    /// The `passed_through_non_greedy` flag of the config.
    #[inline]
    pub(crate) fn non_greedy(&self) -> bool {
        self.flags & 1 != 0
    }

    /// The `current_alt_reached_accept_state` flag the closure of the config
    /// ran with. Meaningful for `CONTINUATION_MOVED_CONFIGS` only; the resume
    /// recomputes it for `CONTINUATION_SOURCE_SET`.
    #[inline]
    pub(crate) fn alt_reached_accept(&self) -> bool {
        self.flags & 2 != 0
    }
}

/// The flag words of a `ContinuationConfig`.
#[cfg(feature = "static-dfa-build")]
#[inline]
pub(crate) fn config_flags(non_greedy: bool, alt_reached_accept: bool) -> u32 {
    u32::from(non_greedy) | (u32::from(alt_reached_accept) << 1)
}

/// A borrowed continuation of the pool and the slices of the packed image it
/// spans.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ContinuationView<'a> {
    kind: u32,
    ctxs: &'a [ContinuationContext],
    configs: &'a [ContinuationConfig],
    actions: &'a [u32],
}

impl<'a> ContinuationView<'a> {
    /// `CONTINUATION_MOVED_CONFIGS` or `CONTINUATION_SOURCE_SET`.
    #[inline]
    pub(crate) fn kind(&self) -> u32 {
        self.kind
    }

    /// The topological context table of the continuation.
    #[inline]
    pub(crate) fn ctxs(&self) -> &'a [ContinuationContext] {
        self.ctxs
    }

    /// The configs of the continuation.
    #[inline]
    pub(crate) fn configs(&self) -> &'a [ContinuationConfig] {
        self.configs
    }

    /// The position-independent action trace of `config`, as indices into
    /// `ATN::lexer_actions`.
    #[inline]
    pub(crate) fn config_actions(&self, config: &ContinuationConfig) -> &'a [u32] {
        &self.actions
            [config.actions_off as usize..(config.actions_off + config.actions_len) as usize]
    }
}

/// A `u16` table of the packed image, two entries per word in little-endian
/// order (`lo | (hi << 16)`).
///
/// On little-endian targets the memory layout matches `[u16]` directly, so the
/// view is a zero-copy `&[u16]` slice; on big-endian targets each read
/// extracts the half-word from the `u32` word on the fly.
#[derive(Debug, Clone, Copy)]
struct PackedU16<'a> {
    #[cfg(target_endian = "little")]
    entries: &'a [u16],
    #[cfg(target_endian = "big")]
    words: &'a [u32],
}

impl PackedU16<'_> {
    /// The entry at `index`.
    ///
    /// Unchecked-index safety: the table was written for at least the entries
    /// its section declares, and every index the walker forms is in bounds.
    #[inline]
    fn get(&self, index: usize) -> u16 {
        #[cfg(target_endian = "little")]
        {
            unsafe { *self.entries.get_unchecked(index) }
        }
        #[cfg(target_endian = "big")]
        {
            (unsafe { *self.words.get_unchecked(index >> 1) } >> ((index & 1) * 16)) as u16
        }
    }
}

/// Words of a `u16` table of `entries` entries.
#[inline]
fn packed_u16_words(entries: usize) -> usize {
    entries.div_ceil(2)
}

/// Per-state run-acceleration descriptor.
///
/// When a run of consecutive input bytes loops back to the same state, the
/// walker consumes the run with one scan. A run ends on the first byte that
/// does not loop, and the normal table walk resumes from that byte.
#[repr(C, u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accel {
    /// Run over the ASCII bytes set in the 128-bit bitmap (four 32-bit words,
    /// low to high).
    Positive([u32; 4]) = 1,
    /// Run over all input until the first occurrence of the needle byte.
    Exclude(u32) = 2,
    /// Run over all input until the first occurrence of either needle byte.
    Exclude2(u32, u32) = 3,
    /// Run over all input until the first occurrence of any of the three needle bytes.
    Exclude3(u32, u32, u32) = 4,
    /// Counted nested run over equal-length 1..=3-byte opener and closer delimiters.
    CountedExclusion {
        /// ASCII bytes of the opening delimiter (zero-padded).
        opener: [u32; 3],
        /// ASCII bytes of the closing delimiter (zero-padded).
        closer: [u32; 3],
        /// Byte length of `opener` and `closer` (`1..=3`).
        n: u32,
    } = 5,
}

impl Accel {
    /// Builds a [`Self::Positive`] descriptor from a 128-bit ASCII bitmap.
    #[inline]
    pub const fn positive(bitmap: u128) -> Self {
        Self::Positive([
            bitmap as u32,
            (bitmap >> 32) as u32,
            (bitmap >> 64) as u32,
            (bitmap >> 96) as u32,
        ])
    }

    /// Reconstructs the 128-bit ASCII bitmap of a [`Self::Positive`] descriptor.
    #[inline]
    pub const fn bitmap_u128(words: [u32; 4]) -> u128 {
        (words[0] as u128)
            | ((words[1] as u128) << 32)
            | ((words[2] as u128) << 64)
            | ((words[3] as u128) << 96)
    }

    /// The 8-word `#[repr(C, u32)]` image representation of this descriptor,
    /// with unused union payload words zero-filled so the image contains no
    /// uninitialized padding bytes.
    pub(crate) fn to_image_words(self) -> [u32; 8] {
        match self {
            Accel::Positive([w0, w1, w2, w3]) => [1, w0, w1, w2, w3, 0, 0, 0],
            Accel::Exclude(n0) => [2, n0, 0, 0, 0, 0, 0, 0],
            Accel::Exclude2(n0, n1) => [3, n0, n1, 0, 0, 0, 0, 0],
            Accel::Exclude3(n0, n1, n2) => [4, n0, n1, n2, 0, 0, 0, 0],
            Accel::CountedExclusion {
                opener: [o0, o1, o2],
                closer: [c0, c1, c2],
                n,
            } => [5, o0, o1, o2, c0, c1, c2, n],
        }
    }

    /// The serialized accel entry of this descriptor for state `s`:
    /// `(state, tag, payload...)`, in the section layout of the word-stream
    /// documentation.
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn to_serialized(self, s: u16) -> Vec<u32> {
        match self {
            Accel::Positive([w0, w1, w2, w3]) => {
                vec![u32::from(s), ACCEL_POSITIVE, w0, w1, w2, w3]
            }
            Accel::Exclude(n0) => {
                vec![u32::from(s), ACCEL_EXCLUSION, 1, n0, 0, 0]
            }
            Accel::Exclude2(n0, n1) => {
                vec![u32::from(s), ACCEL_EXCLUSION, 2, n0, n1, 0]
            }
            Accel::Exclude3(n0, n1, n2) => {
                vec![u32::from(s), ACCEL_EXCLUSION, 3, n0, n1, n2]
            }
            Accel::CountedExclusion { opener, closer, n } => {
                vec![
                    u32::from(s),
                    ACCEL_COUNTED_EXCLUSION,
                    n,
                    u32::from_le_bytes([opener[0] as u8, opener[1] as u8, opener[2] as u8, 0]),
                    u32::from_le_bytes([closer[0] as u8, closer[1] as u8, closer[2] as u8, 0]),
                    0,
                ]
            }
        }
    }
}

impl StaticLexerDFA {
    #[inline]
    fn header(&self) -> &Header {
        // SAFETY: a `StaticLexerDFA` only ever wraps an image whose first
        // `HEADER_WORDS` words are a header: `finalize` writes one, and
        // `from_words` refuses shorter images. A `u32` has no invalid bit
        // pattern, so every word pattern is a valid header.
        debug_assert!(self.data.len() >= HEADER_WORDS);
        unsafe { &*(self.data.as_ptr() as *const Header) }
    }

    #[inline]
    fn bytes(&self) -> &[u8] {
        // SAFETY: `u32` has no alignment or validity requirement above `u8`,
        // and `data.len() * 4` bytes of the allocation are initialized.
        unsafe { std::slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len() * 4) }
    }

    /// The table of `len` entries of `T` at byte `offset` of the image.
    ///
    /// Safety: `offset` and `len` come from the header of the image, `offset`
    /// is a multiple of `align_of::<T>()`, and the section is inside the
    /// image.
    #[inline]
    fn section<T>(&self, offset: u32, len: usize) -> &[T] {
        unsafe {
            let start = self.bytes().as_ptr().add(offset as usize) as *const T;
            std::slice::from_raw_parts(start, len)
        }
    }

    /// The records of a `#[repr(C)]` table at byte `offset` of the image.
    ///
    /// Safety: `T` is one of the record types of the image, which are all
    /// made of 4-byte words only, so every record fits a whole number of
    /// words at a 4-byte-aligned offset.
    #[inline]
    fn records<T>(&self, offset: u32, len: usize) -> &[T] {
        debug_assert_eq!(size_of::<T>() % 4, 0, "records pack into whole words");
        debug_assert!(align_of::<T>() <= 4, "records pack into words");
        self.section(offset, len)
    }

    /// The `u16` table of `entries` entries at byte `offset` of the image.
    #[inline]
    fn u16_table(&self, offset: u32, entries: usize) -> PackedU16<'_> {
        debug_assert_eq!(offset % 4, 0, "u16 sections are word-aligned");
        #[cfg(target_endian = "little")]
        {
            PackedU16 {
                entries: self.section(offset, entries),
            }
        }
        #[cfg(target_endian = "big")]
        {
            PackedU16 {
                words: self.section(offset, packed_u16_words(entries)),
            }
        }
    }

    #[inline]
    fn low(&self) -> PackedU16<'_> {
        let header = self.header();
        self.u16_table(header.low, header.num_states as usize * LOW_ROW)
    }

    #[inline]
    fn class_starts(&self) -> &[u32] {
        let header = self.header();
        self.section(header.class_starts, header.num_classes as usize)
    }

    #[inline]
    fn hi_rows(&self) -> PackedU16<'_> {
        let header = self.header();
        self.u16_table(
            header.hi_rows,
            header.num_hi_rows as usize * header.num_classes as usize,
        )
    }

    #[inline]
    fn row_of(&self) -> &[u32] {
        let header = self.header();
        self.section(header.row_of, header.num_states as usize)
    }

    #[inline]
    fn accept_types(&self) -> &[i32] {
        let header = self.header();
        self.section(header.accept_type, header.num_states as usize)
    }

    #[inline]
    fn accel_tags(&self) -> PackedU16<'_> {
        let header = self.header();
        self.u16_table(header.accel_tag, header.num_states as usize)
    }

    #[inline]
    fn eof_escapes(&self) -> &[u32] {
        let header = self.header();
        self.section(header.eof_escape, header.num_states as usize)
    }

    #[inline]
    fn escape_ranges_of(&self) -> &[EscapeRange] {
        let header = self.header();
        self.records(header.escape_ranges, header.escape_ranges_len as usize)
    }

    #[inline]
    fn escape_range_offsets(&self) -> &[u32] {
        let header = self.header();
        self.section(header.escape_off, header.num_states as usize + 1)
    }

    /// The start and end offsets of the escape ranges of state `s`.
    #[inline]
    fn escape_row(&self, s: u16) -> std::ops::Range<usize> {
        let off = self.escape_range_offsets();
        // Unchecked-index safety: `escape_off` has `num_states + 1` entries
        // and every state id the walker forms is below `num_states`.
        unsafe {
            let start = *off.get_unchecked(s as usize) as usize;
            let end = *off.get_unchecked(s as usize + 1) as usize;
            start..end
        }
    }

    /// The state the walk of a token starts in.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn start_state(&self) -> u16 {
        self.header().start_state as u16
    }

    /// The state the walk of a token starts in.
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn start_state(&self) -> u16 {
        self.header().start_state as u16
    }

    /// Raw edge entry of state `s` on input symbol `t` (`-1` = EOF):
    /// `NO_EDGE` when there is no edge, `ESCAPE` for an escape edge, else
    /// `target | ACCEPT_BIT?` (the bit marks accept states, so the per-char
    /// walk loop needs only this one table load per input symbol).
    ///
    /// No bounds checks: every `s` the walker produces is a valid state id
    /// (the start state and all serialized targets are validated at load
    /// time), `row_of` entries are validated in range at load time, and class
    /// ids are smaller than `num_classes` by construction.
    #[inline]
    fn edge_entry_impl(&self, s: u16, t: i32) -> u16 {
        // Column 0 is EOF (`t = -1` wraps to 0), columns 1..=128 are symbols
        // 0..=127; everything else takes the class-map path.
        let col = (t as u32).wrapping_add(1) as usize;
        if col < LOW_ROW {
            return self.low().get(s as usize * LOW_ROW + col);
        }
        // Partition `class_starts` for the symbol class, then load the state's
        // deduplicated hi row. The bounds also cover modes without hi edges
        // (an empty class-start table) and symbols past MAX_SYMBOL.
        let symbol = t as u32;
        let header = self.header();
        if header.num_classes == 0 || symbol > MAX_SYMBOL {
            return NO_EDGE;
        }
        let class = class_of(self.class_starts(), symbol);
        // Unchecked-index safety: `row_of` has `num_states` entries and every
        // state id the walker forms is below `num_states`.
        let row = unsafe { *self.row_of().get_unchecked(s as usize) } as usize;
        self.hi_rows()
            .get(row * header.num_classes as usize + class)
    }

    /// Raw edge entry of state `s` on input symbol `t`: see
    /// [`StaticLexerDFA::edge_entry_impl`].
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn edge_entry(&self, s: u16, t: i32) -> u16 {
        self.edge_entry_impl(s, t)
    }

    /// Raw edge entry of state `s` on input symbol `t`: see
    /// [`StaticLexerDFA::edge_entry_impl`].
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn edge_entry(&self, s: u16, t: i32) -> u16 {
        self.edge_entry_impl(s, t)
    }

    /// Token type produced when accepting in state `s`; -1 = not an accept
    /// state.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn accept_type(&self, s: u16) -> i32 {
        unsafe { *self.accept_types().get_unchecked(s as usize) }
    }

    /// Token type produced when accepting in state `s`; -1 = not an accept
    /// state.
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn accept_type(&self, s: u16) -> i32 {
        unsafe { *self.accept_types().get_unchecked(s as usize) }
    }

    /// The position-independent lexer actions to execute when accepting in
    /// state `s` (skip, mode, channel, type, ...), in order; an empty slice
    /// when there are none.
    #[inline]
    pub(crate) fn accept_actions(&self, s: u16) -> &[LexerAction] {
        let header = self.header();
        let spans = self.records::<ActionSpan>(header.accept_actions, header.num_states as usize);
        // Unchecked-index safety: `accept_actions` has `num_states` entries
        // and every state id the walker forms is below `num_states`.
        let span = unsafe { *spans.get_unchecked(s as usize) };
        let actions = self.records::<LexerAction>(header.actions, header.actions_len as usize);
        let start = span.off as usize;
        &actions[start..start + span.len as usize]
    }

    /// Run-acceleration flavor tag of state `s`: 0 = none, else an index
    /// + 1 into the payload table (see [`StaticLexerDFA::accel_payload`]).
    #[inline]
    pub(crate) fn accel_tag(&self, s: u16) -> u16 {
        self.accel_tags().get(s as usize)
    }

    /// Run-acceleration payload for a nonzero [`StaticLexerDFA::accel_tag`].
    #[inline]
    pub(crate) fn accel_payload(&self, tag: u16) -> Accel {
        let header = self.header();
        let entries = self.records::<Accel>(header.accel_data, header.accel_data_len as usize);
        // Unchecked-index safety: tags are written only at build time,
        // bounded by the payload table length.
        unsafe { *entries.get_unchecked(tag as usize - 1) }
    }

    /// The continuation index of the escape edge of state `s` on `symbol`
    /// (EOF = -1 takes the EOF escape). `NO_CONTINUATION` when the edge
    /// does not escape or the escape is a budget escape.
    #[inline]
    fn escape_continuation_impl(&self, s: u16, symbol: i32) -> u32 {
        if symbol == EOF {
            return unsafe { *self.eof_escapes().get_unchecked(s as usize) };
        }
        // Valid tables have a range for every ESCAPE entry (checked at
        // load); the restart-on-budget-escape path treats a miss as a
        // budget escape, which stays correct.
        self.escape_range(s, symbol)
            .map_or(NO_CONTINUATION, |r| r.continuation)
    }

    /// The continuation index of the escape edge of state `s` on `symbol`:
    /// see [`StaticLexerDFA::escape_continuation_impl`].
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn escape_continuation(&self, s: u16, symbol: i32) -> u32 {
        self.escape_continuation_impl(s, symbol)
    }

    /// The continuation index of the escape edge of state `s` on `symbol`:
    /// see [`StaticLexerDFA::escape_continuation_impl`].
    #[cfg(not(feature = "static-dfa-build"))]
    #[inline]
    pub(crate) fn escape_continuation(&self, s: u16, symbol: i32) -> u32 {
        self.escape_continuation_impl(s, symbol)
    }

    /// The escape range of state `s` covering `symbol`, if any. The
    /// continuation of the range can be `NO_CONTINUATION` (a budget
    /// escape): coverage and continuation value are separate facts.
    fn escape_range(&self, s: u16, symbol: i32) -> Option<&EscapeRange> {
        let row = unsafe { self.escape_ranges_of().get_unchecked(self.escape_row(s)) };
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
    pub(crate) fn continuation(&self, index: u32) -> ContinuationView<'_> {
        let header = self.header();
        let pool = self
            .records::<LexerContinuation>(header.continuations, header.continuations_len as usize);
        let pool_len = pool.len();
        let continuation = unsafe {
            debug_assert!((index as usize) < pool_len);
            *pool.get_unchecked(index as usize)
        };
        let ctxs = self.records::<ContinuationContext>(
            header.continuation_ctxs,
            header.continuation_ctxs_len as usize,
        );
        let configs = self.records::<ContinuationConfig>(
            header.continuation_configs,
            header.continuation_configs_len as usize,
        );
        let actions = self.section::<u32>(
            header.continuation_actions,
            header.continuation_actions_len as usize,
        );
        ContinuationView {
            kind: continuation.kind,
            ctxs: &ctxs[continuation.ctxs_off as usize
                ..(continuation.ctxs_off + continuation.ctxs_len) as usize],
            configs: &configs[continuation.configs_off as usize
                ..(continuation.configs_off + continuation.configs_len) as usize],
            actions,
        }
    }
}

/// The symbol class of `symbol` (which is `>= 128`): the index of the last
/// class start at or below it.
#[inline]
fn class_of(class_starts: &[u32], symbol: u32) -> usize {
    // Class 0 starts at 128, so the partition point is at least 1.
    class_starts.partition_point(|&start| start <= symbol) - 1
}

/// The unpacked sections of one lexer mode, ready to be frozen into the
/// packed image of a [`StaticLexerDFA`].
///
/// The builder fills these sections directly; `finalize` lays them out in image
/// order, and every runtime accessor reads the image, not the sections.
#[derive(Debug)]
pub(crate) struct StaticLexerDfaBuild {
    /// The state the walk of a token starts in.
    pub(crate) start_state: u16,
    /// `num_states * LOW_ROW` dense edge entries: column 0 is the EOF edge,
    /// columns 1..=128 are the symbols 0..=127.
    pub(crate) low: Vec<u16>,
    /// Deduplicated dense class rows: the hi-edge row of state `s` is
    /// `hi_rows[row_of[s] * num_classes..][..num_classes]`.
    pub(crate) hi_rows: Vec<u16>,
    /// State-to-row map into `hi_rows`.
    pub(crate) row_of: Vec<u32>,
    /// Row width of `hi_rows`: the symbol-class count of the mode.
    pub(crate) num_classes: usize,
    /// Class bounds of the mode; class `i` covers
    /// `class_starts[i]..class_starts[i + 1]`, the last class up to
    /// `MAX_SYMBOL`.
    pub(crate) class_starts: Vec<u32>,
    /// Token type per accept state; -1 when the state is not an accept
    /// state. 0 is a real accept type: rules with a `more`/`type(...)`
    /// command get no token type assigned by the tool.
    pub(crate) accept_type: Vec<i32>,
    /// The accept action list of each state, in `actions`; `ActionSpan::EMPTY`
    /// when empty.
    pub(crate) accept_actions: Vec<ActionSpan>,
    /// The accept action lists of all states, concatenated and
    /// deduplicated by content. Only position-independent actions appear
    /// here, so the walker can run them with plain `LexerAction::execute`
    /// calls.
    pub(crate) actions: Vec<LexerAction>,
    /// Run-acceleration flavor tag per state: 0 = none, else the index of
    /// the payload in `accel_data` + 1.
    pub(crate) accel_tag: Vec<u16>,
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
    /// The context tables of all continuations, concatenated.
    pub(crate) continuation_ctxs: Vec<ContinuationContext>,
    /// The configs of all continuations, concatenated.
    pub(crate) continuation_configs: Vec<ContinuationConfig>,
    /// The action traces of all configs, concatenated.
    pub(crate) continuation_actions: Vec<u32>,
}

impl StaticLexerDfaBuild {
    /// The number of distinct hi-edge rows of the sections.
    fn num_hi_rows(&self) -> usize {
        self.hi_rows.len() / self.num_classes.max(1)
    }

    /// Freezes the sections into the packed image of a [`StaticLexerDFA`].
    pub(crate) fn finalize(self) -> StaticLexerDFA {
        let num_states = self.accept_type.len();
        assert_eq!(self.low.len(), num_states * LOW_ROW, "low rows");
        assert_eq!(self.class_starts.len(), self.num_classes, "class starts");
        assert_eq!(self.accel_tag.len(), num_states, "one accel tag per state");
        assert_eq!(
            self.accept_actions.len(),
            num_states,
            "one accept action span per state"
        );
        assert_eq!(
            self.eof_escape.len(),
            num_states,
            "one EOF escape per state"
        );
        assert_eq!(self.row_of.len(), num_states, "one hi row per state");
        assert_eq!(
            self.escape_off.len(),
            num_states + 1,
            "one escape range offset per state and one end"
        );
        assert_eq!(
            self.hi_rows.len(),
            self.num_hi_rows() * self.num_classes,
            "dense hi rows"
        );
        let mut words: Vec<u32> = vec![0; HEADER_WORDS];
        let mut header = Header {
            magic: IMAGE_MAGIC,
            version: IMAGE_VERSION,
            total_words: 0,
            num_states: num_states as u32,
            num_classes: self.num_classes as u32,
            num_hi_rows: self.num_hi_rows() as u32,
            start_state: u32::from(self.start_state),
            low: 0,
            class_starts: 0,
            hi_rows: 0,
            row_of: 0,
            accept_type: 0,
            accept_actions: 0,
            actions: 0,
            actions_len: self.actions.len() as u32,
            accel_tag: 0,
            accel_data: 0,
            accel_data_len: self.accel_data.len() as u32,
            eof_escape: 0,
            escape_off: 0,
            escape_ranges: 0,
            escape_ranges_len: self.escape_ranges.len() as u32,
            continuations: 0,
            continuations_len: self.continuations.len() as u32,
            continuation_ctxs: 0,
            continuation_ctxs_len: self.continuation_ctxs.len() as u32,
            continuation_configs: 0,
            continuation_configs_len: self.continuation_configs.len() as u32,
            continuation_actions: 0,
            continuation_actions_len: self.continuation_actions.len() as u32,
        };

        header.low = image_offset(&words);
        push_packed_u16(&mut words, &self.low);
        header.class_starts = image_offset(&words);
        words.extend_from_slice(&self.class_starts);
        header.hi_rows = image_offset(&words);
        push_packed_u16(&mut words, &self.hi_rows);
        header.row_of = image_offset(&words);
        words.extend_from_slice(&self.row_of);
        header.accept_type = image_offset(&words);
        words.extend(self.accept_type.iter().map(|&ty| ty as u32));
        header.accept_actions = image_offset(&words);
        push_records(&mut words, &self.accept_actions);
        header.actions = image_offset(&words);
        for &action in &self.actions {
            words.extend(action.to_image_words());
        }
        header.accel_tag = image_offset(&words);
        push_packed_u16(&mut words, &self.accel_tag);
        header.accel_data = image_offset(&words);
        for &accel in &self.accel_data {
            words.extend(accel.to_image_words());
        }
        header.eof_escape = image_offset(&words);
        words.extend_from_slice(&self.eof_escape);
        header.escape_off = image_offset(&words);
        words.extend_from_slice(&self.escape_off);
        header.escape_ranges = image_offset(&words);
        push_records(&mut words, &self.escape_ranges);
        header.continuations = image_offset(&words);
        push_records(&mut words, &self.continuations);
        header.continuation_ctxs = image_offset(&words);
        push_records(&mut words, &self.continuation_ctxs);
        header.continuation_configs = image_offset(&words);
        push_records(&mut words, &self.continuation_configs);
        header.continuation_actions = image_offset(&words);
        words.extend_from_slice(&self.continuation_actions);

        header.total_words = words.len() as u32;
        header.write(&mut words);
        let table = StaticLexerDFA {
            data: Cow::Owned(words),
        };
        debug_assert_eq!(
            table.records::<LexerAction>(header.actions, self.actions.len()),
            self.actions.as_slice()
        );
        debug_assert_eq!(
            table.records::<Accel>(header.accel_data, self.accel_data.len()),
            self.accel_data.as_slice()
        );
        table
    }
}

impl Header {
    /// Writes the header into the first words of `words`.
    fn write(&self, words: &mut [u32]) {
        assert!(words.len() >= HEADER_WORDS, "image holds the header");
        // SAFETY: the header is `HEADER_WORDS` words of `u32` with no
        // padding, and `words` is at least that long and 4-byte aligned.
        unsafe { std::ptr::write(words.as_mut_ptr() as *mut Header, *self) };
    }
}

/// Byte offset of the next section of `words`, which are whole words so far.
#[inline]
fn image_offset(words: &[u32]) -> u32 {
    (words.len() * 4) as u32
}

/// Appends a `u16` table, two entries per word.
fn push_packed_u16(words: &mut Vec<u32>, values: &[u16]) {
    let start = words.len();
    words.resize(start + packed_u16_words(values.len()), 0);
    for (index, &value) in values.iter().enumerate() {
        words[start + (index >> 1)] |= u32::from(value) << ((index & 1) * 16);
    }
}

/// Appends a `#[repr(C)]` table of 4-byte words.
fn push_records<T: Copy>(words: &mut Vec<u32>, records: &[T]) {
    debug_assert_eq!(size_of::<T>() % 4, 0, "records pack into whole words");
    debug_assert!(align_of::<T>() <= 4, "records pack into words");
    let byte_len = size_of_val(records);
    let start = words.len();
    words.resize(start + byte_len / 4, 0);
    if records.is_empty() {
        return;
    }
    // SAFETY: both tables have `byte_len` initialized bytes, and the
    // destination starts at a 4-byte-aligned word boundary.
    unsafe {
        std::ptr::copy_nonoverlapping(
            records.as_ptr() as *const u8,
            words.as_mut_ptr().add(start) as *mut u8,
            byte_len,
        );
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
    /// An exclusion accel entry has a needle count outside `1..=3` or a
    /// needle byte past 127.
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

/// The concatenated continuation sections of one mode: the context tables,
/// the configs, and the action traces of all continuations, in pool order.
#[derive(Debug, Default)]
pub(crate) struct ContinuationPools {
    pub(crate) ctxs: Vec<ContinuationContext>,
    pub(crate) configs: Vec<ContinuationConfig>,
    pub(crate) actions: Vec<u32>,
}

impl ContinuationPools {
    /// Appends the context table, the configs, and the actions of one
    /// continuation and returns its pool entry.
    pub(crate) fn push(
        &mut self,
        kind: u32,
        ctxs: impl IntoIterator<Item = ContinuationContext>,
        configs: impl IntoIterator<Item = ContinuationConfig>,
        actions: impl IntoIterator<Item = u32>,
    ) -> LexerContinuation {
        let ctxs_off = self.ctxs.len() as u32;
        self.ctxs.extend(ctxs);
        let ctxs_len = self.ctxs.len() as u32 - ctxs_off;
        let configs_off = self.configs.len() as u32;
        self.configs.extend(configs);
        let configs_len = self.configs.len() as u32 - configs_off;
        let actions_off = self.actions.len() as u32;
        self.actions.extend(actions);
        // The configs name their actions by offset into this section, so
        // patch them up as they land.
        let configs = &mut self.configs[configs_off as usize..];
        for config in configs.iter_mut() {
            if config.actions_len == 0 {
                config.actions_off = 0;
            } else {
                config.actions_off += actions_off;
            }
        }
        LexerContinuation {
            kind,
            ctxs_off,
            ctxs_len,
            configs_off,
            configs_len,
        }
    }
}

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
            for index in 0..mode.header().continuations_len {
                let continuation = mode.continuation(index);
                for ctx in continuation.ctxs() {
                    assert!(
                        (ctx.return_state as usize) < states,
                        "static lexer DFA table of mode {m} references ATN state {}, \
                         but the ATN has {states} states; regenerate the lexer tables",
                        ctx.return_state
                    );
                }
                for config in continuation.configs() {
                    assert!(
                        (config.state as usize) < states,
                        "static lexer DFA table of mode {m} references ATN state {}, \
                         but the ATN has {states} states; regenerate the lexer tables",
                        config.state
                    );
                    for &action in continuation.config_actions(config) {
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

/// Structural inspection of a committed table, for tests and future
/// codegen. The walker uses the hot-path accessors above; nothing here is
/// on it.
#[cfg(feature = "static-dfa-build")]
impl StaticLexerDFA {
    /// Number of table states of this mode.
    pub fn num_states(&self) -> usize {
        self.header().num_states as usize
    }

    /// Number of hi-symbol classes of this mode (0 = no edges past 127).
    pub fn num_classes(&self) -> usize {
        self.header().num_classes as usize
    }

    /// Number of continuations of this mode.
    pub fn num_continuations(&self) -> usize {
        self.header().continuations_len as usize
    }

    /// The kind of continuation `index`: `CONTINUATION_MOVED_CONFIGS` or
    /// `CONTINUATION_SOURCE_SET`.
    pub fn continuation_kind(&self, index: u32) -> u32 {
        self.continuation(index).kind()
    }

    /// The number of configs of continuation `index`.
    pub fn continuation_num_configs(&self, index: u32) -> usize {
        self.continuation(index).configs().len()
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
        let row = &self.escape_ranges_of()[self.escape_row(s)];
        row.iter().map(|r| (r.lo, r.hi, r.continuation)).collect()
    }

    /// Run-acceleration descriptor of state `s`, if any.
    pub fn state_accel(&self, s: u16) -> Option<Accel> {
        let tag = self.accel_tag(s);
        (tag != 0).then(|| self.accel_payload(tag))
    }

    /// Number of deduplicated hi-symbol class rows in this mode.
    pub fn num_distinct_hi_rows(&self) -> usize {
        self.header().num_hi_rows as usize
    }

    /// Number of populated ASCII edge entries (`0..=127`) across all states.
    pub fn ascii_populated_edges(&self) -> usize {
        let num_states = self.num_states();
        let low = self.low();
        let mut count = 0;
        for s in 0..num_states {
            for col in 1..LOW_ROW {
                count += usize::from(low.get(s * LOW_ROW + col) != NO_EDGE);
            }
        }
        count
    }

    /// Number of singleton context nodes in continuation `index`.
    pub fn continuation_num_contexts(&self, index: u32) -> usize {
        self.continuation(index).ctxs().len()
    }

    /// Number of non-greedy configs in continuation `index`.
    pub fn continuation_num_non_greedy_configs(&self, index: u32) -> usize {
        self.continuation(index)
            .configs()
            .iter()
            .filter(|config| config.non_greedy())
            .count()
    }

    /// ATN state numbers of the configs in continuation `index`, in config order.
    pub fn continuation_config_states(&self, index: u32) -> Vec<usize> {
        self.continuation(index)
            .configs()
            .iter()
            .map(|config| config.state as usize)
            .collect()
    }
}

/// The encode side of the word-stream codec: [`StaticLexerTables::to_words`]
/// and its per-section writers. The section order is the one of the module
/// documentation; the output is the exact inverse of
/// [`StaticLexerTables::from_words`].
#[cfg(feature = "static-dfa-build")]
impl StaticLexerDFA {
    /// Writes one present mode section.
    fn write_mode(&self, out: &mut Vec<u32>) {
        let num_states = self.num_states();
        out.push(num_states as u32);
        out.push(u32::from(self.start_state()));
        for s in 0..num_states {
            out.push(self.accept_type(s as u16) as u32);
        }

        // Accept action lists, deduplicated by content in state order.
        let mut action_lists: Vec<u32> = Vec::new();
        let mut list_ids: HashMap<&[LexerAction], u32> = HashMap::new();
        let mut offsets = Vec::with_capacity(num_states);
        for s in 0..num_states {
            let list = self.accept_actions(s as u16);
            if list.is_empty() {
                offsets.push(-1i32);
                continue;
            }
            let offset = match list_ids.get(list) {
                Some(&offset) => offset,
                None => {
                    let offset = action_lists.len() as u32;
                    action_lists.push(list.len() as u32);
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

        let low = self.low();
        for s in 0..num_states {
            out.push(u32::from(low.get(s * LOW_ROW)));
        }
        for s in 0..num_states {
            for col in 1..LOW_ROW {
                out.push(u32::from(low.get(s * LOW_ROW + col)));
            }
        }

        self.write_hi_classes(out);

        out.push(action_lists.len() as u32);
        out.extend(action_lists);

        let mut accel = Vec::new();
        for s in 0..num_states as u16 {
            let tag = self.accel_tag(s);
            if tag != 0 {
                accel.extend(self.accel_payload(tag).to_serialized(s));
            }
        }
        out.push(accel.len() as u32);
        out.extend(accel);

        let eof_escapes = self.eof_escapes();
        let escape_ranges = self.escape_ranges_of();
        for s in 0..num_states {
            out.push(eof_escapes[s]);
            let row = &escape_ranges[self.escape_row(s as u16)];
            out.push(row.len() as u32);
            for range in row {
                out.extend([range.lo as u32, range.hi as u32, range.continuation]);
            }
        }

        let num_continuations = self.num_continuations() as u32;
        out.push(num_continuations);
        for index in 0..num_continuations {
            let continuation = self.continuation(index);
            out.push(continuation.kind());
            out.push(continuation.ctxs().len() as u32);
            for ctx in continuation.ctxs() {
                out.extend([0, ctx.parent, ctx.return_state as u32]);
            }
            out.push(continuation.configs().len() as u32);
            for config in continuation.configs() {
                let actions = continuation.config_actions(config);
                out.extend([
                    config.state as u32,
                    config.flags,
                    config.alt as u32,
                    config.ctx,
                    actions.len() as u32,
                ]);
                for &action in actions {
                    out.extend([action, BEHIND_POSITION_INDEPENDENT]);
                }
            }
        }
    }

    /// Writes the class-compressed hi-edge section: the deduplicated dense
    /// rows are re-expressed as `(lo_class, hi_class, target)` runs.
    fn write_hi_classes(&self, out: &mut Vec<u32>) {
        let num_classes = self.num_classes();
        out.push(num_classes as u32);
        if num_classes == 0 {
            return;
        }
        out.extend(self.class_starts().iter().copied());
        let num_rows = self.num_distinct_hi_rows();
        let hi_rows = self.hi_rows();
        let row_runs: Vec<Vec<(u32, u32, u16)>> = (0..num_rows)
            .map(|row| {
                let dense: Vec<u16> = (0..num_classes)
                    .map(|class| hi_rows.get(row * num_classes + class))
                    .collect();
                dense_row_runs(&dense)
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
        out.extend(self.row_of().iter().copied());
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

/// Expands per-row `(lo_class, hi_class, target)` runs into dense
/// `num_classes`-wide rows, `NO_EDGE`-filled.
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
    let mut pools = ContinuationPools::default();
    let continuations = read_continuations(r, mode, &mut pools)?;

    let build = StaticLexerDfaBuild {
        start_state: start_state as u16,
        low,
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
        continuation_ctxs: pools.ctxs,
        continuation_configs: pools.configs,
        continuation_actions: pools.actions,
    };
    let table = build.finalize();
    table.validate_escapes(mode)?;
    Ok(Some(table))
}

/// The expanded hi-edge data of one mode.
struct HiClasses {
    class_starts: Vec<u32>,
    hi_rows: Vec<u16>,
    row_of: Vec<u32>,
    num_classes: usize,
}

/// The decoded action-list section of one mode.
struct ActionLists {
    actions: Vec<LexerAction>,
    accept_actions: Vec<ActionSpan>,
}

/// Reads the class-compressed hi-edge section of one mode: the class
/// starts, then the deduplicated class rows (as `(lo_class, hi_class,
/// target)` runs) and the per-state row index. Expands the dense class rows.
fn read_hi_classes(
    r: &mut Reader,
    mode: usize,
    num_states: usize,
    accept_type: &[i32],
) -> Result<HiClasses, StaticLexerError> {
    let num_classes = r.next()? as usize;
    if num_classes == 0 {
        return Ok(HiClasses {
            class_starts: Vec::new(),
            hi_rows: Vec::new(),
            row_of: vec![0; num_states],
            num_classes: 0,
        });
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
    let mut list_ids: HashMap<&[u32], ActionSpan> = HashMap::new();
    let mut lists = Vec::with_capacity(accept_action_offsets.len());
    for (s, &offset) in accept_action_offsets.iter().enumerate() {
        if offset < 0 {
            lists.push(ActionSpan::EMPTY);
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
        let span = ActionSpan {
            off: actions.len() as u32,
            len: len as u32,
        };
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
        list_ids.insert(triples, span);
        lists.push(span);
    }
    Ok(ActionLists {
        actions,
        accept_actions: lists,
    })
}

/// Reads the run-acceleration section of one mode: a flattened list of
/// fixed-stride entries, `(state, ACCEL_POSITIVE, w0..w3)`, `(state,
/// ACCEL_EXCLUSION, n_needles, n0..n2)`, or `(state, ACCEL_COUNTED_EXCLUSION,
/// len, opener_le, closer_le, 0)`, expanded into one descriptor per state.
fn read_accel(
    r: &mut Reader,
    mode: usize,
    num_states: usize,
) -> Result<(Vec<u16>, Vec<Accel>), StaticLexerError> {
    let num_ints = r.next()? as usize;
    let mut accel_tag = vec![0u16; num_states];
    let mut accel_data = Vec::new();
    let mut read = 0;
    while read < num_ints {
        let s = r.next()?;
        check_index(mode, "accel state", s, num_states)?;
        let payload = match r.next()? {
            ACCEL_POSITIVE => {
                let words = [r.next()?, r.next()?, r.next()?, r.next()?];
                read += 6;
                Accel::Positive(words)
            }
            ACCEL_EXCLUSION => {
                let n = r.next()?;
                let mut needle_words = [0; 3];
                for slot in needle_words.iter_mut() {
                    *slot = r.next()?;
                }
                if !(1..=3).contains(&n) || needle_words[..n as usize].iter().any(|&w| w >= 0x80) {
                    return Err(StaticLexerError::InvalidAccelNeedles { mode });
                }
                read += 6;
                match n {
                    1 => Accel::Exclude(needle_words[0]),
                    2 => Accel::Exclude2(needle_words[0], needle_words[1]),
                    3 => Accel::Exclude3(needle_words[0], needle_words[1], needle_words[2]),
                    _ => unreachable!(),
                }
            }
            ACCEL_COUNTED_EXCLUSION => {
                let n = r.next()?;
                let opener_word = r.next()?;
                let closer_word = r.next()?;
                let reserved = r.next()?;
                let opener_bytes = opener_word.to_le_bytes();
                let closer_bytes = closer_word.to_le_bytes();
                if !(1..=3).contains(&n)
                    || reserved != 0
                    || opener_word == closer_word
                    || opener_bytes[..n as usize].iter().any(|&b| b >= 0x80)
                    || closer_bytes[..n as usize].iter().any(|&b| b >= 0x80)
                    || opener_bytes[n as usize..].iter().any(|&b| b != 0)
                    || closer_bytes[n as usize..].iter().any(|&b| b != 0)
                {
                    return Err(StaticLexerError::InvalidAccelNeedles { mode });
                }
                read += 6;
                Accel::CountedExclusion {
                    opener: [
                        u32::from(opener_bytes[0]),
                        u32::from(opener_bytes[1]),
                        u32::from(opener_bytes[2]),
                    ],
                    closer: [
                        u32::from(closer_bytes[0]),
                        u32::from(closer_bytes[1]),
                        u32::from(closer_bytes[2]),
                    ],
                    n,
                }
            }
            value => {
                return Err(StaticLexerError::InvalidAccelTag { mode, value });
            }
        };
        accel_data.push(payload);
        // The tag is the payload index + 1 in a u16.
        check_index(
            mode,
            "accel payload",
            accel_data.len() as u32,
            u16::MAX as usize + 1,
        )?;
        accel_tag[s as usize] = accel_data.len() as u16;
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

/// Reads the continuation pool of one mode into the shared pools.
fn read_continuations(
    r: &mut Reader,
    mode: usize,
    pools: &mut ContinuationPools,
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
            ctxs.push(ContinuationContext {
                parent,
                return_state: return_state as i32,
            });
        }
        let nconfigs = r.next()? as usize;
        let mut configs = Vec::with_capacity(nconfigs);
        let mut actions = Vec::new();
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
            let actions_off = actions.len() as u32;
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
                flags,
                actions_off,
                actions_len: nactions as u32,
            });
        }
        continuations.push(pools.push(kind, ctxs, configs, actions));
    }
    Ok(continuations)
}

impl StaticLexerDFA {
    /// Cross-checks the escape rows against the edge entries: continuation
    /// indices in bounds, range endpoints on ESCAPE entries, every ESCAPE
    /// entry covered by a range (the EOF escape index covers the EOF edge),
    /// and the EOF escape index meaningful only on an ESCAPE EOF edge.
    fn validate_escapes(&self, mode: usize) -> Result<(), StaticLexerError> {
        let num_states = self.header().num_states as usize;
        let num_continuations = self.header().continuations_len as usize;
        let check_continuation = |index: u32| {
            if index != NO_CONTINUATION {
                check_index(mode, "continuation", index, num_continuations)?;
            }
            Ok(())
        };
        let eof_escapes = self.eof_escapes();
        for (s, &eof_escape) in eof_escapes.iter().enumerate() {
            check_continuation(eof_escape)?;
            if eof_escape != NO_CONTINUATION && self.edge_entry(s as u16, EOF) != ESCAPE {
                return Err(StaticLexerError::InconsistentEscapeRow {
                    mode,
                    state: s,
                    symbol: EOF,
                });
            }
        }
        for range in self.escape_ranges_of() {
            check_continuation(range.continuation)?;
        }
        let escape_ranges = self.escape_ranges_of();
        let low = self.low();
        let hi_rows = self.hi_rows();
        let row_of = self.row_of();
        let class_starts = self.class_starts();
        let num_classes = class_starts.len();
        for s in 0..num_states {
            for range in &escape_ranges[self.escape_row(s as u16)] {
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
                if low.get(s * LOW_ROW + col) == ESCAPE
                    && self.escape_range(s as u16, symbol).is_none()
                {
                    return Err(StaticLexerError::InconsistentEscapeRow {
                        mode,
                        state: s,
                        symbol,
                    });
                }
            }
            if num_classes > 0 {
                let row = row_of[s] as usize;
                for (class, &start) in class_starts.iter().enumerate() {
                    if hi_rows.get(row * num_classes + class) == ESCAPE
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
    use crate::dfa::static_dfa_build::StaticLexerDfaBuilder;

    fn fixture(name: &str) -> PathBuf {
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
    /// the generated-code witnesses (CSV, XMLLexer): decoding
    /// the encoded tables gives the tables back, and re-encoding a decoded
    /// stream reproduces it word for word.
    #[test]
    fn word_stream_round_trip() {
        let grammars: Vec<PathBuf> = [
            "lexer/CustomAction.g4",
            "lexer/EofChain.g4",
            "lexer/EofEscape.g4",
            "lexer/EofLexer.g4",
            "lexer/MiniModes.g4",
            "lexer/Spike.g4",
            "lexer/StartAction.g4",
            "lexer/StartPred.g4",
            "lexer/StartRecurse.g4",
            "lexer/RecursiveRule.g4",
            "lexer/SharedComments.g4",
            "CSV.g4",
            "XMLLexer.g4",
        ]
        .into_iter()
        .map(fixture)
        .collect();

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

    /// The accel tag is a `u16`, not a `u8`: a mode with 300 accelerated
    /// states keeps every payload apart across the codec.
    #[test]
    fn accel_tags_beyond_a_u8_round_trip() {
        const NUM_STATES: usize = 300;
        let table = StaticLexerDfaBuild {
            start_state: 0,
            low: vec![NO_EDGE; NUM_STATES * LOW_ROW],
            hi_rows: Vec::new(),
            row_of: vec![0; NUM_STATES],
            num_classes: 0,
            class_starts: Vec::new(),
            accept_type: vec![-1; NUM_STATES],
            accept_actions: vec![ActionSpan::EMPTY; NUM_STATES],
            actions: Vec::new(),
            accel_tag: (1..=NUM_STATES as u16).collect(),
            accel_data: (0..NUM_STATES)
                .map(|state| Accel::positive(state as u128))
                .collect(),
            eof_escape: vec![NO_CONTINUATION; NUM_STATES],
            escape_off: vec![0; NUM_STATES + 1],
            escape_ranges: Vec::new(),
            continuations: Vec::new(),
            continuation_ctxs: Vec::new(),
            continuation_configs: Vec::new(),
            continuation_actions: Vec::new(),
        }
        .finalize();
        let words = StaticLexerTables::from_modes(vec![Some(table)]).to_words();
        let tables = StaticLexerTables::from_words(&words).expect("the words decode");
        assert_eq!(tables.num_modes(), 1);
        assert_eq!(tables.to_words(), words, "re-encoding reproduces the words");
        let decoded = tables.mode(0).expect("mode 0");
        assert_eq!(decoded.num_states(), NUM_STATES);
        for state in [0u16, 1, 254, 255, 256, 299] {
            assert_eq!(decoded.accel_tag(state), state + 1);
            assert_eq!(
                decoded.state_accel(state),
                Some(Accel::positive(u128::from(state)))
            );
        }
    }

    /// The two producers of [`StaticLexerDFA`] agree: the committed tables
    /// and the decoded tables resolve every edge, accept, and escape the
    /// same way.
    #[test]
    fn commit_and_decode_agree() {
        for name in ["lexer/Spike.g4", "XMLLexer.g4", "lexer/EofEscape.g4"] {
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
