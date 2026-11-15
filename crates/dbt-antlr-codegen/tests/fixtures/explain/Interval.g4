grammar Interval;
// `DAY` is the unit of an interval and an alias of a column. The stack guard
// of `unit?` defers when `column` is on the stack, so --static-dfa-only reads
// `DAY` as the alias. With a lookahead budget of 3 tokens, the guard cannot
// see the second `+` of `DAY + 1 + 2`.
options { max_k = 3; }
stmt : 'SELECT' column ('+' INT)? EOF ;
column : add alias? ;
add : unary ('+' unary)* ;
unary : 'INTERVAL' STR unit? | ID | INT ;
unit : 'DAY' | 'HOUR' ;
alias : ID | 'DAY' ;
ID : [a-z]+ ;
INT : [0-9]+ ;
STR : '\'' [0-9]+ '\'' ;
WS : [ \t\r\n]+ -> skip ;
