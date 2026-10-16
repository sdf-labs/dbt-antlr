The following needs to pass on every commit:
- cargo test --workspace
- cargo fmt
- cargo clippy --workspace
- Full e2e testsuite: cargo run -p runtime-testsuite

## Snapshot tests

Snapshot baselines must be identical on every machine and platform:
- Never snapshot absolute/temp paths — invoke tools with relative paths and a
  set working directory
- Don't normalize snapshots by string-replacing paths (Windows separators + JSON
  escaping break it silently)
- Watch for layout-dependent rendering (miette's fixed-width wrapping makes wrap
  points path-length-dependent)
