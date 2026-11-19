// Witness for the entry-context attestation of the uniform-trust resolution
// (issue #91).
//
// In decision 1 of `a`, the alternatives `B (P | Q) A` and `B P a E` meet
// after `'b' 'p' 'a'`. Alternative 1 sits on the entry stack of the decision;
// alternative 3 is inside the nested `a` and still owes `E`. Only alternative
// 3 can read `c b p a d e d x`.
grammar UnsoundSimple;
top : a X | a Y ;
a   : ( A | C ( B (P | Q) A | B P A | B P a E ) M? ) D ;
A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ; M : 'm' ; P : 'p' ; Q : 'q' ; X : 'x' ; Y : 'y' ;
WS : ' '+ -> skip ;
