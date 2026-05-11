# Performance budgets

_Updated by `L4-perf-benchmarker` after each baseline run._

Budgets are upper bounds, enforced in CI at **M10** onward (gated by
`/perf-budget-check` skill). Each bench has an absolute budget AND a relative-to-baseline
budget (±5% default unless overridden per bench).

## Baselines

(none — first measurements at **M6** observability milestone; budgets activate at M10)

## Budget targets (placeholders — to be replaced with measured numbers)

| bench | metric | p50 | p95 | p99 | rationale |
|---|---|---|---|---|---|
| `router/select_engine` | per-call ns | 5 µs | 20 µs | 50 µs | hot path; lookup-table-dominated |
| `router/mime_magic_sniff` | per-call ns | 50 µs | 200 µs | 500 µs | reads first 8 KB of body |
| `ssrf/validate` | per-call ns | 10 µs | 30 µs | 80 µs | DNS lookup amortized via pin |
| `streaming/spool_8MiB` | wall ms | 5 ms | 15 ms | 40 ms | `O_TMPFILE` + bounded buffers |
| `adapter/docling/convert_1MB_pdf` | wall ms | 500 ms | 1000 ms | 2000 ms | dominated by upstream engine |
| `adapter/kreuzberg/convert_1MB_pdf` | wall ms | 500 ms | 1000 ms | 2000 ms | dominated by upstream engine |
| `resilience/circuit_breaker_state_machine` | per-call ns | 100 ns | 300 ns | 800 ns | atomic ops only |
| `resilience/rate_limit_acquire` | per-call ns | 50 ns | 150 ns | 400 ns | token bucket atomic |
| `pyo3/convert_round_trip` | wall µs | 50 µs | 150 µs | 300 µs | GIL acquire + release + ser/de |
| `pyo3/iter_response` | per-call µs | 10 µs | 30 µs | 80 µs | streaming response chunks |
| `observability/log_request_completed` | per-call ns | 1 µs | 5 µs | 20 µs | tracing macro + JSON format |

All p50/p95/p99 numbers are **placeholders**. Real budgets are set after the M6 baseline
run and reviewed at M9 release-prep.

## Memory budgets

| bench | RSS-delta budget | rationale |
|---|---|---|
| `streaming/spool_8MiB` | ≤ 9 MiB | spool body + small overhead |
| `streaming/spool_64MiB` | ≤ 12 MiB | bounded by spool threshold, not body size |
| `adapter/concurrent_100_reqs` | ≤ 200 MiB | conn pools + spool + small per-req state |

## Latency budgets (end-to-end via integration test)

| flow | p99 |
|---|---|
| `/v1/convert/file` (1 MB PDF, docling) | 2 s |
| `/v1/convert/source` (URL, docling) | 5 s |
| `/healthz` | 1 ms |
| `/readyz` (when all healthy) | 5 ms |

## Process

1. Bench runs via `cargo bench` produce `target/criterion/<bench>/new/raw.csv`.
2. `L4-perf-benchmarker` summarizes into a baseline file under `benches/baselines/`.
3. CI compares incoming PR runs against baseline; >5% regression fails.
4. Intentional regressions require an ADR amending the budget.
