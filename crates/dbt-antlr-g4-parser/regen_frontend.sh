#!/usr/bin/env bash
# Regenerate src/generated/ with the Rust codegen (dbt-antlr).
# The checked-in recognizers are the step-6.5 bootstrap fixpoint: rerunning
# this script must reproduce them with zero diff. The base listener is
# emitted for Java-tool parity but is not part of the compiled facade, so it
# is not checked in.
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEN_BIN="$WORKSPACE_ROOT/target/debug/dbt-antlr"

cargo build --quiet --manifest-path "$WORKSPACE_ROOT/Cargo.toml" \
    -p dbt-antlr-codegen --bin dbt-antlr

for grammar in ANTLRv4Lexer ANTLRv4Parser; do
    echo "Generating: $grammar"
    "$GEN_BIN" -o "$SCRIPT_DIR/src/generated" "$SCRIPT_DIR/grammars/$grammar.g4"
done

rm -f "$SCRIPT_DIR/src/generated/antlrv4parserbaselistener.rs"
