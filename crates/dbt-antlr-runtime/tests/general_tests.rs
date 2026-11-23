// #![feature(try_blocks)]
// #![feature(inner_deref)]
// #![feature(specialization)]
// #![feature(coerce_unsized)]
//! Integration tests

// #[macro_use]
// extern crate lazy_static;

#[rustfmt::skip]
mod gen {
    // Generated parsers/lexers: lint with the codegen template, not here.
    #![allow(clippy::all, warnings)]
    pub mod csvlexer;
    pub mod csvlistener;
    pub mod csvparser;
    pub mod csvvisitor;
    pub mod labelslexer;
    pub mod labelslistener;
    pub mod labelsparser;
    pub mod referencetoatnlexer;
    pub mod referencetoatnlistener;
    pub mod referencetoatnparser;
    pub mod simplelrlexer;
    pub mod simplelrlistener;
    pub mod simplelrparser;
    pub mod visitorcalclexer;
    pub mod visitorcalclistener;
    pub mod visitorcalcparser;
    pub mod visitorcalcvisitor;
    pub mod xmllexer;
}

use std::fmt::Write;
use std::iter::FromIterator;

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::int_stream::IntStream;
use dbt_antlr_runtime::lexer::Lexer;
use dbt_antlr_runtime::parser_rule_context::ParserRuleContext as _;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::rule_context::RuleContext as _;
use dbt_antlr_runtime::token::{Token, TOKEN_EOF};
use dbt_antlr_runtime::token_factory::{CommonTokenFactory, TokenFactory};
use dbt_antlr_runtime::token_stream::{TokenStream, UnbufferedTokenStream};
use dbt_antlr_runtime::tree::{ParseTreeListener, TerminalNode};
use dbt_antlr_runtime::trees::string_tree;
use dbt_antlr_runtime::{Arena, InputStream, Parser};
use serial_test::serial;

use crate::gen::csvlexer::*;
use crate::gen::csvlistener::*;
use crate::gen::csvparser::CSVParserNode;
use crate::gen::csvparser::{CSVParser, CSVParserNodeKind};
use crate::gen::labelslexer::LabelsLexer;
use crate::gen::labelsparser::{EContextAll, LabelsParser};
use crate::gen::referencetoatnlexer::ReferenceToATNLexer;
use crate::gen::referencetoatnlistener::ReferenceToATNListener;
use crate::gen::referencetoatnparser::ReferenceToATNParserNode;
use crate::gen::referencetoatnparser::{ReferenceToATNParser, ReferenceToATNParserNodeKind};
use crate::gen::simplelrlexer::SimpleLRLexer;
use crate::gen::simplelrlistener::SimpleLRListener;
use crate::gen::simplelrparser::{SimpleLRParser, SimpleLRParserNodeKind, SimpleLRTreeWalker};
use crate::gen::xmllexer::XMLLexer;
use crate::gen::*;
use crate::r#gen::simplelrparser::SimpleLRParserNode;

#[test]
fn lexer_test_xml() -> std::io::Result<()> {
    let data = r#"<?xml version="1.0"?>
<!--comment-->>
<?xml-stylesheet type="text/css" href="nutrition.css"?>
<script>
<![CDATA[
function f(x) {
if (x < x && a > 0) then duh
}
]]>
</script>"#
        .to_owned();

    Arena::with(|arena| {
        let mut _lexer = XMLLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(&data));
        //        _lexer.base.add_error_listener();
        let _a = "a".to_owned() + "";
        let mut string = String::new();
        {
            let mut token_source = UnbufferedTokenStream::new_unbuffered(&mut _lexer);
            while token_source.la(1) != TOKEN_EOF {
                {
                    let token = token_source.lt(1).unwrap();

                    let len =
                        token.get_stop_index() as usize + 1 - token.get_start_index() as usize;
                    string.extend(
                        format!(
                            "{},len {}:\n{}\n",
                            xmllexer::_SYMBOLIC_NAMES[token.get_token_type() as usize]
                                .unwrap_or(&format!("{}", token.get_token_type())),
                            len,
                            String::from_iter(
                                data.chars()
                                    .skip(token.get_start_index() as usize)
                                    .take(len)
                            )
                        )
                        .chars(),
                    );
                }
                token_source.consume();
            }
        }
        println!("{}", string);
        println!("{}", _lexer.get_interpreter().get_dfa().to_lexer_string());
    });
    Ok(())
}

