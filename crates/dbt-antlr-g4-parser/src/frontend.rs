// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::fmt;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use dbt_antlr_runtime::Arena;
use dbt_antlr_runtime::common_token_stream::CommonTokenStream;
use dbt_antlr_runtime::error_listener::ErrorListener;
use dbt_antlr_runtime::errors::ANTLRError;
use dbt_antlr_runtime::input_stream::InputStream;
use dbt_antlr_runtime::parser::Parser as _;
use dbt_antlr_runtime::parser_rule_context::ParserRuleContext;
use dbt_antlr_runtime::recognizer::Recognizer;
use dbt_antlr_runtime::token::{
    CommonToken, TOKEN_DEFAULT_CHANNEL, TOKEN_EOF as RUNTIME_TOKEN_EOF, Token,
};
use dbt_antlr_runtime::token_factory::CommonTokenFactory;
use dbt_antlr_runtime::token_stream::TokenStream;
use dbt_antlr_runtime::tree::{NodeInner as _, Tree as _};

use super::generated::antlrv4lexer::{
    ANTLRv4Lexer, BLOCK_COMMENT, COLON, DOC_COMMENT, MODE, RANGE, RULE_REF, SEMI, STRING_LITERAL,
    UNTERMINATED_ARGUMENT, UNTERMINATED_CHAR_SET, UNTERMINATED_STRING_LITERAL,
};
use super::generated::antlrv4parser::{self as grammar_parser, ANTLRv4Parser};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceId(u32);

impl SourceId {
    pub const fn new(index: u32) -> Self {
        Self(index)
    }

    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SyntaxId(u64);

impl SyntaxId {
    pub const fn new(index: u32) -> Self {
        Self(index as u64)
    }

    pub const fn for_source(source: SourceId, index: u32) -> Self {
        Self(((source.0 as u64) << 32) | index as u64)
    }

    pub const fn index(self) -> usize {
        (self.0 as u32) as usize
    }

