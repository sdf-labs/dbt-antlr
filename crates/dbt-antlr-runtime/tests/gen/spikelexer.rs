// Generated from SpikeLexer.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::char_stream::CharStream;
use dbt_antlr_runtime::int_stream::IntStream;
use dbt_antlr_runtime::lexer::{BaseLexer, LexerRecog, Lexer as _};
use dbt_antlr_runtime::atn_config_set::LexerATNConfigSet;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::atn_simulator::BaseATNSimulator;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr_runtime::TokenSource;
use dbt_antlr_runtime::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use dbt_antlr_runtime::PredictionContextCache;
use dbt_antlr_runtime::recognizer::Actions;
use dbt_antlr_runtime::token_factory::{CommonTokenFactory, TokenFactory};
use dbt_antlr_runtime::rule_context::{BaseRuleContext,EmptyNodeKind,EmptyCustomRuleContext,EmptyRuleNode};
use dbt_antlr_runtime::vocabulary::{Vocabulary,VocabularyImpl};

use std::ops::{DerefMut, Deref};
use std::sync::LazyLock;

dbt_antlr_runtime::check_version!("0","3");
pub const COMMENT:i32=1; 
pub const KWLET:i32=2; 
pub const ID:i32=3; 
pub const INT:i32=4; 
pub const WS:i32=5;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;5] = [
    "COMMENT", "KWLET", "ID", "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
	None, None, Some("'let'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;6]  = [
	None, Some("COMMENT"), Some("KWLET"), Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, SpikeLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct SpikeLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { SpikeLexer }
dbt_antlr_runtime::impl_deref! { lexer => SpikeLexer }

impl<'input, 'arena, Input, TF> SpikeLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = SpikeLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct SpikeLexerActions {
}

impl SpikeLexerActions {
	fn KWLET_sempred<'arena, 'input, Input, TF>(pred_index:i32, recog: &mut BaseLexerType<'input, 'arena, Input, TF>) -> bool
	where
	    TF: TokenFactory<'input, 'arena> + 'arena,
	    Input: CharStream<'input>,
	 {
		match pred_index {
	        0 => {
			recog.input().la(1) == '=' as i32
		    }
		    _ => true
		}
	}
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>, TF::Tok>
    for SpikeLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn sempred(_localctx: Option<&EmptyRuleNode<'input, 'arena, TF::Tok>>, rule_index: i32, pred_index: i32, recog:&mut BaseLexerType<'input, 'arena, Input, TF>) -> bool {
        match rule_index {
            1 => SpikeLexerActions::KWLET_sempred(pred_index, recog), 
            _ => true
        }
    }
}
impl<'input, 'arena, Input, TF> LexerRecog<'input, 'arena, TF, BaseLexerType<'input, 'arena, Input, TF>>
    for SpikeLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] { &ruleNames }
    fn get_literal_names(&self) -> &[Option<&str>] { &_LITERAL_NAMES }
    fn get_symbolic_names(&self) -> &[Option<&str>] { &_SYMBOLIC_NAMES }
    fn get_grammar_file_name(&self) -> &'static str { "SpikeLexer.g4" }
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 5, 47, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 5, 0, 17, 8, 0, 10, 0, 12, 
    0, 20, 9, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    2, 4, 2, 32, 8, 2, 11, 2, 12, 2, 33, 1, 3, 4, 3, 37, 8, 3, 11, 3, 12, 
    3, 38, 1, 4, 4, 4, 42, 8, 4, 11, 4, 12, 4, 43, 1, 4, 1, 4, 1, 18, 0, 
    5, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 1, 0, 3, 1, 0, 97, 122, 1, 0, 48, 57, 
    3, 0, 9, 10, 13, 13, 32, 32, 51, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 
    0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 1, 11, 1, 0, 0, 
    0, 3, 24, 1, 0, 0, 0, 5, 31, 1, 0, 0, 0, 7, 36, 1, 0, 0, 0, 9, 41, 1, 
    0, 0, 0, 11, 12, 5, 47, 0, 0, 12, 13, 5, 42, 0, 0, 13, 18, 1, 0, 0, 
    0, 14, 17, 3, 1, 0, 0, 15, 17, 9, 0, 0, 0, 16, 14, 1, 0, 0, 0, 16, 15, 
    1, 0, 0, 0, 17, 20, 1, 0, 0, 0, 18, 19, 1, 0, 0, 0, 18, 16, 1, 0, 0, 
    0, 19, 21, 1, 0, 0, 0, 20, 18, 1, 0, 0, 0, 21, 22, 5, 42, 0, 0, 22, 
    23, 5, 47, 0, 0, 23, 2, 1, 0, 0, 0, 24, 25, 5, 108, 0, 0, 25, 26, 5, 
    101, 0, 0, 26, 27, 5, 116, 0, 0, 27, 28, 1, 0, 0, 0, 28, 29, 4, 1, 0, 
    0, 29, 4, 1, 0, 0, 0, 30, 32, 7, 0, 0, 0, 31, 30, 1, 0, 0, 0, 32, 33, 
    1, 0, 0, 0, 33, 31, 1, 0, 0, 0, 33, 34, 1, 0, 0, 0, 34, 6, 1, 0, 0, 
    0, 35, 37, 7, 1, 0, 0, 36, 35, 1, 0, 0, 0, 37, 38, 1, 0, 0, 0, 38, 36, 
    1, 0, 0, 0, 38, 39, 1, 0, 0, 0, 39, 8, 1, 0, 0, 0, 40, 42, 7, 2, 0, 
    0, 41, 40, 1, 0, 0, 0, 42, 43, 1, 0, 0, 0, 43, 41, 1, 0, 0, 0, 43, 44, 
    1, 0, 0, 0, 44, 45, 1, 0, 0, 0, 45, 46, 6, 4, 0, 0, 46, 10, 1, 0, 0, 
    0, 6, 0, 16, 18, 33, 38, 43, 1, 0, 1, 0
]);