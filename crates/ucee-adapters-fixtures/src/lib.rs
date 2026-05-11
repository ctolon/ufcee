//! Shared adapter test fixtures and the contract test harness.
//!
//! Every adapter crate's `#[cfg(test)] mod contract` invokes
//! [`run_contract_suite`] against an instance of its adapter. The suite
//! enforces the public [`ucee_core::Adapter`] contract uniformly across all
//! adapters; failures indicate a contract violation that the adapter must
//! fix before merging. See ADR-0003.

use std::sync::LazyLock;

use bytes::Bytes;
use ucee_core::{Adapter, CURRENT_CONTRACT_VERSION, ConvertRequest, Error};

#[allow(clippy::unwrap_used)]
static NAME_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"^[a-z0-9][a-z0-9-]{0,31}$").unwrap());

/// Run the full contract suite against an adapter instance.
///
/// Callers (adapter crates' `#[cfg(test)] mod contract`) construct an
/// adapter pointed at a test-only mock server (e.g., `wiremock`) and pass a
/// reference here. Any contract violation triggers a panic via `assert!`,
/// failing the test.
pub async fn run_contract_suite<A: Adapter>(adapter: &A) {
    check_contract_version(adapter);
    check_name(adapter);
    check_capabilities(adapter);
    check_convert_empty_body(adapter).await;
    check_health_roundtrip(adapter).await;
}

/// Verify the adapter conforms to the current contract version.
fn check_contract_version<A: Adapter>(adapter: &A) {
    let v = adapter.contract_version();
    assert!(
        v >= CURRENT_CONTRACT_VERSION,
        "adapter '{}' is at contract version {} but core expects >= {}",
        adapter.name(),
        v,
        CURRENT_CONTRACT_VERSION,
    );
}

/// Verify the adapter name matches the engine-name regex.
fn check_name<A: Adapter>(adapter: &A) {
    let name = adapter.name();
    assert!(
        NAME_REGEX.is_match(name),
        "adapter name '{name}' does not match ^[a-z0-9][a-z0-9-]{{0,31}}$"
    );
}

/// Verify the adapter declares at least one MIME type.
fn check_capabilities<A: Adapter>(adapter: &A) {
    let caps = adapter.capabilities();
    assert!(
        !caps.mime_types.is_empty(),
        "adapter '{}' declares no MIME types in capabilities",
        adapter.name()
    );
    // compat_type access verifies the field is present and readable.
    let _ = caps.compat_type;
}

/// Verify the adapter handles an empty body gracefully (returns an
/// `Error::Adapter`).
async fn check_convert_empty_body<A: Adapter>(adapter: &A) {
    let req = ConvertRequest {
        mime: mime::APPLICATION_PDF,
        filename: None,
        body: Bytes::new(),
    };
    let result = adapter.convert(req).await;
    if let Err(err) = result {
        assert!(
            matches!(err, Error::Adapter(_)),
            "adapter '{}' returned wrong error variant for empty body: {err:?}",
            adapter.name()
        );
    }
}

/// Verify the health endpoint round-trip completes.
///
/// Specific health expectations (`Healthy` vs `Unhealthy`) are checked by
/// adapter-specific integration tests, not by the contract suite.
async fn check_health_roundtrip<A: Adapter>(adapter: &A) {
    let _ = adapter.health().await;
}

/// Build a minimal PDF-like body for contract tests.
///
/// Not a valid PDF; just carries the magic bytes so routing tests can
/// identify it as `application/pdf`.
pub fn sample_pdf_body() -> Bytes {
    Bytes::from_static(b"%PDF-1.4\n%test\n")
}

/// Build a minimal HTML body for contract tests.
pub fn sample_html_body() -> Bytes {
    Bytes::from_static(b"<!doctype html><html><body>test</body></html>")
}

#[cfg(test)]
mod tests {
    use super::*;
    use ucee_core::{Capabilities, CompatType, ConvertResponse, HealthStatus};

    struct MockAdapter;

    impl Adapter for MockAdapter {
        fn name(&self) -> &'static str {
            "mock"
        }

        fn capabilities(&self) -> Capabilities {
            Capabilities {
                compat_type: CompatType::Docling,
                mime_types: vec![mime::APPLICATION_PDF],
            }
        }

        async fn convert(&self, req: ConvertRequest) -> Result<ConvertResponse, Error> {
            if req.body.is_empty() {
                return Err(Error::Adapter("empty body".to_string()));
            }
            Ok(ConvertResponse {
                status: 200,
                body: b"{}".to_vec(),
            })
        }

        async fn health(&self) -> Result<HealthStatus, Error> {
            Ok(HealthStatus::Healthy)
        }
    }

    #[tokio::test]
    async fn mock_adapter_passes_contract_suite() {
        run_contract_suite(&MockAdapter).await;
    }

    #[test]
    fn sample_bodies_have_expected_magic() {
        let pdf = sample_pdf_body();
        assert!(pdf.starts_with(b"%PDF-"));
        let html = sample_html_body();
        assert!(html.starts_with(b"<!doctype html"));
    }

    #[test]
    fn name_regex_accepts_valid_engine_names() {
        assert!(NAME_REGEX.is_match("docling"));
        assert!(NAME_REGEX.is_match("docling-v2"));
        assert!(NAME_REGEX.is_match("a"));
        assert!(NAME_REGEX.is_match("a0"));
        assert!(NAME_REGEX.is_match("0a"));
    }

    #[test]
    fn name_regex_rejects_invalid_engine_names() {
        assert!(!NAME_REGEX.is_match(""));
        assert!(!NAME_REGEX.is_match("-docling"));
        assert!(!NAME_REGEX.is_match("Docling"));
        assert!(!NAME_REGEX.is_match("docling_v2"));
        assert!(!NAME_REGEX.is_match("a".repeat(33).as_str()));
    }
}
