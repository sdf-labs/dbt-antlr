// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Shared-descent groups: the runtime protocol (the neutral parse, the resume
//! of its subtree, and the listener muting and replay around them) and the
//! generated `DescentAltBlock`s that drive it.
//!
//! The protocol tests drive the calls directly on the adaptive `GuardExpr.g4`
//! parser, outside of any rule: `value` plays the shared rule. The
//! differential tests parse `ParenExpr.g4` with its static and
//! `--static-dfa-only` twins against the adaptive parser.

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod guardexprlexer;
    pub mod guardexprparser;
    pub mod descentcontextlexer;
    pub mod descentcontextparser;
    pub mod descentcontextparser_static;
    pub mod parenexprlexer;
    pub mod parenexprparser;
    pub mod parenexprparser_static;
    pub mod parenexprparser_static_only;
}

use std::cell::RefCell;
use std::rc::Rc;

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::int_stream::IntStream;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::rule_context::RuleContext;
use dbt_antlr_runtime::token::Token;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::tree::{
    ErrorNode, NodeInner, NodeKindType, ParseTreeListener, TerminalNode, TreeNode,
};
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::Parser;
use dbt_antlr_runtime::{Arena, InputStream};

use crate::gen::guardexprlexer::GuardExprLexer;
use crate::gen::guardexprparser::{
    GuardExprParser, GuardExpr_EOF, GuardExpr_T__1, GuardExpr_T__2, GuardExpr_T__3, RULE_expr,
    RULE_value,
};

/// Records parse listener events as text, prefixed with its tag.
#[derive(Clone, Default)]
struct RecordingListener {
    tag: &'static str,
    events: Rc<RefCell<Vec<String>>>,
}

impl RecordingListener {
    fn record(&self, event: String) {
        self.events
            .borrow_mut()
            .push(format!("{}{event}", self.tag));
    }
}

impl<'arena, Node, Tok> ParseTreeListener<'arena, Node, Tok> for RecordingListener
where
    Node: NodeKindType<'arena, Tok>,
    Tok: Token + 'arena,
{
    fn visit_terminal(&mut self, node: &TerminalNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        self.record(format!("token {}", node.symbol.get_text()));
        Ok(())
    }

    fn visit_error_node(&mut self, node: &ErrorNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        self.record(format!("error {}", node.symbol.get_text()));
        Ok(())
    }

    fn enter_every_rule(
        &mut self,
        ctx: &TreeNode<'_, 'arena, Node, Tok>,
    ) -> Result<(), ANTLRError> {
        self.record(format!("enter {}", ctx.get_rule_index()));
        Ok(())
    }

    fn exit_every_rule(&mut self, ctx: &TreeNode<'_, 'arena, Node, Tok>) -> Result<(), ANTLRError> {
        self.record(format!("exit {}", ctx.get_rule_index()));
        Ok(())
    }
}

/// Collects `syntax_error` notifications.
#[derive(Clone, Default)]
struct CollectingErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'input, 'arena, R, Tok>
    dbt_antlr_runtime::error_listener::ErrorListener<'input, 'arena, R, Tok>
    for CollectingErrorListener
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
            .push(format!("line {line}:{column} {msg}"));
    }
}

/// The parser of `input` with a recording parse listener and a collecting
/// error listener.
macro_rules! with_parser {
    ($input:expr, |$parser:ident, $events:ident, $errors:ident| $body:block) => {
        Arena::with(|arena| {
            let lexer =
                GuardExprLexer::<_, CommonTokenFactory>::new(arena, InputStream::new($input));
            let mut $parser = GuardExprParser::new(arena, CommonTokenStream::new(lexer));
            let listener = RecordingListener::default();
            let $events = listener.events.clone();
            $parser.add_parse_listener(Box::new(listener));
            let errors = CollectingErrorListener::default();
            let $errors = errors.messages.clone();
            $parser.remove_error_listeners();
            $parser.add_error_listener(Box::new(errors));
            $body
        })
    };
}

