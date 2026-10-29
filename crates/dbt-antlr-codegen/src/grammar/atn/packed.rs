// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 Konstantin Vyatkin
//! Writer of the packed parser ATN word format.
//!
//! Generated parsers embed one packed word stream and read it back with
//! `dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer`. This
//! module is the tool-side half of that format: it turns the runtime ATN the
//! parser compiler built into the stream, at emission time.

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_state::{
    ATNState, ATNStateRef, ATNStateType, LoopEndState, RuleStartState, StarLoopEntryState,
};
use dbt_antlr_runtime::interval_set::IntervalSet;
use dbt_antlr_runtime::transition::{
    ActionTransition, AtomTransition, PrecedencePredicateTransition, PredicateTransition,
    RangeTransition, RuleTransition, Transition, TransitionType,
};

const PARSER_ATN_MAGIC: u32 = 0x5041_544e;
const PARSER_ATN_FORMAT_VERSION: u32 = 3;
const PARSER_ATN_BYTE_ORDER: u32 = 0x0102_0304;

const HEADER_WORDS: usize = 29;
const STATE_WORDS: usize = 7;
const TRANSITION_WORDS: usize = 5;
const SET_WORDS: usize = 5;
const PACKED_U64_WORDS: usize = 2;

const INLINE_TOKEN_SET_WORDS: usize = 2;
const INLINE_TOKEN_SET_MAX_SLOT: usize = INLINE_TOKEN_SET_WORDS * u64::BITS as usize - 1;
const MAX_DENSE_TOKEN_SET_BYTES: usize = 64 * 1024;
const MAX_DENSE_TOKEN_SET_WORDS: usize = MAX_DENSE_TOKEN_SET_BYTES / size_of::<u64>();
const DENSE_TOKEN_SET_COST_MULTIPLIER: usize = 2;
const DENSE_TOKEN_SET_MIN_DENSITY_DENOMINATOR: u64 = 8;

const NO_INDEX: u32 = u32::MAX;
const TOKEN_EOF: i32 = -1;

const FLAG_NON_GREEDY: u32 = 1 << 0;
const FLAG_PRECEDENCE_DECISION: u32 = 1 << 1;
const FLAG_LEFT_RECURSIVE_RULE: u32 = 1 << 2;
const FLAG_EPSILON_ONLY: u32 = 1 << 3;
const FLAG_RULE_STOP: u32 = 1 << 4;
const FLAG_HAS_CONSUMING: u32 = 1 << 5;
const FLAG_HAS_SEMANTIC: u32 = 1 << 6;

const HEADER_MAGIC: usize = 0;
const HEADER_VERSION: usize = 1;
const HEADER_BYTE_ORDER: usize = 2;
const HEADER_SIZE: usize = 3;
const HEADER_MAX_TOKEN_TYPE: usize = 4;
const HEADER_STATE_COUNT: usize = 5;
const HEADER_TRANSITION_COUNT: usize = 6;
const HEADER_SET_COUNT: usize = 7;
const HEADER_INTERVAL_COUNT: usize = 8;
const HEADER_DECISION_COUNT: usize = 9;
const HEADER_RULE_COUNT: usize = 10;
const HEADER_STATES: usize = 11;
const HEADER_TRANSITIONS: usize = 13;
const HEADER_SETS: usize = 15;
const HEADER_INTERVALS: usize = 17;
const HEADER_DECISIONS: usize = 19;
const HEADER_RULE_STARTS: usize = 21;
const HEADER_RULE_STOPS: usize = 23;
const HEADER_TOTAL_LEN: usize = 25;
const HEADER_TOKEN_BIT_WORD_COUNT: usize = 26;
const HEADER_TOKEN_BITS: usize = 27;

/// Membership representation selected for one immutable parser token set.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
enum TokenSetKind {
    /// Sorted, coalesced inclusive ranges searched by interval boundary.
    Intervals = 0,
    /// Two packed words covering EOF and token types `1..=127`.
    Inline128 = 1,
    /// A bounded packed word slice for a larger, cost-effective token domain.
    Dense = 2,
}

