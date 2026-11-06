//! Runtime side of the static lexer DFA: the packed table image that the
//! walker in [`crate::lexer_atn_simulator::LexerATNSimulator`] tokenizes
//! from.
//!
//! The builder (the `static_dfa_build` module, feature `static-dfa-build`)
//! runs the subset construction and commits each mode to a
//! `StaticLexerDfaBuild`, whose `finalize` freezes those sections into the
//! packed image of a [`StaticLexerDFA`]. Every [`StaticLexerDFA`] accessor is
//! a view over that image: a header of section offsets, then the sections, all
//! addressed by byte offset from the start of the image.
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
//! Generated code embeds the images, not a serialized form of the tables:
//! `StaticLexerDfaBuilder::build` only concatenates the images that `finalize`
//! produced, and [`StaticLexerTables::from_image`] only locates them. There is
//! nothing left to deserialize at load time.
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
//! - `row_of`: `num_states` `u32` entries: the hi row of the state, 0 for
//!   every state when the mode has no hi edges.
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
//! # Image container
//!
//! Generated code embeds one container as a `static [u32; N]`:
//!
//! - word 0: [`STATIC_DFA_MAGIC`].
//! - word 1: [`STATIC_DFA_FORMAT_VERSION`].
//! - word 2: the word count of the whole container.
//! - word 3: the mode count.
//! - then, per mode, two words: the `(offset, len)` span (in words) of the
//!   mode image in the `data` section that follows the mode directory, or
//!   `(u32::MAX, u32::MAX)` when the mode has no table.
//!
//! A mode image is self-describing: it starts with its own magic and format
//! version and its header declares its own word count, so
//! [`StaticLexerTables::from_image`] only has to locate the images.
//!
//! Loading validates the container, the section offsets of every mode, and
//! the table entries the walker dereferences unchecked (edge targets, row
//! indices, continuation indices, action spans). The tables are a
//! compile-time product that generated code carries as constants, so the
//! loader does not re-derive the tables: it only proves that reading them is
//! in bounds.

use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;
use std::mem::{align_of, size_of};

use crate::atn::ATN;
use crate::int_stream::EOF;
use crate::lexer_action::{
    LexerAction, LEXER_ACTION_TYPE_CHANNEL, LEXER_ACTION_TYPE_MODE, LEXER_ACTION_TYPE_MORE,
    LEXER_ACTION_TYPE_POP_MODE, LEXER_ACTION_TYPE_PUSH_MODE, LEXER_ACTION_TYPE_SKIP,
    LEXER_ACTION_TYPE_TYPE,
};

/// First word of an image container: ASCII "SDFA".
pub const STATIC_DFA_MAGIC: u32 = 0x5344_4641;
/// Version of the packed image format.
pub const STATIC_DFA_FORMAT_VERSION: u32 = 2;
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
/// Accel discriminant: run over the ASCII symbols of a 128-bit bitmap.
pub const ACCEL_POSITIVE: u32 = 1;
/// Accel discriminant: run over all input except one needle byte.
pub const ACCEL_EXCLUDE: u32 = 2;
/// Accel discriminant: run over all input except two needle bytes.
pub const ACCEL_EXCLUDE2: u32 = 3;
/// Accel discriminant: run over all input except three needle bytes.
pub const ACCEL_EXCLUDE3: u32 = 4;
/// Accel discriminant: counted nested run over equal-length 1..=3-byte opener and closer.
pub const ACCEL_COUNTED_EXCLUSION: u32 = 5;

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
/// Words of one [`LexerAction`] record in the `actions` section.
const ACTION_WORDS: usize = size_of::<LexerAction>() / 4;
/// Words of one [`Accel`] record in the `accel_data` section.
const ACCEL_WORDS: usize = size_of::<Accel>() / 4;

/// Index to all static lexer DFA tables of a grammar, one entry per lexer mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StaticLexerTables {
    pub(crate) modes: &'static [[u32; 2]],
    pub(crate) data: &'static [u32],
}

/// The table of one lexer mode: a zero-copy view over its packed image.
///
/// The image is the self-contained word array that `StaticLexerDfaBuild::finalize`
/// freezes the sections into: a `Header` of section offsets, then the sections
/// (see the module documentation).
/// Generated code embeds those words as a `static [u32; N]`, so a table that
/// [`StaticLexerTables::from_image`] loads from such an array borrows it.
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

