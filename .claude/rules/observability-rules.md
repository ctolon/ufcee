---
paths:
  - "crates/**/*.rs"
---

# Observability rules

## Logging

- Structured JSON logs to stdout via `tracing` + `tracing-subscriber` with `json` formatter.
- Every request handler emits **exactly one** `request_completed` log line with these fields:
  - `engine`, `compat_type`, `route`, `method`, `status`, `duration_ms`, `bytes_in`, `bytes_out`, `breaker_state`, `request_id`, `routing_path`, `tenant` (if multi-tenant).
- Log levels: `trace`, `debug`, `info`, `warn`, `error`. Default `info`.
- No PII or secret values in logs; verified by `tracing_test::traced_test` fixtures in `ucee-observability`.

## Metrics (Prometheus)

Metric names mirror the Go reference where possible:

- `ucee_proxy_requests_total{engine,route,method,status}` — counter
- `ucee_proxy_request_duration_seconds{engine,route}` — histogram
- `ucee_proxy_inflight_requests{engine}` — gauge
- `ucee_proxy_upstream_errors_total{engine,kind}` — counter (`kind` = `timeout|connect|http_status|parse|other`)
- `ucee_proxy_breaker_state{engine}` — gauge (0=closed, 1=half-open, 2=open)
- `ucee_proxy_body_bytes{direction}` — histogram (`direction` = `in|out`)
- `ucee_proxy_rate_limit_drops_total{engine}` — counter
- `ucee_proxy_routing_decisions_total{routing_path}` — counter

Exposed at `/metrics` in Prometheus exposition format.

## Tracing (optional)

- OpenTelemetry export via OTLP HTTP when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
- Default sample ratio: `0.1` (configurable via `OTEL_TRACES_SAMPLER_ARG`).
- Spans named `<facade>.<operation>` (e.g., `docling.convert_file`, `external.process`, `router.select_engine`).
- Trace context propagated via W3C `traceparent` header on outbound engine calls.
- Span attributes mirror the structured log fields.

## Health and readiness

- `/healthz` returns 200 if the process is alive.
- `/readyz` returns 200 only if all enabled engines have responded healthy at least once AND have not gone unhealthy within the last `readiness.cooldown` (default 30s).
- Failing readiness includes a JSON body listing the unhealthy engines and their last error.

## Version endpoint

- `/version` returns JSON `{ "version": "<semver>", "commit": "<git sha>", "build_time": "<iso8601>", "rustc": "<version>" }`.
- Fields populated from `build.rs` via env vars at compile time.
