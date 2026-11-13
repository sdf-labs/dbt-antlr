# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-g4-parser-v0.8.1...dbt-antlr-g4-parser-v0.9.0) - 2026-11-13

### Added

- *(codegen)* per-grammar optimization manifests with a decision inventory ([#96](https://github.com/sdf-labs/dbt-antlr/pull/96))
- *(codegen)* share accept states and minimize static parser tables ([#89](https://github.com/sdf-labs/dbt-antlr/pull/89))
- *(codegen,runtime)* keep stack guards under --static-dfa-only ([#79](https://github.com/sdf-labs/dbt-antlr/pull/79))

### Other

- Fix Cargo 1.100 warnings ([#99](https://github.com/sdf-labs/dbt-antlr/pull/99))
- *(runtime,codegen)* flat packed static parser table image, embedded zero-copy ([#72](https://github.com/sdf-labs/dbt-antlr/pull/72))
- *(runtime,codegen)* flat packed static lexer DFA image, embedded zero-copy ([#67](https://github.com/sdf-labs/dbt-antlr/pull/67))

## [0.7.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-g4-parser-v0.6.2...dbt-antlr-g4-parser-v0.7.0) - 2026-11-03

### Added

- *(g4-parser,runtime,codegen)* dogfood static DFA tables on the g4 front-end (Phase 7.1) ([#50](https://github.com/sdf-labs/dbt-antlr/pull/50))

## [0.6.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-g4-parser-v0.5.0...dbt-antlr-g4-parser-v0.6.0) - 2026-10-30

### Added

- *(codegen)* parser static DFA pass scaffolding and generated-code slim-down ([#25](https://github.com/sdf-labs/dbt-antlr/pull/25))

### Other

- *(codegen)* emit ATN and lexer table words as static arrays ([#41](https://github.com/sdf-labs/dbt-antlr/pull/41))
- *(codegen)* drop the tool-side packed parser ATN ([#35](https://github.com/sdf-labs/dbt-antlr/pull/35))

## [0.2.1](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-g4-parser-v0.2.0...dbt-antlr-g4-parser-v0.2.1) - 2026-10-12

### Other

- regenerate checked-in recognizers on the release PR ([#11](https://github.com/sdf-labs/dbt-antlr/pull/11))

### Tests

- unbreak main after 0.2.0, and give release PRs CI ([#9](https://github.com/sdf-labs/dbt-antlr/pull/9))

## [0.1.0](https://github.com/sdf-labs/dbt-antlr/releases/tag/dbt-antlr-g4-parser-v0.1.0) - 2026-10-09

### Other

- CI, build-script codegen, and workspace hardening
- Rust port of the dbt ANTLR4 tool, runtime, and conformance suite
