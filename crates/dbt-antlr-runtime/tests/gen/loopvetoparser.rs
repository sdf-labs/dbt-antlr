// Generated from LoopVeto.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const LoopVeto_T__0:i32=1; 
pub const LoopVeto_T__1:i32=2; 
pub const LoopVeto_INT:i32=3; 
pub const LoopVeto_WS:i32=4;
pub const LoopVeto_EOF:i32=EOF;
pub const RULE_start:usize = 0; 
pub const RULE_send:usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "start", "send"
];

pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
	None, Some("','"), Some("'>>'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;5]  = [
	None, None, None, Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, LoopVetoParserExt<'input, 'arena>, LoopVetoParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopVetoParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> LoopVetoParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				LoopVetoParserExt {
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
        L: ParseTreeListener<'arena, LoopVetoParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct LoopVetoTreeWalker;
impl LoopVetoTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, LoopVetoParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, LoopVetoParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <LoopVetoParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LoopVetoParserNodeKind {
    StartContext,
    SendContext,
    Terminal,
    Error,
}
pub type LoopVetoParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LoopVetoParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => LoopVetoParser }
dbt_antlr_runtime::impl_node_kind! { LoopVetoParserNodeKind {
; StartContext(), SendContext(), 
    }; 
}

pub struct LoopVetoParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> LoopVetoParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { LoopVetoParserExt, LoopVetoParserNodeKind, "LoopVeto.g4"; sempred (LoopVetoParserNode<'input, 'arena, TF::Tok>, LoopVetoParser) { 1 => send_sempred, } }

impl<'input, 'arena, Input, TF> LoopVetoParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn send_sempred(_ctx: Option<&'arena SendContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 1)
		    }
		    _ => true
		}
	}
}
//------------------- start ----------------
pub type StartContextAll<'input, 'arena, Tok = CommonToken<'input>> = StartContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopVetoParserNodeKind, StartContext, StartContextExt, RULE_start }

pub trait StartContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn send(&self) -> Option<&'arena SendContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StartContextAttrs<'input, 'arena, Tok> for StartContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn send(&self) -> Option<&'arena SendContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopVeto_EOF)
    }
}

impl<'input, 'arena, Input, TF> LoopVetoParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn start(&mut self,) -> Result<&'arena StartContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StartContext<TF::Tok>, RULE_start, 0, |_parentctx| StartContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StartContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(4);
		recog.send_rec(0)?;
		recog.base.set_state(5);
		recog.base.match_token(LoopVeto_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- send ----------------
pub type SendContextAll<'input, 'arena, Tok = CommonToken<'input>> = SendContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopVetoParserNodeKind, SendContext, SendContextExt, RULE_send }

pub trait SendContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn send_all(&self) -> Vec<&'arena SendContextAll<'input, 'arena, Tok>>;
    fn send(&self, i: usize) -> Option<&'arena SendContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SendContextAttrs<'input, 'arena, Tok> for SendContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopVeto_INT)
    }
    fn send_all(&self) -> Vec<&'arena SendContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn send(&self, i: usize) -> Option<&'arena SendContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> LoopVetoParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  send(&mut self,) -> Result<&'arena SendContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.send_rec(0)
	}

	fn send_rec(&mut self, _p: i32) -> Result<&'arena SendContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, SendContext<TF::Tok>, RULE_send, 2, |_parentctx| SendContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena SendContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		{
		recog.base.set_state(8);
		recog.base.match_token(LoopVeto_INT,&mut recog.err_handler)?;
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(22);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(1,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				{
				/*recRuleAltStartAction*/
				let tmp = SendContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
				let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_send)?;

				recog.base.set_state(10);
				if !({recog.precpred(None, 1)}) {
					Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
				}
				recog.base.set_state(15);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==LoopVeto_T__0 {
					{
					{
					recog.base.set_state(11);
					recog.base.match_token(LoopVeto_T__0,&mut recog.err_handler)?;
					recog.base.set_state(12);
					recog.send_rec(0)?;
					}
					}
					recog.base.set_state(17);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(18);
				recog.base.match_token(LoopVeto_T__1,&mut recog.err_handler)?;
				recog.base.set_state(19);
				recog.send_rec(2)?;
				}
				} 
			}
			recog.base.set_state(24);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(1,&mut recog.base)?;
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
static _serializedATN: [u32; 342] = [
    1346458702, 3, 16909060, 29, 4, 26, 25, 0, 0, 2, 2, 29, 182, 211, 125, 
    336, 0, 336, 0, 336, 2, 338, 2, 340, 2, 342, 0, 336, 0, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 4294967295, 
    1, 0, 8, 2, 1, 4294967295, 4294967295, 1, 0, 32, 3, 1, 4294967295, 4294967295, 
    1, 0, 8, 4, 1, 4294967295, 4294967295, 1, 1, 72, 5, 1, 4294967295, 4294967295, 
    1, 1, 32, 6, 1, 4294967295, 4294967295, 1, 1, 8, 7, 1, 4294967295, 4294967295, 
    1, 1, 72, 8, 1, 4294967295, 4294967295, 1, 1, 32, 9, 1, 4294967295, 
    4294967295, 1, 1, 8, 10, 1, 4294967295, 4294967295, 5, 1, 8, 11, 1, 
    14, 4294967295, 8, 1, 8, 12, 1, 4294967295, 4294967295, 10, 1, 8, 13, 
    2, 4294967295, 4294967295, 12, 1, 8, 15, 1, 4294967295, 17, 9, 1, 8, 
    16, 1, 4294967295, 4294967295, 1, 1, 32, 17, 1, 4294967295, 4294967295, 
    1, 1, 8, 18, 1, 4294967295, 4294967295, 5, 1, 8, 19, 1, 21, 4294967295, 
    8, 1, 8, 20, 1, 4294967295, 4294967295, 10, 1, 10, 21, 2, 4294967295, 
    4294967295, 12, 1, 8, 23, 1, 4294967295, 24, 9, 1, 8, 24, 1, 4294967295, 
    4294967295, 1, 1, 0, 25, 0, 4294967295, 4294967295, 1, 4, 0, 0, 0, 1, 
    7, 0, 0, 0, 3, 2, 1, 5, 0, 5, 6, 4294967295, 0, 0, 1, 1, 0, 0, 0, 6, 
    8, 1, 4294967295, 0, 5, 9, 3, 0, 0, 1, 22, 0, 0, 0, 10, 15, 1, 0, 0, 
    5, 12, 1, 0, 0, 3, 2, 1, 14, 0, 1, 11, 0, 0, 0, 1, 17, 0, 0, 0, 1, 13, 
    0, 0, 0, 1, 16, 0, 0, 0, 1, 18, 0, 0, 0, 1, 15, 0, 0, 0, 5, 19, 2, 0, 
    0, 3, 2, 1, 21, 2, 1, 10, 0, 0, 0, 1, 24, 0, 0, 0, 1, 20, 0, 0, 0, 1, 
    23, 0, 0, 0, 1, 3, 0, 0, 0, 1, 22, 0, 0, 0, 15, 22, 0, 2, 1, 3
];