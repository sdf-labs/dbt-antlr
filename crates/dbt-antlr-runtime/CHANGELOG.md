# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-runtime-v0.2.1...dbt-antlr-runtime-v0.3.0) - 2026-10-15

### Other

- Move the static lexer DFA modules under dfa/
- Drop the SSSE3 positive scan; Accel::Positive carries the plain bitmap
- Classify accel descriptors structurally with compute_accel
- Merge build outcomes with tables; unify the builder arena
- Separate DFA construction from serialization in the static lexer builder
- Classify accel descriptors with Accel::from_dfa_state + to_serialized
- Compare LexerAction lifetime-agnostically; drop the atn_triples plumbing
- Pin that real grammars emit accel-tagged static lexer tables
- Add compile-time static lexer DFA tables behind --static-lexer-dfa

## [0.1.0](https://github.com/sdf-labs/dbt-antlr/releases/tag/dbt-antlr-runtime-v0.1.0) - 2026-10-09

### Other

- CI, build-script codegen, and workspace hardening
- Rust port of the dbt ANTLR4 tool, runtime, and conformance suite
