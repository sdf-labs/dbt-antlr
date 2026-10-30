// Generated from LoopSibling.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const LoopSibling_T__0:i32=1; 
pub const LoopSibling_T__1:i32=2; 
pub const LoopSibling_T__2:i32=3; 
pub const LoopSibling_T__3:i32=4; 
pub const LoopSibling_T__4:i32=5; 
pub const LoopSibling_T__5:i32=6; 
pub const LoopSibling_T__6:i32=7; 
pub const LoopSibling_T__7:i32=8; 
pub const LoopSibling_ID:i32=9; 
pub const LoopSibling_WS:i32=10;
pub const LoopSibling_EOF:i32=EOF;
pub const RULE_start:usize = 0; 
pub const RULE_expr:usize = 1; 
pub const RULE_term:usize = 2; 
pub const RULE_cond:usize = 3; 
pub const RULE_tail:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "start", "expr", "term", "cond", "tail"
];

pub const _LITERAL_NAMES: [Option<&'static str>;9] = [
	None, Some("';'"), Some("'-'"), Some("'with'"), Some("'if'"), Some("'opt'"), 
	Some("'!'"), Some("'pair'"), Some("'?'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;11]  = [
	None, None, None, None, None, None, None, None, None, Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, LoopSiblingParserExt<'input, 'arena>, LoopSiblingParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				LoopSiblingParserExt {
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
        L: ParseTreeListener<'arena, LoopSiblingParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct LoopSiblingTreeWalker;
impl LoopSiblingTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, LoopSiblingParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, LoopSiblingParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <LoopSiblingParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LoopSiblingParserNodeKind {
    StartContext,
    ExprContext,
    TermContext,
    CondContext,
    TailContext,
    Terminal,
    Error,
}
pub type LoopSiblingParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LoopSiblingParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => LoopSiblingParser }
dbt_antlr_runtime::impl_node_kind! { LoopSiblingParserNodeKind {
; StartContext(), ExprContext(), TermContext(), CondContext(), TailContext(), 
    }; 
}

pub struct LoopSiblingParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> LoopSiblingParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { LoopSiblingParserExt, LoopSiblingParserNodeKind, "LoopSibling.g4"; sempred (LoopSiblingParserNode<'input, 'arena, TF::Tok>, LoopSiblingParser) { 1 => expr_sempred,
2 => term_sempred,
3 => cond_sempred,
4 => tail_sempred, } }

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
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
		    _ => true
		}
	}

	fn term_sempred(_ctx: Option<&'arena TermContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        2 => {
			recog.precpred(None, 4)
		    }
		    _ => true
		}
	}

	fn cond_sempred(_ctx: Option<&'arena CondContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        3 => {
			recog.precpred(None, 4)
		    }
	        4 => {
			recog.precpred(None, 3)
		    }
		    _ => true
		}
	}

	fn tail_sempred(_ctx: Option<&'arena TailContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        5 => {
			recog.precpred(None, 4)
		    }
	        6 => {
			recog.precpred(None, 3)
		    }
		    _ => true
		}
	}
}
//------------------- start ----------------
pub type StartContextAll<'input, 'arena, Tok = CommonToken<'input>> = StartContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSiblingParserNodeKind, StartContext, StartContextExt, RULE_start }

