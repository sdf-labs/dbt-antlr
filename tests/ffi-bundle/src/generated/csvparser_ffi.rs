// Generated from CSV.g4 by ANTLR 4.13.2
//! The `CSV` grammar for `dbt_antlr_ffi::export_grammars!`.

use dbt_antlr_ffi::{AntlrRuleInfo, AntlrStr, AntlrTreeNode, ErrorCollector, Grammar, ParseOutput};
use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::parser::Parser as _;
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::tree::NodeInner as _;
use dbt_antlr_runtime::{Arena, InputStream};

use super::csvlexer::CSVLexer;
use super::csvparser::{self, CSVParser, CSVParserNode};

/// The `CSV` grammar.
pub static GRAMMAR: Grammar = Grammar::new(
    "CSV",
    &RULES,
    &LITERAL_NAMES,
    &SYMBOLIC_NAMES,
    parse,
    string_tree,
);

static RULES: [AntlrRuleInfo; 4] = [
    AntlrRuleInfo::new("csvFile", &[], true),
    AntlrRuleInfo::new("hdr", &[], true),
    AntlrRuleInfo::new("row", &[], true),
    AntlrRuleInfo::new("field", &[], true),
];

static LITERAL_NAMES: [AntlrStr; csvparser::_LITERAL_NAMES.len()] =
    dbt_antlr_ffi::names(&csvparser::_LITERAL_NAMES);

static SYMBOLIC_NAMES: [AntlrStr; csvparser::_SYMBOLIC_NAMES.len()] =
    dbt_antlr_ffi::names(&csvparser::_SYMBOLIC_NAMES);

fn parse<'a>(
    arena: &'a Arena,
    input: &'a str,
    rule_index: u32,
    errors: &ErrorCollector,
) -> Result<Option<ParseOutput>, ANTLRError> {
    let mut lexer = CSVLexer::<_, CommonTokenFactory>::new(arena, InputStream::new(input));
    lexer.remove_error_listeners();
    lexer.add_error_listener(errors.lexer_listener());
    let mut parser = CSVParser::new(arena, CommonTokenStream::new(lexer));
    parser.remove_error_listeners();
    parser.add_error_listener(errors.parser_listener());
    let root = match rule_index {
        0 => parser.csvFile()?.as_node(),
        1 => parser.hdr()?.as_node(),
        2 => parser.row()?.as_node(),
        3 => parser.field()?.as_node(),
        _ => return Ok(None),
    };
    Ok(Some(ParseOutput::new(
        root,
        parser.get_token_factory().source_info(),
    )))
}

/// # Safety
///
/// `node` must be a node of a live parse tree of this grammar.
unsafe fn string_tree(node: *const AntlrTreeNode) -> String {
    // SAFETY: The caller passes a node of a live parse tree of this grammar,
    // and `AntlrTreeNode` mirrors the layout of the tree nodes.
    let node = unsafe { &*node.cast::<CSVParserNode<'_, '_>>() };
    dbt_antlr_runtime::trees::string_tree(node, &csvparser::ruleNames)
}
