// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Port of the Java tool's `ActionTranslator` string-substitution pipeline
//! (`tool/src/org/antlr/v4/codegen/ActionTranslator.java`) together with the
//! `ActionSplitter.g` filter lexer it is driven by.
//!
//! The splitter is a hand-rolled scanner over the action text that reproduces
//! the ANTLR3 filter-lexer semantics of `ActionSplitter.g` (maximal munch,
//! rule-priority tie-breaking, `\\$` escapes, `$` not followed by an ID start
//! passing through as text; there is no string/char-literal state, so `$x`
//! inside `"..."` is translated, matching the Java tool). The translator
//! turns the split events into [`ActionChunkModel`]s exactly like the Java
//! `ActionSplitterListener` methods (`attr`, `qualifiedAttr`, `setAttr`,
//! `text`), resolving `$x` / `$x.y` references against the attribute scopes
//! of the enclosing outer-most alternative.

use std::collections::{BTreeMap, BTreeSet};

use crate::dbt::emit::EmitError;
use crate::dbt::lexer_factory::escape_if_needed;
use crate::dbt::model::{ActionChunkModel, ChunkContextModel};

/// Predefined rule properties (`Rule.predefinedRulePropertiesDict`), the
/// `PREDEFINED_RULE` attribute dictionary.
const RULE_PROPERTIES: [&str; 5] = ["parser", "text", "start", "stop", "ctx"];
/// Predefined token properties (the `TOKEN` attribute dictionary behind
/// `LabelType.TOKEN_LABEL`).
const TOKEN_PROPERTIES: [&str; 7] = ["text", "type", "line", "index", "pos", "channel", "int"];

/// The label dictionary of an outer-most alternative (`LabelElementPair`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LabelDef {
    /// `x=e` on a rule reference; carries the referenced rule name.
    Rule(String),
    /// `x=ID` on a token reference.
    Token,
    /// `x+=e` on a rule reference; carries the referenced rule name.
    RuleList(String),
    /// `x+=ID` on a token reference.
    TokenList,
}

/// The attribute scopes the action translator resolves against: the Java
/// `Alternative` of the enclosing outer-most alternative (token/rule
/// references, label definitions) plus the rule- and grammar-level
/// dictionaries.
#[derive(Clone, Debug, Default)]
pub(crate) struct ActionScope {
    /// Token references of the outer-most alternative (`alt.tokenRefs`
    /// keys): token names and string literals as authored (with quotes).
    pub token_refs: BTreeSet<String>,
    /// Rule references of the outer-most alternative (`alt.ruleRefs` keys).
    pub rule_refs: BTreeSet<String>,
    /// Label definitions of the outer-most alternative (`alt.labelDefs`).
    pub label_defs: BTreeMap<String, LabelDef>,
    /// Rule argument names (`r.args`).
    pub args: BTreeSet<String>,
    /// Rule return-value names (`r.retvals`).
    pub retvals: BTreeSet<String>,
    /// Rule local names (`r.locals`).
    pub locals: BTreeSet<String>,
    /// Return-value names of every rule of the grammar (for `$x.y` where
    /// `x` is a rule reference or rule label).
    pub rule_retvals: BTreeMap<String, BTreeSet<String>>,
    /// All rule names of the grammar (`factory.getGrammar().getRule`).
    pub rule_names: BTreeSet<String>,
    /// Token references also referenced in actions of the alternative
    /// (`alt.tokenRefsInActions` keys, the `ActionSniffer` output); the
    /// implicit-label trigger.
    pub token_refs_in_actions: BTreeSet<String>,
    /// Rule references also referenced in actions of the alternative
    /// (`alt.ruleRefsInActions` keys).
    pub rule_refs_in_actions: BTreeSet<String>,
}

impl ActionScope {
    /// `Alternative.resolvesToToken`.
    fn resolves_to_token(&self, x: &str) -> bool {
        self.token_refs.contains(x) || matches!(self.label_defs.get(x), Some(LabelDef::Token))
    }

    /// `Alternative.resolvesToLabel`.
    fn resolves_to_label(&self, x: &str) -> bool {
        matches!(
            self.label_defs.get(x),
            Some(LabelDef::Rule(_) | LabelDef::Token)
        )
    }

