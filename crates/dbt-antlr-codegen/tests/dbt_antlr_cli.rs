#![cfg(feature = "generator")]
// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Integration tests for the grammar transform (`GrammarTransform`)
//! command-line surface of `dbt-antlr-codegen`: precedence ladder collapse,
//! trivial pure rule inlining, and unreachable rule pruning with entry rule
//! inference.

#[path = "dbt_antlr_cli/support.rs"]
mod support;
#[path = "dbt_antlr_cli/transforms.rs"]
mod transforms;
