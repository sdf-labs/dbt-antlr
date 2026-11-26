//! The C entry points.
//!
//! This module defines the entry points that do not depend on the grammars
//! of a bundle. [`export_grammars!`](crate::export_grammars) defines the
//! other entry points in the bundle crate; the `extern "C"` block below
//! declares them, so that `antlr.h` and Rust callers see one complete ABI.

use std::ffi::c_char;
use std::panic::{self, AssertUnwindSafe};
use std::ptr;

use dbt_antlr_runtime::token::{CommonToken, Token};

use crate::grammar::AntlrGrammarInfo;
use crate::overlay::{AntlrToken, AntlrTreeNode};
use crate::session::{AntlrParseSession, AntlrSessionInfo};

/// The type of [`antlr_grammar_lookup`].
pub type GrammarLookupFn = unsafe extern "C" fn(*const c_char, usize) -> i32;
/// The type of [`antlr_grammar_info`].
pub type GrammarInfoFn = unsafe extern "C" fn(i32) -> *const AntlrGrammarInfo;
/// The type of [`antlr_parse`].
pub type ParseEntryFn =
    unsafe extern "C" fn(i32, u32, *const c_char, usize) -> *mut AntlrParseSession;
/// The type of [`antlr_node_to_string_tree`].
pub type NodeToStringTreeFn =
    unsafe extern "C" fn(i32, *const AntlrTreeNode, *mut usize) -> *mut c_char;

// C code uses `AntlrParseSession` only through pointers, as an opaque type.
#[allow(improper_ctypes)]
unsafe extern "C" {
    /// Finds a grammar of the bundle by the name that `export_grammars!`
    /// gives it. `name` points to `name_len` bytes of UTF-8, which need no
    /// NUL terminator.
    ///
    /// Returns the grammar ID, or -1 when the bundle has no such grammar.
    pub fn antlr_grammar_lookup(name: *const c_char, name_len: usize) -> i32;

    /// Describes a grammar of the bundle.
    ///
    /// Returns NULL when `grammar_id` is not valid. The description is
    /// static.
    pub fn antlr_grammar_info(grammar_id: i32) -> *const AntlrGrammarInfo;

    /// Parses `input_len` bytes of UTF-8 at `input` with a grammar of the
    /// bundle, from the rule with index `rule_index`.
    ///
    /// The parse uses the default error strategy: it recovers from syntax
    /// errors and reports them in `AntlrSessionInfo::syntax_errors`. The
    /// session borrows the input: keep the input valid and unchanged until
    /// `antlr_session_free`.
    ///
    /// Returns a session that `antlr_session_free` must free. The status of
    /// the session tells if the parse produced a tree. A NULL `input` is
    /// valid only when `input_len` is 0. Returns NULL only when the parse
    /// failed in an unexpected way.
    pub fn antlr_parse(
        grammar_id: i32,
        rule_index: u32,
        input: *const c_char,
        input_len: usize,
    ) -> *mut AntlrParseSession;

    /// Formats the subtree at `node` in LISP form, for example
    /// `(s (e 1) <EOF>)`. `node` must be a node of a live session of the
    /// grammar `grammar_id`.
    ///
    /// Returns a UTF-8 string of `*out_len` bytes, which
    /// `antlr_string_free` must free. Returns NULL on failure, or when
    /// `node` or `out_len` is NULL.
    pub fn antlr_node_to_string_tree(
        grammar_id: i32,
        node: *const AntlrTreeNode,
        out_len: *mut usize,
    ) -> *mut c_char;
}

const _: GrammarLookupFn = antlr_grammar_lookup;
const _: GrammarInfoFn = antlr_grammar_info;
const _: ParseEntryFn = antlr_parse;
const _: NodeToStringTreeFn = antlr_node_to_string_tree;

/// Returns the result of the parse of `session`. The result stays valid
/// until `antlr_session_free`. Returns NULL when `session` is NULL.
///
/// # Safety
///
/// `session` must be NULL or a live session from `antlr_parse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn antlr_session_info(
    session: *const AntlrParseSession,
) -> *const AntlrSessionInfo {
    // SAFETY: The caller passes NULL or a live session.
    unsafe { session.as_ref() }.map_or(ptr::null(), |session| ptr::from_ref(session.info()))
}

/// Frees a session, its parse tree, and its diagnostics. After this call,
/// all pointers into the session are dangling. A NULL `session` has no
/// effect.
///
/// # Safety
///
/// `session` must be NULL or a live session from `antlr_parse`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn antlr_session_free(session: *mut AntlrParseSession) {
    if session.is_null() {
        return;
    }
    // SAFETY: The caller passes a live session, which `antlr_parse` allocated
    // with `Box::into_raw`.
    let session = unsafe { Box::from_raw(session) };
    // A panic must not unwind into C. Forget the payload, because its drop
    // can also panic.
    if let Err(payload) = panic::catch_unwind(AssertUnwindSafe(move || drop(session))) {
        std::mem::forget(payload);
    }
}

