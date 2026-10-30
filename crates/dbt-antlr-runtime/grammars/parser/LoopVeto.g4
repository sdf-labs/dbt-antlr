// Witness for the substitution vetoes (`loop_block`): a guarded alternative
// that iterates internally and then owes another token (`(',' send)* '>>'
// send`) breaks the enter/exit mapping, so its iterate/exit conflict must not
// be resolved statically and escapes to the adaptive engine.
grammar LoopVeto;

start : send EOF ;

send
    : INT
    | send (',' send)* '>>' send
    ;

INT : [0-9]+ ;
WS : [ \t\r\n]+ -> skip ;
