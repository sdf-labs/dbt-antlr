// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! serde-serializable output model for the dbt emission layer.
//!
//! The structs mirror the Java tool's `org.antlr.v4.codegen.model` classes
//! (`OutputFile`, `LexerFile`, `Lexer`, `ParserFile`, `Parser`, `Recognizer`,
//! `RuleFunction`, `CodeBlockForAlt`, the `Choice` hierarchy, the `srcop`
//! and `decl` classes, `RuleActionFunction`, `RuleSempredFunction`,
//! `SerializedATN`). Field names serialize in the camelCase shape the
//! `Rust.stg` templates use (`lexer.name`, `lexer.escapedName`,
//! `file.grammarFileName`, ...), so template expressions read like the
//! original STG attribute references.
//!
//! Where the Java model exposes a `Map` keyed by name (`tokens`,
//! `actionFuncs`, `sempredFuncs`), the structs here use a list of key/value
//! pairs, which is what minijinja iterates naturally. Heterogeneous model
//! children (block ops, choice alternatives, declarations) are
//! internally-tagged enums consumed by the template's `Op`/`Getter`
//! dispatcher macros, mirroring the Java `OutputModelWalker`'s
//! template-by-class-name dispatch.

use std::collections::BTreeMap;

use serde::Serialize;

/// Root render context: the formal arguments of the `LexerFile` template.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LexerFileContext {
    pub lexer_file: OutputFileModel,
    pub lexer: LexerModel,
    pub named_actions: NamedActionsModel,
}

/// Mirrors `OutputFile` (the parts the lexer templates read).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputFileModel {
    /// Grammar file name as shown in the `fileHeader` comment.
    pub grammar_file_name: String,
    /// Version string of the Java tool the templates imitate
    /// (`Tool.VERSION` of the dbt fork).
    #[serde(rename = "ANTLRVersion")]
    pub antlr_version: String,
}

/// Named actions (`@header`, `@members`, ...) spliced into the output.
///
/// Fields are `None` when the grammar does not define the action; the
/// templates treat them as empty.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedActionsModel {
    pub header: Option<String>,
    pub init: Option<String>,
    pub fields: Option<String>,
    pub members: Option<String>,
    pub definitions: Option<String>,
    pub extend: Option<String>,
    /// Rule-level `@after` action (parser rule functions only).
    pub after: Option<String>,
}
/// The translated rule-level named actions (`@init`, `@after`) of one rule
/// function: Java `RuleFunction.namedActions`, whose values are `Action`
/// models (translated chunks).
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleNamedActionsModel {
    pub init: Option<Vec<ActionChunkModel>>,
    pub after: Option<Vec<ActionChunkModel>>,
}

/// One entry of the Java `Recognizer.tokens` map (`name -> token type`).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenConstantModel {
    pub key: String,
    pub value: i32,
}

/// One numbered embedded action or sempred: the `index -> text` pairs of the
/// Java `RuleActionFunction.actions` map.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexedActionModel {
    pub index: usize,
    pub chunks: Vec<ActionChunkModel>,
}

/// Mirrors `RuleActionFunction`/`RuleSempredFunction`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleActionFunctionModel {
    pub name: String,
    pub escaped_name: String,
    pub ctx_type: String,
    pub rule_index: usize,
    /// The `r.factory.g.lexer` check of the `RuleSempredFunction` template.
    pub is_lexer: bool,
    pub actions: Vec<IndexedActionModel>,
}

/// Root render context: the formal arguments of the `ListenerFile` and
/// `BaseListenerFile` templates.
///
/// The `kind` tag selects the template, like the `Op` dispatcher's
/// class-name dispatch (`BaseListenerFile` reuses the `ListenerFile` model
/// in the Java tool).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenerFileContext {
    /// Template selector: `ListenerFile` or `BaseListenerFile`.
    pub kind: &'static str,
    pub file: ListenerFileModel,
    /// The grammar-level `@header` action body (scope-less only).
    pub header: Option<String>,
    pub named_actions: NamedActionsModel,
}

