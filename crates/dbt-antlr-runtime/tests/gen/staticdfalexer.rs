// Generated from StaticDFA.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","5");
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
pub const T__10:i32=11; 
pub const T__11:i32=12; 
pub const ID:i32=13; 
pub const WS:i32=14;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;14] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "ID", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;13] = [
	None, Some("'create'"), Some("'table'"), Some("'view'"), Some("'drop'"), 
	Some("'('"), Some("')'"), Some("'.'"), Some("'a'"), Some("'b'"), Some("'c'"), 
	Some("'d'"), Some("'e'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;15]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, StaticDFALexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct StaticDFALexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { StaticDFALexer }
dbt_antlr_runtime::impl_deref! { lexer => StaticDFALexer }

impl<'input, 'arena, Input, TF> StaticDFALexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = StaticDFALexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct StaticDFALexerActions {
}

impl StaticDFALexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { StaticDFALexerActions, "StaticDFALexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 14, 80, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 1, 0, 1, 
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 
    4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 
    1, 10, 1, 11, 1, 11, 1, 12, 4, 12, 70, 8, 12, 11, 12, 12, 12, 71, 1, 
    13, 4, 13, 75, 8, 13, 11, 13, 12, 13, 76, 1, 13, 1, 13, 0, 0, 14, 1, 
    1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 
    23, 12, 25, 13, 27, 14, 1, 0, 2, 1, 0, 97, 122, 3, 0, 9, 10, 13, 13, 
    32, 32, 81, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 
    7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 
    0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 
    0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 1, 
    29, 1, 0, 0, 0, 3, 36, 1, 0, 0, 0, 5, 42, 1, 0, 0, 0, 7, 47, 1, 0, 0, 
    0, 9, 52, 1, 0, 0, 0, 11, 54, 1, 0, 0, 0, 13, 56, 1, 0, 0, 0, 15, 58, 
    1, 0, 0, 0, 17, 60, 1, 0, 0, 0, 19, 62, 1, 0, 0, 0, 21, 64, 1, 0, 0, 
    0, 23, 66, 1, 0, 0, 0, 25, 69, 1, 0, 0, 0, 27, 74, 1, 0, 0, 0, 29, 30, 
    5, 99, 0, 0, 30, 31, 5, 114, 0, 0, 31, 32, 5, 101, 0, 0, 32, 33, 5, 
    97, 0, 0, 33, 34, 5, 116, 0, 0, 34, 35, 5, 101, 0, 0, 35, 2, 1, 0, 0, 
    0, 36, 37, 5, 116, 0, 0, 37, 38, 5, 97, 0, 0, 38, 39, 5, 98, 0, 0, 39, 
    40, 5, 108, 0, 0, 40, 41, 5, 101, 0, 0, 41, 4, 1, 0, 0, 0, 42, 43, 5, 
    118, 0, 0, 43, 44, 5, 105, 0, 0, 44, 45, 5, 101, 0, 0, 45, 46, 5, 119, 
    0, 0, 46, 6, 1, 0, 0, 0, 47, 48, 5, 100, 0, 0, 48, 49, 5, 114, 0, 0, 
    49, 50, 5, 111, 0, 0, 50, 51, 5, 112, 0, 0, 51, 8, 1, 0, 0, 0, 52, 53, 
    5, 40, 0, 0, 53, 10, 1, 0, 0, 0, 54, 55, 5, 41, 0, 0, 55, 12, 1, 0, 
    0, 0, 56, 57, 5, 46, 0, 0, 57, 14, 1, 0, 0, 0, 58, 59, 5, 97, 0, 0, 
    59, 16, 1, 0, 0, 0, 60, 61, 5, 98, 0, 0, 61, 18, 1, 0, 0, 0, 62, 63, 
    5, 99, 0, 0, 63, 20, 1, 0, 0, 0, 64, 65, 5, 100, 0, 0, 65, 22, 1, 0, 
    0, 0, 66, 67, 5, 101, 0, 0, 67, 24, 1, 0, 0, 0, 68, 70, 7, 0, 0, 0, 
    69, 68, 1, 0, 0, 0, 70, 71, 1, 0, 0, 0, 71, 69, 1, 0, 0, 0, 71, 72, 
    1, 0, 0, 0, 72, 26, 1, 0, 0, 0, 73, 75, 7, 1, 0, 0, 74, 73, 1, 0, 0, 
    0, 75, 76, 1, 0, 0, 0, 76, 74, 1, 0, 0, 0, 76, 77, 1, 0, 0, 0, 77, 78, 
    1, 0, 0, 0, 78, 79, 6, 13, 0, 0, 79, 28, 1, 0, 0, 0, 3, 0, 71, 76, 1, 
    6, 0, 0
]);