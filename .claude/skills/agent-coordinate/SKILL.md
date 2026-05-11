---
name: agent-coordinate
description: Allocate / release / inspect agent file locks and update TASKS.md status. Single-writer entry point for L1-memory-coordinator.
arguments: "<verb> <args>"
context: inherit
agent: L1-memory-coordinator
allowed-tools: Read, Write, Edit, Glob, Bash(ls *), Bash(cat *), Bash(date *)
---

# /agent-coordinate — coordination operations

## Purpose

Single entry point for all mutations to `.claude/agent-memory/`. Other agents
call this skill to claim / release locks, update task status, or record
decisions. The `L1-memory-coordinator` agent backs the skill, enforcing the
single-writer invariant.

## Verbs

- `claim <slug> <task_id>` — acquire a file lock.
- `release <slug>` — release a lock owned by the calling agent.
- `update-task <task_id> <new-status>` — advance a task in `TASKS.md`.
- `record-decision <slug>` — open a new `DECISIONS/NNNN-<slug>.md` ADR.
- `snapshot` — refresh `COORDINATION.md` with the current state.

## Outputs

- Verb-specific response (granted / deferred / updated / opened).
- Updated `.claude/agent-memory/` state.
