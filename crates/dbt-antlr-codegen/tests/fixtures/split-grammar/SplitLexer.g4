lexer grammar SplitLexer;

SELECT : 'SELECT' ;
CACHE  : 'CACHE' ;
ID     : [a-z]+ ;
WS     : [ \t\r\n]+ -> skip ;