/// Serializes a compiled parser ATN into its packed word stream.
///
/// The stream is the canonical form of the ATN: every field the reader derives
/// again (state flags, token-set shapes, section offsets) is recomputed here,
/// and the rule return edges are left out because the reader synthesizes them
/// from the rule calls.
pub(crate) fn encode_packed_atn(atn: &ATN) -> Vec<u32> {
    let mut states = Vec::with_capacity(atn.states_count());
    let mut transitions = Vec::new();
    let mut sets = TokenSets::default();
    for state in atn.iter_states() {
        let transition_start = transitions.len();
        // A rule stop state receives one return edge per call site, and only
        // the call site knows the call precedence, so the reader synthesizes
        // those edges from the rule transitions instead of reading them here.
        if state.state_type() != ATNStateType::RuleStop {
            for transition in state.get_transitions() {
                transitions.push(transition_record(transition, &mut sets));
            }
        }
        states.push(StateRecord::new(state, transition_start, &transitions));
    }

    let layout = Layout::new(
        &states,
        &transitions,
        &sets,
        atn.decision_to_state.len(),
        atn.rule_to_start_state.len(),
    );
    let mut words = vec![0; layout.total_len];
    write_header(&mut words, atn, &states, &transitions, &sets, layout);
    write_states(&mut words, &states, layout.states);
    write_transitions(&mut words, &transitions, layout.transitions);
    write_sets(&mut words, &sets, layout);
    write_indices(
        &mut words,
        layout.decisions,
        atn.decision_to_state
            .iter()
            .map(|state| optional_index(state.get_state_number())),
    );
    write_indices(
        &mut words,
        layout.rule_starts,
        atn.rule_to_start_state
            .iter()
            .map(|state| optional_index(state.get_state_number())),
    );
    write_indices(
        &mut words,
        layout.rule_stops,
        atn.rule_to_stop_state
            .iter()
            .map(|state| optional_index(state.get_state_number())),
    );
    words
}

/// One state record of the stream.
struct StateRecord {
    kind: u32,
    rule_index: u32,
    flags: u32,
    transition_start: u32,
    transition_count: u32,
    end_state: u32,
    loop_back_state: u32,
}

impl StateRecord {
    fn new(state: &ATNState, transition_start: usize, transitions: &[TransitionRecord]) -> Self {
        let transition_count = transitions.len() - transition_start;
        let kinds = transitions[transition_start..]
            .iter()
            .map(|transition| transition.kind);
        Self {
            kind: u32::try_from(state.get_state_type_id()).expect("state type id is not negative"),
            rule_index: optional_index(state.get_rule_index()),
            flags: state_flags(state, transition_count, kinds),
            transition_start: compact(transition_start),
            transition_count: compact(transition_count),
            end_state: state.get_decision_end_state().map_or(NO_INDEX, link_index),
            loop_back_state: state
                .try_as::<LoopEndState>()
                .map_or(NO_INDEX, |state| link_index(state.loop_back_state)),
        }
    }
}

/// Derived state flags of the stream.
///
/// The reader recomputes the transition-derived flags from the records, so
/// they describe the emitted edges, not the ATN's caches.
fn state_flags(
    state: &ATNState,
    transition_count: usize,
    kinds: impl Iterator<Item = TransitionType>,
) -> u32 {
    let mut flags = match state.state_type() {
        ATNStateType::RuleStop => FLAG_RULE_STOP,
        _ => 0,
    };
    if state.is_nongreedy_decision() == Some(true) {
        flags |= FLAG_NON_GREEDY;
    }
    if state
        .try_as::<RuleStartState>()
        .is_some_and(|state| state.is_left_recursive)
    {
        flags |= FLAG_LEFT_RECURSIVE_RULE;
    }
    if state
        .try_as::<StarLoopEntryState>()
        .is_some_and(|state| state.is_precedence)
    {
        flags |= FLAG_PRECEDENCE_DECISION;
    }
    let mut epsilon_only = transition_count > 0;
    for kind in kinds {
        epsilon_only &= is_epsilon_kind(kind);
        if is_consuming_kind(kind) {
            flags |= FLAG_HAS_CONSUMING;
        }
        if is_semantic_kind(kind) {
            flags |= FLAG_HAS_SEMANTIC;
        }
    }
    if epsilon_only {
        flags |= FLAG_EPSILON_ONLY;
    }
    flags
}

