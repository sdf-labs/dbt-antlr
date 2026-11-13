parser grammar SplitParser;

options { tokenVocab = SplitLexer; }

statement : SELECT ID ENABLE? (CACHE | ID) EOF ;
