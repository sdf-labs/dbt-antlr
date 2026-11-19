// Generated from UnsoundSimple.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","10");
pub const A:i32=1; 
pub const B:i32=2; 
pub const C:i32=3; 
pub const D:i32=4; 
pub const E:i32=5; 
pub const M:i32=6; 
pub const P:i32=7; 
pub const Q:i32=8; 
pub const X:i32=9; 
pub const Y:i32=10; 
pub const WS:i32=11;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;11] = [
    "A", "B", "C", "D", "E", "M", "P", "Q", "X", "Y", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;11] = [
	None, Some("'a'"), Some("'b'"), Some("'c'"), Some("'d'"), Some("'e'"), 
	Some("'m'"), Some("'p'"), Some("'q'"), Some("'x'"), Some("'y'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;12]  = [
	None, Some("A"), Some("B"), Some("C"), Some("D"), Some("E"), Some("M"), 
	Some("P"), Some("Q"), Some("X"), Some("Y"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, UnsoundSimpleLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct UnsoundSimpleLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { UnsoundSimpleLexer }
dbt_antlr_runtime::impl_deref! { lexer => UnsoundSimpleLexer }

impl<'input, 'arena, Input, TF> UnsoundSimpleLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = UnsoundSimpleLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct UnsoundSimpleLexerActions {
}

impl UnsoundSimpleLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { UnsoundSimpleLexerActions, "UnsoundSimpleLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 442] = [
    4, 0, 11, 50, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 1, 0, 1, 0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 
    4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 1, 9, 
    1, 10, 4, 10, 45, 8, 10, 11, 10, 12, 10, 46, 1, 10, 1, 10, 0, 0, 11, 
    1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 
    11, 1, 0, 0, 50, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 
    0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 
    0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 
    21, 1, 0, 0, 0, 1, 23, 1, 0, 0, 0, 3, 25, 1, 0, 0, 0, 5, 27, 1, 0, 0, 
    0, 7, 29, 1, 0, 0, 0, 9, 31, 1, 0, 0, 0, 11, 33, 1, 0, 0, 0, 13, 35, 
    1, 0, 0, 0, 15, 37, 1, 0, 0, 0, 17, 39, 1, 0, 0, 0, 19, 41, 1, 0, 0, 
    0, 21, 44, 1, 0, 0, 0, 23, 24, 5, 97, 0, 0, 24, 2, 1, 0, 0, 0, 25, 26, 
    5, 98, 0, 0, 26, 4, 1, 0, 0, 0, 27, 28, 5, 99, 0, 0, 28, 6, 1, 0, 0, 
    0, 29, 30, 5, 100, 0, 0, 30, 8, 1, 0, 0, 0, 31, 32, 5, 101, 0, 0, 32, 
    10, 1, 0, 0, 0, 33, 34, 5, 109, 0, 0, 34, 12, 1, 0, 0, 0, 35, 36, 5, 
    112, 0, 0, 36, 14, 1, 0, 0, 0, 37, 38, 5, 113, 0, 0, 38, 16, 1, 0, 0, 
    0, 39, 40, 5, 120, 0, 0, 40, 18, 1, 0, 0, 0, 41, 42, 5, 121, 0, 0, 42, 
    20, 1, 0, 0, 0, 43, 45, 5, 32, 0, 0, 44, 43, 1, 0, 0, 0, 45, 46, 1, 
    0, 0, 0, 46, 44, 1, 0, 0, 0, 46, 47, 1, 0, 0, 0, 47, 48, 1, 0, 0, 0, 
    48, 49, 6, 10, 0, 0, 49, 22, 1, 0, 0, 0, 2, 0, 46, 1, 6, 0, 0
];