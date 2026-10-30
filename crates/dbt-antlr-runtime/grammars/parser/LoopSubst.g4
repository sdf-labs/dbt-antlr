// Witness for the enter/exit substitution (`loop_block`): the operator loop of
// a left-recursive rule whose operand is a rule with structured, multi-token
// alternatives. Beyond the hybrid construction budget the conflict between
// iterating the loop and leaving it is decided by the substitution instead of
// escaping to the adaptive engine.
grammar LoopSubst;

start : expr EOF ;

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
