//! Deserializer for the packed parser ATN `u32` word format.
//!
//! The packed format is the index-addressed parser ATN encoding defined by
//! the `antlr-rust-runtime` crate (`atn::parser_atn`). Generated parsers can
//! embed packed words directly instead of the Java serialized `i32` stream
//! that [`crate::atn_deserializer::ATNDeserializer`] reads. Both paths build
//! the same [`ATN`] graph.
//!
//! Layout of the version 3 word stream:
//!
//! - Words 0..29: header with magic, format version, byte-order marker,
//!   counts, and `(offset, len)` section descriptors.
//! - Then the sections in order: states, transitions, sets, intervals,
//!   token bits, decisions, rule starts, rule stops.
//!
//! State records are 7 words: kind, rule index, flags, transition start,
//! transition count, end state, loop-back state. Transition records are
//! 5 words: kind and flags, target state, and 3 argument words. Set records
//! are 5 words: interval start, interval length, kind, bit start, bit
//! length. The set kind and the token-bits section only select a membership
//! representation; the interval list is always complete.

use std::fmt;
use std::fmt::Formatter;

use crate::atn::ATN;
use crate::atn_state::*;
use crate::atn_type::ATNType;
use crate::interval_set::IntervalSet;
use crate::interval_set::IntervalSetBuf;
use crate::transition::*;

const PACKED_ATN_MAGIC: u32 = 0x5041_544e;
const PACKED_ATN_FORMAT_VERSION: u32 = 3;
const PACKED_ATN_BYTE_ORDER: u32 = 0x0102_0304;
const HEADER_WORDS: usize = 29;
const STATE_WORDS: usize = 7;
const TRANSITION_WORDS: usize = 5;
const SET_WORDS: usize = 5;

const NO_INDEX: u32 = u32::MAX;

const TRANSITION_KIND_MASK: u32 = 0xff;
const TRANSITION_FLAG_TAIL_CALL: u32 = 1 << 8;

const FLAG_NON_GREEDY: u32 = 1 << 0;
const FLAG_PRECEDENCE_DECISION: u32 = 1 << 1;
const FLAG_LEFT_RECURSIVE_RULE: u32 = 1 << 2;
const STATE_FLAGS: u32 = 0x7f;

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
const HEADER_STATES_OFFSET: usize = 11;
const HEADER_TRANSITIONS_OFFSET: usize = 13;
const HEADER_SETS_OFFSET: usize = 15;
const HEADER_INTERVALS_OFFSET: usize = 17;
const HEADER_DECISIONS_OFFSET: usize = 19;
const HEADER_RULE_STARTS_OFFSET: usize = 21;
const HEADER_RULE_STOPS_OFFSET: usize = 23;
const HEADER_TOTAL_LEN: usize = 25;
const HEADER_TOKEN_BIT_WORD_COUNT: usize = 26;
const HEADER_TOKEN_BITS_OFFSET: usize = 27;

