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
search directories. `Config::generate` also prints each grammar warning as a
`cargo:warning` directive, so that cargo shows it in the build output. For
example, a parser grammar that refers to a token that its `tokenVocab` lexer
does not define gets a `G4S030` warning. Call `.strict(true)` (`--strict` on
the command line) to make this warning an error, or `.quiet(true)` (`-q` or
`--quiet` on the command line) to print only the errors. When a run has
diagnostics, the command line ends them with a summary line that counts them
by code, for example
`summary: Trino.g4: 0 errors, 3 warnings (2 G4S192, 1 G4S193)`. With
`--quiet`, the summary line stays and says that warnings were hidden. Wrap
the generated files with `include!` inside a module carrying
`#![allow(clippy::all, warnings)]`.
`default-features = false` skips the `fancy` diagnostics rendering used by
the command-line tool and noticeably shrinks the build-dependency tree.

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

Prebuilt binaries are currently published for `0.1.0` only; later releases
ship crates.io-only until binary distribution is re-enabled (see `RELEASE.md`).

Or install the command:

```bash
cargo install dbt-antlr-codegen
```
