// Witness of guard roots lost when a rooted and an unrooted lineage merge
// (issue #81). The operator loop of `e` has three callers: the `WRAP` prefix
// and the two `CAST`-like wrappers.
//
//   stmt : 'WRAP' e '+' sub '!'            # exit reading roots here first
//        | 'CAST' '(' e 'AS' ID ID ')'     # the clean lineage
//        | 'CAST2' '(' e 'AS' ID ID ')'    # the other clean lineage
//
// The exit reading of the loop consumes the `WRAP` continuation first and
// roots at the `WRAP` caller. On the `CAST` and `CAST2` stacks the same
// `(state, alt)` is reached by a lineage that is not rooted yet. When the two
// lineages merge, the clean lineage inherits `ROOTED_TAINT` from the rooted
// one, so its own first foreign consumption no longer collects the `CAST` and
// `CAST2` roots. The guarded iterate then defers only on the `WRAP` stack.
//
// Without the `WRAP` alternative both lineages collect their own roots and the
// danger set names every caller.
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
