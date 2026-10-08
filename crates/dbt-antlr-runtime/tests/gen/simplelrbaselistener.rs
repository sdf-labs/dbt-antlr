// Generated from SimpleLR.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::token::{CommonToken, Token};
use dbt_antlr_runtime::tree::ParseTreeListener;
use super::simplelrparser::*;

/// A complete listener for a parse tree produced by SimpleLRParser,
/// with empty default bodies.
pub trait SimpleLRBaseListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, SimpleLRParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link SimpleLRParser#s}.
    /// @param ctx the parse tree
    fn enter_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link SimpleLRParser#s}.
    /// @param ctx the parse tree
    fn exit_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link SimpleLRParser#a}.
    /// @param ctx the parse tree
    fn enter_a<'input: 'arena>(&mut self, _ctx: &AContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link SimpleLRParser#a}.
    /// @param ctx the parse tree
    fn exit_a<'input: 'arena>(&mut self, _ctx: &AContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}