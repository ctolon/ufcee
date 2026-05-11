---
name: snapshot-arch
description: Emit an architecture snapshot (crate graph + public traits + endpoints) to the docs branch.
context: fork
agent: L2-system-architect
allowed-tools: Read, Glob, Bash(cargo metadata *), Bash(git worktree *), Bash(git switch *)
---

# /snapshot-arch — architecture snapshot to docs

## Purpose

Generate a markdown architecture snapshot (crate graph, public traits, HTTP
endpoint table, current ADR list) and stage it onto the `docs` branch under
`docs/reference/architecture-snapshot-<UTC>.md`.

## Steps

1. Switch to the docs worktree (`git worktree add ../ufcee-docs docs` if missing).
2. Run `cargo metadata` to capture crate graph.
3. Grep `pub trait` declarations across workspace.
4. Grep route declarations in `ucee-server`.
5. List ADRs.
6. Emit the snapshot file in the docs worktree.

## Outputs

- New file `docs/reference/architecture-snapshot-<YYYY-MM-DD>.md` on the docs branch.
- Suggested commit message (user commits manually).
