// Generated from KeywordsParser.g4 by ANTLR 4.13.2
//! The `KeywordsParser` grammar for `dbt_antlr_runtime::ffi::export_grammars!`.

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::ffi::{
    AntlrRuleInfo, AntlrStr, AntlrTreeNode, ErrorCollector, Grammar, ParseOutput,
};
use dbt_antlr_runtime::parser::Parser as _;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::tree::NodeInner as _;
use dbt_antlr_runtime::{Arena, InputStream};

use super::keywordslexer::KeywordsLexer;
use super::keywordsparser::{self, KeywordsParser, KeywordsParserNode};

/// The `KeywordsParser` grammar.
pub static GRAMMAR: Grammar = Grammar::new(
    "KeywordsParser",
    &RULES,
    &LITERAL_NAMES,
    &SYMBOLIC_NAMES,
    parse,
    string_tree,
);

static RULES: [AntlrRuleInfo; 6] = [
    AntlrRuleInfo::new("from", &[], true),
    AntlrRuleInfo::new("statement", &[], true),
    AntlrRuleInfo::new("class", &[], true),
    AntlrRuleInfo::new("lambda", &[], true),
    AntlrRuleInfo::new("nested", &[], false),
    AntlrRuleInfo::new("boom", &[], true),
];

static LITERAL_NAMES: [AntlrStr; keywordsparser::_LITERAL_NAMES.len()] =
    dbt_antlr_runtime::ffi::names(&keywordsparser::_LITERAL_NAMES);

static SYMBOLIC_NAMES: [AntlrStr; keywordsparser::_SYMBOLIC_NAMES.len()] =
    dbt_antlr_runtime::ffi::names(&keywordsparser::_SYMBOLIC_NAMES);

fn parse<'a>(
    arena: &'a Arena,
    input: &'a str,
    rule_index: u32,
    errors: &ErrorCollector,
) -> Result<Option<ParseOutput>, ANTLRError> {
    let mut lexer = KeywordsLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
    lexer.remove_error_listeners();
    lexer.add_error_listener(errors.lexer_listener());
    let mut parser = KeywordsParser::new(arena, CommonTokenStream::new(lexer));
    parser.remove_error_listeners();
    parser.add_error_listener(errors.parser_listener());
    let root = match rule_index {
        0 => parser.from()?.as_node(),
        1 => parser.statement()?.as_node(),
        2 => parser.class()?.as_node(),
        3 => parser.lambda()?.as_node(),
        5 => parser.boom()?.as_node(),
        _ => return Ok(None),
    };
    Ok(Some(ParseOutput::new(root)))
}

/// # Safety
///
/// `node` must be a node of a live parse tree of this grammar.
unsafe fn string_tree(node: *const AntlrTreeNode) -> String {
    // SAFETY: The caller passes a node of a live parse tree of this grammar,
    // and `AntlrTreeNode` mirrors the layout of the tree nodes.
    let node = unsafe { &*node.cast::<KeywordsParserNode<'_, '_>>() };
    dbt_antlr_runtime::trees::string_tree(node, &keywordsparser::ruleNames)
}
