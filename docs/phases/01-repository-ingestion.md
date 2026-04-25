# Phase 01: Repository Ingestion

## Goal

Make Maple excellent at finding and describing files before deeper language
intelligence exists.

## Deliverables

- Robust local scanner.
- GitHub URL resolver.
- Clone/update cache.
- Binary file detection.
- Generated/dependency folder detection.
- Stable scan JSON.

## Implementation Tasks

1. Add target parsing:
   - local path
   - `owner/repo`
   - `github.com/owner/repo`
   - full `https://github.com/owner/repo`
2. Add repo cache directory.
3. Clone remote repos through `git` first; consider `git2` later.
4. Add cache update behavior:
   - default reuse cache
   - `--refresh` fetches latest
   - `--no-cache` scans temp clone
5. Improve file classification:
   - text vs binary
   - generated vs source
   - docs/config/assets/lockfiles
6. Add scan summary:
   - languages
   - largest files
   - important manifests
   - repo type hints

## Acceptance Criteria

- `maple scan .` works locally.
- `maple scan github.com/Rani367/affected` clones and scans.
- Generated folders are skipped by default.
- Scan output is deterministic across runs.
- Errors explain what the user should do next.

