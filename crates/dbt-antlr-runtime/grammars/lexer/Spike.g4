lexer grammar Spike;

COMMENT : '/*' (COMMENT | .)*? '*/' ;
KWLET   : 'let' {recog.input().la(1) == '=' as i32}? ;
ID      : [a-z]+ ;
INT     : [0-9]+ ;
WS      : [ \t\r\n]+ -> channel(HIDDEN) ;
UNPAIRED : '/*' ;
