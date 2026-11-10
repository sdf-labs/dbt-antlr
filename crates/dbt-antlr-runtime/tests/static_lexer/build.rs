//! Integration tests for the compile-time static lexer DFA builder
//! (`static-dfa-build` feature). Grammars live under `grammars/` and
//! are compiled to lexer ATN words through `dbt-antlr-codegen`'s `atn-export`.
//!
//! The per-grammar expectations assert directly on the committed
//! [`StaticLexerDFA`] tables of `build.modes`. A test-only image decoder
//! (`check_invariants`) keeps the packed image layout itself covered.
#![cfg(feature = "static-dfa-build")]

use std::path::PathBuf;

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::dfa::static_dfa_build::{
    ModeFailure, StaticLexerDfaBuildOutput, StaticLexerDfaBuilder,
};
use dbt_antlr_runtime::dfa::static_lexer_dfa::{
    Accel, StaticLexerDFA, StaticLexerTables, ACCEPT_BIT, CONTINUATION_MOVED_CONFIGS,
    CONTINUATION_SOURCE_SET, ESCAPE, NO_CONTINUATION, NO_EDGE, STATE_MASK,
    STATIC_DFA_FORMAT_VERSION, STATIC_DFA_MAGIC,
};
use dbt_antlr_runtime::int_stream::EOF;

fn grammar_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("grammars")
        .join(name)
}

fn build_with(grammar: &str, builder: &StaticLexerDfaBuilder) -> (ATN, StaticLexerDfaBuildOutput) {
    let units = dbt_antlr_codegen::atn_export::compile_atn_data(&grammar_path(grammar), &[])
        .expect("grammar compiles");
    let words = units
        .iter()
        .find_map(|u| u.lexer_atn_words.clone())
        .expect("lexer unit");
    let atn = ATNDeserializer::new(None).deserialize(&mut words.iter());
    let build = builder.build(&atn);
    (atn, build)
}

fn build(grammar: &str) -> (ATN, StaticLexerDfaBuildOutput) {
    build_with(grammar, &StaticLexerDfaBuilder::new())
}

// ======= Structural helpers over the committed tables =======

/// Follows `text` from `from`; every edge must be a plain edge.
fn walk(dfa: &StaticLexerDFA, from: u16, text: &str) -> u16 {
    let mut s = from;
    for c in text.chars() {
        // SAFETY: `s` is the caller's state or a walked edge target; the check
        // keeps it a valid state number.
        assert!(
            (s as usize) < dfa.num_states(),
            "walk: state {s} is not a state"
        );
        let entry = unsafe { dfa.edge_entry(s, c as i32) };
        assert!(
            entry != NO_EDGE && entry != ESCAPE,
            "walk: edge on {c:?} from state {s} must be a plain edge, got {entry:#06x}"
        );
        s = entry & STATE_MASK;
    }
    s
}

/// The sorted, deduplicated accept token types of the mode.
fn accept_types(dfa: &StaticLexerDFA) -> Vec<i32> {
    let mut types: Vec<i32> = (0..dfa.num_states() as u16)
        .map(|s| {
            // SAFETY: `s` comes from `0..num_states`.
            unsafe { dfa.accept_type(s) }
        })
        .filter(|&t| t >= 0)
        .collect();
    types.sort_unstable();
    types.dedup();
    types
}

/// True when no edge of the mode escapes (symbol edges and EOF edges).
fn has_no_escapes(dfa: &StaticLexerDFA) -> bool {
    (0..dfa.num_states() as u16).all(|s| {
        // SAFETY: `s` comes from `0..num_states`.
        let eof = unsafe { dfa.edge_entry(s, EOF) };
        eof != ESCAPE
            && unsafe { dfa.escape_continuation(s, EOF) } == NO_CONTINUATION
            && dfa.escape_ranges(s).is_empty()
    })
}

// ======= Test-only decoder for the packed image =======

/// Header of a packed mode image, decoded from its raw words.
#[derive(Debug)]
struct ImageHeader {
    total_words: usize,
    num_states: usize,
    num_classes: usize,
    num_hi_rows: usize,
    start_state: u32,
    eof: usize,
    low: usize,
    walker_tag: usize,
    walker_accept: usize,
    walker_positive: usize,
    walker_positive_len: usize,
    class_starts: usize,
    hi_rows: usize,
    row_of: usize,
    accept_actions: usize,
    actions: usize,
    actions_len: usize,
    accel_tag: usize,
    accel_data: usize,
    accel_data_len: usize,
    eof_escape: usize,
    escape_off: usize,
    escape_ranges: usize,
    escape_ranges_len: usize,
    continuations: usize,
    continuations_len: usize,
    continuation_ctxs: usize,
    continuation_ctxs_len: usize,
    continuation_configs: usize,
    continuation_configs_len: usize,
    continuation_actions: usize,
    continuation_actions_len: usize,
}

impl ImageHeader {
    fn read(image: &[u32]) -> Self {
        assert!(image.len() >= 32, "mode image holds the 32-word header");
        let word = |index: usize| image[index] as usize;
        Self {
            total_words: word(0),
            num_states: word(1),
            num_classes: word(2),
            num_hi_rows: word(3),
            start_state: image[4],
            eof: word(5),
            low: word(6),
            walker_tag: word(7),
            walker_accept: word(8),
            walker_positive: word(9),
            walker_positive_len: word(10),
            class_starts: word(11),
            hi_rows: word(12),
            row_of: word(13),
            accept_actions: word(14),
            actions: word(15),
            actions_len: word(16),
            accel_tag: word(17),
            accel_data: word(18),
            accel_data_len: word(19),
            eof_escape: word(20),
            escape_off: word(21),
            escape_ranges: word(22),
            escape_ranges_len: word(23),
            continuations: word(24),
            continuations_len: word(25),
            continuation_ctxs: word(26),
            continuation_ctxs_len: word(27),
            continuation_configs: word(28),
            continuation_configs_len: word(29),
            continuation_actions: word(30),
            continuation_actions_len: word(31),
        }
    }
}

