// Generated from InlineAlias.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","8");
pub const InlineAlias_T__0:i32=1; 
pub const InlineAlias_T__1:i32=2; 
pub const InlineAlias_T__2:i32=3; 
pub const InlineAlias_T__3:i32=4; 
pub const InlineAlias_T__4:i32=5; 
pub const InlineAlias_T__5:i32=6; 
pub const InlineAlias_T__6:i32=7; 
pub const InlineAlias_T__7:i32=8; 
pub const InlineAlias_T__8:i32=9; 
pub const InlineAlias_T__9:i32=10; 
pub const InlineAlias_T__10:i32=11; 
pub const InlineAlias_T__11:i32=12; 
pub const InlineAlias_T__12:i32=13; 
pub const InlineAlias_T__13:i32=14; 
pub const InlineAlias_T__14:i32=15; 
pub const InlineAlias_T__15:i32=16; 
pub const InlineAlias_ID:i32=17; 
pub const InlineAlias_INT:i32=18; 
pub const InlineAlias_WS:i32=19;
pub const InlineAlias_EOF:i32=EOF;
pub const RULE_stmt:usize = 0; 
pub const RULE_query:usize = 1; 
pub const RULE_expr:usize = 2; 
pub const RULE_alias:usize = 3; 
pub const RULE_type:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "stmt", "query", "expr", "alias", "type"
];

