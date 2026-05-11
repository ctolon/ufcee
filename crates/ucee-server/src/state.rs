//! Shared application state for axum handlers.

use ucee_core::Registry;
use ucee_router::Router as UceeRouter;

/// Read-only application state shared across all handlers via `Arc<AppState>`.
pub struct AppState {
    pub registry: Registry,
    pub router: UceeRouter,
}

impl AppState {
    pub fn new(registry: Registry, router: UceeRouter) -> Self {
        Self { registry, router }
    }
}
