//! Router type and supporting structures.

use std::collections::{HashMap, HashSet};

use mime::Mime;

use crate::error::RoutingError;
use crate::ext::mime_from_extension;

/// Signals available to [`Router::select`].
///
/// Constructed by the request handler from headers, magic-byte sniffing,
/// and filename inspection.
#[derive(Debug, Default, Clone)]
pub struct RoutingSignals {
    /// Engine name from the `X-UCEE-Engine` request header.
    pub header_engine: Option<String>,
    /// MIME type detected by magic-byte sniffing of the request body.
    pub mime_sniffed: Option<Mime>,
    /// Filename extension (without the leading dot), case-insensitive.
    pub extension: Option<String>,
}

/// Outcome of a successful routing decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingDecision {
    /// Chosen engine name.
    pub engine_name: String,
    /// Which precedence step produced this decision.
    pub routing_path: RoutingPath,
}

/// Which precedence step produced the routing decision.
///
/// Logged as `routing_path` on the `request_completed` event so
/// operators can attribute traffic by decision provenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoutingPath {
    Header,
    Config,
    Mime,
    Ext,
    Default,
}

/// Routes inbound requests to engine adapters.
///
/// Build via [`Router::builder`]. The router is read-only once built; the
/// HTTP server holds it inside `Arc<AppState>`.
#[derive(Debug, Default)]
pub struct Router {
    engines: HashSet<String>,
    mime_index: HashMap<String, String>,
    default_engine: Option<String>,
}

impl Router {
    /// Start a fluent builder.
    pub fn builder() -> RouterBuilder {
        RouterBuilder::default()
    }

    /// Select an engine using the full precedence chain.
    ///
    /// # Errors
    ///
    /// - [`RoutingError::UnknownEngine`] if a header names an engine that
    ///   was not registered.
    /// - [`RoutingError::NoEngineMatches`] if every step fell through and
    ///   no default engine is set.
    pub fn select(&self, signals: &RoutingSignals) -> Result<RoutingDecision, RoutingError> {
        // 1. Header override.
        if let Some(name) = &signals.header_engine {
            if !self.engines.contains(name) {
                return Err(RoutingError::UnknownEngine(name.clone()));
            }
            return Ok(RoutingDecision {
                engine_name: name.clone(),
                routing_path: RoutingPath::Header,
            });
        }

        // 2. Config rule — placeholder until M7 config loader.

        // 3. MIME magic.
        if let Some(m) = &signals.mime_sniffed {
            if let Some(name) = self.mime_index.get(m.essence_str()) {
                return Ok(RoutingDecision {
                    engine_name: name.clone(),
                    routing_path: RoutingPath::Mime,
                });
            }
        }

        // 4. Extension.
        if let Some(ext) = &signals.extension {
            if let Some(m) = mime_from_extension(ext) {
                if let Some(name) = self.mime_index.get(m.essence_str()) {
                    return Ok(RoutingDecision {
                        engine_name: name.clone(),
                        routing_path: RoutingPath::Ext,
                    });
                }
            }
        }

        // 5. Default.
        if let Some(name) = &self.default_engine {
            return Ok(RoutingDecision {
                engine_name: name.clone(),
                routing_path: RoutingPath::Default,
            });
        }

        Err(RoutingError::NoEngineMatches)
    }

    /// Iterate registered engine names.
    pub fn engines(&self) -> impl Iterator<Item = &str> {
        self.engines.iter().map(String::as_str)
    }

    /// Default engine name, if configured.
    pub fn default_engine(&self) -> Option<&str> {
        self.default_engine.as_deref()
    }

    /// Look up which engine handles the given MIME type (exact match).
    pub fn engine_for_mime(&self, m: &Mime) -> Option<&str> {
        self.mime_index.get(m.essence_str()).map(String::as_str)
    }
}

/// Fluent builder for [`Router`].
#[derive(Debug, Default)]
pub struct RouterBuilder {
    engines: HashSet<String>,
    mime_index: HashMap<String, String>,
    default_engine: Option<String>,
}

