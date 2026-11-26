//! A C ABI over `dbt-antlr-runtime` parse trees, for foreign-language
//! bindings such as the `dbt_antlr` Python package.
//!
//! The ABI has three parts:
//!
//! - **Overlay types** ([`AntlrTreeNode`], [`AntlrToken`], ...): `#[repr(C)]`
//!   mirrors of the runtime parse tree layout. C code reads trees in the
//!   arena directly through pointers, without calls into Rust. The layout is
//!   the same for all grammars.
//! - **Sessions** ([`AntlrParseSession`]): `antlr_parse` parses an input
//!   into an arena that the session owns, and collects the syntax errors.
//!   `antlr_session_free` frees the arena.
//! - **A grammar registry**: [`export_grammars!`] gives names to the
//!   grammars of a bundle and defines the entry points that use them. One
//!   bundle (for example one Python extension module) can export many
//!   grammars with one copy of the runtime.
//!
//! `include/antlr.h` declares the complete ABI for C. A test makes sure that
//! the header agrees with this crate.
//!
//! # Bundles
//!
//! Generate each grammar with `dbt-antlr-codegen --python`. Next to the
//! recognizers, the generator writes `<parser>_ffi.rs`, which defines a
//! [`Grammar`] static named `GRAMMAR`. A bundle crate (`crate-type =
//! ["staticlib"]`) includes the generated modules and exports the grammars:
//!
//! ```ignore
//! mod csv {
//!     pub mod csvlexer;
//!     pub mod csvlistener;
//!     pub mod csvparser;
//!     pub mod csvparser_ffi;
//! }
//!
//! dbt_antlr_ffi::export_grammars! {
//!     "csv" => csv::csvparser_ffi::GRAMMAR,
//! }
//! ```
//!
//! The generated Python module of a grammar looks up its grammar by the name
//! `<name>`, where its file name is `<name>_parser.py`. Give each grammar
//! that name in `export_grammars!`.

mod abi;
mod grammar;
mod overlay;
mod registry;
mod session;

pub use abi::{
    GrammarInfoFn, GrammarLookupFn, NodeToStringTreeFn, ParseEntryFn, antlr_grammar_info,
    antlr_grammar_lookup, antlr_node_to_string_tree, antlr_parse, antlr_session_free,
    antlr_session_info, antlr_set_arena_allocation_limit, antlr_string_free, antlr_token_location,
    antlr_token_text_fallback,
};
pub use grammar::{
    AntlrGrammarInfo, AntlrRuleInfo, AntlrStr, Grammar, ParseFn, ParseOutput, StringTreeFn, names,
};
pub use overlay::{
    ANTLR_CHILDREN_INLINE_SLOTS, ANTLR_NODE_LEAF_SIZE, ANTLR_NODE_TAG_ERROR,
    ANTLR_NODE_TAG_FIRST_RULE, ANTLR_NODE_TAG_TERMINAL, ANTLR_TOKEN_EOF, ANTLR_TOKEN_TYPE_BITS,
    AntlrChildrenStore, AntlrLeafBody, AntlrNodeBody, AntlrNodeChildren, AntlrRuleBody, AntlrToken,
    AntlrTreeNode,
};
pub use session::{
    ANTLR_ERROR_ORIGIN_LEXER, ANTLR_ERROR_ORIGIN_PARSER, ANTLR_PARSE_STATUS_FATAL,
    ANTLR_PARSE_STATUS_INVALID_GRAMMAR, ANTLR_PARSE_STATUS_INVALID_INPUT,
    ANTLR_PARSE_STATUS_INVALID_RULE, ANTLR_PARSE_STATUS_INVALID_UTF8, ANTLR_PARSE_STATUS_OK,
    AntlrParseSession, AntlrSessionInfo, AntlrSyntaxError, CollectingErrorListener, ErrorCollector,
};

#[doc(hidden)]
pub mod __private {
    pub use crate::registry::*;
}

/// Exports grammars from a bundle crate under the given names, and defines
/// the entry points that depend on them: `antlr_grammar_lookup`,
/// `antlr_grammar_info`, `antlr_parse`, and `antlr_node_to_string_tree`.
///
/// Use this macro once, in the crate that becomes the shared library or the
/// static library. The grammar IDs are the positions in the list, starting at
/// 0. Two grammars cannot have the same name.
///
/// ```ignore
/// dbt_antlr_ffi::export_grammars! {
///     "csv" => csvparser_ffi::GRAMMAR,
///     "labels" => labelsparser_ffi::GRAMMAR,
/// }
/// ```
#[macro_export]
macro_rules! export_grammars {
    ($($name:literal => $grammar:path),* $(,)?) => {
        const _: () = {
            const GRAMMARS: &[$crate::__private::RegisteredGrammar] = &[
                $($crate::__private::RegisteredGrammar::new($name, &$grammar),)*
            ];
            const _: () = ::core::assert!(
                $crate::__private::names_are_unique(GRAMMARS),
                "export_grammars!: two grammars have the same name",
            );

            #[unsafe(no_mangle)]
            unsafe extern "C" fn antlr_grammar_lookup(
                name: *const ::core::ffi::c_char,
                name_len: usize,
            ) -> i32 {
                // SAFETY: The C caller passes `name_len` readable bytes.
                unsafe { $crate::__private::grammar_lookup(GRAMMARS, name, name_len) }
            }

            #[unsafe(no_mangle)]
            extern "C" fn antlr_grammar_info(grammar_id: i32) -> *const $crate::AntlrGrammarInfo {
                $crate::__private::grammar_info(GRAMMARS, grammar_id)
            }

            #[unsafe(no_mangle)]
            unsafe extern "C" fn antlr_parse(
                grammar_id: i32,
                rule_index: u32,
                input: *const ::core::ffi::c_char,
                input_len: usize,
            ) -> *mut $crate::AntlrParseSession {
                // SAFETY: The C caller passes `input_len` readable bytes that
                // outlive the session.
                unsafe { $crate::__private::parse(GRAMMARS, grammar_id, rule_index, input, input_len) }
            }

            #[unsafe(no_mangle)]
            unsafe extern "C" fn antlr_node_to_string_tree(
                grammar_id: i32,
                node: *const $crate::AntlrTreeNode,
                out_len: *mut usize,
            ) -> *mut ::core::ffi::c_char {
                // SAFETY: The C caller passes a live node of this grammar and
                // a writable length.
                unsafe { $crate::__private::node_to_string_tree(GRAMMARS, grammar_id, node, out_len) }
            }

            const _: $crate::GrammarLookupFn = antlr_grammar_lookup;
            const _: $crate::GrammarInfoFn = antlr_grammar_info;
            const _: $crate::ParseEntryFn = antlr_parse;
            const _: $crate::NodeToStringTreeFn = antlr_node_to_string_tree;
        };
    };
}
