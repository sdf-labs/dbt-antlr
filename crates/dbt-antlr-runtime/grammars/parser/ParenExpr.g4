// Shared-descent witness with a one-token prefix: the primary block of a
// left-recursive expression rule with a parenthesized family
//
//   '(' e (',' e)+ ')'   (row constructor)
//   '(' e ')'            (plain parens)
//   '(' q ')'            (subquery)
//
// The row-constructor and paren alternatives share the unbounded prefix
// '(' e, so no bounded lookahead tells them apart: that needs the token after
// the whole expression (',' vs ')'). The subquery alternative keeps pace while
// the content can start a query (a parenthesized table name is a valid query).
// Once the subquery reading dies, the table accepts with the group
// {rowConstructor, parens}: the parser parses `e` once after '(' and picks the
// alternative from the next token. Deep nesting ('((((((x)))))') keeps the
// subquery reading alive arbitrarily long; those paths still escape to
// adaptivePredict. Grammar from the k3 reference runtime.
grammar ParenExpr;

s : e EOF ;

e : INT                                    # num
  | ID                                     # col
  | '(' e (',' e)+ ')'                     # rowConstructor
  | '(' e ')'                              # parens
  | '(' q ')'                              # subquery
  | '-' e                                  # unary
  | e ('*'|'/') e                          # mul
  | e ('+'|'-') e                          # add
  ;

q : 'SELECT' ID                            # select
  | ID                                     # table
  | '(' q ')'                              # nested
  ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
