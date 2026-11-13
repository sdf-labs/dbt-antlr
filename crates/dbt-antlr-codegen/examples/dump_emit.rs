// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Dev-loop driver for the dbt emission layer: emits the lexer and parser
//! sources for a `.g4` grammar into `dbt-antlr-dump-emit/` under the system
//! temp dir for diffing against the goldens in `dbt-antlr-runtime/tests/gen/`.
//!
//! ```sh
//! cargo run -p dbt-antlr-codegen --example dump_emit -- ../dbt-antlr-runtime/grammars/SimpleLR.g4
//! cargo run -p dbt-antlr-codegen --example dump_emit -- ../dbt-antlr-runtime/grammars/SimpleLR.g4 --no-visitor
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
        dbt_antlr_codegen::emission::emit_files_with_flags(
            &grammar,
            &[grammar.parent().unwrap().to_path_buf()],
            true,
            false,
        )
    } else {
        dbt_antlr_codegen::emission::emit_files(
            &grammar,
            &[grammar.parent().unwrap().to_path_buf()],
        )
    };
    let files = emit.unwrap();
    let out_dir = std::env::temp_dir().join("dbt-antlr-dump-emit");
    std::fs::create_dir_all(&out_dir).unwrap();
    for f in files {
        let out = out_dir.join(&f.name);
        std::fs::write(&out, &f.content).unwrap();
        println!("wrote {}", out.display());
    }
}
