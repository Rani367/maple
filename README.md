<h1 align="center">Maple</h1>

<p align="center">
  Understand any codebase in seconds.
</p>

<p align="center">
  <a href="https://github.com/Rani367/maple/stargazers"><img src="https://img.shields.io/github/stars/Rani367/maple?style=social" alt="GitHub stars"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT license"></a>
  <img src="https://img.shields.io/badge/status-pre--alpha-orange.svg" alt="pre-alpha">
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust 2024">
</p>

Maple is a fast, local, single-binary tool for turning repositories into clear
architecture maps, repo docs, and compact context packs.

The goal is simple: point Maple at a repo and immediately see what matters,
where to start reading, how the pieces connect, and what context is worth
keeping.

```sh
maple scan .
maple scan github.com/zed-industries/zed
maple serve .
maple pack .
```

> Maple is currently pre-alpha. The first working command is `scan`; the map,
> pack, wiki, and web explorer commands are planned and documented.

## Why Maple

Opening an unfamiliar repository is still slower than it should be. You read the
README, skim folders, chase imports, look for entrypoints, guess which files are
important, and slowly build a mental map.

Maple is meant to make that first pass instant.

- Find the files, folders, packages, commands, tests, and configs that matter.
- Build deterministic repo maps from source code instead of vague summaries.
- Generate human-readable docs and machine-readable context from the same facts.
- Run locally by default, with no account and no required API key.
- Stay small, fast, and easy to install.

## Current Demo

Today, Maple can scan a local repository while respecting ignore files and
skipping common generated folders.

```sh
cargo run -- scan .
```

Example output:

```text
Maple scan

Root:  /path/to/repo
Files: 22
Size:  40.3 KB

Languages:
  Markdown        17 files     23.7 KB
  Other            3 files     10.6 KB
  Rust             1 files      5.6 KB
  TOML             1 files       430 B
```

For structured output:

```sh
cargo run -- scan . --json
```

## Planned Experience

Maple should grow into one tool with five useful outputs:

| Command | Purpose |
| --- | --- |
| `maple scan .` | Fast repo inventory: languages, files, manifests, size, and roles. |
| `maple map .` | Architecture map: entrypoints, important files, packages, dependencies, cycles, and reading path. |
| `maple wiki .` | Human docs: generate `repo-map.md` or a small repo wiki. |
| `maple pack .` | Context pack: compact Markdown/JSON for coding tools and reviews. |
| `maple serve .` | Local web explorer with graphs, file details, and exportable static HTML. |

## What Maple Will Detect

- Languages and file roles
- Package/workspace layout
- Entrypoints and commands
- Important files ranked with explainable reasons
- Imports, exports, symbols, and references
- Tests and likely ownership paths
- Dependency cycles and architecture hotspots
- Config files, manifests, lockfiles, and docs
- GitHub repository targets and cached local clones

## How It Compares

Maple is not trying to replace every code intelligence tool. It is aiming for a
specific lane: a beautiful, local first repo understanding tool that is useful
within seconds.

| Tool | Best At | Maple's Angle |
| --- | --- | --- |
| Repomix | Packing repo contents for LLMs | Adds repo structure, ranking, docs, and maps. |
| GitDiagram | Visual GitHub repo diagrams | Adds local CLI, deterministic analysis, and context exports. |
| DeepWiki | AI-generated repo docs | Keeps local deterministic facts as the base layer. |
| Sourcegraph | Enterprise code search | Focuses on a small single-binary workflow for one repo at a time. |

## Install

Maple is not published yet. For now, run it from source:

```sh
git clone https://github.com/Rani367/maple
cd maple
cargo run -- scan .
```

The GitHub project and CLI are named `maple`. Where a package namespace already
uses that name, such as crates.io, Maple will use `maple-cli`.

## Roadmap

The full implementation plan lives in [docs](docs/README.md). High-level phases:

1. Foundation: CLI structure, config, tests, CI, release skeleton.
2. Repository ingestion: local scanning, GitHub URLs, clone cache, file roles.
3. Language intelligence: tree-sitter adapters for Rust, TypeScript, Python, and Go.
4. Graph and ranking: dependency graph, PageRank-style ranking, cycles, reading path.
5. Reports and context packs: `repo-map.md`, JSON, Markdown packs, token budgets.
6. Local web explorer: `maple serve`, graph UI, file pages, static export.
7. GitHub workflow: paste a public repo and get a local map.
8. Agent integrations: optional MCP and compact context profiles.
9. Quality and performance: benchmarks, large repo testing, release binaries.
10. Launch and growth: README demos, example maps, Homebrew, and public release.

## Project Docs

- [Docs index](docs/README.md)
- [Product plan](docs/PRODUCT_PLAN.md)
- [Architecture](docs/ARCHITECTURE.md)
- [CLI spec](docs/CLI_SPEC.md)
- [Data model](docs/DATA_MODEL.md)
- [Implementation phases](docs/IMPLEMENTATION_PHASES.md)

## Development

```sh
cargo fmt
cargo check
cargo run -- scan .
cargo run -- scan . --json
```

## Contributing

Maple is early, so the best contributions are small and concrete:

- improve scanner correctness
- add fixture repos
- add language detection
- improve docs
- test Maple on real open source repositories

Open an issue before large architectural changes.

## License

MIT
