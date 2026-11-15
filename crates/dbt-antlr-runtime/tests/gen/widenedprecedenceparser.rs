// Generated from WidenedPrecedence.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","9");
pub const WidenedPrecedence_T__0:i32=1; 
pub const WidenedPrecedence_T__1:i32=2; 
pub const WidenedPrecedence_T__2:i32=3; 
pub const WidenedPrecedence_T__3:i32=4; 
pub const WidenedPrecedence_T__4:i32=5; 
pub const WidenedPrecedence_T__5:i32=6; 
pub const WidenedPrecedence_T__6:i32=7; 
pub const WidenedPrecedence_T__7:i32=8; 
pub const WidenedPrecedence_T__8:i32=9; 
pub const WidenedPrecedence_T__9:i32=10; 
pub const WidenedPrecedence_T__10:i32=11; 
pub const WidenedPrecedence_T__11:i32=12; 
pub const WidenedPrecedence_T__12:i32=13; 
pub const WidenedPrecedence_T__13:i32=14; 
pub const WidenedPrecedence_T__14:i32=15; 
pub const WidenedPrecedence_T__15:i32=16; 
pub const WidenedPrecedence_T__16:i32=17; 
pub const WidenedPrecedence_T__17:i32=18; 
pub const WidenedPrecedence_T__18:i32=19; 
pub const WidenedPrecedence_T__19:i32=20; 
pub const WidenedPrecedence_T__20:i32=21; 
pub const WidenedPrecedence_T__21:i32=22; 
pub const WidenedPrecedence_T__22:i32=23; 
pub const WidenedPrecedence_T__23:i32=24; 
pub const WidenedPrecedence_T__24:i32=25; 
pub const WidenedPrecedence_T__25:i32=26; 
pub const WidenedPrecedence_T__26:i32=27; 
pub const WidenedPrecedence_T__27:i32=28; 
pub const WidenedPrecedence_T__28:i32=29; 
pub const WidenedPrecedence_T__29:i32=30; 
pub const WidenedPrecedence_T__30:i32=31; 
pub const WidenedPrecedence_T__31:i32=32; 
pub const WidenedPrecedence_WS:i32=33;
pub const WidenedPrecedence_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_e:usize = 1; 
pub const RULE_op:usize = 2; 
pub const RULE_c0:usize = 3; 
pub const RULE_c1:usize = 4; 
pub const RULE_c2:usize = 5; 
pub const RULE_c3:usize = 6; 
pub const RULE_c4:usize = 7; 
pub const RULE_c5:usize = 8; 
pub const RULE_c6:usize = 9; 
pub const RULE_c7:usize = 10; 
pub const RULE_c8:usize = 11; 
pub const RULE_c9:usize = 12; 
pub const RULE_c10:usize = 13;
pub const ruleNames: [&'static str; 14] = [
    "s", "e", "op", "c0", "c1", "c2", "c3", "c4", "c5", "c6", "c7", "c8", 
    "c9", "c10"
];

