//! Docling adapter for UCEE Proxy.
//!
//! Speaks the docling REST contract (`POST /v1/convert/file`,
//! `GET /healthz`) and returns the docling-native JSON as a
//! [`ucee_core::ConvertResponse`].
//!
//! The contract test in `tests/contract.rs` uses `wiremock` to assert this
//! adapter satisfies the [`ucee_core::Adapter`] contract.

mod client;

pub use client::DoclingAdapter;
