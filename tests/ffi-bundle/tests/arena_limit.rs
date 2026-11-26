//! The arena allocation limit of `antlr_set_arena_allocation_limit`.
//!
//! The limit is global, so this test runs in its own process.

use dbt_antlr_ffi::{
    ANTLR_PARSE_STATUS_FATAL, ANTLR_PARSE_STATUS_OK, antlr_grammar_lookup, antlr_parse,
    antlr_session_free, antlr_session_info, antlr_set_arena_allocation_limit,
};
use dbt_antlr_ffi_test_bundle::GRAMMAR_NAMES;

fn parse_status(input: &str) -> (i32, String) {
    let name = GRAMMAR_NAMES[0];
    // SAFETY: `name` and `input` are readable, and `input` outlives the
    // session.
    unsafe {
        let session = antlr_parse(
            antlr_grammar_lookup(name.as_ptr().cast(), name.len()),
            0,
            input.as_ptr().cast(),
            input.len(),
        );
        let info = &*antlr_session_info(session);
        let message = info.message.as_str().unwrap_or_default().to_owned();
        let status = info.status;
        antlr_session_free(session);
        (status, message)
    }
}

#[test]
fn arena_allocation_limit() {
    let input = "a,b,c\n".repeat(100);
    let supported = antlr_set_arena_allocation_limit(1024);
    assert_eq!(supported, cfg!(feature = "arena-allocation-limit"));
    if !supported {
        return;
    }
    let (status, message) = parse_status(&input);
    assert_eq!(status, ANTLR_PARSE_STATUS_FATAL);
    assert!(message.contains("limit"), "{message}");

    assert!(antlr_set_arena_allocation_limit(0));
    assert_eq!(parse_status(&input).0, ANTLR_PARSE_STATUS_OK);
}
