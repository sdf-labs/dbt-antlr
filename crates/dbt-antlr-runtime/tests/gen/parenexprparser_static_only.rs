// Generated from ParenExpr.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","9");
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
static _STATIC_PARSER_TABLE_WORDS: [u32; 2261] = [
    1396983361, 4, 2261, 4, 5, 1, 9, 4294967295, 0, 1, 2147483648, 0, 2091, 
    2091, 32, 2123, 38, 2161, 38, 2199, 27, 0, 2, 2, 5, 4, 1, 2, 2, 3, 4, 
    2, 2, 3, 4, 2091, 1, 248, 241, 4294967295, 12, 100, 1092, 248, 2084, 
    2580, 8364, 0, 8364, 0, 8364, 0, 8364, 0, 8364, 0, 8364, 0, 8364, 0, 
    0, 0, 6, 2, 1, 0, 0, 5, 0, 3, 0, 0, 0, 0, 4, 0, 65536, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 
    0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 4, 4, 0, 3, 0, 0, 
    0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 4, 4, 4, 4, 3, 0, 0, 3, 0, 0, 0, 0, 
    0, 3, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 4, 3, 4, 3, 3, 0, 0, 3, 3, 0, 3, 
    0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 0, 3, 3, 3, 3, 0, 3, 0, 0, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 0, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 65536, 4294967295, 196607, 
    4294901763, 4294901764, 393221, 524295, 655359, 720895, 786443, 917517, 
    1048591, 1179665, 1310739, 1441813, 1572887, 1703961, 1835035, 1966109, 
    2097183, 2228257, 2359331, 2490405, 2621479, 2752553, 2883627, 3014701, 
    3145775, 3276849, 3407923, 3538997, 3670071, 3801145, 3932219, 4063293, 
    4194367, 4325441, 4456515, 4587589, 4718663, 4849737, 4980811, 5111885, 
    5242959, 5374033, 5505107, 5636181, 5767255, 5898329, 6029403, 6160477, 
    6291551, 6422625, 6553699, 6684773, 6815847, 6946921, 7077995, 7209069, 
    7340143, 7471217, 7602291, 7733365, 7864439, 7995513, 8126587, 8257661, 
    8388735, 8519809, 8650883, 8781957, 8913031, 9044105, 9175179, 9306253, 
    9437327, 9568401, 9699475, 9830549, 9961623, 10092697, 10223771, 10354845, 
    10485919, 10616993, 10748067, 10879141, 11010215, 11141289, 11272363, 
    11403437, 11534511, 11665585, 11796659, 11927733, 12058807, 12189881, 
    12320955, 12452029, 12583103, 12714177, 12845251, 12976325, 13107399, 
    13238473, 13369547, 13500621, 13631695, 13762769, 13893843, 14024917, 
    14155991, 14287065, 14418139, 14549213, 14680287, 14811361, 14942435, 
    15073509, 15204583, 15335657, 15466731, 15597805, 15728879, 4294967295, 
    4294901761, 2147680255, 4294967295, 4294967295, 2147778563, 4294967295, 
    4294901765, 458751, 4294967295, 2148007935, 524296, 4294967295, 4294934537, 
    720895, 4294967295, 786431, 786444, 4294967295, 4294901773, 2148139007, 
    4294967295, 4294967295, 524296, 4294967295, 2148139007, 1015822, 983055, 
    4294901775, 4294967295, 4294967295, 4294934544, 2148139007, 4294967295, 
    4294967295, 786444, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901777, 4294967295, 1245183, 1310739, 1310740, 4294901780, 
    4294967295, 4294967295, 4294934544, 2148139007, 4294967295, 786431, 
    1376277, 4294967295, 4294901782, 1572863, 4294967295, 4294967295, 1572888, 
    4294967295, 4294967295, 4294901768, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901785, 1769471, 4294967295, 4294967295, 1769499, 4294934542, 
    2148139007, 1867790, 1835036, 4294901788, 4294967295, 4294967295, 4294934544, 
    1966079, 4294967295, 4294967295, 1966110, 4294967295, 2097151, 2097171, 
    2097184, 4294901792, 4294967295, 4294967295, 4294934544, 2228223, 4294967295, 
    2293759, 2293795, 4294967295, 4294901796, 2148139007, 4294967295, 4294967295, 
    1572888, 4294967295, 2148139007, 1015822, 2148106249, 4294901775, 4294967295, 
    4294967295, 4294934544, 2490367, 4294967295, 2555903, 2555943, 4294967295, 
    4294901800, 2148139007, 4294967295, 4294967295, 1769499, 4294967295, 
    1245183, 2752553, 2752554, 4294901802, 4294967295, 4294967295, 4294901803, 
    2949119, 4294967295, 4294967295, 2949165, 4294967295, 4294934544, 2148139007, 
    4294967295, 4294967295, 1966110, 4294967295, 1245183, 1310739, 2148106249, 
    4294901780, 4294967295, 4294967295, 4294901806, 3145727, 4294967295, 
    4294967295, 3145776, 4294967295, 4294934544, 2148139007, 4294967295, 
    4294967295, 3211313, 4294967295, 4294934544, 2148139007, 4294967295, 
    4294967295, 2293795, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901810, 4294967295, 3407871, 2148073517, 2148106249, 
    4294934537, 4294967295, 4294967295, 4294934544, 2148139007, 4294967295, 
    2293759, 3407924, 4294967295, 4294934544, 2148139007, 4294967295, 4294967295, 
    2555943, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901813, 4294967295, 2148139007, 3604534, 3604535, 4294901815, 4294967295, 
    4294967295, 4294934544, 2148139007, 4294967295, 2555903, 3670072, 4294934537, 
    2148139007, 3768329, 3735609, 4294901817, 4294967295, 4294967295, 4294901818, 
    3932159, 4294967295, 4294967295, 3932220, 4294967295, 4294934537, 4063231, 
    4294967295, 4128767, 4128831, 4294967295, 4294901824, 2148139007, 4294967295, 
    4294967295, 2949165, 4294934542, 2148139007, 1867790, 2148106249, 4294901788, 
    4294967295, 4294967295, 4294934544, 4325375, 4294967295, 4390911, 4390979, 
    4294967295, 4294901828, 2148139007, 4294967295, 4294967295, 3538998, 
    4294967295, 2097151, 4522025, 4522053, 4294901829, 4294967295, 4294967295, 
    2097151, 2097171, 2148106249, 4294901792, 4294967295, 4294967295, 4294967295, 
    4294901784, 4294967295, 4294967295, 4294967295, 4294967295, 4294901830, 
    4718591, 4294967295, 4294967295, 4718664, 4294967295, 4849663, 2148073517, 
    2148106249, 4294934537, 4294967295, 4294967295, 4294967295, 4294901787, 
    4294967295, 4294967295, 4294967295, 4294934542, 2148139007, 4882441, 
    4849738, 4294901834, 4294967295, 4294967295, 4294934544, 4980735, 4294967295, 
    4294967295, 4980812, 4294967295, 2148139007, 5046326, 5046349, 4294901837, 
    4294967295, 4294967295, 4294901838, 5242879, 4294967295, 4294967295, 
    5242960, 4294967295, 4294934544, 5373951, 4294967295, 5439487, 5439571, 
    4294967295, 4294901844, 2148139007, 4294967295, 4294967295, 3932220, 
    4294967295, 1245183, 2752553, 2148106249, 4294901802, 4294967295, 4294967295, 
    4294934537, 2148139007, 4294967295, 4294967295, 4128831, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901845, 4294967295, 
    5701631, 2148073517, 2148106249, 4294934537, 4294967295, 4294967295, 
    4294934537, 2148139007, 4294967295, 4128767, 5701719, 4294967295, 4294901848, 
    2148139007, 4294967295, 4294967295, 5832793, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901850, 4294967295, 2148139007, 
    5963830, 5963867, 4294901851, 4294967295, 4294967295, 4294934537, 2148139007, 
    4294967295, 6094847, 6094941, 4294967295, 4294901854, 6291455, 4294967295, 
    4294967295, 6291552, 4294967295, 4294934544, 6422527, 4294967295, 6488063, 
    6488163, 4294967295, 4294901860, 2148139007, 4294967295, 4294967295, 
    4718664, 4294967295, 3407871, 4849744, 4849738, 4294901834, 4294967295, 
    4294967295, 4294901861, 3145727, 4294967295, 4294967295, 6684774, 4294967295, 
    4294901863, 6881279, 4294967295, 4294967295, 6881385, 4294967295, 4294934544, 
    2148139007, 4294967295, 4294967295, 4980812, 4294967295, 2148139007, 
    3604534, 2148106249, 4294901815, 4294967295, 4294967295, 4294934544, 
    2148139007, 4294967295, 4294967295, 6946922, 4294967295, 4294934537, 
    7077887, 4294967295, 7143423, 7143533, 4294967295, 4294901870, 2148139007, 
    4294967295, 4294967295, 5242960, 4294934537, 2148139007, 3768329, 2148106249, 
    4294901817, 4294967295, 4294967295, 4294934544, 2148139007, 4294967295, 
    4294967295, 5439571, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901871, 4294967295, 2148139007, 2148073577, 2148106249, 
    4294934537, 4294967295, 4294967295, 4294934544, 2148139007, 4294967295, 
    5439487, 5439571, 4294967295, 4294967295, 4294901805, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901872, 7471103, 4294967295, 4294967295, 
    7471218, 4294967295, 7602175, 2148073517, 2148106249, 4294934537, 4294967295, 
    4294967295, 4294934537, 7667711, 4294967295, 7733247, 7733366, 4294934542, 
    2148139007, 7798838, 7798903, 4294901879, 4294967295, 4294967295, 4294967295, 
    4294901808, 4294967295, 4294967295, 4294967295, 4294967295, 4294934544, 
    7929855, 4294967295, 4294967295, 7929977, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901882, 4294967295, 8126463, 8126518, 8126588, 
    4294901884, 4294967295, 4294967295, 4294934544, 8257535, 4294967295, 
    8323071, 5439571, 4294967295, 4294901887, 2148139007, 4294967295, 4294967295, 
    6881385, 4294967295, 2097151, 4522025, 2148106249, 4294901829, 4294967295, 
    4294967295, 4294934544, 2148139007, 4294967295, 4294967295, 6488163, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901888, 
    4294967295, 2148139007, 7798838, 7798903, 4294901879, 4294967295, 4294967295, 
    4294934544, 2148139007, 4294967295, 6488063, 8454273, 4294967295, 4294934544, 
    4325375, 4294967295, 8585215, 6488163, 4294967295, 4849663, 4849744, 
    4849738, 4294901834, 4294967295, 4294967295, 4294934537, 8650751, 4294967295, 
    8716287, 8716421, 4294967295, 4294901894, 2148139007, 4294967295, 4294967295, 
    6881385, 4294934542, 2148139007, 4882441, 2148106249, 4294901834, 4294967295, 
    4294967295, 2148139007, 5046326, 2148106249, 4294901837, 4294967295, 
    4294967295, 4294934537, 2148139007, 4294967295, 4294967295, 7143533, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901895, 
    4294967295, 8978431, 2148106249, 2148106249, 4294934537, 4294967295, 
    4294967295, 4294934537, 2148139007, 4294967295, 7143423, 8978569, 4294967295, 
    4294967295, 4294901820, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934537, 9109503, 4294967295, 9175039, 9175180, 4294967295, 4294901901, 
    2148139007, 4294967295, 4294967295, 7471218, 4294967295, 5701631, 4882441, 
    4849738, 4294901834, 4294967295, 4294967295, 4294901902, 3145727, 4294967295, 
    4294967295, 9371791, 4294967295, 4294901904, 2148139007, 4294967295, 
    4294967295, 9502865, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901906, 4294967295, 8126463, 9666569, 9633939, 4294901907, 
    4294967295, 4294967295, 4294901908, 9830399, 4294967295, 4294967295, 
    9830550, 4294967295, 4294901911, 2148139007, 4294967295, 4294967295, 
    9830550, 4294967295, 2148139007, 5963830, 2148106249, 4294901851, 4294967295, 
    4294967295, 4294967295, 4294901814, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901912, 3145727, 4294967295, 4294967295, 10027161, 4294967295, 
    4294934537, 2148139007, 4294967295, 4294967295, 10092698, 4294967295, 
    4294901915, 2148139007, 4294967295, 4294967295, 10223772, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901917, 4294967295, 
    4294934537, 2148139007, 4294967295, 8716287, 10354846, 4294967295, 4294967295, 
    4294901832, 4294967295, 4294967295, 4294967295, 4294967295, 2148139007, 
    10420278, 10420383, 4294901919, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901920, 4294967295, 4294934537, 
    2148139007, 4294967295, 4294967295, 8716421, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901921, 4294967295, 5701631, 
    2148073577, 2148106249, 4294934537, 4294967295, 4294967295, 4294934537, 
    2148139007, 4294967295, 8716287, 10616994, 4294967295, 4294967295, 4294901840, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901923, 10813439, 
    4294967295, 4294967295, 10813605, 4294967295, 10944511, 2148106249, 
    2148106249, 4294934537, 4294967295, 4294967295, 4294934537, 2148139007, 
    4294967295, 4294967295, 9175180, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901927, 4294967295, 2148139007, 7798898, 
    7798903, 4294901879, 4294967295, 4294967295, 4294934537, 2148139007, 
    4294967295, 9175039, 11010216, 4294967295, 4294934537, 4325375, 4294967295, 
    11141119, 11141290, 4294967295, 7602175, 4882441, 4849738, 4294901834, 
    4294967295, 4294967295, 4294934537, 7667711, 4294967295, 11272191, 7733366, 
    4294934542, 2148139007, 11304969, 11272364, 4294901932, 4294967295, 
    4294967295, 4294967295, 4294901849, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294934537, 11403263, 4294967295, 4294967295, 11403438, 
    4294967295, 4294934537, 11534335, 4294967295, 11599871, 11600049, 4294967295, 
    4294901938, 2148139007, 4294967295, 4294967295, 9830550, 4294934542, 
    2148139007, 7798838, 2148106249, 4294901879, 4294967295, 4294967295, 
    4294934537, 7667711, 4294967295, 11599871, 7733366, 4294967295, 4294934537, 
    4325375, 4294967295, 11796479, 11796660, 4294967295, 8126463, 11894793, 
    11862197, 4294901941, 4294967295, 4294967295, 8126463, 8126518, 2148106249, 
    4294901884, 4294967295, 4294967295, 4294934537, 7667711, 4294967295, 
    11993087, 7733366, 4294934542, 2148139007, 2148073577, 2148106249, 4294934537, 
    4294967295, 4294967295, 4294967295, 4294901856, 4294967295, 4294967295, 
    4294967295, 4294967295, 8126463, 11993193, 11993271, 4294901943, 4294967295, 
    4294967295, 4294901944, 12189695, 4294967295, 4294967295, 12189882, 
    4294967295, 4294967295, 4294901862, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901865, 4294967295, 4294967295, 4294967295, 
    4294967295, 7602175, 2148073577, 2148106249, 4294934537, 4294967295, 
    4294967295, 4294934537, 12320767, 4294967295, 12386303, 12386493, 4294967295, 
    4294901950, 2148139007, 4294967295, 4294967295, 10813605, 4294967295, 
    8978431, 2148073552, 2148106249, 4294934537, 4294967295, 4294967295, 
    4294901951, 2148139007, 4294967295, 4294967295, 12583104, 4294967295, 
    4294967295, 4294901874, 4294967295, 4294967295, 4294967295, 4294967295, 
    2148139007, 10420338, 10420383, 4294901919, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294901953, 4294967295, 
    2148139007, 7798927, 7798903, 4294901879, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901954, 4294967295, 4294901955, 
    12910591, 4294967295, 4294967295, 12910789, 4294967295, 4294901958, 
    2148139007, 4294967295, 4294967295, 12910789, 4294967295, 8126463, 9666569, 
    2148106249, 4294901907, 4294967295, 4294967295, 4294934537, 2148139007, 
    4294967295, 4294967295, 11600049, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294901959, 4294967295, 5701631, 11304969, 11272364, 
    4294901932, 4294967295, 4294967295, 4294934537, 7667711, 4294967295, 
    11599871, 13107400, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901961, 4294967295, 2148139007, 13238425, 13238474, 
    4294901962, 4294967295, 4294967295, 4294901963, 6291455, 4294967295, 
    4294967295, 13369548, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901965, 4294967295, 4294901966, 13631487, 4294967295, 
    4294967295, 13631696, 4294967295, 4294934537, 11534335, 4294967295, 
    13762559, 11600049, 4294967295, 4294901970, 2148139007, 4294967295, 
    4294967295, 2148106249, 4294934542, 2148139007, 10420278, 2148106249, 
    4294901919, 4294967295, 4294967295, 4294934537, 2148139007, 4294967295, 
    4294967295, 12386493, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901971, 4294967295, 2148139007, 2148073684, 2148106249, 
    4294934537, 4294967295, 4294967295, 4294934537, 2148139007, 4294967295, 
    12386303, 12386493, 4294967295, 4294934537, 2148139007, 4294967295, 
    14024703, 14024918, 4294967295, 10944511, 2148073552, 2148106249, 4294934537, 
    4294967295, 4294967295, 4294967295, 4294901903, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294901905, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294934537, 11534335, 4294967295, 14155775, 
    11600049, 4294967295, 4294901976, 2148139007, 4294967295, 4294967295, 
    12910789, 4294934542, 2148139007, 11304969, 2148106249, 4294901932, 
    4294967295, 4294967295, 4294934537, 7667711, 4294967295, 14155775, 7733366, 
    4294967295, 4294967295, 4294901910, 4294967295, 4294967295, 4294967295, 
    4294967295, 7602175, 11304969, 11272364, 4294901932, 4294967295, 4294967295, 
    4294967295, 4294901913, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294934537, 7929855, 4294967295, 4294967295, 14221529, 4294967295, 4294934537, 
    8257535, 4294967295, 14352383, 14352603, 4294967295, 8126463, 11894793, 
    2148106249, 4294901941, 4294967295, 4294967295, 4294967295, 4294901916, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294934537, 14483455, 
    4294967295, 14548991, 14549214, 4294967295, 4294901983, 2148139007, 
    4294967295, 4294967295, 2148106249, 4294934537, 8126463, 12025865, 2148106249, 
    4294901943, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901984, 4294967295, 4294934537, 2148139007, 4294967295, 
    14811135, 14811362, 4294967295, 4294967295, 4294901925, 4294967295, 
    4294967295, 4294967295, 4294934542, 14942207, 2148106249, 2148106249, 
    4294934537, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901988, 4294967295, 2148139007, 2148073701, 2148106249, 
    4294934537, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901990, 4294967295, 4294934537, 7667711, 4294967295, 
    14155775, 13107400, 4294967295, 2148139007, 13238425, 2148106249, 4294901962, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901991, 4294967295, 2148139007, 2148073676, 2148106249, 4294934537, 
    4294967295, 4294967295, 4294934537, 2148139007, 4294967295, 4294967295, 
    14549214, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294901992, 4294967295, 14942207, 15302665, 15270121, 4294901993, 4294967295, 
    4294967295, 4294934537, 2148139007, 4294967295, 14811135, 15335658, 
    4294967295, 4294967295, 4294901946, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294901995, 
    4294967295, 15532031, 15564809, 15532269, 4294901997, 4294967295, 4294967295, 
    4294901998, 15728639, 4294967295, 4294967295, 15728880, 4294967295, 
    4294967295, 4294901952, 4294967295, 4294967295, 4294967295, 4294934542, 
    15859711, 2148106249, 2148106249, 4294934537, 4294967295, 4294967295, 
    4294967295, 4294901957, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901964, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294967295, 4294901968, 4294967295, 4294967295, 4294967295, 4294967295, 
    4294902002, 15990783, 4294967295, 4294967295, 15991028, 4294967295, 
    15859711, 12025865, 11993271, 4294901943, 4294967295, 4294967295, 4294967295, 
    4294934537, 4294967295, 4294967295, 4294967295, 4294967295, 4294901998, 
    2148139007, 4294967295, 4294967295, 16056565, 4294967295, 4294902006, 
    12189695, 4294967295, 4294967295, 16187639, 4294967295, 4294934537, 
    2148139007, 4294967295, 2148139007, 2148106249, 4294967295, 4294934537, 
    2148139007, 4294967295, 4294967295, 15728880, 4294967295, 14942207, 
    2148106249, 2148106249, 4294934537, 4294967295, 4294967295, 4294934537, 
    2148139007, 4294967295, 4294967295, 2148106249, 4294967295, 4294934537, 
    14483455, 4294967295, 2148139007, 14581769, 4294967295, 4294934537, 
    2148139007, 4294967295, 4294967295, 15991028, 4294934537, 2148139007, 
    15302665, 2148106249, 4294901993, 4294967295, 4294967295, 15532031, 
    2148106249, 2148106249, 4294934537, 4294967295, 4294967295, 4294934537, 
    11534335, 4294967295, 2148139007, 11632649, 4294934542, 2148139007, 
    15564809, 2148106249, 4294901997, 4294967295, 32, 2, 3, 1, 4, 4, 100, 
    112, 0, 112, 120, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 0, 128, 
    0, 0, 2, 1, 4294901760, 65535, 2147647489, 2147581954, 38, 4294967295, 
    3, 1, 4294967295, 9, 100, 112, 3, 124, 132, 152, 0, 152, 0, 152, 0, 
    152, 0, 152, 0, 152, 0, 152, 0, 0, 2, 1, 2, 0, 0, 4294901760, 65535, 
    4294934529, 2147614719, 2147647489, 2147647490, 32770, 38, 4294967295, 
    3, 1, 4294967295, 9, 100, 112, 3, 124, 132, 152, 0, 152, 0, 152, 0, 
    152, 0, 152, 0, 152, 0, 152, 0, 0, 2, 1, 2, 0, 0, 4294901760, 65535, 
    4294934529, 2147614719, 2147581953, 2147647490, 32769, 27, 4294967295, 
    1, 0, 0, 0, 100, 104, 0, 104, 108, 108, 0, 108, 0, 108, 0, 108, 0, 108, 
    0, 108, 0, 108, 0, 2, 65535
];