parser grammar SplitParser;
options { tokenVocab = SplitLexer; }
prog : stat+ EOF ;
stat options { max_dfa_states = 3; }
     : ID ID ID SEMI
     | ID ID EQ ID SEMI
     ;