/// Mirrors `ListenerFile`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenerFileModel {
    pub grammar_file_name: String,
    #[serde(rename = "ANTLRVersion")]
    pub antlr_version: String,
    pub grammar_name: String,
    /// Recognizer name, for example `CSVParser`.
    pub parser_name: String,
    /// The `listenerNames` set: rule names, or the alt labels for rules with
    /// alt-specific contexts, in the Java tool's iteration order.
    pub listener_names: Vec<String>,
    /// The `listenerLabelRuleNames` map: alt label to its defining rule.
    pub listener_label_rule_names: BTreeMap<String, String>,
}

/// Root render context: the formal arguments of the `VisitorFile` and
/// `BaseVisitorFile` templates. The `kind` tag selects the template.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorFileContext {
    /// Template selector: `VisitorFile` or `BaseVisitorFile`.
    pub kind: &'static str,
    pub file: VisitorFileModel,
    /// The grammar-level `@header` action body (scope-less only).
    pub header: Option<String>,
    pub named_actions: NamedActionsModel,
}

/// Mirrors `VisitorFile`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorFileModel {
    pub grammar_file_name: String,
    #[serde(rename = "ANTLRVersion")]
    pub antlr_version: String,
    pub grammar_name: String,
    /// Recognizer name, for example `CSVParser`.
    pub parser_name: String,
    /// The `visitorNames` set (same construction as `listenerNames`).
    pub visitor_names: Vec<String>,
    /// The `visitorLabelRuleNames` map: alt label to its defining rule.
    pub visitor_label_rule_names: BTreeMap<String, String>,
}

/// Mirrors `SerializedATN`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedAtnModel {
    /// Java-format serialized ATN words.
    pub serialized: Vec<i32>,
}

/// Packed parser-ATN words of the vendored packed format (`PATN` v3).
///
/// This intentionally replaces the Java `SerializedATN` model for parsers:
/// generated parsers embed the packed u32 words and decode them with
/// `dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PackedAtnModel {
    /// Packed parser ATN words.
    pub serialized: Vec<u32>,
}

/// Root render context: the formal arguments of the `ParserFile` template.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParserFileContext {
    pub file: ParserFileModel,
    pub parser: ParserModel,
    pub named_actions: NamedActionsModel,
    pub context_super_class: Option<String>,
}

/// Mirrors `ParserFile` (plus the `OutputFile` base fields the templates
/// read).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParserFileModel {
    pub grammar_file_name: String,
    #[serde(rename = "ANTLRVersion")]
    pub antlr_version: String,
    /// Grammar name (`CSV`), used for the listener/visitor `use` lines and
    /// the `TreeWalker` name.
    pub grammar_name: String,
    pub gen_listener: bool,
    pub gen_visitor: bool,
}

/// One entry of the Java `Parser.rules` collection (`name` + `index`),
/// rendered as the `RULE_<name>` constants.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleRefModel {
    pub name: String,
    pub index: usize,
}

/// Mirrors `Parser` (and its `Recognizer` base class).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParserModel {
    /// Recognizer name, for example `CSVParser`.
    pub name: String,
    pub grammar_name: String,
    pub grammar_file_name: String,
    pub tokens: Vec<TokenConstantModel>,
    pub rules: Vec<RuleRefModel>,
    pub rule_names: Vec<String>,
    pub literal_names: Vec<Option<String>>,
    pub symbolic_names: Vec<Option<String>>,
    pub funcs: Vec<RuleFunctionModel>,
    pub sempred_funcs: Vec<RuleActionFunctionModel>,
    pub atn: PackedAtnModel,
}

