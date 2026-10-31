// Generated from FuzzExpr.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","6");
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
pub const T__16:i32=17; 
pub const ID:i32=18; 
pub const INT:i32=19; 
pub const WS:i32=20;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;20] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "T__12", "T__13", "T__14", "T__15", "T__16", 
    "ID", "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;18] = [
	None, Some("','"), Some("';'"), Some("'let'"), Some("'alias'"), Some("'as'"), 
	Some("'pick'"), Some("'='"), Some("'.'"), Some("'['"), Some("']'"), Some("'-'"), 
	Some("'^'"), Some("'*'"), Some("'/'"), Some("'+'"), Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;21]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, FuzzExprLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct FuzzExprLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { FuzzExprLexer }
dbt_antlr_runtime::impl_deref! { lexer => FuzzExprLexer }

impl<'input, 'arena, Input, TF> FuzzExprLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = FuzzExprLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct FuzzExprLexerActions {
}

impl FuzzExprLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { FuzzExprLexerActions, "FuzzExprLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 910] = [
    4, 0, 20, 102, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
    19, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 
    1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 
    6, 1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 1, 10, 1, 11, 1, 
    11, 1, 12, 1, 12, 1, 13, 1, 13, 1, 14, 1, 14, 1, 15, 1, 15, 1, 16, 1, 
    16, 1, 17, 4, 17, 87, 8, 17, 11, 17, 12, 17, 88, 1, 18, 4, 18, 92, 8, 
    18, 11, 18, 12, 18, 93, 1, 19, 4, 19, 97, 8, 19, 11, 19, 12, 19, 98, 
    1, 19, 1, 19, 0, 0, 20, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 
    15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 27, 14, 29, 15, 31, 16, 
    33, 17, 35, 18, 37, 19, 39, 20, 1, 0, 3, 1, 0, 97, 122, 1, 0, 48, 57, 
    3, 0, 9, 10, 13, 13, 32, 32, 104, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 
    0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 
    0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 
    0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 
    27, 1, 0, 0, 0, 0, 29, 1, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 0, 0, 
    0, 0, 35, 1, 0, 0, 0, 0, 37, 1, 0, 0, 0, 0, 39, 1, 0, 0, 0, 1, 41, 1, 
    0, 0, 0, 3, 43, 1, 0, 0, 0, 5, 45, 1, 0, 0, 0, 7, 49, 1, 0, 0, 0, 9, 
    55, 1, 0, 0, 0, 11, 58, 1, 0, 0, 0, 13, 63, 1, 0, 0, 0, 15, 65, 1, 0, 
    0, 0, 17, 67, 1, 0, 0, 0, 19, 69, 1, 0, 0, 0, 21, 71, 1, 0, 0, 0, 23, 
    73, 1, 0, 0, 0, 25, 75, 1, 0, 0, 0, 27, 77, 1, 0, 0, 0, 29, 79, 1, 0, 
    0, 0, 31, 81, 1, 0, 0, 0, 33, 83, 1, 0, 0, 0, 35, 86, 1, 0, 0, 0, 37, 
    91, 1, 0, 0, 0, 39, 96, 1, 0, 0, 0, 41, 42, 5, 44, 0, 0, 42, 2, 1, 0, 
    0, 0, 43, 44, 5, 59, 0, 0, 44, 4, 1, 0, 0, 0, 45, 46, 5, 108, 0, 0, 
    46, 47, 5, 101, 0, 0, 47, 48, 5, 116, 0, 0, 48, 6, 1, 0, 0, 0, 49, 50, 
    5, 97, 0, 0, 50, 51, 5, 108, 0, 0, 51, 52, 5, 105, 0, 0, 52, 53, 5, 
    97, 0, 0, 53, 54, 5, 115, 0, 0, 54, 8, 1, 0, 0, 0, 55, 56, 5, 97, 0, 
    0, 56, 57, 5, 115, 0, 0, 57, 10, 1, 0, 0, 0, 58, 59, 5, 112, 0, 0, 59, 
    60, 5, 105, 0, 0, 60, 61, 5, 99, 0, 0, 61, 62, 5, 107, 0, 0, 62, 12, 
    1, 0, 0, 0, 63, 64, 5, 61, 0, 0, 64, 14, 1, 0, 0, 0, 65, 66, 5, 46, 
    0, 0, 66, 16, 1, 0, 0, 0, 67, 68, 5, 91, 0, 0, 68, 18, 1, 0, 0, 0, 69, 
    70, 5, 93, 0, 0, 70, 20, 1, 0, 0, 0, 71, 72, 5, 45, 0, 0, 72, 22, 1, 
    0, 0, 0, 73, 74, 5, 94, 0, 0, 74, 24, 1, 0, 0, 0, 75, 76, 5, 42, 0, 
    0, 76, 26, 1, 0, 0, 0, 77, 78, 5, 47, 0, 0, 78, 28, 1, 0, 0, 0, 79, 
    80, 5, 43, 0, 0, 80, 30, 1, 0, 0, 0, 81, 82, 5, 40, 0, 0, 82, 32, 1, 
    0, 0, 0, 83, 84, 5, 41, 0, 0, 84, 34, 1, 0, 0, 0, 85, 87, 7, 0, 0, 0, 
    86, 85, 1, 0, 0, 0, 87, 88, 1, 0, 0, 0, 88, 86, 1, 0, 0, 0, 88, 89, 
    1, 0, 0, 0, 89, 36, 1, 0, 0, 0, 90, 92, 7, 1, 0, 0, 91, 90, 1, 0, 0, 
    0, 92, 93, 1, 0, 0, 0, 93, 91, 1, 0, 0, 0, 93, 94, 1, 0, 0, 0, 94, 38, 
    1, 0, 0, 0, 95, 97, 7, 2, 0, 0, 96, 95, 1, 0, 0, 0, 97, 98, 1, 0, 0, 
    0, 98, 96, 1, 0, 0, 0, 98, 99, 1, 0, 0, 0, 99, 100, 1, 0, 0, 0, 100, 
    101, 6, 19, 0, 0, 101, 40, 1, 0, 0, 0, 4, 0, 88, 93, 98, 1, 6, 0, 0
];