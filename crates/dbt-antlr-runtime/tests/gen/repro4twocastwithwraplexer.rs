// Generated from Repro4TwoCastWithWrap.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","13");
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
	None, Some("'WRAP'"), Some("'+'"), Some("'!'"), Some("'CAST'"), Some("'('"), 
	Some("'AS'"), Some("')'"), Some("'CAST2'"), Some("'ONLY_HIGH'"), Some("'*'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;13]  = [
	None, None, None, None, None, None, None, None, None, None, None, Some("ID"), 
	Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, Repro4TwoCastWithWrapLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct Repro4TwoCastWithWrapLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { Repro4TwoCastWithWrapLexer }
dbt_antlr_runtime::impl_deref! { lexer => Repro4TwoCastWithWrapLexer }

impl<'input, 'arena, Input, TF> Repro4TwoCastWithWrapLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = Repro4TwoCastWithWrapLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct Repro4TwoCastWithWrapLexerActions {
}

impl Repro4TwoCastWithWrapLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { Repro4TwoCastWithWrapLexerActions, "Repro4TwoCastWithWrapLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 671] = [
    4, 0, 12, 76, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 
    1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 
    1, 5, 1, 6, 1, 6, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 7, 1, 8, 1, 8, 1, 
    8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 4, 10, 
    66, 8, 10, 11, 10, 12, 10, 67, 1, 11, 4, 11, 71, 8, 11, 11, 11, 12, 
    11, 72, 1, 11, 1, 11, 0, 0, 12, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 
    13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 1, 0, 2, 1, 0, 97, 122, 
    2, 0, 9, 10, 32, 32, 77, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 
    0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 
    1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 
    0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 1, 25, 1, 0, 0, 0, 3, 30, 1, 0, 
    0, 0, 5, 32, 1, 0, 0, 0, 7, 34, 1, 0, 0, 0, 9, 39, 1, 0, 0, 0, 11, 41, 
    1, 0, 0, 0, 13, 44, 1, 0, 0, 0, 15, 46, 1, 0, 0, 0, 17, 52, 1, 0, 0, 
    0, 19, 62, 1, 0, 0, 0, 21, 65, 1, 0, 0, 0, 23, 70, 1, 0, 0, 0, 25, 26, 
    5, 87, 0, 0, 26, 27, 5, 82, 0, 0, 27, 28, 5, 65, 0, 0, 28, 29, 5, 80, 
    0, 0, 29, 2, 1, 0, 0, 0, 30, 31, 5, 43, 0, 0, 31, 4, 1, 0, 0, 0, 32, 
    33, 5, 33, 0, 0, 33, 6, 1, 0, 0, 0, 34, 35, 5, 67, 0, 0, 35, 36, 5, 
    65, 0, 0, 36, 37, 5, 83, 0, 0, 37, 38, 5, 84, 0, 0, 38, 8, 1, 0, 0, 
    0, 39, 40, 5, 40, 0, 0, 40, 10, 1, 0, 0, 0, 41, 42, 5, 65, 0, 0, 42, 
    43, 5, 83, 0, 0, 43, 12, 1, 0, 0, 0, 44, 45, 5, 41, 0, 0, 45, 14, 1, 
    0, 0, 0, 46, 47, 5, 67, 0, 0, 47, 48, 5, 65, 0, 0, 48, 49, 5, 83, 0, 
    0, 49, 50, 5, 84, 0, 0, 50, 51, 5, 50, 0, 0, 51, 16, 1, 0, 0, 0, 52, 
    53, 5, 79, 0, 0, 53, 54, 5, 78, 0, 0, 54, 55, 5, 76, 0, 0, 55, 56, 5, 
    89, 0, 0, 56, 57, 5, 95, 0, 0, 57, 58, 5, 72, 0, 0, 58, 59, 5, 73, 0, 
    0, 59, 60, 5, 71, 0, 0, 60, 61, 5, 72, 0, 0, 61, 18, 1, 0, 0, 0, 62, 
    63, 5, 42, 0, 0, 63, 20, 1, 0, 0, 0, 64, 66, 7, 0, 0, 0, 65, 64, 1, 
    0, 0, 0, 66, 67, 1, 0, 0, 0, 67, 65, 1, 0, 0, 0, 67, 68, 1, 0, 0, 0, 
    68, 22, 1, 0, 0, 0, 69, 71, 7, 1, 0, 0, 70, 69, 1, 0, 0, 0, 71, 72, 
    1, 0, 0, 0, 72, 70, 1, 0, 0, 0, 72, 73, 1, 0, 0, 0, 73, 74, 1, 0, 0, 
    0, 74, 75, 6, 11, 0, 0, 75, 24, 1, 0, 0, 0, 3, 0, 67, 72, 1, 6, 0, 0
];