/// Mirrors `RuleFunction`. When `left_recursive` is set, the template
/// renders `LeftRecursiveRuleFunction` instead (the Java walker picks the
/// template by the `LeftRecursiveRuleFunction` subclass).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleFunctionModel {
    pub name: String,
    pub escaped_name: String,
    pub modifiers: Vec<String>,
    pub ctx_type: String,
    pub index: usize,
    /// Rule start state number.
    pub start_state: i32,
    pub has_lookahead_block: bool,
    /// Whether the rule was rewritten for left recursion.
    pub left_recursive: bool,
    pub args: Vec<DeclModel>,
    pub locals: Vec<DeclModel>,
    pub code: Vec<SrcOpModel>,
    pub rule_ctx: StructDeclModel,
    pub alt_label_ctxs: Vec<StructDeclModel>,
    /// Rule-level named actions (`@init`, `@after`).
    pub named_actions: RuleNamedActionsModel,
    pub finally_action: Option<String>,
    pub postamble: Vec<SrcOpModel>,
    pub exceptions: Vec<String>,
}

/// One token of a lookahead set, as the `Choice.TokenInfo` pairs the
/// templates render (`<parser.grammarName>_<t.name>`).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfoModel {
    pub ttype: i32,
    pub name: String,
}

/// The `choice` attribute of the choice-block ops; fields of the Java
/// `Choice`/`LL1Choice`/`LL1Loop`/`Loop` hierarchy the templates read.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceModel {
    /// The `ast.atnState.stateNumber` of the block or EBNF root.
    pub state_number: i32,
    pub decision: i32,
    /// `Loop.exitAlt`.
    pub exit_alt: i32,
    /// `LL1Loop.blockStartStateNumber` / `Loop.blockStartStateNumber`.
    pub block_start_state_number: i32,
    /// `LL1Loop.loopBackStateNumber` / `Loop.loopBackStateNumber`.
    pub loop_back_state_number: i32,
    /// Whether the EBNF root is greedy (`choice.ast.greedy`).
    pub greedy: bool,
    /// `LL1Choice.altLook`: one sorted token list per alternative. Empty
    /// for non-LL(1) choices.
    pub alt_look: Vec<Vec<TokenInfoModel>>,
}

/// One bitset window of `TestSetInline.Bitset`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BitsetModel {
    pub shift: i32,
    pub tokens: Vec<TokenInfoModel>,
    pub calculated: i64,
}

/// The `ctx` attribute of action chunks and op labels: only the
/// `escapedName` of the owning `StructDecl` is read by the `Rust.stg`
/// templates (`actionChunk.ctx.escapedName`).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChunkContextModel {
    pub escaped_name: String,
}

impl ChunkContextModel {
    pub fn new(escaped_name: impl Into<String>) -> Self {
        Self {
            escaped_name: escaped_name.into(),
        }
    }
}

/// One label of a `LabeledOp` (`InvokeRule`/`MatchToken`/`MatchSet`/
/// `MatchNotSet`/`Wildcard`): the `Decl` fields the `LabelsAssign` template
/// reads (`l.escapedName`, `l.ctx.escapedName`).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpLabelModel {
    pub name: String,
    pub escaped_name: String,
    /// The context struct the label decl was routed to (`Decl.ctx`).
    pub ctx: ChunkContextModel,
}

