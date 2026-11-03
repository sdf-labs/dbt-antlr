lexer grammar SharedComments;

// Witness for the context-level recursion escape. Two identical simple
// recursive comment rules are CountedExclusion candidates on their own, but
// their closed config sets mix, so the simple-recursion strip cannot apply and
// the recursion escapes when the closure re-enters an already active rule.
C1 : '/*' (C1 | .)*? '*/' ;
C2 : '/*' (C2 | .)*? '*/' ;
ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
