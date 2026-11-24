// Generated from DeferredSync.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","11");
pub const DeferredSync_A:i32=1; 
pub const DeferredSync_B:i32=2; 
pub const DeferredSync_C:i32=3; 
pub const DeferredSync_D:i32=4; 
pub const DeferredSync_E:i32=5; 
pub const DeferredSync_PLUS:i32=6; 
pub const DeferredSync_LPAREN:i32=7; 
pub const DeferredSync_RPAREN:i32=8; 
pub const DeferredSync_SEMI:i32=9; 
pub const DeferredSync_ID:i32=10; 
pub const DeferredSync_WS:i32=11;
pub const DeferredSync_EOF:i32=EOF;
pub const RULE_ll1Star:usize = 0; 
pub const RULE_ll1Plus:usize = 1; 
pub const RULE_trailStar:usize = 2; 
pub const RULE_trailPlus:usize = 3; 
pub const RULE_dfaStar:usize = 4; 
pub const RULE_dfaPlus:usize = 5; 
pub const RULE_ll1Alt:usize = 6; 
pub const RULE_dfaAlt:usize = 7; 
pub const RULE_dfaOpt:usize = 8; 
pub const RULE_termOptTop:usize = 9; 
pub const RULE_termOpt:usize = 10; 
pub const RULE_nestedOptTop:usize = 11; 
pub const RULE_outerOpt:usize = 12; 
pub const RULE_innerOpt:usize = 13; 
pub const RULE_zeroSibTop:usize = 14; 
pub const RULE_sibOne:usize = 15; 
pub const RULE_sibTwo:usize = 16; 
pub const RULE_startEmptyTop:usize = 17; 
pub const RULE_nonAdjTop:usize = 18; 
pub const RULE_intraOpt:usize = 19; 
pub const RULE_ambigTermTop:usize = 20; 
pub const RULE_ambigTerm:usize = 21; 
pub const RULE_samePostTermTop:usize = 22; 
pub const RULE_samePostTerm:usize = 23; 
pub const RULE_conjuredTermTop:usize = 24; 
pub const RULE_conjuredTerm:usize = 25;
pub const ruleNames: [&'static str; 26] = [
    "ll1Star", "ll1Plus", "trailStar", "trailPlus", "dfaStar", "dfaPlus", 
    "ll1Alt", "dfaAlt", "dfaOpt", "termOptTop", "termOpt", "nestedOptTop", 
    "outerOpt", "innerOpt", "zeroSibTop", "sibOne", "sibTwo", "startEmptyTop", 
    "nonAdjTop", "intraOpt", "ambigTermTop", "ambigTerm", "samePostTermTop", 
    "samePostTerm", "conjuredTermTop", "conjuredTerm"
];

pub const _LITERAL_NAMES: [Option<&'static str>;10] = [
	None, Some("'a'"), Some("'b'"), Some("'c'"), Some("'d'"), Some("'e'"), 
	Some("'+'"), Some("'('"), Some("')'"), Some("';'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, Some("A"), Some("B"), Some("C"), Some("D"), Some("E"), Some("PLUS"), 
	Some("LPAREN"), Some("RPAREN"), Some("SEMI"), Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, DeferredSyncParserExt<'input, 'arena>, DeferredSyncParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    fn with_delegate(arena: &'arena Arena, input: Input, err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				DeferredSyncParserExt {
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
        L: ParseTreeListener<'arena, DeferredSyncParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct DeferredSyncTreeWalker;
impl DeferredSyncTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, DeferredSyncParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, DeferredSyncParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <DeferredSyncParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum DeferredSyncParserNodeKind {
    Ll1StarContext,
    Ll1PlusContext,
    TrailStarContext,
    TrailPlusContext,
    DfaStarContext,
    DfaPlusContext,
    Ll1AltContext,
    DfaAltContext,
    DfaOptContext,
    TermOptTopContext,
    TermOptContext,
    NestedOptTopContext,
    OuterOptContext,
    InnerOptContext,
    ZeroSibTopContext,
    SibOneContext,
    SibTwoContext,
    StartEmptyTopContext,
    NonAdjTopContext,
    IntraOptContext,
    AmbigTermTopContext,
    AmbigTermContext,
    SamePostTermTopContext,
    SamePostTermContext,
    ConjuredTermTopContext,
    ConjuredTermContext,
    Terminal,
    Error,
}
pub type DeferredSyncParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, DeferredSyncParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => DeferredSyncParser }
dbt_antlr_runtime::impl_node_kind! { DeferredSyncParserNodeKind {
; Ll1StarContext(), Ll1PlusContext(), TrailStarContext(), TrailPlusContext(), DfaStarContext(), DfaPlusContext(), Ll1AltContext(), DfaAltContext(), DfaOptContext(), TermOptTopContext(), TermOptContext(), NestedOptTopContext(), OuterOptContext(), InnerOptContext(), ZeroSibTopContext(), SibOneContext(), SibTwoContext(), StartEmptyTopContext(), NonAdjTopContext(), IntraOptContext(), AmbigTermTopContext(), AmbigTermContext(), SamePostTermTopContext(), SamePostTermContext(), ConjuredTermTopContext(), ConjuredTermContext(), 
    }; 
}

pub struct DeferredSyncParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> DeferredSyncParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { DeferredSyncParserExt, DeferredSyncParserNodeKind, "DeferredSync.g4" }

//------------------- ll1Star ----------------
pub type Ll1StarContextAll<'input, 'arena, Tok = CommonToken<'input>> = Ll1StarContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, Ll1StarContext, Ll1StarContextExt, RULE_ll1Star }

pub trait Ll1StarContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Ll1StarContextAttrs<'input, 'arena, Tok> for Ll1StarContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_RPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_ID, i)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ll1Star(&mut self,) -> Result<&'arena Ll1StarContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, Ll1StarContext<TF::Tok>, RULE_ll1Star, 0, |_parentctx| Ll1StarContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena Ll1StarContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(52);
		recog.base.match_token(DeferredSync_LPAREN,&mut recog.err_handler)?;
		recog.base.set_state(56);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==DeferredSync_ID {
			{
			{
			recog.base.set_state(53);
			recog.base.match_token(DeferredSync_ID,&mut recog.err_handler)?;
			}
			}
			recog.base.set_state(58);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		recog.base.set_state(59);
		recog.base.match_token(DeferredSync_RPAREN,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- ll1Plus ----------------
pub type Ll1PlusContextAll<'input, 'arena, Tok = CommonToken<'input>> = Ll1PlusContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, Ll1PlusContext, Ll1PlusContextExt, RULE_ll1Plus }

pub trait Ll1PlusContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Ll1PlusContextAttrs<'input, 'arena, Tok> for Ll1PlusContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_RPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_ID, i)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ll1Plus(&mut self,) -> Result<&'arena Ll1PlusContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, Ll1PlusContext<TF::Tok>, RULE_ll1Plus, 2, |_parentctx| Ll1PlusContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena Ll1PlusContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(61);
		recog.base.match_token(DeferredSync_LPAREN,&mut recog.err_handler)?;
		recog.base.set_state(63); 
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		loop {
			{
			{
			recog.base.set_state(62);
			recog.base.match_token(DeferredSync_ID,&mut recog.err_handler)?;
			}
			}
			recog.base.set_state(65); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if !(_la==DeferredSync_ID) {break}
		}
		recog.base.set_state(67);
		recog.base.match_token(DeferredSync_RPAREN,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- trailStar ----------------
pub type TrailStarContextAll<'input, 'arena, Tok = CommonToken<'input>> = TrailStarContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, TrailStarContext, TrailStarContextExt, RULE_trailStar }

pub trait TrailStarContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TrailStarContextAttrs<'input, 'arena, Tok> for TrailStarContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_ID, i)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn trailStar(&mut self,) -> Result<&'arena TrailStarContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TrailStarContext<TF::Tok>, RULE_trailStar, 4, |_parentctx| TrailStarContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TrailStarContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(72);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		while _la==DeferredSync_ID {
			{
			{
			recog.base.set_state(69);
			recog.base.match_token(DeferredSync_ID,&mut recog.err_handler)?;
			}
			}
			recog.base.set_state(74);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
		}
		}
		})
	}
}
//------------------- trailPlus ----------------
pub type TrailPlusContextAll<'input, 'arena, Tok = CommonToken<'input>> = TrailPlusContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, TrailPlusContext, TrailPlusContextExt, RULE_trailPlus }

