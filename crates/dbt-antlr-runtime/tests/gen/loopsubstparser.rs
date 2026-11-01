// Generated from LoopSubst.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
dbt_antlr_runtime::check_version!("0","6");
pub const LoopSubst_T__0:i32=1; 
pub const LoopSubst_T__1:i32=2; 
pub const LoopSubst_T__2:i32=3; 
pub const LoopSubst_T__3:i32=4; 
pub const LoopSubst_T__4:i32=5; 
pub const LoopSubst_T__5:i32=6; 
pub const LoopSubst_T__6:i32=7; 
pub const LoopSubst_T__7:i32=8; 
pub const LoopSubst_T__8:i32=9; 
pub const LoopSubst_T__9:i32=10; 
pub const LoopSubst_T__10:i32=11; 
pub const LoopSubst_AS:i32=12; 
pub const LoopSubst_CAST:i32=13; 
pub const LoopSubst_TRY_CAST:i32=14; 
pub const LoopSubst_CASE:i32=15; 
pub const LoopSubst_WHEN:i32=16; 
pub const LoopSubst_THEN:i32=17; 
pub const LoopSubst_ELSE:i32=18; 
pub const LoopSubst_END:i32=19; 
pub const LoopSubst_EXTRACT:i32=20; 
pub const LoopSubst_SUBSTRING:i32=21; 
pub const LoopSubst_POSITION:i32=22; 
pub const LoopSubst_ROW:i32=23; 
pub const LoopSubst_TIMESTAMP:i32=24; 
pub const LoopSubst_INTERVAL:i32=25; 
pub const LoopSubst_FROM:i32=26; 
pub const LoopSubst_FOR:i32=27; 
pub const LoopSubst_IN:i32=28; 
pub const LoopSubst_VARYING:i32=29; 
pub const LoopSubst_STRING:i32=30; 
pub const LoopSubst_ID:i32=31; 
pub const LoopSubst_INT:i32=32; 
pub const LoopSubst_WS:i32=33;
pub const LoopSubst_EOF:i32=EOF;
pub const RULE_start:usize = 0; 
pub const RULE_expr:usize = 1; 
pub const RULE_operand:usize = 2; 
pub const RULE_type_:usize = 3; 
pub const RULE_whenClause:usize = 4;
pub const ruleNames: [&'static str; 5] = [
    "start", "expr", "operand", "type_", "whenClause"
];

pub const _LITERAL_NAMES: [Option<&'static str>;30] = [
	None, Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'||'"), 
	Some("'<<'"), Some("'('"), Some("')'"), Some("','"), Some("'.'"), Some("'->'"), 
	Some("'AS'"), Some("'CAST'"), Some("'TRY_CAST'"), Some("'CASE'"), Some("'WHEN'"), 
	Some("'THEN'"), Some("'ELSE'"), Some("'END'"), Some("'EXTRACT'"), Some("'SUBSTRING'"), 
	Some("'POSITION'"), Some("'ROW'"), Some("'TIMESTAMP'"), Some("'INTERVAL'"), 
	Some("'FROM'"), Some("'FOR'"), Some("'IN'"), Some("'VARYING'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;34]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	Some("AS"), Some("CAST"), Some("TRY_CAST"), Some("CASE"), Some("WHEN"), 
	Some("THEN"), Some("ELSE"), Some("END"), Some("EXTRACT"), Some("SUBSTRING"), 
	Some("POSITION"), Some("ROW"), Some("TIMESTAMP"), Some("INTERVAL"), Some("FROM"), 
	Some("FOR"), Some("IN"), Some("VARYING"), Some("STRING"), Some("ID"), Some("INT"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, LoopSubstParserExt<'input, 'arena>, LoopSubstParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				LoopSubstParserExt {
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
        L: ParseTreeListener<'arena, LoopSubstParserNodeKind, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct LoopSubstTreeWalker;
impl LoopSubstTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ParseTreeListener<'arena, LoopSubstParserNodeKind, Tok> + 'static,
        T: NodeInner<'input, 'arena, LoopSubstParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <LoopSubstParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LoopSubstParserNodeKind {
    StartContext,
    ExprContext,
    OperandContext,
    Type_Context,
    WhenClauseContext,
    Terminal,
    Error,
}
pub type LoopSubstParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, LoopSubstParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => LoopSubstParser }
dbt_antlr_runtime::impl_node_kind! { LoopSubstParserNodeKind {
; StartContext(), ExprContext(), OperandContext(), Type_Context(), WhenClauseContext(), 
    }; 
}

pub struct LoopSubstParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> LoopSubstParserExt<'input, 'arena> {
}

dbt_antlr_runtime::impl_parser_recog! { LoopSubstParserExt, LoopSubstParserNodeKind, "LoopSubst.g4"; sempred (LoopSubstParserNode<'input, 'arena, TF::Tok>, LoopSubstParser) { 1 => expr_sempred, } }

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn expr_sempred(_ctx: Option<&'arena ExprContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 4)
		    }
	        1 => {
			recog.precpred(None, 3)
		    }
	        2 => {
			recog.precpred(None, 2)
		    }
	        3 => {
			recog.precpred(None, 1)
		    }
		    _ => true
		}
	}
}
//------------------- start ----------------
pub type StartContextAll<'input, 'arena, Tok = CommonToken<'input>> = StartContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSubstParserNodeKind, StartContext, StartContextExt, RULE_start }

