# 0004 — HTTP server architecture (axum + Registry + DynAdapter)

- Status: accepted
- Date: 2026-05-11
- Deciders: human + L2-api-designer + L2-system-architect

## Context

M2 stands up the HTTP server that fronts the registered adapters and
exposes the Docling facade. Two design questions had to be settled:

1. How does the server hold a heterogeneous collection of `Adapter`
   implementations when the trait uses native `async fn in traits` (which
   isn't `dyn`-safe)?
2. How does routing decide which adapter handles a given request, given
   that the full precedence chain (header > config > MIME > ext > default)
   doesn't land until M3?

## Decision

### `DynAdapter` — object-safe wrapper

`ucee_core::DynAdapter` is a parallel trait that mirrors `Adapter` but
returns `Pin<Box<dyn Future<...>>>` for the async methods. It is
auto-implemented for every `A: Adapter` via a blanket impl, so adapter
authors continue to write the ergonomic `Adapter` form while the registry
holds `Arc<dyn DynAdapter>`. The `Box::pin` cost is paid once per call and
is negligible vs the network IO it precedes.

### `Registry`

`ucee_core::Registry` is a `HashMap<String, Arc<dyn DynAdapter>>` built at
startup and accessed read-only thereafter through `Arc<AppState>`. The
`register` method rejects adapters whose `contract_version` is lower than
`CURRENT_CONTRACT_VERSION` (per ADR-0003), so registry construction is the
gate at which contract drift is caught.

### `AppBuilder` + `AppState`

`ucee_server::AppBuilder` consumes a populated `Registry` and yields an
`axum::Router` ready for `axum::serve`. The state is an `Arc<AppState>`
holding the registry; handlers extract it via `State<Arc<AppState>>`.

### Routing for M2 — header-only

`/v1/convert/file` reads the engine name from the `X-UCEE-Engine`
request header. M3 will replace this with the full precedence chain.
Missing header → 400; unknown engine → 409; matching adapter is invoked
via `DynAdapter::convert`. M2 deliberately does NOT implement the MIME /
ext / config / default fallbacks — those land at M3 with property tests.

### `/v1/convert/source` — 501 until M3 / M7

The source-URL variant requires SSRF validation (M8) and the streaming
spool (M7) to be safe at arbitrary body sizes. Both are deferred. The
endpoint exists today and returns 501 with an explanatory body so
operators can detect mis-routing early.

### Ops endpoints

- `GET /healthz` — always 200 while the process is alive.
- `GET /readyz` — 200 if at least one adapter is registered. Per-adapter
  health probing lands at M6 with a periodic background poller.
- `GET /version` — JSON `{ name, version, engines }` where `engines` is
  the sorted list of registered adapter names.

## Why not `async-trait`?

Considered: have `Adapter` use the `async-trait` crate, which produces
`dyn`-safe traits via `Box<dyn Future>`. Rejected because:
- It adds a build-time macro dep and a runtime allocation on every call,
  same as our `DynAdapter` wrapper.
- The `DynAdapter` wrapper is local code under our control; we can evolve
  it without depending on an external proc-macro crate.
- Native `async fn in traits` gives ergonomic adapter authoring with zero
  overhead at the trait level. The boxing cost is paid only when crossing
  the `dyn` boundary at the registry.

## Why `Arc<AppState>` and not `axum`'s typed-state generics?

axum supports both. `Arc<AppState>` keeps the state-extraction signature
uniform across all handlers and works with the dyn-dispatched registry
cleanly. If M3+ needs request-local state (per-tenant, per-trace), it
will be added via axum middleware on top of this base.

## Consequences

Easier:
- Adapter authoring stays ergonomic (native `async fn`).
- Registry construction catches contract drift early.
- Handler signatures are uniform (`State<Arc<AppState>>`).

Harder:
- Every adapter call pays a heap allocation for the boxed future. Hot-path
  benchmarks at M6 will quantify this; if needed, M7 introduces a fast
  path for hot adapters via static dispatch (`Box<dyn DynAdapter>` →
  enum-based dispatch over known adapter types).

## References

- `crates/ucee-core/src/adapter.rs` — `DynAdapter` trait + blanket impl.
- `crates/ucee-core/src/registry.rs` — `Registry`.
- `crates/ucee-server/src/{lib.rs,state.rs,routes/*}` — server.
- `crates/ucee-server/tests/integration.rs` — 8-test e2e suite.
- ADR-0001 (meta-config), ADR-0002 (workspace), ADR-0003 (Adapter trait).
