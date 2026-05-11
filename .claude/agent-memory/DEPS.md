# Workspace dependency graph

_Updated by `L1-dependency-tracker` after every crate boundary change._

Workspace is currently empty (M0 not yet started). The table below names the **planned**
M0 crate layout. The dependency tracker will re-derive the actual graph from
`cargo metadata` once crates exist.

## Planned crates (M0)

| crate | role |
|---|---|
| ucee-core | shared types, errors, traits (`Adapter`, `Facade`, `Capabilities`, `compat_type` enum) |
| ucee-config | three-layer config loader (defaults → YAML → env) |
| ucee-router | MIME router, ext router, facade selector |
| ucee-ssrf | SSRF validator, CIDR allowlist, DNS pinning |
| ucee-streaming | bounded spool over `O_TMPFILE`, backpressure |
| ucee-resilience | circuit breaker, rate limiter, retry policy |
| ucee-observability | log schema, Prometheus metrics, OTel spans |
| ucee-adapters-fixtures | shared adapter test fixtures |
| ucee-adapter-docling | first adapter |
| ucee-server | axum HTTP server tying the above together |
| ucee-py | PyO3 bindings (depends on ucee-core + selected ucee-server bits) |
| ucee | binary crate (entrypoint, config loading, signal handling) |

## Allowed inter-crate edges

Lower in this list depends on higher. No cycles permitted.

```
ucee-core
  ucee-config
  ucee-ssrf
  ucee-streaming
  ucee-resilience
  ucee-observability
  ucee-adapters-fixtures
  ucee-router
    (any adapter)
      ucee-server
        ucee
        ucee-py
```

## External dependency policy

- Every new direct external dependency requires an entry in this file plus an ADR
  naming the dep, version range, license, and rationale.
- `cargo deny check` (CI-enforced) mirrors this list.

| crate | direct deps (planned M0) | rationale |
|---|---|---|
| ucee-core | thiserror, serde, serde_json, mime, bytes | base types + serde + MIME parsing |
| ucee-config | ucee-core, serde, serde_yaml, figment | three-layer config |
| ucee-router | ucee-core, mime, infer | MIME + magic-byte detection |
| ucee-ssrf | ucee-core, ipnet, hickory-resolver | CIDR + DNS-pin |
| ucee-streaming | ucee-core, tokio, bytes, tempfile | spool over `O_TMPFILE` |
| ucee-resilience | ucee-core, tokio, governor | CB + token bucket |
| ucee-observability | tracing, tracing-subscriber, metrics, metrics-exporter-prometheus, opentelemetry, opentelemetry-otlp | logs + metrics + traces |
| ucee-adapter-docling | ucee-core, ucee-router, reqwest, tokio, serde, serde_json | HTTP client + docling JSON |
| ucee-server | ucee-core, ucee-router, ucee-resilience, ucee-observability, axum, tower, tower-http, tokio | HTTP server |
| ucee-py | ucee-core, ucee-server (subset), pyo3, pyo3-async-runtimes | Python bindings |
| ucee (bin) | ucee-server, ucee-config, ucee-observability, anyhow, tokio | entrypoint |

(Dev-dependencies omitted for brevity; tracked separately when added.)
