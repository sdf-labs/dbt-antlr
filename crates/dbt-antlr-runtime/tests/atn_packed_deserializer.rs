//! Tests for the packed parser ATN deserializer and the normalized ATN dump.

use dbt_antlr_runtime::atn_dump::dump_atn;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNError;

const NO_INDEX: u32 = u32::MAX;

/// Two rules, five states:
/// - rule 0: state 0 --rule(1)--> state 3, state 1 --atom(7)--> state 2
/// - rule 1: state 3 --epsilon--> state 4
/// - packed rule-stop edges state 2 -> 1 and state 4 -> 1, which the
///   deserializer skips and resynthesizes
fn sample_words() -> Vec<u32> {
    let mut words = vec![
        0x5041_544e, // magic
        3,           // format version
        0x0102_0304, // byte-order marker
        29,          // header size
        7,           // max_token_type
        5,           // state count
        5,           // transition count
        0,           // set count
        0,           // interval count
        0,           // decision count
        2,           // rule count
        29,
        35, // states (offset, len)
        64,
        25, // transitions
        89,
        0, // sets
        89,
        0, // intervals
        89,
        0, // decisions
        89,
        2, // rule starts
        91,
        2,  // rule stops
        93, // total length
        0,  // token-bit word count
        89,
        0, // token bits
    ];
    let states: [[u32; 7]; 5] = [
        // kind, rule, flags, transition start, transition count, end, loop back
        [2, 0, 0, 0, 1, NO_INDEX, NO_INDEX], // 0: rule 0 start
        [1, 0, 0, 1, 1, NO_INDEX, NO_INDEX], // 1: basic
        [7, 0, 0, 2, 1, NO_INDEX, NO_INDEX], // 2: rule 0 stop
        [2, 1, 0, 3, 1, NO_INDEX, NO_INDEX], // 3: rule 1 start
        [7, 1, 0, 4, 1, NO_INDEX, NO_INDEX], // 4: rule 1 stop
    ];
    for state in states {
        words.extend_from_slice(&state);
    }
    let transitions: [[u32; 5]; 5] = [
        // kind, target, arg0, arg1, arg2
        [3, 3, 1, 1, 0], // 0 -> rule 1 (start 3), follow 1, precedence 0
        [5, 2, 7, 0, 0], // 1 -> atom 7 -> 2
        [1, 1, 0, 0, 0], // 2 -> 1 (packed rule-return edge, skipped)
        [1, 4, 0, 0, 0], // 3 -> 4
        [1, 1, 0, 0, 0], // 4 -> 1 (packed rule-return edge, skipped)
    ];
    for transition in transitions {
        words.extend_from_slice(&transition);
    }
    words.extend_from_slice(&[0, 3]); // rule starts
    words.extend_from_slice(&[2, 4]); // rule stops
    words
}

const EXPECTED_DUMP: &str = "\
atn grammar_type=PARSER max_token_type=7
rule 0: start=0 stop=2
rule 1: start=3 stop=4
state 0: RuleStart rule=0 stop=2 left_recursive=false
  -> RULE target=3 rule=1 follow=1 precedence=0
state 1: Basic rule=0
  -> ATOM target=2 label=7
state 2: RuleStop rule=0
state 3: RuleStart rule=1 stop=4 left_recursive=false
  -> EPSILON target=4
state 4: RuleStop rule=1
  -> EPSILON target=1
";

#[test]
fn deserializes_minimal_atn() {
    let atn = PackedATNDeserializer::new()
        .deserialize(&sample_words())
        .expect("packed ATN deserializes");
    assert_eq!(dump_atn(&atn), EXPECTED_DUMP);
}

#[test]
fn rejects_bad_magic() {
    let mut words = sample_words();
    words[0] = 0;
    let err = PackedATNDeserializer::new()
        .deserialize(&words)
        .unwrap_err();
    assert_eq!(err, PackedATNError::BadMagic { found: 0 });
}

#[test]
fn rejects_unsupported_version() {
    let mut words = sample_words();
    words[1] = 2;
    let err = PackedATNDeserializer::new()
        .deserialize(&words)
        .unwrap_err();
    assert_eq!(err, PackedATNError::UnsupportedVersion { found: 2 });
}

#[test]
fn rejects_truncated_stream() {
    let words = sample_words();
    let err = PackedATNDeserializer::new()
        .deserialize(&words[..20])
        .unwrap_err();
    assert_eq!(
        err,
        PackedATNError::TruncatedStream {
            expected_at_least: 29,
            actual: 20
        }
    );
}

#[test]
fn rejects_total_length_mismatch() {
    let mut words = sample_words();
    words.pop();
    let err = PackedATNDeserializer::new()
        .deserialize(&words)
        .unwrap_err();
    assert_eq!(
        err,
        PackedATNError::TotalLengthMismatch {
            declared: 93,
            actual: 92
        }
    );
}

#[test]
fn rejects_invalid_state_kind() {
    let mut words = sample_words();
    words[29] = 13;
    let err = PackedATNDeserializer::new()
        .deserialize(&words)
        .unwrap_err();
    assert_eq!(err, PackedATNError::InvalidStateKind { value: 13 });
}

#[test]
fn rejects_out_of_bounds_transition_target() {
    let mut words = sample_words();
    words[64 + 1] = 9; // transition 0 target
    let err = PackedATNDeserializer::new()
        .deserialize(&words)
        .unwrap_err();
    assert_eq!(
        err,
        PackedATNError::IndexOutOfBounds {
            what: "transition target",
            index: 9,
            count: 5
        }
    );
}
