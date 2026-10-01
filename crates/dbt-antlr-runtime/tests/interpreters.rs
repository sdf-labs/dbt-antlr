//! Integration tests for `ParserInterpreter` and `LexerInterpreter`:
//! driving bare deserialized ATNs through the standard parser/lexer
//! frameworks without generated code.
//!
//! Test fixtures are copied from `tests/gen/simplelrlexer.rs`,
//! `tests/gen/simplelrparser.rs` (grammar: `s: a; a: ID | a ID;` with
//! `ID: [a-z]+; WS: [ \t\r\n]+ -> skip;`) and from a generated `Ambig`
//! grammar (`s: e '!'? EOF; e: ID '!' | ID;`).

use std::cell::RefCell;
use std::rc::Rc;

use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::error_listener::{DiagnosticErrorListener, ErrorListener};
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::lexer_interpreter::LexerInterpreter;
use dbt_antlr_runtime::parser_interpreter::ParserInterpreter;
use dbt_antlr_runtime::parser_rule_context::ParserRuleContext;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::rule_context::RuleContext;
use dbt_antlr_runtime::token::{Token, TOKEN_EOF};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::tree::Tree;
use dbt_antlr_runtime::vocabulary::{Vocabulary, VocabularyImpl};
use dbt_antlr_runtime::{Arena, InputStream, Parser, TokenSource};

// ======= SimpleLR fixtures (copied from tests/gen) =======