const fn is_epsilon_kind(kind: TransitionType) -> bool {
    matches!(
        kind,
        TransitionType::Epsilon
            | TransitionType::Rule
            | TransitionType::Predicate
            | TransitionType::Action
            | TransitionType::PrecedencePredicate
    )
}

const fn is_consuming_kind(kind: TransitionType) -> bool {
    matches!(
        kind,
        TransitionType::Atom
            | TransitionType::Range
            | TransitionType::Set
            | TransitionType::NotSet
            | TransitionType::Wildcard
    )
}

const fn is_semantic_kind(kind: TransitionType) -> bool {
    matches!(
        kind,
        TransitionType::Predicate | TransitionType::Action | TransitionType::PrecedencePredicate
    )
}

/// One transition record of the stream.
struct TransitionRecord {
    kind: TransitionType,
    kind_word: u32,
    target: u32,
    arg0: u32,
    arg1: u32,
    arg2: u32,
}

fn transition_record(transition: &Transition, sets: &mut TokenSets) -> TransitionRecord {
    let kind = transition.transition_type();
    let target = link_index(transition.get_target());
    let (kind_word, arg0, arg1, arg2) = match kind {
        TransitionType::Epsilon => (1, 0, 0, 0),
        TransitionType::Range => {
            let range = transition
                .try_as::<RangeTransition>()
                .expect("range transition has range payload");
            (2, pack_i32(range.start()), pack_i32(range.stop()), 0)
        }
        TransitionType::Rule => {
            let rule = transition
                .try_as::<RuleTransition>()
                .expect("rule transition has rule payload");
            (
                3,
                optional_index(rule.rule_index()),
                link_index(rule.follow_state),
                pack_i32(rule.precedence()),
            )
        }
        TransitionType::Predicate => {
            let predicate = transition
                .try_as::<PredicateTransition>()
                .expect("predicate transition has predicate payload");
            (
                4,
                optional_index(predicate.rule_index()),
                optional_index(predicate.pred_index()),
                u32::from(predicate.is_ctx_dependent()),
            )
        }
        TransitionType::Atom => {
            let atom = transition
                .try_as::<AtomTransition>()
                .expect("atom transition has atom payload");
            (5, pack_i32(atom.label()), 0, 0)
        }
        TransitionType::Action => {
            let action = transition
                .try_as::<ActionTransition>()
                .expect("action transition has action payload");
            (
                6,
                optional_index(action.rule_index()),
                optional_index(action.action_index()),
                u32::from(action.is_ctx_dependent),
            )
        }
        TransitionType::Set => (
            7,
            compact(
                sets.intern(
                    transition
                        .get_label()
                        .expect("set transition has a set payload"),
                ),
            ),
            0,
            0,
        ),
        TransitionType::NotSet => (
            8,
            compact(
                sets.intern(
                    transition
                        .get_label()
                        .expect("not-set transition has a set payload"),
                ),
            ),
            0,
            0,
        ),
        TransitionType::Wildcard => (9, 0, 0, 0),
        TransitionType::PrecedencePredicate => {
            let precedence = transition
                .try_as::<PrecedencePredicateTransition>()
                .expect("precedence transition has precedence payload");
            (10, pack_i32(precedence.precedence()), 0, 0)
        }
    };
    TransitionRecord {
        kind,
        kind_word,
        target,
        arg0,
        arg1,
        arg2,
    }
}

/// The interval sets a stream shares between its transitions.
#[derive(Default)]
struct TokenSets {
    sets: Vec<TokenSetRecord>,
    intervals: Vec<(i32, i32)>,
    bit_words: Vec<u64>,
}

