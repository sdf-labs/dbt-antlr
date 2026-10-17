# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-runtime-v0.3.0...dbt-antlr-runtime-v0.4.0) - 2026-10-17

### Other

- *(runtime)* remove eof_chain bound and split match_static ([#24](https://github.com/sdf-labs/dbt-antlr/pull/24))
- *(runtime)* deduplicate lexer grammar fixtures ([#23](https://github.com/sdf-labs/dbt-antlr/pull/23))
- *(runtime)* group static lexer fixtures and tests ([#20](https://github.com/sdf-labs/dbt-antlr/pull/20))

## [0.3.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-runtime-v0.2.1...dbt-antlr-runtime-v0.3.0) - 2026-10-16

### Other

- *(runtime)* compile simple recursive comment rules to CountedExclusion run_scan ([#19](https://github.com/sdf-labs/dbt-antlr/pull/19))
- *(runtime)* simplify static lexer Accel and run_scan without newline special-casing ([#18](https://github.com/sdf-labs/dbt-antlr/pull/18))
- *(runtime)* lazy token line/column resolution via shared SourceInfo ([#17](https://github.com/sdf-labs/dbt-antlr/pull/17))
- Add compile-time static lexer DFA tables behind --static-lexer-dfa ([#15](https://github.com/sdf-labs/dbt-antlr/pull/15))

## [0.1.0](https://github.com/sdf-labs/dbt-antlr/releases/tag/dbt-antlr-runtime-v0.1.0) - 2026-10-09

### Other

- CI, build-script codegen, and workspace hardening
- Rust port of the dbt ANTLR4 tool, runtime, and conformance suite
