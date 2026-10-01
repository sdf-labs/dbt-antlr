// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
use std::fmt;

use dbt_antlr_runtime::interval_set::IntervalSetBuf;

/// Ordered set of integer intervals used by set and negated-set transitions.
///
/// Construction goes through the shared runtime builder, which stores ranges
/// sorted and merges overlapping and adjacent intervals on insertion.
#[derive(Clone, Default, Eq, PartialEq)]
pub(crate) struct IntervalSet {
    inner: IntervalSetBuf,
}

impl IntervalSet {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn from_range(start: i32, stop: i32) -> Self {
        let mut set = Self::new();
        set.add_range(start, stop);
        set
    }

    pub(crate) fn add(&mut self, value: i32) {
        self.inner.add_one(value);
    }

    /// Adds an inclusive interval and merges it with adjacent or overlapping
    /// intervals.
    pub(crate) fn add_range(&mut self, start: i32, stop: i32) {
        let (start, stop) = if start <= stop {
            (start, stop)
        } else {
            (stop, start)
        };
        self.inner.add_range(start, stop);
    }

    pub(crate) fn ranges(&self) -> Vec<(i32, i32)> {
        self.inner
            .as_slice()
            .iter()
            .map(|interval| (interval.a, interval.b))
            .collect()
    }
}

impl fmt::Debug for IntervalSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IntervalSet")
            .field("ranges", &self.ranges())
            .finish()
    }
}
