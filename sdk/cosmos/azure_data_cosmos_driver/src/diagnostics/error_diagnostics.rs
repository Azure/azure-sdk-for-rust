// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Attaches a [`DiagnosticsContext`] to an [`azure_core::Error`] so callers can
//! inspect rich per-operation diagnostics on failure.
//!
//! # Why
//!
//! The driver pipeline accumulates a [`DiagnosticsContext`] (per-attempt
//! events, region, transport shard, server-side duration, etc.) for every
//! operation. On the success path this context is exposed via
//! [`CosmosResponse::diagnostics`](crate::models::CosmosResponse::diagnostics).
//! On the error path the same context is built but, prior to this module,
//! was discarded — the only surviving signal was the error's `Display`
//! string and (for HTTP errors) the `RawResponse` bytes.
//!
//! This module makes the diagnostics retrievable on error by wrapping the
//! original error inside an `ErrorWithDiagnostics` and storing that as
//! the inner error of an `azure_core::Error::with_error(...)`. Callers
//! retrieve it with [`try_extract_diagnostics`].
//!
//! Observable behavior of the wrapped error is preserved:
//! - [`azure_core::Error::kind`] is unchanged (including
//!   `ErrorKind::HttpResponse { raw_response, .. }`).
//! - The top-level `Display` / `to_string()` is the original message.
//! - The original inner error (if any) is preserved as the source of
//!   `ErrorWithDiagnostics`. When the original had no inner, `source()`
//!   on the carrier returns `None` so error-chain walkers don't see a
//!   duplicated message.
//!
//! `ErrorWithDiagnostics` is an internal carrier type; the public SDK
//! surface (`azure_data_cosmos`) wraps it with a `CosmosError` newtype
//! that exposes only stable accessors and never leaks this type.

use std::sync::Arc;

use crate::diagnostics::DiagnosticsContext;

/// Inner-error wrapper that carries the operation's [`DiagnosticsContext`]
/// alongside the original error.
///
/// Stored as the inner error of an [`azure_core::Error`] (constructed via
/// [`azure_core::Error::with_error`]). Retrieve via [`try_extract_diagnostics`].
///
/// This type is `pub(crate)` so it cannot leak through the public Cosmos
/// SDK surface; callers interact with it only via
/// [`try_extract_diagnostics`] which returns the diagnostics context
/// directly.
#[derive(Debug)]
pub(crate) struct ErrorWithDiagnostics {
    /// The original error's inner, if it had one. `None` when the original
    /// error was a `Repr::Simple` / `Repr::SimpleMessage` variant — in
    /// which case the original message is preserved in `display` and
    /// `source()` returns `None` so that error-chain walkers don't
    /// observe a duplicated message.
    inner: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
    /// The original error's `Display` text. Always populated so the
    /// wrapper's own `Display` matches the original even when there was
    /// no inner.
    display: String,
    diagnostics: Arc<DiagnosticsContext>,
}

impl ErrorWithDiagnostics {
    /// Returns the diagnostics context this carrier holds (used by
    /// [`try_extract_diagnostics`] after a successful downcast).
    fn diagnostics(&self) -> &Arc<DiagnosticsContext> {
        &self.diagnostics
    }
}

impl std::fmt::Display for ErrorWithDiagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.display)
    }
}

impl std::error::Error for ErrorWithDiagnostics {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Only return the original inner when there was one. Don't
        // synthesize a source from the preserved display text — doing
        // so would make error-chain walkers see the original message
        // duplicated as a child source.
        self.inner.as_deref().map(|e| e as _)
    }
}

/// Wraps `err` so that `diagnostics` can be retrieved later via
/// [`try_extract_diagnostics`].
///
/// The returned `azure_core::Error` preserves the original `kind()`
/// (including any `ErrorKind::HttpResponse { raw_response, .. }`) and the
/// original top-level `Display` text. The wrapper becomes the new inner
/// error; the original inner (if any) is preserved as the wrapper's source.
/// When the original error had no inner, the wrapper's `source()` returns
/// `None` rather than synthesizing one from the preserved display text.
pub fn attach_diagnostics(
    err: azure_core::Error,
    diagnostics: Arc<DiagnosticsContext>,
) -> azure_core::Error {
    let kind = err.kind().clone();
    let display = err.to_string();
    let inner = err.into_inner().ok();
    let wrapper = ErrorWithDiagnostics {
        inner,
        display: display.clone(),
        diagnostics,
    };
    azure_core::Error::with_error(kind, wrapper, display)
}