pub trait TrailPlusContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TrailPlusContextAttrs<'input, 'arena, Tok> for TrailPlusContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_ID, i)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn trailPlus(&mut self,) -> Result<&'arena TrailPlusContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TrailPlusContext<TF::Tok>, RULE_trailPlus, 6, |_parentctx| TrailPlusContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TrailPlusContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(76); 
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		loop {
			{
			{
			recog.base.set_state(75);
			recog.base.match_token(DeferredSync_ID,&mut recog.err_handler)?;
			}
			}
			recog.base.set_state(78); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if !(_la==DeferredSync_ID) {break}
		}
		}
		})
	}
}
//------------------- dfaStar ----------------
pub type DfaStarContextAll<'input, 'arena, Tok = CommonToken<'input>> = DfaStarContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, DfaStarContext, DfaStarContextExt, RULE_dfaStar }

pub trait DfaStarContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token A in current rule
    fn A_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token A, starting from 0.
    /// Returns `None` if number of children corresponding to token A is less than or equal to `i`.
    fn A(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token B in current rule
    fn B_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token B, starting from 0.
    /// Returns `None` if number of children corresponding to token B is less than or equal to `i`.
    fn B(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DfaStarContextAttrs<'input, 'arena, Tok> for DfaStarContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_LPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token A in current rule
    fn A_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_A).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token A, starting from 0.
    /// Returns `None` if number of children corresponding to token A is less than or equal to `i`.
    fn A(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, i)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_RPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token B in current rule
    fn B_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_B).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token B, starting from 0.
    /// Returns `None` if number of children corresponding to token B is less than or equal to `i`.
    fn B(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, i)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn dfaStar(&mut self,) -> Result<&'arena DfaStarContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, DfaStarContext<TF::Tok>, RULE_dfaStar, 8, |_parentctx| DfaStarContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena DfaStarContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(80);
		recog.base.match_token(DeferredSync_LPAREN,&mut recog.err_handler)?;
		recog.base.set_state(85);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(4,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				{
				{
				recog.base.set_state(81);
				recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
				recog.base.set_state(82);
				recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
				}
				} 
			}
			recog.base.set_state(87);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(4,&mut recog.base)?;
		}
		recog.base.set_state(88);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(89);
		recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
		recog.base.set_state(90);
		recog.base.match_token(DeferredSync_RPAREN,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- dfaPlus ----------------
pub type DfaPlusContextAll<'input, 'arena, Tok = CommonToken<'input>> = DfaPlusContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, DfaPlusContext, DfaPlusContextExt, RULE_dfaPlus }

pub trait DfaPlusContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token A in current rule
    fn A_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token A, starting from 0.
    /// Returns `None` if number of children corresponding to token A is less than or equal to `i`.
    fn A(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token B in current rule
    fn B_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token B, starting from 0.
    /// Returns `None` if number of children corresponding to token B is less than or equal to `i`.
    fn B(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DfaPlusContextAttrs<'input, 'arena, Tok> for DfaPlusContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_LPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token A in current rule
    fn A_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_A).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token A, starting from 0.
    /// Returns `None` if number of children corresponding to token A is less than or equal to `i`.
    fn A(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, i)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_RPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token B in current rule
    fn B_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_B).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token B, starting from 0.
    /// Returns `None` if number of children corresponding to token B is less than or equal to `i`.
    fn B(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, i)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn dfaPlus(&mut self,) -> Result<&'arena DfaPlusContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, DfaPlusContext<TF::Tok>, RULE_dfaPlus, 10, |_parentctx| DfaPlusContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena DfaPlusContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(92);
		recog.base.match_token(DeferredSync_LPAREN,&mut recog.err_handler)?;
		recog.base.set_state(95); 
		recog.err_handler.sync(&mut recog.base)?;
		_alt = 1;
		loop {
			match _alt {
			    x if x == 1=>
				{
				{
				recog.base.set_state(93);
				recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
				recog.base.set_state(94);
				recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
				}
				}

			_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			recog.base.set_state(97); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(5,&mut recog.base)?;
			if _alt==2 || _alt==INVALID_ALT { break }
		}
		recog.base.set_state(99);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(100);
		recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
		recog.base.set_state(101);
		recog.base.match_token(DeferredSync_RPAREN,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- ll1Alt ----------------
pub type Ll1AltContextAll<'input, 'arena, Tok = CommonToken<'input>> = Ll1AltContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, Ll1AltContext, Ll1AltContextExt, RULE_ll1Alt }

pub trait Ll1AltContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Ll1AltContextAttrs<'input, 'arena, Tok> for Ll1AltContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ll1Alt(&mut self,) -> Result<&'arena Ll1AltContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, Ll1AltContext<TF::Tok>, RULE_ll1Alt, 12, |_parentctx| Ll1AltContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena Ll1AltContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(107);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    DeferredSync_A  => {
		        {
		        recog.base.set_state(103);
		        recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		        recog.base.set_state(104);
		        recog.base.match_token(DeferredSync_ID,&mut recog.err_handler)?;
		        }}
		    DeferredSync_B  => {
		        {
		        recog.base.set_state(105);
		        recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
		        recog.base.set_state(106);
		        recog.base.match_token(DeferredSync_ID,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		recog.base.set_state(109);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- dfaAlt ----------------
pub type DfaAltContextAll<'input, 'arena, Tok = CommonToken<'input>> = DfaAltContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, DfaAltContext, DfaAltContextExt, RULE_dfaAlt }

pub trait DfaAltContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DfaAltContextAttrs<'input, 'arena, Tok> for DfaAltContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_D, 0)
    }
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_E, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn dfaAlt(&mut self,) -> Result<&'arena DfaAltContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, DfaAltContext<TF::Tok>, RULE_dfaAlt, 14, |_parentctx| DfaAltContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena DfaAltContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(117);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(7,&mut recog.base)? {
			1 =>{
				{
				recog.base.set_state(111);
				recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
				recog.base.set_state(112);
				recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				{
				recog.base.set_state(113);
				recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
				recog.base.set_state(114);
				recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				{
				recog.base.set_state(115);
				recog.base.match_token(DeferredSync_D,&mut recog.err_handler)?;
				recog.base.set_state(116);
				recog.base.match_token(DeferredSync_E,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		recog.base.set_state(119);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- dfaOpt ----------------
pub type DfaOptContextAll<'input, 'arena, Tok = CommonToken<'input>> = DfaOptContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, DfaOptContext, DfaOptContextExt, RULE_dfaOpt }

pub trait DfaOptContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token A in current rule
    fn A_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token A, starting from 0.
    /// Returns `None` if number of children corresponding to token A is less than or equal to `i`.
    fn A(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DfaOptContextAttrs<'input, 'arena, Tok> for DfaOptContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_LPAREN, 0)
    }
    /// Retrieves all `TerminalNode`s corresponding to token A in current rule
    fn A_all(&self) -> Vec<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.tokens_of_type(DeferredSync_A).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token A, starting from 0.
    /// Returns `None` if number of children corresponding to token A is less than or equal to `i`.
    fn A(&self, i: usize) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, i)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_RPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn dfaOpt(&mut self,) -> Result<&'arena DfaOptContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, DfaOptContext<TF::Tok>, RULE_dfaOpt, 16, |_parentctx| DfaOptContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena DfaOptContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(121);
		recog.base.match_token(DeferredSync_LPAREN,&mut recog.err_handler)?;
		recog.base.set_state(124);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(8,&mut recog.base)? {
			x if x == 1=>{
				{
				recog.base.set_state(122);
				recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
				recog.base.set_state(123);
				recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		recog.base.set_state(126);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(127);
		recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
		recog.base.set_state(128);
		recog.base.match_token(DeferredSync_RPAREN,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- termOptTop ----------------
pub type TermOptTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = TermOptTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, TermOptTopContext, TermOptTopContextExt, RULE_termOptTop }

pub trait TermOptTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn termOpt(&self) -> Option<&'arena TermOptContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TermOptTopContextAttrs<'input, 'arena, Tok> for TermOptTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn termOpt(&self) -> Option<&'arena TermOptContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn termOptTop(&mut self,) -> Result<&'arena TermOptTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TermOptTopContext<TF::Tok>, RULE_termOptTop, 18, |_parentctx| TermOptTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TermOptTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(130);
		recog.termOpt()?;
		recog.base.set_state(131);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- termOpt ----------------
pub type TermOptContextAll<'input, 'arena, Tok = CommonToken<'input>> = TermOptContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, TermOptContext, TermOptContextExt, RULE_termOpt }

pub trait TermOptContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TermOptContextAttrs<'input, 'arena, Tok> for TermOptContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn termOpt(&mut self,) -> Result<&'arena TermOptContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TermOptContext<TF::Tok>, RULE_termOpt, 20, |_parentctx| TermOptContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TermOptContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(133);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(135);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_B {
			{
			recog.base.set_state(134);
			recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
			}
		}

		}
		})
	}
}
//------------------- nestedOptTop ----------------
pub type NestedOptTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = NestedOptTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, NestedOptTopContext, NestedOptTopContextExt, RULE_nestedOptTop }

pub trait NestedOptTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn outerOpt(&self) -> Option<&'arena OuterOptContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NestedOptTopContextAttrs<'input, 'arena, Tok> for NestedOptTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn outerOpt(&self) -> Option<&'arena OuterOptContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn nestedOptTop(&mut self,) -> Result<&'arena NestedOptTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, NestedOptTopContext<TF::Tok>, RULE_nestedOptTop, 22, |_parentctx| NestedOptTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena NestedOptTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(137);
		recog.outerOpt()?;
		recog.base.set_state(138);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- outerOpt ----------------
pub type OuterOptContextAll<'input, 'arena, Tok = CommonToken<'input>> = OuterOptContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, OuterOptContext, OuterOptContextExt, RULE_outerOpt }

pub trait OuterOptContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn innerOpt(&self) -> Option<&'arena InnerOptContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> OuterOptContextAttrs<'input, 'arena, Tok> for OuterOptContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn innerOpt(&self) -> Option<&'arena InnerOptContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn outerOpt(&mut self,) -> Result<&'arena OuterOptContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, OuterOptContext<TF::Tok>, RULE_outerOpt, 24, |_parentctx| OuterOptContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena OuterOptContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(140);
		recog.innerOpt()?;
		recog.base.set_state(142);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_B {
			{
			recog.base.set_state(141);
			recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
			}
		}

		}
		})
	}
}
//------------------- innerOpt ----------------
pub type InnerOptContextAll<'input, 'arena, Tok = CommonToken<'input>> = InnerOptContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, InnerOptContext, InnerOptContextExt, RULE_innerOpt }

