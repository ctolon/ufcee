# Tasks

Backlog and status board for UCEE Proxy work. Maintained by `L1-memory-coordinator`.

## Active table

| id | title | owner_agent | status | depends_on | created | updated | lock_slug |
|---|---|---|---|---|---|---|---|
| T-2026-0001 | M0: bootstrap workspace, base crates, CI skeleton | L2-system-architect | open | – | 2026-05-11 | 2026-05-11 | – |
| T-2026-0002 | M1: adapter contract trait + harness | L2-adapter-custom-contract | open | T-2026-0001 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0003 | M1: first adapter (docling) | L2-adapter-docling | open | T-2026-0002 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0004 | M2: axum HTTP server + Docling facade | L2-api-designer | open | T-2026-0003 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0005 | M3: routing engine (MIME + ext + facade selector) | L2-routing-engine | open | T-2026-0004 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0006 | M4: kreuzberg adapter + response normalizer | L2-adapter-kreuzberg | open | T-2026-0005 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0007 | M5: PyO3 bridge + Python SDK | L2-pyo3-bridge | open | T-2026-0006 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0008 | M6: observability (logs + metrics + OTel) | L4-observability-engineer | open | T-2026-0004 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0009 | M7: resilience (CB + rate limit + retry + spool) | L2-circuit-breaker-designer | open | T-2026-0005 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0010 | M8: cloud-native + security hardening | L2-ssrf-defender | open | T-2026-0009 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0011 | M9: helm repo split + GA prep | L4-helm-author | open | T-2026-0010 | 2026-05-11 | 2026-05-11 | – |
| T-2026-0012 | M10: GA — perf budgets + CHANGELOG + v1.0.0 tag | L4-release-engineer | open | T-2026-0011 | 2026-05-11 | 2026-05-11 | – |

## Change log

| timestamp_utc | task_id | from | to | by |
|---|---|---|---|---|
| 2026-05-11T12:30:00Z | T-2026-0001..0012 | – | open | human (initial backlog from ADR-0001) |

## Parking lot (unscheduled)

- Additional adapters (`unstructured`, `markitdown`, `paddleocr`, `tika`) — slot between M5 and M7 in parallel with cross-cutting work; each gated by its own contract suite + `/threat-model` pass.
- Property-based test suite expansion (`L3-property-test-author`).
- Fuzz harness for parser/router entry points (`L3-fuzz-test-author`).
- SBOM + supply-chain attestations (`L4-sbom-author`) — required by M9.
