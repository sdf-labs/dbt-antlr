//! Implementation of lexer automata(DFA)
use crate::atn::ATN;
use crate::atn_config::LexerATNConfig;
use crate::atn_config_set::{LexerATNConfigSet, MutableLexerATNConfigSet};
use crate::atn_simulator::{BaseATNSimulator, IATNSimulator};
use crate::atn_state::{ATNState, ATNStateRef, ATNStateType};
use crate::char_stream::CharStream;
use crate::dfa::static_lexer_dfa::{
    Accel, LexerContinuation, StaticLexerDFA, StaticLexerTables, ACCEPT_BIT,
    CONTINUATION_MOVED_CONFIGS, CONTINUATION_SOURCE_SET, ESCAPE, NO_CONTINUATION, NO_EDGE,
    STATE_MASK,
};
use crate::dfa::{DFAState, ProposedDFAState, DFA};
use crate::errors::ANTLRError;
use crate::int_stream::{IntStream, EOF};
use crate::lexer::{Lexer, LEXER_MAX_CHAR_VALUE, LEXER_MIN_CHAR_VALUE};
use crate::lexer_action_executor::LexerActionExecutor;
use crate::prediction_context::PredictionContextRef;
use crate::prediction_context::{PredictionContext, PredictionContextCache};
use crate::token::TOKEN_EOF;

use crate::token_factory::TokenFactory;
use crate::transition::{Transition, TransitionType};

// todo rewrite this to be actually usable
#[doc(hidden)]
pub trait ILexerATNSimulator<'sim>: IATNSimulator<'sim, LexerATNConfigSet<'sim>> {
    fn reset(&mut self);
    fn match_token<'input, 'arena, Input, TF>(
        &mut self,
        mode: usize,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena;
    fn consume<T: IntStream + ?Sized>(&self, input: &mut T);
    #[cold]
    fn recover(&mut self, _re: ANTLRError, input: &mut impl IntStream) {
        if input.la(1) != EOF {
            self.consume(input)
        }
    }
}

/// Simple DFA implementation enough for lexer.
#[derive(Debug)]
pub struct LexerATNSimulator<'sim> {
    base: BaseATNSimulator<'sim, LexerATNConfigSet<'sim>>,

    start_index: isize,
    mode: usize,
    prev_accept: Option<ResolvedAccept<'sim>>,
    static_tables: Option<&'static StaticLexerTables>,
    /// Number of escape edges the table walker handed to the lazy path.
    #[doc(hidden)]
    pub escape_count: usize,
    /// Number of tokens the table walker restarted on the lazy path.
    #[doc(hidden)]
    pub restart_count: usize,
}

impl<'sim> ILexerATNSimulator<'sim> for LexerATNSimulator<'sim> {
    fn reset(&mut self) {
        self.prev_accept = None
    }

    fn match_token<'input, 'arena, Input, TF>(
        &mut self,
        mode: usize,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let scratch = bumpalo::Bump::new();

        self.mode = mode;
        let mark = lexer.input().mark();
        let result = (|| {
            self.start_index = lexer.input().index();
            self.prev_accept = None;
            // Table-driven fast path: modes the builder compiled have a
            // static DFA table. Escape edges inside the walk hand the token
            // back to the lazy path.
            if let Some(tables) = self.static_tables {
                if let Some(table) = tables.mode(mode) {
                    return self.match_static(table, lexer, &scratch);
                }
            }
            self.match_token_lazy(lexer, &scratch)
        })();
        lexer.input().release(mark);
        result
    }

    #[inline]
    fn consume<T: IntStream + ?Sized>(&self, input: &mut T) {
        input.consume();
    }
}

impl<'sim> IATNSimulator<'sim, LexerATNConfigSet<'sim>> for LexerATNSimulator<'sim> {
    fn shared_context_cache(&self) -> &'sim PredictionContextCache<'sim> {
        self.base.shared_context_cache()
    }

    fn atn(&self) -> &'static ATN {
        self.base.atn()
    }

    fn decision_to_dfa(&self, decision: usize) -> Option<&'sim DFA<'sim, LexerATNConfigSet<'sim>>> {
        self.base.decision_to_dfa(decision)
    }
}

#[allow(missing_docs)]
pub const MIN_DFA_EDGE: i32 = 0;
#[allow(missing_docs)]
pub const MAX_DFA_EDGE: i32 = 127;
pub const LEXER_DFA_EDGE_SET_SIZE: usize = (MAX_DFA_EDGE - MIN_DFA_EDGE + 1) as usize;

