# dbt_antlr

Python bindings of `dbt-antlr` parse trees. The parsers run in Rust. Python
reads the parse trees in place, in the arena memory of the Rust parser,
through the C ABI of `dbt_antlr_runtime::ffi` and CFFI.

The package is pure Python. A grammar bundle (a native extension module)
supplies the parsers.

## Usage

`dbt-antlr-codegen --python` generates a `<name>_parser.py` module for each
grammar, with one entry function for each parser rule that has no
arguments. An entry function returns a `ParseContext`. Use it in a `with`
statement:

```python
from my_bundle import csv_parser

with csv_parser.csvFile("name,age\nada,36\n") as tree:
    for row in tree.children_by_rule(csv_parser.RULE_row):
        print([field.source_text for field in row.children_by_rule(csv_parser.RULE_field)])
```

The tree is valid only in the `with` block. After the block, the arena is
free, and all `Tree` and `Token` objects of the parse raise
`ReferenceError`. A tree is also valid only on the thread that parsed it.

- Without `recovery`, a parse with syntax errors raises `ParseSyntaxError`.
  `error.errors` holds the `SyntaxDiagnostic` values.
- With `recovery=True`, the tree holds the recovered parse, with error
  leaves for skipped and conjured (`<missing ...>`) tokens.
  `tree.syntax_errors` holds the diagnostics.
- A parse that produces no tree (for example, a parser action panicked, or
  the parse went above the arena allocation limit) raises `ParseError`.

## Building a bundle

Generate each grammar with `dbt-antlr-codegen --python`. Next to the Rust
recognizers, the generator writes `<parser>_ffi.rs` and `<name>_parser.py`.

Create a Rust crate with `crate-type = ["staticlib"]` and depend on
`dbt-antlr-runtime` with `features = ["ffi"]`. Declare the generated Rust
modules as siblings and export the grammars under their Python module names:

```rust
mod csv {
    #![allow(clippy::all, warnings)]
    pub mod csvlexer;
    pub mod csvparser;
    pub mod csvparser_ffi;
}

dbt_antlr_runtime::ffi::export_grammars! {
    "csv" => csv::csvparser_ffi::GRAMMAR,
}
```

Then build the static library with Cargo and link it into a CFFI extension
module (API mode, `abi3`):

```python
from dbt_antlr import build

library = build.build_staticlib("my-bundle", manifest_path="Cargo.toml", release=True)
build.compile_bundle("my_bundle._native", library, output_dir="build")
```

Put `_native` and the `<name>_parser.py` modules into the same Python package.
Building needs `cffi` and `setuptools`. Using a bundle needs only `cffi`.
## Tests

`tests/` builds the bundle of `tests/ffi-bundle` and tests the package
with it:

```bash
pip install cffi pytest setuptools
python -m pytest python/tests
```