    /// `Alternative.resolvesToListLabel`.
    fn resolves_to_list_label(&self, x: &str) -> bool {
        matches!(
            self.label_defs.get(x),
            Some(LabelDef::RuleList(_) | LabelDef::TokenList)
        )
    }

    /// `Rule.resolveToAttribute`: rule arguments, return values, locals,
    /// then the predefined rule properties.
    fn resolve_to_attribute(&self, x: &str) -> Option<AttributeKind> {
        if self.args.contains(x) {
            Some(AttributeKind::Arg)
        } else if self.retvals.contains(x) {
            Some(AttributeKind::Ret)
        } else if self.locals.contains(x) {
            Some(AttributeKind::Local)
        } else if RULE_PROPERTIES.contains(&x) {
            Some(AttributeKind::PredefinedRule)
        } else {
            None
        }
    }
}

/// The `AttributeDict.DictType` cases `ActionTranslator` switches on.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttributeKind {
    Arg,
    Ret,
    Local,
    PredefinedRule,
    Token,
}

const fn unsupported(message: String) -> EmitError {
    EmitError::Unsupported(message)
}

/// The events of the `ActionSplitter.g` filter lexer; the Java
/// `ActionSplitterListener` methods without the model-building behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SplitEvent {
    /// `text`: a run of random text (comments included).
    Text(String),
    /// `attr`: `$x`.
    Attr(String),
    /// `qualifiedAttr`: `$x.y` (not followed by `(`).
    QualifiedAttr(String, String),
    /// `setAttr`: `$x = rhs;` (`rhs` up to the first `;`).
    SetAttr(String, String),
    /// `nonLocalAttr`: `$x::y`.
    NonLocalAttr(String, String),
    /// `setNonLocalAttr`: `$x::y = rhs;`.
    SetNonLocalAttr(String, String, String),
}

const fn is_id_start(c: u8) -> bool {
    c == b'_' || c.is_ascii_alphabetic()
}

const fn is_id_part(c: u8) -> bool {
    is_id_start(c) || c.is_ascii_digit()
}

/// `ID` at `bytes[start]` (an ID start); returns the end offset.
const fn scan_id(bytes: &[u8], start: usize) -> usize {
    let mut end = start + 1;
    while end < bytes.len() && is_id_part(bytes[end]) {
        end += 1;
    }
    end
}

/// The `WS? '=' ATTR_VALUE_EXPR ';'` tail of `SET_ATTR`/`SET_NONLOCAL_ATTR`
/// at `bytes[start]`. `ATTR_VALUE_EXPR` is `~'=' (~';')*`: at least one
/// character, the first not `=`, up to the first `;`. Returns the right-hand
/// side text and the offset just past the `;`.
fn scan_assignment<'a>(
    action: &'a str,
    bytes: &[u8],
    mut start: usize,
) -> Option<(&'a str, usize)> {
    while start < bytes.len() && matches!(bytes[start], b' ' | b'\t' | b'\n' | b'\r') {
        start += 1;
    }
    if bytes.get(start) != Some(&b'=') {
        return None;
    }
    let rhs_start = start + 1;
    if bytes.get(rhs_start).is_none_or(|c| *c == b'=') {
        return None;
    }
    let semi = bytes[rhs_start..].iter().position(|c| *c == b';')? + rhs_start;
    Some((&action[rhs_start..semi], semi + 1))
}

