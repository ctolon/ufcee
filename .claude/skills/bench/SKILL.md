---
name: bench
description: Run the criterion benchmark suite (full or scoped to a target) and report p50/p95/p99 vs baseline.
arguments: "[target]"
paths:
  - "crates/**"
context: fork
agent: L4-perf-benchmarker
allowed-tools: Read, Bash(cargo bench *)
---

# /bench — run criterion benchmarks

## Purpose

Execute `cargo bench` across the workspace (or a specific bench target),
summarize p50 / p95 / p99, compare against the stored baseline, and emit a
delta report.

## Steps

1. Run `cargo bench` (with `--bench <target>` if argument supplied).
2. Parse `target/criterion/<bench>/new/raw.csv`.
3. Compute deltas vs `benches/baselines/`.
4. Format report with status per bench (PASS / REGRESS / IMPROVE).

## Outputs

- Per-bench p50 / p95 / p99.
- Delta vs baseline with status.
- Highlight any breach of `PERF_BUDGETS.md`.