#[test]
fn lexer_test_csv() {
    println!("test started lexer_test_csv");
    Arena::with(|arena| {
        let mut _lexer = CSVLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new("V123,V2\nd1,d222"),
            // Box::new(UTF16InputStream::from_str("V123,V2\nd1,d222","".into())),
        );
        let mut token_source = UnbufferedTokenStream::new_buffered(_lexer);
        let mut token_source_iter = token_source.token_iter();
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@0,0:3='V123',<5>,1:0]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@1,4:4=',',<1>,1:4]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@2,5:6='V2',<5>,1:5]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@3,7:7='\\n',<3>,1:7]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@4,8:9='d1',<5>,2:0]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@5,10:10=',',<1>,2:2]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@6,11:14='d222',<5>,2:3]"
        );
        assert_eq!(
            token_source_iter.next().unwrap().to_string(),
            "[@7,15:14='<EOF>',<-1>,2:7]"
        );
        assert!(token_source_iter.next().is_none());
    });
}

struct Listener {}

impl<'arena, Tok: Token + 'arena> ParseTreeListener<'arena, CSVParserNodeKind, Tok> for Listener {
    fn enter_every_rule(&mut self, ctx: &CSVParserNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        println!(
            "rule entered {}",
            csvparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error"),
        );
        Ok(())
    }
}

impl<'arena, Tok: Token + 'arena> CSVListener<'arena, Tok> for Listener {}

#[test]
fn parser_test_csv() {
    println!("test started");
    Arena::with(|arena| {
        let mut _lexer =
            CSVLexer::<_, CommonTokenFactory>::new(arena, InputStream::new("V123,V2\nd1,d2\n"));
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = CSVParser::new(arena, token_source);
        parser.add_parse_listener(Box::new(Listener {}));
        println!("\nstart parsing parser_test_csv");
        let result = parser.csvFile();
        assert!(result.is_ok());
        assert_eq!(
            string_tree(result.unwrap(), parser.get_rule_names()),
            "(csvFile (hdr (row (field V123) , (field V2) \\n)) (row (field d1) , (field d2) \\n))"
        );
    });
}

struct Listener2 {}

impl<'arena, Tok: Token + 'arena> ParseTreeListener<'arena, ReferenceToATNParserNodeKind, Tok>
    for Listener2
{
    fn enter_every_rule(
        &mut self,
        ctx: &ReferenceToATNParserNode<'_, 'arena, Tok>,
    ) -> Result<(), ANTLRError> {
        println!(
            "rule entered {}",
            referencetoatnparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        );
        Ok(())
    }
}

impl<'arena, Tok: Token + 'arena> ReferenceToATNListener<'arena, Tok> for Listener2 {}

#[test]
fn test_adaptive_predict_and_tree() {
    let text = "a 34 b".to_owned();
    Arena::with(|arena| {
        let mut _lexer = ReferenceToATNLexer::<_, CommonTokenFactory>::new(
            arena,
            InputStream::new(text.as_str()),
        );
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = ReferenceToATNParser::new(arena, token_source);
        parser.add_parse_listener(Box::new(Listener2 {}));
        println!("\nstart parsing adaptive_predict_test");
        let result = parser.a();
        assert!(result.is_ok());
    });
}

struct Listener3;

impl<'arena, Tok: Token + 'arena> ParseTreeListener<'arena, SimpleLRParserNodeKind, Tok>
    for Listener3
{
    fn visit_terminal(&mut self, node: &TerminalNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        println!("terminal node {}", node.symbol.get_text());
        Ok(())
    }

    fn enter_every_rule(
        &mut self,
        ctx: &SimpleLRParserNode<'_, 'arena, Tok>,
    ) -> Result<(), ANTLRError> {
        println!(
            "rule entered {}",
            simplelrparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        );
        Ok(())
    }

    fn exit_every_rule(
        &mut self,
        ctx: &SimpleLRParserNode<'_, 'arena, Tok>,
    ) -> Result<(), ANTLRError> {
        println!(
            "rule exited {}",
            simplelrparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        );
        Ok(())
    }
}

