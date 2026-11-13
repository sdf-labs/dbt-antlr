// Witness for the precedence filter of a widened precedence-class attempt
// (issue #87).
//
// The chain `c0` to `c10` makes the unbounded attempts of two precedence
// classes spend the state budget, so the classifier falls back to the widened
// attempt. That attempt gives every rule call an empty context. In the class
// of the right operand of `e '*' e`, the exit reading of `x*x+qr` returns to
// the outer loop and reads `+ q r` through `e op`. The iterate reading reads
// `+` through `e op 'a'`. Both readings call `op`, so after widening they meet
// at the same state with the same empty context. The precedence filter must
// not drop the exit config there: it is the only viable reading.
grammar WidenedPrecedence;
s : e EOF ;
e : e op 'a'
  | e c0 'm1'
  | e '*' e
  | e c0
  | e op
  | 'x'
  ;
op : '+' 'q' 'r' ;
c0 : 'u' c1 't0a' | 'v' c1 't0b' ;
c1 : 'u' c2 't1a' | 'v' c2 't1b' ;
c2 : 'u' c3 't2a' | 'v' c3 't2b' ;
c3 : 'u' c4 't3a' | 'v' c4 't3b' ;
c4 : 'u' c5 't4a' | 'v' c5 't4b' ;
c5 : 'u' c6 't5a' | 'v' c6 't5b' ;
c6 : 'u' c7 't6a' | 'v' c7 't6b' ;
c7 : 'u' c8 't7a' | 'v' c8 't7b' ;
c8 : 'u' c9 't8a' | 'v' c9 't8b' ;
c9 : 'u' c10 't9a' | 'v' c10 't9b' ;
c10 : 'u' 'end' 't10a' | 'v' 'end' 't10b' ;
WS : ' '+ -> skip ;
