//! Makes sure that the checked-in copies of `antlr.h` agree with `src/ffi`.
//!
//! To update the copies, run:
//!
//! ```text
//! DBT_ANTLR_FFI_UPDATE_HEADER=1 cargo test -p dbt-antlr-runtime --features ffi --test ffi_header
//! ```

use std::path::{Path, PathBuf};

const UPDATE_ENV_VAR: &str = "DBT_ANTLR_FFI_UPDATE_HEADER";

fn crate_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The checked-in copies of the header: the one of this crate and the one of
/// the `dbt_antlr` Python package.
fn header_copies() -> [PathBuf; 2] {
    let crate_dir = crate_dir();
    [
        crate_dir.join("include/antlr.h"),
        crate_dir.join("../../python/dbt_antlr/antlr.h"),
    ]
}

fn generate_header() -> String {
    let crate_dir = crate_dir();
    let config = cbindgen::Config::from_file(crate_dir.join("cbindgen.toml"))
        .expect("cbindgen.toml is valid");
    let bindings = cbindgen::Builder::new()
        .with_config(config)
        .with_src(crate_dir.join("src/ffi/mod.rs"))
        .generate()
        .expect("cbindgen generates the header");
    let mut header = Vec::new();
    bindings.write(&mut header);
    String::from_utf8(header).expect("the header is UTF-8")
}

fn check_copy(path: &Path, expected: &str) -> Result<(), String> {
    let actual = std::fs::read_to_string(path).unwrap_or_default();
    if actual == expected {
        return Ok(());
    }
    if std::env::var_os(UPDATE_ENV_VAR).is_some() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|error| format!("{}: {error}", parent.display()))?;
        }
        std::fs::write(path, expected).map_err(|error| format!("{}: {error}", path.display()))?;
        return Ok(());
    }
    Err(format!(
        "{} is out of date; run `{UPDATE_ENV_VAR}=1 cargo test -p dbt-antlr-runtime --features ffi --test ffi_header`",
        path.display()
    ))
}

#[test]
fn header_copies_are_up_to_date() {
    let expected = generate_header();
    let failures: Vec<String> = header_copies()
        .iter()
        .filter_map(|path| check_copy(path, &expected).err())
        .collect();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn header_declares_every_entry_point() {
    let header = generate_header();
    for function in [
        "antlr_abi_version",
        "antlr_grammar_lookup",
        "antlr_grammar_info",
        "antlr_parse",
        "antlr_node_to_string_tree",
        "antlr_session_info",
        "antlr_session_free",
        "antlr_token_location",
        "antlr_token_text_fallback",
        "antlr_string_free",
        "antlr_set_arena_allocation_limit",
    ] {
        assert!(
            header.contains(&format!("{function}(")),
            "antlr.h does not declare {function}"
        );
    }
}
