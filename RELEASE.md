# Releasing dbt-antlr

This workspace ships three crates with independent versions:

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
3. A `dbt-antlr-codegen-vX.Y.Z` tag additionally triggers the **Release** workflow
   (cargo-dist): it builds the `dbt-antlr-codegen` binary for the five platform
   targets and attaches archives, checksums, and shell/powershell installers
   to the GitHub release.

Library tags deliberately do not trigger binary builds (see the MANUAL EDIT
comment in `.github/workflows/release.yml`; re-apply it after any
`dist generate` run, e.g. on cargo-dist version upgrades).

## Tag pushes do not start the binary workflow

release-plz pushes release tags with the workflow's `GITHUB_TOKEN`, and
GitHub does not let events from `GITHUB_TOKEN` trigger other workflows. So a
`dbt-antlr-codegen-vX.Y.Z` tag created by release-plz does **not** start the
Release (cargo-dist) workflow, and the GitHub release stays without binaries.
Two ways to get the binaries built:

- Manual (current setup): after release-plz publishes a tool release, re-push
  the tag from any checkout with your own credentials:

  ```sh
  git push origin :refs/tags/dbt-antlr-codegen-vX.Y.Z
  git push origin dbt-antlr-codegen-vX.Y.Z
  ```

  cargo-dist attaches the binaries to the existing GitHub release.
- Automatic: store a personal access token (repository contents: write) as a
  `RELEASE_PLZ_TOKEN` secret and set `GITHUB_TOKEN: ${{ secrets.RELEASE_PLZ_TOKEN }}`
  in `release-plz.yml`. Tag pushes from release-plz then trigger the Release
  workflow like a human push.

## Generated-code compatibility

Generated parsers embed `check_version!("<major>", "<minor>")` naming the
runtime version the tool was built against. The gate accepts any runtime with
the same major and a minor >= the generated one, so additive runtime releases
never break previously generated code. Only bump the runtime's minor in a way
generated code depends on (new APIs the templates emit against) together with
a tool release; the template renders the version from the runtime the tool
was built against, so no manual template edits are needed.
