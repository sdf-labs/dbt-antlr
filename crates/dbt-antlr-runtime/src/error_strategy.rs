//! Error handling and recovery
use std::borrow::{Borrow, Cow};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::sync::Arc;

use crate::atn::ATN;
use crate::atn_simulator::IATNSimulator;
use crate::atn_state::*;
use crate::char_stream::InputData;
use crate::dfa::ScopeExt;
use crate::errors::{
    ANTLRError, ANTLRErrorKind, FailedPredicateError, InputMisMatchError, NoViableAltError,
};
use crate::interval_set::{IntervalSet, IntervalSetBuf};
use crate::parser::Parser;
use crate::rule_context::RuleContext as _;
use crate::token::{
    Token, TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, TOKEN_EPSILON, TOKEN_INVALID_TYPE,
    TOKEN_MIN_USER_TOKEN_TYPE,
};
use crate::token_factory::TokenFactory;
use crate::transition::{RuleTransition, TransitionType};
use crate::tree::{NodeKindType, Tree as _, TreeNode};
use crate::utils::escape_whitespaces;
use hashbrown::HashSet;

/// The interface for defining strategies to deal with syntax errors encountered
/// during a parse by ANTLR-generated parsers. We distinguish between three
/// different kinds of errors:
///  - The parser could not figure out which path to take in the ATN (none of
///    the available alternatives could possibly match)
///  - The current input does not match what we were looking for
///  - A predicate evaluated to false
///
/// Implementations of this interface should report syntax errors by calling [`Parser::notifyErrorListeners`]
///
/// [`Parser::notifyErrorListeners`]: crate::parser::Parser::notifyErrorListeners
pub trait ErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    ///Reset the error handler state for the specified `recognizer`.
    fn reset(&mut self, recognizer: &mut P);

    /// This method is called when an unexpected symbol is encountered during an
    /// inline match operation, such as `Parser::match`. If the error
    /// strategy successfully recovers from the match failure, this method
    /// returns the `Token` instance which should be treated as the
    /// successful result of the match.
    ///
    /// This method handles the consumption of any tokens - the caller should
    /// **not** call `Parser::consume` after a successful recovery.
    ///
    /// Note that the calling code will not report an error if this method
    /// returns successfully. The error strategy implementation is responsible
    /// for calling `Parser::notifyErrorListeners` as appropriate.
    ///
    /// Returns `ANTLRError` if can't recover from unexpected input symbol
    fn recover_inline(&mut self, recognizer: &mut P) -> Result<&'arena TF::Tok, ANTLRError>;

    /// This method is called to recover from error `e`. This method is
    /// called after `ErrorStrategy::reportError` by the default error handler
    /// generated for a rule method.
    ///
    ///
    fn recover(&mut self, recognizer: &mut P, e: &ANTLRError) -> Result<(), ANTLRError>;

    /// This method provides the error handler with an opportunity to handle
    /// syntactic or semantic errors in the input stream before they result in a
    /// error.
    ///
    /// The generated code currently contains calls to `ErrorStrategy::sync` after
    /// entering the decision state of a closure block ({@code (...)*} or
    /// {@code (...)+}).</p>
    fn sync(&mut self, recognizer: &mut P) -> Result<(), ANTLRError>;

    /// Tests whether or not {@code recognizer} is in the process of recovering
    /// from an error. In error recovery mode, `Parser::consume` will create
    /// `ErrorNode` leaf instead of `TerminalNode` one  
    fn in_error_recovery_mode(&mut self, recognizer: &mut P) -> bool;

    /// Report any kind of `ANTLRError`. This method is called by
    /// the default exception handler generated for a rule method.
    fn report_error(&mut self, recognizer: &mut P, e: &ANTLRError);

    /// This method is called when the parser successfully matches an input
    /// symbol.
    fn report_match(&mut self, recognizer: &mut P);
}

/// This is the default implementation of `ErrorStrategy` used for
/// error reporting and recovery in ANTLR parsers.
#[derive(Debug)]
pub struct DefaultErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    error_recovery_mode: bool,
    last_error_index: isize,
    last_error_states: Option<IntervalSetBuf>,
    next_tokens_state: i32,
    next_tokens_ctx: Option<&'arena TreeNode<'input, 'arena, P::Node, TF::Tok>>,
    pd: PhantomData<(TF, P)>,
}

