# dbt-antlr-codegen

Rust library and `dbt-antlr-codegen` command for compiling ANTLR v4 grammars
into Rust source compatible with `dbt-antlr-runtime`.

Use it from a build script:

```toml
[dependencies]
dbt-antlr-runtime = "0.1"

[build-dependencies]
dbt-antlr-codegen = { version = "0.1", default-features = false, features = ["generator"] }
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

## Pinned-binary mode

If compiling the generator itself is too expensive for your build pipeline,
skip the `generator` feature and download a pinned release binary instead:

```toml
[build-dependencies]
dbt-antlr-codegen = { version = "0.1", default-features = false, features = ["download"] }
```

```rust,no_run
fn main() {
    dbt_antlr_codegen::Config::new("grammar/MyParser.g4")
        .pinned_release("0.1.0")
        .generate()
        .expect("grammar generation failed");
}
```

This downloads the `dbt-antlr-codegen` binary for the build host from the
GitHub release `dbt-antlr-codegen-v0.1.0`, verifies its SHA-256 checksum
(optionally pinned further with `pinned_release_sha256`), caches it under
`OUT_DIR`, and invokes it. Requires `curl` on `PATH`.

Or install the command:

```bash
cargo install dbt-antlr-codegen
```
