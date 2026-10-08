// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! minijinja environment for the dbt emission layer.
//!
//! The template source is embedded into the binary with `include_str!`.
//! When the `DBT_RUST_TEMPLATE` environment variable is set, the file at
//! that path is read instead, so template edits do not need a Rust rebuild
//! during development.
//!
//! The environment mirrors the ST4 machinery `Rust.stg` relies on:
//!
//! - `cap` filter — ST4 `format="cap"`.
//! - `rust_type` / `rust_type_init` functions — the `RustTypeMap` and
//!   `rustTypeInitMap` dictionaries.
//! - `st_quote` / `st_vocab_entry` filters — the `{r | "<r>"}` and
//!   `{t | Some(<t>)}; null="None"` anonymous subtemplates.
//! - `st_indent` filter — ST4's `AutoIndentWriter` indentation of every
//!   non-empty line of a value rendered on an indented line.
//! - `wrap_list` filter — ST4's `separator="...", wrap, anchor` line wrapping
//!   against the ANTLR line width of 72 columns.

use minijinja::{Environment, Error, Value};

/// Environment variable overriding the embedded template with a file path
/// (development loop).
pub const TEMPLATE_PATH_ENV_VAR: &str = "DBT_RUST_TEMPLATE";

/// The template source embedded at compile time.
const EMBEDDED_TEMPLATE: &str = include_str!("../../templates/dbt/Rust.stg.jinja");

/// Line width the ANTLR code generator configures on its ST4 writer.
const LINE_WIDTH: usize = 72;

/// Marks an ST4 `wrap` point (without `anchor`) in rendered output. The
/// template emits it through the `ST_WRAP` global; [`resolve_wraps`] turns
/// it into a line break or drops it once absolute columns are known.
const WRAP_MARK: char = '\u{E000}';

/// Resolves the template source for this render.
fn template_source() -> Result<String, Error> {
    if let Some(path) = std::env::var_os(TEMPLATE_PATH_ENV_VAR) {
        return std::fs::read_to_string(path).map_err(|error| {
            Error::new(
                minijinja::ErrorKind::InvalidOperation,
                format!("cannot read dbt Rust template: {error}"),
            )
        });
    }
    Ok(EMBEDDED_TEMPLATE.to_owned())
}

/// Renders the template with the given model context.
///
/// The template source is re-parsed on every call.
///
/// # Errors
///
/// Returns an error when the template file named by
/// [`TEMPLATE_PATH_ENV_VAR`] cannot be read, or when the template cannot be
/// parsed or rendered.
pub fn render_template(ctx: &Value) -> Result<String, Error> {
    let source = template_source()?;
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(false);
    env.add_filter("cap", cap_filter);
    env.add_filter("st_quote", st_quote_filter);
    env.add_filter("st_vocab_entry", st_vocab_entry_filter);
    env.add_filter("st_token_ref", st_token_ref_filter);
    env.add_filter("st_case_label", st_case_label_filter);
    env.add_filter("st_indent", st_indent_filter);
    env.add_filter("wrap_list", wrap_list_filter);
    env.add_function("rust_type", rust_type_function);
    env.add_function("rust_type_init", rust_type_init_function);
    env.add_global("ST_WRAP", WRAP_MARK.to_string());
    env.add_global("RUNTIME_VERSION_MAJOR", dbt_antlr_runtime::VERSION_MAJOR);
    env.add_global("RUNTIME_VERSION_MINOR", dbt_antlr_runtime::VERSION_MINOR);
    let template = env.template_from_str(&source)?;
    Ok(resolve_wraps(&template.render(ctx)?))
}

/// Resolves the [`WRAP_MARK`] wrap points like ST4's `AutoIndentWriter`:
/// when the line has reached [`LINE_WIDTH`], a mark becomes a newline plus
/// the indentation of the current line; otherwise it is dropped.
fn resolve_wraps(text: &str) -> String {
    if !text.contains(WRAP_MARK) {
        return text.to_owned();
    }
    let mut out = String::with_capacity(text.len());
    let mut line_start = 0;
    let mut column = 0;
    for c in text.chars() {
        match c {
            '\n' => {
                out.push(c);
                line_start = out.len();
                column = 0;
            }
            WRAP_MARK => {
                if column >= LINE_WIDTH {
                    let indent: String = out[line_start..]
                        .chars()
                        .take_while(|c| matches!(c, ' ' | '\t'))
                        .collect();
                    out.push('\n');
                    line_start = out.len();
                    column = indent.chars().count();
                    out.push_str(&indent);
                }
            }
            _ => {
                out.push(c);
                column += 1;
            }
        }
    }
    out
}

