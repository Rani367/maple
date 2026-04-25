# Phase 00: Foundation

## Goal

Make the repository pleasant to build, test, and extend.

## Deliverables

- Keep the current `repoprism` binary compiling.
- Move CLI parsing into `src/cli.rs`.
- Add `src/config.rs` for defaults and future config files.
- Add `src/error.rs` only if `anyhow` becomes too loose.
- Add test fixtures under `tests/fixtures/`.
- Add GitHub Actions for `cargo fmt`, `cargo clippy`, and `cargo test`.
- Add release workflow notes for later binary builds.

## Implementation Tasks

1. Create modules without changing behavior.
2. Add integration tests for `repoprism scan . --json`.
3. Add sample fixture repos:
   - tiny Rust crate
   - tiny TypeScript app
   - mixed repo with docs/config
4. Add snapshot testing for terminal/JSON output.
5. Add `justfile` or `Makefile` only if it reduces friction.

## Acceptance Criteria

- `cargo fmt --check` passes.
- `cargo clippy -- -D warnings` passes.
- `cargo test` passes.
- `cargo run -- scan .` still works.
- New contributors can understand the module layout in under 5 minutes.

