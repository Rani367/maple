# Releasing

Maple does not publish release binaries yet. Phase 00 keeps release automation
as notes so the project can add it deliberately in Phase 08.

## Intended Release Targets

- macOS ARM64
- macOS x64
- Linux x64
- Windows x64

## Future Workflow Shape

The release workflow should eventually:

1. Trigger from version tags such as `v0.1.0`.
2. Run `cargo fmt --check`, `cargo clippy --all-targets --locked -- -D warnings`, and `cargo test --all-targets --locked`.
3. Build platform binaries for each target.
4. Package archives with the `maple` binary, README, and license.
5. Attach artifacts and generated checksums to the GitHub release.

## Local Preflight

Before cutting any release candidate, run:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo run -- scan .
cargo run -- scan . --json
```

Binary publishing, Homebrew packaging, and cross-platform installer polish are
Phase 08 work.
