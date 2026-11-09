//! Implementation of lexer automata(DFA)
use std::cell::Cell;

use crate::atn::ATN;
use crate::atn_config::LexerATNConfig;
use crate::atn_config_set::{LexerATNConfigSet, MutableLexerATNConfigSet};
use crate::atn_simulator::{BaseATNSimulator, IATNSimulator};
use crate::atn_state::{ATNState, ATNStateRef, ATNStateType};
use crate::char_stream::CharStream;
use crate::dfa::static_lexer_dfa::{
    Accel, LexerContinuation, StaticLexerDFA, StaticLexerTables, ACCEPT_BIT, ASCII_ROW,
    CONTINUATION_MOVED_CONFIGS, CONTINUATION_SOURCE_SET, ESCAPE, NO_CONTINUATION, NO_EDGE,
    STATE_MASK, WALKER_ACTION_NONE, WALKER_ACTION_SKIP, WALKER_LEAF_TAG, WALKER_NONTERM_POS_BIT,
    WALKER_TERMINAL_POS_BIT,
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
    fn reset(&self);

    fn match_token<'input, 'arena, Input, TF>(
        &self,
        mode: usize,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena;

    fn consume<T: IntStream + ?Sized>(&self, input: &mut T);

    #[cold]
    fn recover(&self, _re: ANTLRError, input: &mut impl IntStream) {
        if input.la(1) != EOF {
            self.consume(input)
        }
    }
}

/// Simple DFA implementation enough for lexer.
#[derive(Debug)]
pub struct LexerATNSimulator<'sim> {
    base: BaseATNSimulator<'sim, LexerATNConfigSet<'sim>>,

    start_index: Cell<isize>,
    mode: Cell<usize>,
    prev_accept: Cell<Option<ResolvedAccept<'sim>>>,
    static_modes: &'static [Option<Box<StaticLexerDFA>>],
    /// Number of escape edges the table walker handed to the lazy path.
    #[doc(hidden)]
    pub escape_count: Cell<usize>,
    /// Number of tokens the table walker restarted on the lazy path.
    #[doc(hidden)]
    pub restart_count: Cell<usize>,
}

