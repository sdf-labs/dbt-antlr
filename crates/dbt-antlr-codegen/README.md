# dbt-antlr-codegen

Rust library and `dbt-antlr-codegen` command for compiling ANTLR v4 grammars
into Rust source compatible with `dbt-antlr-runtime`.

Use it from a build script:

```toml
[dependencies]
dbt-antlr-runtime = "0.1"

[build-dependencies]
dbt-antlr-codegen = { version = "0.1", default-features = false }
```

```rust,no_run
fn main() {
    dbt_antlr_codegen::Config::new("grammar/MyParser.g4")
        .generate()
        .expect("grammar generation failed");
}
```

`Config` writes the generated files to `OUT_DIR` by default and prints
`cargo:rerun-if-changed` directives covering the grammars and their import
search directories. Wrap the generated files with `include!` inside a module
carrying `#![allow(clippy::all, warnings)]`. `default-features = false`
skips the `fancy` diagnostics rendering used by the command-line tool and
noticeably shrinks the build-dependency tree.

Or install the command:

```bash
cargo install dbt-antlr-codegen
```