#[derive(Debug, Default)]
struct DecodedMode {
    present: bool,
    accept_action_triples: Vec<Vec<(i32, i32, i32)>>,
    num_states: usize,
    start_state: u32,
    accept_type: Vec<i32>,
    eof: Vec<u16>,
    low: Vec<[u16; 128]>,
    class_starts: Vec<i32>,
    hi_rows: Vec<Vec<u16>>,
    row_of: Vec<u32>,
    escape_rows: Vec<Vec<(i32, i32, u32)>>,
    /// The run-acceleration descriptor of every state.
    accel: Vec<Option<Accel>>,
    /// The EOF escape index of every state.
    eof_escape: Vec<u32>,
    continuations: Vec<DecodedCont>,
}

#[derive(Debug)]
struct DecodedCont {
    kind: u32,
    ctxs: Vec<(u32, i32)>,
    configs: Vec<DecodedCfg>,
}

#[derive(Debug)]
struct DecodedCfg {
    state: u32,
    flags: u32,
    alt: u32,
    ctx_local: u32,
    actions: Vec<u32>,
}

/// A section of the image as the test decoder reads it: byte offset and
/// element count.
fn section<'a>(image: &'a [u32], offset: usize, len: usize, what: &str) -> &'a [u32] {
    assert_eq!(offset % 4, 0, "{what}: section offset is word aligned");
    let words = offset / 4;
    let end = words.checked_add(len).expect("section end");
    assert!(
        end <= image.len(),
        "{what}: section ends at word {end} of {}",
        image.len()
    );
    &image[words..end]
}

/// The `u16` entries of a word table, two per word.
fn u16s(words: &[u32], entries: usize) -> Vec<u16> {
    (0..entries)
        .map(|index| (words[index >> 1] >> ((index & 1) * 16)) as u16)
        .collect()
}

fn decode(image: &[u32]) -> Vec<DecodedMode> {
    assert_eq!(image[0], STATIC_DFA_MAGIC);
    assert_eq!(image[1], STATIC_DFA_FORMAT_VERSION);
    assert_eq!(image[2] as usize, image.len(), "container length");
    let num_modes = image[3] as usize;
    let modes_end = 4 + 2 * num_modes;
    let data = &image[modes_end..];
    let mut modes = Vec::with_capacity(num_modes);
    let mut consumed = 0;
    for mode in 0..num_modes {
        let offset = image[4 + 2 * mode];
        let len = image[4 + 2 * mode + 1];
        if offset == u32::MAX {
            assert_eq!(len, u32::MAX, "mode {mode} absent length");
            modes.push(DecodedMode::default());
            continue;
        }
        let offset = offset as usize;
        let len = len as usize;
        assert_eq!(offset, consumed, "mode {mode} offset");
        let mode_image = &data[offset..offset + len];
        let header = ImageHeader::read(mode_image);
        assert_eq!(header.total_words, len, "mode {mode} length");
        consumed = offset + len;
        modes.push(decode_mode(mode_image, &header));
    }
    assert_eq!(
        consumed,
        data.len(),
        "the mode table covers the data section"
    );
    modes
}

