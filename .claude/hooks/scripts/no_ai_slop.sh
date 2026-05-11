#!/usr/bin/env bash
# PreToolUse hook for Write|Edit.
# Blocks any tool_input.content / new_string containing a BLOCK-severity pattern
# from .claude/hooks/lib/slop_patterns.txt, unless the target file is allowlisted.
# Allowlist intentionally lives in code (not data) so it cannot be widened from outside.

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
PATTERNS_FILE="${PROJECT_DIR}/.claude/hooks/lib/slop_patterns.txt"

input="$(cat)"
tool_name="$(jq -r '.tool_name // empty' <<<"$input")"
file_path="$(jq -r '.tool_input.file_path // empty' <<<"$input")"

case "$tool_name" in
  Write) content="$(jq -r '.tool_input.content // empty' <<<"$input")" ;;
  Edit)  content="$(jq -r '.tool_input.new_string // empty' <<<"$input")" ;;
  *)     exit 0 ;;
esac

[[ -z "$content" ]] && exit 0
[[ ! -f "$PATTERNS_FILE" ]] && exit 0   # fail-open during bootstrap (file may not exist yet)

rel_path="${file_path#"${PROJECT_DIR}"/}"
case "$rel_path" in
  .claude/hooks/lib/slop_patterns.txt) exit 0 ;;
  .claude/output-styles/ucee-house.md) exit 0 ;;
  .claude/agent-memory/README.md) exit 0 ;;
  .claude/agents/L4-slop-linter.md) exit 0 ;;
  .claude/rules/*.md) exit 0 ;;
  .claude/skills/cleanup-ai-slop/*) exit 0 ;;
  AGENTS.md) exit 0 ;;
  CLAUDE.md) exit 0 ;;
  .github/workflows/meta-config.yml) exit 0 ;;
esac

while IFS=$'\t' read -r pattern severity desc; do
  [[ -z "${pattern// }" ]] && continue
  [[ "$pattern" == \#* ]] && continue
  [[ "$severity" != "BLOCK" ]] && continue
  if printf '%s' "$content" | rg -qP -i -- "$pattern" 2>/dev/null; then
    jq -n --arg desc "$desc" --arg pattern "$pattern" '{
      hookSpecificOutput: {
        hookEventName: "PreToolUse",
        permissionDecision: "deny",
        permissionDecisionReason: ("banned pattern (" + $desc + "): " + $pattern)
      }
    }'
    exit 2
  fi
done < "$PATTERNS_FILE"

exit 0
