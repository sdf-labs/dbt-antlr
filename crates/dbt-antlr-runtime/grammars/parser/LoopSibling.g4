// Witness for the owner of the tokens after the exit of an operator loop
// (`loop_block`). The iterate/exit substitution resolves a conflict of the
// loop to iterate only when an enclosing invocation of the same loop consumes
// the tokens of the exit reading. Here something else consumes them, and only
// the exit reading parses:
// - `start` puts two `expr` side by side: `a - - a ;` is `a` and `- - a`.
//   The second `expr` reads the next `'-'` as a prefix operator where the
//   loop reads a binary operator. The iterate reading `a - (- a)` then has no
//   second `expr`.
// - `pair` does the same in a base alternative of `term`: `with pair a - b`
//   is `pair a (- b)`. The iterate reading `pair (a - b)` has no second
//   `term`.
// - The alternative `cond '?' cond '-' ID` owes `'-' ID` after its middle
//   operand: `if a ? a - a` is `a ? a - a` with `a` in the middle. The iterate
//   reading `a ? (a - a)` then has no `'-' ID`.
// - The alternative `tail '?' tail ('-' ID '-' ID '!')?` can continue after
//   its middle operand: `opt a ? a - a - a !` is `a ? a - a - a !` with `a` in
//   the middle. The iterate reading `a ? (a - a - a)` then has no `'!'`.
// - The lower-precedence alternative `prec '+' prec ':' ID` owes `':' ID` after
//   its right operand, which the higher-precedence `prec '+' prec` does not
//   owe: `prec a * a + a : a` is `(a * a) + a : a`. In the precedence class of
//   `'*'`, the iterate reading `a * (a + a)` has no `':' ID`.
// The static tables of the five loops must defer these conflicts to adaptive
// prediction.
grammar LoopSibling;

start
    : expr expr ';' EOF
    | expr '-' '-' ';' EOF
    | 'with' term EOF
    | 'if' cond EOF
    | 'opt' tail EOF
    | 'prec' prec EOF
    ;

expr
    : expr '-' expr
    | expr '!'
    | '-' expr
    | ID
    ;

term
    : term '-' term
    | 'pair' term term
    | '-' term
    | ID
    ;

cond
    : cond '-' cond
    | cond '?' cond '-' ID
    | '-' cond
    | ID
    ;

tail
    : tail '-' tail
    | tail '?' tail ('-' ID '-' ID '!')?
    | '-' tail
    | ID
    ;

prec
    : prec '+' prec
    | prec '*' prec
    | prec '+' prec ':' ID
    | ID
    ;
ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
