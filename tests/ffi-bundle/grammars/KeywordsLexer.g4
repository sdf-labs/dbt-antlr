lexer grammar KeywordsLexer;

CLASS  : 'class' ;
LAMBDA : 'lambda' ;
BOOM   : 'boom' ;
// An early end of input: the token has text, but its type is EOF.
END    : 'end' -> type(EOF) ;
ID     : [a-z]+ ;
WS     : [ \t\r\n]+ -> skip ;
