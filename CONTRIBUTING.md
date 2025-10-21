### Contributing to WeaveHeap

This repo uses Conventional Commits for all commits. Keep it simple: we do not use scopes.

## Conventional Commits

Use the form:

```
<type>: <subject>

[optional body]

[optional footer(s)]
```

Subject rules:

- Imperative mood, no trailing period, ≤ 72 characters
- UTF‑8 allowed; avoid emoji in the subject

Accepted types:

- `build` – build system or external dependencies (e.g., package.json, tooling)
- `chore` – maintenance (no app behavior change)
- `ci` – continuous integration configuration (workflows, pipelines)
- `docs` – documentation only
- `feat` – user-facing feature or capability
- `fix` – bug fix
- `perf` – performance improvements
- `refactor` – code change that neither fixes a bug nor adds a feature
- `revert` – revert of a previous commit
- `style` – formatting/whitespace (no code behavior)
- `test` – add/adjust tests only

Examples:

```text
feat: add region allocator and capability handles
fix: correct borrow state transitions for mutable slices
docs: document WeaveHeap memory model and error mapping
style: format Rust modules and C headers for heap ABI
chore: update Arrow/Polars fixtures and CI benchmark datasets
ci: add workflow to run zero-copy integration benchmarks
perf: reduce copy-on-write in map insertion path
refactor: extract TypedArray and Arrow view adapters
test: add fixtures for cross-runtime borrow escalation
revert: revert "perf: reduce copy-on-write in map insertion path"
```

Breaking changes:

- Use `!` after the type or a `BREAKING CHANGE:` footer.

```text
feat!: switch Node views from Buffer to TypedArray

BREAKING CHANGE: Node views now expose TypedArray instead of Buffer; update call sites.
```
