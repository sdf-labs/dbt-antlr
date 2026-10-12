#!/usr/bin/env bash
# Regenerate src/generated/ with the Rust codegen (dbt-antlr-codegen).
# The checked-in recognizers are the step-6.5 bootstrap fixpoint: rerunning
# this script must reproduce them with zero diff. The base listener is
# emitted for Java-tool parity but is not part of the compiled facade, so it
# is not checked in.
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEN_BIN="$WORKSPACE_ROOT/target/debug/dbt-antlr-codegen"

cargo build --quiet --manifest-path "$WORKSPACE_ROOT/Cargo.toml" \
    -p dbt-antlr-codegen --bin dbt-antlr-codegen

for grammar in ANTLRv4Lexer ANTLRv4Parser; do
    echo "Generating: $grammar"
    "$GEN_BIN" -o "$SCRIPT_DIR/src/generated" "$SCRIPT_DIR/grammars/$grammar.g4"
done

rm -f "$SCRIPT_DIR/src/generated/antlrv4parserbaselistener.rs"

# The emitted check_version! call stamps the runtime major/minor at
# generation time, where the minor is a compatibility floor. Pin the checked
# in recognizers to the current major with a zero minor floor so that minor
# releases do not break the fixpoint check. Revisit when the major version
# changes.
MAJOR="$(sed -n 's/^version = "\([0-9]*\)\..*/\1/p' "$WORKSPACE_ROOT/Cargo.toml" | head -1)"
perl -pi -e 's/check_version!\("[0-9]+","[0-9]+"\)/check_version!("'"$MAJOR"'","0")/' \
    "$SCRIPT_DIR"/src/generated/*.rs
