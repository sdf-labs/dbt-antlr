# Releasing dbt-antlr

This workspace ships three crates in lock-step from one shared `workspace.package` version:

| Crate                 | What it is                                                           | Cadence                   |
|-----------------------+----------------------------------------------------------------------+---------------------------|
| `dbt-antlr-runtime`   | Library that generated parsers compile against                       | Frequent                  |
| `dbt-antlr-codegen`   | The generator: `dbt-antlr-codegen` binary + `dbt_antlr_codegen` build-script library | Occasional    |
| `dbt-antlr-g4-parser` | Internal grammar front-end of the generator                          | Rides along with the tool |

`tests/runtime-testsuite` and `tests/build-generate` are `publish = false`.

## Setup (one time)

Add a `CARGO_REGISTRY_TOKEN` repository secret: a crates.io API token with
publish rights to the three crates. For the first publish, create the crates
on crates.io first (publishing `dbt-antlr-runtime 0.1.0` once by hand with
`cargo publish -p dbt-antlr-runtime` is the simplest way — the dependency
graph is acyclic, so no bootstrap hacks are needed; order is runtime, then
g4-parser, then dbt-antlr-codegen).

## How a release works

1. Every push to `main` runs the **Release-plz** workflow, which opens or
   refreshes one release PR per crate with unreleased changes (version bump
   inferred from commit messages; conventional commits recommended, e.g.
   `feat:`, `fix:`).
2. Merging a release PR publishes that crate to crates.io in dependency
   order, tags the release commit (`dbt-antlr-runtime-vX.Y.Z`,
   `dbt-antlr-g4-parser-vX.Y.Z`, or `dbt-antlr-codegen-vX.Y.Z`), and creates the
   GitHub release.
Binary distribution is currently disabled; see below.

## Binary distribution (currently disabled)

cargo-dist binary builds are disabled while the tool side stabilizes: the
generated `.github/workflows/release.yml` was removed, so releases are
crates.io packages, git tags, and GitHub releases only. `dist-workspace.toml`
and `[profile.dist]` stay in the repo as inert config so re-enabling is cheap.

The `dbt-antlr-codegen-v0.1.0` release keeps its prebuilt archives, so
`Config::pinned_release("0.1.0")` keeps working; later versions have no
archives until distribution is re-enabled.

To re-enable:

1. Restore `.github/workflows/release.yml` from git history, or regenerate it
   with `dist generate`. If regenerating, re-apply the tag-trigger scoping to
   `dbt-antlr-codegen-v[0-9]+.[0-9]+.[0-9]+*` so library tags do not trigger
   binary builds.
2. Store a personal access token (repository contents: write) as a
   `RELEASE_PLZ_TOKEN` secret and set
   `GITHUB_TOKEN: ${{ secrets.RELEASE_PLZ_TOKEN }}` in `release-plz.yml`.
   release-plz pushes tags with the workflow's `GITHUB_TOKEN`, and GitHub does
   not let events from `GITHUB_TOKEN` trigger other workflows. Without the
   PAT, every tool release needs a manual tag re-push to start the binary
   build: `git push origin :refs/tags/dbt-antlr-codegen-vX.Y.Z` followed by
   `git push origin dbt-antlr-codegen-vX.Y.Z`.

## Generated-code compatibility

Generated parsers embed `check_version!("<major>", "<minor>")` naming the
runtime version the tool was built against. The gate accepts any runtime with
the same major and a minor >= the generated one, so additive runtime releases
never break previously generated code. Only bump the runtime's minor in a way
generated code depends on (new APIs the templates emit against) together with
a tool release; the template renders the version from the runtime the tool
was built against, so no manual template edits are needed.
