// Witness for the owner of the tokens after a skipped optional postfix
// (`postfix`). The take rule and the guarded take resolve a take/skip conflict
// to take only when the skip reading consumes for the decision's own
// discipline. Here a new invocation of the decision rule consumes the tokens
// after the skip, so skip can win:
// - `start` puts two `r` side by side: `A - B ;` is `A` and `- B`. The take
//   reading `A (- B)` then has no second `r`.
// - `q` puts two `q` side by side in its own alternative, with a token after
//   them: `with [ A - B ] X` is `[ A (- B) ] X`.
// - `c` starts a new `c` in an optional element after its own postfix:
//   `cont A - B C !` is `A (- B) C !`. The shape check rejects `c`.
// - `m` calls `m` inside its take path, and the skip of the inner `m` returns
//   into the middle of that path: `mid A - B A - B A - B !` skips in the
//   second `m`.
// The static tables must leave these conflicts to adaptive prediction, or guard
// them on the stacks where the new invocation can follow.
grammar PostfixSibling;

start
    : r r ';' EOF
    | '(' r* ')' EOF
    | 'with' q EOF
    | 'cont' c EOF
    | 'mid' m EOF
    ;

r : 'A' y? | '-' 'B' ;

y : '-' 'B' ;

q : '[' q q ']' 'X' | 'A' z? | '-' 'B' ;

z : '-' 'B' ;

c : 'A' w? (c 'C' '!')? | '-' 'B' ;

w : '-' 'B' ;

m : 'A' (v (m '-' 'B' m? '!')?)? ;

v : '-' 'B' ;

WS : [ \t\r\n]+ -> skip ;
