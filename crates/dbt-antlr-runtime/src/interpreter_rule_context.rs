//! Parse tree node types used by [crate::parser_interpreter::ParserInterpreter].
//!
//! This is the Rust port of Java's `InterpreterRuleContext`: a
//! `ParserRuleContext` that stores the rule index in a field instead of
//! hardcoding it per generated context class.
use std::fmt::Debug;

use crate::atn::INVALID_ALT;
use crate::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use crate::rule_context::CustomRuleContext;
use crate::token::{CommonToken, Token};
use crate::tree::{ErrorNode, NodeKindType, ParseTreeListener, TerminalNode, TreeNode};
use crate::{cast_unchecked, Arena};

/// Node tag for parse trees built by [crate::parser_interpreter::ParserInterpreter].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum InterpreterNodeKind {
    /// Rule context node, see [InterpreterRuleContext]
    Rule,
    /// Token leaf node
    Terminal,
    /// Error leaf node
    Error,
}

/// Extension data of [InterpreterRuleContext]: the backing field for
/// `get_rule_index`, see Java's `InterpreterRuleContext`, plus the backing
/// field for `get_alt_number`/`set_alt_number`, see the ANTLR tool's
/// `GrammarInterpreterRuleContext` (`outerAltNum`). The alt number stays
/// [INVALID_ALT] unless something (e.g.
/// [crate::parser_interpreter::ParserInterpreter::set_track_alt_numbers])
/// sets it.
#[derive(Debug)]
pub struct InterpreterRuleContextExt {
    rule_index: usize,
    alt_number: i32,
}

impl InterpreterRuleContextExt {
    pub(crate) fn new(rule_index: usize) -> Self {
        Self {
            rule_index,
            alt_number: INVALID_ALT,
        }
    }
}

/// `ParserRuleContext` with an explicitly stored rule index.
///
/// [BaseParserRuleContext] does not include field storage for the rule index
/// since the context types created by the code generator implement
/// `get_rule_index` to return the correct value for that type. The parser
/// interpreter does not use generated context types, so this type (with
/// slightly more memory overhead per node) provides equivalent functionality.
pub type InterpreterRuleContext<'input, 'arena, Tok = CommonToken<'input>> =
    BaseParserRuleContext<'input, 'arena, InterpreterRuleContextExt, InterpreterNodeKind, Tok>;

/// Parse tree node type used by the parser interpreter.
pub type InterpreterNode<'input, 'arena, Tok = CommonToken<'input>> =
    TreeNode<'input, 'arena, InterpreterNodeKind, Tok>;

impl<'input, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for InterpreterRuleContextExt
where
    'input: 'arena,
    Tok: Token + 'input,
{
    type NodeKind = InterpreterNodeKind;

    fn node_tag() -> InterpreterNodeKind {
        InterpreterNodeKind::Rule
    }

    fn make_node(
        arena: &'arena Arena,
        ctx: InterpreterRuleContext<'input, 'arena, Tok>,
    ) -> *mut InterpreterNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)
    }

    fn cast_from<'a>(
        node: &'a InterpreterNode<'input, 'arena, Tok>,
    ) -> Option<&'a InterpreterRuleContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(cast_unchecked!(node.ctx_ptr() => InterpreterRuleContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }

    fn cast_from_mut<'a>(
        node: &'a mut InterpreterNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut InterpreterRuleContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(cast_unchecked!(node.ctx_ptr() => mut InterpreterRuleContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }

    fn get_rule_index(&self) -> usize {
        self.rule_index
    }

    fn get_alt_number(&self) -> i32 {
        self.alt_number
    }

    fn set_alt_number(&mut self, alt_number: i32) {
        self.alt_number = alt_number;
    }
}

impl<'arena, Tok> NodeKindType<'arena, Tok> for InterpreterNodeKind
where
    Tok: Token + 'arena,
{
    type Listener = dyn ParseTreeListener<'arena, InterpreterNodeKind, Tok>;

    fn cast_to_ctx<'n, 'input: 'arena>(
        node: &'n InterpreterNode<'input, 'arena, Tok>,
    ) -> &'n dyn ParserRuleContext<'input, 'arena> {
        match node.node_tag() {
            InterpreterNodeKind::Rule => {
                cast_unchecked!(node.ctx_ptr() => InterpreterRuleContext<'input, 'arena, Tok>)
            }
            InterpreterNodeKind::Terminal => {
                cast_unchecked!(node.ctx_ptr() => TerminalNode<'input, 'arena, Tok>)
            }
            InterpreterNodeKind::Error => {
                cast_unchecked!(node.ctx_ptr() => ErrorNode<'input, 'arena, Tok>)
            }
        }
    }

    fn cast_to_ctx_mut<'n, 'input: 'arena>(
        node: &'n mut InterpreterNode<'input, 'arena, Tok>,
    ) -> &'n mut dyn ParserRuleContext<'input, 'arena> {
        match node.node_tag() {
            InterpreterNodeKind::Rule => {
                cast_unchecked!(node.ctx_ptr() => mut InterpreterRuleContext<'input, 'arena, Tok>)
            }
            InterpreterNodeKind::Terminal => {
                cast_unchecked!(node.ctx_ptr() => mut TerminalNode<'input, 'arena, Tok>)
            }
            InterpreterNodeKind::Error => {
                cast_unchecked!(node.ctx_ptr() => mut ErrorNode<'input, 'arena, Tok>)
            }
        }
    }

    fn set_alt_number<'input: 'arena>(
        node: &mut InterpreterNode<'input, 'arena, Tok>,
        alt_number: i32,
    ) {
        if let InterpreterNodeKind::Rule = node.node_tag() {
            cast_unchecked!(node.ctx_ptr() => mut InterpreterRuleContext<'input, 'arena, Tok>)
                .set_alt_number(alt_number)
        }
    }

    fn get_rule_index<'input: 'arena>(node: &InterpreterNode<'input, 'arena, Tok>) -> usize {
        match node.node_tag() {
            InterpreterNodeKind::Rule => crate::rule_context::RuleContext::get_rule_index(
                cast_unchecked!(node.ctx_ptr() => InterpreterRuleContext<'input, 'arena, Tok>),
            ),
            _ => usize::MAX,
        }
    }

    fn get_alt_number<'input: 'arena>(node: &InterpreterNode<'input, 'arena, Tok>) -> i32 {
        match node.node_tag() {
            InterpreterNodeKind::Rule => crate::rule_context::RuleContext::get_alt_number(
                cast_unchecked!(node.ctx_ptr() => InterpreterRuleContext<'input, 'arena, Tok>),
            ),
            _ => INVALID_ALT,
        }
    }

    #[inline]
    fn terminal() -> Self {
        Self::Terminal
    }

    #[inline]
    fn error() -> Self {
        Self::Error
    }

    #[inline]
    fn is_context(&self) -> bool {
        matches!(self, Self::Rule)
    }

    fn enter_rule<'input: 'arena>(
        _node: &InterpreterNode<'input, 'arena, Tok>,
        _listener: &mut Self::Listener,
    ) -> Result<(), crate::errors::ANTLRError> {
        Ok(())
    }

    fn exit_rule<'input: 'arena>(
        _node: &InterpreterNode<'input, 'arena, Tok>,
        _listener: &mut Self::Listener,
    ) -> Result<(), crate::errors::ANTLRError> {
        Ok(())
    }
}
