//! Integration tests for ucee-server.
//!
//! Each test stands up a wiremock-backed docling adapter, builds the axum
//! app with that adapter registered, binds it to a random local port, and
//! exercises the public HTTP surface via a real `reqwest` client.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ucee_adapter_docling::DoclingAdapter;
use ucee_adapters_fixtures::sample_pdf_body;
use ucee_core::Registry;
use ucee_server::AppBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Stand up:
/// 1. A wiremock server playing the role of the docling upstream.
/// 2. A `ucee-server` bound to a random `127.0.0.1` port with a single
///    `docling` adapter pointed at the wiremock.
///
/// Returns the base URL of the ucee-server plus the wiremock handle (held
/// to keep it alive for the duration of the test).
async fn start_test_server() -> (String, MockServer) {
    let docling_mock = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/convert/file"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "result": "stub from docling mock"
        })))
        .mount(&docling_mock)
        .await;
    Mock::given(method("GET"))
        .and(path("/healthz"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&docling_mock)
        .await;

    let adapter = DoclingAdapter::new(docling_mock.uri()).unwrap();
    let mut registry = Registry::new();
    registry.register(adapter).unwrap();
    let app = AppBuilder::new(registry).build();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    (format!("http://{addr}"), docling_mock)
}

#[tokio::test]
async fn healthz_returns_ok() {
    let (base, _mock) = start_test_server().await;
    let resp = reqwest::get(format!("{base}/healthz")).await.unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "ok");
}

#[tokio::test]
async fn readyz_returns_ok_when_adapters_registered() {
    let (base, _mock) = start_test_server().await;
    let resp = reqwest::get(format!("{base}/readyz")).await.unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn version_returns_engines_list() {
    let (base, _mock) = start_test_server().await;
    let resp = reqwest::get(format!("{base}/version")).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "ucee");
    let engines = body["engines"].as_array().unwrap();
    assert!(engines.contains(&serde_json::Value::String("docling".to_string())));
}

#[tokio::test]
async fn convert_file_routes_to_docling_mock() {
    let (base, _mock) = start_test_server().await;

    let part = reqwest::multipart::Part::bytes(sample_pdf_body().to_vec())
        .file_name("test.pdf")
        .mime_str("application/pdf")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);

    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .header("X-UCEE-Engine", "docling")
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["result"], "stub from docling mock");
}

#[tokio::test]
async fn convert_file_400_without_engine_header() {
    let (base, _mock) = start_test_server().await;
    let part = reqwest::multipart::Part::bytes(sample_pdf_body().to_vec())
        .file_name("t.pdf")
        .mime_str("application/pdf")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);
    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn convert_file_409_for_unknown_engine() {
    let (base, _mock) = start_test_server().await;
    let part = reqwest::multipart::Part::bytes(sample_pdf_body().to_vec())
        .file_name("t.pdf")
        .mime_str("application/pdf")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);
    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .header("X-UCEE-Engine", "bogus")
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 409);
}

#[tokio::test]
async fn convert_file_400_when_files_field_missing() {
    let (base, _mock) = start_test_server().await;
    let form = reqwest::multipart::Form::new().text("other", "value");
    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .header("X-UCEE-Engine", "docling")
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 400);
}

#[tokio::test]
async fn convert_source_returns_501() {
    let (base, _mock) = start_test_server().await;
    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/source"))
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 501);
}
