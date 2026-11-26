// Generated from KeywordsParser.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","12");
pub const KeywordsParser_CLASS:i32=1; 
pub const KeywordsParser_LAMBDA:i32=2; 
pub const KeywordsParser_BOOM:i32=3; 
pub const KeywordsParser_ID:i32=4; 
pub const KeywordsParser_WS:i32=5; 
pub const KeywordsParser_END:i32=6;
pub const KeywordsParser_EOF:i32=EOF;
pub const RULE_from:usize = 0; 
pub const RULE_statement:usize = 1; 
pub const RULE_class:usize = 2; 
pub const RULE_lambda:usize = 3; 
pub const RULE_nested:usize = 4; 
pub const RULE_boom:usize = 5;
pub const ruleNames: [&'static str; 6] = [
    "from", "statement", "class", "lambda", "nested", "boom"
];

pub const _LITERAL_NAMES: [Option<&'static str>;7] = [
	None, Some("'class'"), Some("'lambda'"), Some("'boom'"), None, None, Some("'end'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, Some("CLASS"), Some("LAMBDA"), Some("BOOM"), Some("ID"), Some("WS"), 
	Some("END")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, KeywordsParserExt<'input, 'arena>, KeywordsParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn with_delegate(arena: &'arena Arena, input: Input, err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				KeywordsParserExt {
					_pd: Default::default(),
				}
			),
            err_handler,
        }
    }

    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
        Self::with_delegate(arena, input, unsafe { ErrorStrategyDelegate::new(strategy) })
    }

    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        Self::with_delegate(arena, input, ErrorStrategyDelegate::new_default())
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
        L: ParseTreeListener<'arena, KeywordsParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct KeywordsParserTreeWalker;
impl KeywordsParserTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, KeywordsParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, KeywordsParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <KeywordsParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum KeywordsParserNodeKind {
    Error = 0,
    Terminal = 1,
    FromContext = 2,
    StatementContext = 3,
    ClassContext = 4,
    LambdaContext = 5,
    NestedContext = 6,
    BoomContext = 7,
}
pub type KeywordsParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, KeywordsParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => KeywordsParser }
dbt_antlr_runtime::impl_node_kind! { KeywordsParserNodeKind {
; FromContext(), StatementContext(), ClassContext(), LambdaContext(), NestedContext(), BoomContext(), 
    }; 
}

pub struct KeywordsParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> KeywordsParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { KeywordsParserExt, KeywordsParserNodeKind, "KeywordsParser.g4" }

//------------------- from ----------------
pub type FromContextAll<'input, 'arena, Tok = CommonToken<'input>> = FromContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { KeywordsParserNodeKind, FromContext, FromContextExt, RULE_from }

