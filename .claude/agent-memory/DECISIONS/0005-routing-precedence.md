# 0005 — Routing precedence chain

- Status: accepted
- Date: 2026-05-11
- Deciders: human + L2-routing-engine + L2-system-architect

## Context

M3 implements the engine-selection state machine that decides which
registered adapter handles each request. The reference Go implementation
documents a precedence chain; the design in
`docs/architecture/05-routing-precedence.md` codifies it. M3 makes it real
in `crates/ucee-router/`.

## Decision

### Precedence chain

1. **Explicit header** (`X-UCEE-Engine`) — operator override.
2. **Config rule** — path-glob matchers under `Config::routes` (placeholder
   until M7 config loader; the router skips this step today).
3. **MIME magic** — content sniffing of the request body via the `infer`
   crate, looked up in the per-MIME → engine map built from each
   adapter's `Capabilities::mime_types`.
4. **Extension** — filename extension table (`mime_from_extension`)
   resolves to a MIME, then re-uses the MIME map.
5. **Default** — `default_engine` configured on the router.

First match wins.

### Error mapping

- Header naming an unknown engine → `RoutingError::UnknownEngine(name)` →
  HTTP 409 Conflict.
- All steps fall through and no default set →
  `RoutingError::NoEngineMatches` → HTTP 415 Unsupported Media Type.

### Anti-spoof

When the client-declared `Content-Type` differs from the magic-byte sniff,
the sniff wins for routing decisions. The server forwards the sniffed MIME
to the adapter as the `effective_mime` so downstream logic sees the
canonical type. Per `security-rules.md`: "When magic differs from
Content-Type, magic wins for routing decisions."

### Why MIME beats extension

A client can rename `evil.exe` to `safe.pdf`. The `infer` crate looks at
the actual byte signature, so MIME magic is harder to spoof. Extension
remains a useful fallback for unrecognized formats (text/markdown,
docling-specific shapes that don't have magic).

### Observability

The chosen step is exposed both internally (`RoutingDecision.routing_path`)
and on the response as the `X-UCEE-Routing-Path` header so operators and
integration tests can attribute decisions. The Prometheus counter
`ucee_proxy_routing_decisions_total{routing_path}` (declared at M0,
populated at M6 observability) will count decisions per step.

## Implementation

- `ucee_router::Router` — read-only, built by `RouterBuilder`.
- `ucee_router::RoutingSignals { header_engine, mime_sniffed, extension }`
  — the handler constructs this from headers, body magic, and filename.
- `ucee_router::RoutingDecision { engine_name, routing_path }` — what the
  router returns on success.
- `ucee_router::RoutingPath { Header, Config, Mime, Ext, Default }` —
  attribution enum.
- `ucee_server::AppBuilder::default_engine(name)` — optional builder method
  that wires the default-step into the router.

`ucee_server` builds the router from the registry at app-construction time
(reading each adapter's declared `Capabilities::mime_types`). Hot-reload of
adapters requires rebuilding the router; deferred to a later milestone.

## What's NOT in M3

- **Config rule step (#2)**: the router skips it silently. Lands at M7
  with the full YAML config loader.
- **Per-route circuit-breaker selection**: routing decides the engine; CB
  decides whether the engine is currently allowed to receive traffic. Two
  separate concerns. M7 wires CB in.
- **Wildcard MIME patterns**: `image/*` style. M3 ships exact-match only.
  Wildcard support lands when an adapter declares it needs the pattern.

## Property tests

`crates/ucee-router/tests/properties.rs` certifies four invariants over
generated engine names (proptest, 64 cases each):

1. Header always wins for a registered engine.
2. Header naming an unregistered engine NEVER falls through to other steps.
3. MIME always beats extension when both would match different engines.
4. Default fallback always wins when nothing else matches.

## Consequences

Easier:
- Routing logic lives in one place (`ucee_router::Router`), unit- and
  property-tested in isolation, consumed via a clean `select` surface.
- Adapter authors don't need to know about routing; they declare
  `Capabilities::mime_types` and the router consumes them.
- Integration tests verify all five routing paths end-to-end.

Harder:
- The MIME index is rebuilt at registry-snapshot time. Hot-reload of
  adapters requires rebuilding the router; deferred.

## References

- `crates/ucee-router/{src,tests}/` — implementation + tests.
- `crates/ucee-server/src/routes/convert.rs` — handler consuming the router.
- `crates/ucee-server/tests/integration.rs` — 5-path end-to-end coverage.
- ADR-0001 (meta-config), ADR-0002 (workspace), ADR-0003 (Adapter trait),
  ADR-0004 (server architecture).