/// Failure while decoding a packed parser ATN word stream.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackedATNError {
    /// Word 0 is not the packed ATN magic value.
    BadMagic {
        /// The value found in word 0.
        found: u32,
    },
    /// The format version is not 3.
    UnsupportedVersion {
        /// The version found in word 1.
        found: u32,
    },
    /// Word 2 is not the byte-order marker.
    BadByteOrderMarker {
        /// The value found in word 2.
        found: u32,
    },
    /// The header size word does not match the version 3 header.
    BadHeaderSize {
        /// The value found in word 3.
        found: u32,
    },
    /// The total length word does not match the stream length.
    TotalLengthMismatch {
        /// The declared total word count.
        declared: usize,
        /// The actual stream word count.
        actual: usize,
    },
    /// The stream ends before the declared data.
    TruncatedStream {
        /// The word count the stream must have at least.
        expected_at_least: usize,
        /// The actual stream word count.
        actual: usize,
    },
    /// A section length does not match the matching header count.
    SectionLengthMismatch {
        /// The section name.
        section: &'static str,
        /// The word count the section must have.
        expected: usize,
        /// The declared section word count.
        found: usize,
    },
    /// A section or record range falls outside its container.
    RangeOutOfBounds {
        /// The section name.
        section: &'static str,
        /// The range start.
        offset: usize,
        /// The range length.
        len: usize,
        /// The container word count.
        total: usize,
    },
    /// An index points outside its table.
    IndexOutOfBounds {
        /// What the index addresses.
        what: &'static str,
        /// The index value.
        index: u32,
        /// The number of valid entries.
        count: usize,
    },
    /// State transition ranges are not contiguous in state order.
    NonContiguousStateTransitions {
        /// The state with the wrong transition start.
        state: usize,
        /// The declared transition start.
        start: usize,
        /// The expected transition start.
        expected: usize,
    },
    /// The state transition ranges do not cover the transitions section.
    TransitionCountMismatch {
        /// The number of transitions the states cover.
        covered: usize,
        /// The declared transition count.
        declared: usize,
    },
    /// Unknown state kind.
    InvalidStateKind {
        /// The kind word value.
        value: u32,
    },
    /// Unknown transition kind.
    InvalidTransitionKind {
        /// The kind field value.
        value: u32,
    },
    /// Unknown token set kind.
    InvalidTokenSetKind {
        /// The kind word value.
        value: u32,
    },
    /// A state carries unknown flag bits.
    UnknownStateFlags {
        /// The state number.
        state: usize,
        /// The unknown flag bits.
        flags: u32,
    },
    /// A transition carries unknown flag bits.
    UnknownTransitionFlags {
        /// The transition index.
        transition: usize,
        /// The unknown flag bits.
        flags: u32,
    },
    /// A state does not have the kind its role requires.
    UnexpectedStateKind {
        /// The state number.
        state: usize,
        /// The required kind.
        expected: &'static str,
    },
    /// A range transition has start after stop.
    InvalidRangeBounds {
        /// The range start.
        start: i32,
        /// The range stop.
        stop: i32,
    },
    /// A boolean word is not 0 or 1.
    InvalidBooleanFlag {
        /// What the flag controls.
        what: &'static str,
        /// The flag word value.
        value: u32,
    },
}

impl fmt::Display for PackedATNError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            PackedATNError::BadMagic { found } => {
                write!(
                    f,
                    "bad magic 0x{found:08x}; expected 0x{PACKED_ATN_MAGIC:08x}"
                )
            }
            PackedATNError::UnsupportedVersion { found } => {
                write!(
                    f,
                    "unsupported format version {found}; expected {PACKED_ATN_FORMAT_VERSION}"
                )
            }
            PackedATNError::BadByteOrderMarker { found } => {
                write!(
                    f,
                    "bad byte-order marker 0x{found:08x}; expected 0x{PACKED_ATN_BYTE_ORDER:08x}"
                )
            }
            PackedATNError::BadHeaderSize { found } => {
                write!(f, "bad header size {found}; expected {HEADER_WORDS}")
            }
            PackedATNError::TotalLengthMismatch { declared, actual } => {
                write!(
                    f,
                    "declared total length {declared} does not match stream length {actual}"
                )
            }
            PackedATNError::TruncatedStream {
                expected_at_least,
                actual,
            } => {
                write!(
                    f,
                    "stream has {actual} words; expected at least {expected_at_least}"
                )
            }
            PackedATNError::SectionLengthMismatch {
                section,
                expected,
                found,
            } => {
                write!(
                    f,
                    "{section} section has {found} words; expected {expected}"
                )
            }
            PackedATNError::RangeOutOfBounds {
                section,
                offset,
                len,
                total,
            } => {
                write!(
                    f,
                    "{section} range {offset}..{} exceeds total {total}",
                    offset + len
                )
            }
            PackedATNError::IndexOutOfBounds { what, index, count } => {
                write!(f, "{what} index {index} outside 0..{count}")
            }
            PackedATNError::NonContiguousStateTransitions {
                state,
                start,
                expected,
            } => {
                write!(
                    f,
                    "state {state} transitions start at {start}; expected {expected}"
                )
            }
            PackedATNError::TransitionCountMismatch { covered, declared } => {
                write!(f, "states cover {covered} transitions; expected {declared}")
            }
            PackedATNError::InvalidStateKind { value } => {
                write!(f, "invalid state kind {value}")
            }
            PackedATNError::InvalidTransitionKind { value } => {
                write!(f, "invalid transition kind {value}")
            }
            PackedATNError::InvalidTokenSetKind { value } => {
                write!(f, "invalid token set kind {value}")
            }
            PackedATNError::UnknownStateFlags { state, flags } => {
                write!(f, "state {state} has unknown flags 0x{flags:x}")
            }
            PackedATNError::UnknownTransitionFlags { transition, flags } => {
                write!(f, "transition {transition} has unknown flags 0x{flags:x}")
            }
            PackedATNError::UnexpectedStateKind { state, expected } => {
                write!(f, "state {state} is not a {expected} state")
            }
            PackedATNError::InvalidRangeBounds { start, stop } => {
                write!(f, "range starts at {start} after stop {stop}")
            }
            PackedATNError::InvalidBooleanFlag { what, value } => {
                write!(f, "{what} flag is {value}; expected 0 or 1")
            }
        }
    }
}

