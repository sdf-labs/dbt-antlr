// Generated from ANTLRv4Parser.g4 by ANTLR 4.13.2
use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::PredictionContextCache;
use dbt_antlr_runtime::parser::{Parser, BaseParser, ParserRecog, ListenerId};
use dbt_antlr_runtime::token::CommonToken;
use dbt_antlr_runtime::token_stream::TokenStream;
use dbt_antlr_runtime::TokenSource;
use dbt_antlr_runtime::parser_atn_simulator::ParserATNSimulator;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::rule_context::{CustomRuleContext, RuleContext};
use dbt_antlr_runtime::recognizer::{Recognizer,Actions};
use dbt_antlr_runtime::atn_config_set::ATNConfigSet;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
use dbt_antlr_runtime::atn_simulator::BaseATNSimulator;
use dbt_antlr_runtime::atn_simulator::ParserATNSimulatorManager as ATNSimulatorManager;
use dbt_antlr_runtime::atn::{ATN, INVALID_ALT};
use dbt_antlr_runtime::error_strategy::{DefaultErrorStrategy, ErrorStrategyDelegate, ErrorStrategy};
use dbt_antlr_runtime::parser_rule_context::{BaseParserRuleContext, ParserRuleContext};
use dbt_antlr_runtime::tree::*;
use dbt_antlr_runtime::token::{TOKEN_EOF,Token};
use dbt_antlr_runtime::int_stream::EOF;
use dbt_antlr_runtime::vocabulary::{Vocabulary,VocabularyImpl};
use dbt_antlr_runtime::token_factory::TokenFactory;
use super::antlrv4parserlistener::*;
use std::marker::PhantomData;
use std::sync::LazyLock;
use std::rc::Rc;
use std::ops::{DerefMut, Deref};

dbt_antlr_runtime::check_version!("0","1");
pub const ANTLRv4Parser_TOKEN_REF:i32=1; 
pub const ANTLRv4Parser_RULE_REF:i32=2; 
pub const ANTLRv4Parser_LEXER_CHAR_SET:i32=3; 
pub const ANTLRv4Parser_DOC_COMMENT:i32=4; 
pub const ANTLRv4Parser_BLOCK_COMMENT:i32=5; 
pub const ANTLRv4Parser_LINE_COMMENT:i32=6; 
pub const ANTLRv4Parser_INT:i32=7; 
pub const ANTLRv4Parser_STRING_LITERAL:i32=8; 
pub const ANTLRv4Parser_UNTERMINATED_STRING_LITERAL:i32=9; 
pub const ANTLRv4Parser_BEGIN_ARGUMENT:i32=10; 
pub const ANTLRv4Parser_ACTION:i32=11; 
pub const ANTLRv4Parser_OPTIONS:i32=12; 
pub const ANTLRv4Parser_TOKENS:i32=13; 
pub const ANTLRv4Parser_CHANNELS:i32=14; 
pub const ANTLRv4Parser_IMPORT:i32=15; 
pub const ANTLRv4Parser_FRAGMENT:i32=16; 
pub const ANTLRv4Parser_LEXER:i32=17; 
pub const ANTLRv4Parser_PARSER:i32=18; 
pub const ANTLRv4Parser_GRAMMAR:i32=19; 
pub const ANTLRv4Parser_PROTECTED:i32=20; 
pub const ANTLRv4Parser_PUBLIC:i32=21; 
pub const ANTLRv4Parser_PRIVATE:i32=22; 
pub const ANTLRv4Parser_RETURNS:i32=23; 
pub const ANTLRv4Parser_LOCALS:i32=24; 
pub const ANTLRv4Parser_THROWS:i32=25; 
pub const ANTLRv4Parser_CATCH:i32=26; 
pub const ANTLRv4Parser_FINALLY:i32=27; 
pub const ANTLRv4Parser_MODE:i32=28; 
pub const ANTLRv4Parser_COLON:i32=29; 
pub const ANTLRv4Parser_COLONCOLON:i32=30; 
pub const ANTLRv4Parser_COMMA:i32=31; 
pub const ANTLRv4Parser_SEMI:i32=32; 
pub const ANTLRv4Parser_LPAREN:i32=33; 
pub const ANTLRv4Parser_RPAREN:i32=34; 
pub const ANTLRv4Parser_RBRACE:i32=35; 
pub const ANTLRv4Parser_RARROW:i32=36; 
pub const ANTLRv4Parser_LT:i32=37; 
pub const ANTLRv4Parser_GT:i32=38; 
pub const ANTLRv4Parser_ASSIGN:i32=39; 
pub const ANTLRv4Parser_QUESTION:i32=40; 
pub const ANTLRv4Parser_STAR:i32=41; 
pub const ANTLRv4Parser_PLUS_ASSIGN:i32=42; 
pub const ANTLRv4Parser_PLUS:i32=43; 
pub const ANTLRv4Parser_OR:i32=44; 
pub const ANTLRv4Parser_DOLLAR:i32=45; 
pub const ANTLRv4Parser_RANGE:i32=46; 
pub const ANTLRv4Parser_DOT:i32=47; 
pub const ANTLRv4Parser_AT:i32=48; 
pub const ANTLRv4Parser_POUND:i32=49; 
pub const ANTLRv4Parser_NOT:i32=50; 
pub const ANTLRv4Parser_ID:i32=51; 
pub const ANTLRv4Parser_WS:i32=52; 
pub const ANTLRv4Parser_END_ARGUMENT:i32=53; 
pub const ANTLRv4Parser_UNTERMINATED_ARGUMENT:i32=54; 
pub const ANTLRv4Parser_ARGUMENT_CONTENT:i32=55; 
pub const ANTLRv4Parser_UNTERMINATED_CHAR_SET:i32=56;
pub const ANTLRv4Parser_EOF:i32=EOF;
pub const RULE_grammarSpec:usize = 0; 
pub const RULE_grammarDecl:usize = 1; 
pub const RULE_grammarType:usize = 2; 
pub const RULE_prequelConstruct:usize = 3; 
pub const RULE_optionsSpec:usize = 4; 
pub const RULE_option:usize = 5; 
pub const RULE_optionValue:usize = 6; 
pub const RULE_delegateGrammars:usize = 7; 
pub const RULE_delegateGrammar:usize = 8; 
pub const RULE_tokensSpec:usize = 9; 
pub const RULE_channelsSpec:usize = 10; 
pub const RULE_idList:usize = 11; 
pub const RULE_action_:usize = 12; 
pub const RULE_actionScopeName:usize = 13; 
pub const RULE_actionBlock:usize = 14; 
pub const RULE_argActionBlock:usize = 15; 
pub const RULE_modeSpec:usize = 16; 
pub const RULE_rules:usize = 17; 
pub const RULE_ruleSpec:usize = 18; 
pub const RULE_parserRuleSpec:usize = 19; 
pub const RULE_exceptionGroup:usize = 20; 
pub const RULE_exceptionHandler:usize = 21; 
pub const RULE_finallyClause:usize = 22; 
pub const RULE_rulePrequel:usize = 23; 
pub const RULE_ruleReturns:usize = 24; 
pub const RULE_throwsSpec:usize = 25; 
pub const RULE_localsSpec:usize = 26; 
pub const RULE_ruleAction:usize = 27; 
pub const RULE_ruleModifiers:usize = 28; 
pub const RULE_ruleModifier:usize = 29; 
pub const RULE_ruleBlock:usize = 30; 
pub const RULE_ruleAltList:usize = 31; 
pub const RULE_labeledAlt:usize = 32; 
pub const RULE_lexerRuleSpec:usize = 33; 
pub const RULE_lexerRuleBlock:usize = 34; 
pub const RULE_lexerAltList:usize = 35; 
pub const RULE_lexerAlt:usize = 36; 
pub const RULE_lexerElements:usize = 37; 
pub const RULE_lexerElement:usize = 38; 
pub const RULE_lexerBlock:usize = 39; 
pub const RULE_lexerCommands:usize = 40; 
pub const RULE_lexerCommand:usize = 41; 
pub const RULE_lexerCommandName:usize = 42; 
pub const RULE_lexerCommandExpr:usize = 43; 
pub const RULE_altList:usize = 44; 
pub const RULE_alternative:usize = 45; 
pub const RULE_element:usize = 46; 
pub const RULE_predicateOptions:usize = 47; 
pub const RULE_predicateOption:usize = 48; 
pub const RULE_labeledElement:usize = 49; 
pub const RULE_ebnf:usize = 50; 
pub const RULE_blockSuffix:usize = 51; 
pub const RULE_ebnfSuffix:usize = 52; 
pub const RULE_lexerAtom:usize = 53; 
pub const RULE_atom:usize = 54; 
pub const RULE_wildcard:usize = 55; 
pub const RULE_notSet:usize = 56; 
pub const RULE_blockSet:usize = 57; 
pub const RULE_setElement:usize = 58; 
pub const RULE_block:usize = 59; 
pub const RULE_ruleref:usize = 60; 
pub const RULE_characterRange:usize = 61; 
pub const RULE_terminalDef:usize = 62; 
pub const RULE_elementOptions:usize = 63; 
pub const RULE_elementOption:usize = 64; 
pub const RULE_identifier:usize = 65; 
pub const RULE_qualifiedIdentifier:usize = 66;
pub const ruleNames: [&'static str; 67] = [
    "grammarSpec", "grammarDecl", "grammarType", "prequelConstruct", "optionsSpec", 
    "option", "optionValue", "delegateGrammars", "delegateGrammar", "tokensSpec", 
    "channelsSpec", "idList", "action_", "actionScopeName", "actionBlock", 
    "argActionBlock", "modeSpec", "rules", "ruleSpec", "parserRuleSpec", 
    "exceptionGroup", "exceptionHandler", "finallyClause", "rulePrequel", 
    "ruleReturns", "throwsSpec", "localsSpec", "ruleAction", "ruleModifiers", 
    "ruleModifier", "ruleBlock", "ruleAltList", "labeledAlt", "lexerRuleSpec", 
    "lexerRuleBlock", "lexerAltList", "lexerAlt", "lexerElements", "lexerElement", 
    "lexerBlock", "lexerCommands", "lexerCommand", "lexerCommandName", "lexerCommandExpr", 
    "altList", "alternative", "element", "predicateOptions", "predicateOption", 
    "labeledElement", "ebnf", "blockSuffix", "ebnfSuffix", "lexerAtom", 
    "atom", "wildcard", "notSet", "blockSet", "setElement", "block", "ruleref", 
    "characterRange", "terminalDef", "elementOptions", "elementOption", 
    "identifier", "qualifiedIdentifier"
];

pub const _LITERAL_NAMES: [Option<&'static str>;51] = [
	None, None, None, None, None, None, None, None, None, None, Some("'['"), 
	None, None, None, None, Some("'import'"), Some("'fragment'"), Some("'lexer'"), 
	Some("'parser'"), Some("'grammar'"), Some("'protected'"), Some("'public'"), 
	Some("'private'"), Some("'returns'"), Some("'locals'"), Some("'throws'"), 
	Some("'catch'"), Some("'finally'"), Some("'mode'"), Some("':'"), Some("'::'"), 
	Some("','"), Some("';'"), Some("'('"), Some("')'"), Some("'}'"), Some("'->'"), 
	Some("'<'"), Some("'>'"), Some("'='"), Some("'?'"), Some("'*'"), Some("'+='"), 
	Some("'+'"), Some("'|'"), Some("'$'"), Some("'..'"), Some("'.'"), Some("'@'"), 
	Some("'#'"), Some("'~'")
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>;57]  = [
	None, Some("TOKEN_REF"), Some("RULE_REF"), Some("LEXER_CHAR_SET"), Some("DOC_COMMENT"), 
	Some("BLOCK_COMMENT"), Some("LINE_COMMENT"), Some("INT"), Some("STRING_LITERAL"), 
	Some("UNTERMINATED_STRING_LITERAL"), Some("BEGIN_ARGUMENT"), Some("ACTION"), 
	Some("OPTIONS"), Some("TOKENS"), Some("CHANNELS"), Some("IMPORT"), Some("FRAGMENT"), 
	Some("LEXER"), Some("PARSER"), Some("GRAMMAR"), Some("PROTECTED"), Some("PUBLIC"), 
	Some("PRIVATE"), Some("RETURNS"), Some("LOCALS"), Some("THROWS"), Some("CATCH"), 
	Some("FINALLY"), Some("MODE"), Some("COLON"), Some("COLONCOLON"), Some("COMMA"), 
	Some("SEMI"), Some("LPAREN"), Some("RPAREN"), Some("RBRACE"), Some("RARROW"), 
	Some("LT"), Some("GT"), Some("ASSIGN"), Some("QUESTION"), Some("STAR"), 
	Some("PLUS_ASSIGN"), Some("PLUS"), Some("OR"), Some("DOLLAR"), Some("RANGE"), 
	Some("DOT"), Some("AT"), Some("POUND"), Some("NOT"), Some("ID"), Some("WS"), 
	Some("END_ARGUMENT"), Some("UNTERMINATED_ARGUMENT"), Some("ARGUMENT_CONTENT"), 
	Some("UNTERMINATED_CHAR_SET")
];

static VOCABULARY: LazyLock<Box<dyn Vocabulary>> = LazyLock::new(|| Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None)));

pub type BaseParserType<'input, 'arena, Input, TF> = BaseParser<'input, 'arena, ANTLRv4ParserExt<'input, 'arena>, ANTLRv4ParserNodeKind, Input, TF>;
pub fn parser_simulator_manager() -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }

