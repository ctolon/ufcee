//! Engine selection for UCEE Proxy.
//!
//! Precedence: explicit header > config rule > MIME magic > extension > default.
//! See `docs/architecture/05-routing-precedence.md` on the `docs` branch.

use ucee_core::Error;

/// Decision returned by [`select_engine`].
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub engine_name: String,
    pub routing_path: RoutingPath,
}

/// Which precedence rule produced the routing decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingPath {
    Header,
    Config,
    Mime,
    Ext,
    Default,
}

/// Select an engine for the given request signals.
///
/// M0 placeholder; concrete implementation lands at M3 per ADR-0004.
pub fn select_engine(_header_hint: Option<&str>) -> Result<RoutingDecision, Error> {
    Err(Error::Routing("not yet implemented".into()))
}