impl<'sim> ILexerATNSimulator<'sim> for LexerATNSimulator<'sim> {
    fn reset(&self) {
        self.prev_accept.set(None);
    }

    fn match_token<'input, 'arena, Input, TF>(
        &self,
        mode: usize,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        if let Some(Some(table)) = self.static_modes.get(mode).map(Option::as_deref) {
            if lexer.input().has_item_at() {
                if let Some(hay) = lexer.input().bytes_at(0) {
                    return self.match_static_random_access(table, hay, lexer);
                }
            }
            let mark = lexer.input().mark();
            let start_index = lexer.input().index();
            let result = self.match_static_cursor(table, mode, start_index, lexer);
            lexer.input().release(mark);
            return result.map(|t| t as i32);
        }
        let mark = lexer.input().mark();
        let start_index = lexer.input().index();
        let result = self.match_token_atn_entry(mode, start_index, lexer);
        lexer.input().release(mark);
        result.map(|t| t as i32)
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
            static_modes: base
                .static_lexer_tables
                .map_or(&[], StaticLexerTables::mode_slice),
            base,
            start_index: Cell::new(0),
            mode: Cell::new(0),
            prev_accept: Cell::new(None),
            escape_count: Cell::new(0),
            restart_count: Cell::new(0),
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
        self.static_modes = tables.mode_slice();
    }

    #[inline(never)]
    fn match_token_atn_entry<'input, 'arena, Input, TF>(
        &self,
        mode: usize,
        start_index: isize,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<isize, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let scratch = bumpalo::Bump::new();
        self.mode.set(mode);
        self.start_index.set(start_index);
        self.prev_accept.set(None);
        self.match_token_atn(lexer, &scratch).map(|t| t as isize)
    }

    fn match_token_atn<'scratch, 'input, 'arena, Input, TF>(
        &self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let dfa = self
            .decision_to_dfa(self.mode.get())
            .ok_or_else(|| ANTLRError::illegal_state("invalid mode".into()))?;

        match dfa.s0() {
            None => self.match_atn(lexer, dfa, scratch),
            Some(s0) => self.exec_atn(s0, lexer, dfa, scratch),
        }
    }

    /// Token matching with a precomputed [`StaticLexerDFA`] over a
    /// random-access input stream.
    #[inline(always)]
    fn match_static_random_access<'input, 'arena, Input, TF>(
        &self,
        table: &'static StaticLexerDFA,
        hay: &'input [u8],
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let start_index = lexer.input().index();
        let start_entry = table.start_entry;
        let mut s = start_entry & STATE_MASK;
        let mut accept_index: isize = if start_entry & ACCEPT_BIT != 0 {
            start_index
        } else {
            -1
        };
        let mut accept_state: u16 = s;

        let low = table.low.as_ptr();
        let walker_tag = table.walker_tag.as_ptr();

        let mut cursor = start_index as usize;
        let mut pending_wtag: u16 = 0;
        'accept: {
            loop {
                let target: u16;
                if cursor < hay.len() {
                    let b = unsafe { *hay.get_unchecked(cursor) };
                    if b >= 0x80 {
                        break;
                    }
                    let entry = unsafe { *low.add(s as usize * ASCII_ROW + b as usize) };
                    if entry >= ESCAPE {
                        if entry == NO_EDGE && accept_index >= 0 {
                            break 'accept;
                        }
                        break;
                    }
                    target = entry & STATE_MASK;
                    cursor += 1;
                    if entry & ACCEPT_BIT != 0 {
                        accept_index = cursor as isize;
                        accept_state = target;
                    }
                } else {
                    let entry = table.eof_entry(s);
                    if entry >= ESCAPE {
                        if entry == NO_EDGE && accept_index >= 0 {
                            break 'accept;
                        }
                        break;
                    }
                    target = entry & STATE_MASK;
                    if entry & ACCEPT_BIT != 0 {
                        accept_index = cursor as isize;
                        accept_state = target;
                        break 'accept;
                    }
                }
                s = target;

                let wtag = unsafe { *walker_tag.add(s as usize) };
                if wtag != 0 {
                    if wtag == WALKER_LEAF_TAG {
                        break 'accept;
                    }
                    if wtag & WALKER_TERMINAL_POS_BIT != 0 {
                        let tp = table.walker_positive_at(wtag);
                        let c = scan_positive(hay, cursor, tp.lo, tp.hi);
                        if c > cursor {
                            accept_index = c as isize;
                            accept_state = tp.target;
                        }
                        break 'accept;
                    }
                    if wtag >= WALKER_NONTERM_POS_BIT {
                        let tp = table.walker_positive_at(wtag);
                        let c = scan_positive(hay, cursor, tp.lo, tp.hi);
                        if c > cursor {
                            if accept_index == cursor as isize {
                                accept_index = c as isize;
                            }
                            cursor = c;
                        }
                    } else {
                        pending_wtag = wtag;
                        break;
                    }
                }
            }

            return self
                .match_static_slow(
                    table,
                    cursor,
                    s,
                    accept_index,
                    accept_state,
                    pending_wtag,
                    lexer,
                )
                .map(|t| t as i32);
        }

        lexer.input().seek(accept_index);
        let acc = table.walker_accept_at(accept_state);
        if acc.action == WALKER_ACTION_NONE {
            return Ok(acc.ttype);
        }
        if acc.action == WALKER_ACTION_SKIP {
            lexer.skip();
            return Ok(acc.ttype);
        }
        Ok(execute_accept_actions(table, accept_state, lexer))
    }

    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    fn match_static_slow<'input, 'arena, Input, TF>(
        &self,
        table: &'static StaticLexerDFA,
        mut cursor: usize,
        mut s: u16,
        mut accept_index: isize,
        mut accept_state: u16,
        pending_wtag: u16,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<isize, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let start_index = lexer.input().index();
        let hay = lexer.input().bytes_at(0).unwrap_or(&[]);
        if pending_wtag != 0 {
            let accel = table.accel_payload(pending_wtag);
            let c = run_scan(hay, cursor, accel);
            if c > cursor {
                if accept_index == cursor as isize
                    && !matches!(accel, Accel::CountedExclusion { .. })
                {
                    accept_index = c as isize;
                }
                cursor = c;
            }
        }

        let low = table.low.as_slice();
        let walker_tag = table.walker_tag.as_slice();
        let mut symbol: i32;
        let mut entry: u16;
        'accept: {
            loop {
                let target: u16;
                if cursor < hay.len() {
                    let b = unsafe { *hay.get_unchecked(cursor) };
                    if b < 0x80 {
                        symbol = b as i32;
                        entry = unsafe { *low.get_unchecked(s as usize * ASCII_ROW + b as usize) };
                        if entry >= ESCAPE {
                            break;
                        }
                        target = entry & STATE_MASK;
                        cursor += 1;
                        if entry & ACCEPT_BIT != 0 {
                            accept_index = cursor as isize;
                            accept_state = target;
                        }
                    } else {
                        symbol = item_at_non_ascii(lexer.input(), cursor);
                        entry = table.edge_entry(s, symbol);
                        if entry >= ESCAPE {
                            break;
                        }
                        target = entry & STATE_MASK;
                        if symbol != EOF {
                            cursor += char::from_u32(symbol as u32).map_or(1, |c| c.len_utf8());
                        }
                        if entry & ACCEPT_BIT != 0 {
                            accept_index = cursor as isize;
                            accept_state = target;
                            if symbol == EOF {
                                break 'accept;
                            }
                        }
                    }
                } else {
                    symbol = EOF;
                    entry = table.eof_entry(s);
                    if entry >= ESCAPE {
                        break;
                    }
                    target = entry & STATE_MASK;
                    if entry & ACCEPT_BIT != 0 {
                        accept_index = cursor as isize;
                        accept_state = target;
                        break 'accept;
                    }
                }
                s = target;

                let wtag = unsafe { *walker_tag.get_unchecked(s as usize) };
                if wtag != 0 {
                    if wtag == WALKER_LEAF_TAG {
                        break 'accept;
                    }
                    if wtag & WALKER_TERMINAL_POS_BIT != 0 {
                        let tp = table.walker_positive_at(wtag);
                        let c = scan_positive(hay, cursor, tp.lo, tp.hi);
                        if c > cursor {
                            accept_index = c as isize;
                            accept_state = tp.target;
                        }
                        break 'accept;
                    }
                    if wtag >= WALKER_NONTERM_POS_BIT {
                        let tp = table.walker_positive_at(wtag);
                        let c = scan_positive(hay, cursor, tp.lo, tp.hi);
                        if c > cursor {
                            if accept_index == cursor as isize {
                                accept_index = c as isize;
                            }
                            cursor = c;
                        }
                    } else {
                        let accel = table.accel_payload(wtag);
                        let c = run_scan(hay, cursor, accel);
                        if c > cursor {
                            if accept_index == cursor as isize
                                && !matches!(accel, Accel::CountedExclusion { .. })
                            {
                                accept_index = c as isize;
                            }
                            cursor = c;
                        }
                    }
                }
            }

            if entry == ESCAPE {
                return self.static_escape(
                    table,
                    table.mode as usize,
                    start_index,
                    s,
                    symbol,
                    cursor as isize,
                    if accept_index >= 0 {
                        Some((accept_index, accept_state))
                    } else {
                        None
                    },
                    lexer,
                );
            }
            if accept_index < 0 {
                if symbol == EOF && cursor == start_index as usize {
                    return Ok(TOKEN_EOF as isize);
                }
                lexer.input().seek(cursor as isize);
                return Err(ANTLRError::lexer_no_alt(start_index));
            }
        }

        lexer.input().seek(accept_index);
        let acc = table.walker_accept_at(accept_state);
        if acc.action == WALKER_ACTION_NONE {
            return Ok(acc.ttype as isize);
        }
        if acc.action == WALKER_ACTION_SKIP {
            lexer.skip();
            return Ok(acc.ttype as isize);
        }
        Ok(execute_accept_actions(table, accept_state, lexer) as isize)
    }

    /// Token matching with a precomputed [`StaticLexerDFA`] over a cursor-only
    /// input stream.
    #[inline(never)]
    fn match_static_cursor<'input, 'arena, Input, TF>(
        &self,
        table: &'static StaticLexerDFA,
        mode: usize,
        start_index: isize,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<isize, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let mut s = table.start_state();
        let mut prev_accept: Option<(isize, u16)> = if table.accept_type(s) >= 0 {
            Some((start_index, s))
        } else {
            None
        };

        let mut symbol = lexer.input().la(1);
        loop {
            let entry = table.edge_entry(s, symbol);
            if entry == NO_EDGE {
                break;
            }
            if entry == ESCAPE {
                return self.static_escape(
                    table,
                    mode,
                    start_index,
                    s,
                    symbol,
                    lexer.input().index(),
                    prev_accept,
                    lexer,
                );
            }
            let target = entry & STATE_MASK;
            if symbol != EOF {
                lexer.input().consume();
            }
            if entry & ACCEPT_BIT != 0 {
                prev_accept = Some((lexer.input().index(), target));
                if symbol == EOF {
                    break;
                }
            }
            symbol = lexer.input().la(1);
            s = target;
            let tag = table.accel_tag(s);
            if tag != 0 {
                if let &Accel::CountedExclusion { opener, closer, n } = table.accel_payload(tag) {
                    let opener = [opener[0] as u8, opener[1] as u8, opener[2] as u8];
                    let closer = [closer[0] as u8, closer[1] as u8, closer[2] as u8];
                    let input = lexer.input();
                    let start = input.index() as usize;
                    let c = {
                        let text = input.get_text(0, input.size());
                        run_scan_counted(
                            text.as_bytes(),
                            start,
                            &opener[..n as usize],
                            &closer[..n as usize],
                        )
                    };
                    if c > start {
                        input.seek(c as isize);
                        symbol = input.la(1);
                    }
                }
            }
        }
        self.finish_static(table, start_index, prev_accept, symbol, lexer)
    }

    #[inline]
    fn finish_static<'input, 'arena, Input, TF>(
        &self,
        table: &'static StaticLexerDFA,
        start_index: isize,
        prev_accept: Option<(isize, u16)>,
        symbol: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<isize, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        match prev_accept {
            Some((index, state)) => {
                lexer.input().seek(index);
                for action in table.accept_actions(state) {
                    action.execute(lexer);
                }
                Ok(table.accept_type(state) as isize)
            }
            None => {
                if symbol == EOF && lexer.input().index() == start_index {
                    return Ok(TOKEN_EOF as isize);
                }
                Err(ANTLRError::lexer_no_alt(start_index))
            }
        }
    }

    /// Handles an ESCAPE edge hit by
    /// [`LexerATNSimulator::match_static_random_access`] or
    /// [`LexerATNSimulator::match_static_cursor`]: positions the input at the
    /// unconsumed escape symbol, carries the best accept so far into
    /// `self.prev_accept`, and resumes the lazy simulation from the edge's
    /// continuation. A budget escape has no continuation and restarts the token
    /// on the lazy path from the token start.
    #[cold]
    #[inline(never)]
    #[allow(clippy::too_many_arguments)]
    fn static_escape<'input, 'arena, Input, TF>(
        &self,
        table: &'static StaticLexerDFA,
        mode: usize,
        start_index: isize,
        state: u16,
        symbol: i32,
        cursor: isize,
        prev_accept: Option<(isize, u16)>,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<isize, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let scratch = bumpalo::Bump::new();
        self.escape_count.set(self.escape_count.get() + 1);
        self.mode.set(mode);
        self.start_index.set(start_index);
        lexer.input().seek(cursor);
        self.prev_accept.set(prev_accept.map(|(index, state)| {
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
        }));
        let continuation = table.escape_continuation(state, symbol);
        if continuation == NO_CONTINUATION {
            return self.restart_token_lazy(lexer, &scratch).map(|t| t as isize);
        }
        self.match_token_from_continuation(
            table.continuation(continuation),
            symbol,
            lexer,
            &scratch,
        )
        .map(|t| t as isize)
    }

    /// Restarts the current token on the lazy path from the token start.
    /// Used for budget escapes (no continuation).
    #[cold]
    fn restart_token_lazy<'scratch, 'input, 'arena, Input, TF>(
        &self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        self.restart_count.set(self.restart_count.get() + 1);
        lexer.input().seek(self.start_index.get());
        self.prev_accept.set(None);
        self.match_token_atn(lexer, scratch)
    }

    fn match_token_from_continuation<'scratch, 'input, 'arena, Input, TF>(
        &self,
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
        let start_index = self.start_index.get();
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
                        atn.lexer_actions[action_index as usize];
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
        &self,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
        dfa: &'sim DFA<LexerATNConfigSet<'sim>>,
        scratch: &'scratch bumpalo::Bump,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let atn = self.atn();
        let start_state = *atn
            .mode_to_start_state
            .get(self.mode.get())
            .ok_or_else(|| ANTLRError::illegal_state("invalid mode".into()))?;

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
        &self,
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
        self.capture_sim_state(lexer.input(), ds0);

        let symbol = lexer.input().la(1);
        self.exec_atn_loop(ds0, symbol, lexer, dfa, scratch)
    }

    /// The inner loop of `exec_atn`: from state `s` with `symbol` the
    /// (unconsumed) lookahead. Also the tail of the continuation resume
    /// path, which has already consumed the escape symbol.
    fn exec_atn_loop<'scratch, 'input, 'arena, Input, TF>(
        &self,
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
            self.start_index.get(),
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
        &self,
        _t: i32,
        lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
    ) -> Result<i32, ANTLRError>
    where
        'input: 'arena,
        Input: CharStream<'input>,
        TF: TokenFactory<'input, 'arena> + 'arena,
    {
        let start_index = self.start_index.get();
        if let Some(accept) = self.prev_accept.take() {
            lexer.input().seek(accept.index);

            match accept.executor {
                Some(AcceptExecutor::Lazy(x)) => x.execute(lexer, start_index),
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
            if _t == EOF && lexer.input().index() == start_index {
                return Ok(TOKEN_EOF);
            }
            Err(ANTLRError::lexer_no_alt(start_index))
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
        &self,
        input: &impl IntStream,
        dfa_state: &'sim DFAState<'sim, LexerATNConfigSet<'sim>>,
    ) -> bool {
        if dfa_state.is_accept_state() {
            // Resolve the DFAState into a ResolvedAccept here, so the
            // accept slot has one form for lazy states and for the table
            // accepts the static walker carries.
            self.prev_accept.set(Some(ResolvedAccept {
                index: input.index(),
                prediction: dfa_state.prediction(),
                executor: dfa_state.lexer_action_executor().map(AcceptExecutor::Lazy),
            }));
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
        self.decision_to_dfa(self.mode.get())
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
    Static(&'static [crate::lexer_action::LexerAction]),
}

/// Consume a maximal run of self-looping bytes of an accelerated state,
/// starting at byte index `c` in `hay`, and return the index of the first
/// byte that does not loop.
#[inline(always)]
fn scan_positive(hay: &[u8], mut c: usize, lo: u64, hi: u64) -> usize {
    while c < hay.len() {
        let b = hay[c];
        if b >= 0x80 {
            break;
        }
        let word = if b < 64 { lo } else { hi };
        if (word >> (b & 63)) & 1 != 0 {
            c += 1;
        } else {
            break;
        }
    }
    c
}

#[cold]
#[inline(never)]
fn execute_accept_actions<'input, 'arena, Input, TF>(
    table: &'static StaticLexerDFA,
    accept_state: u16,
    lexer: &mut impl Lexer<'input, 'arena, Input, TF>,
) -> i32
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    for action in table.accept_actions(accept_state) {
        action.execute(lexer);
    }
    table.accept_type(accept_state)
}

#[cold]
#[inline(never)]
fn item_at_non_ascii<'input, Input: CharStream<'input>>(input: &Input, cursor: usize) -> i32 {
    input.item_at(cursor as isize).unwrap_or(EOF)
}

#[inline(never)]
fn run_scan(hay: &[u8], mut c: usize, accel: &Accel) -> usize {
    match *accel {
        Accel::Positive([w0, w1, w2, w3]) => {
            let lo = (w0 as u64) | ((w1 as u64) << 32);
            let hi = (w2 as u64) | ((w3 as u64) << 32);
            c = scan_positive(hay, c, lo, hi);
        }
        Accel::Exclude(needle) => match memchr::memchr(needle as u8, &hay[c..]) {
            None => c = hay.len(),
            Some(i) => c += i,
        },
        Accel::Exclude2(needle1, needle2) => {
            match memchr::memchr2(needle1 as u8, needle2 as u8, &hay[c..]) {
                None => c = hay.len(),
                Some(i) => c += i,
            }
        }
        Accel::Exclude3(needle1, needle2, needle3) => {
            match memchr::memchr3(needle1 as u8, needle2 as u8, needle3 as u8, &hay[c..]) {
                None => c = hay.len(),
                Some(i) => c += i,
            }
        }
        Accel::CountedExclusion {
            ref opener,
            ref closer,
            n,
        } => {
            c = run_scan_counted_u32(hay, c, opener, closer, n);
        }
    }
    c
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CountedEntry {
    Body(u32),
    Close(u8, u32),
    Open(u8, u32),
}

fn push_unique(vec: &mut Vec<CountedEntry>, entry: CountedEntry) {
    if !vec.contains(&entry) {
        vec.push(entry);
    }
}

#[cold]
#[inline(never)]
fn run_scan_counted_u32(
    hay: &[u8],
    c: usize,
    opener: &[u32; 3],
    closer: &[u32; 3],
    n: u32,
) -> usize {
    let opener = [opener[0] as u8, opener[1] as u8, opener[2] as u8];
    let closer = [closer[0] as u8, closer[1] as u8, closer[2] as u8];
    run_scan_counted(hay, c, &opener[..n as usize], &closer[..n as usize])
}

#[cold]
#[inline(never)]
fn run_scan_counted(hay: &[u8], mut c: usize, opener: &[u8], closer: &[u8]) -> usize {
    let start = c;
    let n = opener.len();
    let o0 = opener[0];
    let c0 = closer[0];

    while c + n <= hay.len() {
        let Some(i) = memchr::memchr2(o0, c0, &hay[c..]) else {
            return hay.len();
        };
        let pos = c + i;
        if pos + n > hay.len() {
            return hay.len();
        }
        let slice = &hay[pos..pos + n];
        if slice == closer {
            return pos;
        }
        if slice == opener {
            c = pos.saturating_sub(n - 1).max(start);
            break;
        }
        c = pos + 1;
    }
    if c + n > hay.len() {
        return hay.len();
    }

    let mut state = vec![CountedEntry::Body(0)];
    let mut next = Vec::with_capacity(8);
    let mut last_closer = hay.len();

    while c < hay.len() {
        let has_partial = state.iter().any(|e| !matches!(e, CountedEntry::Body(_)));
        if !has_partial {
            let Some(i) = memchr::memchr2(o0, c0, &hay[c..]) else {
                break;
            };
            c += i;
        }
        let b = hay[c];
        next.clear();
        for &entry in &state {
            match entry {
                CountedEntry::Close(idx, d) => {
                    if b == closer[idx as usize] {
                        if idx as usize + 1 == n {
                            if d == 0 {
                                last_closer = c + 1 - n;
                                break;
                            }
                            push_unique(&mut next, CountedEntry::Body(d - 1));
                        } else {
                            push_unique(&mut next, CountedEntry::Close(idx + 1, d));
                        }
                    }
                }
                CountedEntry::Open(idx, d) => {
                    if b == opener[idx as usize] {
                        if idx as usize + 1 == n {
                            push_unique(&mut next, CountedEntry::Body(d));
                        } else {
                            push_unique(&mut next, CountedEntry::Open(idx + 1, d));
                        }
                    }
                }
                CountedEntry::Body(d) => {
                    if b == c0 {
                        if n == 1 {
                            if d == 0 {
                                last_closer = c;
                                break;
                            }
                            push_unique(&mut next, CountedEntry::Body(d - 1));
                        } else {
                            push_unique(&mut next, CountedEntry::Close(1, d));
                        }
                    }
                    if b == o0 {
                        if n == 1 {
                            push_unique(&mut next, CountedEntry::Body(d + 1));
                        } else {
                            push_unique(&mut next, CountedEntry::Open(1, d + 1));
                        }
                    }
                    push_unique(&mut next, CountedEntry::Body(d));
                }
            }
        }
        std::mem::swap(&mut state, &mut next);
        if state.is_empty() {
            return last_closer;
        }
        c += 1;
    }
    last_closer
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
                if ctx.get_return_state(i) != ATNStateRef::empty_return() {
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
                let lexer_action = atn.lexer_actions[tr.action_index() as usize];
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
