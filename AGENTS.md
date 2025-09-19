# Repository Guidelines

## Project Structure & Module Organization
- Rust crate: `orion_conf` (library). Key modules in `src/`: `traits.rs` (core traits), `persist.rs` (file IO + feature gates), `error.rs` (typed errors). 
- Examples in `examples/` demonstrate feature behavior and priorities.
- CI/QA lives in `.github/workflows/`. Build artifacts in `target/`.

## Build, Test, and Development Commands
- Build (no formats enabled by default): `cargo build`
- Build with formats: `cargo build --features formats` or `--features full`
- Run examples:
  - JSON only: `cargo run --example json_feature_test --features json`
  - YAML+JSON priority: `cargo run --example feature_priority_test --features yaml,json`
- Lint/format: `cargo fmt --all`, `cargo clippy --all-targets --all-features -- -D warnings`
- Test: `cargo test` (unit tests behind format features), `cargo test --all-features -- --test-threads=1`

## Coding Style & Naming Conventions
- Rust style via rustfmt (4‑space indent). Keep clippy clean (`-D warnings`).
- Naming: `snake_case` for functions/modules, `CamelCase` for types/traits, `SCREAMING_SNAKE_CASE` for consts.
- Features: use existing names `yaml`, `toml`, `json`, `ini`; combos: `formats`, `full`. Do not introduce overlapping feature names without discussion.

## Testing Guidelines
- Prefer unit tests colocated with code (`#[cfg(test)]`). Add feature‑gated tests where format‑specific.
- Keep tests deterministic and file‑system safe (use `tempfile::NamedTempFile`).
- Coverage (optional, matches CI): `cargo llvm-cov --all-features --workspace -- --test-threads=1`.

## Commit & Pull Request Guidelines
- Commits: short imperative subject, optional body (e.g., "Refactor file loading logic", "Add storage extension traits"). Group logical changes.
- PRs: include purpose, scope, feature flags exercised, and before/after behavior. Link issues. CI must pass (fmt, clippy, tests). Add/adjust examples when changing feature priority or format behavior.

## Security & Configuration Tips
- Formats are opt‑in; default features are empty. Enable only required formats to reduce attack surface: e.g., `--features yaml`.
- Avoid `panic!`/`unwrap()` in library code; propagate `StorageError`/`SerdeError`.
- Validate paths provided by callers; do not assume existence.

## Versioning & Release
- Bump version in `version.txt` (and `Cargo.toml`) and tag `vX.Y.Z` to trigger publish workflow.
- Optional internal flow: `.run.gxl` provides helpers (e.g., `up_patch_ci`, `tag_stable`) if you use the Galaxy tooling.
