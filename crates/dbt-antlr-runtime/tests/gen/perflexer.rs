// Generated from Perf.g4 by ANTLR 4.13.2
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

dbt_antlr_runtime::check_version!("0","1");
pub const T__0:i32=1; 
pub const T__1:i32=2; 
pub const T__2:i32=3; 
pub const T__3:i32=4; 
pub const T__4:i32=5; 
pub const T__5:i32=6; 
pub const T__6:i32=7; 
pub const T__7:i32=8; 
pub const T__8:i32=9; 
pub const T__9:i32=10; 
pub const ID:i32=11; 
pub const WS:i32=12;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;12] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "ID", "WS"
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

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, PerfLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct PerfLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { PerfLexer }
dbt_antlr_runtime::impl_deref! { lexer => PerfLexer }

impl<'input, 'arena, Input, TF> PerfLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = PerfLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct PerfLexerActions {
}

impl PerfLexerActions {
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>, TF::Tok>
    for PerfLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
 {}

impl<'input, 'arena, Input, TF> LexerRecog<'input, 'arena, TF, BaseLexerType<'input, 'arena, Input, TF>>
    for PerfLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] { &ruleNames }
    fn get_literal_names(&self) -> &[Option<&str>] { &_LITERAL_NAMES }
    fn get_symbolic_names(&self) -> &[Option<&str>] { &_SYMBOLIC_NAMES }
    fn get_grammar_file_name(&self) -> &'static str { "PerfLexer.g4" }
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 12, 70, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 
    2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 
    1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 
    9, 1, 9, 1, 10, 1, 10, 5, 10, 59, 8, 10, 10, 10, 12, 10, 62, 9, 10, 
    1, 11, 4, 11, 65, 8, 11, 11, 11, 12, 11, 66, 1, 11, 1, 11, 0, 0, 12, 
    1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 
    11, 23, 12, 1, 0, 3, 3, 0, 65, 90, 95, 95, 97, 122, 4, 0, 48, 57, 65, 
    90, 95, 95, 97, 122, 3, 0, 9, 10, 12, 13, 32, 32, 71, 0, 1, 1, 0, 0, 
    0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 
    0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 
    1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 
    1, 25, 1, 0, 0, 0, 3, 27, 1, 0, 0, 0, 5, 29, 1, 0, 0, 0, 7, 33, 1, 0, 
    0, 0, 9, 37, 1, 0, 0, 0, 11, 40, 1, 0, 0, 0, 13, 42, 1, 0, 0, 0, 15, 
    44, 1, 0, 0, 0, 17, 46, 1, 0, 0, 0, 19, 48, 1, 0, 0, 0, 21, 56, 1, 0, 
    0, 0, 23, 64, 1, 0, 0, 0, 25, 26, 5, 59, 0, 0, 26, 2, 1, 0, 0, 0, 27, 
    28, 5, 46, 0, 0, 28, 4, 1, 0, 0, 0, 29, 30, 5, 110, 0, 0, 30, 31, 5, 
    111, 0, 0, 31, 32, 5, 116, 0, 0, 32, 6, 1, 0, 0, 0, 33, 34, 5, 97, 0, 
    0, 34, 35, 5, 110, 0, 0, 35, 36, 5, 100, 0, 0, 36, 8, 1, 0, 0, 0, 37, 
    38, 5, 111, 0, 0, 38, 39, 5, 114, 0, 0, 39, 10, 1, 0, 0, 0, 40, 41, 
    5, 40, 0, 0, 41, 12, 1, 0, 0, 0, 42, 43, 5, 41, 0, 0, 43, 14, 1, 0, 
    0, 0, 44, 45, 5, 63, 0, 0, 45, 16, 1, 0, 0, 0, 46, 47, 5, 58, 0, 0, 
    47, 18, 1, 0, 0, 0, 48, 49, 5, 98, 0, 0, 49, 50, 5, 101, 0, 0, 50, 51, 
    5, 116, 0, 0, 51, 52, 5, 119, 0, 0, 52, 53, 5, 101, 0, 0, 53, 54, 5, 
    101, 0, 0, 54, 55, 5, 110, 0, 0, 55, 20, 1, 0, 0, 0, 56, 60, 7, 0, 0, 
    0, 57, 59, 7, 1, 0, 0, 58, 57, 1, 0, 0, 0, 59, 62, 1, 0, 0, 0, 60, 58, 
    1, 0, 0, 0, 60, 61, 1, 0, 0, 0, 61, 22, 1, 0, 0, 0, 62, 60, 1, 0, 0, 
    0, 63, 65, 7, 2, 0, 0, 64, 63, 1, 0, 0, 0, 65, 66, 1, 0, 0, 0, 66, 64, 
    1, 0, 0, 0, 66, 67, 1, 0, 0, 0, 67, 68, 1, 0, 0, 0, 68, 69, 6, 11, 0, 
    0, 69, 24, 1, 0, 0, 0, 3, 0, 60, 66, 1, 6, 0, 0
]);