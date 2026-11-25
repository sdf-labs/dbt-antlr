grammar DeferredSync;

ll1Star : '(' ID* ')' ;
ll1Plus : '(' ID+ ')' ;
trailStar : ID* ;
trailPlus : ID+ ;

dfaStar : '(' ('a' 'b')* 'a' 'c' ')' ;
dfaPlus : '(' ('a' 'b')+ 'a' 'c' ')' ;

ll1Alt : ('a' ID | 'b' ID) ';' ;
dfaAlt : ('a' 'b' | 'a' 'c' | 'd' 'e') ';' ;
dfaOpt : '(' ('a' 'b')? 'a' 'c' ')' ;

termOptTop : termOpt ';' ;
termOpt : 'a' 'b'? ;

nestedOptTop : outerOpt ';' ;
outerOpt : innerOpt 'b'? ;
innerOpt : 'a' 'c'? ;

zeroSibTop : sibOne sibTwo ';' ;
sibOne : 'a' 'b'? ;
sibTwo : 'c'? ;

startEmptyTop : sibTwo ';' ;
nonAdjTop : termOpt 'd' ';' ;
intraOpt : 'a' 'b'? ';' ;

ambigTermTop : ambigTerm ';' ;
ambigTerm : 'd' 'a' 'b'? | 'e' 'a' 'c'? ;

samePostTermTop : samePostTerm ';' ;
samePostTerm : ('d' 'a' | 'e' 'a') 'b'? ;

conjuredTermTop : conjuredTerm ';' ;
conjuredTerm : 'd' 'a' 'b'? ;

A : 'a' ;
B : 'b' ;
C : 'c' ;
D : 'd' ;
E : 'e' ;
PLUS : '+' ;
LPAREN : '(' ;
RPAREN : ')' ;
SEMI : ';' ;
ID : [f-z]+ ;
WS : [ \t\r\n]+ -> skip ;
