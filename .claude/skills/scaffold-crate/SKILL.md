---
name: scaffold-crate
description: Generate a new Rust workspace crate skeleton with module / test layout, Cargo.toml, and `#[deny(unsafe_code)]`.
arguments: crate-name [--lib|--bin]
context: fork
agent: L2-system-architect
allowed-tools: Read, Write, Edit, Bash(cargo new *), Bash(cargo metadata *), Bash(cargo check *)
---

# /scaffold-crate — generate a new workspace crate

## Purpose

Create a new Rust crate under `crates/<crate-name>` with the project's standard
layout: `Cargo.toml` with `#[deny(unsafe_code)]`, `src/lib.rs` (or `src/main.rs`
for `--bin`), `src/error.rs` skeleton, `tests/` directory, and a registered
workspace member entry.

## Steps

1. Validate the crate name matches the project convention.
2. Run `cargo new --lib crates/<name>` (or `--bin`).
3. Apply the standard templates from `assets/` (Cargo.toml, lib.rs, error.rs).
4. Update the workspace `Cargo.toml` to include the new member.
5. Ask `L1-dependency-tracker` to refresh `DEPS.md`.

## Outputs

- New crate under `crates/<crate-name>/`.
- Updated workspace `Cargo.toml`.
- Refreshed `DEPS.md`.
