//! Property tests for the routing precedence chain.
//!
//! Exercises invariants over generated engine names and signal
//! combinations. The unit tests in `src/router.rs` cover specific worked
//! examples; these tests certify the algebra.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use proptest::prelude::*;
use ucee_router::{Router, RoutingError, RoutingPath, RoutingSignals};

fn engine_name_strategy() -> impl Strategy<Value = String> {
    "[a-z0-9][a-z0-9-]{0,31}"
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 64, ..ProptestConfig::default() })]

    /// Property: a header naming a registered engine always wins, no matter
    /// what other signals say.
    #[test]
    fn header_always_wins_for_registered_engine(name in engine_name_strategy()) {
        let router = Router::builder()
            .engine(&name, [mime::APPLICATION_PDF])
            .build()
            .unwrap();
        let signals = RoutingSignals {
            header_engine: Some(name.clone()),
            mime_sniffed: Some(mime::TEXT_HTML),
            extension: Some("html".to_string()),
        };
        let d = router.select(&signals).unwrap();
        prop_assert_eq!(d.engine_name, name);
        prop_assert_eq!(d.routing_path, RoutingPath::Header);
    }

    /// Property: a header naming an unregistered engine ALWAYS errors with
    /// `UnknownEngine` — never falls through to a different step.
    #[test]
    fn unknown_header_engine_never_falls_through(
        registered in engine_name_strategy(),
        requested in engine_name_strategy(),
    ) {
        prop_assume!(registered != requested);
        let router = Router::builder()
            .engine(&registered, [mime::APPLICATION_PDF])
            .default_engine(&registered)
            .build()
            .unwrap();
        let signals = RoutingSignals {
            header_engine: Some(requested.clone()),
            mime_sniffed: Some(mime::APPLICATION_PDF),
            extension: Some("pdf".to_string()),
        };
        let err = router.select(&signals).unwrap_err();
        prop_assert!(matches!(err, RoutingError::UnknownEngine(_)));
    }

    /// Property: MIME hit always beats extension when both would match
    /// different engines.
    #[test]
    fn mime_beats_ext(
        a in engine_name_strategy(),
        b in engine_name_strategy(),
    ) {
        prop_assume!(a != b);
        let router = Router::builder()
            .engine(&a, [mime::TEXT_HTML])
            .engine(&b, [mime::APPLICATION_PDF])
            .build()
            .unwrap();
        let signals = RoutingSignals {
            header_engine: None,
            mime_sniffed: Some(mime::TEXT_HTML), // would route to `a`
            extension: Some("pdf".to_string()),  // would route to `b`
        };
        let d = router.select(&signals).unwrap();
        prop_assert_eq!(d.engine_name, a);
        prop_assert_eq!(d.routing_path, RoutingPath::Mime);
    }

    /// Property: with no header / mime / ext match and a default set, the
    /// default always wins.
    #[test]
    fn default_fallback_always_wins_when_nothing_else_matches(
        name in engine_name_strategy(),
    ) {
        let router = Router::builder()
            .engine(&name, [mime::APPLICATION_PDF])
            .default_engine(&name)
            .build()
            .unwrap();
        let signals = RoutingSignals::default();
        let d = router.select(&signals).unwrap();
        prop_assert_eq!(d.engine_name, name);
        prop_assert_eq!(d.routing_path, RoutingPath::Default);
    }
}
