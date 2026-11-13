parser grammar OrphanParser;
options { tokenVocab = OtherLexer; }
prog : stat+ EOF ;
stat : ID ID ID SEMI | ID ID EQ ID SEMI ;
