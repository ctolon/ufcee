---
name: sync-helm
description: Verify helm values.yaml is in sync with the Config schema. Flags drift in either direction.
paths:
  - "helm/**"
  - "crates/ucee-config/**"
context: fork
agent: L4-helm-author
disable-model-invocation: true
allowed-tools: Read, Bash(git worktree *), Bash(helm lint *), Bash(helm template *)
---

# /sync-helm — values↔config sync check

## Purpose

Cross-branch check that the helm chart's `values.yaml` schema matches the Rust
`Config` schema 1:1. Any key in one without the other is flagged.

This skill is `disable-model-invocation: true` because helm changes affect
deployment — the user must explicitly type the slash command.

## Steps

1. Switch to the helm worktree.
2. Parse `helm/ufcee/values.yaml` into a key set.
3. Parse the Rust `Config` derive into a key set (via `cargo expand` or annotated reflection).
4. Diff the two sets.
5. Report extras / missing on each side.
6. Run `helm lint` + `helm template | kubeval` if available.

## Outputs

- Symmetric difference of key sets.
- helm lint + kubeval pass / fail.
- Suggested fix list.
