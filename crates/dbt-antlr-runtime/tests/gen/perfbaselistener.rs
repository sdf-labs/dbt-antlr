// Generated from Perf.g4 by ANTLR 4.13.2

use super::perfparser::*;
use dbt_antlr_runtime::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by PerfParser.

pub trait PerfBaseListener<'arena>:
    ParseTreeListener<'arena, PerfParserNodeKind> {

    /**
     * Enter a parse tree produced by \{@link PerfBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_stat(&mut self, _ctx: &StatContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  PerfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_stat(&mut self, _ctx: &StatContext<'input, 'arena>) {}


    /**
     * Enter a parse tree produced by \{@link PerfBaseParser#s}.
     * @param ctx the parse tree
,      */
    fn enter_expr(&mut self, _ctx: &ExprContext<'input, 'arena>) {}
    /**
     * Exit a parse tree produced by \{@link  PerfBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expr(&mut self, _ctx: &ExprContext<'input, 'arena>) {}


}