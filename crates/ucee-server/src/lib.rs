//! HTTP server for UCEE Proxy.
//!
//! axum-based router that exposes the Docling and External facades, plus
//! ops endpoints (`/healthz`, `/readyz`, `/metrics`, `/version`). See
//! `docs/architecture/07-facade-selection.md` for facade selection logic.

use ucee_core::Error;

/// Builder for the UCEE HTTP application.
///
/// M0 placeholder. The full axum app lands at M2.
#[derive(Debug, Default)]
pub struct AppBuilder;

impl AppBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Build the app (M0: no-op).
    pub fn build(self) -> Result<(), Error> {
        Ok(())
    }
}