impl<'arena, Tok: Token + 'arena> SimpleLRListener<'arena, Tok> for Listener3 {}

#[test]
fn test_lr() {
    Arena::with(|arena| {
        let _lexer = SimpleLRLexer::<_, CommonTokenFactory>::new(arena, InputStream::new("x y z"));
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = SimpleLRParser::new(arena, token_source);
        parser.add_parse_listener(Box::new(Listener3));
        println!("\nstart parsing lr_test");
        let result = parser.s().expect("failed recursion parsion");
        assert_eq!(
            string_tree(result, parser.get_rule_names()),
            "(s (a (a (a x) y) z))"
        );
    });
}

#[test]
fn test_immediate_lr() {
    Arena::with(|arena| {
        let _lexer = SimpleLRLexer::<_, CommonTokenFactory>::new(arena, InputStream::new("x y z"));
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = SimpleLRParser::new(arena, token_source);
        parser.add_parse_listener(Box::new(Listener3));
        println!("\nstart parsing lr_test");
        let result = parser.a().expect("failed immediate recursion parsing");
        assert_eq!(
            string_tree(result, parser.get_rule_names()),
            "(a (a (a x) y) z)"
        );
    });
}

struct Listener4 {
    data: String,
}

impl<'arena, Tok: Token + 'arena> ParseTreeListener<'arena, SimpleLRParserNodeKind, Tok>
    for Listener4
{
    fn visit_terminal(&mut self, node: &TerminalNode<'_, 'arena, Tok>) -> Result<(), ANTLRError> {
        println!("enter terminal");
        let _ = writeln!(&mut self.data, "terminal node {}", node.symbol.get_text());
        Ok(())
    }
    fn enter_every_rule(
        &mut self,
        ctx: &SimpleLRParserNode<'_, 'arena, Tok>,
    ) -> Result<(), ANTLRError> {
        println!(
            "rule entered {}",
            simplelrparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        );
        Ok(())
    }
}

impl<'arena, Tok: Token + 'arena> SimpleLRListener<'arena, Tok> for Listener4 {}

#[test]
fn test_remove_listener() {
    Arena::with(|arena| {
        let mut _lexer =
            SimpleLRLexer::<_, CommonTokenFactory>::new(arena, InputStream::new("x y z"));
        let token_source = CommonTokenStream::new(_lexer);
        let mut parser = SimpleLRParser::new(arena, token_source);
        parser.add_parse_listener(Box::new(Listener3));
        let id = parser.add_parse_listener(Box::new(Listener4 {
            data: String::new(),
        }));
        let result = parser.s().expect("expected to parse successfully");

        let mut listener = parser.remove_parse_listener(id);
        assert_eq!(
            &listener.data,
            "terminal node x\nterminal node y\nterminal node z\n"
        );

        println!("--------");
        listener.data.clear();

        let listener = SimpleLRTreeWalker::walk(listener, result);
        assert_eq!(
            &listener.unwrap().data,
            "terminal node x\nterminal node y\nterminal node z\n"
        );
    });
}
#[test]
fn test_byte_parser() {}

#[test]
#[serial(labelsparser)]
fn test_complex_convert() {
    let input = "(a+4)*2";
    // let codepoints = "(a+4)*2";
    Arena::with(|arena| {
        let input = InputStream::new(input);
        let lexer = LabelsLexer::<_, CommonTokenFactory>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        let result = parser.s().expect("parser error");
        let string = result.q.unwrap().get_v();
        assert_eq!("* + a 4 2", string);
        let x = result.q.unwrap();
        match x {
            EContextAll::MultContext(x) => {
                assert_eq!("(a+4)", x.a.unwrap().get_text())
            }
            _ => panic!("oops"),
        }
    });
}