#[allow(clippy::too_many_lines)]
fn decode_mode(image: &[u32], header: &ImageHeader) -> DecodedMode {
    let n = header.num_states;
    assert!(header.total_words <= image.len(), "mode image length");
    let image = &image[..header.total_words];
    // The sections tile the image in the order of the header fields, so a
    // section offset is the end of the previous section.
    let layout = [
        (header.eof, n.div_ceil(2) * 4),
        (header.low, (n * 128).div_ceil(2) * 4),
        (header.walker_tag, n.div_ceil(2) * 4),
        (header.walker_accept, n * 8),
        (header.walker_positive, header.walker_positive_len * 20),
        (header.class_starts, header.num_classes * 4),
        (
            header.hi_rows,
            (header.num_hi_rows * header.num_classes).div_ceil(2) * 4,
        ),
        (header.row_of, n * 4),
        (header.accept_actions, n * 8),
        (header.actions, header.actions_len * 4 * 4),
        (header.accel_tag, n.div_ceil(2) * 4),
        (header.accel_data, header.accel_data_len * 8 * 4),
        (header.eof_escape, n * 4),
        (header.escape_off, (n + 1) * 4),
        (header.escape_ranges, header.escape_ranges_len * 12),
        (header.continuations, header.continuations_len * 20),
        (header.continuation_ctxs, header.continuation_ctxs_len * 8),
        (
            header.continuation_configs,
            header.continuation_configs_len * 24,
        ),
        (
            header.continuation_actions,
            header.continuation_actions_len * 4,
        ),
    ];
    let mut next = 32 * 4;
    for (offset, len) in layout {
        assert_eq!(offset, next, "sections tile the image in header order");
        next += len;
    }
    assert_eq!(next, header.total_words * 4, "the sections fill the image");
    let eof = u16s(section(image, header.eof, n.div_ceil(2), "eof"), n);
    let low = u16s(
        section(image, header.low, (n * 128).div_ceil(2), "low"),
        n * 128,
    );
    let walker_accept = section(image, header.walker_accept, n * 2, "walker_accept");
    let accept_type: Vec<i32> = walker_accept
        .as_chunks::<2>()
        .0
        .iter()
        .map(|record| {
            assert!(record[1] <= 2, "walker accept action tag in 0..=2");
            record[0] as i32
        })
        .collect();
    let spans = section(image, header.accept_actions, n * 2, "accept_actions");
    for record in section(image, header.actions, header.actions_len * 4, "actions").chunks(4) {
        let ty = record[0] as i32;
        assert!(
            (0..=7).contains(&ty) && ty != 1,
            "accept actions are position-independent LexerAction records"
        );
        assert_eq!(record[2], 0, "unused union payload word 2 is zero");
        assert_eq!(record[3], 0, "unused union payload word 3 is zero");
    }
    let accel_tags = u16s(
        section(image, header.accel_tag, n.div_ceil(2), "accel_tag"),
        n,
    );
    let accel_data = section(
        image,
        header.accel_data,
        header.accel_data_len * 8,
        "accel_data",
    );
    let accel: Vec<Option<Accel>> = accel_tags
        .iter()
        .map(|&tag| {
            assert!(
                (tag as usize) <= header.accel_data_len,
                "accel tag in range"
            );
            (tag != 0).then(|| decode_accel(&accel_data[(tag as usize - 1) * 8..][..8]))
        })
        .collect();
    let eof_escape = section(image, header.eof_escape, n, "eof_escape");
    let escape_off = section(image, header.escape_off, n + 1, "escape_off");
    let escape_ranges = section(
        image,
        header.escape_ranges,
        header.escape_ranges_len * 3,
        "escape_ranges",
    );
    let class_starts: Vec<i32> = section(
        image,
        header.class_starts,
        header.num_classes,
        "class_starts",
    )
    .iter()
    .map(|&word| word as i32)
    .collect();
    let hi_rows_entries = header.num_hi_rows * header.num_classes;
    let hi_rows = u16s(
        section(
            image,
            header.hi_rows,
            hi_rows_entries.div_ceil(2),
            "hi_rows",
        ),
        hi_rows_entries,
    );
    let row_of = section(image, header.row_of, n, "row_of").to_vec();

    if header.num_classes > 0 {
        assert_eq!(class_starts[0], 128, "class starts begin at 128");
        for pair in class_starts.windows(2) {
            assert!(pair[0] < pair[1], "class starts increase");
        }
        assert!(
            *class_starts.last().unwrap() <= 0x10FFFF,
            "class starts in range"
        );
    }
    if header.num_classes > 0 {
        for &row in &row_of {
            assert!((row as usize) < header.num_hi_rows, "row of state in range");
        }
    }

    let mut accept_action_triples = Vec::with_capacity(n);
    for state in 0..n {
        let off = spans[2 * state] as usize;
        let len = spans[2 * state + 1] as usize;
        assert!(off + len <= header.actions_len, "action span in range");
        accept_action_triples.push(
            (0..len)
                .map(|index| {
                    let records = section(image, header.actions, header.actions_len * 4, "actions");
                    let record = &records[(off + index) * 4..(off + index) * 4 + 4];
                    (record[0] as i32, record[1] as i32, record[2] as i32)
                })
                .collect(),
        );
    }

    let mut low_rows = Vec::with_capacity(n);
    let mut escape_rows = Vec::with_capacity(n);
    for state in 0..n {
        let mut row = [0u16; 128];
        row.copy_from_slice(&low[state * 128..(state + 1) * 128]);
        low_rows.push(row);
        let from = escape_off[state] as usize;
        let to = escape_off[state + 1] as usize;
        assert!(from <= to, "escape rows are monotonic");
        assert!(to <= header.escape_ranges_len, "escape row in range");
        let mut ranges = Vec::with_capacity(to - from);
        let mut prev_hi: Option<i32> = None;
        for range in from..to {
            let lo = escape_ranges[range * 3] as i32;
            let hi = escape_ranges[range * 3 + 1] as i32;
            let continuation = escape_ranges[range * 3 + 2];
            assert!(lo >= 0 && lo <= hi && hi <= 0x10FFFF, "escape range bounds");
            assert!(
                prev_hi.is_none_or(|p| p < lo),
                "escape ranges are sorted and disjoint"
            );
            assert!(
                continuation == NO_CONTINUATION
                    || (continuation as usize) < header.continuations_len,
                "escape continuation in range"
            );
            prev_hi = Some(hi);
            ranges.push((lo, hi, continuation));
        }
        escape_rows.push(ranges);
    }
    for &offset in escape_off {
        assert!((offset as usize) <= header.escape_ranges_len);
    }
    for state in 0..n {
        let eof_escape = eof_escape[state];
        if eof_escape != NO_CONTINUATION {
            assert_eq!(
                eof[state], ESCAPE,
                "the EOF escape index needs an EOF escape edge"
            );
            assert!(
                (eof_escape as usize) < header.continuations_len,
                "the EOF escape continuation is in range"
            );
        }
    }

    let ctxs_words = header.continuation_ctxs_len * 2;
    let ctxs = section(
        image,
        header.continuation_ctxs,
        ctxs_words,
        "continuation_ctxs",
    );
    let configs_words = header.continuation_configs_len * 6;
    let configs_pool = section(
        image,
        header.continuation_configs,
        configs_words,
        "continuation_configs",
    );
    let action_pool = section(
        image,
        header.continuation_actions,
        header.continuation_actions_len,
        "continuation_actions",
    );
    let pool = section(
        image,
        header.continuations,
        header.continuations_len * 5,
        "continuations",
    );
    let mut continuations = Vec::with_capacity(header.continuations_len);
    for index in 0..header.continuations_len {
        let entry = &pool[index * 5..index * 5 + 5];
        let kind = entry[0];
        assert!(
            kind == CONTINUATION_MOVED_CONFIGS || kind == CONTINUATION_SOURCE_SET,
            "continuation kind"
        );
        let ctxs_off = entry[1] as usize;
        let ctxs_len = entry[2] as usize;
        let configs_off = entry[3] as usize;
        let configs_len = entry[4] as usize;
        assert!(ctxs_off + ctxs_len <= header.continuation_ctxs_len);
        assert!(configs_off + configs_len <= header.continuation_configs_len);
        let mut decoded_ctxs = Vec::with_capacity(ctxs_len);
        for local in 0..ctxs_len {
            let entry = &ctxs[(ctxs_off + local) * 2..(ctxs_off + local) * 2 + 2];
            let (parent, return_state) = (entry[0], entry[1] as i32);
            assert!((parent as usize) <= local, "context table is topological");
            decoded_ctxs.push((parent, return_state));
        }
        let mut decoded_configs = Vec::with_capacity(configs_len);
        for local in 0..configs_len {
            let entry = &configs_pool[(configs_off + local) * 6..(configs_off + local) * 6 + 6];
            let (state, alt, ctx_local, flags) = (entry[0], entry[1], entry[2], entry[3]);
            let actions_off = entry[4] as usize;
            let actions_len = entry[5] as usize;
            assert!(flags & !0b11 == 0, "only defined flag bits are set");
            assert!(alt >= 1, "lexer config alts are 1-based");
            assert!((ctx_local as usize) <= ctxs_len, "config context in range");
            assert!(actions_off + actions_len <= header.continuation_actions_len);
            let actions = action_pool[actions_off..actions_off + actions_len].to_vec();
            decoded_configs.push(DecodedCfg {
                state,
                flags,
                alt,
                ctx_local,
                actions,
            });
        }
        continuations.push(DecodedCont {
            kind,
            ctxs: decoded_ctxs,
            configs: decoded_configs,
        });
    }
    DecodedMode {
        present: true,
        num_states: n,
        start_state: header.start_state,
        accept_type,
        eof,
        low: low_rows,
        class_starts,
        hi_rows: (0..header.num_hi_rows)
            .map(|row| hi_rows[row * header.num_classes..(row + 1) * header.num_classes].to_vec())
            .collect(),
        row_of,
        escape_rows,
        accel,
        eof_escape: eof_escape.to_vec(),
        continuations,
        accept_action_triples,
    }
}