impl<'input, 'arena, TF, P> Default for DefaultErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'input, 'arena, TF, P> DefaultErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    /// Creates new instance of `DefaultErrorStrategy`
    pub fn new() -> Self {
        Self {
            error_recovery_mode: false,
            last_error_index: -1,
            last_error_states: None,
            next_tokens_state: ATNSTATE_INVALID_STATE_NUMBER,
            next_tokens_ctx: None,
            pd: PhantomData,
        }
    }

    fn begin_error_condition(&mut self, _recognizer: &P) {
        self.error_recovery_mode = true;
    }

    #[inline(always)]
    fn end_error_condition(&mut self, _recognizer: &P) {
        if self.error_recovery_mode {
            self.end_error_condition_cold();
        }
    }

    #[cold]
    #[inline(never)]
    fn end_error_condition_cold(&mut self) {
        self.error_recovery_mode = false;
        self.last_error_index = -1;
        self.last_error_states = None;
    }

    fn report_no_viable_alternative(&self, recognizer: &mut P, e: &NoViableAltError) -> String {
        let input = if e.start_token.get_token_type() == TOKEN_EOF {
            "<EOF>".to_owned()
        } else {
            recognizer.get_input_stream_mut().get_text_from_interval(
                e.start_token.get_token_index(),
                e.base.offending_token.get_token_index(),
            )
        };

        format!("no viable alternative at input '{}'", input)
    }

    fn report_input_mismatch(&self, recognizer: &P, e: &InputMisMatchError) -> String {
        format!(
            "mismatched input {} expecting {}",
            self.get_token_error_display(&e.base.offending_token),
            e.base
                .get_expected_tokens(recognizer)
                .to_token_string(recognizer.get_vocabulary())
        )
    }

    fn report_failed_predicate(&self, recognizer: &P, e: &FailedPredicateError) -> String {
        format!(
            "rule {} {}",
            recognizer.get_rule_names()[recognizer.get_current_context().unwrap().get_rule_index()],
            e.base.message
        )
    }

    fn report_unwanted_token(&mut self, recognizer: &mut P) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let expecting = self.get_expected_tokens(recognizer);
        let expecting = expecting.to_token_string(recognizer.get_vocabulary());
        let t = recognizer.get_current_token().borrow();
        let token_name = self.get_token_error_display(t);
        let msg = format!("extraneous input {} expecting {}", token_name, expecting);
        let t = t.get_token_index();
        recognizer.notify_error_listeners(msg, Some(t), None);
    }

    fn report_missing_token(&mut self, recognizer: &mut P) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let expecting = self.get_expected_tokens(recognizer);
        let expecting = expecting.to_token_string(recognizer.get_vocabulary());
        let t = recognizer.get_current_token().borrow();
        let _token_name = self.get_token_error_display(t);
        let msg = format!(
            "missing {} at {}",
            expecting,
            self.get_token_error_display(t)
        );
        let t = t.get_token_index();
        recognizer.notify_error_listeners(msg, Some(t), None);
    }

    fn single_token_insertion(&mut self, recognizer: &mut P) -> bool {
        let current_token = recognizer.get_input_stream_mut().la(1);

        let atn = recognizer.get_interpreter().atn();
        let current_state = atn.get_state(recognizer.get_state());
        let next = current_state
            .get_transitions()
            .first()
            .unwrap()
            .get_target();
        let expect_at_ll2 = atn.next_tokens_in_ctx(next, recognizer.get_current_context());
        if expect_at_ll2.contains(current_token) {
            self.report_missing_token(recognizer);
            return true;
        }
        false
    }

    fn single_token_deletion(
        &mut self,
        recognizer: &mut P,
    ) -> Result<Option<&'arena TF::Tok>, ANTLRError> {
        let next_token_type = recognizer.get_input_stream_mut().la(2);
        let expecting = self.get_expected_tokens(recognizer);
        //        println!("expecting {}", expecting.to_token_string(recognizer.get_vocabulary()));
        if expecting.contains(next_token_type) {
            self.report_unwanted_token(recognizer);
            recognizer.consume(self)?;
            self.report_match(recognizer);
            let matched_symbol = recognizer.get_current_token();
            return Ok(Some(matched_symbol));
        }
        Ok(None)
    }

    fn get_missing_symbol(&self, recognizer: &mut P) -> &'arena mut TF::Tok {
        let expected = self.get_expected_tokens(recognizer);
        let expected_token_type = expected.get_min().unwrap_or(TOKEN_INVALID_TYPE);
        let token_text = if expected_token_type == TOKEN_EOF {
            "<missing EOF>".to_owned()
        } else {
            format!(
                "<missing {}>",
                recognizer
                    .get_vocabulary()
                    .get_display_name(expected_token_type)
            )
        };

        let mut curr = recognizer.get_current_token().borrow();
        if curr.get_token_type() == TOKEN_EOF {
            curr = recognizer
                .get_input_stream()
                .run(|it| it.get((it.index() - 1).max(0)).borrow());
        }
        recognizer.get_token_factory().create_conjured(
            expected_token_type,
            token_text,
            TOKEN_DEFAULT_CHANNEL,
            curr,
        )
        // Token::to_owned(token.borrow())
        // .modify_with(|it| it.text = token_text)
    }

    fn get_expected_tokens<'r>(&self, recognizer: &'r P) -> Cow<'r, IntervalSet> {
        recognizer.get_expected_tokens()
    }

    fn get_token_error_display(&self, t: &dyn Token) -> String {
        let text = t.get_text().to_display();
        self.escape_ws_and_quote(&text)
    }

    fn escape_ws_and_quote(&self, s: &str) -> String {
        format!("'{}'", escape_whitespaces(s, false))
    }

    fn get_error_recovery_set(&self, recognizer: &P) -> IntervalSetBuf {
        let atn = recognizer.get_interpreter().atn();
        let mut ctx = recognizer.get_current_context();
        let mut recover_set = IntervalSetBuf::new();
        while let Some(c) = ctx {
            if c.get_invoking_state() < 0 {
                break;
            }

            let invoking_state = atn.get_state(c.get_invoking_state());
            let tr = invoking_state.get_transitions().first().unwrap();
            let tr = tr.try_as::<RuleTransition>().unwrap();
            let follow = atn.next_tokens::<TF::Tok>(&tr.follow_state);
            recover_set.add_set(follow);
            ctx = c.get_parent();
        }
        recover_set.remove_one(TOKEN_EPSILON);
        recover_set
    }

    fn consume_until(&mut self, recognizer: &mut P, set: &IntervalSet) -> Result<(), ANTLRError> {
        let mut ttype = recognizer.get_input_stream_mut().la(1);
        while ttype != TOKEN_EOF && !set.contains(ttype) {
            recognizer.consume(self)?;
            ttype = recognizer.get_input_stream_mut().la(1);
        }
        Ok(())
    }

    #[cold]
    #[inline(never)]
    fn reconstruct_skipped_expecting(
        &self,
        recognizer: &mut P,
    ) -> Option<SkippedBookmark<'input, 'arena, P::Node, TF::Tok>> {
        if self.error_recovery_mode {
            return None;
        }
        let current_ctx = recognizer.get_current_context()?;
        let atn = recognizer.get_interpreter().atn();
        let current_state = recognizer.get_state();
        if current_state < 0 {
            return None;
        }
        let current_target = atn.make_state_ref(current_state);
        let current_next = atn.next_tokens::<TF::Tok>(&current_target);
        let prev_token_index = recognizer
            .get_input_stream_mut()
            .lt(-1)
            .map(|t| t.get_token_index());

        let mut ctx = current_ctx;
        let mut target_state = current_target;
        let mut at_current = true;
        let mut best: Option<SkippedBookmark<'input, 'arena, P::Node, TF::Tok>> = None;

        for _ in 0..256 {
            if ctx.get_child_count() == 0 {
                if !at_current {
                    let rule_start = atn.rule_to_start_state[ctx.get_rule_index()];
                    if epsilon_reaches::<TF::Tok>(atn, rule_start, target_state)
                        && has_non_epsilon(atn.next_tokens::<TF::Tok>(&rule_start))
                    {
                        best = Some(SkippedBookmark {
                            state: rule_start.get_state_number(),
                            ctx,
                        });
                    }
                }
                break;
            }

            let Some(TrailingChild {
                mut chosen,
                fallback_rule,
            }) = trailing_child_at(ctx, prev_token_index)
            else {
                break;
            };

            if let Some(term) = chosen.as_terminal_node() {
                if let Some(post_state) = find_post_terminal_state::<TF::Tok>(
                    atn,
                    ctx.get_rule_index(),
                    term.symbol.get_token_type(),
                    target_state,
                ) {
                    let post_next = atn.next_tokens::<TF::Tok>(&post_state);
                    let adds_tokens = if at_current {
                        post_state.get_state_number() != current_state
                            && has_extra_tokens(post_next, current_next)
                    } else {
                        has_non_epsilon(post_next)
                    };
                    if adds_tokens {
                        best = Some(SkippedBookmark {
                            state: post_state.get_state_number(),
                            ctx,
                        });
                    }
                    break;
                }
                if let Some(fallback) = fallback_rule {
                    chosen = fallback;
                } else {
                    break;
                }
            }

            let invoking_state = chosen.get_invoking_state();
            if invoking_state < 0 {
                break;
            }
            let Some(rule_tr) = atn
                .get_state(invoking_state)
                .get_transitions()
                .first()
                .and_then(|tr| tr.try_as::<RuleTransition>())
            else {
                break;
            };
            if rule_tr.get_target().get_rule_index() as usize != chosen.get_rule_index() {
                break;
            }
            let follow = rule_tr.follow_state;
            if !epsilon_reaches::<TF::Tok>(atn, follow, target_state) {
                break;
            }
            let follow_next = atn.next_tokens::<TF::Tok>(&follow);
            let adds_tokens = if at_current {
                follow.get_state_number() != current_state
                    && has_extra_tokens(follow_next, current_next)
            } else {
                has_non_epsilon(follow_next)
            };
            if adds_tokens {
                best = Some(SkippedBookmark {
                    state: follow.get_state_number(),
                    ctx,
                });
            }

            ctx = chosen;
            target_state = atn.rule_to_stop_state[chosen.get_rule_index()];
            at_current = false;
        }

        best
    }
}

