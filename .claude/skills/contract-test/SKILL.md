---
name: contract-test
description: Run the enginetest contract test suite against one (or all) adapters and report conformance.
arguments: adapter-name [--all]
paths:
  - "crates/ucee-adapters/**"
context: fork
agent: L3-contract-test-enforcer
allowed-tools: Read, Bash(cargo test *)
---

# /contract-test — run contract suite

## Purpose

Execute `enginetest::run_contract_suite::<A>()` against a given adapter (or all
adapters with `--all`), reporting conformance failures with the violated
contract requirement.

## Steps

1. Locate the adapter crate.
2. Run `cargo test --package ucee-adapter-<name> --test contract`.
3. Parse failure output and map each failure to the contract requirement.
4. Report pass / fail per adapter with a list of failed checks.

## Outputs

- Pass / fail verdict per adapter.
- List of failing test names mapped to contract requirements.
