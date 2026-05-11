---
name: release
description: Orchestrate a semver release — bump versions, regenerate CHANGELOG, tag, bump helm chart, generate SBOM. High-blast-radius; user-triggered only.
arguments: <semver>
context: inherit
agent: L4-release-engineer
disable-model-invocation: true
allowed-tools: Read, Edit, Bash(git *), Bash(cargo *), Bash(helm *)
---

# /release — orchestrate a release

## Purpose

End-to-end release workflow: semver version bump across workspace crates and the
Python package, `CHANGELOG.md` regeneration via `L4-changelog-curator`, helm
chart version bump on the `helm` branch via `L4-helm-author`, SBOM generation
via `L4-sbom-author`, and an annotated git tag prepared (user pushes manually).

This skill is `disable-model-invocation: true` because releases are
high-blast-radius — the user must explicitly type `/release <semver>`.

## Steps

1. Validate `<semver>` is a valid semver string.
2. Confirm the working tree is clean.
3. Bump `Cargo.toml` versions (workspace + members where appropriate).
4. Bump `python/ucee/_version.py`.
5. Run `L4-changelog-curator` to update CHANGELOG.
6. Switch to the helm worktree; run `L4-helm-author` to bump chart version.
7. Run `L4-sbom-author` to produce `sbom-<ver>.cdx.json`.
8. Create annotated git tag (user reviews and pushes).

## Outputs

- Updated version files across crates + Python.
- Updated `CHANGELOG.md`.
- Bumped helm chart on `helm` branch.
- SBOM artifact.
- Annotated git tag (unpushed).
