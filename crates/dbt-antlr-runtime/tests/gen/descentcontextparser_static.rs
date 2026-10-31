// Generated from DescentContext.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const DescentContext_T__0:i32=1; 
pub const DescentContext_T__1:i32=2; 
pub const DescentContext_T__2:i32=3; 
pub const DescentContext_T__3:i32=4; 
pub const DescentContext_T__4:i32=5; 
pub const DescentContext_T__5:i32=6; 
pub const DescentContext_T__6:i32=7; 
pub const DescentContext_T__7:i32=8; 
pub const DescentContext_T__8:i32=9; 
pub const DescentContext_T__9:i32=10; 
pub const DescentContext_T__10:i32=11; 
pub const DescentContext_ID:i32=12; 
pub const DescentContext_INT:i32=13; 
pub const DescentContext_WS:i32=14;
pub const DescentContext_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_d:usize = 1; 
pub const RULE_r:usize = 2; 
pub const RULE_g:usize = 3; 
pub const RULE_k:usize = 4; 
pub const RULE_h:usize = 5; 
pub const RULE_e:usize = 6;
pub const ruleNames: [&'static str; 7] = [
    "s", "d", "r", "g", "k", "h", "e"
];

pub const _LITERAL_NAMES: [Option<&'static str>;12] = [
	None, Some("'one'"), Some("'prefix'"), Some("'two'"), Some("'three'"), 
	Some("'as'"), Some("';'"), Some("'+'"), Some("'('"), Some("')'"), Some("'.'"), 
	Some("','")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, DescentContextParserExt<'input, 'arena>, DescentContextParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				DescentContextParserExt {
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
        L: ParseTreeListener<'arena, DescentContextParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct DescentContextTreeWalker;
impl DescentContextTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, DescentContextParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, DescentContextParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <DescentContextParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum DescentContextParserNodeKind {
    SContext,
    DContext,
    RContext,
    GContext,
    KContext,
    HContext,
    EContext,
    Terminal,
    Error,
}
pub type DescentContextParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, DescentContextParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => DescentContextParser }
dbt_antlr_runtime::impl_node_kind! { DescentContextParserNodeKind {
; SContext(), DContext(), RContext(), GContext(), KContext(), HContext(), EContext(), 
    }; 
}

pub struct DescentContextParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> DescentContextParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { DescentContextParserExt, DescentContextParserNodeKind, "DescentContext.g4" }

//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, SContext, SContextExt, RULE_s }

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn d(&self) -> Option<&'arena DContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn g(&self) -> Option<&'arena GContextAll<'input, 'arena, Tok>>;
    fn k(&self) -> Option<&'arena KContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_EOF)
    }
    fn d(&self) -> Option<&'arena DContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_INT)
    }
    fn g(&self) -> Option<&'arena GContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn k(&self) -> Option<&'arena KContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_ID)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SContext<TF::Tok>, RULE_s, 0, |_parentctx| SContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(47);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    DescentContext_T__0  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(14);
		        recog.base.match_token(DescentContext_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(20);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.base.input.la(1) {
		            DescentContext_T__1  => {
		                {
		                recog.base.set_state(15);
		                recog.base.match_token(DescentContext_T__1,&mut recog.err_handler)?;
		                recog.base.set_state(16);
		                recog.d()?;
		                recog.base.set_state(17);
		                recog.base.match_token(DescentContext_INT,&mut recog.err_handler)?;
		                }}
		            DescentContext_T__7 |DescentContext_ID  => {
		                {
		                recog.base.set_state(19);
		                recog.d()?;
		                }}
		        	_ => Err(ANTLRError::no_alt(&mut recog.base))?
		        }
		        recog.base.set_state(22);
		        recog.base.match_token(DescentContext_EOF,&mut recog.err_handler)?;
		        }}
		    DescentContext_T__2  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(24);
		        recog.base.match_token(DescentContext_T__2,&mut recog.err_handler)?;
		        recog.base.set_state(30);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.base.input.la(1) {
		            DescentContext_T__1  => {
		                {
		                recog.base.set_state(25);
		                recog.base.match_token(DescentContext_T__1,&mut recog.err_handler)?;
		                recog.base.set_state(26);
		                recog.g()?;
		                recog.base.set_state(27);
		                recog.base.match_token(DescentContext_INT,&mut recog.err_handler)?;
		                }}
		            DescentContext_T__7 |DescentContext_ID  => {
		                {
		                recog.base.set_state(29);
		                recog.g()?;
		                }}
		        	_ => Err(ANTLRError::no_alt(&mut recog.base))?
		        }
		        recog.base.set_state(32);
		        recog.base.match_token(DescentContext_EOF,&mut recog.err_handler)?;
		        }}
		    DescentContext_T__3  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(34);
		        recog.base.match_token(DescentContext_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(43);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.base.input.la(1) {
		            DescentContext_T__1  => {
		                {
		                recog.base.set_state(35);
		                recog.base.match_token(DescentContext_T__1,&mut recog.err_handler)?;
		                recog.base.set_state(36);
		                recog.k()?;
		                recog.base.set_state(37);
		                recog.base.match_token(DescentContext_T__4,&mut recog.err_handler)?;
		                recog.base.set_state(38);
		                recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
		                recog.base.set_state(39);
		                recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
		                recog.base.set_state(40);
		                recog.base.match_token(DescentContext_INT,&mut recog.err_handler)?;
		                }}
		            DescentContext_T__7 |DescentContext_ID  => {
		                {
		                recog.base.set_state(42);
		                recog.k()?;
		                }}
		        	_ => Err(ANTLRError::no_alt(&mut recog.base))?
		        }
		        recog.base.set_state(45);
		        recog.base.match_token(DescentContext_EOF,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- d ----------------
pub type DContextAll<'input, 'arena, Tok = CommonToken<'input>> = DContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, DContext, DContextExt, RULE_d }

pub trait DContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn r(&self) -> Option<&'arena RContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DContextAttrs<'input, 'arena, Tok> for DContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn r(&self) -> Option<&'arena RContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).nth(i)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_INT)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn d(&mut self,) -> Result<&'arena DContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, DContext<TF::Tok>, RULE_d, 2, |_parentctx| DContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena DContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(60);
		recog.err_handler.sync(&mut recog.base)?;
		match {
			let _vap = recog.base.dfa_predict_virtual(4)?;
			match _vap {
				65536 => dbt_antlr_runtime::shared_descent!(
					recog,
					call_site: 49,
					prefix: 0,
					rule: RULE_r,
					invoke: r(),
					tails: { DescentContext_T__6  => 1, DescentContext_T__5  => 2, },
					guarded: { },
					default: 0,
					fallback: recog.get_interpreter().adaptive_predict(4,&mut recog.base)?,
				),
				INVALID_ALT => recog.get_interpreter().adaptive_predict(4,&mut recog.base)?,
				_vap => _vap,
			}
		} {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(49);
				recog.r()?;
				recog.base.set_state(50);
				recog.base.match_token(DescentContext_T__6,&mut recog.err_handler)?;
				recog.base.set_state(51);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				recog.base.set_state(52);
				recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
				recog.base.set_state(53);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(55);
				recog.r()?;
				recog.base.set_state(56);
				recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
				recog.base.set_state(57);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				recog.base.set_state(58);
				recog.base.match_token(DescentContext_INT,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- r ----------------
pub type RContextAll<'input, 'arena, Tok = CommonToken<'input>> = RContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, RContext, RContextExt, RULE_r }

pub trait RContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn r(&self) -> Option<&'arena RContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RContextAttrs<'input, 'arena, Tok> for RContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn r(&self) -> Option<&'arena RContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).nth(i)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r(&mut self,) -> Result<&'arena RContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_r)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, RContext<TF::Tok>, RULE_r, 4, |_parentctx| RContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena RContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(70);
		recog.err_handler.sync(&mut recog.base)?;
		match { let _sdp = recog.base.dfa_predict(5)?; if _sdp == INVALID_ALT { recog.get_interpreter().adaptive_predict(5,&mut recog.base)? } else { _sdp } } {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(62);
				recog.base.match_token(DescentContext_T__7,&mut recog.err_handler)?;
				recog.base.set_state(63);
				recog.r()?;
				recog.base.set_state(64);
				recog.base.match_token(DescentContext_T__8,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(66);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				recog.base.set_state(67);
				recog.base.match_token(DescentContext_T__6,&mut recog.err_handler)?;
				recog.base.set_state(68);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(69);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- g ----------------
pub type GContextAll<'input, 'arena, Tok = CommonToken<'input>> = GContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, GContext, GContextExt, RULE_g }

pub trait GContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn h(&self) -> Option<&'arena HContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> GContextAttrs<'input, 'arena, Tok> for GContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn h(&self) -> Option<&'arena HContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).nth(i)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_INT)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn g(&mut self,) -> Result<&'arena GContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, GContext<TF::Tok>, RULE_g, 6, |_parentctx| GContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena GContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(83);
		recog.err_handler.sync(&mut recog.base)?;
		match {
			let _vap = recog.base.dfa_predict_virtual(6)?;
			match _vap {
				65536 => dbt_antlr_runtime::shared_descent!(
					recog,
					call_site: 72,
					prefix: 0,
					rule: RULE_h,
					invoke: h(),
					tails: { DescentContext_T__4  => 1, DescentContext_T__5  => 2, },
					guarded: { },
					default: 0,
					fallback: recog.get_interpreter().adaptive_predict(6,&mut recog.base)?,
				),
				INVALID_ALT => recog.get_interpreter().adaptive_predict(6,&mut recog.base)?,
				_vap => _vap,
			}
		} {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(72);
				recog.h()?;
				recog.base.set_state(73);
				recog.base.match_token(DescentContext_T__4,&mut recog.err_handler)?;
				recog.base.set_state(74);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				recog.base.set_state(75);
				recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
				recog.base.set_state(76);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(78);
				recog.h()?;
				recog.base.set_state(79);
				recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
				recog.base.set_state(80);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				recog.base.set_state(81);
				recog.base.match_token(DescentContext_INT,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- k ----------------
pub type KContextAll<'input, 'arena, Tok = CommonToken<'input>> = KContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, KContext, KContextExt, RULE_k }

pub trait KContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn h(&self) -> Option<&'arena HContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> KContextAttrs<'input, 'arena, Tok> for KContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn h(&self) -> Option<&'arena HContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_ID)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_INT)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn k(&mut self,) -> Result<&'arena KContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, KContext<TF::Tok>, RULE_k, 8, |_parentctx| KContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena KContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(95);
		recog.err_handler.sync(&mut recog.base)?;
		match {
			let _vap = recog.base.dfa_predict_virtual(7)?;
			match _vap {
				65536 => dbt_antlr_runtime::shared_descent!(
					recog,
					call_site: 85,
					prefix: 0,
					rule: RULE_h,
					invoke: h(),
					tails: { DescentContext_T__4  => 1, DescentContext_T__5  => 2, },
					guarded: { },
					default: 0,
					fallback: recog.get_interpreter().adaptive_predict(7,&mut recog.base)?,
				),
				INVALID_ALT => recog.get_interpreter().adaptive_predict(7,&mut recog.base)?,
				_vap => _vap,
			}
		} {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(85);
				recog.h()?;
				recog.base.set_state(86);
				recog.base.match_token(DescentContext_T__4,&mut recog.err_handler)?;
				recog.base.set_state(87);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				recog.base.set_state(88);
				recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
				recog.base.set_state(89);
				recog.base.match_token(DescentContext_INT,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(91);
				recog.h()?;
				recog.base.set_state(92);
				recog.base.match_token(DescentContext_T__5,&mut recog.err_handler)?;
				recog.base.set_state(93);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- h ----------------
pub type HContextAll<'input, 'arena, Tok = CommonToken<'input>> = HContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, HContext, HContextExt, RULE_h }

pub trait HContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn h(&self) -> Option<&'arena HContextAll<'input, 'arena, Tok>>;
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> HContextAttrs<'input, 'arena, Tok> for HContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn h(&self) -> Option<&'arena HContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == DescentContext_ID)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn h(&mut self,) -> Result<&'arena HContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_h)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, HContext<TF::Tok>, RULE_h, 10, |_parentctx| HContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena HContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(106);
		recog.err_handler.sync(&mut recog.base)?;
		match {
			let _vap = recog.base.dfa_predict_virtual(8)?;
			match _vap {
				65536 => dbt_antlr_runtime::shared_descent!(
					recog,
					call_site: 101,
					prefix: 0,
					rule: RULE_e,
					invoke: e(),
					tails: { },
					guarded: { DescentContext_T__4  => 2, },
					default: 0,
					fallback: recog.get_interpreter().adaptive_predict(8,&mut recog.base)?,
				),
				INVALID_ALT => recog.get_interpreter().adaptive_predict(8,&mut recog.base)?,
				_vap => _vap,
			}
		} {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(97);
				recog.base.match_token(DescentContext_T__7,&mut recog.err_handler)?;
				recog.base.set_state(98);
				recog.h()?;
				recog.base.set_state(99);
				recog.base.match_token(DescentContext_T__8,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(101);
				recog.e()?;
				recog.base.set_state(102);
				recog.base.match_token(DescentContext_T__4,&mut recog.err_handler)?;
				recog.base.set_state(103);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(105);
				recog.e()?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- e ----------------
pub type EContextAll<'input, 'arena, Tok = CommonToken<'input>> = EContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DescentContextParserNodeKind, EContext, EContextExt, RULE_e }

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
}

impl<'input, 'arena, Tok: Token + 'input> EContextAttrs<'input, 'arena, Tok> for EContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == DescentContext_ID).nth(i)
    }
}

impl<'input, 'arena, Input, TF> DescentContextParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn e(&mut self,) -> Result<&'arena EContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		if let Some(_node) = self.base.resume_take(RULE_e)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, EContext<TF::Tok>, RULE_e, 12, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(108);
		recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
		recog.base.set_state(120);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==DescentContext_T__9 {
			{
			{
			recog.base.set_state(109);
			recog.base.match_token(DescentContext_T__9,&mut recog.err_handler)?;
			recog.base.set_state(110);
			recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
			recog.base.set_state(115);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==DescentContext_T__10 {
				{
				{
				recog.base.set_state(111);
				recog.base.match_token(DescentContext_T__10,&mut recog.err_handler)?;
				recog.base.set_state(112);
				recog.base.match_token(DescentContext_ID,&mut recog.err_handler)?;
				}
				}
				recog.base.set_state(117);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			}
			recog.base.set_state(122);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
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
static _serializedATN: [u32; 2233] = [
    1346458702, 3, 16909060, 29, 14, 124, 130, 0, 0, 11, 7, 29, 868, 897, 
    650, 1547, 0, 1547, 0, 1547, 11, 1558, 7, 1565, 7, 2233, 0, 1547, 0, 
    2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 4294967295, 
    2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 4294967295, 
    2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 4294967295, 
    2, 5, 8, 5, 1, 4294967295, 4294967295, 7, 5, 16, 6, 0, 4294967295, 4294967295, 
    2, 6, 8, 6, 1, 4294967295, 4294967295, 7, 6, 16, 7, 0, 4294967295, 4294967295, 
    1, 0, 32, 7, 1, 4294967295, 4294967295, 1, 0, 32, 8, 1, 4294967295, 
    4294967295, 1, 0, 8, 9, 1, 4294967295, 4294967295, 1, 0, 32, 10, 1, 
    4294967295, 4294967295, 1, 0, 8, 11, 1, 4294967295, 4294967295, 1, 0, 
    8, 12, 1, 4294967295, 4294967295, 3, 0, 8, 13, 2, 21, 4294967295, 8, 
    0, 8, 15, 1, 4294967295, 4294967295, 1, 0, 32, 16, 1, 4294967295, 4294967295, 
    1, 0, 8, 17, 1, 4294967295, 4294967295, 1, 0, 32, 18, 1, 4294967295, 
    4294967295, 1, 0, 32, 19, 1, 4294967295, 4294967295, 1, 0, 8, 20, 1, 
    4294967295, 4294967295, 1, 0, 32, 21, 1, 4294967295, 4294967295, 1, 
    0, 8, 22, 1, 4294967295, 4294967295, 1, 0, 8, 23, 1, 4294967295, 4294967295, 
    3, 0, 8, 24, 2, 31, 4294967295, 8, 0, 8, 26, 1, 4294967295, 4294967295, 
    1, 0, 32, 27, 1, 4294967295, 4294967295, 1, 0, 8, 28, 1, 4294967295, 
    4294967295, 1, 0, 32, 29, 1, 4294967295, 4294967295, 1, 0, 32, 30, 1, 
    4294967295, 4294967295, 1, 0, 8, 31, 1, 4294967295, 4294967295, 1, 0, 
    32, 32, 1, 4294967295, 4294967295, 1, 0, 32, 33, 1, 4294967295, 4294967295, 
    1, 0, 32, 34, 1, 4294967295, 4294967295, 1, 0, 32, 35, 1, 4294967295, 
    4294967295, 1, 0, 8, 36, 1, 4294967295, 4294967295, 1, 0, 8, 37, 1, 
    4294967295, 4294967295, 3, 0, 8, 38, 2, 44, 4294967295, 8, 0, 8, 40, 
    1, 4294967295, 4294967295, 1, 0, 32, 41, 1, 4294967295, 4294967295, 
    1, 0, 8, 42, 1, 4294967295, 4294967295, 3, 0, 8, 43, 3, 48, 4294967295, 
    8, 0, 8, 46, 1, 4294967295, 4294967295, 1, 1, 8, 47, 1, 4294967295, 
    4294967295, 1, 1, 32, 48, 1, 4294967295, 4294967295, 1, 1, 32, 49, 1, 
    4294967295, 4294967295, 1, 1, 32, 50, 1, 4294967295, 4294967295, 1, 
    1, 32, 51, 1, 4294967295, 4294967295, 1, 1, 8, 52, 1, 4294967295, 4294967295, 
    1, 1, 8, 53, 1, 4294967295, 4294967295, 1, 1, 32, 54, 1, 4294967295, 
    4294967295, 1, 1, 32, 55, 1, 4294967295, 4294967295, 1, 1, 32, 56, 1, 
    4294967295, 4294967295, 1, 1, 8, 57, 1, 4294967295, 4294967295, 3, 1, 
    8, 58, 2, 61, 4294967295, 8, 1, 8, 60, 1, 4294967295, 4294967295, 1, 
    2, 32, 61, 1, 4294967295, 4294967295, 1, 2, 8, 62, 1, 4294967295, 4294967295, 
    1, 2, 32, 63, 1, 4294967295, 4294967295, 1, 2, 8, 64, 1, 4294967295, 
    4294967295, 1, 2, 32, 65, 1, 4294967295, 4294967295, 1, 2, 32, 66, 1, 
    4294967295, 4294967295, 1, 2, 32, 67, 1, 4294967295, 4294967295, 1, 
    2, 32, 68, 1, 4294967295, 4294967295, 3, 2, 8, 69, 3, 71, 4294967295, 
    8, 2, 8, 72, 1, 4294967295, 4294967295, 1, 3, 8, 73, 1, 4294967295, 
    4294967295, 1, 3, 32, 74, 1, 4294967295, 4294967295, 1, 3, 32, 75, 1, 
    4294967295, 4294967295, 1, 3, 32, 76, 1, 4294967295, 4294967295, 1, 
    3, 32, 77, 1, 4294967295, 4294967295, 1, 3, 8, 78, 1, 4294967295, 4294967295, 
    1, 3, 8, 79, 1, 4294967295, 4294967295, 1, 3, 32, 80, 1, 4294967295, 
    4294967295, 1, 3, 32, 81, 1, 4294967295, 4294967295, 1, 3, 32, 82, 1, 
    4294967295, 4294967295, 1, 3, 8, 83, 1, 4294967295, 4294967295, 3, 3, 
    8, 84, 2, 84, 4294967295, 8, 3, 8, 86, 1, 4294967295, 4294967295, 1, 
    4, 8, 87, 1, 4294967295, 4294967295, 1, 4, 32, 88, 1, 4294967295, 4294967295, 
    1, 4, 32, 89, 1, 4294967295, 4294967295, 1, 4, 32, 90, 1, 4294967295, 
    4294967295, 1, 4, 32, 91, 1, 4294967295, 4294967295, 1, 4, 8, 92, 1, 
    4294967295, 4294967295, 1, 4, 8, 93, 1, 4294967295, 4294967295, 1, 4, 
    32, 94, 1, 4294967295, 4294967295, 1, 4, 32, 95, 1, 4294967295, 4294967295, 
    1, 4, 8, 96, 1, 4294967295, 4294967295, 3, 4, 8, 97, 2, 96, 4294967295, 
    8, 4, 8, 99, 1, 4294967295, 4294967295, 1, 5, 32, 100, 1, 4294967295, 
    4294967295, 1, 5, 8, 101, 1, 4294967295, 4294967295, 1, 5, 32, 102, 
    1, 4294967295, 4294967295, 1, 5, 8, 103, 1, 4294967295, 4294967295, 
    1, 5, 8, 104, 1, 4294967295, 4294967295, 1, 5, 32, 105, 1, 4294967295, 
    4294967295, 1, 5, 32, 106, 1, 4294967295, 4294967295, 1, 5, 8, 107, 
    1, 4294967295, 4294967295, 1, 5, 8, 108, 1, 4294967295, 4294967295, 
    3, 5, 8, 109, 3, 107, 4294967295, 8, 5, 8, 112, 1, 4294967295, 4294967295, 
    1, 6, 32, 113, 1, 4294967295, 4294967295, 1, 6, 32, 114, 1, 4294967295, 
    4294967295, 1, 6, 32, 115, 1, 4294967295, 4294967295, 1, 6, 32, 116, 
    1, 4294967295, 4294967295, 1, 6, 32, 117, 1, 4294967295, 4294967295, 
    5, 6, 8, 118, 1, 114, 4294967295, 8, 6, 8, 119, 1, 4294967295, 4294967295, 
    10, 6, 8, 120, 2, 4294967295, 4294967295, 12, 6, 8, 122, 1, 4294967295, 
    117, 9, 6, 8, 123, 1, 4294967295, 4294967295, 5, 6, 8, 124, 1, 119, 
    4294967295, 8, 6, 8, 125, 1, 4294967295, 4294967295, 10, 6, 8, 126, 
    2, 4294967295, 4294967295, 12, 6, 8, 128, 1, 4294967295, 122, 9, 6, 
    8, 129, 1, 4294967295, 4294967295, 1, 6, 0, 130, 0, 4294967295, 4294967295, 
    1, 47, 0, 0, 0, 1, 60, 0, 0, 0, 1, 70, 0, 0, 0, 1, 83, 0, 0, 0, 1, 95, 
    0, 0, 0, 1, 106, 0, 0, 0, 1, 108, 0, 0, 0, 5, 20, 1, 0, 0, 5, 16, 2, 
    0, 0, 3, 2, 1, 17, 0, 5, 18, 13, 0, 0, 1, 21, 0, 0, 0, 3, 2, 1, 21, 
    0, 1, 15, 0, 0, 0, 1, 19, 0, 0, 0, 1, 22, 0, 0, 0, 5, 23, 4294967295, 
    0, 0, 1, 48, 0, 0, 0, 5, 30, 3, 0, 0, 5, 26, 2, 0, 0, 3, 6, 3, 27, 0, 
    5, 28, 13, 0, 0, 1, 31, 0, 0, 0, 3, 6, 3, 31, 0, 1, 25, 0, 0, 0, 1, 
    29, 0, 0, 0, 1, 32, 0, 0, 0, 5, 33, 4294967295, 0, 0, 1, 48, 0, 0, 0, 
    5, 43, 4, 0, 0, 5, 36, 2, 0, 0, 3, 8, 4, 37, 0, 5, 38, 5, 0, 0, 5, 39, 
    12, 0, 0, 5, 40, 6, 0, 0, 5, 41, 13, 0, 0, 1, 44, 0, 0, 0, 3, 8, 4, 
    44, 0, 1, 35, 0, 0, 0, 1, 42, 0, 0, 0, 1, 45, 0, 0, 0, 5, 46, 4294967295, 
    0, 0, 1, 48, 0, 0, 0, 1, 14, 0, 0, 0, 1, 24, 0, 0, 0, 1, 34, 0, 0, 0, 
    1, 1, 0, 0, 0, 3, 4, 2, 50, 0, 5, 51, 7, 0, 0, 5, 52, 12, 0, 0, 5, 53, 
    6, 0, 0, 5, 54, 12, 0, 0, 1, 61, 0, 0, 0, 3, 4, 2, 56, 0, 5, 57, 6, 
    0, 0, 5, 58, 12, 0, 0, 5, 59, 13, 0, 0, 1, 61, 0, 0, 0, 1, 49, 0, 0, 
    0, 1, 55, 0, 0, 0, 1, 3, 0, 0, 0, 5, 63, 8, 0, 0, 3, 4, 2, 64, 0, 5, 
    65, 9, 0, 0, 1, 71, 0, 0, 0, 5, 67, 12, 0, 0, 5, 68, 7, 0, 0, 5, 71, 
    12, 0, 0, 5, 71, 12, 0, 0, 1, 62, 0, 0, 0, 1, 66, 0, 0, 0, 1, 69, 0, 
    0, 0, 1, 5, 0, 0, 0, 3, 10, 5, 73, 0, 5, 74, 5, 0, 0, 5, 75, 12, 0, 
    0, 5, 76, 6, 0, 0, 5, 77, 12, 0, 0, 1, 84, 0, 0, 0, 3, 10, 5, 79, 0, 
    5, 80, 6, 0, 0, 5, 81, 12, 0, 0, 5, 82, 13, 0, 0, 1, 84, 0, 0, 0, 1, 
    72, 0, 0, 0, 1, 78, 0, 0, 0, 1, 7, 0, 0, 0, 3, 10, 5, 86, 0, 5, 87, 
    5, 0, 0, 5, 88, 12, 0, 0, 5, 89, 6, 0, 0, 5, 90, 13, 0, 0, 1, 96, 0, 
    0, 0, 3, 10, 5, 92, 0, 5, 93, 6, 0, 0, 5, 94, 12, 0, 0, 1, 96, 0, 0, 
    0, 1, 85, 0, 0, 0, 1, 91, 0, 0, 0, 1, 9, 0, 0, 0, 5, 98, 8, 0, 0, 3, 
    10, 5, 99, 0, 5, 100, 9, 0, 0, 1, 107, 0, 0, 0, 3, 12, 6, 102, 0, 5, 
    103, 5, 0, 0, 5, 104, 12, 0, 0, 1, 107, 0, 0, 0, 3, 12, 6, 107, 0, 1, 
    97, 0, 0, 0, 1, 101, 0, 0, 0, 1, 105, 0, 0, 0, 1, 11, 0, 0, 0, 5, 120, 
    12, 0, 0, 5, 110, 10, 0, 0, 5, 115, 12, 0, 0, 5, 112, 11, 0, 0, 5, 114, 
    12, 0, 0, 1, 111, 0, 0, 0, 1, 117, 0, 0, 0, 1, 113, 0, 0, 0, 1, 116, 
    0, 0, 0, 1, 119, 0, 0, 0, 1, 115, 0, 0, 0, 1, 109, 0, 0, 0, 1, 122, 
    0, 0, 0, 1, 118, 0, 0, 0, 1, 121, 0, 0, 0, 1, 13, 0, 0, 0, 1, 120, 0, 
    0, 0, 20, 30, 43, 47, 60, 70, 83, 95, 106, 115, 120, 0, 2, 4, 6, 8, 
    10, 12, 1, 3, 5, 7, 9, 11, 13, 1396983361, 2, 661, 5, 9, 4, 13, 14, 
    9, 42, 0, 0, 0, 65536, 0, 2, 0, 0, 0, 0, 65536, 1, 4294967295, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 3, 5, 6, 7, 8, 3, 3, 
    3, 0, 6, 12, 18, 18, 24, 27, 30, 36, 42, 8, 8, 1, 12, 12, 2, 8, 8, 3, 
    12, 12, 4, 6, 6, 5, 7, 7, 6, 7, 7, 7, 9, 9, 8, 12, 12, 9, 12, 12, 10, 
    6, 6, 5, 7, 7, 11, 6, 6, 12, 7, 7, 11, 0, 1, 12, 0, 2, 53, 1, 4294967295, 
    0, 57, 2, 4294967295, 0, 0, 0, 0, 5, 12, 14, 5, 33, 0, 1, 0, 3, 3, 0, 
    3, 0, 2, 4294967295, 2, 2, 0, 0, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 0, 1, 
    2, 1, 1, 3, 1, 4, 1, 1, 1, 1, 0, 6, 6, 18, 21, 33, 8, 8, 1, 12, 12, 
    2, 4294967295, 4294967295, 3, 6, 6, 4, 7, 7, 5, 9, 9, 6, 12, 12, 7, 
    4294967295, 4294967295, 8, 6, 6, 9, 7, 7, 10, 9, 9, 11, 0, 1, 9, 0, 
    2, 53, 3, 4294967295, 1, 57, 2, 4294967295, 1, 0, 0, 0, 6, 18, 14, 12, 
    66, 0, 0, 0, 65536, 0, 0, 2, 0, 0, 0, 0, 0, 0, 65536, 1, 65536, 4294967295, 
    65536, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 
    3, 4, 5, 3, 6, 7, 8, 9, 10, 11, 3, 3, 3, 3, 3, 0, 6, 12, 21, 21, 30, 
    33, 36, 39, 45, 48, 54, 66, 8, 8, 1, 12, 12, 2, 8, 8, 3, 12, 12, 4, 
    5, 5, 5, 6, 6, 6, 10, 10, 7, 5, 5, 8, 9, 9, 9, 10, 10, 10, 12, 12, 11, 
    12, 12, 12, 12, 12, 13, 5, 5, 14, 6, 6, 6, 12, 12, 15, 5, 5, 14, 6, 
    6, 16, 5, 5, 5, 6, 6, 6, 10, 10, 7, 11, 11, 17, 0, 1, 16, 0, 2, 76, 
    1, 4294967295, 0, 80, 2, 4294967295, 0, 0, 0, 0, 7, 18, 14, 12, 66, 
    0, 0, 0, 65536, 0, 0, 2, 0, 0, 0, 0, 0, 0, 65536, 1, 65536, 4294967295, 
    65536, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 
    3, 4, 5, 3, 6, 7, 8, 9, 10, 11, 3, 3, 3, 3, 3, 0, 6, 12, 21, 21, 30, 
    33, 36, 39, 45, 48, 54, 66, 8, 8, 1, 12, 12, 2, 8, 8, 3, 12, 12, 4, 
    5, 5, 5, 6, 6, 6, 10, 10, 7, 5, 5, 8, 9, 9, 9, 10, 10, 10, 12, 12, 11, 
    12, 12, 12, 12, 12, 13, 5, 5, 14, 6, 6, 6, 12, 12, 15, 5, 5, 14, 6, 
    6, 16, 5, 5, 5, 6, 6, 6, 10, 10, 7, 11, 11, 17, 0, 1, 16, 0, 2, 89, 
    1, 4294967295, 0, 93, 2, 4294967295, 0, 0, 0, 0, 8, 15, 14, 7, 57, 0, 
    1, 0, 3, 0, 3, 3, 0, 0, 0, 2, 2, 4294967295, 2, 65536, 0, 0, 3, 3, 3, 
    3, 3, 0, 2, 3, 2, 2, 2, 2, 0, 0, 1, 2, 1, 3, 1, 1, 4, 5, 6, 1, 1, 1, 
    1, 1, 0, 6, 6, 21, 24, 27, 39, 57, 8, 8, 1, 12, 12, 2, 4294967295, 4294967295, 
    3, 5, 5, 4, 6, 6, 5, 9, 9, 6, 10, 10, 7, 12, 12, 8, 12, 12, 9, 4294967295, 
    4294967295, 10, 5, 5, 11, 6, 6, 12, 9, 9, 13, 4294967295, 4294967295, 
    3, 5, 5, 4, 6, 6, 5, 9, 9, 6, 10, 10, 7, 11, 11, 14, 0, 1, 12, 0, 4, 
    76, 3, 4294967295, 1, 80, 2, 4294967295, 1, 89, 3, 4294967295, 1, 93, 
    2, 4294967295, 1, 0, 0, 0, 0
];