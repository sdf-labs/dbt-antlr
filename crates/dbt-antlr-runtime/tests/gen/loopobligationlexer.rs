// Generated from LoopObligation.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","7");
pub const T__0:i32=1; 
pub const T__1:i32=2; 
pub const T__2:i32=3; 
pub const T__3:i32=4; 
pub const T__4:i32=5; 
pub const T__5:i32=6; 
pub const T__6:i32=7; 
pub const T__7:i32=8; 
pub const T__8:i32=9; 
pub const ID:i32=10; 
pub const WS:i32=11;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;11] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "ID", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;10] = [
	None, Some("'['"), Some("':'"), Some("']'"), Some("'.'"), Some("'+'"), 
	Some("'prior'"), Some("'trim'"), Some("'('"), Some("')'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, LoopObligationLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopObligationLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { LoopObligationLexer }
dbt_antlr_runtime::impl_deref! { lexer => LoopObligationLexer }

impl<'input, 'arena, Input, TF> LoopObligationLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = LoopObligationLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct LoopObligationLexerActions {
}

impl LoopObligationLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { LoopObligationLexerActions, "LoopObligationLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 543] = [
    4, 0, 11, 60, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 
    4, 1, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 1, 6, 1, 6, 1, 6, 1, 6, 
    1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 4, 9, 50, 8, 9, 11, 9, 12, 9, 51, 
    1, 10, 4, 10, 55, 8, 10, 11, 10, 12, 10, 56, 1, 10, 1, 10, 0, 0, 11, 
    1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 
    11, 1, 0, 2, 1, 0, 97, 122, 3, 0, 9, 10, 13, 13, 32, 32, 61, 0, 1, 1, 
    0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 
    1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 
    0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 1, 23, 1, 0, 
    0, 0, 3, 25, 1, 0, 0, 0, 5, 27, 1, 0, 0, 0, 7, 29, 1, 0, 0, 0, 9, 31, 
    1, 0, 0, 0, 11, 33, 1, 0, 0, 0, 13, 39, 1, 0, 0, 0, 15, 44, 1, 0, 0, 
    0, 17, 46, 1, 0, 0, 0, 19, 49, 1, 0, 0, 0, 21, 54, 1, 0, 0, 0, 23, 24, 
    5, 91, 0, 0, 24, 2, 1, 0, 0, 0, 25, 26, 5, 58, 0, 0, 26, 4, 1, 0, 0, 
    0, 27, 28, 5, 93, 0, 0, 28, 6, 1, 0, 0, 0, 29, 30, 5, 46, 0, 0, 30, 
    8, 1, 0, 0, 0, 31, 32, 5, 43, 0, 0, 32, 10, 1, 0, 0, 0, 33, 34, 5, 112, 
    0, 0, 34, 35, 5, 114, 0, 0, 35, 36, 5, 105, 0, 0, 36, 37, 5, 111, 0, 
    0, 37, 38, 5, 114, 0, 0, 38, 12, 1, 0, 0, 0, 39, 40, 5, 116, 0, 0, 40, 
    41, 5, 114, 0, 0, 41, 42, 5, 105, 0, 0, 42, 43, 5, 109, 0, 0, 43, 14, 
    1, 0, 0, 0, 44, 45, 5, 40, 0, 0, 45, 16, 1, 0, 0, 0, 46, 47, 5, 41, 
    0, 0, 47, 18, 1, 0, 0, 0, 48, 50, 7, 0, 0, 0, 49, 48, 1, 0, 0, 0, 50, 
    51, 1, 0, 0, 0, 51, 49, 1, 0, 0, 0, 51, 52, 1, 0, 0, 0, 52, 20, 1, 0, 
    0, 0, 53, 55, 7, 1, 0, 0, 54, 53, 1, 0, 0, 0, 55, 56, 1, 0, 0, 0, 56, 
    54, 1, 0, 0, 0, 56, 57, 1, 0, 0, 0, 57, 58, 1, 0, 0, 0, 58, 59, 6, 10, 
    0, 0, 59, 22, 1, 0, 0, 0, 3, 0, 51, 56, 1, 6, 0, 0
];