pub trait StartContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StartContextAttrs<'input, 'arena, Tok> for StartContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_EOF)
    }
}

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
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
		recog.base.set_state(10);
		recog.expr_rec(0)?;
		recog.base.set_state(11);
		recog.base.match_token(LoopSubst_EOF,&mut recog.err_handler)?;
		}
		})
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExprContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSubstParserNodeKind, ExprContext, ExprContextExt, RULE_expr }

pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn operand(&self) -> Option<&'arena OperandContextAll<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn operand(&self) -> Option<&'arena OperandContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    #[inline]
	pub fn  expr(&mut self,) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: i32) -> Result<&'arena ExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(rec recog = self, _parentctx, _parentState, ExprContext<TF::Tok>, RULE_expr, 2, |_parentctx| ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); _p; {
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let mut _la: i32 = -1;
        let mut _alt: i32;
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(17);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.base.input.la(1) {
		    LoopSubst_T__6 |LoopSubst_CAST |LoopSubst_TRY_CAST |LoopSubst_CASE |
		    LoopSubst_EXTRACT |LoopSubst_SUBSTRING |LoopSubst_POSITION |LoopSubst_ROW |
		    LoopSubst_TIMESTAMP |LoopSubst_INTERVAL |LoopSubst_ID |LoopSubst_INT  => {
		        {
		        recog.base.set_state(14);
		        recog.operand()?;
		        }}
		    LoopSubst_T__0 |LoopSubst_T__1  => {
		        {
		        recog.base.set_state(15);
		        _la = recog.base.input.la(1);
		        if { !(_la==LoopSubst_T__0 || _la==LoopSubst_T__1) } {
		        	recog.err_handler.recover_inline(&mut recog.base)?;
		        }
		        else {
		        	if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
		        	recog.err_handler.report_match(&mut recog.base);
		        	recog.base.consume(&mut recog.err_handler)?;
		        }
		        recog.base.set_state(16);
		        recog.expr_rec(5)?;
		        }}
			_ => Err(ANTLRError::no_alt(&mut recog.base))?
		}
		let tmp = recog.input.lt(-1);
		recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
		recog.base.set_state(33);
		recog.err_handler.sync(&mut recog.base)?;
		_alt = recog.get_interpreter().adaptive_predict(2,&mut recog.base)?;
		while { _alt!=2 && _alt!=INVALID_ALT } {
			if _alt==1 {
				recog.trigger_exit_rule_event()?;
				{
				recog.base.set_state(31);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.get_interpreter().adaptive_predict(1,&mut recog.base)? {
					1 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(19);
						if !({recog.precpred(None, 4)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
						}
						recog.base.set_state(20);
						_la = recog.base.input.la(1);
						if { !(_la==LoopSubst_T__2 || _la==LoopSubst_T__3) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(21);
						recog.expr_rec(5)?;
						}
					}
				,
					2 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(22);
						if !({recog.precpred(None, 3)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
						}
						recog.base.set_state(23);
						_la = recog.base.input.la(1);
						if { !(_la==LoopSubst_T__0 || _la==LoopSubst_T__1) } {
							recog.err_handler.recover_inline(&mut recog.base)?;
						}
						else {
							if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
							recog.err_handler.report_match(&mut recog.base);
							recog.base.consume(&mut recog.err_handler)?;
						}
						recog.base.set_state(24);
						recog.expr_rec(4)?;
						}
					}
				,
					3 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(25);
						if !({recog.precpred(None, 2)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
						}
						recog.base.set_state(26);
						recog.base.match_token(LoopSubst_T__4,&mut recog.err_handler)?;
						recog.base.set_state(27);
						recog.expr_rec(3)?;
						}
					}
				,
					4 =>{
						{
						/*recRuleAltStartAction*/
						let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
						let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

						recog.base.set_state(28);
						if !({recog.precpred(None, 1)}) {
							Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
						}
						recog.base.set_state(29);
						recog.base.match_token(LoopSubst_T__5,&mut recog.err_handler)?;
						recog.base.set_state(30);
						recog.expr_rec(2)?;
						}
					}

					_ => {}
				}
				} 
			}
			recog.base.set_state(35);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(2,&mut recog.base)?;
		}
		}
		})
	}
}
//------------------- operand ----------------
pub type OperandContextAll<'input, 'arena, Tok = CommonToken<'input>> = OperandContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSubstParserNodeKind, OperandContext, OperandContextExt, RULE_operand }

