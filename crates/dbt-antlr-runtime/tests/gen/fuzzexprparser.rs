// Generated from FuzzExpr.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const FuzzExpr_T__0:i32=1; 
pub const FuzzExpr_T__1:i32=2; 
pub const FuzzExpr_T__2:i32=3; 
pub const FuzzExpr_T__3:i32=4; 
pub const FuzzExpr_T__4:i32=5; 
pub const FuzzExpr_T__5:i32=6; 
pub const FuzzExpr_T__6:i32=7; 
pub const FuzzExpr_T__7:i32=8; 
pub const FuzzExpr_T__8:i32=9; 
pub const FuzzExpr_T__9:i32=10; 
pub const FuzzExpr_T__10:i32=11; 
pub const FuzzExpr_T__11:i32=12; 
pub const FuzzExpr_T__12:i32=13; 
pub const FuzzExpr_ID:i32=14; 
pub const FuzzExpr_INT:i32=15; 
pub const FuzzExpr_WS:i32=16;
pub const FuzzExpr_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_item:usize = 1; 
pub const RULE_name:usize = 2; 
pub const RULE_q:usize = 3; 
pub const RULE_e:usize = 4; 
pub const RULE_atom:usize = 5;
pub const ruleNames: [&'static str; 6] = [
    "s", "item", "name", "q", "e", "atom"
];

pub const _LITERAL_NAMES: [Option<&'static str>;14] = [
	None, Some("','"), Some("';'"), Some("'as'"), Some("'.'"), Some("'['"), 
	Some("']'"), Some("'-'"), Some("'^'"), Some("'*'"), Some("'/'"), Some("'+'"), 
	Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;17]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, FuzzExprParserExt<'input, 'arena>, FuzzExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				FuzzExprParserExt {
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
        L: ParseTreeListener<'arena, FuzzExprParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct FuzzExprTreeWalker;
impl FuzzExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, FuzzExprParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, FuzzExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <FuzzExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum FuzzExprParserNodeKind {
    SContext,
    ItemContext,
    NameContext,
    QContext,
    EContext,
    AtomContext,
    Terminal,
    Error,
}
pub type FuzzExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, FuzzExprParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => FuzzExprParser }
dbt_antlr_runtime::impl_node_kind! { FuzzExprParserNodeKind {
; SContext(), ItemContext(), NameContext(), QContext(), EContext(), AtomContext(), 
    }; 
}

