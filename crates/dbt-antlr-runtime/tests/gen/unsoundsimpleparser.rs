// Generated from UnsoundSimple.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","11");
pub const UnsoundSimple_A:i32=1; 
pub const UnsoundSimple_B:i32=2; 
pub const UnsoundSimple_C:i32=3; 
pub const UnsoundSimple_D:i32=4; 
pub const UnsoundSimple_E:i32=5; 
pub const UnsoundSimple_M:i32=6; 
pub const UnsoundSimple_P:i32=7; 
pub const UnsoundSimple_Q:i32=8; 
pub const UnsoundSimple_X:i32=9; 
pub const UnsoundSimple_Y:i32=10; 
pub const UnsoundSimple_WS:i32=11;
pub const UnsoundSimple_EOF:i32=EOF;
pub const RULE_top:usize = 0; 
pub const RULE_a:usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "top", "a"
];

pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
	None, Some("'a'"), Some("'b'"), Some("'c'"), Some("'d'"), Some("'e'"), 
	Some("'m'"), Some("'p'"), Some("'q'"), Some("'x'"), Some("'y'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, Some("A"), Some("B"), Some("C"), Some("D"), Some("E"), Some("M"), 
	Some("P"), Some("Q"), Some("X"), Some("Y"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, UnsoundSimpleParserExt<'input, 'arena>, UnsoundSimpleParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct UnsoundSimpleParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> UnsoundSimpleParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn with_delegate(arena: &'arena Arena, input: Input, err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				UnsoundSimpleParserExt {
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
        L: ParseTreeListener<'arena, UnsoundSimpleParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct UnsoundSimpleTreeWalker;
impl UnsoundSimpleTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, UnsoundSimpleParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, UnsoundSimpleParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <UnsoundSimpleParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum UnsoundSimpleParserNodeKind {
    TopContext,
    AContext,
    Terminal,
    Error,
}
pub type UnsoundSimpleParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, UnsoundSimpleParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => UnsoundSimpleParser }
dbt_antlr_runtime::impl_node_kind! { UnsoundSimpleParserNodeKind {
; TopContext(), AContext(), 
    }; 
}

pub struct UnsoundSimpleParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> UnsoundSimpleParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { UnsoundSimpleParserExt, UnsoundSimpleParserNodeKind, "UnsoundSimple.g4" }

//------------------- top ----------------
pub type TopContextAll<'input, 'arena, Tok = CommonToken<'input>> = TopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { UnsoundSimpleParserNodeKind, TopContext, TopContextExt, RULE_top }

pub trait TopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token X
    /// Returns `None` if there is no child corresponding to token X
    fn X(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Y
    /// Returns `None` if there is no child corresponding to token Y
    fn Y(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TopContextAttrs<'input, 'arena, Tok> for TopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token X
    /// Returns `None` if there is no child corresponding to token X
    fn X(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_X, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Y
    /// Returns `None` if there is no child corresponding to token Y
    fn Y(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_Y, 0)
    }
}

impl<'input, 'arena, Input, TF> UnsoundSimpleParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn top(&mut self,) -> Result<&'arena TopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TopContext<TF::Tok>, RULE_top, 0, |_parentctx| TopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(10);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(0,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				{
				recog.base.set_state(4);
				recog.a()?;
				recog.base.set_state(5);
				recog.base.match_token(UnsoundSimple_X,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				{
				recog.base.set_state(7);
				recog.a()?;
				recog.base.set_state(8);
				recog.base.match_token(UnsoundSimple_Y,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- a ----------------
pub type AContextAll<'input, 'arena, Tok = CommonToken<'input>> = AContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { UnsoundSimpleParserNodeKind, AContext, AContextExt, RULE_a }

pub trait AContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token M
    /// Returns `None` if there is no child corresponding to token M
    fn M(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AContextAttrs<'input, 'arena, Tok> for AContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_D, 0)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_C, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_B, 0)
    }
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_P, 0)
    }
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_E, 0)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_Q, 0)
    }
    /// Retrieves first TerminalNode corresponding to token M
    /// Returns `None` if there is no child corresponding to token M
    fn M(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(UnsoundSimple_M, 0)
    }
}

