---
name: review-branch
description: Parallel review by code, security, and performance reviewers. Aggregates findings by severity.
arguments: "[<base-ref>]"
context: fork
agent: L3-code-reviewer
allowed-tools: Read, Grep, Glob, Bash(git diff *), Bash(git log *), Task
---

# /review-branch — three-way parallel review

## Purpose

Run `L3-code-reviewer`, `L3-security-reviewer`, and `L3-performance-reviewer` in
parallel (each forked) against the current branch's diff vs `<base-ref>`
(default `main`). Aggregate findings by severity.

## Steps

1. Determine base ref (argument or default `main`).
2. Spawn the three reviewers in parallel via the Task tool.
3. Collect findings from each.
4. Merge and sort by severity.
5. Emit consolidated report.

## Outputs

- Consolidated findings list grouped by severity (Blocker / Critical / High / Medium / Low / Nit).
- Per-reviewer breakdown.
- Suggested fix order.
