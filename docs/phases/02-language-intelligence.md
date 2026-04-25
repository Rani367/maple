# Phase 02: Language Intelligence

## Goal

Extract symbols, imports, tests, configs, and entrypoints from real source code.

## Deliverables

- Tree-sitter parser abstraction.
- Language adapters.
- Symbol and import extraction.
- Entrypoint detection.
- Test file detection.

## Initial Languages

1. Rust
2. TypeScript and JavaScript
3. Python
4. Go
5. Markdown and config files for repo documentation

## Implementation Tasks

1. Add `tree-sitter` dependencies.
2. Create `LanguageAdapter` trait:
   - detect file
   - parse file
   - extract symbols
   - extract imports
   - detect tests
   - detect entrypoints
3. Implement Rust adapter:
   - `fn`, `struct`, `enum`, `trait`, `impl`, `mod`, `use`
   - `main.rs`, `lib.rs`, tests
4. Implement TS/JS adapter:
   - imports/exports
   - functions/classes/components
   - route files for common frameworks
5. Implement Python adapter:
   - imports
   - functions/classes
   - `__main__`, tests
6. Store parse failures as warnings, not fatal errors.

## Acceptance Criteria

- Fixtures produce stable symbol/import output.
- Unsupported files do not break the scan.
- Parse errors show useful diagnostics.
- At least Rust and TS/JS are useful enough for a demo.

