//! Compile-time version-gate semantics for generated code.

use dbt_antlr_runtime::version_str_to_u64;

#[test]
fn version_str_to_u64_parses_numeric_components() {
    assert_eq!(version_str_to_u64("0"), 0);
    assert_eq!(version_str_to_u64("1"), 1);
    assert_eq!(version_str_to_u64("9"), 9);
    assert_eq!(version_str_to_u64("10"), 10);
    assert_eq!(version_str_to_u64("123"), 123);
}

/// The runtime must accept generated code stamped with its own major version
/// and any minor floor up to its own minor. The major literal must be bumped
/// by hand when the crate's major version changes.
#[test]
fn check_version_accepts_same_major_and_minor_floor() {
    dbt_antlr_runtime::check_version!("0", "0");
    dbt_antlr_runtime::check_version!("0", "1");
}
