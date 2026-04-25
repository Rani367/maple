# Phase 07: Agent Integrations

## Goal

Make RepoPrism useful for AI coding workflows while keeping the product useful
without AI.

## Deliverables

- AI-ready context packs.
- MCP server mode.
- editor-friendly generated files.
- prompts and integration examples.

## Implementation Tasks

1. Improve `repoprism pack`.
2. Add profiles:
   - `agent`
   - `small-context`
   - `large-context`
   - `review`
3. Add MCP server:
   - `repo_summary`
   - `important_files`
   - `find_symbol`
   - `dependency_path`
   - `context_pack`
4. Add config examples for common tools.
5. Add security defaults:
   - do not include `.env`
   - skip secrets by default
   - warn on suspicious files

## Acceptance Criteria

- Context pack is useful in Codex/Claude/Cursor style workflows.
- MCP tools are deterministic and documented.
- Agent integration does not dominate the README above the core product.
- Sensitive files are skipped by default.

