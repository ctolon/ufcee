# Agent memory — coordination protocol

This directory is the **shared coordination plane** for parallel Claude Code agents
working on UCEE Proxy. It is committed to git; personal notes live in the per-developer
`~/.claude/projects/.../memory/` (Anthropic auto-memory) and never enter this directory.

## Files

| file | role | writer |
|---|---|---|
| README.md | protocol spec (this file) | L2-system-architect / human |
| COORDINATION.md | live snapshot of active agents, locks, recent decisions | L1-memory-coordinator |
| TASKS.md | feature backlog with status board | L1-memory-coordinator |
| DEPS.md | workspace dependency graph snapshot | L1-dependency-tracker |
| PERF_BUDGETS.md | per-benchmark performance budgets | L4-perf-benchmarker |
| CHECKPOINT.md | pre-compaction state dump (gitignored) | `dump_state.sh` hook |
| DECISIONS/NNNN-<slug>.md | architectural decision records | L2-system-architect |
| LOCKS/<slug>.lock | active file locks (gitignored) | L1-memory-coordinator |
| sessions/<UTC>-<id>.md | session summaries (gitignored) | `session_summary.sh` hook |

## Single-writer invariant

Only `L1-memory-coordinator` mutates this directory. Other agents request changes via
`/agent-coordinate <verb> <args>`. This eliminates write conflicts between parallel
agents working on different concerns.

## Lock protocol

`LOCKS/<slug>.lock` is a TOML file with these fields:

- `agent` — owning agent name (e.g., `L3-rust-implementer`).
- `task_id` — TASKS.md row id (e.g., `T-2026-0014`).
- `session_id` — Claude Code session id.
- `host` — hostname of the session.
- `pid` — process id holding the lock.
- `acquired_utc` — ISO 8601 UTC timestamp of last acquire/heartbeat.
- `ttl_seconds` — lock TTL (default 1800).
- `notes` — free-form one-line explanation.

Acquire / release semantics:

- An agent claims via `/agent-coordinate claim <slug> <task_id>`.
- Heartbeat by re-acquire (rewrite `acquired_utc`).
- TTL default 30 minutes.
- Stale = `now - acquired_utc > ttl_seconds` AND (`kill -0 <pid>` fails OR `host` differs from current).
- `session_start.sh` reaps stale locks at session boot.
- Release on `/agent-coordinate release <slug>` or the owning `Stop` hook.

## Task schema

`TASKS.md` is a Markdown table with a change log section below. Columns:

`id | title | owner_agent | status | depends_on | created | updated | lock_slug`

- Status values: `open`, `claimed`, `in-progress`, `in-review`, `done`, `blocked`.
- Status changes appended to the log section with UTC timestamp.

## Decision records (ADRs)

`DECISIONS/NNNN-<slug>.md` follows a lightweight ADR template (see ADR-0001 for the
canonical example). Each ADR:

- Has a unique 4-digit NNNN, assigned in sequence.
- Lives forever (status changes to `superseded by NNNN` rather than deletion).
- Is committed in the same PR as the change it justifies, or the preceding planning PR.

## Coordination snapshot

`COORDINATION.md` is rewritten by `L1-memory-coordinator` whenever the active state
changes. The `UserPromptSubmit` hook (`inject_coordination.sh`) injects the tail 4 KB
of this file into Claude's context at the start of every prompt, so agents always see
current locks and active tasks before responding.

## Allowlist note

This file (`README.md`) and the rule files describing banned slop patterns are
explicitly allowlisted in `.claude/hooks/scripts/no_ai_slop.sh` so they may quote the
banned patterns verbatim for documentation purposes.