pub struct ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	base: BaseParserType<'input, 'arena, Input, TF>,
    err_handler: ErrorStrategyDelegate<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>>,
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
    pub fn with_strategy(arena: &'arena Arena, input: Input, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) -> Self {
		Self {
			base: BaseParser::new_base_parser(
				arena, input,
				ANTLRv4ParserExt {
					_pd: Default::default(),
				}
			),
            err_handler: unsafe { ErrorStrategyDelegate::new(strategy) },
        }
    }

    pub fn new(arena: &'arena Arena, input: Input) -> Self{
    	Self::with_strategy(arena, input, Box::new(DefaultErrorStrategy::new()))
    }

    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input, 'arena, TF, BaseParserType<'input, 'arena, Input, TF>> + 'arena>) {
        self.err_handler = unsafe { ErrorStrategyDelegate::new(strategy) };
    }

    /// Adds parse listener for this parser
    /// returns `listener_id` that can be used later to get listener back
    ///
    /// ### Example for listener usage:
    /// todo
    pub fn add_parse_listener<L>(
        &mut self,
        listener: Box<L>,
    ) -> ListenerId<L>
    where
        L: ANTLRv4ParserListener<'arena, TF::Tok> + 'static,
    {
        let id = ListenerId::new(&listener);
        self.base.add_dyn_parse_listener(listener);
        id
    }
}
pub struct ANTLRv4ParserTreeWalker;
impl ANTLRv4ParserTreeWalker
{
    pub fn walk<'input, 'arena, Tok, L, T>(
        listener: Box<L>,
        tree: &'arena T,
    ) -> Result<Box<L>, ANTLRError>
    where
        'input: 'arena,
        L: ANTLRv4ParserListener<'arena, Tok> + 'static,
        T: NodeInner<'input, 'arena, ANTLRv4ParserNodeKind, Tok>,
        Tok: Token + 'input,
    {
        let listener_ptr = Box::into_raw(listener);
        let listener = unsafe { Box::from_raw(listener_ptr as *mut <ANTLRv4ParserNodeKind as NodeKindType<Tok>>::Listener) };
        let listener = ParseTreeWalker::walk(listener, tree.as_node())?;
        Ok(unsafe { Box::from_raw(Box::into_raw(listener) as *mut L) } )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum ANTLRv4ParserNodeKind {
    GrammarSpecContext,
    GrammarDeclContext,
    GrammarTypeContext,
    PrequelConstructContext,
    OptionsSpecContext,
    OptionContext,
    OptionValueContext,
    DelegateGrammarsContext,
    DelegateGrammarContext,
    TokensSpecContext,
    ChannelsSpecContext,
    IdListContext,
    Action_Context,
    ActionScopeNameContext,
    ActionBlockContext,
    ArgActionBlockContext,
    ModeSpecContext,
    RulesContext,
    RuleSpecContext,
    ParserRuleSpecContext,
    ExceptionGroupContext,
    ExceptionHandlerContext,
    FinallyClauseContext,
    RulePrequelContext,
    RuleReturnsContext,
    ThrowsSpecContext,
    LocalsSpecContext,
    RuleActionContext,
    RuleModifiersContext,
    RuleModifierContext,
    RuleBlockContext,
    RuleAltListContext,
    LabeledAltContext,
    LexerRuleSpecContext,
    LexerRuleBlockContext,
    LexerAltListContext,
    LexerAltContext,
    LexerElementsContext,
    LexerElementContext,
    LexerBlockContext,
    LexerCommandsContext,
    LexerCommandContext,
    LexerCommandNameContext,
    LexerCommandExprContext,
    AltListContext,
    AlternativeContext,
    ElementContext,
    PredicateOptionsContext,
    PredicateOptionContext,
    LabeledElementContext,
    EbnfContext,
    BlockSuffixContext,
    EbnfSuffixContext,
    LexerAtomContext,
    AtomContext,
    WildcardContext,
    NotSetContext,
    BlockSetContext,
    SetElementContext,
    BlockContext,
    RulerefContext,
    CharacterRangeContext,
    TerminalDefContext,
    ElementOptionsContext,
    ElementOptionContext,
    IdentifierContext,
    QualifiedIdentifierContext,
    Terminal,
    Error,
}
pub type ANTLRv4ParserNode<'input, 'arena, Tok = CommonToken<'input>> = TreeNode<'input, 'arena, ANTLRv4ParserNodeKind, Tok>;

dbt_antlr_runtime::impl_deref! { parser => ANTLRv4Parser }
dbt_antlr_runtime::impl_node_kind! { ANTLRv4ParserNodeKind {
; GrammarSpecContext(enter_grammarSpec, exit_grammarSpec, ), GrammarDeclContext(enter_grammarDecl, exit_grammarDecl, ), GrammarTypeContext(enter_grammarType, exit_grammarType, ), PrequelConstructContext(enter_prequelConstruct, exit_prequelConstruct, ), OptionsSpecContext(enter_optionsSpec, exit_optionsSpec, ), OptionContext(enter_option, exit_option, ), OptionValueContext(enter_optionValue, exit_optionValue, ), DelegateGrammarsContext(enter_delegateGrammars, exit_delegateGrammars, ), DelegateGrammarContext(enter_delegateGrammar, exit_delegateGrammar, ), TokensSpecContext(enter_tokensSpec, exit_tokensSpec, ), ChannelsSpecContext(enter_channelsSpec, exit_channelsSpec, ), IdListContext(enter_idList, exit_idList, ), Action_Context(enter_action_, exit_action_, ), ActionScopeNameContext(enter_actionScopeName, exit_actionScopeName, ), ActionBlockContext(enter_actionBlock, exit_actionBlock, ), ArgActionBlockContext(enter_argActionBlock, exit_argActionBlock, ), ModeSpecContext(enter_modeSpec, exit_modeSpec, ), RulesContext(enter_rules, exit_rules, ), RuleSpecContext(enter_ruleSpec, exit_ruleSpec, ), ParserRuleSpecContext(enter_parserRuleSpec, exit_parserRuleSpec, ), ExceptionGroupContext(enter_exceptionGroup, exit_exceptionGroup, ), ExceptionHandlerContext(enter_exceptionHandler, exit_exceptionHandler, ), FinallyClauseContext(enter_finallyClause, exit_finallyClause, ), RulePrequelContext(enter_rulePrequel, exit_rulePrequel, ), RuleReturnsContext(enter_ruleReturns, exit_ruleReturns, ), ThrowsSpecContext(enter_throwsSpec, exit_throwsSpec, ), LocalsSpecContext(enter_localsSpec, exit_localsSpec, ), RuleActionContext(enter_ruleAction, exit_ruleAction, ), RuleModifiersContext(enter_ruleModifiers, exit_ruleModifiers, ), RuleModifierContext(enter_ruleModifier, exit_ruleModifier, ), RuleBlockContext(enter_ruleBlock, exit_ruleBlock, ), RuleAltListContext(enter_ruleAltList, exit_ruleAltList, ), LabeledAltContext(enter_labeledAlt, exit_labeledAlt, ), LexerRuleSpecContext(enter_lexerRuleSpec, exit_lexerRuleSpec, ), LexerRuleBlockContext(enter_lexerRuleBlock, exit_lexerRuleBlock, ), LexerAltListContext(enter_lexerAltList, exit_lexerAltList, ), LexerAltContext(enter_lexerAlt, exit_lexerAlt, ), LexerElementsContext(enter_lexerElements, exit_lexerElements, ), LexerElementContext(enter_lexerElement, exit_lexerElement, ), LexerBlockContext(enter_lexerBlock, exit_lexerBlock, ), LexerCommandsContext(enter_lexerCommands, exit_lexerCommands, ), LexerCommandContext(enter_lexerCommand, exit_lexerCommand, ), LexerCommandNameContext(enter_lexerCommandName, exit_lexerCommandName, ), LexerCommandExprContext(enter_lexerCommandExpr, exit_lexerCommandExpr, ), AltListContext(enter_altList, exit_altList, ), AlternativeContext(enter_alternative, exit_alternative, ), ElementContext(enter_element, exit_element, ), PredicateOptionsContext(enter_predicateOptions, exit_predicateOptions, ), PredicateOptionContext(enter_predicateOption, exit_predicateOption, ), LabeledElementContext(enter_labeledElement, exit_labeledElement, ), EbnfContext(enter_ebnf, exit_ebnf, ), BlockSuffixContext(enter_blockSuffix, exit_blockSuffix, ), EbnfSuffixContext(enter_ebnfSuffix, exit_ebnfSuffix, ), LexerAtomContext(enter_lexerAtom, exit_lexerAtom, ), AtomContext(enter_atom, exit_atom, ), WildcardContext(enter_wildcard, exit_wildcard, ), NotSetContext(enter_notSet, exit_notSet, ), BlockSetContext(enter_blockSet, exit_blockSet, ), SetElementContext(enter_setElement, exit_setElement, ), BlockContext(enter_block, exit_block, ), RulerefContext(enter_ruleref, exit_ruleref, ), CharacterRangeContext(enter_characterRange, exit_characterRange, ), TerminalDefContext(enter_terminalDef, exit_terminalDef, ), ElementOptionsContext(enter_elementOptions, exit_elementOptions, ), ElementOptionContext(enter_elementOption, exit_elementOption, ), IdentifierContext(enter_identifier, exit_identifier, ), QualifiedIdentifierContext(enter_qualifiedIdentifier, exit_qualifiedIdentifier, ), 
    }; listener = dyn ANTLRv4ParserListener<'arena, Tok>,
}

pub struct ANTLRv4ParserExt<'input, 'arena> {
	_pd: PhantomData<(&'input str, &'arena ())>,
}

impl<'input, 'arena> ANTLRv4ParserExt<'input, 'arena> {
}

impl<'input, 'arena, Input, TF> ParserRecog<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for ANTLRv4ParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena {
    fn get_atn_simulator_man(&self) -> &'static ATNSimulatorManager { &ATN_SIMULATOR_MANAGER }        
}

impl<'input, 'arena, Input, TF> Actions<'input, 'arena, BaseParserType<'input, 'arena, Input, TF>, TF::Tok> for ANTLRv4ParserExt<'input, 'arena>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	fn get_grammar_file_name(&self) -> & str{ "ANTLRv4Parser.g4" }
   	fn get_rule_names(&self) -> &[& str] { &ruleNames }
   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- grammarSpec ----------------
pub type GrammarSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = GrammarSpecContext<'input, 'arena, Tok>;

pub type GrammarSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, GrammarSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct GrammarSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for GrammarSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::GrammarSpecContext }
	fn get_rule_index(&self) -> usize { RULE_grammarSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: GrammarSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a GrammarSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => GrammarSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut GrammarSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut GrammarSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> GrammarSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, GrammarSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait GrammarSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn grammarDecl(&self) -> Option<&'arena GrammarDeclContextAll<'input, 'arena, Tok>>;
    fn rules(&self) -> Option<&'arena RulesContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn prequelConstruct_all(&self) -> Vec<&'arena PrequelConstructContextAll<'input, 'arena, Tok>>;
    fn prequelConstruct(&self, i: usize) -> Option<&'arena PrequelConstructContextAll<'input, 'arena, Tok>>;
    fn modeSpec_all(&self) -> Vec<&'arena ModeSpecContextAll<'input, 'arena, Tok>>;
    fn modeSpec(&self, i: usize) -> Option<&'arena ModeSpecContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> GrammarSpecContextAttrs<'input, 'arena, Tok> for GrammarSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn grammarDecl(&self) -> Option<&'arena GrammarDeclContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn rules(&self) -> Option<&'arena RulesContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EOF
    /// Returns `None` if there is no child corresponding to token EOF
    fn EOF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_EOF)
    }
    fn prequelConstruct_all(&self) -> Vec<&'arena PrequelConstructContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn prequelConstruct(&self, i: usize) -> Option<&'arena PrequelConstructContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    fn modeSpec_all(&self) -> Vec<&'arena ModeSpecContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn modeSpec(&self, i: usize) -> Option<&'arena ModeSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn grammarSpec(&mut self,) -> Result<&'arena GrammarSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(GrammarSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 0, RULE_grammarSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena GrammarSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule grammarDecl*/
			recog.base.set_state(134);
			recog.grammarDecl()?;
			recog.base.set_state(138);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 61440) != 0) || _la==ANTLRv4Parser_AT {
				{
				{
				/*InvokeRule prequelConstruct*/
				recog.base.set_state(135);
				recog.prequelConstruct()?;
				}
				}
				recog.base.set_state(140);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			/*InvokeRule rules*/
			recog.base.set_state(141);
			recog.rules()?;
			recog.base.set_state(145);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_MODE {
				{
				{
				/*InvokeRule modeSpec*/
				recog.base.set_state(142);
				recog.modeSpec()?;
				}
				}
				recog.base.set_state(147);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(148);
			recog.base.match_token(ANTLRv4Parser_EOF,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- grammarDecl ----------------
pub type GrammarDeclContextAll<'input, 'arena, Tok = CommonToken<'input>> = GrammarDeclContext<'input, 'arena, Tok>;

pub type GrammarDeclContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, GrammarDeclContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct GrammarDeclContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for GrammarDeclContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::GrammarDeclContext }
	fn get_rule_index(&self) -> usize { RULE_grammarDecl }
    fn make_node(
        arena: &'arena Arena,
        ctx: GrammarDeclContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a GrammarDeclContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => GrammarDeclContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut GrammarDeclContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut GrammarDeclContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> GrammarDeclContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, GrammarDeclContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait GrammarDeclContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn grammarType(&self) -> Option<&'arena GrammarTypeContextAll<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> GrammarDeclContextAttrs<'input, 'arena, Tok> for GrammarDeclContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn grammarType(&self) -> Option<&'arena GrammarTypeContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn grammarDecl(&mut self,) -> Result<&'arena GrammarDeclContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(GrammarDeclContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 2, RULE_grammarDecl)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena GrammarDeclContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule grammarType*/
			recog.base.set_state(150);
			recog.grammarType()?;
			/*InvokeRule identifier*/
			recog.base.set_state(151);
			recog.identifier()?;
			recog.base.set_state(152);
			recog.base.match_token(ANTLRv4Parser_SEMI,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- grammarType ----------------
pub type GrammarTypeContextAll<'input, 'arena, Tok = CommonToken<'input>> = GrammarTypeContext<'input, 'arena, Tok>;

pub type GrammarTypeContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, GrammarTypeContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct GrammarTypeContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for GrammarTypeContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::GrammarTypeContext }
	fn get_rule_index(&self) -> usize { RULE_grammarType }
    fn make_node(
        arena: &'arena Arena,
        ctx: GrammarTypeContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a GrammarTypeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => GrammarTypeContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut GrammarTypeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut GrammarTypeContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> GrammarTypeContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, GrammarTypeContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait GrammarTypeContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LEXER
    /// Returns `None` if there is no child corresponding to token LEXER
    fn LEXER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token GRAMMAR
    /// Returns `None` if there is no child corresponding to token GRAMMAR
    fn GRAMMAR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PARSER
    /// Returns `None` if there is no child corresponding to token PARSER
    fn PARSER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> GrammarTypeContextAttrs<'input, 'arena, Tok> for GrammarTypeContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LEXER
    /// Returns `None` if there is no child corresponding to token LEXER
    fn LEXER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LEXER)
    }
    /// Retrieves first TerminalNode corresponding to token GRAMMAR
    /// Returns `None` if there is no child corresponding to token GRAMMAR
    fn GRAMMAR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_GRAMMAR)
    }
    /// Retrieves first TerminalNode corresponding to token PARSER
    /// Returns `None` if there is no child corresponding to token PARSER
    fn PARSER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PARSER)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn grammarType(&mut self,) -> Result<&'arena GrammarTypeContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(GrammarTypeContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 4, RULE_grammarType)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena GrammarTypeContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(159);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_LEXER  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(154);
			        recog.base.match_token(ANTLRv4Parser_LEXER,&mut recog.err_handler)?;
			        recog.base.set_state(155);
			        recog.base.match_token(ANTLRv4Parser_GRAMMAR,&mut recog.err_handler)?;
			        }}
			    ANTLRv4Parser_PARSER  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(156);
			        recog.base.match_token(ANTLRv4Parser_PARSER,&mut recog.err_handler)?;
			        recog.base.set_state(157);
			        recog.base.match_token(ANTLRv4Parser_GRAMMAR,&mut recog.err_handler)?;
			        }}
			    ANTLRv4Parser_GRAMMAR  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        recog.base.set_state(158);
			        recog.base.match_token(ANTLRv4Parser_GRAMMAR,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- prequelConstruct ----------------
pub type PrequelConstructContextAll<'input, 'arena, Tok = CommonToken<'input>> = PrequelConstructContext<'input, 'arena, Tok>;

pub type PrequelConstructContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PrequelConstructContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct PrequelConstructContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for PrequelConstructContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::PrequelConstructContext }
	fn get_rule_index(&self) -> usize { RULE_prequelConstruct }
    fn make_node(
        arena: &'arena Arena,
        ctx: PrequelConstructContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a PrequelConstructContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => PrequelConstructContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut PrequelConstructContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut PrequelConstructContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PrequelConstructContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, PrequelConstructContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait PrequelConstructContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>>;
    fn delegateGrammars(&self) -> Option<&'arena DelegateGrammarsContextAll<'input, 'arena, Tok>>;
    fn tokensSpec(&self) -> Option<&'arena TokensSpecContextAll<'input, 'arena, Tok>>;
    fn channelsSpec(&self) -> Option<&'arena ChannelsSpecContextAll<'input, 'arena, Tok>>;
    fn action_(&self) -> Option<&'arena Action_ContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> PrequelConstructContextAttrs<'input, 'arena, Tok> for PrequelConstructContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn delegateGrammars(&self) -> Option<&'arena DelegateGrammarsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn tokensSpec(&self) -> Option<&'arena TokensSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn channelsSpec(&self) -> Option<&'arena ChannelsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn action_(&self) -> Option<&'arena Action_ContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn prequelConstruct(&mut self,) -> Result<&'arena PrequelConstructContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(PrequelConstructContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 6, RULE_prequelConstruct)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena PrequelConstructContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(166);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_OPTIONS  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule optionsSpec*/
			        recog.base.set_state(161);
			        recog.optionsSpec()?;
			        }}
			    ANTLRv4Parser_IMPORT  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule delegateGrammars*/
			        recog.base.set_state(162);
			        recog.delegateGrammars()?;
			        }}
			    ANTLRv4Parser_TOKENS  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule tokensSpec*/
			        recog.base.set_state(163);
			        recog.tokensSpec()?;
			        }}
			    ANTLRv4Parser_CHANNELS  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        /*InvokeRule channelsSpec*/
			        recog.base.set_state(164);
			        recog.channelsSpec()?;
			        }}
			    ANTLRv4Parser_AT  => {
			        /*------- Outer Most Alt 5 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
			        {
			        /*InvokeRule action_*/
			        recog.base.set_state(165);
			        recog.action_()?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- optionsSpec ----------------
pub type OptionsSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = OptionsSpecContext<'input, 'arena, Tok>;

pub type OptionsSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, OptionsSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct OptionsSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for OptionsSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::OptionsSpecContext }
	fn get_rule_index(&self) -> usize { RULE_optionsSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: OptionsSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a OptionsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => OptionsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut OptionsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut OptionsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OptionsSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, OptionsSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait OptionsSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token OPTIONS
    /// Returns `None` if there is no child corresponding to token OPTIONS
    fn OPTIONS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn option_all(&self) -> Vec<&'arena OptionContextAll<'input, 'arena, Tok>>;
    fn option(&self, i: usize) -> Option<&'arena OptionContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token SEMI in current rule
    fn SEMI_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token SEMI, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMI is less than or equal to `i`.
    fn SEMI(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> OptionsSpecContextAttrs<'input, 'arena, Tok> for OptionsSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token OPTIONS
    /// Returns `None` if there is no child corresponding to token OPTIONS
    fn OPTIONS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_OPTIONS)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RBRACE)
    }
    fn option_all(&self) -> Vec<&'arena OptionContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn option(&self, i: usize) -> Option<&'arena OptionContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token SEMI in current rule
    fn SEMI_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token SEMI, starting from 0.
    /// Returns `None` if number of children corresponding to token SEMI is less than or equal to `i`.
    fn SEMI(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn optionsSpec(&mut self,) -> Result<&'arena OptionsSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(OptionsSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 8, RULE_optionsSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena OptionsSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(168);
			recog.base.match_token(ANTLRv4Parser_OPTIONS,&mut recog.err_handler)?;
			recog.base.set_state(174);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_TOKEN_REF || _la==ANTLRv4Parser_RULE_REF {
				{
				{
				/*InvokeRule option*/
				recog.base.set_state(169);
				recog.option()?;
				recog.base.set_state(170);
				recog.base.match_token(ANTLRv4Parser_SEMI,&mut recog.err_handler)?;
				}
				}
				recog.base.set_state(176);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(177);
			recog.base.match_token(ANTLRv4Parser_RBRACE,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- option ----------------
pub type OptionContextAll<'input, 'arena, Tok = CommonToken<'input>> = OptionContext<'input, 'arena, Tok>;

pub type OptionContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, OptionContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct OptionContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for OptionContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::OptionContext }
	fn get_rule_index(&self) -> usize { RULE_option }
    fn make_node(
        arena: &'arena Arena,
        ctx: OptionContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a OptionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => OptionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut OptionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut OptionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OptionContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, OptionContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait OptionContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn optionValue(&self) -> Option<&'arena OptionValueContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> OptionContextAttrs<'input, 'arena, Tok> for OptionContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_ASSIGN)
    }
    fn optionValue(&self) -> Option<&'arena OptionValueContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn option(&mut self,) -> Result<&'arena OptionContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(OptionContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 10, RULE_option)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena OptionContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule identifier*/
			recog.base.set_state(179);
			recog.identifier()?;
			recog.base.set_state(180);
			recog.base.match_token(ANTLRv4Parser_ASSIGN,&mut recog.err_handler)?;
			/*InvokeRule optionValue*/
			recog.base.set_state(181);
			recog.optionValue()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- optionValue ----------------
pub type OptionValueContextAll<'input, 'arena, Tok = CommonToken<'input>> = OptionValueContext<'input, 'arena, Tok>;

pub type OptionValueContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, OptionValueContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct OptionValueContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for OptionValueContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::OptionValueContext }
	fn get_rule_index(&self) -> usize { RULE_optionValue }
    fn make_node(
        arena: &'arena Arena,
        ctx: OptionValueContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a OptionValueContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => OptionValueContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut OptionValueContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut OptionValueContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> OptionValueContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, OptionValueContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait OptionValueContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less than or equal to `i`.
    fn DOT(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> OptionValueContextAttrs<'input, 'arena, Tok> for OptionValueContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_DOT).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less than or equal to `i`.
    fn DOT(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_DOT).nth(i)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_INT)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn optionValue(&mut self,) -> Result<&'arena OptionValueContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(OptionValueContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 12, RULE_optionValue)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena OptionValueContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(194);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule identifier*/
			        recog.base.set_state(183);
			        recog.identifier()?;
			        recog.base.set_state(188);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        while _la==ANTLRv4Parser_DOT {
			        	{
			        	{
			        	recog.base.set_state(184);
			        	recog.base.match_token(ANTLRv4Parser_DOT,&mut recog.err_handler)?;
			        	/*InvokeRule identifier*/
			        	recog.base.set_state(185);
			        	recog.identifier()?;
			        	}
			        	}
			        	recog.base.set_state(190);
			        	recog.err_handler.sync(&mut recog.base)?;
			        	_la = recog.base.input.la(1);
			        }
			        }}
			    ANTLRv4Parser_STRING_LITERAL  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(191);
			        recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
			        }}
			    ANTLRv4Parser_ACTION  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule actionBlock*/
			        recog.base.set_state(192);
			        recog.actionBlock()?;
			        }}
			    ANTLRv4Parser_INT  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        recog.base.set_state(193);
			        recog.base.match_token(ANTLRv4Parser_INT,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- delegateGrammars ----------------
pub type DelegateGrammarsContextAll<'input, 'arena, Tok = CommonToken<'input>> = DelegateGrammarsContext<'input, 'arena, Tok>;

pub type DelegateGrammarsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DelegateGrammarsContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct DelegateGrammarsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for DelegateGrammarsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::DelegateGrammarsContext }
	fn get_rule_index(&self) -> usize { RULE_delegateGrammars }
    fn make_node(
        arena: &'arena Arena,
        ctx: DelegateGrammarsContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a DelegateGrammarsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => DelegateGrammarsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut DelegateGrammarsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut DelegateGrammarsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DelegateGrammarsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, DelegateGrammarsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait DelegateGrammarsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token IMPORT
    /// Returns `None` if there is no child corresponding to token IMPORT
    fn IMPORT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn delegateGrammar_all(&self) -> Vec<&'arena DelegateGrammarContextAll<'input, 'arena, Tok>>;
    fn delegateGrammar(&self, i: usize) -> Option<&'arena DelegateGrammarContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DelegateGrammarsContextAttrs<'input, 'arena, Tok> for DelegateGrammarsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token IMPORT
    /// Returns `None` if there is no child corresponding to token IMPORT
    fn IMPORT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_IMPORT)
    }
    fn delegateGrammar_all(&self) -> Vec<&'arena DelegateGrammarContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn delegateGrammar(&self, i: usize) -> Option<&'arena DelegateGrammarContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn delegateGrammars(&mut self,) -> Result<&'arena DelegateGrammarsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(DelegateGrammarsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 14, RULE_delegateGrammars)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena DelegateGrammarsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(196);
			recog.base.match_token(ANTLRv4Parser_IMPORT,&mut recog.err_handler)?;
			/*InvokeRule delegateGrammar*/
			recog.base.set_state(197);
			recog.delegateGrammar()?;
			recog.base.set_state(202);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_COMMA {
				{
				{
				recog.base.set_state(198);
				recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
				/*InvokeRule delegateGrammar*/
				recog.base.set_state(199);
				recog.delegateGrammar()?;
				}
				}
				recog.base.set_state(204);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(205);
			recog.base.match_token(ANTLRv4Parser_SEMI,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- delegateGrammar ----------------
pub type DelegateGrammarContextAll<'input, 'arena, Tok = CommonToken<'input>> = DelegateGrammarContext<'input, 'arena, Tok>;

pub type DelegateGrammarContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, DelegateGrammarContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct DelegateGrammarContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for DelegateGrammarContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::DelegateGrammarContext }
	fn get_rule_index(&self) -> usize { RULE_delegateGrammar }
    fn make_node(
        arena: &'arena Arena,
        ctx: DelegateGrammarContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a DelegateGrammarContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => DelegateGrammarContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut DelegateGrammarContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut DelegateGrammarContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> DelegateGrammarContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, DelegateGrammarContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait DelegateGrammarContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> DelegateGrammarContextAttrs<'input, 'arena, Tok> for DelegateGrammarContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_ASSIGN)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn delegateGrammar(&mut self,) -> Result<&'arena DelegateGrammarContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(DelegateGrammarContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 16, RULE_delegateGrammar)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena DelegateGrammarContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(212);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(8,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule identifier*/
					recog.base.set_state(207);
					recog.identifier()?;
					recog.base.set_state(208);
					recog.base.match_token(ANTLRv4Parser_ASSIGN,&mut recog.err_handler)?;
					/*InvokeRule identifier*/
					recog.base.set_state(209);
					recog.identifier()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule identifier*/
					recog.base.set_state(211);
					recog.identifier()?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- tokensSpec ----------------
pub type TokensSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = TokensSpecContext<'input, 'arena, Tok>;

pub type TokensSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TokensSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct TokensSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for TokensSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::TokensSpecContext }
	fn get_rule_index(&self) -> usize { RULE_tokensSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: TokensSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a TokensSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => TokensSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut TokensSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut TokensSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TokensSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, TokensSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait TokensSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token TOKENS
    /// Returns `None` if there is no child corresponding to token TOKENS
    fn TOKENS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn idList(&self) -> Option<&'arena IdListContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TokensSpecContextAttrs<'input, 'arena, Tok> for TokensSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token TOKENS
    /// Returns `None` if there is no child corresponding to token TOKENS
    fn TOKENS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_TOKENS)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RBRACE)
    }
    fn idList(&self) -> Option<&'arena IdListContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn tokensSpec(&mut self,) -> Result<&'arena TokensSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(TokensSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 18, RULE_tokensSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena TokensSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(214);
			recog.base.match_token(ANTLRv4Parser_TOKENS,&mut recog.err_handler)?;
			recog.base.set_state(216);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_TOKEN_REF || _la==ANTLRv4Parser_RULE_REF {
				{
				/*InvokeRule idList*/
				recog.base.set_state(215);
				recog.idList()?;
				}
			}

			recog.base.set_state(218);
			recog.base.match_token(ANTLRv4Parser_RBRACE,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- channelsSpec ----------------
pub type ChannelsSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = ChannelsSpecContext<'input, 'arena, Tok>;

pub type ChannelsSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ChannelsSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ChannelsSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ChannelsSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ChannelsSpecContext }
	fn get_rule_index(&self) -> usize { RULE_channelsSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: ChannelsSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ChannelsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ChannelsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ChannelsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ChannelsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ChannelsSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ChannelsSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ChannelsSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token CHANNELS
    /// Returns `None` if there is no child corresponding to token CHANNELS
    fn CHANNELS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn idList(&self) -> Option<&'arena IdListContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ChannelsSpecContextAttrs<'input, 'arena, Tok> for ChannelsSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token CHANNELS
    /// Returns `None` if there is no child corresponding to token CHANNELS
    fn CHANNELS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_CHANNELS)
    }
    /// Retrieves first TerminalNode corresponding to token RBRACE
    /// Returns `None` if there is no child corresponding to token RBRACE
    fn RBRACE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RBRACE)
    }
    fn idList(&self) -> Option<&'arena IdListContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn channelsSpec(&mut self,) -> Result<&'arena ChannelsSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ChannelsSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 20, RULE_channelsSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ChannelsSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(220);
			recog.base.match_token(ANTLRv4Parser_CHANNELS,&mut recog.err_handler)?;
			recog.base.set_state(222);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_TOKEN_REF || _la==ANTLRv4Parser_RULE_REF {
				{
				/*InvokeRule idList*/
				recog.base.set_state(221);
				recog.idList()?;
				}
			}

			recog.base.set_state(224);
			recog.base.match_token(ANTLRv4Parser_RBRACE,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- idList ----------------
pub type IdListContextAll<'input, 'arena, Tok = CommonToken<'input>> = IdListContext<'input, 'arena, Tok>;

pub type IdListContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, IdListContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct IdListContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for IdListContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::IdListContext }
	fn get_rule_index(&self) -> usize { RULE_idList }
    fn make_node(
        arena: &'arena Arena,
        ctx: IdListContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a IdListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => IdListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut IdListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut IdListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IdListContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, IdListContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait IdListContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> IdListContextAttrs<'input, 'arena, Tok> for IdListContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn idList(&mut self,) -> Result<&'arena IdListContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(IdListContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 22, RULE_idList)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena IdListContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
	        let mut _alt: i32;
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule identifier*/
			recog.base.set_state(226);
			recog.identifier()?;
			recog.base.set_state(231);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(11,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(227);
					recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
					/*InvokeRule identifier*/
					recog.base.set_state(228);
					recog.identifier()?;
					}
					} 
				}
				recog.base.set_state(233);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.get_interpreter().adaptive_predict(11,&mut recog.base)?;
			}
			recog.base.set_state(235);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_COMMA {
				{
				recog.base.set_state(234);
				recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
				}
			}

			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- action_ ----------------
pub type Action_ContextAll<'input, 'arena, Tok = CommonToken<'input>> = Action_Context<'input, 'arena, Tok>;

pub type Action_Context<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, Action_ContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct Action_ContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for Action_ContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::Action_Context }
	fn get_rule_index(&self) -> usize { RULE_action_ }
    fn make_node(
        arena: &'arena Arena,
        ctx: Action_Context<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a Action_Context<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => Action_Context<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut Action_Context<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut Action_Context<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> Action_ContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, Action_ContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait Action_ContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
    fn actionScopeName(&self) -> Option<&'arena ActionScopeNameContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COLONCOLON
    /// Returns `None` if there is no child corresponding to token COLONCOLON
    fn COLONCOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> Action_ContextAttrs<'input, 'arena, Tok> for Action_Context<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_AT)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn actionScopeName(&self) -> Option<&'arena ActionScopeNameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COLONCOLON
    /// Returns `None` if there is no child corresponding to token COLONCOLON
    fn COLONCOLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_COLONCOLON)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn action_(&mut self,) -> Result<&'arena Action_ContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(Action_ContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 24, RULE_action_)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena Action_Context<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(237);
			recog.base.match_token(ANTLRv4Parser_AT,&mut recog.err_handler)?;
			recog.base.set_state(241);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(13,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule actionScopeName*/
					recog.base.set_state(238);
					recog.actionScopeName()?;
					recog.base.set_state(239);
					recog.base.match_token(ANTLRv4Parser_COLONCOLON,&mut recog.err_handler)?;
					}
				}

				_ => {}
			}
			/*InvokeRule identifier*/
			recog.base.set_state(243);
			recog.identifier()?;
			/*InvokeRule actionBlock*/
			recog.base.set_state(244);
			recog.actionBlock()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- actionScopeName ----------------
pub type ActionScopeNameContextAll<'input, 'arena, Tok = CommonToken<'input>> = ActionScopeNameContext<'input, 'arena, Tok>;

