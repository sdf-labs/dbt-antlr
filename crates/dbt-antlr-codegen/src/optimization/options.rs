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
/// Every applied grammar transform writes an entry to an `optimizations.txt`
/// manifest next to the generated files.
///
/// The `report_*` variants run the transform in projection mode only. The
/// generator emits no recognizer files, and the manifest describes the
/// projected changes. A `report_*` flag conflicts with the apply flag of the
/// same transform.
///
/// Static lexer DFA generation ([`Self::static_lexer_dfa`]) precomputes lexer
/// DFA tables after ATN construction and preserves both tokens and APIs.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptimizationOptions {
    pub(crate) entry_rules: BTreeSet<String>,
    pub(crate) prune_unreachable: bool,
    pub(crate) inline_trivial_rules: bool,
    pub(crate) report_trivial_rules: bool,
    pub(crate) optimize_precedence_ladders: bool,
    pub(crate) report_precedence_ladders: bool,
    pub(crate) static_lexer_dfa: bool,
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

    /// Reports what [`Self::inline_trivial_rules`] would change without
    /// applying it. Conflicts with `inline_trivial_rules`.
    pub const fn report_trivial_rules(&mut self, enabled: bool) -> &mut Self {
        self.report_trivial_rules = enabled;
        self
    }

    /// Collapses precedence-ladder rule chains (`a: b (op b)*; b: c ...`)
    /// into a single left-recursive rule. Default: `false`.
    pub const fn optimize_precedence_ladders(&mut self, enabled: bool) -> &mut Self {
        self.optimize_precedence_ladders = enabled;
        self
    }

    /// Reports what [`Self::optimize_precedence_ladders`] would change
    /// without applying it. Conflicts with `optimize_precedence_ladders`.
    pub const fn report_precedence_ladders(&mut self, enabled: bool) -> &mut Self {
        self.report_precedence_ladders = enabled;
        self
    }

    /// Precomputes static lexer DFA tables (`--static-lexer-dfa`, lexer scope
    /// only). Default: `false`.
    pub const fn static_lexer_dfa(&mut self, enabled: bool) -> &mut Self {
        self.static_lexer_dfa = enabled;
        self
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
    pub(crate) const fn report_trivial(&self) -> bool {
        self.report_trivial_rules
    }

    #[cfg(feature = "download")]
    pub(crate) const fn optimize_ladders(&self) -> bool {
        self.optimize_precedence_ladders
    }

    #[cfg(feature = "download")]
    pub(crate) const fn report_ladders(&self) -> bool {
        self.report_precedence_ladders
    }

    #[cfg(feature = "download")]
    pub(crate) const fn static_dfa(&self) -> bool {
        self.static_lexer_dfa
    }
}
