// Generated from BetweenExpr.g4 by ANTLR 4.13.2
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
pub const ID:i32=15; 
pub const INT:i32=16; 
pub const WS:i32=17;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;17] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "T__12", "T__13", "ID", "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;15] = [
	None, Some("'NOT'"), Some("'AND'"), Some("'OR'"), Some("'BETWEEN'"), Some("'='"), 
	Some("'<'"), Some("'>'"), Some("'*'"), Some("'/'"), Some("'+'"), Some("'-'"), 
	Some("'('"), Some("')'"), Some("'->'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;18]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, Some("ID"), Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, BetweenExprLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct BetweenExprLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { BetweenExprLexer }
dbt_antlr_runtime::impl_deref! { lexer => BetweenExprLexer }

impl<'input, 'arena, Input, TF> BetweenExprLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = BetweenExprLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct BetweenExprLexerActions {
}

impl BetweenExprLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { BetweenExprLexerActions, "BetweenExprLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 824] = [
    4, 0, 17, 92, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 
    3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 
    1, 9, 1, 9, 1, 10, 1, 10, 1, 11, 1, 11, 1, 12, 1, 12, 1, 13, 1, 13, 
    1, 13, 1, 14, 4, 14, 77, 8, 14, 11, 14, 12, 14, 78, 1, 15, 4, 15, 82, 
    8, 15, 11, 15, 12, 15, 83, 1, 16, 4, 16, 87, 8, 16, 11, 16, 12, 16, 
    88, 1, 16, 1, 16, 0, 0, 17, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 
    7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 27, 14, 29, 15, 31, 
    16, 33, 17, 1, 0, 3, 1, 0, 97, 122, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 
    32, 32, 94, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 
    7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 
    0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 
    0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 0, 
    29, 1, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 0, 0, 0, 1, 35, 1, 0, 0, 
    0, 3, 39, 1, 0, 0, 0, 5, 43, 1, 0, 0, 0, 7, 46, 1, 0, 0, 0, 9, 54, 1, 
    0, 0, 0, 11, 56, 1, 0, 0, 0, 13, 58, 1, 0, 0, 0, 15, 60, 1, 0, 0, 0, 
    17, 62, 1, 0, 0, 0, 19, 64, 1, 0, 0, 0, 21, 66, 1, 0, 0, 0, 23, 68, 
    1, 0, 0, 0, 25, 70, 1, 0, 0, 0, 27, 72, 1, 0, 0, 0, 29, 76, 1, 0, 0, 
    0, 31, 81, 1, 0, 0, 0, 33, 86, 1, 0, 0, 0, 35, 36, 5, 78, 0, 0, 36, 
    37, 5, 79, 0, 0, 37, 38, 5, 84, 0, 0, 38, 2, 1, 0, 0, 0, 39, 40, 5, 
    65, 0, 0, 40, 41, 5, 78, 0, 0, 41, 42, 5, 68, 0, 0, 42, 4, 1, 0, 0, 
    0, 43, 44, 5, 79, 0, 0, 44, 45, 5, 82, 0, 0, 45, 6, 1, 0, 0, 0, 46, 
    47, 5, 66, 0, 0, 47, 48, 5, 69, 0, 0, 48, 49, 5, 84, 0, 0, 49, 50, 5, 
    87, 0, 0, 50, 51, 5, 69, 0, 0, 51, 52, 5, 69, 0, 0, 52, 53, 5, 78, 0, 
    0, 53, 8, 1, 0, 0, 0, 54, 55, 5, 61, 0, 0, 55, 10, 1, 0, 0, 0, 56, 57, 
    5, 60, 0, 0, 57, 12, 1, 0, 0, 0, 58, 59, 5, 62, 0, 0, 59, 14, 1, 0, 
    0, 0, 60, 61, 5, 42, 0, 0, 61, 16, 1, 0, 0, 0, 62, 63, 5, 47, 0, 0, 
    63, 18, 1, 0, 0, 0, 64, 65, 5, 43, 0, 0, 65, 20, 1, 0, 0, 0, 66, 67, 
    5, 45, 0, 0, 67, 22, 1, 0, 0, 0, 68, 69, 5, 40, 0, 0, 69, 24, 1, 0, 
    0, 0, 70, 71, 5, 41, 0, 0, 71, 26, 1, 0, 0, 0, 72, 73, 5, 45, 0, 0, 
    73, 74, 5, 62, 0, 0, 74, 28, 1, 0, 0, 0, 75, 77, 7, 0, 0, 0, 76, 75, 
    1, 0, 0, 0, 77, 78, 1, 0, 0, 0, 78, 76, 1, 0, 0, 0, 78, 79, 1, 0, 0, 
    0, 79, 30, 1, 0, 0, 0, 80, 82, 7, 1, 0, 0, 81, 80, 1, 0, 0, 0, 82, 83, 
    1, 0, 0, 0, 83, 81, 1, 0, 0, 0, 83, 84, 1, 0, 0, 0, 84, 32, 1, 0, 0, 
    0, 85, 87, 7, 2, 0, 0, 86, 85, 1, 0, 0, 0, 87, 88, 1, 0, 0, 0, 88, 86, 
    1, 0, 0, 0, 88, 89, 1, 0, 0, 0, 89, 90, 1, 0, 0, 0, 90, 91, 6, 16, 0, 
    0, 91, 34, 1, 0, 0, 0, 4, 0, 78, 83, 88, 1, 6, 0, 0
];