/// Rebuilds the accel descriptor of a payload (the payload words of the
/// image) the way the walker does.
fn decode_accel(payload: &[u32]) -> Accel {
    match payload[0] {
        1 => {
            assert_eq!(&payload[5..8], &[0, 0, 0]);
            Accel::Positive([payload[1], payload[2], payload[3], payload[4]])
        }
        2 => {
            assert_eq!(&payload[2..8], &[0; 6]);
            Accel::Exclude(payload[1])
        }
        3 => {
            assert_eq!(&payload[3..8], &[0; 5]);
            Accel::Exclude2(payload[1], payload[2])
        }
        4 => {
            assert_eq!(&payload[4..8], &[0; 4]);
            Accel::Exclude3(payload[1], payload[2], payload[3])
        }
        5 => Accel::CountedExclusion {
            opener: [payload[1], payload[2], payload[3]],
            closer: [payload[4], payload[5], payload[6]],
            n: payload[7],
        },
        tag => panic!("invalid accel tag {tag}"),
    }
}

impl DecodedMode {
    /// The edge entry of `state` on `symbol`, following the table lookup
    /// rules (EOF, 0..=127 dense, hi classes above).
    fn edge(&self, state: u16, symbol: i32) -> u16 {
        if symbol == EOF {
            return self.eof[state as usize];
        }
        if (0..=127).contains(&symbol) {
            return self.low[state as usize][symbol as usize];
        }
        if self.class_starts.is_empty() || symbol > 0x10FFFF {
            return NO_EDGE;
        }
        let symbol = symbol as u32;
        let class = self
            .class_starts
            .partition_point(|&start| start as u32 <= symbol)
            - 1;
        let row = &self.hi_rows[self.row_of[state as usize] as usize];
        row[class]
    }

    fn escape_range(&self, state: u16, symbol: i32) -> Option<u32> {
        self.escape_rows[state as usize]
            .iter()
            .find(|&&(lo, hi, _)| (lo..=hi).contains(&symbol))
            .map(|&(_, _, cont)| cont)
    }
}

