// Generated from Repro4TwoCastWithWrap.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","11");
pub const Repro4TwoCastWithWrap_T__0:i32=1; 
pub const Repro4TwoCastWithWrap_T__1:i32=2; 
pub const Repro4TwoCastWithWrap_T__2:i32=3; 
pub const Repro4TwoCastWithWrap_T__3:i32=4; 
pub const Repro4TwoCastWithWrap_T__4:i32=5; 
pub const Repro4TwoCastWithWrap_T__5:i32=6; 
pub const Repro4TwoCastWithWrap_T__6:i32=7; 
pub const Repro4TwoCastWithWrap_T__7:i32=8; 
pub const Repro4TwoCastWithWrap_T__8:i32=9; 
pub const Repro4TwoCastWithWrap_T__9:i32=10; 
pub const Repro4TwoCastWithWrap_ID:i32=11; 
pub const Repro4TwoCastWithWrap_WS:i32=12;
pub const Repro4TwoCastWithWrap_EOF:i32=EOF;
pub const RULE_stmt:usize = 0; 
pub const RULE_e:usize = 1; 
pub const RULE_sub:usize = 2;
pub const ruleNames: [&'static str; 3] = [
    "stmt", "e", "sub"
];

pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
	None, Some("'WRAP'"), Some("'+'"), Some("'!'"), Some("'CAST'"), Some("'('"), 
	Some("'AS'"), Some("')'"), Some("'CAST2'"), Some("'ONLY_HIGH'"), Some("'*'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;13]  = [
	None, None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, Repro4TwoCastWithWrapParserExt<'input, 'arena>, Repro4TwoCastWithWrapParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct Repro4TwoCastWithWrapParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> Repro4TwoCastWithWrapParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn with_delegate(arena: &'arena Arena, input: Input, err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				Repro4TwoCastWithWrapParserExt {
					_pd: Default::default(),
				}
			),
            err_handler,
        }
    }

    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
        Self::with_delegate(arena, input, unsafe { ErrorStrategyDelegate::new(strategy) })
    }

    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        Self::with_delegate(arena, input, ErrorStrategyDelegate::new_default())
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
        L: ParseTreeListener<'arena, Repro4TwoCastWithWrapParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct Repro4TwoCastWithWrapTreeWalker;
impl Repro4TwoCastWithWrapTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, Repro4TwoCastWithWrapParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, Repro4TwoCastWithWrapParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <Repro4TwoCastWithWrapParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum Repro4TwoCastWithWrapParserNodeKind {
    StmtContext,
    EContext,
    SubContext,
    Terminal,
    Error,
}
pub type Repro4TwoCastWithWrapParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, Repro4TwoCastWithWrapParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => Repro4TwoCastWithWrapParser }
dbt_antlr_runtime::impl_node_kind! { Repro4TwoCastWithWrapParserNodeKind {
; StmtContext(), EContext(), SubContext(), 
    }; 
}

pub struct Repro4TwoCastWithWrapParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> Repro4TwoCastWithWrapParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { Repro4TwoCastWithWrapParserExt, Repro4TwoCastWithWrapParserNodeKind, "Repro4TwoCastWithWrap.g4"; sempred (Repro4TwoCastWithWrapParserNode<'input, 'arena, TF::Tok>, Repro4TwoCastWithWrapParser) { 1 => e_sempred, } }

impl<'input, 'arena, Input, TF> Repro4TwoCastWithWrapParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn e_sempred(_ctx: Option<&'arena EContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
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
		    _ => true
		}
	}
}
//------------------- stmt ----------------
pub type StmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = StmtContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { Repro4TwoCastWithWrapParserNodeKind, StmtContext, StmtContextExt, RULE_stmt }

pub trait StmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn sub(&self) -> Option<&'arena SubContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StmtContextAttrs<'input, 'arena, Tok> for StmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn sub(&self) -> Option<&'arena SubContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == Repro4TwoCastWithWrap_EOF)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == Repro4TwoCastWithWrap_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == Repro4TwoCastWithWrap_ID).nth(i)
    }
}

