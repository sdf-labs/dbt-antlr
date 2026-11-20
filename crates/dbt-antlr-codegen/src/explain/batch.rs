// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Parser for `--explain-batch` multi-input files.
//!
//! Each case starts with a separator line that begins and ends with the
//! delimiter (default `---`), with no leading or trailing whitespace:
//!
//! ```text
//! --- case-name ---
//! input text
//! --- expr-only [start_rule=expr] ---
//! 1 + 2 * 3
//! ```

use super::ExplainCase;

/// Default delimiter for `--explain-batch` separator lines.
pub(crate) const DEFAULT_DELIMITER: &str = "---";

/// Parses a batch file into cases using `delimiter`.
pub(crate) fn parse_batch(
    content: &str,
    source_name: &str,
    delimiter: &str,
) -> Result<Vec<ExplainCase>, String> {
    if delimiter.is_empty() || delimiter.chars().any(char::is_whitespace) {
        return Err(format!(
            "invalid --explain-delimiter '{delimiter}': delimiter must be non-empty and contain no whitespace"
        ));
    }

    let mut cases = Vec::new();
    let mut current: Option<(String, Option<String>, usize)> = None;
    let mut offset = 0_usize;
    let mut line_number = 0_usize;
    let mut case_index = 0_usize;

    while offset < content.len() {
        line_number += 1;
        let rest = &content[offset..];
        let (line_with_ending, next_offset) = match rest.find('\n') {
            Some(pos) => (&rest[..=pos], offset + pos + 1),
            None => (rest, content.len()),
        };
        let line = strip_line_ending(line_with_ending);

        if let Some(header) = parse_header_line(line, delimiter, source_name, line_number)? {
            if let Some((input_name, start_rule, body_start)) = current.take() {
                let raw_body = &content[body_start..offset];
                cases.push(ExplainCase {
                    input: strip_one_trailing_newline(raw_body).to_owned(),
                    input_name,
                    start_rule,
                });
            }
            case_index += 1;
            let input_name = header
                .name
                .unwrap_or_else(|| format!("{source_name} #{case_index}"));
            current = Some((input_name, header.start_rule, next_offset));
        } else if current.is_none() && !line.trim().is_empty() {
            return Err(format!(
                "batch file {source_name}:{line_number}: expected a separator header like '{delimiter} name {delimiter}' before input content"
            ));
        }

        offset = next_offset;
    }

    if let Some((input_name, start_rule, body_start)) = current {
        let raw_body = &content[body_start..];
        cases.push(ExplainCase {
            input: strip_one_trailing_newline(raw_body).to_owned(),
            input_name,
            start_rule,
        });
    }

    if cases.is_empty() {
        return Err(format!("batch file {source_name} has no cases"));
    }

    Ok(cases)
}

struct Header {
    name: Option<String>,
    start_rule: Option<String>,
}

/// Parses one strict untrimmed separator line.
///
/// Returns `Ok(None)` when `line` is ordinary input text, `Ok(Some(_))` when it
/// is a valid separator line, and `Err(_)` when a separator line has a
/// malformed `[start_rule=...]` suffix.
fn parse_header_line(
    line: &str,
    delimiter: &str,
    source_name: &str,
    line_number: usize,
) -> Result<Option<Header>, String> {
    let Some(after_open) = line.strip_prefix(delimiter) else {
        return Ok(None);
    };
    let Some(inner) = after_open.strip_suffix(delimiter) else {
        return Ok(None);
    };
    if !inner.starts_with(' ') || !inner.ends_with(' ') {
        return Ok(None);
    }

    let body = inner.trim();
    if body.is_empty() {
        return Ok(Some(Header {
            name: None,
            start_rule: None,
        }));
    }

    let (name_part, start_rule) = if let Some(open) = body.find('[') {
        let before = &body[..open];
        if before.contains(']') {
            return Err(format!(
                "batch file {source_name}:{line_number}: unexpected ']' in header '{line}'"
            ));
        }
        if !before.is_empty() && !before.ends_with(char::is_whitespace) {
            return Err(format!(
                "batch file {source_name}:{line_number}: expected whitespace before [...] option in header '{line}'"
            ));
        }
        let Some((option, after)) = body[open + 1..].split_once(']') else {
            return Err(format!(
                "batch file {source_name}:{line_number}: unclosed '[' in header '{line}'"
            ));
        };
        if !after.is_empty() {
            return Err(format!(
                "batch file {source_name}:{line_number}: unexpected trailing text after '[{option}]' in header '{line}'"
            ));
        }
        let Some(rule_value) = option.strip_prefix("start_rule=") else {
            return Err(format!(
                "batch file {source_name}:{line_number}: invalid header option '[{option}]'; expected '[start_rule=<rule>]'"
            ));
        };
        if rule_value.is_empty()
            || rule_value
                .chars()
                .any(|ch| ch.is_whitespace() || ch == '[' || ch == ']')
        {
            return Err(format!(
                "batch file {source_name}:{line_number}: invalid start_rule value '{rule_value}' in header '{line}'"
            ));
        }
        (before.trim(), Some(rule_value.to_owned()))
    } else if body.contains(']') {
        return Err(format!(
            "batch file {source_name}:{line_number}: unexpected ']' in header '{line}'"
        ));
    } else {
        (body, None)
    };

    let name = (!name_part.is_empty()).then(|| name_part.to_owned());
    Ok(Some(Header { name, start_rule }))
}