/// Wire-level coverage: the builder's word stream loads as a zero-copy image,
/// and the test-only decoder below reads every section straight from the image
/// words and cross-checks it against the runtime accessors of the committed
/// table.
fn check_invariants(build: &StaticLexerDfaBuildOutput, atn: &ATN) {
    let image: &'static [u32] = Box::leak(build.words.clone().into_boxed_slice());
    let loaded = StaticLexerTables::from_image(image).expect("builder image loads");
    let modes = decode(image);
    assert_eq!(modes.len(), build.modes.len());
    for (index, (mode, info)) in modes.iter().zip(&build.modes).enumerate() {
        assert_eq!(mode.present, info.table.is_some());
        assert_eq!(build.table(index).is_some(), info.table.is_some());
        if !mode.present {
            assert!(info.failure.is_some());
            continue;
        }
        assert!(info.failure.is_none());
        assert_eq!(mode.num_states, info.num_states);
        assert_eq!(
            mode.start_state as usize,
            info.table.as_ref().unwrap().start_state() as usize
        );
        assert!((mode.start_state as usize) < mode.num_states);
        assert_eq!(mode.continuations.len(), info.num_continuations);
        assert_eq!(
            loaded.mode(index),
            build.table(index),
            "the loaded image is the committed table"
        );

        // Every section of the image agrees with the committed table: EOF and
        // the dense low row, plus one probe per hi class (the target is
        // constant within a class by construction).
        let table = loaded.mode(index).expect("present mode");
        for s in 0..mode.num_states {
            assert_eq!(
                table.state_accel(s as u16),
                mode.accel[s].as_ref(),
                "accel of state {s}"
            );
            let mut symbols: Vec<i32> = (0..=127).collect();
            symbols.push(EOF);
            symbols.extend(mode.class_starts.iter().copied());
            for symbol in symbols {
                let entry = mode.edge(s as u16, symbol);
                check_entry(mode, s, symbol, entry);
                // SAFETY: `s` ranges over `0..mode.num_states`.
                assert_eq!(
                    entry,
                    unsafe { table.edge_entry(s as u16, symbol) },
                    "edge entry of the committed table matches the image"
                );
            }
            // SAFETY: `s` ranges over `0..mode.num_states`.
            assert_eq!(mode.accept_type[s], unsafe { table.accept_type(s as u16) });
            assert_eq!(
                table.accept_action_triples(s as u16),
                mode.accept_action_triples[s],
                "accept actions of state {s}"
            );
            let ranges: Vec<(i32, i32, u32)> = mode.escape_rows[s]
                .iter()
                .map(|&(lo, hi, continuation)| (lo, hi, continuation))
                .collect();
            assert_eq!(
                table.escape_ranges(s as u16),
                ranges,
                "escape ranges of state {s}"
            );
        }
        for (state, ranges) in mode.escape_rows.iter().enumerate() {
            for &(lo, hi, continuation) in ranges {
                // SAFETY: `state` ranges over `0..mode.num_states`.
                assert_eq!(
                    unsafe { table.escape_continuation(state as u16, lo) },
                    continuation,
                    "escape continuation at state {state} symbol {lo}"
                );
                assert_eq!(
                    unsafe { table.escape_continuation(state as u16, hi) },
                    continuation
                );
            }
            // SAFETY: `state` ranges over `0..mode.num_states`.
            assert_eq!(
                unsafe { table.escape_continuation(state as u16, EOF) },
                mode.eof_escape[state],
                "the EOF escape index of state {state}"
            );
        }
        for (id, cont) in mode.continuations.iter().enumerate() {
            assert_eq!(table.continuation_kind(id as u32), cont.kind);
            assert_eq!(
                table.continuation_num_configs(id as u32),
                cont.configs.len()
            );
            assert_eq!(table.continuation_num_contexts(id as u32), cont.ctxs.len());
            assert_eq!(
                table.continuation_config_states(id as u32),
                cont.configs
                    .iter()
                    .map(|config| config.state as usize)
                    .collect::<Vec<_>>()
            );
            assert_eq!(
                table.continuation_num_non_greedy_configs(id as u32),
                cont.configs.iter().filter(|c| c.flags & 1 != 0).count()
            );
        }
        for cont in &mode.continuations {
            assert!(
                cont.kind == CONTINUATION_MOVED_CONFIGS || cont.kind == CONTINUATION_SOURCE_SET
            );
            for (i, &(parent, return_state)) in cont.ctxs.iter().enumerate() {
                assert!((parent as usize) < i + 1, "context table is topological");
                assert!((return_state as usize) < atn.states_count());
            }
            for cfg in &cont.configs {
                assert!((cfg.state as usize) < atn.states_count());
                assert!(cfg.alt >= 1, "lexer config alts are 1-based");
                assert!(cfg.flags & !0b11 == 0, "only defined flag bits are set");
                assert!((cfg.ctx_local as usize) <= cont.ctxs.len());
                for &action in &cfg.actions {
                    assert!(
                        (action as usize) < atn.lexer_actions.len(),
                        "continuation action index in range"
                    );
                }
            }
        }
    }
}

fn check_entry(mode: &DecodedMode, state: usize, symbol: i32, entry: u16) {
    if entry == NO_EDGE {
        if symbol != EOF {
            assert_eq!(mode.escape_range(state as u16, symbol), None);
        }
        return;
    }
    if entry == ESCAPE {
        // An escape edge is the `ESCAPE` sentinel itself (its bit pattern also
        // happens to carry `ACCEPT_BIT`; escape edges never mark accepts) and
        // must have an escape range covering the symbol (the EOF escape index
        // covers EOF).
        if symbol != EOF {
            assert!(
                mode.escape_range(state as u16, symbol).is_some(),
                "escape at state {state} symbol {symbol} has an escape range"
            );
        }
        return;
    }
    let target = (entry & STATE_MASK) as usize;
    assert!(target < mode.num_states);
    assert_eq!(
        entry & ACCEPT_BIT != 0,
        mode.accept_type[target] >= 0,
        "accept bit of entry {entry:#06x} matches accept_type of target"
    );
}

// ======= CSV: a plain, fully static grammar =======

