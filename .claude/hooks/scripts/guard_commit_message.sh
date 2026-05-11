#!/usr/bin/env bash
# PreToolUse hook for Bash, scoped to `git commit` commands.
# Extracts the commit message from -m, heredoc (<<'EOF'), or -F flag forms,
# then blocks if the message contains AI references, emoji, or any BLOCK-severity
# pattern from slop_patterns.txt.

set -euo pipefail

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
PATTERNS_FILE="${PROJECT_DIR}/.claude/hooks/lib/slop_patterns.txt"

input="$(cat)"
[[ "$(jq -r '.tool_name // empty' <<<"$input")" != "Bash" ]] && exit 0
cmd="$(jq -r '.tool_input.command // empty' <<<"$input")"

if [[ ! "$cmd" =~ (^|[[:space:];&|])git[[:space:]]+commit ]]; then
  exit 0
fi

emit_deny() {
  jq -n --arg r "$1" '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"deny",permissionDecisionReason:$r}}'
  exit 2
}

# --- Extract message from each known form ---
msg=""

# Double-quoted -m "..."
while IFS= read -r m; do msg+="${m}"$'\n'; done < <(
  printf '%s' "$cmd" | rg -oP -- '-m\s+"\K[^"]*' 2>/dev/null || true
)
# Single-quoted -m '...'
while IFS= read -r m; do msg+="${m}"$'\n'; done < <(
  printf '%s' "$cmd" | rg -oP -- "-m\s+'\K[^']*" 2>/dev/null || true
)

# Heredoc form: <<EOF, <<'EOF', <<"EOF", <<-EOF, <<-'EOF', <<-"EOF"
if [[ "$cmd" == *"<<"*"EOF"* ]]; then
  body=$(printf '%s' "$cmd" | sed -nE "/<<-?['\"]?EOF['\"]?/,/^[[:space:]]*EOF[[:space:]]*\$/p" | sed '1d;$d')
  msg+="${body}"$'\n'
fi

# -F file
if [[ "$cmd" =~ -F[[:space:]]+([^[:space:]]+) ]]; then
  msg_file="${BASH_REMATCH[1]}"
  [[ -f "$msg_file" ]] && msg+="$(cat "$msg_file")"$'\n'
fi

[[ -z "${msg// }" ]] && exit 0

# --- AI reference patterns (always denied) ---
for p in 'Co-Authored-By:[[:space:]]*Claude' 'Generated with Claude Code' 'claude\.com/claude-code'; do
  if printf '%s' "$msg" | rg -qP -- "$p" 2>/dev/null; then
    emit_deny "commit message contains AI reference: $p"
  fi
done

# Any emoji
if printf '%s' "$msg" | rg -qP -- '[\x{1F300}-\x{1FAFF}\x{2600}-\x{27BF}]' 2>/dev/null; then
  emit_deny "commit message contains emoji"
fi

# Slop patterns from shared file
if [[ -f "$PATTERNS_FILE" ]]; then
  while IFS=$'\t' read -r pattern severity desc; do
    [[ -z "${pattern// }" ]] && continue
    [[ "$pattern" == \#* ]] && continue
    [[ "$severity" != "BLOCK" ]] && continue
    if printf '%s' "$msg" | rg -qP -i -- "$pattern" 2>/dev/null; then
      emit_deny "commit message contains banned pattern (${desc}): ${pattern}"
    fi
  done < "$PATTERNS_FILE"
fi

exit 0
