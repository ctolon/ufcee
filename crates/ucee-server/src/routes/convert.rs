//! Convert endpoints (Docling facade).
//!
//! `/v1/convert/file` consumes the request body via axum's `Multipart`
//! extractor, builds [`ucee_router::RoutingSignals`] from header / magic-
//! bytes / filename, runs the full precedence chain
//! (header > config > MIME > ext > default), and dispatches via the
//! registry. `routing_path` is echoed back on the response as the
//! `X-UCEE-Routing-Path` header so callers can attribute decisions.
//!
//! `/v1/convert/source` is a 501 stub until M7 (needs SSRF + spool).

use std::sync::Arc;

use axum::{
    extract::{Multipart, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use bytes::Bytes;
use ucee_core::ConvertRequest;
use ucee_router::{RoutingError, RoutingSignals, sniff_mime};

use crate::AppState;

const ENGINE_HEADER: &str = "x-ucee-engine";

/// `POST /v1/convert/file` — multipart upload routed via the full
/// precedence chain.
pub async fn convert_file(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    mut multipart: Multipart,
) -> Response {
    // 1. Read the multipart `files` field.
    let mut body = Bytes::new();
    let mut filename: Option<String> = None;
    let mut declared_mime = mime::APPLICATION_OCTET_STREAM;

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("files") {
            filename = field.file_name().map(String::from);
            if let Some(ct) = field.content_type() {
                if let Ok(parsed) = ct.parse::<mime::Mime>() {
                    declared_mime = parsed;
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

    // 2. Build routing signals.
    let header_engine = headers
        .get(ENGINE_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(String::from);
    let extension = filename
        .as_deref()
        .and_then(|f| f.rsplit_once('.').map(|(_, ext)| ext.to_string()));
    let mime_sniffed = sniff_mime(&body);

    let signals = RoutingSignals {
        header_engine,
        mime_sniffed: mime_sniffed.clone(),
        extension,
    };

    // 3. Route.
    let decision = match state.router.select(&signals) {
        Ok(d) => d,
        Err(RoutingError::UnknownEngine(name)) => {
            return (StatusCode::CONFLICT, format!("unknown engine: {name}")).into_response();
        }
        Err(RoutingError::NoEngineMatches) => {
            return (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "no engine matches the request",
            )
                .into_response();
        }
    };

    // 4. Dispatch via the registry.
    let Some(adapter) = state.registry.get(&decision.engine_name) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "router selected an engine that is not in the registry",
        )
            .into_response();
    };

    let effective_mime = mime_sniffed.unwrap_or(declared_mime);
    let req = ConvertRequest {
        mime: effective_mime,
        filename,
        body,
    };

    let routing_path_label = format!("{:?}", decision.routing_path);

    match adapter.convert(req).await {
        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status).unwrap_or(StatusCode::OK);
            (
                status,
                [
                    ("x-ucee-engine", decision.engine_name.as_str()),
                    ("x-ucee-routing-path", routing_path_label.as_str()),
                ],
                resp.body,
            )
                .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            [
                ("x-ucee-engine", decision.engine_name.as_str()),
                ("x-ucee-routing-path", routing_path_label.as_str()),
            ],
            format!("adapter error: {e}"),
        )
            .into_response(),
    }
}

/// `POST /v1/convert/source` — 501 until M7 (needs SSRF + spool).
pub async fn convert_source() -> Response {
    (
        StatusCode::NOT_IMPLEMENTED,
        "convert/source lands at M7 (needs SSRF validator + streaming spool)",
    )
        .into_response()
}
