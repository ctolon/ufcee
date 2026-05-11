#!/usr/bin/env bash
# PostToolUse hook for Write|Edit on *.rs files.
# Non-blocking: emits warnings to stderr if formatting/lint fails.
# Walks up from the edited file to find the owning Cargo.toml.

set -euo pipefail

input="$(cat)"
file_path="$(jq -r '.tool_input.file_path // empty' <<<"$input")"
[[ -z "$file_path" ]] && exit 0
[[ "$file_path" != *.rs ]] && exit 0
[[ ! -f "$file_path" ]] && exit 0

if command -v rustfmt >/dev/null 2>&1; then
  fmt_out=$(rustfmt --check --edition 2024 "$file_path" 2>&1 || true)
  if [[ -n "$fmt_out" ]]; then
    printf 'POST-LINT[rustfmt] %s:\n%s\n' "$file_path" "$fmt_out" >&2
  fi
fi

manifest=""
dir="$(dirname "$file_path")"
while [[ "$dir" != "/" && "$dir" != "." ]]; do
  if [[ -f "$dir/Cargo.toml" ]]; then
    manifest="$dir/Cargo.toml"
    break
  fi
  dir="$(dirname "$dir")"
done

if [[ -n "$manifest" ]] && command -v cargo >/dev/null 2>&1 && command -v timeout >/dev/null 2>&1; then
  base="$(basename "$file_path")"
  clippy_out=$(timeout 30 cargo clippy --manifest-path "$manifest" --quiet --no-deps -- -D warnings 2>&1 | rg -F "$base" || true)
  if [[ -n "$clippy_out" ]]; then
    printf 'POST-LINT[clippy] %s:\n%s\n' "$file_path" "$clippy_out" >&2
  fi
fi

exit 0
