#!/usr/bin/env bash
# SessionStart hook: surfaces active coordination state and reaps stale locks.
set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
LOCKS_DIR="${PROJECT_DIR}/.claude/agent-memory/LOCKS"
TASKS_FILE="${PROJECT_DIR}/.claude/agent-memory/TASKS.md"

if [[ -d "$LOCKS_DIR" ]]; then
  now_epoch="$(date +%s)"
  for lock in "$LOCKS_DIR"/*.lock; do
    [[ ! -f "$lock" ]] && continue
    acquired=$(rg -oP '^acquired_utc\s*=\s*"\K[^"]*' "$lock" 2>/dev/null || echo "")
    ttl=$(rg -oP '^ttl_seconds\s*=\s*\K\d+' "$lock" 2>/dev/null || echo "1800")
    host=$(rg -oP '^host\s*=\s*"\K[^"]*' "$lock" 2>/dev/null || echo "")
    pid=$(rg -oP '^pid\s*=\s*\K\d+' "$lock" 2>/dev/null || echo "")
    [[ -z "$acquired" ]] && continue
    acq_epoch=$(date -d "$acquired" +%s 2>/dev/null || echo "0")
    age=$((now_epoch - acq_epoch))
    if [[ "$age" -gt "$ttl" ]]; then
      if [[ "$host" != "$(hostname)" ]] || { [[ -n "$pid" ]] && ! kill -0 "$pid" 2>/dev/null; }; then
        echo "session_start: reaping stale lock $(basename "$lock") (age=${age}s)" >&2
        rm -f "$lock"
      fi
    fi
  done

  active=$(find "$LOCKS_DIR" -maxdepth 1 -name '*.lock' -type f 2>/dev/null | wc -l)
  if [[ "$active" -gt 0 ]]; then
    echo "session_start: $active active lock(s):" >&2
    for lock in "$LOCKS_DIR"/*.lock; do
      [[ ! -f "$lock" ]] && continue
      agent=$(rg -oP '^agent\s*=\s*"\K[^"]*' "$lock" 2>/dev/null || echo "?")
      task=$(rg -oP '^task_id\s*=\s*"\K[^"]*' "$lock" 2>/dev/null || echo "?")
      echo "  $(basename "$lock" .lock): agent=$agent task=$task" >&2
    done
  fi
fi

if [[ -f "$TASKS_FILE" ]]; then
  open_tasks=$(rg '^\| T-' "$TASKS_FILE" 2>/dev/null | rg '\| (open|claimed|in-progress) \|' 2>/dev/null | head -n 5)
  if [[ -n "$open_tasks" ]]; then
    echo "session_start: top open tasks:" >&2
    echo "$open_tasks" >&2
  fi
fi

if command -v git >/dev/null 2>&1; then
  branch=$(cd "$PROJECT_DIR" 2>/dev/null && git branch --show-current 2>/dev/null || true)
  [[ -z "$branch" ]] && branch="(no-branch)"
  echo "session_start: branch=$branch" >&2
fi

exit 0