#[test]
fn neutral_parse_is_muted_and_resumes_its_subtree() {
    with_parser!("( a ) = b", |parser, events, errors| {
        // `( a )` is one `value`; the group has no prefix.
        let neutral = parser.begin_neutral_parse(&[], 0);
        let node = parser.value().expect("the neutral parse succeeds");
        assert_eq!(parser.input.la(1), GuardExpr_T__1, "the tail is `=`");
        let tail = parser.input.index();
        assert!(events.borrow().is_empty(), "the neutral parse is muted");
        parser.resume_neutral_parse(neutral, node.as_node(), RULE_value);
        assert_eq!(parser.input.index(), 0, "the stream is rewound");
        assert!(!parser.descent_ready(), "the re-descent is pending");

        // Another rule is not the shared rule.
        assert!(parser.resume_take(RULE_value + 1).unwrap().is_none());
        let taken = parser
            .resume_take(RULE_value)
            .unwrap()
            .expect("the shared rule takes the kept subtree");
        assert!(std::ptr::eq(taken, node.as_node()));
        assert_eq!(parser.input.index(), tail, "the stream moves to the tail");
        assert!(parser.descent_ready());
        assert_eq!(
            string_tree(taken, parser.get_rule_names()),
            "(value ( (expr (value a)) ))"
        );
        // The muted events are replayed in parse order.
        assert_eq!(
            *events.borrow(),
            [
                "enter 4", "token (", "enter 2", "enter 4", "token a", "exit 4", "exit 2",
                "token )", "exit 4",
            ]
        );
        assert!(errors.borrow().is_empty());
    });
}

#[test]
fn resume_at_another_position_parses_normally() {
    with_parser!("a ( b )", |parser, events, _errors| {
        let neutral = parser.begin_neutral_parse(&[], 0);
        let node = parser.value().unwrap();
        parser.resume_neutral_parse(neutral, node.as_node(), RULE_value);
        // The re-descent reaches the shared rule one token later: the kept
        // subtree of `a` does not belong there.
        parser.input.consume();
        assert!(parser.resume_take(RULE_value).unwrap().is_none());
        assert!(parser.descent_ready(), "the re-descent is over");
        let value = parser.value().unwrap();
        assert_eq!(
            string_tree(value, parser.get_rule_names()),
            "(value ( (expr (value b)) ))"
        );
        assert_eq!(events.borrow().first().map(String::as_str), Some("enter 4"));
    });
}

#[test]
fn abort_neutral_parse_releases_and_unmutes() {
    with_parser!("a", |parser, events, _errors| {
        let neutral = parser.begin_neutral_parse(&[], 0);
        parser.abort_neutral_parse(neutral);
        assert!(parser.descent_ready());
        let _ = parser.value().unwrap();
        assert!(!events.borrow().is_empty(), "parse listeners are unmuted");
    });
}

#[test]
fn neutral_parse_skips_the_prefix_by_consuming() {
    // `NOT` is the one-token prefix; the shared rule starts at `a`.
    with_parser!("NOT a", |parser, _events, _errors| {
        let neutral = parser.begin_neutral_parse(&[], 1);
        let node = parser.value().unwrap();
        assert_eq!(parser.input.la(1), GuardExpr_EOF);
        assert_eq!(string_tree(node, parser.get_rule_names()), "(value a)");
        parser.resume_neutral_parse(neutral, node.as_node(), RULE_value);
        assert_eq!(parser.input.la(1), GuardExpr_T__3, "rewound to `NOT`");
        parser.input.consume();
        let taken = parser.resume_take(RULE_value).unwrap().unwrap();
        assert!(std::ptr::eq(taken, node.as_node()));
    });
}
/// The listener events and the tree of a normal parse of `expr`.
fn live_expr_events(input: &'static str) -> (Vec<String>, String) {
    with_parser!(input, |parser, events, _errors| {
        let expr = parser.expr().unwrap();
        let tree = string_tree(expr, parser.get_rule_names());
        let events = events.borrow().clone();
        (events, tree)
    })
}

