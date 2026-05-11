//! UCEE Proxy entry point.
//!
//! Loads configuration, sets up structured-JSON observability, builds the
//! adapter registry, and serves the HTTP app via axum.
//!
//! Environment knobs (M2 placeholder; superseded by the YAML/env config
//! loader at M3):
//!
//! - `UCEE_BIND` — `host:port` to listen on (default `0.0.0.0:3000`).
//! - `UCEE_DOCLING_URL` — base URL of the docling upstream
//!   (default `http://localhost:8080`).
//! - `RUST_LOG` — tracing-subscriber env-filter (default `info`).

use std::env;

use anyhow::{Context, Result};
use ucee_adapter_docling::DoclingAdapter;
use ucee_config::Config;
use ucee_core::Registry;
use ucee_server::AppBuilder;

const DEFAULT_BIND: &str = "0.0.0.0:3000";
const DEFAULT_DOCLING_URL: &str = "http://localhost:8080";

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .json()
        .init();

    let _config = Config::load().context("load config")?;

    let docling_url =
        env::var("UCEE_DOCLING_URL").unwrap_or_else(|_| DEFAULT_DOCLING_URL.to_string());
    let docling = DoclingAdapter::new(&docling_url)
        .map_err(|e| anyhow::anyhow!("docling adapter init: {e}"))?;

    let mut registry = Registry::new();
    registry
        .register(docling)
        .map_err(|e| anyhow::anyhow!("register docling: {e}"))?;

    let bind = env::var("UCEE_BIND").unwrap_or_else(|_| DEFAULT_BIND.to_string());
    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .with_context(|| format!("bind {bind}"))?;
    let actual = listener.local_addr().context("local_addr")?;

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        bind = %actual,
        docling_url = %docling_url,
        "ucee serving Docling facade"
    );

    let app = AppBuilder::new(registry).build();
    axum::serve(listener, app).await.context("serve")?;
    Ok(())
}
