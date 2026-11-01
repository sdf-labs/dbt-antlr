// Generated from PostfixSibling.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const PostfixSibling_T__0:i32=1; 
pub const PostfixSibling_T__1:i32=2; 
pub const PostfixSibling_T__2:i32=3; 
pub const PostfixSibling_T__3:i32=4; 
pub const PostfixSibling_T__4:i32=5; 
pub const PostfixSibling_T__5:i32=6; 
pub const PostfixSibling_T__6:i32=7; 
pub const PostfixSibling_T__7:i32=8; 
pub const PostfixSibling_T__8:i32=9; 
pub const PostfixSibling_T__9:i32=10; 
pub const PostfixSibling_T__10:i32=11; 
pub const PostfixSibling_T__11:i32=12; 
pub const PostfixSibling_T__12:i32=13; 
pub const PostfixSibling_T__13:i32=14; 
pub const PostfixSibling_WS:i32=15;
pub const PostfixSibling_EOF:i32=EOF;
pub const RULE_start:usize = 0; 
pub const RULE_r:usize = 1; 
pub const RULE_y:usize = 2; 
pub const RULE_q:usize = 3; 
pub const RULE_z:usize = 4; 
pub const RULE_c:usize = 5; 
pub const RULE_w:usize = 6; 
pub const RULE_m:usize = 7; 
pub const RULE_v:usize = 8;
pub const ruleNames: [&'static str; 9] = [
    "start", "r", "y", "q", "z", "c", "w", "m", "v"
];