#[test]
fn left_recursive_shared_rule_resumes_like_a_live_parse() {
    let input = "a AND ( b OR c ) ;";
    let (live_events, live_tree) = live_expr_events(input);
    with_parser!(input, |parser, events, errors| {
        let neutral = parser.begin_neutral_parse(&[], 0);
        let node = parser.expr().unwrap();
        assert_eq!(parser.input.la(1), GuardExpr_T__2);
        parser.resume_neutral_parse(neutral, node.as_node(), RULE_expr);
        let taken = parser.resume_take(RULE_expr).unwrap().unwrap();
        assert_eq!(string_tree(taken, parser.get_rule_names()), live_tree);
        assert_eq!(parser.input.la(1), GuardExpr_T__2);
        // The replay fires the events in the order of the live parse, which
        // exits the previous context before it enters the recursion context.
        assert_eq!(*events.borrow(), live_events);
        assert!(errors.borrow().is_empty());
    });
}

#[test]
fn replay_dispatches_each_event_to_every_listener_in_order() {
    with_parser!("( a )", |parser, events, _errors| {
        parser.add_parse_listener(Box::new(RecordingListener {
            tag: "second ",
            events: events.clone(),
        }));
        let neutral = parser.begin_neutral_parse(&[], 0);
        let node = parser.value().unwrap();
        parser.resume_neutral_parse(neutral, node.as_node(), RULE_value);
        parser.resume_take(RULE_value).unwrap().unwrap();
        let events = events.borrow().clone();
        assert_eq!(
            events[..4],
            ["enter 4", "second enter 4", "token (", "second token ("]
        );
        assert_eq!(events[events.len() - 2..], ["second exit 4", "exit 4"]);
    });
}

#[test]
fn stale_resume_is_dropped_once_the_stream_passes_it() {
    with_parser!("a b", |parser, _events, _errors| {
        let neutral = parser.begin_neutral_parse(&[], 0);
        let node = parser.value().unwrap();
        parser.resume_neutral_parse(neutral, node.as_node(), RULE_value);
        assert!(!parser.descent_ready());
        parser.input.consume();
        assert!(
            parser.descent_ready(),
            "the re-descent passed the shared rule"
        );
        assert!(parser.resume_take(RULE_value).unwrap().is_none());
    });
}

/// What one parse of `input` with a `ParenExpr.g4` parser module produced.
#[derive(Debug, PartialEq, Eq)]
struct ParenParse {
    tree: String,
    errors: Vec<String>,
    events: Vec<String>,
}

macro_rules! parse_paren_with {
    ($module:ident, $input:expr) => {
        Arena::with(|arena| {
            let lexer = gen::parenexprlexer::ParenExprLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new($input),
            );
            let mut parser =
                gen::$module::ParenExprParser::new(arena, CommonTokenStream::new(lexer));
            let errors = CollectingErrorListener::default();
            let messages = errors.messages.clone();
            parser.remove_error_listeners();
            parser.add_error_listener(Box::new(errors));
            let listener = RecordingListener::default();
            let events = listener.events.clone();
            parser.add_parse_listener(Box::new(listener));
            let tree = match parser.s() {
                Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                Err(error) => format!("error: {error:?}"),
            };
            let errors = messages.borrow().clone();
            let events = events.borrow().clone();
            ParenParse {
                tree,
                errors,
                events,
            }
        })
    };
}