/// Splits an action body (already trimmed of its enclosing `{...}`) into
/// splitter events, the `ActionSplitter.getActionTokens` drive.
pub(crate) fn split_action(action: &str) -> Vec<SplitEvent> {
    let bytes = action.as_bytes();
    let mut events = Vec::new();
    let mut text = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'/' && bytes.get(i + 1) == Some(&b'*') {
            // COMMENT: '/*' (nongreedy .)* '*/'
            if let Some(close) = action[i + 2..].find("*/") {
                let end = i + 2 + close + 2;
                text.push_str(&action[i..end]);
                i = end;
                continue;
            }
        }
        if c == b'/' && bytes.get(i + 1) == Some(&b'/') {
            // LINE_COMMENT: '//' ~('\n'|'\r')* '\r'? '\n'
            let mut end = i + 2;
            while end < bytes.len() && !matches!(bytes[end], b'\n' | b'\r') {
                end += 1;
            }
            let terminated = if bytes.get(end) == Some(&b'\n') {
                Some(end + 1)
            } else if bytes.get(end) == Some(&b'\r') && bytes.get(end + 1) == Some(&b'\n') {
                Some(end + 2)
            } else {
                None
            };
            if let Some(end) = terminated {
                text.push_str(&action[i..end]);
                i = end;
                continue;
            }
        }
        if c == b'$' && bytes.get(i + 1).is_some_and(|next| is_id_start(*next)) {
            let x_end = scan_id(bytes, i + 1);
            let x = &action[i + 1..x_end];
            if bytes[x_end..].starts_with(b"::")
                && bytes.get(x_end + 2).is_some_and(|next| is_id_start(*next))
            {
                let y_end = scan_id(bytes, x_end + 2);
                let y = &action[x_end + 2..y_end];
                if let Some((rhs, end)) = scan_assignment(action, bytes, y_end) {
                    flush_text(&mut events, &mut text);
                    events.push(SplitEvent::SetNonLocalAttr(
                        x.to_owned(),
                        y.to_owned(),
                        rhs.to_owned(),
                    ));
                    i = end;
                    continue;
                }
                flush_text(&mut events, &mut text);
                events.push(SplitEvent::NonLocalAttr(x.to_owned(), y.to_owned()));
                i = y_end;
                continue;
            }
            if bytes.get(x_end) == Some(&b'.')
                && bytes.get(x_end + 1).is_some_and(|next| is_id_start(*next))
            {
                let y_end = scan_id(bytes, x_end + 1);
                // QUALIFIED_ATTR is gated on {input.LA(1)!='('}?
                if bytes.get(y_end) != Some(&b'(') {
                    let y = &action[x_end + 1..y_end];
                    flush_text(&mut events, &mut text);
                    events.push(SplitEvent::QualifiedAttr(x.to_owned(), y.to_owned()));
                    i = y_end;
                    continue;
                }
            } else if let Some((rhs, end)) = scan_assignment(action, bytes, x_end) {
                flush_text(&mut events, &mut text);
                events.push(SplitEvent::SetAttr(x.to_owned(), rhs.to_owned()));
                i = end;
                continue;
            }
            flush_text(&mut events, &mut text);
            events.push(SplitEvent::Attr(x.to_owned()));
            i = x_end;
            continue;
        }
        // TEXT, one iteration of the `(...)+` loop.
        if c == b'\\' && bytes.get(i + 1) == Some(&b'$') {
            // '\\$' renders as a plain '$'.
            text.push('$');
            i += 2;
        } else if c == b'\\' && i + 1 < bytes.len() {
            // '\\' c=~('$') renders as the backslash and the character.
            let next = action[i + 1..].chars().next().expect("byte boundary");
            text.push('\\');
            text.push(next);
            i += 1 + next.len_utf8();
        } else if c == b'\\' {
            // A lone trailing backslash matches no TEXT alternative; the
            // ANTLR3 filter lexer drops it (no-viable-alternative recovery).
            i += 1;
        } else if c == b'$' {
            // {\!isIDStartChar(input.LA(2))}? => '$'
            text.push('$');
            i += 1;
        } else {
            let next = action[i..].chars().next().expect("byte boundary");
            text.push(next);
            i += next.len_utf8();
        }
    }
    flush_text(&mut events, &mut text);
    events
}

fn flush_text(events: &mut Vec<SplitEvent>, text: &mut String) {
    if !text.is_empty() {
        events.push(SplitEvent::Text(std::mem::take(text)));
    }
}

/// Translates one embedded action body into chunks: the Java
/// `ActionTranslator.translateAction` (the body arrives without its
/// enclosing braces) with `rf` fixed to the current rule function.
///
/// `node_ctx` is the escaped name of the `StructDecl` the action lives in
/// (the alt-label context for actions in labeled alternatives, else the
/// rule context); `rule_ctx` is the escaped name of the rule's own context
/// struct (`rf.ruleCtx`, used by `RetValueRef`).
pub(crate) fn translate_action(
    body: &str,
    scope: &ActionScope,
    node_ctx: &str,
    rule_ctx: &str,
) -> Result<Vec<ActionChunkModel>, EmitError> {
    let translator = Translator {
        scope,
        node_ctx,
        rule_ctx,
    };
    translator.translate(body)
}

struct Translator<'a> {
    scope: &'a ActionScope,
    node_ctx: &'a str,
    rule_ctx: &'a str,
}

