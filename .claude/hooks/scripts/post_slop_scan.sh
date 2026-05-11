#!/usr/bin/env bash
# PostToolUse hook for Write|Edit.
# Re-scans the written file. Pre-hook should have blocked any BLOCK pattern;
# a hit here means Pre was bypassed or buggy: log to .claude/agent-memory/sessions/<id>.log
# and emit a stderr warning. Never blocks (the write already happened).

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
PATTERNS_FILE="${PROJECT_DIR}/.claude/hooks/lib/slop_patterns.txt"

input="$(cat)"
file_path="$(jq -r '.tool_input.file_path // empty' <<<"$input")"
[[ -z "$file_path" ]] && exit 0
[[ ! -f "$file_path" ]] && exit 0
[[ ! -f "$PATTERNS_FILE" ]] && exit 0

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
esac

session_id="${CLAUDE_SESSION_ID:-unknown}"
log_dir="${PROJECT_DIR}/.claude/agent-memory/sessions"
mkdir -p "$log_dir"
log_file="${log_dir}/${session_id}.log"
ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

while IFS=$'\t' read -r pattern severity desc; do
  [[ -z "${pattern// }" ]] && continue
  [[ "$pattern" == \#* ]] && continue
  if rg -qP -i -- "$pattern" "$file_path" 2>/dev/null; then
    case "$severity" in
      BLOCK)
        printf '[%s] BLOCK-ESCAPE file=%s pattern=%s desc=%s\n' \
          "$ts" "$rel_path" "$pattern" "$desc" >> "$log_file"
        printf 'WARNING: PostToolUse caught BLOCK-severity slop in %s (%s); investigate Pre-hook gap.\n' \
          "$rel_path" "$desc" >&2
        ;;
      WARN)
        printf '[%s] WARN file=%s pattern=%s desc=%s\n' \
          "$ts" "$rel_path" "$pattern" "$desc" >> "$log_file"
        ;;
    esac
  fi
done < "$PATTERNS_FILE"

exit 0
