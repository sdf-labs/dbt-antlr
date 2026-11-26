// A split lexer/parser grammar for the Python binding tests: the rule names
// are Python keywords, `nested` takes an argument (so it is not an entry
// rule), and `boom` panics.
parser grammar KeywordsParser;

options { tokenVocab = KeywordsLexer; }

from      : statement* EOF ;
statement : class | lambda | boom ;
class     : CLASS ID ;
lambda    : LAMBDA nested[2] ;
nested[int depth] : ID ;
boom      : BOOM { panic!("boom"); } ;
