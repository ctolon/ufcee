//! Shared application state for axum handlers.

use ucee_core::Registry;

/// Read-only application state shared across all handlers via `Arc<AppState>`.
pub struct AppState {
    pub registry: Registry,
}

impl AppState {
    pub fn new(registry: Registry) -> Self {
        Self { registry }
    }
}
