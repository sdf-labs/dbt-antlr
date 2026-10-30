// Generated from SimpleLR.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","5");
pub const ID:i32=1; 
pub const WS:i32=2;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;2] = [
    "ID", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;0] = [
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;3]  = [
	None, Some("ID"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, SimpleLRLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct SimpleLRLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { SimpleLRLexer }
dbt_antlr_runtime::impl_deref! { lexer => SimpleLRLexer }

impl<'input, 'arena, Input, TF> SimpleLRLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = SimpleLRLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct SimpleLRLexerActions {
}

impl SimpleLRLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { SimpleLRLexerActions, "SimpleLRLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 142] = [
    4, 0, 2, 14, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 1, 0, 4, 0, 7, 8, 0, 11, 
    0, 12, 0, 8, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 2, 1, 1, 3, 2, 1, 0, 1, 2, 
    0, 10, 10, 32, 32, 14, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 1, 6, 1, 
    0, 0, 0, 3, 10, 1, 0, 0, 0, 5, 7, 2, 97, 122, 0, 6, 5, 1, 0, 0, 0, 7, 
    8, 1, 0, 0, 0, 8, 6, 1, 0, 0, 0, 8, 9, 1, 0, 0, 0, 9, 2, 1, 0, 0, 0, 
    10, 11, 7, 0, 0, 0, 11, 12, 1, 0, 0, 0, 12, 13, 6, 1, 0, 0, 13, 4, 1, 
    0, 0, 0, 2, 0, 8, 1, 6, 0, 0
];