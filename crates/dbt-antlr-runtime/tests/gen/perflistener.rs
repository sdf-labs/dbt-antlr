// Generated from Perf.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::token::{CommonToken, Token};
use dbt_antlr_runtime::tree::ParseTreeListener;
use super::perfparser::*;

pub trait PerfListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, PerfParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link PerfParser#stat}.
    /// @param ctx the parse tree
    fn enter_stat<'input: 'arena>(&mut self, _ctx: &StatContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link PerfParser#stat}.
    /// @param ctx the parse tree
    fn exit_stat<'input: 'arena>(&mut self, _ctx: &StatContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link PerfParser#expr}.
    /// @param ctx the parse tree
    fn enter_expr<'input: 'arena>(&mut self, _ctx: &ExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link PerfParser#expr}.
    /// @param ctx the parse tree
    fn exit_expr<'input: 'arena>(&mut self, _ctx: &ExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
