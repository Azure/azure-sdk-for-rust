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
//! original error inside an [`ErrorWithDiagnostics`] and storing that as
//! the inner error of an `azure_core::Error::with_error(...)`. Callers
//! retrieve it with [`try_extract_diagnostics`].
//!
//! Observable behavior of the wrapped error is preserved:
//! - [`azure_core::Error::kind`] is unchanged (including
//!   `ErrorKind::HttpResponse { raw_response, .. }`).
//! - The top-level `Display` / `to_string()` is the original message.
//! - The original inner error (if any) is preserved as the source of
//!   [`ErrorWithDiagnostics`], so error-chain walkers continue to work.
//!
//! # Example
//!
//! ```ignore
//! match driver.execute_operation(op, opts).await {
//!     Ok(response) => { /* response.diagnostics() */ }
//!     Err(err) => {
//!         if let Some(ctx) = try_extract_diagnostics(&err) {
//!             let json = ctx.to_json_string(None);
//!             // ship `json` to ADX, log it, etc.
//!         }
//!     }
//! }
//! ```

use std::sync::Arc;

use crate::diagnostics::DiagnosticsContext;

/// Inner-error wrapper that carries the operation's [`DiagnosticsContext`]
/// alongside the original error.
///
/// Stored as the inner error of an [`azure_core::Error`] (constructed via
/// [`azure_core::Error::with_error`]). Retrieve via [`try_extract_diagnostics`].
#[derive(Debug)]
pub struct ErrorWithDiagnostics {
    inner: Box<dyn std::error::Error + Send + Sync + 'static>,
    diagnostics: Arc<DiagnosticsContext>,
}

impl ErrorWithDiagnostics {
    /// The diagnostics context captured for the failed operation.
    pub fn diagnostics(&self) -> &Arc<DiagnosticsContext> {
        &self.diagnostics
    }

    /// The original inner error that was wrapped.
    pub fn inner(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        &*self.inner
    }
}

impl std::fmt::Display for ErrorWithDiagnostics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the inner so user-visible chain text is unchanged.
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl std::error::Error for ErrorWithDiagnostics {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.inner)
    }
}

/// Tiny `std::error::Error` that holds a captured `Display` string. Used as
/// the wrapper's inner when the original error has no inner of its own
/// (i.e. `Repr::Simple` / `Repr::SimpleMessage`), so we can still preserve
/// the original message verbatim.
#[derive(Debug)]
struct PreservedDisplay(String);

impl std::fmt::Display for PreservedDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for PreservedDisplay {}

/// Wraps `err` so that `diagnostics` can be retrieved later via
/// [`try_extract_diagnostics`].
///
/// The returned `azure_core::Error` preserves the original `kind()`
/// (including any `ErrorKind::HttpResponse { raw_response, .. }`) and the
/// original top-level `Display` text. The wrapper becomes the new inner
/// error; the original inner (if any) is preserved as the wrapper's source.
pub fn attach_diagnostics(
    err: azure_core::Error,
    diagnostics: Arc<DiagnosticsContext>,
) -> azure_core::Error {
    let kind = err.kind().clone();
    let display = err.to_string();
    let inner: Box<dyn std::error::Error + Send + Sync + 'static> = match err.into_inner() {
        Ok(boxed) => boxed,
        Err(_) => Box::new(PreservedDisplay(display.clone())),
    };
    let wrapper = ErrorWithDiagnostics { inner, diagnostics };
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
            return Some(Arc::clone(&wrapper.diagnostics));
        }
        current = node.source();
    }
    None
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
        let original = azure_core::Error::with_message(
            ErrorKind::Credential,
            "credential bad".to_string(),
        );
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
}