#[test]
fn test_ast_type_variance() {
    // This is a compile-only test to make sure that the lifetimes on the
    // AST types are working properly
    #![allow(dead_code)]

    // fn context_check1<'long, 'short, 'arena>(
    //     x: visitorcalcparser::ExprContextAll<'long, 'arena>,
    // ) -> visitorcalcparser::ExprContextAll<'short, 'arena>
    // where
    //     'long: 'short,
    //     'short: 'arena,
    // {
    //     x
    // }

    // This should not compile -- 'arena is invariant
    // fn context_check2<'input, 'long, 'short>(
    //     x: visitorcalcparser::ExprContextAll<'input, 'long>,
    // ) -> visitorcalcparser::ExprContextAll<'input, 'short>
    // where
    //     'input: 'long,
    //     'long: 'short,
    // {
    //     x
    // }

    // fn node_check1<'long, 'short, 'arena>(
    //     x: visitorcalcparser::VisitorCalcParserContextNode<'long, 'arena>,
    // ) -> visitorcalcparser::VisitorCalcParserContextNode<'short, 'arena>
    // where
    //     'long: 'short,
    //     'short: 'arena,
    // {
    //     x
    // }

    // This should not compile -- 'arena is invariant
    // fn node_check2<'input, 'long, 'short>(
    //     x: visitorcalcparser::VisitorCalcParserContextNode<'input, 'long>,
    // ) -> visitorcalcparser::VisitorCalcParserContextNode<'input, 'short>
    // where
    //     'input: 'long,
    //     'long: 'short,
    // {
    //     x
    // }

    // This should not compile -- error listeners can not hold onto 'arena references:
    // struct ErrorListenerImpl<'input, 'arena> {
    //     input: &'input str,
    //     _marker: PhantomData<&'arena ()>,
    // }

    struct ErrorListenerImpl<'input> {
        input: &'input str,
    }

    impl<'input, 'arena, Input, TF>
        ErrorListener<
            'input,
            'arena,
            visitorcalcparser::BaseParserType<'input, 'arena, Input, TF>,
            TF::Tok,
        > for ErrorListenerImpl<'input>
    where
        'input: 'arena,
        TF: TokenFactory<'input, 'arena> + 'arena,
        Input: TokenStream<'input, 'arena, TF> + 'arena,
    {
    }

    fn parse_visitor_calc<'input, 'arena>(
        input: &'input str,
        arena: &'arena Arena,
        error_listener: ErrorListenerImpl<'input>,
    ) -> Result<&'arena visitorcalcparser::SContext<'input, 'arena>, ANTLRError>
    where
        'input: 'arena,
    {
        let input = InputStream::new(input);
        let lexer = visitorcalclexer::VisitorCalcLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = visitorcalcparser::VisitorCalcParser::new(arena, token_source);
        parser.add_error_listener(Box::new(error_listener));
        parser.s()
    }
}

// Deep recursion support requires stacker. Without stacker, the test will
// fail with stack overflow
#[cfg(feature = "stacker")]
#[test]
#[serial(labelsparser)]
fn test_deep_recursion() {
    use crate::gen::labelsparser::SContextAttrs as _;

    let input = "(".repeat(2500) + "a" + &(")".repeat(2500));
    Arena::with(|arena| {
        let input = InputStream::new(input.as_str());
        let lexer = LabelsLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        #[cfg(feature = "recursion-limit")]
        {
            parser.set_recursion_limit(2502);
        }
        let result = parser.s().expect("parser error");
        assert!(result.q.is_some());
    });

    let input = "{".repeat(2500) + "a" + &("}".repeat(2500));
    Arena::with(|arena| {
        let input = InputStream::new(input.as_str());
        let lexer = LabelsLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        #[cfg(feature = "recursion-limit")]
        {
            parser.set_recursion_limit(5002);
        }
        let result = parser.s().expect("parser error");
        assert!(result.blk().is_some());
    });
}

#[cfg(feature = "recursion-limit")]
#[test]
#[serial(labelsparser)]
fn test_recursion_limit() {
    let input = "(".repeat(1998) + "a" + &(")".repeat(1998));
    Arena::with(|arena| {
        let input = InputStream::new(input.as_str());
        let lexer = LabelsLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        let result = parser.s();
        assert!(result.is_ok());
    });

    Arena::with(|arena| {
        let input = InputStream::new(input.as_str());
        let lexer = LabelsLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        parser.set_recursion_limit(100);
        let result = parser.s().unwrap_err();
        eprintln!("result: {}", result);
        assert!(result
            .to_string()
            .contains("Recursion limit of 100 exceeded"));
    });
}