pub type ActionScopeNameContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ActionScopeNameContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ActionScopeNameContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ActionScopeNameContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ActionScopeNameContext }
	fn get_rule_index(&self) -> usize { RULE_actionScopeName }
    fn make_node(
        arena: &'arena Arena,
        ctx: ActionScopeNameContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ActionScopeNameContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ActionScopeNameContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ActionScopeNameContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ActionScopeNameContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ActionScopeNameContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ActionScopeNameContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ActionScopeNameContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token LEXER
    /// Returns `None` if there is no child corresponding to token LEXER
    fn LEXER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PARSER
    /// Returns `None` if there is no child corresponding to token PARSER
    fn PARSER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ActionScopeNameContextAttrs<'input, 'arena, Tok> for ActionScopeNameContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LEXER
    /// Returns `None` if there is no child corresponding to token LEXER
    fn LEXER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LEXER)
    }
    /// Retrieves first TerminalNode corresponding to token PARSER
    /// Returns `None` if there is no child corresponding to token PARSER
    fn PARSER(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PARSER)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn actionScopeName(&mut self,) -> Result<&'arena ActionScopeNameContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ActionScopeNameContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 26, RULE_actionScopeName)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ActionScopeNameContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(249);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule identifier*/
			        recog.base.set_state(246);
			        recog.identifier()?;
			        }}
			    ANTLRv4Parser_LEXER  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(247);
			        recog.base.match_token(ANTLRv4Parser_LEXER,&mut recog.err_handler)?;
			        }}
			    ANTLRv4Parser_PARSER  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        recog.base.set_state(248);
			        recog.base.match_token(ANTLRv4Parser_PARSER,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- actionBlock ----------------
pub type ActionBlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = ActionBlockContext<'input, 'arena, Tok>;

pub type ActionBlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ActionBlockContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ActionBlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ActionBlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ActionBlockContext }
	fn get_rule_index(&self) -> usize { RULE_actionBlock }
    fn make_node(
        arena: &'arena Arena,
        ctx: ActionBlockContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ActionBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ActionBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ActionBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ActionBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ActionBlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ActionBlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ActionBlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token ACTION
    /// Returns `None` if there is no child corresponding to token ACTION
    fn ACTION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ActionBlockContextAttrs<'input, 'arena, Tok> for ActionBlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token ACTION
    /// Returns `None` if there is no child corresponding to token ACTION
    fn ACTION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_ACTION)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn actionBlock(&mut self,) -> Result<&'arena ActionBlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ActionBlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 28, RULE_actionBlock)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ActionBlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(251);
			recog.base.match_token(ANTLRv4Parser_ACTION,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- argActionBlock ----------------
pub type ArgActionBlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = ArgActionBlockContext<'input, 'arena, Tok>;

pub type ArgActionBlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ArgActionBlockContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ArgActionBlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ArgActionBlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ArgActionBlockContext }
	fn get_rule_index(&self) -> usize { RULE_argActionBlock }
    fn make_node(
        arena: &'arena Arena,
        ctx: ArgActionBlockContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ArgActionBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ArgActionBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ArgActionBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ArgActionBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ArgActionBlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ArgActionBlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ArgActionBlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token BEGIN_ARGUMENT
    /// Returns `None` if there is no child corresponding to token BEGIN_ARGUMENT
    fn BEGIN_ARGUMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token END_ARGUMENT
    /// Returns `None` if there is no child corresponding to token END_ARGUMENT
    fn END_ARGUMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token ARGUMENT_CONTENT in current rule
    fn ARGUMENT_CONTENT_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token ARGUMENT_CONTENT, starting from 0.
    /// Returns `None` if number of children corresponding to token ARGUMENT_CONTENT is less than or equal to `i`.
    fn ARGUMENT_CONTENT(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ArgActionBlockContextAttrs<'input, 'arena, Tok> for ArgActionBlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token BEGIN_ARGUMENT
    /// Returns `None` if there is no child corresponding to token BEGIN_ARGUMENT
    fn BEGIN_ARGUMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_BEGIN_ARGUMENT)
    }
    /// Retrieves first TerminalNode corresponding to token END_ARGUMENT
    /// Returns `None` if there is no child corresponding to token END_ARGUMENT
    fn END_ARGUMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_END_ARGUMENT)
    }
    /// Retrieves all `TerminalNode`s corresponding to token ARGUMENT_CONTENT in current rule
    fn ARGUMENT_CONTENT_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_ARGUMENT_CONTENT).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token ARGUMENT_CONTENT, starting from 0.
    /// Returns `None` if number of children corresponding to token ARGUMENT_CONTENT is less than or equal to `i`.
    fn ARGUMENT_CONTENT(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_ARGUMENT_CONTENT).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn argActionBlock(&mut self,) -> Result<&'arena ArgActionBlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ArgActionBlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 30, RULE_argActionBlock)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ArgActionBlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
	        let mut _alt: i32;
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(253);
			recog.base.match_token(ANTLRv4Parser_BEGIN_ARGUMENT,&mut recog.err_handler)?;
			recog.base.set_state(257);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.get_interpreter().adaptive_predict(15,&mut recog.base)?;
			while { _alt!=1 && _alt!=INVALID_ALT } {
				if _alt==1+1 {
					{
					{
					recog.base.set_state(254);
					recog.base.match_token(ANTLRv4Parser_ARGUMENT_CONTENT,&mut recog.err_handler)?;
					}
					} 
				}
				recog.base.set_state(259);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.get_interpreter().adaptive_predict(15,&mut recog.base)?;
			}
			recog.base.set_state(260);
			recog.base.match_token(ANTLRv4Parser_END_ARGUMENT,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- modeSpec ----------------
pub type ModeSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = ModeSpecContext<'input, 'arena, Tok>;

pub type ModeSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ModeSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ModeSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ModeSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ModeSpecContext }
	fn get_rule_index(&self) -> usize { RULE_modeSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: ModeSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ModeSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ModeSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ModeSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ModeSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ModeSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ModeSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ModeSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token MODE
    /// Returns `None` if there is no child corresponding to token MODE
    fn MODE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn lexerRuleSpec_all(&self) -> Vec<&'arena LexerRuleSpecContextAll<'input, 'arena, Tok>>;
    fn lexerRuleSpec(&self, i: usize) -> Option<&'arena LexerRuleSpecContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ModeSpecContextAttrs<'input, 'arena, Tok> for ModeSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token MODE
    /// Returns `None` if there is no child corresponding to token MODE
    fn MODE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_MODE)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI)
    }
    fn lexerRuleSpec_all(&self) -> Vec<&'arena LexerRuleSpecContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn lexerRuleSpec(&self, i: usize) -> Option<&'arena LexerRuleSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn modeSpec(&mut self,) -> Result<&'arena ModeSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ModeSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 32, RULE_modeSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ModeSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(262);
			recog.base.match_token(ANTLRv4Parser_MODE,&mut recog.err_handler)?;
			/*InvokeRule identifier*/
			recog.base.set_state(263);
			recog.identifier()?;
			recog.base.set_state(264);
			recog.base.match_token(ANTLRv4Parser_SEMI,&mut recog.err_handler)?;
			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_TOKEN_REF || _la==ANTLRv4Parser_FRAGMENT {
				{
				{
				/*InvokeRule lexerRuleSpec*/
				recog.base.set_state(265);
				recog.lexerRuleSpec()?;
				}
				}
				recog.base.set_state(270);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- rules ----------------
pub type RulesContextAll<'input, 'arena, Tok = CommonToken<'input>> = RulesContext<'input, 'arena, Tok>;

pub type RulesContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RulesContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RulesContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RulesContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RulesContext }
	fn get_rule_index(&self) -> usize { RULE_rules }
    fn make_node(
        arena: &'arena Arena,
        ctx: RulesContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RulesContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RulesContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RulesContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RulesContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RulesContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RulesContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RulesContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn ruleSpec_all(&self) -> Vec<&'arena RuleSpecContextAll<'input, 'arena, Tok>>;
    fn ruleSpec(&self, i: usize) -> Option<&'arena RuleSpecContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RulesContextAttrs<'input, 'arena, Tok> for RulesContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ruleSpec_all(&self) -> Vec<&'arena RuleSpecContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn ruleSpec(&self, i: usize) -> Option<&'arena RuleSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn rules(&mut self,) -> Result<&'arena RulesContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RulesContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 34, RULE_rules)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RulesContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(274);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 7405574) != 0) {
				{
				{
				/*InvokeRule ruleSpec*/
				recog.base.set_state(271);
				recog.ruleSpec()?;
				}
				}
				recog.base.set_state(276);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleSpec ----------------
pub type RuleSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleSpecContext<'input, 'arena, Tok>;

pub type RuleSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleSpecContext }
	fn get_rule_index(&self) -> usize { RULE_ruleSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn parserRuleSpec(&self) -> Option<&'arena ParserRuleSpecContextAll<'input, 'arena, Tok>>;
    fn lexerRuleSpec(&self) -> Option<&'arena LexerRuleSpecContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleSpecContextAttrs<'input, 'arena, Tok> for RuleSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn parserRuleSpec(&self) -> Option<&'arena ParserRuleSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn lexerRuleSpec(&self) -> Option<&'arena LexerRuleSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleSpec(&mut self,) -> Result<&'arena RuleSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 36, RULE_ruleSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(279);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(18,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule parserRuleSpec*/
					recog.base.set_state(277);
					recog.parserRuleSpec()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule lexerRuleSpec*/
					recog.base.set_state(278);
					recog.lexerRuleSpec()?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- parserRuleSpec ----------------
pub type ParserRuleSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = ParserRuleSpecContext<'input, 'arena, Tok>;

pub type ParserRuleSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ParserRuleSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ParserRuleSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ParserRuleSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ParserRuleSpecContext }
	fn get_rule_index(&self) -> usize { RULE_parserRuleSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: ParserRuleSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ParserRuleSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ParserRuleSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ParserRuleSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ParserRuleSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ParserRuleSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ParserRuleSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ParserRuleSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token RULE_REF
    /// Returns `None` if there is no child corresponding to token RULE_REF
    fn RULE_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn ruleBlock(&self) -> Option<&'arena RuleBlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn exceptionGroup(&self) -> Option<&'arena ExceptionGroupContextAll<'input, 'arena, Tok>>;
    fn ruleModifiers(&self) -> Option<&'arena RuleModifiersContextAll<'input, 'arena, Tok>>;
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>>;
    fn ruleReturns(&self) -> Option<&'arena RuleReturnsContextAll<'input, 'arena, Tok>>;
    fn throwsSpec(&self) -> Option<&'arena ThrowsSpecContextAll<'input, 'arena, Tok>>;
    fn localsSpec(&self) -> Option<&'arena LocalsSpecContextAll<'input, 'arena, Tok>>;
    fn rulePrequel_all(&self) -> Vec<&'arena RulePrequelContextAll<'input, 'arena, Tok>>;
    fn rulePrequel(&self, i: usize) -> Option<&'arena RulePrequelContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ParserRuleSpecContextAttrs<'input, 'arena, Tok> for ParserRuleSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token RULE_REF
    /// Returns `None` if there is no child corresponding to token RULE_REF
    fn RULE_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RULE_REF)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_COLON)
    }
    fn ruleBlock(&self) -> Option<&'arena RuleBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI)
    }
    fn exceptionGroup(&self) -> Option<&'arena ExceptionGroupContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ruleModifiers(&self) -> Option<&'arena RuleModifiersContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ruleReturns(&self) -> Option<&'arena RuleReturnsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn throwsSpec(&self) -> Option<&'arena ThrowsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn localsSpec(&self) -> Option<&'arena LocalsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn rulePrequel_all(&self) -> Vec<&'arena RulePrequelContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn rulePrequel(&self, i: usize) -> Option<&'arena RulePrequelContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn parserRuleSpec(&mut self,) -> Result<&'arena ParserRuleSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ParserRuleSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 38, RULE_parserRuleSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ParserRuleSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 7405568) != 0) {
				{
				/*InvokeRule ruleModifiers*/
				recog.base.set_state(281);
				recog.ruleModifiers()?;
				}
			}

			recog.base.set_state(284);
			recog.base.match_token(ANTLRv4Parser_RULE_REF,&mut recog.err_handler)?;
			recog.base.set_state(286);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_BEGIN_ARGUMENT {
				{
				/*InvokeRule argActionBlock*/
				recog.base.set_state(285);
				recog.argActionBlock()?;
				}
			}

			recog.base.set_state(289);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_RETURNS {
				{
				/*InvokeRule ruleReturns*/
				recog.base.set_state(288);
				recog.ruleReturns()?;
				}
			}

			recog.base.set_state(292);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_THROWS {
				{
				/*InvokeRule throwsSpec*/
				recog.base.set_state(291);
				recog.throwsSpec()?;
				}
			}

			recog.base.set_state(295);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_LOCALS {
				{
				/*InvokeRule localsSpec*/
				recog.base.set_state(294);
				recog.localsSpec()?;
				}
			}

			recog.base.set_state(300);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_OPTIONS || _la==ANTLRv4Parser_AT {
				{
				{
				/*InvokeRule rulePrequel*/
				recog.base.set_state(297);
				recog.rulePrequel()?;
				}
				}
				recog.base.set_state(302);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(303);
			recog.base.match_token(ANTLRv4Parser_COLON,&mut recog.err_handler)?;
			/*InvokeRule ruleBlock*/
			recog.base.set_state(304);
			recog.ruleBlock()?;
			recog.base.set_state(305);
			recog.base.match_token(ANTLRv4Parser_SEMI,&mut recog.err_handler)?;
			/*InvokeRule exceptionGroup*/
			recog.base.set_state(306);
			recog.exceptionGroup()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- exceptionGroup ----------------
pub type ExceptionGroupContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExceptionGroupContext<'input, 'arena, Tok>;

pub type ExceptionGroupContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExceptionGroupContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExceptionGroupContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ExceptionGroupContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ExceptionGroupContext }
	fn get_rule_index(&self) -> usize { RULE_exceptionGroup }
    fn make_node(
        arena: &'arena Arena,
        ctx: ExceptionGroupContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ExceptionGroupContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ExceptionGroupContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ExceptionGroupContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ExceptionGroupContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ExceptionGroupContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ExceptionGroupContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ExceptionGroupContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn exceptionHandler_all(&self) -> Vec<&'arena ExceptionHandlerContextAll<'input, 'arena, Tok>>;
    fn exceptionHandler(&self, i: usize) -> Option<&'arena ExceptionHandlerContextAll<'input, 'arena, Tok>>;
    fn finallyClause(&self) -> Option<&'arena FinallyClauseContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExceptionGroupContextAttrs<'input, 'arena, Tok> for ExceptionGroupContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn exceptionHandler_all(&self) -> Vec<&'arena ExceptionHandlerContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn exceptionHandler(&self, i: usize) -> Option<&'arena ExceptionHandlerContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    fn finallyClause(&self) -> Option<&'arena FinallyClauseContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn exceptionGroup(&mut self,) -> Result<&'arena ExceptionGroupContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ExceptionGroupContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 40, RULE_exceptionGroup)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ExceptionGroupContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(311);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_CATCH {
				{
				{
				/*InvokeRule exceptionHandler*/
				recog.base.set_state(308);
				recog.exceptionHandler()?;
				}
				}
				recog.base.set_state(313);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(315);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_FINALLY {
				{
				/*InvokeRule finallyClause*/
				recog.base.set_state(314);
				recog.finallyClause()?;
				}
			}

			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- exceptionHandler ----------------
pub type ExceptionHandlerContextAll<'input, 'arena, Tok = CommonToken<'input>> = ExceptionHandlerContext<'input, 'arena, Tok>;

pub type ExceptionHandlerContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ExceptionHandlerContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ExceptionHandlerContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ExceptionHandlerContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ExceptionHandlerContext }
	fn get_rule_index(&self) -> usize { RULE_exceptionHandler }
    fn make_node(
        arena: &'arena Arena,
        ctx: ExceptionHandlerContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ExceptionHandlerContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ExceptionHandlerContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ExceptionHandlerContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ExceptionHandlerContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ExceptionHandlerContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ExceptionHandlerContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ExceptionHandlerContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token CATCH
    /// Returns `None` if there is no child corresponding to token CATCH
    fn CATCH(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ExceptionHandlerContextAttrs<'input, 'arena, Tok> for ExceptionHandlerContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token CATCH
    /// Returns `None` if there is no child corresponding to token CATCH
    fn CATCH(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_CATCH)
    }
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn exceptionHandler(&mut self,) -> Result<&'arena ExceptionHandlerContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ExceptionHandlerContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 42, RULE_exceptionHandler)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ExceptionHandlerContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(317);
			recog.base.match_token(ANTLRv4Parser_CATCH,&mut recog.err_handler)?;
			/*InvokeRule argActionBlock*/
			recog.base.set_state(318);
			recog.argActionBlock()?;
			/*InvokeRule actionBlock*/
			recog.base.set_state(319);
			recog.actionBlock()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- finallyClause ----------------
pub type FinallyClauseContextAll<'input, 'arena, Tok = CommonToken<'input>> = FinallyClauseContext<'input, 'arena, Tok>;

pub type FinallyClauseContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, FinallyClauseContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct FinallyClauseContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for FinallyClauseContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::FinallyClauseContext }
	fn get_rule_index(&self) -> usize { RULE_finallyClause }
    fn make_node(
        arena: &'arena Arena,
        ctx: FinallyClauseContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a FinallyClauseContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => FinallyClauseContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut FinallyClauseContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut FinallyClauseContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> FinallyClauseContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, FinallyClauseContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait FinallyClauseContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token FINALLY
    /// Returns `None` if there is no child corresponding to token FINALLY
    fn FINALLY(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> FinallyClauseContextAttrs<'input, 'arena, Tok> for FinallyClauseContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token FINALLY
    /// Returns `None` if there is no child corresponding to token FINALLY
    fn FINALLY(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_FINALLY)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn finallyClause(&mut self,) -> Result<&'arena FinallyClauseContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(FinallyClauseContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 44, RULE_finallyClause)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena FinallyClauseContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(321);
			recog.base.match_token(ANTLRv4Parser_FINALLY,&mut recog.err_handler)?;
			/*InvokeRule actionBlock*/
			recog.base.set_state(322);
			recog.actionBlock()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- rulePrequel ----------------
pub type RulePrequelContextAll<'input, 'arena, Tok = CommonToken<'input>> = RulePrequelContext<'input, 'arena, Tok>;

pub type RulePrequelContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RulePrequelContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RulePrequelContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RulePrequelContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RulePrequelContext }
	fn get_rule_index(&self) -> usize { RULE_rulePrequel }
    fn make_node(
        arena: &'arena Arena,
        ctx: RulePrequelContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RulePrequelContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RulePrequelContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RulePrequelContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RulePrequelContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RulePrequelContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RulePrequelContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RulePrequelContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>>;
    fn ruleAction(&self) -> Option<&'arena RuleActionContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RulePrequelContextAttrs<'input, 'arena, Tok> for RulePrequelContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ruleAction(&self) -> Option<&'arena RuleActionContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn rulePrequel(&mut self,) -> Result<&'arena RulePrequelContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RulePrequelContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 46, RULE_rulePrequel)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RulePrequelContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(326);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_OPTIONS  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule optionsSpec*/
			        recog.base.set_state(324);
			        recog.optionsSpec()?;
			        }}
			    ANTLRv4Parser_AT  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule ruleAction*/
			        recog.base.set_state(325);
			        recog.ruleAction()?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleReturns ----------------
pub type RuleReturnsContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleReturnsContext<'input, 'arena, Tok>;

pub type RuleReturnsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleReturnsContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleReturnsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleReturnsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleReturnsContext }
	fn get_rule_index(&self) -> usize { RULE_ruleReturns }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleReturnsContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleReturnsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleReturnsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleReturnsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleReturnsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleReturnsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleReturnsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleReturnsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token RETURNS
    /// Returns `None` if there is no child corresponding to token RETURNS
    fn RETURNS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleReturnsContextAttrs<'input, 'arena, Tok> for RuleReturnsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token RETURNS
    /// Returns `None` if there is no child corresponding to token RETURNS
    fn RETURNS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RETURNS)
    }
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleReturns(&mut self,) -> Result<&'arena RuleReturnsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleReturnsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 48, RULE_ruleReturns)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleReturnsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(328);
			recog.base.match_token(ANTLRv4Parser_RETURNS,&mut recog.err_handler)?;
			/*InvokeRule argActionBlock*/
			recog.base.set_state(329);
			recog.argActionBlock()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- throwsSpec ----------------
pub type ThrowsSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = ThrowsSpecContext<'input, 'arena, Tok>;

pub type ThrowsSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ThrowsSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ThrowsSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ThrowsSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ThrowsSpecContext }
	fn get_rule_index(&self) -> usize { RULE_throwsSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: ThrowsSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ThrowsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ThrowsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ThrowsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ThrowsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ThrowsSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ThrowsSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ThrowsSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token THROWS
    /// Returns `None` if there is no child corresponding to token THROWS
    fn THROWS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn qualifiedIdentifier_all(&self) -> Vec<&'arena QualifiedIdentifierContextAll<'input, 'arena, Tok>>;
    fn qualifiedIdentifier(&self, i: usize) -> Option<&'arena QualifiedIdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ThrowsSpecContextAttrs<'input, 'arena, Tok> for ThrowsSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token THROWS
    /// Returns `None` if there is no child corresponding to token THROWS
    fn THROWS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_THROWS)
    }
    fn qualifiedIdentifier_all(&self) -> Vec<&'arena QualifiedIdentifierContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn qualifiedIdentifier(&self, i: usize) -> Option<&'arena QualifiedIdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn throwsSpec(&mut self,) -> Result<&'arena ThrowsSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ThrowsSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 50, RULE_throwsSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ThrowsSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(331);
			recog.base.match_token(ANTLRv4Parser_THROWS,&mut recog.err_handler)?;
			/*InvokeRule qualifiedIdentifier*/
			recog.base.set_state(332);
			recog.qualifiedIdentifier()?;
			recog.base.set_state(337);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_COMMA {
				{
				{
				recog.base.set_state(333);
				recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
				/*InvokeRule qualifiedIdentifier*/
				recog.base.set_state(334);
				recog.qualifiedIdentifier()?;
				}
				}
				recog.base.set_state(339);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- localsSpec ----------------
pub type LocalsSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = LocalsSpecContext<'input, 'arena, Tok>;

pub type LocalsSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LocalsSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LocalsSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LocalsSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LocalsSpecContext }
	fn get_rule_index(&self) -> usize { RULE_localsSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: LocalsSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LocalsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LocalsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LocalsSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LocalsSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LocalsSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LocalsSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LocalsSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LOCALS
    /// Returns `None` if there is no child corresponding to token LOCALS
    fn LOCALS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LocalsSpecContextAttrs<'input, 'arena, Tok> for LocalsSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LOCALS
    /// Returns `None` if there is no child corresponding to token LOCALS
    fn LOCALS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LOCALS)
    }
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn localsSpec(&mut self,) -> Result<&'arena LocalsSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LocalsSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 52, RULE_localsSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LocalsSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(340);
			recog.base.match_token(ANTLRv4Parser_LOCALS,&mut recog.err_handler)?;
			/*InvokeRule argActionBlock*/
			recog.base.set_state(341);
			recog.argActionBlock()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleAction ----------------
pub type RuleActionContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleActionContext<'input, 'arena, Tok>;

pub type RuleActionContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleActionContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleActionContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleActionContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleActionContext }
	fn get_rule_index(&self) -> usize { RULE_ruleAction }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleActionContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleActionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleActionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleActionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleActionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleActionContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleActionContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleActionContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleActionContextAttrs<'input, 'arena, Tok> for RuleActionContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token AT
    /// Returns `None` if there is no child corresponding to token AT
    fn AT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_AT)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleAction(&mut self,) -> Result<&'arena RuleActionContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleActionContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 54, RULE_ruleAction)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleActionContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(343);
			recog.base.match_token(ANTLRv4Parser_AT,&mut recog.err_handler)?;
			/*InvokeRule identifier*/
			recog.base.set_state(344);
			recog.identifier()?;
			/*InvokeRule actionBlock*/
			recog.base.set_state(345);
			recog.actionBlock()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleModifiers ----------------
pub type RuleModifiersContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleModifiersContext<'input, 'arena, Tok>;

pub type RuleModifiersContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleModifiersContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleModifiersContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleModifiersContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleModifiersContext }
	fn get_rule_index(&self) -> usize { RULE_ruleModifiers }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleModifiersContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleModifiersContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleModifiersContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleModifiersContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleModifiersContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleModifiersContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleModifiersContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleModifiersContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn ruleModifier_all(&self) -> Vec<&'arena RuleModifierContextAll<'input, 'arena, Tok>>;
    fn ruleModifier(&self, i: usize) -> Option<&'arena RuleModifierContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleModifiersContextAttrs<'input, 'arena, Tok> for RuleModifiersContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ruleModifier_all(&self) -> Vec<&'arena RuleModifierContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn ruleModifier(&self, i: usize) -> Option<&'arena RuleModifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleModifiers(&mut self,) -> Result<&'arena RuleModifiersContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleModifiersContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 56, RULE_ruleModifiers)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleModifiersContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(348); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				{
				/*InvokeRule ruleModifier*/
				recog.base.set_state(347);
				recog.ruleModifier()?;
				}
				}
				recog.base.set_state(350); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 7405568) != 0)) {break}
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleModifier ----------------
pub type RuleModifierContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleModifierContext<'input, 'arena, Tok>;

pub type RuleModifierContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleModifierContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleModifierContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleModifierContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleModifierContext }
	fn get_rule_index(&self) -> usize { RULE_ruleModifier }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleModifierContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleModifierContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleModifierContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleModifierContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleModifierContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleModifierContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleModifierContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleModifierContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token PUBLIC
    /// Returns `None` if there is no child corresponding to token PUBLIC
    fn PUBLIC(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PRIVATE
    /// Returns `None` if there is no child corresponding to token PRIVATE
    fn PRIVATE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PROTECTED
    /// Returns `None` if there is no child corresponding to token PROTECTED
    fn PROTECTED(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FRAGMENT
    /// Returns `None` if there is no child corresponding to token FRAGMENT
    fn FRAGMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleModifierContextAttrs<'input, 'arena, Tok> for RuleModifierContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token PUBLIC
    /// Returns `None` if there is no child corresponding to token PUBLIC
    fn PUBLIC(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PUBLIC)
    }
    /// Retrieves first TerminalNode corresponding to token PRIVATE
    /// Returns `None` if there is no child corresponding to token PRIVATE
    fn PRIVATE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PRIVATE)
    }
    /// Retrieves first TerminalNode corresponding to token PROTECTED
    /// Returns `None` if there is no child corresponding to token PROTECTED
    fn PROTECTED(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PROTECTED)
    }
    /// Retrieves first TerminalNode corresponding to token FRAGMENT
    /// Returns `None` if there is no child corresponding to token FRAGMENT
    fn FRAGMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_FRAGMENT)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleModifier(&mut self,) -> Result<&'arena RuleModifierContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleModifierContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 58, RULE_ruleModifier)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleModifierContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(352);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & 7405568) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleBlock ----------------
pub type RuleBlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleBlockContext<'input, 'arena, Tok>;

pub type RuleBlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleBlockContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleBlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleBlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleBlockContext }
	fn get_rule_index(&self) -> usize { RULE_ruleBlock }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleBlockContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleBlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleBlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleBlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn ruleAltList(&self) -> Option<&'arena RuleAltListContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleBlockContextAttrs<'input, 'arena, Tok> for RuleBlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ruleAltList(&self) -> Option<&'arena RuleAltListContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleBlock(&mut self,) -> Result<&'arena RuleBlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleBlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 60, RULE_ruleBlock)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleBlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule ruleAltList*/
			recog.base.set_state(354);
			recog.ruleAltList()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleAltList ----------------
pub type RuleAltListContextAll<'input, 'arena, Tok = CommonToken<'input>> = RuleAltListContext<'input, 'arena, Tok>;

pub type RuleAltListContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RuleAltListContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RuleAltListContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RuleAltListContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RuleAltListContext }
	fn get_rule_index(&self) -> usize { RULE_ruleAltList }
    fn make_node(
        arena: &'arena Arena,
        ctx: RuleAltListContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RuleAltListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RuleAltListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RuleAltListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RuleAltListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RuleAltListContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RuleAltListContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RuleAltListContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn labeledAlt_all(&self) -> Vec<&'arena LabeledAltContextAll<'input, 'arena, Tok>>;
    fn labeledAlt(&self, i: usize) -> Option<&'arena LabeledAltContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RuleAltListContextAttrs<'input, 'arena, Tok> for RuleAltListContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn labeledAlt_all(&self) -> Vec<&'arena LabeledAltContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn labeledAlt(&self, i: usize) -> Option<&'arena LabeledAltContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleAltList(&mut self,) -> Result<&'arena RuleAltListContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RuleAltListContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 62, RULE_ruleAltList)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RuleAltListContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule labeledAlt*/
			recog.base.set_state(356);
			recog.labeledAlt()?;
			recog.base.set_state(361);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_OR {
				{
				{
				recog.base.set_state(357);
				recog.base.match_token(ANTLRv4Parser_OR,&mut recog.err_handler)?;
				/*InvokeRule labeledAlt*/
				recog.base.set_state(358);
				recog.labeledAlt()?;
				}
				}
				recog.base.set_state(363);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- labeledAlt ----------------
pub type LabeledAltContextAll<'input, 'arena, Tok = CommonToken<'input>> = LabeledAltContext<'input, 'arena, Tok>;

pub type LabeledAltContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LabeledAltContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LabeledAltContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LabeledAltContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LabeledAltContext }
	fn get_rule_index(&self) -> usize { RULE_labeledAlt }
    fn make_node(
        arena: &'arena Arena,
        ctx: LabeledAltContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LabeledAltContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LabeledAltContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LabeledAltContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LabeledAltContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LabeledAltContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LabeledAltContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LabeledAltContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn alternative(&self) -> Option<&'arena AlternativeContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token POUND
    /// Returns `None` if there is no child corresponding to token POUND
    fn POUND(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LabeledAltContextAttrs<'input, 'arena, Tok> for LabeledAltContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn alternative(&self) -> Option<&'arena AlternativeContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token POUND
    /// Returns `None` if there is no child corresponding to token POUND
    fn POUND(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_POUND)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn labeledAlt(&mut self,) -> Result<&'arena LabeledAltContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LabeledAltContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 64, RULE_labeledAlt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LabeledAltContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule alternative*/
			recog.base.set_state(364);
			recog.alternative()?;
			recog.base.set_state(367);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_POUND {
				{
				recog.base.set_state(365);
				recog.base.match_token(ANTLRv4Parser_POUND,&mut recog.err_handler)?;
				/*InvokeRule identifier*/
				recog.base.set_state(366);
				recog.identifier()?;
				}
			}

			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerRuleSpec ----------------
pub type LexerRuleSpecContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerRuleSpecContext<'input, 'arena, Tok>;

pub type LexerRuleSpecContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerRuleSpecContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerRuleSpecContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerRuleSpecContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerRuleSpecContext }
	fn get_rule_index(&self) -> usize { RULE_lexerRuleSpec }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerRuleSpecContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerRuleSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerRuleSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerRuleSpecContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerRuleSpecContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerRuleSpecContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerRuleSpecContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerRuleSpecContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn lexerRuleBlock(&self) -> Option<&'arena LexerRuleBlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token FRAGMENT
    /// Returns `None` if there is no child corresponding to token FRAGMENT
    fn FRAGMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerRuleSpecContextAttrs<'input, 'arena, Tok> for LexerRuleSpecContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_TOKEN_REF)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_COLON)
    }
    fn lexerRuleBlock(&self) -> Option<&'arena LexerRuleBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token SEMI
    /// Returns `None` if there is no child corresponding to token SEMI
    fn SEMI(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_SEMI)
    }
    /// Retrieves first TerminalNode corresponding to token FRAGMENT
    /// Returns `None` if there is no child corresponding to token FRAGMENT
    fn FRAGMENT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_FRAGMENT)
    }
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerRuleSpec(&mut self,) -> Result<&'arena LexerRuleSpecContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerRuleSpecContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 66, RULE_lexerRuleSpec)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerRuleSpecContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(370);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_FRAGMENT {
				{
				recog.base.set_state(369);
				recog.base.match_token(ANTLRv4Parser_FRAGMENT,&mut recog.err_handler)?;
				}
			}

			recog.base.set_state(372);
			recog.base.match_token(ANTLRv4Parser_TOKEN_REF,&mut recog.err_handler)?;
			recog.base.set_state(374);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_OPTIONS {
				{
				/*InvokeRule optionsSpec*/
				recog.base.set_state(373);
				recog.optionsSpec()?;
				}
			}

			recog.base.set_state(376);
			recog.base.match_token(ANTLRv4Parser_COLON,&mut recog.err_handler)?;
			/*InvokeRule lexerRuleBlock*/
			recog.base.set_state(377);
			recog.lexerRuleBlock()?;
			recog.base.set_state(378);
			recog.base.match_token(ANTLRv4Parser_SEMI,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerRuleBlock ----------------
pub type LexerRuleBlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerRuleBlockContext<'input, 'arena, Tok>;

pub type LexerRuleBlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerRuleBlockContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerRuleBlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerRuleBlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerRuleBlockContext }
	fn get_rule_index(&self) -> usize { RULE_lexerRuleBlock }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerRuleBlockContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerRuleBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerRuleBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerRuleBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerRuleBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerRuleBlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerRuleBlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerRuleBlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn lexerAltList(&self) -> Option<&'arena LexerAltListContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerRuleBlockContextAttrs<'input, 'arena, Tok> for LexerRuleBlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn lexerAltList(&self) -> Option<&'arena LexerAltListContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerRuleBlock(&mut self,) -> Result<&'arena LexerRuleBlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerRuleBlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 68, RULE_lexerRuleBlock)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerRuleBlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule lexerAltList*/
			recog.base.set_state(380);
			recog.lexerAltList()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerAltList ----------------
pub type LexerAltListContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerAltListContext<'input, 'arena, Tok>;

