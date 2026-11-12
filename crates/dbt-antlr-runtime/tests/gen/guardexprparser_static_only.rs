// Generated from GuardExpr.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","8");
pub const GuardExpr_T__0:i32=1; 
pub const GuardExpr_T__1:i32=2; 
pub const GuardExpr_T__2:i32=3; 
pub const GuardExpr_T__3:i32=4; 
pub const GuardExpr_T__4:i32=5; 
pub const GuardExpr_T__5:i32=6; 
pub const GuardExpr_T__6:i32=7; 
pub const GuardExpr_T__7:i32=8; 
pub const GuardExpr_T__8:i32=9; 
pub const GuardExpr_T__9:i32=10; 
pub const GuardExpr_T__10:i32=11; 
pub const GuardExpr_ID:i32=12; 
pub const GuardExpr_INT:i32=13; 
pub const GuardExpr_WS:i32=14;
pub const GuardExpr_EOF:i32=EOF;
pub const RULE_prog:usize = 0; 
pub const RULE_stmt:usize = 1; 
pub const RULE_expr:usize = 2; 
pub const RULE_pred:usize = 3; 
pub const RULE_value:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "prog", "stmt", "expr", "pred", "value"
];

pub const _LITERAL_NAMES: [Option<&'static str>;12] = [
	None, Some("'set'"), Some("'='"), Some("';'"), Some("'NOT'"), Some("'AND'"), 
	Some("'OR'"), Some("'<'"), Some("'>'"), Some("'IN'"), Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, GuardExprParserExt<'input, 'arena>, GuardExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				GuardExprParserExt {
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
        L: ParseTreeListener<'arena, GuardExprParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct GuardExprTreeWalker;
impl GuardExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, GuardExprParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, GuardExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <GuardExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum GuardExprParserNodeKind {
    ProgContext,
    StmtContext,
    ExprContext,
    PredContext,
    ValueContext,
    Terminal,
    Error,
}
pub type GuardExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, GuardExprParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => GuardExprParser }
dbt_antlr_runtime::impl_node_kind! { GuardExprParserNodeKind {
    StmtContext(StmtContextAll), ExprContext(ExprContextAll), PredContext(PredContextAll), ; ProgContext(), ValueContext(), 
    }; 
}

pub struct GuardExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> GuardExprParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { GuardExprParserExt, GuardExprParserNodeKind, "GuardExpr.g4"; sempred (GuardExprParserNode<'input, 'arena, TF::Tok>, GuardExprParser) { 2 => expr_sempred, } }

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn expr_sempred(_ctx: Option<&'arena ExprContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 2)
		    }
	        1 => {
			recog.precpred(None, 1)
		    }
		    _ => true
		}
	}
}
//------------------- prog ----------------
pub type ProgContextAll<'input, 'arena, Tok = CommonToken<'input>> = ProgContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { GuardExprParserNodeKind, ProgContext, ProgContextExt, RULE_prog }

pub trait ProgContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn stmt_all(&self) -> Vec<&'arena StmtContextAll<'input, 'arena, Tok>>;
    fn stmt(&self, i: usize) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ProgContextAttrs<'input, 'arena, Tok> for ProgContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == GuardExpr_EOF)
    }
    fn stmt_all(&self) -> Vec<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn stmt(&self, i: usize) -> Option<&'arena StmtContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn prog(&mut self,) -> Result<&'arena ProgContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ProgContext<TF::Tok>, RULE_prog, 0, |_parentctx| ProgContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ProgContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(11); 
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		loop {
			{
			{
			recog.base.set_state(10);
			recog.stmt()?;
			}
			}
			recog.base.set_state(13); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 13330) != 0)) {break}
		}
		recog.base.set_state(15);
		recog.base.match_token(GuardExpr_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- stmt ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum StmtContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	AssignContext(AssignContext<'input, 'arena, Tok>),
	EvalContext(EvalContext<'input, 'arena, Tok>),
    Error(StmtContext<'input, 'arena, Tok>)
}

dbt_antlr_runtime::impl_rule_context! { StmtContextAll { } { AssignContext, EvalContext, Error, } }
dbt_antlr_runtime::impl_parser_rule_context! { StmtContextAll { } { AssignContext, EvalContext, Error, } }
dbt_antlr_runtime::impl_tree_trait_delegates! { GuardExprParserNodeKind::StmtContextAll { AssignContext, EvalContext, Error, } }
dbt_antlr_runtime::impl_node_inner! { GuardExprParserNodeKind::StmtContext::StmtContextAll { AssignContext, EvalContext, Error, } }

impl<'input, 'arena, Tok> Deref for StmtContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn StmtContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use StmtContextAll::*;
		match self{
			AssignContext(inner) => inner,
			EvalContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type StmtContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, StmtContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct StmtContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @labeled GuardExprParserNodeKind::StmtContext, StmtContextAll, StmtContextExt, RULE_stmt }
impl<'input: 'arena, 'arena, Tok: Token + 'input> StmtContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena GuardExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut GuardExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, StmtContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait StmtContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> StmtContextAttrs<'input, 'arena, Tok> for StmtContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type AssignContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AssignContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait AssignContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AssignContextAttrs<'input, 'arena, Tok> for AssignContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AssignContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StmtContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::StmtContext, StmtContextAll::AssignContext, AssignContextExt, RULE_stmt }

