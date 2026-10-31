// Generated from LoopVeto.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","6");
pub const T__0:i32=1; 
pub const T__1:i32=2; 
pub const INT:i32=3; 
pub const WS:i32=4;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;4] = [
    "T__0", "T__1", "INT", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;3] = [
	None, Some("','"), Some("'>>'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;5]  = [
	None, None, None, Some("INT"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, LoopVetoLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct LoopVetoLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { LoopVetoLexer }
dbt_antlr_runtime::impl_deref! { lexer => LoopVetoLexer }

impl<'input, 'arena, Input, TF> LoopVetoLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = LoopVetoLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct LoopVetoLexerActions {
}

impl LoopVetoLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { LoopVetoLexerActions, "LoopVetoLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 257] = [
    4, 0, 4, 26, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 2, 4, 2, 16, 8, 2, 11, 2, 12, 2, 17, 
    1, 3, 4, 3, 21, 8, 3, 11, 3, 12, 3, 22, 1, 3, 1, 3, 0, 0, 4, 1, 1, 3, 
    2, 5, 3, 7, 4, 1, 0, 2, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 32, 32, 27, 
    0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 
    0, 1, 9, 1, 0, 0, 0, 3, 11, 1, 0, 0, 0, 5, 15, 1, 0, 0, 0, 7, 20, 1, 
    0, 0, 0, 9, 10, 5, 44, 0, 0, 10, 2, 1, 0, 0, 0, 11, 12, 5, 62, 0, 0, 
    12, 13, 5, 62, 0, 0, 13, 4, 1, 0, 0, 0, 14, 16, 7, 0, 0, 0, 15, 14, 
    1, 0, 0, 0, 16, 17, 1, 0, 0, 0, 17, 15, 1, 0, 0, 0, 17, 18, 1, 0, 0, 
    0, 18, 6, 1, 0, 0, 0, 19, 21, 7, 1, 0, 0, 20, 19, 1, 0, 0, 0, 21, 22, 
    1, 0, 0, 0, 22, 20, 1, 0, 0, 0, 22, 23, 1, 0, 0, 0, 23, 24, 1, 0, 0, 
    0, 24, 25, 6, 3, 0, 0, 25, 8, 1, 0, 0, 0, 3, 0, 17, 22, 1, 6, 0, 0
];