static SIMPLELR_LEXER_RULE_NAMES: &[&str] = &["ID", "WS"];
static SIMPLELR_PARSER_RULE_NAMES: &[&str] = &["s", "a"];
static SIMPLELR_CHANNEL_NAMES: &[&str] = &["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];
static SIMPLELR_MODE_NAMES: &[&str] = &["DEFAULT_MODE"];

const SIMPLELR_ID: i32 = 1;

fn simplelr_vocabulary() -> Box<dyn Vocabulary> {
    Box::new(VocabularyImpl::new(
        [].iter(),
        [None, Some("ID"), Some("WS")].iter(),
        None,
    ))
}

fn simplelr_lexer_atn() -> dbt_antlr_runtime::atn::ATN {
    let words: Vec<i32> = vec![
        4, 0, 2, 14, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 1, 0, 4, 0, 7, 8, 0, 11, 0, 12, 0, 8, 1, 1, 1,
        1, 1, 1, 1, 1, 0, 0, 2, 1, 1, 3, 2, 1, 0, 1, 2, 0, 10, 10, 32, 32, 14, 0, 1, 1, 0, 0, 0, 0,
        3, 1, 0, 0, 0, 1, 6, 1, 0, 0, 0, 3, 10, 1, 0, 0, 0, 5, 7, 2, 97, 122, 0, 6, 5, 1, 0, 0, 0,
        7, 8, 1, 0, 0, 0, 8, 6, 1, 0, 0, 0, 8, 9, 1, 0, 0, 0, 9, 2, 1, 0, 0, 0, 10, 11, 7, 0, 0, 0,
        11, 12, 1, 0, 0, 0, 12, 13, 6, 1, 0, 0, 13, 4, 1, 0, 0, 0, 2, 0, 8, 1, 6, 0, 0,
    ];
    ATNDeserializer::new(None).deserialize(&mut words.iter())
}

fn simplelr_parser_atn() -> dbt_antlr_runtime::atn::ATN {
    let words: Vec<u32> = vec![
        1346458702, 3, 16909060, 29, 2, 17, 16, 0, 0, 1, 2, 29, 119, 148, 80, 228, 0, 228, 0, 228,
        1, 229, 2, 231, 2, 233, 0, 228, 0, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0,
        4294967295, 4294967295, 2, 1, 12, 1, 1, 4294967295, 4294967295, 7, 1, 24, 2, 1, 4294967295,
        4294967295, 1, 0, 8, 3, 1, 4294967295, 4294967295, 1, 0, 8, 4, 1, 4294967295, 4294967295,
        1, 1, 72, 5, 1, 4294967295, 4294967295, 1, 1, 32, 6, 1, 4294967295, 4294967295, 1, 1, 8, 7,
        1, 4294967295, 4294967295, 1, 1, 72, 8, 1, 4294967295, 4294967295, 1, 1, 32, 9, 1,
        4294967295, 4294967295, 5, 1, 8, 10, 1, 12, 4294967295, 8, 1, 8, 11, 1, 4294967295,
        4294967295, 10, 1, 10, 12, 2, 4294967295, 4294967295, 12, 1, 8, 14, 1, 4294967295, 15, 9,
        1, 8, 15, 1, 4294967295, 4294967295, 1, 1, 0, 16, 0, 4294967295, 4294967295, 1, 4, 0, 0, 0,
        1, 6, 0, 0, 0, 1, 5, 0, 0, 0, 259, 2, 1, 5, 0, 1, 1, 0, 0, 0, 6, 7, 1, 4294967295, 0, 5, 8,
        1, 0, 0, 1, 13, 0, 0, 0, 10, 10, 2, 0, 0, 5, 12, 1, 0, 0, 1, 9, 0, 0, 0, 1, 15, 0, 0, 0, 1,
        11, 0, 0, 0, 1, 14, 0, 0, 0, 1, 3, 0, 0, 0, 1, 13, 0, 0, 0, 13, 0, 2, 1, 3,
    ];
    PackedATNDeserializer::new()
        .deserialize(&words)
        .expect("packed SimpleLR parser ATN deserializes")
}

// ======= Ambig fixtures (from generated AmbigLexer/AmbigParser) =======

static AMBIG_LEXER_RULE_NAMES: &[&str] = &["T__0", "ID", "WS"];
static AMBIG_PARSER_RULE_NAMES: &[&str] = &["s", "e"];
static AMBIG_CHANNEL_NAMES: &[&str] = &["DEFAULT_TOKEN_CHANNEL", "HIDDEN"];
static AMBIG_MODE_NAMES: &[&str] = &["DEFAULT_MODE"];

fn ambig_vocabulary() -> Box<dyn Vocabulary> {
    Box::new(VocabularyImpl::new(
        [None, Some("'!'")].iter(),
        [None, None, Some("ID"), Some("WS")].iter(),
        None,
    ))
}

fn ambig_lexer_atn() -> dbt_antlr_runtime::atn::ATN {
    let words: Vec<i32> = vec![
        4, 0, 3, 21, 6, -1, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 1, 0, 1, 0, 1, 1, 4, 1, 11, 8, 1,
        11, 1, 12, 1, 12, 1, 2, 4, 2, 16, 8, 2, 11, 2, 12, 2, 17, 1, 2, 1, 2, 0, 0, 3, 1, 1, 3, 2,
        5, 3, 1, 0, 2, 1, 0, 97, 122, 3, 0, 9, 10, 13, 13, 32, 32, 22, 0, 1, 1, 0, 0, 0, 0, 3, 1,
        0, 0, 0, 0, 5, 1, 0, 0, 0, 1, 7, 1, 0, 0, 0, 3, 10, 1, 0, 0, 0, 5, 15, 1, 0, 0, 0, 7, 8, 5,
        33, 0, 0, 8, 2, 1, 0, 0, 0, 9, 11, 7, 0, 0, 0, 10, 9, 1, 0, 0, 0, 11, 12, 1, 0, 0, 0, 12,
        10, 1, 0, 0, 0, 12, 13, 1, 0, 0, 0, 13, 4, 1, 0, 0, 0, 14, 16, 7, 1, 0, 0, 15, 14, 1, 0, 0,
        0, 16, 17, 1, 0, 0, 0, 17, 15, 1, 0, 0, 0, 17, 18, 1, 0, 0, 0, 18, 19, 1, 0, 0, 0, 19, 20,
        6, 2, 0, 0, 20, 6, 1, 0, 0, 0, 3, 0, 12, 17, 1, 6, 0, 0,
    ];
    ATNDeserializer::new(None).deserialize(&mut words.iter())
}

fn ambig_parser_atn() -> dbt_antlr_runtime::atn::ATN {
    let words: Vec<u32> = vec![
        1346458702, 3, 16909060, 29, 3, 16, 16, 0, 0, 2, 2, 29, 112, 141, 80, 221, 0, 221, 0, 221,
        2, 223, 2, 225, 2, 227, 0, 221, 0, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0,
        4294967295, 4294967295, 2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 24, 2, 1, 4294967295,
        4294967295, 1, 0, 8, 3, 1, 4294967295, 4294967295, 1, 0, 32, 4, 1, 4294967295, 4294967295,
        3, 0, 8, 5, 2, 7, 4294967295, 8, 0, 8, 7, 1, 4294967295, 4294967295, 1, 0, 32, 8, 1,
        4294967295, 4294967295, 1, 0, 8, 9, 1, 4294967295, 4294967295, 1, 1, 32, 10, 1, 4294967295,
        4294967295, 1, 1, 32, 11, 1, 4294967295, 4294967295, 1, 1, 32, 12, 1, 4294967295,
        4294967295, 3, 1, 8, 13, 2, 14, 4294967295, 8, 1, 8, 15, 1, 4294967295, 4294967295, 1, 1,
        0, 16, 0, 4294967295, 4294967295, 1, 4, 0, 0, 0, 1, 13, 0, 0, 0, 1, 6, 0, 0, 0, 3, 2, 1, 6,
        0, 5, 7, 1, 0, 0, 1, 5, 0, 0, 0, 1, 7, 0, 0, 0, 1, 8, 0, 0, 0, 5, 9, 4294967295, 0, 0, 1,
        1, 0, 0, 0, 5, 11, 2, 0, 0, 5, 14, 1, 0, 0, 5, 14, 2, 0, 0, 1, 10, 0, 0, 0, 1, 12, 0, 0, 0,
        1, 3, 0, 0, 0, 6, 13, 0, 2, 1, 3,
    ];
    PackedATNDeserializer::new()
        .deserialize(&words)
        .expect("packed Ambig parser ATN deserializes")
}

// ======= Error collection =======

/// Collects `syntax_error` notifications in Java's `ConsoleErrorListener`
/// format: `line <line>:<column> <msg>`.
#[derive(Clone, Default)]
struct CollectingErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'input, 'arena, R, Tok> ErrorListener<'input, 'arena, R, Tok> for CollectingErrorListener
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: Token + 'input,
{
    fn syntax_error(
        &self,
        _recognizer: &R,
        _offending_symbol: Option<&dyn Token>,
        line: u32,
        column: i32,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.messages
            .borrow_mut()
            .push(format!("line {}:{} {}", line, column, msg));
    }
}

// ======= LexerInterpreter tests =======

#[test]
fn lexer_interpreter_tokenizes() {
    Arena::with(|arena| {
        let mut lexer = LexerInterpreter::<_, CommonTokenFactory>::new(
            arena,
            "SimpleLRLexer.g4",
            simplelr_vocabulary(),
            SIMPLELR_LEXER_RULE_NAMES,
            SIMPLELR_CHANNEL_NAMES,
            SIMPLELR_MODE_NAMES,
            simplelr_lexer_atn(),
            InputStream::new("abc def"),
        )
        .expect("lexer ATN is accepted");

        let mut types = Vec::new();
        let mut texts = Vec::new();
        loop {
            let token = lexer.next_token();
            types.push(token.get_token_type());
            texts.push(token.get_text().to_owned());
            if *types.last().unwrap() == TOKEN_EOF {
                break;
            }
        }
        // WS is skipped by a lexer action stored in the ATN
        assert_eq!(types, vec![SIMPLELR_ID, SIMPLELR_ID, TOKEN_EOF]);
        assert_eq!(texts, vec!["abc", "def", "<EOF>"]);
    });
}

#[test]
fn lexer_interpreter_rejects_parser_atn() {
    Arena::with(|arena| {
        let result = LexerInterpreter::<_, CommonTokenFactory>::new(
            arena,
            "SimpleLR.g4",
            simplelr_vocabulary(),
            SIMPLELR_PARSER_RULE_NAMES,
            SIMPLELR_CHANNEL_NAMES,
            SIMPLELR_MODE_NAMES,
            simplelr_parser_atn(),
            InputStream::new("abc"),
        );
        assert!(result.is_err());
    });
}

// ======= ParserInterpreter tests =======

type SimpleLRLexerInterp<'input, 'arena> =
    LexerInterpreter<'input, 'arena, InputStream<'input>, CommonTokenFactory<'input, 'arena>>;

fn simplelr_parser<'input, 'arena>(
    arena: &'arena Arena,
    input: &'input str,
) -> ParserInterpreter<
    'input,
    'arena,
    CommonTokenStream<
        'input,
        'arena,
        SimpleLRLexerInterp<'input, 'arena>,
        CommonTokenFactory<'input, 'arena>,
    >,
    CommonTokenFactory<'input, 'arena>,
> {
    let lexer = LexerInterpreter::new(
        arena,
        "SimpleLRLexer.g4",
        simplelr_vocabulary(),
        SIMPLELR_LEXER_RULE_NAMES,
        SIMPLELR_CHANNEL_NAMES,
        SIMPLELR_MODE_NAMES,
        simplelr_lexer_atn(),
        InputStream::new(input),
    )
    .expect("lexer ATN is accepted");
    let stream = CommonTokenStream::new(lexer);
    ParserInterpreter::new(
        arena,
        "SimpleLR.g4",
        simplelr_vocabulary(),
        SIMPLELR_PARSER_RULE_NAMES,
        simplelr_parser_atn(),
        stream,
    )
}

#[test]
fn parser_interpreter_builds_tree_for_left_recursive_rule() {
    Arena::with(|arena| {
        let mut parser = simplelr_parser(arena, "abc def");
        let tree = parser.parse(0).expect("input parses");

        assert_eq!(tree.get_rule_index(), 0, "root is rule s");
        let a = tree.get_child(0).expect("s has one child");
        assert_eq!(a.get_rule_index(), 1, "child is rule a");
        assert_eq!(
            tree.to_string_tree(SIMPLELR_PARSER_RULE_NAMES),
            "(s (a (a abc) def))"
        );
    });
}

#[test]
fn parser_interpreter_reports_java_format_syntax_error() {
    Arena::with(|arena| {
        let mut parser = simplelr_parser(arena, "");
        let listener = CollectingErrorListener::default();
        let messages = listener.messages.clone();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener));

        let tree = parser
            .parse(0)
            .expect("error recovery still produces a tree");

        assert_eq!(
            tree.to_string_tree(SIMPLELR_PARSER_RULE_NAMES),
            "(s (a <missing ID>))"
        );
        assert_eq!(
            messages.borrow().as_slice(),
            ["line 1:0 missing ID at '<EOF>'"]
        );
    });
}

