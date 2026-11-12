// Witness of guarded iterates under `--static-dfa-only`, in the shape of the
// ClickHouse grammar with an inline alias: the alias is a postfix operator of
// the recursive expression, and `AS` and `AND` also follow an expression in
// other clauses.
//
//   expr : expr 'AS' alias                  # the alias postfix operator
//        | 'CAST' '(' expr 'AS' type ')'    # `AS` after an expression
//   query : ... 'ROWS' 'BETWEEN' expr 'AND' expr   # `AND` after one
//
// In `CAST ( x AS INT64 )` the alias reading of `AS INT64` reaches `)` too,
// because SLL cannot tell the callers of `expr` apart. So the operator loop
// decision guards its iterate (the alias) with a stack test: the guard defers
// when an invoking state of the `CAST` operand is on the stack. Without the
// adaptive fallback, a deferring guard returns the exit, so the cast closes.
//
// The `AND` states of the same loop have their own guard group, with the
// invoking state of the `ROWS BETWEEN` operand: in `CAST ( a AND b AS INT64 )`
// the `CAST` operand on the stack must not make the `AND` exit.
grammar InlineAlias;

stmt : query EOF
     | 'CREATE' 'VIEW' ID 'ORDER' 'BY' expr (',' expr)* 'AS' query EOF
     ;

query : 'SELECT' expr (',' expr)* ('FROM' ID)? ('ROWS' 'BETWEEN' expr 'AND' expr)? ;

expr : expr 'AS' alias                   # aliased
     | expr '=' expr                     # eq
     | expr 'AND' expr                   # and
     | 'CAST' '(' expr 'AS' type ')'     # cast
     | '(' expr ')'                      # paren
     | ID                                # col
     | INT                               # num
     ;

alias : ID | 'INT64' ;

type : ID | 'INT64' ;

ID : [a-z_]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