pub const _LITERAL_NAMES: [Option<&'static str>;33] = [
	None, Some("'a'"), Some("'m1'"), Some("'*'"), Some("'x'"), Some("'+'"), 
	Some("'q'"), Some("'r'"), Some("'u'"), Some("'t0a'"), Some("'v'"), Some("'t0b'"), 
	Some("'t1a'"), Some("'t1b'"), Some("'t2a'"), Some("'t2b'"), Some("'t3a'"), 
	Some("'t3b'"), Some("'t4a'"), Some("'t4b'"), Some("'t5a'"), Some("'t5b'"), 
	Some("'t6a'"), Some("'t6b'"), Some("'t7a'"), Some("'t7b'"), Some("'t8a'"), 
	Some("'t8b'"), Some("'t9a'"), Some("'t9b'"), Some("'end'"), Some("'t10a'"), 
	Some("'t10b'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;34]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, None, None, None, None, Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, WidenedPrecedenceParserExt<'input, 'arena>, WidenedPrecedenceParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				WidenedPrecedenceParserExt {
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
        L: ParseTreeListener<'arena, WidenedPrecedenceParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct WidenedPrecedenceTreeWalker;
impl WidenedPrecedenceTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, WidenedPrecedenceParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, WidenedPrecedenceParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <WidenedPrecedenceParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum WidenedPrecedenceParserNodeKind {
    SContext,
    EContext,
    OpContext,
    C0Context,
    C1Context,
    C2Context,
    C3Context,
    C4Context,
    C5Context,
    C6Context,
    C7Context,
    C8Context,
    C9Context,
    C10Context,
    Terminal,
    Error,
}
pub type WidenedPrecedenceParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, WidenedPrecedenceParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => WidenedPrecedenceParser }
dbt_antlr_runtime::impl_node_kind! { WidenedPrecedenceParserNodeKind {
; SContext(), EContext(), OpContext(), C0Context(), C1Context(), C2Context(), C3Context(), C4Context(), C5Context(), C6Context(), C7Context(), C8Context(), C9Context(), C10Context(), 
    }; 
}

pub struct WidenedPrecedenceParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> WidenedPrecedenceParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { WidenedPrecedenceParserExt, WidenedPrecedenceParserNodeKind, "WidenedPrecedence.g4"; sempred (WidenedPrecedenceParserNode<'input, 'arena, TF::Tok>, WidenedPrecedenceParser) { 1 => e_sempred, } }

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn e_sempred(_ctx: Option<&'arena EContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 6)
		    }
	        1 => {
			recog.precpred(None, 5)
		    }
	        2 => {
			recog.precpred(None, 4)
		    }
	        3 => {
			recog.precpred(None, 3)
		    }
	        4 => {
			recog.precpred(None, 2)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, SContext, SContextExt, RULE_s }

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedPrecedence_EOF)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SContext<TF::Tok>, RULE_s, 0, |_parentctx| SContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(28);
		recog.e_rec(0)?;
		recog.base.set_state(29);
		recog.base.match_token(WidenedPrecedence_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- e ----------------
pub type EContextAll<'input, 'arena, Tok = CommonToken<'input>> = EContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, EContext, EContextExt, RULE_e }

pub trait EContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn op(&self) -> Option<&'arena OpContextAll<'input, 'arena, Tok>>;
    fn c0(&self) -> Option<&'arena C0ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> EContextAttrs<'input, 'arena, Tok> for EContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    fn op(&self) -> Option<&'arena OpContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn c0(&self) -> Option<&'arena C0ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  e(&mut self,) -> Result<&'arena EContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.e_rec(0)
	}

	fn e_rec(&mut self, _p: i32) -> Result<&'arena EContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, EContext<TF::Tok>, RULE_e, 2, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		{
		recog.base.set_state(32);
		recog.base.match_token(WidenedPrecedence_T__3,&mut recog.err_handler)?;
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(51);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(1,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(49);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.get_interpreter().adaptive_predict(0,&mut recog.base)? {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(34);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(35);
						recog.op()?;
						recog.base.set_state(36);
						recog.base.match_token(WidenedPrecedence_T__0,&mut recog.err_handler)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(38);
						if !({recog.precpred(None, 5)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
						}
						recog.base.set_state(39);
						recog.c0()?;
						recog.base.set_state(40);
						recog.base.match_token(WidenedPrecedence_T__1,&mut recog.err_handler)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(42);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(43);
						recog.base.match_token(WidenedPrecedence_T__2,&mut recog.err_handler)?;
						recog.base.set_state(44);
						recog.e_rec(5)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(45);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(46);
						recog.c0()?;
						}
					}
				,
					5 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(47);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(48);
						recog.op()?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(53);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(1,&mut recog.base)?;
		}
		}
		})
	}
}
//------------------- op ----------------
pub type OpContextAll<'input, 'arena, Tok = CommonToken<'input>> = OpContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, OpContext, OpContextExt, RULE_op }

pub trait OpContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> OpContextAttrs<'input, 'arena, Tok> for OpContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn op(&mut self,) -> Result<&'arena OpContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, OpContext<TF::Tok>, RULE_op, 4, |_parentctx| OpContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena OpContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(54);
		recog.base.match_token(WidenedPrecedence_T__4,&mut recog.err_handler)?;
		recog.base.set_state(55);
		recog.base.match_token(WidenedPrecedence_T__5,&mut recog.err_handler)?;
		recog.base.set_state(56);
		recog.base.match_token(WidenedPrecedence_T__6,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- c0 ----------------
pub type C0ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C0Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C0Context, C0ContextExt, RULE_c0 }

