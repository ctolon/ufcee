# 0001 — Claude Code meta-configuration

- Status: accepted
- Date: 2026-05-11
- Deciders: human (project owner) + L2-system-architect

## Context

Before writing any UCEE Proxy code, the project needs a curated Claude Code setup so
that 45+ specialized agents can collaborate iteratively without conflicts, AI slop
cannot enter the codebase or commits, and the three operational concerns (code, docs,
helm chart) are cleanly separated into branches that can graduate to independent
repositories.

The reference Go implementation at `github.com/ctolon/owui-cee-proxy` provides a strong
architectural baseline (MIME-driven routing, capability-based facade selection,
mandatory contract testing, stateless design), but also has known gaps (SSRF CIDR
allowlist planned but not implemented, per-route circuit breaker granularity, request
signing between proxy and upstream, idempotency keys). The Rust rewrite explicitly
aims to close these gaps and produce zero observable bugs at GA, with explicit
performance budgets enforced in CI.

## Options

1. **Minimal config**: rely on default Claude Code behavior, no hooks, no rules, no
   agent specialization.
   - Pros: simple, no up-front setup.
   - Cons: AI slop will leak into code and commits; no parallel-agent coordination
     protocol; nothing to enforce style or security at the tool boundary.

2. **Heavy config, ad-hoc growth**: add agents / skills / hooks as needed when problems
   arise.
   - Pros: low up-front cost.
   - Cons: inconsistent; rule drift over time; agents disagree about ownership
     boundaries; AI slop has a foothold before guards are added.

3. **Curated, layered config up front (chosen)**: design L1 orchestration / L2 concern
   owners / L3 lifecycle / L4 cross-cutting; declarative rules with `paths:` glob;
   defense-in-depth hooks against slop and dangerous bash; explicit memory
   coordination protocol; three-branch git topology.
   - Pros: scales to many parallel agents; eliminates slop at four independent layers
     (output style → rules → PreToolUse hook → PostToolUse linter → commit-msg gate);
     enforces single-writer invariant on shared state; rules and roles named once,
     reused everywhere.
   - Cons: larger initial setup cost (~50+ files of meta-config before any product
     code).

## Decision

Adopt option 3. Specifically:

- **52 specialized agents** organized in 4 layers (L1=4, L2=23, L3=13, L4=12).
- **17 skills** (slash commands + reusable workflows).
- **10 hook scripts** across PreToolUse, PostToolUse, UserPromptSubmit, SessionStart,
  Stop, PreCompact.
- **9 path-scoped rule files** on `main`, +1 each on `docs` and `helm` branches.
- Repo-committed `.claude/agent-memory/` for shared protocol state; per-user
  Anthropic auto-memory for personal notes (never enters git).
- **Three-branch topology**: `main` (code + Claude Code config), `docs` (long-form
  prose), `helm` (deploy artifacts; graduates to own repo at M9).
- **Strict AI-slop prevention**: output style → CLAUDE.md rules → PreToolUse hard block
  on banned patterns → PostToolUse linter for drift → pre-commit message gate.

The full design is captured in the plan file referenced below.

## Consequences

Easier:

- New agents follow a uniform template (frontmatter + role + when-to-invoke + I/O).
- Reviewers can rely on rules being machine-enforced rather than humanly tracked.
- AI references cannot enter the codebase or commit history via Claude Code (the hook
  blocks at write-time and again at commit-time).
- The three-branch topology means PRs only touch one concern, so review scope is
  bounded.

Harder:

- One-line emergency changes that would violate a rule require either an ADR
  amendment or an allowlist tweak.
- The hook layer adds 5–60 s overhead per Write / Edit depending on which scripts
  fire (rust clippy is the longest at 30 s).
- The 52-agent catalog must be maintained: agent renames or merges require updating
  TASKS.md owner pointers.

## References

- Plan file: `/home/ctolon/.claude/plans/yeni-bir-projeye-ba-lad-m-deep-popcorn.md`.
- Approved 2026-05-11 via ExitPlanMode in the initial planning session.
- Reference Go implementation: `github.com/ctolon/owui-cee-proxy`.

## Follow-ups

- ADR-0002 will record the M0 workspace + crate layout decision.
- ADR-0003 will record the `Adapter` trait surface decision (M1).
- ADR-0004 will record the routing precedence decision (M3).
