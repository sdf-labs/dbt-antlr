// Witness for the `BETWEEN ... AND ...` collision with a boolean `AND`/`OR`
// loop, in the shape that SQL grammars carry it:
//
//   b    : v pred? | 'NOT' b | b 'AND' b | b 'OR' b
//   pred : 'NOT'? 'BETWEEN' v 'AND' v | ('=' | '<' | '>') v
//
// `BETWEEN` is a suffix predicate on the arithmetic value rule `v`, so its
// `'AND'` separator collides with the boolean loop's `'AND'` operator. The
// lambda production `ID '->' b` in `atom` puts a boolean expression directly in
// a value position, so a `BETWEEN` bound can end with a lambda whose body
// absorbs `'AND'` tokens: in `x BETWEEN y -> a AND b AND c` the first `'AND'`
// belongs to the lambda body and the second is the `BETWEEN` separator.
//
// The boolean loop decision guards the two conflict states where `'AND'` can be
// either the loop operator or an enclosing `BETWEEN` separator: when the parse
// stack carries the lower-bound call site of `BETWEEN`, the guard defers to
// `adaptive_predict`; everywhere else it resolves to iterate in the static
// table.
grammar BetweenExpr;

s : b EOF ;

b : v pred?              # predicated
  | 'NOT' b              # not
  | b 'AND' b            # and
  | b 'OR' b             # or
  ;

pred : 'NOT'? 'BETWEEN' v 'AND' v    # between
     | ('=' | '<' | '>') v           # cmp
     ;

v : v ('*' | '/') v
  | v ('+' | '-') v
  | atom
  ;

atom : ID | INT | '(' b ')' | ID '->' b ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