impl<'input, 'arena, Input, TF> UnsoundSimpleParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn a(&mut self,) -> Result<&'arena AContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AContext<TF::Tok>, RULE_a, 2, |_parentctx| AContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(30);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    UnsoundSimple_A  => {
		        {
		        recog.base.set_state(12);
		        recog.base.match_token(UnsoundSimple_A,&mut recog.err_handler)?;
		        }}
		    UnsoundSimple_C  => {
		        {
		        recog.base.set_state(13);
		        recog.base.match_token(UnsoundSimple_C,&mut recog.err_handler)?;
		        recog.base.set_state(25);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.get_interpreter().adaptive_predict(1,&mut recog.base)? {
		        	1 =>{
		        		{
		        		recog.base.set_state(14);
		        		recog.base.match_token(UnsoundSimple_B,&mut recog.err_handler)?;
		        		recog.base.set_state(15);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==UnsoundSimple_P || _la==UnsoundSimple_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(16);
		        		recog.base.match_token(UnsoundSimple_A,&mut recog.err_handler)?;
		        		}
		        	}
		        ,
		        	2 =>{
		        		{
		        		recog.base.set_state(17);
		        		recog.base.match_token(UnsoundSimple_B,&mut recog.err_handler)?;
		        		recog.base.set_state(18);
		        		recog.base.match_token(UnsoundSimple_P,&mut recog.err_handler)?;
		        		recog.base.set_state(19);
		        		recog.base.match_token(UnsoundSimple_A,&mut recog.err_handler)?;
		        		}
		        	}
		        ,
		        	3 =>{
		        		{
		        		recog.base.set_state(20);
		        		recog.base.match_token(UnsoundSimple_B,&mut recog.err_handler)?;
		        		recog.base.set_state(21);
		        		recog.base.match_token(UnsoundSimple_P,&mut recog.err_handler)?;
		        		recog.base.set_state(22);
		        		recog.a()?;
		        		recog.base.set_state(23);
		        		recog.base.match_token(UnsoundSimple_E,&mut recog.err_handler)?;
		        		}
		        	}

		        	_ => {}
		        }
		        recog.base.set_state(28);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        if _la==UnsoundSimple_M {
		        	{
		        	recog.base.set_state(27);
		        	recog.base.match_token(UnsoundSimple_M,&mut recog.err_handler)?;
		        	}
		        }

		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		recog.base.set_state(32);
		recog.base.match_token(UnsoundSimple_D,&mut recog.err_handler)?;
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
static _serializedATN: [u32; 478] = [
    1346458702, 3, 16909060, 29, 11, 35, 37, 1, 1, 4, 2, 29, 245, 274, 185, 
    459, 5, 464, 2, 470, 4, 474, 2, 476, 2, 478, 2, 466, 4, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    1, 0, 8, 2, 1, 4294967295, 4294967295, 1, 0, 32, 3, 1, 4294967295, 4294967295, 
    1, 0, 8, 4, 1, 4294967295, 4294967295, 1, 0, 8, 5, 1, 4294967295, 4294967295, 
    1, 0, 32, 6, 1, 4294967295, 4294967295, 1, 0, 8, 7, 1, 4294967295, 4294967295, 
    3, 0, 8, 8, 2, 11, 4294967295, 8, 0, 8, 10, 1, 4294967295, 4294967295, 
    1, 1, 32, 11, 1, 4294967295, 4294967295, 1, 1, 32, 12, 1, 4294967295, 
    4294967295, 1, 1, 32, 13, 1, 4294967295, 4294967295, 1, 1, 32, 14, 1, 
    4294967295, 4294967295, 1, 1, 32, 15, 1, 4294967295, 4294967295, 1, 
    1, 32, 16, 1, 4294967295, 4294967295, 1, 1, 32, 17, 1, 4294967295, 4294967295, 
    1, 1, 32, 18, 1, 4294967295, 4294967295, 1, 1, 32, 19, 1, 4294967295, 
    4294967295, 1, 1, 32, 20, 1, 4294967295, 4294967295, 1, 1, 8, 21, 1, 
    4294967295, 4294967295, 1, 1, 32, 22, 1, 4294967295, 4294967295, 1, 
    1, 8, 23, 1, 4294967295, 4294967295, 3, 1, 8, 24, 3, 26, 4294967295, 
    8, 1, 8, 27, 1, 4294967295, 4294967295, 1, 1, 32, 28, 1, 4294967295, 
    4294967295, 3, 1, 8, 29, 2, 29, 4294967295, 8, 1, 8, 31, 1, 4294967295, 
    4294967295, 3, 1, 8, 32, 2, 31, 4294967295, 8, 1, 8, 34, 1, 4294967295, 
    4294967295, 1, 1, 32, 35, 1, 4294967295, 4294967295, 1, 1, 8, 36, 1, 
    4294967295, 4294967295, 1, 1, 0, 37, 0, 4294967295, 4294967295, 1, 10, 
    0, 0, 0, 1, 30, 0, 0, 0, 3, 2, 1, 5, 0, 5, 6, 9, 0, 0, 1, 11, 0, 0, 
    0, 3, 2, 1, 8, 0, 5, 9, 10, 0, 0, 1, 11, 0, 0, 0, 1, 4, 0, 0, 0, 1, 
    7, 0, 0, 0, 1, 1, 0, 0, 0, 5, 31, 1, 0, 0, 5, 25, 3, 0, 0, 5, 15, 2, 
    0, 0, 7, 16, 0, 0, 0, 5, 26, 1, 0, 0, 5, 18, 2, 0, 0, 5, 19, 7, 0, 0, 
    5, 26, 1, 0, 0, 5, 21, 2, 0, 0, 5, 22, 7, 0, 0, 3, 2, 1, 23, 0, 5, 24, 
    5, 0, 0, 1, 26, 0, 0, 0, 1, 14, 0, 0, 0, 1, 17, 0, 0, 0, 1, 20, 0, 0, 
    0, 1, 28, 0, 0, 0, 5, 29, 6, 0, 0, 1, 27, 0, 0, 0, 1, 29, 0, 0, 0, 1, 
    31, 0, 0, 0, 1, 12, 0, 0, 0, 1, 13, 0, 0, 0, 1, 32, 0, 0, 0, 5, 33, 
    4, 0, 0, 1, 3, 0, 0, 0, 0, 1, 1, 0, 2, 7, 8, 384, 0, 0, 0, 10, 25, 28, 
    30, 0, 2, 1, 3
];