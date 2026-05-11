# Coordination snapshot

_Rewritten by `L1-memory-coordinator` whenever active state changes. The tail of this
file is injected into Claude's context by `inject_coordination.sh` at every
UserPromptSubmit._

Last updated: 2026-05-11 (post-M1)

## Active agents

(none — M0 and M1 landed; ready for next sprint)

## Active locks

(none)

## Recent decisions

- 2026-05-11 — ADR-0001 accepted: Claude Code meta-configuration plan adopted.
- 2026-05-11 — ADR-0002 accepted: Rust workspace + 10-crate layout for M0.
- 2026-05-11 — ADR-0003 accepted: Adapter trait surface + `contract_version` policy for M1.

## Pending questions

(none)

## Next-up tasks

See `TASKS.md`. M0 (T-2026-0001) and M1 (T-2026-0002, T-2026-0003) done.
Next-up is **T-2026-0004** (M2: axum HTTP server + Docling facade), owner `L2-api-designer`.

## Landed milestones

| ms | commit | summary |
|---|---|---|
| meta-config bootstrap | `e28d902` | All `.claude/` infra + AGENTS.md + CLAUDE.md (main); diagrams + roadmap (docs); helm skeleton (helm) |
| M0 | `471c831` | Workspace + 10 crate skeletons + CI + deny + ADR-0002 |
| M1 | _(next commit)_ | Adapter trait + harness + docling adapter + ADR-0003 |