/// Returns the [`DiagnosticsContext`] attached to `err` by
/// [`attach_diagnostics`], if any.
///
/// Walks the [`std::error::Error::source`] chain so that diagnostics remain
/// reachable after callers add additional context to the error (for example
/// via [`azure_core::Error::with_context`]). Returns `None` for errors that
/// did not flow through the driver pipeline, or that escaped before
/// diagnostics had been initialized.
pub fn try_extract_diagnostics(err: &azure_core::Error) -> Option<Arc<DiagnosticsContext>> {
    let mut current: Option<&(dyn std::error::Error + 'static)> = err.get_ref().map(|e| e as _);
    while let Some(node) = current {
        if let Some(wrapper) = node.downcast_ref::<ErrorWithDiagnostics>() {
            return Some(Arc::clone(wrapper.diagnostics()));
        }
        current = node.source();
    }
    None
}

/// Returns the `source` of `err` with all diagnostics carriers
/// (`ErrorWithDiagnostics`) skipped.
///
/// Use this when implementing `std::error::Error::source` for a
/// public Cosmos error wrapper: callers should not observe the
/// internal carrier in the source chain. Walks any directly-nested
/// `azure_core::Error → ErrorWithDiagnostics` pairs at the head of
/// the chain and returns the first source that is neither.
///
/// **Note**: only the head of the chain is sanitized. Carriers that
/// appear deeper in a chain (e.g. when `with_error` is layered on top
/// of an already-attached error multiple times) still surface when
/// callers walk `.source()` themselves. In normal SDK usage
/// `attach_diagnostics` is the outermost wrap on the operation
/// pipeline's escape sites, so this is sufficient.
pub fn source_skipping_carrier(
    err: &azure_core::Error,
) -> Option<&(dyn std::error::Error + 'static)> {
    use std::error::Error as _;
    let mut current: &(dyn std::error::Error + 'static) = err
        .get_ref()
        .map(|e| e as &(dyn std::error::Error + 'static))?;
    loop {
        if let Some(wrapper) = current.downcast_ref::<ErrorWithDiagnostics>() {
            // Skip the carrier and continue with whatever it wrapped.
            current = wrapper.source()?;
            continue;
        }
        if let Some(inner_err) = current.downcast_ref::<azure_core::Error>() {
            // The chain has nested an `azure_core::Error` — peek inside in
            // case the next layer is another carrier we should skip.
            let nested_inner = inner_err
                .get_ref()
                .map(|e| e as &(dyn std::error::Error + 'static));
            if let Some(next) = nested_inner {
                if next.is::<ErrorWithDiagnostics>() {
                    current = next;
                    continue;
                }
            }
        }
        return Some(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::DiagnosticsContext;
    use azure_core::error::ErrorKind;

    fn make_diagnostics_for_test() -> Arc<DiagnosticsContext> {
        let builder = crate::diagnostics::DiagnosticsContextBuilder::new(
            crate::models::ActivityId::from_string("test-error-diagnostics".to_owned()),
            Arc::new(crate::options::DiagnosticsOptions::default()),
        );
        Arc::new(builder.complete())
    }

    #[test]
    fn preserves_display_message() {
        let original =
            azure_core::Error::with_message(ErrorKind::Other, "original message".to_string());
        let display_before = original.to_string();
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);
        assert_eq!(wrapped.to_string(), display_before);
    }

    #[test]
    fn preserves_kind() {
        let original =
            azure_core::Error::with_message(ErrorKind::Credential, "credential bad".to_string());
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);
        assert!(matches!(wrapped.kind(), ErrorKind::Credential));
    }

    #[test]
    fn round_trip_extracts_diagnostics() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "x".to_string());
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, Arc::clone(&diagnostics));
        let extracted = try_extract_diagnostics(&wrapped).expect("diagnostics should round-trip");
        assert!(Arc::ptr_eq(&extracted, &diagnostics));
    }

    #[test]
    fn extracts_none_for_non_wrapped_error() {
        let plain = azure_core::Error::with_message(ErrorKind::Other, "no diag".to_string());
        assert!(try_extract_diagnostics(&plain).is_none());
    }

    #[test]
    fn extracts_through_context_chain() {
        // A caller layering additional context onto the error must not lose
        // the attached diagnostics — the extractor walks the source chain.
        let original = azure_core::Error::with_message(ErrorKind::Other, "x".to_string());
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, Arc::clone(&diagnostics));
        let with_context = wrapped.with_context("higher-level context");
        let extracted = try_extract_diagnostics(&with_context)
            .expect("diagnostics should survive context wrapping");
        assert!(Arc::ptr_eq(&extracted, &diagnostics));
    }

    #[test]
    fn preserves_http_response_kind_and_status() {
        use azure_core::http::{response::RawResponse, StatusCode};

        let raw_body = b"{\"code\":\"InternalServerError\"}".to_vec();
        let raw = RawResponse::from_bytes(
            StatusCode::InternalServerError,
            azure_core::http::headers::Headers::new(),
            azure_core::Bytes::from(raw_body.clone()),
        );
        let original = ErrorKind::HttpResponse {
            status: StatusCode::InternalServerError,
            error_code: Some("InternalServerError".to_string()),
            raw_response: Some(Box::new(raw)),
        }
        .into_error();

        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);

        assert_eq!(
            wrapped.http_status(),
            Some(StatusCode::InternalServerError),
            "http_status() must survive diagnostics wrapping",
        );
        match wrapped.kind() {
            ErrorKind::HttpResponse {
                status,
                error_code,
                raw_response,
            } => {
                assert_eq!(*status, StatusCode::InternalServerError);
                assert_eq!(error_code.as_deref(), Some("InternalServerError"));
                let raw = raw_response
                    .as_ref()
                    .expect("raw_response must be preserved");
                assert_eq!(raw.body().as_ref(), raw_body.as_slice());
            }
            other => panic!("expected HttpResponse kind, got {other:?}"),
        }
    }

    #[test]
    fn double_wrap_does_not_lose_diagnostics() {
        // If a caller (mistakenly) attaches diagnostics twice, the most-recent
        // attachment wins. We don't promise anything about the inner one but
        // we do guarantee try_extract_diagnostics returns *some* context.
        let original = azure_core::Error::with_message(ErrorKind::Other, "x".to_string());
        let diag1 = make_diagnostics_for_test();
        let diag2 = make_diagnostics_for_test();
        let wrapped_once = attach_diagnostics(original, diag1);
        let wrapped_twice = attach_diagnostics(wrapped_once, Arc::clone(&diag2));
        let extracted =
            try_extract_diagnostics(&wrapped_twice).expect("diagnostics should round-trip");
        assert!(Arc::ptr_eq(&extracted, &diag2));
    }

    #[test]
    fn source_chain_has_no_synthetic_node_when_original_had_no_inner() {
        // The original error has no inner (Simple variant). After
        // attach_diagnostics wraps it, the source chain must look like:
        //
        //   wrapped.source() -> Some(ErrorWithDiagnostics)
        //   ErrorWithDiagnostics.source() -> None
        //
        // i.e. the carrier is present (so try_extract_diagnostics can
        // find it via downcast) but it does NOT manufacture a child
        // source from the preserved display text.
        use std::error::Error as _;
        let original = azure_core::Error::with_message(ErrorKind::Other, "no inner".to_string());
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);

        let inner = wrapped
            .get_ref()
            .expect("wrapper must be present as inner of azure_core::Error");
        let carrier = inner
            .downcast_ref::<ErrorWithDiagnostics>()
            .expect("inner must be the diagnostics carrier");
        assert!(
            carrier.source().is_none(),
            "carrier must not synthesize a source when original had no inner",
        );
    }

    #[test]
    fn source_chain_preserves_original_inner() {
        // When the original error has a real inner (Custom variant via
        // with_error), the carrier's source() should be that original
        // inner so error-chain walkers continue to see it.
        use std::error::Error as _;
        let original = azure_core::Error::with_error(
            ErrorKind::Other,
            std::io::Error::other("io boom"),
            "outer message".to_string(),
        );
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);

        let inner = wrapped
            .get_ref()
            .expect("wrapper must be present as inner of azure_core::Error");
        let carrier = inner
            .downcast_ref::<ErrorWithDiagnostics>()
            .expect("inner must be the diagnostics carrier");
        let inner_source = carrier
            .source()
            .expect("carrier must expose original inner as source");
        // The original inner was an io::Error with text "io boom".
        assert!(
            inner_source.to_string().contains("io boom"),
            "expected to find original inner's display, got: {inner_source}",
        );
    }

    #[test]
    fn source_skipping_carrier_returns_none_when_original_had_no_inner() {
        // Sanity check the helper used by the public CosmosError wrapper:
        // when the original had no inner, source_skipping_carrier returns
        // None — so the public Error::source() chain is empty, matching
        // the original's behavior.
        let original = azure_core::Error::with_message(ErrorKind::Other, "no inner".to_string());
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);
        assert!(super::source_skipping_carrier(&wrapped).is_none());
    }

    #[test]
    fn source_skipping_carrier_returns_original_inner_when_present() {
        // When the original had a real inner, source_skipping_carrier
        // skips the carrier and returns the original inner, so callers
        // see the same chain they would have without diagnostics
        // attachment.
        let original = azure_core::Error::with_error(
            ErrorKind::Other,
            std::io::Error::other("io boom"),
            "outer message".to_string(),
        );
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);
        let src = super::source_skipping_carrier(&wrapped)
            .expect("must expose the original inner via the helper");
        assert!(src.to_string().contains("io boom"));
    }

    #[test]
    fn source_skipping_carrier_walks_through_layered_carriers() {
        // If the SDK ever attaches diagnostics on top of an already-attached
        // error (e.g. a retry path that re-wraps), the helper must skip
        // every carrier — not just the outermost one — so callers never
        // observe `ErrorWithDiagnostics` in their `Error::source()` walk.
        let original = azure_core::Error::with_error(
            ErrorKind::Other,
            std::io::Error::new(std::io::ErrorKind::Other, "io boom"),
            "outer message".to_string(),
        );
        let diag1 = make_diagnostics_for_test();
        let attached_once = attach_diagnostics(original, diag1);
        let diag2 = make_diagnostics_for_test();
        let attached_twice = attach_diagnostics(attached_once, diag2);

        let src = super::source_skipping_carrier(&attached_twice)
            .expect("nested carriers must still surface the original inner");
        assert!(
            !src.is::<ErrorWithDiagnostics>(),
            "helper must not return an ErrorWithDiagnostics, got: {src:?}",
        );
        assert!(
            src.to_string().contains("io boom"),
            "expected to walk past both carriers to original io error, got: {src}",
        );
    }
}
