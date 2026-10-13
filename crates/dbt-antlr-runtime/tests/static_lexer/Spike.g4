lexer grammar Spike;

COMMENT : '/*' (COMMENT | .)*? '*/' ;
KWLET   : 'let' {this.p()}? ;
ID      : [a-z]+ ;
INT     : [0-9]+ ;
WS      : [ \t\r\n]+ -> channel(HIDDEN) ;
