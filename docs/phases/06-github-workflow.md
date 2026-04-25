# Phase 06: GitHub Workflow

## Goal

Make the most viral input path work: paste a GitHub repo and get a map.

## Deliverables

- GitHub URL parsing.
- Clone cache.
- Branch/ref support.
- Better progress output.
- Optional hosted-demo plan.

## Implementation Tasks

1. Parse:
   - `owner/repo`
   - `github.com/owner/repo`
   - `https://github.com/owner/repo`
   - URLs with `/tree/<branch>`
2. Add branch/ref selection.
3. Add progress bars for clone and scan.
4. Add helpful errors for private repos.
5. Add `--refresh` and `--offline` behavior.
6. Prepare examples:
   - `maple github.com/zed-industries/zed`
   - `maple github.com/rust-lang/rust`
   - `maple github.com/vercel/next.js`

## Hosted Demo Plan

The CLI can succeed without a hosted service, but star growth improves if users
can see the product before installing it.

Possible later shape:

```text
https://maple.dev/github.com/owner/repo
```

The hosted service should be optional and should not become required for local
use.

## Acceptance Criteria

- Public GitHub repos work with one command.
- Cached repos do not reclone unnecessarily.
- Branch/ref scanning works.
- README demo uses a famous repo and looks impressive.