    pub const fn source(self) -> SourceId {
        SourceId::new((self.0 >> 32) as u32)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceSpan {
    pub source: SourceId,
    pub bytes: Range<u32>,
}

impl SourceSpan {
    pub const fn empty(source: SourceId) -> Self {
        Self {
            source,
            bytes: 0..0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxToken {
    pub token_type: i32,
    pub channel: i32,
    pub span: SourceSpan,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyntaxNodeKind {
    Rule { rule_index: usize },
    Terminal { token_index: usize },
    Error { token_index: usize },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxNode {
    pub kind: SyntaxNodeKind,
    pub span: SourceSpan,
    child_ids: Range<u32>,
}

#[derive(Debug)]
pub struct Cst {
    nodes: Box<[SyntaxNode]>,
    children: Box<[SyntaxId]>,
    root: SyntaxId,
}

impl Cst {
    pub const fn root_id(&self) -> SyntaxId {
        self.root
    }

    pub fn root(&self) -> &SyntaxNode {
        &self.nodes[self.root.index()]
    }

    pub fn node(&self, id: SyntaxId) -> Option<&SyntaxNode> {
        self.nodes.get(id.index())
    }

    pub fn children(&self, id: SyntaxId) -> impl DoubleEndedIterator<Item = SyntaxId> + '_ {
        self.node(id)
            .into_iter()
            .flat_map(|node| {
                self.children[node.child_ids.start as usize..node.child_ids.end as usize].iter()
            })
            .copied()
    }

    pub fn descendants(&self, id: SyntaxId) -> CstDescendants<'_> {
        CstDescendants {
            cst: self,
            pending: vec![id],
        }
    }
}

#[derive(Debug)]
pub struct CstDescendants<'a> {
    cst: &'a Cst,
    pending: Vec<SyntaxId>,
}

impl Iterator for CstDescendants<'_> {
    type Item = SyntaxId;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.pending.pop()?;
        self.pending.extend(self.cst.children(id).rev());
        Some(id)
    }
}

#[derive(Debug)]
pub struct SourceFile {
    id: SourceId,
    logical_path: PathBuf,
    text: Rc<str>,
    line_starts: Box<[u32]>,
    tokens: Box<[SyntaxToken]>,
    trivia: Box<[u32]>,
    cst: Cst,
}

impl SourceFile {
    pub const fn id(&self) -> SourceId {
        self.id
    }

    pub fn logical_path(&self) -> &Path {
        &self.logical_path
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn tokens(&self) -> &[SyntaxToken] {
        &self.tokens
    }

    pub fn token_text(&self, token: &SyntaxToken) -> &str {
        if token.token_type == RUNTIME_TOKEN_EOF {
            return "<EOF>";
        }
        let span = &token.span.bytes;
        &self.text[span.start as usize..span.end as usize]
    }

    pub fn trivia(&self) -> impl Iterator<Item = &SyntaxToken> {
        self.trivia
            .iter()
            .map(|index| &self.tokens[*index as usize])
    }

    pub const fn cst(&self) -> &Cst {
        &self.cst
    }

    pub fn line_column(&self, byte: u32) -> Option<(usize, usize)> {
        let byte = byte as usize;
        if byte > self.text.len() || !self.text.is_char_boundary(byte) {
            return None;
        }
        let line_index = self
            .line_starts
            .partition_point(|line_start| *line_start as usize <= byte)
            .saturating_sub(1);
        let line_start = self.line_starts[line_index] as usize;
        let column = self.text[line_start..byte].chars().count();
        Some((line_index + 1, column))
    }

    pub fn byte_offset(&self, line: usize, column: usize) -> Option<u32> {
        let line_start = *self.line_starts.get(line.checked_sub(1)?)? as usize;
        let line_end = self.text[line_start..]
            .find('\n')
            .map_or(self.text.len(), |offset| line_start + offset);
        let offset = if column == self.text[line_start..line_end].chars().count() {
            line_end
        } else {
            self.text[line_start..line_end]
                .char_indices()
                .nth(column)?
                .0
                + line_start
        };
        u32::try_from(offset).ok()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticStage {
    Source,
    Lexer,
    Parser,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyntaxDiagnostic {
    pub code: &'static str,
    pub stage: DiagnosticStage,
    pub span: SourceSpan,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontendError {
    diagnostics: Vec<SyntaxDiagnostic>,
}

impl FrontendError {
    pub fn diagnostics(&self) -> &[SyntaxDiagnostic] {
        &self.diagnostics
    }
}

impl fmt::Display for FrontendError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "grammar frontend rejected the source with {} diagnostic(s)",
            self.diagnostics.len()
        )
    }
}

impl std::error::Error for FrontendError {}

#[derive(Debug)]
pub struct RecoveredSource {
    pub file: SourceFile,
    pub diagnostics: Vec<SyntaxDiagnostic>,
}

pub fn parse_source(
    source: SourceId,
    logical_path: impl Into<PathBuf>,
    text: impl Into<Box<str>>,
) -> Result<SourceFile, FrontendError> {
    let recovered = parse_source_recovering(source, logical_path, text)?;
    if recovered.diagnostics.is_empty() {
        Ok(recovered.file)
    } else {
        Err(FrontendError {
            diagnostics: recovered.diagnostics,
        })
    }
}

pub fn parse_source_recovering(
    source: SourceId,
    logical_path: impl Into<PathBuf>,
    text: impl Into<Box<str>>,
) -> Result<RecoveredSource, FrontendError> {
    let logical_path = logical_path.into();
    let text: Rc<str> = Rc::from(text.into());
    let line_starts = line_starts(source, &text)?;
    let lexer_collector = DiagnosticCollector::default();
    let parser_collector = DiagnosticCollector::default();
    let (tokens, cst, parse_error) = Arena::with(|arena| {
        let mut lexer =
            ANTLRv4Lexer::<_, CommonTokenFactory<'_, '_>>::new(arena, InputStream::new(&text));
        lexer.remove_error_listeners();
        lexer.add_error_listener(Box::new(lexer_collector.clone()));
        let mut parser = ANTLRv4Parser::new(arena, CommonTokenStream::new(lexer));
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(parser_collector.clone()));
        let root = parser.grammarSpec();
        let parse_error = root.as_ref().err().map(ToString::to_string);
        let stream = parser.get_input_stream_mut();
        while let Some(token) = stream.lt(1) {
            if token.get_token_type() == RUNTIME_TOKEN_EOF {
                break;
            }
            stream.consume();
        }
        let tokens = copy_tokens(source, &text, parser.get_input_stream())?;
        let cst = root
            .ok()
            .map(|root| copy_cst(source, &tokens, root.as_node()));
        Ok::<_, FrontendError>((tokens, cst, parse_error))
    })?;

    let mut diagnostics = lexer_collector
        .take()
        .into_iter()
        .map(|diagnostic| SyntaxDiagnostic {
            code: "G4F002",
            stage: DiagnosticStage::Lexer,
            span: diagnostic_span(
                source,
                &text,
                &line_starts,
                &tokens,
                diagnostic.line,
                diagnostic.column,
            ),
            message: diagnostic.message,
        })
        .collect::<Vec<_>>();
    diagnostics.extend(unterminated_diagnostics(source, &text, &tokens));
    if !diagnostics.is_empty() {
        return Err(FrontendError { diagnostics });
    }

    let reported = parser_collector.take();
    let syntax_error_count = reported.len();
    diagnostics.extend(reported.into_iter().map(|diagnostic| SyntaxDiagnostic {
        code: "G4F003",
        stage: DiagnosticStage::Parser,
        span: diagnostic_span(
            source,
            &text,
            &line_starts,
            &tokens,
            diagnostic.line,
            diagnostic.column,
        ),
        message: diagnostic.message,
    }));
    normalize_parser_diagnostics(&tokens, &mut diagnostics);

    if let Some(parse_error) = parse_error {
        if diagnostics.is_empty() {
            diagnostics.push(SyntaxDiagnostic {
                code: "G4F003",
                stage: DiagnosticStage::Parser,
                span: SourceSpan::empty(source),
                message: parse_error,
            });
        }
        return Err(FrontendError { diagnostics });
    }
    if diagnostics.is_empty() && syntax_error_count != 0 {
        diagnostics.push(SyntaxDiagnostic {
            code: "G4F003",
            stage: DiagnosticStage::Parser,
            span: SourceSpan::empty(source),
            message: format!("grammar parser recovered from {syntax_error_count} syntax error(s)"),
        });
    }

    let cst = match cst.expect("a successful parse produces a tree") {
        Ok(cst) => cst,
        Err(_) if !diagnostics.is_empty() => return Err(FrontendError { diagnostics }),
        Err(error) => return Err(error),
    };
    let trivia = tokens
        .iter()
        .enumerate()
        .filter(|(_, token)| {
            token.channel != TOKEN_DEFAULT_CHANNEL && token.token_type != RUNTIME_TOKEN_EOF
        })
        .map(|(index, _)| index as u32)
        .collect();
    Ok(RecoveredSource {
        file: SourceFile {
            id: source,
            logical_path,
            text,
            line_starts,
            tokens: tokens.into_boxed_slice(),
            trivia,
            cst,
        },
        diagnostics,
    })
}

fn normalize_parser_diagnostics(tokens: &[SyntaxToken], diagnostics: &mut Vec<SyntaxDiagnostic>) {
    let significant = tokens
        .iter()
        .filter(|token| token.channel == TOKEN_DEFAULT_CHANNEL)
        .collect::<Vec<_>>();

    for diagnostic in diagnostics.iter_mut() {
        let Some(index) = diagnostic_token_index(&significant, &diagnostic.span) else {
            continue;
        };
        if significant[index].token_type == RANGE
            && index > 0
            && significant[index - 1].token_type == STRING_LITERAL
            && significant
                .get(index + 1)
                .is_some_and(|token| token.token_type == STRING_LITERAL)
        {
            diagnostic.code = "G4S009";
            diagnostic.span = significant[index - 1].span.clone();
            "character ranges are not allowed in parser rules".clone_into(&mut diagnostic.message);
        }
    }

    let mut recovered = Vec::new();
    for diagnostic in diagnostics.iter() {
        let Some(index) = diagnostic_token_index(&significant, &diagnostic.span) else {
            continue;
        };
        if significant[index].token_type != RULE_REF
            || significant
                .get(index + 1)
                .is_none_or(|token| token.token_type != COLON)
        {
            continue;
        }
        let Some(mode_index) = significant[..index]
            .iter()
            .rposition(|token| token.token_type == MODE)
        else {
            continue;
        };
        if !significant[mode_index + 1..index]
            .iter()
            .any(|token| token.token_type == SEMI)
        {
            continue;
        }
        let Some(semicolon) = significant[index + 1..]
            .iter()
            .find(|token| token.token_type == SEMI)
        else {
            continue;
        };
        if diagnostics
            .iter()
            .chain(&recovered)
            .any(|existing| existing.span == semicolon.span)
        {
            continue;
        }
        recovered.push(SyntaxDiagnostic {
            code: "G4F003",
            stage: DiagnosticStage::Parser,
            span: semicolon.span.clone(),
            message: "mismatched input ';' expecting COLON while matching a lexer rule".to_owned(),
        });
    }
    diagnostics.extend(recovered);

    let mut seen = Vec::new();
    diagnostics.retain(|diagnostic| {
        if seen.contains(&diagnostic.span) {
            false
        } else {
            seen.push(diagnostic.span.clone());
            true
        }
    });
}

fn diagnostic_token_index(tokens: &[&SyntaxToken], span: &SourceSpan) -> Option<usize> {
    tokens
        .iter()
        .position(|token| token.span.bytes.start == span.bytes.start)
}

fn line_starts(source: SourceId, text: &str) -> Result<Box<[u32]>, FrontendError> {
    const LIMIT_EXCEEDED: &str = "grammar source exceeds the 4 GiB frontend limit";

    u32::try_from(text.len()).map_err(|_| invalid_span(source, LIMIT_EXCEEDED))?;

    let mut starts = vec![0];
    for (index, byte) in text.bytes().enumerate() {
        if byte == b'\n' {
            starts.push((index + 1) as u32);
        }
    }
    Ok(starts.into_boxed_slice())
}

fn copy_tokens<'input, 'arena>(
    source: SourceId,
    text: &str,
    token_stream: &dyn TokenStream<'input, 'arena, CommonTokenFactory<'input, 'arena>>,
) -> Result<Vec<SyntaxToken>, FrontendError>
where
    'input: 'arena,
{
    let mut tokens = Vec::new();
    for index in 0..token_stream.size() {
        let token = token_stream.get(index);
        let start = u32::try_from(token.get_start_index())
            .map_err(|_| invalid_span(source, "token byte span exceeds 4 GiB"))?;
        let end = u32::try_from(token.get_stop_index() + 1)
            .map_err(|_| invalid_span(source, "token byte span exceeds 4 GiB"))?;
        if start > end
            || end as usize > text.len()
            || !text.is_char_boundary(start as usize)
            || !text.is_char_boundary(end as usize)
        {
            return Err(invalid_span(
                source,
                "token span is not on valid UTF-8 boundaries",
            ));
        }
        tokens.push(SyntaxToken {
            token_type: token.get_token_type(),
            channel: token.get_channel(),
            span: SourceSpan {
                source,
                bytes: start..end,
            },
        });
    }
    Ok(tokens)
}

fn invalid_span(source: SourceId, message: &str) -> FrontendError {
    FrontendError {
        diagnostics: vec![SyntaxDiagnostic {
            code: "G4F001",
            stage: DiagnosticStage::Source,
            span: SourceSpan::empty(source),
            message: message.to_owned(),
        }],
    }
}

fn unterminated_diagnostics(
    source: SourceId,
    text: &str,
    tokens: &[SyntaxToken],
) -> Vec<SyntaxDiagnostic> {
    tokens
        .iter()
        .filter_map(|token| {
            let token_text = token_text(text, token);
            let message = match token.token_type {
                UNTERMINATED_STRING_LITERAL => "unterminated string literal",
                UNTERMINATED_ARGUMENT => "unterminated argument",
                UNTERMINATED_CHAR_SET => "unterminated lexer character set",
                BLOCK_COMMENT | DOC_COMMENT if !token_text.ends_with("*/") => {
                    "unterminated block comment"
                }
                _ => return None,
            };
            Some(SyntaxDiagnostic {
                code: "G4F002",
                stage: DiagnosticStage::Lexer,
                span: SourceSpan {
                    source,
                    bytes: token.span.bytes.clone(),
                },
                message: message.to_owned(),
            })
        })
        .collect()
}

fn token_text<'a>(text: &'a str, token: &SyntaxToken) -> &'a str {
    if token.token_type == RUNTIME_TOKEN_EOF {
        "<EOF>"
    } else {
        &text[token.span.bytes.start as usize..token.span.bytes.end as usize]
    }
}

fn diagnostic_span(
    source: SourceId,
    text: &str,
    line_starts: &[u32],
    tokens: &[SyntaxToken],
    line: usize,
    column: usize,
) -> SourceSpan {
    let start = byte_offset(text, line_starts, line, column);
    if let Some(token) = tokens.iter().find(|token| token.span.bytes.start == start) {
        return token.span.clone();
    }
    let start_usize = start as usize;
    let end = text[start_usize..]
        .chars()
        .next()
        .map_or(start, |character| start + character.len_utf8() as u32);
    SourceSpan {
        source,
        bytes: start..end,
    }
}

fn byte_offset(text: &str, line_starts: &[u32], line: usize, column: usize) -> u32 {
    let line_start = line
        .checked_sub(1)
        .and_then(|index| line_starts.get(index))
        .copied()
        .unwrap_or_else(|| u32::try_from(text.len()).expect("source length checked"));
    let line_start_usize = line_start as usize;
    let line_end = text[line_start_usize..]
        .find('\n')
        .map_or(text.len(), |offset| line_start_usize + offset);
    let offset = text[line_start_usize..line_end]
        .char_indices()
        .nth(column)
        .map_or(line_end, |(offset, _)| line_start_usize + offset);
    u32::try_from(offset).expect("source length checked")
}

fn copy_cst(
    source: SourceId,
    tokens: &[SyntaxToken],
    root: &grammar_parser::ANTLRv4ParserNode<'_, '_, CommonToken<'_>>,
) -> Result<Cst, FrontendError> {
    let mut builder = TypedCstBuilder::new(source, tokens);
    builder.walk(root)?;
    builder.finish()
}

struct OpenRule {
    syntax: SyntaxId,
    children: Vec<SyntaxId>,
}

struct TypedCstBuilder<'tokens> {
    source: SourceId,
    tokens: &'tokens [SyntaxToken],
    nodes: Vec<SyntaxNode>,
    children: Vec<SyntaxId>,
    open_rules: Vec<OpenRule>,
    root: Option<SyntaxId>,
}

impl<'tokens> TypedCstBuilder<'tokens> {
    const fn new(source: SourceId, tokens: &'tokens [SyntaxToken]) -> Self {
        Self {
            source,
            tokens,
            nodes: Vec::new(),
            children: Vec::new(),
            open_rules: Vec::new(),
            root: None,
        }
    }