/// The chunks of a translated embedded action: the Java
/// `codegen.model.chunk` classes.
///
/// The `kind` tag drives the template's `Chunk` dispatcher, mirroring the
/// class-name dispatch. Variant fields serialize camelCase like the
/// `Rust.stg` attribute references.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all_fields = "camelCase")]
pub enum ActionChunkModel {
    ActionText {
        ctx: ChunkContextModel,
        text: String,
    },
    ArgRef {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
    },
    LocalRef {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
    },
    RetValueRef {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
    },
    QRetValueRef {
        ctx: ChunkContextModel,
        /// The rule label of the qualified reference (`a.dict`).
        dict: String,
        name: String,
        escaped_name: String,
    },
    TokenRef {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
    },
    LabelRef {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
    },
    ListLabelRef {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
    },
    #[serde(rename = "TokenPropertyRef_text")]
    TokenPropertyRefText {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "TokenPropertyRef_type")]
    TokenPropertyRefType {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "TokenPropertyRef_line")]
    TokenPropertyRefLine {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "TokenPropertyRef_pos")]
    TokenPropertyRefPos {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "TokenPropertyRef_channel")]
    TokenPropertyRefChannel {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "TokenPropertyRef_index")]
    TokenPropertyRefIndex {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "TokenPropertyRef_int")]
    TokenPropertyRefInt {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "RulePropertyRef_start")]
    RulePropertyRefStart {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "RulePropertyRef_stop")]
    RulePropertyRefStop {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "RulePropertyRef_text")]
    RulePropertyRefText {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "RulePropertyRef_ctx")]
    RulePropertyRefCtx {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "RulePropertyRef_parser")]
    RulePropertyRefParser {
        ctx: ChunkContextModel,
        label: String,
    },
    #[serde(rename = "ThisRulePropertyRef_start")]
    ThisRulePropertyRefStart { ctx: ChunkContextModel },
    #[serde(rename = "ThisRulePropertyRef_stop")]
    ThisRulePropertyRefStop { ctx: ChunkContextModel },
    #[serde(rename = "ThisRulePropertyRef_text")]
    ThisRulePropertyRefText { ctx: ChunkContextModel },
    #[serde(rename = "ThisRulePropertyRef_ctx")]
    ThisRulePropertyRefCtx { ctx: ChunkContextModel },
    #[serde(rename = "ThisRulePropertyRef_parser")]
    ThisRulePropertyRefParser { ctx: ChunkContextModel },
    SetAttr {
        ctx: ChunkContextModel,
        name: String,
        escaped_name: String,
        rhs_chunks: Vec<Self>,
    },
}

