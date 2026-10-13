//! Integration tests for the compile-time static lexer DFA builder
//! (`static-dfa-build` feature). Grammars live in `tests/static_lexer/` and
//! are compiled to lexer ATN words through `dbt-antlr-codegen`'s `atn-export`.
#![cfg(feature = "static-dfa-build")]

use std::path::PathBuf;

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_deserializer::ATNDeserializer;
use dbt_antlr_runtime::int_stream::EOF;
use dbt_antlr_runtime::static_dfa_build::{
    ModeFailure, StaticLexerDfaBuild, StaticLexerDfaBuilder, ACCEPT_BIT,
    BEHIND_POSITION_INDEPENDENT, CONTINUATION_MOVED_CONFIGS, CONTINUATION_SOURCE_SET, ESCAPE,
    NO_CONTINUATION, NO_EDGE, STATE_MASK, STATIC_DFA_FORMAT_VERSION, STATIC_DFA_MAGIC,
};

fn grammar_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/static_lexer")
        .join(name)
}

fn build_with(grammar: &str, builder: &StaticLexerDfaBuilder) -> (ATN, StaticLexerDfaBuild) {
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

fn build(grammar: &str) -> (ATN, StaticLexerDfaBuild) {
    build_with(grammar, &StaticLexerDfaBuilder::new())
}

// ======= Test-only decoder for the version-1 word stream =======

#[derive(Debug, Default)]
struct DecodedMode {
    present: bool,
    num_states: usize,
    start_state: u32,
    accept_type: Vec<i32>,
    accept_actions: Vec<i32>,
    eof: Vec<u16>,
    low: Vec<[u16; 128]>,
    class_starts: Vec<i32>,
    hi_rows: Vec<Vec<(u32, u32, u16)>>,
    row_of: Vec<u32>,
    action_lists: Vec<u32>,
    accel: Vec<u32>,
    escape_eof: Vec<u32>,
    escape_rows: Vec<Vec<(i32, i32, u32)>>,
    continuations: Vec<DecodedCont>,
}

#[derive(Debug)]
struct DecodedCont {
    kind: u32,
    ctxs: Vec<(u32, u32, i32)>,
    configs: Vec<DecodedCfg>,
}

#[derive(Debug)]
struct DecodedCfg {
    state: u32,
    flags: u32,
    alt: u32,
    ctx_local: u32,
    actions: Vec<(u32, u32)>,
}

struct Reader<'a> {
    words: &'a [u32],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn next(&mut self) -> u32 {
        let w = self.words[self.pos];
        self.pos += 1;
        w
    }

    fn next_i32(&mut self) -> i32 {
        self.next() as i32
    }

    fn take(&mut self, n: usize) -> &'a [u32] {
        let s = &self.words[self.pos..self.pos + n];
        self.pos += n;
        s
    }
}

fn decode(words: &[u32]) -> Vec<DecodedMode> {
    let mut r = Reader { words, pos: 0 };
    assert_eq!(r.next(), STATIC_DFA_MAGIC);
    assert_eq!(r.next(), STATIC_DFA_FORMAT_VERSION);
    assert_eq!(r.next() as usize, words.len());
    let num_modes = r.next() as usize;
    let modes: Vec<_> = (0..num_modes).map(|_| decode_mode(&mut r)).collect();
    assert_eq!(r.pos, words.len(), "all words consumed");
    modes
}

