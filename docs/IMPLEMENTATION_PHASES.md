# Implementation Phases

Maple should be built in phases where each phase produces a demoable product.

## Phase 00: Foundation

Goal: turn the scaffold into a maintainable CLI project.

Deliverables:

- CLI structure
- config loading
- errors and logging
- test fixtures
- CI
- release automation skeleton

Details: [Phase 00](phases/00-foundation.md)

## Phase 01: Repository Ingestion

Goal: make `maple scan` excellent for local directories and GitHub repos.

Deliverables:

- ignore-aware scanner
- binary/generated file skipping
- GitHub URL resolver
- repo cache
- scan JSON schema

Details: [Phase 01](phases/01-repository-ingestion.md)

## Phase 02: Language Intelligence

Goal: extract real structure from source code.

Deliverables:

- tree-sitter integration
- Rust, TS/JS, Python support
- symbol/import extraction
- entrypoint/test/config detection

Details: [Phase 02](phases/02-language-intelligence.md)

## Phase 03: Graph And Ranking

Goal: identify what matters in a repo.

Deliverables:

- file graph
- package graph
- symbol graph
- PageRank/import ranking
- cycles and hotspots
- reading path

Details: [Phase 03](phases/03-graph-and-ranking.md)

## Phase 04: Reports And Context Packs

Goal: generate useful artifacts.

Deliverables:

- `repo-map.md`
- `repo-context.md`
- JSON export
- token-budgeted pack mode
- copyable terminal output

Details: [Phase 04](phases/04-reports-and-context-packs.md)

## Phase 05: Local Web Explorer

Goal: make the product visually impressive.

Deliverables:

- local `maple serve`
- overview page
- graph page
- file detail page
- static export
- screenshots/GIF workflow

Details: [Phase 05](phases/05-local-web-explorer.md)

## Phase 06: GitHub Workflow

Goal: make the viral input path work.

Deliverables:

- `maple github.com/owner/repo`
- full URL parsing
- branch/ref support
- caching and update behavior
- optional hosted demo plan

Details: [Phase 06](phases/06-github-workflow.md)

## Phase 07: Agent Integrations

Goal: make Maple useful for AI coding tools without making AI mandatory.

Deliverables:

- context pack profiles
- MCP server
- editor-friendly outputs
- prompt snippets

Details: [Phase 07](phases/07-agent-integrations.md)

## Phase 08: Quality And Performance

Goal: make Maple trustworthy on real repositories.

Deliverables:

- benchmarks
- fixture repos
- snapshot tests
- memory limits
- parallel scanning
- binary releases

Details: [Phase 08](phases/08-quality-and-performance.md)

## Phase 09: Launch And Growth

Goal: maximize adoption and GitHub stars.

Deliverables:

- great README
- demo GIF
- example maps for famous repos
- Homebrew install
- release notes
- browser extension or URL trick plan

Details: [Phase 09](phases/09-launch-and-growth.md)

