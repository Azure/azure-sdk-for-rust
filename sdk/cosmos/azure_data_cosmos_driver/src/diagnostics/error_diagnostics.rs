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
//! original error inside an `ErrorWithDiagnostics` carrier and storing
//! that as the inner error of an `azure_core::Error::with_error(...)`.
//! The wrapper crate (`azure_data_cosmos`) recovers it at the SDK
//! boundary via [`split_diagnostics_carrier`], which atomically:
//!
//! 1. extracts the diagnostics out of the carrier (and returns them
//!    separately), AND
//! 2. rebuilds the `azure_core::Error` with the carrier removed so the
//!    final `CosmosError`'s source chain contains only original errors.
//!
//! Observable behavior of the rebuilt error matches the pre-attachment
//! error exactly:
//! - [`azure_core::Error::kind`] is unchanged (including
//!   `ErrorKind::HttpResponse { raw_response, .. }`).
//! - The top-level `Display` / `to_string()` is the original message.
//! - The original inner error (if any) is preserved as the inner of the
//!   rebuilt error. When the original had no inner, the rebuilt error
//!   has no inner either — no synthetic source nodes.
//!
//! `ErrorWithDiagnostics` is `pub(crate)`; the carrier never leaves the
//! driver crate. The public SDK surface (`azure_data_cosmos::CosmosError`)
//! is a plain record with the diagnostics already extracted into a
//! field, so consumers never need to know the carrier exists.

use std::sync::Arc;

use crate::diagnostics::DiagnosticsContext;

/// Inner-error wrapper that carries the operation's [`DiagnosticsContext`]
/// alongside the original error.
///
/// Stored as the inner error of an [`azure_core::Error`] (constructed via
/// [`azure_core::Error::with_error`]) by [`attach_diagnostics`], and
/// later extracted by [`split_diagnostics_carrier`] at the wrapper-crate
/// boundary. The carrier is `pub(crate)` and never appears on the
/// public SDK surface.
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
/// [`split_diagnostics_carrier`].
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

