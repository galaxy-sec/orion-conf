# Repository Guidelines

## Project Structure & Module Organization
- Crate: `orion_conf` (Rust library).
- `src/`: `traits.rs` (core traits: `ConfigIO`, `YamlIO/TomlIO/JsonIO/IniIO`), `persist.rs` (file IO + feature gates), `error.rs` (typed errors).
- `examples/`: runnable demos of format behavior and priority.
- `.github/workflows/`: CI pipelines. `target/`: build artifacts.
- Tests are colocated with code via `#[cfg(test)]` and feature gates.

## Build, Test, and Development Commands
- Build (no formats): `cargo build`
- Build with formats: `cargo build --features formats` (or `--features full`)
- Run examples:
  - JSON only: `cargo run --example json_feature_test --features json`
  - YAML+JSON priority: `cargo run --example feature_priority_test --features yaml,json`
- Lint/format: `cargo fmt --all`; `cargo clippy --all-targets --all-features -- -D warnings`
- Test: `cargo test` (unit tests behind format features); full matrix: `cargo test --all-features -- --test-threads=1`

## Coding Style & Naming Conventions
- `rustfmt` default (4-space indent); keep clippy clean (`-D warnings`).
- Naming: `snake_case` for fns/modules, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts.
- Library code must avoid `panic!`/`unwrap()`; propagate `StorageError`/`SerdeError`.

## Testing Guidelines
- Prefer unit tests next to code (`#[cfg(test)]`); gate format-specific tests with the feature that enables the format.
- Use `tempfile::NamedTempFile` for filesystem safety; keep tests deterministic.
- Optional coverage (matches CI): `cargo llvm-cov --all-features --workspace -- --test-threads=1`

## Commit & Pull Request Guidelines
- Commits: short, imperative subject; group logical changes (e.g., "Refactor file loading logic", "Add storage extension traits").
- PRs: state purpose, scope, feature flags exercised, and before/after behavior; link issues; CI must pass (fmt, clippy, tests).
- Update or add examples when changing feature priority or format behavior.

## Security & Configuration Tips
- Formats are opt-in; default features are empty. Enable only what you need (e.g., `--features yaml`).
- Validate caller-provided paths; do not assume existence.

## Versioning & Release
- Bump version in `version.txt` and `Cargo.toml`; tag `vX.Y.Z` to trigger publish workflow.
- Optional Galaxy helpers in `.run.gxl` (e.g., `up_patch_ci`, `tag_stable`).