/// The ops of rule code blocks, alternatives, and choice preambles: the
/// Java `SrcOp` hierarchy.
///
/// The `kind` tag drives the template's `Op` dispatcher, which mirrors the
/// `OutputModelWalker` class-name dispatch. Variant fields serialize
/// camelCase like the `Rust.stg` attribute references (`op.stateNumber`,
/// `op.altLabel`, ...).
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all_fields = "camelCase")]
pub enum SrcOpModel {
    CodeBlockForOuterMostAlt {
        /// `CodeBlockForOuterMostAlt.alt.altNum`.
        alt_num: usize,
        /// `CodeBlockForOuterMostAlt.altLabel`.
        alt_label: Option<String>,
        locals: Vec<DeclModel>,
        preamble: Vec<Self>,
        ops: Vec<Self>,
    },
    CodeBlockForAlt {
        locals: Vec<DeclModel>,
        preamble: Vec<Self>,
        ops: Vec<Self>,
    },
    LL1AltBlock {
        choice: ChoiceModel,
        preamble: Vec<Self>,
        alts: Vec<Self>,
        error: Box<Self>,
    },
    LL1OptionalBlock {
        choice: ChoiceModel,
        alts: Vec<Self>,
        error: Box<Self>,
    },
    LL1OptionalBlockSingleAlt {
        choice: ChoiceModel,
        expr: Box<Self>,
        alts: Vec<Self>,
        preamble: Vec<Self>,
        error: Box<Self>,
        follow_expr: Box<Self>,
    },
    LL1StarBlockSingleAlt {
        choice: ChoiceModel,
        loop_expr: Box<Self>,
        alts: Vec<Self>,
        preamble: Vec<Self>,
        iteration: Vec<Self>,
    },
    LL1PlusBlockSingleAlt {
        choice: ChoiceModel,
        loop_expr: Box<Self>,
        alts: Vec<Self>,
        preamble: Vec<Self>,
        iteration: Vec<Self>,
    },
    AltBlock {
        choice: ChoiceModel,
        preamble: Vec<Self>,
        alts: Vec<Self>,
    },
    OptionalBlock {
        choice: ChoiceModel,
        alts: Vec<Self>,
    },
    StarBlock {
        choice: ChoiceModel,
        alts: Vec<Self>,
        iteration: Vec<Self>,
    },
    PlusBlock {
        choice: ChoiceModel,
        alts: Vec<Self>,
        error: Box<Self>,
    },
    InvokeRule {
        name: String,
        escaped_name: String,
        state_number: i32,
        ctx_name: String,
        /// The `r.ast.options.p` precedence option: `Some` on references to
        /// left-recursive rules (`0` for external references), which renders
        /// the `_rec` suffix and the precedence argument.
        precedence: Option<u32>,
        labels: Vec<OpLabelModel>,
        /// `InvokeRule.argExprsChunks`: the translated call arguments.
        arg_exprs_chunks: Vec<ActionChunkModel>,
    },
    MatchToken {
        state_number: i32,
        name: String,
        escaped_name: String,
        labels: Vec<OpLabelModel>,
    },
    /// The `AddToLabelList` op following an invoke/match op with a `+=`
    /// list label: pushes the op's first label value onto the list.
    AddToLabelList {
        label: OpLabelModel,
        /// `Target.escapeIfNeeded(getListLabel(label))`.
        list_name: String,
    },
    MatchSet {
        state_number: i32,
        var_name: String,
        labels: Vec<OpLabelModel>,
        expr: Box<Self>,
        capture: Box<Self>,
    },
    MatchNotSet {
        state_number: i32,
        var_name: String,
        labels: Vec<OpLabelModel>,
        expr: Box<Self>,
        capture: Box<Self>,
    },
    Wildcard {
        state_number: i32,
        labels: Vec<OpLabelModel>,
    },
    /// `Action`: an embedded action element, translated to chunks by the
    /// action translator (an empty `chunks` list renders as nothing, like
    /// the empty action the left-recursion transform prepends).
    Action {
        chunks: Vec<ActionChunkModel>,
    },
    TestSetInline {
        var_name: String,
        bitsets: Vec<BitsetModel>,
    },
    ThrowNoViableAlt,
    /// `SemPred`: only the `recRuleAltPredicate` form (left-recursion
    /// precedence predicates) is produced so far; the `fail` option and
    /// user-authored predicates are milestone 4.6.
    SemPred {
        state_number: i32,
        /// The translated predicate expression (`chunks`).
        chunks: Vec<ActionChunkModel>,
        /// The predicate text as a target string literal (with quotes), used
        /// in the failure message.
        predicate: String,
        /// `SemPred.failChunks`: the translated `fail={...}` option.
        fail_chunks: Option<Vec<ActionChunkModel>>,
        /// `SemPred.msg`: the `fail='...'` option as a target string literal
        /// (with quotes).
        msg: Option<String>,
    },
    /// The `recRuleSetStopToken` action injected into left-recursive rule
    /// code (and into the postamble of rules with `@after`/`@finally`).
    RecRuleSetStopToken,
    /// The `recRuleSetPrevCtx` action injected as the iteration op of a
    /// left-recursive rule's operator star block.
    RecRuleSetPrevCtx,
    /// The `recRuleAltStartAction` action injected at the start of each
    /// left-recursive operator alternative without an alt label; `label`
    /// is the label the original alternative put on the deleted leading
    /// recursive reference (`e1` in `e1=e '+' e2=e`), when any.
    RecRuleAltStartAction {
        rule_name: String,
        /// Capitalized rule name (`Utils.capitalize`), without the `Context`
        /// suffix.
        ctx_name: String,
        label: Option<String>,
        is_list_label: bool,
    },
    /// The `recRuleLabeledAltStartAction` action injected at the start of
    /// each left-recursive operator alternative with an alt label; `label`
    /// is the label the original alternative put on the deleted leading
    /// recursive reference (`a` in `a=e op='*' b=e`), when any.
    RecRuleLabeledAltStartAction {
        rule_name: String,
        /// The alternative's alt label (`altInfo.altLabel`).
        current_alt_label: String,
        label: Option<String>,
        is_list_label: bool,
    },
    /// The `recRuleReplaceContext` action injected at the start of each
    /// labeled left-recursive primary alternative; `ctx_name` is the
    /// capitalized alt label without the `Context` suffix.
    RecRuleReplaceContext {
        ctx_name: String,
    },
    CaptureNextToken {
        var_name: String,
    },
    CaptureNextTokenType {
        var_name: String,
    },
}

