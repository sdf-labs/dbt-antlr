lexer grammar SpikeLexer;

// Escape-edge witness for --static-lexer-dfa: a recursive non-greedy comment rule
// (recursive context escapes) and a predicated keyword (predicate escapes).
// The predicate body is Rust run verbatim by the generated lexer: true when
// the character after `let` is '='. The table walker hands both constructs
// to the lazy simulator at the escape edge.

COMMENT : '/*' (COMMENT | .)*? '*/' ;
KWLET   : 'let' {recog.input().la(1) == '=' as i32}? ;
ID      : [a-z]+ ;
INT     : [0-9]+ ;
WS      : [ \t\r\n]+ -> channel(HIDDEN) ;
