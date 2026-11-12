// Generated from ParenExpr.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","8");
pub const ParenExpr_T__0:i32=1; 
pub const ParenExpr_T__1:i32=2; 
pub const ParenExpr_T__2:i32=3; 
pub const ParenExpr_T__3:i32=4; 
pub const ParenExpr_T__4:i32=5; 
pub const ParenExpr_T__5:i32=6; 
pub const ParenExpr_T__6:i32=7; 
pub const ParenExpr_T__7:i32=8; 
pub const ParenExpr_ID:i32=9; 
pub const ParenExpr_INT:i32=10; 
pub const ParenExpr_WS:i32=11;
pub const ParenExpr_EOF:i32=EOF;
pub const RULE_s:usize = 0; 
pub const RULE_e:usize = 1; 
pub const RULE_q:usize = 2;
pub const ruleNames: [&'static str; 3] = [
    "s", "e", "q"
];

pub const _LITERAL_NAMES: [Option<&'static str>;9] = [
	None, Some("'('"), Some("','"), Some("')'"), Some("'-'"), Some("'*'"), 
	Some("'/'"), Some("'+'"), Some("'SELECT'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, None, None, None, None, None, None, None, None, Some("ID"), Some("INT"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, ParenExprParserExt<'input, 'arena>, ParenExprParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				ParenExprParserExt {
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
        L: ParseTreeListener<'arena, ParenExprParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct ParenExprTreeWalker;
impl ParenExprTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, ParenExprParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, ParenExprParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <ParenExprParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum ParenExprParserNodeKind {
    SContext,
    EContext,
    QContext,
    Terminal,
    Error,
}
pub type ParenExprParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, ParenExprParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => ParenExprParser }
dbt_antlr_runtime::impl_node_kind! { ParenExprParserNodeKind {
    EContext(EContextAll), QContext(QContextAll), ; SContext(), 
    }; 
}

pub struct ParenExprParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> ParenExprParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { ParenExprParserExt, ParenExprParserNodeKind, "ParenExpr.g4"; sempred (ParenExprParserNode<'input, 'arena, TF::Tok>, ParenExprParser) { 1 => e_sempred, } }

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn e_sempred(_ctx: Option<&'arena EContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
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
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { ParenExprParserNodeKind, SContext, SContextExt, RULE_s }

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
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_EOF)
    }
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
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
		recog.base.set_state(6);
		recog.e_rec(0)?;
		recog.base.set_state(7);
		recog.base.match_token(ParenExpr_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- e ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum EContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	AddContext(AddContext<'input, 'arena, Tok>),
	ColContext(ColContext<'input, 'arena, Tok>),
	ParensContext(ParensContext<'input, 'arena, Tok>),
	SubqueryContext(SubqueryContext<'input, 'arena, Tok>),
	RowConstructorContext(RowConstructorContext<'input, 'arena, Tok>),
	MulContext(MulContext<'input, 'arena, Tok>),
	NumContext(NumContext<'input, 'arena, Tok>),
	UnaryContext(UnaryContext<'input, 'arena, Tok>),
    Error(EContext<'input, 'arena, Tok>)
}

