// Generated from LoopObligation.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","8");
pub const LoopObligation_T__0:i32=1; 
pub const LoopObligation_T__1:i32=2; 
pub const LoopObligation_T__2:i32=3; 
pub const LoopObligation_T__3:i32=4; 
pub const LoopObligation_T__4:i32=5; 
pub const LoopObligation_T__5:i32=6; 
pub const LoopObligation_T__6:i32=7; 
pub const LoopObligation_T__7:i32=8; 
pub const LoopObligation_T__8:i32=9; 
pub const LoopObligation_ID:i32=10; 
pub const LoopObligation_WS:i32=11;
pub const LoopObligation_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_p:usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "s", "p"
];

pub const _LITERAL_NAMES: [Option<&'static str>;10] = [
	None, Some("'['"), Some("':'"), Some("']'"), Some("'.'"), Some("'+'"), 
	Some("'prior'"), Some("'trim'"), Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, LoopObligationParserExt<'input, 'arena>, LoopObligationParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopObligationParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> LoopObligationParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				LoopObligationParserExt {
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
        L: ParseTreeListener<'arena, LoopObligationParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct LoopObligationTreeWalker;
impl LoopObligationTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, LoopObligationParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, LoopObligationParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <LoopObligationParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LoopObligationParserNodeKind {
    SContext,
    PContext,
    Terminal,
    Error,
}
pub type LoopObligationParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LoopObligationParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => LoopObligationParser }
dbt_antlr_runtime::impl_node_kind! { LoopObligationParserNodeKind {
; SContext(), PContext(), 
    }; 
}

pub struct LoopObligationParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> LoopObligationParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { LoopObligationParserExt, LoopObligationParserNodeKind, "LoopObligation.g4"; sempred (LoopObligationParserNode<'input, 'arena, TF::Tok>, LoopObligationParser) { 1 => p_sempred, } }

impl<'input, 'arena, Input, TF> LoopObligationParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn p_sempred(_ctx: Option<&'arena PContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 7)
		    }
	        1 => {
			recog.precpred(None, 6)
		    }
	        2 => {
			recog.precpred(None, 5)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopObligationParserNodeKind, SContext, SContextExt, RULE_s }

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn p(&self) -> Option<&'arena PContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn p(&self) -> Option<&'arena PContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopObligation_EOF)
    }
}

impl<'input, 'arena, Input, TF> LoopObligationParser<'input, 'arena, Input, TF>
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
		recog.base.set_state(4);
		recog.p_rec(0)?;
		recog.base.set_state(5);
		recog.base.match_token(LoopObligation_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- p ----------------
pub type PContextAll<'input, 'arena, Tok = CommonToken<'input>> = PContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopObligationParserNodeKind, PContext, PContextExt, RULE_p }

pub trait PContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn p_all(&self) -> Vec<&'arena PContextAll<'input, 'arena, Tok>>;
    fn p(&self, i: usize) -> Option<&'arena PContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> PContextAttrs<'input, 'arena, Tok> for PContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn p_all(&self) -> Vec<&'arena PContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn p(&self, i: usize) -> Option<&'arena PContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopObligation_ID)
    }
}

