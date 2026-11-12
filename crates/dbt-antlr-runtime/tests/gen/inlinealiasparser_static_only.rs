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
		_alt = { let _sdp = recog.base.dfa_predict(7)?; _sdp };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(78);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(6)?; _sdp } {
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
			_alt = { let _sdp = recog.base.dfa_predict(7)?; _sdp };
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

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new_with_static_parser_tables(&_ATN, &_STATIC_PARSER_TABLES));
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
];static _STATIC_PARSER_TABLES: LazyLock<StaticParserTables> = LazyLock::new(|| {
    StaticParserTables::from_image(&_STATIC_PARSER_TABLE_WORDS)
        .expect("invalid static parser DFA table image; regenerate the parser")
});
static _STATIC_PARSER_TABLE_WORDS: [u32; 3289] = [
    1396983361, 4, 3289, 8, 5, 1, 16, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 0, 2147483648, 0, 35, 35, 3005, 
    3040, 108, 3148, 67, 3215, 28, 0, 3, 3, 7, 9, 5, 6, 7, 1, 2, 3, 4, 1, 
    1, 1, 1, 1, 1, 2, 3, 4, 35, 6, 4, 1, 6, 7, 100, 116, 0, 116, 124, 140, 
    0, 140, 0, 140, 0, 140, 0, 140, 0, 140, 0, 140, 0, 0, 1, 3, 2, 4294901760, 
    4294967295, 4294934529, 4294967295, 2147680255, 32771, 3005, 4294967295, 
    335, 213, 4294967295, 20, 100, 1440, 335, 2780, 3452, 11972, 3, 11984, 
    6, 12008, 3, 12020, 0, 12020, 0, 12020, 0, 12020, 0, 0, 2, 2, 0, 2, 
    2, 0, 1, 2, 2, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 3221225985, 0, 0, 3221291521, 
    0, 3221291521, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 3221291521, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 3221291521, 0, 0, 3221291521, 3221291521, 3221291521, 3221291521, 
    3221291521, 3221291521, 0, 0, 3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 
    0, 3221291521, 3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 0, 
    0, 3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 
    0, 0, 3221291521, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 0, 3221291521, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 3221291521, 3221291521, 3221291521, 3221291521, 3221291521, 
    3221291521, 3221291521, 3221291521, 3221291521, 3221291521, 3221291521, 
    3221291521, 3221291521, 3221291521, 3221291521, 2, 2, 2, 2, 2, 2, 2, 
    1, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 2, 1, 1, 1, 1, 1, 
    2, 2, 2, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 1, 2, 2, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 4294901760, 131071, 4294967295, 4294901762, 4294967295, 
    262147, 393221, 4294967295, 4294967295, 4294967295, 524287, 4294901768, 
    4294901769, 720906, 4294901772, 917517, 1048591, 1179665, 458751, 1310739, 
    1441813, 4294901783, 1572877, 1703961, 1769485, 1900572, 2031646, 2162720, 
    2293794, 2424831, 2228261, 2555942, 2687016, 2818090, 2949164, 3080238, 
    3211312, 852018, 3407923, 4294901813, 852022, 3670071, 3801145, 3866665, 
    3997756, 4128830, 4259904, 2687042, 1835075, 4522052, 4587533, 4718663, 
    4849737, 4915213, 917503, 4294901836, 4294967295, 4294967295, 917503, 
    4294901837, 5111821, 5242959, 2621521, 5439570, 5505037, 5636181, 5767167, 
    5832792, 5963866, 6029362, 4294901853, 6160397, 6291551, 4294901857, 
    3866623, 6422610, 4194403, 6553613, 6684773, 6750221, 4325480, 4294901842, 
    6881293, 7012458, 7143532, 7274606, 7405680, 852082, 7602291, 7733365, 
    7864439, 7929897, 8061050, 8192124, 8257628, 917503, 852095, 8454272, 
    8585346, 2687108, 6226053, 852102, 852103, 8978568, 9043981, 2687115, 
    9240716, 4128910, 9437327, 9568401, 9699475, 9764984, 6029462, 9961623, 
    4294901913, 10158234, 10289308, 10420382, 7929855, 10551456, 7864482, 
    10682450, 917503, 10813604, 10944678, 11010180, 11141289, 852139, 11337900, 
    11468974, 8650928, 4294901842, 11599885, 9109624, 6488146, 11665551, 
    11796659, 11927551, 11993270, 12124344, 12189788, 917503, 12320955, 
    4294901949, 852158, 12583103, 12648461, 12779714, 12910788, 13041862, 
    852168, 4294901961, 13238285, 8061131, 4294901964, 852173, 13566158, 
    13697232, 13828306, 13697236, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 65535, 4294934529, 4294967295, 4294967295, 
    229378, 2147811327, 4294934533, 2147942406, 4294967295, 4294934536, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294934537, 
    4294967295, 4294967295, 4294967295, 720895, 4294901770, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    786443, 4294967295, 851981, 4294934542, 4294967295, 4294967295, 2148565007, 
    2148663295, 4294934546, 2147975187, 4294967295, 4294934548, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 1441791, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148990998, 
    4294967295, 1572888, 4294934553, 4294967295, 4294967295, 1736719, 2148663295, 
    4294934546, 1835035, 4294967295, 4294934557, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2031646, 4294967295, 2097184, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2228223, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2293759, 4294967295, 4294967295, 
    2293795, 4294967295, 4294901773, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294934564, 4294967295, 4294967295, 4294967295, 
    2490367, 4294901797, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2555942, 4294967295, 2621480, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2752553, 4294967295, 2621480, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2150367231, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148991020, 4294967295, 2949165, 4294967295, 4294967295, 
    4294967295, 3080191, 4294967295, 4294967295, 3080239, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2148991024, 4294967295, 3211313, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 3342335, 4294901810, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 3407923, 4294967295, 
    3473461, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 3604479, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2148991031, 4294967295, 3670072, 4294934553, 4294967295, 4294967295, 
    3768335, 2148663295, 4294934546, 2151284763, 4294967295, 4294934557, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 3932159, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2148991036, 4294967295, 3997757, 4294967295, 4294967295, 4294967295, 
    4128767, 4294967295, 4294967295, 4128831, 4294967295, 4294901792, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4259839, 4294901824, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4325441, 
    4294967295, 4390979, 4294967295, 4294967295, 4294967295, 4521983, 4294967295, 
    4294967295, 4522053, 4294967295, 4294967295, 4294967295, 4294934542, 
    4294967295, 4294967295, 4620303, 2148663295, 4294934546, 2293831, 4294967295, 
    4294901773, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4784127, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148991049, 4294967295, 4849738, 4294934542, 4294967295, 
    4294967295, 3768335, 2148663295, 4294934546, 2151284807, 4294967295, 
    4294901773, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4980811, 4294967295, 5046349, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    5177343, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2152726527, 4294967295, 4294967295, 2151317562, 4294967295, 4294901800, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294934564, 
    4294967295, 4294967295, 4294967295, 5308415, 4294901840, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    5374033, 4294967295, 5439571, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 5570559, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 5636095, 4294901845, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 3407958, 
    4294967295, 5701719, 4294934542, 4294967295, 4294967295, 5799951, 2148663295, 
    4294934546, 3080281, 4294967295, 4294901773, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    5963775, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 2148991067, 4294967295, 
    6029404, 4294934542, 4294967295, 4294967295, 6127631, 2148663295, 4294934546, 
    2151284825, 4294967295, 4294934557, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    6225919, 4294901854, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 3407967, 4294967295, 6291552, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294934564, 4294967295, 4294967295, 
    4294967295, 6422527, 4294901857, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 6488162, 4294967295, 
    3473461, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 6619236, 4294967295, 6684774, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 6815743, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 6881279, 
    4294967295, 4294967295, 6881385, 4294967295, 4294934548, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148991082, 4294967295, 7012459, 4294967295, 4294967295, 
    4294967295, 2154627071, 4294967295, 4294967295, 2151317562, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991085, 4294967295, 7209070, 
    4294934639, 4294967295, 4294967295, 2154922096, 2155020287, 4294934643, 
    2151317620, 4294967295, 4294934548, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991093, 
    4294967295, 7733366, 4294967295, 4294967295, 4294967295, 2155347967, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148991096, 4294967295, 7929977, 4294934542, 4294967295, 
    4294967295, 8028175, 2148663295, 4294934546, 4128891, 4294967295, 4294901884, 
    4294967295, 4294934542, 4294967295, 4294967295, 3768335, 2148663295, 
    4294934546, 2151284859, 4294967295, 4294901884, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294934564, 4294967295, 4294967295, 
    4294967295, 8257535, 4294901885, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 8323198, 4294967295, 
    4390979, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 8454272, 4294967295, 8519810, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 8650751, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2152726527, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294901827, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934564, 4294967295, 
    4294967295, 4294967295, 2156199935, 4294934660, 4294934542, 4294967295, 
    4294967295, 8749071, 2148663295, 4294934546, 4522118, 4294967295, 4294901810, 
    4294967295, 4294934542, 4294967295, 4294967295, 8880143, 2148663295, 
    4294934546, 2151284870, 4294967295, 4294934557, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    8978431, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 2148991113, 4294967295, 
    9044106, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2148991115, 4294967295, 9175180, 4294967295, 
    4294967295, 4294967295, 2156789759, 4294967295, 4294967295, 2151317562, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 2148991118, 4294967295, 
    9371791, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 9502719, 4294901904, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    3408017, 4294967295, 9568402, 4294967295, 4294967295, 4294967295, 2152726527, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294901837, 4294967295, 
    4294967295, 4294967295, 4294967295, 2157182975, 4294967295, 4294967295, 
    2151317562, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2152726527, 4294967295, 4294967295, 2151317562, 4294967295, 
    4294901843, 4294967295, 4294967295, 4294967295, 4294967295, 2157248511, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934564, 4294967295, 
    4294967295, 4294967295, 9830399, 4294901909, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 6488214, 
    4294967295, 5701719, 4294934542, 4294967295, 4294967295, 5799951, 2148663295, 
    4294934546, 3080281, 4294967295, 4294934557, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    9961471, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 2148991128, 4294967295, 
    10027161, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2148991130, 4294967295, 10158235, 4294967295, 
    4294967295, 4294967295, 10289151, 4294967295, 4294967295, 10289309, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 10420382, 4294967295, 
    10485920, 4294967295, 4294967295, 4294967295, 4294967295, 4294934564, 
    4294967295, 4294967295, 4294967295, 10616831, 4294901921, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    6488226, 4294967295, 6291552, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934564, 4294967295, 4294967295, 4294967295, 2158231551, 4294934691, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 10813604, 4294967295, 10879142, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 11010047, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 11075583, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 6881279, 4294967295, 4294967295, 
    6881385, 4294967295, 4294901929, 4294967295, 4294967295, 4294967295, 
    4294967295, 11206655, 4294967295, 4294967295, 11206827, 4294967295, 
    4294967295, 4294967295, 4294934542, 4294967295, 4294967295, 11304975, 
    2148663295, 4294934546, 6881453, 4294967295, 4294934548, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 11468799, 4294967295, 4294967295, 4294934542, 4294967295, 
    4294967295, 3768335, 2148663295, 4294934546, 2151284909, 4294967295, 
    4294934548, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 11534511, 4294967295, 11600049, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 11730943, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 11796479, 4294967295, 4294967295, 11796660, 4294967295, 
    4294901890, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 11927551, 4294901941, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 11993270, 4294967295, 12058808, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991161, 
    4294967295, 12189882, 4294967295, 4294967295, 4294967295, 12320767, 
    4294967295, 4294967295, 10289309, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148991164, 4294967295, 12386493, 4294967295, 4294967295, 
    4294967295, 2160001023, 4294967295, 4294967295, 2151317562, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991167, 4294967295, 12583104, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 12714177, 4294967295, 12779715, 4294934529, 4294967295, 
    4294967295, 12877826, 2147811327, 4294934533, 2151284933, 4294967295, 
    4294934536, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 13041663, 4294901958, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 3408071, 4294967295, 13107400, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294934564, 4294967295, 4294967295, 4294967295, 13238271, 
    4294901961, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 6488266, 4294967295, 9568402, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 6619339, 
    4294967295, 13369548, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991181, 4294967295, 13500622, 
    4294967295, 4294967295, 4294967295, 13631487, 4294967295, 4294967295, 
    10289309, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 10420432, 
    4294967295, 13697233, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 13828095, 4294901970, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 3408083, 4294967295, 13893844, 4294934542, 4294967295, 4294967295, 
    13991951, 2148663295, 4294934546, 10289366, 4294967295, 4294934548, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 2148991191, 4294967295, 14155992, 4294934542, 
    4294967295, 4294967295, 14254095, 2148663295, 4294934546, 2151284950, 
    4294967295, 4294934557, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 14352383, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 14417919, 4294967295, 
    4294967295, 14418140, 4294967295, 4294901981, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 14614527, 4294901982, 4294967295, 4294967295, 4294967295, 
    6881279, 4294967295, 4294967295, 6881385, 4294967295, 4294901983, 4294967295, 
    4294967295, 4294967295, 4294967295, 14745599, 4294967295, 4294967295, 
    11206827, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991201, 
    4294967295, 14811362, 4294967295, 4294967295, 4294967295, 14942207, 
    4294967295, 4294967295, 14942436, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934537, 4294967295, 
    4294967295, 4294967295, 2162556927, 4294934757, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 15139046, 
    4294967295, 2148827156, 4294934542, 4294967295, 4294967295, 15237135, 
    2148663295, 4294934546, 11206889, 4294967295, 4294901904, 4294967295, 
    4294934542, 4294967295, 4294967295, 15368207, 2148663295, 4294934546, 
    2151284969, 4294967295, 4294934557, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 15466495, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2163015679, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 11796479, 4294967295, 4294967295, 
    11796660, 4294967295, 4294901937, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    15597567, 4294901997, 4294967295, 4294967295, 4294967295, 15663103, 
    4294967295, 4294967295, 10289309, 4294967295, 4294967295, 4294967295, 
    4294934542, 4294967295, 4294967295, 15695887, 2148663295, 4294934546, 
    11796720, 4294967295, 4294901981, 4294967295, 4294934542, 4294967295, 
    4294967295, 3768335, 2148663295, 4294934546, 2151284976, 4294967295, 
    4294901981, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934564, 4294967295, 4294967295, 4294967295, 15859711, 4294902001, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 15925490, 4294967295, 12058808, 4294967295, 4294967295, 
    4294967295, 2152726527, 4294967295, 4294967295, 2151317562, 4294967295, 
    4294901944, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934564, 4294967295, 4294967295, 4294967295, 2163539967, 4294934772, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 10420469, 4294967295, 16122102, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    12255231, 4294901946, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991223, 4294967295, 16253176, 
    4294934542, 4294967295, 4294967295, 13991951, 2148663295, 4294934546, 
    10289366, 4294967295, 4294934557, 4294967295, 4294934529, 4294967295, 
    4294967295, 16351234, 2147811327, 4294934533, 2151284933, 4294967295, 
    4294934536, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 16449535, 4294902010, 
    4294967295, 4294967295, 4294967295, 16515071, 4294967295, 4294967295, 
    16515324, 4294967295, 4294902013, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    16711679, 4294902014, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 16777471, 4294967295, 16843009, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 16973823, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991235, 
    4294967295, 17039620, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934564, 4294967295, 4294967295, 4294967295, 17170431, 4294902021, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 6488326, 4294967295, 13107400, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294934564, 4294967295, 4294967295, 4294967295, 2164785151, 
    4294934791, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 10813704, 4294967295, 17367305, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 17498111, 4294902026, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294934564, 4294967295, 4294967295, 4294967295, 17563647, 
    4294902027, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 6488332, 4294967295, 13893844, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991245, 
    4294967295, 17694990, 4294967295, 4294967295, 4294967295, 17825791, 
    4294967295, 4294967295, 17826064, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 17956863, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2152726527, 4294967295, 4294967295, 2151284956, 4294967295, 
    4294901981, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934537, 4294967295, 4294967295, 4294967295, 2165506047, 4294934802, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 14876671, 4294901986, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991251, 
    4294967295, 18088212, 4294934529, 4294967295, 4294967295, 18186242, 
    2147811327, 4294934533, 18284822, 4294967295, 4294934536, 4294967295, 
    4294967295, 4294967295, 4294967295, 14942207, 4294967295, 4294967295, 
    14942436, 4294967295, 4294901904, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991256, 
    4294967295, 18415897, 4294967295, 4294967295, 4294967295, 2166030335, 
    4294967295, 4294967295, 2151284964, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 18612507, 4294967295, 18678045, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 18808831, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2152726527, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294934548, 4294967295, 
    4294967295, 4294967295, 4294967295, 2166358015, 4294967295, 4294967295, 
    2151317562, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    18939903, 4294902048, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 19005729, 4294967295, 19071267, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 10420516, 4294967295, 19202341, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 19333119, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2152726527, 
    4294967295, 4294967295, 2151284988, 4294967295, 4294902013, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934537, 4294967295, 
    4294967295, 4294967295, 19398655, 4294902055, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 19464488, 
    4294967295, 19530026, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 19661099, 4294967295, 19530026, 
    4294967295, 4294967295, 4294967295, 6881279, 4294967295, 4294967295, 
    6881385, 4294967295, 4294902058, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2148991277, 
    4294967295, 19792174, 4294967295, 4294967295, 4294967295, 2167406591, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148991280, 4294967295, 19988785, 4294967295, 4294967295, 
    4294967295, 17825791, 4294967295, 4294967295, 17826064, 4294967295, 
    4294902066, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991283, 4294967295, 20185396, 
    4294967295, 4294967295, 4294967295, 2167799807, 4294967295, 4294967295, 
    2151285008, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 20381695, 4294967295, 4294967295, 10289309, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 12714295, 4294967295, 20447544, 
    4294934529, 4294967295, 4294967295, 18186242, 2147811327, 4294934533, 
    18284822, 4294967295, 4294934548, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 20578303, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991290, 4294967295, 20644155, 
    4294934529, 4294967295, 4294967295, 20742146, 2147811327, 4294934533, 
    2151285014, 4294967295, 4294934536, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 20840447, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2148991294, 4294967295, 20906303, 
    4294967295, 4294967295, 4294967295, 2152726527, 4294967295, 4294967295, 
    2151317562, 4294967295, 4294902045, 4294967295, 4294967295, 4294967295, 
    4294967295, 2168520703, 4294967295, 4294967295, 2151317562, 4294967295, 
    4294967295, 4294967295, 4294934529, 4294967295, 4294967295, 2168553474, 
    2147811327, 4294934533, 2168684866, 4294967295, 4294934548, 4294967295, 
    4294967295, 4294967295, 4294967295, 2168782847, 4294967295, 4294967295, 
    2168684867, 4294967295, 4294901944, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2168848383, 4294934853, 4294967295, 4294967295, 4294967295, 2168913919, 
    4294967295, 4294967295, 18284823, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2169012551, 4294967295, 2169078089, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2169176063, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2152726527, 
    4294967295, 4294967295, 2151317562, 4294967295, 4294902058, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934537, 4294967295, 
    4294967295, 4294967295, 2163539967, 4294934772, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 2169274699, 
    4294967295, 2169340237, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 2169438207, 4294967295, 4294967295, 
    47, 56, 47, 33, 35, 49, 71, 74, 77, 0, 2, 3, 108, 4294967295, 18, 3, 
    4294967295, 19, 100, 172, 18, 244, 280, 396, 1, 400, 6, 424, 2, 432, 
    0, 432, 0, 432, 0, 432, 0, 0, 2, 2, 0, 2, 2, 2, 1, 2, 2, 0, 1, 1, 1, 
    1, 1, 1, 3221225985, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 1, 1, 1, 1, 1, 1, 
    1, 1, 4294901760, 131071, 4294967295, 4294967295, 4294967295, 4294901762, 
    4294967295, 4294967295, 4294967295, 4294934529, 4294967295, 4294967295, 
    229378, 2147811327, 4294934533, 2147975174, 4294967295, 4294934536, 
    4294967295, 4294967295, 4294967295, 4294967295, 2148139007, 4294967295, 
    4294967295, 4294967295, 4294967295, 655370, 4294934539, 4294967295, 
    4294967295, 2148368396, 2148466687, 4294934543, 2147975184, 4294967295, 
    4294934545, 65535, 56, 33, 35, 49, 71, 74, 77, 0, 1, 67, 4294967295, 
    9, 2, 4294967295, 19, 100, 136, 9, 172, 192, 268, 0, 268, 0, 268, 0, 
    268, 0, 268, 0, 268, 0, 268, 0, 0, 2, 2, 0, 2, 2, 2, 2, 1, 2, 2, 2, 
    2, 2, 2, 2, 2, 1, 4294901760, 131071, 4294967295, 4294967295, 65535, 
    4294934529, 4294967295, 4294967295, 229378, 2147811327, 4294934533, 
    4294934534, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 2148007935, 4294967295, 4294967295, 4294967295, 4294967295, 
    2148040712, 28, 4294967295, 1, 0, 0, 0, 100, 104, 1, 108, 112, 112, 
    0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 2, 2, 65535
];