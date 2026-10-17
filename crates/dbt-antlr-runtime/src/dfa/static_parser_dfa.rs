// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Bo Lin
//! Table-driven static parser SLL prediction and `SDFA` wire codec.

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use bit_set::BitSet;

pub const STATIC_PARSER_DFA_MAGIC: u32 = 0x5344_4641;
pub const FORMAT_VERSION: u32 = 1;

pub const ESCAPE: i32 = -1;
pub const CONFLICT_ESCAPE: i32 = -2;
pub const GUARDED: i32 = -3;
pub const VIRTUAL_ALT_BASE: i32 = 65536;

const HEADER_TOTAL_LEN: usize = 25;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StaticParserDfaError {
    TruncatedStream,
    BadMagic { found: u32 },
    UnsupportedVersion { found: u32 },
    LengthMismatch { declared: usize, actual: usize },
    InvalidRowIndex { row: i32, num_rows: usize },
    InvalidEdgeOffsets,
    InvalidEdgeBounds { lo: i32, hi: i32, edge_width: usize },
    InvalidEdgeTarget { target: i32, num_states: usize },
    InvalidStateIndex { state: usize, num_states: usize },
    InvalidDispatchDecision { decision: usize, num_slots: usize },
    InvalidDispatchTable { table_index: i32, num_tables: usize },
    UnsortedCutoffs,
    InvalidContinuationContext { index: u32, num_nodes: usize },
}

impl Display for StaticParserDfaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::TruncatedStream => f.write_str("truncated static parser DFA stream"),
            Self::BadMagic { found } => {
                write!(f, "bad static parser DFA magic 0x{found:08x}")
            }
            Self::UnsupportedVersion { found } => {
                write!(f, "unsupported static parser DFA format version {found}")
            }
            Self::LengthMismatch { declared, actual } => {
                write!(
                    f,
                    "static parser DFA section length {declared} does not match stream length {actual}"
                )
            }
            Self::InvalidRowIndex { row, num_rows } => {
                write!(f, "row index {row} out of bounds for {num_rows} rows")
            }
            Self::InvalidEdgeOffsets => f.write_str("invalid row edge offsets"),
            Self::InvalidEdgeBounds { lo, hi, edge_width } => {
                write!(
                    f,
                    "edge token range {lo}..={hi} out of bounds for width {edge_width}"
                )
            }
            Self::InvalidEdgeTarget { target, num_states } => {
                write!(
                    f,
                    "edge target {target} out of bounds for {num_states} states"
                )
            }
            Self::InvalidStateIndex { state, num_states } => {
                write!(
                    f,
                    "state index {state} out of bounds for {num_states} states"
                )
            }
            Self::InvalidDispatchDecision {
                decision,
                num_slots,
            } => {
                write!(
                    f,
                    "dispatch decision {decision} out of bounds for {num_slots} decision slots"
                )
            }
            Self::InvalidDispatchTable {
                table_index,
                num_tables,
            } => {
                write!(
                    f,
                    "dispatch table index {table_index} out of bounds for {num_tables} tables"
                )
            }
            Self::UnsortedCutoffs => {
                f.write_str("precedence dispatch cutoffs are not strictly sorted")
            }
            Self::InvalidContinuationContext { index, num_nodes } => {
                write!(
                    f,
                    "continuation context index {index} out of bounds for {num_nodes} nodes"
                )
            }
        }
    }
}