impl<'input, 'arena, Input, TF> Repro4TwoCastWithWrapParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stmt(&mut self,) -> Result<&'arena StmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StmtContext<TF::Tok>, RULE_stmt, 0, |_parentctx| StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(34);
		match recog.base.input.la(1) {
		    Repro4TwoCastWithWrap_T__0  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(6);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(7);
		        recog.e_rec(0)?;
		        recog.base.set_state(8);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(9);
		        recog.sub()?;
		        recog.base.set_state(10);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__2,&mut recog.err_handler)?;
		        recog.base.set_state(11);
		        recog.base.match_token(Repro4TwoCastWithWrap_EOF,&mut recog.err_handler)?;
		        }}
		    Repro4TwoCastWithWrap_T__3  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(13);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(14);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__4,&mut recog.err_handler)?;
		        recog.base.set_state(15);
		        recog.e_rec(0)?;
		        recog.base.set_state(16);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(17);
		        recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
		        recog.base.set_state(18);
		        recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
		        recog.base.set_state(19);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(20);
		        recog.base.match_token(Repro4TwoCastWithWrap_EOF,&mut recog.err_handler)?;
		        }}
		    Repro4TwoCastWithWrap_T__7  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(22);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(23);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__4,&mut recog.err_handler)?;
		        recog.base.set_state(24);
		        recog.e_rec(0)?;
		        recog.base.set_state(25);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(26);
		        recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
		        recog.base.set_state(27);
		        recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
		        recog.base.set_state(28);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(29);
		        recog.base.match_token(Repro4TwoCastWithWrap_EOF,&mut recog.err_handler)?;
		        }}
		    Repro4TwoCastWithWrap_ID  => {
		        /*------- Outer Most Alt 4 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
		        {
		        recog.base.set_state(31);
		        recog.e_rec(0)?;
		        recog.base.set_state(32);
		        recog.base.match_token(Repro4TwoCastWithWrap_EOF,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- e ----------------
pub type EContextAll<'input, 'arena, Tok = CommonToken<'input>> = EContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { Repro4TwoCastWithWrapParserNodeKind, EContext, EContextExt, RULE_e }

pub trait EContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn sub(&self) -> Option<&'arena SubContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> EContextAttrs<'input, 'arena, Tok> for EContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == Repro4TwoCastWithWrap_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == Repro4TwoCastWithWrap_ID).nth(i)
    }
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    fn sub(&self) -> Option<&'arena SubContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> Repro4TwoCastWithWrapParser<'input, 'arena, Input, TF>
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
		recog.base.set_state(37);
		recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(55);
		_alt = { let _sdp = recog.base.dfa_predict(2)?; _sdp };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(53);
				match { let _sdp = recog.base.dfa_predict(1)?; _sdp } {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(39);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(40);
						recog.base.match_token(Repro4TwoCastWithWrap_T__1,&mut recog.err_handler)?;
						recog.base.set_state(41);
						recog.sub()?;
						recog.base.set_state(42);
						recog.base.match_token(Repro4TwoCastWithWrap_T__5,&mut recog.err_handler)?;
						recog.base.set_state(43);
						recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
						recog.base.set_state(44);
						recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
						recog.base.set_state(45);
						recog.base.match_token(Repro4TwoCastWithWrap_T__8,&mut recog.err_handler)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(47);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(48);
						recog.base.match_token(Repro4TwoCastWithWrap_T__9,&mut recog.err_handler)?;
						recog.base.set_state(49);
						recog.e_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(50);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(51);
						recog.base.match_token(Repro4TwoCastWithWrap_T__1,&mut recog.err_handler)?;
						recog.base.set_state(52);
						recog.sub()?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(57);
			_alt = { let _sdp = recog.base.dfa_predict(2)?; _sdp };
		}
		}
		})
	}
}
//------------------- sub ----------------
pub type SubContextAll<'input, 'arena, Tok = CommonToken<'input>> = SubContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { Repro4TwoCastWithWrapParserNodeKind, SubContext, SubContextExt, RULE_sub }

pub trait SubContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn sub(&self) -> Option<&'arena SubContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SubContextAttrs<'input, 'arena, Tok> for SubContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn sub(&self) -> Option<&'arena SubContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == Repro4TwoCastWithWrap_ID)
    }
}

impl<'input, 'arena, Input, TF> Repro4TwoCastWithWrapParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn sub(&mut self,) -> Result<&'arena SubContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SubContext<TF::Tok>, RULE_sub, 4, |_parentctx| SubContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SubContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(63);
		match recog.base.input.la(1) {
		    Repro4TwoCastWithWrap_T__4  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(58);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__4,&mut recog.err_handler)?;
		        recog.base.set_state(59);
		        recog.sub()?;
		        recog.base.set_state(60);
		        recog.base.match_token(Repro4TwoCastWithWrap_T__6,&mut recog.err_handler)?;
		        }}
		    Repro4TwoCastWithWrap_ID  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(62);
		        recog.base.match_token(Repro4TwoCastWithWrap_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
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
static _serializedATN: [u32; 846] = [
    1346458702, 3, 16909060, 29, 12, 66, 69, 0, 0, 4, 3, 29, 462, 491, 345, 
    836, 0, 836, 0, 836, 4, 840, 3, 843, 3, 846, 0, 836, 0, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 4294967295, 
    1, 0, 32, 3, 1, 4294967295, 4294967295, 1, 0, 8, 4, 1, 4294967295, 4294967295, 
    1, 0, 32, 5, 1, 4294967295, 4294967295, 1, 0, 8, 6, 1, 4294967295, 4294967295, 
    1, 0, 32, 7, 1, 4294967295, 4294967295, 1, 0, 32, 8, 1, 4294967295, 
    4294967295, 1, 0, 8, 9, 1, 4294967295, 4294967295, 1, 0, 32, 10, 1, 
    4294967295, 4294967295, 1, 0, 32, 11, 1, 4294967295, 4294967295, 1, 
    0, 8, 12, 1, 4294967295, 4294967295, 1, 0, 32, 13, 1, 4294967295, 4294967295, 
    1, 0, 32, 14, 1, 4294967295, 4294967295, 1, 0, 32, 15, 1, 4294967295, 
    4294967295, 1, 0, 32, 16, 1, 4294967295, 4294967295, 1, 0, 32, 17, 1, 
    4294967295, 4294967295, 1, 0, 8, 18, 1, 4294967295, 4294967295, 1, 0, 
    32, 19, 1, 4294967295, 4294967295, 1, 0, 32, 20, 1, 4294967295, 4294967295, 
    1, 0, 8, 21, 1, 4294967295, 4294967295, 1, 0, 32, 22, 1, 4294967295, 
    4294967295, 1, 0, 32, 23, 1, 4294967295, 4294967295, 1, 0, 32, 24, 1, 
    4294967295, 4294967295, 1, 0, 32, 25, 1, 4294967295, 4294967295, 1, 
    0, 32, 26, 1, 4294967295, 4294967295, 1, 0, 8, 27, 1, 4294967295, 4294967295, 
    1, 0, 8, 28, 1, 4294967295, 4294967295, 1, 0, 32, 29, 1, 4294967295, 
    4294967295, 1, 0, 8, 30, 1, 4294967295, 4294967295, 3, 0, 8, 31, 4, 
    35, 4294967295, 8, 0, 8, 35, 1, 4294967295, 4294967295, 1, 1, 72, 36, 
    1, 4294967295, 4294967295, 1, 1, 32, 37, 1, 4294967295, 4294967295, 
    1, 1, 8, 38, 1, 4294967295, 4294967295, 1, 1, 72, 39, 1, 4294967295, 
    4294967295, 1, 1, 32, 40, 1, 4294967295, 4294967295, 1, 1, 8, 41, 1, 
    4294967295, 4294967295, 1, 1, 32, 42, 1, 4294967295, 4294967295, 1, 
    1, 32, 43, 1, 4294967295, 4294967295, 1, 1, 32, 44, 1, 4294967295, 4294967295, 
    1, 1, 32, 45, 1, 4294967295, 4294967295, 1, 1, 8, 46, 1, 4294967295, 
    4294967295, 1, 1, 72, 47, 1, 4294967295, 4294967295, 1, 1, 32, 48, 1, 
    4294967295, 4294967295, 1, 1, 8, 49, 1, 4294967295, 4294967295, 1, 1, 
    72, 50, 1, 4294967295, 4294967295, 1, 1, 32, 51, 1, 4294967295, 4294967295, 
    1, 1, 8, 52, 1, 4294967295, 4294967295, 5, 1, 8, 53, 3, 54, 4294967295, 
    8, 1, 8, 56, 1, 4294967295, 4294967295, 10, 1, 10, 57, 2, 4294967295, 
    4294967295, 12, 1, 8, 59, 1, 4294967295, 57, 9, 1, 8, 60, 1, 4294967295, 
    4294967295, 1, 2, 32, 61, 1, 4294967295, 4294967295, 1, 2, 8, 62, 1, 
    4294967295, 4294967295, 1, 2, 32, 63, 1, 4294967295, 4294967295, 1, 
    2, 8, 64, 1, 4294967295, 4294967295, 1, 2, 32, 65, 1, 4294967295, 4294967295, 
    3, 2, 8, 66, 2, 64, 4294967295, 8, 2, 8, 68, 1, 4294967295, 4294967295, 
    1, 2, 0, 69, 0, 4294967295, 4294967295, 1, 34, 0, 0, 0, 1, 36, 0, 0, 
    0, 1, 63, 0, 0, 0, 5, 7, 1, 0, 0, 3, 2, 1, 8, 0, 5, 9, 2, 0, 0, 3, 4, 
    2, 10, 0, 5, 11, 3, 0, 0, 5, 12, 4294967295, 0, 0, 1, 35, 0, 0, 0, 5, 
    14, 4, 0, 0, 5, 15, 5, 0, 0, 3, 2, 1, 16, 0, 5, 17, 6, 0, 0, 5, 18, 
    11, 0, 0, 5, 19, 11, 0, 0, 5, 20, 7, 0, 0, 5, 21, 4294967295, 0, 0, 
    1, 35, 0, 0, 0, 5, 23, 8, 0, 0, 5, 24, 5, 0, 0, 3, 2, 1, 25, 0, 5, 26, 
    6, 0, 0, 5, 27, 11, 0, 0, 5, 28, 11, 0, 0, 5, 29, 7, 0, 0, 5, 30, 4294967295, 
    0, 0, 1, 35, 0, 0, 0, 3, 2, 1, 32, 0, 5, 33, 4294967295, 0, 0, 1, 35, 
    0, 0, 0, 1, 6, 0, 0, 0, 1, 13, 0, 0, 0, 1, 22, 0, 0, 0, 1, 31, 0, 0, 
    0, 1, 1, 0, 0, 0, 6, 37, 1, 4294967295, 0, 5, 38, 11, 0, 0, 1, 55, 0, 
    0, 0, 10, 40, 4, 0, 0, 5, 41, 2, 0, 0, 3, 4, 2, 42, 0, 5, 43, 6, 0, 
    0, 5, 44, 11, 0, 0, 5, 45, 11, 0, 0, 5, 46, 9, 0, 0, 1, 54, 0, 0, 0, 
    10, 48, 3, 0, 0, 5, 49, 10, 0, 0, 3, 2, 1, 54, 4, 10, 51, 2, 0, 0, 5, 
    52, 2, 0, 0, 3, 4, 2, 54, 0, 1, 39, 0, 0, 0, 1, 47, 0, 0, 0, 1, 50, 
    0, 0, 0, 1, 57, 0, 0, 0, 1, 53, 0, 0, 0, 1, 56, 0, 0, 0, 1, 3, 0, 0, 
    0, 1, 55, 0, 0, 0, 5, 59, 5, 0, 0, 3, 4, 2, 60, 0, 5, 61, 7, 0, 0, 1, 
    64, 0, 0, 0, 5, 64, 11, 0, 0, 1, 58, 0, 0, 0, 1, 62, 0, 0, 0, 1, 5, 
    0, 0, 0, 34, 53, 55, 63, 0, 2, 4, 1, 3, 5
];static _STATIC_PARSER_TABLES: LazyLock<StaticParserTables> = LazyLock::new(|| {
    StaticParserTables::from_image(&_STATIC_PARSER_TABLE_WORDS)
        .expect("invalid static parser DFA table image; regenerate the parser")
});
static _STATIC_PARSER_TABLE_WORDS: [u32; 446] = [
    1396983361, 4, 446, 3, 5, 1, 13, 4294967295, 0, 2147483648, 0, 105, 
    105, 83, 188, 83, 271, 110, 381, 27, 0, 3, 3, 7, 6, 2, 3, 4, 1, 2, 3, 
    4, 1, 1, 1, 2, 3, 4, 105, 1, 11, 8, 4294967295, 13, 100, 144, 11, 188, 
    212, 420, 0, 420, 0, 420, 0, 420, 0, 420, 0, 420, 0, 420, 0, 0, 0, 2, 
    0, 0, 1, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 3, 3, 65536, 196607, 
    4294901763, 4294901764, 393221, 7, 4294967295, 131071, 4294967295, 4294967295, 
    4294967295, 2147680255, 4294967295, 4294967295, 4294967295, 262143, 
    4294967295, 4294967295, 327679, 4294967295, 4294967295, 4294967295, 
    4294934533, 4294967295, 4294967295, 2147942406, 4294967295, 4294934535, 
    4294967295, 4294901768, 4294967295, 4294934535, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901764, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 655359, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901770, 
    4294967295, 4294967295, 4294967295, 2148007935, 2147876863, 4294967295, 
    83, 4294967295, 8, 5, 4294967295, 13, 100, 132, 8, 164, 180, 312, 1, 
    316, 2, 324, 2, 332, 0, 332, 0, 332, 0, 332, 0, 0, 2, 0, 1, 0, 0, 3221225985, 
    0, 2, 0, 2, 0, 2, 1, 0, 2, 4294901760, 4294901761, 196610, 327679, 4294934529, 
    196607, 4294967295, 2147614719, 4294967295, 2147745791, 4294967295, 
    4294967295, 4294967295, 327679, 4294967295, 4294967295, 393215, 4294967295, 
    4294967295, 4294967295, 4294934534, 4294967295, 4294967295, 2147680263, 
    4294967295, 2147581955, 4294967295, 4294934531, 4294967295, 4294934531, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901765, 4294967295, 
    65535, 7, 49, 52, 0, 1, 83, 4294967295, 8, 5, 4294967295, 13, 100, 132, 
    8, 164, 180, 312, 1, 316, 2, 324, 2, 332, 0, 332, 0, 332, 0, 332, 0, 
    0, 2, 0, 1, 0, 0, 3221225985, 0, 2, 0, 2, 0, 2, 2, 0, 2, 4294901760, 
    4294901761, 196610, 327679, 4294934529, 196607, 4294967295, 2147614719, 
    4294967295, 2147745791, 4294967295, 4294967295, 4294967295, 327679, 
    4294967295, 4294967295, 393215, 4294967295, 4294967295, 4294967295, 
    4294934534, 4294967295, 4294967295, 4294901767, 4294967295, 2147614719, 
    4294967295, 4294934531, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901765, 4294967295, 65535, 7, 49, 52, 0, 
    1, 110, 4294967295, 11, 8, 4294967295, 13, 100, 144, 11, 188, 212, 420, 
    1, 424, 2, 432, 2, 440, 0, 440, 0, 440, 0, 440, 0, 0, 2, 0, 0, 0, 3221225985, 
    0, 0, 0, 0, 1, 2, 0, 2, 2, 2, 0, 2, 2, 2, 2, 0, 4294901760, 131073, 
    4294901763, 327684, 458758, 65535, 4294934529, 196607, 4294967295, 2147614719, 
    4294967295, 2147614719, 4294967295, 4294967295, 4294967295, 262143, 
    4294967295, 4294967295, 327679, 4294967295, 4294967295, 4294967295, 
    4294934533, 4294967295, 4294967295, 2147549190, 4294967295, 2147581953, 
    4294967295, 4294901767, 4294967295, 4294934529, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901764, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 589823, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901769, 
    4294967295, 4294967295, 4294967295, 2147614719, 2148204543, 4294967295, 
    7, 49, 52, 0, 1, 27, 4294967295, 1, 0, 0, 0, 100, 104, 0, 104, 108, 
    108, 0, 108, 0, 108, 0, 108, 0, 108, 0, 108, 0, 108, 0, 2, 65535
];