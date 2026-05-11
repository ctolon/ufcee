//! Three-layer configuration loader for UCEE Proxy.
//!
//! Precedence (highest wins): built-in defaults < YAML file < environment
//! variables. Secrets reach the process via env vars only; YAML declares the
//! env var name via `api_key_env` (the actual value is resolved at load
//! time).
//!
//! Concrete schema and loader land at M0 follow-ups and M2 (HTTP server).

use serde::{Deserialize, Serialize};

/// Top-level UCEE configuration.
///
/// M0 placeholder. Concrete fields populated as crates come online:
/// engines map (M1), routes (M3), resilience (M7), observability (M6).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Default engine name (used when routing falls through all other rules).
    #[serde(default)]
    pub default_engine: Option<String>,
}

impl Config {
    /// Load configuration with the three-layer precedence.
    ///
    /// M0 placeholder. Concrete loader (defaults → YAML → env) lands when
    /// `figment`-based layering is wired up.
    pub fn load() -> Result<Self, ucee_core::Error> {
        Ok(Self::default())
    }
}
