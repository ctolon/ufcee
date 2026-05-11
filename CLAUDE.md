# Unified Context Extraction Engine Proxy

Unified Context Extraction Engine Proxy coordinated by Claude Code sub-agents.
Each agent owns a specific concern; separation of concerns and quality are
enforced at the tool boundary by path-scoped rules and lifecycle hooks.

> **Canonical agent catalog**: see `AGENTS.md` (vendor-neutral, also supports
> Cursor / GitHub Copilot / OpenAI Codex / Google Jules / Aider).

## Tech Stack

- **Programming Language**: Rust (latest stable), Python ≥3.12, PyO3 for FFI + Python SDK. Core service in Rust.
- **Version Control**: Git, trunk-based development.
- **CI/CD**: GitHub Actions.
- **Ecosystem**: Tokio.

## Architecture principles

- Clean code
- OOP principles
- DRY
- Security first
- High performance first
- Test-driven development
- Modular crates

## Collaboration Protocol

**User-driven collaboration, not autonomous execution.**
Every task follows: **Question → Options → Decision → Draft → Approval**

- Agents MUST ask "May I write this to [filepath]?" before using Write/Edit tools
- Agents MUST show drafts or summaries before requesting approval
- Multi-file changes require explicit approval for the full changeset
- No commits without user instruction

## Claude-Code-specific extensions

### Output style

The session uses the `ucee-house` output style (`.claude/output-styles/ucee-house.md`):
terse, factual, no emoji, no marketing adjectives, no AI self-reference,
direct declarative tone. Conclusions first, supporting facts second.

### Memory layout

- `.claude/agent-memory/` is the shared coordination plane; **single-writer
  invariant** via `L1-memory-coordinator`.
- `inject_coordination.sh` (UserPromptSubmit hook) injects the tail of
  `COORDINATION.md` into Claude's context at every prompt.
- Personal notes use per-user Anthropic auto-memory
  (`~/.claude/projects/.../memory/`) and never enter git.
- Full protocol: `.claude/agent-memory/README.md`.

### Branch topology

- `main` — Rust + Python code + `.claude/` config + `AGENTS.md` + `CLAUDE.md` (this file).
- `docs` — long-form documentation in diataxis structure (tutorials / how-to / reference / explanation).
- `helm` — Helm chart (graduates to its own repository at milestone M9).
- `/sync-docs` and `/sync-helm` skills operate across branches via `git worktree`.

### Hook summary

| event | scripts | purpose |
|---|---|---|
| PreToolUse (Write/Edit) | `no_ai_slop.sh` | Block writes containing banned slop patterns |
| PreToolUse (Bash) | `guard_bash.sh`, `guard_commit_message.sh` | Deny destructive ops; block AI refs in commit messages |
| PostToolUse (Write/Edit) | `post_lint_rust.sh`, `post_lint_python.sh`, `post_slop_scan.sh` | rustfmt / clippy / ruff warn; defense-in-depth slop re-scan |
| UserPromptSubmit | `inject_coordination.sh` | Inject `COORDINATION.md` tail into context |
| SessionStart | `session_start.sh` | Surface active locks + top tasks + branch; reap stale locks |
| Stop | `session_summary.sh` | Write session summary to `sessions/<UTC>-<id>.md` |
| PreCompact | `dump_state.sh` | Dump in-flight state to `CHECKPOINT.md` |

### MCP integrations

This project does not currently configure project-scoped MCP servers. Add via
`.claude/settings.json` if any agent needs external MCP access. Some
user-scoped MCP servers (e.g., Gmail, Drive, Calendar) may be available
depending on the developer's `~/.claude/` setup but are not required.

## Useful commands

| command | purpose |
|---|---|
| `cargo check --workspace` | quick compile check (M0 onward) |
| `cargo test --workspace` | run tests |
| `/feature <name>` | bootstrap a feature branch + TASKS row |
| `/review-branch` | 3-way parallel review (code + security + perf) |
| `/agent-coordinate claim <slug> <task_id>` | claim a file lock |
| `/cleanup-ai-slop` | standalone slop scan |
| `/threat-model` | security audit of current diff or path |

## Where to look

- Why a decision was made → `.claude/agent-memory/DECISIONS/`.
- Who owns a concern → `.claude/agents/L2-*.md` (concern owners).
- How a rule is enforced → `.claude/rules/<rule>.md` + the corresponding hook script.
- Project status / next-up tasks → `.claude/agent-memory/COORDINATION.md` + `TASKS.md`.
- Performance budgets → `.claude/agent-memory/PERF_BUDGETS.md`.