impl<'sim> LexerATNSimulator<'sim> {
    /// Creates `LexerATNSimulator` instance which creates DFA over `atn`
    pub fn new(base: BaseATNSimulator<'sim, LexerATNConfigSet<'sim>>) -> LexerATNSimulator<'sim> {
        LexerATNSimulator {
            // Note: already validated by the manager at construction:
            static_tables: base.static_lexer_tables,
            base,
            start_index: 0,
            mode: 0,
            prev_accept: None,
            escape_count: 0,
            restart_count: 0,
        }
    }

    /// Attaches static lexer DFA tables. Modes with a table tokenize
    /// through the table walker; modes without one (and every escape edge)
    /// keep using the lazy path.
    ///
    /// Panics when the tables reference ATN states or lexer actions the ATN
    /// of this simulator does not have: the tables and the ATN come from
    /// the same generated lexer, so a mismatch is a build inconsistency.
    /// Regenerate the lexer.
    ///
    /// This is mostly for use by tests.
    pub fn set_static_tables(&mut self, tables: &'static StaticLexerTables) {
        tables.validate_for_atn(self.atn());
        self.static_tables = Some(tables);
    }

    /// The lazy `match_token` path: ATN simulation with the lazily built
    /// DFA cache.
    fn match_token_lazy<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let dfa = self
            .decision_to_dfa(self.mode)
            .ok_or_else(|| ANTLRError::illegal_state("invalid mode".into()))?;

        match dfa.s0() {
            None => self.match_atn(lexer, dfa, scratch),
            Some(s0) => self.exec_atn(s0, lexer, dfa, scratch),
        }
    }

    /// Token matching driven by a statically-precomputed lexer DFA table
    /// (see [`crate::dfa::static_lexer_dfa`]): one dense table lookup per input
    /// symbol, no ATN simulation, no lazily-built DFA states. Mirrors
    /// `exec_atn` + `fail_or_accept` exactly (maximal munch, accept fallback
    /// with position restore, lexer actions at accept, EOF handling); the
    /// tables are built by driving the reference simulator exhaustively at
    /// generation time, so the behavior is identical by construction.
    ///
    /// Edges that cannot get a static target hold the `ESCAPE` sentinel;
    /// the walk then hands the token to
    /// [`LexerATNSimulator::match_token_from_continuation`] or restarts it
    /// lazily when the escape carries no continuation.
    fn match_static<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        table: &'static StaticLexerDFA,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let start_index = self.start_index;
        let mut s = table.start_state();
        // (index, state) of the most recent accept state.
        // Accept states are marked by accept_type >= 0 (not != 0): a rule
        // with a `more`/`type(...)` command accepts with token type 0 (the
        // tool assigns such rules no token type), so a zero-length match at
        // the start state of a mode still counts as an accept.
        let mut prev_accept: Option<(isize, u16)> = if table.accept_type(s) >= 0 {
            Some((start_index, s))
        } else {
            None
        };
        // Consecutive EOF edges taken. EOF consumes no input and grammars
        // can have unbounded EOF epsilon chains, so the walk bounds the
        // chain and restarts the token lazily past the bound.
        let mut eof_chain = 0u32;

        // With a random-access input (InputStream) hold a local byte
        // cursor and peek the next input byte directly - one load +
        // high-bit test for ASCII - instead of a `la(1)` decode plus a
        // `consume()` walk per char; other streams take the cursor-based
        // path. On exit, `symbol` is the (unconsumed) char at the position
        // the walk stopped.
        let mut symbol;
        if lexer.input().has_item_at() {
            // Byte-cursor path: per input char, one byte peek plus one
            // table entry (target state and accept bit in the same load).
            let input = lexer.input();
            let mut cursor = start_index;
            let bytes = input.bytes_at(0);
            symbol = input.item_at(cursor).unwrap_or(EOF);
            loop {
                let entry = table.edge_entry(s, symbol);
                if entry == NO_EDGE {
                    break;
                }
                if entry == ESCAPE {
                    return self.static_escape(
                        table,
                        s,
                        symbol,
                        cursor,
                        prev_accept,
                        lexer,
                        scratch,
                    );
                }
                let target = entry & STATE_MASK;
                if symbol != EOF {
                    eof_chain = 0;
                    cursor += if symbol < 0x80 {
                        1
                    } else {
                        char::from_u32(symbol as u32).map_or(1, |c| c.len_utf8() as isize)
                    };
                } else {
                    eof_chain += 1;
                    if eof_chain > MAX_STATIC_EOF_EDGES {
                        return self.restart_token_lazy(lexer, scratch);
                    }
                }
                if entry & ACCEPT_BIT != 0 {
                    prev_accept = Some((cursor, target));
                    if symbol == EOF {
                        break;
                    }
                }
                symbol = input.item_at(cursor).unwrap_or(EOF);
                s = target;

                if let Some(hay) = bytes {
                    let tag = table.accel_tag(s);
                    if tag != 0 {
                        let accel = table.accel_payload(tag);
                        let start = cursor as usize;
                        let c = run_scan(hay, start, accel);
                        if c > start {
                            cursor = c as isize;
                            // The entry's accept bit is s's: on an
                            // accept state the run end is the
                            // maximal-munch accept.
                            if entry & ACCEPT_BIT != 0 {
                                prev_accept = Some((cursor, s));
                            }
                            symbol = match hay.get(c) {
                                Some(&b) if b < 0x80 => b as i32,
                                _ => input.item_at(cursor).unwrap_or(EOF),
                            };
                        }
                    }
                }
            }
            input.seek(cursor);
        } else {
            symbol = lexer.input().la(1);
            loop {
                let entry = table.edge_entry(s, symbol);
                if entry == NO_EDGE {
                    break;
                }
                if entry == ESCAPE {
                    return self.static_escape(
                        table,
                        s,
                        symbol,
                        lexer.input().index(),
                        prev_accept,
                        lexer,
                        scratch,
                    );
                }
                let target = entry & STATE_MASK;
                if symbol != EOF {
                    eof_chain = 0;
                    lexer.input().consume();
                } else {
                    eof_chain += 1;
                    if eof_chain > MAX_STATIC_EOF_EDGES {
                        return self.restart_token_lazy(lexer, scratch);
                    }
                }
                if entry & ACCEPT_BIT != 0 {
                    prev_accept = Some((lexer.input().index(), target));
                    if symbol == EOF {
                        break;
                    }
                }
                symbol = lexer.input().la(1);
                s = target;
            }
        }

        match prev_accept {
            Some((index, state)) => {
                lexer.input().seek(index);
                // Position-independent lexer actions only (skip, mode,
                // channel, type, ...): the loader rejects everything else
                // in accept lists, and their execution needs no match
                // offset, so plain `execute` matches
                // `LexerActionExecutor::execute` here.
                for action in table.accept_actions(state) {
                    action.execute(lexer);
                }
                Ok(table.accept_type(state))
            }
            None => {
                if symbol == EOF && lexer.input().index() == start_index {
                    return Ok(TOKEN_EOF);
                }
                Err(ANTLRError::lexer_no_alt(start_index))
            }
        }
    }

    /// Handles an ESCAPE edge hit by [`LexerATNSimulator::match_static`]:
    /// positions the input at the unconsumed escape symbol, carries the best
    /// accept so far into `self.prev_accept`, and resumes the lazy simulation
    /// from the edge's continuation. A budget escape has no continuation and
    /// restarts the token on the lazy path from the token start.
    #[cold]
    #[allow(clippy::too_many_arguments)]
    fn static_escape<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        table: &'static StaticLexerDFA,
        state: u16,
        symbol: i32,
        cursor: isize,
        prev_accept: Option<(isize, u16)>,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        self.escape_count += 1;
        lexer.input().seek(cursor);
        self.prev_accept = prev_accept.map(|(index, state)| {
            let actions = table.accept_actions(state);
            ResolvedAccept {
                index,
                prediction: table.accept_type(state),
                executor: if actions.is_empty() {
                    None
                } else {
                    Some(AcceptExecutor::Static(actions))
                },
            }
        });
        let continuation = table.escape_continuation(state, symbol);
        if continuation == NO_CONTINUATION {
            return self.restart_token_lazy(lexer, scratch);
        }
        self.match_token_from_continuation(table.continuation(continuation), symbol, lexer, scratch)
    }

    /// Restarts the current token on the lazy path from the token start.
    /// Used for budget escapes (no continuation) and for EOF epsilon chains
    /// past the walk bound.
    #[cold]
    fn restart_token_lazy<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        self.restart_count += 1;
        lexer.input().seek(self.start_index);
        self.prev_accept = None;
        self.match_token_lazy(lexer, scratch)
    }
    /// Resumes the lazy simulation at an escape edge. Rebuilds the
    /// serialized configs of the continuation, closes them with real
    /// predicate evaluation (the input is at the unconsumed escape symbol),
    /// interns the closed set through `add_dfastate`, and continues the
    /// normal exec loop for the rest of the token. The best accept of the
    /// table walk is already in `self.prev_accept`, so maximal munch across
    /// the hand-off is exact.
    fn match_token_from_continuation<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        continuation: &'static LexerContinuation,
        escape_symbol: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let atn = self.atn();
        let start_index = self.start_index;

        // Rebuild the singleton context chains bottom-up: local id 0 is the
        // empty context, entry i has local id i + 1 with parents that refer
        // to smaller ids.
        let mut contexts = Vec::with_capacity(continuation.ctxs.len() + 1);
        contexts.push(PredictionContextRef::new_empty());
        for &(parent, return_state) in &continuation.ctxs {
            let ctx = PredictionContext::new_singleton(
                Some(contexts[parent as usize]),
                atn.make_state_ref(return_state),
            );
            contexts.push(PredictionContextRef::from(scratch.alloc(ctx)));
        }

        let configs: Vec<LexerATNConfig> = continuation
            .configs
            .iter()
            .map(|c| {
                let mut config = LexerATNConfig::new(
                    atn.make_state_ref(c.state),
                    c.alt,
                    contexts[c.ctx as usize],
                );
                config.set_passed_through_non_greedy_decision(c.non_greedy);
                let mut executor = None;
                for &action_index in &c.actions {
                    let action: crate::lexer_action::LexerAction =
                        atn.lexer_actions[action_index as usize].clone();
                    executor = Some(&*scratch.alloc(LexerActionExecutor::new_copy_append(
                        scratch, executor, action,
                    )));
                }
                config.with_lexer_executor(executor)
            })
            .collect();

        let mut reach = MutableLexerATNConfigSet::new(scratch);
        match continuation.kind {
            CONTINUATION_MOVED_CONFIGS => {
                // The configs are post-move, pre-closure: replay the
                // closure of each with its serialized
                // alt_reached_accept flag.
                let mut env = LazyClosureEnv::new(self, lexer);
                for (config, serialized) in configs.iter().zip(continuation.configs.iter()) {
                    closure_core(
                        atn,
                        config.clone(),
                        &mut reach,
                        serialized.alt_reached_accept,
                        true,
                        escape_symbol == EOF,
                        &mut env,
                        scratch,
                    );
                }
            }
            CONTINUATION_SOURCE_SET => {
                // The configs are the source set of the state: re-run the
                // full move step, so the pruning decisions use runtime
                // predicate results.
                let mut env = LazyClosureEnv::new(self, lexer);
                reachable_core(
                    atn,
                    start_index,
                    configs.iter(),
                    &mut reach,
                    escape_symbol,
                    &mut env,
                    None,
                    scratch,
                );
            }
            _ => unreachable!("continuation kinds are validated at load"),
        }

        if reach.is_empty() {
            // The resumed move has no target: the lazy equivalent is the
            // error state, which ends the walk at the escape position.
            return self.fail_or_accept(escape_symbol, lexer);
        }
        let dfa = self.get_dfa();
        // As in compute_target_state, a predicate in the closure suppresses
        // the caching of the edge; the resumed edge comes from a table
        // state, so there is no edge to cache in any case. The interned
        // state itself dedups against the lazy DFA states.
        reach.set_has_semantic_context(false);
        let to = self.add_dfastate(dfa, reach)?;

        if escape_symbol != EOF {
            self.consume(lexer.input());
        }
        if self.capture_sim_state(lexer.input(), to) && escape_symbol == EOF {
            return self.fail_or_accept(escape_symbol, lexer);
        }
        let symbol = lexer.input().la(1);
        self.exec_atn_loop(to, symbol, lexer, dfa, scratch)
    }

    #[cold]
    fn match_atn<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        dfa: &'sim DFA<LexerATNConfigSet<'sim>>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        //        let start_state = self.atn().mode_to_start_state.get(self.mode as usize).ok_or(ANTLRError::IllegalStateError("invalid mode".into()))?;
        let atn = self.atn();
        let start_state = *atn
            .mode_to_start_state
            .get(self.mode)
            .ok_or_else(|| ANTLRError::illegal_state("invalid mode".into()))?;

        let _old_mode = self.mode;
        let mut s0_closure = self.compute_start_state(start_state.as_ref(), lexer, scratch);
        let _supress_edge = s0_closure.has_semantic_context();
        s0_closure.set_has_semantic_context(false);

        let next_state = self.add_dfastate(dfa, s0_closure)?;
        if !_supress_edge {
            dfa.set_s0(next_state);
        }

        self.exec_atn(next_state, lexer, dfa, scratch)
    }

    fn exec_atn<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        //        input: &'a mut dyn CharStream,
        ds0: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        dfa: &'sim DFA<LexerATNConfigSet<'sim>>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        //        if self.get_dfa().states.read().unwrap().get(ds0).unwrap().is_accept_state{
        self.capture_sim_state(lexer.input(), ds0);
        //        }

        let symbol = lexer.input().la(1);
        self.exec_atn_loop(ds0, symbol, lexer, dfa, scratch)
    }

    /// The inner loop of `exec_atn`: from state `s` with `symbol` the
    /// (unconsumed) lookahead. Also the tail of the continuation resume
    /// path, which has already consumed the escape symbol.
    fn exec_atn_loop<'scratch, 'input, 'arena, Input, TF>(
        &mut self,
        mut s: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
        mut symbol: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        dfa: &'sim DFA<'sim, LexerATNConfigSet<'sim>>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        loop {
            let target = if let Some(target) = dfa.get_edge(s, (symbol - MIN_DFA_EDGE) as usize) {
                target
            } else {
                self.compute_target_state(dfa, s, symbol, lexer, scratch)?
            };

            if target.is_error_state() {
                break;
            }
            //            println!(" --- target computed {:?}", self.get_dfa().states.read().unwrap()[target].configs.configs.iter().map(|it|it.get_state()).collect::<Vec<_>>());

            if symbol != EOF {
                self.consume(lexer.input());
            }

            if self.capture_sim_state(lexer.input(), target) && symbol == EOF {
                break;
            }

            symbol = lexer.input().la(1);

            s = target;
        }
        // let _last = self.get_dfa().states.read().get(s).unwrap();

        self.fail_or_accept(symbol, lexer)
    }

    #[cold]
    fn compute_target_state<'scratch, 'input, 'arena, Input, TF>(
        &self,
        dfa: &'sim DFA<LexerATNConfigSet<'sim>>,
        s: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
        _t: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<&'sim DFAState<'sim, LexerATNConfigSet<'sim>>, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let mut reach = MutableLexerATNConfigSet::new(scratch);
        self.get_reachable_config_set(s.configs(), &mut reach, _t, lexer, scratch);
        //        println!(" --- target computed {:?}", reach.configs.iter().map(|it|it.get_state()).collect::<Vec<_>>());

        // let mut states = dfa_mut.states;
        if reach.is_empty() {
            if !reach.has_semantic_context() {
                self.add_dfaedge(dfa, s, _t, dfa.get_error_state());
            }

            return Ok(dfa.get_error_state());
        }

        let supress_edge = reach.has_semantic_context();
        reach.set_has_semantic_context(false);
        let to = self.add_dfastate(dfa, reach)?;
        if !supress_edge {
            let from = s;
            self.add_dfaedge(dfa, from, _t, to);
        }
        //        println!("target state computed from {:?} to {:?} on symbol {}", _s, to, char::try_from(_t as u32).unwrap());
        Ok(to)
    }

    fn get_reachable_config_set<'scratch, 'input, 'arena, Input, TF>(
        &self,
        // _states: &V,
        //        _input: &mut dyn CharStream,
        _closure: &'scratch LexerATNConfigSet<'scratch>,
        _reach: &mut MutableLexerATNConfigSet<'scratch>,
        _t: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) where
        'input: 'arena,
        'sim: 'scratch,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        reachable_core(
            self.atn(),
            self.start_index,
            _closure.get_items(),
            _reach,
            _t,
            &mut LazyClosureEnv::new(self, lexer),
            None,
            scratch,
        )
    }

    //    fn get_reachable_target<T>(&self, states: &T, _trans: &Transition, _t: i32) -> &ATNState
    //    where
    //        T: Deref<Target = Vec<DFAState>>,
    //    {
    //        unimplemented!()
    //    }

    fn fail_or_accept<'input, 'arena, Input, TF>(
        &mut self,
        _t: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        //        println!("fail_or_accept");
        if let Some(accept) = self.prev_accept.take() {
            lexer.input().seek(accept.index);

            match accept.executor {
                Some(AcceptExecutor::Lazy(x)) => x.execute(lexer, self.start_index),
                // Position-independent actions only: no match offset is
                // needed, so plain `execute` matches
                // `LexerActionExecutor::execute` here.
                Some(AcceptExecutor::Static(actions)) => {
                    for action in actions {
                        action.execute(lexer);
                    }
                }
                None => {}
            }

            Ok(accept.prediction)
        } else {
            if _t == EOF && lexer.input().index() == self.start_index {
                return Ok(TOKEN_EOF);
            }
            Err(ANTLRError::lexer_no_alt(self.start_index))
        }
    }

    fn compute_start_state<'scratch, 'input, 'arena, Input, TF>(
        &self,
        p: &ATNState,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> MutableLexerATNConfigSet<'scratch>
    where
        'input: 'arena,
        'sim: 'scratch,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        start_state_core(
            self.atn(),
            p,
            &mut LazyClosureEnv::new(self, lexer),
            scratch,
        )
    }

    fn evaluate_predicate<'input, 'arena, Input, TF, T: Lexer<'input, 'arena, Input, TF>>(
        &self,
        rule_index: i32,
        pred_index: i32,
        speculative: bool,
        lexer: &mut T,
    ) -> bool
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        if !speculative {
            return lexer.sempred(None, rule_index, pred_index);
        }

        let index = lexer.input().index();
        let marker = lexer.input().mark();
        self.consume(lexer.input());

        let result = lexer.sempred(None, rule_index, pred_index);

        lexer.input().seek(index);
        lexer.input().release(marker);
        result
    }

    fn capture_sim_state(
        &mut self,
        input: &impl IntStream,
        dfa_state: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
    ) -> bool {
        if dfa_state.is_accept_state() {
            // Resolve the DFAState into a ResolvedAccept here, so the
            // accept slot has one form for lazy states and for the table
            // accepts the static walker carries.
            self.prev_accept = Some(ResolvedAccept {
                index: input.index(),
                prediction: dfa_state.prediction(),
                executor: dfa_state.lexer_action_executor().map(AcceptExecutor::Lazy),
            });
            return true;
        }
        false
    }

    fn add_dfaedge(
        &self,
        dfa: &'sim DFA<LexerATNConfigSet<'sim>>,
        from: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
        t: i32,
        to: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
    ) {
        if !(MIN_DFA_EDGE..=MAX_DFA_EDGE).contains(&t) {
            return;
        }

        dfa.set_edge(from, (t - MIN_DFA_EDGE) as usize, to);
    }

    fn add_dfastate<'scratch>(
        &self,
        dfa: &'sim DFA<LexerATNConfigSet<'sim>>,
        configs: MutableLexerATNConfigSet<'scratch>,
    ) -> Result<&'sim DFAState<'sim, LexerATNConfigSet<'sim>>, ANTLRError> {
        assert!(!configs.has_semantic_context());

        let state = ProposedDFAState::new(configs);

        self.base.check_allocation_limit()?;
        Ok(dfa.add_lexer_state(state, self))
    }

    /// Returns current DFA that is currently used.
    pub fn get_dfa(&self) -> &'sim DFA<'sim, LexerATNConfigSet<'sim>> {
        self.decision_to_dfa(self.mode)
            .expect("self.mode should be valid")
    }

    /// Returns current DFA for particular lexer mode
    pub fn get_dfa_for_mode(&self, mode: usize) -> &'sim DFA<'sim, LexerATNConfigSet<'sim>> {
        self.decision_to_dfa(mode).expect("mode should be valid")
    }

    /// Returns the current size of the DFA cache in bytes.
    pub fn total_allocated_bytes(&self) -> usize {
        self.base.total_allocated_bytes()
    }

    // fn get_token_name(&self, _tt: i32) -> String { unimplemented!() }
}

