// Generated from ReferenceToATN.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::token::{CommonToken, Token};
use dbt_antlr_runtime::tree::ParseTreeListener;
use super::referencetoatnparser::*;

/// A complete listener for a parse tree produced by ReferenceToATNParser,
/// with empty default bodies.
pub trait ReferenceToATNBaseListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, ReferenceToATNParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link ReferenceToATNParser#a}.
    /// @param ctx the parse tree
    fn enter_a<'input: 'arena>(&mut self, _ctx: &AContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ReferenceToATNParser#a}.
    /// @param ctx the parse tree
    fn exit_a<'input: 'arena>(&mut self, _ctx: &AContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}