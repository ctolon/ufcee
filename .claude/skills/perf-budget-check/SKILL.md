---
name: perf-budget-check
description: Verify benchmark deltas against PERF_BUDGETS.md. Fails if any p99 breaches budget without ADR exception.
paths:
  - "crates/**"
context: fork
agent: L4-perf-benchmarker
allowed-tools: Read, Bash(cargo bench *)
---

# /perf-budget-check — enforce perf budgets

## Purpose

Run the bench suite (or read the latest run) and check every measured
percentile against `agent-memory/PERF_BUDGETS.md`. Breaches fail the check unless
an ADR amends the budget.

## Steps

1. Run `/bench` or read the most recent run.
2. For each bench, look up budget in `PERF_BUDGETS.md`.
3. Compare measured p50 / p95 / p99 against budget.
4. List breaches with delta.
5. Suggest either a fix (preferred) or an ADR amendment.

## Outputs

- Pass / fail verdict.
- Breach list with delta.
- Suggested next steps.