/// Bound on consecutive EOF edges the table walker takes before it
/// restarts the token lazily: EOF consumes no input, so an EOF-edge cycle
/// would loop forever. Defensive only: the builder closes EOF-edge
/// targets with the lazy `treat_eofas_epsilon` semantics, so a chained
/// EOF edge always lands on an accept state and the walk breaks there
/// before the bound.
const MAX_STATIC_EOF_EDGES: u32 = 8;

/// The best accept seen so far in a token match, with the action source
/// resolved. The lazy path resolves a DFAState into this form at capture
/// time; the table walker carries it directly, so table accepts and lazy
/// accepts share `fail_or_accept`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolvedAccept<'sim> {
    index: isize,
    prediction: i32,
    executor: Option<AcceptExecutor<'sim>>,
}

/// The lexer actions of a resolved accept.
#[derive(Debug, Clone, Copy)]
pub(crate) enum AcceptExecutor<'sim> {
    /// The executor of a lazy DFA accept state.
    Lazy(&'sim LexerActionExecutor<'sim>),
    /// The position-independent action list of a static table accept state.
    Static(&'static [crate::lexer_action::LexerAction<'static>]),
}

/// Consume a maximal run of self-looping bytes of an accelerated state,
/// starting at byte index `c` in `hay`, and return the index of the first
/// byte that does not loop.
#[inline(never)]
fn run_scan(hay: &[u8], mut c: usize, accel: Accel) -> usize {
    match accel {
        Accel::Positive(bitmap) => {
            while c < hay.len() {
                let b = hay[c];
                if b < 0x80 && (bitmap >> b) & 1 != 0 {
                    c += 1;
                } else {
                    break;
                }
            }
        }
        Accel::Exclusion {
            needles,
            n,
            nl_in_set,
        } => {
            loop {
                let rest = &hay[c..];
                let found = match n {
                    1 => memchr::memchr(needles[0], rest),
                    2 => memchr::memchr2(needles[0], needles[1], rest),
                    _ => memchr::memchr3(needles[0], needles[1], needles[2], rest),
                };
                match found {
                    None => return hay.len(),
                    Some(i) => {
                        c += i;
                        if nl_in_set && hay[c] == b'\n' {
                            // '\n' loops here: consume it and resume.
                            c += 1;
                        } else {
                            return c;
                        }
                    }
                }
            }
        }
    }
    c
}

