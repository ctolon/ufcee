//! SSRF validator for outbound HTTP to engine URLs.
//!
//! Implements CIDR allow / deny, scheme allowlist, DNS pinning, and
//! redirect re-validation. Closes the SSRF gap from the Go reference
//! implementation. See `.claude/rules/security-rules.md`.

use ucee_core::Error;

/// Validator that checks an outbound URL against the security policy.
///
/// M0 placeholder. Full validator lands at M8 per the roadmap.
#[derive(Debug, Default)]
pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    /// Validate an outbound URL.
    pub fn check(&self, _url: &str) -> Result<(), Error> {
        Ok(())
    }
}
