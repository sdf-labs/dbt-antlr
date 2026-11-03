lexer grammar RecursiveRule;

// Witness for the recursive-rule escape. NESTED recurses beside other
// alternatives, so it is no counted exclusion and its context depth is
// unbounded: the table escapes at the first symbol inside it instead of
// expanding the recursion. DELEGATED recurses through fragments only, so its
// own states are all epsilon-only and the escape triggers on its fragments'
// symbols. COMMENT is a simple recursive delimiter rule and stays static
// (CountedExclusion acceleration).
ACTION    : NESTED ;
PREFIXED  : 'x' NESTED 'y' ;
DELEGATED : DOPEN (DELEGATED | DBODY)* DCLOSE ;
COMMENT   : '/*' (COMMENT | .)*? '*/' ;
ID        : [a-z]+ ;
INT       : [0-9]+ ;
WS        : [ \t\r\n]+ -> skip ;

fragment NESTED : '{' (NESTED | STRING | ~[{}"])*? '}' ;
fragment STRING : '"' ~["]* '"' ;
fragment DOPEN : '<' ;
fragment DCLOSE : '>' ;
fragment DBODY : [0-9]+ ;