/// Inputs that reach the group of `'(' e (',' e)+ ')'` and `'(' e ')'`, the
/// subquery reading next to it, and deep nesting.
const PAREN_INPUTS: &[&str] = &[
    "(1)",
    "(1, 2)",
    "(1, 2, 3)",
    "(a)",
    "(a, b)",
    "((1))",
    "((1, 2))",
    "((1), 2)",
    "(1, (2, 3))",
    "(1 + 2)",
    "(1 + 2, 3 * 4)",
    "(a + b) * c",
    "-(1, 2)",
    "(-1)",
    "(SELECT a)",
    "((SELECT a))",
    "(((SELECT a)))",
    "((a))",
    "(((((x)))))",
    "((((1, 2))))",
    "((((1)), 2), (3, (4)))",
    "(1) + (2, 3) * (4)",
    "((a) + 1)",
    "((a), 1)",
];

/// Inputs with syntax errors inside and after the shared expression.
const PAREN_ERRORS: &[&str] = &[
    "(1,", "(1 2)", "()", "(1, )", "((1)", "(1))", "(+)", "(1 + )", "((1, 2)", "(SELECT)",
];

#[test]
fn paren_group_parses_like_the_adaptive_parser() {
    for input in PAREN_INPUTS {
        let adaptive = parse_paren_with!(parenexprparser, *input);
        assert!(adaptive.errors.is_empty(), "{input:?}: {adaptive:?}");
        let static_parse = parse_paren_with!(parenexprparser_static, *input);
        assert_eq!(static_parse, adaptive, "{input:?}");
        // A subquery or nested parentheses reach an escape state of the
        // table, which the static-only twin resolves to its minimum
        // alternative: a guess, not a group.
        if !input.contains("SELECT") && !input.contains("((") {
            let static_only = parse_paren_with!(parenexprparser_static_only, *input);
            assert_eq!(static_only, adaptive, "{input:?}");
        }
    }
}

#[test]
fn paren_group_reports_errors_like_the_adaptive_parser() {
    for input in PAREN_ERRORS {
        let adaptive = parse_paren_with!(parenexprparser, *input);
        assert!(!adaptive.errors.is_empty(), "{input:?}: {adaptive:?}");
        let static_parse = parse_paren_with!(parenexprparser_static, *input);
        assert!(
            !static_parse.errors.is_empty(),
            "{input:?}: {static_parse:?}"
        );
        // On `"((1, 2)"`, the outer `'('` reaches a group state, parses `(1, 2)`
        // as the shared `e`, and commits to the default member without
        // reverting the neutral parse.
        if *input != "((1, 2)" {
            assert_eq!(static_parse, adaptive, "{input:?}");
        }
    }
}

/// The static table of the primary block accepts with the group, so the
/// differential above goes through the neutral parse and the resume.
#[test]
fn paren_table_accepts_with_the_group() {
    use dbt_antlr_runtime::dfa::static_parser_dfa::VIRTUAL_ALT_BASE;
    Arena::with(|arena| {
        let manager = gen::parenexprparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");
        let groups = tables
            .tables()
            .iter()
            .flat_map(|table| (0..table.num_states() as i32).map(move |state| table.accept(state)))
            .filter(|accept| *accept >= VIRTUAL_ALT_BASE)
            .count();
        assert_eq!(groups, 8, "the group-accept states of the primary block");
    });
}

/// A neutral parse inside a neutral parse: the inner group resumes while the
/// outer one is still muted, and the outer replay covers both subtrees.
#[test]
fn nested_groups_replay_every_event_once() {
    let input = "((1, 2), ((3), 4))";
    let adaptive = parse_paren_with!(parenexprparser, input);
    let static_parse = parse_paren_with!(parenexprparser_static, input);
    assert_eq!(static_parse, adaptive);
    let enters = static_parse
        .events
        .iter()
        .filter(|event| event.starts_with("enter"))
        .count();
    let exits = static_parse
        .events
        .iter()
        .filter(|event| event.starts_with("exit"))
        .count();
    assert_eq!(enters, exits);
}