pub trait OperandContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token CAST
    /// Returns `None` if there is no child corresponding to token CAST
    fn CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn type_(&self) -> Option<&'arena Type_ContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TRY_CAST
    /// Returns `None` if there is no child corresponding to token TRY_CAST
    fn TRY_CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token END
    /// Returns `None` if there is no child corresponding to token END
    fn END(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn whenClause_all(&self) -> Vec<&'arena WhenClauseContextAll<'input, 'arena, Tok>>;
    fn whenClause(&self, i: usize) -> Option<&'arena WhenClauseContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EXTRACT
    /// Returns `None` if there is no child corresponding to token EXTRACT
    fn EXTRACT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SUBSTRING
    /// Returns `None` if there is no child corresponding to token SUBSTRING
    fn SUBSTRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token POSITION
    /// Returns `None` if there is no child corresponding to token POSITION
    fn POSITION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ROW
    /// Returns `None` if there is no child corresponding to token ROW
    fn ROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TIMESTAMP
    /// Returns `None` if there is no child corresponding to token TIMESTAMP
    fn TIMESTAMP(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INTERVAL
    /// Returns `None` if there is no child corresponding to token INTERVAL
    fn INTERVAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> OperandContextAttrs<'input, 'arena, Tok> for OperandContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_INT)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ID in current rule
    fn ID_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == LoopSubst_ID).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ID, starting from 0.
    /// Returns `None` if number of children corresponding to token ID is less than or equal to `i`.
    fn ID(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == LoopSubst_ID).nth(i)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token CAST
    /// Returns `None` if there is no child corresponding to token CAST
    fn CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_CAST)
    }
    /// Retrieves first TerminalNode corresponding to token AS
    /// Returns `None` if there is no child corresponding to token AS
    fn AS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_AS)
    }
    fn type_(&self) -> Option<&'arena Type_ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token TRY_CAST
    /// Returns `None` if there is no child corresponding to token TRY_CAST
    fn TRY_CAST(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_TRY_CAST)
    }
    /// Retrieves first TerminalNode corresponding to token CASE
    /// Returns `None` if there is no child corresponding to token CASE
    fn CASE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_CASE)
    }
    /// Retrieves first TerminalNode corresponding to token END
    /// Returns `None` if there is no child corresponding to token END
    fn END(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_END)
    }
    fn whenClause_all(&self) -> Vec<&'arena WhenClauseContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn whenClause(&self, i: usize) -> Option<&'arena WhenClauseContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ELSE
    /// Returns `None` if there is no child corresponding to token ELSE
    fn ELSE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_ELSE)
    }
    /// Retrieves first TerminalNode corresponding to token EXTRACT
    /// Returns `None` if there is no child corresponding to token EXTRACT
    fn EXTRACT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_EXTRACT)
    }
    /// Retrieves first TerminalNode corresponding to token FROM
    /// Returns `None` if there is no child corresponding to token FROM
    fn FROM(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_FROM)
    }
    /// Retrieves first TerminalNode corresponding to token SUBSTRING
    /// Returns `None` if there is no child corresponding to token SUBSTRING
    fn SUBSTRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_SUBSTRING)
    }
    /// Retrieves first TerminalNode corresponding to token FOR
    /// Returns `None` if there is no child corresponding to token FOR
    fn FOR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_FOR)
    }
    /// Retrieves first TerminalNode corresponding to token POSITION
    /// Returns `None` if there is no child corresponding to token POSITION
    fn POSITION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_POSITION)
    }
    /// Retrieves first TerminalNode corresponding to token IN
    /// Returns `None` if there is no child corresponding to token IN
    fn IN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_IN)
    }
    /// Retrieves first TerminalNode corresponding to token ROW
    /// Returns `None` if there is no child corresponding to token ROW
    fn ROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_ROW)
    }
    /// Retrieves first TerminalNode corresponding to token TIMESTAMP
    /// Returns `None` if there is no child corresponding to token TIMESTAMP
    fn TIMESTAMP(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_TIMESTAMP)
    }
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_STRING)
    }
    /// Retrieves first TerminalNode corresponding to token INTERVAL
    /// Returns `None` if there is no child corresponding to token INTERVAL
    fn INTERVAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_INTERVAL)
    }
}

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn operand(&mut self,) -> Result<&'arena OperandContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, OperandContext<TF::Tok>, RULE_operand, 4, |_parentctx| OperandContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena OperandContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		recog.base.set_state(129);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(9,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(36);
				recog.base.match_token(LoopSubst_INT,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(37);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(38);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(39);
				recog.expr_rec(0)?;
				recog.base.set_state(40);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			4 =>{
				/*------- Outer Most Alt 4 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
				{
				recog.base.set_state(42);
				recog.base.match_token(LoopSubst_CAST,&mut recog.err_handler)?;
				recog.base.set_state(43);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(44);
				recog.expr_rec(0)?;
				recog.base.set_state(45);
				recog.base.match_token(LoopSubst_AS,&mut recog.err_handler)?;
				recog.base.set_state(46);
				recog.type_()?;
				recog.base.set_state(47);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			5 =>{
				/*------- Outer Most Alt 5 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
				{
				recog.base.set_state(49);
				recog.base.match_token(LoopSubst_TRY_CAST,&mut recog.err_handler)?;
				recog.base.set_state(50);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(51);
				recog.expr_rec(0)?;
				recog.base.set_state(52);
				recog.base.match_token(LoopSubst_AS,&mut recog.err_handler)?;
				recog.base.set_state(53);
				recog.type_()?;
				recog.base.set_state(54);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			6 =>{
				/*------- Outer Most Alt 6 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(6); }
				{
				recog.base.set_state(56);
				recog.base.match_token(LoopSubst_CASE,&mut recog.err_handler)?;
				recog.base.set_state(58); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				loop {
					{
					{
					recog.base.set_state(57);
					recog.whenClause()?;
					}
					}
					recog.base.set_state(60); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if !(_la==LoopSubst_WHEN) {break}
				}
				recog.base.set_state(64);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==LoopSubst_ELSE {
					{
					recog.base.set_state(62);
					recog.base.match_token(LoopSubst_ELSE,&mut recog.err_handler)?;
					recog.base.set_state(63);
					recog.expr_rec(0)?;
					}
				}

				recog.base.set_state(66);
				recog.base.match_token(LoopSubst_END,&mut recog.err_handler)?;
				}
			}
		,
			7 =>{
				/*------- Outer Most Alt 7 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(7); }
				{
				recog.base.set_state(68);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				recog.base.set_state(69);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(78);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if ((((_la - 1)) & !0x3f) == 0 && ((1usize << (_la - 1)) & 3254284355) != 0) {
					{
					recog.base.set_state(70);
					recog.expr_rec(0)?;
					recog.base.set_state(75);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==LoopSubst_T__8 {
						{
						{
						recog.base.set_state(71);
						recog.base.match_token(LoopSubst_T__8,&mut recog.err_handler)?;
						recog.base.set_state(72);
						recog.expr_rec(0)?;
						}
						}
						recog.base.set_state(77);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

				recog.base.set_state(80);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			8 =>{
				/*------- Outer Most Alt 8 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(8); }
				{
				recog.base.set_state(81);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				recog.base.set_state(82);
				recog.base.match_token(LoopSubst_T__9,&mut recog.err_handler)?;
				recog.base.set_state(83);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				}
			}
		,
			9 =>{
				/*------- Outer Most Alt 9 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(9); }
				{
				recog.base.set_state(84);
				recog.base.match_token(LoopSubst_EXTRACT,&mut recog.err_handler)?;
				recog.base.set_state(85);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(86);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				recog.base.set_state(87);
				recog.base.match_token(LoopSubst_FROM,&mut recog.err_handler)?;
				recog.base.set_state(88);
				recog.expr_rec(0)?;
				recog.base.set_state(89);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			10 =>{
				/*------- Outer Most Alt 10 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(10); }
				{
				recog.base.set_state(91);
				recog.base.match_token(LoopSubst_SUBSTRING,&mut recog.err_handler)?;
				recog.base.set_state(92);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(93);
				recog.expr_rec(0)?;
				recog.base.set_state(94);
				recog.base.match_token(LoopSubst_FROM,&mut recog.err_handler)?;
				recog.base.set_state(95);
				recog.expr_rec(0)?;
				recog.base.set_state(98);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==LoopSubst_FOR {
					{
					recog.base.set_state(96);
					recog.base.match_token(LoopSubst_FOR,&mut recog.err_handler)?;
					recog.base.set_state(97);
					recog.expr_rec(0)?;
					}
				}

				recog.base.set_state(100);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			11 =>{
				/*------- Outer Most Alt 11 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(11); }
				{
				recog.base.set_state(102);
				recog.base.match_token(LoopSubst_POSITION,&mut recog.err_handler)?;
				recog.base.set_state(103);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(104);
				recog.expr_rec(0)?;
				recog.base.set_state(105);
				recog.base.match_token(LoopSubst_IN,&mut recog.err_handler)?;
				recog.base.set_state(106);
				recog.expr_rec(0)?;
				recog.base.set_state(107);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			12 =>{
				/*------- Outer Most Alt 12 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(12); }
				{
				recog.base.set_state(109);
				recog.base.match_token(LoopSubst_ROW,&mut recog.err_handler)?;
				recog.base.set_state(110);
				recog.base.match_token(LoopSubst_T__6,&mut recog.err_handler)?;
				recog.base.set_state(111);
				recog.expr_rec(0)?;
				recog.base.set_state(116);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==LoopSubst_T__8 {
					{
					{
					recog.base.set_state(112);
					recog.base.match_token(LoopSubst_T__8,&mut recog.err_handler)?;
					recog.base.set_state(113);
					recog.expr_rec(0)?;
					}
					}
					recog.base.set_state(118);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(119);
				recog.base.match_token(LoopSubst_T__7,&mut recog.err_handler)?;
				}
			}
		,
			13 =>{
				/*------- Outer Most Alt 13 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(13); }
				{
				recog.base.set_state(121);
				recog.base.match_token(LoopSubst_TIMESTAMP,&mut recog.err_handler)?;
				recog.base.set_state(122);
				recog.base.match_token(LoopSubst_STRING,&mut recog.err_handler)?;
				}
			}
		,
			14 =>{
				/*------- Outer Most Alt 14 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(14); }
				{
				recog.base.set_state(123);
				recog.base.match_token(LoopSubst_INTERVAL,&mut recog.err_handler)?;
				recog.base.set_state(124);
				recog.base.match_token(LoopSubst_STRING,&mut recog.err_handler)?;
				recog.base.set_state(125);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				}
			}
		,
			15 =>{
				/*------- Outer Most Alt 15 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(15); }
				{
				recog.base.set_state(126);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				recog.base.set_state(127);
				recog.base.match_token(LoopSubst_T__10,&mut recog.err_handler)?;
				recog.base.set_state(128);
				recog.expr_rec(0)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- type_ ----------------
pub type Type_ContextAll<'input, 'arena, Tok = CommonToken<'input>> = Type_Context<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSubstParserNodeKind, Type_Context, Type_ContextExt, RULE_type_ }

pub trait Type_ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token VARYING
    /// Returns `None` if there is no child corresponding to token VARYING
    fn VARYING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Type_ContextAttrs<'input, 'arena, Tok> for Type_Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_ID)
    }
    /// Retrieves first TerminalNode corresponding to token VARYING
    /// Returns `None` if there is no child corresponding to token VARYING
    fn VARYING(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_VARYING)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_INT)
    }
}

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn type_(&mut self,) -> Result<&'arena Type_ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, Type_Context<TF::Tok>, RULE_type_, 6, |_parentctx| Type_ContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena Type_Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		recog.base.set_state(135);
		recog.err_handler.sync(&mut recog.base)?;
		match recog.get_interpreter().adaptive_predict(10,&mut recog.base)? {
			1 =>{
				/*------- Outer Most Alt 1 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
				{
				recog.base.set_state(131);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				}
			}
		,
			2 =>{
				/*------- Outer Most Alt 2 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
				{
				recog.base.set_state(132);
				recog.base.match_token(LoopSubst_ID,&mut recog.err_handler)?;
				recog.base.set_state(133);
				recog.base.match_token(LoopSubst_VARYING,&mut recog.err_handler)?;
				}
			}
		,
			3 =>{
				/*------- Outer Most Alt 3 -------*/
				unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
				{
				recog.base.set_state(134);
				recog.base.match_token(LoopSubst_INT,&mut recog.err_handler)?;
				}
			}

			_ => {}
		}
		})
	}
}
//------------------- whenClause ----------------
pub type WhenClauseContextAll<'input, 'arena, Tok = CommonToken<'input>> = WhenClauseContext<'input, 'arena, Tok>;