fn decode_mode(r: &mut Reader) -> DecodedMode {
    let mut mode = DecodedMode {
        present: r.next() != 0,
        ..Default::default()
    };
    if !mode.present {
        return mode;
    }
    mode.num_states = r.next() as usize;
    mode.start_state = r.next();
    let n = mode.num_states;
    mode.accept_type = (0..n).map(|_| r.next_i32()).collect();
    mode.accept_actions = (0..n).map(|_| r.next_i32()).collect();
    mode.eof = (0..n).map(|_| r.next() as u16).collect();
    mode.low = (0..n)
        .map(|_| {
            let mut row = [0u16; 128];
            for c in &mut row {
                *c = r.next() as u16;
            }
            row
        })
        .collect();
    let num_classes = r.next() as usize;
    if num_classes > 0 {
        mode.class_starts = r.take(num_classes).iter().map(|&w| w as i32).collect();
        let num_rows = r.next() as usize;
        let num_triples = r.next() as usize;
        let row_offsets: Vec<usize> = (0..=num_rows).map(|_| r.next() as usize).collect();
        assert_eq!(row_offsets[0], 0);
        assert_eq!(row_offsets[num_rows], num_triples);
        for w in row_offsets.windows(2) {
            assert!(w[0] <= w[1]);
        }
        for row in 0..num_rows {
            let mut runs = Vec::new();
            for _ in row_offsets[row]..row_offsets[row + 1] {
                runs.push((r.next(), r.next(), r.next() as u16));
            }
            mode.hi_rows.push(runs);
        }
        mode.row_of = (0..n).map(|_| r.next()).collect();
    }
    let num_action_ints = r.next() as usize;
    mode.action_lists = r.take(num_action_ints).to_vec();
    let num_accel_ints = r.next() as usize;
    mode.accel = r.take(num_accel_ints).to_vec();
    for _ in 0..n {
        mode.escape_eof.push(r.next());
        let num_ranges = r.next() as usize;
        let mut ranges = Vec::new();
        for _ in 0..num_ranges {
            ranges.push((r.next_i32(), r.next_i32(), r.next()));
        }
        mode.escape_rows.push(ranges);
    }
    let num_conts = r.next() as usize;
    for _ in 0..num_conts {
        let kind = r.next();
        let nctx = r.next() as usize;
        let ctxs = (0..nctx)
            .map(|_| (r.next(), r.next(), r.next_i32()))
            .collect();
        let nconfigs = r.next() as usize;
        let mut configs = Vec::new();
        for _ in 0..nconfigs {
            let (state, flags, alt, ctx_local) = (r.next(), r.next(), r.next(), r.next());
            let nactions = r.next() as usize;
            let actions = (0..nactions).map(|_| (r.next(), r.next())).collect();
            configs.push(DecodedCfg {
                state,
                flags,
                alt,
                ctx_local,
                actions,
            });
        }
        mode.continuations.push(DecodedCont {
            kind,
            ctxs,
            configs,
        });
    }
    mode
}

impl DecodedMode {
    /// The edge entry of `state` on `symbol`, following the table lookup
    /// rules (column 0 = EOF, 1..=128 dense, hi classes above).
    fn edge(&self, state: u16, symbol: i32) -> u16 {
        if symbol == EOF {
            return self.eof[state as usize];
        }
        if (0..=127).contains(&symbol) {
            return self.low[state as usize][symbol as usize];
        }
        if self.class_starts.is_empty() {
            return NO_EDGE;
        }
        let class = self
            .class_starts
            .partition_point(|&s| s <= symbol)
            .checked_sub(1)
            .expect("symbol above 127 has a class") as u32;
        let row = &self.hi_rows[self.row_of[state as usize] as usize];
        let mut entry = NO_EDGE;
        for &(lo, hi, target) in row {
            if (lo..=hi).contains(&class) {
                entry = target;
            }
        }
        entry
    }

    /// Follows `text` from `from`; every edge must be a plain edge.
    fn walk(&self, from: u16, text: &str) -> u16 {
        let mut s = from;
        for c in text.chars() {
            let entry = self.edge(s, c as i32);
            assert!(
                entry != NO_EDGE && entry != ESCAPE,
                "walk: edge on {c:?} from state {s} must be a plain edge, got {entry:#06x}"
            );
            s = entry & STATE_MASK;
        }
        s
    }

    fn accept_types(&self) -> Vec<i32> {
        let mut types: Vec<i32> = self
            .accept_type
            .iter()
            .copied()
            .filter(|&t| t >= 0)
            .collect();
        types.sort_unstable();
        types.dedup();
        types
    }

    fn escape_range(&self, state: u16, symbol: i32) -> Option<u32> {
        self.escape_rows[state as usize]
            .iter()
            .find(|&&(lo, hi, _)| (lo..=hi).contains(&symbol))
            .map(|&(_, _, cont)| cont)
    }
}