struct TokenSetRecord {
    interval_start: u32,
    interval_len: u32,
    kind: TokenSetKind,
    bit_start: u32,
    bit_len: u32,
}

impl TokenSets {
    /// Returns the index of `set`, adding it to the stream when it is new.
    fn intern(&mut self, set: &IntervalSet) -> usize {
        if let Some(index) = self
            .sets
            .iter()
            .position(|candidate| candidate.matches(&self.intervals, set))
        {
            return index;
        }
        let ranges = set
            .as_slice()
            .iter()
            .map(|interval| (interval.a, interval.b))
            .collect::<Vec<_>>();
        let prepared = prepare_token_set(&ranges);
        let record = TokenSetRecord {
            interval_start: compact(self.intervals.len()),
            interval_len: compact(ranges.len()),
            kind: prepared.kind,
            bit_start: compact(self.bit_words.len()),
            bit_len: compact(prepared.words.len()),
        };
        self.intervals.extend(ranges);
        self.bit_words.extend(prepared.words);
        self.sets.push(record);
        self.sets.len() - 1
    }
}

impl TokenSetRecord {
    fn matches(&self, intervals: &[(i32, i32)], set: &IntervalSet) -> bool {
        let start = self.interval_start as usize;
        let stored = &intervals[start..start + self.interval_len as usize];
        stored.len() == set.as_slice().len()
            && stored
                .iter()
                .zip(set.as_slice())
                .all(|((start, stop), interval)| (*start, *stop) == (interval.a, interval.b))
    }
}

/// The section offsets of one stream.
#[derive(Clone, Copy)]
struct Layout {
    states: Section,
    transitions: Section,
    sets: Section,
    intervals: Section,
    token_bits: Section,
    decisions: Section,
    rule_starts: Section,
    rule_stops: Section,
    total_len: usize,
}

#[derive(Clone, Copy)]
struct Section {
    offset: usize,
    len: usize,
}

impl Layout {
    fn new(
        states: &[StateRecord],
        transitions: &[TransitionRecord],
        sets: &TokenSets,
        decision_count: usize,
        rule_count: usize,
    ) -> Self {
        let mut cursor = HEADER_WORDS;
        let states = Section::next(&mut cursor, states.len(), STATE_WORDS);
        let transitions = Section::next(&mut cursor, transitions.len(), TRANSITION_WORDS);
        let sets_section = Section::next(&mut cursor, sets.sets.len(), SET_WORDS);
        let intervals = Section::next(&mut cursor, sets.intervals.len(), 2);
        let token_bits = Section::next(&mut cursor, sets.bit_words.len(), PACKED_U64_WORDS);
        let decisions = Section::next(&mut cursor, decision_count, 1);
        let rule_starts = Section::next(&mut cursor, rule_count, 1);
        let rule_stops = Section::next(&mut cursor, rule_count, 1);
        Self {
            states,
            transitions,
            sets: sets_section,
            intervals,
            token_bits,
            decisions,
            rule_starts,
            rule_stops,
            total_len: cursor,
        }
    }
}

impl Section {
    fn next(cursor: &mut usize, count: usize, width: usize) -> Self {
        let section = Self {
            offset: *cursor,
            len: count * width,
        };
        *cursor += section.len;
        section
    }
}

