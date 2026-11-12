//! Tests for the LL(1) lookahead of `ATN::next_tokens_in_ctx` on grammars
//! whose call graph doubles at each level.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use dbt_antlr_runtime::atn::ATN;
use dbt_antlr_runtime::atn_packed_deserializer::PackedATNDeserializer;
use dbt_antlr_runtime::rule_context::EmptyNodeKind;
use dbt_antlr_runtime::token::{CommonToken, TOKEN_EPSILON};

const A: i32 = 1;
const P: i32 = 2;
const X: i32 = 3;
const Y: i32 = 4;

/// Compiles `s : r0 X | r0 Y ; r<i> : r<i+1> | r<i+1> P ; r<depth> : <last> ;`.
fn ladder(name: &str, depth: usize, last: &str) -> ATN {
    let mut grammar = format!("grammar {name};\ns : r0 X | r0 Y ;\n");
    for level in 0..depth {
        let next = level + 1;
        grammar.push_str(&format!("r{level} : r{next} | r{next} P ;\n"));
    }
    grammar.push_str(&format!(
        "r{depth} : {last} ;\nA : 'a' ;\nP : 'p' ;\nX : 'x' ;\nY : 'y' ;\n"
    ));

    let directory =
        std::env::temp_dir().join(format!("dbt-antlr-ll1-{}-{name}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("temp directory");
    let path: PathBuf = directory.join(format!("{name}.g4"));
    std::fs::write(&path, grammar).expect("grammar file");
    let units = dbt_antlr_codegen::atn_export::compile_atn_data(&path, &[]);
    let _ = std::fs::remove_dir_all(&directory);
    let units = units.expect("compiles");
    let words = units
        .iter()
        .find_map(|unit| unit.parser_packed_words.clone())
        .expect("parser unit");
    PackedATNDeserializer::new()
        .deserialize(&words)
        .expect("packed ATN deserializes")
}

fn rule_look(atn: &ATN, rule: usize) -> Vec<i32> {
    let tokens = atn
        .next_tokens_in_ctx::<EmptyNodeKind, CommonToken<'_>>(atn.rule_to_start_state[rule], None);
    tokens
        .as_interval_set()
        .into_iter()
        .flat_map(|interval| interval.a..=interval.b)
        .collect()
}

#[test]
fn doubling_ladder_lookahead_is_not_exponential() {
    let atn = ladder("Ladder", 40, "A");
    let started = Instant::now();
    assert_eq!(rule_look(&atn, 0), [A]);
    assert_eq!(rule_look(&atn, 1), [A]);
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_secs(5),
        "lookahead took {elapsed:?}"
    );
}

#[test]
fn nullable_ladder_continues_after_each_call() {
    let atn = ladder("NullableLadder", 40, "A |");
    let started = Instant::now();
    assert_eq!(rule_look(&atn, 0), [A, P, X, Y]);
    assert_eq!(rule_look(&atn, 1), [TOKEN_EPSILON, A, P]);
    assert_eq!(rule_look(&atn, 41), [TOKEN_EPSILON, A]);
    let elapsed = started.elapsed();
    assert!(
        elapsed < Duration::from_secs(5),
        "lookahead took {elapsed:?}"
    );
}