/// Structural invariants that must hold for every build.
fn check_invariants(build: &StaticLexerDfaBuild, atn: &ATN) -> Vec<DecodedMode> {
    let modes = decode(&build.words);
    assert_eq!(modes.len(), build.modes.len());
    for (mode, info) in modes.iter().zip(&build.modes) {
        assert_eq!(mode.present, info.present);
        if !mode.present {
            assert!(info.failure.is_some());
            continue;
        }
        assert!(info.failure.is_none());
        assert_eq!(mode.num_states, info.num_states);
        assert!((mode.start_state as usize) < mode.num_states);
        assert_eq!(mode.continuations.len(), info.num_continuations);

        for (s, row) in mode.low.iter().enumerate() {
            for (sym, &entry) in row.iter().enumerate() {
                check_entry(mode, s, sym as i32, entry);
            }
            check_entry(mode, s, EOF, mode.eof[s]);
        }
        for (s, &row_idx) in mode.row_of.iter().enumerate() {
            for &(lo_class, _, entry) in &mode.hi_rows[row_idx as usize] {
                check_entry(mode, s, mode.class_starts[lo_class as usize], entry);
            }
        }
        for cont in &mode.continuations {
            assert!(
                cont.kind == CONTINUATION_MOVED_CONFIGS || cont.kind == CONTINUATION_SOURCE_SET
            );
            for (i, &(kind, parent, return_state)) in cont.ctxs.iter().enumerate() {
                assert_eq!(kind, 0, "lexer contexts are singleton chains");
                assert!((parent as usize) < i + 1, "context table is topological");
                assert!((return_state as usize) < atn.states_count());
            }
            for cfg in &cont.configs {
                assert!((cfg.state as usize) < atn.states_count());
                assert!(cfg.alt >= 1, "lexer config alts are 1-based");
                assert!(cfg.flags & !0b11 == 0, "only defined flag bits are set");
                assert!((cfg.ctx_local as usize) <= cont.ctxs.len());
                for &(_, behind) in &cfg.actions {
                    assert_eq!(behind, BEHIND_POSITION_INDEPENDENT);
                }
            }
        }
    }
    modes
}

