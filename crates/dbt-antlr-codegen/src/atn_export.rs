// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Compiled ATN data export, available with the `atn-export` feature.
//!
//! This module gives access to the compiled ATN word streams of a grammar
//! without Rust source generation.

use std::path::{Path, PathBuf};

use crate::error::Error;
use crate::grammar::compiler;
use crate::grammar::loader::LoadOptions;

/// ATN word streams of one compiled grammar unit.
///
/// A lexer grammar unit fills [`Self::lexer_atn_words`]. A parser grammar
/// unit fills [`Self::parser_packed_words`]. A combined grammar compiles to
/// one lexer unit and one parser unit, so it returns two entries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GrammarAtnData {
    /// Grammar unit name, for example `CSV` or `XMLLexer`.
    pub name: String,
    /// Packed parser ATN word stream, when this unit has a parser.
    ///
    /// The words come from `ParserAtn::packed_words`. Use
    /// `ParserAtn::from_owned` or `ParserAtn::from_static` to read them.
    pub parser_packed_words: Option<Vec<u32>>,
    /// Java-format serialized lexer ATN words, when this unit has a lexer.
    pub lexer_atn_words: Option<Vec<i32>>,
}

/// Compiles the grammars that `grammar_path` roots and returns the ATN data
/// of every compiled unit.
///
/// `library_directories` supplies the search paths for imported grammars and
/// token vocabularies. The compile pipeline runs with its default options and
/// its default validation. The result contains one entry per compiled unit,
/// including imported units. The order is stable for identical inputs.
///
/// # Errors
///
/// Returns [`ErrorKind::Compilation`](crate::ErrorKind::Compilation) when the
/// grammar fails to compile.
pub fn compile_atn_data(
    grammar_path: &Path,
    library_directories: &[PathBuf],
) -> Result<Vec<GrammarAtnData>, Error> {
    let roots = vec![grammar_path.to_path_buf()];
    let compilation = compiler::compile(LoadOptions {
        roots: roots.clone(),
        library_directories: library_directories.to_vec(),
    })
    .map_err(|error| crate::compile_error::compilation_error(&error, &roots))?;
    let mut units = Vec::with_capacity(compilation.parsers.len() + compilation.lexers.len());
    units.extend(compilation.parsers.values().map(|parser| GrammarAtnData {
        name: parser.semantic.unit.name.clone(),
        parser_packed_words: Some(parser.packed.packed_words().to_vec()),
        lexer_atn_words: None,
    }));
    units.extend(compilation.lexers.values().map(|lexer| GrammarAtnData {
        name: lexer.semantic.unit.name.clone(),
        parser_packed_words: None,
        lexer_atn_words: Some(lexer.runtime_artifact.atn_words.clone()),
    }));
    Ok(units)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::atn::ParserAtn;
    use crate::grammar::atn::SERIALIZED_VERSION;

    const PARSER_ATN_MAGIC: u32 = 0x5041_544e;
    const PARSER_ATN_FORMAT_VERSION: u32 = 3;

    #[test]
    fn parser_grammar_exports_packed_words() {
        let units = compile_atn_data(&fixture("parser-basic").join("ParserBasic.g4"), &[])
            .expect("parser fixture should compile");
        let [unit] = units.as_slice() else {
            panic!("parser-only fixture should produce one unit");
        };
        assert_eq!(unit.name, "ParserBasic");
        assert!(unit.lexer_atn_words.is_none());
        let packed = unit
            .parser_packed_words
            .as_ref()
            .expect("parser unit has packed words");
        assert!(packed.len() > 2);
        assert_eq!(packed[0], PARSER_ATN_MAGIC);
        assert_eq!(packed[1], PARSER_ATN_FORMAT_VERSION);
        ParserAtn::from_owned(packed.clone()).expect("packed words should validate");
    }

    #[test]
    fn combined_grammar_exports_parser_and_lexer_words() {
        let units = compile_atn_data(&fixture("vscode-sentences").join("sentences.g4"), &[])
            .expect("combined fixture should compile");
        let parser = units
            .iter()
            .find(|unit| unit.parser_packed_words.is_some())
            .expect("combined grammar yields a parser unit");
        let lexer = units
            .iter()
            .find(|unit| unit.lexer_atn_words.is_some())
            .expect("combined grammar yields a lexer unit");
        assert_eq!(parser.name, "sentencesParser");
        assert_eq!(lexer.name, "sentencesLexer");
        let packed = parser
            .parser_packed_words
            .as_ref()
            .expect("parser unit has packed words");
        assert_eq!(packed[0], PARSER_ATN_MAGIC);
        assert_eq!(packed[1], PARSER_ATN_FORMAT_VERSION);
        ParserAtn::from_owned(packed.clone()).expect("packed words should validate");
        let words = lexer
            .lexer_atn_words
            .as_ref()
            .expect("lexer unit has ATN words");
        assert!(words.len() > 2);
        assert_eq!(words[0], SERIALIZED_VERSION);
        assert_eq!(words[1], 0, "lexer grammar type");
    }

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/codegen-direct/fixtures")
            .join(name)
    }
}
