lexer grammar EofChain;

// A chain of EOF matches. The builder computes EOF edges with the lazy
// treat_eofas_epsilon closure semantics, so the closure of each post-EOF
// config follows the rest of the chain as epsilon: every EOF-edge target
// is an accept state, and the walker breaks at the first EOF accept
// (exactly like the lazy simulator). The differential check is the point
// of this grammar; the walker's EOF-chain bound stays defensive and does
// not fire here.

A: 'a' EOF EOF EOF EOF EOF EOF EOF EOF EOF EOF;
B: 'b';
