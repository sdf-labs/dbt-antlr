# Vendored runtime conformance corpus

`cargo run -p runtime-testsuite` runs every case in this tree. The tree is a
one-time import of the ANTLR 4 runtime conformance corpus, post-rendered to
Rust, so the sweep needs no JVM, no tool jar, and no network access.

## Layout

One file per case: `<Group>/<Name>.txt`, in the ANTLR runtime-testsuite
descriptor format. `[grammar]` and `[slaveGrammar]` hold the final Rust
grammar. `[type]`, `[input]`, `[output]`, `[errors]`, `[flags]`, `[skip]`,
`[start]` and `[notes]` are unchanged from upstream. The `#` preamble carries
provenance; the parser ignores text before the first section.

## Provenance

| | |
|---|---|
| Source release | `dbt-antlr4-2.0.0` (`sdf-labs/antlr4`, the fork this project succeeds) |
| Source path | `runtime-testsuite/resources/org/antlr/v4/test/runtime/descriptors/` |
| Source zip SHA-256 | `62583dc78e0d7f698bfe16331efd7f1e33f86df59d5bef73dfd31625756f322f` |
| Template group | `resources/org/antlr/v4/test/runtime/templates/Rust.test.stg` (70 definitions) |
| Renderer | StringTemplate 4.3.4 as bundled in `dbt-antlr4-2.0.0-complete.jar` |
| Import command | `cargo run -p runtime-testsuite -- --export-corpus tests/runtime-testsuite/corpus` |

The importer renders with a JVM and the pinned artifacts. The upstream corpus
and template are frozen, and `sdf-labs/antlr4` receives no updates, so a
re-import is only needed when this corpus is refreshed from another source.

## What the import changed

- `[grammar]` and `[slaveGrammar]` bodies hold the post-render text. Tags such
  as `<writeln("$A.text")>` became Rust actions, and the StringTemplate
  escapes `\<`, `\>` and `\\` were collapsed.
- Every other section is byte-identical to upstream.

## Editing

Edit these files directly. Grammar bodies are plain `.g4` files with Rust
actions, and the corpus is ours.

A `# Local deltas:` header line marks the files where the import retargeted
upstream's rendered text to the current runtime API. The ids in that line name
these rewrites:

| id | rewrite |
|---|---|
| `runtime-crate-name` | `dbt_antlr4` to `dbt_antlr_runtime` |
| `token-start-column-getter` | `recog.token_start_column` to `recog.get_token_start_column()` |
| `lexer-token-start-column` | `this.token_start_column` to `0` |
| `lexer-token-start-line` | `this.token_start_line` to `0` |
| `simulator-set-line` | drop `self.set_line(line);` |
| `simulator-set-char-position` | drop `self.set_char_position_in_line(charPositionInLine);` |
| `lexer-input-field` | `let mut input = this.input.take().unwrap();` to `let input = &mut this.input;` |
| `lexer-interpreter-field` | `this.interpreter.as_mut().unwrap().resetAcceptPosition(&mut input,` to `this.interpreter.resetAcceptPosition(input,` |
| `lexer-input-restore` | drop `this.input = Some(input);` |
| `simulator-reset-accept-position` | `resetAcceptPosition(&mut self` to `resetAcceptPosition(&self` |

`../src/custom_descriptors.rs` holds the five cases that the descriptor format
cannot express: exact CR/LF byte pairs, and loop-generated scale tests.
