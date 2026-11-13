grammar Descent;
// The alternatives of `x` share the rule `y`: a shared-descent group parses
// `y` once and dispatches on the token after it.
s : x EOF ;
x : y '+' ID | y ;
y : '(' y ')' | ID ;
ID : [a-z]+ ;
WS : [ \t]+ -> skip ;
