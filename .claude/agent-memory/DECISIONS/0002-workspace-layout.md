# 0002 — Workspace + crate layout

- Status: accepted
- Date: 2026-05-11
- Deciders: human + L2-system-architect

## Context

M0 needs an initial Rust workspace with the crate layout that future
milestones will populate. The choice affects every downstream agent's
ownership boundary and the dependency graph that `L1-dependency-tracker`
maintains in `.claude/agent-memory/DEPS.md`.

The reference Go implementation uses a single module with internal packages.
We want stronger separation in Rust via separate crates so each L2 concern
owner has a self-contained compilation unit and the test surface aligns with
the ownership boundary.

## Options

1. **Single binary crate**: everything in `src/`.
   - Pros: simplest.
   - Cons: violates the agent ownership model; no isolation; property-test
     boundaries collapse; rebuilds are coarse.

2. **Hand-curated crates (chosen)**: one crate per L2 concern with explicit
   public surface; `[workspace.dependencies]` centralizes external dep
   pinning.

3. **Crate-per-file**: maximally granular.
   - Pros: very tight scoping.
   - Cons: too much ceremony; cross-crate type sharing becomes painful;
     compile-time penalty.

## Decision

Adopt option 2. The M0 workspace ships these crates:

| crate | role | M0 status |
|---|---|---|
| `ucee-core` | Shared types and traits (`Adapter`, `Capabilities`, `CompatType`, `ConvertRequest`, `ConvertResponse`, `Error`) | skeleton |
| `ucee-config` | Three-layer config loader (defaults → YAML → env) | skeleton |
| `ucee-router` | Engine selection (MIME + ext + facade) | skeleton |
| `ucee-server` | axum HTTP server + facades | skeleton |
| `ucee-ssrf` | SSRF validator | skeleton |
| `ucee-streaming` | Bounded spool with `O_TMPFILE` fall-through | skeleton |
| `ucee-resilience` | Circuit breaker + token-bucket rate limiter | skeleton |
| `ucee-observability` | Logs + metrics + tracing | skeleton |
| `ucee-adapters-fixtures` | Adapter contract test harness | skeleton |
| `ucee` (bin) | Entrypoint; loads config, sets up obs, starts server | skeleton (tracing init working) |

Adapter crates (`ucee-adapter-docling`, `ucee-adapter-kreuzberg`,
`ucee-adapter-unstructured`, `ucee-adapter-markitdown`,
`ucee-adapter-paddleocr`, `ucee-adapter-tika`) land at M1 (docling) through
M4 (kreuzberg) and slot into M5–M7 (others). `ucee-py` (PyO3 bindings)
lands at M5.

## Workspace conventions

- `resolver = "3"`, `edition = "2024"`, `rust-version = "1.85"`.
- `[workspace.dependencies]` pins every external crate centrally. Members
  reference them via `dep.workspace = true`.
- `[workspace.lints.rust] unsafe_code = "deny"` workspace-wide. Removing
  the deny on any crate requires a follow-up ADR.
- `[workspace.lints.clippy] unwrap_used = "deny"`, `expect_used = "deny"`,
  `panic = "deny"`, `todo = "warn"`.

## Dependency policy

- New external dep requires an entry in `.claude/agent-memory/DEPS.md` plus
  a one-line rationale in this ADR's amendment history.
- `cargo deny check` enforces license and source allowlists.
- `cargo audit` runs on every PR that touches `Cargo.toml` / `Cargo.lock`.

## Consequences

Easier:
- Each L2 concern owner has a self-contained crate to work in.
- Property-test boundaries align with crate boundaries.
- Build times scale better as adapters are added (incremental compilation
  per crate).
- Versioning per crate is possible if any of them ever ships independently.

Harder:
- New cross-crate types must land in `ucee-core` first (or be added to a
  new "shared" crate via ADR amendment).
- Cross-adapter helpers go through `ucee-adapters-fixtures` to stay
  testable.

## Amendments

(none yet)

## References

- `Cargo.toml` workspace root.
- `.claude/agent-memory/DEPS.md` — declared graph.
- `.claude/agent-memory/DECISIONS/0001-meta-config.md` — meta-config.