#[cfg(feature = "static-dfa-build")]
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
        let start = config.actions_off as usize;
        &self.actions[start..start + config.actions_len as usize]
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
#[cfg(any(target_endian = "big", feature = "static-dfa-build"))]
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
    Positive([u32; 4]) = ACCEL_POSITIVE,
    /// Run over all input until the first occurrence of the needle byte.
    Exclude(u32) = ACCEL_EXCLUDE,
    /// Run over all input until the first occurrence of either needle byte.
    Exclude2(u32, u32) = ACCEL_EXCLUDE2,
    /// Run over all input until the first occurrence of any of the three needle bytes.
    Exclude3(u32, u32, u32) = ACCEL_EXCLUDE3,
    /// Counted nested run over equal-length 1..=3-byte opener and closer delimiters.
    CountedExclusion {
        /// ASCII bytes of the opening delimiter (zero-padded).
        opener: [u32; 3],
        /// ASCII bytes of the closing delimiter (zero-padded).
        closer: [u32; 3],
        /// Byte length of `opener` and `closer` (`1..=3`).
        n: u32,
    } = ACCEL_COUNTED_EXCLUSION,
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
    #[cfg(feature = "static-dfa-build")]
    pub(crate) fn to_image_words(self) -> [u32; ACCEL_WORDS] {
        match self {
            Accel::Positive([w0, w1, w2, w3]) => [ACCEL_POSITIVE, w0, w1, w2, w3, 0, 0, 0],
            Accel::Exclude(n0) => [ACCEL_EXCLUDE, n0, 0, 0, 0, 0, 0, 0],
            Accel::Exclude2(n0, n1) => [ACCEL_EXCLUDE2, n0, n1, 0, 0, 0, 0, 0],
            Accel::Exclude3(n0, n1, n2) => [ACCEL_EXCLUDE3, n0, n1, n2, 0, 0, 0, 0],
            Accel::CountedExclusion {
                opener: [o0, o1, o2],
                closer: [c0, c1, c2],
                n,
            } => [ACCEL_COUNTED_EXCLUSION, o0, o1, o2, c0, c1, c2, n],
        }
    }
}

impl StaticLexerDFA {
    /// The packed-image words of this table.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub(crate) fn words(&self) -> &[u32] {
        &self.data
    }

    #[inline]
    fn header(&self) -> &Header {
        // SAFETY: a `StaticLexerDFA` only ever wraps an image whose first
        // `HEADER_WORDS` words are a header: `finalize` writes one, and
        // `from_image` refuses shorter images. A `u32` has no invalid bit
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
    ///
    /// The loader validates that the offsets are monotonic and inside
    /// `escape_ranges`, so the range is a valid slice index. A state id that
    /// is out of range (which the walker never forms) yields an empty row.
    #[inline]
    fn escape_row(&self, s: u16) -> std::ops::Range<usize> {
        let off = self.escape_range_offsets();
        let start = off.get(s as usize).copied().unwrap_or(0) as usize;
        let end = off.get(s as usize + 1).copied().unwrap_or(0) as usize;
        start..end
    }

    /// Panics unless `s` names a table state.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    fn check_state(&self, s: u16) {
        assert!(
            (s as usize) < self.header().num_states as usize,
            "state {s} outside the {} states of the table",
            self.header().num_states
        );
    }

    /// Panics unless `index` names a continuation of the pool.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    fn check_continuation_index(&self, index: u32) {
        assert!(
            (index as usize) < self.header().continuations_len as usize,
            "continuation {index} outside the {} continuations of the table",
            self.header().continuations_len
        );
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
        // A class search over `class_starts`, then two loads: the row of the
        // state and the entry of the class. The bounds also cover modes
        // without hi edges (an empty class-start table) and symbols past
        // MAX_SYMBOL.
        let symbol = t as u32;
        let header = self.header();
        if header.num_classes == 0 || symbol > MAX_SYMBOL {
            return NO_EDGE;
        }
        let class = class_of(self.class_starts(), symbol);
        // Unchecked-index safety: every state id the walker forms is in
        // bounds (see above).
        let row = unsafe { *self.row_of().get_unchecked(s as usize) } as usize;
        self.hi_rows()
            .get(row * header.num_classes as usize + class)
    }

    /// Raw edge entry of state `s` on input symbol `t`: see
    /// [`StaticLexerDFA::edge_entry_impl`].
    ///
    /// Panics on a state id outside the table; the walker of a shipped lexer
    /// uses the unchecked `pub(crate)` form.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn edge_entry(&self, s: u16, t: i32) -> u16 {
        self.check_state(s);
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
    ///
    /// Panics on a state id outside the table; the walker of a shipped lexer
    /// uses the unchecked `pub(crate)` form.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn accept_type(&self, s: u16) -> i32 {
        self.check_state(s);
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

    /// The position-independent lexer actions to execute when accepting in
    /// state `s`, with the `'static` lifetime of a borrowed runtime image.
    #[inline]
    pub(crate) fn accept_actions_static(&self, s: u16) -> &'static [LexerAction] {
        let Cow::Borrowed(words) = self.data else {
            unreachable!("runtime static lexer tables borrow a 'static image")
        };
        let header = self.header();
        let spans = self.records::<ActionSpan>(header.accept_actions, header.num_states as usize);
        // Unchecked-index safety: `accept_actions` has `num_states` entries
        // and every state id the walker forms is below `num_states`.
        let span = unsafe { *spans.get_unchecked(s as usize) };
        let actions = unsafe {
            let start =
                (words.as_ptr() as *const u8).add(header.actions as usize) as *const LexerAction;
            std::slice::from_raw_parts(start, header.actions_len as usize)
        };
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
        // A builder table has a range for every non-EOF ESCAPE entry, but
        // the loader does not enforce coverage: a miss reads as a budget
        // escape, which stays correct.
        self.escape_range(s, symbol)
            .map_or(NO_CONTINUATION, |r| r.continuation)
    }

    /// The continuation index of the escape edge of state `s` on `symbol`:
    /// see [`StaticLexerDFA::escape_continuation_impl`].
    ///
    /// Panics on a state id outside the table; the walker of a shipped lexer
    /// uses the unchecked `pub(crate)` form.
    #[cfg(feature = "static-dfa-build")]
    #[inline]
    pub fn escape_continuation(&self, s: u16, symbol: i32) -> u32 {
        self.check_state(s);
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
        let row = self.escape_ranges_of().get(self.escape_row(s))?;
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
        let ctxs_end = continuation.ctxs_off as usize + continuation.ctxs_len as usize;
        let configs_end = continuation.configs_off as usize + continuation.configs_len as usize;
        ContinuationView {
            kind: continuation.kind,
            ctxs: &ctxs[continuation.ctxs_off as usize..ctxs_end],
            configs: &configs[continuation.configs_off as usize..configs_end],
            actions,
        }
    }
}