#[test]
#[serial(labelsparser)]
fn test_parser_allocation_limit() {
    struct LabelParserAllocationLimitGuard;

    impl Drop for LabelParserAllocationLimitGuard {
        fn drop(&mut self) {
            labelslexer::lexer_simulator_manager().set_allocation_limit_bytes(0);
            labelsparser::parser_simulator_manager().set_allocation_limit_bytes(0);
        }
    }

    let input = "((a + 4) * 2)";
    {
        let _guard = LabelParserAllocationLimitGuard {};

        labelsparser::parser_simulator_manager().set_allocation_limit_bytes(1);

        Arena::with(|arena| {
            let input = InputStream::new(input);
            let lexer = LabelsLexer::<_>::new(arena, input);
            let token_source = CommonTokenStream::new(lexer);
            let mut parser = LabelsParser::new(arena, token_source);

            let result = parser.s().unwrap_err();
            assert!(result.to_string().contains("Memory limit of 1B exceeded"));
        });
    }
}

#[test]
fn test_error_listeners_ordering_and_dispatch() {
    use std::cell::RefCell;
    use std::rc::Rc;

    struct RecordingErrorListener {
        tag: &'static str,
        events: Rc<RefCell<Vec<String>>>,
    }

    impl<'input, 'arena, R, Tok> ErrorListener<'input, 'arena, R, Tok> for RecordingErrorListener
    where
        'input: 'arena,
        R: Recognizer<'input, 'arena, Tok>,
        Tok: Token + 'input,
    {
        fn syntax_error(
            &self,
            _recognizer: &R,
            _offending_symbol: Option<&dyn Token>,
            _line: u32,
            _column: i32,
            msg: &str,
            _error: Option<&ANTLRError>,
        ) {
            self.events.borrow_mut().push(format!("{}:{msg}", self.tag));
        }
    }

    let events = Rc::new(RefCell::new(Vec::new()));
    Arena::with(|arena| {
        let input = InputStream::new("((+))");
        let lexer = LabelsLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(RecordingErrorListener {
            tag: "primary",
            events: Rc::clone(&events),
        }));
        parser.add_error_listener(Box::new(RecordingErrorListener {
            tag: "second",
            events: Rc::clone(&events),
        }));
        parser.add_error_listener(Box::new(RecordingErrorListener {
            tag: "third",
            events: Rc::clone(&events),
        }));

        let _ = parser.s();
        assert!(
            events.borrow().len() >= 3,
            "expected error notifications from all listeners"
        );
        // Each syntax error notifies listeners in registration order: primary, second, third.
        assert!(events.borrow()[0].starts_with("primary:"));
        assert!(events.borrow()[1].starts_with("second:"));
        assert!(events.borrow()[2].starts_with("third:"));

        events.borrow_mut().clear();
        parser
            .get_error_lister_dispatch()
            .syntax_error(&*parser, None, 1, 0, "direct", None);
        assert_eq!(
            *events.borrow(),
            vec![
                "primary:direct".to_owned(),
                "second:direct".to_owned(),
                "third:direct".to_owned()
            ]
        );

        events.borrow_mut().clear();
        parser.remove_error_listeners();
        parser.notify_error_listeners("silenced".to_owned(), None, None);
        assert!(events.borrow().is_empty());
    });
}

#[test]
fn test_error_strategy_delegate_default_and_custom() {
    use dbt_antlr_runtime::error_strategy::{
        BailErrorStrategy, ErrorStrategy, ErrorStrategyDelegate,
    };

    Arena::with(|arena| {
        // Inline default delegate: parses valid input and recovers from syntax error.
        let input = InputStream::new("a + 4");
        let lexer = LabelsLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = LabelsParser::new(arena, token_source);
        let mut default_delegate = ErrorStrategyDelegate::new_default();
        assert!(format!("{default_delegate:?}").contains("default"));
        assert!(!default_delegate.in_error_recovery_mode(&mut *parser));
        default_delegate.report_match(&mut *parser);
        assert!(!default_delegate.in_error_recovery_mode(&mut *parser));

        // Custom boxed delegate (BailErrorStrategy) bails immediately on syntax error.
        let bad_input = InputStream::new("((+))");
        let mut bad_lexer = LabelsLexer::<_>::new(arena, bad_input);
        bad_lexer.remove_error_listeners();
        let bad_tokens = CommonTokenStream::new(bad_lexer);
        let mut bad_parser =
            LabelsParser::with_strategy(arena, bad_tokens, Box::new(BailErrorStrategy::new()));
        bad_parser.remove_error_listeners();
        assert!(bad_parser.s().is_err());
    });
}

