// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-parser-dfa`, `--report-decisions`, and `--dump-dfas` support for
//! the emission layer (parser scope).

use std::collections::{BTreeSet, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;

use crate::error::{Diagnostic, Severity};
use crate::grammar::atn::CompiledParser;
use crate::grammar::model::RecognizerModel;
use crate::grammar::source::SourceSet;

#[derive(Clone, Debug, Default)]
pub(crate) struct StaticParserBuild {
    pub(crate) diagnostics: Vec<Diagnostic>,
}

pub(crate) fn build_static_parser_tables(
    _compiled: &CompiledParser,
    _sources: &SourceSet,
    _grammar_path: &Path,
) -> StaticParserBuild {
    StaticParserBuild::default()
}

pub(crate) fn token_display_name(recognizer: &RecognizerModel, ttype: i32) -> String {
    if ttype == -1 {
        return "EOF".to_owned();
    }
    if let Ok(index) = usize::try_from(ttype) {
        if let Some(Some(literal)) = recognizer.literal_names.get(index) {
            return literal.clone();
        }
        if let Some(Some(symbolic)) = recognizer.symbolic_names.get(index) {
            return symbolic.clone();
        }
    }
    ttype.to_string()
}

pub(crate) fn report_decisions(
    compiled: &CompiledParser,
    _sources: &SourceSet,
    grammar_name: &str,
) -> String {
    let atn = PackedATNDeserializer::new()
        .deserialize(compiled.packed.packed_words())
        .expect("packed parser ATN deserializes");
    let num_decisions = atn.decision_to_state.len();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "=== decision report: grammar {grammar_name} ({num_decisions} decisions) ==="
    );
    let _ = writeln!(out, "summary:");
    let _ = writeln!(
        out,
        "fallbacks: static-only=0/{num_decisions}; escape-states=0 across 0 hybrid decisions + 0 adaptive precedence classes; adaptive-only decisions={num_decisions}"
    );
    let _ = writeln!(out, "LLK k histogram: {{}}");
    let _ = writeln!(
        out,
        "max k: LLK=0, any acyclic non-LL(1) (incl. conflicted)=0"
    );
    out
}

pub(crate) fn dump_dfas(
    compiled: &CompiledParser,
    _sources: &SourceSet,
    grammar_path: &Path,
    grammar_name: &str,
    spec: &str,
) -> (String, Vec<Diagnostic>) {
    let atn = PackedATNDeserializer::new()
        .deserialize(compiled.packed.packed_words())
        .expect("packed parser ATN deserializes");
    let recognizer = &compiled.semantic.recognizer;

    let mut all = false;
    let mut _with_paths = false;
    let mut by_decision = BTreeSet::new();
    let mut by_rule = HashSet::new();
    let mut by_rule_decision = HashSet::new();
    let mut items = Vec::new();
    let mut diagnostics = Vec::new();

    for raw in spec.split(',') {
        let item = raw.trim();
        if item.is_empty() {
            continue;
        }
        items.push(item.to_owned());
        if item.eq_ignore_ascii_case("all") {
            all = true;
            continue;
        }
        if item.eq_ignore_ascii_case("paths") {
            _with_paths = true;
            items.pop();
            continue;
        }
        if let Some((rule_part, num_part)) = item.split_once(':') {
            let rule = rule_part.trim();
            let num = num_part.trim();
            if !rule.is_empty()
                && num.chars().all(|c| c.is_ascii_digit())
                && let Ok(parsed) = num.parse::<usize>()
            {
                by_rule_decision.insert(format!("{rule}:{parsed}"));
            } else {
                diagnostics.push(dump_no_match_error(grammar_path, grammar_name, item));
                items.pop();
            }
        } else if item.chars().all(|c| c.is_ascii_digit())
            && let Ok(parsed) = item.parse::<usize>()
        {
            by_decision.insert(parsed);
        } else {
            by_rule.insert(item.to_owned());
        }
    }

    if !all && by_decision.is_empty() && by_rule.is_empty() && by_rule_decision.is_empty() {
        diagnostics.push(dump_no_match_error(grammar_path, grammar_name, spec));
    }

    let mut out = String::new();
    let _ = writeln!(out, "=== static DFA dump: grammar {grammar_name} ===");
    let _ = writeln!(
        out,
        "legend: each decision's table maps a lookahead token sequence to the"
    );
    let _ = writeln!(
        out,
        "  alternative adaptivePredict would choose; state 0 is the start state."
    );
    let _ = writeln!(
        out,
        "  State kinds: 'accept alt=N' = predict alternative N (terminal);"
    );
    let _ = writeln!(
        out,
        "  'ESCAPE' = defer the whole prediction to adaptivePredict (terminal);"
    );
    let _ = writeln!(
        out,
        "  'GUARDED alt=N' = resolve to alt N unless the runtime parse-stack check"
    );
    let _ = writeln!(
        out,
        "  trips (guardDanger ATN states); 'accept group virtualAlt=N' = terminal"
    );
    let _ = writeln!(
        out,
        "  state resolving to a left-factoring group whose shared prefix is executed"
    );
    let _ = writeln!(
        out,
        "  before a tail decision resolves the choice. Edges are inclusive token-type"
    );
    let _ = writeln!(
        out,
        "  ranges; a token matching no edge means no viable alternative."
    );
    let _ = writeln!(out, "  Token EOF = -1.");
    let _ = write!(out, "vocabulary:");
    for t in 1..=atn.max_token_type {
        let _ = write!(out, " {}={t}", token_display_name(recognizer, t));
    }
    let _ = writeln!(out);

    let mut matched = HashSet::new();
    let mut shown = 0usize;
    for (decision, state) in atn.decision_to_state.iter().enumerate() {
        let rule_name = recognizer
            .rule_names
            .get(state.get_rule_index() as usize)
            .map_or("", String::as_str);
        let mut match_key = None;
        if by_decision.contains(&decision) {
            match_key = Some(decision.to_string());
        }
        if by_rule.contains(rule_name) {
            match_key = Some(rule_name.to_owned());
        }
        let rule_dec_key = format!("{rule_name}:{decision}");
        if by_rule_decision.contains(&rule_dec_key) {
            match_key = Some(rule_dec_key);
        }
        if !all && match_key.is_none() {
            continue;
        }
        if let Some(key) = match_key {
            matched.insert(key);
        }
        matched.insert(rule_name.to_owned());
        shown += 1;

        let _ = writeln!(out, "\n--- decision {decision}: rule '{rule_name}' ---");
    }

    let _ = writeln!(
        out,
        "\n({shown} of {} decisions dumped)",
        atn.decision_to_state.len()
    );

    for item in &items {
        if item.eq_ignore_ascii_case("all") {
            continue;
        }
        let key = if let Some((rule_part, num_part)) = item.split_once(':') {
            let parsed = num_part.trim().parse::<usize>().unwrap_or(0);
            format!("{}:{parsed}", rule_part.trim())
        } else {
            item.clone()
        };
        if !matched.contains(&key) {
            diagnostics.push(dump_no_match_error(grammar_path, grammar_name, item));
        }
    }

    (out, diagnostics)
}

fn dump_no_match_error(grammar_path: &Path, grammar_name: &str, item: &str) -> Diagnostic {
    Diagnostic::new(
        "G4S191",
        Severity::Error,
        format!("--dump-dfas: no parser decision of grammar {grammar_name} matches '{item}'"),
        PathBuf::from(grammar_path),
        None,
        None,
    )
}
