lexer grammar MiniModes;

TEXT: ~[<]+;
LT: '<' -> pushMode(INSIDE);

mode INSIDE;
GT: '>' -> popMode;
NAME: [a-zA-Z]+;
SLASH: '/';
INSIDE_WS: [ \t\r\n]+ -> skip;
