//! Parse sessions: the owner of a parse tree, its arena, and its diagnostics.

use std::any::Any;
use std::cell::RefCell;
use std::ffi::c_void;
use std::panic::{self, AssertUnwindSafe};
use std::ptr;
use std::rc::Rc;

use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::{ANTLRError, ANTLRErrorKind};
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token::Token;

use crate::grammar::{AntlrStr, Grammar, ParseOutput};
use crate::overlay::AntlrTreeNode;

/// `AntlrSessionInfo::status`: the parse produced a tree. The tree can
/// contain error nodes: examine `AntlrSessionInfo::syntax_error_count`.
pub const ANTLR_PARSE_STATUS_OK: i32 = 0;
/// `AntlrSessionInfo::status`: the parse stopped without a tree, for example
/// because it reached an arena allocation limit or because an action
/// panicked. `AntlrSessionInfo::message` tells the cause.
pub const ANTLR_PARSE_STATUS_FATAL: i32 = 1;
/// `AntlrSessionInfo::status`: the grammar ID is not valid.
pub const ANTLR_PARSE_STATUS_INVALID_GRAMMAR: i32 = 2;
/// `AntlrSessionInfo::status`: the rule index is not valid, or the rule
/// cannot start a parse.
pub const ANTLR_PARSE_STATUS_INVALID_RULE: i32 = 3;
/// `AntlrSessionInfo::status`: the input is not valid UTF-8.
pub const ANTLR_PARSE_STATUS_INVALID_UTF8: i32 = 4;
/// `AntlrSessionInfo::status`: the input pointer is NULL, or the input is
/// too large for the 32-bit offsets of tokens.
pub const ANTLR_PARSE_STATUS_INVALID_INPUT: i32 = 5;

/// `AntlrSyntaxError::origin`: the lexer found no token at the input
/// position.
pub const ANTLR_ERROR_ORIGIN_LEXER: u8 = 0;
/// `AntlrSyntaxError::origin`: the parser found an unexpected token.
pub const ANTLR_ERROR_ORIGIN_PARSER: u8 = 1;

/// A syntax error that a parse reported.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrSyntaxError {
    /// The error message, for example `extraneous input 'x' expecting ID`.
    pub message: AntlrStr,
    /// The line of the error (1-based).
    pub line: u32,
    /// The column of the error in Unicode code points (0-based), or -1.
    pub column: i32,
    /// The type of the offending token, or 0 when there is no offending
    /// token (lexer errors).
    pub token_type: i32,
    /// The token stream index of the offending token, or -1.
    pub token_index: i32,
    /// The UTF-8 byte offset where the offending input starts, or -1.
    pub start: i32,
    /// The UTF-8 byte offset of the last byte of the offending token
    /// (inclusive), or -1.
    pub stop: i32,
    /// `ANTLR_ERROR_ORIGIN_LEXER` or `ANTLR_ERROR_ORIGIN_PARSER`.
    pub origin: u8,
}

/// The result of a parse, which `antlr_session_info` returns.
///
/// All pointers stay valid until `antlr_session_free`.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AntlrSessionInfo {
    /// The parse result: one of the `ANTLR_PARSE_STATUS_*` values.
    pub status: i32,
    /// The root node when `status` is `ANTLR_PARSE_STATUS_OK`, else NULL.
    pub root: *const AntlrTreeNode,
    /// The source of the tokens that slice the input (see
    /// `AntlrToken::source`), or NULL.
    pub primary_source: *const c_void,
    /// The syntax errors, in report order.
    pub syntax_errors: *const AntlrSyntaxError,
    /// The number of entries in `syntax_errors`.
    pub syntax_error_count: usize,
    /// A description of the problem when `status` is not
    /// `ANTLR_PARSE_STATUS_OK`. Absent otherwise.
    pub message: AntlrStr,
}

/// A parse tree with its arena and diagnostics.
///
/// `antlr_parse` creates a session and `antlr_session_free` destroys it. The
/// session borrows the input: the input must stay valid and unchanged until
/// `antlr_session_free`.
#[derive(Debug)]
pub struct AntlrParseSession {
    info: AntlrSessionInfo,
    _syntax_errors: Box<[AntlrSyntaxError]>,
    _strings: Box<[Box<str>]>,
    _arena: Option<Box<Arena>>,
}

impl AntlrParseSession {
    /// The C view of the session.
    pub fn info(&self) -> &AntlrSessionInfo {
        &self.info
    }

    pub(crate) fn failure(status: i32, message: impl Into<Box<str>>) -> Self {
        Self::new(status, None, None, Vec::new(), Some(message.into()))
    }

