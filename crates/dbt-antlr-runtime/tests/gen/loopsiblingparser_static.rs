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
pub const LoopSibling_T__8:i32=9; 
pub const LoopSibling_T__9:i32=10; 
pub const LoopSibling_T__10:i32=11; 
pub const LoopSibling_T__11:i32=12; 
pub const LoopSibling_ID:i32=13; 
pub const LoopSibling_WS:i32=14;
pub const LoopSibling_EOF:i32=EOF;
pub const RULE_start:usize = 0; 
pub const RULE_expr:usize = 1; 
pub const RULE_term:usize = 2; 
pub const RULE_cond:usize = 3; 
pub const RULE_tail:usize = 4; 
pub const RULE_prec:usize = 5;
pub const ruleNames: [&'static str; 6] = [
    "start", "expr", "term", "cond", "tail", "prec"
];

pub const _LITERAL_NAMES: [Option<&'static str>;13] = [
	None, Some("';'"), Some("'-'"), Some("'with'"), Some("'if'"), Some("'opt'"), 
	Some("'prec'"), Some("'!'"), Some("'pair'"), Some("'?'"), Some("'+'"), 
	Some("'*'"), Some("':'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, Some("ID"), Some("WS")
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
    PrecContext,
    Terminal,
    Error,
}
pub type LoopSiblingParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LoopSiblingParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => LoopSiblingParser }
dbt_antlr_runtime::impl_node_kind! { LoopSiblingParserNodeKind {
; StartContext(), ExprContext(), TermContext(), CondContext(), TailContext(), PrecContext(), 
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
4 => tail_sempred,
5 => prec_sempred, } }

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

	fn prec_sempred(_ctx: Option<&'arena PrecContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        7 => {
			recog.precpred(None, 4)
		    }
	        8 => {
			recog.precpred(None, 3)
		    }
	        9 => {
			recog.precpred(None, 2)
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
    fn prec(&self) -> Option<&'arena PrecContextAll<'input, 'arena, Tok>>;
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
    fn prec(&self) -> Option<&'arena PrecContextAll<'input, 'arena, Tok>> {
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
		recog.base.set_state(39);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(0)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(0,&mut recog.base)? } else { _sdp } } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(12);
				recog.expr_rec(0)?;
				recog.base.set_state(13);
				recog.expr_rec(0)?;
				recog.base.set_state(14);
				recog.base.match_token(LoopSibling_T__0,&mut recog.err_handler)?;
				recog.base.set_state(15);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(17);
				recog.expr_rec(0)?;
				recog.base.set_state(18);
				recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
				recog.base.set_state(19);
				recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
				recog.base.set_state(20);
				recog.base.match_token(LoopSibling_T__0,&mut recog.err_handler)?;
				recog.base.set_state(21);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(23);
				recog.base.match_token(LoopSibling_T__2,&mut recog.err_handler)?;
				recog.base.set_state(24);
				recog.term_rec(0)?;
				recog.base.set_state(25);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(27);
				recog.base.match_token(LoopSibling_T__3,&mut recog.err_handler)?;
				recog.base.set_state(28);
				recog.cond_rec(0)?;
				recog.base.set_state(29);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			5 =>{
				/*------- Outer Most Alt 5 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
				{
				recog.base.set_state(31);
				recog.base.match_token(LoopSibling_T__4,&mut recog.err_handler)?;
				recog.base.set_state(32);
				recog.tail_rec(0)?;
				recog.base.set_state(33);
				recog.base.match_token(LoopSibling_EOF,&mut recog.err_handler)?;
				}
			}
		,
			6 =>{
				/*------- Outer Most Alt 6 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
				{
				recog.base.set_state(35);
				recog.base.match_token(LoopSibling_T__5,&mut recog.err_handler)?;
				recog.base.set_state(36);
				recog.prec_rec(0)?;
				recog.base.set_state(37);
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
		recog.base.set_state(45);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(42);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(43);
		        recog.expr_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(44);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(54);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(3)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(3,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(52);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(2)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(47);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(48);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(49);
						recog.expr_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(50);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(51);
						recog.base.match_token(LoopSibling_T__6,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(56);
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
		recog.base.set_state(65);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__7  => {
		        {
		        recog.base.set_state(58);
		        recog.base.match_token(LoopSibling_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(59);
		        recog.term_rec(0)?;
		        recog.base.set_state(60);
		        recog.term_rec(3)?;
		        }}
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(62);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(63);
		        recog.term_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(64);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(72);
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

				recog.base.set_state(67);
				if !({recog.precpred(None, 4)}) {
					Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
				}
				recog.base.set_state(68);
				recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
				recog.base.set_state(69);
				recog.term_rec(5)?;
				}
				} 
			}
			recog.base.set_state(74);
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
		recog.base.set_state(79);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(76);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(77);
		        recog.cond_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(78);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(92);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(8)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(8,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(90);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(7)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = CondContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_cond)?;

						recog.base.set_state(81);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(82);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(83);
						recog.cond_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = CondContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_cond)?;

						recog.base.set_state(84);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(85);
						recog.base.match_token(LoopSibling_T__8,&mut recog.err_handler)?;
						recog.base.set_state(86);
						recog.cond_rec(0)?;
						recog.base.set_state(87);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(88);
						recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(94);
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
		recog.base.set_state(99);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSibling_T__1  => {
		        {
		        recog.base.set_state(96);
		        recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(97);
		        recog.tail_rec(2)?;
		        }}
		    LoopSibling_ID  => {
		        {
		        recog.base.set_state(98);
		        recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(116);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(12)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(12,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(114);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(11)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = TailContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_tail)?;

						recog.base.set_state(101);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(102);
						recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
						recog.base.set_state(103);
						recog.tail_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = TailContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_tail)?;

						recog.base.set_state(104);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(105);
						recog.base.match_token(LoopSibling_T__8,&mut recog.err_handler)?;
						recog.base.set_state(106);
						recog.tail_rec(0)?;
						recog.base.set_state(112);
						recog.err_handler.sync(&mut recog.base)?;
						match { let _sdp = recog.base.dfa_predict(10)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(10,&mut recog.base)? } else { _sdp } } {
							x if x == 1 =>{
								{
								recog.base.set_state(107);
								recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
								recog.base.set_state(108);
								recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
								recog.base.set_state(109);
								recog.base.match_token(LoopSibling_T__1,&mut recog.err_handler)?;
								recog.base.set_state(110);
								recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
								recog.base.set_state(111);
								recog.base.match_token(LoopSibling_T__6,&mut recog.err_handler)?;
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
			recog.base.set_state(118);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(12)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(12,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}
//------------------- prec ----------------
pub type PrecContextAll<'input, 'arena, Tok = CommonToken<'input>> = PrecContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSiblingParserNodeKind, PrecContext, PrecContextExt, RULE_prec }

pub trait PrecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn prec_all(&self) -> Vec<&'arena PrecContextAll<'input, 'arena, Tok>>;
    fn prec(&self, i: usize) -> Option<&'arena PrecContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> PrecContextAttrs<'input, 'arena, Tok> for PrecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSibling_ID)
    }
    fn prec_all(&self) -> Vec<&'arena PrecContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn prec(&self, i: usize) -> Option<&'arena PrecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> LoopSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  prec(&mut self,) -> Result<&'arena PrecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.prec_rec(0)
	}

	fn prec_rec(&mut self, _p: i32) -> Result<&'arena PrecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, PrecContext<TF::Tok>, RULE_prec, 10, |_parentctx| PrecContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena PrecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 10;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		{
		recog.base.set_state(120);
		recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(136);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(14)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(14,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(134);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(13)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(13,&mut recog.base)? } else { _sdp } } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = PrecContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_prec)?;

						recog.base.set_state(122);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(123);
						recog.base.match_token(LoopSibling_T__9,&mut recog.err_handler)?;
						recog.base.set_state(124);
						recog.prec_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = PrecContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_prec)?;

						recog.base.set_state(125);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(126);
						recog.base.match_token(LoopSibling_T__10,&mut recog.err_handler)?;
						recog.base.set_state(127);
						recog.prec_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = PrecContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_prec)?;

						recog.base.set_state(128);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(129);
						recog.base.match_token(LoopSibling_T__9,&mut recog.err_handler)?;
						recog.base.set_state(130);
						recog.prec_rec(0)?;
						recog.base.set_state(131);
						recog.base.match_token(LoopSibling_T__11,&mut recog.err_handler)?;
						recog.base.set_state(132);
						recog.base.match_token(LoopSibling_ID,&mut recog.err_handler)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(138);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(14)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(14,&mut recog.base)? } else { _sdp } };
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
static _serializedATN: [u32; 2998] = [
    1346458702, 3, 16909060, 29, 14, 140, 154, 0, 0, 15, 6, 29, 980, 1009, 
    770, 1779, 0, 1779, 0, 1779, 15, 1794, 6, 1800, 6, 2998, 0, 1779, 0, 
    2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 12, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 
    4294967295, 4294967295, 2, 3, 12, 3, 1, 4294967295, 4294967295, 7, 3, 
    16, 4, 0, 4294967295, 4294967295, 2, 4, 12, 4, 1, 4294967295, 4294967295, 
    7, 4, 16, 5, 0, 4294967295, 4294967295, 2, 5, 12, 5, 1, 4294967295, 
    4294967295, 7, 5, 16, 6, 0, 4294967295, 4294967295, 1, 0, 8, 6, 1, 4294967295, 
    4294967295, 1, 0, 8, 7, 1, 4294967295, 4294967295, 1, 0, 32, 8, 1, 4294967295, 
    4294967295, 1, 0, 32, 9, 1, 4294967295, 4294967295, 1, 0, 8, 10, 1, 
    4294967295, 4294967295, 1, 0, 8, 11, 1, 4294967295, 4294967295, 1, 0, 
    32, 12, 1, 4294967295, 4294967295, 1, 0, 32, 13, 1, 4294967295, 4294967295, 
    1, 0, 32, 14, 1, 4294967295, 4294967295, 1, 0, 32, 15, 1, 4294967295, 
    4294967295, 1, 0, 8, 16, 1, 4294967295, 4294967295, 1, 0, 32, 17, 1, 
    4294967295, 4294967295, 1, 0, 8, 18, 1, 4294967295, 4294967295, 1, 0, 
    32, 19, 1, 4294967295, 4294967295, 1, 0, 8, 20, 1, 4294967295, 4294967295, 
    1, 0, 32, 21, 1, 4294967295, 4294967295, 1, 0, 8, 22, 1, 4294967295, 
    4294967295, 1, 0, 32, 23, 1, 4294967295, 4294967295, 1, 0, 8, 24, 1, 
    4294967295, 4294967295, 1, 0, 32, 25, 1, 4294967295, 4294967295, 1, 
    0, 8, 26, 1, 4294967295, 4294967295, 1, 0, 32, 27, 1, 4294967295, 4294967295, 
    1, 0, 8, 28, 1, 4294967295, 4294967295, 1, 0, 32, 29, 1, 4294967295, 
    4294967295, 1, 0, 8, 30, 1, 4294967295, 4294967295, 1, 0, 32, 31, 1, 
    4294967295, 4294967295, 1, 0, 8, 32, 1, 4294967295, 4294967295, 3, 0, 
    8, 33, 6, 40, 4294967295, 8, 0, 8, 39, 1, 4294967295, 4294967295, 1, 
    1, 72, 40, 1, 4294967295, 4294967295, 1, 1, 32, 41, 1, 4294967295, 4294967295, 
    1, 1, 8, 42, 1, 4294967295, 4294967295, 1, 1, 32, 43, 1, 4294967295, 
    4294967295, 3, 1, 8, 44, 2, 46, 4294967295, 8, 1, 8, 46, 1, 4294967295, 
    4294967295, 1, 1, 72, 47, 1, 4294967295, 4294967295, 1, 1, 32, 48, 1, 
    4294967295, 4294967295, 1, 1, 8, 49, 1, 4294967295, 4294967295, 1, 1, 
    72, 50, 1, 4294967295, 4294967295, 1, 1, 32, 51, 1, 4294967295, 4294967295, 
    5, 1, 8, 52, 2, 53, 4294967295, 8, 1, 8, 54, 1, 4294967295, 4294967295, 
    10, 1, 10, 55, 2, 4294967295, 4294967295, 12, 1, 8, 57, 1, 4294967295, 
    56, 9, 1, 8, 58, 1, 4294967295, 4294967295, 1, 2, 72, 59, 1, 4294967295, 
    4294967295, 1, 2, 32, 60, 1, 4294967295, 4294967295, 1, 2, 8, 61, 1, 
    4294967295, 4294967295, 1, 2, 8, 62, 1, 4294967295, 4294967295, 1, 2, 
    8, 63, 1, 4294967295, 4294967295, 1, 2, 32, 64, 1, 4294967295, 4294967295, 
    1, 2, 8, 65, 1, 4294967295, 4294967295, 1, 2, 32, 66, 1, 4294967295, 
    4294967295, 3, 2, 8, 67, 3, 66, 4294967295, 8, 2, 8, 70, 1, 4294967295, 
    4294967295, 1, 2, 72, 71, 1, 4294967295, 4294967295, 1, 2, 32, 72, 1, 
    4294967295, 4294967295, 1, 2, 8, 73, 1, 4294967295, 4294967295, 5, 2, 
    8, 74, 1, 71, 4294967295, 8, 2, 8, 75, 1, 4294967295, 4294967295, 10, 
    2, 10, 76, 2, 4294967295, 4294967295, 12, 2, 8, 78, 1, 4294967295, 74, 
    9, 2, 8, 79, 1, 4294967295, 4294967295, 1, 3, 72, 80, 1, 4294967295, 
    4294967295, 1, 3, 32, 81, 1, 4294967295, 4294967295, 1, 3, 8, 82, 1, 
    4294967295, 4294967295, 1, 3, 32, 83, 1, 4294967295, 4294967295, 3, 
    3, 8, 84, 2, 80, 4294967295, 8, 3, 8, 86, 1, 4294967295, 4294967295, 
    1, 3, 72, 87, 1, 4294967295, 4294967295, 1, 3, 32, 88, 1, 4294967295, 
    4294967295, 1, 3, 8, 89, 1, 4294967295, 4294967295, 1, 3, 72, 90, 1, 
    4294967295, 4294967295, 1, 3, 32, 91, 1, 4294967295, 4294967295, 1, 
    3, 8, 92, 1, 4294967295, 4294967295, 1, 3, 32, 93, 1, 4294967295, 4294967295, 
    1, 3, 32, 94, 1, 4294967295, 4294967295, 1, 3, 8, 95, 1, 4294967295, 
    4294967295, 5, 3, 8, 96, 2, 91, 4294967295, 8, 3, 8, 98, 1, 4294967295, 
    4294967295, 10, 3, 10, 99, 2, 4294967295, 4294967295, 12, 3, 8, 101, 
    1, 4294967295, 94, 9, 3, 8, 102, 1, 4294967295, 4294967295, 1, 4, 72, 
    103, 1, 4294967295, 4294967295, 1, 4, 32, 104, 1, 4294967295, 4294967295, 
    1, 4, 8, 105, 1, 4294967295, 4294967295, 1, 4, 32, 106, 1, 4294967295, 
    4294967295, 3, 4, 8, 107, 2, 100, 4294967295, 8, 4, 8, 109, 1, 4294967295, 
    4294967295, 1, 4, 72, 110, 1, 4294967295, 4294967295, 1, 4, 32, 111, 
    1, 4294967295, 4294967295, 1, 4, 8, 112, 1, 4294967295, 4294967295, 
    1, 4, 72, 113, 1, 4294967295, 4294967295, 1, 4, 32, 114, 1, 4294967295, 
    4294967295, 1, 4, 8, 115, 1, 4294967295, 4294967295, 1, 4, 32, 116, 
    1, 4294967295, 4294967295, 1, 4, 32, 117, 1, 4294967295, 4294967295, 
    1, 4, 32, 118, 1, 4294967295, 4294967295, 1, 4, 32, 119, 1, 4294967295, 
    4294967295, 1, 4, 32, 120, 1, 4294967295, 4294967295, 3, 4, 8, 121, 
    2, 113, 4294967295, 8, 4, 8, 123, 1, 4294967295, 4294967295, 5, 4, 8, 
    124, 2, 115, 4294967295, 8, 4, 8, 126, 1, 4294967295, 4294967295, 10, 
    4, 10, 127, 2, 4294967295, 4294967295, 12, 4, 8, 129, 1, 4294967295, 
    118, 9, 4, 8, 130, 1, 4294967295, 4294967295, 1, 5, 72, 131, 1, 4294967295, 
    4294967295, 1, 5, 32, 132, 1, 4294967295, 4294967295, 1, 5, 8, 133, 
    1, 4294967295, 4294967295, 1, 5, 72, 134, 1, 4294967295, 4294967295, 
    1, 5, 32, 135, 1, 4294967295, 4294967295, 1, 5, 8, 136, 1, 4294967295, 
    4294967295, 1, 5, 72, 137, 1, 4294967295, 4294967295, 1, 5, 32, 138, 
    1, 4294967295, 4294967295, 1, 5, 8, 139, 1, 4294967295, 4294967295, 
    1, 5, 72, 140, 1, 4294967295, 4294967295, 1, 5, 32, 141, 1, 4294967295, 
    4294967295, 1, 5, 8, 142, 1, 4294967295, 4294967295, 1, 5, 32, 143, 
    1, 4294967295, 4294967295, 1, 5, 32, 144, 1, 4294967295, 4294967295, 
    1, 5, 8, 145, 1, 4294967295, 4294967295, 5, 5, 8, 146, 3, 135, 4294967295, 
    8, 5, 8, 149, 1, 4294967295, 4294967295, 10, 5, 10, 150, 2, 4294967295, 
    4294967295, 12, 5, 8, 152, 1, 4294967295, 138, 9, 5, 8, 153, 1, 4294967295, 
    4294967295, 1, 5, 0, 154, 0, 4294967295, 4294967295, 1, 39, 0, 0, 0, 
    1, 45, 0, 0, 0, 1, 65, 0, 0, 0, 1, 79, 0, 0, 0, 1, 99, 0, 0, 0, 1, 119, 
    0, 0, 0, 3, 2, 1, 13, 0, 3, 2, 1, 14, 0, 5, 15, 1, 0, 0, 5, 16, 4294967295, 
    0, 0, 1, 40, 0, 0, 0, 3, 2, 1, 18, 0, 5, 19, 2, 0, 0, 5, 20, 2, 0, 0, 
    5, 21, 1, 0, 0, 5, 22, 4294967295, 0, 0, 1, 40, 0, 0, 0, 5, 24, 3, 0, 
    0, 3, 4, 2, 25, 0, 5, 26, 4294967295, 0, 0, 1, 40, 0, 0, 0, 5, 28, 4, 
    0, 0, 3, 6, 3, 29, 0, 5, 30, 4294967295, 0, 0, 1, 40, 0, 0, 0, 5, 32, 
    5, 0, 0, 3, 8, 4, 33, 0, 5, 34, 4294967295, 0, 0, 1, 40, 0, 0, 0, 5, 
    36, 6, 0, 0, 3, 10, 5, 37, 0, 5, 38, 4294967295, 0, 0, 1, 40, 0, 0, 
    0, 1, 12, 0, 0, 0, 1, 17, 0, 0, 0, 1, 23, 0, 0, 0, 1, 27, 0, 0, 0, 1, 
    31, 0, 0, 0, 1, 35, 0, 0, 0, 1, 1, 0, 0, 0, 6, 42, 1, 4294967295, 0, 
    5, 43, 2, 0, 0, 3, 2, 1, 46, 2, 5, 46, 13, 0, 0, 1, 41, 0, 0, 0, 1, 
    44, 0, 0, 0, 1, 54, 0, 0, 0, 10, 48, 4, 0, 0, 5, 49, 2, 0, 0, 3, 2, 
    1, 53, 5, 10, 51, 3, 0, 0, 5, 53, 7, 0, 0, 1, 47, 0, 0, 0, 1, 50, 0, 
    0, 0, 1, 56, 0, 0, 0, 1, 52, 0, 0, 0, 1, 55, 0, 0, 0, 1, 3, 0, 0, 0, 
    1, 54, 0, 0, 0, 6, 58, 2, 4294967295, 0, 5, 59, 8, 0, 0, 3, 4, 2, 60, 
    0, 3, 4, 2, 61, 3, 1, 66, 0, 0, 0, 5, 63, 2, 0, 0, 3, 4, 2, 66, 2, 5, 
    66, 13, 0, 0, 1, 57, 0, 0, 0, 1, 62, 0, 0, 0, 1, 64, 0, 0, 0, 1, 72, 
    0, 0, 0, 10, 68, 4, 0, 0, 5, 69, 2, 0, 0, 3, 4, 2, 71, 5, 1, 67, 0, 
    0, 0, 1, 74, 0, 0, 0, 1, 70, 0, 0, 0, 1, 73, 0, 0, 0, 1, 5, 0, 0, 0, 
    1, 72, 0, 0, 0, 6, 76, 3, 4294967295, 0, 5, 77, 2, 0, 0, 3, 6, 3, 80, 
    2, 5, 80, 13, 0, 0, 1, 75, 0, 0, 0, 1, 78, 0, 0, 0, 1, 92, 0, 0, 0, 
    10, 82, 4, 0, 0, 5, 83, 2, 0, 0, 3, 6, 3, 91, 5, 10, 85, 3, 0, 0, 5, 
    86, 9, 0, 0, 3, 6, 3, 87, 0, 5, 88, 2, 0, 0, 5, 89, 13, 0, 0, 1, 91, 
    0, 0, 0, 1, 81, 0, 0, 0, 1, 84, 0, 0, 0, 1, 94, 0, 0, 0, 1, 90, 0, 0, 
    0, 1, 93, 0, 0, 0, 1, 7, 0, 0, 0, 1, 92, 0, 0, 0, 6, 96, 4, 4294967295, 
    0, 5, 97, 2, 0, 0, 3, 8, 4, 100, 2, 5, 100, 13, 0, 0, 1, 95, 0, 0, 0, 
    1, 98, 0, 0, 0, 1, 116, 0, 0, 0, 10, 102, 4, 0, 0, 5, 103, 2, 0, 0, 
    3, 8, 4, 115, 5, 10, 105, 3, 0, 0, 5, 106, 9, 0, 0, 3, 8, 4, 112, 0, 
    5, 108, 2, 0, 0, 5, 109, 13, 0, 0, 5, 110, 2, 0, 0, 5, 111, 13, 0, 0, 
    5, 113, 7, 0, 0, 1, 107, 0, 0, 0, 1, 113, 0, 0, 0, 1, 115, 0, 0, 0, 
    1, 101, 0, 0, 0, 1, 104, 0, 0, 0, 1, 118, 0, 0, 0, 1, 114, 0, 0, 0, 
    1, 117, 0, 0, 0, 1, 9, 0, 0, 0, 1, 116, 0, 0, 0, 6, 120, 5, 4294967295, 
    0, 5, 121, 13, 0, 0, 1, 136, 0, 0, 0, 10, 123, 4, 0, 0, 5, 124, 10, 
    0, 0, 3, 10, 5, 135, 5, 10, 126, 3, 0, 0, 5, 127, 11, 0, 0, 3, 10, 5, 
    135, 4, 10, 129, 2, 0, 0, 5, 130, 10, 0, 0, 3, 10, 5, 131, 0, 5, 132, 
    12, 0, 0, 5, 133, 13, 0, 0, 1, 135, 0, 0, 0, 1, 122, 0, 0, 0, 1, 125, 
    0, 0, 0, 1, 128, 0, 0, 0, 1, 138, 0, 0, 0, 1, 134, 0, 0, 0, 1, 137, 
    0, 0, 0, 1, 11, 0, 0, 0, 1, 136, 0, 0, 0, 39, 45, 52, 54, 65, 72, 79, 
    90, 92, 99, 112, 114, 116, 134, 136, 0, 2, 4, 6, 8, 10, 1, 3, 5, 7, 
    9, 11, 1396983361, 2, 1192, 16, 15, 0, 20, 15, 9, 75, 0, 0, 3, 4, 5, 
    6, 0, 4294967295, 0, 0, 1, 0, 0, 0, 4294967295, 4294967295, 2, 4294967295, 
    1, 4294967295, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 
    0, 0, 0, 1, 2, 2, 2, 2, 3, 2, 4, 5, 2, 6, 7, 8, 2, 2, 2, 2, 2, 2, 0, 
    18, 24, 24, 33, 42, 48, 54, 63, 75, 2, 2, 1, 3, 3, 2, 4, 4, 3, 5, 5, 
    4, 6, 6, 5, 13, 13, 6, 2, 2, 7, 13, 13, 8, 2, 2, 9, 7, 7, 6, 13, 13, 
    10, 2, 2, 11, 7, 7, 8, 13, 13, 10, 2, 2, 12, 13, 13, 13, 2, 2, 14, 13, 
    13, 15, 1, 1, 16, 2, 2, 7, 13, 13, 17, 1, 1, 18, 2, 2, 19, 7, 7, 13, 
    13, 13, 10, 0, 0, 0, 0, 0, 2, 3, 9, 2, 6, 0, 1, 2, 0, 0, 2, 0, 1, 1, 
    0, 6, 6, 2, 2, 1, 7, 7, 2, 0, 0, 0, 0, 0, 4294967295, 16, 15, 6, 51, 
    0, 2, 2, 0, 1, 2, 0, 0, 2, 4294967295, 4294967293, 1, 4294967295, 0, 
    1, 4294967295, 2, 2, 2, 2, 1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 0, 1, 
    1, 2, 1, 1, 3, 4, 1, 1, 1, 1, 1, 5, 1, 1, 0, 15, 15, 21, 30, 45, 51, 
    4294967295, 4294967295, 1, 1, 1, 2, 2, 2, 3, 7, 7, 4, 13, 13, 5, 2, 
    2, 6, 13, 13, 7, 1, 1, 8, 2, 2, 9, 13, 13, 10, 4294967295, 4294967295, 
    11, 1, 1, 12, 2, 2, 13, 7, 7, 7, 13, 13, 14, 2, 2, 15, 13, 13, 7, 0, 
    0, 1, 10, 1, 2, 12, 17, 12, 43, 49, 60, 63, 69, 77, 83, 97, 103, 106, 
    124, 127, 4294967295, 15, 15, 6, 48, 0, 2, 2, 0, 2, 0, 0, 2, 4294967295, 
    4294967293, 1, 4294967295, 0, 1, 4294967295, 2, 2, 2, 2, 2, 2, 1, 2, 
    2, 1, 1, 1, 1, 1, 1, 0, 1, 1, 2, 1, 3, 4, 1, 1, 1, 1, 1, 5, 1, 1, 0, 
    12, 12, 18, 27, 42, 48, 4294967295, 4294967295, 1, 1, 1, 2, 2, 2, 3, 
    13, 13, 4, 2, 2, 5, 13, 13, 6, 1, 1, 7, 2, 2, 8, 13, 13, 9, 4294967295, 
    4294967295, 10, 1, 1, 11, 2, 2, 12, 7, 7, 6, 13, 13, 13, 2, 2, 14, 13, 
    13, 6, 0, 0, 1, 9, 1, 2, 12, 17, 12, 43, 49, 60, 63, 69, 77, 83, 97, 
    103, 106, 124, 127, 4294967295, 1, 1, 1, 0, 2, 2, 0, 0, 0, 0, 0, 0, 
    0, 0, 4294967295, 18, 15, 8, 66, 0, 2, 0, 2, 2, 0, 0, 4294967293, 4294967295, 
    0, 0, 4294967295, 0, 4294967293, 4294967295, 4294967293, 4294967295, 
    4294967293, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 
    1, 2, 1, 1, 3, 4, 1, 1, 5, 6, 1, 7, 1, 1, 1, 1, 1, 0, 12, 12, 21, 30, 
    39, 48, 57, 66, 4294967295, 4294967295, 1, 2, 2, 2, 8, 8, 3, 13, 13, 
    4, 2, 2, 5, 8, 8, 6, 13, 13, 7, 2, 2, 8, 8, 8, 9, 13, 13, 7, 2, 2, 10, 
    8, 8, 11, 13, 13, 12, 2, 2, 8, 8, 8, 11, 13, 13, 13, 2, 2, 8, 8, 8, 
    14, 13, 13, 15, 2, 2, 16, 8, 8, 17, 13, 13, 7, 0, 0, 4, 7, 1, 13, 1, 
    15, 1, 17, 1, 1, 59, 12, 43, 49, 60, 63, 69, 77, 83, 97, 103, 106, 124, 
    127, 7, 3, 11, 2, 6, 0, 1, 2, 0, 0, 0, 0, 1, 1, 0, 6, 6, 2, 2, 1, 9, 
    9, 2, 0, 0, 0, 0, 0, 4294967295, 6, 15, 3, 15, 0, 2, 0, 1, 1, 4294967293, 
    2, 2, 2, 0, 0, 1, 0, 1, 2, 1, 1, 1, 0, 9, 9, 15, 4294967295, 4294967295, 
    1, 2, 2, 2, 9, 9, 3, 2, 2, 4, 13, 13, 5, 0, 0, 1, 5, 1, 1, 86, 12, 43, 
    49, 60, 63, 69, 77, 83, 97, 103, 106, 124, 127, 4294967295, 3, 4, 2, 
    6, 0, 2, 1, 2, 2, 0, 0, 1, 1, 0, 6, 6, 4294967295, 4294967295, 1, 2, 
    2, 2, 0, 0, 0, 0, 0, 10, 8, 15, 5, 30, 0, 2, 0, 2, 2, 0, 0, 4294967295, 
    2, 2, 2, 2, 2, 2, 2, 2, 0, 1, 2, 1, 1, 3, 4, 1, 0, 9, 9, 15, 24, 30, 
    4294967295, 4294967295, 1, 2, 2, 2, 9, 9, 3, 2, 2, 4, 13, 13, 5, 4294967295, 
    4294967295, 1, 2, 2, 6, 9, 9, 3, 2, 2, 4, 13, 13, 7, 0, 0, 0, 0, 0, 
    11, 3, 11, 2, 6, 0, 1, 2, 0, 0, 0, 0, 1, 1, 0, 6, 6, 2, 2, 1, 9, 9, 
    2, 0, 0, 0, 0, 0, 4294967295, 9, 15, 5, 30, 0, 2, 0, 4294967295, 4294967295, 
    0, 4294967293, 0, 4294967293, 2, 2, 2, 1, 1, 1, 1, 1, 1, 0, 1, 2, 1, 
    1, 3, 1, 4, 1, 0, 9, 9, 15, 24, 30, 4294967295, 4294967295, 1, 2, 2, 
    2, 9, 9, 3, 2, 2, 4, 13, 13, 5, 4294967295, 4294967295, 6, 2, 2, 7, 
    9, 9, 3, 2, 2, 4, 13, 13, 8, 0, 0, 2, 6, 1, 8, 1, 2, 32, 106, 12, 43, 
    49, 60, 63, 69, 77, 83, 97, 103, 106, 124, 127, 13, 10, 15, 6, 27, 0, 
    0, 2, 0, 1, 0, 0, 4294967295, 4294967295, 4294967295, 0, 0, 0, 1, 1, 
    1, 1, 1, 1, 1, 0, 1, 2, 3, 2, 4, 5, 2, 2, 2, 0, 6, 9, 9, 21, 24, 27, 
    10, 10, 1, 11, 11, 2, 13, 13, 3, 4294967295, 4294967295, 4, 10, 10, 
    5, 11, 11, 6, 12, 12, 7, 13, 13, 8, 13, 13, 9, 0, 0, 0, 0, 0, 4294967295, 
    5, 14, 2, 12, 0, 2, 1, 1, 2, 2, 2, 0, 0, 2, 0, 1, 1, 1, 1, 0, 12, 12, 
    4294967295, 4294967295, 1, 10, 10, 2, 11, 11, 3, 12, 12, 4, 0, 0, 0, 
    0, 0, 4294967295, 3, 13, 2, 6, 0, 2, 1, 2, 2, 0, 0, 1, 1, 0, 6, 6, 4294967295, 
    4294967295, 1, 10, 11, 2, 0, 0, 0, 0, 0, 4294967295, 12, 15, 6, 33, 
    0, 2, 0, 2, 2, 0, 1, 0, 0, 4294967295, 4294967295, 4294967295, 2, 2, 
    2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 0, 1, 2, 1, 1, 3, 1, 4, 5, 1, 1, 1, 0, 
    12, 12, 15, 27, 30, 33, 4294967295, 4294967295, 1, 10, 10, 2, 11, 11, 
    3, 12, 12, 4, 13, 13, 5, 4294967295, 4294967295, 6, 10, 10, 7, 11, 11, 
    8, 12, 12, 9, 13, 13, 10, 13, 13, 11, 0, 0, 0, 0, 0, 5, 3, 2, 3, 4, 
    2, 3, 4, 5, 1, 4, 5, 4, 8, 2, 3, 4, 7, 8, 4, 12, 2, 3, 4, 11, 8, 4, 
    14, 3, 2, 3, 4, 13, 14, 15, 4
];