impl<'input, 'arena, Tok> StmtContextAttrs<'input, 'arena, Tok> for AssignContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AssignContextExt<'input, 'arena, Tok> {
	fn new(base: StmtContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StmtContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StmtContextAll::AssignContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut StmtContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type EvalContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EvalContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait EvalContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EvalContextAttrs<'input, 'arena, Tok> for EvalContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct EvalContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: StmtContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::StmtContext, StmtContextAll::EvalContext, EvalContextExt, RULE_stmt }

impl<'input, 'arena, Tok> StmtContextAttrs<'input, 'arena, Tok> for EvalContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EvalContextExt<'input, 'arena, Tok> {
	fn new(base: StmtContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut StmtContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            StmtContextAll::EvalContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut StmtContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stmt(&mut self,) -> Result<&'arena StmtContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StmtContext<TF::Tok>, RULE_stmt, 2, |_parentctx| StmtContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StmtContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(26);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_T__0  => {
		        /*------- Outer Most Alt 1 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            AssignContextExt::copy_from(ctx);
		            ctx.set_alt_number(1);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena AssignContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(17);
		        recog.base.match_token(GuardExpr_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(18);
		        recog.expr_rec(0)?;
		        recog.base.set_state(19);
		        recog.base.match_token(GuardExpr_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(20);
		        recog.expr_rec(0)?;
		        recog.base.set_state(21);
		        recog.base.match_token(GuardExpr_T__2,&mut recog.err_handler)?;
		        }}
		    GuardExpr_T__3 |GuardExpr_T__9 |GuardExpr_ID |GuardExpr_INT  => {
		        /*------- Outer Most Alt 2 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            EvalContextExt::copy_from(ctx);
		            ctx.set_alt_number(2);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena EvalContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(23);
		        recog.expr_rec(0)?;
		        recog.base.set_state(24);
		        recog.base.match_token(GuardExpr_T__2,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
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
	NotContext(NotContext<'input, 'arena, Tok>),
	PredicatedContext(PredicatedContext<'input, 'arena, Tok>),
	OrContext(OrContext<'input, 'arena, Tok>),
	AndContext(AndContext<'input, 'arena, Tok>),
    Error(ExprContext<'input, 'arena, Tok>)
}

dbt_antlr_runtime::impl_rule_context! { ExprContextAll { } { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr_runtime::impl_parser_rule_context! { ExprContextAll { } { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr_runtime::impl_tree_trait_delegates! { GuardExprParserNodeKind::ExprContextAll { NotContext, PredicatedContext, OrContext, AndContext, Error, } }
dbt_antlr_runtime::impl_node_inner! { GuardExprParserNodeKind::ExprContext::ExprContextAll { NotContext, PredicatedContext, OrContext, AndContext, Error, } }

impl<'input, 'arena, Tok> Deref for ExprContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn ExprContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			NotContext(inner) => inner,
			PredicatedContext(inner) => inner,
			OrContext(inner) => inner,
			AndContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type ExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExprContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @labeled GuardExprParserNodeKind::ExprContext, ExprContextAll, ExprContextExt, RULE_expr }
impl<'input: 'arena, 'arena, Tok: Token + 'input> ExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena GuardExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut GuardExprParserNode<'input, 'arena, Tok>, ANTLRError>
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

pub type NotContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NotContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait NotContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotContextAttrs<'input, 'arena, Tok> for NotContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct NotContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::NotContext, NotContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for NotContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::NotContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type PredicatedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredicatedContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait PredicatedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>>;
	fn pred(&self) -> Option<&'arena PredContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicatedContextAttrs<'input, 'arena, Tok> for PredicatedContext<'input, 'arena, Tok>
{
    fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn pred(&self) -> Option<&'arena PredContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct PredicatedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::PredicatedContext, PredicatedContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for PredicatedContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicatedContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::PredicatedContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type OrContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, OrContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait OrContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
	fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OrContextAttrs<'input, 'arena, Tok> for OrContext<'input, 'arena, Tok>
{
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct OrContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: ExprContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::OrContext, OrContextExt, RULE_expr }

impl<'input, 'arena, Tok> ExprContextAttrs<'input, 'arena, Tok> for OrContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OrContextExt<'input, 'arena, Tok> {
	fn new(base: ExprContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut ExprContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            ExprContextAll::OrContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut ExprContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type AndContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AndContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

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

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::ExprContext, ExprContextAll::AndContext, AndContextExt, RULE_expr }

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

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
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

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
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
		recog.base.set_state(35);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_T__9 |GuardExpr_ID |GuardExpr_INT  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { PredicatedContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena PredicatedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

		        recog.base.set_state(29);
		        recog.value()?;
		        recog.base.set_state(31);
		        recog.err_handler.sync(&mut recog.base)?;
		        match { let _sdp = recog.base.dfa_predict(2)?; _sdp } {
		        	x if x == 1 =>{
		        		{
		        		recog.base.set_state(30);
		        		recog.pred()?;
		        		}
		        	}

		        	_ => {}
		        }
		        }}
		    GuardExpr_T__3  => {
		        {
		        recog.base.with_mut_ctx(|ctx| { NotContextExt::copy_from(ctx); });
		        let _local_ctx_fn = |recog: &Self| -> &'arena NotContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        recog.base.set_state(33);
		        recog.base.match_token(GuardExpr_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(34);
		        recog.expr_rec(3)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(45);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(5)?; _sdp };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(43);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(4)?; _sdp } {
					1 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						AndContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena AndContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(37);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(38);
						recog.base.match_token(GuardExpr_T__4,&mut recog.err_handler)?;
						recog.base.set_state(39);
						recog.expr_rec(3)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						OrContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_expr)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena OrContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(40);
						if !({recog.precpred(None, 1)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
						}
						recog.base.set_state(41);
						recog.base.match_token(GuardExpr_T__5,&mut recog.err_handler)?;
						recog.base.set_state(42);
						recog.expr_rec(2)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(47);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(5)?; _sdp };
		}
		}
		})
	}
}
//------------------- pred ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum PredContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	CmpContext(CmpContext<'input, 'arena, Tok>),
	InListContext(InListContext<'input, 'arena, Tok>),
    Error(PredContext<'input, 'arena, Tok>)
}

dbt_antlr_runtime::impl_rule_context! { PredContextAll { } { CmpContext, InListContext, Error, } }
dbt_antlr_runtime::impl_parser_rule_context! { PredContextAll { } { CmpContext, InListContext, Error, } }
dbt_antlr_runtime::impl_tree_trait_delegates! { GuardExprParserNodeKind::PredContextAll { CmpContext, InListContext, Error, } }
dbt_antlr_runtime::impl_node_inner! { GuardExprParserNodeKind::PredContext::PredContextAll { CmpContext, InListContext, Error, } }

impl<'input, 'arena, Tok> Deref for PredContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn PredContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use PredContextAll::*;
		match self{
			CmpContext(inner) => inner,
			InListContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type PredContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct PredContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @labeled GuardExprParserNodeKind::PredContext, PredContextAll, PredContextExt, RULE_pred }
impl<'input: 'arena, 'arena, Tok: Token + 'input> PredContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena GuardExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut GuardExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, PredContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait PredContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> PredContextAttrs<'input, 'arena, Tok> for PredContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type CmpContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CmpContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait CmpContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CmpContextAttrs<'input, 'arena, Tok> for CmpContext<'input, 'arena, Tok>
{
    fn value(&self) -> Option<&'arena ValueContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct CmpContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: PredContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::PredContext, PredContextAll::CmpContext, CmpContextExt, RULE_pred }

impl<'input, 'arena, Tok> PredContextAttrs<'input, 'arena, Tok> for CmpContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CmpContextExt<'input, 'arena, Tok> {
	fn new(base: PredContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut PredContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            PredContextAll::CmpContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut PredContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type InListContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, InListContextExt<'input, 'arena, Tok>, GuardExprParserNodeKind, Tok>;

pub trait InListContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> InListContextAttrs<'input, 'arena, Tok> for InListContext<'input, 'arena, Tok>
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct InListContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: PredContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt GuardExprParserNodeKind::PredContext, PredContextAll::InListContext, InListContextExt, RULE_pred }

impl<'input, 'arena, Tok> PredContextAttrs<'input, 'arena, Tok> for InListContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> InListContextExt<'input, 'arena, Tok> {
	fn new(base: PredContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut GuardExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut PredContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            PredContextAll::InListContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut PredContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn pred(&mut self,) -> Result<&'arena PredContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, PredContext<TF::Tok>, RULE_pred, 6, |_parentctx| PredContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena PredContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(55);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_T__1 |GuardExpr_T__6 |GuardExpr_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            CmpContextExt::copy_from(ctx);
		            ctx.set_alt_number(1);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena CmpContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(48);
		        _la = recog.base.input.la(1);
		        if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 388) != 0)) } {
		        	recog.err_handler.recover_inline(&mut recog.base)?;
		        }
		        else {
		        	if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        	recog.err_handler.report_match(&mut recog.base);
		        	recog.base.consume(&mut recog.err_handler)?;
		        }
		        recog.base.set_state(49);
		        recog.value()?;
		        }}
		    GuardExpr_T__8  => {
		        /*------- Outer Most Alt 2 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            InListContextExt::copy_from(ctx);
		            ctx.set_alt_number(2);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena InListContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(50);
		        recog.base.match_token(GuardExpr_T__8,&mut recog.err_handler)?;
		        recog.base.set_state(51);
		        recog.base.match_token(GuardExpr_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(52);
		        recog.expr_rec(0)?;
		        recog.base.set_state(53);
		        recog.base.match_token(GuardExpr_T__10,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- value ----------------
pub type ValueContextAll<'input, 'arena, Tok = CommonToken<'input>> = ValueContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { GuardExprParserNodeKind, ValueContext, ValueContextExt, RULE_value }

pub trait ValueContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ValueContextAttrs<'input, 'arena, Tok> for ValueContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == GuardExpr_ID)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == GuardExpr_INT)
    }
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> GuardExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn value(&mut self,) -> Result<&'arena ValueContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ValueContext<TF::Tok>, RULE_value, 8, |_parentctx| ValueContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ValueContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(63);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    GuardExpr_ID  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(57);
		        recog.base.match_token(GuardExpr_ID,&mut recog.err_handler)?;
		        }}
		    GuardExpr_INT  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(58);
		        recog.base.match_token(GuardExpr_INT,&mut recog.err_handler)?;
		        }}
		    GuardExpr_T__9  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(59);
		        recog.base.match_token(GuardExpr_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(60);
		        recog.expr_rec(0)?;
		        recog.base.set_state(61);
		        recog.base.match_token(GuardExpr_T__10,&mut recog.err_handler)?;
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
static _serializedATN: [u32; 867] = [
    1346458702, 3, 16909060, 29, 14, 66, 69, 1, 2, 8, 5, 29, 462, 491, 345, 
    836, 5, 841, 4, 849, 8, 857, 5, 862, 5, 867, 2, 845, 4, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 12, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 
    4294967295, 1, 0, 8, 5, 1, 4294967295, 4294967295, 4, 0, 8, 6, 1, 12, 
    4294967295, 8, 0, 8, 7, 1, 4294967295, 4294967295, 11, 0, 8, 8, 2, 4294967295, 
    4294967295, 12, 0, 8, 10, 1, 4294967295, 13, 1, 0, 32, 11, 1, 4294967295, 
    4294967295, 1, 0, 8, 12, 1, 4294967295, 4294967295, 1, 1, 32, 13, 1, 
    4294967295, 4294967295, 1, 1, 8, 14, 1, 4294967295, 4294967295, 1, 1, 
    32, 15, 1, 4294967295, 4294967295, 1, 1, 8, 16, 1, 4294967295, 4294967295, 
    1, 1, 32, 17, 1, 4294967295, 4294967295, 1, 1, 8, 18, 1, 4294967295, 
    4294967295, 1, 1, 8, 19, 1, 4294967295, 4294967295, 1, 1, 32, 20, 1, 
    4294967295, 4294967295, 1, 1, 8, 21, 1, 4294967295, 4294967295, 3, 1, 
    8, 22, 2, 27, 4294967295, 8, 1, 8, 24, 1, 4294967295, 4294967295, 1, 
    2, 72, 25, 1, 4294967295, 4294967295, 1, 2, 8, 26, 1, 4294967295, 4294967295, 
    1, 2, 8, 27, 1, 4294967295, 4294967295, 3, 2, 8, 28, 2, 32, 4294967295, 
    8, 2, 8, 30, 1, 4294967295, 4294967295, 1, 2, 32, 31, 1, 4294967295, 
    4294967295, 1, 2, 8, 32, 1, 4294967295, 4294967295, 3, 2, 8, 33, 2, 
    36, 4294967295, 8, 2, 8, 35, 1, 4294967295, 4294967295, 1, 2, 72, 36, 
    1, 4294967295, 4294967295, 1, 2, 32, 37, 1, 4294967295, 4294967295, 
    1, 2, 8, 38, 1, 4294967295, 4294967295, 1, 2, 72, 39, 1, 4294967295, 
    4294967295, 1, 2, 32, 40, 1, 4294967295, 4294967295, 1, 2, 8, 41, 1, 
    4294967295, 4294967295, 5, 2, 8, 42, 2, 44, 4294967295, 8, 2, 8, 44, 
    1, 4294967295, 4294967295, 10, 2, 10, 45, 2, 4294967295, 4294967295, 
    12, 2, 8, 47, 1, 4294967295, 47, 9, 2, 8, 48, 1, 4294967295, 4294967295, 
    1, 3, 32, 49, 1, 4294967295, 4294967295, 1, 3, 8, 50, 1, 4294967295, 
    4294967295, 1, 3, 32, 51, 1, 4294967295, 4294967295, 1, 3, 32, 52, 1, 
    4294967295, 4294967295, 1, 3, 8, 53, 1, 4294967295, 4294967295, 1, 3, 
    32, 54, 1, 4294967295, 4294967295, 1, 3, 8, 55, 1, 4294967295, 4294967295, 
    3, 3, 8, 56, 2, 56, 4294967295, 8, 3, 8, 58, 1, 4294967295, 4294967295, 
    1, 4, 32, 59, 1, 4294967295, 4294967295, 1, 4, 32, 60, 1, 4294967295, 
    4294967295, 1, 4, 32, 61, 1, 4294967295, 4294967295, 1, 4, 8, 62, 1, 
    4294967295, 4294967295, 1, 4, 32, 63, 1, 4294967295, 4294967295, 1, 
    4, 8, 64, 1, 4294967295, 4294967295, 3, 4, 8, 65, 3, 64, 4294967295, 
    8, 4, 8, 68, 1, 4294967295, 4294967295, 1, 4, 0, 69, 0, 4294967295, 
    4294967295, 1, 11, 0, 0, 0, 1, 26, 0, 0, 0, 1, 35, 0, 0, 0, 1, 55, 0, 
    0, 0, 1, 63, 0, 0, 0, 3, 2, 1, 12, 0, 1, 10, 0, 0, 0, 1, 13, 0, 0, 0, 
    1, 11, 0, 0, 0, 1, 14, 0, 0, 0, 1, 15, 0, 0, 0, 5, 16, 4294967295, 0, 
    0, 1, 1, 0, 0, 0, 5, 18, 1, 0, 0, 3, 4, 2, 19, 0, 5, 20, 2, 0, 0, 3, 
    4, 2, 21, 0, 5, 22, 3, 0, 0, 1, 27, 0, 0, 0, 3, 4, 2, 24, 0, 5, 25, 
    3, 0, 0, 1, 27, 0, 0, 0, 1, 17, 0, 0, 0, 1, 23, 0, 0, 0, 1, 3, 0, 0, 
    0, 6, 29, 2, 4294967295, 0, 3, 8, 4, 31, 0, 3, 6, 3, 32, 0, 1, 30, 0, 
    0, 0, 1, 32, 0, 0, 0, 1, 36, 0, 0, 0, 5, 34, 4, 0, 0, 3, 4, 2, 36, 3, 
    1, 28, 0, 0, 0, 1, 33, 0, 0, 0, 1, 45, 0, 0, 0, 10, 38, 2, 0, 0, 5, 
    39, 5, 0, 0, 3, 4, 2, 44, 3, 10, 41, 1, 0, 0, 5, 42, 6, 0, 0, 3, 4, 
    2, 44, 2, 1, 37, 0, 0, 0, 1, 40, 0, 0, 0, 1, 47, 0, 0, 0, 1, 43, 0, 
    0, 0, 1, 46, 0, 0, 0, 1, 5, 0, 0, 0, 1, 45, 0, 0, 0, 7, 49, 0, 0, 0, 
    3, 8, 4, 56, 0, 5, 51, 9, 0, 0, 5, 52, 10, 0, 0, 3, 4, 2, 53, 0, 5, 
    54, 11, 0, 0, 1, 56, 0, 0, 0, 1, 48, 0, 0, 0, 1, 50, 0, 0, 0, 1, 7, 
    0, 0, 0, 5, 64, 12, 0, 0, 5, 64, 13, 0, 0, 5, 60, 10, 0, 0, 3, 4, 2, 
    61, 0, 5, 62, 11, 0, 0, 1, 64, 0, 0, 0, 1, 57, 0, 0, 0, 1, 58, 0, 0, 
    0, 1, 59, 0, 0, 0, 1, 9, 0, 0, 0, 0, 2, 1, 0, 2, 2, 2, 7, 8, 388, 0, 
    0, 0, 13, 26, 31, 35, 43, 45, 55, 63, 0, 2, 4, 6, 8, 1, 3, 5, 7, 9
];static _STATIC_PARSER_TABLES: LazyLock<StaticParserTables> = LazyLock::new(|| {
    StaticParserTables::from_image(&_STATIC_PARSER_TABLE_WORDS)
        .expect("invalid static parser DFA table image; regenerate the parser")
});
static _STATIC_PARSER_TABLE_WORDS: [u32; 2978] = [
    1396983361, 4, 2978, 6, 5, 1, 9, 4294967295, 4294967295, 0, 4294967295, 
    1, 2147483648, 0, 2785, 2785, 31, 2816, 47, 2863, 50, 2913, 28, 0, 2, 
    2, 5, 4, 1, 2, 2, 3, 4, 2, 2, 3, 4, 2785, 2, 371, 243, 4294967295, 15, 
    100, 1584, 371, 3068, 3812, 11104, 1, 11108, 6, 11132, 2, 11140, 0, 
    11140, 0, 11140, 0, 11140, 0, 0, 2, 0, 2, 2, 1, 1, 2, 2, 0, 0, 0, 0, 
    0, 1, 0, 3221225985, 0, 2, 2, 1, 3221225985, 0, 0, 0, 0, 3221225985, 
    3221225985, 0, 1, 0, 0, 0, 3221225985, 0, 0, 0, 0, 0, 0, 3221225985, 
    0, 0, 0, 0, 0, 3221225985, 3221225985, 3221225985, 2, 2, 0, 0, 0, 1, 
    3221225985, 1, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 3221225985, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 3221225985, 0, 2, 0, 0, 3221225985, 
    0, 0, 0, 0, 0, 0, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 
    0, 0, 0, 0, 0, 1, 0, 3221225985, 1, 0, 3221225985, 3221225985, 0, 3221225985, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221225985, 
    0, 3221225985, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221225985, 
    0, 0, 0, 0, 0, 0, 0, 3221225985, 3221225985, 3221225985, 0, 3221225985, 
    3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221225985, 0, 0, 3221225985, 
    3221225985, 0, 0, 0, 0, 3221225985, 0, 3221225985, 3221225985, 3221225985, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3221225985, 
    0, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 3221225985, 0, 3221225985, 3221225985, 0, 0, 3221225985, 0, 
    0, 0, 0, 0, 0, 0, 0, 3221225985, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    3221225985, 0, 0, 0, 0, 0, 0, 0, 3221225985, 3221225985, 3221225985, 
    3221225985, 3221225985, 0, 3221225985, 3221225985, 3221225985, 0, 0, 
    3221225985, 0, 0, 0, 0, 0, 3221225985, 0, 0, 0, 3221225985, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 3221225985, 0, 0, 3221225985, 3221225985, 3221225985, 
    3221225985, 0, 0, 0, 0, 0, 0, 3221225985, 0, 0, 0, 3221225985, 0, 0, 
    0, 0, 3221225985, 0, 0, 3221225985, 3221225985, 2, 2, 2, 2, 2, 1, 1, 
    2, 2, 2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
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
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4294901760, 
    4294901761, 4294967295, 4294967295, 196607, 262147, 393221, 524287, 
    589823, 4294967295, 4294967295, 655369, 786443, 4294967295, 4294901773, 
    983054, 4294901776, 1179665, 1310739, 1441813, 1572863, 1638424, 1769498, 
    4294967295, 4294967295, 1900543, 1966109, 4294967295, 4294967295, 2097183, 
    2228257, 1376275, 2359331, 2490405, 2621479, 2752553, 4294901764, 2883583, 
    589868, 3014701, 3145775, 3276849, 3407871, 3473460, 3604534, 3735608, 
    3866682, 3997756, 4294901769, 4128830, 4259904, 4294901826, 4294901827, 
    4522052, 4653055, 4718663, 4849737, 4294901835, 5046348, 5177422, 5308496, 
    5439570, 5570644, 4294901846, 5767167, 5832792, 5963866, 6094847, 4294967295, 
    4294901853, 6225919, 6291455, 6357088, 6488162, 6619236, 6750310, 6881384, 
    589867, 6946859, 7077995, 262253, 4294901870, 4294901871, 5177343, 7340112, 
    7471217, 7602291, 7733365, 7864439, 4294901881, 8061050, 8126512, 8257661, 
    4294901851, 4294967295, 4294901887, 8454143, 8454187, 589954, 8650883, 
    8781957, 8912895, 4294901896, 9043967, 9109642, 4294901900, 4294901901, 
    4294967295, 3145870, 9437327, 9568401, 9699475, 9830549, 9961471, 10027160, 
    10158234, 10289308, 10420382, 10551456, 8650914, 4294901803, 2818211, 
    10747913, 10879141, 10944555, 11075752, 11206655, 11272191, 11337900, 
    11468974, 11600048, 11731122, 7995572, 11862064, 7995574, 12058807, 
    12189881, 12255352, 12386303, 4294967295, 12386488, 12517375, 12583103, 
    12648613, 12779714, 4294901803, 12910788, 13041862, 13107322, 3145929, 
    13304010, 13435084, 13565951, 13631695, 13762769, 13893843, 4294967295, 
    4294967295, 14024703, 4294967295, 14090239, 4294901975, 7012568, 14287065, 
    4294901979, 13304028, 4294901882, 7995613, 14549040, 14680287, 14745722, 
    14876898, 15007743, 4294901989, 4294967295, 15138815, 15139039, 15270120, 
    4294901882, 15401194, 4294901996, 15597805, 15728879, 15859711, 4294902002, 
    65535, 4294934529, 196607, 4294934531, 2147778564, 2147844101, 4294934534, 
    4294934535, 4294967295, 4294967295, 4294967295, 4294934536, 4294967295, 
    4294967295, 4294901769, 655370, 4294967295, 4294967295, 786431, 4294967295, 
    4294967295, 851967, 917503, 2148401165, 4294967295, 2148532239, 1179647, 
    2148663313, 2148761618, 2148859903, 4294967295, 4294967295, 4294967295, 
    2148925439, 4294967295, 4294967295, 851967, 917503, 4294901773, 4294967295, 
    4294967295, 4294901782, 4294967295, 4294967295, 4294901772, 1507351, 
    4294934529, 1638399, 4294934531, 1638425, 2149220378, 4294934555, 4294901788, 
    4294967295, 4294967295, 4294967295, 4294934557, 4294967295, 4294967295, 
    4294901790, 2031647, 4294967295, 4294967295, 2162687, 4294967295, 4294967295, 
    2149711871, 2293759, 4294901794, 4294967295, 4294967295, 4294934549, 
    4294967295, 4294967295, 4294901772, 2293795, 4294934529, 2424831, 4294934531, 
    2424869, 2490406, 4294901799, 4294934568, 4294967295, 4294967295, 4294967295, 
    4294934536, 4294967295, 4294967295, 4294901801, 2752554, 4294967295, 
    4294967295, 2883583, 4294967295, 4294967295, 2949119, 3014655, 2150498349, 
    4294967295, 2148565039, 2150694911, 2150727728, 2150793265, 2150170623, 
    4294967295, 4294967295, 4294967295, 3407871, 4294967295, 4294967295, 
    3473407, 3538943, 4294901813, 4294967295, 2148565046, 2151153663, 2151055415, 
    2151186486, 4294967295, 4294967295, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 2151284735, 3866623, 2148401210, 4294967295, 
    2148532283, 2150694911, 3932177, 3997756, 2148859903, 4294967295, 4294934529, 
    4128767, 4294934531, 2424869, 4128831, 4294901824, 4294934568, 4294967295, 
    4294967295, 4294967295, 4294934536, 4294967295, 4294967295, 4294901825, 
    4325442, 4294967295, 4294967295, 4456447, 4294967295, 4294967295, 2949119, 
    4521983, 4294901828, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901829, 4587590, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4718591, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901832, 4294967295, 4294967295, 4294901772, 851981, 
    4294934601, 2152398847, 4294934544, 2150662192, 2148696082, 4294934547, 
    4294934568, 4294967295, 4294967295, 4294967295, 4294934549, 4294967295, 
    4294967295, 4294901835, 4980812, 4294967295, 4294967295, 5111807, 4294967295, 
    4294967295, 851967, 1572863, 2147549207, 4294967295, 2147680334, 2150694911, 
    5177369, 5242959, 1900543, 4294967295, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 3473407, 3538943, 4294901813, 4294967295, 4294967295, 
    4294901841, 4294967295, 4294967295, 4294901812, 5374034, 4294934542, 
    5505023, 4294934612, 5570645, 2149220378, 4294934555, 4294901846, 2148466687, 
    4294967295, 2148532311, 2150694911, 5767185, 5832792, 2148859903, 4294967295, 
    4294967295, 4294967295, 2149449727, 4294967295, 4294967295, 5963775, 
    6029311, 4294901851, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901852, 6094941, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 6225919, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901855, 4294967295, 
    4294967295, 4294967295, 6356991, 4294967295, 4294967295, 851967, 1572863, 
    2147549207, 4294967295, 2147713121, 2490367, 2148663333, 2148761618, 
    2150170623, 4294967295, 4294967295, 4294967295, 2148925439, 4294967295, 
    4294967295, 4980735, 6488063, 2147549282, 4294967295, 2147680355, 2150694911, 
    6553637, 6619236, 2150170623, 4294967295, 4294967295, 4294967295, 6750207, 
    4294967295, 4294967295, 2154299391, 6881279, 2147549288, 4294967295, 
    2147713129, 2490367, 4294901797, 4294967295, 2150170623, 4294967295, 
    4294967295, 4294967295, 7012351, 4294967295, 4294967295, 7077887, 2154627071, 
    4294934636, 4294967295, 4294967295, 4294901869, 4294967295, 4294967295, 
    4294901772, 1507351, 4294934529, 7274495, 4294934531, 1671216, 7274607, 
    4294901872, 4294901788, 4294967295, 4294967295, 4294967295, 4294934536, 
    4294967295, 4294967295, 4294901873, 2752554, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 7536639, 2155085823, 4294934643, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901876, 
    4294967295, 4294967295, 4294967295, 2148925439, 4294967295, 4294967295, 
    3473407, 7733247, 2148401269, 4294967295, 2152988790, 7864319, 7864439, 
    7929976, 2150170623, 4294967295, 4294967295, 4294967295, 2149449727, 
    4294967295, 4294967295, 8060927, 8126463, 4294901883, 4294967295, 4294967295, 
    4294901884, 4294967295, 4294967295, 4294901885, 8257662, 4294934655, 
    2155937791, 4294934544, 1114129, 2147844101, 4294934534, 4294934548, 
    4294967295, 4294967295, 4294967295, 4294934557, 4294967295, 4294967295, 
    4294901889, 5963867, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 8585215, 6160383, 4294901853, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901891, 4294967295, 4294967295, 4294967295, 
    8716287, 4294967295, 4294967295, 3473407, 8781823, 2148401285, 4294967295, 
    2148565126, 2150694911, 2151022609, 2151186486, 2148859903, 4294967295, 
    4294967295, 4294967295, 8912895, 4294967295, 4294967295, 2154299391, 
    2156462079, 2148434056, 4294967295, 2148565129, 2150694911, 4294901777, 
    4294967295, 2148859903, 4294967295, 4294967295, 4294967295, 9109503, 
    4294967295, 4294967295, 2156658687, 2156724223, 4294934668, 4294967295, 
    4294967295, 4294901901, 4294967295, 4294967295, 4294901867, 2156822670, 
    4294934529, 9437183, 4294934531, 2457648, 9437328, 4294901905, 4294934568, 
    4294967295, 4294967295, 4294967295, 4294934536, 4294967295, 4294967295, 
    4294901906, 9633939, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 9764863, 9830399, 4294901909, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901910, 4294967295, 4294967295, 4294967295, 
    2148925439, 4294967295, 4294967295, 2154299391, 9961471, 4294901911, 
    4294967295, 4294901798, 10027007, 2490520, 2555942, 10092543, 4294967295, 
    4294967295, 4294967295, 2148925439, 4294967295, 4294967295, 10158079, 
    2154627071, 4294934636, 4294967295, 4294967295, 4294901915, 4294967295, 
    4294967295, 4294901772, 1507351, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 10289151, 10354687, 4294901917, 4294967295, 
    4294967295, 4294934536, 4294967295, 4294967295, 4294901918, 2752554, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 10485759, 
    2155085823, 4294934643, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901920, 4294967295, 4294967295, 4294967295, 10616831, 
    4294967295, 4294967295, 851967, 917503, 4294901773, 4294967295, 4294967295, 
    4294901922, 4294967295, 4294967295, 4294934631, 2158198947, 4294967295, 
    4294967295, 10813439, 4294967295, 4294967295, 2158362623, 2158428159, 
    2148434086, 4294967295, 2152988839, 7864319, 11010167, 11075752, 2150170623, 
    4294967295, 4294967295, 4294967295, 2149449727, 4294967295, 4294967295, 
    11206655, 11272191, 4294901931, 4294967295, 4294967295, 4294901932, 
    4294967295, 4294967295, 4294901885, 11337901, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 11468799, 11534335, 4294901935, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901936, 
    4294967295, 4294967295, 4294967295, 11665407, 4294967295, 4294967295, 
    3473407, 11730943, 2152267954, 4294967295, 2148565171, 2150694911, 2151055408, 
    2151186486, 2150170623, 4294967295, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 11862015, 11927551, 4294901941, 4294967295, 
    4294967295, 4294901942, 4294967295, 4294967295, 4294901812, 5374034, 
    4294934542, 12058623, 4294934612, 5603376, 12058808, 4294901945, 4294901846, 
    4294967295, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294901812, 12189882, 4294967295, 4294967295, 2148925439, 4294967295, 
    4294967295, 2154299391, 2159804415, 4294934715, 4294967295, 4294967295, 
    4294934549, 4294967295, 4294967295, 4294934716, 2159902909, 4294967295, 
    4294967295, 2148925439, 4294967295, 4294967295, 3473407, 8781823, 2148401285, 
    4294967295, 2152988755, 2150694911, 2149220400, 2149285914, 12517375, 
    4294967295, 4294967295, 4294967295, 2148925439, 4294967295, 4294967295, 
    2154299391, 2156462079, 4294934664, 4294967295, 4294967295, 4294934549, 
    4294967295, 4294967295, 4294934719, 2156691596, 4294967295, 4294967295, 
    2148925439, 4294967295, 4294967295, 2160132095, 12713983, 4294901953, 
    4294967295, 4294967295, 4294934536, 4294967295, 4294967295, 4294901954, 
    9633939, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    12845055, 9830399, 4294901909, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901956, 4294967295, 4294967295, 4294967295, 12976127, 
    4294967295, 4294967295, 851967, 1572863, 2147549207, 4294967295, 2147713121, 
    2150694911, 2148663333, 2148761618, 2150170623, 4294967295, 4294967295, 
    4294967295, 13041663, 4294967295, 4294967295, 2154299391, 13107199, 
    2147549383, 4294967295, 2147713129, 2150694911, 4294901797, 4294967295, 
    2150170623, 4294967295, 4294967295, 4294967295, 13172735, 4294967295, 
    4294967295, 13238271, 2160787455, 4294934730, 4294967295, 4294901823, 
    10027007, 4128920, 4194367, 10092543, 4294967295, 4294967295, 4294967295, 
    13369343, 4294967295, 4294967295, 2154299391, 13434879, 2160918732, 
    4294967295, 2148565198, 2150694911, 2147844144, 2147909637, 2150170623, 
    4294967295, 4294934529, 13631487, 4294934531, 2424869, 13631696, 4294901969, 
    4294934568, 4294967295, 4294967295, 4294967295, 4294901970, 4294967295, 
    4294967295, 4294901772, 851981, 4294967295, 4294967295, 13828095, 4294967295, 
    4294967295, 2154299391, 2161377279, 4294934739, 4294967295, 4294967295, 
    4294901972, 4294967295, 4294967295, 4294934741, 2161541334, 4294967295, 
    4294967295, 2148925439, 4294967295, 4294967295, 2154299391, 2158231551, 
    4294934691, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294934743, 2158395558, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 14221311, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901977, 4294967295, 4294967295, 4294901812, 5374034, 4294934542, 
    2156331007, 4294934612, 7798903, 2151055414, 4294934584, 4294934568, 
    4294967295, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294901940, 14287066, 4294934542, 14417919, 4294934612, 7831600, 14418140, 
    4294901981, 4294934568, 4294967295, 4294967295, 4294967295, 4294901982, 
    4294967295, 4294967295, 4294934631, 14614751, 4294934542, 2156527615, 
    4294934612, 7798903, 4294967295, 4294967295, 4294934568, 4294967295, 
    4294967295, 4294967295, 4294901984, 4294967295, 4294967295, 4294901985, 
    2162327778, 4294967295, 4294967295, 2148925439, 4294967295, 4294967295, 
    3473407, 11730943, 2148401330, 4294967295, 2152988755, 14942207, 2149187811, 
    2149285914, 10092543, 4294967295, 4294967295, 4294967295, 15007743, 
    4294967295, 4294967295, 3473407, 5439487, 2148401234, 4294967295, 2152988901, 
    2150694911, 15073365, 15139046, 5701631, 4294967295, 4294967295, 4294967295, 
    2149449727, 4294967295, 4294967295, 15269887, 8126463, 4294901883, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901993, 2155053171, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 15400959, 
    4294967295, 2155872255, 4294967295, 2148565120, 2150694911, 2147811345, 
    2147909637, 2148859903, 4294967295, 4294934542, 15466495, 4294934612, 
    15466732, 15532269, 4294901998, 4294901913, 4294967295, 4294967295, 
    4294967295, 4294901869, 4294967295, 4294967295, 4294934631, 15663343, 
    4294967295, 4294967295, 2148925439, 4294967295, 4294967295, 15794175, 
    2163343359, 4294934769, 4294967295, 4294967295, 4294934549, 4294967295, 
    4294967295, 4294934631, 15859954, 4294967295, 2555903, 4294967295, 2424869, 
    2490406, 4294901799, 4294901913, 4294967295, 4294967295, 4294967295, 
    4294934549, 4294967295, 4294967295, 4294902003, 2160754890, 4294967295, 
    4294967295, 16056319, 4294967295, 4294967295, 851967, 1572863, 4294901783, 
    4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 4294934631, 
    16056565, 4294967295, 16187391, 4294967295, 9994288, 16122102, 4294902007, 
    4294901913, 4294967295, 4294967295, 4294967295, 4294934536, 4294967295, 
    4294967295, 4294902008, 4325442, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 16383999, 4653055, 4294901830, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902010, 4294967295, 4294967295, 
    4294967295, 2148925439, 4294967295, 4294967295, 10289151, 16515071, 
    4294902011, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294934780, 16580861, 4294967295, 4294967295, 13959167, 4294967295, 
    4294967295, 14811135, 2164195327, 2148434174, 4294967295, 2152988927, 
    2150694911, 16777335, 16843008, 2150170623, 4294967295, 4294967295, 
    4294967295, 2149449727, 4294967295, 4294967295, 16973823, 17039359, 
    4294902019, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902020, 17105157, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 17235967, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934549, 4294967295, 4294967295, 4294934631, 17236231, 4294967295, 
    7929855, 4294967295, 17301768, 7864440, 4294901881, 4294901788, 4294967295, 
    4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 4294902025, 
    2162327778, 4294967295, 4294967295, 17498111, 4294967295, 4294967295, 
    3473407, 5439487, 4294901842, 4294967295, 4294967295, 4294902027, 4294967295, 
    4294967295, 4294901885, 17563916, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 17694719, 17760255, 4294902030, 4294967295, 
    4294967295, 4294934557, 4294967295, 4294967295, 4294902031, 8061051, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 17891327, 
    2155085823, 4294934643, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294902033, 4294967295, 4294967295, 4294967295, 18022399, 
    4294967295, 4294967295, 3473407, 11730943, 4294901938, 4294967295, 4294967295, 
    4294902035, 4294967295, 4294967295, 4294934631, 2165604628, 4294967295, 
    4294967295, 18219007, 4294967295, 4294967295, 2165768191, 2165833727, 
    4294934807, 4294967295, 4294967295, 4294934557, 4294967295, 4294967295, 
    4294902040, 8061051, 4294967295, 4294967295, 18481151, 4294967295, 4294967295, 
    2166030335, 18612223, 4294902043, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902044, 2155053171, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 18743295, 4294967295, 4294967295, 
    4294967295, 4294901823, 2490367, 4128805, 4194367, 10092543, 4294967295, 
    4294967295, 18808831, 4294967295, 9994288, 18743582, 4294902047, 4294901913, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901829, 9765013, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 18939903, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901869, 4294967295, 4294967295, 4294934631, 18940193, 4294967295, 
    4294967295, 2148925439, 4294967295, 4294967295, 19070975, 2166620159, 
    2147582243, 4294967295, 2147680548, 1703935, 19202073, 19267877, 1900543, 
    4294967295, 4294934529, 19202047, 4294934531, 19333415, 19202341, 4294902054, 
    4294901788, 4294967295, 4294967295, 4294967295, 4294934557, 4294967295, 
    4294967295, 4294902056, 16974083, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 19529727, 17170431, 4294902021, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294902058, 4294967295, 
    4294967295, 4294967295, 19660799, 4294967295, 4294967295, 3473407, 5439487, 
    2148401234, 4294967295, 2153021574, 2150694911, 2151022711, 2151186486, 
    2150170623, 4294967295, 4294967295, 4294967295, 19726335, 4294967295, 
    4294967295, 2154299391, 19791871, 2148401453, 4294967295, 2153021577, 
    2150694911, 4294901879, 4294967295, 2150170623, 4294967295, 4294967295, 
    4294967295, 19857407, 4294967295, 4294967295, 19922943, 2167472127, 
    4294934832, 4294967295, 4294901928, 17367039, 11010312, 11075752, 1900543, 
    4294967295, 4294967295, 4294967295, 20054015, 4294967295, 4294967295, 
    2154299391, 20119551, 4294902066, 4294967295, 4294967295, 4294934549, 
    4294967295, 4294967295, 4294901940, 20119859, 4294934542, 12058623, 
    4294934612, 14909488, 12058808, 4294901945, 4294901913, 2148466687, 
    4294967295, 2152988980, 7864319, 20250743, 20316469, 2150170623, 4294967295, 
    4294967295, 4294967295, 20447231, 4294967295, 4294967295, 3473407, 11730943, 
    4294901938, 4294967295, 4294967295, 4294902071, 4294967295, 4294967295, 
    4294934631, 2167963960, 4294967295, 4294967295, 9306111, 4294967295, 
    4294967295, 2168061951, 2168127487, 4294934842, 4294967295, 4294967295, 
    4294934549, 4294967295, 4294967295, 4294934631, 2165604628, 4294967295, 
    4294967295, 2148925439, 4294967295, 4294967295, 2168193023, 2165833727, 
    4294934807, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294934844, 20775229, 4294934542, 12058623, 4294934612, 15499312, 12058808, 
    4294901945, 4294901913, 4294967295, 4294967295, 4294967295, 4294902071, 
    4294967295, 4294967295, 4294934631, 2168357182, 4294967295, 4294967295, 
    9306111, 4294967295, 4294967295, 2168455167, 2168520703, 4294934848, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294902081, 
    4294967295, 4294967295, 4294967295, 21168127, 4294967295, 4294967295, 
    7077887, 2168717311, 4294934851, 4294967295, 4294901798, 21299199, 2490692, 
    2555942, 10092543, 4294967295, 4294967295, 4294967295, 2148073471, 4294967295, 
    4294967295, 21364735, 2818047, 4294901802, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902086, 2155053171, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 21495807, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902088, 4294967295, 4294967295, 4294934857, 
    21627210, 4294967295, 4294967295, 15007743, 4294967295, 4294967295, 
    2154299391, 21757951, 4294902091, 4294967295, 4294967295, 4294934549, 
    4294967295, 4294967295, 4294902092, 2169340237, 4294967295, 4294967295, 
    2148925439, 4294967295, 4294967295, 2154299391, 21954559, 4294902094, 
    4294967295, 4294901880, 7864319, 7864439, 7929976, 1900543, 4294967295, 
    4294967295, 4294967295, 2148925439, 4294967295, 4294967295, 22020095, 
    2167472127, 4294934832, 4294967295, 4294967295, 4294902096, 4294967295, 
    4294967295, 4294901812, 5374034, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 2154299391, 22151167, 4294902097, 4294967295, 
    4294902098, 2150694911, 22151432, 22217042, 1900543, 4294967295, 4294934542, 
    15073279, 4294934612, 14909488, 15073510, 4294901991, 4294901913, 4294967295, 
    4294967295, 4294967295, 4294934557, 4294967295, 4294967295, 4294902100, 
    11206827, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    22413311, 11534335, 4294901935, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902102, 4294967295, 4294967295, 4294967295, 
    2148925439, 4294967295, 4294967295, 17694719, 22544383, 2148401495, 
    4294967295, 2152988901, 2150694911, 15073516, 15139046, 10092543, 4294967295, 
    4294967295, 4294967295, 9306111, 4294967295, 4294967295, 7077887, 2170093567, 
    4294934872, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294901914, 2168684867, 4294967295, 4294967295, 22675455, 4294967295, 
    4294967295, 2154299391, 22740991, 4294902106, 4294967295, 4294967295, 
    4294901970, 4294967295, 4294967295, 4294934631, 2170257755, 4294967295, 
    4294967295, 13959167, 4294967295, 4294967295, 2170355711, 2170421247, 
    4294934877, 4294967295, 4294967295, 4294934549, 4294967295, 4294967295, 
    4294934878, 23003487, 4294934529, 5177343, 4294934531, 19365936, 5177423, 
    4294901840, 4294901788, 4294967295, 4294967295, 4294901928, 7864319, 
    11010167, 11075752, 1900543, 4294967295, 4294967295, 23134207, 4294967295, 
    17334320, 23069024, 4294902113, 4294901788, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901934, 17105157, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 23265279, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901988, 4294967295, 
    4294967295, 4294934631, 23265635, 4294967295, 4294967295, 2148925439, 
    4294967295, 4294967295, 23396351, 2170945535, 2148434277, 4294967295, 
    2152988907, 14942207, 15532259, 15597805, 10092543, 4294967295, 4294967295, 
    4294967295, 2148925439, 4294967295, 4294967295, 10289151, 23527423, 
    4294902118, 4294967295, 4294902006, 2150694911, 16122180, 16187638, 
    10092543, 4294967295, 4294934529, 7274495, 4294934531, 19365936, 7274607, 
    4294901872, 4294901788, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902119, 4294967295, 4294967295, 4294967295, 
    23658495, 4294967295, 4294967295, 14811135, 2171207679, 4294934889, 
    4294967295, 4294901880, 23789567, 7864682, 7929976, 1900543, 4294967295, 
    4294934529, 23855103, 4294934531, 2457648, 23855468, 4294902125, 4294934568, 
    4294967295, 4294967295, 4294967295, 4294901972, 4294967295, 4294967295, 
    4294901985, 2171502958, 4294967295, 4294967295, 2148925439, 4294967295, 
    4294967295, 17432575, 2171207679, 4294934889, 4294967295, 4294967295, 
    4294902127, 4294967295, 4294967295, 4294934631, 24117616, 4294967295, 
    4294967295, 2148073471, 4294967295, 4294967295, 16318463, 9699327, 4294901907, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294902009, 
    9765013, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    2171731967, 4294967295, 4294967295, 4294967295, 4294967295, 4294934549, 
    4294967295, 4294967295, 4294902029, 2171765106, 4294967295, 22216703, 
    4294967295, 23756848, 22151506, 4294902099, 4294901788, 65535, 18, 29, 
    30, 34, 39, 42, 49, 0, 1, 31, 4, 3, 1, 5, 2, 100, 112, 0, 112, 120, 
    124, 0, 124, 0, 124, 0, 124, 0, 124, 0, 124, 0, 124, 0, 0, 1, 2, 4294901760, 
    65535, 2147647489, 47, 4294967295, 6, 1, 4294967295, 13, 100, 124, 6, 
    148, 160, 188, 0, 188, 0, 188, 0, 188, 0, 188, 0, 188, 0, 188, 0, 0, 
    2, 2, 2, 1, 2, 2, 2, 2, 2, 0, 2, 4294901760, 4294967295, 4294967295, 
    4294934529, 2147680255, 4294934531, 2147778564, 4294967295, 4294967295, 
    32773, 50, 4294967295, 7, 1, 4294967295, 13, 100, 128, 7, 156, 172, 
    200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 200, 0, 0, 2, 2, 2, 
    1, 2, 2, 2, 2, 2, 2, 0, 2, 2, 4294901760, 4294967295, 4294967295, 65535, 
    4294934529, 2147680255, 4294934531, 2147844100, 4294967295, 4294967295, 
    32774, 28, 4294967295, 1, 0, 0, 0, 100, 104, 1, 108, 112, 112, 0, 112, 
    0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 2, 2, 65535
];