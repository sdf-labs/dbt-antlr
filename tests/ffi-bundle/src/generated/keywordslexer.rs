// Generated from KeywordsLexer.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","12");
pub const CLASS:i32=1; 
pub const LAMBDA:i32=2; 
pub const BOOM:i32=3; 
pub const ID:i32=4; 
pub const WS:i32=5; 
pub const END:i32=6;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;6] = [
    "CLASS", "LAMBDA", "BOOM", "END", "ID", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;7] = [
	None, Some("'class'"), Some("'lambda'"), Some("'boom'"), None, None, Some("'end'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, Some("CLASS"), Some("LAMBDA"), Some("BOOM"), Some("ID"), Some("WS"), 
	Some("END")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, KeywordsLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct KeywordsLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { KeywordsLexer }
dbt_antlr_runtime::impl_deref! { lexer => KeywordsLexer }

impl<'input, 'arena, Input, TF> KeywordsLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = KeywordsLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct KeywordsLexerActions {
}

impl KeywordsLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { KeywordsLexerActions, "KeywordsLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 448] = [
    4, 0, 6, 49, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 
    1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 4, 4, 4, 39, 8, 4, 11, 4, 12, 4, 40, 
    1, 5, 4, 5, 44, 8, 5, 11, 5, 12, 5, 45, 1, 5, 1, 5, 0, 0, 6, 1, 1, 3, 
    2, 5, 3, 7, 6, 9, 4, 11, 5, 1, 0, 2, 1, 0, 97, 122, 3, 0, 9, 10, 13, 
    13, 32, 32, 50, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 
    0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 1, 13, 1, 0, 
    0, 0, 3, 19, 1, 0, 0, 0, 5, 26, 1, 0, 0, 0, 7, 31, 1, 0, 0, 0, 9, 38, 
    1, 0, 0, 0, 11, 43, 1, 0, 0, 0, 13, 14, 5, 99, 0, 0, 14, 15, 5, 108, 
    0, 0, 15, 16, 5, 97, 0, 0, 16, 17, 5, 115, 0, 0, 17, 18, 5, 115, 0, 
    0, 18, 2, 1, 0, 0, 0, 19, 20, 5, 108, 0, 0, 20, 21, 5, 97, 0, 0, 21, 
    22, 5, 109, 0, 0, 22, 23, 5, 98, 0, 0, 23, 24, 5, 100, 0, 0, 24, 25, 
    5, 97, 0, 0, 25, 4, 1, 0, 0, 0, 26, 27, 5, 98, 0, 0, 27, 28, 5, 111, 
    0, 0, 28, 29, 5, 111, 0, 0, 29, 30, 5, 109, 0, 0, 30, 6, 1, 0, 0, 0, 
    31, 32, 5, 101, 0, 0, 32, 33, 5, 110, 0, 0, 33, 34, 5, 100, 0, 0, 34, 
    35, 1, 0, 0, 0, 35, 36, 6, 3, 0, 0, 36, 8, 1, 0, 0, 0, 37, 39, 7, 0, 
    0, 0, 38, 37, 1, 0, 0, 0, 39, 40, 1, 0, 0, 0, 40, 38, 1, 0, 0, 0, 40, 
    41, 1, 0, 0, 0, 41, 10, 1, 0, 0, 0, 42, 44, 7, 1, 0, 0, 43, 42, 1, 0, 
    0, 0, 44, 45, 1, 0, 0, 0, 45, 43, 1, 0, 0, 0, 45, 46, 1, 0, 0, 0, 46, 
    47, 1, 0, 0, 0, 47, 48, 6, 5, 1, 0, 48, 12, 1, 0, 0, 0, 3, 0, 40, 45, 
    2, 7, -1, 0, 6, 0, 0
];