/// Computes the line (1-based) and the column (0-based, in Unicode code
/// points) of the first character of `token`. A conjured token gets the
/// location of the token that the parser conjured it at.
///
/// Returns false when the location is not available, or when a pointer is
/// NULL.
///
/// # Safety
///
/// `token` must be NULL or a token of a live session. `out_line` and
/// `out_column` must be NULL or valid for writes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn antlr_token_location(
    token: *const AntlrToken,
    out_line: *mut u32,
    out_column: *mut i32,
) -> bool {
    if token.is_null() || out_line.is_null() || out_column.is_null() {
        return false;
    }
    // SAFETY: The caller passes a token of a live session.
    let token = unsafe { common_token(token) };
    let Ok((line, column)) = panic::catch_unwind(AssertUnwindSafe(|| {
        (token.get_line(), token.get_char_position_in_line())
    })) else {
        return false;
    };
    // SAFETY: The caller passes pointers that are valid for writes.
    unsafe {
        out_line.write(line);
        out_column.write(column);
    }
    true
}

/// Returns the text of `token` as the runtime computes it: `<EOF>` for the
/// EOF token, the replaced text for tokens with replaced or conjured text
/// (for example `<missing ')'>`), and the input slice for other tokens.
///
/// The text has `*out_len` bytes of UTF-8 and stays valid until
/// `antlr_session_free`. Returns NULL on failure, or when a pointer is NULL.
///
/// # Safety
///
/// `token` must be NULL or a token of a live session. `out_len` must be
/// NULL or valid for writes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn antlr_token_text_fallback(
    token: *const AntlrToken,
    out_len: *mut usize,
) -> *const c_char {
    if token.is_null() || out_len.is_null() {
        return ptr::null();
    }
    // SAFETY: The caller passes a token of a live session.
    let token = unsafe { common_token(token) };
    let (text, len) = match panic::catch_unwind(AssertUnwindSafe(|| token.get_text())) {
        Ok(text) => (text.as_ptr().cast(), text.len()),
        Err(payload) => {
            std::mem::forget(payload);
            (ptr::null(), 0)
        }
    };
    // SAFETY: The caller passes a pointer that is valid for writes.
    unsafe { out_len.write(len) };
    text
}

/// Frees a string from `antlr_node_to_string_tree`. A NULL `text` has no
/// effect.
///
/// # Safety
///
/// `text` must be NULL or a string from `antlr_node_to_string_tree` with its
/// length `len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn antlr_string_free(text: *mut c_char, len: usize) {
    if text.is_null() {
        return;
    }
    // SAFETY: The string came from `Box::<[u8]>::into_raw` with this length.
    drop(unsafe { Box::from_raw(ptr::slice_from_raw_parts_mut(text.cast::<u8>(), len)) });
}

/// Sets the arena allocation limit in bytes for the parses that start after
/// this call, for all grammars of the bundle. 0 removes the limit. A parse
/// that goes above the limit ends with `ANTLR_PARSE_STATUS_FATAL`.
///
/// Returns false and has no effect when the bundle was built without the
/// `arena-allocation-limit` feature of `dbt-antlr-ffi`.
#[unsafe(no_mangle)]
pub extern "C" fn antlr_set_arena_allocation_limit(limit_bytes: usize) -> bool {
    #[cfg(feature = "arena-allocation-limit")]
    {
        dbt_antlr_runtime::set_default_arena_alloc_limit(limit_bytes);
        true
    }
    #[cfg(not(feature = "arena-allocation-limit"))]
    {
        let _ = limit_bytes;
        false
    }
}

/// The token behind an overlay pointer.
///
/// # Safety
///
/// `token` must be a token of a live session.
unsafe fn common_token<'a>(token: *const AntlrToken) -> &'a CommonToken<'a> {
    // SAFETY: `AntlrToken` mirrors the layout of `CommonToken`, and the caller
    // passes a token of a live session.
    unsafe { &*token.cast::<CommonToken<'a>>() }
}

/// Converts a string into a buffer for `antlr_string_free`.
///
/// # Safety
///
/// `out_len` must be valid for writes.
pub(crate) unsafe fn into_c_string(text: String, out_len: *mut usize) -> *mut c_char {
    let bytes = text.into_bytes().into_boxed_slice();
    // SAFETY: The caller passes a pointer that is valid for writes.
    unsafe { out_len.write(bytes.len()) };
    Box::into_raw(bytes).cast::<c_char>()
}
