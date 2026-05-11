//! Adapter trait and supporting types.
//!
//! Every engine adapter implements [`Adapter`]. The contract suite in
//! `ucee-adapters-fixtures` exercises every adapter's implementation against
//! a fixed set of fixtures (lands at M1 per ADR-0003).

use std::future::Future;

use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Compatibility type of an engine — determines which facade(s) the proxy
/// exposes for this engine. See `docs/architecture/07-facade-selection.md`
/// on the `docs` branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatType {
    Docling,
    External,
    DoclingExternal,
    Tika,
    Custom,
}

/// Capabilities declared by an adapter at registry build time.
#[derive(Debug, Clone)]
pub struct Capabilities {
    pub compat_type: CompatType,
    pub mime_types: Vec<mime::Mime>,
}

/// Request to convert a document.
#[derive(Debug)]
pub struct ConvertRequest {
    pub mime: mime::Mime,
    pub filename: Option<String>,
    pub body: Bytes,
}

/// Response from converting a document.
#[derive(Debug, Serialize, Deserialize)]
pub struct ConvertResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

/// Health status reported by an adapter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
}

/// Current contract version expected by `ucee-core`.
///
/// Adapters declare their conformance via [`Adapter::contract_version`].
/// The registry rejects adapters whose version is lower than this constant.
/// Bumped when the trait surface changes incompatibly. See ADR-0003.
pub const CURRENT_CONTRACT_VERSION: u32 = 1;

/// The contract every engine adapter implements.
///
/// The trait is sealed by convention to the workspace via the
/// `enginetest::run_contract_suite::<A>()` harness (in
/// `ucee-adapters-fixtures`), which is the only sanctioned way to certify
/// an `Adapter` implementation.
pub trait Adapter: Send + Sync + 'static {
    /// Adapter name; must match `^[a-z0-9][a-z0-9-]{0,31}$`.
    fn name(&self) -> &'static str;

    /// Declared capabilities (`compat_type` + MIME accept set).
    fn capabilities(&self) -> Capabilities;

    /// Contract version this adapter conforms to.
    ///
    /// New adapters return [`CURRENT_CONTRACT_VERSION`]. The default impl
    /// does so; adapters frozen against an older trait surface override
    /// this and are rejected at registry build by the core.
    fn contract_version(&self) -> u32 {
        CURRENT_CONTRACT_VERSION
    }

    /// Convert a document.
    fn convert(
        &self,
        req: ConvertRequest,
    ) -> impl Future<Output = Result<ConvertResponse, crate::Error>> + Send;

    /// Health check.
    fn health(&self) -> impl Future<Output = Result<HealthStatus, crate::Error>> + Send;
}
