---
name: sync-docs
description: Cherry-pick or rebase docs-relevant commits from main to the docs branch.
arguments: "[<commit-range>]"
paths:
  - "docs/**"
context: inherit
agent: L3-doc-writer
allowed-tools: Read, Bash(git *)
---

# /sync-docs — sync to docs branch

## Purpose

Bring documentation updates from `main` onto the `docs` branch. Cherry-picks
commits whose changes touch `docs/`-relevant content (README, doc-comments in
public API), or rebases the docs branch if requested.

## Steps

1. Switch to the docs worktree.
2. Identify commits in `<commit-range>` (default: since last forward-merge).
3. Cherry-pick (or stage commits for user approval).
4. Resolve conflicts (with user approval per CLAUDE.md).
5. Suggest a commit message; user commits manually.

## Outputs

- Updated `docs` branch with cherry-picked commits.
- Conflict resolution drafts where needed.
