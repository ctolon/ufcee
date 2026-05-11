# Coordination snapshot

_Rewritten by `L1-memory-coordinator` whenever active state changes. The tail of this
file is injected into Claude's context by `inject_coordination.sh` at every
UserPromptSubmit._

Last updated: 2026-05-11 (post-M2)

## Active agents

(none — M0, M1, M2 landed; ready for next sprint)

## Active locks

(none)

## Recent decisions

- 2026-05-11 — ADR-0001 accepted: Claude Code meta-configuration plan adopted.
- 2026-05-11 — ADR-0002 accepted: Rust workspace + 10-crate layout for M0.
- 2026-05-11 — ADR-0003 accepted: Adapter trait surface + `contract_version` policy for M1.
- 2026-05-11 — ADR-0004 accepted: HTTP server architecture (axum + Registry + DynAdapter) for M2.

## Pending questions

(none)

## Next-up tasks

See `TASKS.md`. M0–M2 done. Next-up is **T-2026-0005** (M3: routing engine — MIME + ext + facade selector with full precedence chain), owner `L2-routing-engine`.

## Landed milestones

| ms | commit | summary |
|---|---|---|
| meta-config bootstrap | `e28d902` | All `.claude/` infra + AGENTS.md + CLAUDE.md (main); diagrams + roadmap (docs); helm skeleton (helm) |
| M0 | `471c831` | Workspace + 10 crate skeletons + CI + deny + ADR-0002 |
| M1 | `932e0e7` | Adapter trait + harness + docling adapter + ADR-0003 |
| M2 | _(next commit)_ | axum server + Registry + DynAdapter + Docling facade + ADR-0004 |
