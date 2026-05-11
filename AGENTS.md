# AGENTS.md

This file documents the agent / sub-agent system used by Claude Code (and any
other AI coding tool that honors the AGENTS.md convention — Cursor, GitHub
Copilot, OpenAI Codex, Google Jules, Aider) when working on UCEE Proxy.

- For Claude-Code-specific extensions (memory, hooks, slash commands, output
  style), see `CLAUDE.md`.
- For project context (problem domain, stack, status), see `README.md`.

## Project overview (agent-focused)

UCEE Proxy is a stateless, cloud-native HTTP proxy that routes document-
extraction requests across multiple backends (docling, kreuzberg, unstructured,
markitdown, paddleocr, Apache Tika, plus arbitrary docling-compatible custom
engines) behind unified Docling / External facades. The proxy enforces
capability-aware facade selection, MIME-driven routing with magic-byte
detection, bounded streaming spool, per-engine circuit breakers and rate
limiters, an SSRF validator, and structured observability (JSON logs,
Prometheus metrics, OpenTelemetry spans).

The core service is in Rust (latest stable, Tokio async, modular crates).
Python ≥3.12 SDK via PyO3.

## Agent layers (52 agents in 4 layers)

| layer | count | role |
|---|---|---|
| L1 | 4 | Orchestration: orchestrator, memory-coordinator, task-router, dependency-tracker |
| L2 | 23 | Concern owners: system-architect, api-designer, normalizer-designer, routing-engine, mime-router, ext-router, facade-selector, response-normalizer, 6 adapters, adapter-custom-contract, pyo3-bridge, python-sdk-designer, config-loader, streaming-spool-expert, circuit-breaker-designer, rate-limit-designer, ssrf-defender, secrets-handler |
| L3 | 13 | Lifecycle: feature-planner, rust-implementer, python-implementer, pyo3-implementer, unit-test-author, integration-test-author, contract-test-enforcer, fuzz-test-author, property-test-author, code-reviewer, security-reviewer, performance-reviewer, doc-writer |
| L4 | 12 | Cross-cutting: security-auditor, dependency-scanner, perf-benchmarker, perf-profiler, observability-engineer, github-actions-engineer, helm-author, k8s-manifest-author, changelog-curator, release-engineer, sbom-author, slop-linter |

Per-agent specifications: `.claude/agents/<name>.md`. Each file declares
its model, effort, isolation, tool allowlist, and the when-to-invoke trigger.

## Executable commands

| command | purpose |
|---|---|
| `cargo build --workspace` | build all crates |
| `cargo test --workspace` | run tests |
| `cargo fmt -- --check` | check formatting |
| `cargo clippy -- -D warnings` | lint |
| `cargo bench` | criterion benchmarks |
| `cargo deny check` | dependency policy |
| `cargo audit` | vulnerability scan |
| `cargo cyclonedx` | generate SBOM |
| `ruff check && ruff format --check` | Python lint |
| `pytest` | Python tests |
| `pyright` | Python type check |
| `maturin develop` | build Python bindings locally |
| `helm lint helm/ufcee` | lint chart (on `helm` branch) |
| `helm template helm/ufcee \| kubeval` | validate templates (on `helm` branch) |

## Code & git conventions

- **Style**: see `.claude/rules/rust-style.md`, `.claude/rules/python-style.md`,
  `.claude/rules/tokio-async-rules.md`, `.claude/rules/pyo3-rules.md`. These
  are path-scoped and auto-attached to relevant files.
- **Commits**: Conventional Commits (`type(scope): subject`), ≤72-char
  subject, body wrapped at 100, no emoji, no AI references. The
  `guard_commit_message.sh` hook enforces this.
- **Branches**:
  - `main` — Rust + Python code + `.claude/` configuration + `AGENTS.md` + `CLAUDE.md`.
  - `docs` — long-form documentation (diataxis: tutorials / how-to / reference / explanation).
  - `helm` — Helm chart (graduates to its own repository at milestone M9).
- **No commits without explicit human instruction.**

## Memory coordination protocol

`.claude/agent-memory/` is the shared coordination plane (committed to git).
**Single-writer invariant**: only `L1-memory-coordinator` mutates the
directory; other agents request changes via `/agent-coordinate <verb> <args>`.

| file | role |
|---|---|
| `README.md` | full protocol spec |
| `COORDINATION.md` | live snapshot (injected at every UserPromptSubmit) |
| `TASKS.md` | backlog + status board with change log |
| `DEPS.md` | workspace dependency-graph snapshot |
| `PERF_BUDGETS.md` | per-bench upper bounds (enforced at M10) |
| `DECISIONS/NNNN-<slug>.md` | architectural decision records |
| `LOCKS/<slug>.lock` | file locks (gitignored; TOML with TTL + heartbeat) |
| `sessions/<UTC>-<id>.md` | session summaries (gitignored; Stop hook output) |
| `CHECKPOINT.md` | pre-compaction state dump (gitignored) |

Personal notes live in per-user `~/.claude/projects/.../memory/` (Anthropic
auto-memory). They never enter git.

## Agent boundaries

Agents MUST NOT:

- Commit without explicit user instruction (per CLAUDE.md collaboration protocol).
- Push to a remote without explicit user instruction.
- Force-push to `main` or `master` (hard-blocked at the settings + hook layer).
- Read `.env`, `.env.*`, `secrets/`, `*.key`, `*.pem` (hard-blocked).
- Pipe `curl` or `wget` into a shell (hard-blocked).
- Include any AI self-reference in code or commit messages (hard-blocked):
  - `Co-Authored-By: Claude` trailers
  - `Generated with Claude Code` taglines
  - `claude.com/claude-code` URLs
  - Robot emoji or any other emoji
