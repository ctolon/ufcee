//! Core error type for UCEE Proxy.
//!
//! This is the umbrella error type used by the `Adapter` trait and the router.
//! Each downstream crate may define its own narrower `Error` via `thiserror`
//! and convert into this one at boundaries.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("adapter error: {0}")]
    Adapter(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("routing error: {0}")]
    Routing(String),

    #[error("ssrf rejection: {0}")]
    Ssrf(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
