---
name: scaffold-pyo3-binding
description: Generate a PyO3 binding skeleton with `#[pymodule]` body, error translation, and `.pyi` stub generator integration.
arguments: module-name
paths:
  - "crates/ucee-py/**"
  - "python/ucee/**"
context: fork
agent: L2-pyo3-bridge
allowed-tools: Read, Write, Edit, Bash(cargo *), Bash(maturin *)
---

# /scaffold-pyo3-binding — generate a PyO3 binding

## Purpose

Bootstrap a new PyO3-exposed module under `crates/ucee-py/src/<module>.rs` and
its Python-side counterpart in `python/ucee/<module>.py`, with type stubs and
error translation pre-wired.

## Steps

1. Validate the module name (lower_snake_case).
2. Apply the PyO3 template from `assets/` (Bound idioms, error mapping, `#[pymodule]` registration).
3. Add the module to `crates/ucee-py/src/lib.rs`.
4. Generate or update `.pyi` stubs.
5. Ask `L2-python-sdk-designer` to author the Python-side ergonomic wrapper.

## Outputs

- New PyO3 module file.
- Updated `lib.rs` registration.
- Generated `.pyi` stub.
- Python-side wrapper stub.