impl std::error::Error for StaticParserDfaError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContinuationContextNode {
    Singleton {
        parent: u32,
        return_state: i32,
    },
    Array {
        parents: Vec<u32>,
        return_states: Vec<i32>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContinuationConfigEntry {
    pub state: i32,
    pub alt: i32,
    pub context: u32,
    pub reaches_into_outer_context: i32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParserContinuation {
    pub state: u32,
    pub contexts: Vec<ContinuationContextNode>,
    pub configs: Vec<ContinuationConfigEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticDFATables {
    decisions: Vec<DecisionTables>,
    tables: Vec<StaticDFATable>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecisionTables {
    None,
    Single(usize),
    Dispatch(PrecedenceDispatch),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PrecedenceDispatch {
    pub cutoffs: Vec<i32>,
    pub class_tables: Vec<i32>,
    pub by_precedence: Box<[i32]>,
}

impl PrecedenceDispatch {
    pub fn new(cutoffs: Vec<i32>, class_tables: Vec<i32>) -> Self {
        let max_p = cutoffs.last().copied().unwrap_or(-1) + 1;
        let mut by_precedence = Vec::with_capacity((max_p + 1).max(1) as usize);
        for p in 0..=max_p.max(0) {
            let class = cutoffs.iter().take_while(|&&c| c < p).count();
            by_precedence.push(class_tables[class]);
        }
        Self {
            cutoffs,
            class_tables,
            by_precedence: by_precedence.into_boxed_slice(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticDFATable {
    pub decision: i32,
    pub accepts: Vec<i32>,
    pub fallbacks: Vec<i32>,
    pub guarded_alts: Option<Vec<i32>>,
    pub guard_danger: BitSet,
    pub guard_pass: BitSet,
    pub conflict_min_alts: Option<Vec<i32>>,
    pub continuations: Vec<ParserContinuation>,
    pub continuation_by_state: Option<Vec<u32>>,
    edge_width: usize,
    num_rows: usize,
    row_of: Vec<u32>,
    edges: Vec<i32>,
}

impl StaticDFATables {
    pub fn empty() -> Self {
        Self {
            decisions: Vec::new(),
            tables: Vec::new(),
        }
    }

    pub fn new(decisions: Vec<DecisionTables>, tables: Vec<StaticDFATable>) -> Self {
        Self { decisions, tables }
    }

    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    pub fn tables(&self) -> &[StaticDFATable] {
        &self.tables
    }

    pub fn decisions(&self) -> &[DecisionTables] {
        &self.decisions
    }

    #[inline]
    pub fn is_precedence_dispatched(&self, decision: i32) -> bool {
        matches!(
            self.decisions.get(decision as usize),
            Some(DecisionTables::Dispatch(_))
        )
    }

    #[inline]
    pub fn table(&self, decision: i32, precedence: i32) -> Option<&StaticDFATable> {
        let t = match self.decisions.get(decision as usize)? {
            DecisionTables::Single(t) => *t,
            DecisionTables::Dispatch(dispatch) => {
                let p = (precedence.max(0) as usize).min(dispatch.by_precedence.len() - 1);
                let t = dispatch.by_precedence[p];
                if t < 0 {
                    return None;
                }
                t as usize
            }
            DecisionTables::None => return None,
        };
        self.tables.get(t)
    }

    pub fn to_words(&self) -> Vec<u32> {
        if self.tables.is_empty() {
            return Vec::new();
        }
        let mut words = vec![
            STATIC_PARSER_DFA_MAGIC,
            FORMAT_VERSION,
            0,
            self.tables.len() as u32,
            self.decisions.len() as u32,
        ];

        for table in &self.tables {
            let num_states = table.accepts.len();
            let (row_edge_offsets, row_edges) = table.encode_distinct_rows();
            words.push(table.decision as u32);
            words.push(num_states as u32);
            words.push(table.edge_width as u32);
            words.push(table.num_rows as u32);
            words.push(row_edges.len() as u32);

            for &a in &table.accepts {
                if a == GUARDED || a == CONFLICT_ESCAPE {
                    words.push(ESCAPE as u32);
                } else {
                    words.push(a as u32);
                }
            }
            for &fb in &table.fallbacks {
                words.push(fb as u32);
            }
            for &r in &table.row_of {
                words.push(r);
            }
            for off in row_edge_offsets {
                words.push(off);
            }
            for edge_val in row_edges {
                words.push(edge_val as u32);
            }

            if let Some(guarded) = &table.guarded_alts {
                let pairs: Vec<(usize, i32)> = guarded
                    .iter()
                    .enumerate()
                    .filter(|&(_, &alt)| alt > 0)
                    .map(|(st, &alt)| (st, alt))
                    .collect();
                words.push(pairs.len() as u32);
                for (st, alt) in pairs {
                    words.push(st as u32);
                    words.push(alt as u32);
                }
            } else {
                words.push(0);
            }
            words.push(table.guard_danger.len() as u32);
            for st in &table.guard_danger {
                words.push(st as u32);
            }
            words.push(table.guard_pass.len() as u32);
            for st in &table.guard_pass {
                words.push(st as u32);
            }

            if let Some(conflict_alts) = &table.conflict_min_alts {
                let pairs: Vec<(usize, i32)> = conflict_alts
                    .iter()
                    .enumerate()
                    .filter(|&(_, &alt)| alt > 0)
                    .map(|(st, &alt)| (st, alt))
                    .collect();
                words.push(pairs.len() as u32);
                for (st, alt) in pairs {
                    words.push(st as u32);
                    words.push(alt as u32);
                }
            } else {
                words.push(0);
            }

            words.push(table.continuations.len() as u32);
            for cont in &table.continuations {
                words.push(cont.state);
                words.push(cont.contexts.len() as u32);
                for node in &cont.contexts {
                    match node {
                        ContinuationContextNode::Singleton {
                            parent,
                            return_state,
                        } => {
                            words.push(1);
                            words.push(*parent);
                            words.push(*return_state as u32);
                        }
                        ContinuationContextNode::Array {
                            parents,
                            return_states,
                        } => {
                            words.push(parents.len() as u32);
                            for (&p, &rs) in parents.iter().zip(return_states) {
                                words.push(p);
                                words.push(rs as u32);
                            }
                        }
                    }
                }
                words.push(cont.configs.len() as u32);
                for cfg in &cont.configs {
                    words.push(cfg.state as u32);
                    words.push(cfg.alt as u32);
                    words.push(cfg.context);
                    words.push(cfg.reaches_into_outer_context as u32);
                }
            }
        }

        let dispatches: Vec<(usize, &PrecedenceDispatch)> = self
            .decisions
            .iter()
            .enumerate()
            .filter_map(|(d, entry)| match entry {
                DecisionTables::Dispatch(disp) => Some((d, disp)),
                _ => None,
            })
            .collect();
        words.push(dispatches.len() as u32);
        for (decision, disp) in dispatches {
            words.push(decision as u32);
            words.push(disp.cutoffs.len() as u32);
            for &c in &disp.cutoffs {
                words.push(c as u32);
            }
            for &t in &disp.class_tables {
                words.push(t as u32);
            }
        }

        words[2] = words.len() as u32;
        words
    }

    #[allow(clippy::too_many_lines)]
    pub fn from_words(words: &[u32]) -> Result<Self, StaticParserDfaError> {
        if words.is_empty() {
            return Ok(Self::empty());
        }
        if words.len() < 5 {
            return Err(StaticParserDfaError::TruncatedStream);
        }
        if words[0] != STATIC_PARSER_DFA_MAGIC {
            return Err(StaticParserDfaError::BadMagic { found: words[0] });
        }
        if words[1] != FORMAT_VERSION {
            return Err(StaticParserDfaError::UnsupportedVersion { found: words[1] });
        }
        let declared_len = words[2] as usize;
        if declared_len != words.len() {
            return Err(StaticParserDfaError::LengthMismatch {
                declared: declared_len,
                actual: words.len(),
            });
        }
        let num_tables = words[3] as usize;
        let num_slots = words[4] as usize;
        let mut cursor = 5usize;

        let mut next_u32 = || -> Result<u32, StaticParserDfaError> {
            let val = *words
                .get(cursor)
                .ok_or(StaticParserDfaError::TruncatedStream)?;
            cursor += 1;
            Ok(val)
        };

        let mut tables = Vec::with_capacity(num_tables);
        let mut decisions = vec![DecisionTables::None; num_slots];

        for table_index in 0..num_tables {
            let decision = next_u32()? as i32;
            let num_states = next_u32()? as usize;
            let edge_width = next_u32()? as usize;
            let num_rows = next_u32()? as usize;
            let num_row_edge_ints = next_u32()? as usize;
            if !num_row_edge_ints.is_multiple_of(3) {
                return Err(StaticParserDfaError::InvalidEdgeOffsets);
            }

            let mut accepts = Vec::with_capacity(num_states);
            for _ in 0..num_states {
                accepts.push(next_u32()? as i32);
            }
            let mut fallbacks = Vec::with_capacity(num_states);
            for _ in 0..num_states {
                fallbacks.push(next_u32()? as i32);
            }
            let mut row_of = Vec::with_capacity(num_states);
            for _ in 0..num_states {
                let r = next_u32()? as i32;
                if r < 0 || (r as usize) >= num_rows {
                    return Err(StaticParserDfaError::InvalidRowIndex { row: r, num_rows });
                }
                row_of.push(r as u32);
            }
            let mut row_edge_offsets = Vec::with_capacity(num_rows + 1);
            for _ in 0..=num_rows {
                row_edge_offsets.push(next_u32()? as usize);
            }
            if row_edge_offsets.first().copied() != Some(0)
                || row_edge_offsets.last().copied() != Some(num_row_edge_ints)
                || row_edge_offsets
                    .windows(2)
                    .any(|w| w[0] > w[1] || !w[0].is_multiple_of(3))
            {
                return Err(StaticParserDfaError::InvalidEdgeOffsets);
            }

            let mut triples = Vec::with_capacity(num_row_edge_ints);
            for _ in 0..num_row_edge_ints {
                triples.push(next_u32()? as i32);
            }
            let mut edges = vec![-1i32; num_rows * edge_width];
            for r in 0..num_rows {
                let lo_tri = row_edge_offsets[r] / 3;
                let hi_tri = row_edge_offsets[r + 1] / 3;
                let row = &mut edges[r * edge_width..(r + 1) * edge_width];
                for tri in lo_tri..hi_tri {
                    let lo = triples[tri * 3];
                    let hi = triples[tri * 3 + 1];
                    let target = triples[tri * 3 + 2];
                    if lo < -1 || hi < lo || (hi + 1) as usize >= edge_width {
                        return Err(StaticParserDfaError::InvalidEdgeBounds { lo, hi, edge_width });
                    }
                    if target < 0 || (target as usize) >= num_states {
                        return Err(StaticParserDfaError::InvalidEdgeTarget { target, num_states });
                    }
                    for tok in lo..=hi {
                        row[(tok + 1) as usize] = target;
                    }
                }
            }

            let num_guarded_states = next_u32()? as usize;
            let mut guarded_alts = None;
            if num_guarded_states > 0 {
                let mut alts = vec![0i32; num_states];
                for _ in 0..num_guarded_states {
                    let state = next_u32()? as usize;
                    if state >= num_states {
                        return Err(StaticParserDfaError::InvalidStateIndex { state, num_states });
                    }
                    alts[state] = next_u32()? as i32;
                    accepts[state] = GUARDED;
                }
                guarded_alts = Some(alts);
            }
            let num_danger = next_u32()? as usize;
            let mut guard_danger = BitSet::new();
            for _ in 0..num_danger {
                guard_danger.insert(next_u32()? as usize);
            }
            let num_pass = next_u32()? as usize;
            let mut guard_pass = BitSet::new();
            for _ in 0..num_pass {
                guard_pass.insert(next_u32()? as usize);
            }

            let num_conflict_escapes = next_u32()? as usize;
            let mut conflict_min_alts = None;
            if num_conflict_escapes > 0 {
                let mut alts = vec![0i32; num_states];
                for _ in 0..num_conflict_escapes {
                    let state = next_u32()? as usize;
                    if state >= num_states {
                        return Err(StaticParserDfaError::InvalidStateIndex { state, num_states });
                    }
                    alts[state] = next_u32()? as i32;
                    accepts[state] = CONFLICT_ESCAPE;
                }
                conflict_min_alts = Some(alts);
            }

            let num_continuations = next_u32()? as usize;
            let mut continuations = Vec::with_capacity(num_continuations);
            let mut continuation_by_state = None;
            if num_continuations > 0 {
                let mut by_state = vec![u32::MAX; num_states];
                for cont_idx in 0..num_continuations {
                    let state = next_u32()?;
                    if (state as usize) >= num_states {
                        return Err(StaticParserDfaError::InvalidStateIndex {
                            state: state as usize,
                            num_states,
                        });
                    }
                    let num_ctx_nodes = next_u32()? as usize;
                    let mut contexts = Vec::with_capacity(num_ctx_nodes);
                    for node_idx in 0..num_ctx_nodes {
                        let arity = next_u32()? as usize;
                        if arity == 0 {
                            return Err(StaticParserDfaError::InvalidContinuationContext {
                                index: node_idx as u32,
                                num_nodes: num_ctx_nodes,
                            });
                        }
                        if arity == 1 {
                            let parent = next_u32()?;
                            if parent != u32::MAX && (parent as usize) >= node_idx {
                                return Err(StaticParserDfaError::InvalidContinuationContext {
                                    index: parent,
                                    num_nodes: node_idx,
                                });
                            }
                            let return_state = next_u32()? as i32;
                            contexts.push(ContinuationContextNode::Singleton {
                                parent,
                                return_state,
                            });
                        } else {
                            let mut parents = Vec::with_capacity(arity);
                            let mut return_states = Vec::with_capacity(arity);
                            for _ in 0..arity {
                                let parent = next_u32()?;
                                if parent != u32::MAX && (parent as usize) >= node_idx {
                                    return Err(StaticParserDfaError::InvalidContinuationContext {
                                        index: parent,
                                        num_nodes: node_idx,
                                    });
                                }
                                let return_state = next_u32()? as i32;
                                parents.push(parent);
                                return_states.push(return_state);
                            }
                            contexts.push(ContinuationContextNode::Array {
                                parents,
                                return_states,
                            });
                        }
                    }
                    let num_configs = next_u32()? as usize;
                    let mut configs = Vec::with_capacity(num_configs);
                    for _ in 0..num_configs {
                        let cfg_state = next_u32()? as i32;
                        let alt = next_u32()? as i32;
                        let context = next_u32()?;
                        if context != u32::MAX && (context as usize) >= num_ctx_nodes {
                            return Err(StaticParserDfaError::InvalidContinuationContext {
                                index: context,
                                num_nodes: num_ctx_nodes,
                            });
                        }
                        let reaches_into_outer_context = next_u32()? as i32;
                        configs.push(ContinuationConfigEntry {
                            state: cfg_state,
                            alt,
                            context,
                            reaches_into_outer_context,
                        });
                    }
                    by_state[state as usize] = cont_idx as u32;
                    continuations.push(ParserContinuation {
                        state,
                        contexts,
                        configs,
                    });
                }
                continuation_by_state = Some(by_state);
            }

            if decision >= 0 {
                if (decision as usize) >= num_slots {
                    return Err(StaticParserDfaError::InvalidDispatchDecision {
                        decision: decision as usize,
                        num_slots,
                    });
                }
                decisions[decision as usize] = DecisionTables::Single(table_index);
            }
            tables.push(StaticDFATable {
                decision,
                accepts,
                fallbacks,
                guarded_alts,
                guard_danger,
                guard_pass,
                conflict_min_alts,
                continuations,
                continuation_by_state,
                edge_width,
                num_rows,
                row_of,
                edges,
            });
        }

        let num_dispatches = next_u32()? as usize;
        for _ in 0..num_dispatches {
            let decision = next_u32()? as usize;
            if decision >= num_slots {
                return Err(StaticParserDfaError::InvalidDispatchDecision {
                    decision,
                    num_slots,
                });
            }
            let num_cutoffs = next_u32()? as usize;
            let mut cutoffs = Vec::with_capacity(num_cutoffs);
            for _ in 0..num_cutoffs {
                cutoffs.push(next_u32()? as i32);
            }
            if cutoffs.windows(2).any(|w| w[0] >= w[1]) {
                return Err(StaticParserDfaError::UnsortedCutoffs);
            }
            let mut class_tables = Vec::with_capacity(num_cutoffs + 1);
            for _ in 0..=num_cutoffs {
                let t = next_u32()? as i32;
                if t < -1 || (t >= 0 && (t as usize) >= num_tables) {
                    return Err(StaticParserDfaError::InvalidDispatchTable {
                        table_index: t,
                        num_tables,
                    });
                }
                class_tables.push(t);
            }
            decisions[decision] =
                DecisionTables::Dispatch(PrecedenceDispatch::new(cutoffs, class_tables));
        }

        if cursor != words.len() {
            return Err(StaticParserDfaError::LengthMismatch {
                declared: cursor,
                actual: words.len(),
            });
        }

        Ok(Self { decisions, tables })
    }
}

impl StaticDFATable {
    #[allow(clippy::too_many_arguments)]
    pub fn from_triples(
        decision: i32,
        num_states: usize,
        mut accepts: Vec<i32>,
        fallbacks: Vec<i32>,
        edge_offsets: &[usize],
        edge_triples: &[i32],
        guarded_alts: Option<Vec<i32>>,
        guard_danger: BitSet,
        guard_pass: BitSet,
        conflict_min_alts: Option<Vec<i32>>,
        continuations: Vec<ParserContinuation>,
    ) -> Self {
        let mut max_tok = -1i32;
        let mut idx = 0;
        while idx < edge_triples.len() {
            let hi = edge_triples[idx + 1];
            if hi > max_tok {
                max_tok = hi;
            }
            idx += 3;
        }
        let edge_width = (max_tok + 2) as usize;

        let mut row_map: HashMap<&[i32], u32> = HashMap::new();
        let mut distinct_rows: Vec<&[i32]> = Vec::new();
        let mut row_of = Vec::with_capacity(num_states);
        for s in 0..num_states {
            let slice = &edge_triples[edge_offsets[s]..edge_offsets[s + 1]];
            let row_idx = if let Some(&existing) = row_map.get(slice) {
                existing
            } else {
                let new_idx = distinct_rows.len() as u32;
                distinct_rows.push(slice);
                row_map.insert(slice, new_idx);
                new_idx
            };
            row_of.push(row_idx);
        }

        let num_rows = distinct_rows.len();
        let mut edges = vec![-1i32; num_rows * edge_width];
        for (r, slice) in distinct_rows.iter().enumerate() {
            let row = &mut edges[r * edge_width..(r + 1) * edge_width];
            let mut e = 0;
            while e < slice.len() {
                let lo = slice[e];
                let hi = slice[e + 1];
                let target = slice[e + 2];
                for tok in lo..=hi {
                    row[(tok + 1) as usize] = target;
                }
                e += 3;
            }
        }

        let guarded_alts = guarded_alts.filter(|alts| alts.iter().any(|&a| a > 0));
        if let Some(alts) = &guarded_alts {
            for (s, &alt) in alts.iter().enumerate() {
                if alt > 0 {
                    accepts[s] = GUARDED;
                }
            }
        }

        let conflict_min_alts = conflict_min_alts.filter(|alts| alts.iter().any(|&a| a > 0));
        if let Some(alts) = &conflict_min_alts {
            for (s, &alt) in alts.iter().enumerate() {
                if alt > 0 {
                    accepts[s] = CONFLICT_ESCAPE;
                }
            }
        }

        let continuation_by_state = if continuations.is_empty() {
            None
        } else {
            let mut by_state = vec![u32::MAX; num_states];
            for (cont_idx, cont) in continuations.iter().enumerate() {
                by_state[cont.state as usize] = cont_idx as u32;
            }
            Some(by_state)
        };

        Self {
            decision,
            accepts,
            fallbacks,
            guarded_alts,
            guard_danger,
            guard_pass,
            conflict_min_alts,
            continuations,
            continuation_by_state,
            edge_width,
            num_rows,
            row_of,
            edges,
        }
    }

    fn encode_distinct_rows(&self) -> (Vec<u32>, Vec<i32>) {
        let mut offsets = Vec::with_capacity(self.num_rows + 1);
        let mut triples = Vec::new();
        for r in 0..self.num_rows {
            offsets.push(triples.len() as u32);
            let row = &self.edges[r * self.edge_width..(r + 1) * self.edge_width];
            let mut col = 0usize;
            while col < self.edge_width {
                let target = row[col];
                if target >= 0 {
                    let start_col = col;
                    while col + 1 < self.edge_width && row[col + 1] == target {
                        col += 1;
                    }
                    triples.push(start_col as i32 - 1);
                    triples.push(col as i32 - 1);
                    triples.push(target);
                }
                col += 1;
            }
        }
        offsets.push(triples.len() as u32);
        (offsets, triples)
    }

    #[inline]
    pub fn accept(&self, state: i32) -> Option<i32> {
        let alt = self.accepts[state as usize];
        if alt > 0 {
            Some(alt)
        } else {
            None
        }
    }

    #[inline]
    pub fn is_escape(&self, state: i32) -> bool {
        self.accepts[state as usize] == ESCAPE
    }

    #[inline]
    pub fn conflict_min_alt(&self, state: i32) -> Option<i32> {
        match &self.conflict_min_alts {
            Some(alts) if alts[state as usize] > 0 => Some(alts[state as usize]),
            _ => None,
        }
    }

    #[inline]
    pub fn continuation(&self, state: i32) -> Option<(usize, &ParserContinuation)> {
        let idx = *self.continuation_by_state.as_ref()?.get(state as usize)?;
        if idx == u32::MAX {
            None
        } else {
            Some((idx as usize, &self.continuations[idx as usize]))
        }
    }

    #[inline]
    pub fn guarded_alt(&self, state: i32) -> Option<i32> {
        match &self.guarded_alts {
            Some(alts) if alts[state as usize] > 0 => Some(alts[state as usize]),
            _ => None,
        }
    }

    #[inline]
    pub fn guard_step(&self, invoking_state: i32) -> Option<bool> {
        let s = invoking_state as usize;
        if self.guard_danger.contains(s) {
            Some(true)
        } else if !self.guard_pass.contains(s) {
            Some(false)
        } else {
            None
        }
    }

    #[inline]
    pub fn fallback(&self, state: i32) -> Option<i32> {
        let alt = self.fallbacks[state as usize];
        if alt > 0 {
            Some(alt)
        } else {
            None
        }
    }

    #[inline]
    pub fn edge(&self, state: i32, t: i32) -> Option<i32> {
        let col = t.wrapping_add(1) as usize;
        if col >= self.edge_width {
            return None;
        }
        let row = unsafe { *self.row_of.get_unchecked(state as usize) } as usize;
        let target = unsafe { *self.edges.get_unchecked(row * self.edge_width + col) };
        if target < 0 {
            None
        } else {
            Some(target)
        }
    }
}

pub fn append_sdfa_section(packed_words: &[u32], sdfa_words: &[u32]) -> Vec<u32> {
    if sdfa_words.is_empty() {
        return packed_words.to_vec();
    }
    let mut out = Vec::with_capacity(packed_words.len() + sdfa_words.len());
    out.extend_from_slice(packed_words);
    out.extend_from_slice(sdfa_words);
    out[HEADER_TOTAL_LEN] = out.len() as u32;
    out
}