- Use AI-slop phrasing in code, commits, or documentation: `delve into`,
  `tapestry`, `in the realm of`, `seamlessly`, `unleash`, `cutting-edge`,
  `game-changer`, `revolutionize`, etc. The full list is in
  `.claude/hooks/lib/slop_patterns.txt`. Soft warnings: `leverage`, `robust`,
  `powerful`, `elegant`.
- Add a new external dependency without an entry in `.claude/agent-memory/DEPS.md`
  and an accompanying ADR.
- Use `unsafe` Rust without a `// SAFETY:` block citing the invariants.

## Slash-command skills (17)

Per-skill specifications: `.claude/skills/<name>/SKILL.md`. All slash-invocable.

| skill | trigger | purpose |
|---|---|---|
| `/feature` | manual | Bootstrap a feature branch + TASKS row + planning session |
| `/scaffold-crate` | manual | Generate a new Rust workspace crate skeleton |
| `/scaffold-adapter` | path-scoped (`crates/ucee-adapters/**`) | Generate an engine adapter with contract tests |
| `/scaffold-pyo3-binding` | path-scoped (`crates/ucee-py/**`, `python/ucee/**`) | PyO3 binding skeleton |
| `/contract-test` | path-scoped (`crates/ucee-adapters/**`) | Run engine contract suite |
| `/bench` | path-scoped (`crates/**`) | Run criterion benchmarks |
| `/snapshot-arch` | manual | Emit architecture snapshot to `docs` branch |
| `/sync-helm` | user-only (`disable-model-invocation`) | Verify helm values ↔ Config sync |
| `/sync-docs` | path-scoped (`docs/**`) | Cherry-pick to `docs` branch |
| `/release` | user-only (`disable-model-invocation`) | Orchestrate semver release |
| `/threat-model` | path-scoped (`**/*.rs`, `**/*.py`) | Security audit of diff or path |
| `/perf-budget-check` | path-scoped (`crates/**`) | Verify bench deltas vs `PERF_BUDGETS.md` |
| `/normalizer-validate` | path-scoped (`crates/ucee-normalizer/**`) | Run normalizer fixture suite |
| `/agent-coordinate` | manual | Lock claim / release / status updates |
| `/review-branch` | manual | Parallel 3-way review (code + security + perf) |
| `/spec-feature` | manual | Author a feature spec on the `docs` branch |
| `/cleanup-ai-slop` | manual | Standalone slop scan |

## Hook system (10 hooks)

Hook scripts: `.claude/hooks/scripts/<name>.sh`. Registered in
`.claude/settings.json`.

| script | event | purpose |
|---|---|---|
| `no_ai_slop.sh` | PreToolUse Write\|Edit | Block writes containing banned patterns (allowlist exempts docs that document the patterns) |
| `guard_bash.sh` | PreToolUse Bash | Deny destructive ops; ask on installs / cluster mutation |
| `guard_commit_message.sh` | PreToolUse Bash | Block commits with AI references, emoji, or slop |
| `post_lint_rust.sh` | PostToolUse Write\|Edit | rustfmt + clippy warn on edited `.rs` |
| `post_lint_python.sh` | PostToolUse Write\|Edit | ruff warn on edited `.py` |
| `post_slop_scan.sh` | PostToolUse Write\|Edit | Defense-in-depth slop re-scan; log to `sessions/<id>.log` |
| `inject_coordination.sh` | UserPromptSubmit | Inject `COORDINATION.md` tail into Claude's context |
| `session_start.sh` | SessionStart | Surface active locks + top tasks + branch; reap stale locks |
| `session_summary.sh` | Stop | Write session summary to `sessions/<UTC>-<id>.md` |
| `dump_state.sh` | PreCompact | Dump in-flight state to `CHECKPOINT.md` |

Pattern source-of-truth: `.claude/hooks/lib/slop_patterns.txt`. Allowlisted
files (which may quote banned patterns verbatim for documentation):
`slop_patterns.txt`, `output-styles/ucee-house.md`, `agent-memory/README.md`,
`rules/*.md`, `skills/cleanup-ai-slop/*`, `agents/L4-slop-linter.md`,
`AGENTS.md`, `CLAUDE.md`.

## Common pitfalls

- **Trying to commit with a `Co-Authored-By: Claude` trailer**: the commit-
  message hook blocks. Remove the trailer.
- **Trying to write a file containing 🤖 or any other emoji**: the PreToolUse
  hook blocks. Use plain text.
- **Trying to `cat .env`**: the bash guard blocks. Use the designated
  secret-injection mechanism (env vars resolved via `api_key_env` names).
- **Trying to force-push to `main`**: hard-denied at both the settings layer
  and the hook layer.
- **Two agents editing the same crate**: claim a lock via
  `/agent-coordinate claim <slug> <task_id>` first. The lock TTL is 30
  minutes; heartbeat by re-acquire.
- **Editing `.claude/rules/*.md` without an ADR**: rules are policy; meaningful
  changes need a paper trail in `DECISIONS/`.

## Cross-references

- `CLAUDE.md` — Claude-Code-specific extensions (memory, output style, hooks summary).
- `.claude/agent-memory/README.md` — full memory protocol spec.
- `.claude/agents/<name>.md` — per-agent specifications (52 files).
- `.claude/skills/<name>/SKILL.md` — per-skill specifications (17 dirs).
- `.claude/rules/*.md` — path-scoped enforcement rules (9 on `main`, +1 on `docs`, +1 on `helm`).
- `.claude/hooks/lib/slop_patterns.txt` — canonical banned-pattern list.
- `.claude/agent-memory/DECISIONS/0001-meta-config.md` — ADR-0001 recording the meta-configuration decision.
