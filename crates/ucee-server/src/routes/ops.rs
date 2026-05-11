//! Ops endpoints: `/healthz`, `/readyz`, `/version`.

use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};

use crate::AppState;

/// Liveness probe — always 200 while the process is up.
pub async fn healthz() -> &'static str {
    "ok"
}

/// Readiness probe — 200 if at least one adapter is registered.
///
/// M2 simplification: just checks the registry. Per-adapter health
/// readiness lands at M6 (observability) with a periodic background prober.
pub async fn readyz(State(state): State<Arc<AppState>>) -> Result<&'static str, StatusCode> {
    if state.registry.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    Ok("ok")
}

/// Version info — name, version, registered adapter names.
pub async fn version(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mut names: Vec<&str> = state.registry.names().collect();
    names.sort_unstable();
    Json(serde_json::json!({
        "name": "ucee",
        "version": env!("CARGO_PKG_VERSION"),
        "engines": names,
    }))
}
