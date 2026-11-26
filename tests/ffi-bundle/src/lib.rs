//! A `dbt_antlr_runtime::ffi` bundle with the grammars of the C ABI tests:
//!
//! - `csv`: the combined `CSV` grammar (hidden whitespace, rows with many
//!   children).
//! - `labels`: the combined `Labels` grammar (labeled alternatives, left
//!   recursion, error recovery).
//! - `keywords`: the split `KeywordsLexer`/`KeywordsParser` grammar (a rule
//!   with an argument, a token remapped to `EOF`, and an action that panics).
//!
//! `gen_grammars.sh` regenerates `src/generated`.

#[rustfmt::skip]
mod generated {
    // Generated recognizers: lint with the codegen templates, not here.
    #![allow(clippy::all, warnings)]
    pub mod csvlexer;
    pub mod csvparser;
    pub mod csvparser_ffi;
    pub mod keywordslexer;
    pub mod keywordsparser;
    pub mod keywordsparser_ffi;
    pub mod labelslexer;
    pub mod labelsparser;
    pub mod labelsparser_ffi;
}

/// The names of the grammars of the bundle, in grammar ID order.
pub const GRAMMAR_NAMES: [&str; 3] = ["csv", "labels", "keywords"];

dbt_antlr_runtime::ffi::export_grammars! {
    "csv" => generated::csvparser_ffi::GRAMMAR,
    "labels" => generated::labelsparser_ffi::GRAMMAR,
    "keywords" => generated::keywordsparser_ffi::GRAMMAR,
}
