// Generated from ReferenceToATN.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","5");
pub const ID:i32=1; 
pub const ATN:i32=2; 
pub const WS:i32=3;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;3] = [
    "ID", "ATN", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;0] = [
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;4]  = [
	None, Some("ID"), Some("ATN"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, ReferenceToATNLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ReferenceToATNLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { ReferenceToATNLexer }
dbt_antlr_runtime::impl_deref! { lexer => ReferenceToATNLexer }

impl<'input, 'arena, Input, TF> ReferenceToATNLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = ReferenceToATNLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct ReferenceToATNLexerActions {
}

impl ReferenceToATNLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { ReferenceToATNLexerActions, "ReferenceToATNLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 209] = [
    4, 0, 3, 21, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 1, 0, 4, 0, 
    9, 8, 0, 11, 0, 12, 0, 10, 1, 1, 4, 1, 14, 8, 1, 11, 1, 12, 1, 15, 1, 
    2, 1, 2, 1, 2, 1, 2, 0, 0, 3, 1, 1, 3, 2, 5, 3, 1, 0, 1, 2, 0, 10, 10, 
    32, 32, 22, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 1, 
    8, 1, 0, 0, 0, 3, 13, 1, 0, 0, 0, 5, 17, 1, 0, 0, 0, 7, 9, 2, 97, 122, 
    0, 8, 7, 1, 0, 0, 0, 9, 10, 1, 0, 0, 0, 10, 8, 1, 0, 0, 0, 10, 11, 1, 
    0, 0, 0, 11, 2, 1, 0, 0, 0, 12, 14, 2, 48, 57, 0, 13, 12, 1, 0, 0, 0, 
    14, 15, 1, 0, 0, 0, 15, 13, 1, 0, 0, 0, 15, 16, 1, 0, 0, 0, 16, 4, 1, 
    0, 0, 0, 17, 18, 7, 0, 0, 0, 18, 19, 1, 0, 0, 0, 19, 20, 6, 2, 0, 0, 
    20, 6, 1, 0, 0, 0, 3, 0, 10, 15, 1, 6, 0, 0
];