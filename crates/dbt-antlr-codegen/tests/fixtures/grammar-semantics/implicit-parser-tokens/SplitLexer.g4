lexer grammar SplitLexer;

SELECT : 'SELECT' ;
ID     : [a-z]+ ;
WS     : [ \t\r\n]+ -> skip ;