pub type LexerAltListContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerAltListContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerAltListContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerAltListContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerAltListContext }
	fn get_rule_index(&self) -> usize { RULE_lexerAltList }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerAltListContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerAltListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerAltListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerAltListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerAltListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerAltListContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerAltListContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerAltListContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn lexerAlt_all(&self) -> Vec<&'arena LexerAltContextAll<'input, 'arena, Tok>>;
    fn lexerAlt(&self, i: usize) -> Option<&'arena LexerAltContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerAltListContextAttrs<'input, 'arena, Tok> for LexerAltListContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn lexerAlt_all(&self) -> Vec<&'arena LexerAltContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn lexerAlt(&self, i: usize) -> Option<&'arena LexerAltContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerAltList(&mut self,) -> Result<&'arena LexerAltListContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerAltListContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 70, RULE_lexerAltList)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerAltListContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule lexerAlt*/
			recog.base.set_state(382);
			recog.lexerAlt()?;
			recog.base.set_state(387);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_OR {
				{
				{
				recog.base.set_state(383);
				recog.base.match_token(ANTLRv4Parser_OR,&mut recog.err_handler)?;
				/*InvokeRule lexerAlt*/
				recog.base.set_state(384);
				recog.lexerAlt()?;
				}
				}
				recog.base.set_state(389);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerAlt ----------------
pub type LexerAltContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerAltContext<'input, 'arena, Tok>;

pub type LexerAltContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerAltContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerAltContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerAltContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerAltContext }
	fn get_rule_index(&self) -> usize { RULE_lexerAlt }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerAltContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerAltContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerAltContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerAltContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerAltContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerAltContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerAltContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerAltContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn lexerElements(&self) -> Option<&'arena LexerElementsContextAll<'input, 'arena, Tok>>;
    fn lexerCommands(&self) -> Option<&'arena LexerCommandsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerAltContextAttrs<'input, 'arena, Tok> for LexerAltContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn lexerElements(&self) -> Option<&'arena LexerElementsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn lexerCommands(&self) -> Option<&'arena LexerCommandsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerAlt(&mut self,) -> Result<&'arena LexerAltContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerAltContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 72, RULE_lexerAlt)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerAltContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(395);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(36,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule lexerElements*/
					recog.base.set_state(390);
					recog.lexerElements()?;
					recog.base.set_state(392);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ANTLRv4Parser_RARROW {
						{
						/*InvokeRule lexerCommands*/
						recog.base.set_state(391);
						recog.lexerCommands()?;
						}
					}

					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerElements ----------------
pub type LexerElementsContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerElementsContext<'input, 'arena, Tok>;

pub type LexerElementsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerElementsContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerElementsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerElementsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerElementsContext }
	fn get_rule_index(&self) -> usize { RULE_lexerElements }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerElementsContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerElementsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerElementsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerElementsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerElementsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerElementsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerElementsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerElementsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn lexerElement_all(&self) -> Vec<&'arena LexerElementContextAll<'input, 'arena, Tok>>;
    fn lexerElement(&self, i: usize) -> Option<&'arena LexerElementContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerElementsContextAttrs<'input, 'arena, Tok> for LexerElementsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn lexerElement_all(&self) -> Vec<&'arena LexerElementContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn lexerElement(&self, i: usize) -> Option<&'arena LexerElementContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerElements(&mut self,) -> Result<&'arena LexerElementsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerElementsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 74, RULE_lexerElements)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerElementsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(403);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_LEXER_CHAR_SET |ANTLRv4Parser_STRING_LITERAL |
			    ANTLRv4Parser_ACTION |ANTLRv4Parser_LPAREN |ANTLRv4Parser_DOT |ANTLRv4Parser_NOT  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(398); 
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        loop {
			        	{
			        	{
			        	/*InvokeRule lexerElement*/
			        	recog.base.set_state(397);
			        	recog.lexerElement()?;
			        	}
			        	}
			        	recog.base.set_state(400); 
			        	recog.err_handler.sync(&mut recog.base)?;
			        	_la = recog.base.input.la(1);
			        	if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 2314) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & 147457) != 0)) {break}
			        }
			        }}
			    ANTLRv4Parser_SEMI |ANTLRv4Parser_RPAREN |ANTLRv4Parser_RARROW |ANTLRv4Parser_OR  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerElement ----------------
pub type LexerElementContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerElementContext<'input, 'arena, Tok>;

pub type LexerElementContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerElementContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerElementContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerElementContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerElementContext }
	fn get_rule_index(&self) -> usize { RULE_lexerElement }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerElementContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerElementContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerElementContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerElementContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn lexerAtom(&self) -> Option<&'arena LexerAtomContextAll<'input, 'arena, Tok>>;
    fn ebnfSuffix(&self) -> Option<&'arena EbnfSuffixContextAll<'input, 'arena, Tok>>;
    fn lexerBlock(&self) -> Option<&'arena LexerBlockContextAll<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token QUESTION
    /// Returns `None` if there is no child corresponding to token QUESTION
    fn QUESTION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerElementContextAttrs<'input, 'arena, Tok> for LexerElementContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn lexerAtom(&self) -> Option<&'arena LexerAtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ebnfSuffix(&self) -> Option<&'arena EbnfSuffixContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn lexerBlock(&self) -> Option<&'arena LexerBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token QUESTION
    /// Returns `None` if there is no child corresponding to token QUESTION
    fn QUESTION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_QUESTION)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerElement(&mut self,) -> Result<&'arena LexerElementContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerElementContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 76, RULE_lexerElement)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerElementContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(417);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_LEXER_CHAR_SET |ANTLRv4Parser_STRING_LITERAL |
			    ANTLRv4Parser_DOT |ANTLRv4Parser_NOT  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule lexerAtom*/
			        recog.base.set_state(405);
			        recog.lexerAtom()?;
			        recog.base.set_state(407);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 11) != 0) {
			        	{
			        	/*InvokeRule ebnfSuffix*/
			        	recog.base.set_state(406);
			        	recog.ebnfSuffix()?;
			        	}
			        }

			        }}
			    ANTLRv4Parser_LPAREN  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule lexerBlock*/
			        recog.base.set_state(409);
			        recog.lexerBlock()?;
			        recog.base.set_state(411);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 11) != 0) {
			        	{
			        	/*InvokeRule ebnfSuffix*/
			        	recog.base.set_state(410);
			        	recog.ebnfSuffix()?;
			        	}
			        }

			        }}
			    ANTLRv4Parser_ACTION  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule actionBlock*/
			        recog.base.set_state(413);
			        recog.actionBlock()?;
			        recog.base.set_state(415);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_QUESTION {
			        	{
			        	recog.base.set_state(414);
			        	recog.base.match_token(ANTLRv4Parser_QUESTION,&mut recog.err_handler)?;
			        	}
			        }

			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerBlock ----------------
pub type LexerBlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerBlockContext<'input, 'arena, Tok>;

pub type LexerBlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerBlockContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerBlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerBlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerBlockContext }
	fn get_rule_index(&self) -> usize { RULE_lexerBlock }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerBlockContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerBlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerBlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerBlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerBlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerBlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn lexerAltList(&self) -> Option<&'arena LexerAltListContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerBlockContextAttrs<'input, 'arena, Tok> for LexerBlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LPAREN)
    }
    fn lexerAltList(&self) -> Option<&'arena LexerAltListContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RPAREN)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerBlock(&mut self,) -> Result<&'arena LexerBlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerBlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 78, RULE_lexerBlock)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerBlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(419);
			recog.base.match_token(ANTLRv4Parser_LPAREN,&mut recog.err_handler)?;
			/*InvokeRule lexerAltList*/
			recog.base.set_state(420);
			recog.lexerAltList()?;
			recog.base.set_state(421);
			recog.base.match_token(ANTLRv4Parser_RPAREN,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerCommands ----------------
pub type LexerCommandsContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerCommandsContext<'input, 'arena, Tok>;

pub type LexerCommandsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerCommandsContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerCommandsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerCommandsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerCommandsContext }
	fn get_rule_index(&self) -> usize { RULE_lexerCommands }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerCommandsContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerCommandsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerCommandsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerCommandsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerCommandsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerCommandsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerCommandsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerCommandsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token RARROW
    /// Returns `None` if there is no child corresponding to token RARROW
    fn RARROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn lexerCommand_all(&self) -> Vec<&'arena LexerCommandContextAll<'input, 'arena, Tok>>;
    fn lexerCommand(&self, i: usize) -> Option<&'arena LexerCommandContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerCommandsContextAttrs<'input, 'arena, Tok> for LexerCommandsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token RARROW
    /// Returns `None` if there is no child corresponding to token RARROW
    fn RARROW(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RARROW)
    }
    fn lexerCommand_all(&self) -> Vec<&'arena LexerCommandContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn lexerCommand(&self, i: usize) -> Option<&'arena LexerCommandContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerCommands(&mut self,) -> Result<&'arena LexerCommandsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerCommandsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 80, RULE_lexerCommands)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerCommandsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(423);
			recog.base.match_token(ANTLRv4Parser_RARROW,&mut recog.err_handler)?;
			/*InvokeRule lexerCommand*/
			recog.base.set_state(424);
			recog.lexerCommand()?;
			recog.base.set_state(429);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_COMMA {
				{
				{
				recog.base.set_state(425);
				recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
				/*InvokeRule lexerCommand*/
				recog.base.set_state(426);
				recog.lexerCommand()?;
				}
				}
				recog.base.set_state(431);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerCommand ----------------
pub type LexerCommandContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerCommandContext<'input, 'arena, Tok>;

pub type LexerCommandContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerCommandContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerCommandContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerCommandContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerCommandContext }
	fn get_rule_index(&self) -> usize { RULE_lexerCommand }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerCommandContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerCommandContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerCommandContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerCommandContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerCommandContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerCommandContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerCommandContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerCommandContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn lexerCommandName(&self) -> Option<&'arena LexerCommandNameContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn lexerCommandExpr(&self) -> Option<&'arena LexerCommandExprContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerCommandContextAttrs<'input, 'arena, Tok> for LexerCommandContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn lexerCommandName(&self) -> Option<&'arena LexerCommandNameContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LPAREN)
    }
    fn lexerCommandExpr(&self) -> Option<&'arena LexerCommandExprContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RPAREN)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerCommand(&mut self,) -> Result<&'arena LexerCommandContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerCommandContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 82, RULE_lexerCommand)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerCommandContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(438);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(44,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule lexerCommandName*/
					recog.base.set_state(432);
					recog.lexerCommandName()?;
					recog.base.set_state(433);
					recog.base.match_token(ANTLRv4Parser_LPAREN,&mut recog.err_handler)?;
					/*InvokeRule lexerCommandExpr*/
					recog.base.set_state(434);
					recog.lexerCommandExpr()?;
					recog.base.set_state(435);
					recog.base.match_token(ANTLRv4Parser_RPAREN,&mut recog.err_handler)?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule lexerCommandName*/
					recog.base.set_state(437);
					recog.lexerCommandName()?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerCommandName ----------------
pub type LexerCommandNameContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerCommandNameContext<'input, 'arena, Tok>;

pub type LexerCommandNameContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerCommandNameContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerCommandNameContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerCommandNameContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerCommandNameContext }
	fn get_rule_index(&self) -> usize { RULE_lexerCommandName }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerCommandNameContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerCommandNameContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerCommandNameContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerCommandNameContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerCommandNameContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerCommandNameContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerCommandNameContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerCommandNameContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token MODE
    /// Returns `None` if there is no child corresponding to token MODE
    fn MODE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerCommandNameContextAttrs<'input, 'arena, Tok> for LexerCommandNameContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token MODE
    /// Returns `None` if there is no child corresponding to token MODE
    fn MODE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_MODE)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerCommandName(&mut self,) -> Result<&'arena LexerCommandNameContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerCommandNameContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 84, RULE_lexerCommandName)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerCommandNameContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(442);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule identifier*/
			        recog.base.set_state(440);
			        recog.identifier()?;
			        }}
			    ANTLRv4Parser_MODE  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(441);
			        recog.base.match_token(ANTLRv4Parser_MODE,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerCommandExpr ----------------
pub type LexerCommandExprContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerCommandExprContext<'input, 'arena, Tok>;

pub type LexerCommandExprContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerCommandExprContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerCommandExprContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerCommandExprContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerCommandExprContext }
	fn get_rule_index(&self) -> usize { RULE_lexerCommandExpr }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerCommandExprContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerCommandExprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerCommandExprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerCommandExprContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerCommandExprContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerCommandExprContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerCommandExprContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerCommandExprContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerCommandExprContextAttrs<'input, 'arena, Tok> for LexerCommandExprContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_INT)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerCommandExpr(&mut self,) -> Result<&'arena LexerCommandExprContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerCommandExprContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 86, RULE_lexerCommandExpr)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerCommandExprContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(446);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule identifier*/
			        recog.base.set_state(444);
			        recog.identifier()?;
			        }}
			    ANTLRv4Parser_INT  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(445);
			        recog.base.match_token(ANTLRv4Parser_INT,&mut recog.err_handler)?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- altList ----------------
pub type AltListContextAll<'input, 'arena, Tok = CommonToken<'input>> = AltListContext<'input, 'arena, Tok>;

pub type AltListContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AltListContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct AltListContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for AltListContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::AltListContext }
	fn get_rule_index(&self) -> usize { RULE_altList }
    fn make_node(
        arena: &'arena Arena,
        ctx: AltListContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AltListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => AltListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AltListContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut AltListContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AltListContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, AltListContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait AltListContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn alternative_all(&self) -> Vec<&'arena AlternativeContextAll<'input, 'arena, Tok>>;
    fn alternative(&self, i: usize) -> Option<&'arena AlternativeContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AltListContextAttrs<'input, 'arena, Tok> for AltListContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn alternative_all(&self) -> Vec<&'arena AlternativeContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn alternative(&self, i: usize) -> Option<&'arena AlternativeContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn altList(&mut self,) -> Result<&'arena AltListContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(AltListContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 88, RULE_altList)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena AltListContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule alternative*/
			recog.base.set_state(448);
			recog.alternative()?;
			recog.base.set_state(453);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_OR {
				{
				{
				recog.base.set_state(449);
				recog.base.match_token(ANTLRv4Parser_OR,&mut recog.err_handler)?;
				/*InvokeRule alternative*/
				recog.base.set_state(450);
				recog.alternative()?;
				}
				}
				recog.base.set_state(455);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- alternative ----------------
pub type AlternativeContextAll<'input, 'arena, Tok = CommonToken<'input>> = AlternativeContext<'input, 'arena, Tok>;

pub type AlternativeContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AlternativeContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct AlternativeContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for AlternativeContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::AlternativeContext }
	fn get_rule_index(&self) -> usize { RULE_alternative }
    fn make_node(
        arena: &'arena Arena,
        ctx: AlternativeContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AlternativeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => AlternativeContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AlternativeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut AlternativeContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AlternativeContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, AlternativeContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait AlternativeContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>>;
    fn element_all(&self) -> Vec<&'arena ElementContextAll<'input, 'arena, Tok>>;
    fn element(&self, i: usize) -> Option<&'arena ElementContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AlternativeContextAttrs<'input, 'arena, Tok> for AlternativeContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn element_all(&self) -> Vec<&'arena ElementContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn element(&self, i: usize) -> Option<&'arena ElementContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn alternative(&mut self,) -> Result<&'arena AlternativeContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(AlternativeContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 90, RULE_alternative)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena AlternativeContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(465);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF |ANTLRv4Parser_STRING_LITERAL |
			    ANTLRv4Parser_ACTION |ANTLRv4Parser_LPAREN |ANTLRv4Parser_LT |ANTLRv4Parser_DOT |
			    ANTLRv4Parser_NOT  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(457);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_LT {
			        	{
			        	/*InvokeRule elementOptions*/
			        	recog.base.set_state(456);
			        	recog.elementOptions()?;
			        	}
			        }

			        recog.base.set_state(460); 
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        loop {
			        	{
			        	{
			        	/*InvokeRule element*/
			        	recog.base.set_state(459);
			        	recog.element()?;
			        	}
			        	}
			        	recog.base.set_state(462); 
			        	recog.err_handler.sync(&mut recog.base)?;
			        	_la = recog.base.input.la(1);
			        	if !((((_la) & !0x3f) == 0 && ((1usize << _la) & 2310) != 0) || ((((_la - 33)) & !0x3f) == 0 && ((1usize << (_la - 33)) & 147457) != 0)) {break}
			        }
			        }}
			    ANTLRv4Parser_SEMI |ANTLRv4Parser_RPAREN |ANTLRv4Parser_OR |ANTLRv4Parser_POUND  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- element ----------------
pub type ElementContextAll<'input, 'arena, Tok = CommonToken<'input>> = ElementContext<'input, 'arena, Tok>;

pub type ElementContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ElementContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ElementContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ElementContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ElementContext }
	fn get_rule_index(&self) -> usize { RULE_element }
    fn make_node(
        arena: &'arena Arena,
        ctx: ElementContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ElementContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ElementContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ElementContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn labeledElement(&self) -> Option<&'arena LabeledElementContextAll<'input, 'arena, Tok>>;
    fn ebnfSuffix(&self) -> Option<&'arena EbnfSuffixContextAll<'input, 'arena, Tok>>;
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
    fn ebnf(&self) -> Option<&'arena EbnfContextAll<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token QUESTION
    /// Returns `None` if there is no child corresponding to token QUESTION
    fn QUESTION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn predicateOptions(&self) -> Option<&'arena PredicateOptionsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ElementContextAttrs<'input, 'arena, Tok> for ElementContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn labeledElement(&self) -> Option<&'arena LabeledElementContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ebnfSuffix(&self) -> Option<&'arena EbnfSuffixContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ebnf(&self) -> Option<&'arena EbnfContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token QUESTION
    /// Returns `None` if there is no child corresponding to token QUESTION
    fn QUESTION(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_QUESTION)
    }
    fn predicateOptions(&self) -> Option<&'arena PredicateOptionsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn element(&mut self,) -> Result<&'arena ElementContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ElementContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 92, RULE_element)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ElementContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(485);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(55,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule labeledElement*/
					recog.base.set_state(467);
					recog.labeledElement()?;
					recog.base.set_state(470);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					    ANTLRv4Parser_QUESTION |ANTLRv4Parser_STAR |ANTLRv4Parser_PLUS  => {
					        {
					        /*InvokeRule ebnfSuffix*/
					        recog.base.set_state(468);
					        recog.ebnfSuffix()?;
					        }}
					    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF |ANTLRv4Parser_STRING_LITERAL |
					    ANTLRv4Parser_ACTION |ANTLRv4Parser_SEMI |ANTLRv4Parser_LPAREN |
					    ANTLRv4Parser_RPAREN |ANTLRv4Parser_OR |ANTLRv4Parser_DOT |ANTLRv4Parser_POUND |
					    ANTLRv4Parser_NOT  => {
					        {
					        }}
						_ => Err(ANTLRError::no_alt(&mut recog.base))?
					}
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule atom*/
					recog.base.set_state(472);
					recog.atom()?;
					recog.base.set_state(475);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					    ANTLRv4Parser_QUESTION |ANTLRv4Parser_STAR |ANTLRv4Parser_PLUS  => {
					        {
					        /*InvokeRule ebnfSuffix*/
					        recog.base.set_state(473);
					        recog.ebnfSuffix()?;
					        }}
					    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF |ANTLRv4Parser_STRING_LITERAL |
					    ANTLRv4Parser_ACTION |ANTLRv4Parser_SEMI |ANTLRv4Parser_LPAREN |
					    ANTLRv4Parser_RPAREN |ANTLRv4Parser_OR |ANTLRv4Parser_DOT |ANTLRv4Parser_POUND |
					    ANTLRv4Parser_NOT  => {
					        {
					        }}
						_ => Err(ANTLRError::no_alt(&mut recog.base))?
					}
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					/*InvokeRule ebnf*/
					recog.base.set_state(477);
					recog.ebnf()?;
					}
				}
			,
				4 =>{
					/*------- Outer Most Alt 4 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
					{
					/*InvokeRule actionBlock*/
					recog.base.set_state(478);
					recog.actionBlock()?;
					recog.base.set_state(480);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ANTLRv4Parser_QUESTION {
						{
						recog.base.set_state(479);
						recog.base.match_token(ANTLRv4Parser_QUESTION,&mut recog.err_handler)?;
						}
					}

					recog.base.set_state(483);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ANTLRv4Parser_LT {
						{
						/*InvokeRule predicateOptions*/
						recog.base.set_state(482);
						recog.predicateOptions()?;
						}
					}

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- predicateOptions ----------------
pub type PredicateOptionsContextAll<'input, 'arena, Tok = CommonToken<'input>> = PredicateOptionsContext<'input, 'arena, Tok>;

pub type PredicateOptionsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredicateOptionsContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct PredicateOptionsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for PredicateOptionsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::PredicateOptionsContext }
	fn get_rule_index(&self) -> usize { RULE_predicateOptions }
    fn make_node(
        arena: &'arena Arena,
        ctx: PredicateOptionsContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a PredicateOptionsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => PredicateOptionsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut PredicateOptionsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut PredicateOptionsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicateOptionsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, PredicateOptionsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait PredicateOptionsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn predicateOption_all(&self) -> Vec<&'arena PredicateOptionContextAll<'input, 'arena, Tok>>;
    fn predicateOption(&self, i: usize) -> Option<&'arena PredicateOptionContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> PredicateOptionsContextAttrs<'input, 'arena, Tok> for PredicateOptionsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LT)
    }
    fn predicateOption_all(&self) -> Vec<&'arena PredicateOptionContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn predicateOption(&self, i: usize) -> Option<&'arena PredicateOptionContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_GT)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn predicateOptions(&mut self,) -> Result<&'arena PredicateOptionsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(PredicateOptionsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 94, RULE_predicateOptions)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena PredicateOptionsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(487);
			recog.base.match_token(ANTLRv4Parser_LT,&mut recog.err_handler)?;
			/*InvokeRule predicateOption*/
			recog.base.set_state(488);
			recog.predicateOption()?;
			recog.base.set_state(493);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_COMMA {
				{
				{
				recog.base.set_state(489);
				recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
				/*InvokeRule predicateOption*/
				recog.base.set_state(490);
				recog.predicateOption()?;
				}
				}
				recog.base.set_state(495);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(496);
			recog.base.match_token(ANTLRv4Parser_GT,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- predicateOption ----------------
pub type PredicateOptionContextAll<'input, 'arena, Tok = CommonToken<'input>> = PredicateOptionContext<'input, 'arena, Tok>;

pub type PredicateOptionContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, PredicateOptionContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct PredicateOptionContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for PredicateOptionContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::PredicateOptionContext }
	fn get_rule_index(&self) -> usize { RULE_predicateOption }
    fn make_node(
        arena: &'arena Arena,
        ctx: PredicateOptionContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a PredicateOptionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => PredicateOptionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut PredicateOptionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut PredicateOptionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> PredicateOptionContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, PredicateOptionContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait PredicateOptionContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn elementOption(&self) -> Option<&'arena ElementOptionContextAll<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> PredicateOptionContextAttrs<'input, 'arena, Tok> for PredicateOptionContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn elementOption(&self) -> Option<&'arena ElementOptionContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_ASSIGN)
    }
    fn actionBlock(&self) -> Option<&'arena ActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_INT)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn predicateOption(&mut self,) -> Result<&'arena PredicateOptionContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(PredicateOptionContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 96, RULE_predicateOption)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena PredicateOptionContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(506);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(58,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule elementOption*/
					recog.base.set_state(498);
					recog.elementOption()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule identifier*/
					recog.base.set_state(499);
					recog.identifier()?;
					recog.base.set_state(500);
					recog.base.match_token(ANTLRv4Parser_ASSIGN,&mut recog.err_handler)?;
					recog.base.set_state(504);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					    ANTLRv4Parser_ACTION  => {
					        {
					        /*InvokeRule actionBlock*/
					        recog.base.set_state(501);
					        recog.actionBlock()?;
					        }}
					    ANTLRv4Parser_INT  => {
					        {
					        recog.base.set_state(502);
					        recog.base.match_token(ANTLRv4Parser_INT,&mut recog.err_handler)?;
					        }}
					    ANTLRv4Parser_STRING_LITERAL  => {
					        {
					        recog.base.set_state(503);
					        recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
					        }}
						_ => Err(ANTLRError::no_alt(&mut recog.base))?
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- labeledElement ----------------
pub type LabeledElementContextAll<'input, 'arena, Tok = CommonToken<'input>> = LabeledElementContext<'input, 'arena, Tok>;

pub type LabeledElementContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LabeledElementContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LabeledElementContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LabeledElementContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LabeledElementContext }
	fn get_rule_index(&self) -> usize { RULE_labeledElement }
    fn make_node(
        arena: &'arena Arena,
        ctx: LabeledElementContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LabeledElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LabeledElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LabeledElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LabeledElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LabeledElementContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LabeledElementContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LabeledElementContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PLUS_ASSIGN
    /// Returns `None` if there is no child corresponding to token PLUS_ASSIGN
    fn PLUS_ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>>;
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LabeledElementContextAttrs<'input, 'arena, Tok> for LabeledElementContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_ASSIGN)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS_ASSIGN
    /// Returns `None` if there is no child corresponding to token PLUS_ASSIGN
    fn PLUS_ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PLUS_ASSIGN)
    }
    fn atom(&self) -> Option<&'arena AtomContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn labeledElement(&mut self,) -> Result<&'arena LabeledElementContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LabeledElementContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 98, RULE_labeledElement)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LabeledElementContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule identifier*/
			recog.base.set_state(508);
			recog.identifier()?;
			recog.base.set_state(509);
			_la = recog.base.input.la(1);
			if { !(_la==ANTLRv4Parser_ASSIGN || _la==ANTLRv4Parser_PLUS_ASSIGN) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
			recog.base.set_state(512);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF |ANTLRv4Parser_STRING_LITERAL |
			    ANTLRv4Parser_DOT |ANTLRv4Parser_NOT  => {
			        {
			        /*InvokeRule atom*/
			        recog.base.set_state(510);
			        recog.atom()?;
			        }}
			    ANTLRv4Parser_LPAREN  => {
			        {
			        /*InvokeRule block*/
			        recog.base.set_state(511);
			        recog.block()?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ebnf ----------------
pub type EbnfContextAll<'input, 'arena, Tok = CommonToken<'input>> = EbnfContext<'input, 'arena, Tok>;

pub type EbnfContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EbnfContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct EbnfContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for EbnfContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::EbnfContext }
	fn get_rule_index(&self) -> usize { RULE_ebnf }
    fn make_node(
        arena: &'arena Arena,
        ctx: EbnfContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a EbnfContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => EbnfContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut EbnfContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut EbnfContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EbnfContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, EbnfContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait EbnfContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>>;
    fn blockSuffix(&self) -> Option<&'arena BlockSuffixContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> EbnfContextAttrs<'input, 'arena, Tok> for EbnfContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn block(&self) -> Option<&'arena BlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn blockSuffix(&self) -> Option<&'arena BlockSuffixContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ebnf(&mut self,) -> Result<&'arena EbnfContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(EbnfContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 100, RULE_ebnf)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena EbnfContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule block*/
			recog.base.set_state(514);
			recog.block()?;
			recog.base.set_state(516);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if ((((_la - 40)) & !0x3f) == 0 && ((1usize << (_la - 40)) & 11) != 0) {
				{
				/*InvokeRule blockSuffix*/
				recog.base.set_state(515);
				recog.blockSuffix()?;
				}
			}

			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- blockSuffix ----------------
pub type BlockSuffixContextAll<'input, 'arena, Tok = CommonToken<'input>> = BlockSuffixContext<'input, 'arena, Tok>;

pub type BlockSuffixContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, BlockSuffixContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct BlockSuffixContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for BlockSuffixContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::BlockSuffixContext }
	fn get_rule_index(&self) -> usize { RULE_blockSuffix }
    fn make_node(
        arena: &'arena Arena,
        ctx: BlockSuffixContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a BlockSuffixContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => BlockSuffixContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut BlockSuffixContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut BlockSuffixContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BlockSuffixContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, BlockSuffixContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait BlockSuffixContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn ebnfSuffix(&self) -> Option<&'arena EbnfSuffixContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> BlockSuffixContextAttrs<'input, 'arena, Tok> for BlockSuffixContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn ebnfSuffix(&self) -> Option<&'arena EbnfSuffixContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn blockSuffix(&mut self,) -> Result<&'arena BlockSuffixContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(BlockSuffixContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 102, RULE_blockSuffix)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BlockSuffixContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule ebnfSuffix*/
			recog.base.set_state(518);
			recog.ebnfSuffix()?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ebnfSuffix ----------------
pub type EbnfSuffixContextAll<'input, 'arena, Tok = CommonToken<'input>> = EbnfSuffixContext<'input, 'arena, Tok>;

pub type EbnfSuffixContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, EbnfSuffixContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct EbnfSuffixContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for EbnfSuffixContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::EbnfSuffixContext }
	fn get_rule_index(&self) -> usize { RULE_ebnfSuffix }
    fn make_node(
        arena: &'arena Arena,
        ctx: EbnfSuffixContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a EbnfSuffixContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => EbnfSuffixContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut EbnfSuffixContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut EbnfSuffixContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> EbnfSuffixContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, EbnfSuffixContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait EbnfSuffixContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token QUESTION in current rule
    fn QUESTION_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token QUESTION, starting from 0.
    /// Returns `None` if number of children corresponding to token QUESTION is less than or equal to `i`.
    fn QUESTION(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> EbnfSuffixContextAttrs<'input, 'arena, Tok> for EbnfSuffixContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token QUESTION in current rule
    fn QUESTION_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_QUESTION).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token QUESTION, starting from 0.
    /// Returns `None` if number of children corresponding to token QUESTION is less than or equal to `i`.
    fn QUESTION(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_QUESTION).nth(i)
    }
    /// Retrieves first TerminalNode corresponding to token STAR
    /// Returns `None` if there is no child corresponding to token STAR
    fn STAR(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_STAR)
    }
    /// Retrieves first TerminalNode corresponding to token PLUS
    /// Returns `None` if there is no child corresponding to token PLUS
    fn PLUS(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_PLUS)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ebnfSuffix(&mut self,) -> Result<&'arena EbnfSuffixContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(EbnfSuffixContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 104, RULE_ebnfSuffix)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena EbnfSuffixContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_QUESTION  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(520);
			        recog.base.match_token(ANTLRv4Parser_QUESTION,&mut recog.err_handler)?;
			        recog.base.set_state(522);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_QUESTION {
			        	{
			        	recog.base.set_state(521);
			        	recog.base.match_token(ANTLRv4Parser_QUESTION,&mut recog.err_handler)?;
			        	}
			        }

			        }}
			    ANTLRv4Parser_STAR  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(524);
			        recog.base.match_token(ANTLRv4Parser_STAR,&mut recog.err_handler)?;
			        recog.base.set_state(526);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_QUESTION {
			        	{
			        	recog.base.set_state(525);
			        	recog.base.match_token(ANTLRv4Parser_QUESTION,&mut recog.err_handler)?;
			        	}
			        }

			        }}
			    ANTLRv4Parser_PLUS  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        recog.base.set_state(528);
			        recog.base.match_token(ANTLRv4Parser_PLUS,&mut recog.err_handler)?;
			        recog.base.set_state(530);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_QUESTION {
			        	{
			        	recog.base.set_state(529);
			        	recog.base.match_token(ANTLRv4Parser_QUESTION,&mut recog.err_handler)?;
			        	}
			        }

			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- lexerAtom ----------------
