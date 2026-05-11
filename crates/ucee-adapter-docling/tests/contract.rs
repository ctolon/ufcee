//! Contract test for `ucee-adapter-docling`.
//!
//! Sets up a `wiremock` server mimicking the docling REST contract and runs
//! the shared contract suite from `ucee-adapters-fixtures`. A pass here
//! certifies that this adapter conforms to the public `Adapter` trait.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use bytes::Bytes;
use ucee_adapter_docling::DoclingAdapter;
use ucee_adapters_fixtures::{run_contract_suite, sample_pdf_body};
use ucee_core::{Adapter, ConvertRequest};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn setup_mock() -> MockServer {
    let server = MockServer::start().await;

    // /healthz returns 200
    Mock::given(method("GET"))
        .and(path("/healthz"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    // /v1/convert/file returns a stub docling JSON
    Mock::given(method("POST"))
        .and(path("/v1/convert/file"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "ok",
            "result": "stub"
        })))
        .mount(&server)
        .await;

    server
}

#[tokio::test]
async fn docling_adapter_passes_contract_suite() {
    let server = setup_mock().await;
    let adapter = DoclingAdapter::new(server.uri()).expect("adapter must construct");
    run_contract_suite(&adapter).await;
}

#[tokio::test]
async fn convert_with_valid_body_returns_200() {
    let server = setup_mock().await;
    let adapter = DoclingAdapter::new(server.uri()).expect("adapter must construct");

    let req = ConvertRequest {
        mime: mime::APPLICATION_PDF,
        filename: Some("test.pdf".into()),
        body: sample_pdf_body(),
    };
    let resp = adapter.convert(req).await.expect("convert must succeed");
    assert_eq!(resp.status, 200);
    assert!(!resp.body.is_empty());
}

#[tokio::test]
async fn convert_with_empty_body_returns_adapter_error() {
    let server = setup_mock().await;
    let adapter = DoclingAdapter::new(server.uri()).expect("adapter must construct");

    let req = ConvertRequest {
        mime: mime::APPLICATION_PDF,
        filename: None,
        body: Bytes::new(),
    };
    let err = adapter
        .convert(req)
        .await
        .expect_err("empty body must fail");
    assert!(matches!(err, ucee_core::Error::Adapter(_)));
}

#[tokio::test]
async fn convert_propagates_upstream_5xx_as_adapter_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/convert/file"))
        .respond_with(ResponseTemplate::new(503))
        .mount(&server)
        .await;

    let adapter = DoclingAdapter::new(server.uri()).expect("adapter must construct");
    let req = ConvertRequest {
        mime: mime::APPLICATION_PDF,
        filename: Some("x.pdf".into()),
        body: sample_pdf_body(),
    };
    let err = adapter
        .convert(req)
        .await
        .expect_err("5xx must surface as error");
    assert!(matches!(err, ucee_core::Error::Adapter(_)));
}

#[tokio::test]
async fn health_returns_healthy_on_200() {
    let server = setup_mock().await;
    let adapter = DoclingAdapter::new(server.uri()).expect("adapter must construct");
    let h = adapter.health().await.expect("health must succeed");
    assert_eq!(h, ucee_core::HealthStatus::Healthy);
}

#[tokio::test]
async fn health_returns_unhealthy_on_404() {
    // No mocks configured; healthz will 404
    let server = MockServer::start().await;
    let adapter = DoclingAdapter::new(server.uri()).expect("adapter must construct");
    let h = adapter.health().await.expect("health must complete");
    assert_eq!(h, ucee_core::HealthStatus::Unhealthy);
}
