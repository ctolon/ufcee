---
name: feature
description: Start a new feature. Creates a feature branch from main (or --from <base>), adds a TASKS.md row, and hands off to L3-feature-planner for the implementation plan.
arguments: feature-name [--from <base>]
context: fork
agent: L3-feature-planner
allowed-tools: Read, Glob, Bash(git switch *), Bash(git branch *), Bash(git fetch *), Bash(git rev-parse *)
---

# /feature — start a new feature

## Purpose

Bootstrap a new feature: create a short-lived feature branch from `main` (or
`--from <base>`), add a corresponding `TASKS.md` row via `L1-memory-coordinator`,
and hand off to `L3-feature-planner` for the implementation plan.

## Steps

1. Validate the feature name matches `^[a-z0-9][a-z0-9-]{0,63}$`.
2. `git fetch origin` and verify the base branch exists.
3. Create `feature/<name>` branch from the base.
4. Ask `L1-memory-coordinator` to insert a TASKS.md row.
5. Hand off to `L3-feature-planner` in forked context for the plan.

## Outputs

- New local branch `feature/<name>`.
- New row in `TASKS.md`.
- An initial plan draft from `L3-feature-planner`.
