#!/usr/bin/env bash
# Stop hook: writes a session summary to .claude/agent-memory/sessions/.
set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
PATTERNS_FILE="${PROJECT_DIR}/.claude/hooks/lib/slop_patterns.txt"

session_id="${CLAUDE_SESSION_ID:-unknown}"
sessions_dir="${PROJECT_DIR}/.claude/agent-memory/sessions"
mkdir -p "$sessions_dir"

ts="$(date -u +%Y-%m-%dT%H-%M-%SZ)"
out_file="${sessions_dir}/${ts}-${session_id}.md"

cd "$PROJECT_DIR" 2>/dev/null || exit 0
branch=$(git branch --show-current 2>/dev/null || true)
[[ -z "$branch" ]] && branch="(no-branch)"
modified=$( { git diff --name-only 2>/dev/null; git diff --name-only --staged 2>/dev/null; } | sort -u)
untracked=$(git ls-files --others --exclude-standard 2>/dev/null)

{
  echo "# Session summary"
  echo ""
  echo "- session_id: \`${session_id}\`"
  echo "- timestamp_utc: ${ts}"
  echo "- branch: \`${branch}\`"
  echo ""
  echo "## Modified files"
  if [[ -n "$modified" ]]; then
    while IFS= read -r f; do echo "- \`${f}\`"; done <<<"$modified"
  else
    echo "(none)"
  fi
  echo ""
  echo "## Untracked files"
  if [[ -n "$untracked" ]]; then
    while IFS= read -r f; do echo "- \`${f}\`"; done <<<"$untracked"
  else
    echo "(none)"
  fi
} > "$out_file.tmp"

if [[ -f "$PATTERNS_FILE" ]]; then
  while IFS=$'\t' read -r pattern severity desc; do
    [[ -z "${pattern// }" ]] && continue
    [[ "$pattern" == \#* ]] && continue
    [[ "$severity" != "BLOCK" ]] && continue
    if rg -qP -i -- "$pattern" "$out_file.tmp" 2>/dev/null; then
      echo "session_summary: aborting (BLOCK pattern in own output: $desc)" >&2
      rm -f "$out_file.tmp"
      exit 0
    fi
  done < "$PATTERNS_FILE"
fi

mv "$out_file.tmp" "$out_file"
exit 0
