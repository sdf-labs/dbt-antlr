// Generated from WidenedTrust.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::prelude::*;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;

dbt_antlr_runtime::check_version!("0","9");
pub const A:i32=1; 
pub const B:i32=2; 
pub const C:i32=3; 
pub const D:i32=4; 
pub const E:i32=5; 
pub const M:i32=6; 
pub const N:i32=7; 
pub const P:i32=8; 
pub const Q:i32=9; 
pub const W:i32=10; 
pub const X:i32=11; 
pub const Y:i32=12; 
pub const Z:i32=13; 
pub const U0:i32=14; 
pub const V0:i32=15; 
pub const U1:i32=16; 
pub const V1:i32=17; 
pub const U2:i32=18; 
pub const V2:i32=19; 
pub const U3:i32=20; 
pub const V3:i32=21; 
pub const U4:i32=22; 
pub const V4:i32=23; 
pub const U5:i32=24; 
pub const V5:i32=25; 
pub const U6:i32=26; 
pub const V6:i32=27; 
pub const U7:i32=28; 
pub const V7:i32=29; 
pub const U8:i32=30; 
pub const V8:i32=31; 
pub const U9:i32=32; 
pub const V9:i32=33; 
pub const WS:i32=34;

pub const channelNames: [&'static str;0+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN"
];

pub const modeNames: [&'static str;1] = [
    "DEFAULT_MODE"
];

