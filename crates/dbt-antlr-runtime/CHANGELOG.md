# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/sdf-labs/dbt-antlr/releases/tag/dbt-antlr-runtime-v0.1.0) - 2026-10-09

### Other

- Induct `crates/dbt-antlr-runtime` into workspace linting
- restyle manifests away from dotted workspace inheritance
- release-plz + cargo-dist plumbing, RELEASE.md runbook
- move atn_packed_equivalence into dbt-antlr's test tree
- rename package dbt-antlr-codegen -> dbt-antlr (lib dbt_antlr)
- include!-compatible emission; fix BaseListenerFile
- check_version! as major-eq + minor-floor; codegen renders it from the runtime
- Fix lints from Rust 1.99 stable (CI toolchain)
- Rust port of the dbt ANTLR4 tool, runtime, and conformance suite