pub trait FromContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    fn statement_all(&self) -> Vec<&'arena StatementContextAll<'input, 'arena, Tok>>;
    fn statement(&self, i: usize) -> Option<&'arena StatementContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> FromContextAttrs<'input, 'arena, Tok> for FromContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(KeywordsParser_EOF, 0)
    }
    fn statement_all(&self) -> Vec<&'arena StatementContextAll<'input, 'arena, Tok>> {
        self.children_of_type().collect()
    }
    fn statement(&self, i: usize) -> Option<&'arena StatementContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn from(&mut self,) -> Result<&'arena FromContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, FromContext<TF::Tok>, RULE_from, 0, |_parentctx| FromContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena FromContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(15);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while (((_la) & !0x3f) == 0 && ((1usize << _la) & 14) != 0) {
			{
			{
			recog.base.set_state(12);
			recog.statement()?;
			}
			}
			recog.base.set_state(17);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		recog.base.set_state(18);
		recog.base.match_token(KeywordsParser_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input, 'arena, Tok = CommonToken<'input>> = StatementContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { KeywordsParserNodeKind, StatementContext, StatementContextExt, RULE_statement }

pub trait StatementContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn class(&self) -> Option<&'arena ClassContextAll<'input, 'arena, Tok>>;
    fn lambda(&self) -> Option<&'arena LambdaContextAll<'input, 'arena, Tok>>;
    fn boom(&self) -> Option<&'arena BoomContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StatementContextAttrs<'input, 'arena, Tok> for StatementContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn class(&self) -> Option<&'arena ClassContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn lambda(&self) -> Option<&'arena LambdaContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn boom(&self) -> Option<&'arena BoomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn statement(&mut self,) -> Result<&'arena StatementContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StatementContext<TF::Tok>, RULE_statement, 2, |_parentctx| StatementContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StatementContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(23);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    KeywordsParser_CLASS  => {
		        /*------- Outer Most Alt 1 -------*/
		        {
		        recog.base.set_state(20);
		        recog.class()?;
		        }}
		    KeywordsParser_LAMBDA  => {
		        /*------- Outer Most Alt 2 -------*/
		        {
		        recog.base.set_state(21);
		        recog.lambda()?;
		        }}
		    KeywordsParser_BOOM  => {
		        /*------- Outer Most Alt 3 -------*/
		        {
		        recog.base.set_state(22);
		        recog.boom()?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- class ----------------
pub type ClassContextAll<'input, 'arena, Tok = CommonToken<'input>> = ClassContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { KeywordsParserNodeKind, ClassContext, ClassContextExt, RULE_class }

pub trait ClassContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token CLASS
    /// Returns `None` if there is no child corresponding to token CLASS
    fn CLASS(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ClassContextAttrs<'input, 'arena, Tok> for ClassContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token CLASS
    /// Returns `None` if there is no child corresponding to token CLASS
    fn CLASS(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(KeywordsParser_CLASS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(KeywordsParser_ID, 0)
    }
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn class(&mut self,) -> Result<&'arena ClassContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ClassContext<TF::Tok>, RULE_class, 4, |_parentctx| ClassContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ClassContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(25);
		recog.base.match_token(KeywordsParser_CLASS,&mut recog.err_handler)?;
		recog.base.set_state(26);
		recog.base.match_token(KeywordsParser_ID,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- lambda ----------------
pub type LambdaContextAll<'input, 'arena, Tok = CommonToken<'input>> = LambdaContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { KeywordsParserNodeKind, LambdaContext, LambdaContextExt, RULE_lambda }

pub trait LambdaContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LAMBDA
    /// Returns `None` if there is no child corresponding to token LAMBDA
    fn LAMBDA(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    fn nested(&self) -> Option<&'arena NestedContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LambdaContextAttrs<'input, 'arena, Tok> for LambdaContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LAMBDA
    /// Returns `None` if there is no child corresponding to token LAMBDA
    fn LAMBDA(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(KeywordsParser_LAMBDA, 0)
    }
    fn nested(&self) -> Option<&'arena NestedContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lambda(&mut self,) -> Result<&'arena LambdaContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, LambdaContext<TF::Tok>, RULE_lambda, 6, |_parentctx| LambdaContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena LambdaContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(28);
		recog.base.match_token(KeywordsParser_LAMBDA,&mut recog.err_handler)?;
		recog.base.set_state(29);
		recog.nested(2)?;
		}
		})
	}
}
//------------------- nested ----------------
pub type NestedContextAll<'input, 'arena, Tok = CommonToken<'input>> = NestedContext<'input, 'arena, Tok>;

pub type NestedContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NestedContextExt<'input, 'arena, Tok>, KeywordsParserNodeKind, Tok>;
#[derive(Debug)]
pub struct NestedContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
	pub depth: i32,
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

dbt_antlr_runtime::impl_ctx! { @plain KeywordsParserNodeKind::NestedContext, NestedContextExt, RULE_nested }

impl<'input: 'arena, 'arena, Tok: Token + 'input> NestedContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena KeywordsParserNode<'input, 'arena, Tok>>, invoking_state: i32, depth: i32) -> Result<&'arena mut KeywordsParserNode<'input, 'arena, Tok>, ANTLRError>
    {
		let mut _init_depth = 0;

		 _init_depth = depth;

        BaseParserRuleContext::create(arena, parent, invoking_state, NestedContextExt {
				depth: _init_depth, 
				ph: PhantomData
			}
		)
	}
}
pub trait NestedContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn get_depth(&self) -> &i32; 
    fn set_depth(&mut self,attr: i32); 
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NestedContextAttrs<'input, 'arena, Tok> for NestedContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn get_depth(&self) -> &i32 { &self.deref().depth }  
    fn set_depth(&mut self,attr: i32) { self.deref_mut().depth = attr; }  
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(KeywordsParser_ID, 0)
    }
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn nested(&mut self,depth: i32) -> Result<&'arena NestedContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, NestedContext<TF::Tok>, RULE_nested, 8, |_parentctx| NestedContextExt::create(recog.get_arena(), _parentctx, recog.get_state(), depth); {
        let _local_ctx_fn = |recog: &Self| -> &'arena NestedContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(31);
		recog.base.match_token(KeywordsParser_ID,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- boom ----------------
pub type BoomContextAll<'input, 'arena, Tok = CommonToken<'input>> = BoomContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { KeywordsParserNodeKind, BoomContext, BoomContextExt, RULE_boom }

pub trait BoomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token BOOM
    /// Returns `None` if there is no child corresponding to token BOOM
    fn BOOM(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> BoomContextAttrs<'input, 'arena, Tok> for BoomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token BOOM
    /// Returns `None` if there is no child corresponding to token BOOM
    fn BOOM(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(KeywordsParser_BOOM, 0)
    }
}

impl<'input, 'arena, Input, TF> KeywordsParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn boom(&mut self,) -> Result<&'arena BoomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, BoomContext<TF::Tok>, RULE_boom, 10, |_parentctx| BoomContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena BoomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(33);
		recog.base.match_token(KeywordsParser_BOOM,&mut recog.err_handler)?;
		 panic!("boom"); 
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
static _serializedATN: [u32; 467] = [
    1346458702, 3, 16909060, 29, 6, 37, 33, 0, 0, 2, 6, 29, 259, 288, 165, 
    453, 0, 453, 0, 453, 2, 455, 6, 461, 6, 467, 0, 453, 0, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 4294967295, 
    2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 4294967295, 
    2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 4294967295, 
    2, 5, 8, 5, 1, 4294967295, 4294967295, 7, 5, 16, 6, 0, 4294967295, 4294967295, 
    1, 0, 8, 6, 1, 4294967295, 4294967295, 5, 0, 8, 7, 1, 14, 4294967295, 
    8, 0, 8, 8, 1, 4294967295, 4294967295, 10, 0, 8, 9, 2, 4294967295, 4294967295, 
    12, 0, 8, 11, 1, 4294967295, 17, 9, 0, 8, 12, 1, 4294967295, 4294967295, 
    1, 0, 32, 13, 1, 4294967295, 4294967295, 1, 0, 8, 14, 1, 4294967295, 
    4294967295, 1, 1, 8, 15, 1, 4294967295, 4294967295, 1, 1, 8, 16, 1, 
    4294967295, 4294967295, 1, 1, 8, 17, 1, 4294967295, 4294967295, 3, 1, 
    8, 18, 3, 24, 4294967295, 8, 1, 8, 21, 1, 4294967295, 4294967295, 1, 
    2, 32, 22, 1, 4294967295, 4294967295, 1, 2, 32, 23, 1, 4294967295, 4294967295, 
    1, 2, 8, 24, 1, 4294967295, 4294967295, 1, 3, 32, 25, 1, 4294967295, 
    4294967295, 1, 3, 8, 26, 1, 4294967295, 4294967295, 1, 3, 8, 27, 1, 
    4294967295, 4294967295, 1, 4, 32, 28, 1, 4294967295, 4294967295, 1, 
    4, 8, 29, 1, 4294967295, 4294967295, 1, 5, 32, 30, 1, 4294967295, 4294967295, 
    1, 5, 72, 31, 1, 4294967295, 4294967295, 1, 5, 8, 32, 1, 4294967295, 
    4294967295, 1, 5, 0, 33, 0, 4294967295, 4294967295, 1, 15, 0, 0, 0, 
    1, 23, 0, 0, 0, 1, 25, 0, 0, 0, 1, 28, 0, 0, 0, 1, 31, 0, 0, 0, 1, 33, 
    0, 0, 0, 3, 2, 1, 14, 0, 1, 12, 0, 0, 0, 1, 17, 0, 0, 0, 1, 13, 0, 0, 
    0, 1, 16, 0, 0, 0, 1, 18, 0, 0, 0, 1, 15, 0, 0, 0, 5, 19, 4294967295, 
    0, 0, 1, 1, 0, 0, 0, 3, 4, 2, 24, 0, 3, 6, 3, 24, 0, 3, 10, 5, 24, 0, 
    1, 20, 0, 0, 0, 1, 21, 0, 0, 0, 1, 22, 0, 0, 0, 1, 3, 0, 0, 0, 5, 26, 
    1, 0, 0, 5, 27, 4, 0, 0, 1, 5, 0, 0, 0, 5, 29, 2, 0, 0, 3, 8, 4, 30, 
    0, 1, 7, 0, 0, 0, 5, 32, 4, 0, 0, 1, 9, 0, 0, 0, 5, 34, 3, 0, 0, 6, 
    35, 5, 4294967295, 0, 1, 11, 0, 0, 0, 15, 23, 0, 2, 4, 6, 8, 10, 1, 
    3, 5, 7, 9, 11
];