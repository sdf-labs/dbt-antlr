// Generated from InlineAlias.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","8");
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
pub const T__12:i32=13; 
pub const T__13:i32=14; 
pub const T__14:i32=15; 
pub const T__15:i32=16; 
pub const ID:i32=17; 
pub const INT:i32=18; 
pub const WS:i32=19;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;19] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "T__12", "T__13", "T__14", "T__15", "ID", 
    "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;17] = [
	None, Some("'CREATE'"), Some("'VIEW'"), Some("'ORDER'"), Some("'BY'"), 
	Some("','"), Some("'AS'"), Some("'SELECT'"), Some("'FROM'"), Some("'ROWS'"), 
	Some("'BETWEEN'"), Some("'AND'"), Some("'='"), Some("'CAST'"), Some("'('"), 
	Some("')'"), Some("'INT64'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;20]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, InlineAliasLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct InlineAliasLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { InlineAliasLexer }
dbt_antlr_runtime::impl_deref! { lexer => InlineAliasLexer }

impl<'input, 'arena, Input, TF> InlineAliasLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = InlineAliasLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct InlineAliasLexerActions {
}

impl InlineAliasLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { InlineAliasLexerActions, "InlineAliasLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 1116] = [
    4, 0, 19, 128, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 1, 0, 1, 
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 
    1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 
    5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 
    1, 7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 
    9, 1, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 10, 1, 11, 1, 11, 1, 12, 
    1, 12, 1, 12, 1, 12, 1, 12, 1, 13, 1, 13, 1, 14, 1, 14, 1, 15, 1, 15, 
    1, 15, 1, 15, 1, 15, 1, 15, 1, 16, 4, 16, 113, 8, 16, 11, 16, 12, 16, 
    114, 1, 17, 4, 17, 118, 8, 17, 11, 17, 12, 17, 119, 1, 18, 4, 18, 123, 
    8, 18, 11, 18, 12, 18, 124, 1, 18, 1, 18, 0, 0, 19, 1, 1, 3, 2, 5, 3, 
    7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 
    13, 27, 14, 29, 15, 31, 16, 33, 17, 35, 18, 37, 19, 1, 0, 3, 2, 0, 95, 
    95, 97, 122, 1, 0, 48, 57, 2, 0, 9, 10, 32, 32, 130, 0, 1, 1, 0, 0, 
    0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 
    0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 
    1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 
    0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 0, 29, 1, 0, 0, 0, 0, 31, 1, 0, 
    0, 0, 0, 33, 1, 0, 0, 0, 0, 35, 1, 0, 0, 0, 0, 37, 1, 0, 0, 0, 1, 39, 
    1, 0, 0, 0, 3, 46, 1, 0, 0, 0, 5, 51, 1, 0, 0, 0, 7, 57, 1, 0, 0, 0, 
    9, 60, 1, 0, 0, 0, 11, 62, 1, 0, 0, 0, 13, 65, 1, 0, 0, 0, 15, 72, 1, 
    0, 0, 0, 17, 77, 1, 0, 0, 0, 19, 82, 1, 0, 0, 0, 21, 90, 1, 0, 0, 0, 
    23, 94, 1, 0, 0, 0, 25, 96, 1, 0, 0, 0, 27, 101, 1, 0, 0, 0, 29, 103, 
    1, 0, 0, 0, 31, 105, 1, 0, 0, 0, 33, 112, 1, 0, 0, 0, 35, 117, 1, 0, 
    0, 0, 37, 122, 1, 0, 0, 0, 39, 40, 5, 67, 0, 0, 40, 41, 5, 82, 0, 0, 
    41, 42, 5, 69, 0, 0, 42, 43, 5, 65, 0, 0, 43, 44, 5, 84, 0, 0, 44, 45, 
    5, 69, 0, 0, 45, 2, 1, 0, 0, 0, 46, 47, 5, 86, 0, 0, 47, 48, 5, 73, 
    0, 0, 48, 49, 5, 69, 0, 0, 49, 50, 5, 87, 0, 0, 50, 4, 1, 0, 0, 0, 51, 
    52, 5, 79, 0, 0, 52, 53, 5, 82, 0, 0, 53, 54, 5, 68, 0, 0, 54, 55, 5, 
    69, 0, 0, 55, 56, 5, 82, 0, 0, 56, 6, 1, 0, 0, 0, 57, 58, 5, 66, 0, 
    0, 58, 59, 5, 89, 0, 0, 59, 8, 1, 0, 0, 0, 60, 61, 5, 44, 0, 0, 61, 
    10, 1, 0, 0, 0, 62, 63, 5, 65, 0, 0, 63, 64, 5, 83, 0, 0, 64, 12, 1, 
    0, 0, 0, 65, 66, 5, 83, 0, 0, 66, 67, 5, 69, 0, 0, 67, 68, 5, 76, 0, 
    0, 68, 69, 5, 69, 0, 0, 69, 70, 5, 67, 0, 0, 70, 71, 5, 84, 0, 0, 71, 
    14, 1, 0, 0, 0, 72, 73, 5, 70, 0, 0, 73, 74, 5, 82, 0, 0, 74, 75, 5, 
    79, 0, 0, 75, 76, 5, 77, 0, 0, 76, 16, 1, 0, 0, 0, 77, 78, 5, 82, 0, 
    0, 78, 79, 5, 79, 0, 0, 79, 80, 5, 87, 0, 0, 80, 81, 5, 83, 0, 0, 81, 
    18, 1, 0, 0, 0, 82, 83, 5, 66, 0, 0, 83, 84, 5, 69, 0, 0, 84, 85, 5, 
    84, 0, 0, 85, 86, 5, 87, 0, 0, 86, 87, 5, 69, 0, 0, 87, 88, 5, 69, 0, 
    0, 88, 89, 5, 78, 0, 0, 89, 20, 1, 0, 0, 0, 90, 91, 5, 65, 0, 0, 91, 
    92, 5, 78, 0, 0, 92, 93, 5, 68, 0, 0, 93, 22, 1, 0, 0, 0, 94, 95, 5, 
    61, 0, 0, 95, 24, 1, 0, 0, 0, 96, 97, 5, 67, 0, 0, 97, 98, 5, 65, 0, 
    0, 98, 99, 5, 83, 0, 0, 99, 100, 5, 84, 0, 0, 100, 26, 1, 0, 0, 0, 101, 
    102, 5, 40, 0, 0, 102, 28, 1, 0, 0, 0, 103, 104, 5, 41, 0, 0, 104, 30, 
    1, 0, 0, 0, 105, 106, 5, 73, 0, 0, 106, 107, 5, 78, 0, 0, 107, 108, 
    5, 84, 0, 0, 108, 109, 5, 54, 0, 0, 109, 110, 5, 52, 0, 0, 110, 32, 
    1, 0, 0, 0, 111, 113, 7, 0, 0, 0, 112, 111, 1, 0, 0, 0, 113, 114, 1, 
    0, 0, 0, 114, 112, 1, 0, 0, 0, 114, 115, 1, 0, 0, 0, 115, 34, 1, 0, 
    0, 0, 116, 118, 7, 1, 0, 0, 117, 116, 1, 0, 0, 0, 118, 119, 1, 0, 0, 
    0, 119, 117, 1, 0, 0, 0, 119, 120, 1, 0, 0, 0, 120, 36, 1, 0, 0, 0, 
    121, 123, 7, 2, 0, 0, 122, 121, 1, 0, 0, 0, 123, 124, 1, 0, 0, 0, 124, 
    122, 1, 0, 0, 0, 124, 125, 1, 0, 0, 0, 125, 126, 1, 0, 0, 0, 126, 127, 
    6, 18, 0, 0, 127, 38, 1, 0, 0, 0, 4, 0, 114, 119, 124, 1, 6, 0, 0
];