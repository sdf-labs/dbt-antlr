lexer grammar StartRecurse;

// The start closure of R00 enters 33 nested fragment calls before it
// reaches an atom, so the closed configs carry a context deeper than the
// builder limit of 32 frames.

R00: R01 'a' | 'x';
fragment R01: R02 'a' | 'x';
fragment R02: R03 'a' | 'x';
fragment R03: R04 'a' | 'x';
fragment R04: R05 'a' | 'x';
fragment R05: R06 'a' | 'x';
fragment R06: R07 'a' | 'x';
fragment R07: R08 'a' | 'x';
fragment R08: R09 'a' | 'x';
fragment R09: R10 'a' | 'x';
fragment R10: R11 'a' | 'x';
fragment R11: R12 'a' | 'x';
fragment R12: R13 'a' | 'x';
fragment R13: R14 'a' | 'x';
fragment R14: R15 'a' | 'x';
fragment R15: R16 'a' | 'x';
fragment R16: R17 'a' | 'x';
fragment R17: R18 'a' | 'x';
fragment R18: R19 'a' | 'x';
fragment R19: R20 'a' | 'x';
fragment R20: R21 'a' | 'x';
fragment R21: R22 'a' | 'x';
fragment R22: R23 'a' | 'x';
fragment R23: R24 'a' | 'x';
fragment R24: R25 'a' | 'x';
fragment R25: R26 'a' | 'x';
fragment R26: R27 'a' | 'x';
fragment R27: R28 'a' | 'x';
fragment R28: R29 'a' | 'x';
fragment R29: R30 'a' | 'x';
fragment R30: R31 'a' | 'x';
fragment R31: R32 'a' | 'x';
fragment R32: R33 'a' | 'x';
fragment R33: 'y';
