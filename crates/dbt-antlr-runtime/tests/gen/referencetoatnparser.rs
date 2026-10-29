// Generated from ReferenceToATN.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use super::referencetoatnlistener::*;
dbt_antlr_runtime::check_version!("0","5");
pub const ReferenceToATN_ID:i32=1; 
pub const ReferenceToATN_ATN:i32=2; 
pub const ReferenceToATN_WS:i32=3;
pub const ReferenceToATN_EOF:i32=EOF;
pub const RULE_a:usize = 0;
pub const ruleNames: [&'static str; 1] = [
    "a"
];

pub const _LITERAL_NAMES: [Option<&'static str>;0] = [
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;4]  = [
	None, Some("ID"), Some("ATN"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, ReferenceToATNParserExt<'input, 'arena>, ReferenceToATNParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ReferenceToATNParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> ReferenceToATNParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				ReferenceToATNParserExt {
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
        L: ReferenceToATNListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct ReferenceToATNTreeWalker;
impl ReferenceToATNTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ReferenceToATNListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, ReferenceToATNParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <ReferenceToATNParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum ReferenceToATNParserNodeKind {
    AContext,
    Terminal,
    Error,
}
pub type ReferenceToATNParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, ReferenceToATNParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => ReferenceToATNParser }
dbt_antlr_runtime::impl_node_kind! { ReferenceToATNParserNodeKind {
; AContext(enter_a, exit_a, ), 
    }; listener = dyn ReferenceToATNListener<'arena, Tok>,
}

pub struct ReferenceToATNParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> ReferenceToATNParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { ReferenceToATNParserExt, ReferenceToATNParserNodeKind, "ReferenceToATN.g4" }

//------------------- a ----------------
pub type AContextAll<'input, 'arena, Tok = CommonToken<'input>> = AContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { ReferenceToATNParserNodeKind, AContext, AContextExt, RULE_a }

pub trait AContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token ATN in current rule
    fn ATN_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ATN, starting from 0.
    /// Returns `None` if number of children corresponding to token ATN is less than or equal to `i`.
    fn ATN(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AContextAttrs<'input, 'arena, Tok> for AContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token ATN in current rule
    fn ATN_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ReferenceToATN_ATN).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ATN, starting from 0.
    /// Returns `None` if number of children corresponding to token ATN is less than or equal to `i`.
    fn ATN(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ReferenceToATN_ATN).nth(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ReferenceToATN_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ReferenceToATN_ID).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ReferenceToATNParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn a(&mut self,) -> Result<&'arena AContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AContext<TF::Tok>, RULE_a, 0, |_parentctx| AContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(5);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(0,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				{
				{
				recog.base.set_state(2);
				_la = recog.base.input.la(1);
				if { !(_la==ReferenceToATN_ID || _la==ReferenceToATN_ATN) } {
					recog.err_handler.recover_inline(&mut recog.base)?;
				}
				else {
					if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler)?;
				}
				}
				} 
			}
			recog.base.set_state(7);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(0,&mut recog.base)?;
		}
		recog.base.set_state(9);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==ReferenceToATN_ATN {
			{
			recog.base.set_state(8);
			recog.base.match_token(ReferenceToATN_ATN,&mut recog.err_handler)?;
			}
		}

		println!("{}",{let temp = recog.base.input.lt(-1).map(|it|it.get_token_index()).unwrap_or(-1); recog.input.get_text_from_interval(recog.ctx().unwrap().start().get_token_index(), temp)});
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
static _serializedATN: LazyLock<Vec<u32>> = LazyLock::new(|| vec![
    1346458702, 3, 16909060, 29, 3, 14, 14, 1, 1, 2, 1, 29, 98, 127, 70, 
    197, 5, 202, 2, 208, 2, 210, 1, 211, 1, 212, 2, 204, 4, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 1, 
    0, 32, 1, 1, 4294967295, 4294967295, 5, 0, 8, 2, 1, 4, 4294967295, 8, 
    0, 8, 3, 1, 4294967295, 4294967295, 10, 0, 8, 4, 2, 4294967295, 4294967295, 
    12, 0, 8, 6, 1, 4294967295, 7, 9, 0, 8, 7, 1, 4294967295, 4294967295, 
    1, 0, 32, 8, 1, 4294967295, 4294967295, 3, 0, 8, 9, 2, 10, 4294967295, 
    8, 0, 8, 11, 1, 4294967295, 4294967295, 1, 0, 72, 12, 1, 4294967295, 
    4294967295, 1, 0, 8, 13, 1, 4294967295, 4294967295, 1, 0, 0, 14, 0, 
    4294967295, 4294967295, 1, 5, 0, 0, 0, 7, 4, 0, 0, 0, 1, 2, 0, 0, 0, 
    1, 7, 0, 0, 0, 1, 3, 0, 0, 0, 1, 6, 0, 0, 0, 1, 9, 0, 0, 0, 1, 5, 0, 
    0, 0, 5, 10, 2, 0, 0, 1, 8, 0, 0, 0, 1, 10, 0, 0, 0, 1, 11, 0, 0, 0, 
    6, 12, 0, 4294967295, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 2, 1, 2, 6, 0, 0, 
    0, 5, 9, 0, 1
]);