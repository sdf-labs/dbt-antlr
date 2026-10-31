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
pub const FuzzExpr_T__13:i32=14; 
pub const FuzzExpr_T__14:i32=15; 
pub const FuzzExpr_T__15:i32=16; 
pub const FuzzExpr_T__16:i32=17; 
pub const FuzzExpr_ID:i32=18; 
pub const FuzzExpr_INT:i32=19; 
pub const FuzzExpr_WS:i32=20;
pub const FuzzExpr_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_item:usize = 1; 
pub const RULE_alias:usize = 2; 
pub const RULE_binding:usize = 3; 
pub const RULE_target:usize = 4; 
pub const RULE_name:usize = 5; 
pub const RULE_q:usize = 6; 
pub const RULE_e:usize = 7; 
pub const RULE_atom:usize = 8;
pub const ruleNames: [&'static str; 9] = [
    "s", "item", "alias", "binding", "target", "name", "q", "e", "atom"
];

pub const _LITERAL_NAMES: [Option<&'static str>;18] = [
	None, Some("','"), Some("';'"), Some("'let'"), Some("'alias'"), Some("'as'"), 
	Some("'pick'"), Some("'='"), Some("'.'"), Some("'['"), Some("']'"), Some("'-'"), 
	Some("'^'"), Some("'*'"), Some("'/'"), Some("'+'"), Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;21]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, None, Some("ID"), Some("INT"), Some("WS")
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
    AliasContext,
    BindingContext,
    TargetContext,
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
; SContext(), ItemContext(), AliasContext(), BindingContext(), TargetContext(), NameContext(), QContext(), EContext(), AtomContext(), 
    }; 
}