pub const _LITERAL_NAMES: [Option<&'static str>;17] = [
	None, Some("'CREATE'"), Some("'VIEW'"), Some("'ORDER'"), Some("'BY'"), 
	Some("','"), Some("'AS'"), Some("'SELECT'"), Some("'FROM'"), Some("'ROWS'"), 
	Some("'BETWEEN'"), Some("'AND'"), Some("'='"), Some("'CAST'"), Some("'('"), 
	Some("')'"), Some("'INT64'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;20]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, InlineAliasParserExt<'input, 'arena>, InlineAliasParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				InlineAliasParserExt {
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
        L: ParseTreeListener<'arena, InlineAliasParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct InlineAliasTreeWalker;
impl InlineAliasTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, InlineAliasParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, InlineAliasParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <InlineAliasParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum InlineAliasParserNodeKind {
    StmtContext,
    QueryContext,
    ExprContext,
    AliasContext,
    TypeContext,
    Terminal,
    Error,
}
pub type InlineAliasParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, InlineAliasParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => InlineAliasParser }
dbt_antlr_runtime::impl_node_kind! { InlineAliasParserNodeKind {
    ExprContext(ExprContextAll), ; StmtContext(), QueryContext(), AliasContext(), TypeContext(), 
    }; 
}

pub struct InlineAliasParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> InlineAliasParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { InlineAliasParserExt, InlineAliasParserNodeKind, "InlineAlias.g4"; sempred (InlineAliasParserNode<'input, 'arena, TF::Tok>, InlineAliasParser) { 2 => expr_sempred, } }

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn expr_sempred(_ctx: Option<&'arena ExprContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
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
//------------------- stmt ----------------
pub type StmtContextAll<'input, 'arena, Tok = CommonToken<'input>> = StmtContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { InlineAliasParserNodeKind, StmtContext, StmtContextExt, RULE_stmt }

pub trait StmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn query(&self) -> Option<&'arena QueryContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StmtContextAttrs<'input, 'arena, Tok> for StmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn query(&self) -> Option<&'arena QueryContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_EOF)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_ID)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stmt(&mut self,) -> Result<&'arena StmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StmtContext<TF::Tok>, RULE_stmt, 0, |_parentctx| StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(30);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    InlineAlias_T__6  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(10);
		        recog.query()?;
		        recog.base.set_state(11);
		        recog.base.match_token(InlineAlias_EOF,&mut recog.err_handler)?;
		        }}
		    InlineAlias_T__0  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(13);
		        recog.base.match_token(InlineAlias_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(14);
		        recog.base.match_token(InlineAlias_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(15);
		        recog.base.match_token(InlineAlias_ID,&mut recog.err_handler)?;
		        recog.base.set_state(16);
		        recog.base.match_token(InlineAlias_T__2,&mut recog.err_handler)?;
		        recog.base.set_state(17);
		        recog.base.match_token(InlineAlias_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(18);
		        recog.expr_rec(0)?;
		        recog.base.set_state(23);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        while _la==InlineAlias_T__4 {
		        	{
		        	{
		        	recog.base.set_state(19);
		        	recog.base.match_token(InlineAlias_T__4,&mut recog.err_handler)?;
		        	recog.base.set_state(20);
		        	recog.expr_rec(0)?;
		        	}
		        	}
		        	recog.base.set_state(25);
		        	recog.err_handler.sync(&mut recog.base)?;
		        	_la = recog.base.input.la(1);
		        }
		        recog.base.set_state(26);
		        recog.base.match_token(InlineAlias_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(27);
		        recog.query()?;
		        recog.base.set_state(28);
		        recog.base.match_token(InlineAlias_EOF,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- query ----------------
pub type QueryContextAll<'input, 'arena, Tok = CommonToken<'input>> = QueryContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { InlineAliasParserNodeKind, QueryContext, QueryContextExt, RULE_query }

pub trait QueryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
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

impl<'input, 'arena, Tok: Token + 'input> QueryContextAttrs<'input, 'arena, Tok> for QueryContext<'input, 'arena, Tok>
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
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_ID)
    }
}

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn query(&mut self,) -> Result<&'arena QueryContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, QueryContext<TF::Tok>, RULE_query, 2, |_parentctx| QueryContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena QueryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(32);
		recog.base.match_token(InlineAlias_T__6,&mut recog.err_handler)?;
		recog.base.set_state(33);
		recog.expr_rec(0)?;
		recog.base.set_state(38);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==InlineAlias_T__4 {
			{
			{
			recog.base.set_state(34);
			recog.base.match_token(InlineAlias_T__4,&mut recog.err_handler)?;
			recog.base.set_state(35);
			recog.expr_rec(0)?;
			}
			}
			recog.base.set_state(40);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		recog.base.set_state(43);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==InlineAlias_T__7 {
			{
			recog.base.set_state(41);
			recog.base.match_token(InlineAlias_T__7,&mut recog.err_handler)?;
			recog.base.set_state(42);
			recog.base.match_token(InlineAlias_ID,&mut recog.err_handler)?;
			}
		}

		recog.base.set_state(51);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==InlineAlias_T__8 {
			{
			recog.base.set_state(45);
			recog.base.match_token(InlineAlias_T__8,&mut recog.err_handler)?;
			recog.base.set_state(46);
			recog.base.match_token(InlineAlias_T__9,&mut recog.err_handler)?;
			recog.base.set_state(47);
			recog.expr_rec(0)?;
			recog.base.set_state(48);
			recog.base.match_token(InlineAlias_T__10,&mut recog.err_handler)?;
			recog.base.set_state(49);
			recog.expr_rec(0)?;
			}
		}

		}
		})
	}
}
//------------------- expr ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum ExprContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	CastContext(CastContext<'input, 'arena, Tok>),
	ColContext(ColContext<'input, 'arena, Tok>),
	ParenContext(ParenContext<'input, 'arena, Tok>),
	AliasedContext(AliasedContext<'input, 'arena, Tok>),
	AndContext(AndContext<'input, 'arena, Tok>),
	NumContext(NumContext<'input, 'arena, Tok>),
	EqContext(EqContext<'input, 'arena, Tok>),
    Error(ExprContext<'input, 'arena, Tok>)
}

dbt_antlr_runtime::impl_rule_context! { ExprContextAll { } { CastContext, ColContext, ParenContext, AliasedContext, AndContext, NumContext, EqContext, Error, } }
dbt_antlr_runtime::impl_parser_rule_context! { ExprContextAll { } { CastContext, ColContext, ParenContext, AliasedContext, AndContext, NumContext, EqContext, Error, } }
dbt_antlr_runtime::impl_tree_trait_delegates! { InlineAliasParserNodeKind::ExprContextAll { CastContext, ColContext, ParenContext, AliasedContext, AndContext, NumContext, EqContext, Error, } }
dbt_antlr_runtime::impl_node_inner! { InlineAliasParserNodeKind::ExprContext::ExprContextAll { CastContext, ColContext, ParenContext, AliasedContext, AndContext, NumContext, EqContext, Error, } }