/// Mirrors `decl.Decl` and its subclasses. `signature` marks the
/// trait-method (no body) variant of the context getters. Variant fields
/// serialize camelCase like the `Rust.stg` attribute references.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all_fields = "camelCase")]
pub enum DeclModel {
    TokenDecl {
        name: String,
        escaped_name: String,
    },
    TokenTypeDecl {
        name: String,
        escaped_name: String,
    },
    TokenListDecl {
        name: String,
        escaped_name: String,
    },
    RuleContextDecl {
        name: String,
        escaped_name: String,
        ctx_name: String,
    },
    RuleContextListDecl {
        name: String,
        escaped_name: String,
        ctx_name: String,
    },
    AttributeDecl {
        name: String,
        escaped_name: String,
        #[serde(rename = "type")]
        ty: String,
    },
    ContextTokenGetterDecl {
        name: String,
        signature: bool,
    },
    ContextTokenListGetterDecl {
        name: String,
        signature: bool,
    },
    ContextTokenListIndexedGetterDecl {
        name: String,
        signature: bool,
    },
    ContextRuleGetterDecl {
        name: String,
        escaped_name: String,
        ctx_name: String,
        signature: bool,
    },
    ContextRuleListGetterDecl {
        name: String,
        ctx_name: String,
        signature: bool,
    },
    ContextRuleListIndexedGetterDecl {
        name: String,
        escaped_name: String,
        ctx_name: String,
        signature: bool,
    },
}

/// Mirrors `decl.StructDecl`.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StructDeclModel {
    pub name: String,
    pub escaped_name: String,
    /// Rule name the context struct derives from (`csvFile`).
    pub derived_from_name: String,
    /// Whether the rule has alt-specific contexts (`-> label`).
    pub provide_copy_from: bool,
    pub attrs: Vec<DeclModel>,
    pub getters: Vec<DeclModel>,
    pub signatures: Vec<DeclModel>,
    pub ctor_attrs: Vec<DeclModel>,
    pub token_decls: Vec<DeclModel>,
    pub token_type_decls: Vec<DeclModel>,
    pub token_list_decls: Vec<DeclModel>,
    pub rule_context_decls: Vec<DeclModel>,
    pub rule_context_list_decls: Vec<DeclModel>,
    pub attribute_decls: Vec<DeclModel>,
}

/// Mirrors `Lexer` (and its `Recognizer` base class).
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LexerModel {
    /// Recognizer name, for example `CSVLexer`.
    pub name: String,
    pub grammar_name: String,
    pub grammar_file_name: String,
    /// Token constants with a positive token type, in declaration order.
    pub tokens: Vec<TokenConstantModel>,
    pub rule_names: Vec<String>,
    /// Fully translated Rust string literals (`"'\\r'"`), trailing `None`s
    /// trimmed like the Java tool does.
    pub literal_names: Vec<Option<String>>,
    pub symbolic_names: Vec<Option<String>>,
    /// User-declared channel names (excludes the two built-in channels).
    pub channel_names: Vec<String>,
    pub modes: Vec<String>,
    pub action_funcs: Vec<RuleActionFunctionModel>,
    pub sempred_funcs: Vec<RuleActionFunctionModel>,
    pub atn: SerializedAtnModel,
}
