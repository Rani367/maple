# Phase 08: Quality And Performance

## Goal

Make RepoPrism trustworthy on real repositories.

## Deliverables

- Benchmarks.
- Large repo tests.
- Snapshot tests.
- Performance budget.
- Release binaries.

## Implementation Tasks

1. Add benchmarks for:
   - scan time
   - parse time
   - graph build time
   - memory usage
2. Add fixture repos and golden outputs.
3. Add large repo smoke tests, optional locally:
   - Rust project
   - TS monorepo
   - Python package
4. Add parallel scanning/parsing.
5. Add cancellation and progress reporting.
6. Add release workflow:
   - macOS ARM64
   - macOS x64
   - Linux x64
   - Windows x64
7. Add Homebrew tap instructions.

## Acceptance Criteria

- Small repos scan in seconds.
- Large repos remain responsive.
- Memory usage is bounded and documented.
- Release artifacts install cleanly.
- CI protects the public demo paths.

