# Data Model

## Core Types

```text
Repository
  id
  root
  source
  scanned_at
  files
  packages
  graphs

FileNode
  id
  path
  language
  role
  size_bytes
  hash
  symbols
  imports

SymbolNode
  id
  file_id
  name
  kind
  visibility
  line_start
  line_end

PackageNode
  id
  name
  manager
  root_path
  manifest_path

Edge
  from
  to
  kind
  evidence
```

## File Roles

```text
source
test
entrypoint
config
docs
asset
generated
lockfile
unknown
```

## Edge Kinds

```text
imports
exports
references
defines
tests
belongs_to_package
depends_on_package
configured_by
documents
```

## Artifact Schemas

### Scan Report

Used by `repoprism scan --json`.

```json
{
  "root": "/path/to/repo",
  "files": 123,
  "bytes": 456789,
  "languages": {
    "Rust": {
      "files": 10,
      "bytes": 40000
    }
  }
}
```

### Repo Map

Used by `repoprism map --json`.

```json
{
  "repository": {},
  "important_files": [],
  "entrypoints": [],
  "packages": [],
  "cycles": [],
  "graphs": {
    "files": [],
    "symbols": [],
    "packages": []
  }
}
```

### Context Pack

Used by `repoprism pack`.

```json
{
  "metadata": {},
  "summary": "",
  "reading_path": [],
  "files": [],
  "symbols": [],
  "budget": {
    "requested": 50000,
    "estimated": 43210
  }
}
```

## Stability Rules

- Paths are stored relative to repo root.
- Output order is stable and sorted unless ranked.
- Ranked output includes a reason.
- JSON fields should be additive after the first public release.

