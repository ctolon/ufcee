---
name: threat-model
description: Security audit of the current diff or a specified path. Runs L4-security-auditor in fork.
arguments: "[<path>]"
paths:
  - "**/*.rs"
  - "**/*.py"
context: fork
agent: L4-security-auditor
allowed-tools: Read, Grep, Glob, Bash(git diff *), Bash(git log *)
---

# /threat-model — security audit

## Purpose

Threat-model the current diff (default) or a specified path. Runs
`L4-security-auditor` in forked context to keep the auditor's investigation
read-only and isolated.

## Steps

1. Determine scope (current diff vs path argument).
2. Hand off to `L4-security-auditor`.
3. Aggregate findings by severity.
4. File Critical / High in `agent-memory/COORDINATION.md`.

## Outputs

- Findings report with severity (Critical / High / Medium / Low / Informational).
- File:line citations with violated rule.
- Suggested fixes.
- ADR proposal if a systemic change is warranted.
