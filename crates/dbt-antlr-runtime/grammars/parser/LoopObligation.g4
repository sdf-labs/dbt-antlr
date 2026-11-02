// Witness for obligation discharge in the guarded-iterate analysis of a
// precedence loop (`loop_block`).
//
// In `p`:
// - `'prior' p '.' ID` is a base alternative that consumes `'.' ID` after `p`.
//   On lookahead `'.' ID`, the exit reading of `p` collides with the operator
//   alternative `p '.' ID`.
// - `'trim' '(' p p ')'` puts two `p` calls side by side. `'[' ID ':' p ']'` is
//   a base alternative that collides with `p '[' ID ':' p ']'` out to the
//   hybrid depth cap.
//
// On these paths, the exit lineage returns into the first `p` of `'trim'` and
// enters the second `p`. Its context then holds a pending return into `')'`
// that the iterate reading does not hold at the depth cap. That obligation
// belongs to a foreign invocation that `FOREIGN_CONSUME_TAINT` already roots,
// so `LoopBlock::has_unshared_obligation` discharges it. All 10 iterate/exit
// states (2 conflict states and 8 budget-frontier states) resolve under the
// runtime stack guard, with 0 escapes.
grammar LoopObligation;

s : p EOF ;

p : p '[' ID ':' p ']'
  | p '.' ID
  | p '+' p
  | 'prior' p '.' ID
  | 'trim' '(' p p ')'
  | '[' ID ':' p ']'
  | ID
  ;

ID : [a-z]+ ;
WS : [ \t\r\n]+ -> skip ;