pub trait C0ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c1(&self) -> Option<&'arena C1ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C0ContextAttrs<'input, 'arena, Tok> for C0Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c1(&self) -> Option<&'arena C1ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c0(&mut self,) -> Result<&'arena C0ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C0Context<TF::Tok>, RULE_c0, 6, |_parentctx| C0ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C0Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(66);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(58);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(59);
		        recog.c1()?;
		        recog.base.set_state(60);
		        recog.base.match_token(WidenedPrecedence_T__8,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(62);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(63);
		        recog.c1()?;
		        recog.base.set_state(64);
		        recog.base.match_token(WidenedPrecedence_T__10,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c1 ----------------
pub type C1ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C1Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C1Context, C1ContextExt, RULE_c1 }

pub trait C1ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c2(&self) -> Option<&'arena C2ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C1ContextAttrs<'input, 'arena, Tok> for C1Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c2(&self) -> Option<&'arena C2ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c1(&mut self,) -> Result<&'arena C1ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C1Context<TF::Tok>, RULE_c1, 8, |_parentctx| C1ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C1Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(76);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(68);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(69);
		        recog.c2()?;
		        recog.base.set_state(70);
		        recog.base.match_token(WidenedPrecedence_T__11,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(72);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(73);
		        recog.c2()?;
		        recog.base.set_state(74);
		        recog.base.match_token(WidenedPrecedence_T__12,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c2 ----------------
pub type C2ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C2Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C2Context, C2ContextExt, RULE_c2 }

pub trait C2ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c3(&self) -> Option<&'arena C3ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C2ContextAttrs<'input, 'arena, Tok> for C2Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c3(&self) -> Option<&'arena C3ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c2(&mut self,) -> Result<&'arena C2ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C2Context<TF::Tok>, RULE_c2, 10, |_parentctx| C2ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C2Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(86);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(78);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(79);
		        recog.c3()?;
		        recog.base.set_state(80);
		        recog.base.match_token(WidenedPrecedence_T__13,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(82);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(83);
		        recog.c3()?;
		        recog.base.set_state(84);
		        recog.base.match_token(WidenedPrecedence_T__14,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c3 ----------------
pub type C3ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C3Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C3Context, C3ContextExt, RULE_c3 }

pub trait C3ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c4(&self) -> Option<&'arena C4ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C3ContextAttrs<'input, 'arena, Tok> for C3Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c4(&self) -> Option<&'arena C4ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c3(&mut self,) -> Result<&'arena C3ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C3Context<TF::Tok>, RULE_c3, 12, |_parentctx| C3ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C3Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(96);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(88);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(89);
		        recog.c4()?;
		        recog.base.set_state(90);
		        recog.base.match_token(WidenedPrecedence_T__15,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(92);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(93);
		        recog.c4()?;
		        recog.base.set_state(94);
		        recog.base.match_token(WidenedPrecedence_T__16,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c4 ----------------
pub type C4ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C4Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C4Context, C4ContextExt, RULE_c4 }

pub trait C4ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c5(&self) -> Option<&'arena C5ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C4ContextAttrs<'input, 'arena, Tok> for C4Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c5(&self) -> Option<&'arena C5ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c4(&mut self,) -> Result<&'arena C4ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C4Context<TF::Tok>, RULE_c4, 14, |_parentctx| C4ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C4Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(106);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(98);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(99);
		        recog.c5()?;
		        recog.base.set_state(100);
		        recog.base.match_token(WidenedPrecedence_T__17,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(102);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(103);
		        recog.c5()?;
		        recog.base.set_state(104);
		        recog.base.match_token(WidenedPrecedence_T__18,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c5 ----------------
pub type C5ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C5Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C5Context, C5ContextExt, RULE_c5 }

pub trait C5ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c6(&self) -> Option<&'arena C6ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C5ContextAttrs<'input, 'arena, Tok> for C5Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c6(&self) -> Option<&'arena C6ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c5(&mut self,) -> Result<&'arena C5ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C5Context<TF::Tok>, RULE_c5, 16, |_parentctx| C5ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C5Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(116);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(108);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(109);
		        recog.c6()?;
		        recog.base.set_state(110);
		        recog.base.match_token(WidenedPrecedence_T__19,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(112);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(113);
		        recog.c6()?;
		        recog.base.set_state(114);
		        recog.base.match_token(WidenedPrecedence_T__20,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c6 ----------------
pub type C6ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C6Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C6Context, C6ContextExt, RULE_c6 }

pub trait C6ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c7(&self) -> Option<&'arena C7ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C6ContextAttrs<'input, 'arena, Tok> for C6Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c7(&self) -> Option<&'arena C7ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c6(&mut self,) -> Result<&'arena C6ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C6Context<TF::Tok>, RULE_c6, 18, |_parentctx| C6ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C6Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(126);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(118);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(119);
		        recog.c7()?;
		        recog.base.set_state(120);
		        recog.base.match_token(WidenedPrecedence_T__21,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(122);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(123);
		        recog.c7()?;
		        recog.base.set_state(124);
		        recog.base.match_token(WidenedPrecedence_T__22,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c7 ----------------
pub type C7ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C7Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C7Context, C7ContextExt, RULE_c7 }

pub trait C7ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c8(&self) -> Option<&'arena C8ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C7ContextAttrs<'input, 'arena, Tok> for C7Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c8(&self) -> Option<&'arena C8ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c7(&mut self,) -> Result<&'arena C7ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C7Context<TF::Tok>, RULE_c7, 20, |_parentctx| C7ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C7Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(136);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(128);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(129);
		        recog.c8()?;
		        recog.base.set_state(130);
		        recog.base.match_token(WidenedPrecedence_T__23,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(132);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(133);
		        recog.c8()?;
		        recog.base.set_state(134);
		        recog.base.match_token(WidenedPrecedence_T__24,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c8 ----------------
pub type C8ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C8Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C8Context, C8ContextExt, RULE_c8 }

pub trait C8ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c9(&self) -> Option<&'arena C9ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C8ContextAttrs<'input, 'arena, Tok> for C8Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c9(&self) -> Option<&'arena C9ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c8(&mut self,) -> Result<&'arena C8ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C8Context<TF::Tok>, RULE_c8, 22, |_parentctx| C8ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C8Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(146);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(138);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(139);
		        recog.c9()?;
		        recog.base.set_state(140);
		        recog.base.match_token(WidenedPrecedence_T__25,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(142);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(143);
		        recog.c9()?;
		        recog.base.set_state(144);
		        recog.base.match_token(WidenedPrecedence_T__26,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c9 ----------------
pub type C9ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C9Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C9Context, C9ContextExt, RULE_c9 }

pub trait C9ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn c10(&self) -> Option<&'arena C10ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> C9ContextAttrs<'input, 'arena, Tok> for C9Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn c10(&self) -> Option<&'arena C10ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c9(&mut self,) -> Result<&'arena C9ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C9Context<TF::Tok>, RULE_c9, 24, |_parentctx| C9ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C9Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(156);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(148);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(149);
		        recog.c10()?;
		        recog.base.set_state(150);
		        recog.base.match_token(WidenedPrecedence_T__27,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(152);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(153);
		        recog.c10()?;
		        recog.base.set_state(154);
		        recog.base.match_token(WidenedPrecedence_T__28,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- c10 ----------------
pub type C10ContextAll<'input, 'arena, Tok = CommonToken<'input>> = C10Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedPrecedenceParserNodeKind, C10Context, C10ContextExt, RULE_c10 }

pub trait C10ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> C10ContextAttrs<'input, 'arena, Tok> for C10Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> WidenedPrecedenceParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c10(&mut self,) -> Result<&'arena C10ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, C10Context<TF::Tok>, RULE_c10, 26, |_parentctx| C10ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena C10Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(164);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedPrecedence_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(158);
		        recog.base.match_token(WidenedPrecedence_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(159);
		        recog.base.match_token(WidenedPrecedence_T__29,&mut recog.err_handler)?;
		        recog.base.set_state(160);
		        recog.base.match_token(WidenedPrecedence_T__30,&mut recog.err_handler)?;
		        }}
		    WidenedPrecedence_T__9  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(161);
		        recog.base.match_token(WidenedPrecedence_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(162);
		        recog.base.match_token(WidenedPrecedence_T__29,&mut recog.err_handler)?;
		        recog.base.set_state(163);
		        recog.base.match_token(WidenedPrecedence_T__31,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
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
static _serializedATN: [u32; 2079] = [
    1346458702, 3, 16909060, 29, 33, 167, 168, 0, 0, 13, 14, 29, 1169, 1198, 
    840, 2038, 0, 2038, 0, 2038, 13, 2051, 14, 2065, 14, 2079, 0, 2038, 
    0, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 
    4294967295, 2, 1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 
    4294967295, 4294967295, 2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 
    16, 3, 0, 4294967295, 4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 
    7, 3, 16, 4, 0, 4294967295, 4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 
    7, 4, 16, 5, 0, 4294967295, 4294967295, 2, 5, 8, 5, 1, 4294967295, 4294967295, 
    7, 5, 16, 6, 0, 4294967295, 4294967295, 2, 6, 8, 6, 1, 4294967295, 4294967295, 
    7, 6, 16, 7, 0, 4294967295, 4294967295, 2, 7, 8, 7, 1, 4294967295, 4294967295, 
    7, 7, 16, 8, 0, 4294967295, 4294967295, 2, 8, 8, 8, 1, 4294967295, 4294967295, 
    7, 8, 16, 9, 0, 4294967295, 4294967295, 2, 9, 8, 9, 1, 4294967295, 4294967295, 
    7, 9, 16, 10, 0, 4294967295, 4294967295, 2, 10, 8, 10, 1, 4294967295, 
    4294967295, 7, 10, 16, 11, 0, 4294967295, 4294967295, 2, 11, 8, 11, 
    1, 4294967295, 4294967295, 7, 11, 16, 12, 0, 4294967295, 4294967295, 
    2, 12, 8, 12, 1, 4294967295, 4294967295, 7, 12, 16, 13, 0, 4294967295, 
    4294967295, 2, 13, 8, 13, 1, 4294967295, 4294967295, 7, 13, 16, 14, 
    0, 4294967295, 4294967295, 1, 0, 8, 14, 1, 4294967295, 4294967295, 1, 
    0, 32, 15, 1, 4294967295, 4294967295, 1, 0, 8, 16, 1, 4294967295, 4294967295, 
    1, 1, 72, 17, 1, 4294967295, 4294967295, 1, 1, 32, 18, 1, 4294967295, 
    4294967295, 1, 1, 8, 19, 1, 4294967295, 4294967295, 1, 1, 72, 20, 1, 
    4294967295, 4294967295, 1, 1, 8, 21, 1, 4294967295, 4294967295, 1, 1, 
    32, 22, 1, 4294967295, 4294967295, 1, 1, 8, 23, 1, 4294967295, 4294967295, 
    1, 1, 72, 24, 1, 4294967295, 4294967295, 1, 1, 8, 25, 1, 4294967295, 
    4294967295, 1, 1, 32, 26, 1, 4294967295, 4294967295, 1, 1, 8, 27, 1, 
    4294967295, 4294967295, 1, 1, 72, 28, 1, 4294967295, 4294967295, 1, 
    1, 32, 29, 1, 4294967295, 4294967295, 1, 1, 8, 30, 1, 4294967295, 4294967295, 
    1, 1, 72, 31, 1, 4294967295, 4294967295, 1, 1, 8, 32, 1, 4294967295, 
    4294967295, 1, 1, 72, 33, 1, 4294967295, 4294967295, 1, 1, 8, 34, 1, 
    4294967295, 4294967295, 5, 1, 8, 35, 5, 50, 4294967295, 8, 1, 8, 40, 
    1, 4294967295, 4294967295, 10, 1, 10, 41, 2, 4294967295, 4294967295, 
    12, 1, 8, 43, 1, 4294967295, 53, 9, 1, 8, 44, 1, 4294967295, 4294967295, 
    1, 2, 32, 45, 1, 4294967295, 4294967295, 1, 2, 32, 46, 1, 4294967295, 
    4294967295, 1, 2, 32, 47, 1, 4294967295, 4294967295, 1, 2, 8, 48, 1, 
    4294967295, 4294967295, 1, 3, 32, 49, 1, 4294967295, 4294967295, 1, 
    3, 8, 50, 1, 4294967295, 4294967295, 1, 3, 32, 51, 1, 4294967295, 4294967295, 
    1, 3, 8, 52, 1, 4294967295, 4294967295, 1, 3, 32, 53, 1, 4294967295, 
    4294967295, 1, 3, 8, 54, 1, 4294967295, 4294967295, 1, 3, 32, 55, 1, 
    4294967295, 4294967295, 1, 3, 8, 56, 1, 4294967295, 4294967295, 3, 3, 
    8, 57, 2, 67, 4294967295, 8, 3, 8, 59, 1, 4294967295, 4294967295, 1, 
    4, 32, 60, 1, 4294967295, 4294967295, 1, 4, 8, 61, 1, 4294967295, 4294967295, 
    1, 4, 32, 62, 1, 4294967295, 4294967295, 1, 4, 8, 63, 1, 4294967295, 
    4294967295, 1, 4, 32, 64, 1, 4294967295, 4294967295, 1, 4, 8, 65, 1, 
    4294967295, 4294967295, 1, 4, 32, 66, 1, 4294967295, 4294967295, 1, 
    4, 8, 67, 1, 4294967295, 4294967295, 3, 4, 8, 68, 2, 77, 4294967295, 
    8, 4, 8, 70, 1, 4294967295, 4294967295, 1, 5, 32, 71, 1, 4294967295, 
    4294967295, 1, 5, 8, 72, 1, 4294967295, 4294967295, 1, 5, 32, 73, 1, 
    4294967295, 4294967295, 1, 5, 8, 74, 1, 4294967295, 4294967295, 1, 5, 
    32, 75, 1, 4294967295, 4294967295, 1, 5, 8, 76, 1, 4294967295, 4294967295, 
    1, 5, 32, 77, 1, 4294967295, 4294967295, 1, 5, 8, 78, 1, 4294967295, 
    4294967295, 3, 5, 8, 79, 2, 87, 4294967295, 8, 5, 8, 81, 1, 4294967295, 
    4294967295, 1, 6, 32, 82, 1, 4294967295, 4294967295, 1, 6, 8, 83, 1, 
    4294967295, 4294967295, 1, 6, 32, 84, 1, 4294967295, 4294967295, 1, 
    6, 8, 85, 1, 4294967295, 4294967295, 1, 6, 32, 86, 1, 4294967295, 4294967295, 
    1, 6, 8, 87, 1, 4294967295, 4294967295, 1, 6, 32, 88, 1, 4294967295, 
    4294967295, 1, 6, 8, 89, 1, 4294967295, 4294967295, 3, 6, 8, 90, 2, 
    97, 4294967295, 8, 6, 8, 92, 1, 4294967295, 4294967295, 1, 7, 32, 93, 
    1, 4294967295, 4294967295, 1, 7, 8, 94, 1, 4294967295, 4294967295, 1, 
    7, 32, 95, 1, 4294967295, 4294967295, 1, 7, 8, 96, 1, 4294967295, 4294967295, 
    1, 7, 32, 97, 1, 4294967295, 4294967295, 1, 7, 8, 98, 1, 4294967295, 
    4294967295, 1, 7, 32, 99, 1, 4294967295, 4294967295, 1, 7, 8, 100, 1, 
    4294967295, 4294967295, 3, 7, 8, 101, 2, 107, 4294967295, 8, 7, 8, 103, 
    1, 4294967295, 4294967295, 1, 8, 32, 104, 1, 4294967295, 4294967295, 
    1, 8, 8, 105, 1, 4294967295, 4294967295, 1, 8, 32, 106, 1, 4294967295, 
    4294967295, 1, 8, 8, 107, 1, 4294967295, 4294967295, 1, 8, 32, 108, 
    1, 4294967295, 4294967295, 1, 8, 8, 109, 1, 4294967295, 4294967295, 
    1, 8, 32, 110, 1, 4294967295, 4294967295, 1, 8, 8, 111, 1, 4294967295, 
    4294967295, 3, 8, 8, 112, 2, 117, 4294967295, 8, 8, 8, 114, 1, 4294967295, 
    4294967295, 1, 9, 32, 115, 1, 4294967295, 4294967295, 1, 9, 8, 116, 
    1, 4294967295, 4294967295, 1, 9, 32, 117, 1, 4294967295, 4294967295, 
    1, 9, 8, 118, 1, 4294967295, 4294967295, 1, 9, 32, 119, 1, 4294967295, 
    4294967295, 1, 9, 8, 120, 1, 4294967295, 4294967295, 1, 9, 32, 121, 
    1, 4294967295, 4294967295, 1, 9, 8, 122, 1, 4294967295, 4294967295, 
    3, 9, 8, 123, 2, 127, 4294967295, 8, 9, 8, 125, 1, 4294967295, 4294967295, 
    1, 10, 32, 126, 1, 4294967295, 4294967295, 1, 10, 8, 127, 1, 4294967295, 
    4294967295, 1, 10, 32, 128, 1, 4294967295, 4294967295, 1, 10, 8, 129, 
    1, 4294967295, 4294967295, 1, 10, 32, 130, 1, 4294967295, 4294967295, 
    1, 10, 8, 131, 1, 4294967295, 4294967295, 1, 10, 32, 132, 1, 4294967295, 
    4294967295, 1, 10, 8, 133, 1, 4294967295, 4294967295, 3, 10, 8, 134, 
    2, 137, 4294967295, 8, 10, 8, 136, 1, 4294967295, 4294967295, 1, 11, 
    32, 137, 1, 4294967295, 4294967295, 1, 11, 8, 138, 1, 4294967295, 4294967295, 
    1, 11, 32, 139, 1, 4294967295, 4294967295, 1, 11, 8, 140, 1, 4294967295, 
    4294967295, 1, 11, 32, 141, 1, 4294967295, 4294967295, 1, 11, 8, 142, 
    1, 4294967295, 4294967295, 1, 11, 32, 143, 1, 4294967295, 4294967295, 
    1, 11, 8, 144, 1, 4294967295, 4294967295, 3, 11, 8, 145, 2, 147, 4294967295, 
    8, 11, 8, 147, 1, 4294967295, 4294967295, 1, 12, 32, 148, 1, 4294967295, 
    4294967295, 1, 12, 8, 149, 1, 4294967295, 4294967295, 1, 12, 32, 150, 
    1, 4294967295, 4294967295, 1, 12, 8, 151, 1, 4294967295, 4294967295, 
    1, 12, 32, 152, 1, 4294967295, 4294967295, 1, 12, 8, 153, 1, 4294967295, 
    4294967295, 1, 12, 32, 154, 1, 4294967295, 4294967295, 1, 12, 8, 155, 
    1, 4294967295, 4294967295, 3, 12, 8, 156, 2, 157, 4294967295, 8, 12, 
    8, 158, 1, 4294967295, 4294967295, 1, 13, 32, 159, 1, 4294967295, 4294967295, 
    1, 13, 32, 160, 1, 4294967295, 4294967295, 1, 13, 32, 161, 1, 4294967295, 
    4294967295, 1, 13, 32, 162, 1, 4294967295, 4294967295, 1, 13, 32, 163, 
    1, 4294967295, 4294967295, 1, 13, 32, 164, 1, 4294967295, 4294967295, 
    3, 13, 8, 165, 2, 165, 4294967295, 8, 13, 8, 167, 1, 4294967295, 4294967295, 
    1, 13, 0, 168, 0, 4294967295, 4294967295, 1, 28, 0, 0, 0, 1, 31, 0, 
    0, 0, 1, 54, 0, 0, 0, 1, 66, 0, 0, 0, 1, 76, 0, 0, 0, 1, 86, 0, 0, 0, 
    1, 96, 0, 0, 0, 1, 106, 0, 0, 0, 1, 116, 0, 0, 0, 1, 126, 0, 0, 0, 1, 
    136, 0, 0, 0, 1, 146, 0, 0, 0, 1, 156, 0, 0, 0, 1, 164, 0, 0, 0, 3, 
    2, 1, 29, 0, 5, 30, 4294967295, 0, 0, 1, 1, 0, 0, 0, 6, 32, 1, 4294967295, 
    0, 5, 33, 4, 0, 0, 1, 51, 0, 0, 0, 10, 35, 6, 0, 0, 3, 4, 2, 36, 0, 
    5, 37, 1, 0, 0, 1, 50, 0, 0, 0, 10, 39, 5, 0, 0, 3, 6, 3, 40, 0, 5, 
    41, 2, 0, 0, 1, 50, 0, 0, 0, 10, 43, 4, 0, 0, 5, 44, 3, 0, 0, 3, 2, 
    1, 50, 5, 10, 46, 3, 0, 0, 3, 6, 3, 50, 0, 10, 48, 2, 0, 0, 3, 4, 2, 
    50, 0, 1, 34, 0, 0, 0, 1, 38, 0, 0, 0, 1, 42, 0, 0, 0, 1, 45, 0, 0, 
    0, 1, 47, 0, 0, 0, 1, 53, 0, 0, 0, 1, 49, 0, 0, 0, 1, 52, 0, 0, 0, 1, 
    3, 0, 0, 0, 1, 51, 0, 0, 0, 5, 55, 5, 0, 0, 5, 56, 6, 0, 0, 5, 57, 7, 
    0, 0, 1, 5, 0, 0, 0, 5, 59, 8, 0, 0, 3, 8, 4, 60, 0, 5, 61, 9, 0, 0, 
    1, 67, 0, 0, 0, 5, 63, 10, 0, 0, 3, 8, 4, 64, 0, 5, 65, 11, 0, 0, 1, 
    67, 0, 0, 0, 1, 58, 0, 0, 0, 1, 62, 0, 0, 0, 1, 7, 0, 0, 0, 5, 69, 8, 
    0, 0, 3, 10, 5, 70, 0, 5, 71, 12, 0, 0, 1, 77, 0, 0, 0, 5, 73, 10, 0, 
    0, 3, 10, 5, 74, 0, 5, 75, 13, 0, 0, 1, 77, 0, 0, 0, 1, 68, 0, 0, 0, 
    1, 72, 0, 0, 0, 1, 9, 0, 0, 0, 5, 79, 8, 0, 0, 3, 12, 6, 80, 0, 5, 81, 
    14, 0, 0, 1, 87, 0, 0, 0, 5, 83, 10, 0, 0, 3, 12, 6, 84, 0, 5, 85, 15, 
    0, 0, 1, 87, 0, 0, 0, 1, 78, 0, 0, 0, 1, 82, 0, 0, 0, 1, 11, 0, 0, 0, 
    5, 89, 8, 0, 0, 3, 14, 7, 90, 0, 5, 91, 16, 0, 0, 1, 97, 0, 0, 0, 5, 
    93, 10, 0, 0, 3, 14, 7, 94, 0, 5, 95, 17, 0, 0, 1, 97, 0, 0, 0, 1, 88, 
    0, 0, 0, 1, 92, 0, 0, 0, 1, 13, 0, 0, 0, 5, 99, 8, 0, 0, 3, 16, 8, 100, 
    0, 5, 101, 18, 0, 0, 1, 107, 0, 0, 0, 5, 103, 10, 0, 0, 3, 16, 8, 104, 
    0, 5, 105, 19, 0, 0, 1, 107, 0, 0, 0, 1, 98, 0, 0, 0, 1, 102, 0, 0, 
    0, 1, 15, 0, 0, 0, 5, 109, 8, 0, 0, 3, 18, 9, 110, 0, 5, 111, 20, 0, 
    0, 1, 117, 0, 0, 0, 5, 113, 10, 0, 0, 3, 18, 9, 114, 0, 5, 115, 21, 
    0, 0, 1, 117, 0, 0, 0, 1, 108, 0, 0, 0, 1, 112, 0, 0, 0, 1, 17, 0, 0, 
    0, 5, 119, 8, 0, 0, 3, 20, 10, 120, 0, 5, 121, 22, 0, 0, 1, 127, 0, 
    0, 0, 5, 123, 10, 0, 0, 3, 20, 10, 124, 0, 5, 125, 23, 0, 0, 1, 127, 
    0, 0, 0, 1, 118, 0, 0, 0, 1, 122, 0, 0, 0, 1, 19, 0, 0, 0, 5, 129, 8, 
    0, 0, 3, 22, 11, 130, 0, 5, 131, 24, 0, 0, 1, 137, 0, 0, 0, 5, 133, 
    10, 0, 0, 3, 22, 11, 134, 0, 5, 135, 25, 0, 0, 1, 137, 0, 0, 0, 1, 128, 
    0, 0, 0, 1, 132, 0, 0, 0, 1, 21, 0, 0, 0, 5, 139, 8, 0, 0, 3, 24, 12, 
    140, 0, 5, 141, 26, 0, 0, 1, 147, 0, 0, 0, 5, 143, 10, 0, 0, 3, 24, 
    12, 144, 0, 5, 145, 27, 0, 0, 1, 147, 0, 0, 0, 1, 138, 0, 0, 0, 1, 142, 
    0, 0, 0, 1, 23, 0, 0, 0, 5, 149, 8, 0, 0, 3, 26, 13, 150, 0, 5, 151, 
    28, 0, 0, 1, 157, 0, 0, 0, 5, 153, 10, 0, 0, 3, 26, 13, 154, 0, 5, 155, 
    29, 0, 0, 1, 157, 0, 0, 0, 1, 148, 0, 0, 0, 1, 152, 0, 0, 0, 1, 25, 
    0, 0, 0, 5, 159, 8, 0, 0, 5, 160, 30, 0, 0, 5, 165, 31, 0, 0, 5, 162, 
    10, 0, 0, 5, 163, 30, 0, 0, 5, 165, 32, 0, 0, 1, 158, 0, 0, 0, 1, 161, 
    0, 0, 0, 1, 27, 0, 0, 0, 49, 51, 66, 76, 86, 96, 106, 116, 126, 136, 
    146, 156, 164, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 1, 
    3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27
];