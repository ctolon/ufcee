//! Core types and traits for UCEE Proxy.
//!
//! Defines the [`Adapter`] trait, [`Capabilities`], [`CompatType`] enum, and
//! shared [`ConvertRequest`] / [`ConvertResponse`] shapes. Every downstream
//! crate (router, server, adapters, py) builds on this.

pub mod adapter;
pub mod error;
pub mod registry;

pub use adapter::{
    Adapter, CURRENT_CONTRACT_VERSION, Capabilities, CompatType, ConvertRequest, ConvertResponse,
    DynAdapter, HealthStatus,
};
pub use error::Error;
pub use registry::Registry;