impl<'input, 'arena, Input, TF> LoopObligationParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  p(&mut self,) -> Result<&'arena PContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.p_rec(0)
	}

	fn p_rec(&mut self, _p: i32) -> Result<&'arena PContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, PContext<TF::Tok>, RULE_p, 2, |_parentctx| PContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena PContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(26);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopObligation_T__5  => {
		        {
		        recog.base.set_state(8);
		        recog.base.match_token(LoopObligation_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(9);
		        recog.p_rec(0)?;
		        recog.base.set_state(10);
		        recog.base.match_token(LoopObligation_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(11);
		        recog.base.match_token(LoopObligation_ID,&mut recog.err_handler)?;
		        }}
		    LoopObligation_T__6  => {
		        {
		        recog.base.set_state(13);
		        recog.base.match_token(LoopObligation_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(14);
		        recog.base.match_token(LoopObligation_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(15);
		        recog.p_rec(0)?;
		        recog.base.set_state(16);
		        recog.p_rec(0)?;
		        recog.base.set_state(17);
		        recog.base.match_token(LoopObligation_T__8,&mut recog.err_handler)?;
		        }}
		    LoopObligation_T__0  => {
		        {
		        recog.base.set_state(19);
		        recog.base.match_token(LoopObligation_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(20);
		        recog.base.match_token(LoopObligation_ID,&mut recog.err_handler)?;
		        recog.base.set_state(21);
		        recog.base.match_token(LoopObligation_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(22);
		        recog.p_rec(0)?;
		        recog.base.set_state(23);
		        recog.base.match_token(LoopObligation_T__2,&mut recog.err_handler)?;
		        }}
		    LoopObligation_ID  => {
		        {
		        recog.base.set_state(25);
		        recog.base.match_token(LoopObligation_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(43);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(2)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(2,&mut recog.base)? } else { _sdp } };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(41);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(1)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = PContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_p)?;

						recog.base.set_state(28);
						if !({recog.precpred(None, 7)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
						}
						recog.base.set_state(29);
						recog.base.match_token(LoopObligation_T__0,&mut recog.err_handler)?;
						recog.base.set_state(30);
						recog.base.match_token(LoopObligation_ID,&mut recog.err_handler)?;
						recog.base.set_state(31);
						recog.base.match_token(LoopObligation_T__1,&mut recog.err_handler)?;
						recog.base.set_state(32);
						recog.p_rec(0)?;
						recog.base.set_state(33);
						recog.base.match_token(LoopObligation_T__2,&mut recog.err_handler)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = PContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_p)?;

						recog.base.set_state(35);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(36);
						recog.base.match_token(LoopObligation_T__3,&mut recog.err_handler)?;
						recog.base.set_state(37);
						recog.base.match_token(LoopObligation_ID,&mut recog.err_handler)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = PContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_p)?;

						recog.base.set_state(38);
						if !({recog.precpred(None, 5)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
						}
						recog.base.set_state(39);
						recog.base.match_token(LoopObligation_T__4,&mut recog.err_handler)?;
						recog.base.set_state(40);
						recog.p_rec(6)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(45);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(2)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(2,&mut recog.base)? } else { _sdp } };
		}
		}
		})
	}
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new_with_static_parser_tables(&_ATN, &_STATIC_PARSER_TABLES));
static _ATN: LazyLock<ATN> = LazyLock::new(|| {
    PackedATNDeserializer::new()
        .deserialize(&_serializedATN)
        .expect("invalid packed parser ATN")
});
static _serializedATN: [u32; 615] = [
    1346458702, 3, 16909060, 29, 11, 47, 50, 0, 0, 3, 2, 29, 329, 358, 250, 
    608, 0, 608, 0, 608, 3, 611, 2, 613, 2, 615, 0, 608, 0, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    1, 0, 8, 2, 1, 4294967295, 4294967295, 1, 0, 32, 3, 1, 4294967295, 4294967295, 
    1, 0, 8, 4, 1, 4294967295, 4294967295, 1, 1, 72, 5, 1, 4294967295, 4294967295, 
    1, 1, 32, 6, 1, 4294967295, 4294967295, 1, 1, 8, 7, 1, 4294967295, 4294967295, 
    1, 1, 32, 8, 1, 4294967295, 4294967295, 1, 1, 32, 9, 1, 4294967295, 
    4294967295, 1, 1, 8, 10, 1, 4294967295, 4294967295, 1, 1, 32, 11, 1, 
    4294967295, 4294967295, 1, 1, 32, 12, 1, 4294967295, 4294967295, 1, 
    1, 8, 13, 1, 4294967295, 4294967295, 1, 1, 8, 14, 1, 4294967295, 4294967295, 
    1, 1, 32, 15, 1, 4294967295, 4294967295, 1, 1, 8, 16, 1, 4294967295, 
    4294967295, 1, 1, 32, 17, 1, 4294967295, 4294967295, 1, 1, 32, 18, 1, 
    4294967295, 4294967295, 1, 1, 32, 19, 1, 4294967295, 4294967295, 1, 
    1, 8, 20, 1, 4294967295, 4294967295, 1, 1, 32, 21, 1, 4294967295, 4294967295, 
    1, 1, 8, 22, 1, 4294967295, 4294967295, 1, 1, 32, 23, 1, 4294967295, 
    4294967295, 3, 1, 8, 24, 4, 27, 4294967295, 8, 1, 8, 28, 1, 4294967295, 
    4294967295, 1, 1, 72, 29, 1, 4294967295, 4294967295, 1, 1, 32, 30, 1, 
    4294967295, 4294967295, 1, 1, 32, 31, 1, 4294967295, 4294967295, 1, 
    1, 32, 32, 1, 4294967295, 4294967295, 1, 1, 8, 33, 1, 4294967295, 4294967295, 
    1, 1, 32, 34, 1, 4294967295, 4294967295, 1, 1, 8, 35, 1, 4294967295, 
    4294967295, 1, 1, 72, 36, 1, 4294967295, 4294967295, 1, 1, 32, 37, 1, 
    4294967295, 4294967295, 1, 1, 32, 38, 1, 4294967295, 4294967295, 1, 
    1, 72, 39, 1, 4294967295, 4294967295, 1, 1, 32, 40, 1, 4294967295, 4294967295, 
    1, 1, 8, 41, 1, 4294967295, 4294967295, 5, 1, 8, 42, 3, 42, 4294967295, 
    8, 1, 8, 45, 1, 4294967295, 4294967295, 10, 1, 10, 46, 2, 4294967295, 
    4294967295, 12, 1, 8, 48, 1, 4294967295, 45, 9, 1, 8, 49, 1, 4294967295, 
    4294967295, 1, 1, 0, 50, 0, 4294967295, 4294967295, 1, 4, 0, 0, 0, 1, 
    26, 0, 0, 0, 3, 2, 1, 5, 0, 5, 6, 4294967295, 0, 0, 1, 1, 0, 0, 0, 6, 
    8, 1, 4294967295, 0, 5, 9, 6, 0, 0, 3, 2, 1, 10, 0, 5, 11, 4, 0, 0, 
    5, 12, 10, 0, 0, 1, 27, 0, 0, 0, 5, 14, 7, 0, 0, 5, 15, 8, 0, 0, 3, 
    2, 1, 16, 0, 3, 2, 1, 17, 0, 5, 18, 9, 0, 0, 1, 27, 0, 0, 0, 5, 20, 
    1, 0, 0, 5, 21, 10, 0, 0, 5, 22, 2, 0, 0, 3, 2, 1, 23, 0, 5, 24, 3, 
    0, 0, 1, 27, 0, 0, 0, 5, 27, 10, 0, 0, 1, 7, 0, 0, 0, 1, 13, 0, 0, 0, 
    1, 19, 0, 0, 0, 1, 25, 0, 0, 0, 1, 43, 0, 0, 0, 10, 29, 7, 0, 0, 5, 
    30, 1, 0, 0, 5, 31, 10, 0, 0, 5, 32, 2, 0, 0, 3, 2, 1, 33, 0, 5, 34, 
    3, 0, 0, 1, 42, 0, 0, 0, 10, 36, 6, 0, 0, 5, 37, 4, 0, 0, 5, 42, 10, 
    0, 0, 10, 39, 5, 0, 0, 5, 40, 5, 0, 0, 3, 2, 1, 42, 6, 1, 28, 0, 0, 
    0, 1, 35, 0, 0, 0, 1, 38, 0, 0, 0, 1, 45, 0, 0, 0, 1, 41, 0, 0, 0, 1, 
    44, 0, 0, 0, 1, 3, 0, 0, 0, 1, 43, 0, 0, 0, 26, 41, 43, 0, 2, 1, 3
];static _STATIC_PARSER_TABLES: LazyLock<StaticParserTables> = LazyLock::new(|| {
    StaticParserTables::from_image(&_STATIC_PARSER_TABLE_WORDS)
        .expect("invalid static parser DFA table image; regenerate the parser")
});
static _STATIC_PARSER_TABLE_WORDS: [u32; 340] = [
    1396983361, 4, 340, 3, 5, 1, 16, 4294967295, 0, 2147483648, 0, 34, 34, 
    101, 135, 101, 236, 35, 271, 28, 0, 3, 3, 7, 9, 5, 6, 7, 1, 2, 3, 4, 
    1, 1, 1, 1, 1, 1, 2, 3, 4, 34, 1, 4, 1, 1, 5, 100, 116, 0, 116, 124, 
    136, 0, 136, 0, 136, 0, 136, 0, 136, 0, 136, 0, 136, 0, 0, 1, 2, 3, 
    4294901760, 4294967295, 4294934529, 2147680255, 32771, 101, 4294967295, 
    16, 5, 4294967295, 12, 100, 164, 16, 228, 260, 380, 2, 388, 1, 392, 
    3, 404, 0, 404, 0, 404, 0, 404, 0, 0, 2, 0, 2, 0, 1, 2, 2, 2, 0, 3221225473, 
    0, 3221291009, 3221291009, 3221291009, 3221291009, 2, 2, 2, 2, 2, 0, 
    2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 4294901760, 4294901761, 4294901762, 4294967295, 
    262143, 327679, 4294967295, 4294967295, 4294934529, 4294901762, 294915, 
    2147909637, 4294934535, 2148040707, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 655359, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2148204543, 4294967295, 786431, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934540, 4294967295, 
    2148401151, 4294934542, 2148532223, 9, 15, 40, 0, 1, 2, 101, 4294967295, 
    16, 5, 4294967295, 12, 100, 164, 16, 228, 260, 380, 2, 388, 1, 392, 
    3, 404, 0, 404, 0, 404, 0, 404, 0, 0, 2, 0, 2, 0, 2, 2, 2, 2, 0, 3221225473, 
    0, 3221291009, 3221291009, 3221291009, 3221291009, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 4294901760, 4294901761, 4294901762, 4294967295, 
    262143, 327679, 4294967295, 4294967295, 4294934529, 4294901762, 294915, 
    2147909637, 4294934535, 2148040707, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 655359, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2148204543, 4294967295, 786431, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934540, 4294967295, 
    2148401151, 4294934542, 2148532223, 9, 15, 40, 0, 1, 2, 35, 4294967295, 
    3, 1, 4294967295, 3, 100, 112, 3, 124, 132, 140, 0, 140, 0, 140, 0, 
    140, 0, 140, 0, 140, 0, 140, 0, 0, 2, 1, 2, 2, 0, 4294901760, 65535, 
    4294934529, 32770, 28, 4294967295, 1, 0, 0, 0, 100, 104, 1, 108, 112, 
    112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 2, 2, 65535
];