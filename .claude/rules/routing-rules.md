---
paths:
  - "crates/ucee-router/**/*.rs"
---

# Routing engine rules

## Precedence

Engine selection precedence (first match wins):

1. Explicit header: `X-UCEE-Engine: <name>` — validated against engine registry; unknown name returns 409.
2. Config rule match: path glob in `Config::routes[]` matched against request path.
3. MIME magic detection: content sniffing of the request body or first uploaded file.
4. File extension: from the request path or `filename` parameter.
5. Default engine from config (`Config::default_engine`).

## MIME handling

- Use `mime::Mime` for parsing; never raw strings.
- Wildcard patterns in config: `image/*`, `application/vnd.openxmlformats-officedocument.*`.
- Magic-byte detection via the `file-format` or `infer` crate.
- When magic differs from `Content-Type` header, magic wins for routing decisions.

## Extension table

- Single source of truth: `EXT_TO_MIME` static slice in `ucee-router::ext`.
- Conflicts (extension maps to multiple engines) resolved by MIME magic; if magic is also ambiguous, return 415.

## Error responses

- 415 if no engine matches.
- 409 if header override names an unknown engine.
- 503 if matched engine's circuit breaker is open.
- 503 if matched engine's rate limit is exhausted (with `Retry-After` header).

## Implementation constraints

- No string concatenation for MIME comparisons; use `mime::Mime::eq` or destructure `(type_(), subtype())`.
- Routing decision is logged in the `request_completed` event under field `routing_path` (one of: `header`, `config`, `mime`, `ext`, `default`).
- All routing logic is pure; side effects (CB checks, rate limiter) happen after the decision in a separate stage.
