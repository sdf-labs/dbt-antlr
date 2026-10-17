// Differential-fuzz grammar for --static-parser-dfa.
grammar FuzzExpr;

s : item (',' item)* ';'? EOF ;

item
  : e 'as' name
  | e
  ;

name : ID | 'as' ;

q : name ('.' name)* ;

e : e '[' e ']'
  | '-' e
  | <assoc=right> e '^' e
  | e ('*'|'/') e
  | e ('+'|'-') e
  | atom
  ;

atom
  : q
  | INT
  | '(' e ')'
  | ID '(' (e (',' e)*)? ')'
  ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
