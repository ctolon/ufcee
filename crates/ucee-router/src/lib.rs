//! Engine selection for UCEE Proxy.
//!
//! Implements the routing precedence chain:
//!
//! 1. Explicit header (`X-UCEE-Engine`)
//! 2. Config rule (placeholder until M7 config loader; not yet exercised)
//! 3. MIME magic (content sniffing via `infer`)
//! 4. File extension (looked up in [`mime_from_extension`])
//! 5. Default engine
//!
//! First match wins. See `docs/architecture/05-routing-precedence.md` on the
//! `docs` branch and ADR-0005.

pub mod error;
pub mod ext;
pub mod mime_sniff;
pub mod router;

pub use error::RoutingError;
pub use ext::mime_from_extension;
pub use mime_sniff::sniff_mime;
pub use router::{Router, RouterBuilder, RoutingDecision, RoutingPath, RoutingSignals};