dbt_antlr_runtime::impl_ctx! { LoopSubstParserNodeKind, WhenClauseContext, WhenClauseContextExt, RULE_whenClause }

pub trait WhenClauseContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token WHEN
    /// Returns `None` if there is no child corresponding to token WHEN
    fn WHEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token THEN
    /// Returns `None` if there is no child corresponding to token THEN
    fn THEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> WhenClauseContextAttrs<'input, 'arena, Tok> for WhenClauseContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token WHEN
    /// Returns `None` if there is no child corresponding to token WHEN
    fn WHEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_WHEN)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token THEN
    /// Returns `None` if there is no child corresponding to token THEN
    fn THEN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == LoopSubst_THEN)
    }
}

impl<'input, 'arena, Input, TF> LoopSubstParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn whenClause(&mut self,) -> Result<&'arena WhenClauseContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::parse_rule!(recog = self, _parentctx, WhenClauseContext<TF::Tok>, RULE_whenClause, 8, |_parentctx| WhenClauseContextExt::create(recog.get_arena(), _parentctx, recog.get_state()); {
        let _local_ctx_fn = |recog: &Self| -> &'arena WhenClauseContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		/*------- Outer Most Alt 1 -------*/
		unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
		{
		recog.base.set_state(137);
		recog.base.match_token(LoopSubst_WHEN,&mut recog.err_handler)?;
		recog.base.set_state(138);
		recog.expr_rec(0)?;
		recog.base.set_state(139);
		recog.base.match_token(LoopSubst_THEN,&mut recog.err_handler)?;
		recog.base.set_state(140);
		recog.expr_rec(0)?;
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
static _serializedATN: [u32; 1893] = [
    1346458702, 3, 16909060, 29, 33, 143, 164, 2, 2, 11, 5, 29, 1001, 1030, 
    820, 1850, 10, 1860, 4, 1872, 11, 1883, 5, 1888, 5, 1893, 4, 1864, 8, 
    2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 
    2, 1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 16, 2, 0, 4294967295, 
    4294967295, 2, 2, 8, 2, 1, 4294967295, 4294967295, 7, 2, 16, 3, 0, 4294967295, 
    4294967295, 2, 3, 8, 3, 1, 4294967295, 4294967295, 7, 3, 16, 4, 0, 4294967295, 
    4294967295, 2, 4, 8, 4, 1, 4294967295, 4294967295, 7, 4, 16, 5, 0, 4294967295, 
    4294967295, 1, 0, 8, 5, 1, 4294967295, 4294967295, 1, 0, 32, 6, 1, 4294967295, 
    4294967295, 1, 0, 8, 7, 1, 4294967295, 4294967295, 1, 1, 72, 8, 1, 4294967295, 
    4294967295, 1, 1, 8, 9, 1, 4294967295, 4294967295, 1, 1, 32, 10, 1, 
    4294967295, 4294967295, 1, 1, 8, 11, 1, 4294967295, 4294967295, 3, 1, 
    8, 12, 2, 18, 4294967295, 8, 1, 8, 14, 1, 4294967295, 4294967295, 1, 
    1, 72, 15, 1, 4294967295, 4294967295, 1, 1, 32, 16, 1, 4294967295, 4294967295, 
    1, 1, 8, 17, 1, 4294967295, 4294967295, 1, 1, 72, 18, 1, 4294967295, 
    4294967295, 1, 1, 32, 19, 1, 4294967295, 4294967295, 1, 1, 8, 20, 1, 
    4294967295, 4294967295, 1, 1, 72, 21, 1, 4294967295, 4294967295, 1, 
    1, 32, 22, 1, 4294967295, 4294967295, 1, 1, 8, 23, 1, 4294967295, 4294967295, 
    1, 1, 72, 24, 1, 4294967295, 4294967295, 1, 1, 32, 25, 1, 4294967295, 
    4294967295, 1, 1, 8, 26, 1, 4294967295, 4294967295, 5, 1, 8, 27, 4, 
    32, 4294967295, 8, 1, 8, 31, 1, 4294967295, 4294967295, 10, 1, 10, 32, 
    2, 4294967295, 4294967295, 12, 1, 8, 34, 1, 4294967295, 35, 9, 1, 8, 
    35, 1, 4294967295, 4294967295, 1, 2, 32, 36, 1, 4294967295, 4294967295, 
    1, 2, 32, 37, 1, 4294967295, 4294967295, 1, 2, 32, 38, 1, 4294967295, 
    4294967295, 1, 2, 8, 39, 1, 4294967295, 4294967295, 1, 2, 32, 40, 1, 
    4294967295, 4294967295, 1, 2, 8, 41, 1, 4294967295, 4294967295, 1, 2, 
    32, 42, 1, 4294967295, 4294967295, 1, 2, 32, 43, 1, 4294967295, 4294967295, 
    1, 2, 8, 44, 1, 4294967295, 4294967295, 1, 2, 32, 45, 1, 4294967295, 
    4294967295, 1, 2, 8, 46, 1, 4294967295, 4294967295, 1, 2, 32, 47, 1, 
    4294967295, 4294967295, 1, 2, 8, 48, 1, 4294967295, 4294967295, 1, 2, 
    32, 49, 1, 4294967295, 4294967295, 1, 2, 32, 50, 1, 4294967295, 4294967295, 
    1, 2, 8, 51, 1, 4294967295, 4294967295, 1, 2, 32, 52, 1, 4294967295, 
    4294967295, 1, 2, 8, 53, 1, 4294967295, 4294967295, 1, 2, 32, 54, 1, 
    4294967295, 4294967295, 1, 2, 8, 55, 1, 4294967295, 4294967295, 1, 2, 
    32, 56, 1, 4294967295, 4294967295, 1, 2, 8, 57, 1, 4294967295, 4294967295, 
    4, 2, 8, 58, 1, 59, 4294967295, 8, 2, 8, 59, 1, 4294967295, 4294967295, 
    11, 2, 8, 60, 2, 4294967295, 4294967295, 12, 2, 8, 62, 1, 4294967295, 
    60, 1, 2, 32, 63, 1, 4294967295, 4294967295, 1, 2, 8, 64, 1, 4294967295, 
    4294967295, 3, 2, 8, 65, 2, 65, 4294967295, 8, 2, 8, 67, 1, 4294967295, 
    4294967295, 1, 2, 32, 68, 1, 4294967295, 4294967295, 1, 2, 8, 69, 1, 
    4294967295, 4294967295, 1, 2, 32, 70, 1, 4294967295, 4294967295, 1, 
    2, 32, 71, 1, 4294967295, 4294967295, 1, 2, 8, 72, 1, 4294967295, 4294967295, 
    1, 2, 32, 73, 1, 4294967295, 4294967295, 1, 2, 8, 74, 1, 4294967295, 
    4294967295, 5, 2, 8, 75, 1, 74, 4294967295, 8, 2, 8, 76, 1, 4294967295, 
    4294967295, 10, 2, 8, 77, 2, 4294967295, 4294967295, 12, 2, 8, 79, 1, 
    4294967295, 77, 9, 2, 8, 80, 1, 4294967295, 4294967295, 3, 2, 8, 81, 
    2, 79, 4294967295, 8, 2, 8, 83, 1, 4294967295, 4294967295, 1, 2, 32, 
    84, 1, 4294967295, 4294967295, 1, 2, 32, 85, 1, 4294967295, 4294967295, 
    1, 2, 32, 86, 1, 4294967295, 4294967295, 1, 2, 32, 87, 1, 4294967295, 
    4294967295, 1, 2, 32, 88, 1, 4294967295, 4294967295, 1, 2, 32, 89, 1, 
    4294967295, 4294967295, 1, 2, 32, 90, 1, 4294967295, 4294967295, 1, 
    2, 32, 91, 1, 4294967295, 4294967295, 1, 2, 8, 92, 1, 4294967295, 4294967295, 
    1, 2, 32, 93, 1, 4294967295, 4294967295, 1, 2, 8, 94, 1, 4294967295, 
    4294967295, 1, 2, 32, 95, 1, 4294967295, 4294967295, 1, 2, 32, 96, 1, 
    4294967295, 4294967295, 1, 2, 8, 97, 1, 4294967295, 4294967295, 1, 2, 
    32, 98, 1, 4294967295, 4294967295, 1, 2, 8, 99, 1, 4294967295, 4294967295, 
    1, 2, 32, 100, 1, 4294967295, 4294967295, 1, 2, 8, 101, 1, 4294967295, 
    4294967295, 3, 2, 8, 102, 2, 99, 4294967295, 8, 2, 8, 104, 1, 4294967295, 
    4294967295, 1, 2, 32, 105, 1, 4294967295, 4294967295, 1, 2, 8, 106, 
    1, 4294967295, 4294967295, 1, 2, 32, 107, 1, 4294967295, 4294967295, 
    1, 2, 32, 108, 1, 4294967295, 4294967295, 1, 2, 8, 109, 1, 4294967295, 
    4294967295, 1, 2, 32, 110, 1, 4294967295, 4294967295, 1, 2, 8, 111, 
    1, 4294967295, 4294967295, 1, 2, 32, 112, 1, 4294967295, 4294967295, 
    1, 2, 8, 113, 1, 4294967295, 4294967295, 1, 2, 32, 114, 1, 4294967295, 
    4294967295, 1, 2, 32, 115, 1, 4294967295, 4294967295, 1, 2, 8, 116, 
    1, 4294967295, 4294967295, 1, 2, 32, 117, 1, 4294967295, 4294967295, 
    1, 2, 8, 118, 1, 4294967295, 4294967295, 5, 2, 8, 119, 1, 115, 4294967295, 
    8, 2, 8, 120, 1, 4294967295, 4294967295, 10, 2, 8, 121, 2, 4294967295, 
    4294967295, 12, 2, 8, 123, 1, 4294967295, 118, 9, 2, 8, 124, 1, 4294967295, 
    4294967295, 1, 2, 32, 125, 1, 4294967295, 4294967295, 1, 2, 8, 126, 
    1, 4294967295, 4294967295, 1, 2, 32, 127, 1, 4294967295, 4294967295, 
    1, 2, 32, 128, 1, 4294967295, 4294967295, 1, 2, 32, 129, 1, 4294967295, 
    4294967295, 1, 2, 32, 130, 1, 4294967295, 4294967295, 1, 2, 32, 131, 
    1, 4294967295, 4294967295, 1, 2, 32, 132, 1, 4294967295, 4294967295, 
    1, 2, 32, 133, 1, 4294967295, 4294967295, 1, 2, 8, 134, 1, 4294967295, 
    4294967295, 3, 2, 8, 135, 15, 130, 4294967295, 8, 2, 8, 150, 1, 4294967295, 
    4294967295, 1, 3, 32, 151, 1, 4294967295, 4294967295, 1, 3, 32, 152, 
    1, 4294967295, 4294967295, 1, 3, 32, 153, 1, 4294967295, 4294967295, 
    1, 3, 32, 154, 1, 4294967295, 4294967295, 3, 3, 8, 155, 3, 136, 4294967295, 
    8, 3, 8, 158, 1, 4294967295, 4294967295, 1, 4, 32, 159, 1, 4294967295, 
    4294967295, 1, 4, 8, 160, 1, 4294967295, 4294967295, 1, 4, 32, 161, 
    1, 4294967295, 4294967295, 1, 4, 8, 162, 1, 4294967295, 4294967295, 
    1, 4, 8, 163, 1, 4294967295, 4294967295, 1, 4, 0, 164, 0, 4294967295, 
    4294967295, 1, 10, 0, 0, 0, 1, 17, 0, 0, 0, 1, 129, 0, 0, 0, 1, 135, 
    0, 0, 0, 1, 137, 0, 0, 0, 3, 2, 1, 11, 0, 5, 12, 4294967295, 0, 0, 1, 
    1, 0, 0, 0, 6, 14, 1, 4294967295, 0, 3, 4, 2, 18, 0, 7, 16, 0, 0, 0, 
    3, 2, 1, 18, 5, 1, 13, 0, 0, 0, 1, 15, 0, 0, 0, 1, 33, 0, 0, 0, 10, 
    20, 4, 0, 0, 7, 21, 1, 0, 0, 3, 2, 1, 32, 5, 10, 23, 3, 0, 0, 7, 24, 
    0, 0, 0, 3, 2, 1, 32, 4, 10, 26, 2, 0, 0, 5, 27, 5, 0, 0, 3, 2, 1, 32, 
    3, 10, 29, 1, 0, 0, 5, 30, 6, 0, 0, 3, 2, 1, 32, 2, 1, 19, 0, 0, 0, 
    1, 22, 0, 0, 0, 1, 25, 0, 0, 0, 1, 28, 0, 0, 0, 1, 35, 0, 0, 0, 1, 31, 
    0, 0, 0, 1, 34, 0, 0, 0, 1, 3, 0, 0, 0, 1, 33, 0, 0, 0, 5, 130, 32, 
    0, 0, 5, 130, 31, 0, 0, 5, 39, 7, 0, 0, 3, 2, 1, 40, 0, 5, 41, 8, 0, 
    0, 1, 130, 0, 0, 0, 5, 43, 13, 0, 0, 5, 44, 7, 0, 0, 3, 2, 1, 45, 0, 
    5, 46, 12, 0, 0, 3, 6, 3, 47, 0, 5, 48, 8, 0, 0, 1, 130, 0, 0, 0, 5, 
    50, 14, 0, 0, 5, 51, 7, 0, 0, 3, 2, 1, 52, 0, 5, 53, 12, 0, 0, 3, 6, 
    3, 54, 0, 5, 55, 8, 0, 0, 1, 130, 0, 0, 0, 5, 58, 15, 0, 0, 3, 8, 4, 
    59, 0, 1, 57, 0, 0, 0, 1, 60, 0, 0, 0, 1, 58, 0, 0, 0, 1, 61, 0, 0, 
    0, 1, 64, 0, 0, 0, 5, 63, 18, 0, 0, 3, 2, 1, 65, 0, 1, 62, 0, 0, 0, 
    1, 65, 0, 0, 0, 1, 66, 0, 0, 0, 5, 67, 19, 0, 0, 1, 130, 0, 0, 0, 5, 
    69, 31, 0, 0, 5, 78, 7, 0, 0, 3, 2, 1, 75, 0, 5, 72, 9, 0, 0, 3, 2, 
    1, 74, 0, 1, 71, 0, 0, 0, 1, 77, 0, 0, 0, 1, 73, 0, 0, 0, 1, 76, 0, 
    0, 0, 1, 79, 0, 0, 0, 1, 75, 0, 0, 0, 1, 70, 0, 0, 0, 1, 79, 0, 0, 0, 
    1, 80, 0, 0, 0, 5, 130, 8, 0, 0, 5, 82, 31, 0, 0, 5, 83, 10, 0, 0, 5, 
    130, 31, 0, 0, 5, 85, 20, 0, 0, 5, 86, 7, 0, 0, 5, 87, 31, 0, 0, 5, 
    88, 26, 0, 0, 3, 2, 1, 89, 0, 5, 90, 8, 0, 0, 1, 130, 0, 0, 0, 5, 92, 
    21, 0, 0, 5, 93, 7, 0, 0, 3, 2, 1, 94, 0, 5, 95, 26, 0, 0, 3, 2, 1, 
    98, 0, 5, 97, 27, 0, 0, 3, 2, 1, 99, 0, 1, 96, 0, 0, 0, 1, 99, 0, 0, 
    0, 1, 100, 0, 0, 0, 5, 101, 8, 0, 0, 1, 130, 0, 0, 0, 5, 103, 22, 0, 
    0, 5, 104, 7, 0, 0, 3, 2, 1, 105, 0, 5, 106, 28, 0, 0, 3, 2, 1, 107, 
    0, 5, 108, 8, 0, 0, 1, 130, 0, 0, 0, 5, 110, 23, 0, 0, 5, 111, 7, 0, 
    0, 3, 2, 1, 116, 0, 5, 113, 9, 0, 0, 3, 2, 1, 115, 0, 1, 112, 0, 0, 
    0, 1, 118, 0, 0, 0, 1, 114, 0, 0, 0, 1, 117, 0, 0, 0, 1, 119, 0, 0, 
    0, 1, 116, 0, 0, 0, 5, 120, 8, 0, 0, 1, 130, 0, 0, 0, 5, 122, 24, 0, 
    0, 5, 130, 30, 0, 0, 5, 124, 25, 0, 0, 5, 125, 30, 0, 0, 5, 130, 31, 
    0, 0, 5, 127, 31, 0, 0, 5, 128, 11, 0, 0, 3, 2, 1, 130, 0, 1, 36, 0, 
    0, 0, 1, 37, 0, 0, 0, 1, 38, 0, 0, 0, 1, 42, 0, 0, 0, 1, 49, 0, 0, 0, 
    1, 56, 0, 0, 0, 1, 68, 0, 0, 0, 1, 81, 0, 0, 0, 1, 84, 0, 0, 0, 1, 91, 
    0, 0, 0, 1, 102, 0, 0, 0, 1, 109, 0, 0, 0, 1, 121, 0, 0, 0, 1, 123, 
    0, 0, 0, 1, 126, 0, 0, 0, 1, 5, 0, 0, 0, 5, 136, 31, 0, 0, 5, 133, 31, 
    0, 0, 5, 136, 29, 0, 0, 5, 136, 32, 0, 0, 1, 131, 0, 0, 0, 1, 132, 0, 
    0, 0, 1, 134, 0, 0, 0, 1, 7, 0, 0, 0, 5, 138, 16, 0, 0, 3, 2, 1, 139, 
    0, 5, 140, 17, 0, 0, 3, 2, 1, 141, 0, 1, 9, 0, 0, 0, 0, 1, 1, 0, 2, 
    1, 1, 1, 2, 2, 1, 2, 3, 4, 6, 0, 0, 0, 24, 0, 0, 0, 17, 31, 33, 60, 
    64, 75, 78, 98, 116, 129, 135, 0, 2, 4, 6, 8, 1, 3, 5, 7, 9
];