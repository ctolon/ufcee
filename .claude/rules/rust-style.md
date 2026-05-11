---
paths:
  - "crates/**/*.rs"
---

# Rust style rules

## Formatting

- rustfmt config: `max_width = 100`, `edition = "2024"`, `imports_granularity = "Crate"`, `group_imports = "StdExternalCrate"`.
- File top-level order: `//!` crate-level doc, `#![...]` crate attributes, `use` clauses, items.

## Naming

- Modules and files: `snake_case`.
- Types (struct, enum, trait, type alias): `UpperCamelCase`.
- Constants and statics: `SCREAMING_SNAKE_CASE`.
- Functions and methods: `snake_case`.
- Lifetimes: short single-letter where unambiguous (`'a`); descriptive where multiple lifetimes interact (`'engine`, `'req`).
- Generics: `T`, `U`, `E` (Error) by convention; descriptive names allowed for clarity (`Engine`, `Adapter`).

## Errors

- Each library crate defines its own `Error` enum via `thiserror`.
- No `anyhow` in library crates. Binary crates may use `anyhow`.
- No `unwrap()` or `expect()` outside `#[cfg(test)]` modules.
- No `panic!()` outside `fn main()`. `unreachable!()` is allowed only with a `// SAFETY:`-style comment naming the invariant that makes the branch dead.
- Prefer `?` over explicit `match Err(...)`.
- Convert across crate boundaries via `From` impls or `.map_err(Error::from_<context>)`.

## Control flow

- Prefer `if let Some(x) = ... else` over `match` for two-arm Option/Result.
- Use `let-else` for happy-path extraction.
- Avoid deeply nested closures; extract named helpers.

## Documentation

- Every `pub` item has a `///` doc comment.
- Doc comments lead with a one-sentence summary.
- `# Errors` section documents what error variants are returned.
- `# Panics` section documents any panic paths (should be empty per the rule above).
- `# Examples` section for non-trivial APIs.

## Unsafe

- `#[deny(unsafe_code)]` is set on every library crate.
- Removing `deny(unsafe_code)` requires an accepted ADR.
- Every `unsafe` block has a `// SAFETY:` comment naming the exact invariants that hold.
