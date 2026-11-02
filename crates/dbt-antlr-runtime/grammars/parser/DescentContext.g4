// Shared-descent candidates whose shared rule can both exit and continue on a
// token that disambiguates the members at the shared call site. Condition 2
// excludes all of `d`, `g`, `k`, and `h` from shared-descent groups at compile
// time, so the static parser never starts a neutral parse that would need to
// be reverted.
grammar DescentContext;

s : 'one' ('prefix' d INT | d) EOF
  | 'two' ('prefix' g INT | g) EOF
  | 'three' ('prefix' k 'as' ID ';' INT | k) EOF
  ;

// `r` can end after `a` (alternative 1 goes on with '+') or after `a + b`
// (alternative 2 goes on with ';'). Only a full-context prediction in `r`
// tells them apart, and it reads the call site of `d`.
d : r '+' ID ';' ID
  | r ';' ID INT
  ;

r : '(' r ')'
  | ID '+' ID
  | ID
  ;

// `h` is a shared-descent group of its own, with a guarded arm on 'as': the
// guard reads the call site of `g`.
g : h 'as' ID ';' ID
  | h ';' ID INT
  ;

// The neutral parse of `h` succeeds, but after `prefix` no tail arm of `k`
// takes ')'; the strategy must not keep the neutral parse's recovery state.
k : h 'as' ID ';' INT
  | h ';' ID
  ;

h : '(' h ')'
  | e 'as' ID
  | e
  ;

e : ID ('.' ID (',' ID)*)* ;

ID : [a-z]+ ;
INT : [0-9]+ ;
WS : [ \t\n]+ -> skip ;
