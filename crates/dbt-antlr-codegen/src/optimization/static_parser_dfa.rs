// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! `--static-parser-dfa`, `--report-decisions`, and `--dump-dfas` support for
//! the emission layer (parser scope).

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
use dbt_antlr_runtime::dfa::decision_classifier::{
    BitSet, Category, DecisionClassifier, DecisionResult, ESCAPE, GUARDED,
    PrecedenceStaticParserDFA, StaticParserDFA, VIRTUAL_ALT_BASE, distinct_bitsets, format_bitset,
    format_distinct_bitsets, format_histogram,
};

use crate::error::{Diagnostic, Severity};
use crate::grammar::atn::CompiledParser;
use crate::grammar::frontend::SourceSpan;
use crate::grammar::model::{
    Alternative, Block, Element, ElementKind, ModelNodeId, Quantifier, RecognizerModel, Rule,
};
use crate::grammar::provenance::Origin;
use crate::grammar::source::SourceSet;

#[derive(Clone, Debug, Default)]
pub(crate) struct StaticParserBuild {
    pub(crate) diagnostics: Vec<Diagnostic>,
}

pub(crate) fn build_static_parser_tables(
    compiled: &CompiledParser,
    sources: &SourceSet,
    grammar_path: &Path,
) -> StaticParserBuild {
    let atn = PackedATNDeserializer::new()
        .deserialize(compiled.packed.packed_words())
        .expect("packed parser ATN deserializes");
    let ll1_disjoint = ll1_disjoint_vec(compiled, atn.decision_to_state.len());
    let mut classifier = DecisionClassifier::new(&atn, ll1_disjoint.clone());
    classifier.abort_on_untrusted_conflict = true;

    let mut warnings = Vec::new();
    for (decision, &s) in atn.decision_to_state.iter().enumerate() {
        if ll1_disjoint.get(decision).copied() == Some(true) {
            continue;
        }
        let r = classifier.classify(s);
        if let Some(w) = warn_if_not_fully_static(compiled, sources, grammar_path, &classifier, &r)
        {
            warnings.push(w);
        }
    }

    let diagnostics = emit_decision_warnings(warnings);
    StaticParserBuild { diagnostics }
}

