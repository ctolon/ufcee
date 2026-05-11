//! Convert endpoints (Docling facade).
//!
//! `/v1/convert/file` accepts a multipart upload, picks the engine via the
//! `X-UCEE-Engine` header (M2 routing simplification — M3 adds the full
//! precedence chain), and forwards via the registered adapter.
//!
//! `/v1/convert/source` is a 501 stub until M3 / M7 wire the source-fetch
//! path (needs SSRF + spool).

use std::sync::Arc;

use axum::{
    extract::{Multipart, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use ucee_core::ConvertRequest;

use crate::AppState;

const ENGINE_HEADER: &str = "x-ucee-engine";

/// `POST /v1/convert/file` — multipart upload routed to the named engine.
///
/// M2 simplification: routing is **header-only**. M3 adds the precedence
/// chain `header > config > MIME magic > extension > default`.
pub async fn convert_file(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    let engine_name = match headers.get(ENGINE_HEADER).and_then(|v| v.to_str().ok()) {
        Some(name) => name.to_string(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "missing X-UCEE-Engine header (M2 routing simplification; full precedence at M3)",
            )
                .into_response();
        }
    };

    let adapter = match state.registry.get(&engine_name) {
        Some(a) => a,
        None => {
            return (
                StatusCode::CONFLICT,
                format!("unknown engine: {engine_name}"),
            )
                .into_response();
        }
    };

    let mut body = Bytes::new();
    let mut filename: Option<String> = None;
    let mut content_type = mime::APPLICATION_OCTET_STREAM;

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("files") {
            filename = field.file_name().map(String::from);
            if let Some(ct) = field.content_type() {
                if let Ok(parsed) = ct.parse::<mime::Mime>() {
                    content_type = parsed;
                }
            }
            body = match field.bytes().await {
                Ok(b) => b,
                Err(e) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        format!("failed to read field bytes: {e}"),
                    )
                        .into_response();
                }
            };
            break;
        }
    }

    if body.is_empty() {
        return (StatusCode::BAD_REQUEST, "no body in 'files' field").into_response();
    }

    let req = ConvertRequest {
        mime: content_type,
        filename,
        body,
    };

    match adapter.convert(req).await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status).unwrap_or(StatusCode::OK);
            (status, resp.body).into_response()
        }
        Err(e) => (StatusCode::BAD_GATEWAY, format!("adapter error: {e}")).into_response(),
    }
}

/// `POST /v1/convert/source` — 501 until M3 / M7.
pub async fn convert_source() -> Response {
    (
        StatusCode::NOT_IMPLEMENTED,
        "convert/source lands at M3 (routing) / M7 (resilience)",
    )
        .into_response()
}
