#!/usr/bin/env bash
# UserPromptSubmit hook: injects current coordination state into Claude's context.
set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
COORD_FILE="${PROJECT_DIR}/.claude/agent-memory/COORDINATION.md"

[[ ! -f "$COORD_FILE" ]] && exit 0
content="$(tail -c 4096 "$COORD_FILE")"
[[ -z "${content// }" ]] && exit 0

jq -n --arg c "$content" '{
  hookSpecificOutput: {
    hookEventName: "UserPromptSubmit",
    additionalContext: ("# Agent coordination state (tail)\n\n" + $c)
  }
}'
exit 0