/// Without the adaptive fallback, a group whose tail matches no arm guesses
/// its minimum member. A wrong guess still reports a syntax error.
#[test]
fn static_only_group_fallbacks_still_report_errors() {
    for input in PAREN_ERRORS {
        let static_only = parse_paren_with!(parenexprparser_static_only, *input);
        assert!(!static_only.errors.is_empty(), "{input:?}: {static_only:?}");
    }
}

#[test]
fn no_descent_starts_in_the_sll_prediction_mode() {
    with_parser!("a", |parser, _events, _errors| {
        assert!(parser.descent_ready());
        parser
            .get_interpreter()
            .set_prediction_mode(dbt_antlr_runtime::PredictionMode::SLL);
        assert!(!parser.descent_ready());
    });
}

macro_rules! parse_context_with {
    ($module:ident, $input:expr) => {
        Arena::with(|arena| {
            let lexer = gen::descentcontextlexer::DescentContextLexer::<_, CommonTokenFactory>::new(
                arena,
                InputStream::new($input),
            );
            let mut parser =
                gen::$module::DescentContextParser::new(arena, CommonTokenStream::new(lexer));
            let errors = CollectingErrorListener::default();
            let messages = errors.messages.clone();
            parser.remove_error_listeners();
            parser.add_error_listener(Box::new(errors));
            let tree = match parser.s() {
                Ok(ctx) => string_tree(ctx, parser.get_rule_names()),
                Err(error) => format!("error: {error:?}"),
            };
            let errors = messages.borrow().clone();
            (tree, errors)
        })
    };
}

/// All candidate groups in `DescentContext.g4` violate Condition 2 (their
/// shared rule can both exit and continue on a token of the member follow
/// sets), so none is emitted and the static parser matches the adaptive
/// parser on both valid and invalid inputs.
#[test]
fn excluded_descent_context_rules_have_no_groups_and_match_adaptive() {
    use dbt_antlr_runtime::dfa::static_parser_dfa::VIRTUAL_ALT_BASE;
    Arena::with(|arena| {
        let manager = gen::descentcontextparser_static::parser_simulator_manager();
        let simulator = manager.get_simulator(arena);
        let tables = simulator
            .static_parser_tables()
            .expect("the parser has static tables");
        let groups = tables
            .tables()
            .iter()
            .flat_map(|table| (0..table.num_states() as i32).map(move |state| table.accept(state)))
            .filter(|accept| *accept >= VIRTUAL_ALT_BASE)
            .count();
        assert_eq!(groups, 0, "no shared-descent group is emitted");
    });

    for input in [
        "one a + b ; c 1",
        "one prefix a + b ; c 1 2",
        "one a + b ; c",
        "one ( a ) + b ; c",
        "two a . b , c as x ; y 1",
        "two a . b , c as x ; y",
        "two a . b , c ; y 1",
        "two prefix a . b , c as x ; y 1 2",
    ] {
        let adaptive = parse_context_with!(descentcontextparser, input);
        assert!(adaptive.1.is_empty(), "{input:?}: {adaptive:?}");
        let static_parse = parse_context_with!(descentcontextparser_static, input);
        assert_eq!(static_parse, adaptive, "{input:?}");
    }

    for input in [
        "one ( ; x",
        "one ( a ; b 1",
        "two ( a . ; y 1",
        "one a + ; b 1",
        "two a . b , c ; ;",
        "two ( a . b , c ; y 1 )",
        "two prefix ( a as x )",
        "two ( a as x )",
        "two ( a . b as x )",
        "two ( a )",
        "two prefix ( a )",
        "three prefix ( a as a )",
        "three ( a as a )",
    ] {
        let adaptive = parse_context_with!(descentcontextparser, input);
        assert!(!adaptive.1.is_empty(), "{input:?}: {adaptive:?}");
        let static_parse = parse_context_with!(descentcontextparser_static, input);
        assert_eq!(static_parse, adaptive, "{input:?}");
    }
}