impl<'input, 'arena, Tok> Deref for ExprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn ExprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			CastContext(inner) => inner,
			ColContext(inner) => inner,
			ParenContext(inner) => inner,
			AliasedContext(inner) => inner,
			AndContext(inner) => inner,
			NumContext(inner) => inner,
			EqContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type ExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExprContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @labeled InlineAliasParserNodeKind::ExprContext, ExprContextAll, ExprContextExt, RULE_expr }
impl<'input: 'arena, 'arena, Tok: Token + 'input> ExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena InlineAliasParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut InlineAliasParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ExprContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type CastContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CastContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait CastContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn type_(&self) -> Option<&'arena TypeContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CastContextAttrs<'input, 'arena, Tok> for CastContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn type_(&self) -> Option<&'arena TypeContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct CastContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::CastContext, CastContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for CastContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CastContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::CastContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type ColContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ColContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait ColContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ColContextAttrs<'input, 'arena, Tok> for ColContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_ID)
    }
}
#[derive(Debug)]
pub struct ColContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::ColContext, ColContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for ColContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ColContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::ColContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type ParenContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParenContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait ParenContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParenContextAttrs<'input, 'arena, Tok> for ParenContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct ParenContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::ParenContext, ParenContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for ParenContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParenContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::ParenContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type AliasedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AliasedContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait AliasedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn alias(&self) -> Option<&'arena AliasContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AliasedContextAttrs<'input, 'arena, Tok> for AliasedContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn alias(&self) -> Option<&'arena AliasContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct AliasedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::AliasedContext, AliasedContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for AliasedContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AliasedContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::AliasedContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type AndContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AndContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait AndContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AndContextAttrs<'input, 'arena, Tok> for AndContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AndContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::AndContext, AndContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for AndContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AndContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::AndContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type NumContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NumContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait NumContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NumContextAttrs<'input, 'arena, Tok> for NumContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_INT)
    }
}
#[derive(Debug)]
pub struct NumContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::NumContext, NumContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for NumContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NumContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::NumContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type EqContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EqContextExt<'input, 'arena, Tok>, InlineAliasParserNodeKind, Tok>;

pub trait EqContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EqContextAttrs<'input, 'arena, Tok> for EqContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct EqContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt InlineAliasParserNodeKind::ExprContext, ExprContextAll::EqContext, EqContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for EqContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EqContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut InlineAliasParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::EqContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
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
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, ExprContext<TF::Tok>, RULE_expr, 4, |_parentctx| ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 4;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(67);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    InlineAlias_T__12  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { CastContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena CastContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

		        recog.base.set_state(54);
		        recog.base.match_token(InlineAlias_T__12,&mut recog.err_handler)?;
		        recog.base.set_state(55);
		        recog.base.match_token(InlineAlias_T__13,&mut recog.err_handler)?;
		        recog.base.set_state(56);
		        recog.expr_rec(0)?;
		        recog.base.set_state(57);
		        recog.base.match_token(InlineAlias_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(58);
		        recog.type_()?;
		        recog.base.set_state(59);
		        recog.base.match_token(InlineAlias_T__14,&mut recog.err_handler)?;
		        }}
		    InlineAlias_T__13  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { ParenContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena ParenContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(61);
		        recog.base.match_token(InlineAlias_T__13,&mut recog.err_handler)?;
		        recog.base.set_state(62);
		        recog.expr_rec(0)?;
		        recog.base.set_state(63);
		        recog.base.match_token(InlineAlias_T__14,&mut recog.err_handler)?;
		        }}
		    InlineAlias_ID  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { ColContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena ColContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(65);
		        recog.base.match_token(InlineAlias_ID,&mut recog.err_handler)?;
		        }}
		    InlineAlias_INT  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { NumContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena NumContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(66);
		        recog.base.match_token(InlineAlias_INT,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(80);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(7,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(78);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.get_interpreter().adaptive_predict(6,&mut recog.base)? {
					1 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						AliasedContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena AliasedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(69);
						if !({recog.precpred(None, 7)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
						}
						recog.base.set_state(70);
						recog.base.match_token(InlineAlias_T__5,&mut recog.err_handler)?;
						recog.base.set_state(71);
						recog.alias()?;
						}
					}
				,
					2 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						EqContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena EqContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(72);
						if !({recog.precpred(None, 6)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
						}
						recog.base.set_state(73);
						recog.base.match_token(InlineAlias_T__11,&mut recog.err_handler)?;
						recog.base.set_state(74);
						recog.expr_rec(7)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						AndContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena AndContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(75);
						if !({recog.precpred(None, 5)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
						}
						recog.base.set_state(76);
						recog.base.match_token(InlineAlias_T__10,&mut recog.err_handler)?;
						recog.base.set_state(77);
						recog.expr_rec(6)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(82);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(7,&mut recog.base)?;
		}
		}
		})
	}
}
//------------------- alias ----------------
pub type AliasContextAll<'input, 'arena, Tok = CommonToken<'input>> = AliasContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { InlineAliasParserNodeKind, AliasContext, AliasContextExt, RULE_alias }

