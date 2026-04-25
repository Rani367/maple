# RepoLens

Understand any repository in seconds.

RepoLens is a fast, local, single-binary tool that turns a codebase into a map,
wiki, and AI-ready context pack. It starts as a Rust CLI and will grow into a
beautiful local web view for exploring architecture, important files, entry
points, tests, and dependencies.

```sh
repolens scan .
repolens scan github.com/zed-industries/zed
repolens serve .
repolens pack .
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

## License

MIT