pub trait InnerOptContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> InnerOptContextAttrs<'input, 'arena, Tok> for InnerOptContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn innerOpt(&mut self,) -> Result<&'arena InnerOptContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, InnerOptContext<TF::Tok>, RULE_innerOpt, 26, |_parentctx| InnerOptContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena InnerOptContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(144);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(146);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_C {
			{
			recog.base.set_state(145);
			recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
			}
		}

		}
		})
	}
}
//------------------- zeroSibTop ----------------
pub type ZeroSibTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = ZeroSibTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, ZeroSibTopContext, ZeroSibTopContextExt, RULE_zeroSibTop }

pub trait ZeroSibTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn sibOne(&self) -> Option<&'arena SibOneContextAll<'input, 'arena, Tok>>;
    fn sibTwo(&self) -> Option<&'arena SibTwoContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ZeroSibTopContextAttrs<'input, 'arena, Tok> for ZeroSibTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn sibOne(&self) -> Option<&'arena SibOneContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn sibTwo(&self) -> Option<&'arena SibTwoContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn zeroSibTop(&mut self,) -> Result<&'arena ZeroSibTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ZeroSibTopContext<TF::Tok>, RULE_zeroSibTop, 28, |_parentctx| ZeroSibTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ZeroSibTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(148);
		recog.sibOne()?;
		recog.base.set_state(149);
		recog.sibTwo()?;
		recog.base.set_state(150);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- sibOne ----------------
