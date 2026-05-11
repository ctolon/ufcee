---
paths:
  - "python/**/*.py"
  - "bindings/python/**/*.py"
---

# Python style rules

## Formatting and linting (ruff)

- Target version: `py312`.
- Line length: 100.
- Selected rule sets: `E,F,W,I,N,UP,B,A,C4,PT,SIM,RUF`.
- `ruff format` is the formatter (PEP 8 compatible).

## Typing

- All public functions and methods have full type annotations.
- No untyped `Any` in public surfaces; use `object`, `TypeVar`, or `Protocol`.
- Prefer `collections.abc` over `typing.List` / `typing.Dict` (PEP 585).
- Use `from __future__ import annotations` at file top.
- Public Protocol or ABC interfaces are explicit; no duck typing in public APIs.

## Layout

- Tests in `tests/unit/` and `tests/integration/`.
- No top-level global state; everything lives in functions, classes, or `if __name__ == "__main__":` guards.
- One public class or function per public module unless they form a tight cohort.

## Async

- Async-only public client; sync wrappers may exist as thin shims that call `asyncio.run` only at the outermost layer.
- No `asyncio.run()` inside library code (callers manage the loop).
- Long-running coroutines accept a `cancel_event: asyncio.Event` parameter for cooperative cancellation.

## Errors

- Define `UceeError(Exception)` as the public base.
- Subclass per concern: `UceeNetworkError`, `UceeConfigError`, `UceeAdapterError`.
- No bare `except:`; always specify exception types.
- Never swallow exceptions silently; either re-raise or log with structured context.