#[test]
fn test_parse_listener_add_remove_toggles_events() {
    use std::cell::Cell;
    use std::rc::Rc;

    struct CountListener {
        enters: Rc<Cell<usize>>,
        exits: Rc<Cell<usize>>,
        terminals: Rc<Cell<usize>>,
    }

    impl<'arena, Tok: Token + 'arena> ParseTreeListener<'arena, SimpleLRParserNodeKind, Tok>
        for CountListener
    {
        fn visit_terminal(
            &mut self,
            _node: &TerminalNode<'_, 'arena, Tok>,
        ) -> Result<(), ANTLRError> {
            self.terminals.set(self.terminals.get() + 1);
            Ok(())
        }

        fn enter_every_rule<'input>(
            &mut self,
            _ctx: &SimpleLRParserNode<'input, 'arena, Tok>,
        ) -> Result<(), ANTLRError> {
            self.enters.set(self.enters.get() + 1);
            Ok(())
        }

        fn exit_every_rule<'input>(
            &mut self,
            _ctx: &SimpleLRParserNode<'input, 'arena, Tok>,
        ) -> Result<(), ANTLRError> {
            self.exits.set(self.exits.get() + 1);
            Ok(())
        }
    }

    impl<'arena, Tok: Token + 'arena> SimpleLRListener<'arena, Tok> for CountListener {}

    let enters = Rc::new(Cell::new(0));
    let exits = Rc::new(Cell::new(0));
    let terminals = Rc::new(Cell::new(0));

    Arena::with(|arena| {
        let input = InputStream::new("x y z");
        let lexer = SimpleLRLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = SimpleLRParser::new(arena, token_source);
        let id = parser.add_parse_listener(Box::new(CountListener {
            enters: Rc::clone(&enters),
            exits: Rc::clone(&exits),
            terminals: Rc::clone(&terminals),
        }));
        let _ = parser.s().expect("parse succeeds");
        assert!(enters.get() > 0);
        assert_eq!(enters.get(), exits.get());
        assert_eq!(terminals.get(), 3);

        // Removing the listener deactivates listener dispatch.
        let _removed = parser.remove_parse_listener(id);
        let before = enters.get();
        let input2 = InputStream::new("a b");
        let lexer2 = SimpleLRLexer::<_>::new(arena, input2);
        let token_source2 = CommonTokenStream::new(lexer2);
        parser.set_input_stream(token_source2);
        let _ = parser.s().expect("second parse succeeds");
        assert_eq!(enters.get(), before);
    });
}

#[test]
fn test_token_of_type_and_tokens_of_type() {
    use gen::csvlexer::CSVLexer;
    use gen::csvparser::{
        CSVParser, CsvFileContextAttrs, FieldContextAttrs, HdrContextAttrs, RowContextAttrs,
        CSV_STRING, CSV_TEXT, CSV_T__0,
    };

    Arena::with(|arena| {
        let input = InputStream::new("a,\"b\",c\n");
        let lexer = CSVLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = CSVParser::new(arena, token_source);
        let file = parser.csvFile().expect("csv parse succeeds");
        let row = file.hdr().expect("hdr").row().expect("row");

        assert!(row.token_of_type(CSV_TEXT, 0).is_none());
        assert!(row.tokens_of_type(CSV_TEXT).is_empty());

        assert_eq!(row.tokens_of_type(CSV_T__0).len(), 2);
        assert_eq!(row.token_of_type(CSV_T__0, 0).unwrap().get_text(), ",");
        assert_eq!(row.token_of_type(CSV_T__0, 1).unwrap().get_text(), ",");
        assert!(row.token_of_type(CSV_T__0, 2).is_none());
        let fields = row.field_all();
        assert_eq!(fields.len(), 3);
        assert_eq!(fields[0].TEXT().unwrap().get_text(), "a");
        assert_eq!(
            fields[0].token_of_type(CSV_TEXT, 0).unwrap().get_text(),
            "a"
        );
        assert!(fields[0].token_of_type(CSV_TEXT, 1).is_none());
        assert!(fields[0].STRING().is_none());
        assert_eq!(fields[0].tokens_of_type(CSV_TEXT).len(), 1);
        assert_eq!(fields[1].STRING().unwrap().get_text(), "\"b\"");
        assert_eq!(fields[1].tokens_of_type(CSV_STRING).len(), 1);
    });
}

