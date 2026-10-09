// Generated from VisitorBasic.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::token::{CommonToken, Token};
use dbt_antlr_runtime::tree::ParseTreeListener;
use super::visitorbasicparser::*;

pub trait VisitorBasicListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, VisitorBasicParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link VisitorBasicParser#s}.
    /// @param ctx the parse tree
    fn enter_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link VisitorBasicParser#s}.
    /// @param ctx the parse tree
    fn exit_s<'input: 'arena>(&mut self, _ctx: &SContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
