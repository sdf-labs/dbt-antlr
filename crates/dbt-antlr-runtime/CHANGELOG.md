# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-runtime-v0.5.0...dbt-antlr-runtime-v0.6.0) - 2026-10-30

### Added

- *(codegen)* parser static DFA coverage enrichments (Phase 5.1 + 5.2) ([#39](https://github.com/sdf-labs/dbt-antlr/pull/39))
- *(runtime,codegen)* parser static DFA tables end-to-end ([#38](https://github.com/sdf-labs/dbt-antlr/pull/38)) ([#38](https://github.com/sdf-labs/dbt-antlr/pull/38))
- *(codegen)* classify parser decisions for the static DFA pass ([#37](https://github.com/sdf-labs/dbt-antlr/pull/37))
- *(codegen)* parser static DFA pass scaffolding and generated-code slim-down ([#25](https://github.com/sdf-labs/dbt-antlr/pull/25))

### Other

- bake the 'static ATN convention into the compiled units ([#36](https://github.com/sdf-labs/dbt-antlr/pull/36))
- *(codegen)* drop the tool-side lexer ATN copy ([#34](https://github.com/sdf-labs/dbt-antlr/pull/34))

## [0.5.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-runtime-v0.4.0...dbt-antlr-runtime-v0.5.0) - 2026-10-28

### Added

- *(codegen)* emit optimization manifest for --static-lexer-dfa ([#30](https://github.com/sdf-labs/dbt-antlr/pull/30))

### Other

- *(codegen)* integrate static_lexer_dfa into the optimization pipeline ([#28](https://github.com/sdf-labs/dbt-antlr/pull/28))

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
