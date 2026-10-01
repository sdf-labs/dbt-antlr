// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin

//! Programmatic test descriptors ported from upstream `CustomDescriptors.java`
//! (runtime-testsuite/test/org/antlr/v4/test/runtime).
//!
//! These cases cannot be expressed in the descriptor file format: the
//! line-separator pair needs exact CR/LF bytes that the line-based parser
//! eats (`text.split("\r?\n")` upstream), and the scale tests are generated
//! by loops that would be unmaintainable as checked-in files.

use std::fmt::Write as _;

use crate::Descriptor;

/// Returns the programmatic counterparts of the descriptors directory,
/// mirroring upstream's `CustomDescriptors.descriptors` map as a flat list.
pub(crate) fn custom_descriptors() -> Vec<Descriptor> {
    vec![
        line_separator_lf(),
        line_separator_crlf(),
        large_lexer(),
        atn_states_size_more_than_65535(),
        multi_token_alternative(),
    ]
}

fn lexer_descriptor(name: &str, grammar: String, input: String, output: String) -> Descriptor {
    Descriptor {
        group: "LexerExec".to_owned(),
        name: name.to_owned(),
        test_type: "Lexer".to_owned(),
        grammar_name: "L".to_owned(),
        grammar_template: grammar,
        start_rule: String::new(),
        input,
        output,
        errors: String::new(),
        flags: String::new(),
        skip_targets: Vec::new(),
        slave_grammar_templates: Vec::new(),
    }
}

fn line_separator_lf() -> Descriptor {
    lexer_descriptor(
        "LineSeparatorLf",
        "lexer grammar L;\nT: ~'\\n'+;\nSEPARATOR: '\\n';\n".to_owned(),
        "1\n2\n3".to_owned(),
        concat!(
            "[@0,0:0='1',<1>,1:0]\n",
            "[@1,1:1='\\n',<2>,1:1]\n",
            "[@2,2:2='2',<1>,2:0]\n",
            "[@3,3:3='\\n',<2>,2:1]\n",
            "[@4,4:4='3',<1>,3:0]\n",
            "[@5,5:4='<EOF>',<-1>,3:1]\n",
        )
        .to_owned(),
    )
}

fn line_separator_crlf() -> Descriptor {
    lexer_descriptor(
        "LineSeparatorCrLf",
        "lexer grammar L;\nT: ~'\\r'+;\nSEPARATOR: '\\r\\n';\n".to_owned(),
        "1\r\n2\r\n3".to_owned(),
        concat!(
            "[@0,0:0='1',<1>,1:0]\n",
            "[@1,1:2='\\r\\n',<2>,1:1]\n",
            "[@2,3:3='2',<1>,2:0]\n",
            "[@3,4:5='\\r\\n',<2>,2:1]\n",
            "[@4,6:6='3',<1>,3:0]\n",
            "[@5,7:6='<EOF>',<-1>,3:1]\n",
        )
        .to_owned(),
    )
}

/// Regression test for antlr/antlr4#76 "Serialized ATN strings should be
/// split when longer than 2^16 bytes (class file limitation)".
fn large_lexer() -> Descriptor {
    let tokens_count = 4000;

    let mut grammar = String::from("lexer grammar L;\nWS: [ \\t\\r\\n]+ -> skip;\n");
    for i in 0..tokens_count {
        writeln!(grammar, "KW{i} : 'KW' '{i}';").unwrap();
    }

    lexer_descriptor(
        "LargeLexer",
        grammar,
        "KW400".to_owned(),
        "[@0,0:4='KW400',<402>,1:0]\n[@1,5:4='<EOF>',<-1>,1:5]\n".to_owned(),
    )
}

/// Regression test for antlr/antlr4#1863. Upstream skips eight targets here
/// (`CustomDescriptors.java`), but not Rust.
fn atn_states_size_more_than_65535() -> Descriptor {
    let tokens_count = 1024;
    let suffix = "_".repeat(70);

    let mut grammar = String::from("lexer grammar L;\n\n");
    let mut input = String::new();
    let mut output = String::new();
    let mut stop_offset: isize = -2;
    for i in 0..tokens_count {
        let value = format!("T_{i:06}{suffix}");
        writeln!(grammar, "T_{i:06}: '{value}';").unwrap();
        input.push_str(&value);
        input.push('\n');

        let start_offset = stop_offset + 2;
        stop_offset += value.len().cast_signed() + 1;
        writeln!(
            output,
            "[@{i},{start_offset}:{stop_offset}='{value}',<{token_type}>,{line}:0]",
            token_type = i + 1,
            line = i + 1,
        )
        .unwrap();
    }
    grammar.push_str("\nWS: [ \\t\\r\\n]+ -> skip;\n");

    let start_offset = stop_offset + 2;
    let stop_offset = start_offset - 1;
    writeln!(
        output,
        "[@{tokens_count},{start_offset}:{stop_offset}='<EOF>',<-1>,{line}:0]",
        line = tokens_count + 1,
    )
    .unwrap();

    Descriptor {
        skip_targets: [
            "CSharp",
            "Python3",
            "Go",
            "PHP",
            "Swift",
            "JavaScript",
            "TypeScript",
            "Dart",
        ]
        .iter()
        .map(ToString::to_string)
        .collect(),
        ..lexer_descriptor("AtnStatesSizeMoreThan65535", grammar, input, output)
    }
}

/// Regression test for antlr/antlr4#3698 and antlr/antlr4#3703.
fn multi_token_alternative() -> Descriptor {
    let tokens_count = 64;

    let mut alts = String::from("r1: ");
    let mut tokens = String::new();
    let mut input = String::new();
    let mut output = String::new();
    for i in 0..=tokens_count {
        if i < tokens_count {
            write!(alts, "T{i}").unwrap();
            if i < tokens_count - 1 {
                alts.push_str(" | ");
            } else {
                alts.push(';');
            }
        }
        writeln!(tokens, "T{i}: 'T{i}';").unwrap();
        write!(input, "T{i} ").unwrap();
        write!(output, "T{i}").unwrap();
    }
    output.push('\n');

    let grammar = format!(
        "grammar P;\n\
         r: (r1 | T{tokens_count})+ EOF {{<writeln(\"$text\")>}};\n\
         {alts}\n\
         {tokens}\n\
         WS: [ ]+ -> skip;"
    );

    Descriptor {
        group: "ParserExec".to_owned(),
        name: "MultiTokenAlternative".to_owned(),
        test_type: "Parser".to_owned(),
        grammar_name: "P".to_owned(),
        grammar_template: grammar,
        start_rule: "r".to_owned(),
        input,
        output,
        errors: String::new(),
        flags: String::new(),
        skip_targets: Vec::new(),
        slave_grammar_templates: Vec::new(),
    }
}