impl<'input, 'arena, TF, P> ErrorStrategy<'input, 'arena, TF, P>
    for DefaultErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    fn reset(&mut self, recognizer: &mut P) {
        self.end_error_condition(recognizer)
    }

    fn recover_inline(&mut self, recognizer: &mut P) -> Result<&'arena TF::Tok, ANTLRError> {
        let t = self
            .single_token_deletion(recognizer)?
            .map(|it| it.to_owned());
        if let Some(t) = t {
            recognizer.consume(self)?;
            return Ok(t);
        }

        if self.single_token_insertion(recognizer) {
            return Ok(self.get_missing_symbol(recognizer));
        }

        if let Some(next_tokens_ctx) = &self.next_tokens_ctx {
            Err(ANTLRError::input_mismatch_with_state(
                recognizer,
                self.next_tokens_state,
                next_tokens_ctx,
            ))
        } else if let Some(bookmark) = self.reconstruct_skipped_expecting(recognizer) {
            Err(ANTLRError::input_mismatch_with_state(
                recognizer,
                bookmark.state,
                bookmark.ctx,
            ))
        } else {
            Err(ANTLRError::input_mismatch(recognizer))
        }
        //        Err(ANTLRError::IllegalStateError("aaa".to_string()))
    }

    fn recover(&mut self, recognizer: &mut P, _e: &ANTLRError) -> Result<(), ANTLRError> {
        if self.last_error_index == recognizer.get_input_stream_mut().index()
            && self.last_error_states.is_some()
            && self
                .last_error_states
                .as_ref()
                .unwrap()
                .contains(recognizer.get_state())
        {
            recognizer.consume(self)?;
        }

        self.last_error_index = recognizer.get_input_stream_mut().index();
        self.last_error_states
            .get_or_insert(IntervalSetBuf::new())
            .apply(|x| x.add_one(recognizer.get_state()));
        let follow_set = self.get_error_recovery_set(recognizer);
        self.consume_until(recognizer, &follow_set)?;
        Ok(())
    }

    fn sync(&mut self, recognizer: &mut P) -> Result<(), ANTLRError> {
        if self.in_error_recovery_mode(recognizer) {
            return Ok(());
        }
        let next = recognizer.get_input_stream_mut().la(1);
        let state = recognizer
            .get_interpreter()
            .atn()
            .make_state_ref(recognizer.get_state());

        let next_tokens = recognizer
            .get_interpreter()
            .atn()
            .next_tokens::<TF::Tok>(&state);
        //        println!("{:?}",next_tokens);

        if next_tokens.contains(next) {
            self.next_tokens_state = ATNSTATE_INVALID_STATE_NUMBER;
            self.next_tokens_ctx = None;
            return Ok(());
        }

        if next_tokens.contains(TOKEN_EPSILON) {
            if self.next_tokens_ctx.is_none() {
                self.next_tokens_state = recognizer.get_state();
                self.next_tokens_ctx = recognizer.get_current_context();
            }
            return Ok(());
        }

        match state.get_state_type_id() {
            ATNSTATE_BLOCK_START
            | ATNSTATE_PLUS_BLOCK_START
            | ATNSTATE_STAR_BLOCK_START
            | ATNSTATE_STAR_LOOP_ENTRY => {
                if self.single_token_deletion(recognizer)?.is_none() {
                    return Err(ANTLRError::input_mismatch(recognizer));
                }
            }
            ATNSTATE_PLUS_LOOP_BACK | ATNSTATE_STAR_LOOP_BACK => {
                self.report_unwanted_token(recognizer);
                let mut expecting = recognizer.get_expected_tokens().into_owned();
                expecting.add_set(&self.get_error_recovery_set(recognizer));
                self.consume_until(recognizer, &expecting)?;
            }
            _ => panic!("invalid ANTState type id"),
        }

        Ok(())
    }

    #[inline(always)]
    fn in_error_recovery_mode(&mut self, _recognizer: &mut P) -> bool {
        self.error_recovery_mode
    }

    fn report_error(&mut self, recognizer: &mut P, e: &ANTLRError) {
        if self.in_error_recovery_mode(recognizer) {
            return;
        }

        self.begin_error_condition(recognizer);
        let msg = match e.as_ref() {
            ANTLRErrorKind::NoAltError(e) => self.report_no_viable_alternative(recognizer, e),
            ANTLRErrorKind::InputMismatchError(e) => self.report_input_mismatch(recognizer, e),
            ANTLRErrorKind::PredicateError(e) => self.report_failed_predicate(recognizer, e),
            _ => e.to_string(),
        };
        let offending_token_index = e.get_offending_token().map(|it| it.get_token_index());
        recognizer.notify_error_listeners(msg, offending_token_index, Some(e))
    }

    #[inline(always)]
    fn report_match(&mut self, recognizer: &mut P) {
        self.end_error_condition(recognizer);
        //println!("matched token succesfully {}", recognizer.get_input_stream().la(1))
    }
}