fn ll1_disjoint_vec(compiled: &CompiledParser, num_decisions: usize) -> Vec<bool> {
    let mut by_state = HashMap::new();
    for look in &compiled.analysis.decision_lookahead {
        by_state.insert(look.state, look.disjoint);
    }
    let mut out = vec![false; num_decisions];
    for (decision, &state) in compiled.graph.decisions.iter().enumerate() {
        if decision < out.len() {
            out[decision] = by_state.get(&state).copied().unwrap_or(false);
        }
    }
    out
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
    let ll1_disjoint = ll1_disjoint_vec(compiled, atn.decision_to_state.len());
    let mut classifier = DecisionClassifier::new(&atn, ll1_disjoint);
    classifier.abort_inf_as_overflow = true;
    let mut results = classifier.classify_all();
    classifier.abort_inf_as_overflow = false;

    for (i, &s) in atn.decision_to_state.iter().enumerate() {
        let r = &results[i];
        let adaptive_fallback = if r.category == Category::LR_PRECEDENCE {
            r.prec_dfa.is_none() || r.adaptive_classes > 0
        } else {
            r.category != Category::LL1 && r.dfa.is_none()
        };
        if adaptive_fallback {
            results[i] = classifier.classify(s);
        }
    }

    let mut counts: BTreeMap<Category, usize> = BTreeMap::new();
    for c in Category::ALL {
        counts.insert(c, 0);
    }
    for r in &results {
        *counts.entry(r.category).or_insert(0) += 1;
    }

    let prec_static = results.iter().filter(|r| r.prec_dfa.is_some()).count();

    let mut fb_static_only = 0usize;
    let mut fb_escape_states = 0usize;
    let mut fb_hybrid_decisions = 0usize;
    let mut fb_adaptive_classes = 0usize;
    let mut fb_adaptive_only = 0usize;
    for r in &results {
        if r.category == Category::LR_PRECEDENCE {
            if r.prec_dfa.is_none() {
                fb_adaptive_only += 1;
            } else {
                fb_escape_states += r.escapes;
                fb_adaptive_classes += r.adaptive_classes;
                if r.escapes > 0 {
                    fb_hybrid_decisions += 1;
                } else if r.adaptive_classes == 0 {
                    fb_static_only += 1;
                }
            }
        } else if r.category == Category::LL1 || (r.dfa.is_some() && r.escapes == 0) {
            fb_static_only += 1;
        } else if r.dfa.is_none() {
            fb_adaptive_only += 1;
        } else {
            fb_hybrid_decisions += 1;
            fb_escape_states += r.escapes;
        }
    }

    let recognizer = &compiled.semantic.recognizer;
    let num_decisions = results.len();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "=== decision report: grammar {grammar_name} ({num_decisions} decisions) ==="
    );
    let _ = write!(out, "summary:");
    for c in Category::ALL {
        let count = counts[&c];
        if count > 0 {
            let _ = write!(out, " {c}={count}");
            if c == Category::LR_PRECEDENCE && prec_static > 0 {
                let _ = write!(out, "({prec_static} static)");
            }
        }
    }
    let _ = writeln!(out);
    let _ = writeln!(
        out,
        "fallbacks: static-only={fb_static_only}/{num_decisions}; escape-states={fb_escape_states} across {fb_hybrid_decisions} hybrid decisions + {fb_adaptive_classes} adaptive precedence classes; adaptive-only decisions={fb_adaptive_only}"
    );

    let utrust: usize = results.iter().map(|r| r.utrust_resolutions).sum();
    if utrust > 0 {
        let _ = writeln!(out, "utrust: {utrust} conflict states resolved to min-alt");
    }

    let mut max_k_any_acyclic = 0i32;
    let mut max_k_llk = 0i32;
    let mut llk_histogram: BTreeMap<i32, usize> = BTreeMap::new();
    for r in &results {
        if r.category == Category::LL1 {
            continue;
        }
        if r.category == Category::LR_PRECEDENCE {
            if r.prec_note.is_some() {
                let _ = writeln!(out, "{}", report_line(recognizer, r));
            }
            continue;
        }
        if r.k > max_k_any_acyclic {
            max_k_any_acyclic = r.k;
        }
        if r.category == Category::LLK {
            max_k_llk = max_k_llk.max(r.k);
            *llk_histogram.entry(r.k).or_insert(0) += 1;
        }
        let _ = writeln!(out, "{}", report_line(recognizer, r));
    }
    let _ = writeln!(out, "LLK k histogram: {}", format_histogram(&llk_histogram));
    let _ = writeln!(
        out,
        "max k: LLK={max_k_llk}, any acyclic non-LL(1) (incl. conflicted)={max_k_any_acyclic}"
    );
    out
}

fn report_line(recognizer: &RecognizerModel, r: &DecisionResult) -> String {
    let rule_name = recognizer
        .rule_names
        .get(r.rule_index)
        .map_or("", String::as_str);
    let mut buf = format!(
        "{:<22} d={:<4} {:<17}",
        rule_name,
        r.decision,
        r.category.as_str()
    );
    if r.category == Category::LR_PRECEDENCE
        && let Some(prec_note) = &r.prec_note
    {
        let status = if r.prec_dfa.is_some() {
            " [static]"
        } else {
            " [adaptive]"
        };
        let _ = write!(
            buf,
            " {prec_note}{status} states={} fb={}",
            r.num_dfa_states,
            r.fb_string()
        );
        return buf;
    }
    let _ = write!(buf, " fb={}", r.fb_string());
    if r.k >= 0 {
        let _ = write!(buf, " k={}", r.k);
    }
    let _ = write!(buf, " dfaStates={}", r.num_dfa_states);
    if r.category == Category::HYBRID {
        let adaptive = if r.dfa.is_none() { " [adaptive]" } else { "" };
        let _ = write!(
            buf,
            " escapes={} cover={:.0}%{adaptive}",
            r.escapes,
            r.coverage * 100.0
        );
    }
    if !r.exact_ambig_conflicts.is_empty() {
        let _ = write!(
            buf,
            " exactAmbigAlts={}",
            format_distinct_bitsets(&r.exact_ambig_conflicts)
        );
    }
    if !r.context_sensitive_conflicts.is_empty() {
        let _ = write!(
            buf,
            " ctxSensitiveAlts={}",
            format_distinct_bitsets(&r.context_sensitive_conflicts)
        );
    }
    if !r.approx_conflicts.is_empty() {
        let _ = write!(
            buf,
            " approxConflictAlts={}",
            format_distinct_bitsets(&r.approx_conflicts)
        );
    }
    if r.used_widening {
        if r.widening_depth == i32::MAX {
            buf.push_str(" [widened]");
        } else {
            let _ = write!(buf, " [widened depth={}]", r.widening_depth);
        }
    }
    if r.saw_prec_predicate {
        buf.push_str(" [precpred-in-lookahead]");
    }
    buf
}

