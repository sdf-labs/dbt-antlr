// Differential-fuzz grammar for --static-parser-dfa.
grammar FuzzExpr;

s : item (',' item)* ';'? EOF
  | 'let' binding EOF
  | 'alias' alias 'as' name EOF
  | 'pick' alias EOF
  ;

// Both alternatives start with `e`, so they form a shared-descent group: the
// parser parses `e` once and picks the alternative from the token after it.
item
  : e 'as' name
  | e
  ;

// Unlike `item`, 'as' can also follow `alias` (in the 'alias' statement), so
// the member follow sets at the shared call site overlap on 'as' and no
// shared-descent group is formed.
alias
  : e 'as' name
  | e
  ;

// No shared-descent group: alternative 1 reaches `e` through `target`. The
// table escapes at its lookahead budget, and its frontier states carry
// continuations.
binding
  : target '=' e
  | e
  ;

target : e ;

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
