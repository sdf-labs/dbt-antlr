grammar Expr;

prog : expr EOF ;

expr
    : expr (MUL | DIV) expr
    | expr (ADD | SUB) expr
    | INT
    | LPAREN expr RPAREN
    ;

MUL : '*' ;
DIV : '/' ;
ADD : '+' ;
SUB : '-' ;
LPAREN : '(' ;
RPAREN : ')' ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
