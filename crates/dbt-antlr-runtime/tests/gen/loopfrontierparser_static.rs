// Generated from LoopFrontier.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const LoopFrontier_T__0:i32=1; 
pub const LoopFrontier_T__1:i32=2; 
pub const LoopFrontier_T__2:i32=3; 
pub const LoopFrontier_T__3:i32=4; 
pub const LoopFrontier_T__4:i32=5; 
pub const LoopFrontier_T__5:i32=6; 
pub const LoopFrontier_T__6:i32=7; 
pub const LoopFrontier_T__7:i32=8; 
pub const LoopFrontier_T__8:i32=9; 
pub const LoopFrontier_T__9:i32=10; 
pub const LoopFrontier_T__10:i32=11; 
pub const LoopFrontier_AS:i32=12; 
pub const LoopFrontier_CAST:i32=13; 
pub const LoopFrontier_TRY_CAST:i32=14; 
pub const LoopFrontier_CASE:i32=15; 
pub const LoopFrontier_WHEN:i32=16; 
pub const LoopFrontier_THEN:i32=17; 
pub const LoopFrontier_ELSE:i32=18; 
pub const LoopFrontier_END:i32=19; 
pub const LoopFrontier_EXTRACT:i32=20; 
pub const LoopFrontier_SUBSTRING:i32=21; 
pub const LoopFrontier_POSITION:i32=22; 
pub const LoopFrontier_ROW:i32=23; 
pub const LoopFrontier_TIMESTAMP:i32=24; 
pub const LoopFrontier_INTERVAL:i32=25; 
pub const LoopFrontier_FROM:i32=26; 
pub const LoopFrontier_FOR:i32=27; 
pub const LoopFrontier_IN:i32=28; 
pub const LoopFrontier_VARYING:i32=29; 
pub const LoopFrontier_STRING:i32=30; 
pub const LoopFrontier_ID:i32=31; 
pub const LoopFrontier_INT:i32=32; 
pub const LoopFrontier_WS:i32=33;
pub const LoopFrontier_EOF:i32=EOF;
pub const RULE_start:usize = 0; 
pub const RULE_expr:usize = 1; 
pub const RULE_operand:usize = 2; 
pub const RULE_type_:usize = 3; 
pub const RULE_whenClause:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "start", "expr", "operand", "type_", "whenClause"
];

pub const _LITERAL_NAMES: [Option<&'static str>;30] = [
	None, Some("'set'"), Some("'||'"), Some("'+'"), Some("'-'"), Some("'*'"), 
	Some("'/'"), Some("'<<'"), Some("'('"), Some("')'"), Some("','"), Some("'.'"), 
	Some("'AS'"), Some("'CAST'"), Some("'TRY_CAST'"), Some("'CASE'"), Some("'WHEN'"), 
	Some("'THEN'"), Some("'ELSE'"), Some("'END'"), Some("'EXTRACT'"), Some("'SUBSTRING'"), 
	Some("'POSITION'"), Some("'ROW'"), Some("'TIMESTAMP'"), Some("'INTERVAL'"), 
	Some("'FROM'"), Some("'FOR'"), Some("'IN'"), Some("'VARYING'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;34]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	Some("AS"), Some("CAST"), Some("TRY_CAST"), Some("CASE"), Some("WHEN"), 
	Some("THEN"), Some("ELSE"), Some("END"), Some("EXTRACT"), Some("SUBSTRING"), 
	Some("POSITION"), Some("ROW"), Some("TIMESTAMP"), Some("INTERVAL"), Some("FROM"), 
	Some("FOR"), Some("IN"), Some("VARYING"), Some("STRING"), Some("ID"), Some("INT"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, LoopFrontierParserExt<'input, 'arena>, LoopFrontierParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				LoopFrontierParserExt {
					_pd: Default::default(),
				}
			),
            err_handler: unsafe { ErrorStrategyDelegate::new(strategy) },
        }
    }

    pub fn new(arena: &'arena Arena, input: Input) -> Self{
    	Self::with_strategy(arena, input, Box::new(DefaultErrorStrategy::new()))
    }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) {
        self.err_handler = unsafe { ErrorStrategyDelegate::new(strategy) };
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_parse_listener<L>(
        &mut self,
        listener: Box<L>,
    ) -> ListenerId<L>
    where
        L: ParseTreeListener<'arena, LoopFrontierParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct LoopFrontierTreeWalker;
impl LoopFrontierTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, LoopFrontierParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, LoopFrontierParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <LoopFrontierParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LoopFrontierParserNodeKind {
    StartContext,
    ExprContext,
    OperandContext,
    Type_Context,
    WhenClauseContext,
    Terminal,
    Error,
}
pub type LoopFrontierParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LoopFrontierParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => LoopFrontierParser }
dbt_antlr_runtime::impl_node_kind! { LoopFrontierParserNodeKind {
; StartContext(), ExprContext(), OperandContext(), Type_Context(), WhenClauseContext(), 
    }; 
}

pub struct LoopFrontierParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> LoopFrontierParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { LoopFrontierParserExt, LoopFrontierParserNodeKind, "LoopFrontier.g4"; sempred (LoopFrontierParserNode<'input, 'arena, TF::Tok>, LoopFrontierParser) { 1 => expr_sempred, } }

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn expr_sempred(_ctx: Option<&'arena ExprContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 4)
		    }
	        1 => {
			recog.precpred(None, 3)
		    }
	        2 => {
			recog.precpred(None, 2)
		    }
	        3 => {
			recog.precpred(None, 1)
		    }
		    _ => true
		}
	}
}
//------------------- start ----------------
pub type StartContextAll<'input, 'arena, Tok = CommonToken<'input>> = StartContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopFrontierParserNodeKind, StartContext, StartContextExt, RULE_start }

