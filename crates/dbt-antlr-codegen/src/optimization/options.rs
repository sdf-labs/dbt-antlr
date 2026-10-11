// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeSet;

/// Grammar transform selection for one generation run.
///
/// All transforms are pure tool-side rewrites of the grammar model before ATN
/// construction: they are guaranteed to be recognition-preserving (i.e. the
/// generated parser accepts exactly the same language); however they are **NOT
/// tree-preserving**! Enabling these transformations may cause the generated
/// parse tree to deviate from what is implied by the .g4 grammar.
///
/// If enabled, every applied transform is recorded in an `optimizations.json`
/// manifest emitted next to the generated files, so optimization is explicit
/// and auditable.
///
/// The `report_*` variants run the transform in projection mode only: no
/// recognizer files are emitted, and the manifest describes what *would*
/// change. A `report_*` flag conflicts with the apply flag of the same
/// transform.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OptimizationOptions {
    pub(crate) entry_rules: BTreeSet<String>,
    pub(crate) prune_unreachable: bool,
    pub(crate) inline_trivial_rules: bool,
    pub(crate) report_trivial_rules: bool,
    pub(crate) optimize_precedence_ladders: bool,
    pub(crate) report_precedence_ladders: bool,
}

impl OptimizationOptions {
    /// Starts an empty selection (no transforms).
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

    #[cfg(feature = "download")]
    pub(crate) fn entry_rules(&self) -> impl Iterator<Item = &str> {
        self.entry_rules.iter().map(String::as_str)
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
}