impl std::error::Error for PackedATNError {}

/// Deserializer for the packed parser ATN word format.
///
/// Mirrors the construction semantics of
/// [`crate::atn_deserializer::ATNDeserializer`], but reads the packed `u32`
/// word stream. Packed data is parser-only: lexer fields of the resulting
/// [`ATN`] stay empty.
#[derive(Debug, Default)]
pub struct PackedATNDeserializer;

impl PackedATNDeserializer {
    /// Creates a new deserializer.
    pub fn new() -> Self {
        PackedATNDeserializer
    }

    /// Builds an [`ATN`] from a packed word stream.
    ///
    /// Returns [`PackedATNError`] when the stream violates the format.
    pub fn deserialize(&self, words: &[u32]) -> Result<ATN, PackedATNError> {
        let layout = Layout::read(words)?;
        let mut atn = ATN::new_atn(ATNType::Parser, layout.max_token_type);
        let transition_ranges = self.read_states(&mut atn, words, &layout)?;
        self.read_rules(&mut atn, words, &layout)?;
        let sets = self.read_sets(words, &layout)?;
        self.read_transitions(&mut atn, words, &layout, &transition_ranges, &sets)?;
        self.read_decisions(&mut atn, words, &layout)?;
        self.add_rule_return_edges(&mut atn);
        Ok(atn)
    }

    fn read_states(
        &self,
        atn: &mut ATN,
        words: &[u32],
        layout: &Layout,
    ) -> Result<Vec<(usize, usize)>, PackedATNError> {
        atn.alloc_states(layout.state_count);
        let mut ranges = Vec::with_capacity(layout.state_count);
        let mut cursor = 0;
        for n in 0..layout.state_count {
            let base = layout.states.offset + n * STATE_WORDS;
            let kind = words[base] as i32;
            if !(ATNSTATE_BASIC..=ATNSTATE_LOOP_END).contains(&kind) {
                return Err(PackedATNError::InvalidStateKind { value: words[base] });
            }
            let rule_word = words[base + 1];
            let rule_index = if rule_word == NO_INDEX {
                -1
            } else {
                check_index("state rule", rule_word, layout.rule_count)?;
                rule_word as i32
            };
            let flags = words[base + 2];
            if flags & !STATE_FLAGS != 0 {
                return Err(PackedATNError::UnknownStateFlags {
                    state: n,
                    flags: flags & !STATE_FLAGS,
                });
            }
            let start = words[base + 3] as usize;
            let count = words[base + 4] as usize;
            if start != cursor {
                return Err(PackedATNError::NonContiguousStateTransitions {
                    state: n,
                    start,
                    expected: cursor,
                });
            }
            check_range("state transitions", start, count, layout.transition_count)?;
            cursor += count;
            ranges.push((start, count));

            let mut state = state_factory(kind, rule_index, n as i32, flags);
            let end_word = words[base + 5];
            if end_word != NO_INDEX {
                check_index("block end state", end_word, layout.state_count)?;
                if let Some(slot) = state.get_decision_end_state_mut() {
                    *slot = atn.make_state_ref(end_word as i32);
                }
            }
            let loop_back_word = words[base + 6];
            if loop_back_word != NO_INDEX {
                check_index("loop back state", loop_back_word, layout.state_count)?;
                // Matches ATNDeserializer: only LoopEnd states get the link.
                if let Some(LoopEndState {
                    ref mut loop_back_state,
                    ..
                }) = state.try_as_mut()
                {
                    *loop_back_state = atn.make_state_ref(loop_back_word as i32);
                }
            }
            atn.add_state(state);
        }
        if cursor != layout.transition_count {
            return Err(PackedATNError::TransitionCountMismatch {
                covered: cursor,
                declared: layout.transition_count,
            });
        }
        Ok(ranges)
    }

