//! Shared adapter test fixtures and the contract test harness.
//!
//! Every adapter crate ships a `#[cfg(test)] mod contract` that invokes
//! [`run_contract_suite`] against itself. The suite enforces the public
//! [`ucee_core::Adapter`] contract uniformly across all adapters.
//!
//! M0 placeholder. Concrete checks land at M1 via ADR-0003 when the trait
//! is finalized.

use ucee_core::Adapter;

/// Run the full contract suite against an adapter type.
///
/// M0 placeholder. The harness intentionally does no work yet so that
/// downstream adapter scaffolds can already declare the conformance entry
/// point.
pub fn run_contract_suite<A: Adapter + Default>() {
    let _adapter = A::default();
    // Concrete checks populated at M1.
}