    fn finish(self) -> Result<Cst, FrontendError> {
        if !self.open_rules.is_empty() {
            return Err(invalid_span(
                self.source,
                "typed CST traversal left parser rules open",
            ));
        }
        let root = self
            .root
            .ok_or_else(|| invalid_span(self.source, "typed CST traversal produced no root"))?;
        Ok(Cst {
            nodes: self.nodes.into_boxed_slice(),
            children: self.children.into_boxed_slice(),
            root,
        })
    }

    fn walk<'input>(
        &mut self,
        node: &grammar_parser::ANTLRv4ParserNode<'input, '_, CommonToken<'input>>,
    ) -> Result<(), FrontendError> {
        if let Some(terminal) = node.as_terminal_node() {
            return self.push_token(token_index(self.source, terminal.symbol)?, false);
        }
        if let Some(error) = node.as_error_node() {
            return self.push_token(token_index(self.source, error.symbol)?, true);
        }
        let context = node.get_rule_context();
        let span = rule_span(self.source, self.tokens, context)?;
        let syntax = self.push_node(
            SyntaxNodeKind::Rule {
                rule_index: context.get_rule_index(),
            },
            span,
        )?;
        self.open_rules.push(OpenRule {
            syntax,
            children: Vec::new(),
        });
        for child in node.get_children() {
            self.walk(child)?;
        }
        let frame = self.open_rules.pop().ok_or_else(|| {
            invalid_span(
                self.source,
                "typed CST traversal exited a rule that was not entered",
            )
        })?;
        let child_start = u32::try_from(self.children.len())
            .map_err(|_| invalid_span(self.source, "CST exceeds 2^32 edges"))?;
        self.children.extend(frame.children);
        let child_end = u32::try_from(self.children.len())
            .map_err(|_| invalid_span(self.source, "CST exceeds 2^32 edges"))?;
        self.nodes[frame.syntax.index()].child_ids = child_start..child_end;
        Ok(())
    }