impl Translator<'_> {
    fn translate(&self, body: &str) -> Result<Vec<ActionChunkModel>, EmitError> {
        let mut chunks = Vec::new();
        for event in split_action(body) {
            match event {
                SplitEvent::Text(text) => chunks.push(ActionChunkModel::ActionText {
                    ctx: self.ctx(),
                    text,
                }),
                SplitEvent::Attr(x) => self.attr(&mut chunks, &x),
                SplitEvent::QualifiedAttr(x, y) => self.qualified_attr(&mut chunks, &x, &y)?,
                SplitEvent::SetAttr(x, rhs) => {
                    // setAttr: the right-hand side is translated recursively.
                    let rhs_chunks = self.translate(&rhs)?;
                    chunks.push(ActionChunkModel::SetAttr {
                        ctx: self.ctx(),
                        name: x.clone(),
                        escaped_name: escape_if_needed(&x),
                        rhs_chunks,
                    });
                }
                SplitEvent::NonLocalAttr(x, y) => {
                    return Err(unsupported(format!(
                        "non-local attribute reference ${x}::{y}"
                    )));
                }
                SplitEvent::SetNonLocalAttr(x, y, _) => {
                    return Err(unsupported(format!(
                        "non-local attribute assignment ${x}::{y}"
                    )));
                }
            }
        }
        Ok(chunks)
    }

    fn ctx(&self) -> ChunkContextModel {
        ChunkContextModel::new(self.node_ctx)
    }

    /// `ActionTranslator.attr`. The checks run in the Java order and are
    /// intentionally not exclusive (the Java method does not return after
    /// the attribute branch).
    fn attr(&self, chunks: &mut Vec<ActionChunkModel>, x: &str) {
        if let Some(kind) = self.scope.resolve_to_attribute(x) {
            match kind {
                AttributeKind::Arg => chunks.push(ActionChunkModel::ArgRef {
                    ctx: self.ctx(),
                    name: x.to_owned(),
                    escaped_name: escape_if_needed(x),
                }),
                // RetValueRef goes to `rf.ruleCtx`, not the node context.
                AttributeKind::Ret => chunks.push(ActionChunkModel::RetValueRef {
                    ctx: ChunkContextModel::new(self.rule_ctx),
                    name: x.to_owned(),
                    escaped_name: escape_if_needed(x),
                }),
                AttributeKind::Local => chunks.push(ActionChunkModel::LocalRef {
                    ctx: self.ctx(),
                    name: x.to_owned(),
                    escaped_name: escape_if_needed(x),
                }),
                AttributeKind::PredefinedRule => {
                    chunks.push(self.this_rule_property_ref(x));
                }
                // `Rule.resolveToAttribute` never yields a token attribute;
                // the Java switch's default branch does nothing.
                AttributeKind::Token => {}
            }
        }
        if self.scope.resolves_to_token(x) {
            chunks.push(ActionChunkModel::TokenRef {
                ctx: self.ctx(),
                name: x.to_owned(),
                escaped_name: escape_if_needed(x),
            });
            return;
        }
        if self.scope.resolves_to_label(x) {
            chunks.push(ActionChunkModel::LabelRef {
                ctx: self.ctx(),
                name: x.to_owned(),
                escaped_name: escape_if_needed(x),
            });
            return;
        }
        if self.scope.resolves_to_list_label(x) {
            chunks.push(ActionChunkModel::ListLabelRef {
                ctx: self.ctx(),
                name: x.to_owned(),
                escaped_name: escape_if_needed(x),
            });
            return;
        }
        if self.scope.rule_names.contains(x) {
            chunks.push(ActionChunkModel::LabelRef {
                ctx: self.ctx(),
                name: x.to_owned(),
                escaped_name: escape_if_needed(x),
            });
        }
    }

    /// `ActionTranslator.qualifiedAttr`.
    fn qualified_attr(
        &self,
        chunks: &mut Vec<ActionChunkModel>,
        x: &str,
        y: &str,
    ) -> Result<(), EmitError> {
        if self.scope.resolve_to_attribute(x).is_some() {
            // A member access to a predefined attribute like $ctx.foo.
            self.attr(chunks, x);
            chunks.push(ActionChunkModel::ActionText {
                ctx: self.ctx(),
                text: format!(".{y}"),
            });
            return Ok(());
        }
        match self.resolve_qualified(x, y) {
            Some(AttributeKind::Arg) => chunks.push(ActionChunkModel::ArgRef {
                ctx: self.ctx(),
                name: y.to_owned(),
                escaped_name: escape_if_needed(y),
            }),
            Some(AttributeKind::Ret) => chunks.push(ActionChunkModel::QRetValueRef {
                ctx: self.ctx(),
                dict: x.to_owned(),
                name: y.to_owned(),
                escaped_name: escape_if_needed(y),
            }),
            Some(AttributeKind::PredefinedRule) => {
                chunks.push(self.rule_property_ref(x, y));
            }
            Some(AttributeKind::Token) => {
                chunks.push(self.token_property_ref(x, y));
            }
            Some(AttributeKind::Local) | None => {
                return Err(unsupported(format!(
                    "unknown attribute in ${x}.{y} reference"
                )));
            }
        }
        Ok(())
    }

    /// `Alternative.resolveToAttribute(x, y)`: token references first, then
    /// rule references, then label definitions.
    fn resolve_qualified(&self, x: &str, y: &str) -> Option<AttributeKind> {
        if self.scope.token_refs.contains(x) {
            return TOKEN_PROPERTIES
                .contains(&y)
                .then_some(AttributeKind::Token);
        }
        if self.scope.rule_refs.contains(x) {
            return self.resolve_retval_or_property(x, y);
        }
        match self.scope.label_defs.get(x) {
            Some(LabelDef::Rule(target)) => self.resolve_retval_or_property(target, y),
            Some(LabelDef::Token) => TOKEN_PROPERTIES
                .contains(&y)
                .then_some(AttributeKind::Token),
            // List labels have no predefined scope here (the Java
            // `getPredefinedScope` returns null for the list label types).
            Some(LabelDef::RuleList(_) | LabelDef::TokenList) | None => None,
        }
    }

    /// `Rule.resolveRetvalOrProperty`: return values, then the predefined
    /// rule properties.
    fn resolve_retval_or_property(&self, rule: &str, y: &str) -> Option<AttributeKind> {
        if self
            .scope
            .rule_retvals
            .get(rule)
            .is_some_and(|retvals| retvals.contains(y))
        {
            return Some(AttributeKind::Ret);
        }
        RULE_PROPERTIES
            .contains(&y)
            .then_some(AttributeKind::PredefinedRule)
    }

    /// `getRulePropertyRef(null, prop)`: the `ThisRulePropertyRef_*` chunks.
    fn this_rule_property_ref(&self, prop: &str) -> ActionChunkModel {
        match prop {
            "start" => ActionChunkModel::ThisRulePropertyRefStart { ctx: self.ctx() },
            "stop" => ActionChunkModel::ThisRulePropertyRefStop { ctx: self.ctx() },
            "text" => ActionChunkModel::ThisRulePropertyRefText { ctx: self.ctx() },
            "ctx" => ActionChunkModel::ThisRulePropertyRefCtx { ctx: self.ctx() },
            "parser" => ActionChunkModel::ThisRulePropertyRefParser { ctx: self.ctx() },
            other => unreachable!("predefined rule property {other}"),
        }
    }

    /// `getRulePropertyRef(x, prop)`: the `RulePropertyRef_*` chunks.
    fn rule_property_ref(&self, x: &str, prop: &str) -> ActionChunkModel {
        let label = x.to_owned();
        match prop {
            "start" => ActionChunkModel::RulePropertyRefStart {
                ctx: self.ctx(),
                label,
            },
            "stop" => ActionChunkModel::RulePropertyRefStop {
                ctx: self.ctx(),
                label,
            },
            "text" => ActionChunkModel::RulePropertyRefText {
                ctx: self.ctx(),
                label,
            },
            "ctx" => ActionChunkModel::RulePropertyRefCtx {
                ctx: self.ctx(),
                label,
            },
            "parser" => ActionChunkModel::RulePropertyRefParser {
                ctx: self.ctx(),
                label,
            },
            other => unreachable!("predefined rule property {other}"),
        }
    }

    /// `getTokenPropertyRef`: the `TokenPropertyRef_*` chunks.
    fn token_property_ref(&self, x: &str, prop: &str) -> ActionChunkModel {
        let label = x.to_owned();
        match prop {
            "text" => ActionChunkModel::TokenPropertyRefText {
                ctx: self.ctx(),
                label,
            },
            "type" => ActionChunkModel::TokenPropertyRefType {
                ctx: self.ctx(),
                label,
            },
            "line" => ActionChunkModel::TokenPropertyRefLine {
                ctx: self.ctx(),
                label,
            },
            "pos" => ActionChunkModel::TokenPropertyRefPos {
                ctx: self.ctx(),
                label,
            },
            "channel" => ActionChunkModel::TokenPropertyRefChannel {
                ctx: self.ctx(),
                label,
            },
            "index" => ActionChunkModel::TokenPropertyRefIndex {
                ctx: self.ctx(),
                label,
            },
            "int" => ActionChunkModel::TokenPropertyRefInt {
                ctx: self.ctx(),
                label,
            },
            other => unreachable!("predefined token property {other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The events of one action as `(kind, text)` pairs for compact
    /// assertions; the expected values were probed against the Java
    /// `ActionSplitter` of the dbt tool jar.
    fn events(action: &str) -> Vec<String> {
        split_action(action)
            .into_iter()
            .map(|event| match event {
                SplitEvent::Text(text) => format!("text({text:?})"),
                SplitEvent::Attr(x) => format!("attr({x})"),
                SplitEvent::QualifiedAttr(x, y) => format!("qattr({x}.{y})"),
                SplitEvent::SetAttr(x, rhs) => format!("set({x}={rhs:?})"),
                SplitEvent::NonLocalAttr(x, y) => format!("nlattr({x}::{y})"),
                SplitEvent::SetNonLocalAttr(x, y, rhs) => format!("nlset({x}::{y}={rhs:?})"),
            })
            .collect()
    }

    #[test]
    fn splitter_events_match_the_java_action_splitter() {
        assert_eq!(
            events("println!(\"{}\",$text);"),
            [
                "text(\"println!(\\\"{}\\\",\")".to_owned(),
                "attr(text)".to_owned(),
                "text(\");\")".to_owned(),
            ]
        );
        // `$` inside comments is not translated.
        assert_eq!(
            events("/* $x */ foo($y);"),
            ["text(\"/* $x */ foo(\")", "attr(y)", "text(\");\")"]
        );
        assert_eq!(
            events("// $x\n$y;"),
            ["text(\"// $x\\n\")", "attr(y)", "text(\";\")"]
        );
        // `$` inside a string literal IS translated (the Java splitter has
        // no string state).
        assert_eq!(
            events("\"$notattr\" + $real;"),
            [
                "text(\"\\\"\")",
                "attr(notattr)",
                "text(\"\\\" + \")",
                "attr(real)",
                "text(\";\")"
            ]
        );
        assert_eq!(
            events("'$' + $real;"),
            ["text(\"'$' + \")", "attr(real)", "text(\";\")"]
        );
        assert_eq!(
            events("$v = \"* \".to_owned() + $a.v + \" \" + $b.v;"),
            ["set(v=\" \\\"* \\\".to_owned() + $a.v + \\\" \\\" + $b.v\")"]
        );
        assert_eq!(events("$v = $x.v;"), ["set(v=\" $x.v\")"]);
        assert_eq!(events("$x == 3;"), ["attr(x)", "text(\" == 3;\")"]);
        // ATTR_VALUE_EXPR stops at the first ';'.
        assert_eq!(events("$x = a;b;"), ["set(x=\" a\")", "text(\"b;\")"]);
        // A method call is not a qualified attribute.
        assert_eq!(
            events("$x.y($z);"),
            ["attr(x)", "text(\".y(\")", "attr(z)", "text(\");\")"]
        );
        assert_eq!(events("$x::y = 1;"), ["nlset(x::y=\" 1\")"]);
        assert_eq!(events("$x::y;"), ["nlattr(x::y)", "text(\";\")"]);
        // '\\$' escapes to a plain '$' and the text run continues.
        assert_eq!(
            events("cost = $x + \\$5;"),
            ["text(\"cost = \")", "attr(x)", "text(\" + $5;\")"]
        );
        assert_eq!(events("$x\t=\t1;"), ["set(x=\"\\t1\")"]);
        assert_eq!(events("$x = 1"), ["attr(x)", "text(\" = 1\")"]);
    }
}