#[test]
fn csv_is_fully_static() {
    let (atn, build) = build("CSV.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    assert!(build.modes[0].table.is_some());
    assert_eq!(build.modes[0].escape_counts, Default::default());
    let dfa = build.table(0).expect("mode 0");
    assert_eq!(dfa.num_continuations(), 0);
    assert!(has_no_escapes(dfa));

    // Token types: CSV.g4 is a combined grammar, so the literals of the
    // parser rules take token types 1..=3 (',', '\r', '\n'); the lexer rules
    // follow: WS = 4, TEXT = 5, STRING = 6.
    assert_eq!(accept_types(dfa), vec![1, 2, 3, 4, 5, 6]);

    // The STRING accept: '"' ... '"' — walk a minimal string.
    let s = walk(dfa, dfa.start_state(), "\"a\"");
    // SAFETY: `s` is a state returned by `walk`.
    assert_eq!(unsafe { dfa.accept_type(s) }, 6);

    // The TEXT state loops on hi symbols.
    let text = walk(dfa, dfa.start_state(), "x");
    // SAFETY: `text` is a state returned by `walk`.
    assert_eq!(unsafe { dfa.accept_type(text) }, 5);
    let entry = unsafe { dfa.edge_entry(text, 0x400) }; // a hi symbol: TEXT loops
    assert_ne!(entry, NO_EDGE);
    assert_eq!(entry & STATE_MASK, text);

    // The WS action list is channel(HIDDEN): one (0, 1, 0) triple.
    let ws = walk(dfa, dfa.start_state(), " ");
    // SAFETY: `ws` is a state returned by `walk`.
    assert_eq!(unsafe { dfa.accept_type(ws) }, 4);
    assert_eq!(dfa.accept_action_triples(ws), vec![(0, 1, 0)]);
}

#[test]
fn csv_build_is_deterministic() {
    let (_, a) = build("CSV.g4");
    let (_, b) = build("CSV.g4");
    assert_eq!(a.words, b.words);
    assert!(a
        .modes
        .iter()
        .zip(&b.modes)
        .all(|(x, y)| x.table == y.table));
}

// ======= Spike: predicate and recursion escapes =======

#[test]
fn spike_escape_coordinates() {
    let (atn, build) = build("lexer/Spike.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    let dfa = build.table(0).expect("mode 0");
    let counts = build.modes[0].escape_counts;
    assert!(counts.predicate >= 1, "predicate escape exists");
    assert_eq!(
        counts.recursive_context, 0,
        "nested comment is compiled statically"
    );
    assert_eq!(counts.budget, 0);

    // The predicate escape: 't' after "le" (`KWLET: 'let' {p}?`). The move
    // step of that edge reaches the KWLET accept under the assumed-true
    // predicate, so skip_alt fires and the continuation is source-set.
    let le = walk(dfa, dfa.start_state(), "le");
    // SAFETY: `le` is a state returned by `walk`.
    assert_eq!(unsafe { dfa.edge_entry(le, 't' as i32) }, ESCAPE);
    let cont = unsafe { dfa.escape_continuation(le, 't' as i32) };
    assert_ne!(cont, NO_CONTINUATION, "escape range for 't'");
    assert_eq!(dfa.continuation_kind(cont), CONTINUATION_SOURCE_SET);
    assert!(dfa.continuation_num_configs(cont) > 0);

    let kinds: Vec<u32> = (0..dfa.num_continuations() as u32)
        .map(|i| dfa.continuation_kind(i))
        .collect();
    assert!(kinds.contains(&CONTINUATION_SOURCE_SET));

    // Nested comment bodies stay static and do not escape on nested "/*".
    let body = walk(dfa, dfa.start_state(), "/*");
    let slash = walk(dfa, body, "/");
    // SAFETY: `body` and `slash` are states returned by `walk`.
    assert_ne!(unsafe { dfa.edge_entry(body, 'a' as i32) }, ESCAPE);
    assert_ne!(unsafe { dfa.edge_entry(body, 'a' as i32) }, NO_EDGE);
    assert_ne!(unsafe { dfa.edge_entry(slash, '*' as i32) }, ESCAPE);
    assert_ne!(unsafe { dfa.edge_entry(slash, '*' as i32) }, NO_EDGE);
}

#[test]
fn non_counted_recursion_escapes() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../dbt-antlr-codegen/tests/codegen-direct/fixtures/lexer-recursion/LexerRecursion.g4",
    );
    let units =
        dbt_antlr_codegen::atn_export::compile_atn_data(&path, &[]).expect("grammar compiles");
    let words = units
        .iter()
        .find_map(|u| u.lexer_atn_words.clone())
        .expect("lexer unit");
    let atn = ATNDeserializer::new(None).deserialize(&mut words.iter());
    let build = StaticLexerDfaBuilder::new().build(&atn);
    check_invariants(&build, &atn);
    let dfa = build.table(0).expect("mode 0");
    let counts = build.modes[0].escape_counts;
    assert_eq!(counts.recursive_context, 1, "recursion escapes at entry");
    assert_eq!(dfa.num_states(), 2);
    // SAFETY: `start_state` is a state of the table.
    assert_eq!(
        unsafe { dfa.edge_entry(dfa.start_state(), '{' as i32) },
        ESCAPE
    );
    let kinds: Vec<u32> = (0..dfa.num_continuations() as u32)
        .map(|i| dfa.continuation_kind(i))
        .collect();
    assert!(kinds.contains(&CONTINUATION_MOVED_CONFIGS));
}