pub(crate) fn dump_dfas(
    compiled: &CompiledParser,
    sources: &SourceSet,
    grammar_path: &Path,
    grammar_name: &str,
    spec: &str,
) -> (String, Vec<Diagnostic>) {
    let atn = PackedATNDeserializer::new()
        .deserialize(compiled.packed.packed_words())
        .expect("packed parser ATN deserializes");
    let ll1_disjoint = ll1_disjoint_vec(compiled, atn.decision_to_state.len());
    let mut classifier = DecisionClassifier::new(&atn, ll1_disjoint);
    classifier.abort_on_untrusted_conflict = true;
    let recognizer = &compiled.semantic.recognizer;

    let mut all = false;
    let mut with_paths = false;
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
            with_paths = true;
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
    let states = atn.decision_to_state.clone();
    for (decision, state) in states.into_iter().enumerate() {
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

        let r = classifier.classify(state);
        let _ = write!(out, "\n--- decision {decision}: rule '{rule_name}'");
        if let Some((file, line, col)) = decision_location(compiled, sources, grammar_path, &r) {
            let _ = write!(out, " ({}:{line}:{col})", file.display());
        }
        let _ = writeln!(out, " ---");
        let _ = writeln!(out, "{}", report_line(recognizer, &r));

        if r.category == Category::LL1 {
            let _ = writeln!(
                out,
                "no static table: single-token lookahead sets are disjoint; the"
            );
            let _ = writeln!(
                out,
                "  generated parser's LL(1) fast path resolves this decision."
            );
            continue;
        }

        if let Some(prec_dfa) = &r.prec_dfa {
            dump_precedence_dfa(&mut out, recognizer, prec_dfa, with_paths);
        } else if let Some(dfa) = &r.dfa {
            dump_static_dfa(&mut out, recognizer, dfa, "", with_paths);
        } else {
            let _ = writeln!(
                out,
                "no static table: every prediction of this decision uses"
            );
            let _ = writeln!(out, "  adaptivePredict at runtime.");
        }
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

fn dump_static_dfa(
    buf: &mut String,
    recognizer: &RecognizerModel,
    dfa: &StaticParserDFA,
    indent: &str,
    with_paths: bool,
) {
    let _ = write!(
        buf,
        "{indent}table: states={} cyclic={}",
        dfa.num_states, dfa.cyclic
    );
    if !dfa.cyclic {
        let _ = write!(buf, " maxK={}", dfa.max_k);
    }
    let _ = writeln!(buf);

    let paths = with_paths.then(|| shortest_paths(dfa));
    for st in 0..dfa.num_states {
        let _ = write!(buf, "{indent}  s{st}: ");
        let a = dfa.accepts[st];
        if a == ESCAPE {
            buf.push_str("ESCAPE");
        } else if a == GUARDED {
            let _ = write!(
                buf,
                "GUARDED alt={}; defers if the parse stack carries guardDanger={:?}",
                dfa.guarded_alts[st], dfa.guard_danger
            );
        } else if StaticParserDFA::is_virtual_alt(a) {
            let _ = write!(buf, "accept group virtualAlt={a} (unknown group)");
        } else if a > 0 {
            let _ = write!(buf, "accept alt={a}");
        } else {
            buf.push_str("(non-accept)");
        }
        if dfa.fallbacks[st] > 0 {
            let _ = write!(buf, "; error-recovery fallback alt={}", dfa.fallbacks[st]);
        }
        if let Some((prev_state, prev_edge)) = &paths {
            let _ = write!(
                buf,
                "; path: {}",
                path_string(recognizer, dfa, prev_state, prev_edge, st)
            );
        }
        let _ = writeln!(buf);

        let end = dfa.edge_offsets[st + 1];
        let mut e = dfa.edge_offsets[st];
        while e < end {
            let lo = dfa.edges[e];
            let hi = dfa.edges[e + 1];
            let target = dfa.edges[e + 2];
            let _ = write!(buf, "{indent}    s{st} ");
            if lo == hi {
                buf.push_str(&token_display_name(recognizer, lo));
            } else {
                let _ = write!(
                    buf,
                    "{}..{}",
                    token_display_name(recognizer, lo),
                    token_display_name(recognizer, hi)
                );
            }
            let _ = write!(buf, "({lo}");
            if hi != lo {
                let _ = write!(buf, "..{hi}");
            }
            let _ = writeln!(buf, ") -> s{target}");
            e += 3;
        }
    }
}

fn dump_precedence_dfa(
    buf: &mut String,
    recognizer: &RecognizerModel,
    p: &PrecedenceStaticParserDFA,
    with_paths: bool,
) {
    let _ = writeln!(
        buf,
        "precedence cutoffs: {:?}; class(p) = number of cutoffs < p",
        p.cutoffs
    );
    let k = p.cutoffs.len();
    for cls in 0..=k {
        let range = if cls == 0 {
            format!("p<={}", p.cutoffs[0])
        } else if cls == k {
            format!("p>{}", p.cutoffs[k - 1])
        } else {
            format!("{}<p<={}", p.cutoffs[cls - 1], p.cutoffs[cls])
        };
        let _ = write!(buf, "class {cls} ({range}):");
        if let Some(table) = &p.tables[cls] {
            let _ = writeln!(buf);
            dump_static_dfa(buf, recognizer, table, "  ", with_paths);
        } else {
            let _ = writeln!(buf, " no static table (adaptivePredict)");
        }
    }
}

fn shortest_paths(dfa: &StaticParserDFA) -> (Vec<Option<usize>>, Vec<usize>) {
    let mut prev_state = vec![None; dfa.num_states];
    let mut prev_edge = vec![0usize; dfa.num_states];
    if dfa.num_states == 0 {
        return (prev_state, prev_edge);
    }
    prev_state[0] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(0usize);
    while let Some(st) = queue.pop_front() {
        let end = dfa.edge_offsets[st + 1];
        let mut e = dfa.edge_offsets[st];
        while e < end {
            let target = dfa.edges[e + 2] as usize;
            if prev_state[target].is_none() {
                prev_state[target] = Some(st);
                prev_edge[target] = e;
                queue.push_back(target);
            }
            e += 3;
        }
    }
    (prev_state, prev_edge)
}

fn path_string(
    recognizer: &RecognizerModel,
    dfa: &StaticParserDFA,
    prev_state: &[Option<usize>],
    prev_edge: &[usize],
    st: usize,
) -> String {
    if st == 0 {
        return "(start)".to_owned();
    }
    let mut labels = VecDeque::new();
    let mut cur = st;
    while cur != 0
        && let Some(prev) = prev_state[cur]
    {
        let e = prev_edge[cur];
        let lo = dfa.edges[e];
        let hi = dfa.edges[e + 1];
        let label = if lo == hi {
            token_display_name(recognizer, lo)
        } else {
            format!(
                "{}..{}",
                token_display_name(recognizer, lo),
                token_display_name(recognizer, hi)
            )
        };
        labels.push_front(label);
        cur = prev;
    }
    let mut out = String::new();
    for (shown, label) in labels.into_iter().enumerate() {
        if shown == 16 {
            out.push_str(" ...");
            break;
        }
        if shown > 0 {
            out.push(' ');
        }
        out.push_str(&label);
    }
    out
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

struct DecisionWarning {
    decision: usize,
    rule_name: String,
    file: PathBuf,
    line: Option<usize>,
    col: Option<usize>,
    reason: String,
    key: String,
    group_key: String,
}

fn warn_if_not_fully_static(
    compiled: &CompiledParser,
    sources: &SourceSet,
    grammar_path: &Path,
    classifier: &DecisionClassifier<'_>,
    r: &DecisionResult,
) -> Option<DecisionWarning> {
    let reason = not_fully_static_reason(compiled, sources, classifier, r)?;
    let rule_name = compiled
        .semantic
        .recognizer
        .rule_names
        .get(r.rule_index)
        .cloned()
        .unwrap_or_default();
    let (file, line, col) = decision_location(compiled, sources, grammar_path, r).map_or_else(
        || (grammar_path.to_path_buf(), None, None),
        |(f, l, c)| (f, Some(l), Some(c)),
    );
    let canon = file.canonicalize().unwrap_or_else(|_| file.clone());
    let key = format!(
        "{}:{}:{}: {reason}",
        canon.display(),
        line.unwrap_or(0),
        col.unwrap_or(0)
    );
    let group_key: String = reason.chars().filter(|c| !c.is_ascii_digit()).collect();
    Some(DecisionWarning {
        decision: r.decision,
        rule_name,
        file,
        line,
        col,
        reason,
        key,
        group_key,
    })
}

fn emit_decision_warnings(warnings: Vec<DecisionWarning>) -> Vec<Diagnostic> {
    let mut seen = HashSet::new();
    let mut group_order = Vec::new();
    let mut groups: HashMap<String, Vec<DecisionWarning>> = HashMap::new();
    for w in warnings {
        if !seen.insert(w.key.clone()) {
            continue;
        }
        if !groups.contains_key(&w.group_key) {
            group_order.push(w.group_key.clone());
        }
        groups.entry(w.group_key.clone()).or_default().push(w);
    }

    let mut diagnostics = Vec::new();
    for group_key in group_order {
        let group = groups.remove(&group_key).unwrap();
        let first = &group[0];
        let mut suffix = String::new();
        if group.len() > 1 {
            let _ = write!(
                suffix,
                " [same issue at {} more decision(s): ",
                group.len() - 1
            );
            let shown = group.len().min(5);
            for (idx, w) in group.iter().enumerate().take(shown).skip(1) {
                if idx > 1 {
                    suffix.push_str(", ");
                }
                let _ = write!(suffix, "rule '{}' decision {} (", w.rule_name, w.decision);
                if w.file != first.file
                    && let Some(name) = w.file.file_name()
                {
                    let _ = write!(suffix, "{}:", name.to_string_lossy());
                }
                let _ = write!(suffix, "{}:{})", w.line.unwrap_or(0), w.col.unwrap_or(0));
            }
            let rest = group.len() - shown;
            if rest > 0 {
                let _ = write!(suffix, ", +{rest} more");
            }
            suffix.push(']');
        }
        let position = first.line.zip(first.col);
        let message = format!(
            "--static-parser-dfa: decision {} of rule '{}' is not fully static: {}{suffix}",
            first.decision, first.rule_name, first.reason
        );
        diagnostics.push(Diagnostic::new(
            "G4S190",
            Severity::Warning,
            message,
            first.file.clone(),
            position,
            None,
        ));
    }
    diagnostics
}

fn not_fully_static_reason(
    compiled: &CompiledParser,
    sources: &SourceSet,
    classifier: &DecisionClassifier<'_>,
    r: &DecisionResult,
) -> Option<String> {
    let recognizer = &compiled.semantic.recognizer;
    match r.category {
        Category::NON_GREEDY => Some(
            "non-greedy loops ((...)*? / (...)+?) inherently need runtime input; to make fully static: rewrite the loop greedily with explicit terminators, or match the delimited region in a lexer rule"
                .to_owned(),
        ),
        Category::PREDICATED => Some(format!(
            "lookahead crosses {} evaluated at runtime; to make fully static: hoist the predicate out of the lookahead path (validate the parse afterwards instead), or make the guarded alternative's leading tokens disjoint from its siblings",
            predicate_ref(compiled, sources, r)
        )),
        Category::OVERFLOW => {
            if r.num_transitions >= VIRTUAL_ALT_BASE as usize {
                Some(format!(
                    "the decision has {} alternatives, at or past the static table's alternative-number ceiling of {VIRTUAL_ALT_BASE} (where left-factoring group numbering begins); to make fully static: split the block into nested sub-blocks",
                    r.num_transitions
                ))
            } else {
                Some(format!(
                    "the static DFA exceeded the cap of {} states in every construction attempt (the decision likely needs unbounded lookahead); to make fully static: reduce the lookahead it needs (left-factor common prefixes), or raise the cap with -Dantlr.dfa.maxDfaStates",
                    classifier.max_dfa_states
                ))
            }
        }
        Category::CONTEXT_SENSITIVE => Some(format!(
            "{}; prediction will use adaptivePredict{}; to make fully static: left-factor the common prefix of the conflicting alternatives, or restructure so the choice depends only on the input",
            conflict_reason(compiled, sources, r),
            witness_clause(recognizer, r)
        )),
        Category::HYBRID => {
            let has_conflicts =
                !r.context_sensitive_conflicts.is_empty() || !r.approx_conflicts.is_empty();
            let mut hb = String::new();
            append_deferrals(&mut hb, r);
            let _ = write!(
                hb,
                " (table covers {:.0}% of first tokens",
                (r.coverage * 100.0).round()
            );
            if has_conflicts {
                let _ = write!(hb, "): {}", conflict_reason(compiled, sources, r));
            } else {
                hb.push_str("; escapes are lookahead beyond the static depth/state budget)");
            }
            hb.push_str(&witness_clause(recognizer, r));
            if has_conflicts {
                hb.push_str("; to make fully static: resolve the conflict (left-factor the common prefix, or restructure so the choice depends only on the input)");
            } else {
                hb.push_str("; to make fully static: reduce the lookahead this decision needs below the -Dantlr.dfa.hybridDepthCap/-Dantlr.dfa.hybridStateCap budget, or raise the budget");
            }
            Some(hb)
        }
        Category::LR_PRECEDENCE => {
            let hint = "; to make fully static: restructure the operator alternatives so loop iteration vs exit is decidable from the input alone";
            let prec_note = r.prec_note.as_deref().unwrap_or("");
            if r.prec_dfa.is_none() {
                return Some(format!(
                    "no precedence class of this left-recursive loop decision is statically decidable; prediction will use adaptivePredict ({prec_note}){}{hint}",
                    witness_clause(recognizer, r)
                ));
            }
            let mut b = String::new();
            if r.adaptive_classes > 0 {
                let _ = write!(b, "{} precedence class(es) have no table", r.adaptive_classes);
            }
            let tail = b.len();
            append_deferrals(&mut b, r);
            if b.is_empty() {
                return None;
            }
            if b.len() > tail && tail > 0 {
                b.insert_str(tail, "; ");
            }
            let _ = write!(b, " ({prec_note}){}{hint}", witness_clause(recognizer, r));
            Some(b)
        }
        Category::LL1 | Category::LLK | Category::LLSTAR | Category::EXACT_AMBIG => None,
    }
}

fn append_deferrals(b: &mut String, r: &DecisionResult) {
    let guarded = r.guarded_takes + r.guarded_iterates + r.guarded_obligations;
    if r.escapes > 0 {
        let _ = write!(b, "{} escape state(s) defer to adaptivePredict", r.escapes);
    }
    if guarded > 0 {
        if r.escapes > 0 {
            b.push_str(" and ");
        }
        let _ = write!(
            b,
            "{guarded} guarded state(s) check the parse stack at runtime and may defer to adaptivePredict"
        );
    }
}

fn conflict_reason(compiled: &CompiledParser, sources: &SourceSet, r: &DecisionResult) -> String {
    let mut parts = Vec::new();
    if !r.context_sensitive_conflicts.is_empty() {
        parts.push(format!(
            "{} conflict in unbounded SLL lookahead, but the full parse context may resolve them differently (SLL/LL divergence)",
            alt_set_ref(compiled, sources, r, &r.context_sensitive_conflicts)
        ));
    }
    if !r.approx_conflicts.is_empty() {
        parts.push(format!(
            "{} conflict only under approximated (widened) context; the conflict may be an analysis artifact the runtime never encounters",
            alt_set_ref(compiled, sources, r, &r.approx_conflicts)
        ));
    }
    if parts.is_empty() {
        "static analysis cannot prove the decision context-independent".to_owned()
    } else {
        parts.join("; ")
    }
}

fn alt_set_ref(
    compiled: &CompiledParser,
    sources: &SourceSet,
    r: &DecisionResult,
    sets: &[BitSet],
) -> String {
    let mut b = String::new();
    for (idx, conflicting) in distinct_bitsets(sets).iter().enumerate() {
        if idx > 0 {
            b.push_str(", ");
        }
        if conflicting.len() == 2 {
            let mut iter = conflicting.iter();
            let a1 = iter.next().unwrap();
            let a2 = iter.next().unwrap();
            let _ = write!(
                b,
                "{} vs {}",
                alt_ref(compiled, sources, r, a1),
                alt_ref(compiled, sources, r, a2)
            );
        } else {
            let _ = write!(b, "alts {}", format_bitset(conflicting));
        }
    }
    b
}

fn alt_ref(
    compiled: &CompiledParser,
    sources: &SourceSet,
    r: &DecisionResult,
    alt: usize,
) -> String {
    let base = format!("alt {alt}");
    let state_num = r.state_number as usize;
    let Some(state) = compiled.graph.states.get(state_num) else {
        return base;
    };
    let owner = compiled
        .provenance
        .state_origins(state.original)
        .iter()
        .find_map(|origin| match origin {
            Origin::Synthetic { owner, .. } => Some(*owner),
            _ => None,
        });
    let Some(owner) = owner else {
        return base;
    };

    if state.kind == crate::grammar::atn::AtnStateKind::StarLoopEntry {
        return match alt {
            1 => "enter the loop".to_owned(),
            2 => "exit the loop".to_owned(),
            _ => base,
        };
    }
    if state.kind == crate::grammar::atn::AtnStateKind::PlusLoopBack {
        return match alt {
            1 => "iterate the loop".to_owned(),
            2 => "exit the loop".to_owned(),
            _ => base,
        };
    }

    match owner {
        ModelNodeId::Rule(rule_id) => {
            if let Some(rule) = compiled
                .semantic
                .unit
                .rules
                .iter()
                .find(|rule| rule.id == rule_id)
                && alt >= 1
                && alt <= rule.block.alternatives.len()
            {
                let span = &rule.block.alternatives[alt - 1].span;
                if let Some((line, _)) = sources.line_column(span.source, span.bytes.start) {
                    return format!("{base} (line {line})");
                }
            }
            base
        }
        ModelNodeId::Element(elem_id) => {
            let Some(element) = find_element(&compiled.semantic.unit.rules, elem_id) else {
                return base;
            };
            if let ElementKind::Block(block) = &element.kind {
                if alt >= 1 && alt <= block.alternatives.len() {
                    let span = &block.alternatives[alt - 1].span;
                    if let Some((line, _)) = sources.line_column(span.source, span.bytes.start) {
                        return format!("{base} (line {line})");
                    }
                    return base;
                }
                if matches!(element.quantifier, Quantifier::Optional { .. }) {
                    return "skip".to_owned();
                }
                return base;
            }
            if alt == 1 {
                if let Some((line, _)) =
                    sources.line_column(element.span.source, element.span.bytes.start)
                {
                    return format!("{base} (line {line})");
                }
            } else if alt == 2 && matches!(element.quantifier, Quantifier::Optional { .. }) {
                return "skip".to_owned();
            }
            base
        }
        _ => base,
    }
}

fn witness_clause(recognizer: &RecognizerModel, r: &DecisionResult) -> String {
    let mut b = String::new();
    if let Some(w) = &r.conflict_witness {
        let _ = write!(b, "; e.g. on lookahead {}", render_witness(recognizer, w));
    }
    if let Some(bw) = &r.budget_witness
        && r.conflict_witness.as_ref() != Some(bw)
    {
        let _ = write!(
            b,
            "; deep lookahead e.g. {}",
            render_witness(recognizer, bw)
        );
    }
    if let Some(cls) = &r.witness_class
        && !b.is_empty()
    {
        let _ = write!(b, " (precedence class {cls})");
    }
    b
}

fn render_witness(recognizer: &RecognizerModel, witness: &[i32]) -> String {
    let mut b = String::new();
    let n = witness.len().min(8);
    for (i, &t) in witness.iter().take(n).enumerate() {
        if i > 0 {
            b.push(' ');
        }
        b.push_str(&token_display_name(recognizer, t));
    }
    if witness.len() > n {
        b.push_str(" ...");
    }
    b
}

fn predicate_ref(compiled: &CompiledParser, sources: &SourceSet, r: &DecisionResult) -> String {
    let Some((rule_index, pred_index)) = r.first_predicate else {
        return "a semantic predicate ({...}?)".to_owned();
    };
    let rule_name = compiled
        .semantic
        .recognizer
        .rule_names
        .get(rule_index as usize)
        .map_or("", String::as_str);
    if let Some(span) = find_predicate_span(compiled, pred_index as usize)
        && let Some((line, _)) = sources.line_column(span.source, span.bytes.start)
    {
        return format!("the predicate of rule '{rule_name}' (line {line})");
    }
    format!("a semantic predicate ({{...}}?) of rule '{rule_name}'")
}

fn find_predicate_span(compiled: &CompiledParser, pred_index: usize) -> Option<SourceSpan> {
    let pred_id = compiled
        .semantic
        .recognizer
        .predicate_numbers
        .iter()
        .find_map(|(&id, &idx)| (idx == pred_index).then_some(id))?;
    for rule in &compiled.semantic.unit.rules {
        if let Some(span) = find_predicate_in_block(&rule.block, pred_id) {
            return Some(span);
        }
    }
    None
}

fn find_predicate_in_block(
    block: &Block,
    target_id: crate::grammar::model::PredicateId,
) -> Option<SourceSpan> {
    for alt in &block.alternatives {
        for elem in &alt.elements {
            match &elem.kind {
                ElementKind::Predicate { id, .. } if *id == target_id => {
                    return Some(elem.span.clone());
                }
                ElementKind::Block(inner) => {
                    if let Some(span) = find_predicate_in_block(inner, target_id) {
                        return Some(span);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn decision_location(
    compiled: &CompiledParser,
    sources: &SourceSet,
    grammar_path: &Path,
    r: &DecisionResult,
) -> Option<(PathBuf, usize, usize)> {
    let state_num = r.state_number as usize;
    let state = compiled.graph.states.get(state_num)?;
    if r.category != Category::LR_PRECEDENCE {
        let owner = compiled
            .provenance
            .state_origins(state.original)
            .iter()
            .find_map(|origin| match origin {
                Origin::Synthetic { owner, .. } => Some(*owner),
                _ => None,
            });
        if let Some(owner) = owner {
            match owner {
                ModelNodeId::Rule(rule_id) => {
                    if let Some(rule) = compiled
                        .semantic
                        .unit
                        .rules
                        .iter()
                        .find(|rule| rule.id == rule_id)
                    {
                        let colon_offset = sources
                            .get(rule.block.span.source)
                            .and_then(|file| {
                                let start = rule.span.bytes.start as usize;
                                let end = rule.block.span.bytes.start as usize;
                                file.text()
                                    .get(start..end)?
                                    .rfind(':')
                                    .map(|rel| rule.span.bytes.start + rel as u32)
                            })
                            .unwrap_or(rule.block.span.bytes.start);
                        return span_location(
                            sources,
                            grammar_path,
                            &rule.block.span,
                            colon_offset,
                        );
                    }
                }
                ModelNodeId::Element(elem_id) => {
                    if let Some(elem) = find_element(&compiled.semantic.unit.rules, elem_id) {
                        let is_quantifier_decision =
                            matches!(
                                state.kind,
                                crate::grammar::atn::AtnStateKind::StarLoopEntry
                                    | crate::grammar::atn::AtnStateKind::PlusLoopBack
                            ) || matches!(elem.quantifier, Quantifier::Optional { .. });
                        if is_quantifier_decision && elem.quantifier != Quantifier::One {
                            let suffix_len = match elem.quantifier {
                                Quantifier::Optional { greedy: false }
                                | Quantifier::ZeroOrMore { greedy: false }
                                | Quantifier::OneOrMore { greedy: false } => 2u32,
                                _ => 1u32,
                            };
                            let offset = elem.enclosing_span.bytes.end.saturating_sub(suffix_len);
                            return span_location(
                                sources,
                                grammar_path,
                                &elem.enclosing_span,
                                offset,
                            );
                        }
                        return span_location(
                            sources,
                            grammar_path,
                            &elem.span,
                            elem.span.bytes.start,
                        );
                    }
                }
                _ => {}
            }
        }
    }
    let rule = compiled.semantic.unit.rules.get(r.rule_index)?;
    span_location(sources, grammar_path, &rule.span, rule.span.bytes.start)
}

fn span_location(
    sources: &SourceSet,
    grammar_path: &Path,
    span: &SourceSpan,
    byte_offset: u32,
) -> Option<(PathBuf, usize, usize)> {
    let (line, col) = sources.line_column(span.source, byte_offset)?;
    let path = sources
        .logical_path(span.source)
        .map_or_else(|| grammar_path.to_path_buf(), Path::to_path_buf);
    Some((path, line, col))
}

fn find_element(rules: &[Rule], target: crate::grammar::model::ElementId) -> Option<&Element> {
    for rule in rules {
        if let Some(e) = find_element_in_block(&rule.block, target) {
            return Some(e);
        }
    }
    None
}

fn find_element_in_block(
    block: &Block,
    target: crate::grammar::model::ElementId,
) -> Option<&Element> {
    for alt in &block.alternatives {
        if let Some(e) = find_element_in_alt(alt, target) {
            return Some(e);
        }
    }
    None
}

fn find_element_in_alt(
    alt: &Alternative,
    target: crate::grammar::model::ElementId,
) -> Option<&Element> {
    for elem in &alt.elements {
        if elem.id == target {
            return Some(elem);
        }
        if let ElementKind::Block(inner) = &elem.kind
            && let Some(found) = find_element_in_block(inner, target)
        {
            return Some(found);
        }
    }
    None
}
