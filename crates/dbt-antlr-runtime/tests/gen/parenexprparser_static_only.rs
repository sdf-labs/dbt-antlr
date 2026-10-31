// Generated from ParenExpr.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
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
					call_site: 13,
					prefix: 1,
					rule: RULE_e,
					invoke: e_rec(0),
					tails: { ParenExpr_T__1  => 3, ParenExpr_T__2  => 4, },
					guarded: { },
					default: 0,
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

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> = LazyLock::new(|| {
    PackedATNDeserializer::new()
        .deserialize(&_serializedATN)
        .expect("invalid packed parser ATN")
});
static _serializedATN: [u32; 4870] = [
    1346458702, 3, 16909060, 29, 11, 55, 61, 2, 3, 5, 3, 29, 385, 414, 305, 
    719, 10, 729, 6, 743, 5, 748, 3, 751, 3, 4870, 4, 735, 8, 2, 0, 8, 0, 
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
    1, 3, 5, 1396983361, 2, 4116, 5, 4, 1, 425, 12, 264, 2418, 0, 0, 6, 
    2, 1, 0, 0, 5, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 4, 0, 4, 65536, 0, 0, 0, 
    0, 0, 65536, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 0, 65536, 0, 0, 0, 0, 
    0, 65536, 0, 0, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 65536, 
    0, 0, 0, 65536, 0, 65536, 0, 0, 65536, 0, 0, 0, 65536, 0, 0, 0, 3, 0, 
    0, 0, 3, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 3, 
    0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 65536, 65536, 0, 3, 0, 0, 0, 0, 0, 65536, 0, 3, 0, 0, 0, 0, 0, 
    0, 0, 0, 3, 0, 0, 0, 0, 0, 65536, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 65536, 
    0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 
    3, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 
    0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 
    0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 
    3, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 3, 0, 
    0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 
    3, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
    0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 0, 0, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 4, 0, 
    4, 0, 0, 0, 4, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 3, 4, 0, 0, 0, 
    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 4, 4, 4, 0, 0, 
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 3, 3, 3, 0, 
    0, 0, 4, 4, 4, 4, 4, 4, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 
    0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 4, 4, 3, 4, 4, 4, 4, 3, 3, 0, 0, 
    3, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 3, 3, 
    3, 3, 3, 3, 0, 0, 0, 0, 0, 0, 3, 3, 3, 4, 3, 3, 3, 3, 3, 3, 3, 0, 3, 
    0, 3, 3, 3, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 3, 3, 3, 3, 0, 3, 0, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 0, 1, 2, 2, 2, 3, 4, 2, 5, 
    6, 2, 7, 8, 9, 10, 11, 2, 2, 2, 12, 2, 2, 13, 14, 15, 16, 17, 2, 18, 
    19, 19, 20, 21, 22, 23, 24, 25, 2, 2, 26, 2, 27, 28, 29, 30, 31, 2, 
    32, 33, 34, 34, 35, 2, 2, 36, 37, 38, 38, 39, 40, 41, 42, 43, 44, 2, 
    45, 46, 47, 2, 48, 2, 49, 50, 2, 51, 52, 52, 2, 53, 54, 55, 2, 56, 57, 
    57, 2, 2, 58, 59, 60, 61, 2, 62, 63, 64, 64, 65, 2, 66, 67, 68, 68, 
    69, 70, 71, 72, 73, 49, 74, 75, 2, 76, 77, 53, 78, 79, 80, 81, 2, 82, 
    83, 84, 84, 85, 2, 86, 87, 2, 88, 89, 89, 90, 91, 92, 93, 2, 94, 95, 
    95, 96, 97, 98, 2, 99, 100, 101, 101, 102, 103, 104, 105, 106, 107, 
    2, 2, 108, 2, 109, 110, 111, 111, 112, 2, 113, 2, 114, 84, 84, 115, 
    116, 117, 86, 118, 2, 119, 120, 121, 121, 122, 2, 123, 124, 125, 126, 
    127, 2, 128, 129, 84, 84, 130, 2, 131, 2, 132, 133, 133, 2, 134, 101, 
    101, 2, 135, 136, 137, 137, 138, 2, 139, 140, 2, 141, 142, 142, 113, 
    2, 143, 144, 145, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 
    155, 156, 157, 158, 2, 159, 160, 161, 162, 2, 163, 164, 164, 131, 165, 
    166, 2, 167, 2, 168, 169, 169, 170, 171, 172, 139, 173, 2, 174, 2, 175, 
    176, 176, 2, 177, 178, 178, 2, 179, 121, 180, 2, 181, 182, 2, 183, 184, 
    185, 185, 186, 2, 187, 121, 2, 188, 189, 189, 190, 2, 191, 121, 167, 
    192, 193, 194, 195, 167, 2, 196, 197, 198, 198, 199, 200, 201, 174, 
    202, 203, 204, 205, 206, 207, 2, 208, 2, 209, 210, 210, 208, 211, 212, 
    213, 214, 215, 216, 217, 218, 2, 219, 185, 220, 2, 221, 222, 2, 223, 
    198, 198, 2, 224, 225, 225, 2, 226, 185, 227, 2, 228, 121, 208, 2, 229, 
    2, 230, 231, 231, 2, 232, 233, 234, 234, 235, 236, 2, 237, 238, 238, 
    239, 221, 240, 241, 242, 2, 243, 210, 242, 244, 2, 245, 246, 2, 247, 
    248, 248, 249, 250, 251, 252, 253, 254, 255, 242, 256, 257, 258, 249, 
    259, 260, 261, 262, 263, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 12, 27, 27, 42, 51, 60, 69, 78, 81, 
    90, 99, 114, 123, 126, 135, 147, 156, 168, 171, 180, 195, 204, 219, 
    234, 243, 252, 261, 270, 285, 288, 297, 306, 315, 318, 327, 342, 351, 
    354, 363, 378, 390, 399, 414, 423, 441, 456, 465, 474, 489, 492, 501, 
    504, 513, 516, 528, 537, 540, 549, 558, 573, 582, 597, 606, 609, 618, 
    633, 642, 645, 654, 669, 678, 693, 702, 711, 720, 729, 738, 753, 762, 
    777, 786, 804, 813, 816, 825, 840, 843, 852, 855, 864, 879, 891, 894, 
    903, 906, 915, 930, 939, 954, 963, 966, 975, 990, 1005, 1014, 1029, 
    1038, 1056, 1071, 1080, 1083, 1092, 1107, 1110, 1113, 1128, 1137, 1146, 
    1155, 1164, 1167, 1176, 1185, 1194, 1209, 1212, 1221, 1230, 1239, 1242, 
    1257, 1260, 1263, 1272, 1275, 1284, 1287, 1296, 1311, 1314, 1323, 1326, 
    1335, 1344, 1347, 1356, 1371, 1386, 1395, 1410, 1422, 1425, 1434, 1449, 
    1458, 1476, 1491, 1506, 1515, 1530, 1545, 1557, 1560, 1563, 1572, 1581, 
    1584, 1587, 1590, 1599, 1614, 1623, 1632, 1641, 1644, 1647, 1656, 1659, 
    1668, 1671, 1680, 1689, 1704, 1713, 1716, 1725, 1740, 1743, 1746, 1755, 
    1764, 1767, 1776, 1791, 1800, 1818, 1827, 1830, 1839, 1854, 1869, 1878, 
    1881, 1884, 1899, 1908, 1926, 1941, 1944, 1947, 1956, 1959, 1968, 1983, 
    1998, 2001, 2016, 2025, 2043, 2046, 2061, 2064, 2076, 2079, 2082, 2091, 
    2094, 2109, 2112, 2127, 2130, 2139, 2148, 2151, 2160, 2175, 2178, 2181, 
    2190, 2199, 2202, 2214, 2217, 2220, 2223, 2226, 2235, 2238, 2247, 2250, 
    2259, 2268, 2283, 2292, 2301, 2310, 2325, 2334, 2352, 2361, 2376, 2385, 
    2400, 2418, 1, 1, 1, 4, 4, 2, 9, 9, 3, 10, 10, 4, 1, 1, 5, 4, 4, 6, 
    8, 8, 7, 9, 9, 8, 10, 10, 9, 1, 1, 10, 4, 4, 11, 8, 8, 12, 9, 9, 13, 
    10, 10, 14, 1, 1, 15, 4, 4, 16, 9, 10, 9, 2, 2, 17, 3, 3, 18, 4, 7, 
    19, 2, 2, 17, 3, 3, 20, 4, 7, 19, 1, 1, 21, 4, 4, 16, 9, 10, 14, 9, 
    9, 22, 2, 2, 23, 3, 3, 24, 4, 7, 25, 2, 2, 23, 3, 3, 26, 4, 7, 25, 1, 
    1, 27, 4, 4, 16, 8, 8, 28, 9, 9, 29, 10, 10, 30, 1, 1, 31, 4, 4, 32, 
    9, 10, 33, 3, 3, 8, 1, 1, 34, 4, 4, 35, 9, 10, 36, 4294967295, 4294967295, 
    37, 2, 2, 38, 3, 3, 18, 4, 7, 39, 1, 1, 40, 4, 4, 41, 9, 10, 42, 4294967295, 
    4294967295, 37, 2, 2, 38, 3, 3, 20, 4, 7, 39, 9, 9, 43, 2, 2, 44, 3, 
    3, 26, 4, 7, 45, 1, 1, 46, 4, 4, 47, 8, 8, 48, 9, 9, 49, 10, 10, 50, 
    1, 1, 51, 4, 4, 16, 9, 10, 33, 2, 2, 17, 3, 3, 20, 4, 4, 19, 5, 6, 52, 
    7, 7, 19, 1, 1, 53, 4, 4, 54, 8, 8, 55, 9, 9, 56, 10, 10, 57, 1, 1, 
    58, 4, 4, 16, 9, 10, 36, 2, 2, 23, 3, 3, 59, 4, 7, 60, 1, 1, 61, 4, 
    4, 62, 9, 10, 63, 1, 1, 64, 4, 4, 16, 9, 10, 42, 2, 2, 23, 3, 3, 26, 
    4, 4, 25, 5, 6, 52, 7, 7, 25, 3, 3, 9, 1, 1, 65, 4, 4, 66, 9, 10, 67, 
    1, 1, 68, 4, 4, 16, 9, 10, 69, 1, 1, 70, 4, 4, 16, 9, 10, 50, 9, 9, 
    71, 2, 2, 72, 3, 3, 63, 4, 7, 52, 1, 1, 73, 4, 4, 16, 8, 8, 74, 9, 9, 
    75, 10, 10, 76, 1, 1, 77, 4, 4, 16, 9, 10, 57, 9, 9, 78, 2, 2, 38, 3, 
    3, 79, 4, 7, 80, 1, 1, 81, 4, 4, 16, 8, 8, 82, 9, 9, 83, 10, 10, 84, 
    4294967295, 4294967295, 85, 2, 2, 17, 3, 3, 86, 4, 7, 87, 1, 1, 88, 
    4, 4, 89, 9, 10, 90, 1, 1, 91, 4, 4, 92, 8, 8, 93, 9, 9, 94, 10, 10, 
    95, 1, 1, 96, 4, 4, 16, 9, 10, 63, 4294967295, 4294967295, 37, 2, 2, 
    38, 3, 3, 20, 4, 4, 39, 5, 6, 52, 7, 7, 39, 1, 1, 97, 4, 4, 98, 8, 8, 
    99, 9, 9, 100, 10, 10, 101, 1, 1, 102, 4, 4, 16, 9, 10, 79, 2, 2, 44, 
    3, 3, 59, 4, 7, 103, 2, 2, 44, 3, 3, 26, 4, 4, 45, 5, 6, 52, 7, 7, 45, 
    3, 3, 33, 1, 1, 104, 4, 4, 105, 9, 10, 106, 9, 9, 107, 2, 2, 108, 3, 
    3, 63, 4, 7, 52, 3, 3, 36, 4294967295, 4294967295, 37, 2, 2, 38, 3, 
    3, 86, 4, 7, 109, 1, 1, 110, 4, 4, 111, 9, 10, 112, 9, 9, 113, 2, 2, 
    38, 3, 3, 79, 4, 7, 114, 1, 1, 115, 4, 4, 116, 9, 10, 117, 1, 1, 118, 
    4, 4, 119, 8, 8, 120, 9, 9, 121, 10, 10, 122, 1, 1, 123, 4, 4, 16, 9, 
    10, 90, 2, 2, 23, 3, 3, 59, 4, 4, 60, 5, 6, 52, 7, 7, 60, 1, 1, 124, 
    4, 4, 16, 9, 10, 95, 9, 9, 125, 2, 2, 126, 3, 3, 63, 4, 7, 52, 1, 1, 
    127, 4, 4, 16, 8, 8, 128, 9, 9, 129, 10, 10, 130, 1, 1, 131, 4, 4, 16, 
    9, 10, 132, 9, 9, 133, 2, 2, 38, 3, 3, 79, 4, 7, 134, 1, 1, 135, 4, 
    4, 16, 8, 8, 136, 9, 9, 137, 10, 10, 138, 1, 1, 139, 4, 4, 140, 9, 10, 
    141, 1, 1, 142, 4, 4, 143, 8, 8, 144, 9, 9, 145, 10, 10, 146, 1, 1, 
    147, 4, 4, 16, 9, 10, 106, 2, 2, 72, 3, 3, 117, 4, 7, 109, 1, 1, 148, 
    4, 4, 66, 9, 10, 149, 1, 1, 150, 4, 4, 151, 9, 10, 152, 1, 1, 153, 4, 
    4, 16, 9, 10, 112, 2, 2, 38, 3, 3, 79, 4, 4, 80, 5, 6, 52, 7, 7, 80, 
    1, 1, 154, 4, 4, 16, 9, 10, 155, 1, 1, 156, 4, 4, 157, 8, 8, 158, 9, 
    9, 159, 10, 10, 160, 1, 1, 161, 4, 4, 16, 9, 10, 117, 4294967295, 4294967295, 
    85, 2, 2, 17, 3, 3, 86, 4, 4, 87, 5, 6, 52, 7, 7, 87, 1, 1, 162, 4, 
    4, 16, 9, 10, 122, 9, 9, 163, 2, 2, 38, 3, 3, 152, 4, 7, 52, 1, 1, 164, 
    4, 4, 16, 8, 8, 165, 9, 9, 166, 10, 10, 167, 3, 3, 63, 1, 1, 168, 4, 
    4, 169, 9, 10, 170, 9, 9, 171, 2, 2, 172, 3, 3, 63, 4, 7, 52, 1, 1, 
    173, 4, 4, 174, 8, 8, 175, 9, 9, 176, 10, 10, 177, 4294967295, 4294967295, 
    37, 2, 2, 38, 3, 3, 79, 4, 7, 178, 3, 3, 67, 1, 1, 179, 4, 4, 180, 9, 
    10, 181, 9, 9, 182, 2, 2, 183, 3, 3, 79, 4, 7, 184, 1, 1, 185, 4, 4, 
    186, 8, 8, 187, 9, 9, 188, 10, 10, 189, 1, 1, 190, 4, 4, 16, 9, 10, 
    152, 2, 2, 44, 3, 3, 59, 4, 4, 103, 5, 6, 52, 7, 7, 103, 1, 1, 191, 
    4, 4, 16, 9, 10, 146, 9, 9, 192, 2, 2, 38, 3, 3, 79, 4, 7, 178, 1, 1, 
    193, 4, 4, 16, 8, 8, 194, 9, 9, 195, 10, 10, 196, 1, 1, 197, 4, 4, 98, 
    8, 8, 198, 9, 9, 199, 10, 10, 200, 2, 2, 108, 3, 3, 117, 4, 7, 109, 
    1, 1, 201, 4, 4, 202, 8, 8, 203, 9, 9, 204, 10, 10, 205, 1, 1, 206, 
    4, 4, 16, 9, 10, 152, 4294967295, 4294967295, 37, 2, 2, 38, 3, 3, 86, 
    4, 4, 109, 5, 6, 52, 7, 7, 109, 2, 2, 38, 3, 3, 79, 4, 4, 114, 5, 6, 
    52, 7, 7, 114, 1, 1, 207, 4, 4, 16, 9, 10, 160, 9, 9, 208, 2, 2, 209, 
    3, 3, 86, 4, 7, 52, 1, 1, 210, 4, 4, 16, 8, 8, 211, 9, 9, 212, 10, 10, 
    213, 3, 3, 90, 9, 9, 214, 1, 1, 215, 4, 4, 216, 8, 8, 217, 9, 9, 218, 
    10, 10, 219, 1, 1, 220, 4, 4, 16, 9, 10, 170, 2, 2, 126, 3, 3, 86, 4, 
    7, 109, 1, 1, 221, 4, 4, 66, 9, 10, 222, 1, 1, 223, 4, 4, 16, 9, 10, 
    224, 9, 9, 225, 2, 2, 183, 3, 3, 86, 4, 7, 226, 1, 1, 227, 4, 4, 228, 
    9, 10, 229, 1, 1, 230, 4, 4, 16, 9, 10, 229, 2, 2, 38, 3, 3, 79, 4, 
    4, 134, 5, 6, 52, 7, 7, 134, 3, 3, 79, 1, 1, 231, 4, 4, 66, 9, 10, 232, 
    1, 1, 233, 4, 4, 16, 9, 10, 234, 1, 1, 235, 4, 4, 16, 9, 10, 236, 9, 
    9, 237, 1, 1, 238, 4, 4, 16, 8, 8, 239, 9, 9, 240, 10, 10, 241, 3, 3, 
    106, 9, 9, 242, 2, 2, 38, 3, 3, 79, 4, 7, 243, 9, 9, 244, 1, 1, 245, 
    4, 4, 16, 9, 10, 205, 9, 9, 246, 2, 2, 126, 3, 3, 152, 4, 7, 52, 1, 
    1, 247, 4, 4, 16, 8, 8, 248, 9, 9, 249, 10, 10, 250, 3, 3, 117, 1, 1, 
    251, 4, 4, 252, 9, 10, 253, 9, 9, 254, 2, 2, 255, 3, 3, 86, 4, 7, 52, 
    1, 1, 256, 4, 4, 16, 9, 10, 219, 9, 9, 257, 2, 2, 38, 3, 3, 170, 4, 
    7, 178, 1, 1, 258, 4, 4, 16, 8, 8, 259, 9, 9, 260, 10, 10, 261, 1, 1, 
    262, 4, 4, 98, 8, 8, 263, 9, 9, 264, 10, 10, 265, 2, 2, 172, 3, 3, 86, 
    4, 7, 109, 1, 1, 266, 4, 4, 174, 8, 8, 267, 9, 9, 268, 10, 10, 177, 
    4294967295, 4294967295, 37, 2, 2, 38, 3, 3, 86, 4, 7, 269, 3, 3, 132, 
    1, 1, 270, 4, 4, 271, 9, 10, 272, 1, 1, 273, 4, 4, 274, 8, 8, 275, 9, 
    9, 276, 10, 10, 277, 1, 1, 278, 4, 4, 16, 9, 10, 229, 4294967295, 4294967295, 
    37, 2, 2, 38, 3, 3, 79, 4, 4, 178, 5, 6, 52, 7, 7, 178, 1, 1, 279, 4, 
    4, 174, 8, 8, 280, 9, 9, 281, 10, 10, 177, 1, 1, 282, 4, 4, 98, 8, 8, 
    283, 9, 9, 284, 10, 10, 285, 2, 2, 183, 3, 3, 86, 4, 7, 286, 2, 2, 183, 
    3, 3, 79, 4, 4, 184, 5, 6, 52, 7, 7, 184, 1, 1, 287, 4, 4, 174, 8, 8, 
    288, 9, 9, 289, 10, 10, 177, 4294967295, 4294967295, 37, 2, 2, 38, 3, 
    3, 152, 4, 7, 52, 3, 3, 141, 9, 9, 290, 2, 2, 183, 3, 3, 152, 4, 7, 
    291, 1, 1, 292, 4, 4, 293, 9, 10, 294, 3, 3, 149, 3, 3, 152, 9, 9, 295, 
    2, 2, 172, 3, 3, 152, 4, 7, 52, 1, 1, 296, 4, 4, 297, 8, 8, 298, 9, 
    9, 299, 10, 10, 300, 1, 1, 301, 4, 4, 16, 9, 10, 253, 2, 2, 209, 3, 
    3, 117, 4, 7, 52, 1, 1, 302, 4, 4, 16, 9, 10, 303, 3, 3, 170, 9, 9, 
    304, 2, 2, 38, 3, 3, 170, 4, 7, 243, 9, 9, 305, 2, 2, 38, 3, 3, 222, 
    4, 7, 178, 9, 9, 306, 1, 1, 307, 4, 4, 308, 9, 10, 309, 1, 1, 310, 4, 
    4, 16, 9, 10, 309, 2, 2, 183, 3, 3, 86, 4, 4, 226, 5, 6, 52, 7, 7, 226, 
    1, 1, 311, 4, 4, 16, 9, 10, 277, 9, 9, 312, 2, 2, 126, 3, 3, 86, 4, 
    7, 269, 1, 1, 313, 4, 4, 174, 8, 8, 314, 9, 9, 315, 10, 10, 316, 9, 
    9, 317, 9, 9, 318, 2, 2, 38, 3, 3, 232, 4, 7, 319, 1, 1, 320, 4, 4, 
    140, 9, 10, 321, 9, 9, 322, 1, 1, 323, 4, 4, 324, 9, 10, 325, 1, 1, 
    326, 4, 4, 274, 8, 8, 327, 9, 9, 328, 10, 10, 277, 1, 1, 329, 4, 4, 
    16, 9, 10, 86, 4294967295, 4294967295, 37, 2, 2, 38, 3, 3, 79, 4, 4, 
    243, 5, 6, 52, 7, 7, 243, 1, 1, 330, 4, 4, 16, 9, 10, 300, 9, 9, 331, 
    2, 2, 38, 3, 3, 332, 4, 7, 52, 1, 1, 333, 4, 4, 16, 8, 8, 334, 9, 9, 
    335, 10, 10, 336, 1, 1, 337, 4, 4, 16, 8, 8, 338, 9, 9, 339, 10, 10, 
    340, 2, 2, 255, 3, 3, 117, 4, 7, 52, 3, 3, 222, 3, 3, 224, 1, 1, 341, 
    4, 4, 274, 8, 8, 342, 9, 9, 343, 10, 10, 277, 1, 1, 344, 4, 4, 16, 9, 
    10, 309, 4294967295, 4294967295, 37, 2, 2, 38, 3, 3, 86, 4, 4, 269, 
    5, 6, 52, 7, 7, 269, 1, 1, 345, 4, 4, 174, 8, 8, 346, 9, 9, 347, 10, 
    10, 177, 3, 3, 229, 9, 9, 348, 2, 2, 172, 3, 3, 86, 4, 7, 269, 3, 3, 
    232, 1, 1, 349, 4, 4, 180, 9, 10, 350, 1, 1, 351, 4, 4, 186, 8, 8, 352, 
    9, 9, 353, 10, 10, 354, 2, 2, 183, 3, 3, 86, 4, 4, 286, 5, 6, 52, 7, 
    7, 286, 3, 3, 236, 1, 1, 355, 4, 4, 356, 8, 8, 357, 9, 9, 358, 10, 10, 
    359, 1, 1, 360, 4, 4, 16, 9, 10, 86, 4294967295, 4294967295, 85, 2, 
    2, 183, 3, 3, 86, 4, 4, 291, 5, 6, 52, 7, 7, 291, 9, 9, 361, 1, 1, 362, 
    4, 4, 16, 8, 8, 363, 9, 9, 364, 10, 10, 365, 3, 3, 253, 4294967295, 
    4294967295, 37, 2, 2, 366, 3, 3, 86, 4, 7, 52, 9, 9, 367, 9, 9, 368, 
    2, 2, 38, 3, 3, 369, 4, 7, 52, 9, 9, 370, 1, 1, 371, 4, 4, 174, 8, 8, 
    372, 9, 9, 373, 10, 10, 316, 9, 9, 374, 2, 2, 38, 3, 3, 232, 4, 4, 319, 
    5, 6, 52, 7, 7, 319, 9, 9, 375, 2, 2, 38, 3, 3, 321, 4, 7, 52, 1, 1, 
    376, 4, 4, 16, 9, 10, 359, 9, 9, 377, 2, 2, 366, 3, 3, 86, 4, 7, 378, 
    1, 1, 379, 4, 4, 16, 8, 8, 380, 9, 9, 381, 10, 10, 382, 3, 3, 294, 9, 
    9, 383, 2, 2, 384, 3, 3, 86, 4, 7, 385, 1, 1, 386, 4, 4, 387, 9, 10, 
    388, 3, 3, 303, 4294967295, 4294967295, 37, 2, 2, 389, 3, 3, 86, 4, 
    7, 52, 3, 3, 309, 9, 9, 390, 3, 3, 321, 3, 3, 325, 1, 1, 391, 4, 4, 
    392, 9, 10, 393, 9, 9, 394, 2, 2, 395, 3, 3, 86, 4, 7, 291, 3, 3, 86, 
    1, 1, 396, 4, 4, 16, 9, 10, 397, 1, 1, 398, 4, 4, 293, 9, 10, 399, 1, 
    1, 400, 4, 4, 401, 8, 8, 402, 9, 9, 403, 10, 10, 404, 1, 1, 405, 4, 
    4, 16, 9, 10, 388, 2, 2, 366, 3, 3, 86, 4, 7, 406, 1, 1, 407, 4, 4, 
    408, 9, 10, 409, 1, 1, 410, 4, 4, 356, 8, 8, 411, 9, 9, 412, 10, 10, 
    359, 1, 1, 413, 4, 4, 16, 9, 10, 393, 4294967295, 4294967295, 85, 2, 
    2, 38, 3, 3, 86, 4, 4, 378, 5, 6, 52, 7, 7, 378, 1, 1, 414, 4, 4, 16, 
    9, 10, 415, 1, 1, 416, 4, 4, 16, 8, 8, 417, 9, 9, 418, 10, 10, 419, 
    2, 2, 384, 3, 3, 86, 4, 7, 420, 1, 1, 421, 4, 4, 274, 8, 8, 422, 9, 
    9, 423, 10, 10, 277, 4294967295, 4294967295, 37, 2, 2, 424, 3, 3, 86, 
    4, 4, 385, 5, 6, 52, 7, 7, 385, 0, 0, 0, 0, 0, 2, 3, 9, 2, 9, 0, 2, 
    1, 0, 0, 0, 0, 1, 1, 0, 9, 9, 4, 4, 1, 5, 6, 2, 7, 7, 1, 0, 0, 0, 0, 
    0, 4294967295, 5, 9, 2, 12, 0, 2, 2, 2, 1, 2, 2, 2, 2, 0, 0, 1, 1, 1, 
    1, 0, 12, 12, 4294967295, 4294967295, 1, 2, 2, 2, 3, 3, 3, 4, 7, 4, 
    0, 0, 0, 0, 0, 4294967295, 6, 9, 2, 18, 0, 2, 2, 2, 2, 1, 2, 2, 2, 2, 
    2, 0, 0, 1, 1, 1, 1, 1, 0, 18, 18, 4294967295, 4294967295, 1, 2, 2, 
    2, 3, 3, 3, 4, 4, 4, 5, 6, 5, 7, 7, 4, 0, 0, 0, 0, 0, 4294967295, 1, 
    1, 1, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1, 3, 2, 1, 2, 2, 3, 4
];