/// This implementation of `ANTLRErrorStrategy` responds to syntax errors
/// by immediately canceling the parse operation with a
/// `ParseCancellationException`. The implementation ensures that the
/// [`ParserRuleContext.exception`] field is set for all parse tree nodes
/// that were not completed prior to encountering the error.
///
/// <p> This error strategy is useful in the following scenarios.</p>
///
///  - Two-stage parsing: This error strategy allows the first stage of
///    two-stage parsing to immediately terminate if an error is encountered,
///    and immediately fall back to the second stage. In addition to avoiding
///    wasted work by attempting to recover from errors here, the empty
///    implementation of `sync` improves the performance of the first stage.
///  - Silent validation: When syntax errors are not being reported or logged,
///    and the parse result is simply ignored if errors occur, the
///    `BailErrorStrategy` avoids wasting work on recovering from errors when
///    the result will be ignored either way.
///
/// # Usage
/// ```ignore
/// use dbt_antlr_runtime::error_strategy::BailErrorStrategy;
/// myparser.err_handler = BailErrorStrategy::new();
/// ```
///
/// [`ParserRuleContext.exception`]: todo
/// */
#[derive(Default, Debug)]
pub struct BailErrorStrategy<'input, 'arena, TF, P>(DefaultErrorStrategy<'input, 'arena, TF, P>)
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>;

