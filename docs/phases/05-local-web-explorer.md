# Phase 05: Local Web Explorer

## Goal

Make RepoPrism visually impressive enough to earn stars from screenshots alone.

## Deliverables

- `repoprism serve .`
- embedded local web server
- overview page
- graph explorer
- file detail pages
- static export

## Implementation Tasks

1. Add `axum` server.
2. Add embedded web assets.
3. Build API endpoints:
   - `/api/summary`
   - `/api/files`
   - `/api/graph`
   - `/api/search`
4. Build UI views:
   - overview
   - architecture graph
   - file explorer
   - important files
   - reading path
   - context pack preview
5. Add static export mode.
6. Add demo assets for README.

## UX Requirements

- First screen must show the repo name and useful facts.
- Graph must be readable on a real repo, not just a toy fixture.
- Users should know where to start reading within 10 seconds.
- No login, no API key, no cloud dependency.

## Acceptance Criteria

- `repoprism serve . --open` opens a browser.
- UI works for this repo and one large public repo.
- Static export can be shared as a folder.
- README has a strong screenshot or GIF.

