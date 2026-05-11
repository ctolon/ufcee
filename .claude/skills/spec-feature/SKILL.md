---
name: spec-feature
description: Author the formal spec for a feature before planning. Produces a structured spec under docs/specs/ on the docs branch.
arguments: feature-name
context: inherit
agent: L2-system-architect
allowed-tools: Read, Edit, Write, Bash(git worktree *), Bash(git switch *)
---

# /spec-feature — author a feature spec

## Purpose

Author a structured feature spec before planning begins. The spec captures the
problem, constraints, options considered, the chosen approach, and exit
criteria. Lives on the `docs` branch under `docs/specs/<feature-name>.md`.

## Steps

1. Switch to the docs worktree.
2. Create `docs/specs/<feature-name>.md` using the standard spec template.
3. Fill in: Problem, Constraints, Options, Decision, Exit criteria.
4. Cross-reference any related ADRs.
5. Ask the user to review and commit on the docs branch.

## Outputs

- New spec file under `docs/specs/`.
- Cross-references to relevant ADRs.