    fn push_token(&mut self, token_index: usize, error: bool) -> Result<(), FrontendError> {
        let token = self
            .tokens
            .get(token_index)
            .ok_or_else(|| invalid_span(self.source, "CST references a missing token"))?;
        let kind = if error {
            SyntaxNodeKind::Error { token_index }
        } else {
            SyntaxNodeKind::Terminal { token_index }
        };
        self.push_node(kind, token.span.clone())?;
        Ok(())
    }

    fn push_node(
        &mut self,
        kind: SyntaxNodeKind,
        span: SourceSpan,
    ) -> Result<SyntaxId, FrontendError> {
        let node_index = u32::try_from(self.nodes.len())
            .map_err(|_| invalid_span(self.source, "CST exceeds 2^32 nodes"))?;
        let syntax = SyntaxId::for_source(self.source, node_index);
        self.nodes.push(SyntaxNode {
            kind,
            span,
            child_ids: 0..0,
        });
        if let Some(parent) = self.open_rules.last_mut() {
            parent.children.push(syntax);
        } else if self.root.is_none() {
            self.root = Some(syntax);
        } else {
            return Err(invalid_span(
                self.source,
                "typed CST traversal produced multiple roots",
            ));
        }
        Ok(syntax)
    }
}

fn token_index(source: SourceId, token: &dyn Token) -> Result<usize, FrontendError> {
    usize::try_from(token.get_token_index())
        .map_err(|_| invalid_span(source, "CST references a missing token"))
}