fn write_header(
    words: &mut [u32],
    atn: &ATN,
    states: &[StateRecord],
    transitions: &[TransitionRecord],
    sets: &TokenSets,
    layout: Layout,
) {
    words[HEADER_MAGIC] = PARSER_ATN_MAGIC;
    words[HEADER_VERSION] = PARSER_ATN_FORMAT_VERSION;
    words[HEADER_BYTE_ORDER] = PARSER_ATN_BYTE_ORDER;
    words[HEADER_SIZE] = compact(HEADER_WORDS);
    words[HEADER_MAX_TOKEN_TYPE] = pack_i32(atn.max_token_type);
    words[HEADER_STATE_COUNT] = compact(states.len());
    words[HEADER_TRANSITION_COUNT] = compact(transitions.len());
    words[HEADER_SET_COUNT] = compact(sets.sets.len());
    words[HEADER_INTERVAL_COUNT] = compact(sets.intervals.len());
    words[HEADER_DECISION_COUNT] = compact(atn.decision_to_state.len());
    words[HEADER_RULE_COUNT] = compact(atn.rule_to_start_state.len());
    write_section(words, HEADER_STATES, layout.states);
    write_section(words, HEADER_TRANSITIONS, layout.transitions);
    write_section(words, HEADER_SETS, layout.sets);
    write_section(words, HEADER_INTERVALS, layout.intervals);
    words[HEADER_TOKEN_BIT_WORD_COUNT] = compact(sets.bit_words.len());
    write_section(words, HEADER_TOKEN_BITS, layout.token_bits);
    write_section(words, HEADER_DECISIONS, layout.decisions);
    write_section(words, HEADER_RULE_STARTS, layout.rule_starts);
    write_section(words, HEADER_RULE_STOPS, layout.rule_stops);
    words[HEADER_TOTAL_LEN] = compact(layout.total_len);
}

fn write_section(words: &mut [u32], header: usize, section: Section) {
    words[header] = compact(section.offset);
    words[header + 1] = compact(section.len);
}

fn write_states(words: &mut [u32], states: &[StateRecord], section: Section) {
    for (index, state) in states.iter().enumerate() {
        let base = section.offset + index * STATE_WORDS;
        words[base] = state.kind;
        words[base + 1] = state.rule_index;
        words[base + 2] = state.flags;
        words[base + 3] = state.transition_start;
        words[base + 4] = state.transition_count;
        words[base + 5] = state.end_state;
        words[base + 6] = state.loop_back_state;
    }
}

/// Writes the transition records.
///
/// A transition kind word also has room for flags. The only one the format
/// defines marks a tail call, and the runtime deserializer validates it and
/// then drops it, so no classifier runs here and every record leaves it unset.
fn write_transitions(words: &mut [u32], transitions: &[TransitionRecord], section: Section) {
    for (index, transition) in transitions.iter().enumerate() {
        let base = section.offset + index * TRANSITION_WORDS;
        words[base] = transition.kind_word;
        words[base + 1] = transition.target;
        words[base + 2] = transition.arg0;
        words[base + 3] = transition.arg1;
        words[base + 4] = transition.arg2;
    }
}

fn write_sets(words: &mut [u32], sets: &TokenSets, layout: Layout) {
    for (index, set) in sets.sets.iter().enumerate() {
        let base = layout.sets.offset + index * SET_WORDS;
        words[base] = set.interval_start;
        words[base + 1] = set.interval_len;
        words[base + 2] = set.kind as u32;
        words[base + 3] = set.bit_start;
        words[base + 4] = set.bit_len;
    }
    for (index, &(start, stop)) in sets.intervals.iter().enumerate() {
        let base = layout.intervals.offset + index * 2;
        words[base] = pack_i32(start);
        words[base + 1] = pack_i32(stop);
    }
    for (index, &bits) in sets.bit_words.iter().enumerate() {
        let base = layout.token_bits.offset + index * PACKED_U64_WORDS;
        words[base] = bits as u32;
        words[base + 1] = (bits >> u32::BITS) as u32;
    }
}

fn write_indices(words: &mut [u32], section: Section, indices: impl Iterator<Item = u32>) {
    for (index, value) in indices.enumerate() {
        words[section.offset + index] = value;
    }
}