/// `true` when `words` is the canonical 4-word `#[repr(C, u32)]` image
/// representation of a position-independent [`LexerAction`], so casting the
/// `actions` section to `&[LexerAction]` is sound.
fn action_words_are_valid(words: &[u32; ACTION_WORDS]) -> bool {
    let [tag, w1, w2, w3] = *words;
    match tag as i32 {
        LEXER_ACTION_TYPE_CHANNEL
        | LEXER_ACTION_TYPE_MODE
        | LEXER_ACTION_TYPE_PUSH_MODE
        | LEXER_ACTION_TYPE_TYPE => w2 == 0 && w3 == 0,
        LEXER_ACTION_TYPE_MORE | LEXER_ACTION_TYPE_POP_MODE | LEXER_ACTION_TYPE_SKIP => {
            w1 == 0 && w2 == 0 && w3 == 0
        }
        _ => false,
    }
}

/// `true` when `words` is a valid, canonical 8-word `#[repr(C, u32)]` image
/// representation of an [`Accel`] descriptor, so casting the `accel_data`
/// section to `&[Accel]` and running `run_scan` on it is sound.
fn accel_words_are_valid(words: &[u32; ACCEL_WORDS]) -> bool {
    let [tag, w0, w1, w2, w3, w4, w5, w6] = *words;
    match tag {
        ACCEL_POSITIVE => w4 == 0 && w5 == 0 && w6 == 0,
        ACCEL_EXCLUDE => w0 < 0x80 && [w1, w2, w3, w4, w5, w6] == [0; 6],
        ACCEL_EXCLUDE2 => w0 < 0x80 && w1 < 0x80 && [w2, w3, w4, w5, w6] == [0; 5],
        ACCEL_EXCLUDE3 => w0 < 0x80 && w1 < 0x80 && w2 < 0x80 && [w3, w4, w5, w6] == [0; 4],
        ACCEL_COUNTED_EXCLUSION => {
            let opener = [w0, w1, w2];
            let closer = [w3, w4, w5];
            let n = w6 as usize;
            (1..=3).contains(&w6)
                && opener != closer
                && opener[..n].iter().all(|&byte| byte < 0x80)
                && closer[..n].iter().all(|&byte| byte < 0x80)
                && opener[n..].iter().all(|&byte| byte == 0)
                && closer[n..].iter().all(|&byte| byte == 0)
        }
        _ => false,
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
#[cfg(feature = "static-dfa-build")]
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

#[cfg(feature = "static-dfa-build")]
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
        // The sections are the only source of the tables, so an image that
        // the loader would reject is a builder bug, not a runtime condition.
        // (The mode index only numbers the error payload.)
        table
            .header()
            .validate(0, table.data.len())
            .and_then(|()| table.validate_entries(0))
            .expect("the builder sections form a valid image");
        table
    }
}

#[cfg(feature = "static-dfa-build")]
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
#[cfg(feature = "static-dfa-build")]
#[inline]
fn image_offset(words: &[u32]) -> u32 {
    (words.len() * 4) as u32
}

/// Appends a `u16` table, two entries per word.
#[cfg(feature = "static-dfa-build")]
fn push_packed_u16(words: &mut Vec<u32>, values: &[u16]) {
    let start = words.len();
    words.resize(start + packed_u16_words(values.len()), 0);
    for (index, &value) in values.iter().enumerate() {
        words[start + (index >> 1)] |= u32::from(value) << ((index & 1) * 16);
    }
}

