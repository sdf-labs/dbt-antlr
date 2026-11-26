# dbt-antlr-ffi

A C ABI over `dbt-antlr-runtime` parse trees, for foreign-language bindings.
The `dbt_antlr` Python package (`python/` in this repository) uses it through
CFFI.

`include/antlr.h` declares the ABI. It has three parts:

- **Overlay types.** `AntlrTreeNode`, `AntlrToken`, and the types that they
  contain are `#[repr(C)]` mirrors of the runtime parse tree layout. C code
  reads the arena memory of a tree directly, without calls into Rust. The
  layout is the same for all grammars, and compile-time assertions keep it
  equal to the layout of `dbt-antlr-runtime`. The `static inline` accessors
  at the end of the header (`antlr_rule_child`, `antlr_leaf_symbol`,
  `antlr_token_type`, ...) read the fields that most callers need.
- **Sessions.** `antlr_parse` parses an input into an arena that an
  `AntlrParseSession` owns, with the default error strategy. The session
  collects the syntax errors instead of printing them, and it turns panics
  and fatal runtime errors (for example the arena allocation limit) into a
  status. `antlr_session_free` frees the arena.
- **A grammar registry.** One bundle (for example one Python extension
  module) can hold many grammars with one copy of the runtime.
  `export_grammars!` gives a name to each grammar, and
  `antlr_grammar_lookup` finds a grammar by its name.

## Bundles

Generate each grammar with `dbt-antlr-codegen --python`. Next to the lexer and
the parser, the generator writes:

- `<parser>_ffi.rs`, which describes the grammar to the bundle
  (a `dbt_antlr_ffi::Grammar` static named `GRAMMAR`), and
- `<name>_parser.py`, the Python entry functions of the grammar. `<name>` is
  the lowercase parser name without the `Parser` suffix, for example
  `csv_parser.py` for `CSVParser`.

Make a bundle crate with the `staticlib` crate type. Declare the generated
files as sibling modules, and export the grammars under the names of their
Python modules:

```rust,ignore
mod csv {
    #![allow(clippy::all, warnings)]
    pub mod csvlexer;
    pub mod csvparser;
    pub mod csvparser_ffi;
}

dbt_antlr_ffi::export_grammars! {
    "csv" => csv::csvparser_ffi::GRAMMAR,
}
```

`dbt_antlr.build` links the static library into a CFFI extension module
named `_native`. Put the extension module and the `<name>_parser.py` modules
into one Python package. `tests/ffi-bundle` in this repository is an example.

## Features

- `arena-allocation-limit`: makes `antlr_set_arena_allocation_limit` set the
  arena allocation limit of `dbt-antlr-runtime`. Without this feature, the
  function has no effect and returns `false`.

## The header

`tests/header.rs` generates the header with `cbindgen` and compares it with
`include/antlr.h` and `python/dbt_antlr/antlr.h`. To update both copies,
run:

```bash
DBT_ANTLR_FFI_UPDATE_HEADER=1 cargo test -p dbt-antlr-ffi --test header
```
