// Witness of guard roots lost to the full-context follow chase (issue #81).
// The operator loop of `e` is reachable from three callers: the `WRAP`
// prefix and the two `CAST`-like wrappers. The exit reading of the loop
// collects its guard roots at its first foreign consumption, which happens
// on the `WRAP` stack: exiting the loop and consuming the `'+'` of
// `'WRAP' e '+' sub '!'`. That consumption roots the exit config. The
// full-context follow chase of the rule stop of `e` then carries the rooted
// taint to the follow positions of the other callers, so when the same
// config consumes the `'AS'` of a `CAST` wrapper — its first foreign
// consumption on that stack — it collects no root for it. The guarded
// iterate then defers only on the `WRAP` stack, and
// `CAST ( a * b + y AS t u )` misreads the loop as iterating where the exit
// is the only valid reading.
//
// Without the `WRAP` alternative, the first foreign consumption of the exit
// reading is the `'AS'` of a `CAST` wrapper itself, so the danger set names
// every caller.
grammar Repro4TwoCastWithWrap;

stmt : 'WRAP' e '+' sub '!' EOF
     | 'CAST' '(' e 'AS' ID ID ')' EOF
     | 'CAST2' '(' e 'AS' ID ID ')' EOF
     | e EOF
     ;

e : e '+' sub 'AS' ID ID 'ONLY_HIGH'
  | e '*' e
  | e '+' sub
  | ID
  ;

sub : '(' sub ')' | ID ;

ID : [a-z]+ ;
WS : [ \t\n]+ -> skip ;