pub trait StartContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn term(&self) -> Option<&'arena TermContextAll<'input, 'arena, Tok>>;
    fn cond(&self) -> Option<&'arena CondContextAll<'input, 'arena, Tok>>;
    fn tail(&self) -> Option<&'arena TailContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StartContextAttrs<'input, 'arena, Tok> for StartContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSibling_EOF)
    }
    fn term(&self) -> Option<&'arena TermContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn cond(&self) -> Option<&'arena CondContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn tail(&self) -> Option<&'arena TailContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn start(&mut self,) -> Result<&'arena StartContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StartContext<TF::Tok>, RULE_start, 0, |_parentctx| StartContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StartContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(33);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(0)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(0,&mut recog.base)? } else { _sdp } } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(10);
				recog.expr_rec(0)?;
				recog.base.set_state(11);
				recog.expr_rec(0)?;
				recog.base.set_state(12);
				recog.base.match_token(LoopSibling_T__0,&mut recog.err_handler)?;
				recog.base.set_state(13);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(15);
				recog.expr_rec(0)?;
				recog.base.set_state(16);
				recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
				recog.base.set_state(17);
				recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
				recog.base.set_state(18);
				recog.base.match_token(LoopSibling_T__0,&mut recog.err_handler)?;
				recog.base.set_state(19);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(21);
				recog.base.match_token(LoopSibling_T__2,&mut recog.err_handler)?;
				recog.base.set_state(22);
				recog.term_rec(0)?;
				recog.base.set_state(23);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(25);
				recog.base.match_token(LoopSibling_T__3,&mut recog.err_handler)?;
				recog.base.set_state(26);
				recog.cond_rec(0)?;
				recog.base.set_state(27);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			5 =>{
				/*------- Outer Most Alt 5 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
				{
				recog.base.set_state(29);
				recog.base.match_token(LoopSibling_T__4,&mut recog.err_handler)?;
				recog.base.set_state(30);
				recog.tail_rec(0)?;
				recog.base.set_state(31);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExprContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSiblingParserNodeKind, ExprContext, ExprContextExt, RULE_expr }

pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSibling_ID)
    }
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
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
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(39);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(36);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(37);
		        recog.expr_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(38);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(48);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(3)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(3,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(46);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(2)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(41);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(42);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(43);
						recog.expr_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(44);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(45);
						recog.base.match_token(LoopSibling_T__5,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(50);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(3)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(3,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}
//------------------- term ----------------
pub type TermContextAll<'input, 'arena, Tok = CommonToken<'input>> = TermContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSiblingParserNodeKind, TermContext, TermContextExt, RULE_term }

pub trait TermContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn term_all(&self) -> Vec<&'arena TermContextAll<'input, 'arena, Tok>>;
    fn term(&self, i: usize) -> Option<&'arena TermContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TermContextAttrs<'input, 'arena, Tok> for TermContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn term_all(&self) -> Vec<&'arena TermContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn term(&self, i: usize) -> Option<&'arena TermContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSibling_ID)
    }
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  term(&mut self,) -> Result<&'arena TermContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.term_rec(0)
	}

	fn term_rec(&mut self, _p: i32) -> Result<&'arena TermContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, TermContext<TF::Tok>, RULE_term, 4, |_parentctx| TermContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena TermContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 4;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(59);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__6  => {
		        {
		        recog.base.set_state(52);
		        recog.base.match_token(LoopSibling_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(53);
		        recog.term_rec(0)?;
		        recog.base.set_state(54);
		        recog.term_rec(3)?;
		        }}
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(56);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(57);
		        recog.term_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(58);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(66);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(5)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(5,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				{
				/*recRuleAltStartAction*/
				let tmp = TermContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
				let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_term)?;

				recog.base.set_state(61);
				if !({recog.precpred(None, 4)}) {
					Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
				}
				recog.base.set_state(62);
				recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
				recog.base.set_state(63);
				recog.term_rec(5)?;
				}
				} 
			}
			recog.base.set_state(68);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(5)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(5,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}
//------------------- cond ----------------
pub type CondContextAll<'input, 'arena, Tok = CommonToken<'input>> = CondContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSiblingParserNodeKind, CondContext, CondContextExt, RULE_cond }

pub trait CondContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn cond_all(&self) -> Vec<&'arena CondContextAll<'input, 'arena, Tok>>;
    fn cond(&self, i: usize) -> Option<&'arena CondContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> CondContextAttrs<'input, 'arena, Tok> for CondContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn cond_all(&self) -> Vec<&'arena CondContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn cond(&self, i: usize) -> Option<&'arena CondContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSibling_ID)
    }
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  cond(&mut self,) -> Result<&'arena CondContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.cond_rec(0)
	}

	fn cond_rec(&mut self, _p: i32) -> Result<&'arena CondContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, CondContext<TF::Tok>, RULE_cond, 6, |_parentctx| CondContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena CondContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 6;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(73);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(70);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(71);
		        recog.cond_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(72);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(86);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(8)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(8,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(84);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(7)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = CondContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_cond)?;

						recog.base.set_state(75);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(76);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(77);
						recog.cond_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = CondContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_cond)?;

						recog.base.set_state(78);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(79);
						recog.base.match_token(LoopSibling_T__7,&mut recog.err_handler)?;
						recog.base.set_state(80);
						recog.cond_rec(0)?;
						recog.base.set_state(81);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(82);
						recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(88);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(8)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(8,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}