/// Appends a `#[repr(C)]` table of 4-byte words.
#[cfg(feature = "static-dfa-build")]
fn push_records<T: Copy>(words: &mut Vec<u32>, records: &[T]) {
    use std::mem::size_of_val;

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

/// Failure while loading a packed static lexer DFA image.
///
/// The tables are a compile-time product that generated code carries as
/// constants, so these are format errors of the image itself, not conditions
/// a correct build can produce.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StaticLexerError {
    /// Word 0 of the image is not the static DFA magic value.
    BadMagic {
        /// The value found in word 0.
        found: u32,
    },
    /// A mode image does not start with the image magic.
    BadModeMagic {
        /// The mode index.
        mode: usize,
        /// The value found in word 0 of the mode image.
        found: u32,
    },
    /// The word count of the image does not match the image length.
    TotalLengthMismatch {
        /// The declared word count.
        declared: usize,
        /// The actual image length, in words.
        actual: usize,
    },
    /// The image ends before the container preamble declares data.
    TruncatedImage {
        /// The word count the image must have at least.
        expected_at_least: usize,
        /// The actual image length, in words.
        actual: usize,
    },
    /// A mode image, or a section of it, leaves the image it belongs to.
    OutOfBounds {
        /// The mode index.
        mode: usize,
        /// What leaves its image.
        what: &'static str,
        /// The end offset of the data, in bytes.
        end: usize,
        /// The length of the image, in bytes.
        total: usize,
    },
    /// A table of the image breaks an invariant of the format.
    Malformed {
        /// The mode index.
        mode: usize,
        /// What is malformed.
        what: &'static str,
    },
    /// A table entry points outside the table it addresses.
    InvalidReference {
        /// The mode index.
        mode: usize,
        /// What the entry addresses.
        what: &'static str,
        /// The value of the entry.
        value: u32,
        /// The number of valid entries.
        count: usize,
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
            StaticLexerError::BadModeMagic { mode, found } => {
                write!(
                    f,
                    "mode {mode}: bad image magic 0x{found:08x}; expected 0x{IMAGE_MAGIC:08x}"
                )
            }
            StaticLexerError::TotalLengthMismatch { declared, actual } => {
                write!(
                    f,
                    "declared total length {declared} does not match image length {actual}"
                )
            }
            StaticLexerError::TruncatedImage {
                expected_at_least,
                actual,
            } => {
                write!(
                    f,
                    "image has {actual} words; expected at least {expected_at_least}"
                )
            }
            StaticLexerError::OutOfBounds {
                mode,
                what,
                end,
                total,
            } => {
                write!(
                    f,
                    "mode {mode}: {what} ends at byte {end} of a {total}-byte image"
                )
            }
            StaticLexerError::Malformed { mode, what } => {
                write!(f, "mode {mode}: {what} is malformed")
            }
            StaticLexerError::InvalidReference {
                mode,
                what,
                value,
                count,
            } => {
                write!(f, "mode {mode}: {what} {value} outside 0..{count}")
            }
        }
    }
}

impl std::error::Error for StaticLexerError {}

/// The concatenated continuation sections of one mode: the context tables,
/// the configs, and the action traces of all continuations, in pool order.
#[cfg(feature = "static-dfa-build")]
#[derive(Debug, Default)]
pub(crate) struct ContinuationPools {
    pub(crate) ctxs: Vec<ContinuationContext>,
    pub(crate) configs: Vec<ContinuationConfig>,
    pub(crate) actions: Vec<u32>,
}

