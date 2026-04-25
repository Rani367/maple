# Phase 03: Graph And Ranking

## Goal

Turn extracted facts into an architecture map that highlights what matters.

## Deliverables

- File dependency graph.
- Package graph.
- Symbol graph.
- Ranking algorithm.
- Reading path.
- Cycle and hotspot detection.

## Implementation Tasks

1. Add graph model with stable node IDs.
2. Add edges:
   - file imports file
   - symbol references symbol
   - file belongs to package
   - test covers source file
   - config controls package/app
3. Add ranking signals:
   - import fan-in
   - import fan-out
   - entrypoint boost
   - package manifest boost
   - README/docs boost
   - git activity boost
4. Add explainable ranking:
   - every ranked item has reasons
   - no black-box scores in user-facing output
5. Add cycle detection and cluster detection.
6. Generate a reading path from entrypoints to core modules.

## Acceptance Criteria

- `repoprism map .` shows important files and why.
- Cycles are detected in fixture repos.
- Reading path is stable and plausible.
- Ranking output is useful on `repoprism` itself and `affected`.