//------------------- tail ----------------
pub type TailContextAll<'input, 'arena, Tok = CommonToken<'input>> = TailContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSiblingParserNodeKind, TailContext, TailContextExt, RULE_tail }

pub trait TailContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn tail_all(&self) -> Vec<&'arena TailContextAll<'input, 'arena, Tok>>;
    fn tail(&self, i: usize) -> Option<&'arena TailContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TailContextAttrs<'input, 'arena, Tok> for TailContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn tail_all(&self) -> Vec<&'arena TailContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn tail(&self, i: usize) -> Option<&'arena TailContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == LoopSibling_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == LoopSibling_ID).nth(i)
    }
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  tail(&mut self,) -> Result<&'arena TailContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.tail_rec(0)
	}

	fn tail_rec(&mut self, _p: i32) -> Result<&'arena TailContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, TailContext<TF::Tok>, RULE_tail, 8, |_parentctx| TailContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena TailContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 8;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(93);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(90);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(91);
		        recog.tail_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(92);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(110);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(12)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(12,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(108);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(11)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = TailContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_tail)?;

						recog.base.set_state(95);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(96);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(97);
						recog.tail_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = TailContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_tail)?;

						recog.base.set_state(98);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(99);
						recog.base.match_token(LoopSibling_T__7,&mut recog.err_handler)?;
						recog.base.set_state(100);
						recog.tail_rec(0)?;
						recog.base.set_state(106);
						recog.err_handler.sync(&mut recog.base)?;
						match { let _sdp = recog.base.dfa_predict(10)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(10,&mut recog.base)? } else { _sdp } } {
							x if x == 1 =>{
								{
								recog.base.set_state(101);
								recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
								recog.base.set_state(102);
								recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
								recog.base.set_state(103);
								recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
								recog.base.set_state(104);
								recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
								recog.base.set_state(105);
								recog.base.match_token(LoopSibling_T__5,&mut recog.err_handler)?;
								}
							}

							_ => {}
						}
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(112);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(12)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(12,&mut recog.base)? } else { _sdp } };
		}
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
static _serializedATN: [u32; 2406] = [
    1346458702, 3, 16909060, 29, 10, 114, 125, 0, 0, 13, 5, 29, 798, 827, 
    625, 1452, 0, 1452, 0, 1452, 13, 1465, 5, 1470, 5, 2406, 0, 1452, 0, 
    2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 12, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 
    4294967295, 4294967295, 2, 3, 12, 3, 1, 4294967295, 4294967295, 7, 3, 
    16, 4, 0, 4294967295, 4294967295, 2, 4, 12, 4, 1, 4294967295, 4294967295, 
    7, 4, 16, 5, 0, 4294967295, 4294967295, 1, 0, 8, 5, 1, 4294967295, 4294967295, 
    1, 0, 8, 6, 1, 4294967295, 4294967295, 1, 0, 32, 7, 1, 4294967295, 4294967295, 
    1, 0, 32, 8, 1, 4294967295, 4294967295, 1, 0, 8, 9, 1, 4294967295, 4294967295, 
    1, 0, 8, 10, 1, 4294967295, 4294967295, 1, 0, 32, 11, 1, 4294967295, 
    4294967295, 1, 0, 32, 12, 1, 4294967295, 4294967295, 1, 0, 32, 13, 1, 
    4294967295, 4294967295, 1, 0, 32, 14, 1, 4294967295, 4294967295, 1, 
    0, 8, 15, 1, 4294967295, 4294967295, 1, 0, 32, 16, 1, 4294967295, 4294967295, 
    1, 0, 8, 17, 1, 4294967295, 4294967295, 1, 0, 32, 18, 1, 4294967295, 
    4294967295, 1, 0, 8, 19, 1, 4294967295, 4294967295, 1, 0, 32, 20, 1, 
    4294967295, 4294967295, 1, 0, 8, 21, 1, 4294967295, 4294967295, 1, 0, 
    32, 22, 1, 4294967295, 4294967295, 1, 0, 8, 23, 1, 4294967295, 4294967295, 
    1, 0, 32, 24, 1, 4294967295, 4294967295, 1, 0, 8, 25, 1, 4294967295, 
    4294967295, 1, 0, 32, 26, 1, 4294967295, 4294967295, 1, 0, 8, 27, 1, 
    4294967295, 4294967295, 3, 0, 8, 28, 5, 34, 4294967295, 8, 0, 8, 33, 
    1, 4294967295, 4294967295, 1, 1, 72, 34, 1, 4294967295, 4294967295, 
    1, 1, 32, 35, 1, 4294967295, 4294967295, 1, 1, 8, 36, 1, 4294967295, 
    4294967295, 1, 1, 32, 37, 1, 4294967295, 4294967295, 3, 1, 8, 38, 2, 
    40, 4294967295, 8, 1, 8, 40, 1, 4294967295, 4294967295, 1, 1, 72, 41, 
    1, 4294967295, 4294967295, 1, 1, 32, 42, 1, 4294967295, 4294967295, 
    1, 1, 8, 43, 1, 4294967295, 4294967295, 1, 1, 72, 44, 1, 4294967295, 
    4294967295, 1, 1, 32, 45, 1, 4294967295, 4294967295, 5, 1, 8, 46, 2, 
    47, 4294967295, 8, 1, 8, 48, 1, 4294967295, 4294967295, 10, 1, 10, 49, 
    2, 4294967295, 4294967295, 12, 1, 8, 51, 1, 4294967295, 50, 9, 1, 8, 
    52, 1, 4294967295, 4294967295, 1, 2, 72, 53, 1, 4294967295, 4294967295, 
    1, 2, 32, 54, 1, 4294967295, 4294967295, 1, 2, 8, 55, 1, 4294967295, 
    4294967295, 1, 2, 8, 56, 1, 4294967295, 4294967295, 1, 2, 8, 57, 1, 
    4294967295, 4294967295, 1, 2, 32, 58, 1, 4294967295, 4294967295, 1, 
    2, 8, 59, 1, 4294967295, 4294967295, 1, 2, 32, 60, 1, 4294967295, 4294967295, 
    3, 2, 8, 61, 3, 60, 4294967295, 8, 2, 8, 64, 1, 4294967295, 4294967295, 
    1, 2, 72, 65, 1, 4294967295, 4294967295, 1, 2, 32, 66, 1, 4294967295, 
    4294967295, 1, 2, 8, 67, 1, 4294967295, 4294967295, 5, 2, 8, 68, 1, 
    65, 4294967295, 8, 2, 8, 69, 1, 4294967295, 4294967295, 10, 2, 10, 70, 
    2, 4294967295, 4294967295, 12, 2, 8, 72, 1, 4294967295, 68, 9, 2, 8, 
    73, 1, 4294967295, 4294967295, 1, 3, 72, 74, 1, 4294967295, 4294967295, 
    1, 3, 32, 75, 1, 4294967295, 4294967295, 1, 3, 8, 76, 1, 4294967295, 
    4294967295, 1, 3, 32, 77, 1, 4294967295, 4294967295, 3, 3, 8, 78, 2, 
    74, 4294967295, 8, 3, 8, 80, 1, 4294967295, 4294967295, 1, 3, 72, 81, 
    1, 4294967295, 4294967295, 1, 3, 32, 82, 1, 4294967295, 4294967295, 
    1, 3, 8, 83, 1, 4294967295, 4294967295, 1, 3, 72, 84, 1, 4294967295, 
    4294967295, 1, 3, 32, 85, 1, 4294967295, 4294967295, 1, 3, 8, 86, 1, 
    4294967295, 4294967295, 1, 3, 32, 87, 1, 4294967295, 4294967295, 1, 
    3, 32, 88, 1, 4294967295, 4294967295, 1, 3, 8, 89, 1, 4294967295, 4294967295, 
    5, 3, 8, 90, 2, 85, 4294967295, 8, 3, 8, 92, 1, 4294967295, 4294967295, 
    10, 3, 10, 93, 2, 4294967295, 4294967295, 12, 3, 8, 95, 1, 4294967295, 
    88, 9, 3, 8, 96, 1, 4294967295, 4294967295, 1, 4, 72, 97, 1, 4294967295, 
    4294967295, 1, 4, 32, 98, 1, 4294967295, 4294967295, 1, 4, 8, 99, 1, 
    4294967295, 4294967295, 1, 4, 32, 100, 1, 4294967295, 4294967295, 3, 
    4, 8, 101, 2, 94, 4294967295, 8, 4, 8, 103, 1, 4294967295, 4294967295, 
    1, 4, 72, 104, 1, 4294967295, 4294967295, 1, 4, 32, 105, 1, 4294967295, 
    4294967295, 1, 4, 8, 106, 1, 4294967295, 4294967295, 1, 4, 72, 107, 
    1, 4294967295, 4294967295, 1, 4, 32, 108, 1, 4294967295, 4294967295, 
    1, 4, 8, 109, 1, 4294967295, 4294967295, 1, 4, 32, 110, 1, 4294967295, 
    4294967295, 1, 4, 32, 111, 1, 4294967295, 4294967295, 1, 4, 32, 112, 
    1, 4294967295, 4294967295, 1, 4, 32, 113, 1, 4294967295, 4294967295, 
    1, 4, 32, 114, 1, 4294967295, 4294967295, 3, 4, 8, 115, 2, 107, 4294967295, 
    8, 4, 8, 117, 1, 4294967295, 4294967295, 5, 4, 8, 118, 2, 109, 4294967295, 
    8, 4, 8, 120, 1, 4294967295, 4294967295, 10, 4, 10, 121, 2, 4294967295, 
    4294967295, 12, 4, 8, 123, 1, 4294967295, 112, 9, 4, 8, 124, 1, 4294967295, 
    4294967295, 1, 4, 0, 125, 0, 4294967295, 4294967295, 1, 33, 0, 0, 0, 
    1, 39, 0, 0, 0, 1, 59, 0, 0, 0, 1, 73, 0, 0, 0, 1, 93, 0, 0, 0, 3, 2, 
    1, 11, 0, 3, 2, 1, 12, 0, 5, 13, 1, 0, 0, 5, 14, 4294967295, 0, 0, 1, 
    34, 0, 0, 0, 3, 2, 1, 16, 0, 5, 17, 2, 0, 0, 5, 18, 2, 0, 0, 5, 19, 
    1, 0, 0, 5, 20, 4294967295, 0, 0, 1, 34, 0, 0, 0, 5, 22, 3, 0, 0, 3, 
    4, 2, 23, 0, 5, 24, 4294967295, 0, 0, 1, 34, 0, 0, 0, 5, 26, 4, 0, 0, 
    3, 6, 3, 27, 0, 5, 28, 4294967295, 0, 0, 1, 34, 0, 0, 0, 5, 30, 5, 0, 
    0, 3, 8, 4, 31, 0, 5, 32, 4294967295, 0, 0, 1, 34, 0, 0, 0, 1, 10, 0, 
    0, 0, 1, 15, 0, 0, 0, 1, 21, 0, 0, 0, 1, 25, 0, 0, 0, 1, 29, 0, 0, 0, 
    1, 1, 0, 0, 0, 6, 36, 1, 4294967295, 0, 5, 37, 2, 0, 0, 3, 2, 1, 40, 
    2, 5, 40, 9, 0, 0, 1, 35, 0, 0, 0, 1, 38, 0, 0, 0, 1, 48, 0, 0, 0, 10, 
    42, 4, 0, 0, 5, 43, 2, 0, 0, 3, 2, 1, 47, 5, 10, 45, 3, 0, 0, 5, 47, 
    6, 0, 0, 1, 41, 0, 0, 0, 1, 44, 0, 0, 0, 1, 50, 0, 0, 0, 1, 46, 0, 0, 
    0, 1, 49, 0, 0, 0, 1, 3, 0, 0, 0, 1, 48, 0, 0, 0, 6, 52, 2, 4294967295, 
    0, 5, 53, 7, 0, 0, 3, 4, 2, 54, 0, 3, 4, 2, 55, 3, 1, 60, 0, 0, 0, 5, 
    57, 2, 0, 0, 3, 4, 2, 60, 2, 5, 60, 9, 0, 0, 1, 51, 0, 0, 0, 1, 56, 
    0, 0, 0, 1, 58, 0, 0, 0, 1, 66, 0, 0, 0, 10, 62, 4, 0, 0, 5, 63, 2, 
    0, 0, 3, 4, 2, 65, 5, 1, 61, 0, 0, 0, 1, 68, 0, 0, 0, 1, 64, 0, 0, 0, 
    1, 67, 0, 0, 0, 1, 5, 0, 0, 0, 1, 66, 0, 0, 0, 6, 70, 3, 4294967295, 
    0, 5, 71, 2, 0, 0, 3, 6, 3, 74, 2, 5, 74, 9, 0, 0, 1, 69, 0, 0, 0, 1, 
    72, 0, 0, 0, 1, 86, 0, 0, 0, 10, 76, 4, 0, 0, 5, 77, 2, 0, 0, 3, 6, 
    3, 85, 5, 10, 79, 3, 0, 0, 5, 80, 8, 0, 0, 3, 6, 3, 81, 0, 5, 82, 2, 
    0, 0, 5, 83, 9, 0, 0, 1, 85, 0, 0, 0, 1, 75, 0, 0, 0, 1, 78, 0, 0, 0, 
    1, 88, 0, 0, 0, 1, 84, 0, 0, 0, 1, 87, 0, 0, 0, 1, 7, 0, 0, 0, 1, 86, 
    0, 0, 0, 6, 90, 4, 4294967295, 0, 5, 91, 2, 0, 0, 3, 8, 4, 94, 2, 5, 
    94, 9, 0, 0, 1, 89, 0, 0, 0, 1, 92, 0, 0, 0, 1, 110, 0, 0, 0, 10, 96, 
    4, 0, 0, 5, 97, 2, 0, 0, 3, 8, 4, 109, 5, 10, 99, 3, 0, 0, 5, 100, 8, 
    0, 0, 3, 8, 4, 106, 0, 5, 102, 2, 0, 0, 5, 103, 9, 0, 0, 5, 104, 2, 
    0, 0, 5, 105, 9, 0, 0, 5, 107, 6, 0, 0, 1, 101, 0, 0, 0, 1, 107, 0, 
    0, 0, 1, 109, 0, 0, 0, 1, 95, 0, 0, 0, 1, 98, 0, 0, 0, 1, 112, 0, 0, 
    0, 1, 108, 0, 0, 0, 1, 111, 0, 0, 0, 1, 9, 0, 0, 0, 1, 110, 0, 0, 0, 
    33, 39, 46, 48, 59, 66, 73, 84, 86, 93, 106, 108, 110, 0, 2, 4, 6, 8, 
    1, 3, 5, 7, 9, 1396983361, 2, 931, 12, 13, 0, 19, 11, 9, 72, 0, 0, 3, 
    4, 5, 0, 4294967295, 0, 0, 1, 0, 0, 0, 4294967295, 4294967295, 2, 4294967295, 
    1, 4294967295, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 
    0, 0, 1, 2, 2, 2, 3, 2, 4, 5, 2, 6, 7, 8, 2, 2, 2, 2, 2, 2, 0, 15, 21, 
    21, 30, 39, 45, 51, 60, 72, 2, 2, 1, 3, 3, 2, 4, 4, 3, 5, 5, 4, 9, 9, 
    5, 2, 2, 6, 9, 9, 7, 2, 2, 8, 6, 6, 5, 9, 9, 9, 2, 2, 10, 6, 6, 7, 9, 
    9, 9, 2, 2, 11, 9, 9, 12, 2, 2, 13, 9, 9, 14, 1, 1, 15, 2, 2, 6, 9, 
    9, 16, 1, 1, 17, 2, 2, 18, 6, 6, 12, 9, 9, 9, 0, 0, 0, 0, 0, 2, 3, 8, 
    2, 6, 0, 1, 2, 0, 0, 2, 0, 1, 1, 0, 6, 6, 2, 2, 1, 6, 6, 2, 0, 0, 0, 
    0, 0, 4294967295, 16, 11, 6, 51, 0, 2, 2, 0, 1, 2, 0, 0, 2, 4294967295, 
    4294967293, 1, 4294967295, 0, 1, 4294967295, 2, 2, 2, 2, 1, 2, 2, 1, 
    2, 2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 1, 3, 4, 1, 1, 1, 1, 1, 5, 1, 
    1, 0, 15, 15, 21, 30, 45, 51, 4294967295, 4294967295, 1, 1, 1, 2, 2, 
    2, 3, 6, 6, 4, 9, 9, 5, 2, 2, 6, 9, 9, 7, 1, 1, 8, 2, 2, 9, 9, 9, 10, 
    4294967295, 4294967295, 11, 1, 1, 12, 2, 2, 13, 6, 6, 7, 9, 9, 14, 2, 
    2, 15, 9, 9, 7, 0, 0, 1, 10, 1, 2, 10, 15, 10, 37, 43, 54, 57, 63, 71, 
    77, 91, 97, 100, 4294967295, 15, 11, 6, 48, 0, 2, 2, 0, 2, 0, 0, 2, 
    4294967295, 4294967293, 1, 4294967295, 0, 1, 4294967295, 2, 2, 2, 2, 
    2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 3, 4, 1, 1, 1, 1, 1, 
    5, 1, 1, 0, 12, 12, 18, 27, 42, 48, 4294967295, 4294967295, 1, 1, 1, 
    2, 2, 2, 3, 9, 9, 4, 2, 2, 5, 9, 9, 6, 1, 1, 7, 2, 2, 8, 9, 9, 9, 4294967295, 
    4294967295, 10, 1, 1, 11, 2, 2, 12, 6, 6, 6, 9, 9, 13, 2, 2, 14, 9, 
    9, 6, 0, 0, 1, 9, 1, 2, 10, 15, 10, 37, 43, 54, 57, 63, 71, 77, 91, 
    97, 100, 4294967295, 1, 1, 1, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 4294967295, 
    18, 11, 8, 66, 0, 2, 0, 2, 2, 0, 0, 4294967293, 4294967295, 0, 0, 4294967295, 
    0, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2, 2, 
    2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 1, 1, 3, 4, 
    1, 1, 5, 6, 1, 7, 1, 1, 1, 1, 1, 0, 12, 12, 21, 30, 39, 48, 57, 66, 
    4294967295, 4294967295, 1, 2, 2, 2, 7, 7, 3, 9, 9, 4, 2, 2, 5, 7, 7, 
    6, 9, 9, 7, 2, 2, 8, 7, 7, 9, 9, 9, 7, 2, 2, 10, 7, 7, 11, 9, 9, 12, 
    2, 2, 8, 7, 7, 11, 9, 9, 13, 2, 2, 8, 7, 7, 14, 9, 9, 15, 2, 2, 16, 
    7, 7, 17, 9, 9, 7, 0, 0, 1, 7, 1, 1, 53, 10, 37, 43, 54, 57, 63, 71, 
    77, 91, 97, 100, 7, 3, 10, 2, 6, 0, 1, 2, 0, 0, 0, 0, 1, 1, 0, 6, 6, 
    2, 2, 1, 8, 8, 2, 0, 0, 0, 0, 0, 4294967295, 6, 11, 3, 15, 0, 2, 0, 
    1, 1, 4294967293, 2, 2, 2, 0, 0, 1, 0, 1, 2, 1, 1, 1, 0, 9, 9, 15, 4294967295, 
    4294967295, 1, 2, 2, 2, 8, 8, 3, 2, 2, 4, 9, 9, 5, 0, 0, 1, 5, 1, 1, 
    80, 10, 37, 43, 54, 57, 63, 71, 77, 91, 97, 100, 4294967295, 3, 4, 2, 
    6, 0, 2, 1, 2, 2, 0, 0, 1, 1, 0, 6, 6, 4294967295, 4294967295, 1, 2, 
    2, 2, 0, 0, 0, 0, 0, 10, 8, 11, 5, 30, 0, 2, 0, 2, 2, 0, 0, 4294967295, 
    2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 1, 1, 3, 4, 1, 0, 9, 9, 15, 24, 30, 
    4294967295, 4294967295, 1, 2, 2, 2, 8, 8, 3, 2, 2, 4, 9, 9, 5, 4294967295, 
    4294967295, 1, 2, 2, 6, 8, 8, 3, 2, 2, 4, 9, 9, 7, 0, 0, 0, 0, 0, 11, 
    3, 10, 2, 6, 0, 1, 2, 0, 0, 0, 0, 1, 1, 0, 6, 6, 2, 2, 1, 8, 8, 2, 0, 
    0, 0, 0, 0, 4294967295, 9, 11, 5, 30, 0, 2, 0, 4294967295, 4294967295, 
    0, 4294967293, 0, 4294967295, 2, 2, 2, 1, 1, 1, 1, 1, 1, 0, 1, 2, 1, 
    1, 3, 1, 4, 1, 0, 9, 9, 15, 24, 30, 4294967295, 4294967295, 1, 2, 2, 
    2, 8, 8, 3, 2, 2, 4, 9, 9, 5, 4294967295, 4294967295, 6, 2, 2, 7, 8, 
    8, 3, 2, 2, 4, 9, 9, 8, 0, 0, 1, 6, 1, 2, 30, 100, 10, 37, 43, 54, 57, 
    63, 71, 77, 91, 97, 100, 4, 3, 2, 3, 4, 2, 3, 4, 5, 1, 4, 5, 4, 8, 2, 
    3, 4, 7, 8, 4, 12, 2, 3, 4, 11, 8, 4
];