/// ST4 `format="cap"`: uppercases the first character.
fn cap_filter(value: &Value) -> String {
    let text = value.to_string();
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };
    first.to_uppercase().chain(chars).collect()
}

/// ST4 anonymous subtemplate `{r | "<r>"}`.
fn st_quote_filter(value: &Value) -> String {
    format!("\"{value}\"")
}

/// ST4 `{t | Some(<t>)}; null="None"`; the value already carries its Rust
/// string-literal quoting.
fn st_vocab_entry_filter(value: &Value) -> String {
    if value.is_none() {
        "None".to_owned()
    } else {
        format!("Some({value})")
    }
}

/// The `<parser.grammarName>_<t.name>` token-constant reference of the
/// parser `bitsetInlineComparison` template; `value` is a `TokenInfo` (or
/// a plain name string).
fn st_token_ref_filter(value: &Value, grammar_name: &str) -> String {
    format!("{grammar_name}_{}", token_info_name(value))
}

/// The `{t | <parser.grammarName>_<t.name> }` map body of the parser
/// `cases` template: the token constant reference with its trailing space.
fn st_case_label_filter(value: &Value, grammar_name: &str) -> String {
    format!("{grammar_name}_{} ", token_info_name(value))
}

fn token_info_name(value: &Value) -> String {
    value
        .get_attr("name")
        .map_or_else(|_| value.to_string(), |name| name.to_string())
}

/// ST4 `AutoIndentWriter` indentation: prefixes every non-empty line of the
/// value with `indent`.
fn st_indent_filter(value: &Value, indent: &str) -> String {
    let text = value.to_string();
    text.split('\n')
        .map(|line| {
            if line.is_empty() {
                line.to_owned()
            } else {
                format!("{indent}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// ST4 `<list; separator=", ", wrap, anchor>` against a line width of 72.
///
/// `items` are the pre-rendered elements, `separator` is written verbatim
/// between elements, `start_col` is the column at which the first element
/// starts, `indent` is the indentation string active at the call site, and
/// `anchor` is the column wrapped lines pad to when it exceeds the indent
/// width. Wraps happen before an element once the current column reaches the
/// line width; the separator stays at the end of the previous line.
fn wrap_list_filter(
    items: Vec<Value>,
    separator: &str,
    start_col: u32,
    indent: &str,
    anchor: u32,
) -> String {
    let mut out = String::new();
    let mut column = start_col as usize;
    let mut at_line_start = start_col == 0;
    for (index, item) in items.into_iter().enumerate() {
        if index > 0 {
            out.push_str(separator);
            match separator.rfind('\n') {
                Some(position) => {
                    column = separator.len() - position - 1;
                    at_line_start = true;
                }
                None => column += separator.len(),
            }
        }
        if column >= LINE_WIDTH && !at_line_start {
            out.push('\n');
            out.push_str(indent);
            column = indent.len();
            let anchor = anchor as usize;
            if anchor > column {
                out.push_str(&" ".repeat(anchor - column));
                column = anchor;
            }
        }
        let text = item.to_string();
        column += text.chars().count();
        at_line_start = false;
        out.push_str(&text);
    }
    out
}

/// The `RustTypeMap` dictionary of `Rust.stg`.
fn rust_type_function(type_name: &str) -> String {
    match type_name {
        "int" => "i32".to_owned(),
        "string" => "String".to_owned(),
        _ => type_name.to_owned(),
    }
}

/// The `rustTypeInitMap` dictionary of `Rust.stg`.
fn rust_type_init_function(type_name: &str) -> String {
    match type_name {
        "int" | "long" => "0".to_owned(),
        "float" => "0.0f".to_owned(),
        "double" => "0.0".to_owned(),
        "boolean" => "false".to_owned(),
        "byte" | "short" | "char" => "0".to_owned(),
        "String" => "String::new()".to_owned(),
        "Vec<String>" => "Vec::new()".to_owned(),
        _ => "null".to_owned(),
    }
}