dbt_antlr_runtime::impl_rule_context! { EContextAll { } { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr_runtime::impl_parser_rule_context! { EContextAll { } { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr_runtime::impl_tree_trait_delegates! { ParenExprParserNodeKind::EContextAll { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }
dbt_antlr_runtime::impl_node_inner! { ParenExprParserNodeKind::EContext::EContextAll { AddContext, ColContext, ParensContext, SubqueryContext, RowConstructorContext, MulContext, NumContext, UnaryContext, Error, } }

impl<'input, 'arena, Tok> Deref for EContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn EContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use EContextAll::*;
		match self{
			AddContext(inner) => inner,
			ColContext(inner) => inner,
			ParensContext(inner) => inner,
			SubqueryContext(inner) => inner,
			RowConstructorContext(inner) => inner,
			MulContext(inner) => inner,
			NumContext(inner) => inner,
			UnaryContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type EContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct EContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @labeled ParenExprParserNodeKind::EContext, EContextAll, EContextExt, RULE_e }
impl<'input: 'arena, 'arena, Tok: Token + 'input> EContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ParenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ParenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, EContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait EContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> EContextAttrs<'input, 'arena, Tok> for EContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type AddContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AddContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait AddContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AddContextAttrs<'input, 'arena, Tok> for AddContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct AddContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::AddContext, AddContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for AddContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AddContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::AddContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type ColContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ColContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

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
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_ID)
    }
}
#[derive(Debug)]
pub struct ColContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::ColContext, ColContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for ColContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ColContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::ColContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type ParensContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParensContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait ParensContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParensContextAttrs<'input, 'arena, Tok> for ParensContext<'input, 'arena, Tok>
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct ParensContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::ParensContext, ParensContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for ParensContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParensContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::ParensContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type SubqueryContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SubqueryContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait SubqueryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SubqueryContextAttrs<'input, 'arena, Tok> for SubqueryContext<'input, 'arena, Tok>
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct SubqueryContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::SubqueryContext, SubqueryContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for SubqueryContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SubqueryContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::SubqueryContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type RowConstructorContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RowConstructorContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait RowConstructorContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RowConstructorContextAttrs<'input, 'arena, Tok> for RowConstructorContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct RowConstructorContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::RowConstructorContext, RowConstructorContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for RowConstructorContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RowConstructorContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::RowConstructorContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type MulContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, MulContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait MulContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>>;
	fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> MulContextAttrs<'input, 'arena, Tok> for MulContext<'input, 'arena, Tok>
{
    fn e_all(&self) -> Vec<&'arena EContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn e(&self, i: usize) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}
#[derive(Debug)]
pub struct MulContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::MulContext, MulContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for MulContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> MulContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::MulContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type NumContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NumContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

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
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_INT)
    }
}
#[derive(Debug)]
pub struct NumContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::NumContext, NumContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for NumContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NumContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::NumContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type UnaryContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, UnaryContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait UnaryContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> UnaryContextAttrs<'input, 'arena, Tok> for UnaryContext<'input, 'arena, Tok>
{
    fn e(&self) -> Option<&'arena EContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct UnaryContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: EContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::EContext, EContextAll::UnaryContext, UnaryContextExt, RULE_e }

impl<'input, 'arena, Tok> EContextAttrs<'input, 'arena, Tok> for UnaryContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> UnaryContextExt<'input, 'arena, Tok> {
	fn new(base: EContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut EContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            EContextAll::UnaryContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut EContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
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
		if let Some(_node) = self.base.resume_take(RULE_e)? {
		    return Ok(_node.as_rule_context().unwrap());
		}
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, EContext<TF::Tok>, RULE_e, 2, |_parentctx| EContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena EContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(32);
		recog.err_handler.sync(&mut recog.base)?;
		match {
			let _vap = recog.base.dfa_predict_virtual(1)?;
			match _vap {
				65536 => dbt_antlr_runtime::shared_descent!(
					recog,
					call_sites: &[13, 23],
					prefix: 1,
					rule: RULE_e,
					invoke: e_rec(0),
					tails: { ParenExpr_T__1  => 3, ParenExpr_T__2  => 4, },
					default: 3,
					fallback: 3,
				),
				_vap => _vap,
			}
		} {
			1 =>{
				{
				recog.base.with_mut_ctx(|ctx| { NumContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena NumContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

				recog.base.set_state(10);
				recog.base.match_token(ParenExpr_INT,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				{
				recog.base.with_mut_ctx(|ctx| { ColContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena ColContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(11);
				recog.base.match_token(ParenExpr_ID,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				{
				recog.base.with_mut_ctx(|ctx| { RowConstructorContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena RowConstructorContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(12);
				recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
				recog.base.set_state(13);
				recog.e_rec(0)?;
				recog.base.set_state(16); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				loop {
					{
					{
					recog.base.set_state(14);
					recog.base.match_token(ParenExpr_T__1,&mut recog.err_handler)?;
					recog.base.set_state(15);
					recog.e_rec(0)?;
					}
					}
					recog.base.set_state(18); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if !(_la==ParenExpr_T__1) {break}
				}
				recog.base.set_state(20);
				recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				{
				recog.base.with_mut_ctx(|ctx| { ParensContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena ParensContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(22);
				recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
				recog.base.set_state(23);
				recog.e_rec(0)?;
				recog.base.set_state(24);
				recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
				}
			}
		,
			5 =>{
				{
				recog.base.with_mut_ctx(|ctx| { SubqueryContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena SubqueryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(26);
				recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
				recog.base.set_state(27);
				recog.q()?;
				recog.base.set_state(28);
				recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
				}
			}
		,
			6 =>{
				{
				recog.base.with_mut_ctx(|ctx| { UnaryContextExt::copy_from(ctx); });
				let _local_ctx_fn = |recog: &Self| -> &'arena UnaryContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
				recog.base.set_state(30);
				recog.base.match_token(ParenExpr_T__3,&mut recog.err_handler)?;
				recog.base.set_state(31);
				recog.e_rec(3)?;
				}
			}

			_ => {}
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(42);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = { let _sdp = recog.base.dfa_predict(3)?; _sdp };
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(40);
				recog.err_handler.sync(&mut recog.base)?;
				match { let _sdp = recog.base.dfa_predict(2)?; _sdp } {
					1 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						MulContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena MulContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(34);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(35);
						_la = recog.base.input.la(1);
						if { !(_la==ParenExpr_T__4 || _la==ParenExpr_T__5) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(36);
						recog.e_rec(3)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleLabeledAltStartAction*/
						let tmp = EContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						AddContextExt::copy_from(tmp);
						let _prevctx = recog.push_new_recursion_context(tmp, _startState, RULE_e)?;
						let _local_ctx_fn = |recog: &Self| -> &'arena AddContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};

						recog.base.set_state(37);
						if !({recog.precpred(None, 1)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
						}
						recog.base.set_state(38);
						_la = recog.base.input.la(1);
						if { !(_la==ParenExpr_T__3 || _la==ParenExpr_T__6) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(39);
						recog.e_rec(2)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(44);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = { let _sdp = recog.base.dfa_predict(3)?; _sdp };
		}
		}
		})
	}
}
//------------------- q ----------------
#[derive(Debug)]
#[repr(C, u16)]
pub enum QContextAll<'input, 'arena, Tok = CommonToken<'input>>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	SelectContext(SelectContext<'input, 'arena, Tok>),
	TableContext(TableContext<'input, 'arena, Tok>),
	NestedContext(NestedContext<'input, 'arena, Tok>),
    Error(QContext<'input, 'arena, Tok>)
}

dbt_antlr_runtime::impl_rule_context! { QContextAll { } { SelectContext, TableContext, NestedContext, Error, } }
dbt_antlr_runtime::impl_parser_rule_context! { QContextAll { } { SelectContext, TableContext, NestedContext, Error, } }
dbt_antlr_runtime::impl_tree_trait_delegates! { ParenExprParserNodeKind::QContextAll { SelectContext, TableContext, NestedContext, Error, } }
dbt_antlr_runtime::impl_node_inner! { ParenExprParserNodeKind::QContext::QContextAll { SelectContext, TableContext, NestedContext, Error, } }

impl<'input, 'arena, Tok> Deref for QContextAll<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	type Target = dyn QContextAttrs<'input, 'arena, Tok> + 'arena;
	fn deref(&self) -> &Self::Target{
		use QContextAll::*;
		match self{
			SelectContext(inner) => inner,
			TableContext(inner) => inner,
			NestedContext(inner) => inner,
            Error(inner) => inner
		}
	}
}

pub type QContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, QContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;
#[derive(Debug)]
pub struct QContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @labeled ParenExprParserNodeKind::QContext, QContextAll, QContextExt, RULE_q }
impl<'input: 'arena, 'arena, Tok: Token + 'input> QContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ParenExprParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ParenExprParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, QContextExt {
				ph: PhantomData
			}
		)
	}
}
pub trait QContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> QContextAttrs<'input, 'arena, Tok> for QContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

pub type SelectContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SelectContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait SelectContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SelectContextAttrs<'input, 'arena, Tok> for SelectContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_ID)
    }
}
#[derive(Debug)]
pub struct SelectContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: QContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::QContext, QContextAll::SelectContext, SelectContextExt, RULE_q }

impl<'input, 'arena, Tok> QContextAttrs<'input, 'arena, Tok> for SelectContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SelectContextExt<'input, 'arena, Tok> {
	fn new(base: QContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut QContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            QContextAll::SelectContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut QContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type TableContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TableContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait TableContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TableContextAttrs<'input, 'arena, Tok> for TableContext<'input, 'arena, Tok>
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ParenExpr_ID)
    }
}
#[derive(Debug)]
pub struct TableContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: QContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::QContext, QContextAll::TableContext, TableContextExt, RULE_q }

impl<'input, 'arena, Tok> QContextAttrs<'input, 'arena, Tok> for TableContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TableContextExt<'input, 'arena, Tok> {
	fn new(base: QContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut QContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            QContextAll::TableContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut QContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

pub type NestedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NestedContextExt<'input, 'arena, Tok>, ParenExprParserNodeKind, Tok>;

pub trait NestedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
	fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NestedContextAttrs<'input, 'arena, Tok> for NestedContext<'input, 'arena, Tok>
{
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}
#[derive(Debug)]
pub struct NestedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	base: QContextExt<'input, 'arena, Tok>,
    pd: PhantomData<(&'arena (), &'input Tok)>
}

dbt_antlr_runtime::impl_ctx! { @alt ParenExprParserNodeKind::QContext, QContextAll::NestedContext, NestedContextExt, RULE_q }

impl<'input, 'arena, Tok> QContextAttrs<'input, 'arena, Tok> for NestedContext<'input, 'arena, Tok>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NestedContextExt<'input, 'arena, Tok> {
	fn new(base: QContextExt<'input, 'arena, Tok>) -> Self {
        Self {
            base,
            pd: PhantomData
        }
    }

	fn copy_from(src: &mut ParenExprParserNode<'input, 'arena, Tok>) {
        let invoking_state = src.get_invoking_state();
        let ctx = {
            let Some(base_ctx): Option<&mut QContext<'input, 'arena, Tok>> = src.as_rule_context_mut() else {
                panic!("invalid node type for copy_from!");
            };
            let tmp = unsafe { std::ptr::read(base_ctx) };
            QContextAll::NestedContext(tmp.morph(|ext_src| Self::new(ext_src)))
        };
let _old = std::mem::replace(dbt_antlr_runtime::cast_unchecked!(src => mut QContextAll<'input, 'arena, Tok>), ctx);
        std::mem::forget(_old);
        src.set_invoking_state(invoking_state);
        src.node_tag = <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag();
	}
}

impl<'input, 'arena, Input, TF> ParenExprParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn q(&mut self,) -> Result<&'arena QContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, QContext<TF::Tok>, RULE_q, 4, |_parentctx| QContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena QContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(52);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    ParenExpr_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            SelectContextExt::copy_from(ctx);
		            ctx.set_alt_number(1);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena SelectContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(45);
		        recog.base.match_token(ParenExpr_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(46);
		        recog.base.match_token(ParenExpr_ID,&mut recog.err_handler)?;
		        }}
		    ParenExpr_ID  => {
		        /*------- Outer Most Alt 2 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            TableContextExt::copy_from(ctx);
		            ctx.set_alt_number(2);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena TableContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(47);
		        recog.base.match_token(ParenExpr_ID,&mut recog.err_handler)?;
		        }}
		    ParenExpr_T__0  => {
		        /*------- Outer Most Alt 3 -------*/
		        recog.base.with_mut_ctx(|ctx| {
		            NestedContextExt::copy_from(ctx);
		            ctx.set_alt_number(3);
		        });
		        let _local_ctx_fn = |recog: &Self| -> &'arena NestedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		        {
		        recog.base.set_state(48);
		        recog.base.match_token(ParenExpr_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(49);
		        recog.q()?;
		        recog.base.set_state(50);
		        recog.base.match_token(ParenExpr_T__2,&mut recog.err_handler)?;
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
static _serializedATN: [u32; 754] = [
    1346458702, 3, 16909060, 29, 11, 55, 61, 2, 3, 5, 3, 29, 385, 414, 305, 
    719, 10, 729, 6, 743, 5, 748, 3, 751, 3, 754, 4, 735, 8, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 4294967295, 
    1, 0, 8, 3, 1, 4294967295, 4294967295, 1, 0, 32, 4, 1, 4294967295, 4294967295, 
    1, 0, 8, 5, 1, 4294967295, 4294967295, 1, 1, 72, 6, 1, 4294967295, 4294967295, 
    1, 1, 32, 7, 1, 4294967295, 4294967295, 1, 1, 32, 8, 1, 4294967295, 
    4294967295, 1, 1, 32, 9, 1, 4294967295, 4294967295, 1, 1, 8, 10, 1, 
    4294967295, 4294967295, 1, 1, 32, 11, 1, 4294967295, 4294967295, 1, 
    1, 8, 12, 1, 4294967295, 4294967295, 4, 1, 8, 13, 1, 17, 4294967295, 
    8, 1, 8, 14, 1, 4294967295, 4294967295, 11, 1, 8, 15, 2, 4294967295, 
    4294967295, 12, 1, 8, 17, 1, 4294967295, 18, 1, 1, 32, 18, 1, 4294967295, 
    4294967295, 1, 1, 8, 19, 1, 4294967295, 4294967295, 1, 1, 32, 20, 1, 
    4294967295, 4294967295, 1, 1, 8, 21, 1, 4294967295, 4294967295, 1, 1, 
    32, 22, 1, 4294967295, 4294967295, 1, 1, 8, 23, 1, 4294967295, 4294967295, 
    1, 1, 32, 24, 1, 4294967295, 4294967295, 1, 1, 8, 25, 1, 4294967295, 
    4294967295, 1, 1, 32, 26, 1, 4294967295, 4294967295, 1, 1, 8, 27, 1, 
    4294967295, 4294967295, 1, 1, 32, 28, 1, 4294967295, 4294967295, 1, 
    1, 8, 29, 1, 4294967295, 4294967295, 3, 1, 8, 30, 6, 33, 4294967295, 
    8, 1, 8, 36, 1, 4294967295, 4294967295, 1, 1, 72, 37, 1, 4294967295, 
    4294967295, 1, 1, 32, 38, 1, 4294967295, 4294967295, 1, 1, 8, 39, 1, 
    4294967295, 4294967295, 1, 1, 72, 40, 1, 4294967295, 4294967295, 1, 
    1, 32, 41, 1, 4294967295, 4294967295, 1, 1, 8, 42, 1, 4294967295, 4294967295, 
    5, 1, 8, 43, 2, 41, 4294967295, 8, 1, 8, 45, 1, 4294967295, 4294967295, 
    10, 1, 10, 46, 2, 4294967295, 4294967295, 12, 1, 8, 48, 1, 4294967295, 
    44, 9, 1, 8, 49, 1, 4294967295, 4294967295, 1, 2, 32, 50, 1, 4294967295, 
    4294967295, 1, 2, 32, 51, 1, 4294967295, 4294967295, 1, 2, 32, 52, 1, 
    4294967295, 4294967295, 1, 2, 32, 53, 1, 4294967295, 4294967295, 1, 
    2, 8, 54, 1, 4294967295, 4294967295, 1, 2, 32, 55, 1, 4294967295, 4294967295, 
    1, 2, 8, 56, 1, 4294967295, 4294967295, 3, 2, 8, 57, 3, 53, 4294967295, 
    8, 2, 8, 60, 1, 4294967295, 4294967295, 1, 2, 0, 61, 0, 4294967295, 
    4294967295, 1, 6, 0, 0, 0, 1, 32, 0, 0, 0, 1, 52, 0, 0, 0, 3, 2, 1, 
    7, 0, 5, 8, 4294967295, 0, 0, 1, 1, 0, 0, 0, 6, 10, 1, 4294967295, 0, 
    5, 33, 10, 0, 0, 5, 33, 9, 0, 0, 5, 13, 1, 0, 0, 3, 2, 1, 16, 0, 5, 
    15, 2, 0, 0, 3, 2, 1, 17, 0, 1, 14, 0, 0, 0, 1, 18, 0, 0, 0, 1, 16, 
    0, 0, 0, 1, 19, 0, 0, 0, 1, 20, 0, 0, 0, 5, 21, 3, 0, 0, 1, 33, 0, 0, 
    0, 5, 23, 1, 0, 0, 3, 2, 1, 24, 0, 5, 25, 3, 0, 0, 1, 33, 0, 0, 0, 5, 
    27, 1, 0, 0, 3, 4, 2, 28, 0, 5, 29, 3, 0, 0, 1, 33, 0, 0, 0, 5, 31, 
    4, 0, 0, 3, 2, 1, 33, 3, 1, 9, 0, 0, 0, 1, 11, 0, 0, 0, 1, 12, 0, 0, 
    0, 1, 22, 0, 0, 0, 1, 26, 0, 0, 0, 1, 30, 0, 0, 0, 1, 42, 0, 0, 0, 10, 
    35, 2, 0, 0, 7, 36, 0, 0, 0, 3, 2, 1, 41, 3, 10, 38, 1, 0, 0, 7, 39, 
    1, 0, 0, 3, 2, 1, 41, 2, 1, 34, 0, 0, 0, 1, 37, 0, 0, 0, 1, 44, 0, 0, 
    0, 1, 40, 0, 0, 0, 1, 43, 0, 0, 0, 1, 3, 0, 0, 0, 1, 42, 0, 0, 0, 5, 
    46, 8, 0, 0, 5, 53, 9, 0, 0, 5, 53, 9, 0, 0, 5, 49, 1, 0, 0, 3, 4, 2, 
    50, 0, 5, 51, 3, 0, 0, 1, 53, 0, 0, 0, 1, 45, 0, 0, 0, 1, 47, 0, 0, 
    0, 1, 48, 0, 0, 0, 1, 5, 0, 0, 0, 0, 1, 1, 0, 2, 1, 2, 1, 2, 2, 5, 6, 
    4, 4, 7, 7, 96, 0, 0, 0, 144, 0, 0, 0, 18, 32, 40, 42, 52, 0, 2, 4, 
    1, 3, 5
];static _STATIC_PARSER_TABLES: LazyLock<StaticParserTables> = LazyLock::new(|| {
    StaticParserTables::from_image(&_STATIC_PARSER_TABLE_WORDS)
        .expect("invalid static parser DFA table image; regenerate the parser")
});
static _STATIC_PARSER_TABLE_WORDS: [u32; 2849] = [
    1396983361, 4, 2849, 4, 5, 1, 9, 4294967295, 0, 1, 2147483648, 0, 2666, 
    2666, 32, 2698, 43, 2741, 45, 2786, 28, 0, 2, 2, 5, 4, 1, 2, 2, 3, 4, 
    2, 2, 3, 4, 2666, 1, 425, 263, 4294967295, 12, 100, 1800, 425, 3500, 
    4352, 10664, 0, 10664, 0, 10664, 0, 10664, 0, 10664, 0, 10664, 0, 10664, 
    0, 0, 0, 6, 2, 1, 0, 0, 5, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 4, 0, 4, 65536, 
    0, 0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 65536, 0, 
    0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 3, 65536, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 65536, 0, 0, 0, 65536, 0, 65536, 0, 0, 65536, 0, 0, 0, 65536, 
    0, 0, 0, 65536, 0, 0, 0, 3, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 65536, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 0, 0, 65536, 
    0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 
    65536, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65536, 65536, 0, 3, 0, 0, 0, 0, 
    0, 65536, 0, 65536, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 65536, 
    0, 0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 65536, 0, 65536, 0, 0, 0, 65536, 
    0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 
    0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 
    0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 
    0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 3, 0, 0, 
    0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 3, 0, 0, 0, 3, 0, 0, 0, 
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 0, 0, 
    2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 4, 0, 4, 0, 0, 0, 4, 0, 
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 4, 4, 4, 0, 0, 3, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 4, 4, 4, 4, 
    4, 4, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 3, 
    0, 0, 0, 0, 0, 0, 4, 4, 3, 4, 4, 4, 4, 3, 3, 0, 0, 3, 3, 3, 3, 0, 3, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 3, 3, 3, 3, 3, 3, 0, 0, 
    0, 0, 0, 0, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 0, 3, 0, 3, 3, 3, 0, 3, 
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 0, 3, 3, 3, 3, 0, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 65536, 4294967295, 196607, 4294901763, 327684, 
    458751, 524295, 655369, 4294967295, 786431, 4294967295, 851980, 983054, 
    4294901776, 1179665, 1245202, 1376276, 1507350, 4294901784, 1703935, 
    1769471, 1835035, 1966109, 2097151, 2162720, 2228257, 4294967295, 2359331, 
    2424869, 2555942, 2687016, 2818090, 2949119, 3014701, 3145727, 3211263, 
    4294901809, 3342386, 4294901811, 3473460, 4294901814, 3670071, 4294901816, 
    3801087, 3866682, 4294901820, 4063293, 4128831, 4294901824, 4325441, 
    4390979, 4522052, 4653126, 3145800, 4849737, 4980735, 3407948, 5111885, 
    5242959, 5373951, 5439570, 5505107, 5636095, 4294901846, 5767255, 5832792, 
    5963866, 4294901852, 6160477, 6226014, 6357088, 6488063, 6553699, 6619236, 
    6750310, 6881384, 4294901866, 7077887, 7143423, 7209069, 7274606, 7405567, 
    7471103, 5439571, 7536754, 5570676, 4294901877, 7798902, 7864440, 4294901881, 
    8061050, 8192124, 4294901886, 8388735, 5439571, 4294901889, 4294901890, 
    8650883, 4294901892, 6553733, 4294901860, 8847494, 8913032, 4294901897, 
    9109642, 9240575, 9240717, 4294901872, 9371790, 9437328, 9568401, 9699475, 
    9830549, 9961623, 10092697, 10223771, 4294901917, 10420382, 10551456, 
    10682367, 10682531, 10748034, 4294901925, 4294901926, 11010215, 11075752, 
    11206826, 11272330, 11403263, 11468799, 11468975, 11599871, 11600049, 
    11730943, 11731064, 11862015, 4294901941, 11993270, 12058808, 4294901945, 
    7864506, 12320767, 12320956, 4294901949, 7864510, 12517542, 12648640, 
    10879170, 12845055, 12910788, 12976325, 13107399, 13172909, 13304010, 
    13435084, 4294901966, 4294901967, 13697232, 13566161, 13828306, 13959380, 
    14090454, 14221528, 14352383, 14352568, 14483455, 4294901981, 12910814, 
    4294901957, 14680287, 4294901984, 12058849, 4294901986, 7864547, 4294901967, 
    4294901988, 15073509, 4294901990, 15204583, 15270121, 15401194, 15532031, 
    15532269, 14418158, 15728879, 4294902001, 13697266, 15925489, 16056319, 
    4294902005, 16187638, 16253175, 16384249, 16515323, 16646397, 16711921, 
    16843008, 16908536, 17039619, 17170693, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 65535, 4294967295, 4294901761, 2147680255, 
    4294967295, 4294967295, 2147778563, 4294967295, 4294901765, 458751, 
    4294967295, 2148007935, 589832, 4294967295, 4294934538, 786431, 4294967295, 
    851967, 917517, 4294967295, 4294901775, 2148597759, 4294967295, 4294967295, 
    589833, 4294967295, 2148663295, 1277970, 1245203, 4294901779, 4294967295, 
    4294967295, 2148663295, 1277972, 1245203, 4294901779, 4294967295, 4294967295, 
    4294934549, 2148597759, 4294967295, 4294967295, 917518, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901782, 4294967295, 
    1572863, 1638424, 1638425, 4294901785, 4294967295, 4294967295, 1572863, 
    1638426, 1638425, 4294901785, 4294967295, 4294967295, 4294934555, 2148597759, 
    4294967295, 1900543, 1966109, 4294967295, 4294901791, 2162687, 4294967295, 
    4294967295, 2162721, 4294967295, 4294967295, 4294901768, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901794, 2359295, 4294967295, 
    4294967295, 2359332, 4294934565, 2150039551, 2588690, 2555943, 4294901799, 
    4294967295, 4294967295, 4294934568, 2752511, 4294967295, 4294967295, 
    2752554, 4294934565, 2150039551, 2588692, 2555943, 4294901799, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901803, 
    4294967295, 2949119, 2949146, 2949165, 4294901805, 4294967295, 4294967295, 
    4294934574, 3145727, 4294967295, 3211263, 3276849, 4294967295, 4294901811, 
    2148597759, 4294967295, 4294967295, 2162721, 4294967295, 2148663295, 
    1277972, 2150924340, 4294901779, 4294967295, 4294967295, 4294934581, 
    3604479, 4294967295, 3670015, 3735608, 4294967295, 4294901818, 2148597759, 
    4294967295, 4294967295, 2359332, 4294967295, 1572863, 3932219, 3932220, 
    4294901820, 4294967295, 4294967295, 4294901821, 4128767, 4294967295, 
    4294967295, 4128831, 4294967295, 4294934592, 2148597759, 4294967295, 
    4294967295, 2752554, 4294967295, 1572863, 1638426, 2150924340, 4294901785, 
    4294967295, 4294967295, 4294967295, 4294901769, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901825, 4390911, 4294967295, 4294967295, 
    4390979, 4294967295, 4294934596, 2148597759, 4294967295, 4294967295, 
    4522053, 4294967295, 4294934598, 2148597759, 4294967295, 4294967295, 
    3276850, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901831, 4294967295, 4784127, 2150891583, 2150924340, 4294934580, 
    4294967295, 4294967295, 4294934601, 2148597759, 4294967295, 4915199, 
    4980811, 4294967295, 4294934605, 2148597759, 4294967295, 4294967295, 
    3735609, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901838, 4294967295, 2150039551, 5242959, 5242960, 4294901840, 4294967295, 
    4294967295, 4294934609, 2148597759, 4294967295, 5439487, 5505107, 4294934613, 
    2148663295, 5734486, 5701719, 4294901847, 4294967295, 4294967295, 4294901848, 
    5898239, 4294967295, 4294967295, 5898330, 4294967295, 4294934619, 6094847, 
    4294967295, 6160383, 6226014, 4294967295, 4294901856, 2148597759, 4294967295, 
    4294967295, 4128831, 4294934565, 2150039551, 2588692, 2150924340, 4294901799, 
    4294967295, 4294967295, 4294934625, 6488063, 4294967295, 6553599, 6619236, 
    4294967295, 4294901862, 2148597759, 4294967295, 4294967295, 5177423, 
    4294967295, 2949119, 6750267, 6750311, 4294901863, 4294967295, 4294967295, 
    2949119, 2949146, 2150924340, 4294901805, 4294967295, 4294967295, 4294967295, 
    4294901793, 4294967295, 4294967295, 4294967295, 4294967295, 4294901864, 
    6946815, 4294967295, 4294967295, 6946922, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901867, 4294967295, 7143423, 2150891583, 
    2150924340, 4294934580, 4294967295, 4294967295, 4294967295, 4294901796, 
    4294967295, 4294967295, 4294967295, 4294934565, 2150039551, 7176278, 
    7143533, 4294901869, 4294967295, 4294967295, 4294934638, 7340031, 4294967295, 
    4294967295, 7340144, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901873, 4294967295, 2150039551, 7471183, 7471218, 4294901874, 
    4294967295, 4294967295, 4294901875, 7667711, 4294967295, 4294967295, 
    7667829, 4294967295, 4294934646, 7864319, 4294967295, 7929855, 7995513, 
    4294967295, 4294901883, 2148597759, 4294967295, 4294967295, 5898330, 
    4294967295, 1572863, 3932219, 2150924340, 4294901820, 4294967295, 4294967295, 
    4294934652, 2148597759, 4294967295, 4294967295, 6226015, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901885, 4294967295, 
    8323071, 2150891583, 2150924340, 4294934580, 4294967295, 4294967295, 
    4294934655, 2148597759, 4294967295, 8454143, 8519809, 4294967295, 4294901891, 
    2148597759, 4294967295, 4294967295, 8650884, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901893, 4294967295, 2150039551, 
    8781903, 8781958, 4294901894, 4294967295, 4294967295, 4294934663, 2148597759, 
    4294967295, 8978431, 9044105, 4294967295, 4294901899, 9240575, 4294967295, 
    4294967295, 9240717, 4294967295, 4294934670, 9437183, 4294967295, 9502719, 
    9568401, 4294967295, 4294901907, 2148597759, 4294967295, 4294967295, 
    6946922, 4294967295, 4784127, 7143541, 7143533, 4294901869, 4294967295, 
    4294967295, 4294901908, 4390911, 4294967295, 4294967295, 9765013, 4294967295, 
    4294901910, 9961471, 4294967295, 4294967295, 9961624, 4294967295, 4294934681, 
    2148597759, 4294967295, 4294967295, 7340144, 4294967295, 2150039551, 
    5242959, 2150924340, 4294901840, 4294967295, 4294967295, 4294934682, 
    2148597759, 4294967295, 4294967295, 10158235, 4294967295, 4294934684, 
    10354687, 4294967295, 10420223, 10485919, 4294967295, 4294901921, 2148597759, 
    4294967295, 4294967295, 7667829, 4294934613, 2148663295, 5734486, 2150924340, 
    4294901847, 4294967295, 4294967295, 4294934690, 2148597759, 4294967295, 
    4294967295, 7995514, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901923, 4294967295, 2150039551, 2150891672, 2150924340, 
    4294934580, 4294967295, 4294967295, 4294934692, 2148597759, 4294967295, 
    10878975, 10944678, 4294967295, 4294967295, 4294901823, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901928, 11141119, 4294967295, 
    4294967295, 11141290, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901931, 4294967295, 11337727, 2150891583, 2150924340, 
    4294934580, 4294967295, 4294967295, 4294934701, 11468799, 4294967295, 
    11534335, 11600048, 4294934565, 2150039551, 11665487, 11665586, 4294901938, 
    4294967295, 4294967295, 4294967295, 4294901827, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294934707, 11862015, 4294967295, 4294967295, 
    11862197, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901942, 4294967295, 12058623, 12058703, 12058808, 4294901944, 4294967295, 
    4294967295, 4294934713, 12255231, 4294967295, 12320767, 12386492, 4294967295, 
    4294901950, 2148597759, 4294967295, 4294967295, 9961624, 4294967295, 
    2949119, 6750267, 2150924340, 4294901863, 4294967295, 4294967295, 4294934719, 
    2148597759, 4294967295, 4294967295, 9568402, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901952, 4294967295, 2150039551, 
    11665487, 11665586, 4294901938, 4294967295, 4294967295, 4294934721, 
    2148597759, 4294967295, 12779519, 12845251, 4294967295, 4294934725, 
    6488063, 4294967295, 13041663, 13107399, 4294967295, 7143423, 7143541, 
    7143533, 4294901869, 4294967295, 4294967295, 4294934729, 13303807, 4294967295, 
    13369343, 13435084, 4294967295, 4294901966, 2148597759, 4294967295, 
    4294967295, 9961624, 4294934565, 2150039551, 7176278, 2150924340, 4294901869, 
    4294967295, 4294967295, 2150039551, 7471183, 2150924340, 4294901874, 
    4294967295, 4294967295, 4294934735, 2148597759, 4294967295, 4294967295, 
    10485920, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901968, 4294967295, 13762559, 2150924374, 2150924340, 4294934580, 
    4294967295, 4294967295, 4294934738, 2148597759, 4294967295, 13893631, 
    13959380, 4294967295, 4294967295, 4294901850, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901974, 4294967295, 4294934743, 14221311, 4294967295, 14286847, 
    14352602, 4294967295, 4294901980, 2148597759, 4294967295, 4294967295, 
    11141290, 4294967295, 8323071, 7176278, 7143533, 4294901869, 4294967295, 
    4294967295, 4294901981, 4390911, 4294967295, 4294967295, 14549214, 4294967295, 
    4294901983, 2148597759, 4294967295, 4294967295, 14680288, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901985, 4294967295, 
    12058623, 14843990, 14811362, 4294901986, 4294967295, 4294967295, 4294901987, 
    15007743, 4294967295, 4294967295, 15007973, 4294967295, 4294901990, 
    2148597759, 4294967295, 4294967295, 15007973, 4294967295, 2150039551, 
    8781903, 2150924340, 4294901894, 4294967295, 4294967295, 4294967295, 
    4294901839, 4294967295, 4294967295, 4294967295, 4294967295, 4294901991, 
    4390911, 4294967295, 4294967295, 15204584, 4294967295, 4294934761, 2148597759, 
    4294967295, 4294967295, 15335658, 4294967295, 4294901995, 2148597759, 
    4294967295, 4294967295, 15466732, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901997, 4294967295, 4294934766, 2148597759, 
    4294967295, 15728639, 15794416, 4294967295, 4294967295, 4294901866, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902002, 4294967295, 2150039551, 15925327, 
    15925491, 4294902003, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902004, 4294967295, 4294934773, 2148597759, 
    4294967295, 4294967295, 13435085, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902006, 4294967295, 8323071, 2150891672, 
    2150924340, 4294934580, 4294967295, 4294967295, 4294934775, 2148597759, 
    4294967295, 16318463, 16384249, 4294967295, 4294967295, 4294901877, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294902011, 16580607, 
    4294967295, 4294967295, 16580861, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902014, 4294967295, 16777215, 2150924374, 
    2150924340, 4294934580, 4294967295, 4294967295, 4294934784, 2148597759, 
    4294967295, 4294967295, 14352603, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902017, 4294967295, 2150039551, 11665578, 
    11665586, 4294901938, 4294967295, 4294967295, 4294934786, 2148597759, 
    4294967295, 17039359, 17105156, 4294967295, 4294934790, 6488063, 4294967295, 
    17301503, 17367304, 4294967295, 11337727, 7176278, 7143533, 4294901869, 
    4294967295, 4294967295, 4294934794, 11468799, 4294967295, 17563647, 
    11600140, 4294934565, 2150039551, 17662038, 17629453, 4294902029, 4294967295, 
    4294967295, 4294967295, 4294901892, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294934798, 17825791, 4294967295, 4294967295, 17826064, 
    4294967295, 4294934801, 18022399, 4294967295, 18087935, 18153748, 4294967295, 
    4294902038, 2148597759, 4294967295, 4294967295, 15007973, 4294934565, 
    2150039551, 11665487, 2150924340, 4294901938, 4294967295, 4294967295, 
    4294934807, 11468799, 4294967295, 18415615, 11600153, 4294967295, 4294934810, 
    6488063, 4294967295, 18612223, 18678044, 4294967295, 12058623, 18776150, 
    18743582, 4294902046, 4294967295, 4294967295, 12058623, 12058703, 2150924340, 
    4294901944, 4294967295, 4294967295, 4294934815, 11468799, 4294967295, 
    18939903, 11600161, 4294934565, 2150039551, 2150891672, 2150924340, 
    4294934580, 4294967295, 4294967295, 4294967295, 4294901901, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294902050, 4294967295, 12058623, 19071128, 19071267, 4294902051, 
    4294967295, 4294967295, 4294902052, 19267583, 4294967295, 4294967295, 
    19267878, 4294967295, 4294967295, 4294901909, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901912, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902055, 4294967295, 11337727, 2150891672, 2150924340, 4294934580, 
    4294967295, 4294967295, 4294934824, 19529727, 4294967295, 19595263, 
    19661099, 4294967295, 4294902061, 2148597759, 4294967295, 4294967295, 
    16580861, 4294967295, 13762559, 2150891637, 2150924340, 4294934580, 
    4294967295, 4294967295, 4294902062, 2148597759, 4294967295, 4294967295, 
    19857711, 4294967295, 4294967295, 4294901930, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902064, 4294967295, 2150039551, 15925418, 15925491, 4294902003, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902065, 4294967295, 2150039551, 11665630, 11665586, 4294901938, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902066, 4294967295, 4294902067, 20250623, 4294967295, 4294967295, 
    20250933, 4294967295, 4294902070, 2148597759, 4294967295, 4294967295, 
    20250933, 4294967295, 12058623, 14843990, 2150924340, 4294901986, 4294967295, 
    4294967295, 4294934839, 2148597759, 4294967295, 4294967295, 18153749, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294902072, 
    4294967295, 8323071, 17662038, 17629453, 4294902029, 4294967295, 4294967295, 
    4294934841, 11468799, 4294967295, 20643839, 20709691, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902077, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902078, 4294967295, 2150039551, 
    20906216, 20906303, 4294902079, 4294967295, 4294967295, 4294902080, 
    9240575, 4294967295, 4294967295, 21037377, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902082, 4294967295, 4294902083, 21299199, 
    4294967295, 4294967295, 21299525, 4294967295, 4294934854, 18022399, 
    4294967295, 21495807, 18153800, 4294967295, 4294902089, 2148597759, 
    4294967295, 4294967295, 2153152598, 4294934565, 2150039551, 15925327, 
    2150924340, 4294902003, 4294967295, 4294967295, 4294934858, 2148597759, 
    4294967295, 4294967295, 19661100, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294902091, 4294967295, 2150039551, 2150891852, 
    2150924340, 4294934580, 4294967295, 4294967295, 4294934861, 2148597759, 
    4294967295, 21954559, 22020431, 4294967295, 4294934865, 2148597759, 
    4294967295, 22216703, 22282579, 4294967295, 16777215, 2150891637, 2150924340, 
    4294934580, 4294967295, 4294967295, 4294967295, 4294901982, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901984, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294934869, 18022399, 4294967295, 
    22478847, 18153815, 4294967295, 4294902104, 2148597759, 4294967295, 
    4294967295, 20250933, 4294934565, 2150039551, 17662038, 2150924340, 
    4294902029, 4294967295, 4294967295, 4294934873, 11468799, 4294967295, 
    22740991, 11600219, 4294967295, 4294967295, 4294901989, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294902108, 4294967295, 11337727, 17662038, 17629453, 4294902029, 
    4294967295, 4294967295, 4294967295, 4294901992, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294934877, 11862015, 4294967295, 4294967295, 
    22937950, 4294967295, 4294934879, 12255231, 4294967295, 23134207, 23200097, 
    4294967295, 12058623, 18776150, 2150924340, 4294902046, 4294967295, 
    4294967295, 4294967295, 4294901996, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294934883, 23396351, 4294967295, 23461887, 23527782, 4294967295, 
    4294902120, 2148597759, 4294967295, 4294967295, 2153152598, 4294934613, 
    12058623, 19103830, 2150924340, 4294902051, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294902121, 4294967295, 
    4294934890, 2148597759, 4294967295, 23855103, 23921004, 4294967295, 
    4294967295, 4294902013, 4294967295, 4294967295, 4294967295, 4294934565, 
    24051711, 2150924374, 2150924340, 4294934580, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294902127, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294902128, 4294967295, 
    2150039551, 2150891889, 2150924340, 4294934580, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294902130, 4294967295, 
    4294934899, 11468799, 4294967295, 24444927, 20709749, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902134, 4294967295, 2150039551, 
    20906216, 2150924340, 4294902079, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902135, 4294967295, 2150039551, 
    2150891841, 2150924340, 4294934580, 4294967295, 4294967295, 4294934904, 
    2148597759, 4294967295, 4294967295, 23527783, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902137, 4294967295, 24051711, 
    24805462, 24772986, 4294902138, 4294967295, 4294967295, 4294934907, 
    2148597759, 4294967295, 24969215, 25035133, 4294967295, 4294967295, 
    4294902054, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902143, 4294967295, 25231359, 
    25264214, 25231745, 4294902145, 4294967295, 4294967295, 4294902146, 
    25427967, 4294967295, 4294967295, 25428356, 4294967295, 4294967295, 
    4294902063, 4294967295, 4294967295, 4294967295, 4294934565, 25559039, 
    2150924374, 2150924340, 4294934580, 4294967295, 4294967295, 4294967295, 
    4294902069, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902150, 4294967295, 4294967295, 
    4294902081, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902085, 4294967295, 4294967295, 4294967295, 4294967295, 4294902151, 
    25755647, 4294967295, 4294967295, 25756041, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294902154, 4294967295, 25952255, 
    19103830, 19071267, 4294902051, 4294967295, 4294967295, 4294967295, 
    4294934614, 4294967295, 4294967295, 4294967295, 4294967295, 4294902156, 
    2148597759, 4294967295, 4294967295, 26018189, 4294967295, 4294902158, 
    19267583, 4294967295, 4294967295, 26149263, 4294967295, 4294934928, 
    2173829119, 4294967295, 2173894655, 2173993363, 4294967295, 4294934933, 
    2148597759, 4294967295, 4294967295, 25428356, 4294967295, 24051711, 
    2174124118, 2174124438, 4294934934, 4294967295, 4294967295, 4294934935, 
    2174287871, 4294967295, 4294967295, 2174321049, 4294967295, 4294934938, 
    23396351, 4294967295, 2174484479, 23560604, 4294967295, 4294934941, 
    2148597759, 4294967295, 4294967295, 25756041, 4294934613, 2150039551, 
    24805462, 2150924340, 4294902138, 4294967295, 4294967295, 4294934942, 
    2148597759, 4294967295, 4294967295, 2174714271, 4294967295, 4294934944, 
    2148597759, 4294967295, 2174877695, 2174976418, 4294967295, 25231359, 
    2175041622, 2175041956, 4294934948, 4294967295, 4294967295, 4294934949, 
    18022399, 4294967295, 2175205375, 18186663, 4294934565, 2175336447, 
    25264214, 2150924340, 4294902145, 4294967295, 32, 2, 3, 1, 4, 4, 100, 
    112, 0, 112, 120, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 
    0, 0, 2, 1, 4294901760, 65535, 2147647489, 2147581954, 43, 4294967295, 
    5, 1, 4294967295, 9, 100, 120, 5, 140, 152, 172, 0, 172, 0, 172, 0, 
    172, 0, 172, 0, 172, 0, 172, 0, 0, 2, 2, 2, 1, 2, 2, 2, 2, 0, 4294901760, 
    4294967295, 65535, 4294934529, 2147680255, 2147778563, 2147778564, 32772, 
    45, 4294967295, 6, 1, 4294967295, 9, 100, 124, 6, 148, 160, 180, 0, 
    180, 0, 180, 0, 180, 0, 180, 0, 180, 0, 180, 0, 0, 2, 2, 2, 2, 1, 2, 
    2, 2, 2, 2, 0, 4294901760, 4294967295, 4294967295, 4294934529, 2147680255, 
    2147778563, 2147844101, 32772, 28, 4294967295, 1, 0, 0, 0, 100, 104, 
    1, 108, 112, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 112, 0, 
    2, 2, 65535
];