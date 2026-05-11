---
paths:
  - "crates/ucee-adapters/**/*.rs"
---

# Adapter contract rules

## Trait

Every adapter implements the `Adapter` trait defined in `ucee-core::adapter`:

- `fn name(&self) -> &'static str`
- `fn capabilities(&self) -> Capabilities`
- `async fn convert(&self, req: ConvertRequest) -> Result<ConvertResponse, AdapterError>`
- `async fn health(&self) -> Result<HealthStatus, AdapterError>`

`compat_type` is one of: `Docling`, `External`, `DoclingExternal`, `Tika`, `Custom`.

## Contract tests

- Every adapter ships `#[cfg(test)] mod contract { ... }` invoking `enginetest::run_contract_suite::<Self>()`.
- The contract suite covers: request building, response parsing, MIME accept set, health roundtrip, error mapping, large-file streaming.
- Fixtures live in `crates/ucee-adapters-fixtures/` and are shared across all adapters. Adapter-specific fixtures are co-located under that crate's submodule.

## Configuration

- Each adapter's config keys are namespaced by adapter name in `Config::engines[name]`.
- Required fields: `url`, `mime_types`, `compat_type`.
- Optional fields: `timeouts`, `conn_pool`, `circuit_breaker`, `rate_limit`, `tls`, `path_overrides`.
- Secrets resolved via `api_key_env` env var name (NOT inline values).

## Naming

- Engine names must match `^[a-z0-9][a-z0-9-]{0,31}$`.
- Crate name pattern: `ucee-adapter-<engine>` (e.g., `ucee-adapter-docling`).
- One adapter per crate; multi-engine adapters live in `ucee-adapter-custom` only.

## Versioning

- Each adapter's `Cargo.toml` declares a `package.metadata.ucee.contract_version = "N"`.
- Breaking changes to the `Adapter` trait increment `contract_version` and require an ADR.
- Adapters whose `contract_version` is lower than the core's are rejected at registry build.
