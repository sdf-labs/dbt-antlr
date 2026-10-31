// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Full-context prediction with an empty outer context.
//!
//! A call of the left-recursive `send` rule of `LoopVeto.g4` as the parse
//! entry predicts its operator loop with full-context simulation, and the
//! simulation reaches the end of input with configs that merged an empty
//! path into other stacks. The expected trees are the output of the Java
//! runtime.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod loopvetolexer;
    pub mod loopvetoparser;
}

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::{Arena, InputStream};

use crate::gen::loopvetolexer::LoopVetoLexer;
use crate::gen::loopvetoparser::LoopVetoParser;

/// Parses `input` with `send` as the parse entry. The parse runs on a separate
/// thread, so that a prediction that does not stop fails the test.
fn parse_send(input: &'static str) -> String {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let tree = Arena::with(|arena| {
            let lexer = LoopVetoLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
            let mut parser = LoopVetoParser::new(arena, CommonTokenStream::new(lexer));
            parser
                .send()
                .map(|tree| string_tree(tree, parser.get_rule_names()))
                .map_err(|error| format!("{error:?}"))
        });
        let _ = sender.send(tree);
    });
    match receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(tree) => tree.unwrap_or_else(|error| panic!("{input:?}: {error}")),
        Err(_) => panic!("{input:?}: the prediction did not stop"),
    }
}

#[test]
fn entry_rule_predictions_stop_at_the_end_of_input() {
    let cases = [
        (
            "1 , 1 , 1 >> 1 >> 1",
            "(send (send 1) , (send (send 1) , (send 1) >> (send 1)) >> (send 1))",
        ),
        (
            "1 , 1 >> 1 , 1 >> 1 >> 1",
            "(send (send 1) , (send (send (send 1) >> (send 1)) , (send 1) >> (send 1)) >> (send 1))",
        ),
        (
            "1 , 1 , 1 , 1 >> 1 >> 1 >> 1",
            "(send (send 1) , (send (send 1) , (send (send 1) , (send 1) >> (send 1)) >> (send 1)) >> (send 1))",
        ),
    ];
    for (input, expected) in cases {
        assert_eq!(parse_send(input), expected, "{input:?}");
    }
}