pub type SibOneContextAll<'input, 'arena, Tok = CommonToken<'input>> = SibOneContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, SibOneContext, SibOneContextExt, RULE_sibOne }

pub trait SibOneContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SibOneContextAttrs<'input, 'arena, Tok> for SibOneContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn sibOne(&mut self,) -> Result<&'arena SibOneContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SibOneContext<TF::Tok>, RULE_sibOne, 30, |_parentctx| SibOneContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SibOneContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(152);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(154);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_B {
			{
			recog.base.set_state(153);
			recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
			}
		}

		}
		})
	}
}
//------------------- sibTwo ----------------
pub type SibTwoContextAll<'input, 'arena, Tok = CommonToken<'input>> = SibTwoContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, SibTwoContext, SibTwoContextExt, RULE_sibTwo }

pub trait SibTwoContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SibTwoContextAttrs<'input, 'arena, Tok> for SibTwoContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn sibTwo(&mut self,) -> Result<&'arena SibTwoContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SibTwoContext<TF::Tok>, RULE_sibTwo, 32, |_parentctx| SibTwoContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SibTwoContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(157);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_C {
			{
			recog.base.set_state(156);
			recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
			}
		}

		}
		})
	}
}
//------------------- startEmptyTop ----------------
pub type StartEmptyTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = StartEmptyTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, StartEmptyTopContext, StartEmptyTopContextExt, RULE_startEmptyTop }

pub trait StartEmptyTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn sibTwo(&self) -> Option<&'arena SibTwoContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StartEmptyTopContextAttrs<'input, 'arena, Tok> for StartEmptyTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn sibTwo(&self) -> Option<&'arena SibTwoContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn startEmptyTop(&mut self,) -> Result<&'arena StartEmptyTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, StartEmptyTopContext<TF::Tok>, RULE_startEmptyTop, 34, |_parentctx| StartEmptyTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena StartEmptyTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(159);
		recog.sibTwo()?;
		recog.base.set_state(160);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- nonAdjTop ----------------
pub type NonAdjTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = NonAdjTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, NonAdjTopContext, NonAdjTopContextExt, RULE_nonAdjTop }

pub trait NonAdjTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn termOpt(&self) -> Option<&'arena TermOptContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NonAdjTopContextAttrs<'input, 'arena, Tok> for NonAdjTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn termOpt(&self) -> Option<&'arena TermOptContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_D, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn nonAdjTop(&mut self,) -> Result<&'arena NonAdjTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, NonAdjTopContext<TF::Tok>, RULE_nonAdjTop, 36, |_parentctx| NonAdjTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena NonAdjTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(162);
		recog.termOpt()?;
		recog.base.set_state(163);
		recog.base.match_token(DeferredSync_D,&mut recog.err_handler)?;
		recog.base.set_state(164);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- intraOpt ----------------
pub type IntraOptContextAll<'input, 'arena, Tok = CommonToken<'input>> = IntraOptContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, IntraOptContext, IntraOptContextExt, RULE_intraOpt }

pub trait IntraOptContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> IntraOptContextAttrs<'input, 'arena, Tok> for IntraOptContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn intraOpt(&mut self,) -> Result<&'arena IntraOptContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, IntraOptContext<TF::Tok>, RULE_intraOpt, 38, |_parentctx| IntraOptContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena IntraOptContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(166);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(168);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_B {
			{
			recog.base.set_state(167);
			recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
			}
		}

		recog.base.set_state(170);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- ambigTermTop ----------------
pub type AmbigTermTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = AmbigTermTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, AmbigTermTopContext, AmbigTermTopContextExt, RULE_ambigTermTop }

pub trait AmbigTermTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn ambigTerm(&self) -> Option<&'arena AmbigTermContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AmbigTermTopContextAttrs<'input, 'arena, Tok> for AmbigTermTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ambigTerm(&self) -> Option<&'arena AmbigTermContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ambigTermTop(&mut self,) -> Result<&'arena AmbigTermTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AmbigTermTopContext<TF::Tok>, RULE_ambigTermTop, 40, |_parentctx| AmbigTermTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AmbigTermTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(172);
		recog.ambigTerm()?;
		recog.base.set_state(173);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- ambigTerm ----------------
pub type AmbigTermContextAll<'input, 'arena, Tok = CommonToken<'input>> = AmbigTermContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, AmbigTermContext, AmbigTermContextExt, RULE_ambigTerm }

