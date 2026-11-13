# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.8.1...dbt-antlr-codegen-v0.9.0) - 2026-11-13

### Added

- *(codegen)* name the callers of guard danger sets in --dump-dfas and G4S192 ([#101](https://github.com/sdf-labs/dbt-antlr/pull/101))
- *(codegen)* -q/--quiet, Config::quiet, and a CLI diagnostics summary line ([#100](https://github.com/sdf-labs/dbt-antlr/pull/100))
- *(codegen)* surface implicit parser token definitions and reject them with --strict ([#95](https://github.com/sdf-labs/dbt-antlr/pull/95))
- *(codegen)* per-grammar optimization manifests with a decision inventory ([#96](https://github.com/sdf-labs/dbt-antlr/pull/96))
- *(codegen)* --explain differential fault localizer for static parser DFAs ([#97](https://github.com/sdf-labs/dbt-antlr/pull/97))
- *(codegen)* rank --static-dfa-only guesses by prefix-hijack hazard ([#94](https://github.com/sdf-labs/dbt-antlr/pull/94))
- *(codegen)* name the competing alternatives in static DFA diagnostics ([#93](https://github.com/sdf-labs/dbt-antlr/pull/93))
- *(codegen)* share accept states and minimize static parser tables ([#89](https://github.com/sdf-labs/dbt-antlr/pull/89))
- *(codegen)* configurable static parser DFA budgets ([#88](https://github.com/sdf-labs/dbt-antlr/pull/88))
- *(codegen,runtime)* keep stack guards under --static-dfa-only ([#79](https://github.com/sdf-labs/dbt-antlr/pull/79))
- *(codegen)* warn on unfolded single-token blocks in hot rules (G4S194) ([#78](https://github.com/sdf-labs/dbt-antlr/pull/78))
- *(codegen)* report the cost of parser DFA decisions (G4S193, --profile-decisions) ([#75](https://github.com/sdf-labs/dbt-antlr/pull/75))

### Fixed

- *(codegen)* do not trust widened contexts in static parser tables ([#90](https://github.com/sdf-labs/dbt-antlr/pull/90))
- *(tests)* resolve harness artifacts under cargo 1.100's target layout ([#80](https://github.com/sdf-labs/dbt-antlr/pull/80))

### Other

- Fix Cargo 1.100 warnings ([#99](https://github.com/sdf-labs/dbt-antlr/pull/99))
- *(codegen)* share one closure between token classes that start from the same steps ([#77](https://github.com/sdf-labs/dbt-antlr/pull/77))
- *(codegen,runtime)* make the LL(1) lookahead linear in the call depth ([#76](https://github.com/sdf-labs/dbt-antlr/pull/76))
- *(codegen)* classify parser DFA decisions in parallel ([#74](https://github.com/sdf-labs/dbt-antlr/pull/74))
- *(codegen)* prune redundant parser DFA classification attempts ([#73](https://github.com/sdf-labs/dbt-antlr/pull/73))
- *(runtime,codegen)* flat packed static parser table image, embedded zero-copy ([#72](https://github.com/sdf-labs/dbt-antlr/pull/72))
- *(runtime,codegen)* flat packed static lexer DFA image, embedded zero-copy ([#67](https://github.com/sdf-labs/dbt-antlr/pull/67))

### Tests

- stage scratch crates in the target dir, not /tmp/pi ([#86](https://github.com/sdf-labs/dbt-antlr/pull/86))

## [0.8.0](https://github.com/sdf-labs/dbt-antlr/compare/dbt-antlr-codegen-v0.7.0...dbt-antlr-codegen-v0.8.0) - 2026-11-10

### Other

- *(runtime)* use CacheProvider trait object in BaseATNSimulator ([#64](https://github.com/sdf-labs/dbt-antlr/pull/64))
- *(runtime)* optimize static lexer DFA character scanning and inner walk loop ([#59](https://github.com/sdf-labs/dbt-antlr/pull/59))
- *(runtime)* avoid detaching LexerATNSimulator per token in BaseLexer ([#55](https://github.com/sdf-labs/dbt-antlr/pull/55))
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
