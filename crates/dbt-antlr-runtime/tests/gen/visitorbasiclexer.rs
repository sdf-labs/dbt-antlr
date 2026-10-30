// Generated from VisitorBasic.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","6");
pub const A:i32=1;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;1] = [
    "A"
];
pub const _LITERAL_NAMES: [Option<&'static str>;2] = [
	None, Some("'A'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;2]  = [
	None, Some("A")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, VisitorBasicLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct VisitorBasicLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { VisitorBasicLexer }
dbt_antlr_runtime::impl_deref! { lexer => VisitorBasicLexer }

impl<'input, 'arena, Input, TF> VisitorBasicLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = VisitorBasicLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct VisitorBasicLexerActions {
}

impl VisitorBasicLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { VisitorBasicLexerActions, "VisitorBasicLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 50] = [
    4, 0, 1, 5, 6, -1, 2, 0, 7, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 4, 
    0, 1, 1, 0, 0, 0, 1, 3, 1, 0, 0, 0, 3, 4, 5, 65, 0, 0, 4, 2, 1, 0, 0, 
    0, 1, 0, 0
];