fn check_entry(mode: &DecodedMode, state: usize, symbol: i32, entry: u16) {
    if entry == NO_EDGE {
        if symbol != EOF {
            assert_eq!(mode.escape_range(state as u16, symbol), None);
        }
        return;
    }
    if entry == ESCAPE {
        // Escape edges never carry the accept bit and must have an escape
        // range covering the symbol (the EOF escape index covers EOF).
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
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    let mode = &modes[0];
    assert!(mode.present);
    assert_eq!(build.modes[0].escape_counts, Default::default());
    assert!(mode.continuations.is_empty());
    assert!(mode.escape_rows.iter().all(|r| r.is_empty()));
    assert!(mode.eof.iter().all(|&e| e == NO_EDGE));
    assert!(mode.low.iter().flatten().all(|&e| e != ESCAPE));

    // Token types: CSV.g4 is a combined grammar, so the literals of the
    // parser rules take token types 1..=3 (',', '\r', '\n'); the lexer rules
    // follow: WS = 4, TEXT = 5, STRING = 6.
    assert_eq!(mode.accept_types(), vec![1, 2, 3, 4, 5, 6]);

    // The STRING accept: '"' ... '"' — walk a minimal string.
    let s = mode.walk(mode.start_state as u16, "\"a\"");
    assert_eq!(mode.accept_type[s as usize], 6);

    // The TEXT state loops on hi symbols.
    let text = mode.walk(mode.start_state as u16, "x");
    assert_eq!(mode.accept_type[text as usize], 5);
    let entry = mode.edge(text, 0x400); // a hi symbol: TEXT loops
    assert_ne!(entry, NO_EDGE);
    assert_eq!(entry & STATE_MASK, text);

    // The WS action list is channel(HIDDEN): one (0, 1, 0) triple.
    let ws = mode.walk(mode.start_state as u16, " ");
    assert_eq!(mode.accept_type[ws as usize], 4);
    let offset = mode.accept_actions[ws as usize];
    assert!(offset >= 0);
    let lists = &mode.action_lists[offset as usize..];
    assert_eq!(lists[0], 1, "one action");
    assert_eq!(&lists[1..4], &[0, 1, 0], "channel(HIDDEN)");
}

#[test]
fn csv_build_is_deterministic() {
    let (_, a) = build("CSV.g4");
    let (_, b) = build("CSV.g4");
    assert_eq!(a.words, b.words);
}

// ======= Spike: predicate and recursion escapes =======

#[test]
fn spike_escape_coordinates() {
    let (atn, build) = build("Spike.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    let mode = &modes[0];
    assert!(mode.present);
    let counts = build.modes[0].escape_counts;
    assert!(counts.predicate >= 1, "predicate escape exists");
    assert!(counts.recursive_context >= 1, "recursion escape exists");
    assert_eq!(counts.budget, 0);

    // The predicate escape: 't' after "le" (`KWLET: 'let' {p}?`). The move
    // step of that edge reaches the KWLET accept under the assumed-true
    // predicate, so skip_alt fires and the continuation is source-set.
    let le = mode.walk(mode.start_state as u16, "le");
    assert_eq!(mode.edge(le, 't' as i32), ESCAPE);
    let cont = mode
        .escape_range(le, 't' as i32)
        .expect("escape range for 't'");
    assert_eq!(
        mode.continuations[cont as usize].kind,
        CONTINUATION_SOURCE_SET
    );
    assert!(!mode.continuations[cont as usize].configs.is_empty());

    // The recursion escape: '*' of a nested `/*`. Walk "/*", then repeat
    // "/*" until the '*' edge escapes; it must escape within a few levels.
    let mut s = mode.walk(mode.start_state as u16, "/*");
    let mut escaped = None;
    for _ in 0..4 {
        let slash = mode.walk(s, "/");
        let entry = mode.edge(slash, '*' as i32);
        if entry == ESCAPE {
            escaped = Some(slash);
            break;
        }
        assert_ne!(entry, NO_EDGE);
        s = entry & STATE_MASK;
    }
    let slash = escaped.expect("nested comment '*' escapes");
    let cont = mode
        .escape_range(slash, '*' as i32)
        .expect("escape range for nested '*'");
    let cont = &mode.continuations[cont as usize];
    assert_eq!(cont.kind, CONTINUATION_MOVED_CONFIGS);
    assert!(!cont.configs.is_empty());

    // Both continuation kinds exist in the pool.
    assert!(mode
        .continuations
        .iter()
        .any(|c| c.kind == CONTINUATION_MOVED_CONFIGS));
    assert!(mode
        .continuations
        .iter()
        .any(|c| c.kind == CONTINUATION_SOURCE_SET));

    // Non-nested comment bodies stay static: after "/*", '/' and the loop
    // characters are plain edges.
    let body = mode.walk(mode.start_state as u16, "/*");
    let slash = mode.walk(body, "/");
    assert_ne!(mode.edge(body, 'a' as i32), ESCAPE);
    assert_ne!(mode.edge(body, 'a' as i32), NO_EDGE);
    let _ = slash;
}

#[test]
fn spike_build_is_deterministic() {
    let (_, a) = build("Spike.g4");
    let (_, b) = build("Spike.g4");
    assert_eq!(a.words, b.words);
}

// ======= MiniModes: modes and hi classes =======

#[test]
fn modes_grammar_builds_all_modes() {
    let (atn, build) = build("MiniModes.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 2);
    assert!(modes.iter().all(|m| m.present));

    // Mode 0 (DEFAULT_MODE): TEXT is ~[<]+, so hi classes exist and cover
    // everything above 127.
    let default_mode = &modes[0];
    assert!(!default_mode.class_starts.is_empty());
    let text = default_mode.walk(default_mode.start_state as u16, "x");
    let entry = default_mode.edge(text, 0x10FFFF);
    assert_ne!(entry, NO_EDGE);
    assert_eq!(entry & STATE_MASK, text);

    // The '<' accept of mode 0 pushes mode 1: pushMode(INSIDE) is action
    // triple (5, 1, 0).
    let lt = default_mode.walk(default_mode.start_state as u16, "<");
    let offset = default_mode.accept_actions[lt as usize];
    assert!(offset >= 0);
    let lists = &default_mode.action_lists[offset as usize..];
    assert_eq!(lists[0], 1, "one action");
    assert_eq!(&lists[1..4], &[5, 1, 0], "pushMode(INSIDE)");

    // Mode 1 (INSIDE): NAME accept, and '>' pops. popMode is (4, 0, 0).
    let inside = &modes[1];
    let name = inside.walk(inside.start_state as u16, "abc");
    assert!(name != inside.start_state as u16);
    let gt = inside.walk(inside.start_state as u16, ">");
    let offset = inside.accept_actions[gt as usize];
    assert!(offset >= 0);
    let lists = &inside.action_lists[offset as usize..];
    assert_eq!(&lists[1..4], &[4, 0, 0], "popMode");
}

// ======= CustomAction: position-dependent action escape =======

#[test]
fn custom_action_escapes() {
    let (atn, build) = build("CustomAction.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    let mode = &modes[0];
    assert!(mode.present);
    let counts = build.modes[0].escape_counts;
    assert!(counts.custom_action >= 1, "custom action escape exists");
    assert_eq!(counts.predicate, 0);
    assert_eq!(counts.budget, 0);

    // The closure of the edge on 't' after "cus" crosses the custom action
    // of `CUST: 'cust' {..} 'x'`. The same closure also reaches the ID accept,
    // so skip_alt fires and the continuation is source-set.
    let cus = mode.walk(mode.start_state as u16, "cus");
    assert_eq!(mode.edge(cus, 't' as i32), ESCAPE);
    let cont = mode
        .escape_range(cus, 't' as i32)
        .expect("escape range for 't'");
    assert_eq!(
        mode.continuations[cont as usize].kind,
        CONTINUATION_SOURCE_SET
    );
}

// ======= Budget: forced per-edge budget escapes =======

#[test]
fn budget_overflow_escapes_without_continuation() {
    let (atn, build) = build_with(
        "CSV.g4",
        &StaticLexerDfaBuilder::new().with_max_states_per_mode(1),
    );
    let modes = check_invariants(&build, &atn);
    let mode = &modes[0];
    assert!(mode.present);
    assert_eq!(mode.num_states, 1);
    assert!(build.modes[0].escape_counts.budget > 0);
    // Budget escapes carry no continuation: the pool stays empty.
    assert!(mode.continuations.is_empty());
    assert_eq!(build.modes[0].num_continuations, 0);
    // Every escape range of the start state has the NO_CONTINUATION index.
    assert!(!mode.escape_rows[0].is_empty());
    for &(_, _, cont) in &mode.escape_rows[0] {
        assert_eq!(cont, NO_CONTINUATION);
    }
}

#[test]
fn zero_budget_fails_the_mode() {
    let (atn, build) = build_with(
        "CSV.g4",
        &StaticLexerDfaBuilder::new().with_max_states_per_mode(0),
    );
    let modes = check_invariants(&build, &atn);
    assert!(!modes[0].present);
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartStateBudgetExceeded)
    );
}

// ======= EofLexer: explicit EOF edges =======

#[test]
fn eof_edges_are_compiled() {
    let (atn, build) = build("EofLexer.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    let mode = &modes[0];
    assert!(mode.present);
    assert_eq!(build.modes[0].escape_counts, Default::default());

    // After "ab", the EOF edge reaches the AB accept: AB = 1.
    let ab = mode.walk(mode.start_state as u16, "ab");
    let entry = mode.edge(ab, EOF);
    assert_ne!(entry, NO_EDGE);
    assert_ne!(entry, ESCAPE);
    assert_eq!(entry & ACCEPT_BIT, ACCEPT_BIT);
    assert_eq!(mode.accept_type[(entry & STATE_MASK) as usize], 1);

    // EOF from the start state and after "a" alone is no edge: 'a' matches
    // the A rule and stops there.
    assert_eq!(mode.edge(mode.start_state as u16, EOF), NO_EDGE);
    assert_eq!(
        mode.edge(mode.walk(mode.start_state as u16, "a"), EOF),
        NO_EDGE
    );
}

// ======= XMLLexer: modes, more, predicate and custom-action escapes =======

#[test]
fn xml_lexer_builds_with_escapes() {
    let (atn, build) = build("XMLLexer.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 3, "DEFAULT_MODE, INSIDE, PROC_INSTR");
    assert!(modes.iter().all(|m| m.present));
    let counts: Vec<_> = build.modes.iter().map(|m| m.escape_counts).collect();
    assert!(counts.iter().any(|c| c.predicate > 0), "COMMENT predicate");
    assert!(
        counts.iter().any(|c| c.custom_action > 0),
        "CLOSE custom action"
    );
    assert!(counts.iter().all(|&c| c.budget == 0));

    // NameStartChar has wide ranges above 127: hi classes exist in INSIDE.
    assert!(modes.iter().any(|m| !m.class_starts.is_empty()));

    // `more` is a position-independent action and compiles statically:
    // SPECIAL_OPEN ends in `more, pushMode(PROC_INSTR)`, triples (3, 0, 0)
    // and (5, 2, 0).
    let default_mode = &modes[0];
    let special = default_mode.walk(default_mode.start_state as u16, "<?x");
    let offset = default_mode.accept_actions[special as usize];
    assert!(offset >= 0);
    let lists = &default_mode.action_lists[offset as usize..];
    assert_eq!(lists[0], 2, "two actions");
    assert_eq!(&lists[1..4], &[3, 0, 0], "more");
    assert_eq!(&lists[4..7], &[5, 2, 0], "pushMode(PROC_INSTR)");

    assert!(modes.iter().all(|m| m.present));
}

// ======= Start-closure failures: one ModeFailure variant each =======

#[test]
fn start_closure_predicate_fails_the_mode() {
    let (atn, build) = build("StartPred.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    assert!(!modes[0].present);
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartClosurePredicate)
    );
}

#[test]
fn start_closure_custom_action_fails_the_mode() {
    let (atn, build) = build("StartAction.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    assert!(!modes[0].present);
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartClosureCustomAction)
    );
}

#[test]
fn start_closure_recursive_context_fails_the_mode() {
    let (atn, build) = build("StartRecurse.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    assert!(!modes[0].present);
    assert_eq!(
        build.modes[0].failure,
        Some(ModeFailure::StartClosureRecursiveContext)
    );
}

// ======= EofEscape: a predicate-guarded EOF edge escapes =======

#[test]
fn eof_edge_escapes_on_guarded_eof() {
    let (atn, build) = build("EofEscape.g4");
    let modes = check_invariants(&build, &atn);
    assert_eq!(modes.len(), 1);
    let mode = &modes[0];
    assert!(mode.present);
    let counts = build.modes[0].escape_counts;
    assert_eq!(counts.predicate, 1);
    assert_eq!(counts.custom_action, 0);
    assert_eq!(counts.recursive_context, 0);
    assert_eq!(counts.budget, 0);

    // The 'a' edge is static: the predicate sits behind the EOF match.
    let s = mode.walk(mode.start_state as u16, "a");
    assert_eq!(mode.edge(s, EOF), ESCAPE);

    // The moved EOF config reaches the A accept under the assumed-true
    // predicate, so skip_alt fires and the continuation is source-set.
    let cont = mode.escape_eof[s as usize];
    assert_ne!(cont, NO_CONTINUATION);
    let cont = &mode.continuations[cont as usize];
    assert_eq!(cont.kind, CONTINUATION_SOURCE_SET);
    assert!(!cont.configs.is_empty());

    // The start state has no EOF edge at all.
    assert_eq!(mode.edge(mode.start_state as u16, EOF), NO_EDGE);
    assert_eq!(mode.escape_eof[mode.start_state as usize], NO_CONTINUATION);
}

// ======= Dump helper (manual inspection) =======

#[test]
#[ignore]
fn dump_spike() {
    let (atn, build) = build("Spike.g4");
    let modes = check_invariants(&build, &atn);
    let mode = &modes[0];
    eprintln!("states: {}", mode.num_states);
    for s in 0..mode.num_states as u16 {
        let mut edges = Vec::new();
        for sym in 0..=127i32 {
            let entry = mode.edge(s, sym);
            if entry == NO_EDGE {
                continue;
            }
            let c = char::from_u32(sym as u32).unwrap();
            if entry == ESCAPE {
                edges.push(format!("{c:?}->ESC(c{:?})", mode.escape_range(s, sym)));
            } else {
                edges.push(format!(
                    "{c:?}->{}{}",
                    entry & STATE_MASK,
                    if entry & ACCEPT_BIT != 0 { "[acc]" } else { "" }
                ));
            }
        }
        eprintln!(
            "  s{s} accept={} actions_off={} edges: {}",
            mode.accept_type[s as usize],
            mode.accept_actions[s as usize],
            edges.join(" ")
        );
    }
    eprintln!("continuations: {}", mode.continuations.len());
    for (i, c) in mode.continuations.iter().enumerate() {
        eprintln!(
            "  c{i} kind={} ctxs={:?} configs={}",
            c.kind,
            c.ctxs,
            c.configs.len()
        );
    }
    eprintln!("escape counts: {:?}", build.modes[0].escape_counts);
}
