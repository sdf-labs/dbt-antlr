# dbt-antlr

A pure-Rust ANTLR v4 toolchain for dbt: a grammar compiler that generates
Rust parsers targeting the `dbt-antlr-runtime` runtime.

## Repository layout

- `crates/dbt-antlr-codegen` — the tool: `.g4` loading, semantics, ATN
  construction, and code generation. The emission layer (`src/dbt/`) renders
  recognizers for the `dbt-antlr-runtime` runtime from a minijinja template that
  mirrors the Java tool's `Rust.stg`; the `dbt-antlr-codegen` binary is the
  Java-tool-like command line (`-o`, `-lib`, `-visitor`, `-no-listener`).
- `crates/dbt-antlr-g4-parser` — the `.g4` grammar front-end.
- `crates/dbt-antlr-runtime` — our ANTLR4 runtime for Rust. This is the code
  generation target.
- `tests/runtime-testsuite` — harness for the official ANTLR
  runtime conformance suite.

## Provenance and credits

This repository stands on three projects, all BSD-3-Clause licensed:

- **[ANTLR 4](https://github.com/antlr/antlr4)** (The ANTLR Project, Terence
  Parr and contributors) — the original parser generator and the definition of
  the semantics this toolchain reimplements. Our runtime crate is a fork of the
  [dbt-antlr-runtime fork](https://github.com/sdf-labs/antlr4) of ANTLR 4, and our
  conformance testing uses the official ANTLR runtime test suite and `.interp`
  fixtures produced by the official Java tool.
- **[antlr4rust](https://github.com/rrevenantt/antlr4rust)** (Konstantin
  Anisimov, Alex Snaps, and contributors) — the original ANTLR4 runtime for
  Rust, from which the `dbt-antlr-runtime` runtime crate descends (see
  `crates/dbt-antlr-runtime/LICENSE.txt`).
- **[antlr-rust-runtime](https://github.com/ophi-dev/antlr-rust-runtime)**
  (Konstantin Vyatkin / Ophidiarium contributors) — a from-scratch, pure-Rust
  reimplementation of the ANTLR v4 tool and runtime. The tool side of this
  repository (`dbt-antlr`, `dbt-antlr-g4-parser`, the
  conformance-test harness, and — for now — `antlr-rust-runtime`) was
  seeded from v0.34.0 of that project (see `LICENSE`).

We are grateful to the authors of all three projects.
