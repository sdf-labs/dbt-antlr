grammar Context;
// Only the caller of `e` tells its alternatives apart: an SLL conflict that
// --static-dfa-only resolves to the minimum alternative.
s : 'x' e 'y' EOF | 'w' e EOF ;
e : 'a' 'y' | 'a' ;
WS : [ \t\r\n]+ -> skip ;
