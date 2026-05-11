#!/usr/bin/env bash
# PostToolUse hook for Write|Edit on *.py files.
# Non-blocking: emits warnings to stderr if ruff check/format finds issues.

set -euo pipefail

input="$(cat)"
file_path="$(jq -r '.tool_input.file_path // empty' <<<"$input")"
[[ -z "$file_path" ]] && exit 0
[[ "$file_path" != *.py ]] && exit 0
[[ ! -f "$file_path" ]] && exit 0

if ! command -v ruff >/dev/null 2>&1; then
  exit 0
fi

check_out=$(ruff check "$file_path" 2>&1 || true)
if [[ -n "$check_out" ]] && ! printf '%s' "$check_out" | rg -q 'All checks passed|^$'; then
  printf 'POST-LINT[ruff check] %s:\n%s\n' "$file_path" "$check_out" >&2
fi

fmt_out=$(ruff format --check "$file_path" 2>&1 || true)
if [[ -n "$fmt_out" ]] && ! printf '%s' "$fmt_out" | rg -q 'already formatted'; then
  printf 'POST-LINT[ruff format] %s:\n%s\n' "$file_path" "$fmt_out" >&2
fi

exit 0
