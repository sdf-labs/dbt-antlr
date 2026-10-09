// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin

/// Serialized ANTLR state kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AtnStateKind {
    Invalid,
    Basic,
    RuleStart,
    BlockStart,
    PlusBlockStart,
    StarBlockStart,
    TokenStart,
    RuleStop,
    BlockEnd,
    StarLoopBack,
    StarLoopEntry,
    PlusLoopBack,
    LoopEnd,
}
