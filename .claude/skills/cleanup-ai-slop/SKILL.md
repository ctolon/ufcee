---
name: cleanup-ai-slop
description: Standalone scan + suggested cleanup of any AI-slop, banned phrases, or emoji that slipped past PreToolUse hook.
arguments: "[<path>]"
context: fork
agent: L4-slop-linter
allowed-tools: Read, Grep, Glob, Bash(rg *)
---

# /cleanup-ai-slop — standalone slop scan

## Purpose

Re-run the patterns from `.claude/hooks/lib/slop_patterns.txt` against the
entire tree (or a specified subpath). Catches anything that may have slipped
past the PreToolUse hook — e.g., due to fail-open bootstrap, or content edited
outside Claude Code.

This skill's directory is allowlisted in `no_ai_slop.sh` so its files may
reference banned patterns verbatim for documentation.

## Steps

1. Read `.claude/hooks/lib/slop_patterns.txt`.
2. Scan the target path with `rg -P -i` for each BLOCK pattern.
3. Group results by file.
4. Suggest fixes (preferred replacement language) per match.

## Outputs

- Per-file findings with severity (BLOCK / WARN), pattern, offending substring.
- Suggested replacements where the wording is obvious.
- Files allowlisted (i.e., excluded from the scan) listed at the end for transparency.
