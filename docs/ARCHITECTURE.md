# Architecture

## Data Flow

```text
input target
  -> source resolver
  -> repository scanner
  -> language parsers
  -> symbol/import graph
  -> ranker
  -> artifact writers
  -> CLI/TUI/web outputs
```

## Planned Modules

The current crate is a small single binary. As the project grows, split the code
into modules before creating multiple crates.

```text
src/
  main.rs
  cli.rs
  config.rs
  resolver.rs
  scan/
  language/
  graph/
  rank/
  report/
  pack/
  web/
  util/
```

Future workspace split:

```text
crates/
  repoprism-cli/
  repoprism-core/
  repoprism-lang/
  repoprism-web/
```

Only split into a workspace when the single crate becomes painful.

## Source Resolver

Responsibilities:

- Accept local paths.
- Accept GitHub shorthands like `github.com/owner/repo`.
- Accept full GitHub URLs.
- Clone or update remote repos into a cache directory.
- Return a local working directory for scanning.

Cache location:

```text
~/Library/Caches/repoprism/repos/
~/.cache/repoprism/repos/
```

## Scanner

Responsibilities:

- Walk files while respecting ignore files.
- Skip generated and dependency folders.
- Detect binary files.
- Track language, size, path, and file role.
- Produce stable IDs for files.

Scanner output must be deterministic so generated docs do not churn.

## Language Intelligence

Responsibilities:

- Parse supported languages with tree-sitter.
- Extract symbols, imports, exports, routes, tests, and entrypoints.
- Fall back gracefully for unsupported languages.
- Keep language adapters isolated and testable.

Initial language priority:

1. Rust
2. TypeScript and JavaScript
3. Python
4. Go
5. Markdown, JSON, TOML, YAML as config/docs support

## Graph Layer

Responsibilities:

- Represent files, packages, symbols, imports, tests, commands, and configs.
- Add edges for imports, symbol references, test ownership, and package membership.
- Run graph algorithms for centrality, cycles, clusters, and entrypoints.

## Ranker

Responsibilities:

- Rank important files.
- Rank folders and packages.
- Rank symbols.
- Explain why each item is important.

Signals:

- Import fan-in and fan-out.
- Entrypoint status.
- Test coverage relationship.
- README/package metadata.
- Git activity.
- File size and complexity.

## Artifact Writers

Artifacts:

- Terminal summary.
- `repo-map.md`.
- `repo-context.md`.
- `repo-context.json`.
- Static HTML export.
- Search index for the local web explorer.

## Web Explorer

The local web explorer should be embedded in the binary. The server can use
`axum`, with static assets embedded at build time.

Pages:

- Overview
- File explorer
- Dependency graph
- Entrypoints
- Tests
- Docs/wiki
- Context pack preview