pub trait AmbigTermContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
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
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AmbigTermContextAttrs<'input, 'arena, Tok> for AmbigTermContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_D, 0)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_E, 0)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_C, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ambigTerm(&mut self,) -> Result<&'arena AmbigTermContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, AmbigTermContext<TF::Tok>, RULE_ambigTerm, 42, |_parentctx| AmbigTermContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena AmbigTermContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(185);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    DeferredSync_D  => {
		        /*------- Outer Most Alt 1 -------*/
		        {
		        recog.base.set_state(175);
		        recog.base.match_token(DeferredSync_D,&mut recog.err_handler)?;
		        recog.base.set_state(176);
		        recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		        recog.base.set_state(178);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        if _la==DeferredSync_B {
		        	{
		        	recog.base.set_state(177);
		        	recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
		        	}
		        }

		        }}
		    DeferredSync_E  => {
		        /*------- Outer Most Alt 2 -------*/
		        {
		        recog.base.set_state(180);
		        recog.base.match_token(DeferredSync_E,&mut recog.err_handler)?;
		        recog.base.set_state(181);
		        recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		        recog.base.set_state(183);
		        recog.err_handler.sync(&mut recog.base)?;
		        _la = recog.base.input.la(1);
		        if _la==DeferredSync_C {
		        	{
		        	recog.base.set_state(182);
		        	recog.base.match_token(DeferredSync_C,&mut recog.err_handler)?;
		        	}
		        }

		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- samePostTermTop ----------------
pub type SamePostTermTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = SamePostTermTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, SamePostTermTopContext, SamePostTermTopContextExt, RULE_samePostTermTop }

pub trait SamePostTermTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn samePostTerm(&self) -> Option<&'arena SamePostTermContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SamePostTermTopContextAttrs<'input, 'arena, Tok> for SamePostTermTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn samePostTerm(&self) -> Option<&'arena SamePostTermContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn samePostTermTop(&mut self,) -> Result<&'arena SamePostTermTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SamePostTermTopContext<TF::Tok>, RULE_samePostTermTop, 44, |_parentctx| SamePostTermTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SamePostTermTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(187);
		recog.samePostTerm()?;
		recog.base.set_state(188);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- samePostTerm ----------------
pub type SamePostTermContextAll<'input, 'arena, Tok = CommonToken<'input>> = SamePostTermContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, SamePostTermContext, SamePostTermContextExt, RULE_samePostTerm }

pub trait SamePostTermContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
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
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SamePostTermContextAttrs<'input, 'arena, Tok> for SamePostTermContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_D, 0)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_E, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn samePostTerm(&mut self,) -> Result<&'arena SamePostTermContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, SamePostTermContext<TF::Tok>, RULE_samePostTerm, 46, |_parentctx| SamePostTermContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena SamePostTermContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(194);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    DeferredSync_D  => {
		        {
		        recog.base.set_state(190);
		        recog.base.match_token(DeferredSync_D,&mut recog.err_handler)?;
		        recog.base.set_state(191);
		        recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		        }}
		    DeferredSync_E  => {
		        {
		        recog.base.set_state(192);
		        recog.base.match_token(DeferredSync_E,&mut recog.err_handler)?;
		        recog.base.set_state(193);
		        recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		recog.base.set_state(197);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_B {
			{
			recog.base.set_state(196);
			recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
			}
		}

		}
		})
	}
}
//------------------- conjuredTermTop ----------------
pub type ConjuredTermTopContextAll<'input, 'arena, Tok = CommonToken<'input>> = ConjuredTermTopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, ConjuredTermTopContext, ConjuredTermTopContextExt, RULE_conjuredTermTop }

pub trait ConjuredTermTopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn conjuredTerm(&self) -> Option<&'arena ConjuredTermContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ConjuredTermTopContextAttrs<'input, 'arena, Tok> for ConjuredTermTopContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn conjuredTerm(&self) -> Option<&'arena ConjuredTermContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_SEMI, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn conjuredTermTop(&mut self,) -> Result<&'arena ConjuredTermTopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ConjuredTermTopContext<TF::Tok>, RULE_conjuredTermTop, 48, |_parentctx| ConjuredTermTopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ConjuredTermTopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(199);
		recog.conjuredTerm()?;
		recog.base.set_state(200);
		recog.base.match_token(DeferredSync_SEMI,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- conjuredTerm ----------------
pub type ConjuredTermContextAll<'input, 'arena, Tok = CommonToken<'input>> = ConjuredTermContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { DeferredSyncParserNodeKind, ConjuredTermContext, ConjuredTermContextExt, RULE_conjuredTerm }

pub trait ConjuredTermContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
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
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ConjuredTermContextAttrs<'input, 'arena, Tok> for ConjuredTermContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_D, 0)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_A, 0)
    }
    /// Retrieves first TerminalNode corresponding to token B
    /// Returns `None` if there is no child corresponding to token B
    fn B(&self) -> Option<&'arena TerminalNode<'input, 'arena, Tok>> {
        self.token_of_type(DeferredSync_B, 0)
    }
}

