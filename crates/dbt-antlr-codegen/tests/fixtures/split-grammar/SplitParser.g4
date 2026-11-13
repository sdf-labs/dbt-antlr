parser grammar SplitParser;

options { tokenVocab = SplitLexer; }

statement : SELECT ID CACHE? (CACHE | ID) EOF ;
