// Generated from WidenedTrust.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","8");
pub const WidenedTrust_A:i32=1; 
pub const WidenedTrust_B:i32=2; 
pub const WidenedTrust_C:i32=3; 
pub const WidenedTrust_D:i32=4; 
pub const WidenedTrust_E:i32=5; 
pub const WidenedTrust_M:i32=6; 
pub const WidenedTrust_N:i32=7; 
pub const WidenedTrust_P:i32=8; 
pub const WidenedTrust_Q:i32=9; 
pub const WidenedTrust_W:i32=10; 
pub const WidenedTrust_X:i32=11; 
pub const WidenedTrust_Y:i32=12; 
pub const WidenedTrust_Z:i32=13; 
pub const WidenedTrust_U0:i32=14; 
pub const WidenedTrust_V0:i32=15; 
pub const WidenedTrust_U1:i32=16; 
pub const WidenedTrust_V1:i32=17; 
pub const WidenedTrust_U2:i32=18; 
pub const WidenedTrust_V2:i32=19; 
pub const WidenedTrust_U3:i32=20; 
pub const WidenedTrust_V3:i32=21; 
pub const WidenedTrust_U4:i32=22; 
pub const WidenedTrust_V4:i32=23; 
pub const WidenedTrust_U5:i32=24; 
pub const WidenedTrust_V5:i32=25; 
pub const WidenedTrust_U6:i32=26; 
pub const WidenedTrust_V6:i32=27; 
pub const WidenedTrust_U7:i32=28; 
pub const WidenedTrust_V7:i32=29; 
pub const WidenedTrust_U8:i32=30; 
pub const WidenedTrust_V8:i32=31; 
pub const WidenedTrust_U9:i32=32; 
pub const WidenedTrust_V9:i32=33; 
pub const WidenedTrust_WS:i32=34;
pub const WidenedTrust_EOF:i32=EOF;
pub const RULE_top:usize = 0; 
pub const RULE_a:usize = 1; 
pub const RULE_r0:usize = 2; 
pub const RULE_r1:usize = 3; 
pub const RULE_r2:usize = 4; 
pub const RULE_r3:usize = 5; 
pub const RULE_r4:usize = 6; 
pub const RULE_r5:usize = 7; 
pub const RULE_r6:usize = 8; 
pub const RULE_r7:usize = 9; 
pub const RULE_r8:usize = 10; 
pub const RULE_r9:usize = 11; 
pub const RULE_r10:usize = 12;
pub const ruleNames: [&'static str; 13] = [
    "top", "a", "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", 
    "r10"
];