impl<'input, 'arena, TF, P> BailErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    /// Creates new instance of `BailErrorStrategy`
    pub fn new() -> Self {
        Self(DefaultErrorStrategy::new())
    }

    fn process_error(&self, recognizer: &mut P, e: &ANTLRError) -> ANTLRError {
        let mut ctx = recognizer.get_current_context();
        let _: Option<()> = (|| loop {
            ctx?.set_exception(e.clone(), recognizer.get_arena());
            ctx = ctx.unwrap().get_parent()
        })();
        ANTLRError::fall_through(Arc::new(ParseCancelledError(e.clone())))
    }
}

/// `ANTLRError::FallThrough` Error returned `BailErrorStrategy` to bail out from parsing
#[derive(Debug)]
pub struct ParseCancelledError(ANTLRError);

impl Error for ParseCancelledError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl Display for ParseCancelledError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("ParseCancelledError, caused by ")?;
        self.0.fmt(f)
    }
}

impl<'input, 'arena, TF, P> ErrorStrategy<'input, 'arena, TF, P>
    for BailErrorStrategy<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    #[inline(always)]
    fn reset(&mut self, recognizer: &mut P) {
        self.0.reset(recognizer)
    }

    #[cold]
    fn recover_inline(&mut self, recognizer: &mut P) -> Result<&'arena TF::Tok, ANTLRError> {
        let err = ANTLRError::input_mismatch(recognizer);

        Err(self.process_error(recognizer, &err))
    }

    #[cold]
    fn recover(&mut self, recognizer: &mut P, e: &ANTLRError) -> Result<(), ANTLRError> {
        Err(self.process_error(recognizer, e))
    }

    #[inline(always)]
    fn sync(&mut self, _recognizer: &mut P) -> Result<(), ANTLRError> {
        /* empty */
        Ok(())
    }

    #[inline(always)]
    fn in_error_recovery_mode(&mut self, recognizer: &mut P) -> bool {
        self.0.in_error_recovery_mode(recognizer)
    }

    #[inline(always)]
    fn report_error(&mut self, recognizer: &mut P, e: &ANTLRError) {
        self.0.report_error(recognizer, e)
    }

    #[inline(always)]
    fn report_match(&mut self, _recognizer: &mut P) {}
}

