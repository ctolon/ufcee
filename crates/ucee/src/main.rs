//! UCEE Proxy entry point.
//!
//! Loads configuration, sets up observability, and starts the HTTP server.
//! M0 placeholder: initializes tracing, loads the (currently empty) config,
//! prints version, and exits. The HTTP server is wired in at M2.

use anyhow::Result;
use ucee_config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .json()
        .init();

    let _config = Config::load()?;

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "ucee starting (M0 placeholder; HTTP server lands at M2)"
    );

    Ok(())
}
