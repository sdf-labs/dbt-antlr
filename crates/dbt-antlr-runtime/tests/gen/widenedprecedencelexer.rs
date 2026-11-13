// Generated from WidenedPrecedence.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","9");
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
pub const T__10:i32=11; 
pub const T__11:i32=12; 
pub const T__12:i32=13; 
pub const T__13:i32=14; 
pub const T__14:i32=15; 
pub const T__15:i32=16; 
pub const T__16:i32=17; 
pub const T__17:i32=18; 
pub const T__18:i32=19; 
pub const T__19:i32=20; 
pub const T__20:i32=21; 
pub const T__21:i32=22; 
pub const T__22:i32=23; 
pub const T__23:i32=24; 
pub const T__24:i32=25; 
pub const T__25:i32=26; 
pub const T__26:i32=27; 
pub const T__27:i32=28; 
pub const T__28:i32=29; 
pub const T__29:i32=30; 
pub const T__30:i32=31; 
pub const T__31:i32=32; 
pub const WS:i32=33;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;33] = [
    "T__0", "T__1", "T__2", "T__3", "T__4", "T__5", "T__6", "T__7", "T__8", 
    "T__9", "T__10", "T__11", "T__12", "T__13", "T__14", "T__15", "T__16", 
    "T__17", "T__18", "T__19", "T__20", "T__21", "T__22", "T__23", "T__24", 
    "T__25", "T__26", "T__27", "T__28", "T__29", "T__30", "T__31", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;33] = [
	None, Some("'a'"), Some("'m1'"), Some("'*'"), Some("'x'"), Some("'+'"), 
	Some("'q'"), Some("'r'"), Some("'u'"), Some("'t0a'"), Some("'v'"), Some("'t0b'"), 
	Some("'t1a'"), Some("'t1b'"), Some("'t2a'"), Some("'t2b'"), Some("'t3a'"), 
	Some("'t3b'"), Some("'t4a'"), Some("'t4b'"), Some("'t5a'"), Some("'t5b'"), 
	Some("'t6a'"), Some("'t6b'"), Some("'t7a'"), Some("'t7b'"), Some("'t8a'"), 
	Some("'t8b'"), Some("'t9a'"), Some("'t9b'"), Some("'end'"), Some("'t10a'"), 
	Some("'t10b'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;34]  = [
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, None, None, None, None, None, None, None, 
	None, None, None, None, None, None, None, None, None, Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, WidenedPrecedenceLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct WidenedPrecedenceLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { WidenedPrecedenceLexer }
dbt_antlr_runtime::impl_deref! { lexer => WidenedPrecedenceLexer }

impl<'input, 'arena, Input, TF> WidenedPrecedenceLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = WidenedPrecedenceLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct WidenedPrecedenceLexerActions {
}

impl WidenedPrecedenceLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { WidenedPrecedenceLexerActions, "WidenedPrecedenceLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 1582] = [
    4, 0, 33, 187, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
    19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 
    24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 
    29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 1, 0, 1, 0, 1, 1, 1, 1, 
    1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 1, 6, 1, 
    7, 1, 7, 1, 8, 1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 
    10, 1, 11, 1, 11, 1, 11, 1, 11, 1, 12, 1, 12, 1, 12, 1, 12, 1, 13, 1, 
    13, 1, 13, 1, 13, 1, 14, 1, 14, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 
    15, 1, 16, 1, 16, 1, 16, 1, 16, 1, 17, 1, 17, 1, 17, 1, 17, 1, 18, 1, 
    18, 1, 18, 1, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 20, 1, 20, 1, 20, 1, 
    20, 1, 21, 1, 21, 1, 21, 1, 21, 1, 22, 1, 22, 1, 22, 1, 22, 1, 23, 1, 
    23, 1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 25, 1, 
    25, 1, 26, 1, 26, 1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 27, 1, 28, 1, 
    28, 1, 28, 1, 28, 1, 29, 1, 29, 1, 29, 1, 29, 1, 30, 1, 30, 1, 30, 1, 
    30, 1, 30, 1, 31, 1, 31, 1, 31, 1, 31, 1, 31, 1, 32, 4, 32, 182, 8, 
    32, 11, 32, 12, 32, 183, 1, 32, 1, 32, 0, 0, 33, 1, 1, 3, 2, 5, 3, 7, 
    4, 9, 5, 11, 6, 13, 7, 15, 8, 17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 
    27, 14, 29, 15, 31, 16, 33, 17, 35, 18, 37, 19, 39, 20, 41, 21, 43, 
    22, 45, 23, 47, 24, 49, 25, 51, 26, 53, 27, 55, 28, 57, 29, 59, 30, 
    61, 31, 63, 32, 65, 33, 1, 0, 0, 187, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 
    0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 
    0, 0, 0, 0, 13, 1, 0, 0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 
    19, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 
    0, 0, 27, 1, 0, 0, 0, 0, 29, 1, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 
    0, 0, 0, 0, 35, 1, 0, 0, 0, 0, 37, 1, 0, 0, 0, 0, 39, 1, 0, 0, 0, 0, 
    41, 1, 0, 0, 0, 0, 43, 1, 0, 0, 0, 0, 45, 1, 0, 0, 0, 0, 47, 1, 0, 0, 
    0, 0, 49, 1, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 53, 1, 0, 0, 0, 0, 55, 1, 
    0, 0, 0, 0, 57, 1, 0, 0, 0, 0, 59, 1, 0, 0, 0, 0, 61, 1, 0, 0, 0, 0, 
    63, 1, 0, 0, 0, 0, 65, 1, 0, 0, 0, 1, 67, 1, 0, 0, 0, 3, 69, 1, 0, 0, 
    0, 5, 72, 1, 0, 0, 0, 7, 74, 1, 0, 0, 0, 9, 76, 1, 0, 0, 0, 11, 78, 
    1, 0, 0, 0, 13, 80, 1, 0, 0, 0, 15, 82, 1, 0, 0, 0, 17, 84, 1, 0, 0, 
    0, 19, 88, 1, 0, 0, 0, 21, 90, 1, 0, 0, 0, 23, 94, 1, 0, 0, 0, 25, 98, 
    1, 0, 0, 0, 27, 102, 1, 0, 0, 0, 29, 106, 1, 0, 0, 0, 31, 110, 1, 0, 
    0, 0, 33, 114, 1, 0, 0, 0, 35, 118, 1, 0, 0, 0, 37, 122, 1, 0, 0, 0, 
    39, 126, 1, 0, 0, 0, 41, 130, 1, 0, 0, 0, 43, 134, 1, 0, 0, 0, 45, 138, 
    1, 0, 0, 0, 47, 142, 1, 0, 0, 0, 49, 146, 1, 0, 0, 0, 51, 150, 1, 0, 
    0, 0, 53, 154, 1, 0, 0, 0, 55, 158, 1, 0, 0, 0, 57, 162, 1, 0, 0, 0, 
    59, 166, 1, 0, 0, 0, 61, 170, 1, 0, 0, 0, 63, 175, 1, 0, 0, 0, 65, 181, 
    1, 0, 0, 0, 67, 68, 5, 97, 0, 0, 68, 2, 1, 0, 0, 0, 69, 70, 5, 109, 
    0, 0, 70, 71, 5, 49, 0, 0, 71, 4, 1, 0, 0, 0, 72, 73, 5, 42, 0, 0, 73, 
    6, 1, 0, 0, 0, 74, 75, 5, 120, 0, 0, 75, 8, 1, 0, 0, 0, 76, 77, 5, 43, 
    0, 0, 77, 10, 1, 0, 0, 0, 78, 79, 5, 113, 0, 0, 79, 12, 1, 0, 0, 0, 
    80, 81, 5, 114, 0, 0, 81, 14, 1, 0, 0, 0, 82, 83, 5, 117, 0, 0, 83, 
    16, 1, 0, 0, 0, 84, 85, 5, 116, 0, 0, 85, 86, 5, 48, 0, 0, 86, 87, 5, 
    97, 0, 0, 87, 18, 1, 0, 0, 0, 88, 89, 5, 118, 0, 0, 89, 20, 1, 0, 0, 
    0, 90, 91, 5, 116, 0, 0, 91, 92, 5, 48, 0, 0, 92, 93, 5, 98, 0, 0, 93, 
    22, 1, 0, 0, 0, 94, 95, 5, 116, 0, 0, 95, 96, 5, 49, 0, 0, 96, 97, 5, 
    97, 0, 0, 97, 24, 1, 0, 0, 0, 98, 99, 5, 116, 0, 0, 99, 100, 5, 49, 
    0, 0, 100, 101, 5, 98, 0, 0, 101, 26, 1, 0, 0, 0, 102, 103, 5, 116, 
    0, 0, 103, 104, 5, 50, 0, 0, 104, 105, 5, 97, 0, 0, 105, 28, 1, 0, 0, 
    0, 106, 107, 5, 116, 0, 0, 107, 108, 5, 50, 0, 0, 108, 109, 5, 98, 0, 
    0, 109, 30, 1, 0, 0, 0, 110, 111, 5, 116, 0, 0, 111, 112, 5, 51, 0, 
    0, 112, 113, 5, 97, 0, 0, 113, 32, 1, 0, 0, 0, 114, 115, 5, 116, 0, 
    0, 115, 116, 5, 51, 0, 0, 116, 117, 5, 98, 0, 0, 117, 34, 1, 0, 0, 0, 
    118, 119, 5, 116, 0, 0, 119, 120, 5, 52, 0, 0, 120, 121, 5, 97, 0, 0, 
    121, 36, 1, 0, 0, 0, 122, 123, 5, 116, 0, 0, 123, 124, 5, 52, 0, 0, 
    124, 125, 5, 98, 0, 0, 125, 38, 1, 0, 0, 0, 126, 127, 5, 116, 0, 0, 
    127, 128, 5, 53, 0, 0, 128, 129, 5, 97, 0, 0, 129, 40, 1, 0, 0, 0, 130, 
    131, 5, 116, 0, 0, 131, 132, 5, 53, 0, 0, 132, 133, 5, 98, 0, 0, 133, 
    42, 1, 0, 0, 0, 134, 135, 5, 116, 0, 0, 135, 136, 5, 54, 0, 0, 136, 
    137, 5, 97, 0, 0, 137, 44, 1, 0, 0, 0, 138, 139, 5, 116, 0, 0, 139, 
    140, 5, 54, 0, 0, 140, 141, 5, 98, 0, 0, 141, 46, 1, 0, 0, 0, 142, 143, 
    5, 116, 0, 0, 143, 144, 5, 55, 0, 0, 144, 145, 5, 97, 0, 0, 145, 48, 
    1, 0, 0, 0, 146, 147, 5, 116, 0, 0, 147, 148, 5, 55, 0, 0, 148, 149, 
    5, 98, 0, 0, 149, 50, 1, 0, 0, 0, 150, 151, 5, 116, 0, 0, 151, 152, 
    5, 56, 0, 0, 152, 153, 5, 97, 0, 0, 153, 52, 1, 0, 0, 0, 154, 155, 5, 
    116, 0, 0, 155, 156, 5, 56, 0, 0, 156, 157, 5, 98, 0, 0, 157, 54, 1, 
    0, 0, 0, 158, 159, 5, 116, 0, 0, 159, 160, 5, 57, 0, 0, 160, 161, 5, 
    97, 0, 0, 161, 56, 1, 0, 0, 0, 162, 163, 5, 116, 0, 0, 163, 164, 5, 
    57, 0, 0, 164, 165, 5, 98, 0, 0, 165, 58, 1, 0, 0, 0, 166, 167, 5, 101, 
    0, 0, 167, 168, 5, 110, 0, 0, 168, 169, 5, 100, 0, 0, 169, 60, 1, 0, 
    0, 0, 170, 171, 5, 116, 0, 0, 171, 172, 5, 49, 0, 0, 172, 173, 5, 48, 
    0, 0, 173, 174, 5, 97, 0, 0, 174, 62, 1, 0, 0, 0, 175, 176, 5, 116, 
    0, 0, 176, 177, 5, 49, 0, 0, 177, 178, 5, 48, 0, 0, 178, 179, 5, 98, 
    0, 0, 179, 64, 1, 0, 0, 0, 180, 182, 5, 32, 0, 0, 181, 180, 1, 0, 0, 
    0, 182, 183, 1, 0, 0, 0, 183, 181, 1, 0, 0, 0, 183, 184, 1, 0, 0, 0, 
    184, 185, 1, 0, 0, 0, 185, 186, 6, 32, 0, 0, 186, 66, 1, 0, 0, 0, 2, 
    0, 183, 1, 6, 0, 0
];