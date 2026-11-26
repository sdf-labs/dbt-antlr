//! Static grammar descriptions and the hooks that generated code provides.

use std::ffi::c_char;
use std::ptr;

use crate::errors::ANTLRError;
use crate::token::CommonToken;
use crate::tree::{NodeKindType, TreeNode};
use crate::Arena;

use super::overlay::AntlrTreeNode;
use super::session::ErrorCollector;

/// A UTF-8 string that is not NUL-terminated. A NULL `ptr` tells that the
/// string is absent.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrStr {
    /// The first byte, or NULL.
    pub ptr: *const c_char,
    /// The number of bytes.
    pub len: usize,
}

// SAFETY: `AntlrStr` is an immutable view of string data that nothing
// mutates while the view exists.
unsafe impl Send for AntlrStr {}
// SAFETY: See the `Send` implementation.
unsafe impl Sync for AntlrStr {}

impl AntlrStr {
    /// The absent string.
    pub const NONE: Self = Self {
        ptr: ptr::null(),
        len: 0,
    };

    /// A view of `text`.
    pub const fn new(text: &'static str) -> Self {
        Self::borrowed(text)
    }

    /// A view of `text`, or [`AntlrStr::NONE`].
    pub const fn from_option(text: Option<&'static str>) -> Self {
        match text {
            Some(text) => Self::new(text),
            None => Self::NONE,
        }
    }

    /// A view of `text` that is valid while `text` is.
    pub(super) const fn borrowed(text: &str) -> Self {
        Self {
            ptr: text.as_ptr().cast(),
            len: text.len(),
        }
    }

    /// The string that this view refers to.
    ///
    /// # Safety
    ///
    /// The view must refer to UTF-8 data that stays valid for `'a`.
    pub unsafe fn as_str<'a>(self) -> Option<&'a str> {
        if self.ptr.is_null() {
            return None;
        }
        // SAFETY: The caller guarantees that `ptr` refers to `len` bytes of
        // UTF-8 that stay valid for `'a`.
        Some(unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(self.ptr.cast(), self.len))
        })
    }
}

/// Converts the `_LITERAL_NAMES` or `_SYMBOLIC_NAMES` table of a generated
/// parser into views.
pub const fn names<const N: usize>(names: &[Option<&'static str>; N]) -> [AntlrStr; N] {
    let mut views = [AntlrStr::NONE; N];
    let mut index = 0;
    while index < N {
        views[index] = AntlrStr::from_option(names[index]);
        index += 1;
    }
    views
}

/// A parser rule.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrRuleInfo {
    /// The rule name.
    pub name: AntlrStr,
    /// The labels of the labeled alternatives (`# label`), in the order of
    /// `AntlrTreeNode::label_tag`.
    pub alt_labels: *const AntlrStr,
    /// The number of entries in `alt_labels`.
    pub alt_label_count: u32,
    /// True when `antlr_parse` can start at this rule. Rules with arguments
    /// cannot start a parse.
    pub is_entry: bool,
}

// SAFETY: `AntlrRuleInfo` refers only to immutable static data.
unsafe impl Send for AntlrRuleInfo {}
// SAFETY: See the `Send` implementation.
unsafe impl Sync for AntlrRuleInfo {}

impl AntlrRuleInfo {
    /// Describes a rule.
    ///
    /// # Panics
    ///
    /// Panics (at compile time in a `static`) when `alt_labels` has more than
    /// `u32::MAX` entries.
    pub const fn new(name: &'static str, alt_labels: &'static [AntlrStr], is_entry: bool) -> Self {
        assert!(alt_labels.len() <= u32::MAX as usize);
        Self {
            name: AntlrStr::new(name),
            alt_labels: alt_labels.as_ptr(),
            alt_label_count: alt_labels.len() as u32,
            is_entry,
        }
    }
}

