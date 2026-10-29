// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeSet;

/// Optimization selection for one generation run.
///
/// Grammar transforms rewrite the grammar model before ATN construction.
/// These transforms preserve recognition (that is, the generated parser
/// accepts the same language). However, they are **NOT tree-preserving**!
/// When enabled, the generated parse tree can differ from the `.g4` grammar.
///
/// Static lexer DFA generation ([`Self::static_lexer_dfa`]) precomputes lexer
/// DFA tables after ATN construction and preserves both tokens and APIs.
/// Static parser DFA generation ([`Self::static_parser_dfa`]) precomputes
/// parser prediction tables after ATN construction and preserves both parse
/// trees and recognizer APIs.
///
/// Selected optimization passes write their reports to an `optimizations.txt`
/// manifest next to the generated files.
///
/// When [`Self::report_only`] is enabled, the generator compiles, analyzes,
/// and optimizes the input grammars without emitting recognizer files.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptimizationOptions {
    pub(crate) entry_rules: BTreeSet<String>,
    pub(crate) prune_unreachable: bool,
    pub(crate) inline_trivial_rules: bool,
    pub(crate) optimize_precedence_ladders: bool,
    pub(crate) static_lexer_dfa: bool,
    pub(crate) static_parser_dfa: bool,
    pub(crate) report_decisions: bool,
    pub(crate) dump_dfas: Option<String>,
    pub(crate) report_only: bool,
}

impl OptimizationOptions {
    /// Starts an empty selection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Marks `name` as an explicit parser entry rule. Configured entry rules
    /// are preserved by every transform and anchor the reachability analysis
    /// behind [`Self::prune_unreachable_rules`]. Repeatable.
    pub fn entry_rule(&mut self, name: impl Into<String>) -> &mut Self {
        self.entry_rules.insert(name.into());
        self
    }

    /// Deletes parser rules that are unreachable from the entry rules.
    /// Default: `false` (unreachable rules only produce a warning).
    pub const fn prune_unreachable_rules(&mut self, enabled: bool) -> &mut Self {
        self.prune_unreachable = enabled;
        self
    }

    /// Inlines trivial pure parser rules (token sets and single-use pure
    /// sequences) into their call sites. Default: `false`.
    pub const fn inline_trivial_rules(&mut self, enabled: bool) -> &mut Self {
        self.inline_trivial_rules = enabled;
        self
    }

    /// Collapses precedence-ladder rule chains (`a: b (op b)*; b: c ...`)
    /// into a single left-recursive rule. Default: `false`.
    pub const fn optimize_precedence_ladders(&mut self, enabled: bool) -> &mut Self {
        self.optimize_precedence_ladders = enabled;
        self
    }

    /// Precomputes static lexer DFA tables (`--static-lexer-dfa`, lexer scope
    /// only). Default: `false`.
    pub const fn static_lexer_dfa(&mut self, enabled: bool) -> &mut Self {
        self.static_lexer_dfa = enabled;
        self
    }

    /// Precomputes static parser DFA tables (`--static-parser-dfa`, parser
    /// scope only). Default: `false`.
    pub const fn static_parser_dfa(&mut self, enabled: bool) -> &mut Self {
        self.static_parser_dfa = enabled;
        self
    }

    /// Renders the parser decision report of the `static-parser-dfa` pass
    /// (`--report-decisions`) and implies [`Self::report_only`].
    pub const fn report_decisions(&mut self, enabled: bool) -> &mut Self {
        self.report_decisions = enabled;
        self
    }

    /// Renders the static parser DFA tables matching `spec`
    /// (`--dump-dfas <SPEC>`) and implies [`Self::report_only`].
    pub fn dump_dfas(&mut self, spec: impl Into<String>) -> &mut Self {
        self.dump_dfas = Some(spec.into());
        self
    }

    /// Compiles, analyzes, and optimizes the input grammars without writing
    /// output files (`--report-only`, `--dry-run`, `-n`). Default: `false`.
    pub const fn report_only(&mut self, enabled: bool) -> &mut Self {
        self.report_only = enabled;
        self
    }

    /// Whether the selection prints instead of writing files: `--report-only`
    /// or a report mode that implies it (`--report-decisions`,
    /// `--dump-dfas`).
    pub(crate) const fn is_report_only(&self) -> bool {
        self.report_only || self.report_decisions || self.dump_dfas.is_some()
    }

    #[cfg(feature = "download")]
    pub(crate) fn entry_rules(&self) -> impl Iterator<Item = &str> {
        self.entry_rules.iter().map(String::as_str)
    }

    #[cfg(feature = "download")]
    pub(crate) const fn prune_unreachable(&self) -> bool {
        self.prune_unreachable
    }

    #[cfg(feature = "download")]
    pub(crate) const fn inline_trivial(&self) -> bool {
        self.inline_trivial_rules
    }

    #[cfg(feature = "download")]
    pub(crate) const fn optimize_ladders(&self) -> bool {
        self.optimize_precedence_ladders
    }

    #[cfg(feature = "download")]
    pub(crate) const fn static_dfa(&self) -> bool {
        self.static_lexer_dfa
    }

    #[cfg(feature = "download")]
    pub(crate) const fn static_parser_dfa_enabled(&self) -> bool {
        self.static_parser_dfa
    }

    /// Whether the selection prints parser reports, which only the in-process
    /// generator can render.
    #[cfg(feature = "download")]
    pub(crate) const fn has_parser_reports(&self) -> bool {
        self.report_decisions || self.dump_dfas.is_some()
    }
}
