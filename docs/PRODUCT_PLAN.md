# Product Plan

## One Line

Maple turns any repository into a local architecture map, repo wiki, and
context pack in seconds.

## Positioning

Maple should feel like a local, single-binary blend of GitDiagram, DeepWiki,
and Repomix:

- GitDiagram style visual clarity.
- DeepWiki style repo explanation.
- Repomix style AI-friendly context export.
- Rust style speed, privacy, and single-binary distribution.

The product should not be positioned as a generic static analyzer. It should be
positioned as a fast way to understand an unfamiliar repo.

## Naming

Use `Maple` for the product name, `maple` for the command and GitHub repository,
and `maple-cli` where the plain package name is unavailable.

## Audience

Primary users:

- Developers opening unfamiliar repositories.
- Open source maintainers who want a beautiful repo overview.
- AI coding users who need compact, trustworthy project context.
- Students and builders exploring famous projects.

Secondary users:

- Teams onboarding new engineers.
- Consultants auditing client repos.
- Technical writers generating docs from source.

## Core Use Cases

1. Open a local repo and see what matters.
2. Paste a GitHub repo and get a map without manual setup.
3. Generate `repo-map.md` for humans.
4. Generate compact context files for AI coding tools.
5. Explore architecture in a local browser.
6. Share a static HTML export in issues, docs, or READMEs.

## Non-Goals

- Full IDE replacement.
- Full Sourcegraph replacement.
- Perfect semantic understanding of every language on day one.
- Cloud account requirement.
- Hidden code upload by default.

## Product Principles

- Local first. No network unless the user asks for a GitHub URL or update.
- Deterministic first. Prefer parsed facts over invented summaries.
- Fast first impression. The first run should produce something useful quickly.
- Beautiful demo. Screenshots and GIFs are part of the product.
- Progressive depth. Start with a useful overview, then let users drill down.

## Success Metrics

Early project metrics:

- `maple .` produces a useful summary in under 5 seconds on small repos.
- `maple github.com/owner/repo` works with no extra setup.
- README demo communicates the product in under 10 seconds.
- First release has binaries for macOS, Linux, and Windows.

GitHub-star metrics:

- 500 stars: CLI, context pack, and README demo are useful.
- 2,000 stars: local web explorer and GitHub URL workflow feel polished.
- 10,000 stars: hosted demo, browser extension, and viral URL trick exist.
