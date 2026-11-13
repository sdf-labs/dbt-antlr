grammar Subtree;
// The guess in `pair` returns before the syntax error, so the error is in
// `prog` and the culprit is in a completed subtree.
prog : pair ';' EOF ;
pair options { max_dfa_states = 3; } : ID ID | ID ID '=' ID ;
ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
