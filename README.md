# Maple

Understand any repository in seconds.

Maple is a fast, local, single-binary tool that turns a codebase into a map,
wiki, and AI-ready context pack. It starts as a Rust CLI and will grow into a
beautiful local web view for exploring architecture, important files, entry
points, tests, and dependencies.

```sh
maple scan .
maple scan github.com/zed-industries/zed
maple serve .
maple pack .
```

## Vision

- Map any GitHub repo or local directory.
- Show the files and folders that matter first.
- Generate a clean `repo-map.md` for humans.
- Generate compact context packs for coding tools.
- Export a shareable static HTML architecture map.
- Run fully local by default, with no API key required.

## Current Status

The first scaffold includes a working `scan` command that summarizes a local
repository while respecting `.gitignore`.

```sh
cargo run -- scan .
cargo run -- scan . --json
```

The GitHub project and CLI are named `maple`. Where a package namespace already
uses that name, such as crates.io, use `maple-cli`.

## Project Docs

- [Docs index](docs/README.md)
- [Product plan](docs/PRODUCT_PLAN.md)
- [Architecture](docs/ARCHITECTURE.md)
- [CLI spec](docs/CLI_SPEC.md)
- [Data model](docs/DATA_MODEL.md)
- [Implementation phases](docs/IMPLEMENTATION_PHASES.md)

## License

MIT