    fn read_rules(
        &self,
        atn: &mut ATN,
        words: &[u32],
        layout: &Layout,
    ) -> Result<(), PackedATNError> {
        atn.rule_to_start_state
            .resize(layout.rule_count, ATNStateRef::invalid());
        atn.rule_to_stop_state
            .resize(layout.rule_count, ATNStateRef::invalid());
        for i in 0..layout.rule_count {
            let start_word = words[layout.rule_starts.offset + i];
            let stop_word = words[layout.rule_stops.offset + i];
            check_index("rule start state", start_word, layout.state_count)?;
            check_index("rule stop state", stop_word, layout.state_count)?;
            if atn.get_state(start_word as i32).state_type() != ATNStateType::RuleStart {
                return Err(PackedATNError::UnexpectedStateKind {
                    state: start_word as usize,
                    expected: "rule start",
                });
            }
            if atn.get_state(stop_word as i32).state_type() != ATNStateType::RuleStop {
                return Err(PackedATNError::UnexpectedStateKind {
                    state: stop_word as usize,
                    expected: "rule stop",
                });
            }
            let start_ref = atn.make_state_ref(start_word as i32);
            let stop_ref = atn.make_state_ref(stop_word as i32);
            atn.rule_to_start_state[i] = start_ref;
            atn.rule_to_stop_state[i] = stop_ref;
            unsafe {
                if let Some(RuleStartState {
                    ref mut stop_state, ..
                }) = start_ref.as_mut().try_as_mut()
                {
                    *stop_state = stop_ref;
                }
            }
        }
        Ok(())
    }

    fn read_sets(
        &self,
        words: &[u32],
        layout: &Layout,
    ) -> Result<Vec<&'static IntervalSet>, PackedATNError> {
        let interval_count = layout.intervals.len / 2;
        let mut sets = Vec::with_capacity(layout.set_count);
        for s in 0..layout.set_count {
            let base = layout.sets.offset + s * SET_WORDS;
            let start = words[base] as usize;
            let len = words[base + 1] as usize;
            check_range("interval set", start, len, interval_count)?;
            let kind = words[base + 2];
            if kind > 2 {
                return Err(PackedATNError::InvalidTokenSetKind { value: kind });
            }
            let mut set = IntervalSetBuf::new();
            for k in 0..len {
                let a = words[layout.intervals.offset + (start + k) * 2] as i32;
                let b = words[layout.intervals.offset + (start + k) * 2 + 1] as i32;
                set.add_range(a, b);
            }
            sets.push(set.into_static() as &'static IntervalSet);
        }
        Ok(sets)
    }