    fn new(
        status: i32,
        arena: Option<Box<Arena>>,
        output: Option<ParseOutput>,
        errors: Vec<CollectedError>,
        message: Option<Box<str>>,
    ) -> Self {
        let mut strings = Vec::with_capacity(errors.len() + 1);
        let syntax_errors: Box<[AntlrSyntaxError]> = errors
            .into_iter()
            .map(|error| {
                let message = error.message.into_boxed_str();
                let view = AntlrStr::borrowed(&message);
                strings.push(message);
                AntlrSyntaxError {
                    message: view,
                    ..error.fields
                }
            })
            .collect();
        let message = message.map_or(AntlrStr::NONE, |message| {
            let view = AntlrStr::borrowed(&message);
            strings.push(message);
            view
        });
        let (root, primary_source) = output.map_or((ptr::null(), ptr::null()), |output| {
            (output.root, output.primary_source)
        });
        Self {
            info: AntlrSessionInfo {
                status,
                root,
                primary_source,
                syntax_errors: syntax_errors.as_ptr(),
                syntax_error_count: syntax_errors.len(),
                message,
            },
            _syntax_errors: syntax_errors,
            _strings: strings.into_boxed_slice(),
            _arena: arena,
        }
    }
}

/// Parses `input` with `grammar` from the rule with index `rule_index`.
pub(crate) fn parse(grammar: &Grammar, rule_index: u32, input: &[u8]) -> AntlrParseSession {
    let Ok(input) = std::str::from_utf8(input) else {
        return AntlrParseSession::failure(
            ANTLR_PARSE_STATUS_INVALID_UTF8,
            "the input is not valid UTF-8",
        );
    };
    if i32::try_from(input.len()).is_err() {
        return AntlrParseSession::failure(
            ANTLR_PARSE_STATUS_INVALID_INPUT,
            "the input is too large: token offsets are 32-bit",
        );
    }
    let arena = Box::new(Arena::new());
    let errors = ErrorCollector::default();
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        (grammar.parse)(&arena, input, rule_index, &errors)
    }));
    let collected = errors.take();
    match outcome {
        Ok(Ok(Some(output))) => AntlrParseSession::new(
            ANTLR_PARSE_STATUS_OK,
            Some(arena),
            Some(output),
            collected,
            None,
        ),
        Ok(Ok(None)) => {
            // SAFETY: `Grammar::new` stores a view of a `&'static str`.
            let name = unsafe { grammar.info.name.as_str() }.unwrap_or_default();
            let message = format!("rule {rule_index} of grammar {name} cannot start a parse");
            AntlrParseSession::new(
                ANTLR_PARSE_STATUS_INVALID_RULE,
                None,
                None,
                collected,
                Some(message.into()),
            )
        }
        Ok(Err(error)) => AntlrParseSession::new(
            ANTLR_PARSE_STATUS_FATAL,
            None,
            None,
            collected,
            Some(error.to_string().into()),
        ),
        Err(payload) => AntlrParseSession::new(
            ANTLR_PARSE_STATUS_FATAL,
            None,
            None,
            collected,
            Some(panic_message(payload.as_ref()).into()),
        ),
    }
}

/// The text of a panic payload.
pub(crate) fn panic_message(payload: &(dyn Any + Send)) -> String {
    let detail = payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
        .unwrap_or("non-string panic payload");
    format!("the parser panicked: {detail}")
}

#[derive(Debug)]
struct CollectedError {
    message: String,
    fields: AntlrSyntaxError,
}

/// Collects the syntax errors of one parse.
///
/// Generated parse functions attach the listeners of a collector to the lexer
/// and to the parser, in place of the default console listeners.
#[derive(Debug, Default)]
pub struct ErrorCollector {
    errors: Rc<RefCell<Vec<CollectedError>>>,
}

impl ErrorCollector {
    /// A listener for the lexer.
    pub fn lexer_listener(&self) -> Box<CollectingErrorListener> {
        self.listener(ANTLR_ERROR_ORIGIN_LEXER)
    }

    /// A listener for the parser.
    pub fn parser_listener(&self) -> Box<CollectingErrorListener> {
        self.listener(ANTLR_ERROR_ORIGIN_PARSER)
    }

    fn listener(&self, origin: u8) -> Box<CollectingErrorListener> {
        Box::new(CollectingErrorListener {
            errors: Rc::clone(&self.errors),
            origin,
        })
    }

    fn take(&self) -> Vec<CollectedError> {
        self.errors.take()
    }
}

/// An error listener that records syntax errors into an [`ErrorCollector`].
#[derive(Debug)]
pub struct CollectingErrorListener {
    errors: Rc<RefCell<Vec<CollectedError>>>,
    origin: u8,
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
        offending_symbol: Option<&dyn Token>,
        line: u32,
        column: i32,
        msg: &str,
        error: Option<&ANTLRError>,
    ) {
        let (token_type, token_index, start, stop) = match offending_symbol {
            Some(token) => (
                token.get_token_type(),
                token.get_token_index() as i32,
                token.get_start_index() as i32,
                token.get_stop_index() as i32,
            ),
            None => match error.map(AsRef::as_ref) {
                Some(ANTLRErrorKind::LexerNoAltError { start_index }) => {
                    (0, -1, *start_index as i32, -1)
                }
                _ => (0, -1, -1, -1),
            },
        };
        self.errors.borrow_mut().push(CollectedError {
            message: msg.to_owned(),
            fields: AntlrSyntaxError {
                message: AntlrStr::NONE,
                line,
                column,
                token_type,
                token_index,
                start,
                stop,
                origin: self.origin,
            },
        });
    }
}
