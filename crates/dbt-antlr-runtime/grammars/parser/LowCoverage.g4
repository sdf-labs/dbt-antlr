// Witness grammar for the coverage threshold of hybrid tables. In rule `a`,
// every first token except 'u' leads into a predicate, so the partial table
// decides only 1 of 11 first tokens. With the adaptive fallback, the table is
// not emitted. Under --static-dfa-only, the table is kept, and the predicated
// branches report a syntax error.
grammar LowCoverage;

s : a EOF ;

a : x 'b'
  | x 'c'
  | 'u'
  ;

x : 'p0' {true}? 'p0'
  | 'p1' {true}? 'p1'
  | 'p2' {true}? 'p2'
  | 'p3' {true}? 'p3'
  | 'p4' {true}? 'p4'
  | 'p5' {true}? 'p5'
  | 'p6' {true}? 'p6'
  | 'p7' {true}? 'p7'
  | 'p8' {true}? 'p8'
  | 'p9' {true}? 'p9'
  ;

WS : [ \t\r\n]+ -> skip ;
