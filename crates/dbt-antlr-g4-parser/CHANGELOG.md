# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
