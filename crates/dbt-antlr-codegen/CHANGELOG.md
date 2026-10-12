# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.2.0...dbt-antlr-codegen-v0.2.1) - 2026-10-12

### Other

- regenerate checked-in recognizers on the release PR ([#11](https://github.com/sdf-labs/dbt-antlr/pull/11))

### Tests

- unbreak main after 0.2.0, and give release PRs CI ([#9](https://github.com/sdf-labs/dbt-antlr/pull/9))

## [0.2.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.1.0...dbt-antlr-codegen-v0.2.0) - 2026-10-12

### Added

- revive GrammarTransform optimizations (ladder collapse, inlining, pruning) ([#6](https://github.com/sdf-labs/dbt-antlr/pull/6))

### Other

- disable cargo-dist binary distribution for now ([#4](https://github.com/sdf-labs/dbt-antlr/pull/4))

## [0.1.0](https://github.com/sdf-labs/dbt-antlr/releases/tag/dbt-antlr-codegen-v0.1.0) - 2026-10-09

### Other

- CI, build-script codegen, and workspace hardening
- Rust port of the dbt ANTLR4 tool, runtime, and conformance suite
