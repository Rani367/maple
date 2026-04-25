# Phase 04: Reports And Context Packs

## Goal

Generate artifacts people will actually copy, commit, or feed into tools.

## Deliverables

- `repo-map.md`
- `repo-context.md`
- `repo-context.json`
- token-budgeted context packs
- report profiles

## Implementation Tasks

1. Add report writer abstraction.
2. Implement Markdown repo map:
   - overview
   - commands
   - entrypoints
   - important files
   - packages
   - tests
   - configs
   - reading path
3. Implement context pack:
   - ranked files
   - selected symbol outlines
   - key manifests
   - dependency summaries
4. Add profiles:
   - `human`
   - `agent`
   - `review`
   - `docs`
5. Add approximate token counting.
6. Add `--output`, `--stdout`, and `--copy` later if clipboard support is added.

## Acceptance Criteria

- `maple wiki . --output repo-map.md` produces readable docs.
- `maple pack . --budget 50000` respects the budget approximately.
- Markdown output looks good on GitHub.
- JSON output is stable enough for integrations.

