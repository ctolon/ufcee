//! Core types and traits for UCEE Proxy.
//!
//! Defines the [`Adapter`] trait, [`Capabilities`], [`CompatType`] enum, and
//! shared [`ConvertRequest`] / [`ConvertResponse`] shapes. Every downstream
//! crate (router, server, adapters, py) builds on this.

pub mod adapter;
pub mod error;

pub use adapter::{
    Adapter, Capabilities, CompatType, ConvertRequest, ConvertResponse, HealthStatus,
};
pub use error::Error;
