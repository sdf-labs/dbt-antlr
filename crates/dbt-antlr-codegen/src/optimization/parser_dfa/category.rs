// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Decision categories of the parser DFA classifier.

use std::fmt::{self, Display, Formatter};

/// How a prediction decision is classified.
///
/// The categories are the vocabulary of `--report-decisions`: they say *why* a
/// decision does or does not get a static table.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
pub(crate) enum Category {
    /// Alternatives have disjoint first tokens: the generated code predicts
    /// with plain `switch`/`if` chains, no DFA at all.
    LL1,
    /// A finite SLL DFA decides the alternatives after exactly `k` tokens.
    #[default]
    LLK,
    /// A finite SLL DFA decides, but it has cycles: unbounded lookahead.
    LLSTAR,
    /// Every alternative can match every witness: a true ambiguity.
    EXACT_AMBIG,
    /// An SLL conflict that only full-context (LL) prediction can resolve.
    CONTEXT_SENSITIVE,
    /// A predicate is crossed, so the prediction depends on parser state.
    PREDICATED,
    /// A non-greedy loop: the decision always prefers the lowest alternative.
    NON_GREEDY,
    /// A precedence-loop decision of a left-recursive rule: classified per
    /// precedence class.
    LR_PRECEDENCE,
    /// The construction hit an analysis budget - the state graph, the visited
    /// config budget, or the alternative-number range - without a usable
    /// approximation.
    OVERFLOW,
    /// A partial table: some states are static, the rest escape to the
    /// adaptive simulation.
    HYBRID,
}

impl Category {
    /// The categories in report order.
    pub(crate) const ALL: [Self; 10] = [
        Self::LL1,
        Self::LLK,
        Self::LLSTAR,
        Self::EXACT_AMBIG,
        Self::CONTEXT_SENSITIVE,
        Self::PREDICATED,
        Self::NON_GREEDY,
        Self::LR_PRECEDENCE,
        Self::OVERFLOW,
        Self::HYBRID,
    ];

    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::LL1 => "LL1",
            Self::LLK => "LLK",
            Self::LLSTAR => "LLSTAR",
            Self::EXACT_AMBIG => "EXACT_AMBIG",
            Self::CONTEXT_SENSITIVE => "CONTEXT_SENSITIVE",
            Self::PREDICATED => "PREDICATED",
            Self::NON_GREEDY => "NON_GREEDY",
            Self::LR_PRECEDENCE => "LR_PRECEDENCE",
            Self::OVERFLOW => "OVERFLOW",
            Self::HYBRID => "HYBRID",
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
