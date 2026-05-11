//! Docling HTTP client + [`ucee_core::Adapter`] implementation.

use std::time::Duration;

use reqwest::multipart::{Form, Part};
use ucee_core::{
    Adapter, Capabilities, CompatType, ConvertRequest, ConvertResponse, Error, HealthStatus,
};
use url::Url;

/// Adapter that speaks the docling REST contract.
#[derive(Debug, Clone)]
pub struct DoclingAdapter {
    base_url: Url,
    http: reqwest::Client,
}

impl DoclingAdapter {
    /// Engine name — matches the engine-name regex `^[a-z0-9][a-z0-9-]{0,31}$`.
    pub const NAME: &'static str = "docling";

    /// Default request timeout.
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

    /// Build a new adapter pointing at the docling REST base URL.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Adapter`] if `base_url` is not a valid URL or if the
    /// underlying HTTP client cannot be constructed.
    pub fn new(base_url: impl AsRef<str>) -> Result<Self, Error> {
        let base_url = Url::parse(base_url.as_ref())
            .map_err(|e| Error::Adapter(format!("invalid base url: {e}")))?;
        let http = reqwest::Client::builder()
            .timeout(Self::DEFAULT_TIMEOUT)
            .build()
            .map_err(|e| Error::Adapter(format!("http client init: {e}")))?;
        Ok(Self { base_url, http })
    }

    fn endpoint(&self, path: &str) -> Result<Url, Error> {
        self.base_url
            .join(path)
            .map_err(|e| Error::Adapter(format!("url join {path}: {e}")))
    }
}

impl Adapter for DoclingAdapter {
    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            compat_type: CompatType::Docling,
            mime_types: vec![mime::APPLICATION_PDF, mime::TEXT_HTML],
        }
    }

    async fn convert(&self, req: ConvertRequest) -> Result<ConvertResponse, Error> {
        if req.body.is_empty() {
            return Err(Error::Adapter("empty body".to_string()));
        }

        let endpoint = self.endpoint("/v1/convert/file")?;
        let filename = req.filename.unwrap_or_else(|| "upload".to_string());
        let part = Part::bytes(req.body.to_vec())
            .file_name(filename)
            .mime_str(req.mime.as_ref())
            .map_err(|e| Error::Adapter(format!("multipart part: {e}")))?;
        let form = Form::new().part("files", part);

        let resp = self
            .http
            .post(endpoint)
            .multipart(form)
            .send()
            .await
            .map_err(|e| Error::Adapter(format!("send: {e}")))?;

        let status = resp.status();
        let body = resp
            .bytes()
            .await
            .map_err(|e| Error::Adapter(format!("read body: {e}")))?
            .to_vec();

        if !status.is_success() {
            return Err(Error::Adapter(format!(
                "docling upstream returned {status}"
            )));
        }

        Ok(ConvertResponse {
            status: status.as_u16(),
            body,
        })
    }

    async fn health(&self) -> Result<HealthStatus, Error> {
        let endpoint = self.endpoint("/healthz")?;
        match self.http.get(endpoint).send().await {
            Ok(r) if r.status().is_success() => Ok(HealthStatus::Healthy),
            Ok(_) | Err(_) => Ok(HealthStatus::Unhealthy),
        }
    }
}