fn rule_span<'input, 'arena>(
    source: SourceId,
    tokens: &[SyntaxToken],
    context: &dyn ParserRuleContext<'input, 'arena>,
) -> Result<SourceSpan, FrontendError>
where
    'input: 'arena,
{
    let token_span = |token: &dyn Token| -> Result<Option<Range<u32>>, FrontendError> {
        let index = token.get_token_index();
        if index < 0 {
            return Ok(None);
        }
        let span = tokens
            .get(usize::try_from(index).expect("non-negative index checked"))
            .map(|token| token.span.bytes.clone())
            .ok_or_else(|| invalid_span(source, "CST references a missing token"))?;
        Ok(Some(span))
    };
    let start = token_span(context.start())?.map_or(0, |span| span.start);
    let end = token_span(context.stop())?
        .map_or(start, |span| span.end)
        .max(start);
    Ok(SourceSpan {
        source,
        bytes: start..end,
    })
}

#[derive(Clone, Debug)]
struct ReportedDiagnostic {
    line: usize,
    column: usize,
    message: String,
}

#[derive(Clone, Debug, Default)]
struct DiagnosticCollector(Arc<Mutex<Vec<ReportedDiagnostic>>>);

impl DiagnosticCollector {
    fn take(&self) -> Vec<ReportedDiagnostic> {
        std::mem::take(
            &mut *self
                .0
                .lock()
                .expect("grammar diagnostic collector mutex poisoned"),
        )
    }
}

