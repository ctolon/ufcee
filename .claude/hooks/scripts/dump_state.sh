#!/usr/bin/env bash
# PreCompact hook: dumps in-flight state to CHECKPOINT.md so post-compaction
# Claude can re-orient.
set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
LOCKS_DIR="${PROJECT_DIR}/.claude/agent-memory/LOCKS"
TASKS_FILE="${PROJECT_DIR}/.claude/agent-memory/TASKS.md"
COORD_FILE="${PROJECT_DIR}/.claude/agent-memory/COORDINATION.md"
CHECKPOINT="${PROJECT_DIR}/.claude/agent-memory/CHECKPOINT.md"

ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

mkdir -p "$(dirname "$CHECKPOINT")"

{
  echo "# Pre-compaction checkpoint"
  echo ""
  echo "Captured: ${ts}"
  echo "Session: ${CLAUDE_SESSION_ID:-unknown}"
  echo ""
  if [[ -d "$LOCKS_DIR" ]]; then
    echo "## Active locks"
    for lock in "$LOCKS_DIR"/*.lock; do
      [[ ! -f "$lock" ]] && continue
      echo ""
      echo "### $(basename "$lock" .lock)"
      echo '```'
      cat "$lock"
      echo '```'
    done
  fi
  if [[ -f "$TASKS_FILE" ]]; then
    echo ""
    echo "## In-progress tasks"
    rg '^\|' "$TASKS_FILE" 2>/dev/null | rg '\| in-progress \|' 2>/dev/null || echo "(none)"
  fi
  if [[ -f "$COORD_FILE" ]]; then
    echo ""
    echo "## Coordination snapshot (tail 2KB)"
    echo '```markdown'
    tail -c 2048 "$COORD_FILE"
    echo '```'
  fi
} > "$CHECKPOINT"

exit 0
