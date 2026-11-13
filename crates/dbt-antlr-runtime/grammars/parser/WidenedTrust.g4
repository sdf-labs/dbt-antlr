// Witness for the uniform-trust resolution of a conflict that only widening
// makes (issue #84).
//
// In decision 1 of `a`, the alternative `B B B B a E` calls `a` again. The
// padding alternatives make the unbounded attempt spend its state budget, so
// the classifier falls back to the widened attempt. That attempt gives the
// inner `a` an empty context. After `'b' 'b' 'b' 'b' 'a'`, both alternatives
// then meet at the same state with the same context. The real stacks are
// different: only alternative 2 can read `c b b b b a d e d x`.
grammar WidenedTrust;
top : a X | a Y ;
a   : ( A
      | C ( B B B B A
          | B B B B a E
          | Z r0 M
          | Z (P|Q) (P|Q) (P|Q) (P|Q) (P|Q) (P|Q) (P|Q) (P|Q) (P|Q) (P|Q) W N
          )
      ) D ;
r0  : P r1 U0 | Q r1 V0 ;
r1  : P r2 U1 | Q r2 V1 ;
r2  : P r3 U2 | Q r3 V2 ;
r3  : P r4 U3 | Q r4 V3 ;
r4  : P r5 U4 | Q r5 V4 ;
r5  : P r6 U5 | Q r6 V5 ;
r6  : P r7 U6 | Q r7 V6 ;
r7  : P r8 U7 | Q r8 V7 ;
r8  : P r9 U8 | Q r9 V8 ;
r9  : P r10 U9 | Q r10 V9 ;
r10 : W ;
A : 'a' ; B : 'b' ; C : 'c' ; D : 'd' ; E : 'e' ; M : 'm' ; N : 'n' ;
P : 'p' ; Q : 'q' ; W : 'w' ; X : 'x' ; Y : 'y' ; Z : 'z' ;
U0 : 'u0' ; V0 : 'v0' ; U1 : 'u1' ; V1 : 'v1' ; U2 : 'u2' ; V2 : 'v2' ;
U3 : 'u3' ; V3 : 'v3' ; U4 : 'u4' ; V4 : 'v4' ; U5 : 'u5' ; V5 : 'v5' ;
U6 : 'u6' ; V6 : 'v6' ; U7 : 'u7' ; V7 : 'v7' ; U8 : 'u8' ; V8 : 'v8' ;
U9 : 'u9' ; V9 : 'v9' ;
WS : ' '+ -> skip ;