impl<'input, 'arena, Input, TF> DeferredSyncParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn conjuredTerm(&mut self,) -> Result<&'arena ConjuredTermContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, ConjuredTermContext<TF::Tok>, RULE_conjuredTerm, 50, |_parentctx| ConjuredTermContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena ConjuredTermContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		/*------- Outer Most Alt 1 -------*/
		{
		recog.base.set_state(202);
		recog.base.match_token(DeferredSync_D,&mut recog.err_handler)?;
		recog.base.set_state(203);
		recog.base.match_token(DeferredSync_A,&mut recog.err_handler)?;
		recog.base.set_state(205);
		recog.err_handler.sync(&mut recog.base)?;
		_la = recog.base.input.la(1);
		if _la==DeferredSync_B {
			{
			recog.base.set_state(204);
			recog.base.match_token(DeferredSync_B,&mut recog.err_handler)?;
			}
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
static _serializedATN: [u32; 2573] = [
    1346458702, 3, 16909060, 29, 11, 208, 203, 0, 0, 21, 26, 29, 1456, 1485, 
    1015, 2500, 0, 2500, 0, 2500, 21, 2521, 26, 2547, 26, 2573, 0, 2500, 
    0, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 
    4294967295, 2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 
    4294967295, 2, 5, 8, 5, 1, 4294967295, 4294967295, 7, 5, 16, 6, 0, 4294967295, 
    4294967295, 2, 6, 8, 6, 1, 4294967295, 4294967295, 7, 6, 16, 7, 0, 4294967295, 
    4294967295, 2, 7, 8, 7, 1, 4294967295, 4294967295, 7, 7, 16, 8, 0, 4294967295, 
    4294967295, 2, 8, 8, 8, 1, 4294967295, 4294967295, 7, 8, 16, 9, 0, 4294967295, 
    4294967295, 2, 9, 8, 9, 1, 4294967295, 4294967295, 7, 9, 16, 10, 0, 
    4294967295, 4294967295, 2, 10, 8, 10, 1, 4294967295, 4294967295, 7, 
    10, 16, 11, 0, 4294967295, 4294967295, 2, 11, 8, 11, 1, 4294967295, 
    4294967295, 7, 11, 16, 12, 0, 4294967295, 4294967295, 2, 12, 8, 12, 
    1, 4294967295, 4294967295, 7, 12, 16, 13, 0, 4294967295, 4294967295, 
    2, 13, 8, 13, 1, 4294967295, 4294967295, 7, 13, 16, 14, 0, 4294967295, 
    4294967295, 2, 14, 8, 14, 1, 4294967295, 4294967295, 7, 14, 16, 15, 
    0, 4294967295, 4294967295, 2, 15, 8, 15, 1, 4294967295, 4294967295, 
    7, 15, 16, 16, 0, 4294967295, 4294967295, 2, 16, 8, 16, 1, 4294967295, 
    4294967295, 7, 16, 16, 17, 0, 4294967295, 4294967295, 2, 17, 8, 17, 
    1, 4294967295, 4294967295, 7, 17, 16, 18, 0, 4294967295, 4294967295, 
    2, 18, 8, 18, 1, 4294967295, 4294967295, 7, 18, 16, 19, 0, 4294967295, 
    4294967295, 2, 19, 8, 19, 1, 4294967295, 4294967295, 7, 19, 16, 20, 
    0, 4294967295, 4294967295, 2, 20, 8, 20, 1, 4294967295, 4294967295, 
    7, 20, 16, 21, 0, 4294967295, 4294967295, 2, 21, 8, 21, 1, 4294967295, 
    4294967295, 7, 21, 16, 22, 0, 4294967295, 4294967295, 2, 22, 8, 22, 
    1, 4294967295, 4294967295, 7, 22, 16, 23, 0, 4294967295, 4294967295, 
    2, 23, 8, 23, 1, 4294967295, 4294967295, 7, 23, 16, 24, 0, 4294967295, 
    4294967295, 2, 24, 8, 24, 1, 4294967295, 4294967295, 7, 24, 16, 25, 
    0, 4294967295, 4294967295, 2, 25, 8, 25, 1, 4294967295, 4294967295, 
    7, 25, 16, 26, 0, 4294967295, 4294967295, 1, 0, 32, 26, 1, 4294967295, 
    4294967295, 1, 0, 32, 27, 1, 4294967295, 4294967295, 5, 0, 8, 28, 1, 
    55, 4294967295, 8, 0, 8, 29, 1, 4294967295, 4294967295, 10, 0, 8, 30, 
    2, 4294967295, 4294967295, 12, 0, 8, 32, 1, 4294967295, 58, 9, 0, 8, 
    33, 1, 4294967295, 4294967295, 1, 0, 32, 34, 1, 4294967295, 4294967295, 
    1, 0, 8, 35, 1, 4294967295, 4294967295, 1, 1, 32, 36, 1, 4294967295, 
    4294967295, 1, 1, 32, 37, 1, 4294967295, 4294967295, 4, 1, 8, 38, 1, 
    64, 4294967295, 8, 1, 8, 39, 1, 4294967295, 4294967295, 11, 1, 8, 40, 
    2, 4294967295, 4294967295, 12, 1, 8, 42, 1, 4294967295, 65, 1, 1, 32, 
    43, 1, 4294967295, 4294967295, 1, 1, 8, 44, 1, 4294967295, 4294967295, 
    1, 2, 32, 45, 1, 4294967295, 4294967295, 5, 2, 8, 46, 1, 71, 4294967295, 
    8, 2, 8, 47, 1, 4294967295, 4294967295, 10, 2, 8, 48, 2, 4294967295, 
    4294967295, 12, 2, 8, 50, 1, 4294967295, 74, 9, 2, 8, 51, 1, 4294967295, 
    4294967295, 1, 3, 32, 52, 1, 4294967295, 4294967295, 4, 3, 8, 53, 1, 
    77, 4294967295, 8, 3, 8, 54, 1, 4294967295, 4294967295, 11, 3, 8, 55, 
    2, 4294967295, 4294967295, 12, 3, 8, 57, 1, 4294967295, 78, 1, 4, 32, 
    58, 1, 4294967295, 4294967295, 1, 4, 32, 59, 1, 4294967295, 4294967295, 
    1, 4, 32, 60, 1, 4294967295, 4294967295, 5, 4, 8, 61, 1, 84, 4294967295, 
    8, 4, 8, 62, 1, 4294967295, 4294967295, 10, 4, 8, 63, 2, 4294967295, 
    4294967295, 12, 4, 8, 65, 1, 4294967295, 87, 9, 4, 8, 66, 1, 4294967295, 
    4294967295, 1, 4, 32, 67, 1, 4294967295, 4294967295, 1, 4, 32, 68, 1, 
    4294967295, 4294967295, 1, 4, 32, 69, 1, 4294967295, 4294967295, 1, 
    4, 8, 70, 1, 4294967295, 4294967295, 1, 5, 32, 71, 1, 4294967295, 4294967295, 
    1, 5, 32, 72, 1, 4294967295, 4294967295, 1, 5, 32, 73, 1, 4294967295, 
    4294967295, 4, 5, 8, 74, 1, 96, 4294967295, 8, 5, 8, 75, 1, 4294967295, 
    4294967295, 11, 5, 8, 76, 2, 4294967295, 4294967295, 12, 5, 8, 78, 1, 
    4294967295, 97, 1, 5, 32, 79, 1, 4294967295, 4294967295, 1, 5, 32, 80, 
    1, 4294967295, 4294967295, 1, 5, 32, 81, 1, 4294967295, 4294967295, 
    1, 5, 8, 82, 1, 4294967295, 4294967295, 1, 6, 32, 83, 1, 4294967295, 
    4294967295, 1, 6, 32, 84, 1, 4294967295, 4294967295, 1, 6, 32, 85, 1, 
    4294967295, 4294967295, 1, 6, 32, 86, 1, 4294967295, 4294967295, 3, 
    6, 8, 87, 2, 108, 4294967295, 8, 6, 8, 89, 1, 4294967295, 4294967295, 
    1, 6, 32, 90, 1, 4294967295, 4294967295, 1, 6, 8, 91, 1, 4294967295, 
    4294967295, 1, 7, 32, 92, 1, 4294967295, 4294967295, 1, 7, 32, 93, 1, 
    4294967295, 4294967295, 1, 7, 32, 94, 1, 4294967295, 4294967295, 1, 
    7, 32, 95, 1, 4294967295, 4294967295, 1, 7, 32, 96, 1, 4294967295, 4294967295, 
    1, 7, 32, 97, 1, 4294967295, 4294967295, 3, 7, 8, 98, 3, 118, 4294967295, 
    8, 7, 8, 101, 1, 4294967295, 4294967295, 1, 7, 32, 102, 1, 4294967295, 
    4294967295, 1, 7, 8, 103, 1, 4294967295, 4294967295, 1, 8, 32, 104, 
    1, 4294967295, 4294967295, 1, 8, 32, 105, 1, 4294967295, 4294967295, 
    1, 8, 32, 106, 1, 4294967295, 4294967295, 3, 8, 8, 107, 2, 125, 4294967295, 
    8, 8, 8, 109, 1, 4294967295, 4294967295, 1, 8, 32, 110, 1, 4294967295, 
    4294967295, 1, 8, 32, 111, 1, 4294967295, 4294967295, 1, 8, 32, 112, 
    1, 4294967295, 4294967295, 1, 8, 8, 113, 1, 4294967295, 4294967295, 
    1, 9, 8, 114, 1, 4294967295, 4294967295, 1, 9, 32, 115, 1, 4294967295, 
    4294967295, 1, 9, 8, 116, 1, 4294967295, 4294967295, 1, 10, 32, 117, 
    1, 4294967295, 4294967295, 1, 10, 32, 118, 1, 4294967295, 4294967295, 
    3, 10, 8, 119, 2, 136, 4294967295, 8, 10, 8, 121, 1, 4294967295, 4294967295, 
    1, 11, 8, 122, 1, 4294967295, 4294967295, 1, 11, 32, 123, 1, 4294967295, 
    4294967295, 1, 11, 8, 124, 1, 4294967295, 4294967295, 1, 12, 8, 125, 
    1, 4294967295, 4294967295, 1, 12, 32, 126, 1, 4294967295, 4294967295, 
    3, 12, 8, 127, 2, 143, 4294967295, 8, 12, 8, 129, 1, 4294967295, 4294967295, 
    1, 13, 32, 130, 1, 4294967295, 4294967295, 1, 13, 32, 131, 1, 4294967295, 
    4294967295, 3, 13, 8, 132, 2, 147, 4294967295, 8, 13, 8, 134, 1, 4294967295, 
    4294967295, 1, 14, 8, 135, 1, 4294967295, 4294967295, 1, 14, 8, 136, 
    1, 4294967295, 4294967295, 1, 14, 32, 137, 1, 4294967295, 4294967295, 
    1, 14, 8, 138, 1, 4294967295, 4294967295, 1, 15, 32, 139, 1, 4294967295, 
    4294967295, 1, 15, 32, 140, 1, 4294967295, 4294967295, 3, 15, 8, 141, 
    2, 155, 4294967295, 8, 15, 8, 143, 1, 4294967295, 4294967295, 1, 16, 
    32, 144, 1, 4294967295, 4294967295, 3, 16, 8, 145, 2, 158, 4294967295, 
    8, 16, 8, 147, 1, 4294967295, 4294967295, 1, 17, 8, 148, 1, 4294967295, 
    4294967295, 1, 17, 32, 149, 1, 4294967295, 4294967295, 1, 17, 8, 150, 
    1, 4294967295, 4294967295, 1, 18, 8, 151, 1, 4294967295, 4294967295, 
    1, 18, 32, 152, 1, 4294967295, 4294967295, 1, 18, 32, 153, 1, 4294967295, 
    4294967295, 1, 18, 8, 154, 1, 4294967295, 4294967295, 1, 19, 32, 155, 
    1, 4294967295, 4294967295, 1, 19, 32, 156, 1, 4294967295, 4294967295, 
    3, 19, 8, 157, 2, 169, 4294967295, 8, 19, 8, 159, 1, 4294967295, 4294967295, 
    1, 19, 32, 160, 1, 4294967295, 4294967295, 1, 19, 8, 161, 1, 4294967295, 
    4294967295, 1, 20, 8, 162, 1, 4294967295, 4294967295, 1, 20, 32, 163, 
    1, 4294967295, 4294967295, 1, 20, 8, 164, 1, 4294967295, 4294967295, 
    1, 21, 32, 165, 1, 4294967295, 4294967295, 1, 21, 32, 166, 1, 4294967295, 
    4294967295, 1, 21, 32, 167, 1, 4294967295, 4294967295, 3, 21, 8, 168, 
    2, 179, 4294967295, 8, 21, 8, 170, 1, 4294967295, 4294967295, 1, 21, 
    32, 171, 1, 4294967295, 4294967295, 1, 21, 32, 172, 1, 4294967295, 4294967295, 
    1, 21, 32, 173, 1, 4294967295, 4294967295, 3, 21, 8, 174, 2, 184, 4294967295, 
    8, 21, 8, 176, 1, 4294967295, 4294967295, 3, 21, 8, 177, 2, 186, 4294967295, 
    8, 21, 8, 179, 1, 4294967295, 4294967295, 1, 22, 8, 180, 1, 4294967295, 
    4294967295, 1, 22, 32, 181, 1, 4294967295, 4294967295, 1, 22, 8, 182, 
    1, 4294967295, 4294967295, 1, 23, 32, 183, 1, 4294967295, 4294967295, 
    1, 23, 32, 184, 1, 4294967295, 4294967295, 1, 23, 32, 185, 1, 4294967295, 
    4294967295, 1, 23, 32, 186, 1, 4294967295, 4294967295, 3, 23, 8, 187, 
    2, 195, 4294967295, 8, 23, 8, 189, 1, 4294967295, 4294967295, 1, 23, 
    32, 190, 1, 4294967295, 4294967295, 3, 23, 8, 191, 2, 198, 4294967295, 
    8, 23, 8, 193, 1, 4294967295, 4294967295, 1, 24, 8, 194, 1, 4294967295, 
    4294967295, 1, 24, 32, 195, 1, 4294967295, 4294967295, 1, 24, 8, 196, 
    1, 4294967295, 4294967295, 1, 25, 32, 197, 1, 4294967295, 4294967295, 
    1, 25, 32, 198, 1, 4294967295, 4294967295, 1, 25, 32, 199, 1, 4294967295, 
    4294967295, 3, 25, 8, 200, 2, 206, 4294967295, 8, 25, 8, 202, 1, 4294967295, 
    4294967295, 1, 25, 0, 203, 0, 4294967295, 4294967295, 1, 52, 0, 0, 0, 
    1, 61, 0, 0, 0, 1, 72, 0, 0, 0, 1, 76, 0, 0, 0, 1, 80, 0, 0, 0, 1, 92, 
    0, 0, 0, 1, 107, 0, 0, 0, 1, 117, 0, 0, 0, 1, 121, 0, 0, 0, 1, 130, 
    0, 0, 0, 1, 133, 0, 0, 0, 1, 137, 0, 0, 0, 1, 140, 0, 0, 0, 1, 144, 
    0, 0, 0, 1, 148, 0, 0, 0, 1, 152, 0, 0, 0, 1, 157, 0, 0, 0, 1, 159, 
    0, 0, 0, 1, 162, 0, 0, 0, 1, 166, 0, 0, 0, 1, 172, 0, 0, 0, 1, 185, 
    0, 0, 0, 1, 187, 0, 0, 0, 1, 194, 0, 0, 0, 1, 199, 0, 0, 0, 1, 202, 
    0, 0, 0, 5, 56, 7, 0, 0, 5, 55, 10, 0, 0, 1, 53, 0, 0, 0, 1, 58, 0, 
    0, 0, 1, 54, 0, 0, 0, 1, 57, 0, 0, 0, 1, 59, 0, 0, 0, 1, 56, 0, 0, 0, 
    5, 60, 8, 0, 0, 1, 1, 0, 0, 0, 5, 63, 7, 0, 0, 5, 64, 10, 0, 0, 1, 62, 
    0, 0, 0, 1, 65, 0, 0, 0, 1, 63, 0, 0, 0, 1, 66, 0, 0, 0, 1, 67, 0, 0, 
    0, 5, 68, 8, 0, 0, 1, 3, 0, 0, 0, 5, 71, 10, 0, 0, 1, 69, 0, 0, 0, 1, 
    74, 0, 0, 0, 1, 70, 0, 0, 0, 1, 73, 0, 0, 0, 1, 5, 0, 0, 0, 1, 72, 0, 
    0, 0, 5, 77, 10, 0, 0, 1, 75, 0, 0, 0, 1, 78, 0, 0, 0, 1, 76, 0, 0, 
    0, 1, 79, 0, 0, 0, 1, 7, 0, 0, 0, 5, 85, 7, 0, 0, 5, 82, 1, 0, 0, 5, 
    84, 2, 0, 0, 1, 81, 0, 0, 0, 1, 87, 0, 0, 0, 1, 83, 0, 0, 0, 1, 86, 
    0, 0, 0, 1, 88, 0, 0, 0, 1, 85, 0, 0, 0, 5, 89, 1, 0, 0, 5, 90, 3, 0, 
    0, 5, 91, 8, 0, 0, 1, 9, 0, 0, 0, 5, 95, 7, 0, 0, 5, 94, 1, 0, 0, 5, 
    96, 2, 0, 0, 1, 93, 0, 0, 0, 1, 97, 0, 0, 0, 1, 95, 0, 0, 0, 1, 98, 
    0, 0, 0, 1, 99, 0, 0, 0, 5, 100, 1, 0, 0, 5, 101, 3, 0, 0, 5, 102, 8, 
    0, 0, 1, 11, 0, 0, 0, 5, 104, 1, 0, 0, 5, 108, 10, 0, 0, 5, 106, 2, 
    0, 0, 5, 108, 10, 0, 0, 1, 103, 0, 0, 0, 1, 105, 0, 0, 0, 1, 109, 0, 
    0, 0, 5, 110, 9, 0, 0, 1, 13, 0, 0, 0, 5, 112, 1, 0, 0, 5, 118, 2, 0, 
    0, 5, 114, 1, 0, 0, 5, 118, 3, 0, 0, 5, 116, 4, 0, 0, 5, 118, 5, 0, 
    0, 1, 111, 0, 0, 0, 1, 113, 0, 0, 0, 1, 115, 0, 0, 0, 1, 119, 0, 0, 
    0, 5, 120, 9, 0, 0, 1, 15, 0, 0, 0, 5, 124, 7, 0, 0, 5, 123, 1, 0, 0, 
    5, 125, 2, 0, 0, 1, 122, 0, 0, 0, 1, 125, 0, 0, 0, 1, 126, 0, 0, 0, 
    5, 127, 1, 0, 0, 5, 128, 3, 0, 0, 5, 129, 8, 0, 0, 1, 17, 0, 0, 0, 3, 
    20, 10, 131, 0, 5, 132, 9, 0, 0, 1, 19, 0, 0, 0, 5, 135, 1, 0, 0, 5, 
    136, 2, 0, 0, 1, 134, 0, 0, 0, 1, 136, 0, 0, 0, 1, 21, 0, 0, 0, 3, 24, 
    12, 138, 0, 5, 139, 9, 0, 0, 1, 23, 0, 0, 0, 3, 26, 13, 142, 0, 5, 143, 
    2, 0, 0, 1, 141, 0, 0, 0, 1, 143, 0, 0, 0, 1, 25, 0, 0, 0, 5, 146, 1, 
    0, 0, 5, 147, 3, 0, 0, 1, 145, 0, 0, 0, 1, 147, 0, 0, 0, 1, 27, 0, 0, 
    0, 3, 30, 15, 149, 0, 3, 32, 16, 150, 0, 5, 151, 9, 0, 0, 1, 29, 0, 
    0, 0, 5, 154, 1, 0, 0, 5, 155, 2, 0, 0, 1, 153, 0, 0, 0, 1, 155, 0, 
    0, 0, 1, 31, 0, 0, 0, 5, 158, 3, 0, 0, 1, 156, 0, 0, 0, 1, 158, 0, 0, 
    0, 1, 33, 0, 0, 0, 3, 32, 16, 160, 0, 5, 161, 9, 0, 0, 1, 35, 0, 0, 
    0, 3, 20, 10, 163, 0, 5, 164, 4, 0, 0, 5, 165, 9, 0, 0, 1, 37, 0, 0, 
    0, 5, 168, 1, 0, 0, 5, 169, 2, 0, 0, 1, 167, 0, 0, 0, 1, 169, 0, 0, 
    0, 1, 170, 0, 0, 0, 5, 171, 9, 0, 0, 1, 39, 0, 0, 0, 3, 42, 21, 173, 
    0, 5, 174, 9, 0, 0, 1, 41, 0, 0, 0, 5, 176, 4, 0, 0, 5, 178, 1, 0, 0, 
    5, 179, 2, 0, 0, 1, 177, 0, 0, 0, 1, 179, 0, 0, 0, 1, 186, 0, 0, 0, 
    5, 181, 5, 0, 0, 5, 183, 1, 0, 0, 5, 184, 3, 0, 0, 1, 182, 0, 0, 0, 
    1, 184, 0, 0, 0, 1, 186, 0, 0, 0, 1, 175, 0, 0, 0, 1, 180, 0, 0, 0, 
    1, 43, 0, 0, 0, 3, 46, 23, 188, 0, 5, 189, 9, 0, 0, 1, 45, 0, 0, 0, 
    5, 191, 4, 0, 0, 5, 195, 1, 0, 0, 5, 193, 5, 0, 0, 5, 195, 1, 0, 0, 
    1, 190, 0, 0, 0, 1, 192, 0, 0, 0, 1, 197, 0, 0, 0, 5, 198, 2, 0, 0, 
    1, 196, 0, 0, 0, 1, 198, 0, 0, 0, 1, 47, 0, 0, 0, 3, 50, 25, 200, 0, 
    5, 201, 9, 0, 0, 1, 49, 0, 0, 0, 5, 203, 4, 0, 0, 5, 205, 1, 0, 0, 5, 
    206, 2, 0, 0, 1, 204, 0, 0, 0, 1, 206, 0, 0, 0, 1, 51, 0, 0, 0, 56, 
    65, 72, 78, 85, 97, 107, 117, 124, 135, 142, 146, 154, 157, 168, 178, 
    183, 185, 194, 197, 205, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 
    24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 1, 3, 5, 7, 
    9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 
    45, 47, 49, 51
];