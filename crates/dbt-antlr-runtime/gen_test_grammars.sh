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

# The emitted check_version! call stamps the runtime major/minor at
# generation time, where the minor is a compatibility floor. Pin goldens to
# the current major with a zero minor floor so that minor releases do not
# churn the goldens. Revisit when the major version changes.
MAJOR="$(sed -n 's/^version = "\([0-9]*\)\..*/\1/p' "$WORKSPACE_ROOT/Cargo.toml" | head -1)"
perl -pi -e 's/check_version!\("[0-9]+","[0-9]+"\)/check_version!("'"$MAJOR"'","0")/' \
    "$SCRIPT_DIR"/tests/gen/*.rs