pub type LexerAtomContextAll<'input, 'arena, Tok = CommonToken<'input>> = LexerAtomContext<'input, 'arena, Tok>;

pub type LexerAtomContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, LexerAtomContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct LexerAtomContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for LexerAtomContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::LexerAtomContext }
	fn get_rule_index(&self) -> usize { RULE_lexerAtom }
    fn make_node(
        arena: &'arena Arena,
        ctx: LexerAtomContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a LexerAtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => LexerAtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut LexerAtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut LexerAtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> LexerAtomContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, LexerAtomContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait LexerAtomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn characterRange(&self) -> Option<&'arena CharacterRangeContextAll<'input, 'arena, Tok>>;
    fn terminalDef(&self) -> Option<&'arena TerminalDefContextAll<'input, 'arena, Tok>>;
    fn notSet(&self) -> Option<&'arena NotSetContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token LEXER_CHAR_SET
    /// Returns `None` if there is no child corresponding to token LEXER_CHAR_SET
    fn LEXER_CHAR_SET(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn wildcard(&self) -> Option<&'arena WildcardContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> LexerAtomContextAttrs<'input, 'arena, Tok> for LexerAtomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn characterRange(&self) -> Option<&'arena CharacterRangeContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn terminalDef(&self) -> Option<&'arena TerminalDefContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn notSet(&self) -> Option<&'arena NotSetContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LEXER_CHAR_SET
    /// Returns `None` if there is no child corresponding to token LEXER_CHAR_SET
    fn LEXER_CHAR_SET(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LEXER_CHAR_SET)
    }
    fn wildcard(&self) -> Option<&'arena WildcardContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn lexerAtom(&mut self,) -> Result<&'arena LexerAtomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(LexerAtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 106, RULE_lexerAtom)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena LexerAtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(539);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(65,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule characterRange*/
					recog.base.set_state(534);
					recog.characterRange()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule terminalDef*/
					recog.base.set_state(535);
					recog.terminalDef()?;
					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					/*InvokeRule notSet*/
					recog.base.set_state(536);
					recog.notSet()?;
					}
				}
			,
				4 =>{
					/*------- Outer Most Alt 4 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
					{
					recog.base.set_state(537);
					recog.base.match_token(ANTLRv4Parser_LEXER_CHAR_SET,&mut recog.err_handler)?;
					}
				}
			,
				5 =>{
					/*------- Outer Most Alt 5 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(5); }
					{
					/*InvokeRule wildcard*/
					recog.base.set_state(538);
					recog.wildcard()?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- atom ----------------
pub type AtomContextAll<'input, 'arena, Tok = CommonToken<'input>> = AtomContext<'input, 'arena, Tok>;

pub type AtomContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, AtomContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct AtomContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for AtomContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::AtomContext }
	fn get_rule_index(&self) -> usize { RULE_atom }
    fn make_node(
        arena: &'arena Arena,
        ctx: AtomContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a AtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => AtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut AtomContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut AtomContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> AtomContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, AtomContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait AtomContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn terminalDef(&self) -> Option<&'arena TerminalDefContextAll<'input, 'arena, Tok>>;
    fn ruleref(&self) -> Option<&'arena RulerefContextAll<'input, 'arena, Tok>>;
    fn notSet(&self) -> Option<&'arena NotSetContextAll<'input, 'arena, Tok>>;
    fn wildcard(&self) -> Option<&'arena WildcardContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> AtomContextAttrs<'input, 'arena, Tok> for AtomContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn terminalDef(&self) -> Option<&'arena TerminalDefContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ruleref(&self) -> Option<&'arena RulerefContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn notSet(&self) -> Option<&'arena NotSetContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn wildcard(&self) -> Option<&'arena WildcardContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn atom(&mut self,) -> Result<&'arena AtomContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(AtomContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 108, RULE_atom)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena AtomContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(545);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_STRING_LITERAL  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        /*InvokeRule terminalDef*/
			        recog.base.set_state(541);
			        recog.terminalDef()?;
			        }}
			    ANTLRv4Parser_RULE_REF  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        /*InvokeRule ruleref*/
			        recog.base.set_state(542);
			        recog.ruleref()?;
			        }}
			    ANTLRv4Parser_NOT  => {
			        /*------- Outer Most Alt 3 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
			        {
			        /*InvokeRule notSet*/
			        recog.base.set_state(543);
			        recog.notSet()?;
			        }}
			    ANTLRv4Parser_DOT  => {
			        /*------- Outer Most Alt 4 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
			        {
			        /*InvokeRule wildcard*/
			        recog.base.set_state(544);
			        recog.wildcard()?;
			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- wildcard ----------------
pub type WildcardContextAll<'input, 'arena, Tok = CommonToken<'input>> = WildcardContext<'input, 'arena, Tok>;

pub type WildcardContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, WildcardContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct WildcardContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for WildcardContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::WildcardContext }
	fn get_rule_index(&self) -> usize { RULE_wildcard }
    fn make_node(
        arena: &'arena Arena,
        ctx: WildcardContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a WildcardContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => WildcardContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut WildcardContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut WildcardContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> WildcardContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, WildcardContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait WildcardContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> WildcardContextAttrs<'input, 'arena, Tok> for WildcardContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token DOT
    /// Returns `None` if there is no child corresponding to token DOT
    fn DOT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_DOT)
    }
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn wildcard(&mut self,) -> Result<&'arena WildcardContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(WildcardContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 110, RULE_wildcard)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena WildcardContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(547);
			recog.base.match_token(ANTLRv4Parser_DOT,&mut recog.err_handler)?;
			recog.base.set_state(549);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_LT {
				{
				/*InvokeRule elementOptions*/
				recog.base.set_state(548);
				recog.elementOptions()?;
				}
			}

			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- notSet ----------------
pub type NotSetContextAll<'input, 'arena, Tok = CommonToken<'input>> = NotSetContext<'input, 'arena, Tok>;

pub type NotSetContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, NotSetContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct NotSetContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for NotSetContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::NotSetContext }
	fn get_rule_index(&self) -> usize { RULE_notSet }
    fn make_node(
        arena: &'arena Arena,
        ctx: NotSetContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a NotSetContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => NotSetContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut NotSetContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut NotSetContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> NotSetContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, NotSetContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait NotSetContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn setElement(&self) -> Option<&'arena SetElementContextAll<'input, 'arena, Tok>>;
    fn blockSet(&self) -> Option<&'arena BlockSetContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> NotSetContextAttrs<'input, 'arena, Tok> for NotSetContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token NOT
    /// Returns `None` if there is no child corresponding to token NOT
    fn NOT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_NOT)
    }
    fn setElement(&self) -> Option<&'arena SetElementContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn blockSet(&self) -> Option<&'arena BlockSetContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn notSet(&mut self,) -> Result<&'arena NotSetContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(NotSetContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 112, RULE_notSet)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena NotSetContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(555);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(68,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(551);
					recog.base.match_token(ANTLRv4Parser_NOT,&mut recog.err_handler)?;
					/*InvokeRule setElement*/
					recog.base.set_state(552);
					recog.setElement()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(553);
					recog.base.match_token(ANTLRv4Parser_NOT,&mut recog.err_handler)?;
					/*InvokeRule blockSet*/
					recog.base.set_state(554);
					recog.blockSet()?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- blockSet ----------------
pub type BlockSetContextAll<'input, 'arena, Tok = CommonToken<'input>> = BlockSetContext<'input, 'arena, Tok>;

pub type BlockSetContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, BlockSetContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct BlockSetContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for BlockSetContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::BlockSetContext }
	fn get_rule_index(&self) -> usize { RULE_blockSet }
    fn make_node(
        arena: &'arena Arena,
        ctx: BlockSetContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a BlockSetContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => BlockSetContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut BlockSetContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut BlockSetContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BlockSetContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, BlockSetContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait BlockSetContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn setElement_all(&self) -> Vec<&'arena SetElementContextAll<'input, 'arena, Tok>>;
    fn setElement(&self, i: usize) -> Option<&'arena SetElementContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> BlockSetContextAttrs<'input, 'arena, Tok> for BlockSetContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LPAREN)
    }
    fn setElement_all(&self) -> Vec<&'arena SetElementContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn setElement(&self, i: usize) -> Option<&'arena SetElementContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RPAREN)
    }
    /// Retrieves all `TerminalNode`s corresponding to token OR in current rule
    fn OR_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token OR, starting from 0.
    /// Returns `None` if number of children corresponding to token OR is less than or equal to `i`.
    fn OR(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_OR).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn blockSet(&mut self,) -> Result<&'arena BlockSetContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(BlockSetContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 114, RULE_blockSet)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BlockSetContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(557);
			recog.base.match_token(ANTLRv4Parser_LPAREN,&mut recog.err_handler)?;
			/*InvokeRule setElement*/
			recog.base.set_state(558);
			recog.setElement()?;
			recog.base.set_state(563);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_OR {
				{
				{
				recog.base.set_state(559);
				recog.base.match_token(ANTLRv4Parser_OR,&mut recog.err_handler)?;
				/*InvokeRule setElement*/
				recog.base.set_state(560);
				recog.setElement()?;
				}
				}
				recog.base.set_state(565);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(566);
			recog.base.match_token(ANTLRv4Parser_RPAREN,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- setElement ----------------
pub type SetElementContextAll<'input, 'arena, Tok = CommonToken<'input>> = SetElementContext<'input, 'arena, Tok>;

pub type SetElementContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, SetElementContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct SetElementContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for SetElementContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::SetElementContext }
	fn get_rule_index(&self) -> usize { RULE_setElement }
    fn make_node(
        arena: &'arena Arena,
        ctx: SetElementContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a SetElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => SetElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut SetElementContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut SetElementContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> SetElementContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, SetElementContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait SetElementContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn characterRange(&self) -> Option<&'arena CharacterRangeContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token LEXER_CHAR_SET
    /// Returns `None` if there is no child corresponding to token LEXER_CHAR_SET
    fn LEXER_CHAR_SET(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> SetElementContextAttrs<'input, 'arena, Tok> for SetElementContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_TOKEN_REF)
    }
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL)
    }
    fn characterRange(&self) -> Option<&'arena CharacterRangeContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token LEXER_CHAR_SET
    /// Returns `None` if there is no child corresponding to token LEXER_CHAR_SET
    fn LEXER_CHAR_SET(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LEXER_CHAR_SET)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn setElement(&mut self,) -> Result<&'arena SetElementContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(SetElementContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 116, RULE_setElement)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena SetElementContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(578);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(72,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					recog.base.set_state(568);
					recog.base.match_token(ANTLRv4Parser_TOKEN_REF,&mut recog.err_handler)?;
					recog.base.set_state(570);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ANTLRv4Parser_LT {
						{
						/*InvokeRule elementOptions*/
						recog.base.set_state(569);
						recog.elementOptions()?;
						}
					}

					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					recog.base.set_state(572);
					recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
					recog.base.set_state(574);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ANTLRv4Parser_LT {
						{
						/*InvokeRule elementOptions*/
						recog.base.set_state(573);
						recog.elementOptions()?;
						}
					}

					}
				}
			,
				3 =>{
					/*------- Outer Most Alt 3 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(3); }
					{
					/*InvokeRule characterRange*/
					recog.base.set_state(576);
					recog.characterRange()?;
					}
				}
			,
				4 =>{
					/*------- Outer Most Alt 4 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(4); }
					{
					recog.base.set_state(577);
					recog.base.match_token(ANTLRv4Parser_LEXER_CHAR_SET,&mut recog.err_handler)?;
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input, 'arena, Tok = CommonToken<'input>> = BlockContext<'input, 'arena, Tok>;

pub type BlockContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, BlockContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct BlockContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for BlockContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::BlockContext }
	fn get_rule_index(&self) -> usize { RULE_block }
    fn make_node(
        arena: &'arena Arena,
        ctx: BlockContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a BlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => BlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut BlockContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut BlockContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> BlockContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, BlockContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait BlockContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn altList(&self) -> Option<&'arena AltListContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>>;
    fn ruleAction_all(&self) -> Vec<&'arena RuleActionContextAll<'input, 'arena, Tok>>;
    fn ruleAction(&self, i: usize) -> Option<&'arena RuleActionContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> BlockContextAttrs<'input, 'arena, Tok> for BlockContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LPAREN)
    }
    fn altList(&self) -> Option<&'arena AltListContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RPAREN)
    }
    /// Retrieves first TerminalNode corresponding to token COLON
    /// Returns `None` if there is no child corresponding to token COLON
    fn COLON(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_COLON)
    }
    fn optionsSpec(&self) -> Option<&'arena OptionsSpecContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn ruleAction_all(&self) -> Vec<&'arena RuleActionContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn ruleAction(&self, i: usize) -> Option<&'arena RuleActionContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn block(&mut self,) -> Result<&'arena BlockContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(BlockContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 118, RULE_block)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena BlockContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(580);
			recog.base.match_token(ANTLRv4Parser_LPAREN,&mut recog.err_handler)?;
			recog.base.set_state(591);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_OPTIONS || _la==ANTLRv4Parser_COLON || _la==ANTLRv4Parser_AT {
				{
				recog.base.set_state(582);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if _la==ANTLRv4Parser_OPTIONS {
					{
					/*InvokeRule optionsSpec*/
					recog.base.set_state(581);
					recog.optionsSpec()?;
					}
				}

				recog.base.set_state(587);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==ANTLRv4Parser_AT {
					{
					{
					/*InvokeRule ruleAction*/
					recog.base.set_state(584);
					recog.ruleAction()?;
					}
					}
					recog.base.set_state(589);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(590);
				recog.base.match_token(ANTLRv4Parser_COLON,&mut recog.err_handler)?;
				}
			}

			/*InvokeRule altList*/
			recog.base.set_state(593);
			recog.altList()?;
			recog.base.set_state(594);
			recog.base.match_token(ANTLRv4Parser_RPAREN,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- ruleref ----------------
pub type RulerefContextAll<'input, 'arena, Tok = CommonToken<'input>> = RulerefContext<'input, 'arena, Tok>;

pub type RulerefContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, RulerefContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct RulerefContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for RulerefContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::RulerefContext }
	fn get_rule_index(&self) -> usize { RULE_ruleref }
    fn make_node(
        arena: &'arena Arena,
        ctx: RulerefContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a RulerefContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => RulerefContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut RulerefContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut RulerefContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> RulerefContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, RulerefContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait RulerefContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token RULE_REF
    /// Returns `None` if there is no child corresponding to token RULE_REF
    fn RULE_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>>;
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> RulerefContextAttrs<'input, 'arena, Tok> for RulerefContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token RULE_REF
    /// Returns `None` if there is no child corresponding to token RULE_REF
    fn RULE_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RULE_REF)
    }
    fn argActionBlock(&self) -> Option<&'arena ArgActionBlockContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn ruleref(&mut self,) -> Result<&'arena RulerefContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(RulerefContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 120, RULE_ruleref)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena RulerefContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(596);
			recog.base.match_token(ANTLRv4Parser_RULE_REF,&mut recog.err_handler)?;
			recog.base.set_state(598);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_BEGIN_ARGUMENT {
				{
				/*InvokeRule argActionBlock*/
				recog.base.set_state(597);
				recog.argActionBlock()?;
				}
			}

			recog.base.set_state(601);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ANTLRv4Parser_LT {
				{
				/*InvokeRule elementOptions*/
				recog.base.set_state(600);
				recog.elementOptions()?;
				}
			}

			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- characterRange ----------------
pub type CharacterRangeContextAll<'input, 'arena, Tok = CommonToken<'input>> = CharacterRangeContext<'input, 'arena, Tok>;

pub type CharacterRangeContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, CharacterRangeContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct CharacterRangeContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for CharacterRangeContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::CharacterRangeContext }
	fn get_rule_index(&self) -> usize { RULE_characterRange }
    fn make_node(
        arena: &'arena Arena,
        ctx: CharacterRangeContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a CharacterRangeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => CharacterRangeContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut CharacterRangeContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut CharacterRangeContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> CharacterRangeContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, CharacterRangeContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait CharacterRangeContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves all `TerminalNode`s corresponding to token STRING_LITERAL in current rule
    fn STRING_LITERAL_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token STRING_LITERAL, starting from 0.
    /// Returns `None` if number of children corresponding to token STRING_LITERAL is less than or equal to `i`.
    fn STRING_LITERAL(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token RANGE
    /// Returns `None` if there is no child corresponding to token RANGE
    fn RANGE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> CharacterRangeContextAttrs<'input, 'arena, Tok> for CharacterRangeContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves all `TerminalNode`s corresponding to token STRING_LITERAL in current rule
    fn STRING_LITERAL_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token STRING_LITERAL, starting from 0.
    /// Returns `None` if number of children corresponding to token STRING_LITERAL is less than or equal to `i`.
    fn STRING_LITERAL(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL).nth(i)
    }
    /// Retrieves first TerminalNode corresponding to token RANGE
    /// Returns `None` if there is no child corresponding to token RANGE
    fn RANGE(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RANGE)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn characterRange(&mut self,) -> Result<&'arena CharacterRangeContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(CharacterRangeContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 122, RULE_characterRange)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena CharacterRangeContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(603);
			recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
			recog.base.set_state(604);
			recog.base.match_token(ANTLRv4Parser_RANGE,&mut recog.err_handler)?;
			recog.base.set_state(605);
			recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- terminalDef ----------------
pub type TerminalDefContextAll<'input, 'arena, Tok = CommonToken<'input>> = TerminalDefContext<'input, 'arena, Tok>;

pub type TerminalDefContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, TerminalDefContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct TerminalDefContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for TerminalDefContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::TerminalDefContext }
	fn get_rule_index(&self) -> usize { RULE_terminalDef }
    fn make_node(
        arena: &'arena Arena,
        ctx: TerminalDefContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a TerminalDefContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => TerminalDefContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut TerminalDefContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut TerminalDefContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> TerminalDefContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, TerminalDefContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait TerminalDefContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> TerminalDefContextAttrs<'input, 'arena, Tok> for TerminalDefContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_TOKEN_REF)
    }
    fn elementOptions(&self) -> Option<&'arena ElementOptionsContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn terminalDef(&mut self,) -> Result<&'arena TerminalDefContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(TerminalDefContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 124, RULE_terminalDef)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena TerminalDefContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(615);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			    ANTLRv4Parser_TOKEN_REF  => {
			        /*------- Outer Most Alt 1 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			        {
			        recog.base.set_state(607);
			        recog.base.match_token(ANTLRv4Parser_TOKEN_REF,&mut recog.err_handler)?;
			        recog.base.set_state(609);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_LT {
			        	{
			        	/*InvokeRule elementOptions*/
			        	recog.base.set_state(608);
			        	recog.elementOptions()?;
			        	}
			        }

			        }}
			    ANTLRv4Parser_STRING_LITERAL  => {
			        /*------- Outer Most Alt 2 -------*/
			        unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
			        {
			        recog.base.set_state(611);
			        recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
			        recog.base.set_state(613);
			        recog.err_handler.sync(&mut recog.base)?;
			        _la = recog.base.input.la(1);
			        if _la==ANTLRv4Parser_LT {
			        	{
			        	/*InvokeRule elementOptions*/
			        	recog.base.set_state(612);
			        	recog.elementOptions()?;
			        	}
			        }

			        }}
				_ => Err(ANTLRError::no_alt(&mut recog.base))?
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- elementOptions ----------------
pub type ElementOptionsContextAll<'input, 'arena, Tok = CommonToken<'input>> = ElementOptionsContext<'input, 'arena, Tok>;

pub type ElementOptionsContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ElementOptionsContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ElementOptionsContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ElementOptionsContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ElementOptionsContext }
	fn get_rule_index(&self) -> usize { RULE_elementOptions }
    fn make_node(
        arena: &'arena Arena,
        ctx: ElementOptionsContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ElementOptionsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ElementOptionsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ElementOptionsContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ElementOptionsContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ElementOptionsContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ElementOptionsContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ElementOptionsContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    fn elementOption_all(&self) -> Vec<&'arena ElementOptionContextAll<'input, 'arena, Tok>>;
    fn elementOption(&self, i: usize) -> Option<&'arena ElementOptionContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ElementOptionsContextAttrs<'input, 'arena, Tok> for ElementOptionsContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token LT
    /// Returns `None` if there is no child corresponding to token LT
    fn LT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_LT)
    }
    fn elementOption_all(&self) -> Vec<&'arena ElementOptionContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn elementOption(&self, i: usize) -> Option<&'arena ElementOptionContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token GT
    /// Returns `None` if there is no child corresponding to token GT
    fn GT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_GT)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less than or equal to `i`.
    fn COMMA(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_COMMA).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn elementOptions(&mut self,) -> Result<&'arena ElementOptionsContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ElementOptionsContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 126, RULE_elementOptions)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ElementOptionsContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(617);
			recog.base.match_token(ANTLRv4Parser_LT,&mut recog.err_handler)?;
			/*InvokeRule elementOption*/
			recog.base.set_state(618);
			recog.elementOption()?;
			recog.base.set_state(623);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_COMMA {
				{
				{
				recog.base.set_state(619);
				recog.base.match_token(ANTLRv4Parser_COMMA,&mut recog.err_handler)?;
				/*InvokeRule elementOption*/
				recog.base.set_state(620);
				recog.elementOption()?;
				}
				}
				recog.base.set_state(625);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(626);
			recog.base.match_token(ANTLRv4Parser_GT,&mut recog.err_handler)?;
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- elementOption ----------------
pub type ElementOptionContextAll<'input, 'arena, Tok = CommonToken<'input>> = ElementOptionContext<'input, 'arena, Tok>;

pub type ElementOptionContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, ElementOptionContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct ElementOptionContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for ElementOptionContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::ElementOptionContext }
	fn get_rule_index(&self) -> usize { RULE_elementOption }
    fn make_node(
        arena: &'arena Arena,
        ctx: ElementOptionContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a ElementOptionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => ElementOptionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut ElementOptionContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut ElementOptionContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> ElementOptionContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, ElementOptionContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait ElementOptionContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn qualifiedIdentifier(&self) -> Option<&'arena QualifiedIdentifierContextAll<'input, 'arena, Tok>>;
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> ElementOptionContextAttrs<'input, 'arena, Tok> for ElementOptionContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn qualifiedIdentifier(&self) -> Option<&'arena QualifiedIdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    fn identifier(&self) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token ASSIGN
    /// Returns `None` if there is no child corresponding to token ASSIGN
    fn ASSIGN(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_ASSIGN)
    }
    /// Retrieves first TerminalNode corresponding to token STRING_LITERAL
    /// Returns `None` if there is no child corresponding to token STRING_LITERAL
    fn STRING_LITERAL(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_STRING_LITERAL)
    }
    /// Retrieves first TerminalNode corresponding to token INT
    /// Returns `None` if there is no child corresponding to token INT
    fn INT(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_INT)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn elementOption(&mut self,) -> Result<&'arena ElementOptionContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(ElementOptionContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 128, RULE_elementOption)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena ElementOptionContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let result: Result<(), ANTLRError> = (|| {
			recog.base.set_state(636);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.get_interpreter().adaptive_predict(83,&mut recog.base)? {
				1 =>{
					/*------- Outer Most Alt 1 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
					{
					/*InvokeRule qualifiedIdentifier*/
					recog.base.set_state(628);
					recog.qualifiedIdentifier()?;
					}
				}
			,
				2 =>{
					/*------- Outer Most Alt 2 -------*/
					unsafe { recog.ctx_mut().unwrap().set_alt_number(2); }
					{
					/*InvokeRule identifier*/
					recog.base.set_state(629);
					recog.identifier()?;
					recog.base.set_state(630);
					recog.base.match_token(ANTLRv4Parser_ASSIGN,&mut recog.err_handler)?;
					recog.base.set_state(634);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					    ANTLRv4Parser_TOKEN_REF |ANTLRv4Parser_RULE_REF  => {
					        {
					        /*InvokeRule qualifiedIdentifier*/
					        recog.base.set_state(631);
					        recog.qualifiedIdentifier()?;
					        }}
					    ANTLRv4Parser_STRING_LITERAL  => {
					        {
					        recog.base.set_state(632);
					        recog.base.match_token(ANTLRv4Parser_STRING_LITERAL,&mut recog.err_handler)?;
					        }}
					    ANTLRv4Parser_INT  => {
					        {
					        recog.base.set_state(633);
					        recog.base.match_token(ANTLRv4Parser_INT,&mut recog.err_handler)?;
					        }}
						_ => Err(ANTLRError::no_alt(&mut recog.base))?
					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- identifier ----------------
pub type IdentifierContextAll<'input, 'arena, Tok = CommonToken<'input>> = IdentifierContext<'input, 'arena, Tok>;

pub type IdentifierContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, IdentifierContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct IdentifierContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for IdentifierContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::IdentifierContext }
	fn get_rule_index(&self) -> usize { RULE_identifier }
    fn make_node(
        arena: &'arena Arena,
        ctx: IdentifierContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a IdentifierContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => IdentifierContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut IdentifierContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut IdentifierContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> IdentifierContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, IdentifierContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait IdentifierContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    /// Retrieves first TerminalNode corresponding to token RULE_REF
    /// Returns `None` if there is no child corresponding to token RULE_REF
    fn RULE_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> IdentifierContextAttrs<'input, 'arena, Tok> for IdentifierContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    /// Retrieves first TerminalNode corresponding to token RULE_REF
    /// Returns `None` if there is no child corresponding to token RULE_REF
    fn RULE_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_RULE_REF)
    }
    /// Retrieves first TerminalNode corresponding to token TOKEN_REF
    /// Returns `None` if there is no child corresponding to token TOKEN_REF
    fn TOKEN_REF(&self) -> Option<&TerminalNode<'input, 'arena, Tok>> {
        self.children_of_type::<TerminalNode<Tok>>().into_iter().find(|child| child.symbol.get_token_type() == ANTLRv4Parser_TOKEN_REF)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn identifier(&mut self,) -> Result<&'arena IdentifierContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(IdentifierContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 130, RULE_identifier)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena IdentifierContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			recog.base.set_state(638);
			_la = recog.base.input.la(1);
			if { !(_la==ANTLRv4Parser_TOKEN_REF || _la==ANTLRv4Parser_RULE_REF) } {
				recog.err_handler.recover_inline(&mut recog.base)?;
			}
			else {
				if recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler)?;
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}
//------------------- qualifiedIdentifier ----------------
pub type QualifiedIdentifierContextAll<'input, 'arena, Tok = CommonToken<'input>> = QualifiedIdentifierContext<'input, 'arena, Tok>;

