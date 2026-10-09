// Generated from ANTLRv4Parser.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::token::{CommonToken, Token};
use dbt_antlr_runtime::tree::ParseTreeListener;
use super::antlrv4parser::*;

pub trait ANTLRv4ParserListener<'arena, Tok = CommonToken<'arena>> : ParseTreeListener<'arena, ANTLRv4ParserNodeKind, Tok>
where
    Tok: Token + 'arena,
{
    /// Enter a parse tree produced by {@link ANTLRv4Parser#grammarSpec}.
    /// @param ctx the parse tree
    fn enter_grammarSpec<'input: 'arena>(&mut self, _ctx: &GrammarSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#grammarSpec}.
    /// @param ctx the parse tree
    fn exit_grammarSpec<'input: 'arena>(&mut self, _ctx: &GrammarSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#grammarDecl}.
    /// @param ctx the parse tree
    fn enter_grammarDecl<'input: 'arena>(&mut self, _ctx: &GrammarDeclContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#grammarDecl}.
    /// @param ctx the parse tree
    fn exit_grammarDecl<'input: 'arena>(&mut self, _ctx: &GrammarDeclContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#grammarType}.
    /// @param ctx the parse tree
    fn enter_grammarType<'input: 'arena>(&mut self, _ctx: &GrammarTypeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#grammarType}.
    /// @param ctx the parse tree
    fn exit_grammarType<'input: 'arena>(&mut self, _ctx: &GrammarTypeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#prequelConstruct}.
    /// @param ctx the parse tree
    fn enter_prequelConstruct<'input: 'arena>(&mut self, _ctx: &PrequelConstructContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#prequelConstruct}.
    /// @param ctx the parse tree
    fn exit_prequelConstruct<'input: 'arena>(&mut self, _ctx: &PrequelConstructContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#optionsSpec}.
    /// @param ctx the parse tree
    fn enter_optionsSpec<'input: 'arena>(&mut self, _ctx: &OptionsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#optionsSpec}.
    /// @param ctx the parse tree
    fn exit_optionsSpec<'input: 'arena>(&mut self, _ctx: &OptionsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#option}.
    /// @param ctx the parse tree
    fn enter_option<'input: 'arena>(&mut self, _ctx: &OptionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#option}.
    /// @param ctx the parse tree
    fn exit_option<'input: 'arena>(&mut self, _ctx: &OptionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#optionValue}.
    /// @param ctx the parse tree
    fn enter_optionValue<'input: 'arena>(&mut self, _ctx: &OptionValueContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#optionValue}.
    /// @param ctx the parse tree
    fn exit_optionValue<'input: 'arena>(&mut self, _ctx: &OptionValueContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#delegateGrammars}.
    /// @param ctx the parse tree
    fn enter_delegateGrammars<'input: 'arena>(&mut self, _ctx: &DelegateGrammarsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#delegateGrammars}.
    /// @param ctx the parse tree
    fn exit_delegateGrammars<'input: 'arena>(&mut self, _ctx: &DelegateGrammarsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#delegateGrammar}.
    /// @param ctx the parse tree
    fn enter_delegateGrammar<'input: 'arena>(&mut self, _ctx: &DelegateGrammarContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#delegateGrammar}.
    /// @param ctx the parse tree
    fn exit_delegateGrammar<'input: 'arena>(&mut self, _ctx: &DelegateGrammarContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#tokensSpec}.
    /// @param ctx the parse tree
    fn enter_tokensSpec<'input: 'arena>(&mut self, _ctx: &TokensSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#tokensSpec}.
    /// @param ctx the parse tree
    fn exit_tokensSpec<'input: 'arena>(&mut self, _ctx: &TokensSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#channelsSpec}.
    /// @param ctx the parse tree
    fn enter_channelsSpec<'input: 'arena>(&mut self, _ctx: &ChannelsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#channelsSpec}.
    /// @param ctx the parse tree
    fn exit_channelsSpec<'input: 'arena>(&mut self, _ctx: &ChannelsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#idList}.
    /// @param ctx the parse tree
    fn enter_idList<'input: 'arena>(&mut self, _ctx: &IdListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#idList}.
    /// @param ctx the parse tree
    fn exit_idList<'input: 'arena>(&mut self, _ctx: &IdListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#action_}.
    /// @param ctx the parse tree
    fn enter_action_<'input: 'arena>(&mut self, _ctx: &Action_Context<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#action_}.
    /// @param ctx the parse tree
    fn exit_action_<'input: 'arena>(&mut self, _ctx: &Action_Context<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#actionScopeName}.
    /// @param ctx the parse tree
    fn enter_actionScopeName<'input: 'arena>(&mut self, _ctx: &ActionScopeNameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#actionScopeName}.
    /// @param ctx the parse tree
    fn exit_actionScopeName<'input: 'arena>(&mut self, _ctx: &ActionScopeNameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#actionBlock}.
    /// @param ctx the parse tree
    fn enter_actionBlock<'input: 'arena>(&mut self, _ctx: &ActionBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#actionBlock}.
    /// @param ctx the parse tree
    fn exit_actionBlock<'input: 'arena>(&mut self, _ctx: &ActionBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#argActionBlock}.
    /// @param ctx the parse tree
    fn enter_argActionBlock<'input: 'arena>(&mut self, _ctx: &ArgActionBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#argActionBlock}.
    /// @param ctx the parse tree
    fn exit_argActionBlock<'input: 'arena>(&mut self, _ctx: &ArgActionBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#modeSpec}.
    /// @param ctx the parse tree
    fn enter_modeSpec<'input: 'arena>(&mut self, _ctx: &ModeSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#modeSpec}.
    /// @param ctx the parse tree
    fn exit_modeSpec<'input: 'arena>(&mut self, _ctx: &ModeSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#rules}.
    /// @param ctx the parse tree
    fn enter_rules<'input: 'arena>(&mut self, _ctx: &RulesContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#rules}.
    /// @param ctx the parse tree
    fn exit_rules<'input: 'arena>(&mut self, _ctx: &RulesContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleSpec}.
    /// @param ctx the parse tree
    fn enter_ruleSpec<'input: 'arena>(&mut self, _ctx: &RuleSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleSpec}.
    /// @param ctx the parse tree
    fn exit_ruleSpec<'input: 'arena>(&mut self, _ctx: &RuleSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#parserRuleSpec}.
    /// @param ctx the parse tree
    fn enter_parserRuleSpec<'input: 'arena>(&mut self, _ctx: &ParserRuleSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#parserRuleSpec}.
    /// @param ctx the parse tree
    fn exit_parserRuleSpec<'input: 'arena>(&mut self, _ctx: &ParserRuleSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#exceptionGroup}.
    /// @param ctx the parse tree
    fn enter_exceptionGroup<'input: 'arena>(&mut self, _ctx: &ExceptionGroupContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#exceptionGroup}.
    /// @param ctx the parse tree
    fn exit_exceptionGroup<'input: 'arena>(&mut self, _ctx: &ExceptionGroupContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#exceptionHandler}.
    /// @param ctx the parse tree
    fn enter_exceptionHandler<'input: 'arena>(&mut self, _ctx: &ExceptionHandlerContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#exceptionHandler}.
    /// @param ctx the parse tree
    fn exit_exceptionHandler<'input: 'arena>(&mut self, _ctx: &ExceptionHandlerContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#finallyClause}.
    /// @param ctx the parse tree
    fn enter_finallyClause<'input: 'arena>(&mut self, _ctx: &FinallyClauseContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#finallyClause}.
    /// @param ctx the parse tree
    fn exit_finallyClause<'input: 'arena>(&mut self, _ctx: &FinallyClauseContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#rulePrequel}.
    /// @param ctx the parse tree
    fn enter_rulePrequel<'input: 'arena>(&mut self, _ctx: &RulePrequelContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#rulePrequel}.
    /// @param ctx the parse tree
    fn exit_rulePrequel<'input: 'arena>(&mut self, _ctx: &RulePrequelContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleReturns}.
    /// @param ctx the parse tree
    fn enter_ruleReturns<'input: 'arena>(&mut self, _ctx: &RuleReturnsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleReturns}.
    /// @param ctx the parse tree
    fn exit_ruleReturns<'input: 'arena>(&mut self, _ctx: &RuleReturnsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#throwsSpec}.
    /// @param ctx the parse tree
    fn enter_throwsSpec<'input: 'arena>(&mut self, _ctx: &ThrowsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#throwsSpec}.
    /// @param ctx the parse tree
    fn exit_throwsSpec<'input: 'arena>(&mut self, _ctx: &ThrowsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#localsSpec}.
    /// @param ctx the parse tree
    fn enter_localsSpec<'input: 'arena>(&mut self, _ctx: &LocalsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#localsSpec}.
    /// @param ctx the parse tree
    fn exit_localsSpec<'input: 'arena>(&mut self, _ctx: &LocalsSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleAction}.
    /// @param ctx the parse tree
    fn enter_ruleAction<'input: 'arena>(&mut self, _ctx: &RuleActionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleAction}.
    /// @param ctx the parse tree
    fn exit_ruleAction<'input: 'arena>(&mut self, _ctx: &RuleActionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleModifiers}.
    /// @param ctx the parse tree
    fn enter_ruleModifiers<'input: 'arena>(&mut self, _ctx: &RuleModifiersContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleModifiers}.
    /// @param ctx the parse tree
    fn exit_ruleModifiers<'input: 'arena>(&mut self, _ctx: &RuleModifiersContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleModifier}.
    /// @param ctx the parse tree
    fn enter_ruleModifier<'input: 'arena>(&mut self, _ctx: &RuleModifierContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleModifier}.
    /// @param ctx the parse tree
    fn exit_ruleModifier<'input: 'arena>(&mut self, _ctx: &RuleModifierContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleBlock}.
    /// @param ctx the parse tree
    fn enter_ruleBlock<'input: 'arena>(&mut self, _ctx: &RuleBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleBlock}.
    /// @param ctx the parse tree
    fn exit_ruleBlock<'input: 'arena>(&mut self, _ctx: &RuleBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleAltList}.
    /// @param ctx the parse tree
    fn enter_ruleAltList<'input: 'arena>(&mut self, _ctx: &RuleAltListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleAltList}.
    /// @param ctx the parse tree
    fn exit_ruleAltList<'input: 'arena>(&mut self, _ctx: &RuleAltListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#labeledAlt}.
    /// @param ctx the parse tree
    fn enter_labeledAlt<'input: 'arena>(&mut self, _ctx: &LabeledAltContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#labeledAlt}.
    /// @param ctx the parse tree
    fn exit_labeledAlt<'input: 'arena>(&mut self, _ctx: &LabeledAltContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerRuleSpec}.
    /// @param ctx the parse tree
    fn enter_lexerRuleSpec<'input: 'arena>(&mut self, _ctx: &LexerRuleSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerRuleSpec}.
    /// @param ctx the parse tree
    fn exit_lexerRuleSpec<'input: 'arena>(&mut self, _ctx: &LexerRuleSpecContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerRuleBlock}.
    /// @param ctx the parse tree
    fn enter_lexerRuleBlock<'input: 'arena>(&mut self, _ctx: &LexerRuleBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerRuleBlock}.
    /// @param ctx the parse tree
    fn exit_lexerRuleBlock<'input: 'arena>(&mut self, _ctx: &LexerRuleBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerAltList}.
    /// @param ctx the parse tree
    fn enter_lexerAltList<'input: 'arena>(&mut self, _ctx: &LexerAltListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerAltList}.
    /// @param ctx the parse tree
    fn exit_lexerAltList<'input: 'arena>(&mut self, _ctx: &LexerAltListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerAlt}.
    /// @param ctx the parse tree
    fn enter_lexerAlt<'input: 'arena>(&mut self, _ctx: &LexerAltContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerAlt}.
    /// @param ctx the parse tree
    fn exit_lexerAlt<'input: 'arena>(&mut self, _ctx: &LexerAltContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerElements}.
    /// @param ctx the parse tree
    fn enter_lexerElements<'input: 'arena>(&mut self, _ctx: &LexerElementsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerElements}.
    /// @param ctx the parse tree
    fn exit_lexerElements<'input: 'arena>(&mut self, _ctx: &LexerElementsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerElement}.
    /// @param ctx the parse tree
    fn enter_lexerElement<'input: 'arena>(&mut self, _ctx: &LexerElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerElement}.
    /// @param ctx the parse tree
    fn exit_lexerElement<'input: 'arena>(&mut self, _ctx: &LexerElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerBlock}.
    /// @param ctx the parse tree
    fn enter_lexerBlock<'input: 'arena>(&mut self, _ctx: &LexerBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerBlock}.
    /// @param ctx the parse tree
    fn exit_lexerBlock<'input: 'arena>(&mut self, _ctx: &LexerBlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerCommands}.
    /// @param ctx the parse tree
    fn enter_lexerCommands<'input: 'arena>(&mut self, _ctx: &LexerCommandsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerCommands}.
    /// @param ctx the parse tree
    fn exit_lexerCommands<'input: 'arena>(&mut self, _ctx: &LexerCommandsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerCommand}.
    /// @param ctx the parse tree
    fn enter_lexerCommand<'input: 'arena>(&mut self, _ctx: &LexerCommandContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerCommand}.
    /// @param ctx the parse tree
    fn exit_lexerCommand<'input: 'arena>(&mut self, _ctx: &LexerCommandContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerCommandName}.
    /// @param ctx the parse tree
    fn enter_lexerCommandName<'input: 'arena>(&mut self, _ctx: &LexerCommandNameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerCommandName}.
    /// @param ctx the parse tree
    fn exit_lexerCommandName<'input: 'arena>(&mut self, _ctx: &LexerCommandNameContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerCommandExpr}.
    /// @param ctx the parse tree
    fn enter_lexerCommandExpr<'input: 'arena>(&mut self, _ctx: &LexerCommandExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerCommandExpr}.
    /// @param ctx the parse tree
    fn exit_lexerCommandExpr<'input: 'arena>(&mut self, _ctx: &LexerCommandExprContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#altList}.
    /// @param ctx the parse tree
    fn enter_altList<'input: 'arena>(&mut self, _ctx: &AltListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#altList}.
    /// @param ctx the parse tree
    fn exit_altList<'input: 'arena>(&mut self, _ctx: &AltListContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#alternative}.
    /// @param ctx the parse tree
    fn enter_alternative<'input: 'arena>(&mut self, _ctx: &AlternativeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#alternative}.
    /// @param ctx the parse tree
    fn exit_alternative<'input: 'arena>(&mut self, _ctx: &AlternativeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#element}.
    /// @param ctx the parse tree
    fn enter_element<'input: 'arena>(&mut self, _ctx: &ElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#element}.
    /// @param ctx the parse tree
    fn exit_element<'input: 'arena>(&mut self, _ctx: &ElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#predicateOptions}.
    /// @param ctx the parse tree
    fn enter_predicateOptions<'input: 'arena>(&mut self, _ctx: &PredicateOptionsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#predicateOptions}.
    /// @param ctx the parse tree
    fn exit_predicateOptions<'input: 'arena>(&mut self, _ctx: &PredicateOptionsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#predicateOption}.
    /// @param ctx the parse tree
    fn enter_predicateOption<'input: 'arena>(&mut self, _ctx: &PredicateOptionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#predicateOption}.
    /// @param ctx the parse tree
    fn exit_predicateOption<'input: 'arena>(&mut self, _ctx: &PredicateOptionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#labeledElement}.
    /// @param ctx the parse tree
    fn enter_labeledElement<'input: 'arena>(&mut self, _ctx: &LabeledElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#labeledElement}.
    /// @param ctx the parse tree
    fn exit_labeledElement<'input: 'arena>(&mut self, _ctx: &LabeledElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ebnf}.
    /// @param ctx the parse tree
    fn enter_ebnf<'input: 'arena>(&mut self, _ctx: &EbnfContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ebnf}.
    /// @param ctx the parse tree
    fn exit_ebnf<'input: 'arena>(&mut self, _ctx: &EbnfContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#blockSuffix}.
    /// @param ctx the parse tree
    fn enter_blockSuffix<'input: 'arena>(&mut self, _ctx: &BlockSuffixContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#blockSuffix}.
    /// @param ctx the parse tree
    fn exit_blockSuffix<'input: 'arena>(&mut self, _ctx: &BlockSuffixContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ebnfSuffix}.
    /// @param ctx the parse tree
    fn enter_ebnfSuffix<'input: 'arena>(&mut self, _ctx: &EbnfSuffixContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ebnfSuffix}.
    /// @param ctx the parse tree
    fn exit_ebnfSuffix<'input: 'arena>(&mut self, _ctx: &EbnfSuffixContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#lexerAtom}.
    /// @param ctx the parse tree
    fn enter_lexerAtom<'input: 'arena>(&mut self, _ctx: &LexerAtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#lexerAtom}.
    /// @param ctx the parse tree
    fn exit_lexerAtom<'input: 'arena>(&mut self, _ctx: &LexerAtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#atom}.
    /// @param ctx the parse tree
    fn enter_atom<'input: 'arena>(&mut self, _ctx: &AtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#atom}.
    /// @param ctx the parse tree
    fn exit_atom<'input: 'arena>(&mut self, _ctx: &AtomContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#wildcard}.
    /// @param ctx the parse tree
    fn enter_wildcard<'input: 'arena>(&mut self, _ctx: &WildcardContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#wildcard}.
    /// @param ctx the parse tree
    fn exit_wildcard<'input: 'arena>(&mut self, _ctx: &WildcardContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#notSet}.
    /// @param ctx the parse tree
    fn enter_notSet<'input: 'arena>(&mut self, _ctx: &NotSetContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#notSet}.
    /// @param ctx the parse tree
    fn exit_notSet<'input: 'arena>(&mut self, _ctx: &NotSetContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#blockSet}.
    /// @param ctx the parse tree
    fn enter_blockSet<'input: 'arena>(&mut self, _ctx: &BlockSetContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#blockSet}.
    /// @param ctx the parse tree
    fn exit_blockSet<'input: 'arena>(&mut self, _ctx: &BlockSetContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#setElement}.
    /// @param ctx the parse tree
    fn enter_setElement<'input: 'arena>(&mut self, _ctx: &SetElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#setElement}.
    /// @param ctx the parse tree
    fn exit_setElement<'input: 'arena>(&mut self, _ctx: &SetElementContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#block}.
    /// @param ctx the parse tree
    fn enter_block<'input: 'arena>(&mut self, _ctx: &BlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#block}.
    /// @param ctx the parse tree
    fn exit_block<'input: 'arena>(&mut self, _ctx: &BlockContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#ruleref}.
    /// @param ctx the parse tree
    fn enter_ruleref<'input: 'arena>(&mut self, _ctx: &RulerefContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#ruleref}.
    /// @param ctx the parse tree
    fn exit_ruleref<'input: 'arena>(&mut self, _ctx: &RulerefContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#characterRange}.
    /// @param ctx the parse tree
    fn enter_characterRange<'input: 'arena>(&mut self, _ctx: &CharacterRangeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#characterRange}.
    /// @param ctx the parse tree
    fn exit_characterRange<'input: 'arena>(&mut self, _ctx: &CharacterRangeContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#terminalDef}.
    /// @param ctx the parse tree
    fn enter_terminalDef<'input: 'arena>(&mut self, _ctx: &TerminalDefContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#terminalDef}.
    /// @param ctx the parse tree
    fn exit_terminalDef<'input: 'arena>(&mut self, _ctx: &TerminalDefContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#elementOptions}.
    /// @param ctx the parse tree
    fn enter_elementOptions<'input: 'arena>(&mut self, _ctx: &ElementOptionsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#elementOptions}.
    /// @param ctx the parse tree
    fn exit_elementOptions<'input: 'arena>(&mut self, _ctx: &ElementOptionsContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#elementOption}.
    /// @param ctx the parse tree
    fn enter_elementOption<'input: 'arena>(&mut self, _ctx: &ElementOptionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#elementOption}.
    /// @param ctx the parse tree
    fn exit_elementOption<'input: 'arena>(&mut self, _ctx: &ElementOptionContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#identifier}.
    /// @param ctx the parse tree
    fn enter_identifier<'input: 'arena>(&mut self, _ctx: &IdentifierContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#identifier}.
    /// @param ctx the parse tree
    fn exit_identifier<'input: 'arena>(&mut self, _ctx: &IdentifierContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Enter a parse tree produced by {@link ANTLRv4Parser#qualifiedIdentifier}.
    /// @param ctx the parse tree
    fn enter_qualifiedIdentifier<'input: 'arena>(&mut self, _ctx: &QualifiedIdentifierContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

    /// Exit a parse tree produced by {@link ANTLRv4Parser#qualifiedIdentifier}.
    /// @param ctx the parse tree
    fn exit_qualifiedIdentifier<'input: 'arena>(&mut self, _ctx: &QualifiedIdentifierContext<'input, 'arena, Tok>) -> Result<(), ANTLRError> { Ok(()) }

}