pub struct FuzzExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> FuzzExprParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { FuzzExprParserExt, FuzzExprParserNodeKind, "FuzzExpr.g4"; sempred (FuzzExprParserNode<'input, 'arena, TF::Tok>, FuzzExprParser) { 7 => e_sempred, } }

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
    fn binding(&self) -> Option<&'arena BindingContextAll<'input, 'arena, Tok>>;
    fn alias(&self) -> Option<&'arena AliasContextAll<'input, 'arena, Tok>>;
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
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
    fn binding(&self) -> Option<&'arena BindingContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn alias(&self) -> Option<&'arena AliasContextAll<'input, 'arena, Tok>> {
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
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SContext<TF::Tok>, RULE_s, 0, |_parentctx| SContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(45);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    FuzzExpr_T__4 |FuzzExpr_T__10 |FuzzExpr_T__15 |FuzzExpr_ID |FuzzExpr_INT  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(18);
		        recog.item()?;
		        recog.base.set_state(23);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        while _la==FuzzExpr_T__0 {
		        	{
		        	{
		        	recog.base.set_state(19);
		        	recog.base.match_token(FuzzExpr_T__0,&mut recog.err_handler)?;
		        	recog.base.set_state(20);
		        	recog.item()?;
		        	}
		        	}
		        	recog.base.set_state(25);
		        	recog.err_handler.sync(&mut recog.base)?;
		        	_la = recog.base.input.la(1);
		        }
		        recog.base.set_state(27);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        if _la==FuzzExpr_T__1 {
		        	{
		        	recog.base.set_state(26);
		        	recog.base.match_token(FuzzExpr_T__1,&mut recog.err_handler)?;
		        	}
		        }

		        recog.base.set_state(29);
		        recog.base.match_token(FuzzExpr_EOF,&mut recog.err_handler)?;
		        }}
		    FuzzExpr_T__2  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(31);
		        recog.base.match_token(FuzzExpr_T__2,&mut recog.err_handler)?;
		        recog.base.set_state(32);
		        recog.binding()?;
		        recog.base.set_state(33);
		        recog.base.match_token(FuzzExpr_EOF,&mut recog.err_handler)?;
		        }}
		    FuzzExpr_T__3  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(35);
		        recog.base.match_token(FuzzExpr_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(36);
		        recog.alias()?;
		        recog.base.set_state(37);
		        recog.base.match_token(FuzzExpr_T__4,&mut recog.err_handler)?;
		        recog.base.set_state(38);
		        recog.name()?;
		        recog.base.set_state(39);
		        recog.base.match_token(FuzzExpr_EOF,&mut recog.err_handler)?;
		        }}
		    FuzzExpr_T__5  => {
		        /*------- Outer Most Alt 4 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
		        {
		        recog.base.set_state(41);
		        recog.base.match_token(FuzzExpr_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(42);
		        recog.alias()?;
		        recog.base.set_state(43);
		        recog.base.match_token(FuzzExpr_EOF,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
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
		recog.base.set_state(52);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(3,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(47);
				recog.e_rec(0)?;
				recog.base.set_state(48);
				recog.base.match_token(FuzzExpr_T__4,&mut recog.err_handler)?;
				recog.base.set_state(49);
				recog.name()?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(51);
				recog.e_rec(0)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- alias ----------------
pub type AliasContextAll<'input, 'arena, Tok = CommonToken<'input>> = AliasContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, AliasContext, AliasContextExt, RULE_alias }

pub trait AliasContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
    fn name(&self) -> Option<&'arena NameContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AliasContextAttrs<'input, 'arena, Tok> for AliasContext<'input, 'arena, Tok>
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
	pub fn alias(&mut self,) -> Result<&'arena AliasContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AliasContext<TF::Tok>, RULE_alias, 4, |_parentctx| AliasContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AliasContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(59);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(4,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(54);
				recog.e_rec(0)?;
				recog.base.set_state(55);
				recog.base.match_token(FuzzExpr_T__4,&mut recog.err_handler)?;
				recog.base.set_state(56);
				recog.name()?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(58);
				recog.e_rec(0)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- binding ----------------
pub type BindingContextAll<'input, 'arena, Tok = CommonToken<'input>> = BindingContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, BindingContext, BindingContextExt, RULE_binding }

pub trait BindingContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn target(&self) -> Option<&'arena TargetContextAll<'input, 'arena, Tok>>;
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> BindingContextAttrs<'input, 'arena, Tok> for BindingContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn target(&self) -> Option<&'arena TargetContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn binding(&mut self,) -> Result<&'arena BindingContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, BindingContext<TF::Tok>, RULE_binding, 6, |_parentctx| BindingContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena BindingContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(66);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(5,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(61);
				recog.target()?;
				recog.base.set_state(62);
				recog.base.match_token(FuzzExpr_T__6,&mut recog.err_handler)?;
				recog.base.set_state(63);
				recog.e_rec(0)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(65);
				recog.e_rec(0)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- target ----------------
pub type TargetContextAll<'input, 'arena, Tok = CommonToken<'input>> = TargetContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { FuzzExprParserNodeKind, TargetContext, TargetContextExt, RULE_target }

pub trait TargetContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TargetContextAttrs<'input, 'arena, Tok> for TargetContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> FuzzExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn target(&mut self,) -> Result<&'arena TargetContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TargetContext<TF::Tok>, RULE_target, 8, |_parentctx| TargetContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TargetContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(68);
		recog.e_rec(0)?;
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
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, NameContext<TF::Tok>, RULE_name, 10, |_parentctx| NameContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena NameContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(70);
		_la = recog.base.input.la(1);
		if { !(_la==FuzzExpr_T__4 || _la==FuzzExpr_ID) } {
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
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, QContext<TF::Tok>, RULE_q, 12, |_parentctx| QContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena QContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(72);
		recog.name()?;
		recog.base.set_state(77);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(6,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				{
				{
				recog.base.set_state(73);
				recog.base.match_token(FuzzExpr_T__7,&mut recog.err_handler)?;
				recog.base.set_state(74);
				recog.name()?;
				}
				} 
			}
			recog.base.set_state(79);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(6,&mut recog.base)?;
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
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, EContext<TF::Tok>, RULE_e, 14, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 14;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(84);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    FuzzExpr_T__10  => {
		        {
		        recog.base.set_state(81);
		        recog.base.match_token(FuzzExpr_T__10,&mut recog.err_handler)?;
		        recog.base.set_state(82);
		        recog.e_rec(5)?;
		        }}
		    FuzzExpr_T__4 |FuzzExpr_T__15 |FuzzExpr_ID |FuzzExpr_INT  => {
		        {
		        recog.base.set_state(83);
		        recog.atom()?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(102);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(9,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(100);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.get_interpreter().adaptive_predict(8,&mut recog.base)? {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(86);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(87);
						recog.base.match_token(FuzzExpr_T__8,&mut recog.err_handler)?;
						recog.base.set_state(88);
						recog.e_rec(0)?;
						recog.base.set_state(89);
						recog.base.match_token(FuzzExpr_T__9,&mut recog.err_handler)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(91);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(92);
						recog.base.match_token(FuzzExpr_T__11,&mut recog.err_handler)?;
						recog.base.set_state(93);
						recog.e_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(94);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(95);
						_la = recog.base.input.la(1);
						if { !(_la==FuzzExpr_T__12 || _la==FuzzExpr_T__13) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(96);
						recog.e_rec(4)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_e)?;

						recog.base.set_state(97);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(98);
						_la = recog.base.input.la(1);
						if { !(_la==FuzzExpr_T__10 || _la==FuzzExpr_T__14) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(99);
						recog.e_rec(3)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(104);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(9,&mut recog.base)?;
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
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AtomContext<TF::Tok>, RULE_atom, 16, |_parentctx| AtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(124);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(12,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(105);
				recog.q()?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(106);
				recog.base.match_token(FuzzExpr_INT,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(107);
				recog.base.match_token(FuzzExpr_T__15,&mut recog.err_handler)?;
				recog.base.set_state(108);
				recog.e_rec(0)?;
				recog.base.set_state(109);
				recog.base.match_token(FuzzExpr_T__16,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(111);
				recog.base.match_token(FuzzExpr_ID,&mut recog.err_handler)?;
				recog.base.set_state(112);
				recog.base.match_token(FuzzExpr_T__15,&mut recog.err_handler)?;
				recog.base.set_state(121);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if (((_la) & !0x3f) == 0 && ((1usize << _la) & 854048) != 0) {
					{
					recog.base.set_state(113);
					recog.e_rec(0)?;
					recog.base.set_state(118);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==FuzzExpr_T__0 {
						{
						{
						recog.base.set_state(114);
						recog.base.match_token(FuzzExpr_T__0,&mut recog.err_handler)?;
						recog.base.set_state(115);
						recog.e_rec(0)?;
						}
						}
						recog.base.set_state(120);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				recog.base.set_state(123);
				recog.base.match_token(FuzzExpr_T__16,&mut recog.err_handler)?;
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
static _serializedATN: [u32; 1666] = [
    1346458702, 3, 16909060, 29, 20, 127, 136, 3, 5, 13, 9, 29, 889, 918, 
    680, 1598, 15, 1613, 10, 1635, 13, 1648, 9, 1657, 9, 1666, 6, 1623, 
    12, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 
    4294967295, 2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 
    4294967295, 2, 5, 8, 5, 1, 4294967295, 4294967295, 7, 5, 16, 6, 0, 4294967295, 
    4294967295, 2, 6, 8, 6, 1, 4294967295, 4294967295, 7, 6, 16, 7, 0, 4294967295, 
    4294967295, 2, 7, 12, 7, 1, 4294967295, 4294967295, 7, 7, 16, 8, 0, 
    4294967295, 4294967295, 2, 8, 8, 8, 1, 4294967295, 4294967295, 7, 8, 
    16, 9, 0, 4294967295, 4294967295, 1, 0, 8, 9, 1, 4294967295, 4294967295, 
    1, 0, 32, 10, 1, 4294967295, 4294967295, 1, 0, 8, 11, 1, 4294967295, 
    4294967295, 5, 0, 8, 12, 1, 22, 4294967295, 8, 0, 8, 13, 1, 4294967295, 
    4294967295, 10, 0, 8, 14, 2, 4294967295, 4294967295, 12, 0, 8, 16, 1, 
    4294967295, 25, 9, 0, 8, 17, 1, 4294967295, 4294967295, 1, 0, 32, 18, 
    1, 4294967295, 4294967295, 3, 0, 8, 19, 2, 28, 4294967295, 8, 0, 8, 
    21, 1, 4294967295, 4294967295, 1, 0, 32, 22, 1, 4294967295, 4294967295, 
    1, 0, 8, 23, 1, 4294967295, 4294967295, 1, 0, 32, 24, 1, 4294967295, 
    4294967295, 1, 0, 8, 25, 1, 4294967295, 4294967295, 1, 0, 32, 26, 1, 
    4294967295, 4294967295, 1, 0, 8, 27, 1, 4294967295, 4294967295, 1, 0, 
    32, 28, 1, 4294967295, 4294967295, 1, 0, 8, 29, 1, 4294967295, 4294967295, 
    1, 0, 32, 30, 1, 4294967295, 4294967295, 1, 0, 8, 31, 1, 4294967295, 
    4294967295, 1, 0, 32, 32, 1, 4294967295, 4294967295, 1, 0, 8, 33, 1, 
    4294967295, 4294967295, 1, 0, 32, 34, 1, 4294967295, 4294967295, 1, 
    0, 8, 35, 1, 4294967295, 4294967295, 1, 0, 32, 36, 1, 4294967295, 4294967295, 
    1, 0, 8, 37, 1, 4294967295, 4294967295, 3, 0, 8, 38, 4, 46, 4294967295, 
    8, 0, 8, 42, 1, 4294967295, 4294967295, 1, 1, 8, 43, 1, 4294967295, 
    4294967295, 1, 1, 32, 44, 1, 4294967295, 4294967295, 1, 1, 8, 45, 1, 
    4294967295, 4294967295, 1, 1, 8, 46, 1, 4294967295, 4294967295, 1, 1, 
    8, 47, 1, 4294967295, 4294967295, 3, 1, 8, 48, 2, 53, 4294967295, 8, 
    1, 8, 50, 1, 4294967295, 4294967295, 1, 2, 8, 51, 1, 4294967295, 4294967295, 
    1, 2, 32, 52, 1, 4294967295, 4294967295, 1, 2, 8, 53, 1, 4294967295, 
    4294967295, 1, 2, 8, 54, 1, 4294967295, 4294967295, 1, 2, 8, 55, 1, 
    4294967295, 4294967295, 3, 2, 8, 56, 2, 60, 4294967295, 8, 2, 8, 58, 
    1, 4294967295, 4294967295, 1, 3, 8, 59, 1, 4294967295, 4294967295, 1, 
    3, 32, 60, 1, 4294967295, 4294967295, 1, 3, 8, 61, 1, 4294967295, 4294967295, 
    1, 3, 8, 62, 1, 4294967295, 4294967295, 1, 3, 8, 63, 1, 4294967295, 
    4294967295, 3, 3, 8, 64, 2, 67, 4294967295, 8, 3, 8, 66, 1, 4294967295, 
    4294967295, 1, 4, 8, 67, 1, 4294967295, 4294967295, 1, 4, 8, 68, 1, 
    4294967295, 4294967295, 1, 5, 32, 69, 1, 4294967295, 4294967295, 1, 
    5, 8, 70, 1, 4294967295, 4294967295, 1, 6, 8, 71, 1, 4294967295, 4294967295, 
    1, 6, 32, 72, 1, 4294967295, 4294967295, 1, 6, 8, 73, 1, 4294967295, 
    4294967295, 5, 6, 8, 74, 1, 76, 4294967295, 8, 6, 8, 75, 1, 4294967295, 
    4294967295, 10, 6, 8, 76, 2, 4294967295, 4294967295, 12, 6, 8, 78, 1, 
    4294967295, 79, 9, 6, 8, 79, 1, 4294967295, 4294967295, 1, 7, 72, 80, 
    1, 4294967295, 4294967295, 1, 7, 32, 81, 1, 4294967295, 4294967295, 
    1, 7, 8, 82, 1, 4294967295, 4294967295, 1, 7, 8, 83, 1, 4294967295, 
    4294967295, 3, 7, 8, 84, 2, 85, 4294967295, 8, 7, 8, 86, 1, 4294967295, 
    4294967295, 1, 7, 72, 87, 1, 4294967295, 4294967295, 1, 7, 32, 88, 1, 
    4294967295, 4294967295, 1, 7, 8, 89, 1, 4294967295, 4294967295, 1, 7, 
    32, 90, 1, 4294967295, 4294967295, 1, 7, 8, 91, 1, 4294967295, 4294967295, 
    1, 7, 72, 92, 1, 4294967295, 4294967295, 1, 7, 32, 93, 1, 4294967295, 
    4294967295, 1, 7, 8, 94, 1, 4294967295, 4294967295, 1, 7, 72, 95, 1, 
    4294967295, 4294967295, 1, 7, 32, 96, 1, 4294967295, 4294967295, 1, 
    7, 8, 97, 1, 4294967295, 4294967295, 1, 7, 72, 98, 1, 4294967295, 4294967295, 
    1, 7, 32, 99, 1, 4294967295, 4294967295, 1, 7, 8, 100, 1, 4294967295, 
    4294967295, 5, 7, 8, 101, 4, 101, 4294967295, 8, 7, 8, 105, 1, 4294967295, 
    4294967295, 10, 7, 10, 106, 2, 4294967295, 4294967295, 12, 7, 8, 108, 
    1, 4294967295, 104, 9, 7, 8, 109, 1, 4294967295, 4294967295, 1, 8, 8, 
    110, 1, 4294967295, 4294967295, 1, 8, 32, 111, 1, 4294967295, 4294967295, 
    1, 8, 32, 112, 1, 4294967295, 4294967295, 1, 8, 8, 113, 1, 4294967295, 
    4294967295, 1, 8, 32, 114, 1, 4294967295, 4294967295, 1, 8, 8, 115, 
    1, 4294967295, 4294967295, 1, 8, 32, 116, 1, 4294967295, 4294967295, 
    1, 8, 32, 117, 1, 4294967295, 4294967295, 1, 8, 8, 118, 1, 4294967295, 
    4294967295, 1, 8, 32, 119, 1, 4294967295, 4294967295, 1, 8, 8, 120, 
    1, 4294967295, 4294967295, 5, 8, 8, 121, 1, 117, 4294967295, 8, 8, 8, 
    122, 1, 4294967295, 4294967295, 10, 8, 8, 123, 2, 4294967295, 4294967295, 
    12, 8, 8, 125, 1, 4294967295, 120, 9, 8, 8, 126, 1, 4294967295, 4294967295, 
    3, 8, 8, 127, 2, 122, 4294967295, 8, 8, 8, 129, 1, 4294967295, 4294967295, 
    1, 8, 32, 130, 1, 4294967295, 4294967295, 3, 8, 8, 131, 4, 125, 4294967295, 
    8, 8, 8, 135, 1, 4294967295, 4294967295, 1, 8, 0, 136, 0, 4294967295, 
    4294967295, 1, 45, 0, 0, 0, 1, 52, 0, 0, 0, 1, 59, 0, 0, 0, 1, 66, 0, 
    0, 0, 1, 68, 0, 0, 0, 1, 70, 0, 0, 0, 1, 72, 0, 0, 0, 1, 84, 0, 0, 0, 
    1, 124, 0, 0, 0, 3, 2, 1, 23, 0, 5, 20, 1, 0, 0, 3, 2, 1, 22, 0, 1, 
    19, 0, 0, 0, 1, 25, 0, 0, 0, 1, 21, 0, 0, 0, 1, 24, 0, 0, 0, 1, 27, 
    0, 0, 0, 1, 23, 0, 0, 0, 5, 28, 2, 0, 0, 1, 26, 0, 0, 0, 1, 28, 0, 0, 
    0, 1, 29, 0, 0, 0, 5, 30, 4294967295, 0, 0, 1, 46, 0, 0, 0, 5, 32, 3, 
    0, 0, 3, 6, 3, 33, 0, 5, 34, 4294967295, 0, 0, 1, 46, 0, 0, 0, 5, 36, 
    4, 0, 0, 3, 4, 2, 37, 0, 5, 38, 5, 0, 0, 3, 10, 5, 39, 0, 5, 40, 4294967295, 
    0, 0, 1, 46, 0, 0, 0, 5, 42, 6, 0, 0, 3, 4, 2, 43, 0, 5, 44, 4294967295, 
    0, 0, 1, 46, 0, 0, 0, 1, 18, 0, 0, 0, 1, 31, 0, 0, 0, 1, 35, 0, 0, 0, 
    1, 41, 0, 0, 0, 1, 1, 0, 0, 0, 3, 14, 7, 48, 0, 5, 49, 5, 0, 0, 3, 10, 
    5, 50, 0, 1, 53, 0, 0, 0, 3, 14, 7, 53, 0, 1, 47, 0, 0, 0, 1, 51, 0, 
    0, 0, 1, 3, 0, 0, 0, 3, 14, 7, 55, 0, 5, 56, 5, 0, 0, 3, 10, 5, 57, 
    0, 1, 60, 0, 0, 0, 3, 14, 7, 60, 0, 1, 54, 0, 0, 0, 1, 58, 0, 0, 0, 
    1, 5, 0, 0, 0, 3, 8, 4, 62, 0, 5, 63, 7, 0, 0, 3, 14, 7, 64, 0, 1, 67, 
    0, 0, 0, 3, 14, 7, 67, 0, 1, 61, 0, 0, 0, 1, 65, 0, 0, 0, 1, 7, 0, 0, 
    0, 3, 14, 7, 69, 0, 1, 9, 0, 0, 0, 7, 71, 0, 0, 0, 1, 11, 0, 0, 0, 3, 
    10, 5, 77, 0, 5, 74, 8, 0, 0, 3, 10, 5, 76, 0, 1, 73, 0, 0, 0, 1, 79, 
    0, 0, 0, 1, 75, 0, 0, 0, 1, 78, 0, 0, 0, 1, 13, 0, 0, 0, 1, 77, 0, 0, 
    0, 6, 81, 7, 4294967295, 0, 5, 82, 11, 0, 0, 3, 14, 7, 85, 5, 3, 16, 
    8, 85, 0, 1, 80, 0, 0, 0, 1, 83, 0, 0, 0, 1, 102, 0, 0, 0, 10, 87, 6, 
    0, 0, 5, 88, 9, 0, 0, 3, 14, 7, 89, 0, 5, 90, 10, 0, 0, 1, 101, 0, 0, 
    0, 10, 92, 4, 0, 0, 5, 93, 12, 0, 0, 3, 14, 7, 101, 4, 10, 95, 3, 0, 
    0, 7, 96, 1, 0, 0, 3, 14, 7, 101, 4, 10, 98, 2, 0, 0, 7, 99, 2, 0, 0, 
    3, 14, 7, 101, 3, 1, 86, 0, 0, 0, 1, 91, 0, 0, 0, 1, 94, 0, 0, 0, 1, 
    97, 0, 0, 0, 1, 104, 0, 0, 0, 1, 100, 0, 0, 0, 1, 103, 0, 0, 0, 1, 15, 
    0, 0, 0, 1, 102, 0, 0, 0, 3, 12, 6, 125, 0, 5, 125, 19, 0, 0, 5, 108, 
    16, 0, 0, 3, 14, 7, 109, 0, 5, 110, 17, 0, 0, 1, 125, 0, 0, 0, 5, 112, 
    18, 0, 0, 5, 121, 16, 0, 0, 3, 14, 7, 118, 0, 5, 115, 1, 0, 0, 3, 14, 
    7, 117, 0, 1, 114, 0, 0, 0, 1, 120, 0, 0, 0, 1, 116, 0, 0, 0, 1, 119, 
    0, 0, 0, 1, 122, 0, 0, 0, 1, 118, 0, 0, 0, 1, 113, 0, 0, 0, 1, 122, 
    0, 0, 0, 1, 123, 0, 0, 0, 5, 125, 17, 0, 0, 1, 105, 0, 0, 0, 1, 106, 
    0, 0, 0, 1, 107, 0, 0, 0, 1, 111, 0, 0, 0, 1, 17, 0, 0, 0, 0, 2, 1, 
    0, 2, 2, 1, 1, 2, 2, 3, 2, 1, 4, 2, 5, 5, 18, 18, 13, 14, 11, 11, 15, 
    15, 262176, 0, 0, 0, 24576, 0, 0, 0, 34816, 0, 0, 0, 23, 27, 45, 52, 
    59, 66, 77, 84, 100, 102, 118, 121, 124, 0, 2, 4, 6, 8, 10, 12, 14, 
    16, 1, 3, 5, 7, 9, 11, 13, 15, 17
];