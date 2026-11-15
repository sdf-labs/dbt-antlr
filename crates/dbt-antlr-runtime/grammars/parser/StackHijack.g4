// Witness of a stack-hijack hazard under `--static-dfa-only`, in the shape of
// the ClickHouse grammar, where a statement and each of its SELECT clauses
// can end with SETTINGS:
//
//   command : query settings? ';' ;
//   select  : 'SELECT' expr settings? ;
//
// The skip of the `settings?` of `select` reads SETTINGS in `command`, so the
// guard of the take defers when the call of `query` from `command` is on the
// parse stack. That call is an outer wrapper of `select`: between them, the
// call of `select` from `query` pops out of its rule. With a lookahead budget
// of 4 tokens, the guard cannot see the token after `SETTINGS a = 1`. So
// `SELECT 1 SETTINGS a = 1 UNION ALL SELECT 2 ;` skips, and then reports a
// syntax error at UNION.
grammar StackHijack;

options { max_k = 4; }

command : query settings? ';' ;

query : select ('UNION' 'ALL' select)* ;

select : 'SELECT' expr settings? ;

settings : 'SETTINGS' ID '=' INT (',' ID '=' INT)* ;

expr : ID | INT ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