/// Environment that the lexer move/closure core reads runtime input facts
/// from: the absolute input index (used to fix position-dependent lexer
/// action offsets at move time) and the predicate evaluation.
///
/// The lazy simulator drives the core through [`LazyClosureEnv`], which
/// evaluates predicates against the live input. The static lexer DFA builder
/// (feature `static-dfa-build`) drives the same core headless: it reports a
/// fixed input index and evaluates every predicate as `true`, so the builder
/// reproduces the lazy config sets exactly when no predicate is involved.
pub(crate) trait LexerClosureEnv {
    /// The absolute input index at the move step.
    fn input_index(&mut self) -> isize;

    /// Evaluate one lexer predicate.
    fn eval_predicate(&mut self, rule_index: i32, pred_index: i32, speculative: bool) -> bool;
}

/// The [`LexerClosureEnv`] of the lazy simulator: delegates to the live lexer.
struct LazyClosureEnv<'a, 'sim, L, Input, TF> {
    sim: &'a LexerATNSimulator<'sim>,
    lexer: &'a mut L,
    _marker: std::marker::PhantomData<fn() -> (Input, TF)>,
}

impl<'a, 'sim, L, Input, TF> LazyClosureEnv<'a, 'sim, L, Input, TF> {
    fn new(sim: &'a LexerATNSimulator<'sim>, lexer: &'a mut L) -> Self {
        LazyClosureEnv {
            sim,
            lexer,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'input, 'arena, Input, TF, L> LexerClosureEnv for LazyClosureEnv<'_, '_, L, Input, TF>
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
    L: Lexer<'input, 'arena, Input, TF>,
{
    fn input_index(&mut self) -> isize {
        self.lexer.input().index()
    }

    fn eval_predicate(&mut self, rule_index: i32, pred_index: i32, speculative: bool) -> bool {
        self.sim
            .evaluate_predicate(rule_index, pred_index, speculative, &mut *self.lexer)
    }
}

/// Capture of one move step, requested by the static lexer DFA builder only.
/// The lazy path passes `None` and pays no cost.
#[derive(Debug, Default)]
pub(crate) struct ReachCapture<'scratch> {
    /// The post-move, pre-closure configs in move order. Each config carries
    /// the `current_alt_reached_accept_state` flag its closure ran with.
    pub(crate) moved: Vec<(LexerATNConfig<'scratch>, bool)>,
    /// True when the move step set `skip_alt` and broke out of the transition
    /// loop early. The build-time pruning decision then depends on the
    /// build-time closure results, so an escaping edge must not bake the
    /// moved configs into a continuation.
    pub(crate) pruning_activity: bool,
}

/// The start-state computation of [`LexerATNSimulator`], callable without a
/// live lexer through [`LexerClosureEnv`].
pub(crate) fn start_state_core<'scratch>(
    atn: &ATN,
    p: &ATNState,
    env: &mut impl LexerClosureEnv,
    scratch: &'scratch bumpalo::Bump,
) -> MutableLexerATNConfigSet<'scratch> {
    //        let initial_context = &EMPTY_PREDICTION_CONTEXT;
    let mut config_set = MutableLexerATNConfigSet::new(scratch);
    for (i, tr) in p.get_transitions().iter().enumerate() {
        let target = tr.get_target();
        let atn_config =
            LexerATNConfig::new(target, (i + 1) as i32, PredictionContextRef::new_empty());
        closure_core(
            atn,
            atn_config,
            &mut config_set,
            false,
            false,
            false,
            env,
            scratch,
        );
    }

    config_set
}

/// The move+closure step of [`LexerATNSimulator::get_reachable_config_set`],
/// callable without a live lexer through [`LexerClosureEnv`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn reachable_core<'scratch, 'a>(
    atn: &ATN,
    start_index: isize,
    source: impl Iterator<Item = &'a LexerATNConfig<'scratch>>,
    reach: &mut MutableLexerATNConfigSet<'scratch>,
    t: i32,
    env: &mut impl LexerClosureEnv,
    mut capture: Option<&mut ReachCapture<'scratch>>,
    scratch: &'scratch bumpalo::Bump,
) where
    'scratch: 'a,
{
    let mut skip_alt = 0;
    //        println!(" --- source {:?}", _closure.configs.iter().map(|it|it.get_state()).collect::<Vec<_>>());
    for config in source {
        let current_alt_reached_accept_state = config.get_alt() == skip_alt;
        if current_alt_reached_accept_state && config.has_passed_through_non_greedy_decision() {
            continue;
        }
        let atn_state = config.get_state();
        for tr in atn_state.get_transitions() {
            if let Some(target) = tr.get_reachable_target(t) {
                let exec = config.get_lexer_executor().map(|x| {
                    scratch
                        .alloc(x.fix_offset_before_match(scratch, env.input_index() - start_index))
                        as &'scratch LexerActionExecutor<'scratch>
                });

                let new = config.clone().with_state(target).with_lexer_executor(exec);
                if let Some(capture) = capture.as_deref_mut() {
                    capture
                        .moved
                        .push((new.clone(), current_alt_reached_accept_state));
                }
                if closure_core(
                    atn,
                    new,
                    reach,
                    current_alt_reached_accept_state,
                    true,
                    t == EOF,
                    env,
                    scratch,
                ) {
                    skip_alt = config.get_alt();
                    if let Some(capture) = capture.as_deref_mut() {
                        capture.pruning_activity = true;
                    }
                    break;
                }
            }
        }
    }
}