pub struct FuzzExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> FuzzExprParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { FuzzExprParserExt, FuzzExprParserNodeKind, "FuzzExpr.g4"; sempred (FuzzExprParserNode<'input, 'arena, TF::Tok>, FuzzExprParser) { 4 => e_sempred, } }

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
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
			recog.precpred(None, 4)
		    }
	        2 => {
			recog.precpred(None, 3)
		    }
	        3 => {
			recog.precpred(None, 2)
		    }
		    _ => true
		}
	}
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, SContext, SContextExt, RULE_s }

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn item_all(&self) -> Vec<&'arena ItemContextAll<'input, 'arena, Tok>>;
    fn item(&self, i: usize) -> Option<&'arena ItemContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn item_all(&self) -> Vec<&'arena ItemContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn item(&self, i: usize) -> Option<&'arena ItemContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_EOF)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SContext<TF::Tok>, RULE_s, 0, |_parentctx| SContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(12);
		recog.item()?;
		recog.base.set_state(17);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==FuzzExpr_T__0 {
			{
			{
			recog.base.set_state(13);
			recog.base.match_token(FuzzExpr_T__0,&mut recog.err_handler)?;
			recog.base.set_state(14);
			recog.item()?;
			}
			}
			recog.base.set_state(19);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		recog.base.set_state(21);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==FuzzExpr_T__1 {
			{
			recog.base.set_state(20);
			recog.base.match_token(FuzzExpr_T__1,&mut recog.err_handler)?;
			}
		}

		recog.base.set_state(23);
		recog.base.match_token(FuzzExpr_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- item ----------------
pub type ItemContextAll<'input, 'arena, Tok = CommonToken<'input>> = ItemContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, ItemContext, ItemContextExt, RULE_item }

pub trait ItemContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ItemContextAttrs<'input, 'arena, Tok> for ItemContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn item(&mut self,) -> Result<&'arena ItemContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ItemContext<TF::Tok>, RULE_item, 2, |_parentctx| ItemContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ItemContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(30);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(2,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(25);
				recog.e_rec(0)?;
				recog.base.set_state(26);
				recog.base.match_token(FuzzExpr_T__2,&mut recog.err_handler)?;
				recog.base.set_state(27);
				recog.name()?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(29);
				recog.e_rec(0)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- name ----------------
pub type NameContextAll<'input, 'arena, Tok = CommonToken<'input>> = NameContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, NameContext, NameContextExt, RULE_name }

pub trait NameContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NameContextAttrs<'input, 'arena, Tok> for NameContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_ID)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn name(&mut self,) -> Result<&'arena NameContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, NameContext<TF::Tok>, RULE_name, 4, |_parentctx| NameContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena NameContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(32);
		_la = recog.base.input.la(1);
		if { !(_la==FuzzExpr_T__2 || _la==FuzzExpr_ID) } {
			recog.err_handler.recover_inline(&mut recog.base)?;
		}
		else {
			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
			recog.err_handler.report_match(&mut recog.base);
			recog.base.consume(&mut recog.err_handler)?;
		}
		}
		})
	}
}
//------------------- q ----------------
pub type QContextAll<'input, 'arena, Tok = CommonToken<'input>> = QContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, QContext, QContextExt, RULE_q }

pub trait QContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn name_all(&self) -> Vec<&'arena NameContextAll<'input, 'arena, Tok>>;
    fn name(&self, i: usize) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> QContextAttrs<'input, 'arena, Tok> for QContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn name_all(&self) -> Vec<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn name(&self, i: usize) -> Option<&'arena NameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn q(&mut self,) -> Result<&'arena QContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, QContext<TF::Tok>, RULE_q, 6, |_parentctx| QContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena QContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(34);
		recog.name()?;
		recog.base.set_state(39);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(3,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				{
				{
				recog.base.set_state(35);
				recog.base.match_token(FuzzExpr_T__3,&mut recog.err_handler)?;
				recog.base.set_state(36);
				recog.name()?;
				}
				} 
			}
			recog.base.set_state(41);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(3,&mut recog.base)?;
		}
		}
		})
	}
}
//------------------- e ----------------
pub type EContextAll<'input, 'arena, Tok = CommonToken<'input>> = EContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, EContext, EContextExt, RULE_e }