enum ErrorStrategyRepr<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    Default(DefaultErrorStrategy<'input, 'arena, TF, P>),
    Custom { data: *mut (), vtable: *const () },
}

/// Internal error-strategy wrapper that stores `DefaultErrorStrategy` inline
/// and type-erases custom boxed strategies covariantly over `'input`.
///
/// NOTE: this is an *internal* type, declared public only because generated
/// parsers hold it as a field.
pub struct ErrorStrategyDelegate<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    repr: ErrorStrategyRepr<'input, 'arena, TF, P>,

    // Invariant over 'arena, covariant over 'input, TF, P
    _marker: PhantomData<(&'input (), *mut &'arena (), TF, P)>,
}

impl<'input, 'arena, TF, P> fmt::Debug for ErrorStrategyDelegate<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.repr {
            ErrorStrategyRepr::Default(s) => f
                .debug_struct("ErrorStrategyDelegate")
                .field("kind", &"default")
                .field("error_recovery_mode", &s.error_recovery_mode)
                .finish_non_exhaustive(),
            ErrorStrategyRepr::Custom { data, .. } => f
                .debug_struct("ErrorStrategyDelegate")
                .field("data", data)
                .finish_non_exhaustive(),
        }
    }
}

impl<'input, 'arena, TF, P> ErrorStrategyDelegate<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    /// Creates an inline `DefaultErrorStrategy` delegate with no heap allocation.
    #[inline]
    pub fn new_default() -> Self {
        Self {
            repr: ErrorStrategyRepr::Default(DefaultErrorStrategy::new()),
            _marker: PhantomData,
        }
    }

    /// # Safety
    ///
    /// This is an internal type that is only meant to be constructed by
    /// generated code, NOT for public consumption.
    pub unsafe fn new(s: Box<dyn ErrorStrategy<'input, 'arena, TF, P> + 'arena>) -> Self {
        let raw = Box::into_raw(s);
        // SAFETY: *mut dyn Trait is a fat pointer (data_ptr, vtable_ptr).
        let (data, vtable): (*mut (), *const ()) = unsafe { std::mem::transmute(raw) };
        Self {
            repr: ErrorStrategyRepr::Custom { data, vtable },
            _marker: PhantomData,
        }
    }

    #[inline(always)]
    unsafe fn custom_as_mut_dyn<'a>(
        data: *mut (),
        vtable: *const (),
    ) -> &'a mut (dyn ErrorStrategy<'input, 'arena, TF, P> + 'arena) {
        // SAFETY: the [ErrorStrategy] trait is covariant over 'input and
        // 'arena, and the caller holds `&mut self` for the lifetime of the call.
        std::mem::transmute((data, vtable))
    }

    /// Reports and recovers from a rule error, or returns unrecoverable errors.
    #[cold]
    #[inline(never)]
    pub fn handle_rule_error(
        &mut self,
        recognizer: &mut P,
        e: ANTLRError,
    ) -> Result<(), ANTLRError> {
        if !e.is_recoverable() {
            return Err(e);
        }
        self.report_error(recognizer, &e);
        self.recover(recognizer, &e)
    }

    /// Runs `ErrorStrategy::sync` on a deferred mismatch or loop-exit path.
    ///
    /// Returns `Ok(true)` when `sync` advances the input stream.
    /// Returns `Ok(false)` when the stream position does not change.
    #[cold]
    #[inline(never)]
    pub fn sync_deferred(&mut self, recognizer: &mut P) -> Result<bool, ANTLRError> {
        let before = recognizer.get_input_stream().index();
        self.sync(recognizer)?;
        Ok(recognizer.get_input_stream().index() != before)
    }

    /// Runs `ErrorStrategy::sync` on a cold mismatch path.
    #[cold]
    #[inline(never)]
    pub fn sync_cold(&mut self, recognizer: &mut P) -> Result<(), ANTLRError> {
        self.sync(recognizer)
    }

    /// Runs `ErrorStrategy::sync` on a block-start mismatch path.
    ///
    /// Returns `Ok(())` when `sync` advances the input stream.
    /// Returns `NoAltError` when the stream position does not change.
    #[cold]
    #[inline(never)]
    pub fn sync_block_start_or_no_alt(&mut self, recognizer: &mut P) -> Result<(), ANTLRError> {
        let before = recognizer.get_input_stream().index();
        self.sync(recognizer)?;
        if recognizer.get_input_stream().index() != before {
            Ok(())
        } else {
            Err(ANTLRError::no_alt(recognizer))
        }
    }
}