pub const _LITERAL_NAMES: [Option<&'static str>;34] = [
	None, Some("'a'"), Some("'b'"), Some("'c'"), Some("'d'"), Some("'e'"), 
	Some("'m'"), Some("'n'"), Some("'p'"), Some("'q'"), Some("'w'"), Some("'x'"), 
	Some("'y'"), Some("'z'"), Some("'u0'"), Some("'v0'"), Some("'u1'"), Some("'v1'"), 
	Some("'u2'"), Some("'v2'"), Some("'u3'"), Some("'v3'"), Some("'u4'"), Some("'v4'"), 
	Some("'u5'"), Some("'v5'"), Some("'u6'"), Some("'v6'"), Some("'u7'"), Some("'v7'"), 
	Some("'u8'"), Some("'v8'"), Some("'u9'"), Some("'v9'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;35]  = [
	None, Some("A"), Some("B"), Some("C"), Some("D"), Some("E"), Some("M"), 
	Some("N"), Some("P"), Some("Q"), Some("W"), Some("X"), Some("Y"), Some("Z"), 
	Some("U0"), Some("V0"), Some("U1"), Some("V1"), Some("U2"), Some("V2"), 
	Some("U3"), Some("V3"), Some("U4"), Some("V4"), Some("U5"), Some("V5"), 
	Some("U6"), Some("V6"), Some("U7"), Some("V7"), Some("U8"), Some("V8"), 
	Some("U9"), Some("V9"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, WidenedTrustParserExt<'input, 'arena>, WidenedTrustParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				WidenedTrustParserExt {
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
        L: ParseTreeListener<'arena, WidenedTrustParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct WidenedTrustTreeWalker;
impl WidenedTrustTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, WidenedTrustParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, WidenedTrustParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <WidenedTrustParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum WidenedTrustParserNodeKind {
    TopContext,
    AContext,
    R0Context,
    R1Context,
    R2Context,
    R3Context,
    R4Context,
    R5Context,
    R6Context,
    R7Context,
    R8Context,
    R9Context,
    R10Context,
    Terminal,
    Error,
}
pub type WidenedTrustParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, WidenedTrustParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => WidenedTrustParser }
dbt_antlr_runtime::impl_node_kind! { WidenedTrustParserNodeKind {
; TopContext(), AContext(), R0Context(), R1Context(), R2Context(), R3Context(), R4Context(), R5Context(), R6Context(), R7Context(), R8Context(), R9Context(), R10Context(), 
    }; 
}

pub struct WidenedTrustParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> WidenedTrustParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { WidenedTrustParserExt, WidenedTrustParserNodeKind, "WidenedTrust.g4" }

//------------------- top ----------------
pub type TopContextAll<'input, 'arena, Tok = CommonToken<'input>> = TopContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, TopContext, TopContextExt, RULE_top }

pub trait TopContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token X
    /// Returns `None` if there is no child corresponding to token X
    fn X(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Y
    /// Returns `None` if there is no child corresponding to token Y
    fn Y(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
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
    fn X(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_X)
    }
    /// Retrieves first TerminalNode corresponding to token Y
    /// Returns `None` if there is no child corresponding to token Y
    fn Y(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Y)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn top(&mut self,) -> Result<&'arena TopContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, TopContext<TF::Tok>, RULE_top, 0, |_parentctx| TopContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena TopContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(32);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(0,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(26);
				recog.a()?;
				recog.base.set_state(27);
				recog.base.match_token(WidenedTrust_X,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(29);
				recog.a()?;
				recog.base.set_state(30);
				recog.base.match_token(WidenedTrust_Y,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- a ----------------
pub type AContextAll<'input, 'arena, Tok = CommonToken<'input>> = AContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, AContext, AContextExt, RULE_a }

pub trait AContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token B in current rule
    fn B_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token B, starting from 0.
    /// Returns `None` if number of children corresponding to token B is less than or equal to `i`.
    fn B(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Z
    /// Returns `None` if there is no child corresponding to token Z
    fn Z(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r0(&self) -> Option<&'arena R0ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token M
    /// Returns `None` if there is no child corresponding to token M
    fn M(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token W
    /// Returns `None` if there is no child corresponding to token W
    fn W(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token N
    /// Returns `None` if there is no child corresponding to token N
    fn N(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token P in current rule
    fn P_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token P, starting from 0.
    /// Returns `None` if number of children corresponding to token P is less than or equal to `i`.
    fn P(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token Q in current rule
    fn Q_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token Q, starting from 0.
    /// Returns `None` if number of children corresponding to token Q is less than or equal to `i`.
    fn Q(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AContextAttrs<'input, 'arena, Tok> for AContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token D
    /// Returns `None` if there is no child corresponding to token D
    fn D(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_D)
    }
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_A)
    }
    /// Retrieves first TerminalNode corresponding to token C
    /// Returns `None` if there is no child corresponding to token C
    fn C(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_C)
    }
    /// Retrieves all `TerminalNode`s corresponding to token B in current rule
    fn B_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == WidenedTrust_B).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token B, starting from 0.
    /// Returns `None` if number of children corresponding to token B is less than or equal to `i`.
    fn B(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == WidenedTrust_B).nth(i)
    }
    fn a(&self) -> Option<&'arena AContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token E
    /// Returns `None` if there is no child corresponding to token E
    fn E(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_E)
    }
    /// Retrieves first TerminalNode corresponding to token Z
    /// Returns `None` if there is no child corresponding to token Z
    fn Z(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Z)
    }
    fn r0(&self) -> Option<&'arena R0ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token M
    /// Returns `None` if there is no child corresponding to token M
    fn M(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_M)
    }
    /// Retrieves first TerminalNode corresponding to token W
    /// Returns `None` if there is no child corresponding to token W
    fn W(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_W)
    }
    /// Retrieves first TerminalNode corresponding to token N
    /// Returns `None` if there is no child corresponding to token N
    fn N(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_N)
    }
    /// Retrieves all `TerminalNode`s corresponding to token P in current rule
    fn P_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == WidenedTrust_P).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token P, starting from 0.
    /// Returns `None` if number of children corresponding to token P is less than or equal to `i`.
    fn P(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == WidenedTrust_P).nth(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token Q in current rule
    fn Q_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == WidenedTrust_Q).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token Q, starting from 0.
    /// Returns `None` if number of children corresponding to token Q is less than or equal to `i`.
    fn Q(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == WidenedTrust_Q).nth(i)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
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
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(67);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_A  => {
		        {
		        recog.base.set_state(34);
		        recog.base.match_token(WidenedTrust_A,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_C  => {
		        {
		        recog.base.set_state(35);
		        recog.base.match_token(WidenedTrust_C,&mut recog.err_handler)?;
		        recog.base.set_state(65);
		        recog.err_handler.sync(&mut recog.base)?;
		        match recog.get_interpreter().adaptive_predict(1,&mut recog.base)? {
		        	1 =>{
		        		{
		        		recog.base.set_state(36);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(37);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(38);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(39);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(40);
		        		recog.base.match_token(WidenedTrust_A,&mut recog.err_handler)?;
		        		}
		        	}
		        ,
		        	2 =>{
		        		{
		        		recog.base.set_state(41);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(42);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(43);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(44);
		        		recog.base.match_token(WidenedTrust_B,&mut recog.err_handler)?;
		        		recog.base.set_state(45);
		        		recog.a()?;
		        		recog.base.set_state(46);
		        		recog.base.match_token(WidenedTrust_E,&mut recog.err_handler)?;
		        		}
		        	}
		        ,
		        	3 =>{
		        		{
		        		recog.base.set_state(48);
		        		recog.base.match_token(WidenedTrust_Z,&mut recog.err_handler)?;
		        		recog.base.set_state(49);
		        		recog.r0()?;
		        		recog.base.set_state(50);
		        		recog.base.match_token(WidenedTrust_M,&mut recog.err_handler)?;
		        		}
		        	}
		        ,
		        	4 =>{
		        		{
		        		recog.base.set_state(52);
		        		recog.base.match_token(WidenedTrust_Z,&mut recog.err_handler)?;
		        		recog.base.set_state(53);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(54);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(55);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(56);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(57);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(58);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(59);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(60);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(61);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(62);
		        		_la = recog.base.input.la(1);
		        		if { !(_la==WidenedTrust_P || _la==WidenedTrust_Q) } {
		        			recog.err_handler.recover_inline(&mut recog.base)?;
		        		}
		        		else {
		        			if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        			recog.err_handler.report_match(&mut recog.base);
		        			recog.base.consume(&mut recog.err_handler)?;
		        		}
		        		recog.base.set_state(63);
		        		recog.base.match_token(WidenedTrust_W,&mut recog.err_handler)?;
		        		recog.base.set_state(64);
		        		recog.base.match_token(WidenedTrust_N,&mut recog.err_handler)?;
		        		}
		        	}

		        	_ => {}
		        }
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		recog.base.set_state(69);
		recog.base.match_token(WidenedTrust_D,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- r0 ----------------
pub type R0ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R0Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R0Context, R0ContextExt, RULE_r0 }

pub trait R0ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r1(&self) -> Option<&'arena R1ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U0
    /// Returns `None` if there is no child corresponding to token U0
    fn U0(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V0
    /// Returns `None` if there is no child corresponding to token V0
    fn V0(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R0ContextAttrs<'input, 'arena, Tok> for R0Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r1(&self) -> Option<&'arena R1ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U0
    /// Returns `None` if there is no child corresponding to token U0
    fn U0(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U0)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V0
    /// Returns `None` if there is no child corresponding to token V0
    fn V0(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V0)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r0(&mut self,) -> Result<&'arena R0ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R0Context<TF::Tok>, RULE_r0, 4, |_parentctx| R0ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R0Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(79);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(71);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(72);
		        recog.r1()?;
		        recog.base.set_state(73);
		        recog.base.match_token(WidenedTrust_U0,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(75);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(76);
		        recog.r1()?;
		        recog.base.set_state(77);
		        recog.base.match_token(WidenedTrust_V0,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r1 ----------------
pub type R1ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R1Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R1Context, R1ContextExt, RULE_r1 }

pub trait R1ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r2(&self) -> Option<&'arena R2ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U1
    /// Returns `None` if there is no child corresponding to token U1
    fn U1(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V1
    /// Returns `None` if there is no child corresponding to token V1
    fn V1(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R1ContextAttrs<'input, 'arena, Tok> for R1Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r2(&self) -> Option<&'arena R2ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U1
    /// Returns `None` if there is no child corresponding to token U1
    fn U1(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U1)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V1
    /// Returns `None` if there is no child corresponding to token V1
    fn V1(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V1)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r1(&mut self,) -> Result<&'arena R1ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R1Context<TF::Tok>, RULE_r1, 6, |_parentctx| R1ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R1Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(89);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(81);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(82);
		        recog.r2()?;
		        recog.base.set_state(83);
		        recog.base.match_token(WidenedTrust_U1,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(85);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(86);
		        recog.r2()?;
		        recog.base.set_state(87);
		        recog.base.match_token(WidenedTrust_V1,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r2 ----------------
pub type R2ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R2Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R2Context, R2ContextExt, RULE_r2 }

pub trait R2ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r3(&self) -> Option<&'arena R3ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U2
    /// Returns `None` if there is no child corresponding to token U2
    fn U2(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V2
    /// Returns `None` if there is no child corresponding to token V2
    fn V2(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R2ContextAttrs<'input, 'arena, Tok> for R2Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r3(&self) -> Option<&'arena R3ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U2
    /// Returns `None` if there is no child corresponding to token U2
    fn U2(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U2)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V2
    /// Returns `None` if there is no child corresponding to token V2
    fn V2(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V2)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r2(&mut self,) -> Result<&'arena R2ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R2Context<TF::Tok>, RULE_r2, 8, |_parentctx| R2ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R2Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(99);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(91);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(92);
		        recog.r3()?;
		        recog.base.set_state(93);
		        recog.base.match_token(WidenedTrust_U2,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(95);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(96);
		        recog.r3()?;
		        recog.base.set_state(97);
		        recog.base.match_token(WidenedTrust_V2,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r3 ----------------
pub type R3ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R3Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R3Context, R3ContextExt, RULE_r3 }

pub trait R3ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r4(&self) -> Option<&'arena R4ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U3
    /// Returns `None` if there is no child corresponding to token U3
    fn U3(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V3
    /// Returns `None` if there is no child corresponding to token V3
    fn V3(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R3ContextAttrs<'input, 'arena, Tok> for R3Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r4(&self) -> Option<&'arena R4ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U3
    /// Returns `None` if there is no child corresponding to token U3
    fn U3(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U3)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V3
    /// Returns `None` if there is no child corresponding to token V3
    fn V3(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V3)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r3(&mut self,) -> Result<&'arena R3ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R3Context<TF::Tok>, RULE_r3, 10, |_parentctx| R3ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R3Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(109);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(101);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(102);
		        recog.r4()?;
		        recog.base.set_state(103);
		        recog.base.match_token(WidenedTrust_U3,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(105);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(106);
		        recog.r4()?;
		        recog.base.set_state(107);
		        recog.base.match_token(WidenedTrust_V3,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r4 ----------------
pub type R4ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R4Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R4Context, R4ContextExt, RULE_r4 }

pub trait R4ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r5(&self) -> Option<&'arena R5ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U4
    /// Returns `None` if there is no child corresponding to token U4
    fn U4(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V4
    /// Returns `None` if there is no child corresponding to token V4
    fn V4(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R4ContextAttrs<'input, 'arena, Tok> for R4Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r5(&self) -> Option<&'arena R5ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U4
    /// Returns `None` if there is no child corresponding to token U4
    fn U4(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U4)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V4
    /// Returns `None` if there is no child corresponding to token V4
    fn V4(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V4)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r4(&mut self,) -> Result<&'arena R4ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R4Context<TF::Tok>, RULE_r4, 12, |_parentctx| R4ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R4Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(119);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(111);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(112);
		        recog.r5()?;
		        recog.base.set_state(113);
		        recog.base.match_token(WidenedTrust_U4,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(115);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(116);
		        recog.r5()?;
		        recog.base.set_state(117);
		        recog.base.match_token(WidenedTrust_V4,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r5 ----------------
pub type R5ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R5Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R5Context, R5ContextExt, RULE_r5 }

pub trait R5ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r6(&self) -> Option<&'arena R6ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U5
    /// Returns `None` if there is no child corresponding to token U5
    fn U5(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V5
    /// Returns `None` if there is no child corresponding to token V5
    fn V5(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R5ContextAttrs<'input, 'arena, Tok> for R5Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r6(&self) -> Option<&'arena R6ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U5
    /// Returns `None` if there is no child corresponding to token U5
    fn U5(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U5)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V5
    /// Returns `None` if there is no child corresponding to token V5
    fn V5(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V5)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r5(&mut self,) -> Result<&'arena R5ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R5Context<TF::Tok>, RULE_r5, 14, |_parentctx| R5ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R5Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(129);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(121);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(122);
		        recog.r6()?;
		        recog.base.set_state(123);
		        recog.base.match_token(WidenedTrust_U5,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(125);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(126);
		        recog.r6()?;
		        recog.base.set_state(127);
		        recog.base.match_token(WidenedTrust_V5,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r6 ----------------
pub type R6ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R6Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R6Context, R6ContextExt, RULE_r6 }

pub trait R6ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r7(&self) -> Option<&'arena R7ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U6
    /// Returns `None` if there is no child corresponding to token U6
    fn U6(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V6
    /// Returns `None` if there is no child corresponding to token V6
    fn V6(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R6ContextAttrs<'input, 'arena, Tok> for R6Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r7(&self) -> Option<&'arena R7ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U6
    /// Returns `None` if there is no child corresponding to token U6
    fn U6(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U6)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V6
    /// Returns `None` if there is no child corresponding to token V6
    fn V6(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V6)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r6(&mut self,) -> Result<&'arena R6ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R6Context<TF::Tok>, RULE_r6, 16, |_parentctx| R6ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R6Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(139);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(131);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(132);
		        recog.r7()?;
		        recog.base.set_state(133);
		        recog.base.match_token(WidenedTrust_U6,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(135);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(136);
		        recog.r7()?;
		        recog.base.set_state(137);
		        recog.base.match_token(WidenedTrust_V6,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r7 ----------------
pub type R7ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R7Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R7Context, R7ContextExt, RULE_r7 }

pub trait R7ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r8(&self) -> Option<&'arena R8ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U7
    /// Returns `None` if there is no child corresponding to token U7
    fn U7(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V7
    /// Returns `None` if there is no child corresponding to token V7
    fn V7(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R7ContextAttrs<'input, 'arena, Tok> for R7Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r8(&self) -> Option<&'arena R8ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U7
    /// Returns `None` if there is no child corresponding to token U7
    fn U7(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U7)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V7
    /// Returns `None` if there is no child corresponding to token V7
    fn V7(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V7)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r7(&mut self,) -> Result<&'arena R7ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R7Context<TF::Tok>, RULE_r7, 18, |_parentctx| R7ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R7Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(149);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(141);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(142);
		        recog.r8()?;
		        recog.base.set_state(143);
		        recog.base.match_token(WidenedTrust_U7,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(145);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(146);
		        recog.r8()?;
		        recog.base.set_state(147);
		        recog.base.match_token(WidenedTrust_V7,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r8 ----------------
pub type R8ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R8Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R8Context, R8ContextExt, RULE_r8 }

pub trait R8ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r9(&self) -> Option<&'arena R9ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U8
    /// Returns `None` if there is no child corresponding to token U8
    fn U8(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V8
    /// Returns `None` if there is no child corresponding to token V8
    fn V8(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R8ContextAttrs<'input, 'arena, Tok> for R8Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r9(&self) -> Option<&'arena R9ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U8
    /// Returns `None` if there is no child corresponding to token U8
    fn U8(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U8)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V8
    /// Returns `None` if there is no child corresponding to token V8
    fn V8(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V8)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r8(&mut self,) -> Result<&'arena R8ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R8Context<TF::Tok>, RULE_r8, 20, |_parentctx| R8ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R8Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(159);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(151);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(152);
		        recog.r9()?;
		        recog.base.set_state(153);
		        recog.base.match_token(WidenedTrust_U8,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(155);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(156);
		        recog.r9()?;
		        recog.base.set_state(157);
		        recog.base.match_token(WidenedTrust_V8,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r9 ----------------
pub type R9ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R9Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R9Context, R9ContextExt, RULE_r9 }

pub trait R9ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn r10(&self) -> Option<&'arena R10ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token U9
    /// Returns `None` if there is no child corresponding to token U9
    fn U9(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token V9
    /// Returns `None` if there is no child corresponding to token V9
    fn V9(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R9ContextAttrs<'input, 'arena, Tok> for R9Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token P
    /// Returns `None` if there is no child corresponding to token P
    fn P(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_P)
    }
    fn r10(&self) -> Option<&'arena R10ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token U9
    /// Returns `None` if there is no child corresponding to token U9
    fn U9(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_U9)
    }
    /// Retrieves first TerminalNode corresponding to token Q
    /// Returns `None` if there is no child corresponding to token Q
    fn Q(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_Q)
    }
    /// Retrieves first TerminalNode corresponding to token V9
    /// Returns `None` if there is no child corresponding to token V9
    fn V9(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_V9)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r9(&mut self,) -> Result<&'arena R9ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R9Context<TF::Tok>, RULE_r9, 22, |_parentctx| R9ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R9Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(169);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    WidenedTrust_P  => {
		        /*------- Outer Most Alt 1 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		        {
		        recog.base.set_state(161);
		        recog.base.match_token(WidenedTrust_P,&mut recog.err_handler)?;
		        recog.base.set_state(162);
		        recog.r10()?;
		        recog.base.set_state(163);
		        recog.base.match_token(WidenedTrust_U9,&mut recog.err_handler)?;
		        }}
		    WidenedTrust_Q  => {
		        /*------- Outer Most Alt 2 -------*/
		        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
		        {
		        recog.base.set_state(165);
		        recog.base.match_token(WidenedTrust_Q,&mut recog.err_handler)?;
		        recog.base.set_state(166);
		        recog.r10()?;
		        recog.base.set_state(167);
		        recog.base.match_token(WidenedTrust_V9,&mut recog.err_handler)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		})
	}
}
//------------------- r10 ----------------
pub type R10ContextAll<'input, 'arena, Tok = CommonToken<'input>> = R10Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { WidenedTrustParserNodeKind, R10Context, R10ContextExt, RULE_r10 }

pub trait R10ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token W
    /// Returns `None` if there is no child corresponding to token W
    fn W(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> R10ContextAttrs<'input, 'arena, Tok> for R10Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token W
    /// Returns `None` if there is no child corresponding to token W
    fn W(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == WidenedTrust_W)
    }
}

impl<'input, 'arena, Input, TF> WidenedTrustParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn r10(&mut self,) -> Result<&'arena R10ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, R10Context<TF::Tok>, RULE_r10, 24, |_parentctx| R10ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena R10Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(171);
		recog.base.match_token(WidenedTrust_W,&mut recog.err_handler)?;
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
static _serializedATN: [u32; 2172] = [
    1346458702, 3, 16909060, 29, 34, 174, 175, 1, 1, 13, 13, 29, 1218, 1247, 
    875, 2122, 5, 2127, 2, 2133, 13, 2146, 13, 2159, 13, 2172, 2, 2129, 
    4, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 
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
    1, 0, 8, 13, 1, 4294967295, 4294967295, 1, 0, 32, 14, 1, 4294967295, 
    4294967295, 1, 0, 8, 15, 1, 4294967295, 4294967295, 1, 0, 8, 16, 1, 
    4294967295, 4294967295, 1, 0, 32, 17, 1, 4294967295, 4294967295, 1, 
    0, 8, 18, 1, 4294967295, 4294967295, 3, 0, 8, 19, 2, 33, 4294967295, 
    8, 0, 8, 21, 1, 4294967295, 4294967295, 1, 1, 32, 22, 1, 4294967295, 
    4294967295, 1, 1, 32, 23, 1, 4294967295, 4294967295, 1, 1, 32, 24, 1, 
    4294967295, 4294967295, 1, 1, 32, 25, 1, 4294967295, 4294967295, 1, 
    1, 32, 26, 1, 4294967295, 4294967295, 1, 1, 32, 27, 1, 4294967295, 4294967295, 
    1, 1, 32, 28, 1, 4294967295, 4294967295, 1, 1, 32, 29, 1, 4294967295, 
    4294967295, 1, 1, 32, 30, 1, 4294967295, 4294967295, 1, 1, 32, 31, 1, 
    4294967295, 4294967295, 1, 1, 32, 32, 1, 4294967295, 4294967295, 1, 
    1, 8, 33, 1, 4294967295, 4294967295, 1, 1, 32, 34, 1, 4294967295, 4294967295, 
    1, 1, 8, 35, 1, 4294967295, 4294967295, 1, 1, 32, 36, 1, 4294967295, 
    4294967295, 1, 1, 8, 37, 1, 4294967295, 4294967295, 1, 1, 32, 38, 1, 
    4294967295, 4294967295, 1, 1, 8, 39, 1, 4294967295, 4294967295, 1, 1, 
    32, 40, 1, 4294967295, 4294967295, 1, 1, 32, 41, 1, 4294967295, 4294967295, 
    1, 1, 32, 42, 1, 4294967295, 4294967295, 1, 1, 32, 43, 1, 4294967295, 
    4294967295, 1, 1, 32, 44, 1, 4294967295, 4294967295, 1, 1, 32, 45, 1, 
    4294967295, 4294967295, 1, 1, 32, 46, 1, 4294967295, 4294967295, 1, 
    1, 32, 47, 1, 4294967295, 4294967295, 1, 1, 32, 48, 1, 4294967295, 4294967295, 
    1, 1, 32, 49, 1, 4294967295, 4294967295, 1, 1, 32, 50, 1, 4294967295, 
    4294967295, 1, 1, 32, 51, 1, 4294967295, 4294967295, 1, 1, 32, 52, 1, 
    4294967295, 4294967295, 3, 1, 8, 53, 4, 66, 4294967295, 8, 1, 8, 57, 
    1, 4294967295, 4294967295, 3, 1, 8, 58, 2, 68, 4294967295, 8, 1, 8, 
    60, 1, 4294967295, 4294967295, 1, 1, 32, 61, 1, 4294967295, 4294967295, 
    1, 1, 8, 62, 1, 4294967295, 4294967295, 1, 2, 32, 63, 1, 4294967295, 
    4294967295, 1, 2, 8, 64, 1, 4294967295, 4294967295, 1, 2, 32, 65, 1, 
    4294967295, 4294967295, 1, 2, 8, 66, 1, 4294967295, 4294967295, 1, 2, 
    32, 67, 1, 4294967295, 4294967295, 1, 2, 8, 68, 1, 4294967295, 4294967295, 
    1, 2, 32, 69, 1, 4294967295, 4294967295, 1, 2, 8, 70, 1, 4294967295, 
    4294967295, 3, 2, 8, 71, 2, 80, 4294967295, 8, 2, 8, 73, 1, 4294967295, 
    4294967295, 1, 3, 32, 74, 1, 4294967295, 4294967295, 1, 3, 8, 75, 1, 
    4294967295, 4294967295, 1, 3, 32, 76, 1, 4294967295, 4294967295, 1, 
    3, 8, 77, 1, 4294967295, 4294967295, 1, 3, 32, 78, 1, 4294967295, 4294967295, 
    1, 3, 8, 79, 1, 4294967295, 4294967295, 1, 3, 32, 80, 1, 4294967295, 
    4294967295, 1, 3, 8, 81, 1, 4294967295, 4294967295, 3, 3, 8, 82, 2, 
    90, 4294967295, 8, 3, 8, 84, 1, 4294967295, 4294967295, 1, 4, 32, 85, 
    1, 4294967295, 4294967295, 1, 4, 8, 86, 1, 4294967295, 4294967295, 1, 
    4, 32, 87, 1, 4294967295, 4294967295, 1, 4, 8, 88, 1, 4294967295, 4294967295, 
    1, 4, 32, 89, 1, 4294967295, 4294967295, 1, 4, 8, 90, 1, 4294967295, 
    4294967295, 1, 4, 32, 91, 1, 4294967295, 4294967295, 1, 4, 8, 92, 1, 
    4294967295, 4294967295, 3, 4, 8, 93, 2, 100, 4294967295, 8, 4, 8, 95, 
    1, 4294967295, 4294967295, 1, 5, 32, 96, 1, 4294967295, 4294967295, 
    1, 5, 8, 97, 1, 4294967295, 4294967295, 1, 5, 32, 98, 1, 4294967295, 
    4294967295, 1, 5, 8, 99, 1, 4294967295, 4294967295, 1, 5, 32, 100, 1, 
    4294967295, 4294967295, 1, 5, 8, 101, 1, 4294967295, 4294967295, 1, 
    5, 32, 102, 1, 4294967295, 4294967295, 1, 5, 8, 103, 1, 4294967295, 
    4294967295, 3, 5, 8, 104, 2, 110, 4294967295, 8, 5, 8, 106, 1, 4294967295, 
    4294967295, 1, 6, 32, 107, 1, 4294967295, 4294967295, 1, 6, 8, 108, 
    1, 4294967295, 4294967295, 1, 6, 32, 109, 1, 4294967295, 4294967295, 
    1, 6, 8, 110, 1, 4294967295, 4294967295, 1, 6, 32, 111, 1, 4294967295, 
    4294967295, 1, 6, 8, 112, 1, 4294967295, 4294967295, 1, 6, 32, 113, 
    1, 4294967295, 4294967295, 1, 6, 8, 114, 1, 4294967295, 4294967295, 
    3, 6, 8, 115, 2, 120, 4294967295, 8, 6, 8, 117, 1, 4294967295, 4294967295, 
    1, 7, 32, 118, 1, 4294967295, 4294967295, 1, 7, 8, 119, 1, 4294967295, 
    4294967295, 1, 7, 32, 120, 1, 4294967295, 4294967295, 1, 7, 8, 121, 
    1, 4294967295, 4294967295, 1, 7, 32, 122, 1, 4294967295, 4294967295, 
    1, 7, 8, 123, 1, 4294967295, 4294967295, 1, 7, 32, 124, 1, 4294967295, 
    4294967295, 1, 7, 8, 125, 1, 4294967295, 4294967295, 3, 7, 8, 126, 2, 
    130, 4294967295, 8, 7, 8, 128, 1, 4294967295, 4294967295, 1, 8, 32, 
    129, 1, 4294967295, 4294967295, 1, 8, 8, 130, 1, 4294967295, 4294967295, 
    1, 8, 32, 131, 1, 4294967295, 4294967295, 1, 8, 8, 132, 1, 4294967295, 
    4294967295, 1, 8, 32, 133, 1, 4294967295, 4294967295, 1, 8, 8, 134, 
    1, 4294967295, 4294967295, 1, 8, 32, 135, 1, 4294967295, 4294967295, 
    1, 8, 8, 136, 1, 4294967295, 4294967295, 3, 8, 8, 137, 2, 140, 4294967295, 
    8, 8, 8, 139, 1, 4294967295, 4294967295, 1, 9, 32, 140, 1, 4294967295, 
    4294967295, 1, 9, 8, 141, 1, 4294967295, 4294967295, 1, 9, 32, 142, 
    1, 4294967295, 4294967295, 1, 9, 8, 143, 1, 4294967295, 4294967295, 
    1, 9, 32, 144, 1, 4294967295, 4294967295, 1, 9, 8, 145, 1, 4294967295, 
    4294967295, 1, 9, 32, 146, 1, 4294967295, 4294967295, 1, 9, 8, 147, 
    1, 4294967295, 4294967295, 3, 9, 8, 148, 2, 150, 4294967295, 8, 9, 8, 
    150, 1, 4294967295, 4294967295, 1, 10, 32, 151, 1, 4294967295, 4294967295, 
    1, 10, 8, 152, 1, 4294967295, 4294967295, 1, 10, 32, 153, 1, 4294967295, 
    4294967295, 1, 10, 8, 154, 1, 4294967295, 4294967295, 1, 10, 32, 155, 
    1, 4294967295, 4294967295, 1, 10, 8, 156, 1, 4294967295, 4294967295, 
    1, 10, 32, 157, 1, 4294967295, 4294967295, 1, 10, 8, 158, 1, 4294967295, 
    4294967295, 3, 10, 8, 159, 2, 160, 4294967295, 8, 10, 8, 161, 1, 4294967295, 
    4294967295, 1, 11, 32, 162, 1, 4294967295, 4294967295, 1, 11, 8, 163, 
    1, 4294967295, 4294967295, 1, 11, 32, 164, 1, 4294967295, 4294967295, 
    1, 11, 8, 165, 1, 4294967295, 4294967295, 1, 11, 32, 166, 1, 4294967295, 
    4294967295, 1, 11, 8, 167, 1, 4294967295, 4294967295, 1, 11, 32, 168, 
    1, 4294967295, 4294967295, 1, 11, 8, 169, 1, 4294967295, 4294967295, 
    3, 11, 8, 170, 2, 170, 4294967295, 8, 11, 8, 172, 1, 4294967295, 4294967295, 
    1, 12, 32, 173, 1, 4294967295, 4294967295, 1, 12, 8, 174, 1, 4294967295, 
    4294967295, 1, 12, 0, 175, 0, 4294967295, 4294967295, 1, 32, 0, 0, 0, 
    1, 67, 0, 0, 0, 1, 79, 0, 0, 0, 1, 89, 0, 0, 0, 1, 99, 0, 0, 0, 1, 109, 
    0, 0, 0, 1, 119, 0, 0, 0, 1, 129, 0, 0, 0, 1, 139, 0, 0, 0, 1, 149, 
    0, 0, 0, 1, 159, 0, 0, 0, 1, 169, 0, 0, 0, 1, 171, 0, 0, 0, 3, 2, 1, 
    27, 0, 5, 28, 11, 0, 0, 1, 33, 0, 0, 0, 3, 2, 1, 30, 0, 5, 31, 12, 0, 
    0, 1, 33, 0, 0, 0, 1, 26, 0, 0, 0, 1, 29, 0, 0, 0, 1, 1, 0, 0, 0, 5, 
    68, 1, 0, 0, 5, 65, 3, 0, 0, 5, 37, 2, 0, 0, 5, 38, 2, 0, 0, 5, 39, 
    2, 0, 0, 5, 40, 2, 0, 0, 5, 66, 1, 0, 0, 5, 42, 2, 0, 0, 5, 43, 2, 0, 
    0, 5, 44, 2, 0, 0, 5, 45, 2, 0, 0, 3, 2, 1, 46, 0, 5, 47, 5, 0, 0, 1, 
    66, 0, 0, 0, 5, 49, 13, 0, 0, 3, 4, 2, 50, 0, 5, 51, 6, 0, 0, 1, 66, 
    0, 0, 0, 5, 53, 13, 0, 0, 7, 54, 0, 0, 0, 7, 55, 0, 0, 0, 7, 56, 0, 
    0, 0, 7, 57, 0, 0, 0, 7, 58, 0, 0, 0, 7, 59, 0, 0, 0, 7, 60, 0, 0, 0, 
    7, 61, 0, 0, 0, 7, 62, 0, 0, 0, 7, 63, 0, 0, 0, 5, 64, 10, 0, 0, 5, 
    66, 7, 0, 0, 1, 36, 0, 0, 0, 1, 41, 0, 0, 0, 1, 48, 0, 0, 0, 1, 52, 
    0, 0, 0, 1, 68, 0, 0, 0, 1, 34, 0, 0, 0, 1, 35, 0, 0, 0, 1, 69, 0, 0, 
    0, 5, 70, 4, 0, 0, 1, 3, 0, 0, 0, 5, 72, 8, 0, 0, 3, 6, 3, 73, 0, 5, 
    74, 14, 0, 0, 1, 80, 0, 0, 0, 5, 76, 9, 0, 0, 3, 6, 3, 77, 0, 5, 78, 
    15, 0, 0, 1, 80, 0, 0, 0, 1, 71, 0, 0, 0, 1, 75, 0, 0, 0, 1, 5, 0, 0, 
    0, 5, 82, 8, 0, 0, 3, 8, 4, 83, 0, 5, 84, 16, 0, 0, 1, 90, 0, 0, 0, 
    5, 86, 9, 0, 0, 3, 8, 4, 87, 0, 5, 88, 17, 0, 0, 1, 90, 0, 0, 0, 1, 
    81, 0, 0, 0, 1, 85, 0, 0, 0, 1, 7, 0, 0, 0, 5, 92, 8, 0, 0, 3, 10, 5, 
    93, 0, 5, 94, 18, 0, 0, 1, 100, 0, 0, 0, 5, 96, 9, 0, 0, 3, 10, 5, 97, 
    0, 5, 98, 19, 0, 0, 1, 100, 0, 0, 0, 1, 91, 0, 0, 0, 1, 95, 0, 0, 0, 
    1, 9, 0, 0, 0, 5, 102, 8, 0, 0, 3, 12, 6, 103, 0, 5, 104, 20, 0, 0, 
    1, 110, 0, 0, 0, 5, 106, 9, 0, 0, 3, 12, 6, 107, 0, 5, 108, 21, 0, 0, 
    1, 110, 0, 0, 0, 1, 101, 0, 0, 0, 1, 105, 0, 0, 0, 1, 11, 0, 0, 0, 5, 
    112, 8, 0, 0, 3, 14, 7, 113, 0, 5, 114, 22, 0, 0, 1, 120, 0, 0, 0, 5, 
    116, 9, 0, 0, 3, 14, 7, 117, 0, 5, 118, 23, 0, 0, 1, 120, 0, 0, 0, 1, 
    111, 0, 0, 0, 1, 115, 0, 0, 0, 1, 13, 0, 0, 0, 5, 122, 8, 0, 0, 3, 16, 
    8, 123, 0, 5, 124, 24, 0, 0, 1, 130, 0, 0, 0, 5, 126, 9, 0, 0, 3, 16, 
    8, 127, 0, 5, 128, 25, 0, 0, 1, 130, 0, 0, 0, 1, 121, 0, 0, 0, 1, 125, 
    0, 0, 0, 1, 15, 0, 0, 0, 5, 132, 8, 0, 0, 3, 18, 9, 133, 0, 5, 134, 
    26, 0, 0, 1, 140, 0, 0, 0, 5, 136, 9, 0, 0, 3, 18, 9, 137, 0, 5, 138, 
    27, 0, 0, 1, 140, 0, 0, 0, 1, 131, 0, 0, 0, 1, 135, 0, 0, 0, 1, 17, 
    0, 0, 0, 5, 142, 8, 0, 0, 3, 20, 10, 143, 0, 5, 144, 28, 0, 0, 1, 150, 
    0, 0, 0, 5, 146, 9, 0, 0, 3, 20, 10, 147, 0, 5, 148, 29, 0, 0, 1, 150, 
    0, 0, 0, 1, 141, 0, 0, 0, 1, 145, 0, 0, 0, 1, 19, 0, 0, 0, 5, 152, 8, 
    0, 0, 3, 22, 11, 153, 0, 5, 154, 30, 0, 0, 1, 160, 0, 0, 0, 5, 156, 
    9, 0, 0, 3, 22, 11, 157, 0, 5, 158, 31, 0, 0, 1, 160, 0, 0, 0, 1, 151, 
    0, 0, 0, 1, 155, 0, 0, 0, 1, 21, 0, 0, 0, 5, 162, 8, 0, 0, 3, 24, 12, 
    163, 0, 5, 164, 32, 0, 0, 1, 170, 0, 0, 0, 5, 166, 9, 0, 0, 3, 24, 12, 
    167, 0, 5, 168, 33, 0, 0, 1, 170, 0, 0, 0, 1, 161, 0, 0, 0, 1, 165, 
    0, 0, 0, 1, 23, 0, 0, 0, 5, 172, 10, 0, 0, 1, 25, 0, 0, 0, 0, 1, 1, 
    0, 2, 8, 9, 768, 0, 0, 0, 32, 65, 67, 79, 89, 99, 109, 119, 129, 139, 
    149, 159, 169, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 1, 3, 
    5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25
];