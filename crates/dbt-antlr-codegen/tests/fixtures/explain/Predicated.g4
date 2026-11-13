grammar Predicated;

s : ({true}? 'a' | 'b') ';' stat EOF ;

stat options { max_dfa_states = 3; }
    : ID ID ID ';'
    | ID ID '=' ID ';'
    ;

ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
