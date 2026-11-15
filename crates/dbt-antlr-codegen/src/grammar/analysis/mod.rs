// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Offline grammar analysis: the parked SLL decision classifier.
//!
//! [`DecisionClassifier`] classified each prediction decision of a parser in
//! the categories of [`Category`] (`LL1`, `LLK`, `LLSTAR`, `EXACT_AMBIG`,
//! `CONTEXT_SENSITIVE`, `PREDICATED`, and so on), and `--report-decisions`
//! printed the result. The categories describe the grammar, not the generated
//! parser, so the classifier tried hard to find the true category of a
//! decision, for example with a second pass that found the conflict witness.
//!
//! The static parser DFA pass now uses `DecisionTableBuilder` of
//! `optimization::parser_dfa`, which builds the tables that the generated
//! parser uses and reports what the generated parser does. The classifier is
//! parked here and is not compiled. To use it again, make it compile against
//! the current `optimization::parser_dfa` modules, remove the `cfg` of this
//! module in `grammar`, and give it its own command-line flag.

mod category;
mod classifier;
mod report;
#[cfg(test)]
mod tests;

pub(crate) use category::Category;
pub(crate) use classifier::{DecisionClassifier, DecisionResult};
