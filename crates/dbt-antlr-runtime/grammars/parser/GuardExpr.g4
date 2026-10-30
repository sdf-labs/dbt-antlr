// Witness for the optional-postfix take rule (`postfix`): `pred=predicate?`
// is the canonical dangling-else shape. The shape is detected on this
// grammar (one postfix decision, no take-rule fire), which the reference
// classifier reports identically. Phase 5.2b extends it with the guarded
// take, where the mirror check vetoes the conflict instead.
grammar GuardExpr;
start : expr EOF ;
expr : value pred?
     | NOT expr
     | expr 'AND' expr
     | expr 'OR' expr
     ;
pred : '=' value
     | NOT? 'IN' '(' value ')'
     | 'IS' NOT? NULL
     | NOT? 'BETWEEN' value 'AND' value
     ;
value : ID | INT | '(' expr ')' ;
NOT : 'NOT' ; NULL : 'NULL' ; IN : 'IN' ; BETWEEN : 'BETWEEN' ;
ID : [a-z]+ ; INT : [0-9]+ ; WS : [ \t\r\n]+ -> skip ;
