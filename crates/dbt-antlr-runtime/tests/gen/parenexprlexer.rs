// Generated from ParenExpr.g4 by ANTLR 4.13.2
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
pub const ID:i32=9; 
pub const INT:i32=10; 
pub const WS:i32=11;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;11] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "ID", 
    "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;9] = [
	None, Some("'('"), Some("','"), Some("')'"), Some("'-'"), Some("'*'"), 
	Some("'/'"), Some("'+'"), Some("'SELECT'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, None, None, None, None, None, None, None, None, Some("ID"), Some("INT"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, ParenExprLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ParenExprLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { ParenExprLexer }
dbt_antlr_runtime::impl_deref! { lexer => ParenExprLexer }

impl<'input, 'arena, Input, TF> ParenExprLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = ParenExprLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct ParenExprLexerActions {
}

impl ParenExprLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { ParenExprLexerActions, "ParenExprLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 562] = [
    4, 0, 11, 61, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 
    4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 
    1, 7, 1, 8, 4, 8, 46, 8, 8, 11, 8, 12, 8, 47, 1, 9, 4, 9, 51, 8, 9, 
    11, 9, 12, 9, 52, 1, 10, 4, 10, 56, 8, 10, 11, 10, 12, 10, 57, 1, 10, 
    1, 10, 0, 0, 11, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 
    17, 9, 19, 10, 21, 11, 1, 0, 3, 1, 0, 97, 122, 1, 0, 48, 57, 2, 0, 9, 
    10, 32, 32, 63, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 
    0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 
    0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 
    1, 0, 0, 0, 1, 23, 1, 0, 0, 0, 3, 25, 1, 0, 0, 0, 5, 27, 1, 0, 0, 0, 
    7, 29, 1, 0, 0, 0, 9, 31, 1, 0, 0, 0, 11, 33, 1, 0, 0, 0, 13, 35, 1, 
    0, 0, 0, 15, 37, 1, 0, 0, 0, 17, 45, 1, 0, 0, 0, 19, 50, 1, 0, 0, 0, 
    21, 55, 1, 0, 0, 0, 23, 24, 5, 40, 0, 0, 24, 2, 1, 0, 0, 0, 25, 26, 
    5, 44, 0, 0, 26, 4, 1, 0, 0, 0, 27, 28, 5, 41, 0, 0, 28, 6, 1, 0, 0, 
    0, 29, 30, 5, 45, 0, 0, 30, 8, 1, 0, 0, 0, 31, 32, 5, 42, 0, 0, 32, 
    10, 1, 0, 0, 0, 33, 34, 5, 47, 0, 0, 34, 12, 1, 0, 0, 0, 35, 36, 5, 
    43, 0, 0, 36, 14, 1, 0, 0, 0, 37, 38, 5, 83, 0, 0, 38, 39, 5, 69, 0, 
    0, 39, 40, 5, 76, 0, 0, 40, 41, 5, 69, 0, 0, 41, 42, 5, 67, 0, 0, 42, 
    43, 5, 84, 0, 0, 43, 16, 1, 0, 0, 0, 44, 46, 7, 0, 0, 0, 45, 44, 1, 
    0, 0, 0, 46, 47, 1, 0, 0, 0, 47, 45, 1, 0, 0, 0, 47, 48, 1, 0, 0, 0, 
    48, 18, 1, 0, 0, 0, 49, 51, 7, 1, 0, 0, 50, 49, 1, 0, 0, 0, 51, 52, 
    1, 0, 0, 0, 52, 50, 1, 0, 0, 0, 52, 53, 1, 0, 0, 0, 53, 20, 1, 0, 0, 
    0, 54, 56, 7, 2, 0, 0, 55, 54, 1, 0, 0, 0, 56, 57, 1, 0, 0, 0, 57, 55, 
    1, 0, 0, 0, 57, 58, 1, 0, 0, 0, 58, 59, 1, 0, 0, 0, 59, 60, 6, 10, 0, 
    0, 60, 22, 1, 0, 0, 0, 4, 0, 47, 52, 57, 1, 6, 0, 0
];