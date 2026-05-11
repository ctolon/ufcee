# 0003 — Adapter trait surface + contract_version policy

- Status: accepted
- Date: 2026-05-11
- Deciders: human + L2-adapter-custom-contract + L2-system-architect

## Context

M1 needs the final `Adapter` trait surface that every engine adapter will
implement, plus a mechanism to detect contract drift when the trait evolves
in later milestones.

## Decision

### Trait surface (contract version 1)

```rust
pub const CURRENT_CONTRACT_VERSION: u32 = 1;

pub trait Adapter: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn capabilities(&self) -> Capabilities;
    fn contract_version(&self) -> u32 { CURRENT_CONTRACT_VERSION }
    fn convert(&self, req: ConvertRequest)
        -> impl Future<Output = Result<ConvertResponse, Error>> + Send;
    fn health(&self)
        -> impl Future<Output = Result<HealthStatus, Error>> + Send;
}
```

### Bound: `Send + Sync + 'static`

- `Send` — adapters move across tokio tasks (multi-tenant request handling, shared registry).
- `Sync` — shared registry hands out references; adapter handles concurrent calls.
- `'static` — adapters live for the process lifetime; no borrowed state.

### `contract_version` policy

- Single source of truth: `ucee_core::CURRENT_CONTRACT_VERSION` (`1` for M1).
- Adapters return this constant by default. An adapter frozen against an
  older trait surface overrides `contract_version` to return its locked value.
- Registry build (lands at M2) rejects any adapter where
  `contract_version() < CURRENT_CONTRACT_VERSION`.
- Bumps to `CURRENT_CONTRACT_VERSION` require an ADR amendment.

### Mirrored in `[package.metadata.ucee]`

Each adapter crate declares `contract_version` redundantly in its `Cargo.toml`:

```toml
[package.metadata.ucee]
contract_version = 1
```

This lets static tooling (`cargo-deny` rules, CI scripts) check conformance
without instantiating the adapter.

## Contract test suite (`enginetest::run_contract_suite::<A>()`)

The harness in `crates/ucee-adapters-fixtures/src/lib.rs` exercises every
adapter against fixed assertions:

1. **`check_contract_version`** — `adapter.contract_version() >= CURRENT_CONTRACT_VERSION`.
2. **`check_name`** — name matches `^[a-z0-9][a-z0-9-]{0,31}$`.
3. **`check_capabilities`** — `capabilities().mime_types` is non-empty; `compat_type` set.
4. **`check_convert_empty_body`** — empty body returns `Error::Adapter(_)` (not a different variant).
5. **`check_health_roundtrip`** — `health()` call completes (Healthy or Unhealthy; both acceptable).

Adapter crates' `#[cfg(test)] mod contract` (or `tests/contract.rs`)
constructs an adapter pointed at a `wiremock::MockServer` and invokes the
suite. The harness uses `assert!` so failures map to ordinary test
failures.

## Why not a sealed trait?

Considered making `Adapter` a sealed trait so only the workspace can
implement it. Rejected: third-party adapters are a stated goal of the
project (`Custom` `compat_type`). Sealing would block adapter-as-plugin
use cases.

## Why not `async-trait`?

`async-trait` adds a boxing layer on every async call. Native
`async fn in traits` (stable Rust 1.75+, with `impl Future + Send` return
types for `Send` bounds) is zero-overhead and fully stable in the
`edition = "2024"` toolchain.

## Consequences

Easier:
- Every adapter is interchangeable from the registry's point of view.
- Contract drift is caught at compile time (trait change → adapters fail to
  compile) AND at runtime (`contract_version` check at registry build).
- Test harness amortizes per-adapter test boilerplate; new adapters need
  only ~30 lines of contract-test glue.

Harder:
- Bumping the trait surface is a breaking change for every adapter; needs
  an ADR amendment and a coordinated rollout.
- `async fn in traits` does not produce `dyn Adapter` directly without a
  workaround (boxed futures); the registry will use `Arc<dyn Adapter>` via
  a thin wrapper crate if needed (deferred to M2 if the issue surfaces).

## References

- `crates/ucee-core/src/adapter.rs` — trait definition.
- `crates/ucee-adapters-fixtures/src/lib.rs` — contract harness.
- `crates/ucee-adapter-docling/` — first adapter using harness.
- ADR-0001 (meta-config), ADR-0002 (workspace layout).
