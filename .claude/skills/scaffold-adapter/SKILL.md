---
name: scaffold-adapter
description: Generate a new engine adapter satisfying the Adapter contract trait + contract test stubs.
arguments: engine-name
paths:
  - "crates/ucee-adapters/**"
context: fork
agent: L2-adapter-custom-contract
allowed-tools: Read, Write, Edit, Bash(cargo new *), Bash(cargo check *), Bash(cargo test *)
---

# /scaffold-adapter — generate a new adapter

## Purpose

Create a new adapter crate `crates/ucee-adapter-<engine>/` that implements the
`Adapter` trait from `ucee-core::adapter` and ships a contract test invoking
`enginetest::run_contract_suite::<Self>()`.

## Steps

1. Validate the engine name matches `^[a-z0-9][a-z0-9-]{0,31}$`.
2. Run `/scaffold-crate ucee-adapter-<engine>` first to get the workspace member.
3. Apply adapter templates from `assets/` (lib.rs with trait impl skeleton,
   tests/contract.rs invoking the contract suite, config schema fragment).
4. Register the adapter in `ucee-server::registry`.
5. Ask `L2-adapter-custom-contract` to review trait conformance.

## Outputs

- New adapter crate with all five trait methods (skeleton).
- Contract test wired in.
- Registry registration in `ucee-server`.