#[test]
fn test_plain_rule_entry_alt_number_and_error_handling() {
    use dbt_antlr_runtime::atn::INVALID_ALT;
    use dbt_antlr_runtime::error_strategy::BailErrorStrategy;
    use gen::simplelrparser::{AContextAttrs, SContextAttrs};

    Arena::with(|arena| {
        let input = InputStream::new("x y z");
        let lexer = SimpleLRLexer::<_>::new(arena, input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = SimpleLRParser::new(arena, token_source);
        let root = parser.s().expect("plain + plain-rec parse succeeds");
        assert_eq!(root.get_alt_number(), INVALID_ALT);
        assert_eq!(root.start().get_text(), "x");
        assert_eq!(root.stop().get_text(), "z");
        let inner = root.a().expect("a child");
        assert_eq!(inner.get_alt_number(), INVALID_ALT);
        assert_eq!(inner.ID().unwrap().get_text(), "z");

        // Recoverable error through handle_rule_error on a plain rule.
        let bad_input = InputStream::new("");
        let bad_lexer = SimpleLRLexer::<_>::new(arena, bad_input);
        let bad_tokens = CommonTokenStream::new(bad_lexer);
        let mut rec_parser = SimpleLRParser::new(arena, bad_tokens);
        rec_parser.remove_error_listeners();
        assert!(rec_parser.s().is_ok());

        // Unrecoverable error through handle_rule_error (BailErrorStrategy).
        let bail_input = InputStream::new("");
        let bail_lexer = SimpleLRLexer::<_>::new(arena, bail_input);
        let bail_tokens = CommonTokenStream::new(bail_lexer);
        let mut bail_parser =
            SimpleLRParser::with_strategy(arena, bail_tokens, Box::new(BailErrorStrategy::new()));
        bail_parser.remove_error_listeners();
        assert!(bail_parser.s().is_err());
    });
}

#[test]
fn test_parse_rule_catch_plain_and_catch_plain_rec() {
    use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager;
    use dbt_antlr_runtime::error_strategy::ErrorStrategyDelegate;
    use dbt_antlr_runtime::errors::ANTLRErrorKind;
    use dbt_antlr_runtime::parser::{BaseParser, ParserRecog};
    use dbt_antlr_runtime::recognizer::Actions;
    use dbt_antlr_runtime::token_factory::TokenFactory;
    use dbt_antlr_runtime::token_stream::TokenStream;
    use dbt_antlr_runtime::vocabulary::{Vocabulary, VocabularyImpl};
    use gen::simplelrparser::{
        parser_simulator_manager, AContext, SContext, SimpleLRParserNodeKind, _LITERAL_NAMES,
        _SYMBOLIC_NAMES,
    };
    use std::sync::LazyLock;

    static TEST_VOCAB: LazyLock<VocabularyImpl> =
        LazyLock::new(|| VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));

    struct TestRecog;

    impl<'input, 'arena, Input, TF>
        Actions<
            'input,
            'arena,
            BaseParser<'input, 'arena, Self, SimpleLRParserNodeKind, Input, TF>,
            TF::Tok,
        > for TestRecog
    where
        'input: 'arena,
        TF: TokenFactory<'input, 'arena> + 'arena,
        Input: TokenStream<'input, 'arena, TF> + 'arena,
    {
        fn get_vocabulary(&self) -> &dyn Vocabulary {
            &*TEST_VOCAB
        }
    }

    impl<'input, 'arena, Input, TF>
        ParserRecog<
            'input,
            'arena,
            BaseParser<'input, 'arena, Self, SimpleLRParserNodeKind, Input, TF>,
            TF::Tok,
        > for TestRecog
    where
        'input: 'arena,
        TF: TokenFactory<'input, 'arena> + 'arena,
        Input: TokenStream<'input, 'arena, TF> + 'arena,
    {
        fn get_atn_simulator_man(&self) -> &'static ParserATNSimulatorManager {
            parser_simulator_manager()
        }
    }

    type TestBase<'input, 'arena, Input, TF> =
        BaseParser<'input, 'arena, TestRecog, SimpleLRParserNodeKind, Input, TF>;

    struct CatchPlainHarness<'input, 'arena, Input, TF>
    where
        'input: 'arena,
        TF: TokenFactory<'input, 'arena> + 'arena,
        Input: TokenStream<'input, 'arena, TF> + 'arena,
    {
        base: TestBase<'input, 'arena, Input, TF>,
        err_handler: ErrorStrategyDelegate<'input, 'arena, TF, TestBase<'input, 'arena, Input, TF>>,
    }

    impl<'input, 'arena, Input, TF> CatchPlainHarness<'input, 'arena, Input, TF>
    where
        'input: 'arena,
        TF: TokenFactory<'input, 'arena> + 'arena,
        Input: TokenStream<'input, 'arena, TF> + 'arena,
    {
        fn new(arena: &'arena Arena, input: Input) -> Self {
            let mut base = BaseParser::new_base_parser(arena, input, TestRecog);
            base.remove_error_listeners();
            Self {
                base,
                err_handler: ErrorStrategyDelegate::new_default(),
            }
        }

        fn run_catch_plain(
            &mut self,
            mode: u8,
            caught: &mut bool,
            finally_ran: &mut bool,
        ) -> Result<&'arena SContext<'input, 'arena, TF::Tok>, ANTLRError> {
            dbt_antlr_runtime::parse_rule!(
                catch plain recog = self, SimpleLRParserNodeKind::SContext, 0;
                {
                    match mode {
                        1 => return Err(ANTLRError::no_alt(&mut recog.base)),
                        2 => return Err(ANTLRError::input_mismatch(&mut recog.base)),
                        _ => {}
                    }
                };
                [Err(ref e) if matches!(&**e, ANTLRErrorKind::NoAltError(_)) => { *caught = true; }];
                [*finally_ran = true;]
            )
        }

        fn run_catch_plain_rec(
            &mut self,
            _p: i32,
            mode: u8,
            caught: &mut bool,
            finally_ran: &mut bool,
        ) -> Result<&'arena AContext<'input, 'arena, TF::Tok>, ANTLRError> {
            dbt_antlr_runtime::parse_rule!(
                catch plain rec recog = self, _parentctx, _parent_state, SimpleLRParserNodeKind::AContext, 2; _p;
                {
                    let _ = _parent_state;
                    match mode {
                        1 => return Err(ANTLRError::no_alt(&mut recog.base)),
                        2 => return Err(ANTLRError::input_mismatch(&mut recog.base)),
                        _ => {}
                    }
                };
                [Err(ref e) if matches!(&**e, ANTLRErrorKind::NoAltError(_)) => { *caught = true; }];
                [*finally_ran = true;]
            )
        }
    }

    Arena::with(|arena| {
        for (mode, expect_caught) in [(0, false), (1, true), (2, false)] {
            let lexer = SimpleLRLexer::<_>::new(arena, InputStream::new("x"));
            let mut harness = CatchPlainHarness::new(arena, CommonTokenStream::new(lexer));
            let (mut caught, mut finally_ran) = (false, false);
            assert!(harness
                .run_catch_plain(mode, &mut caught, &mut finally_ran)
                .is_ok());
            assert_eq!(caught, expect_caught);
            assert!(finally_ran);

            let lexer_rec = SimpleLRLexer::<_>::new(arena, InputStream::new("x"));
            let mut harness_rec = CatchPlainHarness::new(arena, CommonTokenStream::new(lexer_rec));
            let (mut caught, mut finally_ran) = (false, false);
            assert!(harness_rec
                .run_catch_plain_rec(0, mode, &mut caught, &mut finally_ran)
                .is_ok());
            assert_eq!(caught, expect_caught);
            assert!(finally_ran);
        }
    });
}
