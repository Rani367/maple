# CLI Spec

## Command Principles

- Commands should work with a local path by default.
- GitHub URL support should feel magic but remain explicit.
- Human output should be beautiful.
- Machine output should be stable JSON.
- Every destructive command must have dry-run support.

## Commands

### `repoprism scan [target]`

Scans a repository and prints a terminal summary.

Options:

```text
--json                 print stable JSON
--include-hidden       include hidden files
--max-file-size <size> skip large files
--no-gitignore         ignore ignore files
```

### `repoprism map [target]`

Builds a structural repository map.

Outputs:

- important files
- entrypoints
- package/workspace layout
- dependency graph summary
- cycles
- test relationships

### `repoprism pack [target]`

Generates AI-friendly context.

Options:

```text
--format markdown|json|xml
--budget <tokens>
--output <path>
--profile agent|human|review|docs
```

### `repoprism wiki [target]`

Generates human documentation.

Outputs:

- `repo-map.md`
- optional `docs/repoprism/` wiki folder

### `repoprism serve [target]`

Starts the local web explorer.

Options:

```text
--port <port>
--open
--no-open
```

### `repoprism export [target]`

Creates a static HTML export.

Options:

```text
--output <dir>
--embed-assets
```

### `repoprism cache`

Manages cloned GitHub repos and indexes.

Subcommands:

```text
list
clean
remove <repo>
```

## Exit Codes

```text
0 success
1 user/input error
2 scan or parse error
3 output/write error
4 network or GitHub error
```

