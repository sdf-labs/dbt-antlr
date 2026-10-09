# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/sdf-labs/dbt-antlr/releases/tag/dbt-antlr-codegen-v0.1.0) - 2026-10-09

### Other

- Revive a reviewed list of sensible lints
- Misc. doc adjustments
- fix Windows zip extraction (shadowed binding), CI cross-checks
- pinned-binary mode for build scripts (Config::pinned_release)
- run stock rust/clippy lint rules
- restyle manifests away from dotted workspace inheritance
- Fix left-over `dbt-antlr` references
- gate miette fancy diagnostics behind a default feature
- revert package rename, rename binary to dbt-antlr-codegen
- rename package dbt-antlr-codegen -> dbt-antlr (lib dbt_antlr)
- include!-compatible emission; fix BaseListenerFile
- build-script API (Config) for cargo-native generation
- check_version! as major-eq + minor-floor; codegen renders it from the runtime
- Fix lints from Rust 1.99 stable (CI toolchain)
- Rust port of the dbt ANTLR4 tool, runtime, and conformance suite