/// Splits an `azure_core::Error` produced by the driver pipeline into:
/// 1. a "clean" `azure_core::Error` whose source chain no longer
///    contains the [`ErrorWithDiagnostics`] carrier (callers walking
///    `Error::source()` see only the original wrapped errors), and
/// 2. the outermost [`DiagnosticsContext`] that was attached (if any).
///
/// This is the only public way to recover diagnostics from a driver
/// error and is the single point used by the
/// [`azure_data_cosmos::CosmosError`](https://docs.rs/azure_data_cosmos)
/// wrapper's `From<azure_core::Error>` impl. Wrapper-crate plumbing —
/// `#[doc(hidden)]` for that reason.
///
/// Peels every directly-nested carrier from the head of the chain so
/// double-wrapping (e.g. a retry loop re-attaching after an inner
/// attach) does not leave a carrier visible. Returns the original
/// error unmodified plus `None` when no carrier is present.
#[doc(hidden)]
pub fn split_diagnostics_carrier(
    mut err: azure_core::Error,
) -> (azure_core::Error, Option<Arc<DiagnosticsContext>>) {
    let mut found: Option<Arc<DiagnosticsContext>> = None;
    loop {
        // Peek whether the immediate inner is the carrier; if not, we're done.
        let is_carrier = err
            .get_ref()
            .map(|inner| inner.is::<ErrorWithDiagnostics>())
            .unwrap_or(false);
        if !is_carrier {
            return (err, found);
        }

        // Take ownership of the carrier and rebuild err without it.
        let kind = err.kind().clone();
        let display = err.to_string();
        let boxed = match err.into_inner() {
            Ok(b) => b,
            // Defensive: the is_carrier check guarantees Some, but if a
            // future azure_core change broke that invariant we'd loop
            // forever. Bail cleanly.
            Err(original) => return (original, found),
        };
        let carrier: Box<ErrorWithDiagnostics> = match boxed.downcast::<ErrorWithDiagnostics>() {
            Ok(c) => c,
            Err(other) => {
                // Same defensive bail: rebuild err with the inner intact
                // so we don't lose information.
                return (azure_core::Error::with_error(kind, other, display), found);
            }
        };
        let ErrorWithDiagnostics {
            inner,
            display: _,
            diagnostics,
        } = *carrier;
        // Keep the OUTERMOST diagnostics (first seen = most recently attached).
        if found.is_none() {
            found = Some(diagnostics);
        }
        err = match inner {
            Some(orig) => azure_core::Error::with_error(kind, orig, display),
            None => azure_core::Error::with_message(kind, display),
        };
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
        let (clean, extracted) = split_diagnostics_carrier(wrapped);
        let extracted = extracted.expect("diagnostics should round-trip");
        assert!(Arc::ptr_eq(&extracted, &diagnostics));
        // The clean error must have no carrier on its source chain.
        let mut node: Option<&(dyn std::error::Error + 'static)> = clean.get_ref().map(|e| e as _);
        while let Some(n) = node {
            assert!(
                !n.is::<ErrorWithDiagnostics>(),
                "split_diagnostics_carrier must remove every carrier"
            );
            node = n.source();
        }
    }

    #[test]
    fn extracts_none_for_non_wrapped_error() {
        let plain = azure_core::Error::with_message(ErrorKind::Other, "no diag".to_string());
        let plain_display = plain.to_string();
        let (clean, extracted) = split_diagnostics_carrier(plain);
        assert!(extracted.is_none());
        // Passing through is lossless when no carrier is present.
        assert_eq!(clean.to_string(), plain_display);
    }

    #[test]
    fn preserves_http_response_kind_and_status_round_trip() {
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
        let (clean, extracted) = split_diagnostics_carrier(wrapped);

        assert!(extracted.is_some());
        assert_eq!(
            clean.http_status(),
            Some(StatusCode::InternalServerError),
            "http_status() must survive the carrier strip",
        );
        match clean.kind() {
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
    fn double_wrap_returns_outermost_diagnostics_and_strips_all_carriers() {
        // The SDK only attaches once per operation, but if a retry path
        // ever layers a second attach over a first one, the splitter
        // must peel both off and return the outermost diagnostics —
        // matching the convention "the most recently-attached context
        // is the most relevant".
        let original = azure_core::Error::with_error(
            ErrorKind::Other,
            std::io::Error::other("io boom"),
            "outer message".to_string(),
        );
        let diag1 = make_diagnostics_for_test();
        let diag2 = make_diagnostics_for_test();
        let wrapped_once = attach_diagnostics(original, diag1);
        let wrapped_twice = attach_diagnostics(wrapped_once, Arc::clone(&diag2));

        let (clean, extracted) = split_diagnostics_carrier(wrapped_twice);
        assert!(Arc::ptr_eq(
            &extracted.expect("must return diagnostics"),
            &diag2
        ));
        // Every carrier must be gone.
        let mut node: Option<&(dyn std::error::Error + 'static)> = clean.get_ref().map(|e| e as _);
        while let Some(n) = node {
            assert!(
                !n.is::<ErrorWithDiagnostics>(),
                "all carriers must be stripped, got: {n:?}",
            );
            node = n.source();
        }
        // Original inner must still be reachable.
        assert!(
            clean
                .get_ref()
                .map(|e| e.to_string().contains("io boom"))
                .unwrap_or(false),
            "original io::Error inner must survive the strip",
        );
    }

    #[test]
    fn carrier_present_in_chain_before_split() {
        // Sanity: the carrier IS present in the raw chain produced by
        // attach_diagnostics — splitting is what removes it. This guards
        // against accidentally making attach_diagnostics a no-op.
        let original = azure_core::Error::with_message(ErrorKind::Other, "x".to_string());
        let diagnostics = make_diagnostics_for_test();
        let wrapped = attach_diagnostics(original, diagnostics);
        let inner = wrapped.get_ref().expect("carrier present as inner");
        assert!(inner.is::<ErrorWithDiagnostics>());
    }
}
