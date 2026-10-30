// Witness for the enter/exit substitution beyond the hybrid construction
// budget, where every follow of the loop is foreign. It is the LoopSubst
// grammar with two follows:
// - `expr+` starts a new invocation of `expr` after the exit. That
//   invocation can read the next `'-'` as a prefix operator where the loop
//   reads a binary operator.
// - `expr '||' INT EOF` consumes an operator token in a frame that is not this
//   loop's own.
//
// The `set` prefix makes the foreign caller reachable at run time: inside
// `set expr '||' INT`, the exit reading of the expression is the right one, so
// the guard must defer there. Everywhere else the iterate reading wins.
//
// Neither follow is owned by the loop: the budget-frontier states of `expr+`
// escape, and the two `'||' INT EOF` conflict states resolve under the runtime
// stack guard.
grammar LoopFrontier;

start : expr+ EOF
      | 'set' expr '||' INT EOF
      ;

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
