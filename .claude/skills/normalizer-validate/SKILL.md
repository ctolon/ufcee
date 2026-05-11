---
name: normalizer-validate
description: Run the normalizer fixture suite (golden files) covering /v1/convert/file vs /v1/convert/source vs External /process.
paths:
  - "crates/ucee-normalizer/**"
context: fork
agent: L2-normalizer-designer
allowed-tools: Read, Bash(cargo test *)
---

# /normalizer-validate — fixture suite run

## Purpose

Execute the normalizer's golden-file fixture suite and report any mismatch
between expected and actual normalization output.

## Steps

1. Run `cargo test --package ucee-normalizer --test fixtures`.
2. On failure, dump the diff between expected and actual JSON.
3. Group failures by request-shape variant.

## Outputs

- Per-fixture pass / fail.
- Diff dump for each failure.
