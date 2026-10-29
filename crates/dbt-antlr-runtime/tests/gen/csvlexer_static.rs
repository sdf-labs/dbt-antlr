// Generated from CSV.g4 by ANTLR 4.13.2
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

use dbt_antlr_runtime::dfa::static_lexer_dfa::StaticLexerTables;

use std::ops::{DerefMut, Deref};
use std::sync::LazyLock;

dbt_antlr_runtime::check_version!("0","5");
pub const T__0:i32=1; 
pub const T__1:i32=2; 
pub const T__2:i32=3; 
pub const WS:i32=4; 
pub const TEXT:i32=5; 
pub const STRING:i32=6;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;6] = [
    "T__0", "T__1", "T__2", "WS", "TEXT", "STRING"
];
pub const _LITERAL_NAMES: [Option<&'static str>;4] = [
	None, Some("','"), Some("'\\r'"), Some("'\\n'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, None, None, None, Some("WS"), Some("TEXT"), Some("STRING")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, CSVLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct CSVLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { CSVLexer }
dbt_antlr_runtime::impl_deref! { lexer => CSVLexer }

impl<'input, 'arena, Input, TF> CSVLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = CSVLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct CSVLexerActions {
}

impl CSVLexerActions {
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>, TF::Tok>
    for CSVLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
 {}

impl<'input, 'arena, Input, TF> LexerRecog<'input, 'arena, TF, BaseLexerType<'input, 'arena, Input, TF>>
    for CSVLexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn get_rule_names(&self) -> &'static [&'static str] { &ruleNames }
    fn get_literal_names(&self) -> &[Option<&str>] { &_LITERAL_NAMES }
    fn get_symbolic_names(&self) -> &[Option<&str>] { &_SYMBOLIC_NAMES }
    fn get_grammar_file_name(&self) -> &'static str { "CSVLexer.g4" }
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new_static_lexer(&_STATIC_LEXER_TABLES));
static _STATIC_LEXER_TABLES: LazyLock<StaticLexerTables> =
    LazyLock::new(|| StaticLexerTables::from_words(&_STATIC_LEXER_TABLE_WORDS).expect("invalid static lexer DFA table words; regenerate the lexer"));
static _STATIC_LEXER_TABLE_WORDS: LazyLock<Vec<u32>> = LazyLock::new(|| vec![
    1396983361, 1, 1118, 1, 1, 8, 0, 4294967295, 5, 3, 2, 4, 4294967295, 
    1, 6, 4294967295, 4294967295, 4294967295, 4294967295, 0, 4294967295, 
    4294967295, 4294967295, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32770, 32769, 32769, 32771, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32772, 32769, 5, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32774, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 65535, 32769, 
    32769, 65535, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    65535, 32769, 65535, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 65535, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 32769, 
    32769, 32769, 32769, 32769, 32769, 32769, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 32772, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 32775, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 5, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 
    65535, 1, 128, 3, 2, 0, 1, 1, 2, 0, 0, 32769, 0, 0, 5, 0, 0, 1, 1, 1, 
    2, 1, 1, 4, 1, 0, 1, 0, 18, 1, 1, 4294958079, 4294963194, 4294967295, 
    4294967295, 4, 1, 0, 1, 0, 0, 5, 2, 1, 34, 0, 0, 4294967295, 0, 4294967295, 
    0, 4294967295, 0, 4294967295, 0, 4294967295, 0, 4294967295, 0, 4294967295, 
    0, 4294967295, 0, 0
]);