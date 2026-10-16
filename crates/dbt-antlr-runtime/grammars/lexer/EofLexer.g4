lexer grammar EofLexer;

AB: 'ab' EOF;
A: 'a';
WS: [ \t\r\n]+ -> skip;
