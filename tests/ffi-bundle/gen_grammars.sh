#!/usr/bin/env bash
# Regenerate src/generated/ and python/ with the Rust codegen (dbt-antlr-codegen).
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEN_BIN="$WORKSPACE_ROOT/target/debug/dbt-antlr-codegen"
RUNTIME_GRAMMARS="$WORKSPACE_ROOT/crates/dbt-antlr-runtime/grammars"

cargo build --quiet --manifest-path "$WORKSPACE_ROOT/Cargo.toml" \
    -p dbt-antlr-codegen --bin dbt-antlr-codegen

rm -f "$SCRIPT_DIR"/src/generated/*.rs "$SCRIPT_DIR"/python/*_parser.py

generate() {
    echo "Generating: $(basename "$1")"
    "$GEN_BIN" -no-listener -o "$SCRIPT_DIR/src/generated" "$@"
}

generate "$RUNTIME_GRAMMARS/CSV.g4" --python-out "$SCRIPT_DIR/python"
generate "$RUNTIME_GRAMMARS/Labels.g4" --python-out "$SCRIPT_DIR/python"
generate "$SCRIPT_DIR/grammars/KeywordsLexer.g4"
generate "$SCRIPT_DIR/grammars/KeywordsParser.g4" --python-out "$SCRIPT_DIR/python"