// ======= Ambiguity diagnostics and decision override =======

type AmbigLexerInterp<'input, 'arena> =
    LexerInterpreter<'input, 'arena, InputStream<'input>, CommonTokenFactory<'input, 'arena>>;

type AmbigParserInterp<'input, 'arena> = ParserInterpreter<
    'input,
    'arena,
    CommonTokenStream<
        'input,
        'arena,
        AmbigLexerInterp<'input, 'arena>,
        CommonTokenFactory<'input, 'arena>,
    >,
    CommonTokenFactory<'input, 'arena>,
>;

fn ambig_parser<'input, 'arena>(
    arena: &'arena Arena,
    input: &'input str,
) -> AmbigParserInterp<'input, 'arena> {
    let lexer = LexerInterpreter::new(
        arena,
        "AmbigLexer.g4",
        ambig_vocabulary(),
        AMBIG_LEXER_RULE_NAMES,
        AMBIG_CHANNEL_NAMES,
        AMBIG_MODE_NAMES,
        ambig_lexer_atn(),
        InputStream::new(input),
    )
    .expect("lexer ATN is accepted");
    let stream = CommonTokenStream::new(lexer);
    ParserInterpreter::new(
        arena,
        "Ambig.g4",
        ambig_vocabulary(),
        AMBIG_PARSER_RULE_NAMES,
        ambig_parser_atn(),
        stream,
    )
}

