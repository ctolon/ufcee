//! Logs, metrics, and traces for UCEE Proxy.
//!
//! See `.claude/rules/observability-rules.md` for the canonical metric and
//! log-field catalog.

use ucee_core::Error;

/// Initialize observability (logs + metrics + tracing).
///
/// M0 placeholder; the concrete builder pattern lands at M6.
pub fn init() -> Result<(), Error> {
    Ok(())
}

/// Names of Prometheus metrics emitted by the proxy.
///
/// The names mirror the Go reference where possible.
pub mod metrics_names {
    pub const REQUESTS_TOTAL: &str = "ucee_proxy_requests_total";
    pub const REQUEST_DURATION: &str = "ucee_proxy_request_duration_seconds";
    pub const INFLIGHT_REQUESTS: &str = "ucee_proxy_inflight_requests";
    pub const UPSTREAM_ERRORS: &str = "ucee_proxy_upstream_errors_total";
    pub const BREAKER_STATE: &str = "ucee_proxy_breaker_state";
    pub const BODY_BYTES: &str = "ucee_proxy_body_bytes";
    pub const RATE_LIMIT_DROPS: &str = "ucee_proxy_rate_limit_drops_total";
    pub const ROUTING_DECISIONS: &str = "ucee_proxy_routing_decisions_total";
    pub const SPOOL_OVERFLOW: &str = "ucee_proxy_spool_overflow_total";
}