pub trait EContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
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
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
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
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, EContext<TF::Tok>, RULE_e, 8, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 8;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(46);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    FuzzExpr_T__6  => {
		        {
		        recog.base.set_state(43);
		        recog.base.match_token(FuzzExpr_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(44);
		        recog.e_rec(5)?;
		        }}
		    FuzzExpr_T__2 |FuzzExpr_T__11 |FuzzExpr_ID |FuzzExpr_INT  => {
		        {
		        recog.base.set_state(45);
		        recog.atom()?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(64);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(6,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(62);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.get_interpreter().adaptive_predict(5,&mut recog.base)? {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(48);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(49);
						recog.base.match_token(FuzzExpr_T__4,&mut recog.err_handler)?;
						recog.base.set_state(50);
						recog.e_rec(0)?;
						recog.base.set_state(51);
						recog.base.match_token(FuzzExpr_T__5,&mut recog.err_handler)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(53);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(54);
						recog.base.match_token(FuzzExpr_T__7,&mut recog.err_handler)?;
						recog.base.set_state(55);
						recog.e_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(56);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(57);
						_la = recog.base.input.la(1);
						if { !(_la==FuzzExpr_T__8 || _la==FuzzExpr_T__9) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(58);
						recog.e_rec(4)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(59);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(60);
						_la = recog.base.input.la(1);
						if { !(_la==FuzzExpr_T__6 || _la==FuzzExpr_T__10) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(61);
						recog.e_rec(3)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(66);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(6,&mut recog.base)?;
		}
		}
		})
	}
}
//------------------- atom ----------------
pub type AtomContextAll<'input, 'arena, Tok = CommonToken<'input>> = AtomContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, AtomContext, AtomContextExt, RULE_atom }

pub trait AtomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AtomContextAttrs<'input, 'arena, Tok> for AtomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_INT)
    }
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == FuzzExpr_ID)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn atom(&mut self,) -> Result<&'arena AtomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AtomContext<TF::Tok>, RULE_atom, 10, |_parentctx| AtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(86);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(9,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(67);
				recog.q()?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(68);
				recog.base.match_token(FuzzExpr_INT,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(69);
				recog.base.match_token(FuzzExpr_T__11,&mut recog.err_handler)?;
				recog.base.set_state(70);
				recog.e_rec(0)?;
				recog.base.set_state(71);
				recog.base.match_token(FuzzExpr_T__12,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(73);
				recog.base.match_token(FuzzExpr_ID,&mut recog.err_handler)?;
				recog.base.set_state(74);
				recog.base.match_token(FuzzExpr_T__11,&mut recog.err_handler)?;
				recog.base.set_state(83);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & 53384) != 0) {
					{
					recog.base.set_state(75);
					recog.e_rec(0)?;
					recog.base.set_state(80);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==FuzzExpr_T__0 {
						{
						{
						recog.base.set_state(76);
						recog.base.match_token(FuzzExpr_T__0,&mut recog.err_handler)?;
						recog.base.set_state(77);
						recog.e_rec(0)?;
						}
						}
						recog.base.set_state(82);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				recog.base.set_state(85);
				recog.base.match_token(FuzzExpr_T__12,&mut recog.err_handler)?;
				}
			}

			_ => {}
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
static _serializedATN: [u32; 1191] = [
    1346458702, 3, 16909060, 29, 16, 89, 96, 3, 5, 10, 6, 29, 623, 652, 
    480, 1132, 15, 1147, 10, 1169, 10, 1179, 6, 1185, 6, 1191, 6, 1157, 
    12, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 
    4294967295, 2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 12, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 
    4294967295, 4294967295, 2, 5, 8, 5, 1, 4294967295, 4294967295, 7, 5, 
    16, 6, 0, 4294967295, 4294967295, 1, 0, 8, 6, 1, 4294967295, 4294967295, 
    1, 0, 32, 7, 1, 4294967295, 4294967295, 1, 0, 8, 8, 1, 4294967295, 4294967295, 
    5, 0, 8, 9, 1, 16, 4294967295, 8, 0, 8, 10, 1, 4294967295, 4294967295, 
    10, 0, 8, 11, 2, 4294967295, 4294967295, 12, 0, 8, 13, 1, 4294967295, 
    19, 9, 0, 8, 14, 1, 4294967295, 4294967295, 1, 0, 32, 15, 1, 4294967295, 
    4294967295, 3, 0, 8, 16, 2, 22, 4294967295, 8, 0, 8, 18, 1, 4294967295, 
    4294967295, 1, 0, 32, 19, 1, 4294967295, 4294967295, 1, 0, 8, 20, 1, 
    4294967295, 4294967295, 1, 1, 8, 21, 1, 4294967295, 4294967295, 1, 1, 
    32, 22, 1, 4294967295, 4294967295, 1, 1, 8, 23, 1, 4294967295, 4294967295, 
    1, 1, 8, 24, 1, 4294967295, 4294967295, 1, 1, 8, 25, 1, 4294967295, 
    4294967295, 3, 1, 8, 26, 2, 31, 4294967295, 8, 1, 8, 28, 1, 4294967295, 
    4294967295, 1, 2, 32, 29, 1, 4294967295, 4294967295, 1, 2, 8, 30, 1, 
    4294967295, 4294967295, 1, 3, 8, 31, 1, 4294967295, 4294967295, 1, 3, 
    32, 32, 1, 4294967295, 4294967295, 1, 3, 8, 33, 1, 4294967295, 4294967295, 
    5, 3, 8, 34, 1, 38, 4294967295, 8, 3, 8, 35, 1, 4294967295, 4294967295, 
    10, 3, 8, 36, 2, 4294967295, 4294967295, 12, 3, 8, 38, 1, 4294967295, 
    41, 9, 3, 8, 39, 1, 4294967295, 4294967295, 1, 4, 72, 40, 1, 4294967295, 
    4294967295, 1, 4, 32, 41, 1, 4294967295, 4294967295, 1, 4, 8, 42, 1, 
    4294967295, 4294967295, 1, 4, 8, 43, 1, 4294967295, 4294967295, 3, 4, 
    8, 44, 2, 47, 4294967295, 8, 4, 8, 46, 1, 4294967295, 4294967295, 1, 
    4, 72, 47, 1, 4294967295, 4294967295, 1, 4, 32, 48, 1, 4294967295, 4294967295, 
    1, 4, 8, 49, 1, 4294967295, 4294967295, 1, 4, 32, 50, 1, 4294967295, 
    4294967295, 1, 4, 8, 51, 1, 4294967295, 4294967295, 1, 4, 72, 52, 1, 
    4294967295, 4294967295, 1, 4, 32, 53, 1, 4294967295, 4294967295, 1, 
    4, 8, 54, 1, 4294967295, 4294967295, 1, 4, 72, 55, 1, 4294967295, 4294967295, 
    1, 4, 32, 56, 1, 4294967295, 4294967295, 1, 4, 8, 57, 1, 4294967295, 
    4294967295, 1, 4, 72, 58, 1, 4294967295, 4294967295, 1, 4, 32, 59, 1, 
    4294967295, 4294967295, 1, 4, 8, 60, 1, 4294967295, 4294967295, 5, 4, 
    8, 61, 4, 63, 4294967295, 8, 4, 8, 65, 1, 4294967295, 4294967295, 10, 
    4, 10, 66, 2, 4294967295, 4294967295, 12, 4, 8, 68, 1, 4294967295, 66, 
    9, 4, 8, 69, 1, 4294967295, 4294967295, 1, 5, 8, 70, 1, 4294967295, 
    4294967295, 1, 5, 32, 71, 1, 4294967295, 4294967295, 1, 5, 32, 72, 1, 
    4294967295, 4294967295, 1, 5, 8, 73, 1, 4294967295, 4294967295, 1, 5, 
    32, 74, 1, 4294967295, 4294967295, 1, 5, 8, 75, 1, 4294967295, 4294967295, 
    1, 5, 32, 76, 1, 4294967295, 4294967295, 1, 5, 32, 77, 1, 4294967295, 
    4294967295, 1, 5, 8, 78, 1, 4294967295, 4294967295, 1, 5, 32, 79, 1, 
    4294967295, 4294967295, 1, 5, 8, 80, 1, 4294967295, 4294967295, 5, 5, 
    8, 81, 1, 79, 4294967295, 8, 5, 8, 82, 1, 4294967295, 4294967295, 10, 
    5, 8, 83, 2, 4294967295, 4294967295, 12, 5, 8, 85, 1, 4294967295, 82, 
    9, 5, 8, 86, 1, 4294967295, 4294967295, 3, 5, 8, 87, 2, 84, 4294967295, 
    8, 5, 8, 89, 1, 4294967295, 4294967295, 1, 5, 32, 90, 1, 4294967295, 
    4294967295, 3, 5, 8, 91, 4, 87, 4294967295, 8, 5, 8, 95, 1, 4294967295, 
    4294967295, 1, 5, 0, 96, 0, 4294967295, 4294967295, 1, 12, 0, 0, 0, 
    1, 30, 0, 0, 0, 1, 32, 0, 0, 0, 1, 34, 0, 0, 0, 1, 46, 0, 0, 0, 1, 86, 
    0, 0, 0, 3, 2, 1, 17, 0, 5, 14, 1, 0, 0, 3, 2, 1, 16, 0, 1, 13, 0, 0, 
    0, 1, 19, 0, 0, 0, 1, 15, 0, 0, 0, 1, 18, 0, 0, 0, 1, 21, 0, 0, 0, 1, 
    17, 0, 0, 0, 5, 22, 2, 0, 0, 1, 20, 0, 0, 0, 1, 22, 0, 0, 0, 1, 23, 
    0, 0, 0, 5, 24, 4294967295, 0, 0, 1, 1, 0, 0, 0, 3, 8, 4, 26, 0, 5, 
    27, 3, 0, 0, 3, 4, 2, 28, 0, 1, 31, 0, 0, 0, 3, 8, 4, 31, 0, 1, 25, 
    0, 0, 0, 1, 29, 0, 0, 0, 1, 3, 0, 0, 0, 7, 33, 0, 0, 0, 1, 5, 0, 0, 
    0, 3, 4, 2, 39, 0, 5, 36, 4, 0, 0, 3, 4, 2, 38, 0, 1, 35, 0, 0, 0, 1, 
    41, 0, 0, 0, 1, 37, 0, 0, 0, 1, 40, 0, 0, 0, 1, 7, 0, 0, 0, 1, 39, 0, 
    0, 0, 6, 43, 4, 4294967295, 0, 5, 44, 7, 0, 0, 3, 8, 4, 47, 5, 3, 10, 
    5, 47, 0, 1, 42, 0, 0, 0, 1, 45, 0, 0, 0, 1, 64, 0, 0, 0, 10, 49, 6, 
    0, 0, 5, 50, 5, 0, 0, 3, 8, 4, 51, 0, 5, 52, 6, 0, 0, 1, 63, 0, 0, 0, 
    10, 54, 4, 0, 0, 5, 55, 8, 0, 0, 3, 8, 4, 63, 4, 10, 57, 3, 0, 0, 7, 
    58, 1, 0, 0, 3, 8, 4, 63, 4, 10, 60, 2, 0, 0, 7, 61, 2, 0, 0, 3, 8, 
    4, 63, 3, 1, 48, 0, 0, 0, 1, 53, 0, 0, 0, 1, 56, 0, 0, 0, 1, 59, 0, 
    0, 0, 1, 66, 0, 0, 0, 1, 62, 0, 0, 0, 1, 65, 0, 0, 0, 1, 9, 0, 0, 0, 
    1, 64, 0, 0, 0, 3, 6, 3, 87, 0, 5, 87, 15, 0, 0, 5, 70, 12, 0, 0, 3, 
    8, 4, 71, 0, 5, 72, 13, 0, 0, 1, 87, 0, 0, 0, 5, 74, 14, 0, 0, 5, 83, 
    12, 0, 0, 3, 8, 4, 80, 0, 5, 77, 1, 0, 0, 3, 8, 4, 79, 0, 1, 76, 0, 
    0, 0, 1, 82, 0, 0, 0, 1, 78, 0, 0, 0, 1, 81, 0, 0, 0, 1, 84, 0, 0, 0, 
    1, 80, 0, 0, 0, 1, 75, 0, 0, 0, 1, 84, 0, 0, 0, 1, 85, 0, 0, 0, 5, 87, 
    13, 0, 0, 1, 67, 0, 0, 0, 1, 68, 0, 0, 0, 1, 69, 0, 0, 0, 1, 73, 0, 
    0, 0, 1, 11, 0, 0, 0, 0, 2, 1, 0, 2, 2, 1, 1, 2, 2, 3, 2, 1, 4, 2, 3, 
    3, 14, 14, 9, 10, 7, 7, 11, 11, 16392, 0, 0, 0, 1536, 0, 0, 0, 2176, 
    0, 0, 0, 17, 21, 30, 39, 46, 62, 64, 80, 83, 86, 0, 2, 4, 6, 8, 10, 
    1, 3, 5, 7, 9, 11
];