fn strip_line_ending(line: &str) -> &str {
    let line = line.strip_suffix('\n').unwrap_or(line);
    line.strip_suffix('\r').unwrap_or(line)
}

fn strip_one_trailing_newline(text: &str) -> &str {
    let Some(without_lf) = text.strip_suffix('\n') else {
        return text;
    };
    without_lf.strip_suffix('\r').unwrap_or(without_lf)
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_DELIMITER, ExplainCase, parse_batch};

    #[test]
    fn parses_named_unnamed_and_start_rule_cases() {
        let input = "\n--- first ---\na b c ;\n--- expr-only [start_rule=expr] ---\n1 + 2\n--- [start_rule=stat] ---\nx y ;\n--- ---\n\n";
        let cases = parse_batch(input, "cases.txt", DEFAULT_DELIMITER).expect("batch parses");
        assert_eq!(
            cases,
            vec![
                ExplainCase {
                    input: "a b c ;".to_owned(),
                    input_name: "first".to_owned(),
                    start_rule: None,
                },
                ExplainCase {
                    input: "1 + 2".to_owned(),
                    input_name: "expr-only".to_owned(),
                    start_rule: Some("expr".to_owned()),
                },
                ExplainCase {
                    input: "x y ;".to_owned(),
                    input_name: "cases.txt #3".to_owned(),
                    start_rule: Some("stat".to_owned()),
                },
                ExplainCase {
                    input: String::new(),
                    input_name: "cases.txt #4".to_owned(),
                    start_rule: None,
                },
            ]
        );
    }

    #[test]
    fn ignores_non_strict_separator_lines_inside_body() {
        let input = "\
--- sql-comments ---
--- bare sql comment
------
  --- indented ---
--- trailing-space --- 
SELECT 1;
--- second ---
SELECT 2;
";
        let cases = parse_batch(input, "sql.txt", DEFAULT_DELIMITER).expect("batch parses");
        assert_eq!(cases.len(), 2);
        assert_eq!(
            cases[0].input,
            "--- bare sql comment\n------\n  --- indented ---\n--- trailing-space --- \nSELECT 1;"
        );
        assert_eq!(cases[1].input, "SELECT 2;");
    }

    #[test]
    fn supports_crlf_and_custom_delimiter() {
        let input = "=== one ===\r\na\r\nb\r\n=== two [start_rule=r] ===\r\nc\r\n";
        let cases = parse_batch(input, "win.txt", "===").expect("batch parses");
        assert_eq!(
            cases,
            vec![
                ExplainCase {
                    input: "a\r\nb".to_owned(),
                    input_name: "one".to_owned(),
                    start_rule: None,
                },
                ExplainCase {
                    input: "c".to_owned(),
                    input_name: "two".to_owned(),
                    start_rule: Some("r".to_owned()),
                },
            ]
        );
    }

    #[test]
    fn rejects_preamble_empty_batch_and_malformed_headers() {
        assert!(parse_batch("stray text\n--- a ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("  \n\n", "empty.txt", "---").is_err());
        assert!(parse_batch("--- bad [start_rule=] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad [start_rule=a b] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad [start_rule=a] extra ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad[start_rule=a] ---\nx", "bad.txt", "---").is_err());
        assert!(
            parse_batch(
                "--- bad [start_rule=a] [start_rule=b] ---\nx",
                "bad.txt",
                "---"
            )
            .is_err()
        );
        assert!(parse_batch("--- bad [start_rule=a]b] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad [start_rule = a] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad [start-rule=a] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad [start_rule] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad [start_rule=a ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- bad] ---\nx", "bad.txt", "---").is_err());
        assert!(parse_batch("--- a ---\nx", "bad.txt", "").is_err());
        assert!(parse_batch("--- a ---\nx", "bad.txt", "- -").is_err());
    }
}