impl RouterBuilder {
    /// Register an engine with the MIME types it accepts.
    ///
    /// Later calls with the same MIME type silently overwrite the earlier
    /// mapping (last-wins).
    pub fn engine(
        mut self,
        name: impl Into<String>,
        mime_types: impl IntoIterator<Item = Mime>,
    ) -> Self {
        let name = name.into();
        for m in mime_types {
            self.mime_index
                .insert(m.essence_str().to_string(), name.clone());
        }
        self.engines.insert(name);
        self
    }

    /// Set the default engine.
    pub fn default_engine(mut self, name: impl Into<String>) -> Self {
        self.default_engine = Some(name.into());
        self
    }

    /// Build the router.
    ///
    /// # Errors
    ///
    /// Returns [`RoutingError::UnknownEngine`] if a default engine was set
    /// but was never registered via [`engine`](Self::engine).
    pub fn build(self) -> Result<Router, RoutingError> {
        if let Some(d) = &self.default_engine {
            if !self.engines.contains(d) {
                return Err(RoutingError::UnknownEngine(d.clone()));
            }
        }
        Ok(Router {
            engines: self.engines,
            mime_index: self.mime_index,
            default_engine: self.default_engine,
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    fn router() -> Router {
        Router::builder()
            .engine("pdf-engine", [mime::APPLICATION_PDF])
            .engine("html-engine", [mime::TEXT_HTML])
            .default_engine("pdf-engine")
            .build()
            .unwrap()
    }

    #[test]
    fn header_step_wins() {
        let r = router();
        let s = RoutingSignals {
            header_engine: Some("html-engine".to_string()),
            mime_sniffed: Some(mime::APPLICATION_PDF),
            extension: Some("pdf".to_string()),
        };
        let d = r.select(&s).unwrap();
        assert_eq!(d.engine_name, "html-engine");
        assert_eq!(d.routing_path, RoutingPath::Header);
    }

    #[test]
    fn unknown_header_engine_errors() {
        let r = router();
        let s = RoutingSignals {
            header_engine: Some("bogus".to_string()),
            ..Default::default()
        };
        let err = r.select(&s).unwrap_err();
        assert!(matches!(err, RoutingError::UnknownEngine(_)));
    }

    #[test]
    fn mime_step_wins_over_ext() {
        let r = router();
        let s = RoutingSignals {
            header_engine: None,
            mime_sniffed: Some(mime::TEXT_HTML),
            extension: Some("pdf".to_string()),
        };
        let d = r.select(&s).unwrap();
        assert_eq!(d.engine_name, "html-engine");
        assert_eq!(d.routing_path, RoutingPath::Mime);
    }

    #[test]
    fn ext_step_when_no_mime() {
        let r = router();
        let s = RoutingSignals {
            header_engine: None,
            mime_sniffed: None,
            extension: Some("html".to_string()),
        };
        let d = r.select(&s).unwrap();
        assert_eq!(d.engine_name, "html-engine");
        assert_eq!(d.routing_path, RoutingPath::Ext);
    }

    #[test]
    fn default_step_when_nothing_matches() {
        let r = router();
        let s = RoutingSignals::default();
        let d = r.select(&s).unwrap();
        assert_eq!(d.engine_name, "pdf-engine");
        assert_eq!(d.routing_path, RoutingPath::Default);
    }

    #[test]
    fn no_default_no_match_errors() {
        let r = Router::builder()
            .engine("pdf-engine", [mime::APPLICATION_PDF])
            .build()
            .unwrap();
        let s = RoutingSignals::default();
        assert!(matches!(
            r.select(&s).unwrap_err(),
            RoutingError::NoEngineMatches
        ));
    }

    #[test]
    fn unknown_default_engine_errors_at_build() {
        let err = Router::builder()
            .engine("a", [mime::APPLICATION_PDF])
            .default_engine("b")
            .build()
            .unwrap_err();
        assert!(matches!(err, RoutingError::UnknownEngine(_)));
    }

    #[test]
    fn mime_unmatched_falls_through_to_ext() {
        let r = router();
        let s = RoutingSignals {
            header_engine: None,
            mime_sniffed: Some(mime::IMAGE_PNG), // not registered
            extension: Some("html".to_string()),
        };
        let d = r.select(&s).unwrap();
        assert_eq!(d.engine_name, "html-engine");
        assert_eq!(d.routing_path, RoutingPath::Ext);
    }
}