/// The membership shape and bit words of one token set.
///
/// Every compatible set at or below token slot 127 uses two inline words.
/// Larger sets use dense words only when the payload is at most 64 KiB and
/// either no larger than interval storage, or at most twice that storage while
/// covering at least one eighth of the indexed domain. Sparse, malformed, and
/// very large domains retain normalized interval lookup.
fn token_set_shape(ranges: impl IntoIterator<Item = (i32, i32)>) -> (TokenSetKind, usize) {
    let mut compatible = true;
    let mut max_slot = 0;
    let mut represented = 0_u64;
    let mut range_count = 0_usize;
    for (start, stop) in ranges {
        range_count += 1;
        represented = represented.saturating_add(
            u64::try_from(i64::from(stop) - i64::from(start) + 1).unwrap_or(u64::MAX),
        );
        if start == TOKEN_EOF && stop == TOKEN_EOF {
            continue;
        }
        if start < 1 {
            compatible = false;
            continue;
        }
        let stop = usize::try_from(stop).expect("positive i32 token type fits usize");
        max_slot = max_slot.max(stop);
    }
    if !compatible {
        return (TokenSetKind::Intervals, 0);
    }
    if max_slot <= INLINE_TOKEN_SET_MAX_SLOT {
        return (TokenSetKind::Inline128, INLINE_TOKEN_SET_WORDS);
    }
    let word_len = max_slot / u64::BITS as usize + 1;
    let Some(dense_bytes) = word_len.checked_mul(size_of::<u64>()) else {
        return (TokenSetKind::Intervals, 0);
    };
    let interval_bytes = range_count.saturating_mul(size_of::<(i32, i32)>());
    let dense_enough = represented.saturating_mul(DENSE_TOKEN_SET_MIN_DENSITY_DENOMINATOR)
        >= u64::try_from(max_slot)
            .unwrap_or(u64::MAX)
            .saturating_add(1);
    let cost_effective = dense_bytes <= interval_bytes
        || (dense_bytes <= interval_bytes.saturating_mul(DENSE_TOKEN_SET_COST_MULTIPLIER)
            && dense_enough);
    if word_len <= MAX_DENSE_TOKEN_SET_WORDS && cost_effective {
        (TokenSetKind::Dense, word_len)
    } else {
        (TokenSetKind::Intervals, 0)
    }
}

struct PreparedTokenSet {
    kind: TokenSetKind,
    words: Vec<u64>,
}

fn prepare_token_set(ranges: &[(i32, i32)]) -> PreparedTokenSet {
    let (kind, word_len) = token_set_shape(ranges.iter().copied());
    let mut words = vec![0; word_len];
    for &(start, stop) in ranges {
        insert_token_set_range(&mut words, start, stop);
    }
    PreparedTokenSet { kind, words }
}

fn insert_token_set_range(words: &mut [u64], start: i32, stop: i32) {
    if words.is_empty() {
        return;
    }
    if start == TOKEN_EOF && stop == TOKEN_EOF {
        words[0] |= 1;
        return;
    }
    debug_assert!(start >= 1 && stop >= start);
    let start = usize::try_from(start).expect("positive i32 token type fits usize");
    let stop = usize::try_from(stop).expect("positive i32 token type fits usize");
    let start_word = start / u64::BITS as usize;
    let stop_word = stop / u64::BITS as usize;
    if start_word == stop_word {
        words[start_word] |= token_word_mask(start % u64::BITS as usize, stop % u64::BITS as usize);
        return;
    }
    words[start_word] |= !0_u64 << (start % u64::BITS as usize);
    words[(start_word + 1)..stop_word].fill(!0);
    words[stop_word] |= !0_u64 >> (u64::BITS as usize - 1 - stop % u64::BITS as usize);
}

const fn token_word_mask(start: usize, stop: usize) -> u64 {
    (!0_u64 << start) & (!0_u64 >> (u64::BITS as usize - 1 - stop))
}

fn link_index(reference: ATNStateRef) -> u32 {
    optional_index(reference.get_state_number())
}

/// Packs an index that may be absent, such as a rule index of a state that
/// belongs to no rule, as the `NO_INDEX` sentinel.
fn optional_index(value: i32) -> u32 {
    u32::try_from(value).unwrap_or(NO_INDEX)
}

fn compact(value: usize) -> u32 {
    u32::try_from(value).expect("packed parser ATN value exceeds u32")
}