impl<'input, 'arena, TF, P> Drop for ErrorStrategyDelegate<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    fn drop(&mut self) {
        if let ErrorStrategyRepr::Custom { data, vtable } = self.repr {
            unsafe {
                drop(Box::from_raw(Self::custom_as_mut_dyn(data, vtable)));
            }
        }
    }
}

impl<'input, 'arena, TF, P> ErrorStrategy<'input, 'arena, TF, P>
    for ErrorStrategyDelegate<'input, 'arena, TF, P>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    P: Parser<'input, 'arena, TF>,
{
    #[inline(always)]
    fn reset(&mut self, recognizer: &mut P) {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.reset(recognizer),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }.reset(recognizer)
            }
        }
    }

    #[inline(always)]
    fn recover_inline(&mut self, recognizer: &mut P) -> Result<&'arena TF::Tok, ANTLRError> {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.recover_inline(recognizer),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }.recover_inline(recognizer)
            }
        }
    }

    #[inline(always)]
    fn recover(&mut self, recognizer: &mut P, e: &ANTLRError) -> Result<(), ANTLRError> {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.recover(recognizer, e),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }.recover(recognizer, e)
            }
        }
    }

    #[inline(always)]
    fn sync(&mut self, recognizer: &mut P) -> Result<(), ANTLRError> {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.sync(recognizer),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }.sync(recognizer)
            }
        }
    }

    #[inline(always)]
    fn in_error_recovery_mode(&mut self, recognizer: &mut P) -> bool {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.in_error_recovery_mode(recognizer),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }
                    .in_error_recovery_mode(recognizer)
            }
        }
    }

    #[inline(always)]
    fn report_error(&mut self, recognizer: &mut P, e: &ANTLRError) {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.report_error(recognizer, e),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }.report_error(recognizer, e)
            }
        }
    }

    #[inline(always)]
    fn report_match(&mut self, recognizer: &mut P) {
        match &mut self.repr {
            ErrorStrategyRepr::Default(s) => s.report_match(recognizer),
            ErrorStrategyRepr::Custom { data, vtable } => {
                unsafe { Self::custom_as_mut_dyn(*data, *vtable) }.report_match(recognizer)
            }
        }
    }
}

struct SkippedBookmark<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    state: i32,
    ctx: &'arena TreeNode<'input, 'arena, Node, Tok>,
}

struct TrailingChild<'input, 'arena, Node, Tok>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    chosen: &'arena TreeNode<'input, 'arena, Node, Tok>,
    fallback_rule: Option<&'arena TreeNode<'input, 'arena, Node, Tok>>,
}

fn is_zero_token_rule<'input, 'arena, Node, Tok>(
    node: &'arena TreeNode<'input, 'arena, Node, Tok>,
) -> bool
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    if node.get_child_count() == 0 {
        return true;
    }
    let start_idx = node.get_start_token().map(|t| t.get_token_index());
    let stop_idx = node.get_stop_token().map(|t| t.get_token_index());
    match (start_idx, stop_idx) {
        (_, None) => true,
        (Some(start), Some(stop)) => start > stop,
        (None, Some(_)) => false,
    }
}

