//! The implementations of the entry points that `export_grammars!` defines.

use std::ffi::c_char;
use std::panic::{self, AssertUnwindSafe};
use std::ptr;

use crate::abi::into_c_string;
use crate::grammar::{AntlrGrammarInfo, Grammar};
use crate::overlay::AntlrTreeNode;
use crate::session::{
    self, ANTLR_PARSE_STATUS_INVALID_GRAMMAR, ANTLR_PARSE_STATUS_INVALID_INPUT, AntlrParseSession,
};

/// A grammar with the name that `export_grammars!` gives it.
#[derive(Debug)]
pub struct RegisteredGrammar {
    name: &'static str,
    grammar: &'static Grammar,
}

impl RegisteredGrammar {
    /// Gives `grammar` the name `name`.
    pub const fn new(name: &'static str, grammar: &'static Grammar) -> Self {
        Self { name, grammar }
    }
}

/// Returns true when no two grammars have the same name.
pub const fn names_are_unique(grammars: &[RegisteredGrammar]) -> bool {
    let mut i = 0;
    while i < grammars.len() {
        let mut j = i + 1;
        while j < grammars.len() {
            if str_eq(grammars[i].name, grammars[j].name) {
                return false;
            }
            j += 1;
        }
        i += 1;
    }
    true
}

const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

fn grammar(grammars: &[RegisteredGrammar], grammar_id: i32) -> Option<&'static Grammar> {
    let index = usize::try_from(grammar_id).ok()?;
    grammars.get(index).map(|registered| registered.grammar)
}

/// See `antlr_grammar_lookup`.
///
/// # Safety
///
/// `name` must point to `name_len` readable bytes.
pub unsafe fn grammar_lookup(
    grammars: &[RegisteredGrammar],
    name: *const c_char,
    name_len: usize,
) -> i32 {
    if name.is_null() {
        return -1;
    }
    // SAFETY: The caller passes `name_len` readable bytes at `name`.
    let name = unsafe { std::slice::from_raw_parts(name.cast::<u8>(), name_len) };
    grammars
        .iter()
        .position(|registered| registered.name.as_bytes() == name)
        .and_then(|index| i32::try_from(index).ok())
        .unwrap_or(-1)
}

/// See `antlr_grammar_info`.
pub fn grammar_info(grammars: &[RegisteredGrammar], grammar_id: i32) -> *const AntlrGrammarInfo {
    grammar(grammars, grammar_id).map_or(ptr::null(), |grammar| ptr::from_ref(grammar.info()))
}

/// See `antlr_parse`.
///
/// # Safety
///
/// `input` must point to `input_len` readable bytes, which stay valid and
/// unchanged until the session is freed.
pub unsafe fn parse(
    grammars: &[RegisteredGrammar],
    grammar_id: i32,
    rule_index: u32,
    input: *const c_char,
    input_len: usize,
) -> *mut AntlrParseSession {
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        let Some(grammar) = grammar(grammars, grammar_id) else {
            return AntlrParseSession::failure(
                ANTLR_PARSE_STATUS_INVALID_GRAMMAR,
                format!("the bundle has no grammar with ID {grammar_id}"),
            );
        };
        let input = if input_len == 0 {
            &[][..]
        } else if input.is_null() {
            return AntlrParseSession::failure(
                ANTLR_PARSE_STATUS_INVALID_INPUT,
                "the input pointer is NULL",
            );
        } else {
            // SAFETY: The caller passes `input_len` readable bytes at `input`,
            // which outlive the session.
            unsafe { std::slice::from_raw_parts(input.cast::<u8>(), input_len) }
        };
        session::parse(grammar, rule_index, input)
    }));
    match outcome {
        Ok(session) => Box::into_raw(Box::new(session)),
        Err(payload) => {
            std::mem::forget(payload);
            ptr::null_mut()
        }
    }
}

/// See `antlr_node_to_string_tree`.
///
/// # Safety
///
/// `node` must be a node of a live session of the grammar `grammar_id`, and
/// `out_len` must be valid for writes.
pub unsafe fn node_to_string_tree(
    grammars: &[RegisteredGrammar],
    grammar_id: i32,
    node: *const AntlrTreeNode,
    out_len: *mut usize,
) -> *mut c_char {
    let Some(grammar) = grammar(grammars, grammar_id) else {
        return ptr::null_mut();
    };
    if node.is_null() || out_len.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: The caller passes a node of a live session of this grammar.
    match panic::catch_unwind(AssertUnwindSafe(|| unsafe { (grammar.string_tree)(node) })) {
        // SAFETY: The caller passes a pointer that is valid for writes.
        Ok(text) => unsafe { into_c_string(text, out_len) },
        Err(payload) => {
            std::mem::forget(payload);
            ptr::null_mut()
        }
    }
}
