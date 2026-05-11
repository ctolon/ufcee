---
paths:
  - "**"
---

# Commit message rules

## Format — Conventional Commits

```
<type>(<scope>): <subject>

<body>

<footer>
```

- `<type>`: one of `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `build`, `ci`, `chore`, `revert`.
- `<scope>`: optional crate or area name (`router`, `adapter-docling`, `pyo3`, `helm`, ...).
- `<subject>`: imperative present tense, lowercase first letter, no trailing period, ≤72 chars.

## Body

- Wrapped at 100 columns.
- Explains the *why*, not the *what*. The diff shows the what.
- Multiple paragraphs separated by blank lines.

## Footer

- `BREAKING CHANGE: <description>` if the PR introduces breaking changes.
- Issue refs: `Closes #N`, `Refs #M`.
- No `Signed-off-by` unless DCO is enabled (currently not).

## Banned content (enforced by `guard_commit_message.sh` hook)

- No AI references: no `Co-Authored-By: Claude`, no `Generated with Claude Code`, no `claude.com` URLs, no robot emoji.
- No emoji of any kind (including U+1F300–U+1FAFF, U+2600–U+27BF).
- No marketing language: "powerful", "robust", "elegant", "amazing", "comprehensive".
- No slop phrases enumerated in `.claude/hooks/lib/slop_patterns.txt`.

## Examples

Good:

```
feat(router): add MIME magic precedence over Content-Type header

When client-provided Content-Type and magic-byte detection disagree,
magic wins. This closes a routing-spoofing vector where a renamed file
could be routed to the wrong adapter.

Closes #42
```

Bad:

```
✨ Add awesome routing feature

Co-Authored-By: Claude
```

(Banned: emoji, marketing adjective, AI reference. The hook will block this.)
