//! Circuit breaker and rate limiter for per-engine + per-route gating.
//!
//! See `docs/architecture/04-circuit-breaker.md` for the state machine.

use std::time::Duration;

use ucee_core::Error;

/// Circuit-breaker states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreakerState {
    Closed,
    Open,
    HalfOpen,
}

/// Per-engine + per-route circuit breaker.
///
/// M0 placeholder. Concrete impl lands at M7.
#[derive(Debug)]
pub struct Breaker {
    state: BreakerState,
    failure_threshold: u32,
    recovery_timeout: Duration,
}

impl Breaker {
    pub fn new(failure_threshold: u32, recovery_timeout: Duration) -> Self {
        Self {
            state: BreakerState::Closed,
            failure_threshold,
            recovery_timeout,
        }
    }

    pub fn state(&self) -> BreakerState {
        self.state
    }

    pub fn failure_threshold(&self) -> u32 {
        self.failure_threshold
    }

    pub fn recovery_timeout(&self) -> Duration {
        self.recovery_timeout
    }

    /// Check whether a request may proceed.
    pub fn probe(&self) -> Result<(), Error> {
        match self.state {
            BreakerState::Open => Err(Error::Routing("circuit breaker open".into())),
            BreakerState::Closed | BreakerState::HalfOpen => Ok(()),
        }
    }
}

/// Token-bucket rate limiter.
///
/// M0 placeholder. Concrete impl lands at M7.
#[derive(Debug)]
pub struct TokenBucket {
    capacity: u32,
    rps: u32,
}

impl TokenBucket {
    pub fn new(capacity: u32, rps: u32) -> Self {
        Self { capacity, rps }
    }

    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    pub fn rps(&self) -> u32 {
        self.rps
    }

    /// Try to acquire a token.
    pub fn acquire(&self) -> Result<(), Error> {
        Ok(())
    }
}