/// The closure computation of [`LexerATNSimulator`], callable without a live
/// lexer through [`LexerClosureEnv`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn closure_core<'scratch>(
    atn: &ATN,
    mut config: LexerATNConfig<'scratch>,
    config_set: &mut MutableLexerATNConfigSet<'scratch>,
    mut _current_alt_reached_accept_state: bool,
    speculative: bool,
    treat_eofas_epsilon: bool,
    env: &mut impl LexerClosureEnv,
    scratch: &'scratch bumpalo::Bump,
) -> bool {
    let state = config.get_state();
    //        println!("closure called on state {} {:?}", state.get_state_number(), state.get_state_type());

    if state.state_type() == ATNStateType::RuleStop {
        //            println!("reached rulestopstate {}",state.get_state_number());
        if config.get_context().map(|x| x.has_empty_path()) != Some(false) {
            if config.get_context().map(|x| x.is_empty()) != Some(false) {
                config_set.add(config);
                return true;
            } else {
                config_set.add(
                    config
                        .clone()
                        .with_state(state)
                        .with_prediction_context(Some(PredictionContextRef::new_empty())),
                );
                _current_alt_reached_accept_state = true
            }
        }

        if config.get_context().map(|x| x.is_empty()) == Some(false) {
            let ctx = config.take_context();
            for i in 0..ctx.length() {
                if ctx.get_return_state(i) != ATNStateRef::invalid() {
                    let new_ctx = ctx.get_parent(i);
                    let return_state = ctx.get_return_state(i);
                    let next_config = config
                        .clone()
                        .with_state(return_state)
                        .with_prediction_context(new_ctx);
                    _current_alt_reached_accept_state = closure_core(
                        atn,
                        next_config,
                        config_set,
                        _current_alt_reached_accept_state,
                        speculative,
                        treat_eofas_epsilon,
                        env,
                        scratch,
                    )
                }
            }
        }

        return _current_alt_reached_accept_state;
    }

    if !state.has_epsilon_only_transitions()
        && (!_current_alt_reached_accept_state || !config.has_passed_through_non_greedy_decision())
    {
        config_set.add(config.clone());
    }

    let state = config.get_state();
    let config = scratch.alloc(config);

    for tr in state.get_transitions() {
        let c = epsilon_target_core(
            atn,
            config,
            tr,
            config_set,
            speculative,
            treat_eofas_epsilon,
            env,
            scratch,
        );

        if let Some(c) = c {
            _current_alt_reached_accept_state = closure_core(
                atn,
                c,
                config_set,
                _current_alt_reached_accept_state,
                speculative,
                treat_eofas_epsilon,
                env,
                scratch,
            );
        }
    }

    _current_alt_reached_accept_state
}

