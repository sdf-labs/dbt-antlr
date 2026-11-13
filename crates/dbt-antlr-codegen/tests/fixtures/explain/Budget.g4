grammar Budget;
// The state cap stops the lookahead of `stat` after two tokens, so
// --static-dfa-only guesses its minimum viable alternative there.
prog : stat+ EOF ;
stat options { max_dfa_states = 3; }
     : ID ID ID ';'
     | ID ID '=' ID ';'
     ;
ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
