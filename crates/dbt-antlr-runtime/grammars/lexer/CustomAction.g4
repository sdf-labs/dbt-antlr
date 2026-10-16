lexer grammar CustomAction;

CUST: 'cust' {custom();} 'x';
ID: [a-z]+;
INT: [0-9]+;
WS: [ \t\r\n]+ -> skip;