fn trailing_child_at<'input, 'arena, Node, Tok>(
    ctx: &'arena TreeNode<'input, 'arena, Node, Tok>,
    prev_token_index: Option<isize>,
) -> Option<TrailingChild<'input, 'arena, Node, Tok>>
where
    'input: 'arena,
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'input,
{
    let mut idx = ctx.get_child_count().checked_sub(1)?;
    let mut cur = ctx.get_child(idx)?;
    if cur.as_error_node().is_some() {
        return None;
    }
    if let Some(term) = cur.as_terminal_node() {
        let tidx = term.symbol.get_token_index();
        if tidx < 0 || Some(tidx) != prev_token_index {
            return None;
        }
        return Some(TrailingChild {
            chosen: cur,
            fallback_rule: None,
        });
    }
    let stop_idx = cur.get_stop_token().map(|t| t.get_token_index());
    if stop_idx.is_some_and(|i| i < 0) || stop_idx != prev_token_index {
        return None;
    }

    while idx > 0 && is_zero_token_rule(cur) {
        let prev = ctx.get_child(idx - 1)?;
        if prev.as_error_node().is_some() {
            break;
        }
        if let Some(term) = prev.as_terminal_node() {
            let tidx = term.symbol.get_token_index();
            if tidx >= 0 && Some(tidx) == prev_token_index {
                return Some(TrailingChild {
                    chosen: prev,
                    fallback_rule: Some(cur),
                });
            }
            break;
        }
        let prev_stop = prev.get_stop_token().map(|t| t.get_token_index());
        if prev_stop.is_some_and(|i| i < 0) || prev_stop != prev_token_index {
            break;
        }
        idx -= 1;
        cur = prev;
    }

    Some(TrailingChild {
        chosen: cur,
        fallback_rule: None,
    })
}

fn epsilon_reaches<Tok: Token>(atn: &ATN, from: ATNStateRef, target: ATNStateRef) -> bool {
    let target_num = target.get_state_number();
    let mut visited = HashSet::new();
    let mut stack = vec![from];
    while let Some(state) = stack.pop() {
        let state_num = state.get_state_number();
        if state_num == target_num {
            return true;
        }
        if !visited.insert(state_num) {
            continue;
        }
        for tr in state.get_transitions() {
            match tr.transition_type() {
                TransitionType::Rule => {
                    let rule_tr = tr.try_as::<RuleTransition>().unwrap();
                    if atn
                        .next_tokens::<Tok>(&rule_tr.get_target())
                        .contains(TOKEN_EPSILON)
                    {
                        stack.push(rule_tr.follow_state);
                    }
                }
                TransitionType::Predicate | TransitionType::PrecedencePredicate => {
                    stack.push(tr.get_target());
                }
                _ if tr.is_epsilon() => {
                    stack.push(tr.get_target());
                }
                _ => {}
            }
        }
    }
    false
}

fn find_post_terminal_state<Tok: Token>(
    atn: &ATN,
    rule_index: usize,
    ttype: i32,
    target_state: ATNStateRef,
) -> Option<ATNStateRef> {
    let start = atn.rule_to_start_state[rule_index];
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    let mut found: Option<(ATNStateRef, &IntervalSet)> = None;

    while let Some(state) = stack.pop() {
        let state_num = state.get_state_number();
        if state.get_rule_index() as usize != rule_index || !visited.insert(state_num) {
            continue;
        }

        for tr in state.get_transitions() {
            if let Some(rule_tr) = tr.try_as::<RuleTransition>() {
                stack.push(rule_tr.follow_state);
                continue;
            }
            let next = tr.get_target();
            stack.push(next);
            if !tr.is_epsilon()
                && tr.matches(ttype, TOKEN_MIN_USER_TOKEN_TYPE, atn.max_token_type)
                && epsilon_reaches::<Tok>(atn, next, target_state)
            {
                let next_tokens = atn.next_tokens::<Tok>(&next);
                match found {
                    None => found = Some((next, next_tokens)),
                    Some((_, prev_tokens)) if prev_tokens == next_tokens => {}
                    Some(_) => return None,
                }
            }
        }
    }

    found.map(|(state, _)| state)
}

fn has_non_epsilon(set: &IntervalSet) -> bool {
    set.as_slice()
        .iter()
        .any(|iv| iv.a != TOKEN_EPSILON || iv.b != TOKEN_EPSILON)
}

fn has_extra_tokens(candidate: &IntervalSet, baseline: &IntervalSet) -> bool {
    let base = baseline.as_slice();
    for iv in candidate.as_slice() {
        let mut pos = if iv.a == TOKEN_EPSILON {
            iv.a + 1
        } else {
            iv.a
        };
        if pos > iv.b {
            continue;
        }
        for b in base {
            if b.b < pos {
                continue;
            }
            if b.a > pos {
                return true;
            }
            if b.b >= iv.b {
                pos = iv.b.saturating_add(1);
                break;
            }
            pos = b.b.saturating_add(1);
        }
        if pos <= iv.b {
            return true;
        }
    }
    false
}
