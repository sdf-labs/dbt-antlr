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

# The guarded-take witness is generated both ways: its static table carries
# stack-guarded takes, so the differential test parses the same corpus with the
# adaptive twin.
echo "Generating: parser/GuardExpr"
"$GEN_BIN" -no-listener -o "$SCRIPT_DIR/tests/gen" "$SCRIPT_DIR/grammars/parser/GuardExpr.g4"
echo "Generating: guardexprparser_static"
"$GEN_BIN" -no-listener --static-parser-dfa -o "$STATIC_TMP" "$SCRIPT_DIR/grammars/parser/GuardExpr.g4"
mv "$STATIC_TMP/guardexprparser.rs" "$SCRIPT_DIR/tests/gen/guardexprparser_static.rs"

# The `--static-dfa-only` twin of the guarded-take witness: its guarded takes
# and the states at its lookahead budget predict alternative 1 (the take), so
# the test pins the inputs where the static-only parser differs from the
# adaptive one.
echo "Generating: guardexprparser_static_only"
"$GEN_BIN" -no-listener --static-dfa-only -o "$STATIC_TMP" "$SCRIPT_DIR/grammars/parser/GuardExpr.g4"
mv "$STATIC_TMP/guardexprparser.rs" "$SCRIPT_DIR/tests/gen/guardexprparser_static_only.rs"

# The full-context witness: a call of `send` as the parse entry predicts its
# operator loop with an empty outer context (tests/full_context.rs).
echo "Generating: parser/LoopVeto"
"$GEN_BIN" -no-listener -o "$SCRIPT_DIR/tests/gen" "$SCRIPT_DIR/grammars/parser/LoopVeto.g4"

# The ownership and guarded-iterate witnesses are generated both ways.
for grammar in "LoopFrontier" "LoopSibling" "LoopSubst" "PostfixSibling"; do
    lower="$(echo "$grammar" | tr '[:upper:]' '[:lower:]')"
    echo "Generating: parser/$grammar"
    "$GEN_BIN" -no-listener -o "$SCRIPT_DIR/tests/gen" "$SCRIPT_DIR/grammars/parser/$grammar.g4"
    echo "Generating: ${lower}parser_static"
    "$GEN_BIN" -no-listener --static-parser-dfa -o "$STATIC_TMP" "$SCRIPT_DIR/grammars/parser/$grammar.g4"
    mv "$STATIC_TMP/${lower}parser.rs" "$SCRIPT_DIR/tests/gen/${lower}parser_static.rs"
done