#[test]
fn spike_build_is_deterministic() {
    let (_, a) = build("lexer/Spike.g4");
    let (_, b) = build("lexer/Spike.g4");
    assert_eq!(a.words, b.words);
    assert!(a
        .modes
        .iter()
        .zip(&b.modes)
        .all(|(x, y)| x.table == y.table));
}

// ======= MiniModes: modes and hi classes =======

#[test]
fn modes_grammar_builds_all_modes() {
    let (atn, build) = build("lexer/MiniModes.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 2);
    assert!(build.modes.iter().all(|m| m.table.is_some()));

    // Mode 0 (DEFAULT_MODE): TEXT is ~[<]+, so hi classes exist and cover
    // everything above 127.
    let default_mode = build.table(0).expect("mode 0");
    assert!(default_mode.num_classes() > 0);
    let text = walk(default_mode, default_mode.start_state(), "x");
    // SAFETY: `text` is a state returned by `walk`.
    let entry = unsafe { default_mode.edge_entry(text, 0x10FFFF) };
    assert_ne!(entry, NO_EDGE);
    assert_eq!(entry & STATE_MASK, text);

    // The '<' accept of mode 0 pushes mode 1: pushMode(INSIDE) is action
    // triple (5, 1, 0).
    let lt = walk(default_mode, default_mode.start_state(), "<");
    assert_eq!(default_mode.accept_action_triples(lt), vec![(5, 1, 0)]);

    // Mode 1 (INSIDE): NAME accept, and '>' pops. popMode is (4, 0, 0).
    let inside = build.table(1).expect("mode 1");
    let name = walk(inside, inside.start_state(), "abc");
    assert!(name != inside.start_state());
    let gt = walk(inside, inside.start_state(), ">");
    assert_eq!(inside.accept_action_triples(gt), vec![(4, 0, 0)]);
}

// ======= CustomAction: position-dependent action escape =======

#[test]
fn custom_action_escapes() {
    let (atn, build) = build("lexer/CustomAction.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    let dfa = build.table(0).expect("mode 0");
    let counts = build.modes[0].escape_counts;
    assert!(counts.custom_action >= 1, "custom action escape exists");
    assert_eq!(counts.predicate, 0);
    assert_eq!(counts.budget, 0);

    // The closure of the edge on 't' after "cus" crosses the custom action
    // of `CUST: 'cust' {..} 'x'`. The same closure also reaches the ID accept,
    // so skip_alt fires and the continuation is source-set.
    let cus = walk(dfa, dfa.start_state(), "cus");
    // SAFETY: `cus` is a state returned by `walk`.
    assert_eq!(unsafe { dfa.edge_entry(cus, 't' as i32) }, ESCAPE);
    let cont = unsafe { dfa.escape_continuation(cus, 't' as i32) };
    assert_ne!(cont, NO_CONTINUATION, "escape range for 't'");
    assert_eq!(dfa.continuation_kind(cont), CONTINUATION_SOURCE_SET);
}

// ======= Budget: forced per-edge budget escapes =======

#[test]
fn budget_overflow_escapes_without_continuation() {
    let (atn, build) = build_with(
        "CSV.g4",
        &StaticLexerDfaBuilder::new().with_max_states_per_mode(1),
    );
    check_invariants(&build, &atn);
    assert!(build.modes[0].table.is_some());
    let dfa = build.table(0).expect("mode 0");
    assert_eq!(dfa.num_states(), 1);
    assert!(build.modes[0].escape_counts.budget > 0);
    // Budget escapes carry no continuation: the pool stays empty.
    assert_eq!(dfa.num_continuations(), 0);
    assert_eq!(build.modes[0].num_continuations, 0);
    // Every escape range of the start state has the NO_CONTINUATION index.
    assert!(!dfa.escape_ranges(0).is_empty());
    for (_, _, cont) in dfa.escape_ranges(0) {
        assert_eq!(cont, NO_CONTINUATION);
    }
}

#[test]
fn zero_budget_fails_the_mode() {
    let (atn, build) = build_with(
        "CSV.g4",
        &StaticLexerDfaBuilder::new().with_max_states_per_mode(0),
    );
    check_invariants(&build, &atn);
    assert!(build.modes[0].table.is_none());
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartStateBudgetExceeded)
    );
}

// ======= EofLexer: explicit EOF edges =======

