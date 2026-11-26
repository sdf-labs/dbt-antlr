// Generated from Labels.g4 by ANTLR 4.13.2
//! The `Labels` grammar for `dbt_antlr_runtime::ffi::export_grammars!`.

use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::ffi::{
    AntlrRuleInfo, AntlrStr, AntlrTreeNode, ErrorCollector, Grammar, ParseOutput,
};
use dbt_antlr_runtime::parser::Parser as _;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::tree::NodeInner as _;
use dbt_antlr_runtime::{Arena, InputStream};

use super::labelslexer::LabelsLexer;
use super::labelsparser::{self, LabelsParser, LabelsParserNode};

/// The `Labels` grammar.
pub static GRAMMAR: Grammar = Grammar::new(
    "Labels",
    &RULES,
    &LITERAL_NAMES,
    &SYMBOLIC_NAMES,
    parse,
    string_tree,
);

static RULES: [AntlrRuleInfo; 3] = [
    AntlrRuleInfo::new("s", &[], true),
    AntlrRuleInfo::new(
        "e",
        &[
            AntlrStr::new("add"),
            AntlrStr::new("parens"),
            AntlrStr::new("mult"),
            AntlrStr::new("dec"),
            AntlrStr::new("anID"),
            AntlrStr::new("anInt"),
            AntlrStr::new("inc"),
        ],
        true,
    ),
    AntlrRuleInfo::new("blk", &[], true),
];

static LITERAL_NAMES: [AntlrStr; labelsparser::_LITERAL_NAMES.len()] =
    dbt_antlr_runtime::ffi::names(&labelsparser::_LITERAL_NAMES);

static SYMBOLIC_NAMES: [AntlrStr; labelsparser::_SYMBOLIC_NAMES.len()] =
    dbt_antlr_runtime::ffi::names(&labelsparser::_SYMBOLIC_NAMES);

fn parse<'a>(
    arena: &'a Arena,
    input: &'a str,
    rule_index: u32,
    errors: &ErrorCollector,
) -> Result<Option<ParseOutput>, ANTLRError> {
    let mut lexer = LabelsLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
    lexer.remove_error_listeners();
    lexer.add_error_listener(errors.lexer_listener());
    let mut parser = LabelsParser::new(arena, CommonTokenStream::new(lexer));
    parser.remove_error_listeners();
    parser.add_error_listener(errors.parser_listener());
    let root = match rule_index {
        0 => parser.s()?.as_node(),
        1 => parser.e()?.as_node(),
        2 => parser.blk()?.as_node(),
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
    let node = unsafe { &*node.cast::<LabelsParserNode<'_, '_>>() };
    dbt_antlr_runtime::trees::string_tree(node, &labelsparser::ruleNames)
}
