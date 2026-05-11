---
paths:
  - "**/*.rs"
  - "**/*.py"
---

# Security rules

## Secrets

- No secret values in code, fixtures, tests, examples, or YAML config.
- Secrets reach the process via environment variables only; YAML names the env var (e.g., `api_key_env: UCEE_DOCLING_API_KEY`).
- Logs redact secret-shaped values (anything matching token-like regex) — verified by a fixture test in `ucee-observability`.
- `Debug` and `Display` impls on secret-bearing types redact (use `Secret<T>` newtype or `secrecy` crate).

## SSRF defense

Every outbound HTTP to a configured engine URL passes through `ucee_ssrf::Validator`:

- URL scheme is `http` or `https` only (no `file://`, `gopher://`, etc.).
- Host CIDR is checked against an allow/deny list. Defaults deny:
  - `127.0.0.0/8`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `169.254.0.0/16`
  - `::1/128`, `fc00::/7`, `fe80::/10`
- DNS-pin: resolve the hostname once, pin the resulting IP for the request; no follow-up resolution mid-redirect.
- Max redirects: 3, each target re-validated against the same rules.

## Injection

- No `eval`, `exec`, `pickle.loads`, or `subprocess.run(shell=True)` on user-influenced input.
- All shell-out (rare) uses explicit argv arrays, never shell-interpolated strings.
- All SQL (if any) via parameterized queries via `sqlx` or equivalent.

## Unsafe Rust

- `#[deny(unsafe_code)]` is required at the top of every library crate.
- Removing the deny attribute requires an accepted ADR.
- Each `unsafe` block has a `// SAFETY:` comment naming the exact invariants that hold.

## Dependencies

- `cargo audit` and `cargo deny check` pass on every PR (CI-enforced).
- `pip-audit` on Python deps.
- New dependencies require an ADR entry; the entry names the dependency, version, license, and the reason existing deps don't suffice.

## Cryptography

- Never roll your own crypto.
- Approved primitives: TLS via `rustls` (no `native-tls`), hashing via `blake3` or `sha2`, MAC via `hmac`, KDF via `argon2`.
- No use of MD5 or SHA-1 except for non-security checksums (and even then, label clearly).