#[test]
fn eof_edges_are_compiled() {
    let (atn, build) = build("lexer/EofLexer.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    let dfa = build.table(0).expect("mode 0");
    assert_eq!(build.modes[0].escape_counts, Default::default());

    // After "ab", the EOF edge reaches the AB accept: AB = 1.
    let ab = walk(dfa, dfa.start_state(), "ab");
    // SAFETY: `ab` is a state returned by `walk`.
    let entry = unsafe { dfa.edge_entry(ab, EOF) };
    assert_ne!(entry, NO_EDGE);
    assert_ne!(entry, ESCAPE);
    assert_eq!(entry & ACCEPT_BIT, ACCEPT_BIT);
    // SAFETY: `entry` is an edge target masked to a state number.
    assert_eq!(unsafe { dfa.accept_type(entry & STATE_MASK) }, 1);

    // EOF from the start state and after "a" alone is no edge: 'a' matches
    // the A rule and stops there.
    // SAFETY: `start_state` and the walked state are states of the table.
    assert_eq!(unsafe { dfa.edge_entry(dfa.start_state(), EOF) }, NO_EDGE);
    assert_eq!(
        unsafe { dfa.edge_entry(walk(dfa, dfa.start_state(), "a"), EOF) },
        NO_EDGE
    );
}

// ======= XMLLexer: modes, more, predicate and custom-action escapes =======

#[test]
fn xml_lexer_builds_with_escapes() {
    let (atn, build) = build("XMLLexer.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 3, "DEFAULT_MODE, INSIDE, PROC_INSTR");
    assert!(build.modes.iter().all(|m| m.table.is_some()));
    let counts: Vec<_> = build.modes.iter().map(|m| m.escape_counts).collect();
    assert!(counts.iter().any(|c| c.predicate > 0), "COMMENT predicate");
    assert!(
        counts.iter().any(|c| c.custom_action > 0),
        "CLOSE custom action"
    );
    assert!(counts.iter().all(|&c| c.budget == 0));

    // NameStartChar has wide ranges above 127: hi classes exist in INSIDE.
    assert!(
        (0..build.modes.len()).any(|m| { build.table(m).expect("present mode").num_classes() > 0 })
    );

    // `more` is a position-independent action and compiles statically:
    // SPECIAL_OPEN ends in `more, pushMode(PROC_INSTR)`, triples (3, 0, 0)
    // and (5, 2, 0).
    let default_mode = build.table(0).expect("mode 0");
    let special = walk(default_mode, default_mode.start_state(), "<?x");
    assert_eq!(
        default_mode.accept_action_triples(special),
        vec![(3, 0, 0), (5, 2, 0)]
    );
}

// ======= Start-closure failures: one ModeFailure variant each =======

#[test]
fn start_closure_predicate_fails_the_mode() {
    let (atn, build) = build("lexer/StartPred.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    assert!(build.modes[0].table.is_none());
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartClosurePredicate)
    );
}

#[test]
fn start_closure_custom_action_fails_the_mode() {
    let (atn, build) = build("lexer/StartAction.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    assert!(build.modes[0].table.is_none());
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartClosureCustomAction)
    );
}

#[test]
fn start_closure_recursive_context_fails_the_mode() {
    let (atn, build) = build("lexer/StartRecurse.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    assert!(build.modes[0].table.is_none());
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartClosureRecursiveContext)
    );
}

// ======= EofEscape: a predicate-guarded EOF edge escapes =======

#[test]
fn eof_edge_escapes_on_guarded_eof() {
    let (atn, build) = build("lexer/EofEscape.g4");
    check_invariants(&build, &atn);
    assert_eq!(build.modes.len(), 1);
    let dfa = build.table(0).expect("mode 0");
    let counts = build.modes[0].escape_counts;
    assert_eq!(counts.predicate, 1);
    assert_eq!(counts.custom_action, 0);
    assert_eq!(counts.recursive_context, 0);
    assert_eq!(counts.budget, 0);

    // The 'a' edge is static: the predicate sits behind the EOF match.
    let s = walk(dfa, dfa.start_state(), "a");
    // SAFETY: `s` is a state returned by `walk`.
    assert_eq!(unsafe { dfa.edge_entry(s, EOF) }, ESCAPE);

    // The moved EOF config reaches the A accept under the assumed-true
    // predicate, so skip_alt fires and the continuation is source-set.
    // SAFETY: `s` is a state returned by `walk`.
    let cont = unsafe { dfa.escape_continuation(s, EOF) };
    assert_ne!(cont, NO_CONTINUATION);
    assert_eq!(dfa.continuation_kind(cont), CONTINUATION_SOURCE_SET);
    assert!(dfa.continuation_num_configs(cont) > 0);

    // The start state has no EOF edge at all.
    // SAFETY: `start_state` is a state of the table.
    assert_eq!(unsafe { dfa.edge_entry(dfa.start_state(), EOF) }, NO_EDGE);
    assert_eq!(
        unsafe { dfa.escape_continuation(dfa.start_state(), EOF) },
        NO_CONTINUATION
    );
}

// ======= Dump helper (manual inspection) =======

#[test]
#[ignore]
fn dump_spike() {
    let (atn, build) = build("lexer/Spike.g4");
    check_invariants(&build, &atn);
    let dfa = build.table(0).expect("mode 0");
    eprintln!("states: {}", dfa.num_states());
    // SAFETY (below): `s` ranges over the states of `dfa`.
    for s in 0..dfa.num_states() as u16 {
        let mut edges = Vec::new();
        for sym in 0..=127i32 {
            let entry = unsafe { dfa.edge_entry(s, sym) };
            if entry == NO_EDGE {
                continue;
            }
            let c = char::from_u32(sym as u32).unwrap();
            if entry == ESCAPE {
                let cont = unsafe { dfa.escape_continuation(s, sym) };
                edges.push(format!("{c:?}->ESC(c{cont:?})"));
            } else {
                edges.push(format!(
                    "{c:?}->{}{}",
                    entry & STATE_MASK,
                    if entry & ACCEPT_BIT != 0 { "[acc]" } else { "" }
                ));
            }
        }
        eprintln!(
            "  s{s} accept={} actions={:?} edges: {}",
            unsafe { dfa.accept_type(s) },
            dfa.accept_action_triples(s),
            edges.join(" ")
        );
    }
    eprintln!("continuations: {}", dfa.num_continuations());
    for i in 0..dfa.num_continuations() as u32 {
        eprintln!(
            "  c{i} kind={} configs={}",
            dfa.continuation_kind(i),
            dfa.continuation_num_configs(i)
        );
    }
    eprintln!("escape counts: {:?}", build.modes[0].escape_counts);
}
