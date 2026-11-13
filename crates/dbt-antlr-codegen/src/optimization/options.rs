// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeSet;

/// The analysis budget limits that one source sets. A limit that is `None`
/// comes from a source with a lower priority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct BudgetLimits {
    /// The state cap of a decision table.
    pub(crate) max_states: Option<usize>,
    /// The depth at which a hybrid table stops.
    pub(crate) max_k: Option<usize>,
    /// The configs that one build attempt can visit.
    pub(crate) max_visits: Option<u64>,
}

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
/// Static-only DFA generation ([`Self::static_dfa_only`]) selects both static
/// DFA passes and removes every `adaptivePredict` fallback. As a result, the
/// generated parser can reject input that the grammar accepts. A part of the
/// grammar that the static tables cannot decide stops the generation.
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
    pub(crate) static_dfa_only: bool,
    pub(crate) report_decisions: bool,
    pub(crate) dump_dfas: Option<String>,
    pub(crate) profile_decisions: bool,
    pub(crate) report_only: bool,
    pub(crate) dfa_budget: BudgetLimits,
    pub(crate) dfa_budget_specs: Vec<String>,
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

    /// Generates only fully static lexers and parsers (`--static-dfa-only`).
    /// Default: `false`.
    ///
    /// This option selects [`Self::static_lexer_dfa`] and
    /// [`Self::static_parser_dfa`]. The generated recognizers never fall back
    /// to the adaptive ATN simulation:
    ///
    /// - If a lexer mode does not compile to a complete static table, the
    ///   generation fails.
    /// - If a parser decision has no static table, the generation fails.
    /// - A partial parser table looks further than in the default mode.
    /// - In a partial parser table, a state with an SLL conflict predicts the
    ///   minimum conflicting alternative. A state where the lookahead budget
    ///   stops predicts its minimum viable alternative. A predicated state
    ///   reports a syntax error.
    ///
    /// In report-only mode, these failures are error diagnostics and the
    /// reports are still rendered.
    pub const fn static_dfa_only(&mut self, enabled: bool) -> &mut Self {
        self.static_dfa_only = enabled;
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

    /// Renders the time and the work that the `static-parser-dfa` pass spends
    /// on each parser decision (`--profile-decisions`) and implies
    /// [`Self::report_only`]. The times change from run to run.
    pub const fn profile_decisions(&mut self, enabled: bool) -> &mut Self {
        self.profile_decisions = enabled;
        self
    }

    /// Sets the state cap of each static parser DFA table
    /// (`--max-dfa-states`). Default: 1000.
    ///
    /// The cap must be at most 32767. When a table row has many token columns,
    /// the generator clamps the cap to the size that the table image can hold.
    /// The `max_dfa_states` option of a grammar, a rule, or a block, and
    /// [`Self::dfa_budget`], change the cap for some decisions.
    pub const fn max_dfa_states(&mut self, states: usize) -> &mut Self {
        self.dfa_budget.max_states = Some(states);
        self
    }

    /// Sets the depth at which a hybrid static parser DFA table stops
    /// (`--max-dfa-depth`). Default: 16 with
    /// [`Self::static_dfa_only`], otherwise 4.
    ///
    /// The `max_k` option of a grammar, a rule, or a block, and
    /// [`Self::dfa_budget`], change the depth for some decisions.
    pub const fn max_dfa_depth(&mut self, depth: usize) -> &mut Self {
        self.dfa_budget.max_k = Some(depth);
        self
    }

    /// Sets the configs that one static parser DFA build attempt can visit
    /// (`--max-dfa-visits`). Default: 2000 for each state of the state cap,
    /// and at least 2,000,000.
    ///
    /// The `max_dfa_visits` option of a grammar, a rule, or a block, and
    /// [`Self::dfa_budget`], change the budget for some decisions.
    pub const fn max_dfa_visits(&mut self, visits: u64) -> &mut Self {
        self.dfa_budget.max_visits = Some(visits);
        self
    }

    /// Sets the analysis budget of some parser decisions
    /// (`--dfa-budget <SPEC>`). Repeatable. A later setting wins over an
    /// earlier one.
    ///
    /// `spec` is `SELECTOR:SETTING[,SETTING...]`. The selector is a decision
    /// number, a rule name, or `rule:decision`. A setting is `states=N`,
    /// `k=N`, or `visits=N`. For example, `selectStmt:states=4000,k=6`.
    pub fn dfa_budget(&mut self, spec: impl Into<String>) -> &mut Self {
        self.dfa_budget_specs.push(spec.into());
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
    /// `--dump-dfas`, `--profile-decisions`).
    pub(crate) const fn is_report_only(&self) -> bool {
        self.report_only || self.has_parser_reports()
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

    #[cfg(feature = "download")]
    pub(crate) const fn static_dfa_only_enabled(&self) -> bool {
        self.static_dfa_only
    }

    #[cfg(feature = "download")]
    pub(crate) const fn dfa_budget_limits(&self) -> BudgetLimits {
        self.dfa_budget
    }

    #[cfg(feature = "download")]
    pub(crate) fn dfa_budget_specs(&self) -> impl Iterator<Item = &str> {
        self.dfa_budget_specs.iter().map(String::as_str)
    }

    /// Whether the selection prints parser reports, which only the in-process
    /// generator can render.
    pub(crate) const fn has_parser_reports(&self) -> bool {
        self.report_decisions || self.dump_dfas.is_some() || self.profile_decisions
    }
}