const fn pack_i32(value: i32) -> u32 {
    value as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
        ranges.to_vec()
    }

    /// Minimal ATN with one rule, one decision, no transitions, and no token
    /// sets, so the writer's handling of empty sections stays covered.
    fn minimal_atn() -> ATN {
        use dbt_antlr_runtime::atn_state::{
            BaseATNState, BasicBlockStartState, RuleStartState, RuleStopState,
        };
        use dbt_antlr_runtime::atn_type::ATNType;

        let mut atn = ATN::new(ATNType::Parser, 3);
        atn.alloc_states(3);
        for (number, state) in [
            RuleStartState::create(BaseATNState::new(0, 0, 2), ATNStateRef::invalid(), false),
            BasicBlockStartState::create(
                BaseATNState::new(1, 0, 3),
                -1,
                false,
                ATNStateRef::invalid(),
            ),
            RuleStopState::create(BaseATNState::new(2, 0, 7)),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(state.get_state_number() as usize, number);
            atn.add_state(state);
        }
        let start = atn.make_state_ref(0);
        let stop = atn.make_state_ref(2);
        atn.rule_to_start_state = vec![start];
        atn.rule_to_stop_state = vec![stop];
        atn.get_state_mut(0)
            .try_as_mut::<RuleStartState>()
            .expect("rule start state")
            .stop_state = stop;
        atn.decision_to_state = vec![atn.make_state_ref(1)];
        *atn.get_state_mut(1)
            .get_decision_mut()
            .expect("decision state") = 0;
        atn
    }

    #[test]
    fn minimal_atn_round_trips_through_the_runtime_reader() {
        use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;

        let words = encode_packed_atn(&minimal_atn());
        assert_eq!(words[HEADER_MAGIC], PARSER_ATN_MAGIC);
        assert_eq!(words[HEADER_VERSION], PARSER_ATN_FORMAT_VERSION);
        assert_eq!(words[HEADER_STATE_COUNT], 3);
        assert_eq!(words[HEADER_SET_COUNT], 0);
        assert_eq!(words[HEADER_DECISION_COUNT], 1);
        assert_eq!(words[HEADER_RULE_COUNT], 1);
        let atn = PackedATNDeserializer::new()
            .deserialize(&words)
            .expect("written words deserialize");
        assert_eq!(
            dbt_antlr_runtime::atn_dump::dump_atn(&atn),
            dbt_antlr_runtime::atn_dump::dump_atn(&minimal_atn()),
        );
    }

    #[test]
    fn small_token_domains_pack_inline() {
        assert_eq!(
            token_set_shape(ranges(&[(-1, -1), (1, 1), (5, 9)])),
            (TokenSetKind::Inline128, INLINE_TOKEN_SET_WORDS),
        );
        assert_eq!(
            prepare_token_set(&ranges(&[(-1, -1), (1, 1), (5, 9)])).words,
            vec![0b11_1110_0011, 0],
        );
    }

    #[test]
    fn dense_domains_cover_the_indexed_range() {
        let mut words = vec![0_u64; 4];
        insert_token_set_range(&mut words, 1, 200);
        assert_eq!(words, vec![!0 << 1, !0, !0, 0b1_1111_1111]);
        insert_token_set_range(&mut words, -1, -1);
        assert_eq!(words[0], !0);
    }

    #[test]
    fn negative_slots_keep_interval_lookup() {
        assert_eq!(
            token_set_shape(ranges(&[(-5, -1), (1, 3)])),
            (TokenSetKind::Intervals, 0),
        );
    }

    #[test]
    fn sparse_large_domains_keep_interval_lookup() {
        assert_eq!(
            token_set_shape(ranges(&[(1, 1), (100_000, 100_000)])).0,
            TokenSetKind::Intervals,
        );
    }

    #[test]
    fn dense_domains_with_many_ranges_pack_dense() {
        let ranges = (129..=257).step_by(2).map(|token| (token, token));
        assert_eq!(
            token_set_shape(ranges),
            (TokenSetKind::Dense, 257 / u64::BITS as usize + 1),
        );
    }

    #[test]
    fn few_ranges_in_a_wide_domain_keep_interval_lookup() {
        assert_eq!(
            token_set_shape(ranges(&[(1, 1_000)])),
            (TokenSetKind::Intervals, 0),
        );
    }
}
