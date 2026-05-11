---
paths:
  - "crates/**/*.rs"
---

# Tokio async rules

## Forbidden

- `tokio::task::block_on` inside an `async fn`.
- `std::sync::Mutex` or `std::sync::RwLock` held across an `.await`.
- `std::thread::sleep` in async code (use `tokio::time::sleep`).
- Blocking IO (`std::fs`, `std::net`) directly inside async fns; use `tokio::fs`, `tokio::net`, or wrap with `tokio::task::spawn_blocking`.
- `tokio::runtime::Runtime::new()` outside `main`. Library crates never create their own runtime.

## Required

- `tokio::sync::*` (`Mutex`, `RwLock`, `mpsc`, `oneshot`, `Notify`) for cross-task synchronization.
- Every `tokio::spawn(...)` result is either awaited / joined OR explicitly detached with a one-line comment justifying the fire-and-forget.
- Structured concurrency via `tokio::task::JoinSet`, `tokio::try_join!`, or `tokio::select!` with named branches.
- Cooperative cancellation: long-running loops periodically check a `tokio_util::sync::CancellationToken`.
- All `select!` arms either complete or are biased with explicit cancellation semantics.

## Testing

- `#[tokio::test]` for async tests.
- Time-dependent tests use `tokio::time::pause` + `tokio::time::advance`; no real `sleep`.
- Integration tests use `#[tokio::test(flavor = "multi_thread", worker_threads = N)]` when threading semantics matter.
- Loom (`#[cfg(loom)]`) for concurrency correctness proofs on lock-free data structures.

## Backpressure

- `mpsc::channel` with bounded capacity for all producer/consumer paths.
- `mpsc::unbounded_channel` requires an ADR justifying the unboundedness.
- HTTP request body reading uses `axum::extract::DefaultBodyLimit` or per-route limit; no unbounded reads.
