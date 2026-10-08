// Generated from VisitorBasic.g4 by ANTLR 4.13.2
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
use super::visitorbasiclistener::*;
use super::visitorbasicvisitor::*;

use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr_runtime::check_version!("0","1");
pub const VisitorBasic_A:i32=1;
pub const VisitorBasic_EOF:i32=EOF;
pub const RULE_s:usize = 0;
pub const ruleNames: [&'static str; 1] = [
    "s"
];

pub const _LITERAL_NAMES: [Option<&'static str>;2] = [
	None, Some("'A'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;2]  = [
	None, Some("A")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, VisitorBasicParserExt<'input, 'arena>, VisitorBasicParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct VisitorBasicParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> VisitorBasicParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				VisitorBasicParserExt {
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
        L: VisitorBasicListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub trait Visitable<'input: 'arena, 'arena, Tok: Token + 'input> {
    fn accept<V>(&'arena self, visitor: &mut V) -> Result<V::Return, ANTLRError>
    where
        'input: 'arena,
        V: VisitorBasicVisitor<'input, 'arena, Tok> + ?Sized;
}
pub struct VisitorBasicTreeWalker;
impl VisitorBasicTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: VisitorBasicListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, VisitorBasicParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <VisitorBasicParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum VisitorBasicParserNodeKind {
    SContext,
    Terminal,
    Error,
}
pub type VisitorBasicParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, VisitorBasicParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => VisitorBasicParser }
dbt_antlr_runtime::impl_node_kind! { VisitorBasicParserNodeKind {
; SContext(enter_s, exit_s,  visit_s), 
    }; listener = dyn VisitorBasicListener<'arena, Tok>, visitor = VisitorBasicVisitor,
}

pub struct VisitorBasicParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> VisitorBasicParserExt<'input, 'arena> {
}

impl<'input, 'arena, Input, TF> ParserRecog<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for VisitorBasicParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena {
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }        
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for VisitorBasicParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn get_grammar_file_name(&self) -> & str{ "VisitorBasic.g4" }
   	fn get_rule_names(&self) -> &[& str] { &ruleNames }
   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- s ----------------
pub type SContextAll<'input, 'arena, Tok = CommonToken<'input>> = SContext<'input, 'arena, Tok>;

pub type SContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SContextExt<'input, 'arena, Tok>, VisitorBasicParserNodeKind, Tok>;
dbt_antlr_runtime::impl_visitable! { VisitorBasicVisitor::SContext(visit_s) }
#[derive(Debug)]
pub struct SContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for SContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = VisitorBasicParserNodeKind;
    fn node_tag() -> VisitorBasicParserNodeKind { VisitorBasicParserNodeKind::SContext }
	fn get_rule_index(&self) -> usize { RULE_s }
    fn make_node(
        arena: &'arena Arena,
        ctx: SContext<'input, 'arena, Tok>,
    ) -> *mut VisitorBasicParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a VisitorBasicParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a SContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => SContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut VisitorBasicParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut SContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut SContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena VisitorBasicParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut VisitorBasicParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, SContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait SContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SContextAttrs<'input, 'arena, Tok> for SContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token A
    /// Returns `None` if there is no child corresponding to token A
    fn A(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == VisitorBasic_A)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == VisitorBasic_EOF)
    }
}

impl<'input, 'arena, Input, TF> VisitorBasicParser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn s(&mut self,) -> Result<&'arena SContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(SContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 0, RULE_s)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena SContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(2);
			recog.base.match_token(VisitorBasic_A,&mut recog.err_handler)?;
			recog.base.set_state(3);
			recog.base.match_token(VisitorBasic_EOF,&mut recog.err_handler)?;
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

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> = LazyLock::new(|| {
    PackedATNDeserializer::new()
        .deserialize(&_serializedATN)
        .expect("invalid packed parser ATN")
});
static _serializedATN: LazyLock<Vec<u32>> = LazyLock::new(|| vec![
    1346458702, 3, 16909060, 29, 1, 6, 4, 0, 0, 0, 1, 29, 42, 71, 20, 91, 
    0, 91, 0, 91, 0, 91, 1, 92, 1, 93, 0, 91, 0, 2, 0, 8, 0, 1, 4294967295, 
    4294967295, 7, 0, 16, 1, 0, 4294967295, 4294967295, 1, 0, 32, 1, 1, 
    4294967295, 4294967295, 1, 0, 32, 2, 1, 4294967295, 4294967295, 1, 0, 
    8, 3, 1, 4294967295, 4294967295, 1, 0, 0, 4, 0, 4294967295, 4294967295, 
    1, 2, 0, 0, 0, 5, 3, 1, 0, 0, 5, 4, 4294967295, 0, 0, 1, 1, 0, 0, 0, 
    0, 1
]);