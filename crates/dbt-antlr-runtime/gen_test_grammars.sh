#!/usr/bin/env bash
# Regenerate tests/gen/ with the Rust codegen (dbt-antlr-codegen).
set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
GEN_BIN="$WORKSPACE_ROOT/target/debug/dbt-antlr-codegen"

cargo build --quiet --manifest-path "$WORKSPACE_ROOT/Cargo.toml" \
    -p dbt-antlr-codegen --bin dbt-antlr-codegen

declare -a GRAMMARS=(
    "VisitorBasic"
    "VisitorCalc"
    "CSV"
    "ReferenceToATN"
    "XMLLexer"
    "lexer/Spike"
    "SimpleLR"
    "Labels"
#    "FHIRPath"
    "Perf" # no committed goldens; verified against a fresh Java oracle run
)

declare -a ADDITIONAL_ARGS=(
    "-visitor"
    "-visitor"
    "-visitor"
    ""
    ""
    ""
    ""
    ""
    ""
)

for i in "${!GRAMMARS[@]}"; do
    grammar="${GRAMMARS[$i]}"
    arg="${ADDITIONAL_ARGS[$i]}"
    file_name="${grammar}.g4"

    cmd=("$GEN_BIN" -o "$SCRIPT_DIR/tests/gen" "$SCRIPT_DIR/grammars/$file_name")
    if [[ -n "$arg" ]]; then
        cmd+=("$arg")
    fi

    echo "Generating: $grammar"
    "${cmd[@]}"
done

# `--static-lexer-dfa` lexer twins. Spike.g4 is generated plain above; CSV and
# XMLLexer get a static variant next to their plain golden. The CLI writes
# every recognizer of the grammar into the output directory, so the static
# runs go to a scratch dir and only the lexer file is kept, renamed with the
# `_static` suffix. The warnings of the escape-edge witness (Spike) go
# to stderr as usual.
declare -a STATIC_GRAMMARS=(
    "CSV"
    "XMLLexer"
    "lexer/Spike"
)
# Emitted lexer file stem per grammar: the implicit lexer of a combined
# grammar is `<name>Lexer`, a lexer grammar emits under its own name.
declare -a STATIC_LEXER_NAMES=(
    "csvlexer"
    "xmllexer"
    "spike"
)

STATIC_TMP="$(mktemp -d)"
trap 'rm -rf "$STATIC_TMP"' EXIT

for i in "${!STATIC_GRAMMARS[@]}"; do
    grammar="${STATIC_GRAMMARS[$i]}"
    lexer="${STATIC_LEXER_NAMES[$i]}"
    echo "Generating: ${lexer}_static"
    "$GEN_BIN" --static-lexer-dfa -o "$STATIC_TMP" "$SCRIPT_DIR/grammars/$grammar.g4"
    mv "$STATIC_TMP/${lexer}.rs" "$SCRIPT_DIR/tests/gen/${lexer}_static.rs"
done

# `--static-parser-dfa` parser witnesses. StaticDFA and LrDfa are generated
# directly with `--static-parser-dfa`; FuzzExpr is generated both adaptive and
# with `--static-parser-dfa` (`fuzzexprparser_static.rs`) for differential
# fuzzing.
for grammar in "parser/StaticDFA" "parser/LrDfa"; do
    echo "Generating: $grammar (--static-parser-dfa)"
    "$GEN_BIN" -no-listener --static-parser-dfa -o "$SCRIPT_DIR/tests/gen" "$SCRIPT_DIR/grammars/${grammar}.g4"
done

echo "Generating: parser/FuzzExpr"
"$GEN_BIN" -no-listener -o "$SCRIPT_DIR/tests/gen" "$SCRIPT_DIR/grammars/parser/FuzzExpr.g4"
echo "Generating: fuzzexprparser_static"
"$GEN_BIN" -no-listener --static-parser-dfa -o "$STATIC_TMP" "$SCRIPT_DIR/grammars/parser/FuzzExpr.g4"
mv "$STATIC_TMP/fuzzexprparser.rs" "$SCRIPT_DIR/tests/gen/fuzzexprparser_static.rs"