#[test]
fn parser_interpreter_ambiguity_diagnostic() {
    Arena::with(|arena| {
        let mut parser = ambig_parser(arena, "x!");
        let listener = CollectingErrorListener::default();
        let messages = listener.messages.clone();
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(
            DiagnosticErrorListener::<CommonTokenFactory>::new(false),
        ));
        parser.add_error_listener(Box::new(listener));

        let tree = parser.parse(0).expect("input parses");

        // default prediction resolves the ambiguity to the first alternative
        assert_eq!(
            tree.to_string_tree(AMBIG_PARSER_RULE_NAMES),
            "(s (e x !) <EOF>)"
        );
        let messages = messages.borrow();
        assert!(
            messages.iter().any(|msg| msg.contains("reportAmbiguity")),
            "expected a reportAmbiguity diagnostic, got: {:?}",
            messages
        );
    });
}

#[test]
fn parser_interpreter_decision_override() {
    Arena::with(|arena| {
        // decision 1 (in rule e, at token index 0) is forced to the second
        // alternative, producing the other ambiguous parse tree
        let mut parser = ambig_parser(arena, "x!");
        parser.remove_error_listeners();
        parser.add_decision_override(1, 0, 2);

        let tree = parser.parse(0).expect("input parses");

        assert_eq!(
            tree.to_string_tree(AMBIG_PARSER_RULE_NAMES),
            "(s (e x) ! <EOF>)"
        );
    });
}
