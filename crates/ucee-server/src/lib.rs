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
use ucee_router::{Router as UceeRouter, RoutingError};

mod routes;
mod state;

pub use state::AppState;

/// Builder for the UCEE HTTP application.
///
/// Consumes a populated [`Registry`], builds a [`UceeRouter`] from the
/// adapters' declared `Capabilities`, and yields an [`axum::Router`].
pub struct AppBuilder {
    registry: Registry,
    default_engine: Option<String>,
}

impl AppBuilder {
    /// Construct a builder from an already-populated registry.
    pub fn new(registry: Registry) -> Self {
        Self {
            registry,
            default_engine: None,
        }
    }

    /// Set the default engine name.
    ///
    /// The name must be one already registered in the registry; otherwise
    /// [`build`](Self::build) returns [`RoutingError::UnknownEngine`].
    pub fn default_engine(mut self, name: impl Into<String>) -> Self {
        self.default_engine = Some(name.into());
        self
    }

    /// Build the axum [`Router`] ready to be served via `axum::serve`.
    ///
    /// # Errors
    ///
    /// - [`RoutingError::UnknownEngine`] if a default engine was set to a
    ///   name not present in the registry.
    pub fn build(self) -> Result<Router, RoutingError> {
        let mut rb = UceeRouter::builder();
        let names: Vec<String> = self.registry.names().map(String::from).collect();
        for n in &names {
            if let Some(adapter) = self.registry.get(n) {
                rb = rb.engine(n.as_str(), adapter.capabilities().mime_types);
            }
        }
        if let Some(d) = self.default_engine {
            rb = rb.default_engine(d);
        }
        let router = rb.build()?;

        let state = Arc::new(AppState::new(self.registry, router));
        Ok(Router::new()
            .route("/healthz", get(routes::ops::healthz))
            .route("/readyz", get(routes::ops::readyz))
            .route("/version", get(routes::ops::version))
            .route("/v1/convert/file", post(routes::convert::convert_file))
            .route("/v1/convert/source", post(routes::convert::convert_source))
            .with_state(state))
    }
}
