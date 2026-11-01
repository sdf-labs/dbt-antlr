// Witness for the take rule of an optional postfix (`postfix`): the classic
// dangling else. When the inner `st` skips its `e?`, the skip reading pops into
// the enclosing `st` and takes the `else` on that invocation's take path. The
// take reading consumes the same tokens through the inner take path, so the
// take/skip conflict resolves to take without an escape.
grammar DanglingElse;

start : st EOF ;

st : 'if' st e? | 'a' ;

e : 'else' st ;

WS : [ \t\r\n]+ -> skip ;