/// One epsilon step of [`closure_core`].
#[allow(clippy::too_many_arguments)]
fn epsilon_target_core<'scratch>(
    atn: &ATN,
    //        _input: &mut dyn CharStream,
    _config: &'scratch LexerATNConfig<'scratch>,
    _trans: &Transition,
    _configs: &mut MutableLexerATNConfigSet<'scratch>,
    _speculative: bool,
    _treat_eofas_epsilon: bool,
    env: &mut impl LexerClosureEnv,
    scratch: &'scratch bumpalo::Bump,
) -> Option<LexerATNConfig<'scratch>> {
    let mut result = None;
    let target = _trans.get_target();
    //        println!("epsilon target for {:?} is {:?}", _trans, target.get_state_type());
    match _trans.transition_type() {
        TransitionType::Epsilon => {
            result = Some(_config.clone().with_state(target));
        }
        TransitionType::Rule => {
            let rt = _trans
                .try_as::<crate::transition::RuleTransition>()
                .unwrap();
            //println!("rule transition follow state{}", rt.follow_state);
            let pred_ctx = PredictionContext::new_singleton(
                Some(_config.get_context().unwrap()),
                rt.follow_state,
            );
            result = Some(
                _config
                    .clone()
                    .with_state(target)
                    .with_prediction_context(Some(scratch.alloc(pred_ctx))),
            );
        }
        TransitionType::Predicate => {
            let tr = _trans
                .try_as::<crate::transition::PredicateTransition>()
                .unwrap();
            _configs.set_has_semantic_context(true);
            if env.eval_predicate(tr.rule_index(), tr.pred_index(), _speculative) {
                result = Some(_config.clone().with_state(target));
            }
        }
        TransitionType::Action => {
            //println!("action transition");
            if _config.get_context().map(|x| x.has_empty_path()) != Some(false) {
                let tr = _trans
                    .try_as::<crate::transition::ActionTransition>()
                    .unwrap();
                let lexer_action = atn.lexer_actions[tr.action_index() as usize].clone();
                //dbg!(&lexer_action);
                let lexer_action_executor = LexerActionExecutor::new_copy_append(
                    scratch,
                    _config.get_lexer_executor(),
                    lexer_action,
                );
                result = Some(
                    _config
                        .clone()
                        .with_state(target)
                        .with_lexer_executor(Some(scratch.alloc(lexer_action_executor))),
                )
            } else {
                result = Some(_config.clone().with_state(target));
            }
        }
        TransitionType::Range | TransitionType::Set | TransitionType::Atom => {
            if _treat_eofas_epsilon
                && _trans.matches(EOF, LEXER_MIN_CHAR_VALUE, LEXER_MAX_CHAR_VALUE)
            {
                let target = _trans.get_target();
                result = Some(_config.clone().with_state(target));
            }
        }
        TransitionType::Wildcard => {}
        TransitionType::NotSet => {}
        TransitionType::PrecedencePredicate => {
            panic!("precedence predicates are not supposed to be in lexer");
        }
    }

    result
}