    fn read_transitions(
        &self,
        atn: &mut ATN,
        words: &[u32],
        layout: &Layout,
        transition_ranges: &[(usize, usize)],
        sets: &[&'static IntervalSet],
    ) -> Result<(), PackedATNError> {
        for (n, &(start, count)) in transition_ranges.iter().enumerate() {
            let is_rule_stop = atn.get_state(n as i32).state_type() == ATNStateType::RuleStop;
            for t in start..start + count {
                let base = layout.transitions.offset + t * TRANSITION_WORDS;
                let kind_word = words[base];
                let flags = kind_word & !TRANSITION_KIND_MASK;
                // The tail-call bit is a packed-runtime optimization. It
                // carries no graph semantics here.
                if flags & !TRANSITION_FLAG_TAIL_CALL != 0 {
                    return Err(PackedATNError::UnknownTransitionFlags {
                        transition: t,
                        flags: flags & !TRANSITION_FLAG_TAIL_CALL,
                    });
                }
                let kind = (kind_word & TRANSITION_KIND_MASK) as i32;
                if kind == TRANSITION_EPSILON && is_rule_stop {
                    // Rule-return edges are resynthesized from the rule
                    // transitions, because packed epsilon edges carry no
                    // precedence-return payload.
                    continue;
                }
                let target_word = words[base + 1];
                check_index("transition target", target_word, layout.state_count)?;
                let target = atn.make_state_ref(target_word as i32);
                let arg0 = words[base + 2];
                let arg1 = words[base + 3];
                let arg2 = words[base + 4];
                let transition = match kind {
                    TRANSITION_EPSILON => EpsilonTransition::create(target, 0),
                    TRANSITION_RANGE => {
                        let (start, stop) = (arg0 as i32, arg1 as i32);
                        if start > stop {
                            return Err(PackedATNError::InvalidRangeBounds { start, stop });
                        }
                        RangeTransition::create(target, start, stop)
                    }
                    TRANSITION_RULE => {
                        check_index("rule transition rule", arg0, layout.rule_count)?;
                        check_index("rule follow state", arg1, layout.state_count)?;
                        if target.state_type() != ATNStateType::RuleStart {
                            return Err(PackedATNError::UnexpectedStateKind {
                                state: target_word as usize,
                                expected: "rule start",
                            });
                        }
                        RuleTransition::create(
                            target,
                            atn.make_state_ref(arg1 as i32),
                            arg0 as i32,
                            arg2 as i32,
                        )
                    }
                    TRANSITION_PREDICATE => {
                        check_index("predicate rule", arg0, layout.rule_count)?;
                        check_bool("predicate context-dependent", arg2)?;
                        PredicateTransition::create(target, arg2 != 0, arg0 as i32, arg1 as i32)
                    }
                    TRANSITION_ATOM => AtomTransition::create(target, arg0 as i32),
                    TRANSITION_ACTION => {
                        check_index("action rule", arg0, layout.rule_count)?;
                        check_bool("action context-dependent", arg2)?;
                        let action_index = if arg1 == NO_INDEX { -1 } else { arg1 as i32 };
                        ActionTransition::create(target, arg2 != 0, arg0 as i32, action_index, 0)
                    }
                    TRANSITION_SET => {
                        check_index("set transition set", arg0, layout.set_count)?;
                        SetTransition::create(target, sets[arg0 as usize])
                    }
                    TRANSITION_NOTSET => {
                        check_index("not-set transition set", arg0, layout.set_count)?;
                        NotSetTransition::create(target, sets[arg0 as usize])
                    }
                    TRANSITION_WILDCARD => WildcardTransition::create(target),
                    TRANSITION_PRECEDENCE => {
                        PrecedencePredicateTransition::create(target, arg0 as i32)
                    }
                    _ => return Err(PackedATNError::InvalidTransitionKind { value: kind as u32 }),
                };
                atn.get_state_mut(n as i32).add_transition(transition);
            }
        }
        Ok(())
    }

    fn read_decisions(
        &self,
        atn: &mut ATN,
        words: &[u32],
        layout: &Layout,
    ) -> Result<(), PackedATNError> {
        for i in 0..layout.decision_count {
            let word = words[layout.decisions.offset + i];
            check_index("decision state", word, layout.state_count)?;
            atn.decision_to_state.push(atn.make_state_ref(word as i32));
            if let Some(decision) = atn.get_state_mut(word as i32).get_decision_mut() {
                *decision = i as i32;
            }
        }
        Ok(())
    }

    fn add_rule_return_edges(&self, atn: &mut ATN) {
        for state in atn.iter_states() {
            for tr in state.get_transitions() {
                let Some(tr) = tr.try_as::<RuleTransition>() else {
                    continue;
                };
                let target = tr.get_target();
                let outermost_prec_return = if let Some(RuleStartState {
                    is_left_recursive: true,
                    ..
                }) =
                    atn.rule_to_start_state[target.get_rule_index() as usize].try_as()
                {
                    if tr.precedence() == 0 {
                        target.get_rule_index()
                    } else {
                        -1
                    }
                } else {
                    -1
                };
                let return_tr = EpsilonTransition::create(tr.follow_state, outermost_prec_return);
                unsafe {
                    atn.rule_to_stop_state[target.get_rule_index() as usize]
                        .as_mut()
                        .add_transition(return_tr);
                }
            }
        }
    }
}

fn state_factory(kind: i32, rule_index: i32, state_number: i32, flags: u32) -> ATNState {
    let base = BaseATNState::new(state_number, rule_index, kind);
    let nongreedy = flags & FLAG_NON_GREEDY != 0;
    let invalid_ref = ATNStateRef::invalid();
    match kind {
        ATNSTATE_BASIC => BasicState::create(base),
        ATNSTATE_RULE_START => {
            RuleStartState::create(base, invalid_ref, flags & FLAG_LEFT_RECURSIVE_RULE != 0)
        }
        ATNSTATE_BLOCK_START => BasicBlockStartState::create(base, -1, nongreedy, invalid_ref),
        ATNSTATE_PLUS_BLOCK_START => {
            PlusBlockStartState::create(base, -1, nongreedy, invalid_ref, invalid_ref)
        }
        ATNSTATE_STAR_BLOCK_START => StarBlockStartState::create(base, -1, nongreedy, invalid_ref),
        ATNSTATE_TOKEN_START => TokenStartState::create(base, -1, nongreedy),
        ATNSTATE_RULE_STOP => RuleStopState::create(base),
        ATNSTATE_BLOCK_END => BlockEndState::create(base, invalid_ref),
        ATNSTATE_STAR_LOOP_BACK => StarLoopbackState::create(base),
        ATNSTATE_STAR_LOOP_ENTRY => StarLoopEntryState::create(
            base,
            -1,
            nongreedy,
            invalid_ref,
            flags & FLAG_PRECEDENCE_DECISION != 0,
        ),
        ATNSTATE_PLUS_LOOP_BACK => PlusLoopBackState::create(base, -1, nongreedy),
        ATNSTATE_LOOP_END => LoopEndState::create(base, invalid_ref),
        _ => unreachable!("state kind was validated"),
    }
}

fn check_index(what: &'static str, index: u32, count: usize) -> Result<(), PackedATNError> {
    if index as usize >= count {
        return Err(PackedATNError::IndexOutOfBounds { what, index, count });
    }
    Ok(())
}

fn check_bool(what: &'static str, value: u32) -> Result<(), PackedATNError> {
    if value > 1 {
        return Err(PackedATNError::InvalidBooleanFlag { what, value });
    }
    Ok(())
}

fn check_range(
    section: &'static str,
    offset: usize,
    len: usize,
    total: usize,
) -> Result<(), PackedATNError> {
    let out_of_bounds = offset.checked_add(len).is_none_or(|end| end > total);
    if out_of_bounds {
        return Err(PackedATNError::RangeOutOfBounds {
            section,
            offset,
            len,
            total,
        });
    }
    Ok(())
}

#[derive(Debug)]
struct Section {
    offset: usize,
    len: usize,
}

#[derive(Debug)]
struct Layout {
    max_token_type: i32,
    state_count: usize,
    transition_count: usize,
    set_count: usize,
    decision_count: usize,
    rule_count: usize,
    states: Section,
    transitions: Section,
    sets: Section,
    intervals: Section,
    decisions: Section,
    rule_starts: Section,
    rule_stops: Section,
}

impl Layout {
    fn read(words: &[u32]) -> Result<Layout, PackedATNError> {
        if words.len() < HEADER_WORDS {
            return Err(PackedATNError::TruncatedStream {
                expected_at_least: HEADER_WORDS,
                actual: words.len(),
            });
        }
        if words[HEADER_MAGIC] != PACKED_ATN_MAGIC {
            return Err(PackedATNError::BadMagic {
                found: words[HEADER_MAGIC],
            });
        }
        if words[HEADER_VERSION] != PACKED_ATN_FORMAT_VERSION {
            return Err(PackedATNError::UnsupportedVersion {
                found: words[HEADER_VERSION],
            });
        }
        if words[HEADER_BYTE_ORDER] != PACKED_ATN_BYTE_ORDER {
            return Err(PackedATNError::BadByteOrderMarker {
                found: words[HEADER_BYTE_ORDER],
            });
        }
        if words[HEADER_SIZE] as usize != HEADER_WORDS {
            return Err(PackedATNError::BadHeaderSize {
                found: words[HEADER_SIZE],
            });
        }
        if words[HEADER_TOTAL_LEN] as usize != words.len() {
            return Err(PackedATNError::TotalLengthMismatch {
                declared: words[HEADER_TOTAL_LEN] as usize,
                actual: words.len(),
            });
        }

        let state_count = words[HEADER_STATE_COUNT] as usize;
        let transition_count = words[HEADER_TRANSITION_COUNT] as usize;
        let set_count = words[HEADER_SET_COUNT] as usize;
        let interval_count = words[HEADER_INTERVAL_COUNT] as usize;
        let decision_count = words[HEADER_DECISION_COUNT] as usize;
        let rule_count = words[HEADER_RULE_COUNT] as usize;
        let token_bit_word_count = words[HEADER_TOKEN_BIT_WORD_COUNT] as usize;

        let states = read_section(words, HEADER_STATES_OFFSET, "states")?;
        let transitions = read_section(words, HEADER_TRANSITIONS_OFFSET, "transitions")?;
        let sets = read_section(words, HEADER_SETS_OFFSET, "sets")?;
        let intervals = read_section(words, HEADER_INTERVALS_OFFSET, "intervals")?;
        let decisions = read_section(words, HEADER_DECISIONS_OFFSET, "decisions")?;
        let rule_starts = read_section(words, HEADER_RULE_STARTS_OFFSET, "rule starts")?;
        let rule_stops = read_section(words, HEADER_RULE_STOPS_OFFSET, "rule stops")?;
        let token_bits = read_section(words, HEADER_TOKEN_BITS_OFFSET, "token bits")?;

        check_section_len("states", &states, state_count, STATE_WORDS)?;
        check_section_len(
            "transitions",
            &transitions,
            transition_count,
            TRANSITION_WORDS,
        )?;
        check_section_len("sets", &sets, set_count, SET_WORDS)?;
        check_section_len("intervals", &intervals, interval_count, 2)?;
        check_section_len("decisions", &decisions, decision_count, 1)?;
        check_section_len("rule starts", &rule_starts, rule_count, 1)?;
        check_section_len("rule stops", &rule_stops, rule_count, 1)?;
        check_section_len("token bits", &token_bits, token_bit_word_count, 2)?;

        Ok(Layout {
            max_token_type: words[HEADER_MAX_TOKEN_TYPE] as i32,
            state_count,
            transition_count,
            set_count,
            decision_count,
            rule_count,
            states,
            transitions,
            sets,
            intervals,
            decisions,
            rule_starts,
            rule_stops,
        })
    }
}

fn read_section(
    words: &[u32],
    header_offset: usize,
    section: &'static str,
) -> Result<Section, PackedATNError> {
    let offset = words[header_offset] as usize;
    let len = words[header_offset + 1] as usize;
    check_range(section, offset, len, words.len())?;
    Ok(Section { offset, len })
}

fn check_section_len(
    section: &'static str,
    found: &Section,
    count: usize,
    width: usize,
) -> Result<(), PackedATNError> {
    let expected = count
        .checked_mul(width)
        .ok_or(PackedATNError::SectionLengthMismatch {
            section,
            expected: usize::MAX,
            found: found.len,
        })?;
    if found.len != expected {
        return Err(PackedATNError::SectionLengthMismatch {
            section,
            expected,
            found: found.len,
        });
    }
    Ok(())
}
