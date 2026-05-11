#!/usr/bin/env bash
# PreToolUse hook for Bash.
# Second-line defense; primary defense is settings.json deny/ask rules.
# Denies destructive ops and main-branch force-pushes; asks for installs / cluster mutations.

set -euo pipefail

input="$(cat)"
[[ "$(jq -r '.tool_name // empty' <<<"$input")" != "Bash" ]] && exit 0
cmd="$(jq -r '.tool_input.command // empty' <<<"$input")"
[[ -z "$cmd" ]] && exit 0

emit_deny() {
  jq -n --arg r "$1" '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"deny",permissionDecisionReason:$r}}'
  exit 2
}
emit_ask() {
  jq -n --arg r "$1" '{hookSpecificOutput:{hookEventName:"PreToolUse",permissionDecision:"ask",permissionDecisionReason:$r}}'
  exit 0
}

# --- DENY: destructive rm -rf on protected paths ---
if [[ "$cmd" =~ rm[[:space:]]+(-[a-zA-Z]*[rR][a-zA-Z]*[[:space:]]+)+/([[:space:]]|$) ]]; then
  emit_deny "rm -rf at filesystem root"
fi
if [[ "$cmd" =~ rm[[:space:]]+(-[a-zA-Z]*[rR][a-zA-Z]*[[:space:]]+)+~([[:space:]/]|$) ]]; then
  emit_deny "rm -rf at home directory"
fi
if [[ "$cmd" =~ rm[[:space:]]+(-[a-zA-Z]*[rR][a-zA-Z]*[[:space:]]+)+[^[:space:]]*\.git([[:space:]/]|$) ]]; then
  emit_deny "rm -rf on .git directory"
fi

# --- DENY: force-push to main/master ---
if [[ "$cmd" =~ git[[:space:]]+push.*--force ]] && [[ "$cmd" =~ (^|[[:space:]])(main|master)([[:space:]]|$) ]]; then
  emit_deny "force-push to main/master forbidden"
fi

# --- DENY: piping curl/wget into a shell ---
if [[ "$cmd" =~ (curl|wget)[^|]*\|[[:space:]]*(bash|sh|zsh) ]]; then
  emit_deny "piping curl/wget into shell"
fi

# --- DENY: reading .env files ---
if [[ "$cmd" =~ (cat|less|more|head|tail|bat)[[:space:]]+[^|]*\.env([[:space:].]|$) ]]; then
  emit_deny "reading .env files"
fi

# --- ASK: force-push to non-main branches ---
if [[ "$cmd" =~ git[[:space:]]+push.*--force ]]; then
  emit_ask "force-push requires approval"
fi

# --- ASK: package install/publish/add ---
if [[ "$cmd" =~ (^|[[:space:];&|])(cargo|pip|uv|npm|gem|brew|pipx)[[:space:]]+(install|publish|add) ]]; then
  emit_ask "package install/publish requires approval"
fi

# --- ASK: cluster mutation ---
if [[ "$cmd" =~ (^|[[:space:];&|])(kubectl|helm)[[:space:]]+(apply|install|upgrade|uninstall|delete|destroy|create|replace|patch) ]]; then
  emit_ask "cluster mutation requires approval"
fi

exit 0