pub trait AliasContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AliasContextAttrs<'input, 'arena, Tok> for AliasContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_ID)
    }
}

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn alias(&mut self,) -> Result<&'arena AliasContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AliasContext<TF::Tok>, RULE_alias, 6, |_parentctx| AliasContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AliasContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(83);
		_la = recog.base.input.la(1);
		if { !(_la==InlineAlias_T__15 || _la==InlineAlias_ID) } {
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
//------------------- type ----------------
pub type TypeContextAll<'input, 'arena, Tok = CommonToken<'input>> = TypeContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { InlineAliasParserNodeKind, TypeContext, TypeContextExt, RULE_type }

pub trait TypeContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TypeContextAttrs<'input, 'arena, Tok> for TypeContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == InlineAlias_ID)
    }
}

impl<'input, 'arena, Input, TF> InlineAliasParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn type_(&mut self,) -> Result<&'arena TypeContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TypeContext<TF::Tok>, RULE_type, 8, |_parentctx| TypeContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TypeContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(85);
		_la = recog.base.input.la(1);
		if { !(_la==InlineAlias_T__15 || _la==InlineAlias_ID) } {
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

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> = LazyLock::new(|| {
    PackedATNDeserializer::new()
        .deserialize(&_serializedATN)
        .expect("invalid packed parser ATN")
});
static _serializedATN: [u32; 1139] = [
    1346458702, 3, 16909060, 29, 19, 88, 93, 1, 1, 8, 5, 29, 616, 645, 465, 
    1110, 5, 1115, 2, 1121, 8, 1129, 5, 1134, 5, 1139, 2, 1117, 4, 2, 0, 
    8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 12, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 
    4294967295, 1, 0, 8, 5, 1, 4294967295, 4294967295, 1, 0, 32, 6, 1, 4294967295, 
    4294967295, 1, 0, 8, 7, 1, 4294967295, 4294967295, 1, 0, 32, 8, 1, 4294967295, 
    4294967295, 1, 0, 32, 9, 1, 4294967295, 4294967295, 1, 0, 32, 10, 1, 
    4294967295, 4294967295, 1, 0, 32, 11, 1, 4294967295, 4294967295, 1, 
    0, 32, 12, 1, 4294967295, 4294967295, 1, 0, 8, 13, 1, 4294967295, 4294967295, 
    1, 0, 32, 14, 1, 4294967295, 4294967295, 1, 0, 8, 15, 1, 4294967295, 
    4294967295, 5, 0, 8, 16, 1, 22, 4294967295, 8, 0, 8, 17, 1, 4294967295, 
    4294967295, 10, 0, 8, 18, 2, 4294967295, 4294967295, 12, 0, 8, 20, 1, 
    4294967295, 25, 9, 0, 8, 21, 1, 4294967295, 4294967295, 1, 0, 32, 22, 
    1, 4294967295, 4294967295, 1, 0, 8, 23, 1, 4294967295, 4294967295, 1, 
    0, 32, 24, 1, 4294967295, 4294967295, 1, 0, 8, 25, 1, 4294967295, 4294967295, 
    3, 0, 8, 26, 2, 31, 4294967295, 8, 0, 8, 28, 1, 4294967295, 4294967295, 
    1, 1, 32, 29, 1, 4294967295, 4294967295, 1, 1, 8, 30, 1, 4294967295, 
    4294967295, 1, 1, 32, 31, 1, 4294967295, 4294967295, 1, 1, 8, 32, 1, 
    4294967295, 4294967295, 5, 1, 8, 33, 1, 37, 4294967295, 8, 1, 8, 34, 
    1, 4294967295, 4294967295, 10, 1, 8, 35, 2, 4294967295, 4294967295, 
    12, 1, 8, 37, 1, 4294967295, 40, 9, 1, 8, 38, 1, 4294967295, 4294967295, 
    1, 1, 32, 39, 1, 4294967295, 4294967295, 1, 1, 32, 40, 1, 4294967295, 
    4294967295, 3, 1, 8, 41, 2, 44, 4294967295, 8, 1, 8, 43, 1, 4294967295, 
    4294967295, 1, 1, 32, 44, 1, 4294967295, 4294967295, 1, 1, 32, 45, 1, 
    4294967295, 4294967295, 1, 1, 8, 46, 1, 4294967295, 4294967295, 1, 1, 
    32, 47, 1, 4294967295, 4294967295, 1, 1, 8, 48, 1, 4294967295, 4294967295, 
    1, 1, 8, 49, 1, 4294967295, 4294967295, 3, 1, 8, 50, 2, 52, 4294967295, 
    8, 1, 8, 52, 1, 4294967295, 4294967295, 1, 2, 72, 53, 1, 4294967295, 
    4294967295, 1, 2, 32, 54, 1, 4294967295, 4294967295, 1, 2, 32, 55, 1, 
    4294967295, 4294967295, 1, 2, 8, 56, 1, 4294967295, 4294967295, 1, 2, 
    32, 57, 1, 4294967295, 4294967295, 1, 2, 8, 58, 1, 4294967295, 4294967295, 
    1, 2, 32, 59, 1, 4294967295, 4294967295, 1, 2, 8, 60, 1, 4294967295, 
    4294967295, 1, 2, 32, 61, 1, 4294967295, 4294967295, 1, 2, 8, 62, 1, 
    4294967295, 4294967295, 1, 2, 32, 63, 1, 4294967295, 4294967295, 1, 
    2, 8, 64, 1, 4294967295, 4294967295, 1, 2, 32, 65, 1, 4294967295, 4294967295, 
    1, 2, 32, 66, 1, 4294967295, 4294967295, 3, 2, 8, 67, 4, 68, 4294967295, 
    8, 2, 8, 71, 1, 4294967295, 4294967295, 1, 2, 72, 72, 1, 4294967295, 
    4294967295, 1, 2, 32, 73, 1, 4294967295, 4294967295, 1, 2, 8, 74, 1, 
    4294967295, 4294967295, 1, 2, 72, 75, 1, 4294967295, 4294967295, 1, 
    2, 32, 76, 1, 4294967295, 4294967295, 1, 2, 8, 77, 1, 4294967295, 4294967295, 
    1, 2, 72, 78, 1, 4294967295, 4294967295, 1, 2, 32, 79, 1, 4294967295, 
    4294967295, 1, 2, 8, 80, 1, 4294967295, 4294967295, 5, 2, 8, 81, 3, 
    79, 4294967295, 8, 2, 8, 84, 1, 4294967295, 4294967295, 10, 2, 10, 85, 
    2, 4294967295, 4294967295, 12, 2, 8, 87, 1, 4294967295, 82, 9, 2, 8, 
    88, 1, 4294967295, 4294967295, 1, 3, 32, 89, 1, 4294967295, 4294967295, 
    1, 3, 8, 90, 1, 4294967295, 4294967295, 1, 4, 32, 91, 1, 4294967295, 
    4294967295, 1, 4, 8, 92, 1, 4294967295, 4294967295, 1, 4, 0, 93, 0, 
    4294967295, 4294967295, 1, 30, 0, 0, 0, 1, 32, 0, 0, 0, 1, 67, 0, 0, 
    0, 1, 83, 0, 0, 0, 1, 85, 0, 0, 0, 3, 2, 1, 11, 0, 5, 12, 4294967295, 
    0, 0, 1, 31, 0, 0, 0, 5, 14, 1, 0, 0, 5, 15, 2, 0, 0, 5, 16, 17, 0, 
    0, 5, 17, 3, 0, 0, 5, 18, 4, 0, 0, 3, 4, 2, 23, 0, 5, 20, 5, 0, 0, 3, 
    4, 2, 22, 0, 1, 19, 0, 0, 0, 1, 25, 0, 0, 0, 1, 21, 0, 0, 0, 1, 24, 
    0, 0, 0, 1, 26, 0, 0, 0, 1, 23, 0, 0, 0, 5, 27, 6, 0, 0, 3, 2, 1, 28, 
    0, 5, 29, 4294967295, 0, 0, 1, 31, 0, 0, 0, 1, 10, 0, 0, 0, 1, 13, 0, 
    0, 0, 1, 1, 0, 0, 0, 5, 33, 7, 0, 0, 3, 4, 2, 38, 0, 5, 35, 5, 0, 0, 
    3, 4, 2, 37, 0, 1, 34, 0, 0, 0, 1, 40, 0, 0, 0, 1, 36, 0, 0, 0, 1, 39, 
    0, 0, 0, 1, 43, 0, 0, 0, 1, 38, 0, 0, 0, 5, 42, 8, 0, 0, 5, 44, 17, 
    0, 0, 1, 41, 0, 0, 0, 1, 44, 0, 0, 0, 1, 51, 0, 0, 0, 5, 46, 9, 0, 0, 
    5, 47, 10, 0, 0, 3, 4, 2, 48, 0, 5, 49, 11, 0, 0, 3, 4, 2, 50, 0, 1, 
    52, 0, 0, 0, 1, 45, 0, 0, 0, 1, 52, 0, 0, 0, 1, 3, 0, 0, 0, 6, 54, 2, 
    4294967295, 0, 5, 55, 13, 0, 0, 5, 56, 14, 0, 0, 3, 4, 2, 57, 0, 5, 
    58, 6, 0, 0, 3, 8, 4, 59, 0, 5, 60, 15, 0, 0, 1, 68, 0, 0, 0, 5, 62, 
    14, 0, 0, 3, 4, 2, 63, 0, 5, 64, 15, 0, 0, 1, 68, 0, 0, 0, 5, 68, 17, 
    0, 0, 5, 68, 18, 0, 0, 1, 53, 0, 0, 0, 1, 61, 0, 0, 0, 1, 65, 0, 0, 
    0, 1, 66, 0, 0, 0, 1, 80, 0, 0, 0, 10, 70, 7, 0, 0, 5, 71, 6, 0, 0, 
    3, 6, 3, 79, 0, 10, 73, 6, 0, 0, 5, 74, 12, 0, 0, 3, 4, 2, 79, 7, 10, 
    76, 5, 0, 0, 5, 77, 11, 0, 0, 3, 4, 2, 79, 6, 1, 69, 0, 0, 0, 1, 72, 
    0, 0, 0, 1, 75, 0, 0, 0, 1, 82, 0, 0, 0, 1, 78, 0, 0, 0, 1, 81, 0, 0, 
    0, 1, 5, 0, 0, 0, 1, 80, 0, 0, 0, 7, 84, 0, 0, 0, 1, 7, 0, 0, 0, 7, 
    86, 0, 0, 0, 1, 9, 0, 0, 0, 0, 1, 1, 0, 2, 16, 17, 196608, 0, 0, 0, 
    23, 30, 38, 43, 51, 67, 78, 80, 0, 2, 4, 6, 8, 1, 3, 5, 7, 9
];