#[cfg(feature = "static-dfa-build")]
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
        // The configs name their actions by offset into this pool, so patch
        // them up as they land. A config without actions always names the
        // empty span.
        for config in &mut self.configs[configs_off as usize..] {
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
    /// Wraps the packed image that generated code embeds: a zero-copy view
    /// over the `static` words of the lexer.
    ///
    /// The container is validated against its preamble, and the sections and
    /// dereferenced entries of every mode image are validated against their
    /// tables. The tables themselves are not re-derived: they are the exact
    /// bytes the builder produced.
    ///
    /// Panics on a format-version mismatch: the words are generated together
    /// with the code that embeds them, so a version mismatch is a build
    /// inconsistency, not a runtime condition. Regenerate the lexer with the
    /// matching tool version.
    pub fn from_image(image: &'static [u32]) -> Result<Self, StaticLexerError> {
        const PREAMBLE_WORDS: usize = 4;
        const MODE_WORDS: usize = 2;
        if image.len() < PREAMBLE_WORDS {
            return Err(StaticLexerError::TruncatedImage {
                expected_at_least: PREAMBLE_WORDS,
                actual: image.len(),
            });
        }
        if image[0] != STATIC_DFA_MAGIC {
            return Err(StaticLexerError::BadMagic { found: image[0] });
        }
        let version = image[1];
        assert_eq!(
            version, STATIC_DFA_FORMAT_VERSION,
            "static lexer DFA table format version mismatch (found {version}, expected \
             {STATIC_DFA_FORMAT_VERSION}): the table was generated with a different tool \
             version; regenerate the lexer tables"
        );
        let total = image[2] as usize;
        if total != image.len() {
            return Err(StaticLexerError::TotalLengthMismatch {
                declared: total,
                actual: image.len(),
            });
        }
        let num_modes = image[3] as usize;
        let modes_end = num_modes
            .checked_mul(MODE_WORDS)
            .and_then(|words| words.checked_add(PREAMBLE_WORDS))
            .filter(|&end| end <= image.len());
        let Some(modes_end) = modes_end else {
            return Err(StaticLexerError::OutOfBounds {
                mode: 0,
                what: "the mode table",
                end: num_modes
                    .saturating_mul(MODE_WORDS)
                    .saturating_add(PREAMBLE_WORDS)
                    .saturating_mul(4),
                total: image.len() * 4,
            });
        };
        let modes = image[PREAMBLE_WORDS..modes_end].as_chunks::<MODE_WORDS>().0;
        let data = &image[modes_end..];
        // The mode images sit back to back in `data` and together cover it,
        // so a container has exactly one encoding.
        let mut next_offset = 0usize;
        for (mode, &[offset, len]) in modes.iter().enumerate() {
            if offset == u32::MAX {
                if len != u32::MAX {
                    return Err(StaticLexerError::Malformed {
                        mode,
                        what: "the mode table entry of an absent mode",
                    });
                }
                continue;
            }
            if offset as usize != next_offset {
                return Err(StaticLexerError::Malformed {
                    mode,
                    what: "the mode table entry",
                });
            }
            validate_mode_image(data, mode, offset, len)?;
            next_offset += len as usize;
        }
        if next_offset != data.len() {
            return Err(StaticLexerError::TotalLengthMismatch {
                declared: modes_end + next_offset,
                actual: image.len(),
            });
        }
        Ok(StaticLexerTables { modes, data })
    }

    /// The table for lexer mode `m`, if the builder compiled one.
    #[inline]
    pub fn mode(&self, m: usize) -> Option<StaticLexerDFA> {
        self.modes.get(m).and_then(|&[offset, len]| {
            if offset == u32::MAX {
                None
            } else {
                let start = offset as usize;
                Some(StaticLexerDFA {
                    data: Cow::Borrowed(&self.data[start..start + len as usize]),
                })
            }
        })
    }

    /// Number of lexer modes in the container, including modes without a
    /// table.
    pub fn num_modes(&self) -> usize {
        self.modes.len()
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
        for m in 0..self.num_modes() {
            let Some(mode) = self.mode(m) else { continue };
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
        self.check_continuation_index(index);
        self.continuation(index).kind()
    }

    /// The number of configs of continuation `index`.
    pub fn continuation_num_configs(&self, index: u32) -> usize {
        self.check_continuation_index(index);
        self.continuation(index).configs().len()
    }

    /// The accept action list of state `s` as ATN `(type, data1, data2)`
    /// triples, in order.
    pub fn accept_action_triples(&self, s: u16) -> Vec<(i32, i32, i32)> {
        self.check_state(s);
        self.accept_actions(s)
            .iter()
            .map(LexerAction::to_serialized)
            .collect()
    }

    /// The escape ranges of state `s` as `(lo, hi, continuation)` triples,
    /// sorted and disjoint (`NO_CONTINUATION` marks a budget escape).
    pub fn escape_ranges(&self, s: u16) -> Vec<(i32, i32, u32)> {
        self.check_state(s);
        let row = self
            .escape_ranges_of()
            .get(self.escape_row(s))
            .unwrap_or_default();
        row.iter().map(|r| (r.lo, r.hi, r.continuation)).collect()
    }

    /// Run-acceleration descriptor of state `s`, if any.
    pub fn state_accel(&self, s: u16) -> Option<Accel> {
        self.check_state(s);
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
        self.check_continuation_index(index);
        self.continuation(index).ctxs().len()
    }

    /// Number of non-greedy configs in continuation `index`.
    pub fn continuation_num_non_greedy_configs(&self, index: u32) -> usize {
        self.check_continuation_index(index);
        self.continuation(index)
            .configs()
            .iter()
            .filter(|config| config.non_greedy())
            .count()
    }

    /// ATN state numbers of the configs in continuation `index`, in config order.
    pub fn continuation_config_states(&self, index: u32) -> Vec<usize> {
        self.check_continuation_index(index);
        self.continuation(index)
            .configs()
            .iter()
            .map(|config| config.state as usize)
            .collect()
    }
}

/// Expands per-row `(lo_class, hi_class, target)` runs into dense
/// `num_classes`-wide rows, `NO_EDGE`-filled.
#[cfg(feature = "static-dfa-build")]
pub(crate) fn dense_hi_rows(row_runs: &[Vec<(u32, u32, u16)>], num_classes: usize) -> Vec<u16> {
    let mut hi_rows = vec![NO_EDGE; row_runs.len() * num_classes];
    for (row, runs) in row_runs.iter().enumerate() {
        for &(lo, hi, target) in runs {
            hi_rows[row * num_classes + lo as usize..=row * num_classes + hi as usize].fill(target);
        }
    }
    hi_rows
}

/// Validates the image of one mode in `data` at `(offset, len)` words.
fn validate_mode_image(
    data: &'static [u32],
    mode: usize,
    offset: u32,
    len: u32,
) -> Result<(), StaticLexerError> {
    let offset = offset as usize;
    let len = len as usize;
    let words = offset
        .checked_add(len)
        .and_then(|end| data.get(offset..end))
        .ok_or(StaticLexerError::OutOfBounds {
            mode,
            what: "the mode image",
            end: offset.saturating_add(len).saturating_mul(4),
            total: data.len().saturating_mul(4),
        })?;
    match words.first() {
        Some(&magic) if magic == IMAGE_MAGIC => {}
        Some(&found) => return Err(StaticLexerError::BadModeMagic { mode, found }),
        None => {
            return Err(StaticLexerError::OutOfBounds {
                mode,
                what: "the mode header",
                end: offset.saturating_add(HEADER_WORDS).saturating_mul(4),
                total: data.len().saturating_mul(4),
            });
        }
    }
    if len < HEADER_WORDS {
        return Err(StaticLexerError::OutOfBounds {
            mode,
            what: "the mode header",
            end: offset.saturating_add(HEADER_WORDS).saturating_mul(4),
            total: data.len().saturating_mul(4),
        });
    }
    let table = StaticLexerDFA {
        data: Cow::Borrowed(words),
    };
    table.header().validate(mode, words.len())?;
    table.validate_entries(mode)
}

impl Header {
    /// Checks the image version, the declared length, and that every section
    /// is inside the `words` the table borrows.
    fn validate(&self, mode: usize, words: usize) -> Result<(), StaticLexerError> {
        assert_eq!(
            self.version, IMAGE_VERSION,
            "static lexer DFA image version mismatch (found {}, expected {IMAGE_VERSION}): \
             the table was generated with a different tool version; regenerate the lexer tables",
            self.version
        );
        if self.total_words as usize != words {
            return Err(StaticLexerError::TotalLengthMismatch {
                declared: self.total_words as usize,
                actual: words,
            });
        }
        self.check_sections(mode, words)
    }

    /// Checks that every section of the image is in bounds and word aligned.
    fn check_sections(&self, mode: usize, words: usize) -> Result<(), StaticLexerError> {
        // The borrowed length is what the checks compare against, so a
        // section that does not fit is rejected before any accessor slices
        // it.
        let total = words as u64 * 4;
        let bytes = |entries: u64, size: u64| entries.checked_mul(size);
        let num_states = u64::from(self.num_states);
        let num_classes = u64::from(self.num_classes);
        let num_hi_rows = u64::from(self.num_hi_rows);
        let packed = |entries: u64| entries.div_ceil(2).checked_mul(4);
        let sections: [(&'static str, u32, Option<u64>); 16] = [
            ("low", self.low, packed(num_states * 129)),
            ("class_starts", self.class_starts, bytes(num_classes, 4)),
            ("hi_rows", self.hi_rows, packed(num_hi_rows * num_classes)),
            ("row_of", self.row_of, bytes(num_states, 4)),
            ("accept_type", self.accept_type, bytes(num_states, 4)),
            (
                "accept_actions",
                self.accept_actions,
                bytes(num_states, size_of::<ActionSpan>() as u64),
            ),
            (
                "actions",
                self.actions,
                bytes(u64::from(self.actions_len), size_of::<LexerAction>() as u64),
            ),
            ("accel_tag", self.accel_tag, packed(num_states)),
            (
                "accel_data",
                self.accel_data,
                bytes(u64::from(self.accel_data_len), size_of::<Accel>() as u64),
            ),
            ("eof_escape", self.eof_escape, bytes(num_states, 4)),
            ("escape_off", self.escape_off, bytes(num_states + 1, 4)),
            (
                "escape_ranges",
                self.escape_ranges,
                bytes(
                    u64::from(self.escape_ranges_len),
                    size_of::<EscapeRange>() as u64,
                ),
            ),
            (
                "continuations",
                self.continuations,
                bytes(
                    u64::from(self.continuations_len),
                    size_of::<LexerContinuation>() as u64,
                ),
            ),
            (
                "continuation_ctxs",
                self.continuation_ctxs,
                bytes(
                    u64::from(self.continuation_ctxs_len),
                    size_of::<ContinuationContext>() as u64,
                ),
            ),
            (
                "continuation_configs",
                self.continuation_configs,
                bytes(
                    u64::from(self.continuation_configs_len),
                    size_of::<ContinuationConfig>() as u64,
                ),
            ),
            (
                "continuation_actions",
                self.continuation_actions,
                bytes(u64::from(self.continuation_actions_len), 4),
            ),
        ];
        for (what, offset, len) in sections {
            let start = u64::from(offset);
            let in_bounds = len
                .and_then(|len| start.checked_add(len))
                .is_some_and(|end| end <= total);
            if start % 4 != 0 || !in_bounds {
                return Err(StaticLexerError::OutOfBounds {
                    mode,
                    what,
                    end: usize::try_from(start.saturating_add(len.unwrap_or(u64::MAX)))
                        .unwrap_or(usize::MAX),
                    total: usize::try_from(total).unwrap_or(usize::MAX),
                });
            }
        }
        Ok(())
    }
}

impl StaticLexerDFA {
    /// Checks the table entries that the walker dereferences without bounds
    /// checks: edge targets stay below the state count, row indices below the
    /// row count, and continuation, action, and escape indices below their
    /// tables.
    fn validate_entries(&self, mode: usize) -> Result<(), StaticLexerError> {
        let header = self.header();
        let num_states = header.num_states as usize;
        if num_states == 0 || num_states > MAX_STATE_COUNT {
            return Err(StaticLexerError::InvalidReference {
                mode,
                what: "the state count",
                value: header.num_states,
                count: MAX_STATE_COUNT,
            });
        }
        if header.start_state as usize >= num_states {
            return Err(StaticLexerError::InvalidReference {
                mode,
                what: "the start state",
                value: header.start_state,
                count: num_states,
            });
        }
        // `class_of` subtracts one from the partition point, so class 0 must
        // start at the first hi symbol and the bounds must be sorted.
        let class_starts = self.class_starts();
        if !class_starts.is_empty()
            && (class_starts[0] != ASCII_ROW as u32
                || !class_starts.windows(2).all(|pair| pair[0] < pair[1])
                || class_starts[class_starts.len() - 1] > MAX_SYMBOL)
        {
            return Err(StaticLexerError::Malformed {
                mode,
                what: "the class starts",
            });
        }
        self.validate_states(mode, num_states)?;
        self.validate_escape_indices(mode)?;
        self.validate_continuations(mode)
    }

    /// Checks the edge entries and the hi-row index of every state.
    fn validate_states(&self, mode: usize, num_states: usize) -> Result<(), StaticLexerError> {
        let header = self.header();
        let accept_type = self.accept_types();
        let check_entry = |entry: u16| -> Result<(), StaticLexerError> {
            if entry == NO_EDGE || entry == ESCAPE {
                return Ok(());
            }
            let target = (entry & STATE_MASK) as usize;
            if target >= num_states {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "an edge target",
                    value: u32::from(entry & STATE_MASK),
                    count: num_states,
                });
            }
            if (entry & ACCEPT_BIT != 0) != (accept_type[target] >= 0) {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "the accept bit of edge entry",
                    value: u32::from(entry),
                    count: num_states,
                });
            }
            Ok(())
        };
        let low = self.low();
        for index in 0..num_states * LOW_ROW {
            check_entry(low.get(index))?;
        }
        let hi_rows = self.hi_rows();
        let num_hi_rows = header.num_hi_rows as usize;
        let num_classes = header.num_classes as usize;
        for index in 0..num_hi_rows * num_classes {
            check_entry(hi_rows.get(index))?;
        }
        for &row in self.row_of() {
            if num_classes > 0 && row as usize >= num_hi_rows {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "a hi row index",
                    value: row,
                    count: num_hi_rows,
                });
            }
        }
        Ok(())
    }

    /// Checks the escape rows, the accept action spans, and the accel tags.
    fn validate_escape_indices(&self, mode: usize) -> Result<(), StaticLexerError> {
        let header = self.header();
        let num_states = header.num_states as usize;
        let num_continuations = header.continuations_len as usize;
        let check_continuation = |value: u32| -> Result<(), StaticLexerError> {
            if value != NO_CONTINUATION && value as usize >= num_continuations {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "a continuation index",
                    value,
                    count: num_continuations,
                });
            }
            Ok(())
        };
        for &value in self.eof_escapes() {
            check_continuation(value)?;
        }
        for range in self.escape_ranges_of() {
            check_continuation(range.continuation)?;
        }
        let mut previous = 0u32;
        for &offset in self.escape_range_offsets() {
            if offset < previous || offset as usize > header.escape_ranges_len as usize {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "an escape range offset",
                    value: offset,
                    count: header.escape_ranges_len as usize,
                });
            }
            previous = offset;
        }
        let spans = self.records::<ActionSpan>(header.accept_actions, num_states);
        for span in spans {
            let end = (span.off as usize).checked_add(span.len as usize);
            if end.is_none_or(|end| end > header.actions_len as usize) {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "an accept action span",
                    value: span.off,
                    count: header.actions_len as usize,
                });
            }
        }
        let accel_tags = self.accel_tags();
        for state in 0..num_states {
            let tag = accel_tags.get(state);
            if tag as usize > header.accel_data_len as usize {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "an accel tag",
                    value: u32::from(tag),
                    count: header.accel_data_len as usize,
                });
            }
        }
        for words in
            self.records::<[u32; ACTION_WORDS]>(header.actions, header.actions_len as usize)
        {
            if !action_words_are_valid(words) {
                return Err(StaticLexerError::Malformed {
                    mode,
                    what: "an accept action",
                });
            }
        }
        for words in
            self.records::<[u32; ACCEL_WORDS]>(header.accel_data, header.accel_data_len as usize)
        {
            if !accel_words_are_valid(words) {
                return Err(StaticLexerError::Malformed {
                    mode,
                    what: "an accel payload",
                });
            }
        }
        Ok(())
    }

    /// Checks the continuation pool: kinds, spans, and action traces.
    fn validate_continuations(&self, mode: usize) -> Result<(), StaticLexerError> {
        let header = self.header();
        let num_continuations = header.continuations_len as usize;
        let pool = self.records::<LexerContinuation>(header.continuations, num_continuations);
        for continuation in pool {
            if continuation.kind != CONTINUATION_MOVED_CONFIGS
                && continuation.kind != CONTINUATION_SOURCE_SET
            {
                return Err(StaticLexerError::Malformed {
                    mode,
                    what: "a continuation kind",
                });
            }
            let ctxs_end = (continuation.ctxs_off as usize)
                .checked_add(continuation.ctxs_len as usize)
                .filter(|&end| end <= header.continuation_ctxs_len as usize);
            if ctxs_end.is_none() {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "a continuation context span",
                    value: continuation.ctxs_off,
                    count: header.continuation_ctxs_len as usize,
                });
            }
            let configs_end = (continuation.configs_off as usize)
                .checked_add(continuation.configs_len as usize)
                .filter(|&end| end <= header.continuation_configs_len as usize);
            if configs_end.is_none() {
                return Err(StaticLexerError::InvalidReference {
                    mode,
                    what: "a continuation config span",
                    value: continuation.configs_off,
                    count: header.continuation_configs_len as usize,
                });
            }
            let ctxs = &self.records::<ContinuationContext>(
                header.continuation_ctxs,
                header.continuation_ctxs_len as usize,
            )[continuation.ctxs_off as usize..ctxs_end.unwrap()];
            for (index, ctx) in ctxs.iter().enumerate() {
                // Entry `i` has local id `i + 1`; parents refer to smaller ids.
                if ctx.parent as usize > index {
                    return Err(StaticLexerError::InvalidReference {
                        mode,
                        what: "a continuation context parent",
                        value: ctx.parent,
                        count: index + 1,
                    });
                }
            }
            let configs = &self.records::<ContinuationConfig>(
                header.continuation_configs,
                header.continuation_configs_len as usize,
            )[continuation.configs_off as usize..configs_end.unwrap()];
            for config in configs {
                if config.ctx as usize > continuation.ctxs_len as usize {
                    return Err(StaticLexerError::InvalidReference {
                        mode,
                        what: "a continuation config context",
                        value: config.ctx,
                        count: continuation.ctxs_len as usize + 1,
                    });
                }
                let actions_end =
                    (config.actions_off as usize).checked_add(config.actions_len as usize);
                if actions_end.is_none_or(|end| end > header.continuation_actions_len as usize) {
                    return Err(StaticLexerError::InvalidReference {
                        mode,
                        what: "a continuation config action span",
                        value: config.actions_off,
                        count: header.continuation_actions_len as usize,
                    });
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

    /// The builder's words are the image that generated code embeds, so they
    /// can back a borrowed table: loading them gives the committed tables
    /// back.
    #[test]
    fn image_round_trip() {
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
                let image: &'static [u32] = Box::leak(build.words.clone().into_boxed_slice());
                let decoded = StaticLexerTables::from_image(image)
                    .unwrap_or_else(|e| panic!("{name}: the builder's image loads: {e}"));
                assert_eq!(decoded.num_modes(), build.modes.len(), "{name}: mode count");
                for m in 0..build.modes.len() {
                    assert_eq!(
                        decoded.mode(m).as_ref(),
                        build.table(m),
                        "{name}: the loaded image is the committed table (mode {m})"
                    );
                }
            }
        }
    }

    /// The accel tag is a `u16`, not a `u8`: a mode with 300 accelerated
    /// states keeps every payload apart across the image.
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
        let mut words = vec![
            STATIC_DFA_MAGIC,
            STATIC_DFA_FORMAT_VERSION,
            6 + table.words().len() as u32,
            1,
            0,
            table.words().len() as u32,
        ];
        words.extend_from_slice(table.words());
        let image: &'static [u32] = Box::leak(words.into_boxed_slice());
        let tables = StaticLexerTables::from_image(image).expect("the image loads");
        assert_eq!(tables.num_modes(), 1);
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

    /// The loader validates what the accessors assume: a table entry that
    /// points outside its table is a load error, not a walker concern.
    #[test]
    fn out_of_range_entries_are_load_errors() {
        let path = fixture("CSV.g4");
        let atn = compile_tables(&path);
        let build = StaticLexerDfaBuilder::new().build(&atn);
        let image: Vec<u32> = build.words.clone();

        // The mode table sits right after the 4-word preamble.
        let mode = 6 + image[4] as usize;
        let num_states = image[mode + 3] as usize;
        let low = image[mode + 7] as usize / 4;
        let accept_bit = u32::from(ACCEPT_BIT);

        // An edge target past the state count.
        let mut words = image.clone();
        words[mode + low + 1] = accept_bit | num_states as u32;
        let leaked: &'static [u32] = Box::leak(words.into_boxed_slice());
        assert!(matches!(
            StaticLexerTables::from_image(leaked),
            Err(StaticLexerError::InvalidReference { .. })
        ));

        // An edge entry whose accept bit disagrees with the target.
        let mut words = image.clone();
        let low_words = (num_states * LOW_ROW).div_ceil(2);
        let index = (mode + low..mode + low + low_words)
            .find(|&index| {
                let entry = words[index] as u16;
                entry != NO_EDGE && entry != ESCAPE && entry & ACCEPT_BIT != 0
            })
            .expect("CSV has accept edge entries");
        words[index] ^= accept_bit;
        let leaked: &'static [u32] = Box::leak(words.into_boxed_slice());
        assert!(matches!(
            StaticLexerTables::from_image(leaked),
            Err(StaticLexerError::InvalidReference {
                what: "the accept bit of edge entry",
                ..
            })
        ));

        // Class starts that do not begin at the first hi symbol.
        let mut words = image.clone();
        let num_classes = image[mode + 4] as usize;
        assert!(num_classes > 0, "CSV has hi classes");
        let class_starts = image[mode + 8] as usize / 4;
        words[mode + class_starts] = 129;
        let leaked: &'static [u32] = Box::leak(words.into_boxed_slice());
        assert!(matches!(
            StaticLexerTables::from_image(leaked),
            Err(StaticLexerError::Malformed { .. })
        ));

        // An invalid `Accel` discriminant in `accel_data`.
        let mut words = image.clone();
        let accel_data_len = image[mode + 17] as usize;
        assert!(accel_data_len > 0, "CSV has accelerated states");
        let accel_data = image[mode + 16] as usize / 4;
        words[mode + accel_data] = 99;
        let leaked: &'static [u32] = Box::leak(words.into_boxed_slice());
        assert!(matches!(
            StaticLexerTables::from_image(leaked),
            Err(StaticLexerError::Malformed { .. })
        ));
    }
}