pub type QualifiedIdentifierContext<'input, 'arena, Tok = CommonToken<'input>> = BaseParserRuleContext<'input, 'arena, QualifiedIdentifierContextExt<'input, 'arena, Tok>, ANTLRv4ParserNodeKind, Tok>;
#[derive(Debug)]
pub struct QualifiedIdentifierContextExt<'input: 'arena, 'arena, Tok: Token + 'input = CommonToken<'input>> {
    ph: PhantomData<(&'arena (), &'input Tok)>,
}

impl<'input: 'arena, 'arena, Tok> CustomRuleContext<'input, 'arena, Tok> for QualifiedIdentifierContextExt<'input, 'arena, Tok>
where
    Tok: Token + 'input,
{
	type NodeKind = ANTLRv4ParserNodeKind;
    fn node_tag() -> ANTLRv4ParserNodeKind { ANTLRv4ParserNodeKind::QualifiedIdentifierContext }
	fn get_rule_index(&self) -> usize { RULE_qualifiedIdentifier }
    fn make_node(
        arena: &'arena Arena,
        ctx: QualifiedIdentifierContext<'input, 'arena, Tok>,
    ) -> *mut ANTLRv4ParserNode<'input, 'arena, Tok> {
        arena.alloc_zeroed_node(ctx)}
    fn cast_from<'a>(
        node: &'a ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a QualifiedIdentifierContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => QualifiedIdentifierContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
    fn cast_from_mut<'a>(
        node: &'a mut ANTLRv4ParserNode<'input, 'arena, Tok>,
    ) -> Option<&'a mut QualifiedIdentifierContext<'input, 'arena, Tok>> {
        if node.node_tag() == <Self as CustomRuleContext<'input, 'arena, Tok>>::node_tag() {
            Some(dbt_antlr_runtime::cast_unchecked!(node.ctx_ptr() => mut QualifiedIdentifierContext<'input, 'arena, Tok>))
        } else {
            None
        }
    }
}

impl<'input: 'arena, 'arena, Tok: Token + 'input> QualifiedIdentifierContextExt<'input, 'arena, Tok>{
	fn create(arena: &'arena Arena, parent: Option<&'arena ANTLRv4ParserNode<'input, 'arena, Tok>>, invoking_state: i32) -> Result<&'arena mut ANTLRv4ParserNode<'input, 'arena, Tok>, ANTLRError>
    {
        BaseParserRuleContext::create(arena, parent, invoking_state, QualifiedIdentifierContextExt {
				ph: PhantomData
			}
		)
	}
}

pub trait QualifiedIdentifierContextAttrs<'input, 'arena, Tok>: ParserRuleContext<'input, 'arena>
where
    'input: 'arena,
    Tok: Token + 'input,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>>;
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>>;
    /// Retrieves 'i'th TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less than or equal to `i`.
    fn DOT(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>>;
}

impl<'input, 'arena, Tok: Token + 'input> QualifiedIdentifierContextAttrs<'input, 'arena, Tok> for QualifiedIdentifierContext<'input, 'arena, Tok>
where
    'input: 'arena,
{
    fn identifier_all(&self) -> Vec<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.children_of_type()
    }
    fn identifier(&self, i: usize) -> Option<&'arena IdentifierContextAll<'input, 'arena, Tok>> {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token DOT in current rule
    fn DOT_all(&self) -> Vec<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_DOT).collect()
    }
    /// Retrieves 'i'th TerminalNode corresponding to token DOT, starting from 0.
    /// Returns `None` if number of children corresponding to token DOT is less than or equal to `i`.
    fn DOT(&self, i: usize) -> Option<&TerminalNode<'input, 'arena, Tok>> {
    	self.children_of_type::<TerminalNode<Tok>>().into_iter().filter(|child| child.symbol.get_token_type() == ANTLRv4Parser_DOT).nth(i)
    }
}

