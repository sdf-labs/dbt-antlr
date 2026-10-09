// Generated from ANTLRv4Lexer.g4 by ANTLR 4.13.2

use dbt_antlr_runtime::token::TOKEN_INVALID_TYPE;

use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::char_stream::CharStream;
use dbt_antlr_runtime::int_stream::IntStream;
use dbt_antlr_runtime::lexer::{BaseLexer, LexerRecog, Lexer as _};
use dbt_antlr_runtime::atn_config_set::LexerATNConfigSet;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::atn_simulator::BaseATNSimulator;
use dbt_antlr_runtime::atn_simulator::LexerATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr_runtime::TokenSource;
use dbt_antlr_runtime::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use dbt_antlr_runtime::PredictionContextCache;
use dbt_antlr_runtime::recognizer::Actions;
use dbt_antlr_runtime::token_factory::{CommonTokenFactory, TokenFactory};
use dbt_antlr_runtime::rule_context::{BaseRuleContext,EmptyNodeKind,EmptyCustomRuleContext,EmptyRuleNode};
use dbt_antlr_runtime::vocabulary::{Vocabulary,VocabularyImpl};

use std::ops::{DerefMut, Deref};
use std::sync::LazyLock;

dbt_antlr_runtime::check_version!("0","1");
pub const TOKEN_REF:i32=1; 
pub const RULE_REF:i32=2; 
pub const LEXER_CHAR_SET:i32=3; 
pub const DOC_COMMENT:i32=4; 
pub const BLOCK_COMMENT:i32=5; 
pub const LINE_COMMENT:i32=6; 
pub const INT:i32=7; 
pub const STRING_LITERAL:i32=8; 
pub const UNTERMINATED_STRING_LITERAL:i32=9; 
pub const BEGIN_ARGUMENT:i32=10; 
pub const ACTION:i32=11; 
pub const OPTIONS:i32=12; 
pub const TOKENS:i32=13; 
pub const CHANNELS:i32=14; 
pub const IMPORT:i32=15; 
pub const FRAGMENT:i32=16; 
pub const LEXER:i32=17; 
pub const PARSER:i32=18; 
pub const GRAMMAR:i32=19; 
pub const PROTECTED:i32=20; 
pub const PUBLIC:i32=21; 
pub const PRIVATE:i32=22; 
pub const RETURNS:i32=23; 
pub const LOCALS:i32=24; 
pub const THROWS:i32=25; 
pub const CATCH:i32=26; 
pub const FINALLY:i32=27; 
pub const MODE:i32=28; 
pub const COLON:i32=29; 
pub const COLONCOLON:i32=30; 
pub const COMMA:i32=31; 
pub const SEMI:i32=32; 
pub const LPAREN:i32=33; 
pub const RPAREN:i32=34; 
pub const RBRACE:i32=35; 
pub const RARROW:i32=36; 
pub const LT:i32=37; 
pub const GT:i32=38; 
pub const ASSIGN:i32=39; 
pub const QUESTION:i32=40; 
pub const STAR:i32=41; 
pub const PLUS_ASSIGN:i32=42; 
pub const PLUS:i32=43; 
pub const OR:i32=44; 
pub const DOLLAR:i32=45; 
pub const RANGE:i32=46; 
pub const DOT:i32=47; 
pub const AT:i32=48; 
pub const POUND:i32=49; 
pub const NOT:i32=50; 
pub const ID:i32=51; 
pub const WS:i32=52; 
pub const END_ARGUMENT:i32=53; 
pub const UNTERMINATED_ARGUMENT:i32=54; 
pub const ARGUMENT_CONTENT:i32=55; 
pub const UNTERMINATED_CHAR_SET:i32=56;

pub const channelNames: [&'static str;2+2] = [
    "DEFAULT_TOKEN_CHANNEL", "HIDDEN", "OFF_CHANNEL", "COMMENT"
];

pub const modeNames: [&'static str;3] = [
    "DEFAULT_MODE", "Argument", "LexerCharSet"
];

