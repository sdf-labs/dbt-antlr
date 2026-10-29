// Generated from Spike.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","5");
pub const COMMENT:i32=1; 
pub const KWLET:i32=2; 
pub const ID:i32=3; 
pub const INT:i32=4; 
pub const WS:i32=5; 
pub const UNPAIRED:i32=6;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;6] = [
    "COMMENT", "KWLET", "ID", "INT", "WS", "UNPAIRED"
];
pub const _LITERAL_NAMES: [Option<&'static str>;7] = [
	None, None, Some("'let'"), None, None, None, Some("'/*'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;7]  = [
	None, Some("COMMENT"), Some("KWLET"), Some("ID"), Some("INT"), Some("WS"), 
	Some("UNPAIRED")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, SpikeActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct Spike<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { Spike }
dbt_antlr_runtime::impl_deref! { lexer => Spike }

impl<'input, 'arena, Input, TF> Spike<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = SpikeActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct SpikeActions {
}

impl SpikeActions {
	fn KWLET_sempred<'arena, 'input, Input, TF>(pred_index:i32, recog: &mut BaseLexerType<'input, 'arena, Input, TF>) -> bool
	where
	    TF: TokenFactory<'input, 'arena> + 'arena,
	    Input: CharStream<'input>,
	 {
		match pred_index {
	        0 => {
			recog.input().la(1) == '=' as i32
		    }
		    _ => true
		}
	}
}

dbt_antlr_runtime::impl_lexer_recog! { SpikeActions, "Spike.g4"; sempred { 1 => KWLET_sempred, } }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 6, 52, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 5, 0, 19, 8, 0, 
    10, 0, 12, 0, 22, 9, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 2, 4, 2, 34, 8, 2, 11, 2, 12, 2, 35, 1, 3, 4, 3, 39, 8, 
    3, 11, 3, 12, 3, 40, 1, 4, 4, 4, 44, 8, 4, 11, 4, 12, 4, 45, 1, 4, 1, 
    4, 1, 5, 1, 5, 1, 5, 1, 20, 0, 6, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 
    6, 1, 0, 3, 1, 0, 97, 122, 1, 0, 48, 57, 3, 0, 9, 10, 13, 13, 32, 32, 
    56, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 
    0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 1, 13, 1, 0, 0, 0, 3, 26, 
    1, 0, 0, 0, 5, 33, 1, 0, 0, 0, 7, 38, 1, 0, 0, 0, 9, 43, 1, 0, 0, 0, 
    11, 49, 1, 0, 0, 0, 13, 14, 5, 47, 0, 0, 14, 15, 5, 42, 0, 0, 15, 20, 
    1, 0, 0, 0, 16, 19, 3, 1, 0, 0, 17, 19, 9, 0, 0, 0, 18, 16, 1, 0, 0, 
    0, 18, 17, 1, 0, 0, 0, 19, 22, 1, 0, 0, 0, 20, 21, 1, 0, 0, 0, 20, 18, 
    1, 0, 0, 0, 21, 23, 1, 0, 0, 0, 22, 20, 1, 0, 0, 0, 23, 24, 5, 42, 0, 
    0, 24, 25, 5, 47, 0, 0, 25, 2, 1, 0, 0, 0, 26, 27, 5, 108, 0, 0, 27, 
    28, 5, 101, 0, 0, 28, 29, 5, 116, 0, 0, 29, 30, 1, 0, 0, 0, 30, 31, 
    4, 1, 0, 0, 31, 4, 1, 0, 0, 0, 32, 34, 7, 0, 0, 0, 33, 32, 1, 0, 0, 
    0, 34, 35, 1, 0, 0, 0, 35, 33, 1, 0, 0, 0, 35, 36, 1, 0, 0, 0, 36, 6, 
    1, 0, 0, 0, 37, 39, 7, 1, 0, 0, 38, 37, 1, 0, 0, 0, 39, 40, 1, 0, 0, 
    0, 40, 38, 1, 0, 0, 0, 40, 41, 1, 0, 0, 0, 41, 8, 1, 0, 0, 0, 42, 44, 
    7, 2, 0, 0, 43, 42, 1, 0, 0, 0, 44, 45, 1, 0, 0, 0, 45, 43, 1, 0, 0, 
    0, 45, 46, 1, 0, 0, 0, 46, 47, 1, 0, 0, 0, 47, 48, 6, 4, 0, 0, 48, 10, 
    1, 0, 0, 0, 49, 50, 5, 47, 0, 0, 50, 51, 5, 42, 0, 0, 51, 12, 1, 0, 
    0, 0, 6, 0, 18, 20, 35, 40, 45, 1, 0, 1, 0
]);