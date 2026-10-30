// Witness for the enter/exit substitution beyond the hybrid construction
// budget. It is the LoopSubst grammar with a follow that re-enters the loop:
// `expr+` puts the loop's own first tokens in FOLLOW(expr), and
// `expr '||' INT EOF` makes the exit lineage consume an operator token in a
// frame that is not this loop's own.
//
// Two facts to keep:
// - Without the substitution this decision escapes 20 times: the budget
//   frontier cuts iterate/exit states at the depth cap, and the foreign
//   consumption of the `'||' INT EOF` follow vetoes the rest.
// - With it, 18 of those states are resolved and 2 escapes remain, both
//   foreign-vetoed iterate/exit conflicts.
grammar LoopFrontier;

start : expr+ EOF | expr '||' INT EOF ;

expr
    : operand
    | ('+' | '-') expr
    | expr ('*' | '/') expr
    | expr ('+' | '-') expr
    | expr '||' expr
    | expr '<<' expr
    ;

operand
    : INT
    | ID
    | '(' expr ')'
    | CAST '(' expr AS type_ ')'
    | TRY_CAST '(' expr AS type_ ')'
    | CASE whenClause+ (ELSE expr)? END
    | ID '(' (expr (',' expr)*)? ')'
    | ID '.' ID
    | EXTRACT '(' ID FROM expr ')'
    | SUBSTRING '(' expr FROM? expr (FOR expr)? ')'
    | POSITION '(' expr IN expr ')'
    | ROW '(' expr (',' expr)* ')'
    | TIMESTAMP STRING
    | INTERVAL STRING ID
    ;

type_ : ID | ID VARYING | INT ;

whenClause : WHEN expr THEN expr ;

AS : 'AS' ;
CAST : 'CAST' ;
TRY_CAST : 'TRY_CAST' ;
CASE : 'CASE' ;
WHEN : 'WHEN' ;
THEN : 'THEN' ;
ELSE : 'ELSE' ;
END : 'END' ;
EXTRACT : 'EXTRACT' ;
SUBSTRING : 'SUBSTRING' ;
POSITION : 'POSITION' ;
ROW : 'ROW' ;
TIMESTAMP : 'TIMESTAMP' ;
INTERVAL : 'INTERVAL' ;
FROM : 'FROM' ;
FOR : 'FOR' ;
IN : 'IN' ;
VARYING : 'VARYING' ;
STRING : '\'' ~'\''* '\'' ;
ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