pub const ruleNames: [&'static str;68] = [
    "DOC_COMMENT", "BLOCK_COMMENT", "LINE_COMMENT", "INT", "STRING_LITERAL", 
    "UNTERMINATED_STRING_LITERAL", "BEGIN_ARGUMENT", "ACTION", "NESTED_ACTION", 
    "OPTIONS", "TOKENS", "CHANNELS", "IMPORT", "FRAGMENT", "LEXER", "PARSER", 
    "GRAMMAR", "PROTECTED", "PUBLIC", "PRIVATE", "RETURNS", "LOCALS", "THROWS", 
    "CATCH", "FINALLY", "MODE", "COLON", "COLONCOLON", "COMMA", "SEMI", 
    "LPAREN", "RPAREN", "RBRACE", "RARROW", "LT", "GT", "ASSIGN", "QUESTION", 
    "STAR", "PLUS_ASSIGN", "PLUS", "OR", "DOLLAR", "RANGE", "DOT", "AT", 
    "POUND", "NOT", "ID", "WS", "NESTED_ARGUMENT", "ARGUMENT_ESCAPE", "ARGUMENT_STRING_LITERAL", 
    "ARGUMENT_CHAR_LITERAL", "END_ARGUMENT", "UNTERMINATED_ARGUMENT", "ARGUMENT_CONTENT", 
    "LEXER_CHAR_SET_BODY", "LEXER_CHAR_SET", "UNTERMINATED_CHAR_SET", "ESC_SEQUENCE", 
    "HexDigit", "UnicodeESC", "DoubleQuoteLiteral", "TripleQuoteLiteral", 
    "BacktickQuoteLiteral", "NameChar", "NameStartChar"
];
pub const _LITERAL_NAMES: [Option<&'static str>;51] = [
	None, None, None, None, None, None, None, None, None, None, Some("'['"), 
	None, None, None, None, Some("'import'"), Some("'fragment'"), Some("'lexer'"), 
	Some("'parser'"), Some("'grammar'"), Some("'protected'"), Some("'public'"), 
	Some("'private'"), Some("'returns'"), Some("'locals'"), Some("'throws'"), 
	Some("'catch'"), Some("'finally'"), Some("'mode'"), Some("':'"), Some("'::'"), 
	Some("','"), Some("';'"), Some("'('"), Some("')'"), Some("'}'"), Some("'->'"), 
	Some("'<'"), Some("'>'"), Some("'='"), Some("'?'"), Some("'*'"), Some("'+='"), 
	Some("'+'"), Some("'|'"), Some("'$'"), Some("'..'"), Some("'.'"), Some("'@'"), 
	Some("'#'"), Some("'~'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;57]  = [
	None, Some("TOKEN_REF"), Some("RULE_REF"), Some("LEXER_CHAR_SET"), Some("DOC_COMMENT"), 
	Some("BLOCK_COMMENT"), Some("LINE_COMMENT"), Some("INT"), Some("STRING_LITERAL"), 
	Some("UNTERMINATED_STRING_LITERAL"), Some("BEGIN_ARGUMENT"), Some("ACTION"), 
	Some("OPTIONS"), Some("TOKENS"), Some("CHANNELS"), Some("IMPORT"), Some("FRAGMENT"), 
	Some("LEXER"), Some("PARSER"), Some("GRAMMAR"), Some("PROTECTED"), Some("PUBLIC"), 
	Some("PRIVATE"), Some("RETURNS"), Some("LOCALS"), Some("THROWS"), Some("CATCH"), 
	Some("FINALLY"), Some("MODE"), Some("COLON"), Some("COLONCOLON"), Some("COMMA"), 
	Some("SEMI"), Some("LPAREN"), Some("RPAREN"), Some("RBRACE"), Some("RARROW"), 
	Some("LT"), Some("GT"), Some("ASSIGN"), Some("QUESTION"), Some("STAR"), 
	Some("PLUS_ASSIGN"), Some("PLUS"), Some("OR"), Some("DOLLAR"), Some("RANGE"), 
	Some("DOT"), Some("AT"), Some("POUND"), Some("NOT"), Some("ID"), Some("WS"), 
	Some("END_ARGUMENT"), Some("UNTERMINATED_ARGUMENT"), Some("ARGUMENT_CONTENT"), 
	Some("UNTERMINATED_CHAR_SET")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

const PREQUEL_CONSTRUCT: i32 = -10;
const OPTIONS_CONSTRUCT: i32 = -11;
const ARGUMENT_MODE: usize = 1;
const LEXER_CHAR_SET_MODE: usize = 2;


pub type LexerContext<'input, 'arena> = BaseRuleContext<'input, 'arena, EmptyNodeKind, EmptyCustomRuleContext<'input, 'arena>>;
pub type BaseLexerType<'input, 'arena, Input, TF> = BaseLexer<'input, 'arena, ANTLRv4LexerActions, Input, TF>;
pub fn lexer_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ANTLRv4Lexer<'input, 'arena, Input, TF = CommonTokenFactory<'input, 'arena>>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
	base: BaseLexerType<'input, 'arena, Input, TF>,
}

dbt_antlr_runtime::impl_token_source! { ANTLRv4Lexer }
dbt_antlr_runtime::impl_deref! { lexer => ANTLRv4Lexer }

impl<'input, 'arena, Input, TF> ANTLRv4Lexer<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: CharStream<'input>,
{
    pub fn new(arena: &'arena Arena, input: Input) -> Self {
        let actions = ANTLRv4LexerActions {

                current_rule_type: TOKEN_INVALID_TYPE,
                enclosing_rule_type: None,

        };
        let base = BaseLexerType::new_base_lexer(input, actions, arena);
        Self { base }
    }
}

pub struct ANTLRv4LexerActions {

	    current_rule_type: i32,
	    enclosing_rule_type: Option<i32>,

}

impl ANTLRv4LexerActions {
	fn BEGIN_ARGUMENT_action<'input, 'arena, Input, TF>(action_index: i32, recog: &mut BaseLexerType<'input, 'arena, Input, TF>)
	where
	    TF: TokenFactory<'input, 'arena> + 'arena,
	    Input: CharStream<'input>,
	{
		match action_index {
	        0 => {

			        if recog.current_rule_type == TOKEN_REF {
			            recog.push_mode(LEXER_CHAR_SET_MODE);
			            recog.more();
			        } else {
			            recog.push_mode(ARGUMENT_MODE);
			        }
			    
	        },
			_ => {}
		}
	}
	fn END_ARGUMENT_action<'input, 'arena, Input, TF>(action_index: i32, recog: &mut BaseLexerType<'input, 'arena, Input, TF>)
	where
	    TF: TokenFactory<'input, 'arena> + 'arena,
	    Input: CharStream<'input>,
	{
		match action_index {
	        1 => {

			        if recog.pop_mode() == Some(ARGUMENT_MODE) {
			            recog.set_type(ARGUMENT_CONTENT);
			        }
			    
	        },
			_ => {}
		}
	}
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseLexerType<'input, 'arena, Input, TF>, TF::Tok>
    for ANTLRv4LexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{
    fn action(_localctx: Option<&EmptyRuleNode<'input, 'arena, TF::Tok>>, rule_index: i32, action_index: i32, recog:&mut BaseLexerType<'input, 'arena, Input, TF>) {
        match rule_index {
            6 => ANTLRv4LexerActions::BEGIN_ARGUMENT_action(action_index, recog), 
            54 => ANTLRv4LexerActions::END_ARGUMENT_action(action_index, recog), 
            _ => {}
        }
    }
}
impl<'input, 'arena, Input, TF> LexerRecog<'input, 'arena, TF, BaseLexerType<'input, 'arena, Input, TF>>
    for ANTLRv4LexerActions
where
    'input: 'arena,
    Input: CharStream<'input>,
    TF: TokenFactory<'input, 'arena> + 'arena,
{

	    fn before_emit(lexer: &mut BaseLexerType<'input, 'arena, Input, TF>) {
	        let token_type = lexer.token_type;
	        if matches!(token_type, OPTIONS | TOKENS | CHANNELS)
	            && lexer.current_rule_type == TOKEN_INVALID_TYPE
	        {
	            lexer.current_rule_type = PREQUEL_CONSTRUCT;
	        } else if token_type == OPTIONS && matches!(lexer.current_rule_type, RULE_REF | TOKEN_REF) {
	            lexer.enclosing_rule_type = Some(lexer.current_rule_type);
	            lexer.current_rule_type = OPTIONS_CONSTRUCT;
	        } else if token_type == RBRACE && lexer.current_rule_type == PREQUEL_CONSTRUCT {
	            lexer.current_rule_type = TOKEN_INVALID_TYPE;
	        } else if token_type == RBRACE && lexer.current_rule_type == OPTIONS_CONSTRUCT {
	            lexer.current_rule_type = lexer.enclosing_rule_type.take().unwrap_or(TOKEN_INVALID_TYPE);
	        } else if token_type == AT && lexer.current_rule_type == TOKEN_INVALID_TYPE {
	            lexer.current_rule_type = AT;
	        } else if token_type == SEMI && lexer.current_rule_type == OPTIONS_CONSTRUCT {
	            // The option terminator does not end the surrounding rule.
	        } else if token_type == ACTION && lexer.current_rule_type == AT {
	            lexer.current_rule_type = TOKEN_INVALID_TYPE;
	        } else if token_type == ID {
	            let first = lexer.get_text().chars().next().expect("ID tokens are non-empty");
	            let classified = if first.is_uppercase() { TOKEN_REF } else { RULE_REF };
	            lexer.set_type(classified);
	            if lexer.current_rule_type == TOKEN_INVALID_TYPE {
	                lexer.current_rule_type = classified;
	            }
	        } else if token_type == SEMI {
	            lexer.current_rule_type = TOKEN_INVALID_TYPE;
	        }
	    }

    fn get_rule_names(&self) -> &'static [&'static str] { &ruleNames }
    fn get_literal_names(&self) -> &[Option<&str>] { &_LITERAL_NAMES }
    fn get_symbolic_names(&self) -> &[Option<&str>] { &_SYMBOLIC_NAMES }
    fn get_grammar_file_name(&self) -> &'static str { "ANTLRv4Lexer.g4" }
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> =
    LazyLock::new(|| ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
static _serializedATN: LazyLock<Vec<i32>> = LazyLock::new(|| vec![
    4, 0, 56, 573, 6, -1, 6, -1, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 
    2, 2, 3, 7, 3, 2, 4, 7, 4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 
    7, 8, 2, 9, 7, 9, 2, 10, 7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 
    13, 2, 14, 7, 14, 2, 15, 7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 
    18, 2, 19, 7, 19, 2, 20, 7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 
    23, 2, 24, 7, 24, 2, 25, 7, 25, 2, 26, 7, 26, 2, 27, 7, 27, 2, 28, 7, 
    28, 2, 29, 7, 29, 2, 30, 7, 30, 2, 31, 7, 31, 2, 32, 7, 32, 2, 33, 7, 
    33, 2, 34, 7, 34, 2, 35, 7, 35, 2, 36, 7, 36, 2, 37, 7, 37, 2, 38, 7, 
    38, 2, 39, 7, 39, 2, 40, 7, 40, 2, 41, 7, 41, 2, 42, 7, 42, 2, 43, 7, 
    43, 2, 44, 7, 44, 2, 45, 7, 45, 2, 46, 7, 46, 2, 47, 7, 47, 2, 48, 7, 
    48, 2, 49, 7, 49, 2, 50, 7, 50, 2, 51, 7, 51, 2, 52, 7, 52, 2, 53, 7, 
    53, 2, 54, 7, 54, 2, 55, 7, 55, 2, 56, 7, 56, 2, 57, 7, 57, 2, 58, 7, 
    58, 2, 59, 7, 59, 2, 60, 7, 60, 2, 61, 7, 61, 2, 62, 7, 62, 2, 63, 7, 
    63, 2, 64, 7, 64, 2, 65, 7, 65, 2, 66, 7, 66, 2, 67, 7, 67, 1, 0, 1, 
    0, 1, 0, 1, 0, 1, 0, 5, 0, 145, 8, 0, 10, 0, 12, 0, 148, 9, 0, 1, 0, 
    1, 0, 1, 0, 3, 0, 153, 8, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 5, 
    1, 161, 8, 1, 10, 1, 12, 1, 164, 9, 1, 1, 1, 1, 1, 1, 1, 3, 1, 169, 
    8, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 2, 1, 2, 5, 2, 177, 8, 2, 10, 2, 12, 
    2, 180, 9, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 5, 3, 187, 8, 3, 10, 3, 
    12, 3, 190, 9, 3, 3, 3, 192, 8, 3, 1, 4, 1, 4, 1, 4, 5, 4, 197, 8, 4, 
    10, 4, 12, 4, 200, 9, 4, 1, 4, 1, 4, 1, 5, 1, 5, 1, 5, 5, 5, 207, 8, 
    5, 10, 5, 12, 5, 210, 9, 5, 1, 6, 1, 6, 1, 6, 1, 7, 1, 7, 1, 8, 1, 8, 
    1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 5, 8, 227, 8, 8, 10, 
    8, 12, 8, 230, 9, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 1, 8, 5, 8, 238, 
    8, 8, 10, 8, 12, 8, 241, 9, 8, 1, 8, 1, 8, 1, 8, 1, 8, 5, 8, 247, 8, 
    8, 10, 8, 12, 8, 250, 9, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 
    1, 9, 1, 9, 1, 9, 1, 9, 5, 9, 263, 8, 9, 10, 9, 12, 9, 266, 9, 9, 1, 
    9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 5, 
    10, 278, 8, 10, 10, 10, 12, 10, 281, 9, 10, 1, 10, 1, 10, 1, 11, 1, 
    11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 5, 11, 295, 
    8, 11, 10, 11, 12, 11, 298, 9, 11, 1, 11, 1, 11, 1, 12, 1, 12, 1, 12, 
    1, 12, 1, 12, 1, 12, 1, 12, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 1, 13, 
    1, 13, 1, 13, 1, 13, 1, 14, 1, 14, 1, 14, 1, 14, 1, 14, 1, 14, 1, 15, 
    1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 1, 16, 1, 16, 1, 16, 1, 16, 
    1, 16, 1, 16, 1, 16, 1, 16, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 1, 17, 
    1, 17, 1, 17, 1, 17, 1, 17, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 18, 
    1, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 20, 
    1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 20, 1, 21, 1, 21, 1, 21, 
    1, 21, 1, 21, 1, 21, 1, 21, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 1, 22, 
    1, 22, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 23, 1, 24, 1, 24, 1, 24, 
    1, 24, 1, 24, 1, 24, 1, 24, 1, 24, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 
    1, 26, 1, 26, 1, 27, 1, 27, 1, 27, 1, 28, 1, 28, 1, 29, 1, 29, 1, 30, 
    1, 30, 1, 31, 1, 31, 1, 32, 1, 32, 1, 33, 1, 33, 1, 33, 1, 34, 1, 34, 
    1, 35, 1, 35, 1, 36, 1, 36, 1, 37, 1, 37, 1, 38, 1, 38, 1, 39, 1, 39, 
    1, 39, 1, 40, 1, 40, 1, 41, 1, 41, 1, 42, 1, 42, 1, 43, 1, 43, 1, 43, 
    1, 44, 1, 44, 1, 45, 1, 45, 1, 46, 1, 46, 1, 47, 1, 47, 1, 48, 1, 48, 
    5, 48, 455, 8, 48, 10, 48, 12, 48, 458, 9, 48, 1, 49, 4, 49, 461, 8, 
    49, 11, 49, 12, 49, 462, 1, 49, 1, 49, 1, 50, 1, 50, 1, 50, 1, 50, 1, 
    50, 1, 51, 1, 51, 1, 51, 1, 51, 1, 51, 1, 52, 1, 52, 1, 52, 1, 52, 1, 
    53, 1, 53, 1, 53, 1, 53, 1, 54, 1, 54, 1, 54, 1, 55, 1, 55, 1, 55, 1, 
    55, 1, 56, 1, 56, 1, 57, 1, 57, 1, 57, 4, 57, 497, 8, 57, 11, 57, 12, 
    57, 498, 1, 57, 1, 57, 1, 58, 1, 58, 1, 58, 1, 58, 1, 59, 1, 59, 1, 
    59, 1, 59, 1, 60, 1, 60, 1, 60, 1, 60, 1, 60, 3, 60, 516, 8, 60, 1, 
    61, 1, 61, 1, 62, 1, 62, 1, 62, 1, 62, 1, 62, 3, 62, 525, 8, 62, 3, 
    62, 527, 8, 62, 3, 62, 529, 8, 62, 3, 62, 531, 8, 62, 1, 63, 1, 63, 
    1, 63, 5, 63, 536, 8, 63, 10, 63, 12, 63, 539, 9, 63, 1, 63, 1, 63, 
    1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 64, 5, 64, 549, 8, 64, 10, 64, 
    12, 64, 552, 9, 64, 1, 64, 1, 64, 1, 64, 1, 64, 1, 65, 1, 65, 1, 65, 
    5, 65, 561, 8, 65, 10, 65, 12, 65, 564, 9, 65, 1, 65, 1, 65, 1, 66, 
    1, 66, 3, 66, 570, 8, 66, 1, 67, 1, 67, 7, 146, 162, 228, 248, 537, 
    550, 562, 0, 68, 3, 4, 5, 5, 7, 6, 9, 7, 11, 8, 13, 9, 15, 10, 17, 11, 
    19, 0, 21, 12, 23, 13, 25, 14, 27, 15, 29, 16, 31, 17, 33, 18, 35, 19, 
    37, 20, 39, 21, 41, 22, 43, 23, 45, 24, 47, 25, 49, 26, 51, 27, 53, 
    28, 55, 29, 57, 30, 59, 31, 61, 32, 63, 33, 65, 34, 67, 35, 69, 36, 
    71, 37, 73, 38, 75, 39, 77, 40, 79, 41, 81, 42, 83, 43, 85, 44, 87, 
    45, 89, 46, 91, 47, 93, 48, 95, 49, 97, 50, 99, 51, 101, 52, 103, 0, 
    105, 0, 107, 0, 109, 0, 111, 53, 113, 54, 115, 55, 117, 0, 119, 3, 121, 
    56, 123, 0, 125, 0, 127, 0, 129, 0, 131, 0, 133, 0, 135, 0, 137, 0, 
    3, 0, 1, 2, 13, 2, 0, 10, 10, 13, 13, 1, 0, 49, 57, 1, 0, 48, 57, 4, 
    0, 10, 10, 13, 13, 39, 39, 92, 92, 5, 0, 34, 34, 39, 39, 92, 92, 96, 
    96, 123, 123, 3, 0, 34, 34, 39, 39, 96, 96, 3, 0, 9, 10, 12, 13, 32, 
    32, 1, 0, 92, 93, 8, 0, 34, 34, 39, 39, 92, 92, 98, 98, 102, 102, 110, 
    110, 114, 114, 116, 116, 3, 0, 48, 57, 65, 70, 97, 102, 4, 0, 10, 10, 
    13, 13, 34, 34, 92, 92, 5, 0, 48, 57, 95, 95, 183, 183, 768, 879, 8255, 
    8256, 13, 0, 65, 90, 97, 122, 192, 214, 216, 246, 248, 767, 880, 893, 
    895, 8191, 8204, 8205, 8304, 8591, 11264, 12271, 12289, 55295, 63744, 
    64975, 65008, 65533, 605, 0, 3, 1, 0, 0, 0, 0, 5, 1, 0, 0, 0, 0, 7, 
    1, 0, 0, 0, 0, 9, 1, 0, 0, 0, 0, 11, 1, 0, 0, 0, 0, 13, 1, 0, 0, 0, 
    0, 15, 1, 0, 0, 0, 0, 17, 1, 0, 0, 0, 0, 21, 1, 0, 0, 0, 0, 23, 1, 0, 
    0, 0, 0, 25, 1, 0, 0, 0, 0, 27, 1, 0, 0, 0, 0, 29, 1, 0, 0, 0, 0, 31, 
    1, 0, 0, 0, 0, 33, 1, 0, 0, 0, 0, 35, 1, 0, 0, 0, 0, 37, 1, 0, 0, 0, 
    0, 39, 1, 0, 0, 0, 0, 41, 1, 0, 0, 0, 0, 43, 1, 0, 0, 0, 0, 45, 1, 0, 
    0, 0, 0, 47, 1, 0, 0, 0, 0, 49, 1, 0, 0, 0, 0, 51, 1, 0, 0, 0, 0, 53, 
    1, 0, 0, 0, 0, 55, 1, 0, 0, 0, 0, 57, 1, 0, 0, 0, 0, 59, 1, 0, 0, 0, 
    0, 61, 1, 0, 0, 0, 0, 63, 1, 0, 0, 0, 0, 65, 1, 0, 0, 0, 0, 67, 1, 0, 
    0, 0, 0, 69, 1, 0, 0, 0, 0, 71, 1, 0, 0, 0, 0, 73, 1, 0, 0, 0, 0, 75, 
    1, 0, 0, 0, 0, 77, 1, 0, 0, 0, 0, 79, 1, 0, 0, 0, 0, 81, 1, 0, 0, 0, 
    0, 83, 1, 0, 0, 0, 0, 85, 1, 0, 0, 0, 0, 87, 1, 0, 0, 0, 0, 89, 1, 0, 
    0, 0, 0, 91, 1, 0, 0, 0, 0, 93, 1, 0, 0, 0, 0, 95, 1, 0, 0, 0, 0, 97, 
    1, 0, 0, 0, 0, 99, 1, 0, 0, 0, 0, 101, 1, 0, 0, 0, 1, 103, 1, 0, 0, 
    0, 1, 105, 1, 0, 0, 0, 1, 107, 1, 0, 0, 0, 1, 109, 1, 0, 0, 0, 1, 111, 
    1, 0, 0, 0, 1, 113, 1, 0, 0, 0, 1, 115, 1, 0, 0, 0, 2, 117, 1, 0, 0, 
    0, 2, 119, 1, 0, 0, 0, 2, 121, 1, 0, 0, 0, 3, 139, 1, 0, 0, 0, 5, 156, 
    1, 0, 0, 0, 7, 172, 1, 0, 0, 0, 9, 191, 1, 0, 0, 0, 11, 193, 1, 0, 0, 
    0, 13, 203, 1, 0, 0, 0, 15, 211, 1, 0, 0, 0, 17, 214, 1, 0, 0, 0, 19, 
    216, 1, 0, 0, 0, 21, 253, 1, 0, 0, 0, 23, 269, 1, 0, 0, 0, 25, 284, 
    1, 0, 0, 0, 27, 301, 1, 0, 0, 0, 29, 308, 1, 0, 0, 0, 31, 317, 1, 0, 
    0, 0, 33, 323, 1, 0, 0, 0, 35, 330, 1, 0, 0, 0, 37, 338, 1, 0, 0, 0, 
    39, 348, 1, 0, 0, 0, 41, 355, 1, 0, 0, 0, 43, 363, 1, 0, 0, 0, 45, 371, 
    1, 0, 0, 0, 47, 378, 1, 0, 0, 0, 49, 385, 1, 0, 0, 0, 51, 391, 1, 0, 
    0, 0, 53, 399, 1, 0, 0, 0, 55, 404, 1, 0, 0, 0, 57, 406, 1, 0, 0, 0, 
    59, 409, 1, 0, 0, 0, 61, 411, 1, 0, 0, 0, 63, 413, 1, 0, 0, 0, 65, 415, 
    1, 0, 0, 0, 67, 417, 1, 0, 0, 0, 69, 419, 1, 0, 0, 0, 71, 422, 1, 0, 
    0, 0, 73, 424, 1, 0, 0, 0, 75, 426, 1, 0, 0, 0, 77, 428, 1, 0, 0, 0, 
    79, 430, 1, 0, 0, 0, 81, 432, 1, 0, 0, 0, 83, 435, 1, 0, 0, 0, 85, 437, 
    1, 0, 0, 0, 87, 439, 1, 0, 0, 0, 89, 441, 1, 0, 0, 0, 91, 444, 1, 0, 
    0, 0, 93, 446, 1, 0, 0, 0, 95, 448, 1, 0, 0, 0, 97, 450, 1, 0, 0, 0, 
    99, 452, 1, 0, 0, 0, 101, 460, 1, 0, 0, 0, 103, 466, 1, 0, 0, 0, 105, 
    471, 1, 0, 0, 0, 107, 476, 1, 0, 0, 0, 109, 480, 1, 0, 0, 0, 111, 484, 
    1, 0, 0, 0, 113, 487, 1, 0, 0, 0, 115, 491, 1, 0, 0, 0, 117, 496, 1, 
    0, 0, 0, 119, 502, 1, 0, 0, 0, 121, 506, 1, 0, 0, 0, 123, 510, 1, 0, 
    0, 0, 125, 517, 1, 0, 0, 0, 127, 519, 1, 0, 0, 0, 129, 532, 1, 0, 0, 
    0, 131, 542, 1, 0, 0, 0, 133, 557, 1, 0, 0, 0, 135, 569, 1, 0, 0, 0, 
    137, 571, 1, 0, 0, 0, 139, 140, 5, 47, 0, 0, 140, 141, 5, 42, 0, 0, 
    141, 142, 5, 42, 0, 0, 142, 146, 1, 0, 0, 0, 143, 145, 9, 0, 0, 0, 144, 
    143, 1, 0, 0, 0, 145, 148, 1, 0, 0, 0, 146, 147, 1, 0, 0, 0, 146, 144, 
    1, 0, 0, 0, 147, 152, 1, 0, 0, 0, 148, 146, 1, 0, 0, 0, 149, 150, 5, 
    42, 0, 0, 150, 153, 5, 47, 0, 0, 151, 153, 5, 0, 0, 1, 152, 149, 1, 
    0, 0, 0, 152, 151, 1, 0, 0, 0, 153, 154, 1, 0, 0, 0, 154, 155, 6, 0, 
    0, 0, 155, 4, 1, 0, 0, 0, 156, 157, 5, 47, 0, 0, 157, 158, 5, 42, 0, 
    0, 158, 162, 1, 0, 0, 0, 159, 161, 9, 0, 0, 0, 160, 159, 1, 0, 0, 0, 
    161, 164, 1, 0, 0, 0, 162, 163, 1, 0, 0, 0, 162, 160, 1, 0, 0, 0, 163, 
    168, 1, 0, 0, 0, 164, 162, 1, 0, 0, 0, 165, 166, 5, 42, 0, 0, 166, 169, 
    5, 47, 0, 0, 167, 169, 5, 0, 0, 1, 168, 165, 1, 0, 0, 0, 168, 167, 1, 
    0, 0, 0, 169, 170, 1, 0, 0, 0, 170, 171, 6, 1, 0, 0, 171, 6, 1, 0, 0, 
    0, 172, 173, 5, 47, 0, 0, 173, 174, 5, 47, 0, 0, 174, 178, 1, 0, 0, 
    0, 175, 177, 8, 0, 0, 0, 176, 175, 1, 0, 0, 0, 177, 180, 1, 0, 0, 0, 
    178, 176, 1, 0, 0, 0, 178, 179, 1, 0, 0, 0, 179, 181, 1, 0, 0, 0, 180, 
    178, 1, 0, 0, 0, 181, 182, 6, 2, 0, 0, 182, 8, 1, 0, 0, 0, 183, 192, 
    5, 48, 0, 0, 184, 188, 7, 1, 0, 0, 185, 187, 7, 2, 0, 0, 186, 185, 1, 
    0, 0, 0, 187, 190, 1, 0, 0, 0, 188, 186, 1, 0, 0, 0, 188, 189, 1, 0, 
    0, 0, 189, 192, 1, 0, 0, 0, 190, 188, 1, 0, 0, 0, 191, 183, 1, 0, 0, 
    0, 191, 184, 1, 0, 0, 0, 192, 10, 1, 0, 0, 0, 193, 198, 5, 39, 0, 0, 
    194, 197, 3, 123, 60, 0, 195, 197, 8, 3, 0, 0, 196, 194, 1, 0, 0, 0, 
    196, 195, 1, 0, 0, 0, 197, 200, 1, 0, 0, 0, 198, 196, 1, 0, 0, 0, 198, 
    199, 1, 0, 0, 0, 199, 201, 1, 0, 0, 0, 200, 198, 1, 0, 0, 0, 201, 202, 
    5, 39, 0, 0, 202, 12, 1, 0, 0, 0, 203, 208, 5, 39, 0, 0, 204, 207, 3, 
    123, 60, 0, 205, 207, 8, 3, 0, 0, 206, 204, 1, 0, 0, 0, 206, 205, 1, 
    0, 0, 0, 207, 210, 1, 0, 0, 0, 208, 206, 1, 0, 0, 0, 208, 209, 1, 0, 
    0, 0, 209, 14, 1, 0, 0, 0, 210, 208, 1, 0, 0, 0, 211, 212, 5, 91, 0, 
    0, 212, 213, 6, 6, 1, 0, 213, 16, 1, 0, 0, 0, 214, 215, 3, 19, 8, 0, 
    215, 18, 1, 0, 0, 0, 216, 248, 5, 123, 0, 0, 217, 247, 3, 19, 8, 0, 
    218, 247, 3, 11, 4, 0, 219, 247, 3, 129, 63, 0, 220, 247, 3, 131, 64, 
    0, 221, 247, 3, 133, 65, 0, 222, 223, 5, 47, 0, 0, 223, 224, 5, 42, 
    0, 0, 224, 228, 1, 0, 0, 0, 225, 227, 9, 0, 0, 0, 226, 225, 1, 0, 0, 
    0, 227, 230, 1, 0, 0, 0, 228, 229, 1, 0, 0, 0, 228, 226, 1, 0, 0, 0, 
    229, 231, 1, 0, 0, 0, 230, 228, 1, 0, 0, 0, 231, 232, 5, 42, 0, 0, 232, 
    247, 5, 47, 0, 0, 233, 234, 5, 47, 0, 0, 234, 235, 5, 47, 0, 0, 235, 
    239, 1, 0, 0, 0, 236, 238, 8, 0, 0, 0, 237, 236, 1, 0, 0, 0, 238, 241, 
    1, 0, 0, 0, 239, 237, 1, 0, 0, 0, 239, 240, 1, 0, 0, 0, 240, 247, 1, 
    0, 0, 0, 241, 239, 1, 0, 0, 0, 242, 243, 5, 92, 0, 0, 243, 247, 9, 0, 
    0, 0, 244, 247, 8, 4, 0, 0, 245, 247, 7, 5, 0, 0, 246, 217, 1, 0, 0, 
    0, 246, 218, 1, 0, 0, 0, 246, 219, 1, 0, 0, 0, 246, 220, 1, 0, 0, 0, 
    246, 221, 1, 0, 0, 0, 246, 222, 1, 0, 0, 0, 246, 233, 1, 0, 0, 0, 246, 
    242, 1, 0, 0, 0, 246, 244, 1, 0, 0, 0, 246, 245, 1, 0, 0, 0, 247, 250, 
    1, 0, 0, 0, 248, 249, 1, 0, 0, 0, 248, 246, 1, 0, 0, 0, 249, 251, 1, 
    0, 0, 0, 250, 248, 1, 0, 0, 0, 251, 252, 5, 125, 0, 0, 252, 20, 1, 0, 
    0, 0, 253, 254, 5, 111, 0, 0, 254, 255, 5, 112, 0, 0, 255, 256, 5, 116, 
    0, 0, 256, 257, 5, 105, 0, 0, 257, 258, 5, 111, 0, 0, 258, 259, 5, 110, 
    0, 0, 259, 260, 5, 115, 0, 0, 260, 264, 1, 0, 0, 0, 261, 263, 3, 101, 
    49, 0, 262, 261, 1, 0, 0, 0, 263, 266, 1, 0, 0, 0, 264, 262, 1, 0, 0, 
    0, 264, 265, 1, 0, 0, 0, 265, 267, 1, 0, 0, 0, 266, 264, 1, 0, 0, 0, 
    267, 268, 5, 123, 0, 0, 268, 22, 1, 0, 0, 0, 269, 270, 5, 116, 0, 0, 
    270, 271, 5, 111, 0, 0, 271, 272, 5, 107, 0, 0, 272, 273, 5, 101, 0, 
    0, 273, 274, 5, 110, 0, 0, 274, 275, 5, 115, 0, 0, 275, 279, 1, 0, 0, 
    0, 276, 278, 3, 101, 49, 0, 277, 276, 1, 0, 0, 0, 278, 281, 1, 0, 0, 
    0, 279, 277, 1, 0, 0, 0, 279, 280, 1, 0, 0, 0, 280, 282, 1, 0, 0, 0, 
    281, 279, 1, 0, 0, 0, 282, 283, 5, 123, 0, 0, 283, 24, 1, 0, 0, 0, 284, 
    285, 5, 99, 0, 0, 285, 286, 5, 104, 0, 0, 286, 287, 5, 97, 0, 0, 287, 
    288, 5, 110, 0, 0, 288, 289, 5, 110, 0, 0, 289, 290, 5, 101, 0, 0, 290, 
    291, 5, 108, 0, 0, 291, 292, 5, 115, 0, 0, 292, 296, 1, 0, 0, 0, 293, 
    295, 3, 101, 49, 0, 294, 293, 1, 0, 0, 0, 295, 298, 1, 0, 0, 0, 296, 
    294, 1, 0, 0, 0, 296, 297, 1, 0, 0, 0, 297, 299, 1, 0, 0, 0, 298, 296, 
    1, 0, 0, 0, 299, 300, 5, 123, 0, 0, 300, 26, 1, 0, 0, 0, 301, 302, 5, 
    105, 0, 0, 302, 303, 5, 109, 0, 0, 303, 304, 5, 112, 0, 0, 304, 305, 
    5, 111, 0, 0, 305, 306, 5, 114, 0, 0, 306, 307, 5, 116, 0, 0, 307, 28, 
    1, 0, 0, 0, 308, 309, 5, 102, 0, 0, 309, 310, 5, 114, 0, 0, 310, 311, 
    5, 97, 0, 0, 311, 312, 5, 103, 0, 0, 312, 313, 5, 109, 0, 0, 313, 314, 
    5, 101, 0, 0, 314, 315, 5, 110, 0, 0, 315, 316, 5, 116, 0, 0, 316, 30, 
    1, 0, 0, 0, 317, 318, 5, 108, 0, 0, 318, 319, 5, 101, 0, 0, 319, 320, 
    5, 120, 0, 0, 320, 321, 5, 101, 0, 0, 321, 322, 5, 114, 0, 0, 322, 32, 
    1, 0, 0, 0, 323, 324, 5, 112, 0, 0, 324, 325, 5, 97, 0, 0, 325, 326, 
    5, 114, 0, 0, 326, 327, 5, 115, 0, 0, 327, 328, 5, 101, 0, 0, 328, 329, 
    5, 114, 0, 0, 329, 34, 1, 0, 0, 0, 330, 331, 5, 103, 0, 0, 331, 332, 
    5, 114, 0, 0, 332, 333, 5, 97, 0, 0, 333, 334, 5, 109, 0, 0, 334, 335, 
    5, 109, 0, 0, 335, 336, 5, 97, 0, 0, 336, 337, 5, 114, 0, 0, 337, 36, 
    1, 0, 0, 0, 338, 339, 5, 112, 0, 0, 339, 340, 5, 114, 0, 0, 340, 341, 
    5, 111, 0, 0, 341, 342, 5, 116, 0, 0, 342, 343, 5, 101, 0, 0, 343, 344, 
    5, 99, 0, 0, 344, 345, 5, 116, 0, 0, 345, 346, 5, 101, 0, 0, 346, 347, 
    5, 100, 0, 0, 347, 38, 1, 0, 0, 0, 348, 349, 5, 112, 0, 0, 349, 350, 
    5, 117, 0, 0, 350, 351, 5, 98, 0, 0, 351, 352, 5, 108, 0, 0, 352, 353, 
    5, 105, 0, 0, 353, 354, 5, 99, 0, 0, 354, 40, 1, 0, 0, 0, 355, 356, 
    5, 112, 0, 0, 356, 357, 5, 114, 0, 0, 357, 358, 5, 105, 0, 0, 358, 359, 
    5, 118, 0, 0, 359, 360, 5, 97, 0, 0, 360, 361, 5, 116, 0, 0, 361, 362, 
    5, 101, 0, 0, 362, 42, 1, 0, 0, 0, 363, 364, 5, 114, 0, 0, 364, 365, 
    5, 101, 0, 0, 365, 366, 5, 116, 0, 0, 366, 367, 5, 117, 0, 0, 367, 368, 
    5, 114, 0, 0, 368, 369, 5, 110, 0, 0, 369, 370, 5, 115, 0, 0, 370, 44, 
    1, 0, 0, 0, 371, 372, 5, 108, 0, 0, 372, 373, 5, 111, 0, 0, 373, 374, 
    5, 99, 0, 0, 374, 375, 5, 97, 0, 0, 375, 376, 5, 108, 0, 0, 376, 377, 
    5, 115, 0, 0, 377, 46, 1, 0, 0, 0, 378, 379, 5, 116, 0, 0, 379, 380, 
    5, 104, 0, 0, 380, 381, 5, 114, 0, 0, 381, 382, 5, 111, 0, 0, 382, 383, 
    5, 119, 0, 0, 383, 384, 5, 115, 0, 0, 384, 48, 1, 0, 0, 0, 385, 386, 
    5, 99, 0, 0, 386, 387, 5, 97, 0, 0, 387, 388, 5, 116, 0, 0, 388, 389, 
    5, 99, 0, 0, 389, 390, 5, 104, 0, 0, 390, 50, 1, 0, 0, 0, 391, 392, 
    5, 102, 0, 0, 392, 393, 5, 105, 0, 0, 393, 394, 5, 110, 0, 0, 394, 395, 
    5, 97, 0, 0, 395, 396, 5, 108, 0, 0, 396, 397, 5, 108, 0, 0, 397, 398, 
    5, 121, 0, 0, 398, 52, 1, 0, 0, 0, 399, 400, 5, 109, 0, 0, 400, 401, 
    5, 111, 0, 0, 401, 402, 5, 100, 0, 0, 402, 403, 5, 101, 0, 0, 403, 54, 
    1, 0, 0, 0, 404, 405, 5, 58, 0, 0, 405, 56, 1, 0, 0, 0, 406, 407, 5, 
    58, 0, 0, 407, 408, 5, 58, 0, 0, 408, 58, 1, 0, 0, 0, 409, 410, 5, 44, 
    0, 0, 410, 60, 1, 0, 0, 0, 411, 412, 5, 59, 0, 0, 412, 62, 1, 0, 0, 
    0, 413, 414, 5, 40, 0, 0, 414, 64, 1, 0, 0, 0, 415, 416, 5, 41, 0, 0, 
    416, 66, 1, 0, 0, 0, 417, 418, 5, 125, 0, 0, 418, 68, 1, 0, 0, 0, 419, 
    420, 5, 45, 0, 0, 420, 421, 5, 62, 0, 0, 421, 70, 1, 0, 0, 0, 422, 423, 
    5, 60, 0, 0, 423, 72, 1, 0, 0, 0, 424, 425, 5, 62, 0, 0, 425, 74, 1, 
    0, 0, 0, 426, 427, 5, 61, 0, 0, 427, 76, 1, 0, 0, 0, 428, 429, 5, 63, 
    0, 0, 429, 78, 1, 0, 0, 0, 430, 431, 5, 42, 0, 0, 431, 80, 1, 0, 0, 
    0, 432, 433, 5, 43, 0, 0, 433, 434, 5, 61, 0, 0, 434, 82, 1, 0, 0, 0, 
    435, 436, 5, 43, 0, 0, 436, 84, 1, 0, 0, 0, 437, 438, 5, 124, 0, 0, 
    438, 86, 1, 0, 0, 0, 439, 440, 5, 36, 0, 0, 440, 88, 1, 0, 0, 0, 441, 
    442, 5, 46, 0, 0, 442, 443, 5, 46, 0, 0, 443, 90, 1, 0, 0, 0, 444, 445, 
    5, 46, 0, 0, 445, 92, 1, 0, 0, 0, 446, 447, 5, 64, 0, 0, 447, 94, 1, 
    0, 0, 0, 448, 449, 5, 35, 0, 0, 449, 96, 1, 0, 0, 0, 450, 451, 5, 126, 
    0, 0, 451, 98, 1, 0, 0, 0, 452, 456, 3, 137, 67, 0, 453, 455, 3, 135, 
    66, 0, 454, 453, 1, 0, 0, 0, 455, 458, 1, 0, 0, 0, 456, 454, 1, 0, 0, 
    0, 456, 457, 1, 0, 0, 0, 457, 100, 1, 0, 0, 0, 458, 456, 1, 0, 0, 0, 
    459, 461, 7, 6, 0, 0, 460, 459, 1, 0, 0, 0, 461, 462, 1, 0, 0, 0, 462, 
    460, 1, 0, 0, 0, 462, 463, 1, 0, 0, 0, 463, 464, 1, 0, 0, 0, 464, 465, 
    6, 49, 2, 0, 465, 102, 1, 0, 0, 0, 466, 467, 5, 91, 0, 0, 467, 468, 
    1, 0, 0, 0, 468, 469, 6, 50, 3, 0, 469, 470, 6, 50, 4, 0, 470, 104, 
    1, 0, 0, 0, 471, 472, 5, 92, 0, 0, 472, 473, 9, 0, 0, 0, 473, 474, 1, 
    0, 0, 0, 474, 475, 6, 51, 3, 0, 475, 106, 1, 0, 0, 0, 476, 477, 3, 129, 
    63, 0, 477, 478, 1, 0, 0, 0, 478, 479, 6, 52, 3, 0, 479, 108, 1, 0, 
    0, 0, 480, 481, 3, 11, 4, 0, 481, 482, 1, 0, 0, 0, 482, 483, 6, 53, 
    3, 0, 483, 110, 1, 0, 0, 0, 484, 485, 5, 93, 0, 0, 485, 486, 6, 54, 
    5, 0, 486, 112, 1, 0, 0, 0, 487, 488, 5, 0, 0, 1, 488, 489, 1, 0, 0, 
    0, 489, 490, 6, 55, 6, 0, 490, 114, 1, 0, 0, 0, 491, 492, 9, 0, 0, 0, 
    492, 116, 1, 0, 0, 0, 493, 497, 8, 7, 0, 0, 494, 495, 5, 92, 0, 0, 495, 
    497, 9, 0, 0, 0, 496, 493, 1, 0, 0, 0, 496, 494, 1, 0, 0, 0, 497, 498, 
    1, 0, 0, 0, 498, 496, 1, 0, 0, 0, 498, 499, 1, 0, 0, 0, 499, 500, 1, 
    0, 0, 0, 500, 501, 6, 57, 7, 0, 501, 118, 1, 0, 0, 0, 502, 503, 5, 93, 
    0, 0, 503, 504, 1, 0, 0, 0, 504, 505, 6, 58, 6, 0, 505, 120, 1, 0, 0, 
    0, 506, 507, 5, 0, 0, 1, 507, 508, 1, 0, 0, 0, 508, 509, 6, 59, 6, 0, 
    509, 122, 1, 0, 0, 0, 510, 515, 5, 92, 0, 0, 511, 516, 7, 8, 0, 0, 512, 
    516, 3, 127, 62, 0, 513, 516, 9, 0, 0, 0, 514, 516, 5, 0, 0, 1, 515, 
    511, 1, 0, 0, 0, 515, 512, 1, 0, 0, 0, 515, 513, 1, 0, 0, 0, 515, 514, 
    1, 0, 0, 0, 516, 124, 1, 0, 0, 0, 517, 518, 7, 9, 0, 0, 518, 126, 1, 
    0, 0, 0, 519, 530, 5, 117, 0, 0, 520, 528, 3, 125, 61, 0, 521, 526, 
    3, 125, 61, 0, 522, 524, 3, 125, 61, 0, 523, 525, 3, 125, 61, 0, 524, 
    523, 1, 0, 0, 0, 524, 525, 1, 0, 0, 0, 525, 527, 1, 0, 0, 0, 526, 522, 
    1, 0, 0, 0, 526, 527, 1, 0, 0, 0, 527, 529, 1, 0, 0, 0, 528, 521, 1, 
    0, 0, 0, 528, 529, 1, 0, 0, 0, 529, 531, 1, 0, 0, 0, 530, 520, 1, 0, 
    0, 0, 530, 531, 1, 0, 0, 0, 531, 128, 1, 0, 0, 0, 532, 537, 5, 34, 0, 
    0, 533, 536, 3, 123, 60, 0, 534, 536, 8, 10, 0, 0, 535, 533, 1, 0, 0, 
    0, 535, 534, 1, 0, 0, 0, 536, 539, 1, 0, 0, 0, 537, 538, 1, 0, 0, 0, 
    537, 535, 1, 0, 0, 0, 538, 540, 1, 0, 0, 0, 539, 537, 1, 0, 0, 0, 540, 
    541, 5, 34, 0, 0, 541, 130, 1, 0, 0, 0, 542, 543, 5, 34, 0, 0, 543, 
    544, 5, 34, 0, 0, 544, 545, 5, 34, 0, 0, 545, 550, 1, 0, 0, 0, 546, 
    549, 3, 123, 60, 0, 547, 549, 9, 0, 0, 0, 548, 546, 1, 0, 0, 0, 548, 
    547, 1, 0, 0, 0, 549, 552, 1, 0, 0, 0, 550, 551, 1, 0, 0, 0, 550, 548, 
    1, 0, 0, 0, 551, 553, 1, 0, 0, 0, 552, 550, 1, 0, 0, 0, 553, 554, 5, 
    34, 0, 0, 554, 555, 5, 34, 0, 0, 555, 556, 5, 34, 0, 0, 556, 132, 1, 
    0, 0, 0, 557, 562, 5, 96, 0, 0, 558, 561, 3, 123, 60, 0, 559, 561, 8, 
    10, 0, 0, 560, 558, 1, 0, 0, 0, 560, 559, 1, 0, 0, 0, 561, 564, 1, 0, 
    0, 0, 562, 563, 1, 0, 0, 0, 562, 560, 1, 0, 0, 0, 563, 565, 1, 0, 0, 
    0, 564, 562, 1, 0, 0, 0, 565, 566, 5, 96, 0, 0, 566, 134, 1, 0, 0, 0, 
    567, 570, 3, 137, 67, 0, 568, 570, 7, 11, 0, 0, 569, 567, 1, 0, 0, 0, 
    569, 568, 1, 0, 0, 0, 570, 136, 1, 0, 0, 0, 571, 572, 7, 12, 0, 0, 572, 
    138, 1, 0, 0, 0, 37, 0, 1, 2, 146, 152, 162, 168, 178, 188, 191, 196, 
    198, 206, 208, 228, 239, 246, 248, 264, 279, 296, 456, 462, 496, 498, 
    515, 524, 526, 528, 530, 535, 537, 548, 550, 560, 562, 569, 8, 0, 3, 
    0, 1, 6, 0, 0, 2, 0, 7, 55, 0, 5, 1, 0, 1, 54, 1, 4, 0, 0, 3, 0, 0
]);