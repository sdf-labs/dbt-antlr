// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Build script: generate the expression recognizer from `grammars/Expr.g4`
//! with the dbt-antlr generator as a plain build dependency.

fn main() {
    dbt_antlr::Config::new("grammars/Expr.g4")
        .generate()
        .unwrap_or_else(|error| panic!("{error:?}"));
}
