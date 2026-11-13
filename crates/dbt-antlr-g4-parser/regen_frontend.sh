#!/usr/bin/env bash
# Regenerate src/generated/ with the Rust codegen (dbt-antlr-codegen).
# The checked-in recognizers are the bootstrap fixpoint: rerunning this
# script must reproduce them with zero diff. The base listener is emitted for
# Java-tool parity but is not part of the compiled facade, and the
# per-grammar `*.optimizations.txt` manifests are build reports, so neither is
# checked in.
#
# The front-end dogfoods the static DFA tables (7.1): the lexer generated
# from ANTLRv4Lexer.g4 walks static lexer tables plus the lazy simulator for
# its escape edges, and the parser generated from ANTLRv4Parser.g4 predicts
# its decisions from static parser tables.
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEN_BIN="$WORKSPACE_ROOT/target/debug/dbt-antlr-codegen"

cargo build --quiet --manifest-path "$WORKSPACE_ROOT/Cargo.toml" \
    -p dbt-antlr-codegen --bin dbt-antlr-codegen

echo "Generating: ANTLRv4Lexer (--static-lexer-dfa)"
"$GEN_BIN" --static-lexer-dfa -o "$SCRIPT_DIR/src/generated" \
    "$SCRIPT_DIR/grammars/ANTLRv4Lexer.g4"

echo "Generating: ANTLRv4Parser (--static-parser-dfa)"
"$GEN_BIN" --static-parser-dfa -o "$SCRIPT_DIR/src/generated" \
    "$SCRIPT_DIR/grammars/ANTLRv4Parser.g4"

rm -f "$SCRIPT_DIR/src/generated/antlrv4parserbaselistener.rs" \
    "$SCRIPT_DIR/src/generated/antlrv4lexer.optimizations.txt" \
    "$SCRIPT_DIR/src/generated/antlrv4parser.optimizations.txt"
