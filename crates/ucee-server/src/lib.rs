//! HTTP server for UCEE Proxy.
//!
//! axum-based router exposing the Docling facade
//! (`POST /v1/convert/file`, `POST /v1/convert/source`) plus ops endpoints
//! (`GET /healthz`, `GET /readyz`, `GET /version`).
//!
//! See `docs/architecture/03-sequence-convert-file.md` (on the `docs`
//! branch) for the end-to-end request flow.

use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use ucee_core::Registry;

mod routes;
mod state;

pub use state::AppState;

/// Builder for the UCEE HTTP application.
pub struct AppBuilder {
    registry: Registry,
}

impl AppBuilder {
    /// Construct a builder from an already-populated registry.
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }

    /// Build the axum [`Router`] ready to be served via `axum::serve`.
    pub fn build(self) -> Router {
        let state = Arc::new(AppState::new(self.registry));
        Router::new()
            .route("/healthz", get(routes::ops::healthz))
            .route("/readyz", get(routes::ops::readyz))
            .route("/version", get(routes::ops::version))
            .route("/v1/convert/file", post(routes::convert::convert_file))
            .route("/v1/convert/source", post(routes::convert::convert_source))
            .with_state(state)
    }
}