pub const ruleNames: [&'static str;34] = [
    "A", "B", "C", "D", "E", "M", "N", "P", "Q", "W", "X", "Y", "Z", "U0", 
    "V0", "U1", "V1", "U2", "V2", "U3", "V3", "U4", "V4", "U5", "V5", "U6", 
    "V6", "U7", "V7", "U8", "V8", "U9", "V9", "WS"
];
pub const _LITERAL_NAMES: [Option<&'static str>;34] = [
	None, Some("'a'"), Some("'b'"), Some("'c'"), Some("'d'"), Some("'e'"), 
	Some("'m'"), Some("'n'"), Some("'p'"), Some("'q'"), Some("'w'"), Some("'x'"), 
	Some("'y'"), Some("'z'"), Some("'u0'"), Some("'v0'"), Some("'u1'"), Some("'v1'"), 
	Some("'u2'"), Some("'v2'"), Some("'u3'"), Some("'v3'"), Some("'u4'"), Some("'v4'"), 
	Some("'u5'"), Some("'v5'"), Some("'u6'"), Some("'v6'"), Some("'u7'"), Some("'v7'"), 
	Some("'u8'"), Some("'v8'"), Some("'u9'"), Some("'v9'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;35]  = [
	None, Some("A"), Some("B"), Some("C"), Some("D"), Some("E"), Some("M"), 
	Some("N"), Some("P"), Some("Q"), Some("W"), Some("X"), Some("Y"), Some("Z"), 
	Some("U0"), Some("V0"), Some("U1"), Some("V1"), Some("U2"), Some("V2"), 
	Some("U3"), Some("V3"), Some("U4"), Some("V4"), Some("U5"), Some("V5"), 
	Some("U6"), Some("V6"), Some("U7"), Some("V7"), Some("U8"), Some("V8"), 
	Some("U9"), Some("V9"), Some("WS")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, WidenedTrustLexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct WidenedTrustLexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { WidenedTrustLexer }
dbt_antlr_runtime::impl_deref! { lexer => WidenedTrustLexer }

impl<'input, 'arena, Input, TF> WidenedTrustLexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = WidenedTrustLexerActions {
        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct WidenedTrustLexerActions {
}

impl WidenedTrustLexerActions {
}

dbt_antlr_runtime::impl_lexer_recog! { WidenedTrustLexerActions, "WidenedTrustLexer.g4" }

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: [i32; 1384] = [
    4, 0, 34, 162, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 
    2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 
    9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 
    14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 
    19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 
    24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 28, 2, 29, 7, 
    29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 33, 1, 0, 1, 
    0, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 1, 3, 1, 4, 1, 4, 1, 5, 1, 5, 1, 6, 
    1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 1, 9, 1, 9, 1, 10, 1, 10, 1, 11, 1, 11, 
    1, 12, 1, 12, 1, 13, 1, 13, 1, 13, 1, 14, 1, 14, 1, 14, 1, 15, 1, 15, 
    1, 15, 1, 16, 1, 16, 1, 16, 1, 17, 1, 17, 1, 17, 1, 18, 1, 18, 1, 18, 
    1, 19, 1, 19, 1, 19, 1, 20, 1, 20, 1, 20, 1, 21, 1, 21, 1, 21, 1, 22, 
    1, 22, 1, 22, 1, 23, 1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 
    1, 25, 1, 26, 1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 28, 1, 28, 1, 28, 
    1, 29, 1, 29, 1, 29, 1, 30, 1, 30, 1, 30, 1, 31, 1, 31, 1, 31, 1, 32, 
    1, 32, 1, 32, 1, 33, 4, 33, 157, 8, 33, 11, 33, 12, 33, 158, 1, 33, 
    1, 33, 0, 0, 34, 1, 1, 3, 2, 5, 3, 7, 4, 9, 5, 11, 6, 13, 7, 15, 8, 
    17, 9, 19, 10, 21, 11, 23, 12, 25, 13, 27, 14, 29, 15, 31, 16, 33, 17, 
    35, 18, 37, 19, 39, 20, 41, 21, 43, 22, 45, 23, 47, 24, 49, 25, 51, 
    26, 53, 27, 55, 28, 57, 29, 59, 30, 61, 31, 63, 32, 65, 33, 67, 34, 
    1, 0, 0, 162, 0, 1, 1, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 
    0, 7, 1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 
    0, 0, 0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 19, 1, 0, 0, 0, 0, 21, 
    1, 0, 0, 0, 0, 23, 1, 0, 0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 
    0, 29, 1, 0, 0, 0, 0, 31, 1, 0, 0, 0, 0, 33, 1, 0, 0, 0, 0, 35, 1, 0, 
    0, 0, 0, 37, 1, 0, 0, 0, 0, 39, 1, 0, 0, 0, 0, 41, 1, 0, 0, 0, 0, 43, 
    1, 0, 0, 0, 0, 45, 1, 0, 0, 0, 0, 47, 1, 0, 0, 0, 0, 49, 1, 0, 0, 0, 
    0, 51, 1, 0, 0, 0, 0, 53, 1, 0, 0, 0, 0, 55, 1, 0, 0, 0, 0, 57, 1, 0, 
    0, 0, 0, 59, 1, 0, 0, 0, 0, 61, 1, 0, 0, 0, 0, 63, 1, 0, 0, 0, 0, 65, 
    1, 0, 0, 0, 0, 67, 1, 0, 0, 0, 1, 69, 1, 0, 0, 0, 3, 71, 1, 0, 0, 0, 
    5, 73, 1, 0, 0, 0, 7, 75, 1, 0, 0, 0, 9, 77, 1, 0, 0, 0, 11, 79, 1, 
    0, 0, 0, 13, 81, 1, 0, 0, 0, 15, 83, 1, 0, 0, 0, 17, 85, 1, 0, 0, 0, 
    19, 87, 1, 0, 0, 0, 21, 89, 1, 0, 0, 0, 23, 91, 1, 0, 0, 0, 25, 93, 
    1, 0, 0, 0, 27, 95, 1, 0, 0, 0, 29, 98, 1, 0, 0, 0, 31, 101, 1, 0, 0, 
    0, 33, 104, 1, 0, 0, 0, 35, 107, 1, 0, 0, 0, 37, 110, 1, 0, 0, 0, 39, 
    113, 1, 0, 0, 0, 41, 116, 1, 0, 0, 0, 43, 119, 1, 0, 0, 0, 45, 122, 
    1, 0, 0, 0, 47, 125, 1, 0, 0, 0, 49, 128, 1, 0, 0, 0, 51, 131, 1, 0, 
    0, 0, 53, 134, 1, 0, 0, 0, 55, 137, 1, 0, 0, 0, 57, 140, 1, 0, 0, 0, 
    59, 143, 1, 0, 0, 0, 61, 146, 1, 0, 0, 0, 63, 149, 1, 0, 0, 0, 65, 152, 
    1, 0, 0, 0, 67, 156, 1, 0, 0, 0, 69, 70, 5, 97, 0, 0, 70, 2, 1, 0, 0, 
    0, 71, 72, 5, 98, 0, 0, 72, 4, 1, 0, 0, 0, 73, 74, 5, 99, 0, 0, 74, 
    6, 1, 0, 0, 0, 75, 76, 5, 100, 0, 0, 76, 8, 1, 0, 0, 0, 77, 78, 5, 101, 
    0, 0, 78, 10, 1, 0, 0, 0, 79, 80, 5, 109, 0, 0, 80, 12, 1, 0, 0, 0, 
    81, 82, 5, 110, 0, 0, 82, 14, 1, 0, 0, 0, 83, 84, 5, 112, 0, 0, 84, 
    16, 1, 0, 0, 0, 85, 86, 5, 113, 0, 0, 86, 18, 1, 0, 0, 0, 87, 88, 5, 
    119, 0, 0, 88, 20, 1, 0, 0, 0, 89, 90, 5, 120, 0, 0, 90, 22, 1, 0, 0, 
    0, 91, 92, 5, 121, 0, 0, 92, 24, 1, 0, 0, 0, 93, 94, 5, 122, 0, 0, 94, 
    26, 1, 0, 0, 0, 95, 96, 5, 117, 0, 0, 96, 97, 5, 48, 0, 0, 97, 28, 1, 
    0, 0, 0, 98, 99, 5, 118, 0, 0, 99, 100, 5, 48, 0, 0, 100, 30, 1, 0, 
    0, 0, 101, 102, 5, 117, 0, 0, 102, 103, 5, 49, 0, 0, 103, 32, 1, 0, 
    0, 0, 104, 105, 5, 118, 0, 0, 105, 106, 5, 49, 0, 0, 106, 34, 1, 0, 
    0, 0, 107, 108, 5, 117, 0, 0, 108, 109, 5, 50, 0, 0, 109, 36, 1, 0, 
    0, 0, 110, 111, 5, 118, 0, 0, 111, 112, 5, 50, 0, 0, 112, 38, 1, 0, 
    0, 0, 113, 114, 5, 117, 0, 0, 114, 115, 5, 51, 0, 0, 115, 40, 1, 0, 
    0, 0, 116, 117, 5, 118, 0, 0, 117, 118, 5, 51, 0, 0, 118, 42, 1, 0, 
    0, 0, 119, 120, 5, 117, 0, 0, 120, 121, 5, 52, 0, 0, 121, 44, 1, 0, 
    0, 0, 122, 123, 5, 118, 0, 0, 123, 124, 5, 52, 0, 0, 124, 46, 1, 0, 
    0, 0, 125, 126, 5, 117, 0, 0, 126, 127, 5, 53, 0, 0, 127, 48, 1, 0, 
    0, 0, 128, 129, 5, 118, 0, 0, 129, 130, 5, 53, 0, 0, 130, 50, 1, 0, 
    0, 0, 131, 132, 5, 117, 0, 0, 132, 133, 5, 54, 0, 0, 133, 52, 1, 0, 
    0, 0, 134, 135, 5, 118, 0, 0, 135, 136, 5, 54, 0, 0, 136, 54, 1, 0, 
    0, 0, 137, 138, 5, 117, 0, 0, 138, 139, 5, 55, 0, 0, 139, 56, 1, 0, 
    0, 0, 140, 141, 5, 118, 0, 0, 141, 142, 5, 55, 0, 0, 142, 58, 1, 0, 
    0, 0, 143, 144, 5, 117, 0, 0, 144, 145, 5, 56, 0, 0, 145, 60, 1, 0, 
    0, 0, 146, 147, 5, 118, 0, 0, 147, 148, 5, 56, 0, 0, 148, 62, 1, 0, 
    0, 0, 149, 150, 5, 117, 0, 0, 150, 151, 5, 57, 0, 0, 151, 64, 1, 0, 
    0, 0, 152, 153, 5, 118, 0, 0, 153, 154, 5, 57, 0, 0, 154, 66, 1, 0, 
    0, 0, 155, 157, 5, 32, 0, 0, 156, 155, 1, 0, 0, 0, 157, 158, 1, 0, 0, 
    0, 158, 156, 1, 0, 0, 0, 158, 159, 1, 0, 0, 0, 159, 160, 1, 0, 0, 0, 
    160, 161, 6, 33, 0, 0, 161, 68, 1, 0, 0, 0, 2, 0, 158, 1, 6, 0, 0
];