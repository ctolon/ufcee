//! Bounded streaming spool for inbound request bodies.
//!
//! Holds up to `body_buffer_max` bytes in memory; above the threshold falls
//! through to `O_TMPFILE` on Linux so the proxy does not buffer arbitrary
//! body sizes in RAM. See `docs/architecture/06-spool-backpressure.md`.

use bytes::Bytes;
use ucee_core::Error;

/// Bounded spool for an inbound body.
///
/// M0 placeholder. Concrete impl lands at M7 (resilience milestone).
#[derive(Debug)]
pub struct BoundedSpool {
    threshold: usize,
}

impl BoundedSpool {
    /// Default in-memory threshold: 8 MiB.
    pub const DEFAULT_THRESHOLD_BYTES: usize = 8 * 1024 * 1024;

    pub fn new(threshold: usize) -> Self {
        Self { threshold }
    }

    pub fn threshold(&self) -> usize {
        self.threshold
    }

    /// Push a chunk into the spool.
    pub fn push(&self, _chunk: Bytes) -> Result<(), Error> {
        Ok(())
    }
}

impl Default for BoundedSpool {
    fn default() -> Self {
        Self::new(Self::DEFAULT_THRESHOLD_BYTES)
    }
}
