// A split lexer/parser grammar for the FFI bundle tests: `nested` takes an
// argument (so it is not an entry rule), `END` remaps to `EOF`, and `boom`
// panics in an action.
parser grammar KeywordsParser;

options { tokenVocab = KeywordsLexer; }

from      : statement* EOF ;
statement : class | lambda | boom ;
class     : CLASS ID ;
lambda    : LAMBDA nested[2] ;
nested[int depth] : ID ;
boom      : BOOM { panic!("boom"); } ;
