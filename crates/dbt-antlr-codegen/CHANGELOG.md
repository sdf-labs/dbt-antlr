# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.1](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.7.0...dbt-antlr-codegen-v0.7.1) - 2026-11-06

### Other

- *(runtime)* make LexerAction and Accel word-shaped #[repr(C, u32)] enums ([#53](https://github.com/sdf-labs/dbt-antlr/pull/53))

## [0.7.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.6.2...dbt-antlr-codegen-v0.7.0) - 2026-11-03

### Added

- *(g4-parser,runtime,codegen)* dogfood static DFA tables on the g4 front-end (Phase 7.1) ([#50](https://github.com/sdf-labs/dbt-antlr/pull/50))
- *(codegen,runtime)* shared-descent groups for static parser tables (Phase 6) ([#42](https://github.com/sdf-labs/dbt-antlr/pull/42))

### Fixed

- *(runtime)* detect recursive lexer contexts on first recursive rule entry ([#51](https://github.com/sdf-labs/dbt-antlr/pull/51))

## [0.6.2](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.6.1...dbt-antlr-codegen-v0.6.2) - 2026-11-02

### Added

- *(codegen)* guarded-iterate escape for precedence-loop decisions (Phase 5.3) ([#46](https://github.com/sdf-labs/dbt-antlr/pull/46))

### Tests

- *(codegen,runtime)* add LoopObligation.g4 witness for guarded-iterate obligation discharge ([#48](https://github.com/sdf-labs/dbt-antlr/pull/48))

## [0.6.1](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.6.0...dbt-antlr-codegen-v0.6.1) - 2026-11-01

### Fixed

- *(codegen)* attribute consumption after a decision by invocation, not by rule ([#45](https://github.com/sdf-labs/dbt-antlr/pull/45))
- *(runtime)* give prediction contexts a distinct empty return state ([#43](https://github.com/sdf-labs/dbt-antlr/pull/43))

## [0.6.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.5.0...dbt-antlr-codegen-v0.6.0) - 2026-10-30

### Added

- *(codegen)* add --static-dfa-only for recognizers without adaptive fallback ([#40](https://github.com/sdf-labs/dbt-antlr/pull/40))
- *(codegen)* parser static DFA coverage enrichments (Phase 5.1 + 5.2) ([#39](https://github.com/sdf-labs/dbt-antlr/pull/39))
- *(runtime,codegen)* parser static DFA tables end-to-end ([#38](https://github.com/sdf-labs/dbt-antlr/pull/38)) ([#38](https://github.com/sdf-labs/dbt-antlr/pull/38))
- *(codegen)* classify parser decisions for the static DFA pass ([#37](https://github.com/sdf-labs/dbt-antlr/pull/37))
- *(codegen)* parser static DFA pass scaffolding and generated-code slim-down ([#25](https://github.com/sdf-labs/dbt-antlr/pull/25))

### Other

- *(codegen)* emit ATN and lexer table words as static arrays ([#41](https://github.com/sdf-labs/dbt-antlr/pull/41))
- bake the 'static ATN convention into the compiled units ([#36](https://github.com/sdf-labs/dbt-antlr/pull/36))
- *(codegen)* drop the tool-side packed parser ATN ([#35](https://github.com/sdf-labs/dbt-antlr/pull/35))
- *(codegen)* drop the tool-side lexer ATN copy ([#34](https://github.com/sdf-labs/dbt-antlr/pull/34))

## [0.5.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.4.0...dbt-antlr-codegen-v0.5.0) - 2026-10-28

### Added

- *(codegen)* emit optimization manifest for --static-lexer-dfa ([#30](https://github.com/sdf-labs/dbt-antlr/pull/30))

### Other

- *(codegen)* make static DFA passes first-class optimization stages ([#31](https://github.com/sdf-labs/dbt-antlr/pull/31))
- *(codegen)* integrate static_lexer_dfa into the optimization pipeline ([#28](https://github.com/sdf-labs/dbt-antlr/pull/28))

## [0.4.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.3.0...dbt-antlr-codegen-v0.4.0) - 2026-10-17

### Other

- *(runtime)* deduplicate lexer grammar fixtures ([#23](https://github.com/sdf-labs/dbt-antlr/pull/23))
- *(codegen)* restructure the emission layer ([#22](https://github.com/sdf-labs/dbt-antlr/pull/22))

## [0.3.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.2.1...dbt-antlr-codegen-v0.3.0) - 2026-10-16

### Added

- dump the optimization manifest with minijinja instead of a JSON writer ([#13](https://github.com/sdf-labs/dbt-antlr/pull/13))

### Other

- *(runtime)* compile simple recursive comment rules to CountedExclusion run_scan ([#19](https://github.com/sdf-labs/dbt-antlr/pull/19))
- Add compile-time static lexer DFA tables behind --static-lexer-dfa ([#15](https://github.com/sdf-labs/dbt-antlr/pull/15))

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
