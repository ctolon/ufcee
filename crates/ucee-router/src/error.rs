//! Routing errors.

use thiserror::Error;

/// Errors returned by [`crate::Router::select`].
#[derive(Debug, Error)]
pub enum RoutingError {
    /// No precedence rule matched the request and no default engine is set.
    ///
    /// Surfaced by the server as HTTP 415 Unsupported Media Type.
    #[error("no engine matches the request and no default is configured")]
    NoEngineMatches,

    /// The request named an engine via the `X-UCEE-Engine` header but the
    /// router does not know that engine.
    ///
    /// Surfaced by the server as HTTP 409 Conflict.
    #[error("explicit engine '{0}' is not registered")]
    UnknownEngine(String),
}
