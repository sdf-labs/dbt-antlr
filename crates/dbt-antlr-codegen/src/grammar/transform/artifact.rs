// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
use std::collections::BTreeMap;

use crate::grammar::frontend::SourceId;
use crate::grammar::source::SourceSet;

pub(crate) fn render_unmodified_sources(sources: &SourceSet) -> BTreeMap<SourceId, String> {
    sources
        .iter()
        .map(|source| (source.id(), source.text().to_owned()))
        .collect()
}