/// A grammar of a bundle.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrGrammarInfo {
    /// The grammar name, for example `CSV`.
    pub name: AntlrStr,
    /// The parser rules, in rule index order.
    pub rules: *const AntlrRuleInfo,
    /// The number of entries in `rules`.
    pub rule_count: u32,
    /// The literal name of each token type (for example `'+'`). Absent
    /// names have a NULL `ptr`.
    pub literal_names: *const AntlrStr,
    /// The number of entries in `literal_names`.
    pub literal_name_count: u32,
    /// The symbolic name of each token type (for example `ID`). Absent names
    /// have a NULL `ptr`.
    pub symbolic_names: *const AntlrStr,
    /// The number of entries in `symbolic_names`.
    pub symbolic_name_count: u32,
}

// SAFETY: `AntlrGrammarInfo` refers only to immutable static data.
unsafe impl Send for AntlrGrammarInfo {}
// SAFETY: See the `Send` implementation.
unsafe impl Sync for AntlrGrammarInfo {}

/// The result of a successful [`ParseFn`] call.
#[derive(Debug)]
pub struct ParseOutput {
    pub(super) root: *const AntlrTreeNode,
}

impl ParseOutput {
    /// Records the root node of a parse.
    pub fn new<'a, N>(root: &'a TreeNode<'a, 'a, N, CommonToken<'a>>) -> Self
    where
        N: NodeKindType<'a, CommonToken<'a>>,
    {
        Self {
            root: ptr::from_ref(root).cast(),
        }
    }
}

/// Parses `input` from the rule with index `rule_index`.
///
/// The function must attach `errors` to the lexer and to the parser (in place
/// of the default console listeners), and must use the default error
/// strategy. It returns `Ok(None)` when `rule_index` is not an entry rule.
pub type ParseFn = for<'a> fn(
    arena: &'a Arena,
    input: &'a str,
    rule_index: u32,
    errors: &ErrorCollector,
) -> Result<Option<ParseOutput>, ANTLRError>;

/// Formats a parse tree in LISP form, for example `(s (e 1) <EOF>)`.
///
/// # Safety
///
/// `node` must be a node of a live parse tree of the same grammar.
pub type StringTreeFn = unsafe fn(node: *const AntlrTreeNode) -> String;

/// A grammar that a bundle exports (see [`export_grammars!`]).
///
/// The `<parser>_ffi.rs` file that `dbt-antlr-codegen --ffi` generates
/// defines a `GRAMMAR` static of this type.
///
/// [`export_grammars!`]: super::export_grammars
#[derive(Debug)]
pub struct Grammar {
    pub(super) info: AntlrGrammarInfo,
    pub(super) parse: ParseFn,
    pub(super) string_tree: StringTreeFn,
}

// SAFETY: `Grammar` refers only to immutable static data and to functions.
unsafe impl Sync for Grammar {}

impl Grammar {
    /// Describes a grammar.
    ///
    /// # Panics
    ///
    /// Panics (at compile time in a `static`) when a table has more than
    /// `u32::MAX` entries.
    pub const fn new(
        name: &'static str,
        rules: &'static [AntlrRuleInfo],
        literal_names: &'static [AntlrStr],
        symbolic_names: &'static [AntlrStr],
        parse: ParseFn,
        string_tree: StringTreeFn,
    ) -> Self {
        assert!(rules.len() <= u32::MAX as usize);
        assert!(literal_names.len() <= u32::MAX as usize);
        assert!(symbolic_names.len() <= u32::MAX as usize);
        Self {
            info: AntlrGrammarInfo {
                name: AntlrStr::new(name),
                rules: rules.as_ptr(),
                rule_count: rules.len() as u32,
                literal_names: literal_names.as_ptr(),
                literal_name_count: literal_names.len() as u32,
                symbolic_names: symbolic_names.as_ptr(),
                symbolic_name_count: symbolic_names.len() as u32,
            },
            parse,
            string_tree,
        }
    }

    /// The C description of the grammar.
    pub const fn info(&self) -> &AntlrGrammarInfo {
        &self.info
    }
}