impl<'input, 'arena, R, Tok> ErrorListener<'input, 'arena, R, Tok> for DiagnosticCollector
where
    'input: 'arena,
    R: Recognizer<'input, 'arena, Tok>,
    Tok: Token + 'input,
{
    fn syntax_error(
        &self,
        _recognizer: &R,
        _offending_symbol: Option<&dyn Token>,
        line: u32,
        column: i32,
        message: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.0
            .lock()
            .expect("grammar diagnostic collector mutex poisoned")
            .push(ReportedDiagnostic {
                line: line as usize,
                column: usize::try_from(column).unwrap_or(0),
                message: message.to_owned(),
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generated::antlrv4lexer::{
        ARGUMENT_CONTENT, ASSIGN, BEGIN_ARGUMENT, END_ARGUMENT, GRAMMAR, INT, LEXER,
        LEXER_CHAR_SET, OPTIONS, RBRACE, TOKEN_REF,
    };
    use dbt_antlr_runtime::TokenSource as _;
    use std::fmt::Write as _;
    use std::fs;

    const SNAPSHOTS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/frontend/frontend-snapshots.tsv"
    ));
    const CORPUS: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/frontend/frontend-corpus.json"
    ));

    #[test]
    fn frontend_corpus_paths_match_snapshot_paths() {
        let corpus: serde_json::Value =
            serde_json::from_str(CORPUS).expect("frontend corpus should be valid JSON");
        let corpus_paths = corpus["cases"]
            .as_array()
            .expect("frontend corpus should contain cases")
            .iter()
            .map(|case| {
                case["path"]
                    .as_str()
                    .expect("frontend corpus case should contain a path")
            })
            .collect::<Vec<_>>();
        let snapshot_paths = SNAPSHOTS
            .lines()
            .skip(1)
            .map(|row| {
                row.split('\t')
                    .nth(1)
                    .expect("frontend snapshot row should contain a path")
            })
            .collect::<Vec<_>>();

        assert_eq!(corpus_paths, snapshot_paths);
        for path in corpus_paths {
            assert!(
                workspace_root().join(path).is_file(),
                "frontend corpus path does not exist: {path}"
            );
        }
    }

    #[test]
    fn pinned_frontend_corpus_matches_token_and_tree_oracles() {
        for (case_index, row) in SNAPSHOTS.lines().skip(1).enumerate() {
            let fields = row.split('\t').collect::<Vec<_>>();
            assert_eq!(fields.len(), 7, "malformed snapshot row: {row}");
            let path = workspace_root().join(fields[1]);
            let text = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            let file = parse_source(SourceId::new(case_index as u32), fields[1], text)
                .unwrap_or_else(|error| {
                    panic!("{}: {error}: {:?}", fields[0], error.diagnostics())
                });

            let (token_count, token_hash) = token_snapshot(&file);
            assert_eq!(token_count.to_string(), fields[3], "{} tokens", fields[0]);
            assert_eq!(token_hash, fields[4], "{} tokens", fields[0]);

            let (node_count, tree_hash) = tree_snapshot(&file);
            assert_eq!(node_count.to_string(), fields[5], "{} CST", fields[0]);
            assert_eq!(tree_hash, fields[6], "{} CST", fields[0]);
        }
    }

    #[test]
    fn malformed_bootstrap_inputs_fail_closed() {
        for name in ["BadAlternative.g4", "MissingDelimiter.g4"] {
            let path = workspace_root()
                .join("tests/frontend/bootstrap/malformed")
                .join(name);
            let text = fs::read_to_string(&path).expect("malformed fixture should be readable");
            let error = parse_source(SourceId::new(0), &path, text)
                .expect_err("malformed grammar must not return a CST");
            assert_frontend_installed(&error);
            assert!(
                error
                    .diagnostics()
                    .iter()
                    .any(|diagnostic| diagnostic.stage == DiagnosticStage::Parser),
                "{name}: {:?}",
                error.diagnostics()
            );
        }
    }

    #[test]
    fn malformed_editor_edit_fails_but_valid_undefined_rules_return_a_tree() {
        let malformed = "grammar A; a:: b \n| c; c: b+;";
        let error = parse_source(SourceId::new(7), "memory:malformed-edit", malformed)
            .expect_err("a:: must fail closed");
        assert_frontend_installed(&error);
        let spans = error
            .diagnostics()
            .iter()
            .filter(|diagnostic| diagnostic.stage == DiagnosticStage::Parser)
            .map(|diagnostic| diagnostic.span.bytes.clone())
            .collect::<Vec<_>>();
        assert_eq!(spans, [12..14, 18..19, 21..22]);

        let valid = "grammar A; a: b \n| c; c: b+;";
        let file = parse_source(SourceId::new(8), "memory:undefined-rules", valid)
            .expect("syntax-only Phase A must return a CST");
        assert_eq!(file.cst().root().span.bytes, 0..valid.len() as u32);
    }

    #[test]
    fn unterminated_constructs_fail_closed() {
        let cases = [
            ("string", "grammar A; a: 'unterminated\n;"),
            ("action", "grammar A; @members { unterminated"),
            ("argument", "grammar A; a[unterminated: A;"),
            ("character set", "lexer grammar A; A: [unterminated;"),
            ("comment", "grammar A; /* unterminated"),
        ];
        for (name, text) in cases {
            let error = parse_source(SourceId::new(0), name, text)
                .expect_err("unterminated input must not return a CST");
            assert_frontend_installed(&error);
            assert!(
                error.diagnostics().iter().any(|diagnostic| matches!(
                    diagnostic.stage,
                    DiagnosticStage::Lexer | DiagnosticStage::Parser
                )),
                "{name}: {:?}",
                error.diagnostics()
            );
        }
    }

    #[test]
    fn tool_syntax_cases_match_upstream_outcomes() {
        let accepted = [
            ("testA/parser-no-rules", "grammar A;\n"),
            ("testA/lexer-no-rules", "lexer grammar A;\n"),
            (
                "testEmptyGrammarOptions",
                "grammar A;\noptions {}\na : 'x' ;\n",
            ),
            ("testEmptyRuleOptions", "grammar A;\na options{} : 'x' ;\n"),
            (
                "testEmptyBlockOptions",
                "grammar A;\na : (options{} : 'x') ;\n",
            ),
            ("testEmptyTokensBlock", "grammar A;\ntokens {}\na : 'x' ;\n"),
        ];
        for (name, source) in accepted {
            parse_source(SourceId::new(0), name, source)
                .unwrap_or_else(|error| panic!("{name}: {:?}", error.diagnostics()));
        }

        let rejected = [
            (
                "testA/missing-grammar-keyword",
                "A;",
                DiagnosticStage::Parser,
            ),
            (
                "testA/missing-grammar-name",
                "grammar ;",
                DiagnosticStage::Parser,
            ),
            (
                "testA/missing-grammar-semi",
                "grammar A\na : ID ;\n",
                DiagnosticStage::Parser,
            ),
            (
                "testA/extra-rule-semi",
                "grammar A;\na : ID ;;\nb : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testA/extra-grammar-semi",
                "grammar A;;\na : ID ;\n",
                DiagnosticStage::Parser,
            ),
            (
                "testA/missing-rule-action",
                "grammar A;\na @init : ID ;\n",
                DiagnosticStage::Parser,
            ),
            (
                "testA/malformed-rule-prequel",
                "grammar A;\na  ( A | B ) D ;\nb : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testExtraColon",
                "grammar A;\na : : A ;\nb : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testMissingRuleSemi",
                "grammar A;\na : A \nb : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testMissingRuleSemi2",
                "lexer grammar A;\nA : 'a' \nB : 'b' ;",
                DiagnosticStage::Parser,
            ),
            (
                "testMissingRuleSemi3",
                "grammar A;\na : A \nb[int i] returns [int y] : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testMissingRuleSemi4",
                "grammar A;\na : b \n  catch [Exception e] {...}\nb : B ;\n",
                DiagnosticStage::Parser,
            ),
            (
                "testMissingRuleSemi5",
                "grammar A;\na : A \n  catch [Exception e] {...}\n",
                DiagnosticStage::Parser,
            ),
            (
                "testBadRulePrequelStart",
                "grammar A;\na @ options {k=1;} : A ;\nb : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testBadRulePrequelStart2",
                "grammar A;\na } : A ;\nb : B ;",
                DiagnosticStage::Parser,
            ),
            (
                "testUnterminatedStringLiteral",
                "grammar A;\na : 'x\n  ;\n",
                DiagnosticStage::Lexer,
            ),
            (
                "testParserRuleNameStartingWithUnderscore",
                "grammar A;\n_a : 'x' ;\n",
                DiagnosticStage::Lexer,
            ),
        ];
        let mut observed = Vec::new();
        for (name, source, expected_stage) in rejected {
            let error = parse_source(SourceId::new(0), name, source)
                .expect_err("invalid grammar must not return a CST");
            assert!(
                error
                    .diagnostics()
                    .iter()
                    .any(|diagnostic| diagnostic.stage == expected_stage),
                "{name}: {:?}",
                error.diagnostics()
            );
            observed.push((name, error.diagnostics().to_vec()));
        }

        insta::assert_debug_snapshot!("tool_syntax_cases_match_upstream_outcomes", observed);
    }

    /// The checked-in `ANTLRv4Lexer.g4` keeps U+FEFF inside `NameStartChar`,
    /// exactly like the upstream grammars-v4 grammar, so a leading byte order
    /// mark glues to the first token and the parse fails closed. The vendored
    /// frontend skipped the mark only because its generation-source grammar
    /// folded U+FEFF into the `WS` rule.
    #[test]
    fn utf8_byte_order_mark_fails_closed() {
        let with_bom = "\u{feff}grammar A; a: 'x';";
        let error = parse_source(SourceId::new(0), "memory:bom", with_bom)
            .expect_err("a leading byte order mark must fail closed");
        assert_frontend_installed(&error);
        assert!(
            error
                .diagnostics()
                .iter()
                .any(|diagnostic| diagnostic.stage == DiagnosticStage::Parser),
            "{:?}",
            error.diagnostics()
        );

        let without_bom = "grammar A; a: 'x';";
        let plain = parse_source(SourceId::new(1), "memory:no-bom", without_bom)
            .expect("the control grammar should parse");
        assert_eq!(
            plain.line_column(without_bom.len() as u32 - 1),
            Some((1, 17))
        );
    }

    /// A byte order mark is only whitespace outside literals; inside a string
    /// literal or a character set it stays ordinary content.
    #[test]
    fn byte_order_mark_inside_literals_remains_content() {
        let text = "lexer grammar A; A: '\u{feff}'; B: [\u{feff}a-z]+;";
        let file = parse_source(SourceId::new(0), "memory:bom-literal", text)
            .expect("a byte order mark inside literals should parse");
        let literals = file
            .tokens()
            .iter()
            .filter(|token| matches!(token.token_type, STRING_LITERAL | LEXER_CHAR_SET))
            .map(|token| file.token_text(token))
            .collect::<Vec<_>>();
        assert_eq!(literals, ["'\u{feff}'", "[\u{feff}a-z]"]);
    }

    /// Windows and classic-Mac line endings are plain whitespace, and the line
    /// map counts them the way Java ANTLR does.
    #[test]
    fn carriage_returns_are_whitespace_and_end_lines() {
        for (name, text) in [
            ("crlf", "grammar A;\r\na: 'x';\r\n"),
            ("cr", "grammar A;\ra: 'x';\r"),
        ] {
            let file = parse_source(SourceId::new(0), name, text)
                .unwrap_or_else(|error| panic!("{name}: {:?}", error.diagnostics()));
            assert_eq!(
                file.tokens()
                    .iter()
                    .filter(|token| token.channel == TOKEN_DEFAULT_CHANNEL
                        && token.token_type != RUNTIME_TOKEN_EOF)
                    .map(|token| file.token_text(token))
                    .collect::<Vec<_>>(),
                ["grammar", "A", ";", "a", ":", "'x'", ";"],
                "{name}",
            );
        }

        // A CRLF pair must advance the line once, not twice, so diagnostics on
        // the second line report the same column as the LF-only spelling.
        let crlf = parse_source(SourceId::new(0), "crlf-lines", "grammar A;\r\na: 'x';\r\n")
            .expect("CRLF grammar should parse");
        let lf = parse_source(SourceId::new(1), "lf-lines", "grammar A;\na: 'x';\n")
            .expect("LF grammar should parse");
        let semicolon = |file: &SourceFile| {
            let token = file
                .tokens()
                .iter()
                .filter(|token| token.token_type == SEMI)
                .nth(1)
                .expect("both grammars have two semicolons");
            file.line_column(token.span.bytes.start)
        };
        assert_eq!(semicolon(&crlf), Some((2, 6)));
        assert_eq!(semicolon(&crlf), semicolon(&lf));
    }

    #[test]
    fn tparser_preserves_named_action_rule_and_argument_spans() {
        const ACTION_BLOCK_RULE: usize = 14;
        const ARG_ACTION_BLOCK_RULE: usize = 15;
        const PARSER_RULE_SPEC_RULE: usize = 19;

        let path = workspace_root()
            .join("tests/frontend/external/vscode-antlr4/tests/backend/test-data/TParser.g4");
        let text = fs::read_to_string(&path).expect("TParser fixture should be readable");
        let file = parse_source(SourceId::new(11), &path, text.clone())
            .expect("TParser should be syntactically valid");

        assert_rule_span(
            &file,
            ACTION_BLOCK_RULE,
            byte_offset(&text, 30, 17)..byte_offset(&text, 37, 1),
        );
        assert_rule_span(
            &file,
            PARSER_RULE_SPEC_RULE,
            byte_offset(&text, 82, 0)..byte_offset(&text, 90, 1),
        );
        assert_rule_span(
            &file,
            ARG_ACTION_BLOCK_RULE,
            byte_offset(&text, 82, 63)..byte_offset(&text, 82, 90),
        );
    }

    #[test]
    fn grammar_options_preserve_following_lexer_rule_mode() {
        let tokens = lex_token_types(concat!(
            "lexer grammar G;\n",
            "options { language=Rust; tokenVocab=T; }\n",
            "A: [a];\n",
        ));

        assert_eq!(
            tokens,
            [
                LEXER,
                GRAMMAR,
                TOKEN_REF,
                SEMI,
                OPTIONS,
                RULE_REF,
                ASSIGN,
                TOKEN_REF,
                SEMI,
                RULE_REF,
                ASSIGN,
                TOKEN_REF,
                SEMI,
                RBRACE,
                TOKEN_REF,
                COLON,
                LEXER_CHAR_SET,
                SEMI,
                RUNTIME_TOKEN_EOF,
            ],
        );
    }

    #[test]
    fn parser_rule_options_preserve_argument_mode() {
        let tokens = lex_token_types(concat!(
            "grammar G;\n",
            "a options { k=1; } : T b[0];\n",
            "b[int value] : T;\n",
            "T: 'x';\n",
        ));

        assert_eq!(
            tokens,
            [
                GRAMMAR,
                TOKEN_REF,
                SEMI,
                RULE_REF,
                OPTIONS,
                RULE_REF,
                ASSIGN,
                INT,
                SEMI,
                RBRACE,
                COLON,
                TOKEN_REF,
                RULE_REF,
                BEGIN_ARGUMENT,
                ARGUMENT_CONTENT,
                END_ARGUMENT,
                SEMI,
                RULE_REF,
                BEGIN_ARGUMENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                ARGUMENT_CONTENT,
                END_ARGUMENT,
                COLON,
                TOKEN_REF,
                SEMI,
                TOKEN_REF,
                COLON,
                STRING_LITERAL,
                SEMI,
                RUNTIME_TOKEN_EOF,
            ],
        );
    }

    #[test]
    fn uncased_initials_are_rule_references() {
        let references = lex_tokens("文: 'x'; ÄToken: 'z';")
            .into_iter()
            .filter(|(_, token_type)| matches!(*token_type, RULE_REF | TOKEN_REF))
            .collect::<Vec<_>>();

        assert_eq!(
            references,
            [
                ("文".to_owned(), RULE_REF),
                ("ÄToken".to_owned(), TOKEN_REF),
            ]
        );
    }

    fn lex_token_types(text: &str) -> Vec<i32> {
        lex_tokens(text)
            .into_iter()
            .map(|(_, token_type)| token_type)
            .collect()
    }

    fn lex_tokens(text: &str) -> Vec<(String, i32)> {
        Arena::with(|arena| {
            let mut lexer =
                ANTLRv4Lexer::<_, CommonTokenFactory<'_, '_>>::new(arena, InputStream::new(text));
            lexer.remove_error_listeners();
            let mut tokens = Vec::new();
            loop {
                let token = lexer.next_token();
                let token_type = token.get_token_type();
                if token.get_channel() == TOKEN_DEFAULT_CHANNEL {
                    tokens.push((token.get_text().to_owned(), token_type));
                }
                if token_type == RUNTIME_TOKEN_EOF {
                    break;
                }
            }
            tokens
        })
    }

    fn workspace_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    }

    fn assert_frontend_installed(error: &FrontendError) {
        assert!(
            error
                .diagnostics()
                .iter()
                .all(|diagnostic| diagnostic.code != "G4F000"),
            "red fingerprint: Stage 0 frontend is not installed"
        );
    }

    fn assert_rule_span(file: &SourceFile, rule_index: usize, expected: Range<u32>) {
        assert!(
            file.cst.nodes.iter().any(|node| {
                node.kind == SyntaxNodeKind::Rule { rule_index } && node.span.bytes == expected
            }),
            "missing rule {rule_index} span {expected:?}"
        );
    }

    fn byte_offset(text: &str, one_based_line: usize, column: usize) -> u32 {
        let line_start = text
            .split_inclusive('\n')
            .take(one_based_line.saturating_sub(1))
            .map(str::len)
            .sum::<usize>();
        let line = text[line_start..]
            .split_once('\n')
            .map_or(&text[line_start..], |(line, _)| line);
        let column_bytes = line
            .char_indices()
            .nth(column)
            .map_or(line.len(), |(offset, _)| offset);
        u32::try_from(line_start + column_bytes).expect("fixture offset should fit in u32")
    }

    fn token_snapshot(file: &SourceFile) -> (usize, String) {
        let mut hash = Fnv1a64::new();
        let mut count = 0;
        for token in file
            .tokens()
            .iter()
            .filter(|token| token.token_type != RUNTIME_TOKEN_EOF)
        {
            let mut row = format!(
                "{}\t{}\t{}\t{}\t",
                token.token_type, token.channel, token.span.bytes.start, token.span.bytes.end
            );
            push_json_string(&mut row, file.token_text(token));
            row.push('\n');
            hash.update(row.as_bytes());
            count += 1;
        }
        (count, hash.finish())
    }

    fn tree_snapshot(file: &SourceFile) -> (usize, String) {
        let mut hash = Fnv1a64::new();
        let mut count = 0;
        snapshot_node(file, file.cst.root, &mut hash, &mut count);
        (count, hash.finish())
    }

    fn snapshot_node(file: &SourceFile, id: SyntaxId, hash: &mut Fnv1a64, count: &mut usize) {
        *count += 1;
        let node = file.cst().node(id).expect("CST child ID should resolve");
        let mut row = String::new();
        match node.kind {
            SyntaxNodeKind::Rule { rule_index } => {
                writeln!(
                    row,
                    "R\t{}\t{}",
                    rule_index,
                    file.cst().children(id).count()
                )
                .expect("writing to String cannot fail");
            }
            SyntaxNodeKind::Terminal { token_index } | SyntaxNodeKind::Error { token_index } => {
                let prefix = if matches!(node.kind, SyntaxNodeKind::Error { .. }) {
                    'E'
                } else {
                    'T'
                };
                let token = &file.tokens()[token_index];
                write!(row, "{prefix}\t{}\t", token.token_type)
                    .expect("writing to String cannot fail");
                push_json_string(&mut row, file.token_text(token));
                row.push('\n');
            }
        }
        hash.update(row.as_bytes());
        for child in file.cst().children(id) {
            snapshot_node(file, child, hash, count);
        }
    }

    fn push_json_string(output: &mut String, text: &str) {
        output.push_str(&serde_json::to_string(text).expect("token text should serialize as JSON"));
    }

    struct Fnv1a64(u64);

    impl Fnv1a64 {
        const fn new() -> Self {
            Self(0xcbf2_9ce4_8422_2325)
        }

        fn update(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.0 ^= u64::from(*byte);
                self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }

        fn finish(self) -> String {
            format!("{:016x}", self.0)
        }
    }
}