pub trait StartContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StartContextAttrs<'input, 'arena, Tok> for StartContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_EOF)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_INT)
    }
}

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn start(&mut self,) -> Result<&'arena StartContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StartContext<TF::Tok>, RULE_start, 0, |_parentctx| StartContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StartContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(23);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopFrontier_T__2 |LoopFrontier_T__3 |LoopFrontier_T__7 |LoopFrontier_CAST |
		    LoopFrontier_TRY_CAST |LoopFrontier_CASE |LoopFrontier_EXTRACT |LoopFrontier_SUBSTRING |
		    LoopFrontier_POSITION |LoopFrontier_ROW |LoopFrontier_TIMESTAMP |LoopFrontier_INTERVAL |
		    LoopFrontier_ID |LoopFrontier_INT  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(11); 
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        loop {
		        	{
		        	{
		        	recog.base.set_state(10);
		        	recog.expr_rec(0)?;
		        	}
		        	}
		        	recog.base.set_state(13); 
		        	recog.err_handler.sync(&mut recog.base)?;
		        	_la = recog.base.input.la(1);
		        	if !(((((_la - 3)) & !0x3f) == 0 && ((1usize << (_la - 3)) & 813571107) != 0)) {break}
		        }
		        recog.base.set_state(15);
		        recog.base.match_token(LoopFrontier_EOF,&mut recog.err_handler)?;
		        }}
		    LoopFrontier_T__0  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(17);
		        recog.base.match_token(LoopFrontier_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(18);
		        recog.expr_rec(0)?;
		        recog.base.set_state(19);
		        recog.base.match_token(LoopFrontier_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(20);
		        recog.base.match_token(LoopFrontier_INT,&mut recog.err_handler)?;
		        recog.base.set_state(21);
		        recog.base.match_token(LoopFrontier_EOF,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExprContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopFrontierParserNodeKind, ExprContext, ExprContextExt, RULE_expr }

pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn operand(&self) -> Option<&'arena OperandContextAll<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn operand(&self) -> Option<&'arena OperandContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  expr(&mut self,) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: i32) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, ExprContext<TF::Tok>, RULE_expr, 2, |_parentctx| ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(29);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopFrontier_T__7 |LoopFrontier_CAST |LoopFrontier_TRY_CAST |LoopFrontier_CASE |
		    LoopFrontier_EXTRACT |LoopFrontier_SUBSTRING |LoopFrontier_POSITION |
		    LoopFrontier_ROW |LoopFrontier_TIMESTAMP |LoopFrontier_INTERVAL |LoopFrontier_ID |
		    LoopFrontier_INT  => {
		        {
		        recog.base.set_state(26);
		        recog.operand()?;
		        }}
		    LoopFrontier_T__2 |LoopFrontier_T__3  => {
		        {
		        recog.base.set_state(27);
		        _la = recog.base.input.la(1);
		        if { !(_la==LoopFrontier_T__2 || _la==LoopFrontier_T__3) } {
		        	recog.err_handler.recover_inline(&mut recog.base)?;
		        }
		        else {
		        	if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        	recog.err_handler.report_match(&mut recog.base);
		        	recog.base.consume(&mut recog.err_handler)?;
		        }
		        recog.base.set_state(28);
		        recog.expr_rec(5)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(45);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(4)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(4,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(43);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(3)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(31);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(32);
						_la = recog.base.input.la(1);
						if { !(_la==LoopFrontier_T__4 || _la==LoopFrontier_T__5) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(33);
						recog.expr_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(34);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(35);
						_la = recog.base.input.la(1);
						if { !(_la==LoopFrontier_T__2 || _la==LoopFrontier_T__3) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(36);
						recog.expr_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(37);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(38);
						recog.base.match_token(LoopFrontier_T__1,&mut recog.err_handler)?;
						recog.base.set_state(39);
						recog.expr_rec(3)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(40);
						if !({recog.precpred(None, 1)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
						}
						recog.base.set_state(41);
						recog.base.match_token(LoopFrontier_T__6,&mut recog.err_handler)?;
						recog.base.set_state(42);
						recog.expr_rec(2)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(47);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(4)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(4,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}
//------------------- operand ----------------
pub type OperandContextAll<'input, 'arena, Tok = CommonToken<'input>> = OperandContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopFrontierParserNodeKind, OperandContext, OperandContextExt, RULE_operand }

pub trait OperandContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token CAST
    /// Returns `None` if there is no child corresponding to token CAST
    fn CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn type_(&self) -> Option<&'arena Type_ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TRY_CAST
    /// Returns `None` if there is no child corresponding to token TRY_CAST
    fn TRY_CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token END
    /// Returns `None` if there is no child corresponding to token END
    fn END(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn whenClause_all(&self) -> Vec<&'arena WhenClauseContextAll<'input, 'arena, Tok>>;
    fn whenClause(&self, i: usize) -> Option<&'arena WhenClauseContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EXTRACT
    /// Returns `None` if there is no child corresponding to token EXTRACT
    fn EXTRACT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SUBSTRING
    /// Returns `None` if there is no child corresponding to token SUBSTRING
    fn SUBSTRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token POSITION
    /// Returns `None` if there is no child corresponding to token POSITION
    fn POSITION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ROW
    /// Returns `None` if there is no child corresponding to token ROW
    fn ROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TIMESTAMP
    /// Returns `None` if there is no child corresponding to token TIMESTAMP
    fn TIMESTAMP(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INTERVAL
    /// Returns `None` if there is no child corresponding to token INTERVAL
    fn INTERVAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> OperandContextAttrs<'input, 'arena, Tok> for OperandContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_INT)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == LoopFrontier_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == LoopFrontier_ID).nth(i)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CAST
    /// Returns `None` if there is no child corresponding to token CAST
    fn CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_CAST)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_AS)
    }
    fn type_(&self) -> Option<&'arena Type_ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token TRY_CAST
    /// Returns `None` if there is no child corresponding to token TRY_CAST
    fn TRY_CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_TRY_CAST)
    }
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_CASE)
    }
    /// Retrieves first TerminalNode corresponding to token END
    /// Returns `None` if there is no child corresponding to token END
    fn END(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_END)
    }
    fn whenClause_all(&self) -> Vec<&'arena WhenClauseContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn whenClause(&self, i: usize) -> Option<&'arena WhenClauseContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_ELSE)
    }
    /// Retrieves first TerminalNode corresponding to token EXTRACT
    /// Returns `None` if there is no child corresponding to token EXTRACT
    fn EXTRACT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_EXTRACT)
    }
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_FROM)
    }
    /// Retrieves first TerminalNode corresponding to token SUBSTRING
    /// Returns `None` if there is no child corresponding to token SUBSTRING
    fn SUBSTRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_SUBSTRING)
    }
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_FOR)
    }
    /// Retrieves first TerminalNode corresponding to token POSITION
    /// Returns `None` if there is no child corresponding to token POSITION
    fn POSITION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_POSITION)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_IN)
    }
    /// Retrieves first TerminalNode corresponding to token ROW
    /// Returns `None` if there is no child corresponding to token ROW
    fn ROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_ROW)
    }
    /// Retrieves first TerminalNode corresponding to token TIMESTAMP
    /// Returns `None` if there is no child corresponding to token TIMESTAMP
    fn TIMESTAMP(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_TIMESTAMP)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_STRING)
    }
    /// Retrieves first TerminalNode corresponding to token INTERVAL
    /// Returns `None` if there is no child corresponding to token INTERVAL
    fn INTERVAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_INTERVAL)
    }
}

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn operand(&mut self,) -> Result<&'arena OperandContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, OperandContext<TF::Tok>, RULE_operand, 4, |_parentctx| OperandContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena OperandContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(140);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(12)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(12,&mut recog.base)? } else { _sdp } } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(48);
				recog.base.match_token(LoopFrontier_INT,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(49);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(50);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(51);
				recog.expr_rec(0)?;
				recog.base.set_state(52);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(54);
				recog.base.match_token(LoopFrontier_CAST,&mut recog.err_handler)?;
				recog.base.set_state(55);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(56);
				recog.expr_rec(0)?;
				recog.base.set_state(57);
				recog.base.match_token(LoopFrontier_AS,&mut recog.err_handler)?;
				recog.base.set_state(58);
				recog.type_()?;
				recog.base.set_state(59);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			5 =>{
				/*------- Outer Most Alt 5 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
				{
				recog.base.set_state(61);
				recog.base.match_token(LoopFrontier_TRY_CAST,&mut recog.err_handler)?;
				recog.base.set_state(62);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(63);
				recog.expr_rec(0)?;
				recog.base.set_state(64);
				recog.base.match_token(LoopFrontier_AS,&mut recog.err_handler)?;
				recog.base.set_state(65);
				recog.type_()?;
				recog.base.set_state(66);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			6 =>{
				/*------- Outer Most Alt 6 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
				{
				recog.base.set_state(68);
				recog.base.match_token(LoopFrontier_CASE,&mut recog.err_handler)?;
				recog.base.set_state(70); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				loop {
					{
					{
					recog.base.set_state(69);
					recog.whenClause()?;
					}
					}
					recog.base.set_state(72); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if !(_la==LoopFrontier_WHEN) {break}
				}
				recog.base.set_state(76);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==LoopFrontier_ELSE {
					{
					recog.base.set_state(74);
					recog.base.match_token(LoopFrontier_ELSE,&mut recog.err_handler)?;
					recog.base.set_state(75);
					recog.expr_rec(0)?;
					}
				}

				recog.base.set_state(78);
				recog.base.match_token(LoopFrontier_END,&mut recog.err_handler)?;
				}
			}
		,
			7 =>{
				/*------- Outer Most Alt 7 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(7); }
				{
				recog.base.set_state(80);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				recog.base.set_state(81);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(90);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 3)) & !0x3f) == 0 && ((1usize << (_la - 3)) & 813571107) != 0) {
					{
					recog.base.set_state(82);
					recog.expr_rec(0)?;
					recog.base.set_state(87);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==LoopFrontier_T__9 {
						{
						{
						recog.base.set_state(83);
						recog.base.match_token(LoopFrontier_T__9,&mut recog.err_handler)?;
						recog.base.set_state(84);
						recog.expr_rec(0)?;
						}
						}
						recog.base.set_state(89);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				recog.base.set_state(92);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			8 =>{
				/*------- Outer Most Alt 8 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(8); }
				{
				recog.base.set_state(93);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				recog.base.set_state(94);
				recog.base.match_token(LoopFrontier_T__10,&mut recog.err_handler)?;
				recog.base.set_state(95);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				}
			}
		,
			9 =>{
				/*------- Outer Most Alt 9 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(9); }
				{
				recog.base.set_state(96);
				recog.base.match_token(LoopFrontier_EXTRACT,&mut recog.err_handler)?;
				recog.base.set_state(97);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(98);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				recog.base.set_state(99);
				recog.base.match_token(LoopFrontier_FROM,&mut recog.err_handler)?;
				recog.base.set_state(100);
				recog.expr_rec(0)?;
				recog.base.set_state(101);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			10 =>{
				/*------- Outer Most Alt 10 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(10); }
				{
				recog.base.set_state(103);
				recog.base.match_token(LoopFrontier_SUBSTRING,&mut recog.err_handler)?;
				recog.base.set_state(104);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(105);
				recog.expr_rec(0)?;
				recog.base.set_state(107);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==LoopFrontier_FROM {
					{
					recog.base.set_state(106);
					recog.base.match_token(LoopFrontier_FROM,&mut recog.err_handler)?;
					}
				}

				recog.base.set_state(109);
				recog.expr_rec(0)?;
				recog.base.set_state(112);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==LoopFrontier_FOR {
					{
					recog.base.set_state(110);
					recog.base.match_token(LoopFrontier_FOR,&mut recog.err_handler)?;
					recog.base.set_state(111);
					recog.expr_rec(0)?;
					}
				}

				recog.base.set_state(114);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			11 =>{
				/*------- Outer Most Alt 11 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(11); }
				{
				recog.base.set_state(116);
				recog.base.match_token(LoopFrontier_POSITION,&mut recog.err_handler)?;
				recog.base.set_state(117);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(118);
				recog.expr_rec(0)?;
				recog.base.set_state(119);
				recog.base.match_token(LoopFrontier_IN,&mut recog.err_handler)?;
				recog.base.set_state(120);
				recog.expr_rec(0)?;
				recog.base.set_state(121);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			12 =>{
				/*------- Outer Most Alt 12 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(12); }
				{
				recog.base.set_state(123);
				recog.base.match_token(LoopFrontier_ROW,&mut recog.err_handler)?;
				recog.base.set_state(124);
				recog.base.match_token(LoopFrontier_T__7,&mut recog.err_handler)?;
				recog.base.set_state(125);
				recog.expr_rec(0)?;
				recog.base.set_state(130);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==LoopFrontier_T__9 {
					{
					{
					recog.base.set_state(126);
					recog.base.match_token(LoopFrontier_T__9,&mut recog.err_handler)?;
					recog.base.set_state(127);
					recog.expr_rec(0)?;
					}
					}
					recog.base.set_state(132);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(133);
				recog.base.match_token(LoopFrontier_T__8,&mut recog.err_handler)?;
				}
			}
		,
			13 =>{
				/*------- Outer Most Alt 13 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(13); }
				{
				recog.base.set_state(135);
				recog.base.match_token(LoopFrontier_TIMESTAMP,&mut recog.err_handler)?;
				recog.base.set_state(136);
				recog.base.match_token(LoopFrontier_STRING,&mut recog.err_handler)?;
				}
			}
		,
			14 =>{
				/*------- Outer Most Alt 14 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(14); }
				{
				recog.base.set_state(137);
				recog.base.match_token(LoopFrontier_INTERVAL,&mut recog.err_handler)?;
				recog.base.set_state(138);
				recog.base.match_token(LoopFrontier_STRING,&mut recog.err_handler)?;
				recog.base.set_state(139);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- type_ ----------------
pub type Type_ContextAll<'input, 'arena, Tok = CommonToken<'input>> = Type_Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopFrontierParserNodeKind, Type_Context, Type_ContextExt, RULE_type_ }

pub trait Type_ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token VARYING
    /// Returns `None` if there is no child corresponding to token VARYING
    fn VARYING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Type_ContextAttrs<'input, 'arena, Tok> for Type_Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_ID)
    }
    /// Retrieves first TerminalNode corresponding to token VARYING
    /// Returns `None` if there is no child corresponding to token VARYING
    fn VARYING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_VARYING)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_INT)
    }
}

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn type_(&mut self,) -> Result<&'arena Type_ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, Type_Context<TF::Tok>, RULE_type_, 6, |_parentctx| Type_ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena Type_Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(146);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(13)?; _sdp } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(142);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(143);
				recog.base.match_token(LoopFrontier_ID,&mut recog.err_handler)?;
				recog.base.set_state(144);
				recog.base.match_token(LoopFrontier_VARYING,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(145);
				recog.base.match_token(LoopFrontier_INT,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- whenClause ----------------
pub type WhenClauseContextAll<'input, 'arena, Tok = CommonToken<'input>> = WhenClauseContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopFrontierParserNodeKind, WhenClauseContext, WhenClauseContextExt, RULE_whenClause }

pub trait WhenClauseContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token WHEN
    /// Returns `None` if there is no child corresponding to token WHEN
    fn WHEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token THEN
    /// Returns `None` if there is no child corresponding to token THEN
    fn THEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> WhenClauseContextAttrs<'input, 'arena, Tok> for WhenClauseContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token WHEN
    /// Returns `None` if there is no child corresponding to token WHEN
    fn WHEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_WHEN)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token THEN
    /// Returns `None` if there is no child corresponding to token THEN
    fn THEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopFrontier_THEN)
    }
}

impl<'input, 'arena, Input, TF> LoopFrontierParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn whenClause(&mut self,) -> Result<&'arena WhenClauseContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, WhenClauseContext<TF::Tok>, RULE_whenClause, 8, |_parentctx| WhenClauseContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena WhenClauseContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(148);
		recog.base.match_token(LoopFrontier_WHEN,&mut recog.err_handler)?;
		recog.base.set_state(149);
		recog.expr_rec(0)?;
		recog.base.set_state(150);
		recog.base.match_token(LoopFrontier_THEN,&mut recog.err_handler)?;
		recog.base.set_state(151);
		recog.expr_rec(0)?;
		}
		})
	}
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> = LazyLock::new(|| {
    PackedATNDeserializer::new()
        .deserialize(&_serializedATN)
        .expect("invalid packed parser ATN")
});
static _serializedATN: [u32; 4213] = [
    1346458702, 3, 16909060, 29, 33, 154, 177, 2, 2, 14, 5, 29, 1078, 1107, 
    885, 1992, 10, 2002, 4, 2014, 14, 2028, 5, 2033, 5, 4213, 4, 2006, 8, 
    2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 
    4294967295, 1, 0, 8, 5, 1, 4294967295, 4294967295, 4, 0, 8, 6, 1, 12, 
    4294967295, 8, 0, 8, 7, 1, 4294967295, 4294967295, 11, 0, 8, 8, 2, 4294967295, 
    4294967295, 12, 0, 8, 10, 1, 4294967295, 13, 1, 0, 32, 11, 1, 4294967295, 
    4294967295, 1, 0, 8, 12, 1, 4294967295, 4294967295, 1, 0, 32, 13, 1, 
    4294967295, 4294967295, 1, 0, 8, 14, 1, 4294967295, 4294967295, 1, 0, 
    32, 15, 1, 4294967295, 4294967295, 1, 0, 32, 16, 1, 4294967295, 4294967295, 
    1, 0, 32, 17, 1, 4294967295, 4294967295, 1, 0, 8, 18, 1, 4294967295, 
    4294967295, 3, 0, 8, 19, 2, 24, 4294967295, 8, 0, 8, 21, 1, 4294967295, 
    4294967295, 1, 1, 72, 22, 1, 4294967295, 4294967295, 1, 1, 8, 23, 1, 
    4294967295, 4294967295, 1, 1, 32, 24, 1, 4294967295, 4294967295, 1, 
    1, 8, 25, 1, 4294967295, 4294967295, 3, 1, 8, 26, 2, 30, 4294967295, 
    8, 1, 8, 28, 1, 4294967295, 4294967295, 1, 1, 72, 29, 1, 4294967295, 
    4294967295, 1, 1, 32, 30, 1, 4294967295, 4294967295, 1, 1, 8, 31, 1, 
    4294967295, 4294967295, 1, 1, 72, 32, 1, 4294967295, 4294967295, 1, 
    1, 32, 33, 1, 4294967295, 4294967295, 1, 1, 8, 34, 1, 4294967295, 4294967295, 
    1, 1, 72, 35, 1, 4294967295, 4294967295, 1, 1, 32, 36, 1, 4294967295, 
    4294967295, 1, 1, 8, 37, 1, 4294967295, 4294967295, 1, 1, 72, 38, 1, 
    4294967295, 4294967295, 1, 1, 32, 39, 1, 4294967295, 4294967295, 1, 
    1, 8, 40, 1, 4294967295, 4294967295, 5, 1, 8, 41, 4, 44, 4294967295, 
    8, 1, 8, 45, 1, 4294967295, 4294967295, 10, 1, 10, 46, 2, 4294967295, 
    4294967295, 12, 1, 8, 48, 1, 4294967295, 47, 9, 1, 8, 49, 1, 4294967295, 
    4294967295, 1, 2, 32, 50, 1, 4294967295, 4294967295, 1, 2, 32, 51, 1, 
    4294967295, 4294967295, 1, 2, 32, 52, 1, 4294967295, 4294967295, 1, 
    2, 8, 53, 1, 4294967295, 4294967295, 1, 2, 32, 54, 1, 4294967295, 4294967295, 
    1, 2, 8, 55, 1, 4294967295, 4294967295, 1, 2, 32, 56, 1, 4294967295, 
    4294967295, 1, 2, 32, 57, 1, 4294967295, 4294967295, 1, 2, 8, 58, 1, 
    4294967295, 4294967295, 1, 2, 32, 59, 1, 4294967295, 4294967295, 1, 
    2, 8, 60, 1, 4294967295, 4294967295, 1, 2, 32, 61, 1, 4294967295, 4294967295, 
    1, 2, 8, 62, 1, 4294967295, 4294967295, 1, 2, 32, 63, 1, 4294967295, 
    4294967295, 1, 2, 32, 64, 1, 4294967295, 4294967295, 1, 2, 8, 65, 1, 
    4294967295, 4294967295, 1, 2, 32, 66, 1, 4294967295, 4294967295, 1, 
    2, 8, 67, 1, 4294967295, 4294967295, 1, 2, 32, 68, 1, 4294967295, 4294967295, 
    1, 2, 8, 69, 1, 4294967295, 4294967295, 1, 2, 32, 70, 1, 4294967295, 
    4294967295, 1, 2, 8, 71, 1, 4294967295, 4294967295, 4, 2, 8, 72, 1, 
    71, 4294967295, 8, 2, 8, 73, 1, 4294967295, 4294967295, 11, 2, 8, 74, 
    2, 4294967295, 4294967295, 12, 2, 8, 76, 1, 4294967295, 72, 1, 2, 32, 
    77, 1, 4294967295, 4294967295, 1, 2, 8, 78, 1, 4294967295, 4294967295, 
    3, 2, 8, 79, 2, 77, 4294967295, 8, 2, 8, 81, 1, 4294967295, 4294967295, 
    1, 2, 32, 82, 1, 4294967295, 4294967295, 1, 2, 8, 83, 1, 4294967295, 
    4294967295, 1, 2, 32, 84, 1, 4294967295, 4294967295, 1, 2, 32, 85, 1, 
    4294967295, 4294967295, 1, 2, 8, 86, 1, 4294967295, 4294967295, 1, 2, 
    32, 87, 1, 4294967295, 4294967295, 1, 2, 8, 88, 1, 4294967295, 4294967295, 
    5, 2, 8, 89, 1, 86, 4294967295, 8, 2, 8, 90, 1, 4294967295, 4294967295, 
    10, 2, 8, 91, 2, 4294967295, 4294967295, 12, 2, 8, 93, 1, 4294967295, 
    89, 9, 2, 8, 94, 1, 4294967295, 4294967295, 3, 2, 8, 95, 2, 91, 4294967295, 
    8, 2, 8, 97, 1, 4294967295, 4294967295, 1, 2, 32, 98, 1, 4294967295, 
    4294967295, 1, 2, 32, 99, 1, 4294967295, 4294967295, 1, 2, 32, 100, 
    1, 4294967295, 4294967295, 1, 2, 32, 101, 1, 4294967295, 4294967295, 
    1, 2, 32, 102, 1, 4294967295, 4294967295, 1, 2, 32, 103, 1, 4294967295, 
    4294967295, 1, 2, 32, 104, 1, 4294967295, 4294967295, 1, 2, 32, 105, 
    1, 4294967295, 4294967295, 1, 2, 8, 106, 1, 4294967295, 4294967295, 
    1, 2, 32, 107, 1, 4294967295, 4294967295, 1, 2, 8, 108, 1, 4294967295, 
    4294967295, 1, 2, 32, 109, 1, 4294967295, 4294967295, 1, 2, 32, 110, 
    1, 4294967295, 4294967295, 1, 2, 8, 111, 1, 4294967295, 4294967295, 
    1, 2, 32, 112, 1, 4294967295, 4294967295, 3, 2, 8, 113, 2, 108, 4294967295, 
    8, 2, 8, 115, 1, 4294967295, 4294967295, 1, 2, 8, 116, 1, 4294967295, 
    4294967295, 1, 2, 32, 117, 1, 4294967295, 4294967295, 1, 2, 8, 118, 
    1, 4294967295, 4294967295, 3, 2, 8, 119, 2, 113, 4294967295, 8, 2, 8, 
    121, 1, 4294967295, 4294967295, 1, 2, 32, 122, 1, 4294967295, 4294967295, 
    1, 2, 8, 123, 1, 4294967295, 4294967295, 1, 2, 32, 124, 1, 4294967295, 
    4294967295, 1, 2, 32, 125, 1, 4294967295, 4294967295, 1, 2, 8, 126, 
    1, 4294967295, 4294967295, 1, 2, 32, 127, 1, 4294967295, 4294967295, 
    1, 2, 8, 128, 1, 4294967295, 4294967295, 1, 2, 32, 129, 1, 4294967295, 
    4294967295, 1, 2, 8, 130, 1, 4294967295, 4294967295, 1, 2, 32, 131, 
    1, 4294967295, 4294967295, 1, 2, 32, 132, 1, 4294967295, 4294967295, 
    1, 2, 8, 133, 1, 4294967295, 4294967295, 1, 2, 32, 134, 1, 4294967295, 
    4294967295, 1, 2, 8, 135, 1, 4294967295, 4294967295, 5, 2, 8, 136, 1, 
    129, 4294967295, 8, 2, 8, 137, 1, 4294967295, 4294967295, 10, 2, 8, 
    138, 2, 4294967295, 4294967295, 12, 2, 8, 140, 1, 4294967295, 132, 9, 
    2, 8, 141, 1, 4294967295, 4294967295, 1, 2, 32, 142, 1, 4294967295, 
    4294967295, 1, 2, 8, 143, 1, 4294967295, 4294967295, 1, 2, 32, 144, 
    1, 4294967295, 4294967295, 1, 2, 32, 145, 1, 4294967295, 4294967295, 
    1, 2, 32, 146, 1, 4294967295, 4294967295, 1, 2, 32, 147, 1, 4294967295, 
    4294967295, 1, 2, 32, 148, 1, 4294967295, 4294967295, 3, 2, 8, 149, 
    14, 141, 4294967295, 8, 2, 8, 163, 1, 4294967295, 4294967295, 1, 3, 
    32, 164, 1, 4294967295, 4294967295, 1, 3, 32, 165, 1, 4294967295, 4294967295, 
    1, 3, 32, 166, 1, 4294967295, 4294967295, 1, 3, 32, 167, 1, 4294967295, 
    4294967295, 3, 3, 8, 168, 3, 147, 4294967295, 8, 3, 8, 171, 1, 4294967295, 
    4294967295, 1, 4, 32, 172, 1, 4294967295, 4294967295, 1, 4, 8, 173, 
    1, 4294967295, 4294967295, 1, 4, 32, 174, 1, 4294967295, 4294967295, 
    1, 4, 8, 175, 1, 4294967295, 4294967295, 1, 4, 8, 176, 1, 4294967295, 
    4294967295, 1, 4, 0, 177, 0, 4294967295, 4294967295, 1, 23, 0, 0, 0, 
    1, 29, 0, 0, 0, 1, 140, 0, 0, 0, 1, 146, 0, 0, 0, 1, 148, 0, 0, 0, 3, 
    2, 1, 12, 0, 1, 10, 0, 0, 0, 1, 13, 0, 0, 0, 1, 11, 0, 0, 0, 1, 14, 
    0, 0, 0, 1, 15, 0, 0, 0, 5, 16, 4294967295, 0, 0, 1, 24, 0, 0, 0, 5, 
    18, 1, 0, 0, 3, 2, 1, 19, 0, 5, 20, 2, 0, 0, 5, 21, 32, 0, 0, 5, 22, 
    4294967295, 0, 0, 1, 24, 0, 0, 0, 1, 11, 0, 0, 0, 1, 17, 0, 0, 0, 1, 
    1, 0, 0, 0, 6, 26, 1, 4294967295, 0, 3, 4, 2, 30, 0, 7, 28, 0, 0, 0, 
    3, 2, 1, 30, 5, 1, 25, 0, 0, 0, 1, 27, 0, 0, 0, 1, 45, 0, 0, 0, 10, 
    32, 4, 0, 0, 7, 33, 1, 0, 0, 3, 2, 1, 44, 5, 10, 35, 3, 0, 0, 7, 36, 
    0, 0, 0, 3, 2, 1, 44, 4, 10, 38, 2, 0, 0, 5, 39, 2, 0, 0, 3, 2, 1, 44, 
    3, 10, 41, 1, 0, 0, 5, 42, 7, 0, 0, 3, 2, 1, 44, 2, 1, 31, 0, 0, 0, 
    1, 34, 0, 0, 0, 1, 37, 0, 0, 0, 1, 40, 0, 0, 0, 1, 47, 0, 0, 0, 1, 43, 
    0, 0, 0, 1, 46, 0, 0, 0, 1, 3, 0, 0, 0, 1, 45, 0, 0, 0, 5, 141, 32, 
    0, 0, 5, 141, 31, 0, 0, 5, 51, 8, 0, 0, 3, 2, 1, 52, 0, 5, 53, 9, 0, 
    0, 1, 141, 0, 0, 0, 5, 55, 13, 0, 0, 5, 56, 8, 0, 0, 3, 2, 1, 57, 0, 
    5, 58, 12, 0, 0, 3, 6, 3, 59, 0, 5, 60, 9, 0, 0, 1, 141, 0, 0, 0, 5, 
    62, 14, 0, 0, 5, 63, 8, 0, 0, 3, 2, 1, 64, 0, 5, 65, 12, 0, 0, 3, 6, 
    3, 66, 0, 5, 67, 9, 0, 0, 1, 141, 0, 0, 0, 5, 70, 15, 0, 0, 3, 8, 4, 
    71, 0, 1, 69, 0, 0, 0, 1, 72, 0, 0, 0, 1, 70, 0, 0, 0, 1, 73, 0, 0, 
    0, 1, 76, 0, 0, 0, 5, 75, 18, 0, 0, 3, 2, 1, 77, 0, 1, 74, 0, 0, 0, 
    1, 77, 0, 0, 0, 1, 78, 0, 0, 0, 5, 79, 19, 0, 0, 1, 141, 0, 0, 0, 5, 
    81, 31, 0, 0, 5, 90, 8, 0, 0, 3, 2, 1, 87, 0, 5, 84, 10, 0, 0, 3, 2, 
    1, 86, 0, 1, 83, 0, 0, 0, 1, 89, 0, 0, 0, 1, 85, 0, 0, 0, 1, 88, 0, 
    0, 0, 1, 91, 0, 0, 0, 1, 87, 0, 0, 0, 1, 82, 0, 0, 0, 1, 91, 0, 0, 0, 
    1, 92, 0, 0, 0, 5, 141, 9, 0, 0, 5, 94, 31, 0, 0, 5, 95, 11, 0, 0, 5, 
    141, 31, 0, 0, 5, 97, 20, 0, 0, 5, 98, 8, 0, 0, 5, 99, 31, 0, 0, 5, 
    100, 26, 0, 0, 3, 2, 1, 101, 0, 5, 102, 9, 0, 0, 1, 141, 0, 0, 0, 5, 
    104, 21, 0, 0, 5, 105, 8, 0, 0, 3, 2, 1, 107, 0, 5, 108, 26, 0, 0, 1, 
    106, 0, 0, 0, 1, 108, 0, 0, 0, 1, 109, 0, 0, 0, 3, 2, 1, 112, 0, 5, 
    111, 27, 0, 0, 3, 2, 1, 113, 0, 1, 110, 0, 0, 0, 1, 113, 0, 0, 0, 1, 
    114, 0, 0, 0, 5, 115, 9, 0, 0, 1, 141, 0, 0, 0, 5, 117, 22, 0, 0, 5, 
    118, 8, 0, 0, 3, 2, 1, 119, 0, 5, 120, 28, 0, 0, 3, 2, 1, 121, 0, 5, 
    122, 9, 0, 0, 1, 141, 0, 0, 0, 5, 124, 23, 0, 0, 5, 125, 8, 0, 0, 3, 
    2, 1, 130, 0, 5, 127, 10, 0, 0, 3, 2, 1, 129, 0, 1, 126, 0, 0, 0, 1, 
    132, 0, 0, 0, 1, 128, 0, 0, 0, 1, 131, 0, 0, 0, 1, 133, 0, 0, 0, 1, 
    130, 0, 0, 0, 5, 134, 9, 0, 0, 1, 141, 0, 0, 0, 5, 136, 24, 0, 0, 5, 
    141, 30, 0, 0, 5, 138, 25, 0, 0, 5, 139, 30, 0, 0, 5, 141, 31, 0, 0, 
    1, 48, 0, 0, 0, 1, 49, 0, 0, 0, 1, 50, 0, 0, 0, 1, 54, 0, 0, 0, 1, 61, 
    0, 0, 0, 1, 68, 0, 0, 0, 1, 80, 0, 0, 0, 1, 93, 0, 0, 0, 1, 96, 0, 0, 
    0, 1, 103, 0, 0, 0, 1, 116, 0, 0, 0, 1, 123, 0, 0, 0, 1, 135, 0, 0, 
    0, 1, 137, 0, 0, 0, 1, 5, 0, 0, 0, 5, 147, 31, 0, 0, 5, 144, 31, 0, 
    0, 5, 147, 29, 0, 0, 5, 147, 32, 0, 0, 1, 142, 0, 0, 0, 1, 143, 0, 0, 
    0, 1, 145, 0, 0, 0, 1, 7, 0, 0, 0, 5, 149, 16, 0, 0, 3, 2, 1, 150, 0, 
    5, 151, 17, 0, 0, 3, 2, 1, 152, 0, 1, 9, 0, 0, 0, 0, 1, 1, 0, 2, 1, 
    1, 1, 2, 2, 3, 4, 5, 6, 24, 0, 0, 0, 96, 0, 0, 0, 13, 23, 29, 43, 45, 
    72, 76, 87, 90, 107, 112, 130, 140, 146, 0, 2, 4, 6, 8, 1, 3, 5, 7, 
    9, 1396983361, 2, 2175, 8, 14, 3, 5, 9, 2, 12, 0, 3, 2, 1, 4, 0, 0, 
    0, 0, 0, 0, 1, 1, 1, 1, 0, 12, 12, 2, 2, 1, 3, 4, 2, 5, 6, 3, 7, 7, 
    4, 0, 0, 0, 0, 0, 4294967295, 76, 34, 5, 234, 0, 2, 0, 0, 1, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 0, 4294967293, 4294967293, 4294967293, 4294967293, 
    4294967293, 4294967293, 4294967293, 4294967293, 4294967293, 4294967293, 
    4294967293, 4294967293, 4294967293, 4294967293, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    0, 1, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 0, 78, 78, 117, 156, 234, 4294967295, 4294967295, 
    1, 2, 2, 2, 3, 4, 3, 5, 7, 4, 8, 8, 5, 9, 9, 6, 10, 10, 7, 12, 12, 8, 
    13, 13, 9, 14, 14, 10, 15, 15, 11, 16, 16, 12, 17, 17, 13, 18, 18, 14, 
    19, 19, 6, 20, 20, 15, 21, 21, 16, 22, 22, 17, 23, 23, 18, 24, 24, 19, 
    25, 25, 20, 26, 26, 21, 27, 27, 22, 28, 28, 23, 31, 31, 24, 32, 32, 
    25, 3, 4, 26, 8, 8, 27, 13, 13, 28, 14, 14, 29, 15, 15, 30, 20, 20, 
    31, 21, 21, 32, 22, 22, 33, 23, 23, 34, 24, 24, 35, 25, 25, 36, 31, 
    31, 37, 32, 32, 38, 3, 4, 39, 8, 8, 40, 13, 13, 41, 14, 14, 42, 15, 
    15, 43, 20, 20, 44, 21, 21, 45, 22, 22, 46, 23, 23, 47, 24, 24, 48, 
    25, 25, 49, 31, 31, 50, 32, 32, 51, 4294967295, 4294967295, 52, 2, 2, 
    53, 3, 4, 54, 5, 7, 4, 8, 8, 55, 9, 9, 56, 10, 10, 57, 12, 12, 58, 13, 
    13, 59, 14, 14, 60, 15, 15, 61, 16, 16, 62, 17, 17, 63, 18, 18, 64, 
    19, 19, 56, 20, 20, 65, 21, 21, 66, 22, 22, 67, 23, 23, 68, 24, 24, 
    69, 25, 25, 70, 26, 26, 71, 27, 27, 72, 28, 28, 73, 31, 31, 74, 32, 
    32, 75, 0, 0, 14, 39, 1, 40, 1, 41, 1, 42, 1, 43, 1, 44, 1, 45, 1, 46, 
    1, 47, 1, 48, 1, 49, 1, 50, 1, 51, 1, 52, 1, 3, 10, 18, 105, 7, 26, 
    28, 33, 36, 39, 42, 151, 4294967295, 77, 34, 5, 237, 0, 2, 0, 0, 1, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 4294967293, 4294967293, 4294967293, 
    4294967293, 4294967293, 4294967293, 4294967293, 4294967293, 4294967293, 
    4294967293, 4294967293, 4294967293, 4294967293, 4294967293, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 
    2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 0, 1, 2, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 81, 81, 120, 159, 237, 4294967295, 
    4294967295, 1, 2, 2, 2, 3, 4, 3, 5, 6, 4, 7, 7, 5, 8, 8, 6, 9, 9, 7, 
    10, 10, 8, 12, 12, 9, 13, 13, 10, 14, 14, 11, 15, 15, 12, 16, 16, 13, 
    17, 17, 14, 18, 18, 15, 19, 19, 7, 20, 20, 16, 21, 21, 17, 22, 22, 18, 
    23, 23, 19, 24, 24, 20, 25, 25, 21, 26, 26, 22, 27, 27, 23, 28, 28, 
    24, 31, 31, 25, 32, 32, 26, 3, 4, 27, 8, 8, 28, 13, 13, 29, 14, 14, 
    30, 15, 15, 31, 20, 20, 32, 21, 21, 33, 22, 22, 34, 23, 23, 35, 24, 
    24, 36, 25, 25, 37, 31, 31, 38, 32, 32, 39, 3, 4, 40, 8, 8, 41, 13, 
    13, 42, 14, 14, 43, 15, 15, 44, 20, 20, 45, 21, 21, 46, 22, 22, 47, 
    23, 23, 48, 24, 24, 49, 25, 25, 50, 31, 31, 51, 32, 32, 52, 4294967295, 
    4294967295, 53, 2, 2, 54, 3, 4, 55, 5, 7, 4, 8, 8, 56, 9, 9, 57, 10, 
    10, 58, 12, 12, 59, 13, 13, 60, 14, 14, 61, 15, 15, 62, 16, 16, 63, 
    17, 17, 64, 18, 18, 65, 19, 19, 57, 20, 20, 66, 21, 21, 67, 22, 22, 
    68, 23, 23, 69, 24, 24, 70, 25, 25, 71, 26, 26, 72, 27, 27, 73, 28, 
    28, 74, 31, 31, 75, 32, 32, 76, 0, 0, 14, 40, 1, 41, 1, 42, 1, 43, 1, 
    44, 1, 45, 1, 46, 1, 47, 1, 48, 1, 49, 1, 50, 1, 51, 1, 52, 1, 53, 1, 
    3, 10, 18, 105, 7, 26, 28, 33, 36, 39, 42, 151, 4294967295, 48, 34, 
    14, 150, 0, 2, 2, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 4294967293, 4294967293, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    4294967293, 4294967293, 4294967293, 4294967293, 4294967293, 0, 4294967293, 
    4294967293, 4294967293, 0, 4294967293, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 2, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3, 4, 5, 
    6, 7, 8, 9, 10, 11, 1, 1, 1, 1, 1, 12, 1, 1, 1, 13, 1, 0, 78, 78, 117, 
    120, 123, 126, 129, 132, 135, 138, 141, 144, 147, 150, 4294967295, 4294967295, 
    1, 2, 2, 2, 3, 4, 3, 5, 6, 4, 8, 8, 5, 9, 9, 6, 10, 10, 7, 12, 12, 8, 
    13, 13, 9, 14, 14, 10, 15, 15, 11, 16, 16, 12, 17, 17, 13, 18, 18, 14, 
    19, 19, 6, 20, 20, 15, 21, 21, 16, 22, 22, 17, 23, 23, 18, 24, 24, 19, 
    25, 25, 20, 26, 26, 21, 27, 27, 22, 28, 28, 23, 31, 31, 24, 32, 32, 
    25, 3, 4, 26, 8, 8, 27, 13, 13, 28, 14, 14, 29, 15, 15, 30, 20, 20, 
    31, 21, 21, 32, 22, 22, 33, 23, 23, 34, 24, 24, 35, 25, 25, 36, 31, 
    31, 37, 32, 32, 38, 8, 8, 39, 8, 8, 40, 16, 16, 41, 8, 8, 42, 8, 8, 
    43, 8, 8, 44, 8, 8, 45, 30, 30, 38, 30, 30, 46, 31, 31, 47, 31, 31, 
    38, 0, 0, 11, 26, 1, 27, 1, 37, 1, 38, 1, 39, 1, 40, 1, 41, 1, 43, 1, 
    44, 1, 45, 1, 47, 1, 2, 10, 105, 7, 26, 28, 33, 36, 39, 42, 151, 4294967295, 
    26, 34, 2, 78, 0, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 78, 78, 4294967295, 4294967295, 
    1, 2, 2, 2, 3, 4, 3, 5, 6, 4, 8, 8, 5, 9, 9, 6, 10, 10, 7, 12, 12, 8, 
    13, 13, 9, 14, 14, 10, 15, 15, 11, 16, 16, 12, 17, 17, 13, 18, 18, 14, 
    19, 19, 6, 20, 20, 15, 21, 21, 16, 22, 22, 17, 23, 23, 18, 24, 24, 19, 
    25, 25, 20, 26, 26, 21, 27, 27, 22, 28, 28, 23, 31, 31, 24, 32, 32, 
    25, 0, 0, 0, 0, 0, 4294967295, 1, 1, 1, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 
    0, 12, 69, 34, 14, 267, 0, 3, 4, 5, 6, 9, 10, 11, 12, 13, 14, 0, 1, 
    2, 2, 2, 2, 0, 2, 2, 8, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 4294967295, 4294967295, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    2, 1, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 7, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    2, 1, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 36, 36, 117, 159, 162, 165, 
    168, 171, 174, 177, 180, 183, 186, 267, 8, 8, 1, 13, 13, 2, 14, 14, 
    3, 15, 15, 4, 20, 20, 5, 21, 21, 6, 22, 22, 7, 23, 23, 8, 24, 24, 9, 
    25, 25, 10, 31, 31, 11, 32, 32, 12, 4294967295, 4294967295, 13, 2, 2, 
    14, 3, 4, 15, 5, 7, 16, 8, 8, 17, 9, 9, 18, 10, 10, 19, 11, 11, 20, 
    12, 12, 21, 13, 13, 22, 14, 14, 23, 15, 15, 24, 16, 16, 25, 17, 17, 
    26, 18, 18, 27, 19, 19, 18, 20, 20, 28, 21, 21, 29, 22, 22, 30, 23, 
    23, 31, 24, 24, 32, 25, 25, 33, 26, 26, 34, 27, 27, 35, 28, 28, 36, 
    31, 31, 37, 32, 32, 38, 3, 4, 39, 8, 8, 40, 9, 9, 41, 13, 13, 42, 14, 
    14, 43, 15, 15, 44, 20, 20, 45, 21, 21, 46, 22, 22, 47, 23, 23, 48, 
    24, 24, 49, 25, 25, 50, 31, 31, 51, 32, 32, 52, 8, 8, 53, 8, 8, 54, 
    16, 16, 55, 8, 8, 56, 8, 8, 57, 8, 8, 58, 8, 8, 59, 30, 30, 52, 30, 
    30, 60, 4294967295, 4294967295, 61, 2, 2, 62, 3, 4, 63, 5, 7, 64, 8, 
    8, 65, 9, 9, 66, 10, 10, 67, 11, 11, 68, 12, 12, 21, 13, 13, 22, 14, 
    14, 23, 15, 15, 24, 16, 16, 25, 17, 17, 26, 18, 18, 27, 19, 19, 18, 
    20, 20, 28, 21, 21, 29, 22, 22, 30, 23, 23, 31, 24, 24, 32, 25, 25, 
    33, 26, 26, 34, 27, 27, 35, 28, 28, 36, 31, 31, 37, 32, 32, 38, 0, 0, 
    0, 0, 0, 13, 6, 34, 3, 15, 0, 0, 3, 1, 1, 2, 0, 1, 3, 1, 1, 2, 0, 1, 
    2, 2, 2, 2, 0, 6, 15, 15, 31, 31, 1, 32, 32, 2, 4294967295, 4294967295, 
    3, 9, 9, 4, 29, 29, 5, 0, 0, 0, 0, 0, 1, 4, 4, 1, 2, 3, 4, 1, 2, 3, 
    4, 5
];