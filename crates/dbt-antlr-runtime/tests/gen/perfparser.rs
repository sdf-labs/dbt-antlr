// Generated from Perf.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_braces)]
#![allow(unused_parens)]
use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::PredictionContextCache;
use dbt_antlr_runtime::parser::{Parser, BaseParser, ParserRecog, ListenerId};
use dbt_antlr_runtime::token::CommonToken;
use dbt_antlr_runtime::token_stream::TokenStream;
use dbt_antlr_runtime::TokenSource;
use dbt_antlr_runtime::parser_atn_simulator::ParserATNSimulator;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::rule_context::{CustomRuleContext, RuleContext};
use dbt_antlr_runtime::recognizer::{Recognizer,Actions};
use dbt_antlr_runtime::atn_config_set::ATNConfigSet;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
use dbt_antlr_runtime::atn_simulator::BaseATNSimulator;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr_runtime::atn::{ATN, INVALID_ALT};
use dbt_antlr_runtime::error_strategy::{DefaultErrorStrategy, ErrorStrategyDelegate, ErrorStrategy};
use dbt_antlr_runtime::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use dbt_antlr_runtime::tree::*;
use dbt_antlr_runtime::token::{TOKEN_EOF,Token};
use dbt_antlr_runtime::int_stream::EOF;
use dbt_antlr_runtime::vocabulary::{Vocabulary,VocabularyImpl};
use dbt_antlr_runtime::token_factory::TokenFactory;
use super::perflistener::*;
use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr_runtime::check_version!("0","1");
pub const Perf_T__0:i32=1; 
pub const Perf_T__1:i32=2; 
pub const Perf_T__2:i32=3; 
pub const Perf_T__3:i32=4; 
pub const Perf_T__4:i32=5; 
pub const Perf_T__5:i32=6; 
pub const Perf_T__6:i32=7; 
pub const Perf_T__7:i32=8; 
pub const Perf_T__8:i32=9; 
pub const Perf_T__9:i32=10; 
pub const Perf_ID:i32=11; 
pub const Perf_WS:i32=12;
pub const Perf_EOF:i32=EOF;
pub const RULE_stat:usize = 0; 
pub const RULE_expr:usize = 1;
pub const ruleNames: [&'static str; 2] = [
    "stat", "expr"
];

pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
	None, Some("';'"), Some("'.'"), Some("'not'"), Some("'and'"), Some("'or'"), 
	Some("'('"), Some("')'"), Some("'?'"), Some("':'"), Some("'between'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;13]  = [
	None, None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, PerfParserExt<'input, 'arena>, PerfParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct PerfParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> PerfParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				PerfParserExt {
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
        L: PerfListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct PerfTreeWalker;
impl PerfTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: PerfListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, PerfParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <PerfParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum PerfParserNodeKind {
    StatContext,
    ExprContext,
    Terminal,
    Error,
}
pub type PerfParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, PerfParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => PerfParser }
dbt_antlr_runtime::impl_node_kind! { PerfParserNodeKind {
; StatContext(enter_stat, exit_stat, ), ExprContext(enter_expr, exit_expr, ), 
    }; listener = dyn PerfListener<'arena, Tok>,
}

pub struct PerfParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> PerfParserExt<'input, 'arena> {
}

impl<'input, 'arena, Input, TF> ParserRecog<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for PerfParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena {
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }        
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for PerfParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn get_grammar_file_name(&self) -> & str{ "Perf.g4" }
   	fn get_rule_names(&self) -> &[& str] { &ruleNames }
   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&'arena PerfParserNode<'input, 'arena, TF::Tok>>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input, 'arena, Input, TF>
	) -> bool {
		match rule_index {
		    1 => PerfParser::<'input, 'arena, Input, TF>::expr_sempred(_localctx.and_then(|x| x.as_rule_context()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, 'arena, Input, TF> PerfParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn expr_sempred(_ctx: Option<&'arena ExprContext<'input, 'arena, TF::Tok>>, pred_index:i32, recog: &mut <Self as Deref>::Target) -> bool
	 {
		match pred_index {
	        0 => {
			recog.precpred(None, 5)
		    }
	        1 => {
			recog.precpred(None, 4)
		    }
	        2 => {
			recog.precpred(None, 2)
		    }
		    _ => true
		}
	}
}
//------------------- stat ----------------
pub type StatContextAll<'input, 'arena, Tok = CommonToken<'input>> = StatContext<'input, 'arena, Tok>;

pub type StatContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, StatContextExt<'input, 'arena, Tok>, PerfParserNodeKind, Tok>;
#[derive(Debug)]
pub struct StatContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for StatContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = PerfParserNodeKind;
    fn node_tag() -> PerfParserNodeKind { PerfParserNodeKind::StatContext }
	fn get_rule_index(&self) -> usize { RULE_stat }
    fn make_node(
        arena: &'arena Arena,
        ctx: StatContext<'input, 'arena, Tok>,
    ) -> *mut PerfParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a PerfParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a StatContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => StatContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut PerfParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut StatContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut StatContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> StatContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena PerfParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut PerfParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, StatContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait StatContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> StatContextAttrs<'input, 'arena, Tok> for StatContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn expr(&self) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> PerfParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn stat(&mut self,) -> Result<&'arena StatContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(StatContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 0, RULE_stat)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena StatContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(10);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(0,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(4);
					recog.expr_rec(0)?;
					recog.base.set_state(5);
					recog.base.match_token(Perf_T__0,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule expr*/
					recog.base.set_state(7);
					recog.expr_rec(0)?;
					recog.base.set_state(8);
					recog.base.match_token(Perf_T__1,&mut recog.err_handler)?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExprContext<'input, 'arena, Tok>;

pub type ExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExprContextExt<'input, 'arena, Tok>, PerfParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ExprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = PerfParserNodeKind;
    fn node_tag() -> PerfParserNodeKind { PerfParserNodeKind::ExprContext }
	fn get_rule_index(&self) -> usize { RULE_expr }
    fn make_node(
        arena: &'arena Arena,
        ctx: ExprContext<'input, 'arena, Tok>,
    ) -> *mut PerfParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a PerfParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ExprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ExprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut PerfParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ExprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ExprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena PerfParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut PerfParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ExprContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>>;
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExprContextAttrs<'input, 'arena, Tok> for ExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == Perf_ID)
    }
    fn expr_all(&self) -> Vec<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn expr(&self, i: usize) -> Option<&'arena ExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> PerfParser<'input, 'arena, Input, TF>
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
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
		let _parentctx = recog.base.take_ctx();
		let _parentState = recog.base.get_state();
		recog.base.enter_recursion_rule(ExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 2, RULE_expr, _p)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let _startState = 2;
		let result: Result<(), ANTLRError> = (|| {
	        let mut _alt: i32;
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(25);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    Perf_ID  => {
			        {
			        recog.base.set_state(13);
			        recog.base.match_token(Perf_ID,&mut recog.err_handler)?;
			        }}
			    Perf_T__2  => {
			        {
			        recog.base.set_state(14);
			        recog.base.match_token(Perf_T__2,&mut recog.err_handler)?;
			        /*InvokeRule expr*/
			        recog.base.set_state(15);
			        recog.expr_rec(6)?;
			        }}
			    Perf_T__5  => {
			        {
			        recog.base.set_state(16);
			        recog.base.match_token(Perf_T__5,&mut recog.err_handler)?;
			        recog.base.set_state(17);
			        recog.base.match_token(Perf_ID,&mut recog.err_handler)?;
			        recog.base.set_state(18);
			        recog.base.match_token(Perf_T__6,&mut recog.err_handler)?;
			        /*InvokeRule expr*/
			        recog.base.set_state(19);
			        recog.expr_rec(3)?;
			        }}
			    Perf_T__9  => {
			        {
			        recog.base.set_state(20);
			        recog.base.match_token(Perf_T__9,&mut recog.err_handler)?;
			        /*InvokeRule expr*/
			        recog.base.set_state(21);
			        recog.expr_rec(0)?;
			        recog.base.set_state(22);
			        recog.base.match_token(Perf_T__3,&mut recog.err_handler)?;
			        /*InvokeRule expr*/
			        recog.base.set_state(23);
			        recog.expr_rec(1)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			let tmp = recog.input.lt(-1);
			recog.base.with_mut_ctx(|ctx| { ctx.set_stop(tmp.map(|t| t as _)); });
			recog.base.set_state(41);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					{
					recog.base.set_state(39);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.get_interpreter().adaptive_predict(2,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

							recog.base.set_state(27);
							if !({recog.precpred(None, 5)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(28);
							recog.base.match_token(Perf_T__3,&mut recog.err_handler)?;
							/*InvokeRule expr*/
							recog.base.set_state(29);
							recog.expr_rec(6)?;
							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

							recog.base.set_state(30);
							if !({recog.precpred(None, 4)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(31);
							recog.base.match_token(Perf_T__4,&mut recog.err_handler)?;
							/*InvokeRule expr*/
							recog.base.set_state(32);
							recog.expr_rec(5)?;
							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let tmp = ExprContextExt::create(recog.get_arena(), _parentctx, _parentState)?;
							let _prevctx = recog.push_new_recursion_context(tmp.into(), _startState, RULE_expr)?;

							recog.base.set_state(33);
							if !({recog.precpred(None, 2)}) {
								Err(ANTLRError::failed_predicate(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(34);
							recog.base.match_token(Perf_T__7,&mut recog.err_handler)?;
							/*InvokeRule expr*/
							recog.base.set_state(35);
							recog.expr_rec(0)?;
							recog.base.set_state(36);
							recog.base.match_token(Perf_T__8,&mut recog.err_handler)?;
							/*InvokeRule expr*/
							recog.base.set_state(37);
							recog.expr_rec(3)?;
							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(43);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.get_interpreter().adaptive_predict(3,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e) if !e.is_recoverable() => return Err(e),
		Err(ref re)=>{
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx).map(|ctx| { ctx.as_rule_context().unwrap() } )
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
    1346458702, 3, 16909060, 29, 12, 45, 57, 0, 0, 4, 2, 29, 315, 344, 285, 
    629, 0, 629, 0, 629, 4, 633, 2, 635, 2, 637, 0, 629, 0, 2, 0, 8, 0, 
    1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 2, 
    1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 24, 2, 8, 4294967295, 4294967295, 
    1, 0, 8, 10, 1, 4294967295, 4294967295, 1, 0, 32, 11, 1, 4294967295, 
    4294967295, 1, 0, 8, 12, 1, 4294967295, 4294967295, 1, 0, 8, 13, 1, 
    4294967295, 4294967295, 1, 0, 32, 14, 1, 4294967295, 4294967295, 1, 
    0, 8, 15, 1, 4294967295, 4294967295, 3, 0, 8, 16, 2, 11, 4294967295, 
    8, 0, 8, 18, 1, 4294967295, 4294967295, 1, 1, 72, 19, 1, 4294967295, 
    4294967295, 1, 1, 32, 20, 1, 4294967295, 4294967295, 1, 1, 32, 21, 1, 
    4294967295, 4294967295, 1, 1, 8, 22, 1, 4294967295, 4294967295, 1, 1, 
    32, 23, 1, 4294967295, 4294967295, 1, 1, 32, 24, 1, 4294967295, 4294967295, 
    1, 1, 32, 25, 1, 4294967295, 4294967295, 1, 1, 8, 26, 1, 4294967295, 
    4294967295, 1, 1, 32, 27, 1, 4294967295, 4294967295, 1, 1, 8, 28, 1, 
    4294967295, 4294967295, 1, 1, 32, 29, 1, 4294967295, 4294967295, 1, 
    1, 8, 30, 1, 4294967295, 4294967295, 1, 1, 8, 31, 1, 4294967295, 4294967295, 
    3, 1, 8, 32, 4, 26, 4294967295, 8, 1, 8, 36, 1, 4294967295, 4294967295, 
    1, 1, 72, 37, 1, 4294967295, 4294967295, 1, 1, 32, 38, 1, 4294967295, 
    4294967295, 1, 1, 8, 39, 1, 4294967295, 4294967295, 1, 1, 72, 40, 1, 
    4294967295, 4294967295, 1, 1, 32, 41, 1, 4294967295, 4294967295, 1, 
    1, 8, 42, 1, 4294967295, 4294967295, 1, 1, 72, 43, 1, 4294967295, 4294967295, 
    1, 1, 32, 44, 1, 4294967295, 4294967295, 1, 1, 8, 45, 1, 4294967295, 
    4294967295, 1, 1, 32, 46, 1, 4294967295, 4294967295, 1, 1, 8, 47, 1, 
    4294967295, 4294967295, 1, 1, 8, 48, 1, 4294967295, 4294967295, 5, 1, 
    8, 49, 3, 40, 4294967295, 8, 1, 8, 52, 1, 4294967295, 4294967295, 10, 
    1, 10, 53, 2, 4294967295, 4294967295, 12, 1, 8, 55, 1, 4294967295, 43, 
    9, 1, 8, 56, 1, 4294967295, 4294967295, 1, 1, 0, 57, 0, 4294967295, 
    4294967295, 1, 10, 0, 0, 0, 1, 25, 0, 0, 0, 1, 5, 0, 0, 0, 1, 8, 0, 
    0, 0, 1, 26, 0, 0, 0, 1, 22, 0, 0, 0, 1, 24, 0, 0, 0, 1, 40, 0, 0, 0, 
    1, 36, 0, 0, 0, 1, 38, 0, 0, 0, 3, 2, 1, 5, 0, 5, 6, 1, 0, 0, 1, 11, 
    0, 0, 0, 3, 2, 1, 8, 0, 5, 9, 2, 0, 0, 1, 11, 0, 0, 0, 1, 4, 0, 0, 0, 
    1, 7, 0, 0, 0, 1, 1, 0, 0, 0, 6, 13, 1, 4294967295, 0, 5, 26, 11, 0, 
    0, 5, 15, 3, 0, 0, 3, 2, 1, 26, 6, 5, 17, 6, 0, 0, 5, 18, 11, 0, 0, 
    5, 19, 7, 0, 0, 3, 2, 1, 26, 3, 5, 21, 10, 0, 0, 3, 2, 1, 22, 0, 5, 
    23, 4, 0, 0, 3, 2, 1, 24, 1, 1, 26, 0, 0, 0, 1, 12, 0, 0, 0, 1, 14, 
    0, 0, 0, 1, 16, 0, 0, 0, 1, 20, 0, 0, 0, 1, 41, 0, 0, 0, 10, 28, 5, 
    0, 0, 5, 29, 4, 0, 0, 3, 2, 1, 40, 6, 10, 31, 4, 0, 0, 5, 32, 5, 0, 
    0, 3, 2, 1, 40, 5, 10, 34, 2, 0, 0, 5, 35, 8, 0, 0, 3, 2, 1, 36, 0, 
    5, 37, 9, 0, 0, 3, 2, 1, 38, 3, 1, 40, 0, 0, 0, 1, 27, 0, 0, 0, 1, 30, 
    0, 0, 0, 1, 33, 0, 0, 0, 1, 43, 0, 0, 0, 1, 39, 0, 0, 0, 1, 42, 0, 0, 
    0, 1, 3, 0, 0, 0, 1, 41, 0, 0, 0, 10, 25, 39, 41, 0, 2, 1, 3
]);