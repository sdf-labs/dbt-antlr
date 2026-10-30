// Generated from LrDfa.g4 by ANTLR 4.13.2
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
pub const ID:i32=11; 
pub const INT:i32=12; 
pub const WS:i32=13;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;13] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "ID", "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
	None, Some("'['"), Some("']'"), Some("'!'"), Some("'-'"), Some("'^'"), 
	Some("'*'"), Some("'/'"), Some("'+'"), Some("'?'"), Some("':'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;14]  = [
	None, None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, LrDfaLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LrDfaLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { LrDfaLexer }
dbt_antlr_runtime::impl_deref! { lexer => LrDfaLexer }

impl<'input, 'arena, Input, TF> LrDfaLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = LrDfaLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct LrDfaLexerActions {
}

impl LrDfaLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { LrDfaLexerActions, "LrDfaLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 592] = [
    4, 0, 13, 64, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 1, 0, 1, 0, 1, 1, 1, 1, 
    1, 2, 1, 2, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 7, 1, 
    7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 4, 10, 49, 8, 10, 11, 10, 12, 10, 
    50, 1, 11, 4, 11, 54, 8, 11, 11, 11, 12, 11, 55, 1, 12, 4, 12, 59, 8, 
    12, 11, 12, 12, 12, 60, 1, 12, 1, 12, 0, 0, 13, 1, 1, 3, 2, 5, 3, 7, 
    4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 
    1, 0, 3, 1, 0, 97, 122, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 32, 32, 66, 
    0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 
    0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 
    0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 
    23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 1, 27, 1, 0, 0, 0, 3, 29, 1, 0, 0, 
    0, 5, 31, 1, 0, 0, 0, 7, 33, 1, 0, 0, 0, 9, 35, 1, 0, 0, 0, 11, 37, 
    1, 0, 0, 0, 13, 39, 1, 0, 0, 0, 15, 41, 1, 0, 0, 0, 17, 43, 1, 0, 0, 
    0, 19, 45, 1, 0, 0, 0, 21, 48, 1, 0, 0, 0, 23, 53, 1, 0, 0, 0, 25, 58, 
    1, 0, 0, 0, 27, 28, 5, 91, 0, 0, 28, 2, 1, 0, 0, 0, 29, 30, 5, 93, 0, 
    0, 30, 4, 1, 0, 0, 0, 31, 32, 5, 33, 0, 0, 32, 6, 1, 0, 0, 0, 33, 34, 
    5, 45, 0, 0, 34, 8, 1, 0, 0, 0, 35, 36, 5, 94, 0, 0, 36, 10, 1, 0, 0, 
    0, 37, 38, 5, 42, 0, 0, 38, 12, 1, 0, 0, 0, 39, 40, 5, 47, 0, 0, 40, 
    14, 1, 0, 0, 0, 41, 42, 5, 43, 0, 0, 42, 16, 1, 0, 0, 0, 43, 44, 5, 
    63, 0, 0, 44, 18, 1, 0, 0, 0, 45, 46, 5, 58, 0, 0, 46, 20, 1, 0, 0, 
    0, 47, 49, 7, 0, 0, 0, 48, 47, 1, 0, 0, 0, 49, 50, 1, 0, 0, 0, 50, 48, 
    1, 0, 0, 0, 50, 51, 1, 0, 0, 0, 51, 22, 1, 0, 0, 0, 52, 54, 7, 1, 0, 
    0, 53, 52, 1, 0, 0, 0, 54, 55, 1, 0, 0, 0, 55, 53, 1, 0, 0, 0, 55, 56, 
    1, 0, 0, 0, 56, 24, 1, 0, 0, 0, 57, 59, 7, 2, 0, 0, 58, 57, 1, 0, 0, 
    0, 59, 60, 1, 0, 0, 0, 60, 58, 1, 0, 0, 0, 60, 61, 1, 0, 0, 0, 61, 62, 
    1, 0, 0, 0, 62, 63, 6, 12, 0, 0, 63, 26, 1, 0, 0, 0, 4, 0, 50, 55, 60, 
    1, 6, 0, 0
];