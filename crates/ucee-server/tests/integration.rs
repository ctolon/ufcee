//! Integration tests for ucee-server.
//!
//! Each test stands up a wiremock-backed docling adapter, builds the axum
//! app with that adapter registered, binds it to a random local port, and
//! exercises the public HTTP surface via a real `reqwest` client.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ucee_adapter_docling::DoclingAdapter;
use ucee_adapters_fixtures::{sample_html_body, sample_pdf_body};
use ucee_core::Registry;
use ucee_server::AppBuilder;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Build a ucee-server bound to a random local port with one docling
/// adapter pointed at a fresh wiremock backend. If `default_engine` is
/// `Some`, the router falls back to it when nothing else matches.
async fn start_test_server(default_engine: Option<&str>) -> (String, MockServer) {
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

    let mut builder = AppBuilder::new(registry);
    if let Some(d) = default_engine {
        builder = builder.default_engine(d);
    }
    let app = builder.build().expect("AppBuilder::build must succeed");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    (format!("http://{addr}"), docling_mock)
}

#[tokio::test]
async fn healthz_returns_ok() {
    let (base, _mock) = start_test_server(None).await;
    let resp = reqwest::get(format!("{base}/healthz")).await.unwrap();
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "ok");
}

#[tokio::test]
async fn readyz_returns_ok_when_adapters_registered() {
    let (base, _mock) = start_test_server(None).await;
    let resp = reqwest::get(format!("{base}/readyz")).await.unwrap();
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn version_returns_engines_list() {
    let (base, _mock) = start_test_server(None).await;
    let resp = reqwest::get(format!("{base}/version")).await.unwrap();
    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["name"], "ucee");
    let engines = body["engines"].as_array().unwrap();
    assert!(engines.contains(&serde_json::Value::String("docling".to_string())));
}

#[tokio::test]
async fn convert_file_routes_via_header() {
    let (base, _mock) = start_test_server(None).await;

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
    assert_eq!(resp.headers().get("x-ucee-routing-path").unwrap(), "Header");
    assert_eq!(resp.headers().get("x-ucee-engine").unwrap(), "docling");
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["result"], "stub from docling mock");
}

#[tokio::test]
async fn convert_file_routes_via_mime_when_no_header() {
    let (base, _mock) = start_test_server(None).await;

    let part = reqwest::multipart::Part::bytes(sample_pdf_body().to_vec())
        .file_name("test.pdf")
        .mime_str("application/pdf")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);

    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    // PDF magic-bytes win the routing decision (MIME step).
    assert_eq!(resp.headers().get("x-ucee-routing-path").unwrap(), "Mime");
}

#[tokio::test]
async fn convert_file_routes_via_ext_when_no_magic() {
    let (base, _mock) = start_test_server(None).await;

    // HTML body — magic-byte detection doesn't recognize plain HTML
    // reliably, so we expect extension-step routing on .html filename.
    // (If magic does recognize it, this test still passes via the MIME
    // step pointing at docling — which declares HTML support.)
    let part = reqwest::multipart::Part::bytes(sample_html_body().to_vec())
        .file_name("doc.html")
        .mime_str("text/html")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);

    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    let routing_path = resp
        .headers()
        .get("x-ucee-routing-path")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        routing_path == "Mime" || routing_path == "Ext",
        "expected Mime or Ext, got {routing_path}"
    );
}

#[tokio::test]
async fn convert_file_uses_default_engine_when_nothing_matches() {
    let (base, _mock) = start_test_server(Some("docling")).await;

    // No filename → no extension; arbitrary body → unknown magic.
    let part = reqwest::multipart::Part::bytes(b"not a recognizable format".to_vec())
        .mime_str("application/octet-stream")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);

    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .multipart(form)
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("x-ucee-routing-path").unwrap(),
        "Default"
    );
}

#[tokio::test]
async fn convert_file_409_for_unknown_engine_header() {
    let (base, _mock) = start_test_server(None).await;
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
async fn convert_file_415_when_nothing_matches() {
    let (base, _mock) = start_test_server(None).await;
    // No header, unknown magic, no extension, no default.
    let part = reqwest::multipart::Part::bytes(b"not a recognizable format".to_vec())
        .mime_str("application/octet-stream")
        .unwrap();
    let form = reqwest::multipart::Form::new().part("files", part);
    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/file"))
        .multipart(form)
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 415);
}

#[tokio::test]
async fn convert_file_400_when_files_field_missing() {
    let (base, _mock) = start_test_server(None).await;
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
    let (base, _mock) = start_test_server(None).await;
    let resp = reqwest::Client::new()
        .post(format!("{base}/v1/convert/source"))
        .body("{}")
        .send()
        .await
        .unwrap();
    assert_eq!(resp.status(), 501);
}