impl<'input, 'arena, Input, TF> ANTLRv4Parser<'input, 'arena, Input, TF>
where
    'input: 'arena,
    TF: TokenFactory<'input, 'arena> + 'arena,
    Input: TokenStream<'input, 'arena, TF> + 'arena,
{
	pub fn qualifiedIdentifier(&mut self,) -> Result<&'arena QualifiedIdentifierContextAll<'input, 'arena, TF::Tok>, ANTLRError> {
        dbt_antlr_runtime::maybe_grow_stack!({
		let recog = self;
        let _parentctx = recog.base.take_ctx();
        recog.base.enter_rule(QualifiedIdentifierContextExt::create(recog.get_arena(), _parentctx, recog.get_state())?, 132, RULE_qualifiedIdentifier)?;
        let _local_ctx_fn = |recog: &Self| -> &'arena QualifiedIdentifierContext<TF::Tok> {recog.ctx().unwrap().as_rule_context().unwrap()};
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			/*------- Outer Most Alt 1 -------*/
			unsafe { recog.ctx_mut().unwrap().set_alt_number(1); }
			{
			/*InvokeRule identifier*/
			recog.base.set_state(640);
			recog.identifier()?;
			recog.base.set_state(645);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==ANTLRv4Parser_DOT {
				{
				{
				recog.base.set_state(641);
				recog.base.match_token(ANTLRv4Parser_DOT,&mut recog.err_handler)?;
				/*InvokeRule identifier*/
				recog.base.set_state(642);
				recog.identifier()?;
				}
				}
				recog.base.set_state(647);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			}
			Ok(())
		})();
		match result {
            Ok(_)=>{},
            Err(e) if !e.is_recoverable() => return Err(e),
            Err(ref re) => {
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule().map(|ctx: &'arena _| { ctx.as_rule_context().unwrap() })
        })
	}
}

static ATN_SIMULATOR_MANAGER: LazyLock<ATNSimulatorManager> = LazyLock::new(|| ATNSimulatorManager::new(&_ATN));
static _ATN: LazyLock<ATN> = LazyLock::new(|| {
    PackedATNDeserializer::new()
        .deserialize(&_serializedATN)
        .expect("invalid packed parser ATN")
});
static _serializedATN: LazyLock<Vec<u32>> = LazyLock::new(|| vec![
    1346458702, 3, 16909060, 29, 56, 649, 821, 3, 5, 85, 67, 29, 4543, 4572, 
    4105, 8677, 15, 8692, 10, 8714, 85, 8799, 67, 8866, 67, 8933, 6, 8702, 
    12, 2, 0, 8, 0, 1, 4294967295, 4294967295, 7, 0, 16, 1, 0, 4294967295, 
    4294967295, 2, 1, 8, 1, 1, 4294967295, 4294967295, 7, 1, 24, 2, 1, 4294967295, 
    4294967295, 2, 2, 8, 3, 1, 4294967295, 4294967295, 7, 2, 24, 4, 1, 4294967295, 
    4294967295, 2, 3, 8, 5, 1, 4294967295, 4294967295, 7, 3, 24, 6, 1, 4294967295, 
    4294967295, 2, 4, 8, 7, 1, 4294967295, 4294967295, 7, 4, 24, 8, 4, 4294967295, 
    4294967295, 2, 5, 8, 12, 1, 4294967295, 4294967295, 7, 5, 24, 13, 1, 
    4294967295, 4294967295, 2, 6, 8, 14, 1, 4294967295, 4294967295, 7, 6, 
    24, 15, 1, 4294967295, 4294967295, 2, 7, 8, 16, 1, 4294967295, 4294967295, 
    7, 7, 24, 17, 1, 4294967295, 4294967295, 2, 8, 8, 18, 1, 4294967295, 
    4294967295, 7, 8, 24, 19, 2, 4294967295, 4294967295, 2, 9, 8, 21, 1, 
    4294967295, 4294967295, 7, 9, 24, 22, 1, 4294967295, 4294967295, 2, 
    10, 8, 23, 1, 4294967295, 4294967295, 7, 10, 24, 24, 1, 4294967295, 
    4294967295, 2, 11, 8, 25, 1, 4294967295, 4294967295, 7, 11, 24, 26, 
    2, 4294967295, 4294967295, 2, 12, 8, 28, 1, 4294967295, 4294967295, 
    7, 12, 24, 29, 1, 4294967295, 4294967295, 2, 13, 8, 30, 1, 4294967295, 
    4294967295, 7, 13, 24, 31, 1, 4294967295, 4294967295, 2, 14, 8, 32, 
    1, 4294967295, 4294967295, 7, 14, 24, 33, 8, 4294967295, 4294967295, 
    2, 15, 8, 41, 1, 4294967295, 4294967295, 7, 15, 24, 42, 5, 4294967295, 
    4294967295, 2, 16, 8, 47, 1, 4294967295, 4294967295, 7, 16, 24, 48, 
    1, 4294967295, 4294967295, 2, 17, 8, 49, 1, 4294967295, 4294967295, 
    7, 17, 24, 50, 1, 4294967295, 4294967295, 2, 18, 8, 51, 1, 4294967295, 
    4294967295, 7, 18, 24, 52, 1, 4294967295, 4294967295, 2, 19, 8, 53, 
    1, 4294967295, 4294967295, 7, 19, 24, 54, 1, 4294967295, 4294967295, 
    2, 20, 8, 55, 1, 4294967295, 4294967295, 7, 20, 24, 56, 1, 4294967295, 
    4294967295, 2, 21, 8, 57, 1, 4294967295, 4294967295, 7, 21, 24, 58, 
    1, 4294967295, 4294967295, 2, 22, 8, 59, 1, 4294967295, 4294967295, 
    7, 22, 24, 60, 1, 4294967295, 4294967295, 2, 23, 8, 61, 1, 4294967295, 
    4294967295, 7, 23, 24, 62, 1, 4294967295, 4294967295, 2, 24, 8, 63, 
    1, 4294967295, 4294967295, 7, 24, 24, 64, 1, 4294967295, 4294967295, 
    2, 25, 8, 65, 1, 4294967295, 4294967295, 7, 25, 24, 66, 1, 4294967295, 
    4294967295, 2, 26, 8, 67, 1, 4294967295, 4294967295, 7, 26, 24, 68, 
    1, 4294967295, 4294967295, 2, 27, 8, 69, 1, 4294967295, 4294967295, 
    7, 27, 24, 70, 2, 4294967295, 4294967295, 2, 28, 8, 72, 1, 4294967295, 
    4294967295, 7, 28, 24, 73, 1, 4294967295, 4294967295, 2, 29, 8, 74, 
    1, 4294967295, 4294967295, 7, 29, 24, 75, 1, 4294967295, 4294967295, 
    2, 30, 8, 76, 1, 4294967295, 4294967295, 7, 30, 24, 77, 1, 4294967295, 
    4294967295, 2, 31, 8, 78, 1, 4294967295, 4294967295, 7, 31, 24, 79, 
    1, 4294967295, 4294967295, 2, 32, 8, 80, 1, 4294967295, 4294967295, 
    7, 32, 24, 81, 2, 4294967295, 4294967295, 2, 33, 8, 83, 1, 4294967295, 
    4294967295, 7, 33, 24, 84, 2, 4294967295, 4294967295, 2, 34, 8, 86, 
    1, 4294967295, 4294967295, 7, 34, 24, 87, 1, 4294967295, 4294967295, 
    2, 35, 8, 88, 1, 4294967295, 4294967295, 7, 35, 24, 89, 2, 4294967295, 
    4294967295, 2, 36, 8, 91, 1, 4294967295, 4294967295, 7, 36, 24, 92, 
    2, 4294967295, 4294967295, 2, 37, 8, 94, 1, 4294967295, 4294967295, 
    7, 37, 24, 95, 1, 4294967295, 4294967295, 2, 38, 8, 96, 1, 4294967295, 
    4294967295, 7, 38, 24, 97, 1, 4294967295, 4294967295, 2, 39, 8, 98, 
    1, 4294967295, 4294967295, 7, 39, 24, 99, 1, 4294967295, 4294967295, 
    2, 40, 8, 100, 1, 4294967295, 4294967295, 7, 40, 24, 101, 1, 4294967295, 
    4294967295, 2, 41, 8, 102, 1, 4294967295, 4294967295, 7, 41, 24, 103, 
    2, 4294967295, 4294967295, 2, 42, 8, 105, 1, 4294967295, 4294967295, 
    7, 42, 24, 106, 2, 4294967295, 4294967295, 2, 43, 8, 108, 1, 4294967295, 
    4294967295, 7, 43, 24, 109, 1, 4294967295, 4294967295, 2, 44, 8, 110, 
    1, 4294967295, 4294967295, 7, 44, 24, 111, 1, 4294967295, 4294967295, 
    2, 45, 8, 112, 1, 4294967295, 4294967295, 7, 45, 24, 113, 3, 4294967295, 
    4294967295, 2, 46, 8, 116, 1, 4294967295, 4294967295, 7, 46, 24, 117, 
    1, 4294967295, 4294967295, 2, 47, 8, 118, 1, 4294967295, 4294967295, 
    7, 47, 24, 119, 1, 4294967295, 4294967295, 2, 48, 8, 120, 1, 4294967295, 
    4294967295, 7, 48, 24, 121, 2, 4294967295, 4294967295, 2, 49, 8, 123, 
    1, 4294967295, 4294967295, 7, 49, 24, 124, 1, 4294967295, 4294967295, 
    2, 50, 8, 125, 1, 4294967295, 4294967295, 7, 50, 24, 126, 1, 4294967295, 
    4294967295, 2, 51, 8, 127, 1, 4294967295, 4294967295, 7, 51, 24, 128, 
    1, 4294967295, 4294967295, 2, 52, 8, 129, 1, 4294967295, 4294967295, 
    7, 52, 24, 130, 5, 4294967295, 4294967295, 2, 53, 8, 135, 1, 4294967295, 
    4294967295, 7, 53, 24, 136, 1, 4294967295, 4294967295, 2, 54, 8, 137, 
    1, 4294967295, 4294967295, 7, 54, 24, 138, 2, 4294967295, 4294967295, 
    2, 55, 8, 140, 1, 4294967295, 4294967295, 7, 55, 24, 141, 2, 4294967295, 
    4294967295, 2, 56, 8, 143, 1, 4294967295, 4294967295, 7, 56, 24, 144, 
    2, 4294967295, 4294967295, 2, 57, 8, 146, 1, 4294967295, 4294967295, 
    7, 57, 24, 147, 1, 4294967295, 4294967295, 2, 58, 8, 148, 1, 4294967295, 
    4294967295, 7, 58, 24, 149, 3, 4294967295, 4294967295, 2, 59, 8, 152, 
    1, 4294967295, 4294967295, 7, 59, 24, 153, 2, 4294967295, 4294967295, 
    2, 60, 8, 155, 1, 4294967295, 4294967295, 7, 60, 24, 156, 1, 4294967295, 
    4294967295, 2, 61, 8, 157, 1, 4294967295, 4294967295, 7, 61, 24, 158, 
    2, 4294967295, 4294967295, 2, 62, 8, 160, 1, 4294967295, 4294967295, 
    7, 62, 24, 161, 2, 4294967295, 4294967295, 2, 63, 8, 163, 1, 4294967295, 
    4294967295, 7, 63, 24, 164, 7, 4294967295, 4294967295, 2, 64, 8, 171, 
    1, 4294967295, 4294967295, 7, 64, 24, 172, 3, 4294967295, 4294967295, 
    2, 65, 8, 175, 1, 4294967295, 4294967295, 7, 65, 24, 176, 21, 4294967295, 
    4294967295, 2, 66, 8, 197, 1, 4294967295, 4294967295, 7, 66, 24, 198, 
    4, 4294967295, 4294967295, 1, 0, 8, 202, 1, 4294967295, 4294967295, 
    1, 0, 8, 203, 1, 4294967295, 4294967295, 5, 0, 8, 204, 1, 137, 4294967295, 
    8, 0, 8, 205, 1, 4294967295, 4294967295, 10, 0, 8, 206, 2, 4294967295, 
    4294967295, 12, 0, 8, 208, 1, 4294967295, 140, 9, 0, 8, 209, 1, 4294967295, 
    4294967295, 1, 0, 8, 210, 1, 4294967295, 4294967295, 1, 0, 8, 211, 1, 
    4294967295, 4294967295, 5, 0, 8, 212, 1, 144, 4294967295, 8, 0, 8, 213, 
    1, 4294967295, 4294967295, 10, 0, 8, 214, 2, 4294967295, 4294967295, 
    12, 0, 8, 216, 1, 4294967295, 147, 9, 0, 8, 217, 1, 4294967295, 4294967295, 
    1, 0, 32, 218, 1, 4294967295, 4294967295, 1, 0, 8, 219, 1, 4294967295, 
    4294967295, 1, 1, 8, 220, 1, 4294967295, 4294967295, 1, 1, 8, 221, 1, 
    4294967295, 4294967295, 1, 1, 32, 222, 1, 4294967295, 4294967295, 1, 
    1, 8, 223, 1, 4294967295, 4294967295, 1, 2, 32, 224, 1, 4294967295, 
    4294967295, 1, 2, 32, 225, 1, 4294967295, 4294967295, 1, 2, 32, 226, 
    1, 4294967295, 4294967295, 1, 2, 32, 227, 1, 4294967295, 4294967295, 
    1, 2, 32, 228, 1, 4294967295, 4294967295, 3, 2, 8, 229, 3, 160, 4294967295, 
    8, 2, 8, 232, 1, 4294967295, 4294967295, 1, 3, 8, 233, 1, 4294967295, 
    4294967295, 1, 3, 8, 234, 1, 4294967295, 4294967295, 1, 3, 8, 235, 1, 
    4294967295, 4294967295, 1, 3, 8, 236, 1, 4294967295, 4294967295, 1, 
    3, 8, 237, 1, 4294967295, 4294967295, 3, 3, 8, 238, 5, 167, 4294967295, 
    8, 3, 8, 243, 1, 4294967295, 4294967295, 1, 4, 32, 244, 1, 4294967295, 
    4294967295, 1, 4, 8, 245, 1, 4294967295, 4294967295, 1, 4, 32, 246, 
    1, 4294967295, 4294967295, 1, 4, 8, 247, 1, 4294967295, 4294967295, 
    5, 4, 8, 248, 1, 173, 4294967295, 8, 4, 8, 249, 1, 4294967295, 4294967295, 
    10, 4, 8, 250, 2, 4294967295, 4294967295, 12, 4, 8, 252, 1, 4294967295, 
    176, 9, 4, 8, 253, 1, 4294967295, 4294967295, 1, 4, 32, 254, 1, 4294967295, 
    4294967295, 1, 4, 8, 255, 1, 4294967295, 4294967295, 1, 5, 8, 256, 1, 
    4294967295, 4294967295, 1, 5, 32, 257, 1, 4294967295, 4294967295, 1, 
    5, 8, 258, 1, 4294967295, 4294967295, 1, 5, 8, 259, 1, 4294967295, 4294967295, 
    1, 6, 8, 260, 1, 4294967295, 4294967295, 1, 6, 32, 261, 1, 4294967295, 
    4294967295, 1, 6, 8, 262, 1, 4294967295, 4294967295, 5, 6, 8, 263, 1, 
    187, 4294967295, 8, 6, 8, 264, 1, 4294967295, 4294967295, 10, 6, 8, 
    265, 2, 4294967295, 4294967295, 12, 6, 8, 267, 1, 4294967295, 190, 9, 
    6, 8, 268, 1, 4294967295, 4294967295, 1, 6, 32, 269, 1, 4294967295, 
    4294967295, 1, 6, 8, 270, 1, 4294967295, 4294967295, 1, 6, 32, 271, 
    1, 4294967295, 4294967295, 3, 6, 8, 272, 4, 195, 4294967295, 8, 6, 8, 
    276, 1, 4294967295, 4294967295, 1, 7, 32, 277, 1, 4294967295, 4294967295, 
    1, 7, 8, 278, 1, 4294967295, 4294967295, 1, 7, 32, 279, 1, 4294967295, 
    4294967295, 1, 7, 8, 280, 1, 4294967295, 4294967295, 5, 7, 8, 281, 1, 
    201, 4294967295, 8, 7, 8, 282, 1, 4294967295, 4294967295, 10, 7, 8, 
    283, 2, 4294967295, 4294967295, 12, 7, 8, 285, 1, 4294967295, 204, 9, 
    7, 8, 286, 1, 4294967295, 4294967295, 1, 7, 32, 287, 1, 4294967295, 
    4294967295, 1, 7, 8, 288, 1, 4294967295, 4294967295, 1, 8, 8, 289, 1, 
    4294967295, 4294967295, 1, 8, 32, 290, 1, 4294967295, 4294967295, 1, 
    8, 8, 291, 1, 4294967295, 4294967295, 1, 8, 8, 292, 1, 4294967295, 4294967295, 
    1, 8, 8, 293, 1, 4294967295, 4294967295, 3, 8, 8, 294, 2, 213, 4294967295, 
    8, 8, 8, 296, 1, 4294967295, 4294967295, 1, 9, 32, 297, 1, 4294967295, 
    4294967295, 1, 9, 8, 298, 1, 4294967295, 4294967295, 3, 9, 8, 299, 2, 
    217, 4294967295, 8, 9, 8, 301, 1, 4294967295, 4294967295, 1, 9, 32, 
    302, 1, 4294967295, 4294967295, 1, 9, 8, 303, 1, 4294967295, 4294967295, 
    1, 10, 32, 304, 1, 4294967295, 4294967295, 1, 10, 8, 305, 1, 4294967295, 
    4294967295, 3, 10, 8, 306, 2, 223, 4294967295, 8, 10, 8, 308, 1, 4294967295, 
    4294967295, 1, 10, 32, 309, 1, 4294967295, 4294967295, 1, 10, 8, 310, 
    1, 4294967295, 4294967295, 1, 11, 8, 311, 1, 4294967295, 4294967295, 
    1, 11, 32, 312, 1, 4294967295, 4294967295, 1, 11, 8, 313, 1, 4294967295, 
    4294967295, 5, 11, 8, 314, 1, 230, 4294967295, 8, 11, 8, 315, 1, 4294967295, 
    4294967295, 10, 11, 8, 316, 2, 4294967295, 4294967295, 12, 11, 8, 318, 
    1, 4294967295, 233, 9, 11, 8, 319, 1, 4294967295, 4294967295, 1, 11, 
    32, 320, 1, 4294967295, 4294967295, 3, 11, 8, 321, 2, 236, 4294967295, 
    8, 11, 8, 323, 1, 4294967295, 4294967295, 1, 12, 32, 324, 1, 4294967295, 
    4294967295, 1, 12, 8, 325, 1, 4294967295, 4294967295, 1, 12, 32, 326, 
    1, 4294967295, 4294967295, 1, 12, 8, 327, 1, 4294967295, 4294967295, 
    3, 12, 8, 328, 2, 242, 4294967295, 8, 12, 8, 330, 1, 4294967295, 4294967295, 
    1, 12, 8, 331, 1, 4294967295, 4294967295, 1, 12, 8, 332, 1, 4294967295, 
    4294967295, 1, 12, 8, 333, 1, 4294967295, 4294967295, 1, 13, 8, 334, 
    1, 4294967295, 4294967295, 1, 13, 32, 335, 1, 4294967295, 4294967295, 
    1, 13, 32, 336, 1, 4294967295, 4294967295, 3, 13, 8, 337, 3, 250, 4294967295, 
    8, 13, 8, 340, 1, 4294967295, 4294967295, 1, 14, 32, 341, 1, 4294967295, 
    4294967295, 1, 14, 8, 342, 1, 4294967295, 4294967295, 1, 15, 32, 343, 
    1, 4294967295, 4294967295, 1, 15, 32, 344, 1, 4294967295, 4294967295, 
    5, 15, 8, 345, 1, 256, 4294967295, 8, 15, 8, 346, 1, 4294967295, 4294967295, 
    10, 15, 9, 347, 2, 4294967295, 4294967295, 12, 15, 8, 349, 1, 4294967295, 
    259, 9, 15, 8, 350, 1, 4294967295, 4294967295, 1, 15, 32, 351, 1, 4294967295, 
    4294967295, 1, 15, 8, 352, 1, 4294967295, 4294967295, 1, 16, 32, 353, 
    1, 4294967295, 4294967295, 1, 16, 8, 354, 1, 4294967295, 4294967295, 
    1, 16, 32, 355, 1, 4294967295, 4294967295, 1, 16, 8, 356, 1, 4294967295, 
    4294967295, 5, 16, 8, 357, 1, 267, 4294967295, 8, 16, 8, 358, 1, 4294967295, 
    4294967295, 10, 16, 8, 359, 2, 4294967295, 4294967295, 12, 16, 8, 361, 
    1, 4294967295, 270, 9, 16, 8, 362, 1, 4294967295, 4294967295, 1, 17, 
    8, 363, 1, 4294967295, 4294967295, 5, 17, 8, 364, 1, 273, 4294967295, 
    8, 17, 8, 365, 1, 4294967295, 4294967295, 10, 17, 8, 366, 2, 4294967295, 
    4294967295, 12, 17, 8, 368, 1, 4294967295, 276, 9, 17, 8, 369, 1, 4294967295, 
    4294967295, 1, 18, 8, 370, 1, 4294967295, 4294967295, 1, 18, 8, 371, 
    1, 4294967295, 4294967295, 3, 18, 8, 372, 2, 280, 4294967295, 8, 18, 
    8, 374, 1, 4294967295, 4294967295, 1, 19, 8, 375, 1, 4294967295, 4294967295, 
    3, 19, 8, 376, 2, 283, 4294967295, 8, 19, 8, 378, 1, 4294967295, 4294967295, 
    1, 19, 32, 379, 1, 4294967295, 4294967295, 1, 19, 8, 380, 1, 4294967295, 
    4294967295, 3, 19, 8, 381, 2, 287, 4294967295, 8, 19, 8, 383, 1, 4294967295, 
    4294967295, 1, 19, 8, 384, 1, 4294967295, 4294967295, 3, 19, 8, 385, 
    2, 290, 4294967295, 8, 19, 8, 387, 1, 4294967295, 4294967295, 1, 19, 
    8, 388, 1, 4294967295, 4294967295, 3, 19, 8, 389, 2, 293, 4294967295, 
    8, 19, 8, 391, 1, 4294967295, 4294967295, 1, 19, 8, 392, 1, 4294967295, 
    4294967295, 3, 19, 8, 393, 2, 296, 4294967295, 8, 19, 8, 395, 1, 4294967295, 
    4294967295, 1, 19, 8, 396, 1, 4294967295, 4294967295, 5, 19, 8, 397, 
    1, 299, 4294967295, 8, 19, 8, 398, 1, 4294967295, 4294967295, 10, 19, 
    8, 399, 2, 4294967295, 4294967295, 12, 19, 8, 401, 1, 4294967295, 302, 
    9, 19, 8, 402, 1, 4294967295, 4294967295, 1, 19, 32, 403, 1, 4294967295, 
    4294967295, 1, 19, 8, 404, 1, 4294967295, 4294967295, 1, 19, 32, 405, 
    1, 4294967295, 4294967295, 1, 19, 8, 406, 1, 4294967295, 4294967295, 
    1, 19, 8, 407, 1, 4294967295, 4294967295, 1, 20, 8, 408, 1, 4294967295, 
    4294967295, 5, 20, 8, 409, 1, 310, 4294967295, 8, 20, 8, 410, 1, 4294967295, 
    4294967295, 10, 20, 8, 411, 2, 4294967295, 4294967295, 12, 20, 8, 413, 
    1, 4294967295, 313, 9, 20, 8, 414, 1, 4294967295, 4294967295, 1, 20, 
    8, 415, 1, 4294967295, 4294967295, 3, 20, 8, 416, 2, 316, 4294967295, 
    8, 20, 8, 418, 1, 4294967295, 4294967295, 1, 21, 32, 419, 1, 4294967295, 
    4294967295, 1, 21, 8, 420, 1, 4294967295, 4294967295, 1, 21, 8, 421, 
    1, 4294967295, 4294967295, 1, 21, 8, 422, 1, 4294967295, 4294967295, 
    1, 22, 32, 423, 1, 4294967295, 4294967295, 1, 22, 8, 424, 1, 4294967295, 
    4294967295, 1, 22, 8, 425, 1, 4294967295, 4294967295, 1, 23, 8, 426, 
    1, 4294967295, 4294967295, 1, 23, 8, 427, 1, 4294967295, 4294967295, 
    3, 23, 8, 428, 2, 327, 4294967295, 8, 23, 8, 430, 1, 4294967295, 4294967295, 
    1, 24, 32, 431, 1, 4294967295, 4294967295, 1, 24, 8, 432, 1, 4294967295, 
    4294967295, 1, 24, 8, 433, 1, 4294967295, 4294967295, 1, 25, 32, 434, 
    1, 4294967295, 4294967295, 1, 25, 8, 435, 1, 4294967295, 4294967295, 
    1, 25, 32, 436, 1, 4294967295, 4294967295, 1, 25, 8, 437, 1, 4294967295, 
    4294967295, 5, 25, 8, 438, 1, 336, 4294967295, 8, 25, 8, 439, 1, 4294967295, 
    4294967295, 10, 25, 8, 440, 2, 4294967295, 4294967295, 12, 25, 8, 442, 
    1, 4294967295, 339, 9, 25, 8, 443, 1, 4294967295, 4294967295, 1, 26, 
    32, 444, 1, 4294967295, 4294967295, 1, 26, 8, 445, 1, 4294967295, 4294967295, 
    1, 26, 8, 446, 1, 4294967295, 4294967295, 1, 27, 32, 447, 1, 4294967295, 
    4294967295, 1, 27, 8, 448, 1, 4294967295, 4294967295, 1, 27, 8, 449, 
    1, 4294967295, 4294967295, 1, 27, 8, 450, 1, 4294967295, 4294967295, 
    1, 28, 8, 451, 1, 4294967295, 4294967295, 4, 28, 8, 452, 1, 349, 4294967295, 
    8, 28, 8, 453, 1, 4294967295, 4294967295, 11, 28, 8, 454, 2, 4294967295, 
    4294967295, 12, 28, 8, 456, 1, 4294967295, 350, 1, 29, 32, 457, 1, 4294967295, 
    4294967295, 1, 29, 8, 458, 1, 4294967295, 4294967295, 1, 30, 8, 459, 
    1, 4294967295, 4294967295, 1, 30, 8, 460, 1, 4294967295, 4294967295, 
    1, 31, 8, 461, 1, 4294967295, 4294967295, 1, 31, 32, 462, 1, 4294967295, 
    4294967295, 1, 31, 8, 463, 1, 4294967295, 4294967295, 5, 31, 8, 464, 
    1, 360, 4294967295, 8, 31, 8, 465, 1, 4294967295, 4294967295, 10, 31, 
    8, 466, 2, 4294967295, 4294967295, 12, 31, 8, 468, 1, 4294967295, 363, 
    9, 31, 8, 469, 1, 4294967295, 4294967295, 1, 32, 8, 470, 1, 4294967295, 
    4294967295, 1, 32, 32, 471, 1, 4294967295, 4294967295, 1, 32, 8, 472, 
    1, 4294967295, 4294967295, 3, 32, 8, 473, 2, 368, 4294967295, 8, 32, 
    8, 475, 1, 4294967295, 4294967295, 1, 33, 32, 476, 1, 4294967295, 4294967295, 
    3, 33, 8, 477, 2, 371, 4294967295, 8, 33, 8, 479, 1, 4294967295, 4294967295, 
    1, 33, 32, 480, 1, 4294967295, 4294967295, 1, 33, 8, 481, 1, 4294967295, 
    4294967295, 3, 33, 8, 482, 2, 375, 4294967295, 8, 33, 8, 484, 1, 4294967295, 
    4294967295, 1, 33, 32, 485, 1, 4294967295, 4294967295, 1, 33, 8, 486, 
    1, 4294967295, 4294967295, 1, 33, 32, 487, 1, 4294967295, 4294967295, 
    1, 33, 8, 488, 1, 4294967295, 4294967295, 1, 34, 8, 489, 1, 4294967295, 
    4294967295, 1, 34, 8, 490, 1, 4294967295, 4294967295, 1, 35, 8, 491, 
    1, 4294967295, 4294967295, 1, 35, 32, 492, 1, 4294967295, 4294967295, 
    1, 35, 8, 493, 1, 4294967295, 4294967295, 5, 35, 8, 494, 1, 386, 4294967295, 
    8, 35, 8, 495, 1, 4294967295, 4294967295, 10, 35, 8, 496, 2, 4294967295, 
    4294967295, 12, 35, 8, 498, 1, 4294967295, 389, 9, 35, 8, 499, 1, 4294967295, 
    4294967295, 1, 36, 8, 500, 1, 4294967295, 4294967295, 1, 36, 8, 501, 
    1, 4294967295, 4294967295, 3, 36, 8, 502, 2, 393, 4294967295, 8, 36, 
    8, 504, 1, 4294967295, 4294967295, 1, 36, 8, 505, 1, 4294967295, 4294967295, 
    3, 36, 8, 506, 2, 396, 4294967295, 8, 36, 8, 508, 1, 4294967295, 4294967295, 
    1, 37, 8, 509, 1, 4294967295, 4294967295, 4, 37, 8, 510, 1, 399, 4294967295, 
    8, 37, 8, 511, 1, 4294967295, 4294967295, 11, 37, 8, 512, 2, 4294967295, 
    4294967295, 12, 37, 8, 514, 1, 4294967295, 400, 1, 37, 8, 515, 1, 4294967295, 
    4294967295, 3, 37, 8, 516, 2, 404, 4294967295, 8, 37, 8, 518, 1, 4294967295, 
    4294967295, 1, 38, 8, 519, 1, 4294967295, 4294967295, 1, 38, 8, 520, 
    1, 4294967295, 4294967295, 3, 38, 8, 521, 2, 408, 4294967295, 8, 38, 
    8, 523, 1, 4294967295, 4294967295, 1, 38, 8, 524, 1, 4294967295, 4294967295, 
    1, 38, 8, 525, 1, 4294967295, 4294967295, 3, 38, 8, 526, 2, 412, 4294967295, 
    8, 38, 8, 528, 1, 4294967295, 4294967295, 1, 38, 8, 529, 1, 4294967295, 
    4294967295, 1, 38, 32, 530, 1, 4294967295, 4294967295, 3, 38, 8, 531, 
    2, 416, 4294967295, 8, 38, 8, 533, 1, 4294967295, 4294967295, 3, 38, 
    8, 534, 3, 418, 4294967295, 8, 38, 8, 537, 1, 4294967295, 4294967295, 
    1, 39, 32, 538, 1, 4294967295, 4294967295, 1, 39, 8, 539, 1, 4294967295, 
    4294967295, 1, 39, 32, 540, 1, 4294967295, 4294967295, 1, 39, 8, 541, 
    1, 4294967295, 4294967295, 1, 40, 32, 542, 1, 4294967295, 4294967295, 
    1, 40, 8, 543, 1, 4294967295, 4294967295, 1, 40, 32, 544, 1, 4294967295, 
    4294967295, 1, 40, 8, 545, 1, 4294967295, 4294967295, 5, 40, 8, 546, 
    1, 428, 4294967295, 8, 40, 8, 547, 1, 4294967295, 4294967295, 10, 40, 
    8, 548, 2, 4294967295, 4294967295, 12, 40, 8, 550, 1, 4294967295, 431, 
    9, 40, 8, 551, 1, 4294967295, 4294967295, 1, 41, 8, 552, 1, 4294967295, 
    4294967295, 1, 41, 32, 553, 1, 4294967295, 4294967295, 1, 41, 8, 554, 
    1, 4294967295, 4294967295, 1, 41, 32, 555, 1, 4294967295, 4294967295, 
    1, 41, 8, 556, 1, 4294967295, 4294967295, 1, 41, 8, 557, 1, 4294967295, 
    4294967295, 3, 41, 8, 558, 2, 439, 4294967295, 8, 41, 8, 560, 1, 4294967295, 
    4294967295, 1, 42, 8, 561, 1, 4294967295, 4294967295, 1, 42, 32, 562, 
    1, 4294967295, 4294967295, 3, 42, 8, 563, 2, 443, 4294967295, 8, 42, 
    8, 565, 1, 4294967295, 4294967295, 1, 43, 8, 566, 1, 4294967295, 4294967295, 
    1, 43, 32, 567, 1, 4294967295, 4294967295, 3, 43, 8, 568, 2, 447, 4294967295, 
    8, 43, 8, 570, 1, 4294967295, 4294967295, 1, 44, 8, 571, 1, 4294967295, 
    4294967295, 1, 44, 32, 572, 1, 4294967295, 4294967295, 1, 44, 8, 573, 
    1, 4294967295, 4294967295, 5, 44, 8, 574, 1, 452, 4294967295, 8, 44, 
    8, 575, 1, 4294967295, 4294967295, 10, 44, 8, 576, 2, 4294967295, 4294967295, 
    12, 44, 8, 578, 1, 4294967295, 455, 9, 44, 8, 579, 1, 4294967295, 4294967295, 
    1, 45, 8, 580, 1, 4294967295, 4294967295, 3, 45, 8, 581, 2, 458, 4294967295, 
    8, 45, 8, 583, 1, 4294967295, 4294967295, 1, 45, 8, 584, 1, 4294967295, 
    4294967295, 4, 45, 8, 585, 1, 461, 4294967295, 8, 45, 8, 586, 1, 4294967295, 
    4294967295, 11, 45, 8, 587, 2, 4294967295, 4294967295, 12, 45, 8, 589, 
    1, 4294967295, 462, 1, 45, 8, 590, 1, 4294967295, 4294967295, 3, 45, 
    8, 591, 2, 466, 4294967295, 8, 45, 8, 593, 1, 4294967295, 4294967295, 
    1, 46, 8, 594, 1, 4294967295, 4294967295, 1, 46, 8, 595, 1, 4294967295, 
    4294967295, 1, 46, 8, 596, 1, 4294967295, 4294967295, 3, 46, 8, 597, 
    2, 471, 4294967295, 8, 46, 8, 599, 1, 4294967295, 4294967295, 1, 46, 
    8, 600, 1, 4294967295, 4294967295, 1, 46, 8, 601, 1, 4294967295, 4294967295, 
    1, 46, 8, 602, 1, 4294967295, 4294967295, 3, 46, 8, 603, 2, 476, 4294967295, 
    8, 46, 8, 605, 1, 4294967295, 4294967295, 1, 46, 8, 606, 1, 4294967295, 
    4294967295, 1, 46, 8, 607, 1, 4294967295, 4294967295, 1, 46, 32, 608, 
    1, 4294967295, 4294967295, 3, 46, 8, 609, 2, 481, 4294967295, 8, 46, 
    8, 611, 1, 4294967295, 4294967295, 1, 46, 8, 612, 1, 4294967295, 4294967295, 
    3, 46, 8, 613, 2, 484, 4294967295, 8, 46, 8, 615, 1, 4294967295, 4294967295, 
    3, 46, 8, 616, 4, 486, 4294967295, 8, 46, 8, 620, 1, 4294967295, 4294967295, 
    1, 47, 32, 621, 1, 4294967295, 4294967295, 1, 47, 8, 622, 1, 4294967295, 
    4294967295, 1, 47, 32, 623, 1, 4294967295, 4294967295, 1, 47, 8, 624, 
    1, 4294967295, 4294967295, 5, 47, 8, 625, 1, 492, 4294967295, 8, 47, 
    8, 626, 1, 4294967295, 4294967295, 10, 47, 8, 627, 2, 4294967295, 4294967295, 
    12, 47, 8, 629, 1, 4294967295, 495, 9, 47, 8, 630, 1, 4294967295, 4294967295, 
    1, 47, 32, 631, 1, 4294967295, 4294967295, 1, 47, 8, 632, 1, 4294967295, 
    4294967295, 1, 48, 8, 633, 1, 4294967295, 4294967295, 1, 48, 8, 634, 
    1, 4294967295, 4294967295, 1, 48, 32, 635, 1, 4294967295, 4294967295, 
    1, 48, 8, 636, 1, 4294967295, 4294967295, 1, 48, 32, 637, 1, 4294967295, 
    4294967295, 1, 48, 32, 638, 1, 4294967295, 4294967295, 3, 48, 8, 639, 
    3, 505, 4294967295, 8, 48, 8, 642, 1, 4294967295, 4294967295, 3, 48, 
    8, 643, 2, 507, 4294967295, 8, 48, 8, 645, 1, 4294967295, 4294967295, 
    1, 49, 8, 646, 1, 4294967295, 4294967295, 1, 49, 32, 647, 1, 4294967295, 
    4294967295, 1, 49, 8, 648, 1, 4294967295, 4294967295, 1, 49, 8, 649, 
    1, 4294967295, 4294967295, 3, 49, 8, 650, 2, 513, 4294967295, 8, 49, 
    8, 652, 1, 4294967295, 4294967295, 1, 50, 8, 653, 1, 4294967295, 4294967295, 
    1, 50, 8, 654, 1, 4294967295, 4294967295, 3, 50, 8, 655, 2, 517, 4294967295, 
    8, 50, 8, 657, 1, 4294967295, 4294967295, 1, 51, 8, 658, 1, 4294967295, 
    4294967295, 1, 51, 8, 659, 1, 4294967295, 4294967295, 1, 52, 32, 660, 
    1, 4294967295, 4294967295, 1, 52, 32, 661, 1, 4294967295, 4294967295, 
    3, 52, 8, 662, 2, 523, 4294967295, 8, 52, 8, 664, 1, 4294967295, 4294967295, 
    1, 52, 32, 665, 1, 4294967295, 4294967295, 1, 52, 32, 666, 1, 4294967295, 
    4294967295, 3, 52, 8, 667, 2, 527, 4294967295, 8, 52, 8, 669, 1, 4294967295, 
    4294967295, 1, 52, 32, 670, 1, 4294967295, 4294967295, 1, 52, 32, 671, 
    1, 4294967295, 4294967295, 3, 52, 8, 672, 2, 531, 4294967295, 8, 52, 
    8, 674, 1, 4294967295, 4294967295, 3, 52, 8, 675, 3, 533, 4294967295, 
    8, 52, 8, 678, 1, 4294967295, 4294967295, 1, 53, 8, 679, 1, 4294967295, 
    4294967295, 1, 53, 8, 680, 1, 4294967295, 4294967295, 1, 53, 8, 681, 
    1, 4294967295, 4294967295, 1, 53, 32, 682, 1, 4294967295, 4294967295, 
    1, 53, 8, 683, 1, 4294967295, 4294967295, 3, 53, 8, 684, 5, 540, 4294967295, 
    8, 53, 8, 689, 1, 4294967295, 4294967295, 1, 54, 8, 690, 1, 4294967295, 
    4294967295, 1, 54, 8, 691, 1, 4294967295, 4294967295, 1, 54, 8, 692, 
    1, 4294967295, 4294967295, 1, 54, 8, 693, 1, 4294967295, 4294967295, 
    3, 54, 8, 694, 4, 546, 4294967295, 8, 54, 8, 698, 1, 4294967295, 4294967295, 
    1, 55, 32, 699, 1, 4294967295, 4294967295, 1, 55, 8, 700, 1, 4294967295, 
    4294967295, 3, 55, 8, 701, 2, 550, 4294967295, 8, 55, 8, 703, 1, 4294967295, 
    4294967295, 1, 56, 32, 704, 1, 4294967295, 4294967295, 1, 56, 8, 705, 
    1, 4294967295, 4294967295, 1, 56, 32, 706, 1, 4294967295, 4294967295, 
    1, 56, 8, 707, 1, 4294967295, 4294967295, 3, 56, 8, 708, 2, 556, 4294967295, 
    8, 56, 8, 710, 1, 4294967295, 4294967295, 1, 57, 32, 711, 1, 4294967295, 
    4294967295, 1, 57, 8, 712, 1, 4294967295, 4294967295, 1, 57, 32, 713, 
    1, 4294967295, 4294967295, 1, 57, 8, 714, 1, 4294967295, 4294967295, 
    5, 57, 8, 715, 1, 562, 4294967295, 8, 57, 8, 716, 1, 4294967295, 4294967295, 
    10, 57, 8, 717, 2, 4294967295, 4294967295, 12, 57, 8, 719, 1, 4294967295, 
    565, 9, 57, 8, 720, 1, 4294967295, 4294967295, 1, 57, 32, 721, 1, 4294967295, 
    4294967295, 1, 57, 8, 722, 1, 4294967295, 4294967295, 1, 58, 32, 723, 
    1, 4294967295, 4294967295, 1, 58, 8, 724, 1, 4294967295, 4294967295, 
    3, 58, 8, 725, 2, 571, 4294967295, 8, 58, 8, 727, 1, 4294967295, 4294967295, 
    1, 58, 32, 728, 1, 4294967295, 4294967295, 1, 58, 8, 729, 1, 4294967295, 
    4294967295, 3, 58, 8, 730, 2, 575, 4294967295, 8, 58, 8, 732, 1, 4294967295, 
    4294967295, 1, 58, 8, 733, 1, 4294967295, 4294967295, 1, 58, 32, 734, 
    1, 4294967295, 4294967295, 3, 58, 8, 735, 4, 579, 4294967295, 8, 58, 
    8, 739, 1, 4294967295, 4294967295, 1, 59, 32, 740, 1, 4294967295, 4294967295, 
    1, 59, 8, 741, 1, 4294967295, 4294967295, 3, 59, 8, 742, 2, 583, 4294967295, 
    8, 59, 8, 744, 1, 4294967295, 4294967295, 1, 59, 8, 745, 1, 4294967295, 
    4294967295, 5, 59, 8, 746, 1, 586, 4294967295, 8, 59, 8, 747, 1, 4294967295, 
    4294967295, 10, 59, 8, 748, 2, 4294967295, 4294967295, 12, 59, 8, 750, 
    1, 4294967295, 589, 9, 59, 8, 751, 1, 4294967295, 4294967295, 1, 59, 
    32, 752, 1, 4294967295, 4294967295, 3, 59, 8, 753, 2, 592, 4294967295, 
    8, 59, 8, 755, 1, 4294967295, 4294967295, 1, 59, 8, 756, 1, 4294967295, 
    4294967295, 1, 59, 32, 757, 1, 4294967295, 4294967295, 1, 59, 8, 758, 
    1, 4294967295, 4294967295, 1, 60, 32, 759, 1, 4294967295, 4294967295, 
    1, 60, 8, 760, 1, 4294967295, 4294967295, 3, 60, 8, 761, 2, 599, 4294967295, 
    8, 60, 8, 763, 1, 4294967295, 4294967295, 1, 60, 8, 764, 1, 4294967295, 
    4294967295, 3, 60, 8, 765, 2, 602, 4294967295, 8, 60, 8, 767, 1, 4294967295, 
    4294967295, 1, 61, 32, 768, 1, 4294967295, 4294967295, 1, 61, 32, 769, 
    1, 4294967295, 4294967295, 1, 61, 32, 770, 1, 4294967295, 4294967295, 
    1, 61, 8, 771, 1, 4294967295, 4294967295, 1, 62, 32, 772, 1, 4294967295, 
    4294967295, 1, 62, 8, 773, 1, 4294967295, 4294967295, 3, 62, 8, 774, 
    2, 610, 4294967295, 8, 62, 8, 776, 1, 4294967295, 4294967295, 1, 62, 
    32, 777, 1, 4294967295, 4294967295, 1, 62, 8, 778, 1, 4294967295, 4294967295, 
    3, 62, 8, 779, 2, 614, 4294967295, 8, 62, 8, 781, 1, 4294967295, 4294967295, 
    3, 62, 8, 782, 2, 616, 4294967295, 8, 62, 8, 784, 1, 4294967295, 4294967295, 
    1, 63, 32, 785, 1, 4294967295, 4294967295, 1, 63, 8, 786, 1, 4294967295, 
    4294967295, 1, 63, 32, 787, 1, 4294967295, 4294967295, 1, 63, 8, 788, 
    1, 4294967295, 4294967295, 5, 63, 8, 789, 1, 622, 4294967295, 8, 63, 
    8, 790, 1, 4294967295, 4294967295, 10, 63, 8, 791, 2, 4294967295, 4294967295, 
    12, 63, 8, 793, 1, 4294967295, 625, 9, 63, 8, 794, 1, 4294967295, 4294967295, 
    1, 63, 32, 795, 1, 4294967295, 4294967295, 1, 63, 8, 796, 1, 4294967295, 
    4294967295, 1, 64, 8, 797, 1, 4294967295, 4294967295, 1, 64, 8, 798, 
    1, 4294967295, 4294967295, 1, 64, 32, 799, 1, 4294967295, 4294967295, 
    1, 64, 8, 800, 1, 4294967295, 4294967295, 1, 64, 32, 801, 1, 4294967295, 
    4294967295, 1, 64, 32, 802, 1, 4294967295, 4294967295, 3, 64, 8, 803, 
    3, 635, 4294967295, 8, 64, 8, 806, 1, 4294967295, 4294967295, 3, 64, 
    8, 807, 2, 637, 4294967295, 8, 64, 8, 809, 1, 4294967295, 4294967295, 
    1, 65, 32, 810, 1, 4294967295, 4294967295, 1, 65, 8, 811, 1, 4294967295, 
    4294967295, 1, 66, 8, 812, 1, 4294967295, 4294967295, 1, 66, 32, 813, 
    1, 4294967295, 4294967295, 1, 66, 8, 814, 1, 4294967295, 4294967295, 
    5, 66, 8, 815, 1, 644, 4294967295, 8, 66, 8, 816, 1, 4294967295, 4294967295, 
    10, 66, 8, 817, 2, 4294967295, 4294967295, 12, 66, 8, 819, 1, 4294967295, 
    647, 9, 66, 8, 820, 1, 4294967295, 4294967295, 1, 66, 0, 821, 0, 4294967295, 
    4294967295, 1, 134, 0, 0, 0, 1, 150, 0, 0, 0, 1, 138, 0, 0, 0, 1, 159, 
    0, 0, 0, 1, 151, 0, 0, 0, 1, 166, 0, 0, 0, 1, 137, 0, 0, 0, 1, 168, 
    0, 0, 0, 1, 167, 0, 0, 0, 1, 327, 0, 0, 0, 1, 375, 0, 0, 0, 1, 583, 
    0, 0, 0, 1, 179, 0, 0, 0, 1, 170, 0, 0, 0, 1, 194, 0, 0, 0, 1, 182, 
    0, 0, 0, 1, 196, 0, 0, 0, 1, 167, 0, 0, 0, 1, 212, 0, 0, 0, 1, 202, 
    0, 0, 0, 1, 201, 0, 0, 0, 1, 214, 0, 0, 0, 1, 167, 0, 0, 0, 1, 220, 
    0, 0, 0, 1, 167, 0, 0, 0, 1, 226, 0, 0, 0, 1, 217, 0, 0, 0, 1, 223, 
    0, 0, 0, 1, 237, 0, 0, 0, 1, 167, 0, 0, 0, 1, 249, 0, 0, 0, 1, 239, 
    0, 0, 0, 1, 251, 0, 0, 0, 1, 195, 0, 0, 0, 1, 245, 0, 0, 0, 1, 320, 
    0, 0, 0, 1, 323, 0, 0, 0, 1, 346, 0, 0, 0, 1, 415, 0, 0, 0, 1, 480, 
    0, 0, 0, 1, 505, 0, 0, 0, 1, 253, 0, 0, 0, 1, 287, 0, 0, 0, 1, 319, 
    0, 0, 0, 1, 330, 0, 0, 0, 1, 342, 0, 0, 0, 1, 599, 0, 0, 0, 1, 262, 
    0, 0, 0, 1, 144, 0, 0, 0, 1, 274, 0, 0, 0, 1, 145, 0, 0, 0, 1, 279, 
    0, 0, 0, 1, 273, 0, 0, 0, 1, 282, 0, 0, 0, 1, 280, 0, 0, 0, 1, 311, 
    0, 0, 0, 1, 307, 0, 0, 0, 1, 317, 0, 0, 0, 1, 310, 0, 0, 0, 1, 321, 
    0, 0, 0, 1, 316, 0, 0, 0, 1, 326, 0, 0, 0, 1, 299, 0, 0, 0, 1, 328, 
    0, 0, 0, 1, 290, 0, 0, 0, 1, 331, 0, 0, 0, 1, 293, 0, 0, 0, 1, 340, 
    0, 0, 0, 1, 296, 0, 0, 0, 1, 343, 0, 0, 0, 1, 327, 0, 0, 0, 1, 586, 
    0, 0, 0, 1, 348, 0, 0, 0, 1, 283, 0, 0, 0, 1, 352, 0, 0, 0, 1, 349, 
    0, 0, 0, 1, 354, 0, 0, 0, 1, 305, 0, 0, 0, 1, 356, 0, 0, 0, 1, 355, 
    0, 0, 0, 1, 364, 0, 0, 0, 1, 361, 0, 0, 0, 1, 360, 0, 0, 0, 1, 370, 
    0, 0, 0, 1, 267, 0, 0, 0, 1, 280, 0, 0, 0, 1, 380, 0, 0, 0, 1, 378, 
    0, 0, 0, 1, 382, 0, 0, 0, 1, 381, 0, 0, 0, 1, 421, 0, 0, 0, 1, 395, 
    0, 0, 0, 1, 387, 0, 0, 0, 1, 386, 0, 0, 0, 1, 403, 0, 0, 0, 1, 392, 
    0, 0, 0, 1, 417, 0, 0, 0, 1, 399, 0, 0, 0, 1, 419, 0, 0, 0, 1, 411, 
    0, 0, 0, 1, 423, 0, 0, 0, 1, 393, 0, 0, 0, 1, 438, 0, 0, 0, 1, 429, 
    0, 0, 0, 1, 428, 0, 0, 0, 1, 442, 0, 0, 0, 1, 433, 0, 0, 0, 1, 439, 
    0, 0, 0, 1, 446, 0, 0, 0, 1, 435, 0, 0, 0, 1, 448, 0, 0, 0, 1, 594, 
    0, 0, 0, 1, 465, 0, 0, 0, 1, 367, 0, 0, 0, 1, 453, 0, 0, 0, 1, 452, 
    0, 0, 0, 1, 485, 0, 0, 0, 1, 461, 0, 0, 0, 1, 487, 0, 0, 0, 1, 484, 
    0, 0, 0, 1, 506, 0, 0, 0, 1, 493, 0, 0, 0, 1, 492, 0, 0, 0, 1, 508, 
    0, 0, 0, 1, 470, 0, 0, 0, 1, 514, 0, 0, 0, 1, 486, 0, 0, 0, 1, 518, 
    0, 0, 0, 1, 517, 0, 0, 0, 1, 532, 0, 0, 0, 1, 408, 0, 0, 0, 1, 412, 
    0, 0, 0, 1, 471, 0, 0, 0, 1, 476, 0, 0, 0, 1, 519, 0, 0, 0, 1, 539, 
    0, 0, 0, 1, 407, 0, 0, 0, 1, 545, 0, 0, 0, 1, 475, 0, 0, 0, 1, 513, 
    0, 0, 0, 1, 547, 0, 0, 0, 1, 540, 0, 0, 0, 1, 546, 0, 0, 0, 1, 555, 
    0, 0, 0, 1, 540, 0, 0, 0, 1, 546, 0, 0, 0, 1, 557, 0, 0, 0, 1, 556, 
    0, 0, 0, 1, 578, 0, 0, 0, 1, 556, 0, 0, 0, 1, 563, 0, 0, 0, 1, 562, 
    0, 0, 0, 1, 580, 0, 0, 0, 1, 513, 0, 0, 0, 1, 516, 0, 0, 0, 1, 596, 
    0, 0, 0, 1, 546, 0, 0, 0, 1, 603, 0, 0, 0, 1, 540, 0, 0, 0, 1, 579, 
    0, 0, 0, 1, 615, 0, 0, 0, 1, 540, 0, 0, 0, 1, 546, 0, 0, 0, 1, 617, 
    0, 0, 0, 1, 458, 0, 0, 0, 1, 550, 0, 0, 0, 1, 571, 0, 0, 0, 1, 575, 
    0, 0, 0, 1, 602, 0, 0, 0, 1, 610, 0, 0, 0, 1, 614, 0, 0, 0, 1, 636, 
    0, 0, 0, 1, 507, 0, 0, 0, 1, 623, 0, 0, 0, 1, 622, 0, 0, 0, 1, 638, 
    0, 0, 0, 1, 152, 0, 0, 0, 1, 180, 0, 0, 0, 1, 188, 0, 0, 0, 1, 187, 
    0, 0, 0, 1, 208, 0, 0, 0, 1, 210, 0, 0, 0, 1, 213, 0, 0, 0, 1, 231, 
    0, 0, 0, 1, 230, 0, 0, 0, 1, 244, 0, 0, 0, 1, 250, 0, 0, 0, 1, 264, 
    0, 0, 0, 1, 345, 0, 0, 0, 1, 368, 0, 0, 0, 1, 443, 0, 0, 0, 1, 447, 
    0, 0, 0, 1, 500, 0, 0, 0, 1, 509, 0, 0, 0, 1, 630, 0, 0, 0, 1, 645, 
    0, 0, 0, 1, 644, 0, 0, 0, 1, 640, 0, 0, 0, 1, 337, 0, 0, 0, 1, 336, 
    0, 0, 0, 1, 637, 0, 0, 0, 1, 635, 0, 0, 0, 3, 2, 1, 138, 0, 3, 6, 3, 
    137, 0, 1, 135, 0, 0, 0, 1, 140, 0, 0, 0, 1, 136, 0, 0, 0, 1, 139, 0, 
    0, 0, 1, 141, 0, 0, 0, 1, 138, 0, 0, 0, 3, 34, 17, 145, 0, 3, 32, 16, 
    144, 0, 1, 142, 0, 0, 0, 1, 147, 0, 0, 0, 1, 143, 0, 0, 0, 1, 146, 0, 
    0, 0, 1, 148, 0, 0, 0, 1, 145, 0, 0, 0, 5, 149, 4294967295, 0, 0, 1, 
    1, 0, 0, 0, 3, 4, 2, 151, 0, 3, 130, 65, 152, 0, 5, 153, 32, 0, 0, 1, 
    3, 0, 0, 0, 5, 155, 17, 0, 0, 5, 160, 19, 0, 0, 5, 157, 18, 0, 0, 5, 
    160, 19, 0, 0, 5, 160, 19, 0, 0, 1, 154, 0, 0, 0, 1, 156, 0, 0, 0, 1, 
    158, 0, 0, 0, 1, 5, 0, 0, 0, 259, 8, 4, 167, 0, 259, 14, 7, 167, 0, 
    259, 18, 9, 167, 0, 259, 20, 10, 167, 0, 259, 24, 12, 167, 0, 1, 161, 
    0, 0, 0, 1, 162, 0, 0, 0, 1, 163, 0, 0, 0, 1, 164, 0, 0, 0, 1, 165, 
    0, 0, 0, 1, 7, 0, 0, 0, 5, 174, 12, 0, 0, 3, 10, 5, 170, 0, 5, 171, 
    32, 0, 0, 1, 173, 0, 0, 0, 1, 169, 0, 0, 0, 1, 176, 0, 0, 0, 1, 172, 
    0, 0, 0, 1, 175, 0, 0, 0, 1, 177, 0, 0, 0, 1, 174, 0, 0, 0, 5, 178, 
    35, 0, 0, 1, 9, 0, 0, 0, 3, 130, 65, 180, 0, 5, 181, 39, 0, 0, 259, 
    12, 6, 182, 0, 1, 11, 0, 0, 0, 3, 130, 65, 188, 0, 5, 185, 47, 0, 0, 
    3, 130, 65, 187, 0, 1, 184, 0, 0, 0, 1, 190, 0, 0, 0, 1, 186, 0, 0, 
    0, 1, 189, 0, 0, 0, 1, 195, 0, 0, 0, 1, 188, 0, 0, 0, 5, 195, 8, 0, 
    0, 259, 28, 14, 195, 0, 5, 195, 7, 0, 0, 1, 183, 0, 0, 0, 1, 191, 0, 
    0, 0, 1, 192, 0, 0, 0, 1, 193, 0, 0, 0, 1, 13, 0, 0, 0, 5, 197, 15, 
    0, 0, 3, 16, 8, 202, 0, 5, 199, 31, 0, 0, 3, 16, 8, 201, 0, 1, 198, 
    0, 0, 0, 1, 204, 0, 0, 0, 1, 200, 0, 0, 0, 1, 203, 0, 0, 0, 1, 205, 
    0, 0, 0, 1, 202, 0, 0, 0, 5, 206, 32, 0, 0, 1, 15, 0, 0, 0, 3, 130, 
    65, 208, 0, 5, 209, 39, 0, 0, 259, 130, 65, 210, 0, 1, 213, 0, 0, 0, 
    259, 130, 65, 213, 0, 1, 207, 0, 0, 0, 1, 211, 0, 0, 0, 1, 17, 0, 0, 
    0, 5, 216, 13, 0, 0, 3, 22, 11, 217, 0, 1, 215, 0, 0, 0, 1, 217, 0, 
    0, 0, 1, 218, 0, 0, 0, 5, 219, 35, 0, 0, 1, 19, 0, 0, 0, 5, 222, 14, 
    0, 0, 3, 22, 11, 223, 0, 1, 221, 0, 0, 0, 1, 223, 0, 0, 0, 1, 224, 0, 
    0, 0, 5, 225, 35, 0, 0, 1, 21, 0, 0, 0, 3, 130, 65, 231, 0, 5, 228, 
    31, 0, 0, 3, 130, 65, 230, 0, 1, 227, 0, 0, 0, 1, 233, 0, 0, 0, 1, 229, 
    0, 0, 0, 1, 232, 0, 0, 0, 1, 235, 0, 0, 0, 1, 231, 0, 0, 0, 5, 236, 
    31, 0, 0, 1, 234, 0, 0, 0, 1, 236, 0, 0, 0, 1, 23, 0, 0, 0, 5, 241, 
    48, 0, 0, 3, 26, 13, 239, 0, 5, 240, 30, 0, 0, 1, 242, 0, 0, 0, 1, 238, 
    0, 0, 0, 1, 242, 0, 0, 0, 1, 243, 0, 0, 0, 3, 130, 65, 244, 0, 259, 
    28, 14, 245, 0, 1, 25, 0, 0, 0, 259, 130, 65, 250, 0, 5, 250, 17, 0, 
    0, 5, 250, 18, 0, 0, 1, 246, 0, 0, 0, 1, 247, 0, 0, 0, 1, 248, 0, 0, 
    0, 1, 27, 0, 0, 0, 5, 252, 11, 0, 0, 1, 29, 0, 0, 0, 5, 257, 10, 0, 
    0, 5, 256, 55, 0, 0, 1, 254, 0, 0, 0, 1, 259, 0, 0, 0, 1, 258, 0, 0, 
    0, 1, 255, 0, 0, 0, 1, 260, 0, 0, 0, 1, 257, 0, 0, 0, 5, 261, 53, 0, 
    0, 1, 31, 0, 0, 0, 5, 263, 28, 0, 0, 3, 130, 65, 264, 0, 5, 268, 32, 
    0, 0, 3, 66, 33, 267, 0, 1, 265, 0, 0, 0, 1, 270, 0, 0, 0, 1, 266, 0, 
    0, 0, 1, 269, 0, 0, 0, 1, 33, 0, 0, 0, 1, 268, 0, 0, 0, 3, 36, 18, 273, 
    0, 1, 271, 0, 0, 0, 1, 276, 0, 0, 0, 1, 272, 0, 0, 0, 1, 275, 0, 0, 
    0, 1, 35, 0, 0, 0, 1, 274, 0, 0, 0, 259, 38, 19, 280, 0, 259, 66, 33, 
    280, 0, 1, 277, 0, 0, 0, 1, 278, 0, 0, 0, 1, 37, 0, 0, 0, 3, 56, 28, 
    283, 0, 1, 281, 0, 0, 0, 1, 283, 0, 0, 0, 1, 284, 0, 0, 0, 5, 286, 2, 
    0, 0, 3, 30, 15, 287, 0, 1, 285, 0, 0, 0, 1, 287, 0, 0, 0, 1, 289, 0, 
    0, 0, 3, 48, 24, 290, 0, 1, 288, 0, 0, 0, 1, 290, 0, 0, 0, 1, 292, 0, 
    0, 0, 3, 50, 25, 293, 0, 1, 291, 0, 0, 0, 1, 293, 0, 0, 0, 1, 295, 0, 
    0, 0, 3, 52, 26, 296, 0, 1, 294, 0, 0, 0, 1, 296, 0, 0, 0, 1, 300, 0, 
    0, 0, 3, 46, 23, 299, 0, 1, 297, 0, 0, 0, 1, 302, 0, 0, 0, 1, 298, 0, 
    0, 0, 1, 301, 0, 0, 0, 1, 303, 0, 0, 0, 1, 300, 0, 0, 0, 5, 304, 29, 
    0, 0, 3, 60, 30, 305, 0, 5, 306, 32, 0, 0, 259, 40, 20, 307, 0, 1, 39, 
    0, 0, 0, 3, 42, 21, 310, 0, 1, 308, 0, 0, 0, 1, 313, 0, 0, 0, 1, 309, 
    0, 0, 0, 1, 312, 0, 0, 0, 1, 315, 0, 0, 0, 1, 311, 0, 0, 0, 259, 44, 
    22, 316, 0, 1, 314, 0, 0, 0, 1, 316, 0, 0, 0, 1, 41, 0, 0, 0, 5, 318, 
    26, 0, 0, 3, 30, 15, 319, 0, 259, 28, 14, 320, 0, 1, 43, 0, 0, 0, 5, 
    322, 27, 0, 0, 259, 28, 14, 323, 0, 1, 45, 0, 0, 0, 259, 8, 4, 327, 
    0, 259, 54, 27, 327, 0, 1, 324, 0, 0, 0, 1, 325, 0, 0, 0, 1, 47, 0, 
    0, 0, 5, 329, 23, 0, 0, 259, 30, 15, 330, 0, 1, 49, 0, 0, 0, 5, 332, 
    25, 0, 0, 3, 132, 66, 337, 0, 5, 334, 31, 0, 0, 3, 132, 66, 336, 0, 
    1, 333, 0, 0, 0, 1, 339, 0, 0, 0, 1, 335, 0, 0, 0, 1, 338, 0, 0, 0, 
    1, 51, 0, 0, 0, 1, 337, 0, 0, 0, 5, 341, 24, 0, 0, 259, 30, 15, 342, 
    0, 1, 53, 0, 0, 0, 5, 344, 48, 0, 0, 3, 130, 65, 345, 0, 259, 28, 14, 
    346, 0, 1, 55, 0, 0, 0, 3, 58, 29, 349, 0, 1, 347, 0, 0, 0, 1, 350, 
    0, 0, 0, 1, 348, 0, 0, 0, 1, 351, 0, 0, 0, 1, 57, 0, 0, 0, 7, 353, 0, 
    0, 0, 1, 59, 0, 0, 0, 259, 62, 31, 355, 0, 1, 61, 0, 0, 0, 3, 64, 32, 
    361, 0, 5, 358, 44, 0, 0, 3, 64, 32, 360, 0, 1, 357, 0, 0, 0, 1, 363, 
    0, 0, 0, 1, 359, 0, 0, 0, 1, 362, 0, 0, 0, 1, 63, 0, 0, 0, 1, 361, 0, 
    0, 0, 3, 90, 45, 367, 0, 5, 366, 49, 0, 0, 259, 130, 65, 368, 0, 1, 
    365, 0, 0, 0, 1, 368, 0, 0, 0, 1, 65, 0, 0, 0, 5, 371, 16, 0, 0, 1, 
    369, 0, 0, 0, 1, 371, 0, 0, 0, 1, 372, 0, 0, 0, 5, 374, 1, 0, 0, 3, 
    8, 4, 375, 0, 1, 373, 0, 0, 0, 1, 375, 0, 0, 0, 1, 376, 0, 0, 0, 5, 
    377, 29, 0, 0, 3, 68, 34, 378, 0, 5, 379, 32, 0, 0, 1, 67, 0, 0, 0, 
    259, 70, 35, 381, 0, 1, 69, 0, 0, 0, 3, 72, 36, 387, 0, 5, 384, 44, 
    0, 0, 3, 72, 36, 386, 0, 1, 383, 0, 0, 0, 1, 389, 0, 0, 0, 1, 385, 0, 
    0, 0, 1, 388, 0, 0, 0, 1, 71, 0, 0, 0, 1, 387, 0, 0, 0, 3, 74, 37, 392, 
    0, 259, 80, 40, 393, 0, 1, 391, 0, 0, 0, 1, 393, 0, 0, 0, 1, 396, 0, 
    0, 0, 1, 396, 0, 0, 0, 1, 390, 0, 0, 0, 1, 394, 0, 0, 0, 1, 73, 0, 0, 
    0, 3, 76, 38, 399, 0, 1, 397, 0, 0, 0, 1, 400, 0, 0, 0, 1, 398, 0, 0, 
    0, 1, 401, 0, 0, 0, 1, 404, 0, 0, 0, 1, 404, 0, 0, 0, 1, 398, 0, 0, 
    0, 1, 402, 0, 0, 0, 1, 75, 0, 0, 0, 3, 106, 53, 407, 0, 259, 104, 52, 
    408, 0, 1, 406, 0, 0, 0, 1, 408, 0, 0, 0, 1, 418, 0, 0, 0, 3, 78, 39, 
    411, 0, 259, 104, 52, 412, 0, 1, 410, 0, 0, 0, 1, 412, 0, 0, 0, 1, 418, 
    0, 0, 0, 3, 28, 14, 415, 0, 5, 416, 40, 0, 0, 1, 414, 0, 0, 0, 1, 416, 
    0, 0, 0, 1, 418, 0, 0, 0, 1, 405, 0, 0, 0, 1, 409, 0, 0, 0, 1, 413, 
    0, 0, 0, 1, 77, 0, 0, 0, 5, 420, 33, 0, 0, 3, 70, 35, 421, 0, 5, 422, 
    34, 0, 0, 1, 79, 0, 0, 0, 5, 424, 36, 0, 0, 3, 82, 41, 429, 0, 5, 426, 
    31, 0, 0, 3, 82, 41, 428, 0, 1, 425, 0, 0, 0, 1, 431, 0, 0, 0, 1, 427, 
    0, 0, 0, 1, 430, 0, 0, 0, 1, 81, 0, 0, 0, 1, 429, 0, 0, 0, 3, 84, 42, 
    433, 0, 5, 434, 33, 0, 0, 3, 86, 43, 435, 0, 5, 436, 34, 0, 0, 1, 439, 
    0, 0, 0, 259, 84, 42, 439, 0, 1, 432, 0, 0, 0, 1, 437, 0, 0, 0, 1, 83, 
    0, 0, 0, 259, 130, 65, 443, 0, 5, 443, 28, 0, 0, 1, 440, 0, 0, 0, 1, 
    441, 0, 0, 0, 1, 85, 0, 0, 0, 259, 130, 65, 447, 0, 5, 447, 7, 0, 0, 
    1, 444, 0, 0, 0, 1, 445, 0, 0, 0, 1, 87, 0, 0, 0, 3, 90, 45, 453, 0, 
    5, 450, 44, 0, 0, 3, 90, 45, 452, 0, 1, 449, 0, 0, 0, 1, 455, 0, 0, 
    0, 1, 451, 0, 0, 0, 1, 454, 0, 0, 0, 1, 89, 0, 0, 0, 1, 453, 0, 0, 0, 
    3, 126, 63, 458, 0, 1, 456, 0, 0, 0, 1, 458, 0, 0, 0, 1, 460, 0, 0, 
    0, 3, 92, 46, 461, 0, 1, 459, 0, 0, 0, 1, 462, 0, 0, 0, 1, 460, 0, 0, 
    0, 1, 463, 0, 0, 0, 1, 466, 0, 0, 0, 1, 466, 0, 0, 0, 1, 457, 0, 0, 
    0, 1, 464, 0, 0, 0, 1, 91, 0, 0, 0, 3, 98, 49, 470, 0, 259, 104, 52, 
    471, 0, 1, 471, 0, 0, 0, 1, 468, 0, 0, 0, 1, 469, 0, 0, 0, 1, 486, 0, 
    0, 0, 3, 108, 54, 475, 0, 259, 104, 52, 476, 0, 1, 476, 0, 0, 0, 1, 
    473, 0, 0, 0, 1, 474, 0, 0, 0, 1, 486, 0, 0, 0, 259, 100, 50, 486, 0, 
    3, 28, 14, 480, 0, 5, 481, 40, 0, 0, 1, 479, 0, 0, 0, 1, 481, 0, 0, 
    0, 1, 483, 0, 0, 0, 259, 94, 47, 484, 0, 1, 482, 0, 0, 0, 1, 484, 0, 
    0, 0, 1, 486, 0, 0, 0, 1, 467, 0, 0, 0, 1, 472, 0, 0, 0, 1, 477, 0, 
    0, 0, 1, 478, 0, 0, 0, 1, 93, 0, 0, 0, 5, 488, 37, 0, 0, 3, 96, 48, 
    493, 0, 5, 490, 31, 0, 0, 3, 96, 48, 492, 0, 1, 489, 0, 0, 0, 1, 495, 
    0, 0, 0, 1, 491, 0, 0, 0, 1, 494, 0, 0, 0, 1, 496, 0, 0, 0, 1, 493, 
    0, 0, 0, 5, 497, 38, 0, 0, 1, 95, 0, 0, 0, 259, 128, 64, 507, 0, 3, 
    130, 65, 500, 0, 5, 504, 39, 0, 0, 259, 28, 14, 505, 0, 5, 505, 7, 0, 
    0, 5, 505, 8, 0, 0, 1, 501, 0, 0, 0, 1, 502, 0, 0, 0, 1, 503, 0, 0, 
    0, 1, 507, 0, 0, 0, 1, 498, 0, 0, 0, 1, 499, 0, 0, 0, 1, 97, 0, 0, 0, 
    3, 130, 65, 509, 0, 7, 512, 1, 0, 0, 259, 108, 54, 513, 0, 259, 118, 
    59, 513, 0, 1, 510, 0, 0, 0, 1, 511, 0, 0, 0, 1, 99, 0, 0, 0, 3, 118, 
    59, 516, 0, 259, 102, 51, 517, 0, 1, 515, 0, 0, 0, 1, 517, 0, 0, 0, 
    1, 101, 0, 0, 0, 259, 104, 52, 519, 0, 1, 103, 0, 0, 0, 5, 522, 40, 
    0, 0, 5, 523, 40, 0, 0, 1, 521, 0, 0, 0, 1, 523, 0, 0, 0, 1, 533, 0, 
    0, 0, 5, 526, 41, 0, 0, 5, 527, 40, 0, 0, 1, 525, 0, 0, 0, 1, 527, 0, 
    0, 0, 1, 533, 0, 0, 0, 5, 530, 43, 0, 0, 5, 531, 40, 0, 0, 1, 529, 0, 
    0, 0, 1, 531, 0, 0, 0, 1, 533, 0, 0, 0, 1, 520, 0, 0, 0, 1, 524, 0, 
    0, 0, 1, 528, 0, 0, 0, 1, 105, 0, 0, 0, 259, 122, 61, 540, 0, 259, 124, 
    62, 540, 0, 259, 112, 56, 540, 0, 5, 540, 3, 0, 0, 259, 110, 55, 540, 
    0, 1, 534, 0, 0, 0, 1, 535, 0, 0, 0, 1, 536, 0, 0, 0, 1, 537, 0, 0, 
    0, 1, 538, 0, 0, 0, 1, 107, 0, 0, 0, 259, 124, 62, 546, 0, 259, 120, 
    60, 546, 0, 259, 112, 56, 546, 0, 259, 110, 55, 546, 0, 1, 541, 0, 0, 
    0, 1, 542, 0, 0, 0, 1, 543, 0, 0, 0, 1, 544, 0, 0, 0, 1, 109, 0, 0, 
    0, 5, 549, 47, 0, 0, 259, 126, 63, 550, 0, 1, 548, 0, 0, 0, 1, 550, 
    0, 0, 0, 1, 111, 0, 0, 0, 5, 552, 50, 0, 0, 259, 116, 58, 556, 0, 5, 
    554, 50, 0, 0, 259, 114, 57, 556, 0, 1, 551, 0, 0, 0, 1, 553, 0, 0, 
    0, 1, 113, 0, 0, 0, 5, 558, 33, 0, 0, 3, 116, 58, 563, 0, 5, 560, 44, 
    0, 0, 3, 116, 58, 562, 0, 1, 559, 0, 0, 0, 1, 565, 0, 0, 0, 1, 561, 
    0, 0, 0, 1, 564, 0, 0, 0, 1, 566, 0, 0, 0, 1, 563, 0, 0, 0, 5, 567, 
    34, 0, 0, 1, 115, 0, 0, 0, 5, 570, 1, 0, 0, 259, 126, 63, 571, 0, 1, 
    569, 0, 0, 0, 1, 571, 0, 0, 0, 1, 579, 0, 0, 0, 5, 574, 8, 0, 0, 259, 
    126, 63, 575, 0, 1, 573, 0, 0, 0, 1, 575, 0, 0, 0, 1, 579, 0, 0, 0, 
    259, 122, 61, 579, 0, 5, 579, 3, 0, 0, 1, 568, 0, 0, 0, 1, 572, 0, 0, 
    0, 1, 576, 0, 0, 0, 1, 577, 0, 0, 0, 1, 117, 0, 0, 0, 5, 591, 33, 0, 
    0, 3, 8, 4, 583, 0, 1, 581, 0, 0, 0, 1, 583, 0, 0, 0, 1, 587, 0, 0, 
    0, 3, 54, 27, 586, 0, 1, 584, 0, 0, 0, 1, 589, 0, 0, 0, 1, 585, 0, 0, 
    0, 1, 588, 0, 0, 0, 1, 590, 0, 0, 0, 1, 587, 0, 0, 0, 5, 592, 29, 0, 
    0, 1, 582, 0, 0, 0, 1, 592, 0, 0, 0, 1, 593, 0, 0, 0, 3, 88, 44, 594, 
    0, 5, 595, 34, 0, 0, 1, 119, 0, 0, 0, 5, 598, 2, 0, 0, 3, 30, 15, 599, 
    0, 1, 597, 0, 0, 0, 1, 599, 0, 0, 0, 1, 601, 0, 0, 0, 259, 126, 63, 
    602, 0, 1, 600, 0, 0, 0, 1, 602, 0, 0, 0, 1, 121, 0, 0, 0, 5, 604, 8, 
    0, 0, 5, 605, 46, 0, 0, 5, 606, 8, 0, 0, 1, 123, 0, 0, 0, 5, 609, 1, 
    0, 0, 259, 126, 63, 610, 0, 1, 608, 0, 0, 0, 1, 610, 0, 0, 0, 1, 616, 
    0, 0, 0, 5, 613, 8, 0, 0, 259, 126, 63, 614, 0, 1, 612, 0, 0, 0, 1, 
    614, 0, 0, 0, 1, 616, 0, 0, 0, 1, 607, 0, 0, 0, 1, 611, 0, 0, 0, 1, 
    125, 0, 0, 0, 5, 618, 37, 0, 0, 3, 128, 64, 623, 0, 5, 620, 31, 0, 0, 
    3, 128, 64, 622, 0, 1, 619, 0, 0, 0, 1, 625, 0, 0, 0, 1, 621, 0, 0, 
    0, 1, 624, 0, 0, 0, 1, 626, 0, 0, 0, 1, 623, 0, 0, 0, 5, 627, 38, 0, 
    0, 1, 127, 0, 0, 0, 259, 132, 66, 637, 0, 3, 130, 65, 630, 0, 5, 634, 
    39, 0, 0, 259, 132, 66, 635, 0, 5, 635, 8, 0, 0, 5, 635, 7, 0, 0, 1, 
    631, 0, 0, 0, 1, 632, 0, 0, 0, 1, 633, 0, 0, 0, 1, 637, 0, 0, 0, 1, 
    628, 0, 0, 0, 1, 629, 0, 0, 0, 1, 129, 0, 0, 0, 7, 639, 2, 0, 0, 1, 
    131, 0, 0, 0, 3, 130, 65, 645, 0, 5, 642, 47, 0, 0, 3, 130, 65, 644, 
    0, 1, 641, 0, 0, 0, 1, 647, 0, 0, 0, 1, 643, 0, 0, 0, 1, 646, 0, 0, 
    0, 1, 133, 0, 0, 0, 1, 645, 0, 0, 0, 0, 2, 1, 0, 2, 2, 2, 1, 2, 2, 4, 
    1, 1, 4, 2, 16, 16, 20, 22, 39, 39, 42, 42, 1, 2, 7405568, 0, 0, 0, 
    0, 1152, 0, 0, 6, 0, 0, 0, 138, 145, 159, 166, 174, 188, 194, 202, 212, 
    216, 222, 231, 235, 241, 249, 257, 268, 274, 279, 282, 286, 289, 292, 
    295, 300, 311, 315, 326, 337, 350, 361, 367, 370, 374, 387, 392, 395, 
    400, 403, 407, 411, 415, 417, 429, 438, 442, 446, 453, 457, 462, 465, 
    470, 475, 480, 483, 485, 493, 504, 506, 512, 516, 522, 526, 530, 532, 
    539, 545, 549, 555, 563, 570, 574, 578, 582, 587, 591, 598, 601, 609, 
    613, 615, 623, 634, 636, 645, 0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 
    22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 
    56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 
    90, 92, 94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 
    120, 122, 124, 126, 128, 130, 132, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 
    21, 23, 25, 27, 29, 31, 33, 35, 37, 39, 41, 43, 45, 47, 49, 51, 53, 
    55, 57, 59, 61, 63, 65, 67, 69, 71, 73, 75, 77, 79, 81, 83, 85, 87, 
    89, 91, 93, 95, 97, 99, 101, 103, 105, 107, 109, 111, 113, 115, 117, 
    119, 121, 123, 125, 127, 129, 131, 133
]);