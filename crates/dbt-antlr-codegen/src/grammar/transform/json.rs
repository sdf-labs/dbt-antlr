// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Minimal JSON-like writer for the optimization manifest.
//!
//! Used for writing the inspection-only optimization manifest. Avoids a
//! `serde_json` dependency bloat.

use std::borrow::Cow;
use std::fmt::Write as _;

pub(crate) enum Json<'a> {
    Null,
    Bool(bool),
    Number(u64),
    String(Cow<'a, str>),
    Array(Vec<Json<'a>>),
    Object(Vec<(&'static str, Json<'a>)>),
}

impl<'a> Json<'a> {
    pub(crate) fn string(text: &'a str) -> Self {
        Self::String(Cow::Borrowed(text))
    }

    pub(crate) fn to_pretty_string(&self) -> String {
        let mut output = String::new();
        write_value(&mut output, self, 0);
        output.push('\n');
        output
    }
}

impl From<usize> for Json<'_> {
    fn from(value: usize) -> Self {
        Self::Number(value as u64)
    }
}

impl From<u32> for Json<'_> {
    fn from(value: u32) -> Self {
        Self::Number(u64::from(value))
    }
}

fn write_value(output: &mut String, value: &Json<'_>, depth: usize) {
    match value {
        Json::Null => output.push_str("null"),
        Json::Bool(boolean) => output.push_str(if *boolean { "true" } else { "false" }),
        Json::Number(number) => write!(output, "{number}").expect("writing to String cannot fail"),
        Json::String(text) => write_string(output, text),
        Json::Array(items) => write_container(
            output,
            depth,
            '[',
            ']',
            items.iter(),
            |output, depth, item| write_value(output, item, depth),
        ),
        Json::Object(entries) => write_container(
            output,
            depth,
            '{',
            '}',
            entries.iter(),
            |output, depth, (key, value)| {
                write_string(output, key);
                output.push_str(": ");
                write_value(output, value, depth);
            },
        ),
    }
}

fn write_container<T>(
    output: &mut String,
    depth: usize,
    open: char,
    close: char,
    items: impl Iterator<Item = T>,
    write_item: impl Fn(&mut String, usize, T),
) {
    let items: Vec<T> = items.collect();
    if items.is_empty() {
        output.push(open);
        output.push(close);
        return;
    }
    output.push(open);
    output.push('\n');
    for (index, item) in items.into_iter().enumerate() {
        if index > 0 {
            output.push_str(",\n");
        }
        write_indent(output, depth + 1);
        write_item(output, depth + 1, item);
    }
    output.push('\n');
    write_indent(output, depth);
    output.push(close);
}

fn write_indent(output: &mut String, depth: usize) {
    for _ in 0..depth {
        output.push_str("  ");
    }
}

fn write_string(output: &mut String, text: &str) {
    output.push('"');
    for ch in text.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            ch if ch < '\u{20}' => {
                write!(output, "\\u{:04x}", ch as u32).expect("writing to String cannot fail");
            }
            ch => output.push(ch),
        }
    }
    output.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pretty_prints_nested_containers() {
        let value = Json::Object(vec![
            ("version", 1usize.into()),
            ("reportOnly", Json::Bool(false)),
            (
                "passes",
                Json::Array(vec![Json::Object(vec![
                    ("name", Json::string("pass")),
                    ("rungs", Json::Array(vec![])),
                    ("boundary", Json::Null),
                ])]),
            ),
        ]);
        assert_eq!(
            value.to_pretty_string(),
            "{\n  \"version\": 1,\n  \"reportOnly\": false,\n  \"passes\": [\n    {\n      \"name\": \"pass\",\n      \"rungs\": [],\n      \"boundary\": null\n    }\n  ]\n}\n"
        );
    }

    #[test]
    fn escapes_strings_like_json() {
        let value = Json::Array(vec![Json::string("a\"b\\c\nd\te\u{01}fé")]);
        assert_eq!(
            value.to_pretty_string(),
            "[\n  \"a\\\"b\\\\c\\nd\\te\\u0001fé\"\n]\n"
        );
    }

    #[test]
    fn prints_empty_object_inline() {
        let value = Json::Object(vec![("empty", Json::Object(vec![]))]);
        assert_eq!(value.to_pretty_string(), "{\n  \"empty\": {}\n}\n");
    }
}
