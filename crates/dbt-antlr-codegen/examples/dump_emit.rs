// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Dev-loop driver for the dbt emission layer: emits the lexer and parser
//! sources for a `.g4` grammar into `/tmp/pi/` for diffing against the
//! goldens in `dbt-antlr-runtime/tests/gen/`.
//!
//! ```sh
//! cargo run -p dbt-antlr --example dump_emit -- ../dbt-antlr-runtime/grammars/SimpleLR.g4
//! cargo run -p dbt-antlr --example dump_emit -- ../dbt-antlr-runtime/grammars/SimpleLR.g4 --no-visitor
//! ```

#![allow(clippy::print_stdout)]

fn main() {
    let mut args = std::env::args().skip(1);
    let grammar = args
        .next()
        .expect("usage: dump_emit <grammar.g4> [--no-visitor]");
    let no_visitor = args.any(|arg| arg == "--no-visitor");
    let grammar = std::path::Path::new(&grammar).canonicalize().unwrap();
    let emit = if no_visitor {
        dbt_antlr_codegen::dbt::emit_files_with_flags(
            &grammar,
            &[grammar.parent().unwrap().to_path_buf()],
            true,
            false,
        )
    } else {
        dbt_antlr_codegen::dbt::emit_files(&grammar, &[grammar.parent().unwrap().to_path_buf()])
    };
    let files = emit.unwrap();
    for f in files {
        let out = format!("/tmp/pi/{}", f.name);
        std::fs::write(&out, &f.content).unwrap();
        println!("wrote {out}");
    }
}
