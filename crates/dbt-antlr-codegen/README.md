# dbt-antlr-codegen

Rust library and `dbt-antlr` command for compiling ANTLR v4 grammars
into source compatible with `antlr-rust-runtime`.

Use it from a build script:

```toml
# x-release-please-start-version
[dependencies]
antlr-rust-runtime = "0.34.0"

[build-dependencies]
dbt-antlr-codegen = "0.34.0"
# x-release-please-end
```

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let generation = dbt_antlr_codegen::Builder::new()
        .grammar("grammar/MyLexer.g4")
        .grammar("grammar/MyParser.g4")
        .library_directory("grammar")
        .out_dir(std::env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"))
        .generate()?;
    generation.emit_rerun_if_changed();
    Ok(())
}
```

Successful generations expose structured compiler warnings through
`Generation::diagnostics()`, including the diagnostic code, severity, source
path, and exact UTF-8 byte span. `Generation::warnings()` retains the rendered
CLI messages.

Or install the command:

```bash
cargo install dbt-antlr-codegen --bin dbt-antlr
```
