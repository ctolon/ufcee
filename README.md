# Unified Context Extraction Engine (UCEE) Proxy

A stateless, cloud-native HTTP proxy that routes document-extraction requests to
multiple backends (docling, kreuzberg, unstructured, markitdown, paddleocr,
Apache Tika, and arbitrary docling-compatible custom engines) under unified
Docling / External facades. Rust core (Tokio async, modular crates) with a
Python SDK via PyO3.

## Status

Bootstrapping. Claude Code meta-configuration is in place (52 specialized
agents, 17 skills, 10 hook scripts, 9 path-scoped rules, full memory
coordination protocol). Product code starts at milestone M0; see
`.claude/agent-memory/TASKS.md`.

## Repository layout

- `crates/` — Rust workspace crates (added at M0).
- `python/ucee/` — Python SDK (added at M5).
- `bindings/python/` — PyO3-side glue (added at M5).
- `.claude/` — Claude Code configuration (agents, skills, hooks, rules, memory).
- `CLAUDE.md` — project collaboration protocol.

## Branches

- `main` — code + Claude Code configuration.
- `docs` — long-form documentation (diataxis).
- `helm` — Helm chart (graduates to its own repository at M9).

## Build (after M0)

```
cargo build --workspace
```

## License

TBD.