pub const _LITERAL_NAMES: [Option<&'static str>;15] = [
	None, Some("';'"), Some("'('"), Some("')'"), Some("'with'"), Some("'cont'"), 
	Some("'mid'"), Some("'A'"), Some("'-'"), Some("'B'"), Some("'['"), Some("']'"), 
	Some("'X'"), Some("'C'"), Some("'!'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;16]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, PostfixSiblingParserExt<'input, 'arena>, PostfixSiblingParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				PostfixSiblingParserExt {
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
        L: ParseTreeListener<'arena, PostfixSiblingParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct PostfixSiblingTreeWalker;
impl PostfixSiblingTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, PostfixSiblingParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, PostfixSiblingParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <PostfixSiblingParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum PostfixSiblingParserNodeKind {
    StartContext,
    RContext,
    YContext,
    QContext,
    ZContext,
    CContext,
    WContext,
    MContext,
    VContext,
    Terminal,
    Error,
}
pub type PostfixSiblingParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, PostfixSiblingParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => PostfixSiblingParser }
dbt_antlr_runtime::impl_node_kind! { PostfixSiblingParserNodeKind {
; StartContext(), RContext(), YContext(), QContext(), ZContext(), CContext(), WContext(), MContext(), VContext(), 
    }; 
}

pub struct PostfixSiblingParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> PostfixSiblingParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { PostfixSiblingParserExt, PostfixSiblingParserNodeKind, "PostfixSibling.g4" }

//------------------- start ----------------
pub type StartContextAll<'input, 'arena, Tok = CommonToken<'input>> = StartContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, StartContext, StartContextExt, RULE_start }

pub trait StartContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn r_all(&self) -> Vec<&'arena RContextAll<'input, 'arena, Tok>>;
    fn r(&self, i: usize) -> Option<&'arena RContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
    fn c(&self) -> Option<&'arena CContextAll<'input, 'arena, Tok>>;
    fn m(&self) -> Option<&'arena MContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StartContextAttrs<'input, 'arena, Tok> for StartContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn r_all(&self) -> Vec<&'arena RContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn r(&self, i: usize) -> Option<&'arena RContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == PostfixSibling_EOF)
    }
    fn q(&self) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn c(&self) -> Option<&'arena CContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn m(&self) -> Option<&'arena MContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn start(&mut self,) -> Result<&'arena StartContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StartContext<TF::Tok>, RULE_start, 0, |_parentctx| StartContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StartContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(44);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    PostfixSibling_T__6 |PostfixSibling_T__7  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(18);
		        recog.r()?;
		        recog.base.set_state(19);
		        recog.r()?;
		        recog.base.set_state(20);
		        recog.base.match_token(PostfixSibling_T__0,&mut recog.err_handler)?;
		        recog.base.set_state(21);
		        recog.base.match_token(PostfixSibling_EOF,&mut recog.err_handler)?;
		        }}
		    PostfixSibling_T__1  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(23);
		        recog.base.match_token(PostfixSibling_T__1,&mut recog.err_handler)?;
		        recog.base.set_state(27);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        while _la==PostfixSibling_T__6 || _la==PostfixSibling_T__7 {
		        	{
		        	{
		        	recog.base.set_state(24);
		        	recog.r()?;
		        	}
		        	}
		        	recog.base.set_state(29);
		        	recog.err_handler.sync(&mut recog.base)?;
		        	_la = recog.base.input.la(1);
		        }
		        recog.base.set_state(30);
		        recog.base.match_token(PostfixSibling_T__2,&mut recog.err_handler)?;
		        recog.base.set_state(31);
		        recog.base.match_token(PostfixSibling_EOF,&mut recog.err_handler)?;
		        }}
		    PostfixSibling_T__3  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(32);
		        recog.base.match_token(PostfixSibling_T__3,&mut recog.err_handler)?;
		        recog.base.set_state(33);
		        recog.q()?;
		        recog.base.set_state(34);
		        recog.base.match_token(PostfixSibling_EOF,&mut recog.err_handler)?;
		        }}
		    PostfixSibling_T__4  => {
		        /*------- Outer Most Alt 4 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
		        {
		        recog.base.set_state(36);
		        recog.base.match_token(PostfixSibling_T__4,&mut recog.err_handler)?;
		        recog.base.set_state(37);
		        recog.c()?;
		        recog.base.set_state(38);
		        recog.base.match_token(PostfixSibling_EOF,&mut recog.err_handler)?;
		        }}
		    PostfixSibling_T__5  => {
		        /*------- Outer Most Alt 5 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
		        {
		        recog.base.set_state(40);
		        recog.base.match_token(PostfixSibling_T__5,&mut recog.err_handler)?;
		        recog.base.set_state(41);
		        recog.m()?;
		        recog.base.set_state(42);
		        recog.base.match_token(PostfixSibling_EOF,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r ----------------
pub type RContextAll<'input, 'arena, Tok = CommonToken<'input>> = RContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, RContext, RContextExt, RULE_r }

pub trait RContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn y(&self) -> Option<&'arena YContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RContextAttrs<'input, 'arena, Tok> for RContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn y(&self) -> Option<&'arena YContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r(&mut self,) -> Result<&'arena RContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, RContext<TF::Tok>, RULE_r, 2, |_parentctx| RContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena RContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(52);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    PostfixSibling_T__6  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(46);
		        recog.base.match_token(PostfixSibling_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(48);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.get_interpreter().adaptive_predict(2,&mut recog.base)? {
		        	x if x == 1=>{
		        		{
		        		recog.base.set_state(47);
		        		recog.y()?;
		        		}
		        	}

		        	_ => {}
		        }
		        }}
		    PostfixSibling_T__7  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(50);
		        recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(51);
		        recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- y ----------------
pub type YContextAll<'input, 'arena, Tok = CommonToken<'input>> = YContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, YContext, YContextExt, RULE_y }

pub trait YContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> YContextAttrs<'input, 'arena, Tok> for YContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn y(&mut self,) -> Result<&'arena YContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, YContext<TF::Tok>, RULE_y, 4, |_parentctx| YContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena YContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(54);
		recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		recog.base.set_state(55);
		recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- q ----------------
pub type QContextAll<'input, 'arena, Tok = CommonToken<'input>> = QContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, QContext, QContextExt, RULE_q }

pub trait QContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn q_all(&self) -> Vec<&'arena QContextAll<'input, 'arena, Tok>>;
    fn q(&self, i: usize) -> Option<&'arena QContextAll<'input, 'arena, Tok>>;
    fn z(&self) -> Option<&'arena ZContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> QContextAttrs<'input, 'arena, Tok> for QContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn q_all(&self) -> Vec<&'arena QContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn q(&self, i: usize) -> Option<&'arena QContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    fn z(&self) -> Option<&'arena ZContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn q(&mut self,) -> Result<&'arena QContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, QContext<TF::Tok>, RULE_q, 6, |_parentctx| QContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena QContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(69);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    PostfixSibling_T__9  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(57);
		        recog.base.match_token(PostfixSibling_T__9,&mut recog.err_handler)?;
		        recog.base.set_state(58);
		        recog.q()?;
		        recog.base.set_state(59);
		        recog.q()?;
		        recog.base.set_state(60);
		        recog.base.match_token(PostfixSibling_T__10,&mut recog.err_handler)?;
		        recog.base.set_state(61);
		        recog.base.match_token(PostfixSibling_T__11,&mut recog.err_handler)?;
		        }}
		    PostfixSibling_T__6  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(63);
		        recog.base.match_token(PostfixSibling_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(65);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.get_interpreter().adaptive_predict(4,&mut recog.base)? {
		        	x if x == 1=>{
		        		{
		        		recog.base.set_state(64);
		        		recog.z()?;
		        		}
		        	}

		        	_ => {}
		        }
		        }}
		    PostfixSibling_T__7  => {
		        /*------- Outer Most Alt 3 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
		        {
		        recog.base.set_state(67);
		        recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(68);
		        recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- z ----------------
pub type ZContextAll<'input, 'arena, Tok = CommonToken<'input>> = ZContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, ZContext, ZContextExt, RULE_z }

pub trait ZContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> ZContextAttrs<'input, 'arena, Tok> for ZContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn z(&mut self,) -> Result<&'arena ZContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ZContext<TF::Tok>, RULE_z, 8, |_parentctx| ZContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ZContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(71);
		recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		recog.base.set_state(72);
		recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- c ----------------
pub type CContextAll<'input, 'arena, Tok = CommonToken<'input>> = CContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, CContext, CContextExt, RULE_c }

pub trait CContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn w(&self) -> Option<&'arena WContextAll<'input, 'arena, Tok>>;
    fn c(&self) -> Option<&'arena CContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> CContextAttrs<'input, 'arena, Tok> for CContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn w(&self) -> Option<&'arena WContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn c(&self) -> Option<&'arena CContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn c(&mut self,) -> Result<&'arena CContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, CContext<TF::Tok>, RULE_c, 10, |_parentctx| CContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena CContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(86);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    PostfixSibling_T__6  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(74);
		        recog.base.match_token(PostfixSibling_T__6,&mut recog.err_handler)?;
		        recog.base.set_state(76);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.get_interpreter().adaptive_predict(6,&mut recog.base)? {
		        	x if x == 1=>{
		        		{
		        		recog.base.set_state(75);
		        		recog.w()?;
		        		}
		        	}

		        	_ => {}
		        }
		        recog.base.set_state(82);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        if _la==PostfixSibling_T__6 || _la==PostfixSibling_T__7 {
		        	{
		        	recog.base.set_state(78);
		        	recog.c()?;
		        	recog.base.set_state(79);
		        	recog.base.match_token(PostfixSibling_T__12,&mut recog.err_handler)?;
		        	recog.base.set_state(80);
		        	recog.base.match_token(PostfixSibling_T__13,&mut recog.err_handler)?;
		        	}
		        }

		        }}
		    PostfixSibling_T__7  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(84);
		        recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		        recog.base.set_state(85);
		        recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- w ----------------
pub type WContextAll<'input, 'arena, Tok = CommonToken<'input>> = WContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, WContext, WContextExt, RULE_w }

pub trait WContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> WContextAttrs<'input, 'arena, Tok> for WContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn w(&mut self,) -> Result<&'arena WContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, WContext<TF::Tok>, RULE_w, 12, |_parentctx| WContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena WContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(88);
		recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		recog.base.set_state(89);
		recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- m ----------------
pub type MContextAll<'input, 'arena, Tok = CommonToken<'input>> = MContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, MContext, MContextExt, RULE_m }

pub trait MContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn v(&self) -> Option<&'arena VContextAll<'input, 'arena, Tok>>;
    fn m_all(&self) -> Vec<&'arena MContextAll<'input, 'arena, Tok>>;
    fn m(&self, i: usize) -> Option<&'arena MContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> MContextAttrs<'input, 'arena, Tok> for MContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn v(&self) -> Option<&'arena VContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn m_all(&self) -> Vec<&'arena MContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn m(&self, i: usize) -> Option<&'arena MContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn m(&mut self,) -> Result<&'arena MContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, MContext<TF::Tok>, RULE_m, 14, |_parentctx| MContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena MContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(91);
		recog.base.match_token(PostfixSibling_T__6,&mut recog.err_handler)?;
		recog.base.set_state(103);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(11,&mut recog.base)? {
			x if x == 1=>{
				{
				recog.base.set_state(92);
				recog.v()?;
				recog.base.set_state(101);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==PostfixSibling_T__6 {
					{
					recog.base.set_state(93);
					recog.m()?;
					recog.base.set_state(94);
					recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
					recog.base.set_state(95);
					recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
					recog.base.set_state(97);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PostfixSibling_T__6 {
						{
						recog.base.set_state(96);
						recog.m()?;
						}
					}

					recog.base.set_state(99);
					recog.base.match_token(PostfixSibling_T__13,&mut recog.err_handler)?;
					}
				}

				}
			}

			_ => {}
		}
		}
		})
	}
}
//------------------- v ----------------
pub type VContextAll<'input, 'arena, Tok = CommonToken<'input>> = VContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { PostfixSiblingParserNodeKind, VContext, VContextExt, RULE_v }

pub trait VContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
}

impl<'input, 'arena, Tok: Token + 'input> VContextAttrs<'input, 'arena, Tok> for VContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
}

impl<'input, 'arena, Input, TF> PostfixSiblingParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn v(&mut self,) -> Result<&'arena VContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, VContext<TF::Tok>, RULE_v, 16, |_parentctx| VContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena VContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(105);
		recog.base.match_token(PostfixSibling_T__7,&mut recog.err_handler)?;
		recog.base.set_state(106);
		recog.base.match_token(PostfixSibling_T__8,&mut recog.err_handler)?;
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
static _serializedATN: [u32; 1397] = [
    1346458702, 3, 16909060, 29, 15, 109, 115, 0, 0, 12, 9, 29, 763, 792, 
    575, 1367, 0, 1367, 0, 1367, 12, 1379, 9, 1388, 9, 1397, 0, 1367, 0, 
    2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 4294967295, 
    2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 4294967295, 
    2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 4294967295, 
    2, 5, 8, 5, 1, 4294967295, 4294967295, 7, 5, 16, 6, 0, 4294967295, 4294967295, 
    2, 6, 8, 6, 1, 4294967295, 4294967295, 7, 6, 16, 7, 0, 4294967295, 4294967295, 
    2, 7, 8, 7, 1, 4294967295, 4294967295, 7, 7, 16, 8, 0, 4294967295, 4294967295, 
    2, 8, 8, 8, 1, 4294967295, 4294967295, 7, 8, 16, 9, 0, 4294967295, 4294967295, 
    1, 0, 8, 9, 1, 4294967295, 4294967295, 1, 0, 8, 10, 1, 4294967295, 4294967295, 
    1, 0, 32, 11, 1, 4294967295, 4294967295, 1, 0, 32, 12, 1, 4294967295, 
    4294967295, 1, 0, 8, 13, 1, 4294967295, 4294967295, 1, 0, 32, 14, 1, 
    4294967295, 4294967295, 1, 0, 8, 15, 1, 4294967295, 4294967295, 5, 0, 
    8, 16, 1, 26, 4294967295, 8, 0, 8, 17, 1, 4294967295, 4294967295, 10, 
    0, 8, 18, 2, 4294967295, 4294967295, 12, 0, 8, 20, 1, 4294967295, 29, 
    9, 0, 8, 21, 1, 4294967295, 4294967295, 1, 0, 32, 22, 1, 4294967295, 
    4294967295, 1, 0, 32, 23, 1, 4294967295, 4294967295, 1, 0, 32, 24, 1, 
    4294967295, 4294967295, 1, 0, 8, 25, 1, 4294967295, 4294967295, 1, 0, 
    32, 26, 1, 4294967295, 4294967295, 1, 0, 8, 27, 1, 4294967295, 4294967295, 
    1, 0, 32, 28, 1, 4294967295, 4294967295, 1, 0, 8, 29, 1, 4294967295, 
    4294967295, 1, 0, 32, 30, 1, 4294967295, 4294967295, 1, 0, 8, 31, 1, 
    4294967295, 4294967295, 1, 0, 32, 32, 1, 4294967295, 4294967295, 1, 
    0, 8, 33, 1, 4294967295, 4294967295, 1, 0, 32, 34, 1, 4294967295, 4294967295, 
    1, 0, 8, 35, 1, 4294967295, 4294967295, 3, 0, 8, 36, 5, 45, 4294967295, 
    8, 0, 8, 41, 1, 4294967295, 4294967295, 1, 1, 32, 42, 1, 4294967295, 
    4294967295, 1, 1, 8, 43, 1, 4294967295, 4294967295, 3, 1, 8, 44, 2, 
    49, 4294967295, 8, 1, 8, 46, 1, 4294967295, 4294967295, 1, 1, 32, 47, 
    1, 4294967295, 4294967295, 1, 1, 32, 48, 1, 4294967295, 4294967295, 
    3, 1, 8, 49, 2, 53, 4294967295, 8, 1, 8, 51, 1, 4294967295, 4294967295, 
    1, 2, 32, 52, 1, 4294967295, 4294967295, 1, 2, 32, 53, 1, 4294967295, 
    4294967295, 1, 2, 8, 54, 1, 4294967295, 4294967295, 1, 3, 32, 55, 1, 
    4294967295, 4294967295, 1, 3, 8, 56, 1, 4294967295, 4294967295, 1, 3, 
    8, 57, 1, 4294967295, 4294967295, 1, 3, 32, 58, 1, 4294967295, 4294967295, 
    1, 3, 32, 59, 1, 4294967295, 4294967295, 1, 3, 8, 60, 1, 4294967295, 
    4294967295, 1, 3, 32, 61, 1, 4294967295, 4294967295, 1, 3, 8, 62, 1, 
    4294967295, 4294967295, 3, 3, 8, 63, 2, 66, 4294967295, 8, 3, 8, 65, 
    1, 4294967295, 4294967295, 1, 3, 32, 66, 1, 4294967295, 4294967295, 
    1, 3, 32, 67, 1, 4294967295, 4294967295, 3, 3, 8, 68, 3, 70, 4294967295, 
    8, 3, 8, 71, 1, 4294967295, 4294967295, 1, 4, 32, 72, 1, 4294967295, 
    4294967295, 1, 4, 32, 73, 1, 4294967295, 4294967295, 1, 4, 8, 74, 1, 
    4294967295, 4294967295, 1, 5, 32, 75, 1, 4294967295, 4294967295, 1, 
    5, 8, 76, 1, 4294967295, 4294967295, 3, 5, 8, 77, 2, 77, 4294967295, 
    8, 5, 8, 79, 1, 4294967295, 4294967295, 1, 5, 8, 80, 1, 4294967295, 
    4294967295, 1, 5, 32, 81, 1, 4294967295, 4294967295, 1, 5, 32, 82, 1, 
    4294967295, 4294967295, 1, 5, 8, 83, 1, 4294967295, 4294967295, 3, 5, 
    8, 84, 2, 83, 4294967295, 8, 5, 8, 86, 1, 4294967295, 4294967295, 1, 
    5, 32, 87, 1, 4294967295, 4294967295, 1, 5, 32, 88, 1, 4294967295, 4294967295, 
    3, 5, 8, 89, 2, 87, 4294967295, 8, 5, 8, 91, 1, 4294967295, 4294967295, 
    1, 6, 32, 92, 1, 4294967295, 4294967295, 1, 6, 32, 93, 1, 4294967295, 
    4294967295, 1, 6, 8, 94, 1, 4294967295, 4294967295, 1, 7, 32, 95, 1, 
    4294967295, 4294967295, 1, 7, 8, 96, 1, 4294967295, 4294967295, 1, 7, 
    8, 97, 1, 4294967295, 4294967295, 1, 7, 32, 98, 1, 4294967295, 4294967295, 
    1, 7, 32, 99, 1, 4294967295, 4294967295, 1, 7, 8, 100, 1, 4294967295, 
    4294967295, 3, 7, 8, 101, 2, 98, 4294967295, 8, 7, 8, 103, 1, 4294967295, 
    4294967295, 1, 7, 32, 104, 1, 4294967295, 4294967295, 1, 7, 8, 105, 
    1, 4294967295, 4294967295, 3, 7, 8, 106, 2, 102, 4294967295, 8, 7, 8, 
    108, 1, 4294967295, 4294967295, 3, 7, 8, 109, 2, 104, 4294967295, 8, 
    7, 8, 111, 1, 4294967295, 4294967295, 1, 8, 32, 112, 1, 4294967295, 
    4294967295, 1, 8, 32, 113, 1, 4294967295, 4294967295, 1, 8, 8, 114, 
    1, 4294967295, 4294967295, 1, 8, 0, 115, 0, 4294967295, 4294967295, 
    1, 44, 0, 0, 0, 1, 52, 0, 0, 0, 1, 54, 0, 0, 0, 1, 69, 0, 0, 0, 1, 71, 
    0, 0, 0, 1, 86, 0, 0, 0, 1, 88, 0, 0, 0, 1, 91, 0, 0, 0, 1, 105, 0, 
    0, 0, 3, 2, 1, 19, 0, 3, 2, 1, 20, 0, 5, 21, 1, 0, 0, 5, 22, 4294967295, 
    0, 0, 1, 45, 0, 0, 0, 5, 27, 2, 0, 0, 3, 2, 1, 26, 0, 1, 24, 0, 0, 0, 
    1, 29, 0, 0, 0, 1, 25, 0, 0, 0, 1, 28, 0, 0, 0, 1, 30, 0, 0, 0, 1, 27, 
    0, 0, 0, 5, 31, 3, 0, 0, 5, 45, 4294967295, 0, 0, 5, 33, 4, 0, 0, 3, 
    6, 3, 34, 0, 5, 35, 4294967295, 0, 0, 1, 45, 0, 0, 0, 5, 37, 5, 0, 0, 
    3, 10, 5, 38, 0, 5, 39, 4294967295, 0, 0, 1, 45, 0, 0, 0, 5, 41, 6, 
    0, 0, 3, 14, 7, 42, 0, 5, 43, 4294967295, 0, 0, 1, 45, 0, 0, 0, 1, 18, 
    0, 0, 0, 1, 23, 0, 0, 0, 1, 32, 0, 0, 0, 1, 36, 0, 0, 0, 1, 40, 0, 0, 
    0, 1, 1, 0, 0, 0, 5, 48, 7, 0, 0, 3, 4, 2, 49, 0, 1, 47, 0, 0, 0, 1, 
    49, 0, 0, 0, 1, 53, 0, 0, 0, 5, 51, 8, 0, 0, 5, 53, 9, 0, 0, 1, 46, 
    0, 0, 0, 1, 50, 0, 0, 0, 1, 3, 0, 0, 0, 5, 55, 8, 0, 0, 5, 56, 9, 0, 
    0, 1, 5, 0, 0, 0, 5, 58, 10, 0, 0, 3, 6, 3, 59, 0, 3, 6, 3, 60, 0, 5, 
    61, 11, 0, 0, 5, 62, 12, 0, 0, 1, 70, 0, 0, 0, 5, 65, 7, 0, 0, 3, 8, 
    4, 66, 0, 1, 64, 0, 0, 0, 1, 66, 0, 0, 0, 1, 70, 0, 0, 0, 5, 68, 8, 
    0, 0, 5, 70, 9, 0, 0, 1, 57, 0, 0, 0, 1, 63, 0, 0, 0, 1, 67, 0, 0, 0, 
    1, 7, 0, 0, 0, 5, 72, 8, 0, 0, 5, 73, 9, 0, 0, 1, 9, 0, 0, 0, 5, 76, 
    7, 0, 0, 3, 12, 6, 77, 0, 1, 75, 0, 0, 0, 1, 77, 0, 0, 0, 1, 82, 0, 
    0, 0, 3, 10, 5, 79, 0, 5, 80, 13, 0, 0, 5, 81, 14, 0, 0, 1, 83, 0, 0, 
    0, 1, 78, 0, 0, 0, 1, 83, 0, 0, 0, 1, 87, 0, 0, 0, 5, 85, 8, 0, 0, 5, 
    87, 9, 0, 0, 1, 74, 0, 0, 0, 1, 84, 0, 0, 0, 1, 11, 0, 0, 0, 5, 89, 
    8, 0, 0, 5, 90, 9, 0, 0, 1, 13, 0, 0, 0, 5, 103, 7, 0, 0, 3, 16, 8, 
    101, 0, 3, 14, 7, 94, 0, 5, 95, 8, 0, 0, 5, 97, 9, 0, 0, 3, 14, 7, 98, 
    0, 1, 96, 0, 0, 0, 1, 98, 0, 0, 0, 1, 99, 0, 0, 0, 5, 100, 14, 0, 0, 
    1, 102, 0, 0, 0, 1, 93, 0, 0, 0, 1, 102, 0, 0, 0, 1, 104, 0, 0, 0, 1, 
    92, 0, 0, 0, 1, 104, 0, 0, 0, 1, 15, 0, 0, 0, 5, 106, 8, 0, 0, 5, 107, 
    9, 0, 0, 1, 17, 0, 0, 0, 27, 44, 48, 52, 65, 69, 76, 82, 86, 97, 101, 
    103, 0, 2, 4, 6, 8, 10, 12, 14, 16, 1, 3, 5, 7, 9, 11, 13, 15, 17
];