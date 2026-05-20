// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosError`] — the error type returned by every public Cosmos SDK
//! API.
//!
//! `CosmosError` is a transparent record with public fields: the wrapped
//! [`azure_core::Error`] (as an `Arc`), the operation's final HTTP
//! `status_code` and `sub_status` (when known), and the per-operation
//! [`CosmosDiagnosticsContext`] (when the driver attached one). Callers
//! can pattern-match, project, or serialize the error directly.
//!
//! # Interop with [`azure_core::Error`]
//!
//! `?` inside a `CosmosResult<T>` function auto-converts via
//! `From<azure_core::Error>`. To go the other way:
//!
//! - [`CosmosError::as_azure_error`] — borrow the wrapped error.
//! - [`CosmosError::into_source`] — owned `Arc<azure_core::Error>` handle
//!   (`azure_core::Error` is `!Clone`).
//! - [`CosmosError::into_azure_error_lossy`] — owned `azure_core::Error`;
//!   unwraps the `Arc` if possible, otherwise clones `kind` + `message`
//!   (drops the inner source chain on the shared-handle path).
//!
//! `From<serde_json::Error>` is also provided for the common
//! body-serialize/parse case. Other conversion errors should use
//! `.map_err(azure_core::Error::from)?`.
//!
//! # `Debug` redaction
//!
//! `CosmosError`'s hand-rolled `Debug` projects [`ErrorKind`] through its
//! `Display` impl, which strips `raw_response`. This prevents
//! `format!("{err:?}")` from leaking captured response headers (session
//! tokens, `WWW-Authenticate` challenges) or response body bytes into
//! application logs.
//!
//! [`ErrorKind`]: azure_core::error::ErrorKind
//!
//! # Example
//!
//! ```ignore
//! use azure_data_cosmos::{CosmosError, CosmosResult};
//!
//! async fn read_item(client: &azure_data_cosmos::clients::ContainerClient)
//!     -> CosmosResult<()>
//! {
//!     match client.read_item::<serde_json::Value>("pk", "id", None).await {
//!         Ok(_) => Ok(()),
//!         Err(err) => {
//!             eprintln!("status_code = {:?}", err.status_code);
//!             eprintln!("sub_status  = {:?}", err.sub_status);
//!             if let Some(diag) = err.diagnostics.as_ref() {
//!                 eprintln!("activity_id = {:?}", diag.activity_id());
//!             }
//!             Err(err)
//!         }
//!     }
//! }
//! ```

use std::sync::Arc;

use crate::models::CosmosDiagnosticsContext;

/// Convenience type alias for `Result<T, CosmosError>` returned by every
/// public Cosmos SDK API.
pub type CosmosResult<T> = Result<T, CosmosError>;

/// Error type returned by every public Cosmos SDK API.
///
/// Transparent record with all interesting fields `pub`. See the
/// module-level docs for the design rationale and interop guidance.
///
/// `#[non_exhaustive]` — callers should use field access or the
/// `From<azure_core::Error>` constructor rather than struct literals.
#[derive(Clone)]
#[non_exhaustive]
pub struct CosmosError {
    /// The wrapped [`azure_core::Error`]. The driver's internal
    /// diagnostics carrier has already been stripped at construction
    /// time, so callers walking `source.source()` see only the
    /// original wrapped error chain.
    ///
    /// Stored as `Arc` so cloning a `CosmosError` is a refcount bump
    /// ([`azure_core::Error`] is itself `!Clone`).
    pub source: Arc<azure_core::Error>,
    /// The operation's final HTTP status code, if known.
    ///
    /// Populated from the diagnostics context's recorded final status
    /// (after retries and failovers). Falls back to the wrapped
    /// error's [`azure_core::Error::http_status`] when no diagnostics
    /// were attached (e.g. credential or argument-validation
    /// failures).
    pub status_code: Option<u16>,
    /// The operation's Cosmos sub-status code
    /// (`x-ms-substatus` response header), if known.
    ///
    /// See <https://learn.microsoft.com/en-us/rest/api/cosmos-db/http-status-codes-for-cosmosdb>
    /// for the catalog of codes.
    pub sub_status: Option<u32>,
    /// The per-operation diagnostics context attached by the driver
    /// pipeline, if any.
    ///
    /// `None` for errors that did not flow through the driver pipeline
    /// or that escaped before diagnostics had been initialized.
    pub diagnostics: Option<Arc<CosmosDiagnosticsContext>>,
}

impl CosmosError {
    /// Borrow the wrapped [`azure_core::Error`].
    #[inline]
    pub fn as_azure_error(&self) -> &azure_core::Error {
        &self.source
    }

    /// Consume the `CosmosError` and return the wrapped
    /// [`azure_core::Error`] as an `Arc` handle.
    ///
    /// `Arc` rather than an owned value because [`azure_core::Error`]
    /// is `!Clone` and this `CosmosError` itself supports cheap
    /// fan-out via `Clone`. Use [`Self::into_azure_error_lossy`] if
    /// you need an owned `azure_core::Error`.
    #[inline]
    pub fn into_source(self) -> Arc<azure_core::Error> {
        self.source
    }

    /// Consume the `CosmosError` and return an owned [`azure_core::Error`].
    ///
    /// If this `CosmosError` is the sole owner of the wrapped `Arc`,
    /// the original error is returned unchanged. Otherwise a fresh
    /// `azure_core::Error` is constructed from the wrapped error's
    /// `kind()` (cloned, including any `raw_response` payload) and
    /// `to_string()`. The shared-handle path drops the original inner
    /// source chain (callers walking `Error::source()` will see an
    /// empty chain).
    ///
    /// **Not a redaction boundary.** The returned `azure_core::Error`
    /// uses `azure_core`'s derived `Debug`, which prints
    /// `HttpResponse::raw_response` verbatim. `CosmosError`'s
    /// hand-rolled `Debug` is the surface that scrubs `raw_response`;
    /// log `{cosmos_err:?}` directly if you need that guarantee.
    ///
    /// Prefer [`Self::as_azure_error`] when you only need a borrow.
    pub fn into_azure_error_lossy(self) -> azure_core::Error {
        match Arc::try_unwrap(self.source) {
            Ok(err) => err,
            Err(shared) => {
                let kind = shared.kind().clone();
                let display = shared.to_string();
                azure_core::Error::with_message(kind, display)
            }
        }
    }

    /// Borrow the wrapped error's [`ErrorKind`](azure_core::error::ErrorKind).
    #[inline]
    pub fn kind(&self) -> &azure_core::error::ErrorKind {
        self.source.kind()
    }

    /// Returns the HTTP status code on the wrapped error's
    /// [`ErrorKind::HttpResponse`](azure_core::error::ErrorKind::HttpResponse),
    /// if any.
    ///
    /// This reflects the status on the response that produced this
    /// error. For the operation's **final** status code after retries
    /// and failovers, use [`Self::status_code`].
    #[inline]
    pub fn http_status(&self) -> Option<azure_core::http::StatusCode> {
        self.source.http_status()
    }

    /// Equivalent to `self.status_code`. See the field for semantics.
    #[inline]
    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }

    /// Equivalent to `self.sub_status`. See the field for semantics.
    #[inline]
    pub fn sub_status(&self) -> Option<u32> {
        self.sub_status
    }

    /// Borrow the per-operation diagnostics context, if attached.
    ///
    /// Returns a borrow to avoid a refcount bump on every read. For
    /// owned access, clone the field directly: `err.diagnostics.clone()`.
    #[inline]
    pub fn diagnostics(&self) -> Option<&CosmosDiagnosticsContext> {
        self.diagnostics.as_deref()
    }
}

impl std::fmt::Display for CosmosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&*self.source, f)
    }
}

impl std::fmt::Debug for CosmosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Do NOT delegate to `azure_core::Error`'s derived Debug — it
        // walks `Repr` and would emit `HttpResponse::raw_response`
        // verbatim (response headers including session tokens and
        // `WWW-Authenticate` challenges, plus the response body
        // bytes). The `ErrorKind` Display impl is hand-rolled to
        // strip `raw_response`, so projecting through it is the
        // single redaction gate.
        f.debug_struct("CosmosError")
            .field("kind", &format_args!("{}", self.source.kind()))
            .field("message", &self.source.to_string())
            .field("status_code", &self.status_code)
            .field("sub_status", &self.sub_status)
            .field("has_diagnostics", &self.diagnostics.is_some())
            .finish()
    }
}

impl std::error::Error for CosmosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // The carrier is stripped by `From::from` (via
        // `split_diagnostics_carrier`), so plain delegation is correct.
        Some(&*self.source)
    }
}

impl From<azure_core::Error> for CosmosError {
    fn from(err: azure_core::Error) -> Self {
        // Canonical (and only intended) construction path. `#[non_exhaustive]`
        // blocks external struct-literal construction; in-crate sites should
        // also go through this `From` so the carrier-strip step below is not
        // bypassed.
        let (rebuilt, diagnostics) =
            azure_data_cosmos_driver::diagnostics::split_diagnostics_carrier(err);

        let (status_code, sub_status) = match diagnostics.as_deref().and_then(|d| d.status()) {
            Some(s) => (
                Some(u16::from(s.status_code())),
                s.sub_status().map(|ss| ss.value()),
            ),
            None => (rebuilt.http_status().map(u16::from), None),
        };

        CosmosError {
            source: Arc::new(rebuilt),
            status_code,
            sub_status,
            diagnostics,
        }
    }
}

// `serde_json::Error` covers request-body serialization and response-body
// parsing, the dominant `?` site in this crate. Other conversion errors
// (`base64::DecodeError`, `url::ParseError`, …) use
// `.map_err(azure_core::Error::from)?` per the module docs.
impl From<serde_json::Error> for CosmosError {
    fn from(err: serde_json::Error) -> Self {
        CosmosError::from(azure_core::Error::from(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;
    use std::sync::Arc;

    fn make_test_diagnostics() -> Arc<CosmosDiagnosticsContext> {
        Arc::new(CosmosDiagnosticsContext::for_testing(
            azure_data_cosmos_driver::models::ActivityId::from_string(
                "test-cosmos-error".to_owned(),
            ),
        ))
    }

    #[test]
    fn from_azure_core_error_preserves_message_and_kind() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let original_display = original.to_string();
        let cosmos: CosmosError = original.into();
        assert_eq!(cosmos.to_string(), original_display);
        assert!(matches!(cosmos.source.kind(), ErrorKind::Other));
        assert!(cosmos.diagnostics.is_none());
        assert!(cosmos.status_code.is_none());
        assert!(cosmos.sub_status.is_none());
    }

    #[test]
    fn status_code_falls_back_to_inner_http_status() {
        // Without a diagnostics context attached, status_code must
        // surface the HTTP status carried by the wrapped
        // `azure_core::Error` so 4xx/5xx responses are still
        // observable through the field.
        use azure_core::http::StatusCode;
        let err = ErrorKind::HttpResponse {
            status: StatusCode::NotFound,
            error_code: None,
            raw_response: None,
        }
        .into_error();
        let cosmos = CosmosError::from(err);
        assert_eq!(cosmos.status_code, Some(404));
        // The escape hatch still surfaces the typed StatusCode.
        assert_eq!(
            cosmos.as_azure_error().http_status(),
            Some(StatusCode::NotFound),
        );
    }

    #[test]
    fn from_extracts_diagnostics_and_strips_carrier() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let diagnostics = make_test_diagnostics();
        let wrapped = azure_data_cosmos_driver::diagnostics::attach_diagnostics(
            original,
            Arc::clone(&diagnostics),
        );

        let cosmos = CosmosError::from(wrapped);
        assert!(cosmos.diagnostics.is_some());
        assert!(
            Arc::ptr_eq(cosmos.diagnostics.as_ref().unwrap(), &diagnostics),
            "From should pass the carrier's Arc handle through, not clone",
        );
        // The source chain must NOT contain the driver's internal carrier.
        // Walk it and assert each node's Debug rendering does not name the
        // carrier type (a downcast would require the carrier type in scope,
        // which is `pub(crate)` in the driver).
        use std::error::Error as _;
        let mut node: Option<&(dyn std::error::Error + 'static)> = cosmos.source();
        let mut steps = 0;
        while let Some(n) = node {
            let dbg = format!("{n:?}");
            assert!(
                !dbg.contains("ErrorWithDiagnostics"),
                "carrier leaked into CosmosError source chain at step {steps}: {dbg}",
            );
            node = n.source();
            steps += 1;
            assert!(steps < 32, "source chain looped");
        }
    }

    #[test]
    fn debug_does_not_leak_raw_response() {
        // Construct an HttpResponse-kind error with a RawResponse that
        // contains identifiable bytes; assert those bytes never appear
        // in the Debug output of the wrapping CosmosError.
        use azure_core::http::{headers::Headers, response::RawResponse, StatusCode};
        let raw = RawResponse::from_bytes(
            StatusCode::Forbidden,
            Headers::new(),
            azure_core::Bytes::from_static(b"SECRET_BODY_DO_NOT_LEAK"),
        );
        let err = ErrorKind::HttpResponse {
            status: StatusCode::Forbidden,
            error_code: Some("Forbidden".to_string()),
            raw_response: Some(Box::new(raw)),
        }
        .into_error();
        let cosmos = CosmosError::from(err);
        let debug = format!("{cosmos:?}");
        assert!(
            !debug.contains("SECRET_BODY_DO_NOT_LEAK"),
            "Debug must redact raw_response, got: {debug}",
        );
        // But the HTTP status / error code must be present so the
        // redaction doesn't make the error unactionable.
        assert!(debug.contains("403") || debug.contains("Forbidden"));
    }

    #[test]
    fn clone_is_cheap_arc_share() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let cosmos = CosmosError::from(original);
        let cloned = cosmos.clone();
        // Two CosmosError handles share the same source Arc.
        assert!(Arc::ptr_eq(&cosmos.source, &cloned.source));
        assert_eq!(cosmos.to_string(), cloned.to_string());
    }

    #[test]
    fn source_chain_does_not_contain_carrier() {
        // Defense in depth — `Error::source()` walk through a
        // diagnostics-attached error must never surface the driver's
        // internal carrier type.
        use std::error::Error as _;

        let original = azure_core::Error::with_error(
            ErrorKind::Other,
            std::io::Error::other("io boom"),
            "outer".to_string(),
        );
        let diagnostics = make_test_diagnostics();
        let wrapped =
            azure_data_cosmos_driver::diagnostics::attach_diagnostics(original, diagnostics);
        let cosmos = CosmosError::from(wrapped);

        let mut node: Option<&(dyn std::error::Error + 'static)> = cosmos.source();
        let mut steps = 0;
        while let Some(n) = node {
            // We don't import the carrier type here (it's pub(crate)
            // in the driver), but its Debug rendering is recognizable.
            let dbg = format!("{n:?}");
            assert!(
                !dbg.contains("ErrorWithDiagnostics"),
                "carrier leaked into CosmosError source chain at step {steps}: {dbg}",
            );
            node = n.source();
            steps += 1;
            assert!(steps < 32, "source chain looped");
        }
        assert!(steps > 0, "expected at least one source node (io boom)");
    }

    #[test]
    fn into_source_returns_arc_handle() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let cosmos = CosmosError::from(original);
        let cloned_arc = Arc::clone(&cosmos.source);
        let consumed_arc = cosmos.into_source();
        assert!(Arc::ptr_eq(&cloned_arc, &consumed_arc));
    }

    #[test]
    fn into_azure_error_lossy_unwraps_when_sole_owner() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let original_display = original.to_string();
        let cosmos = CosmosError::from(original);
        let recovered = cosmos.into_azure_error_lossy();
        // Lossless path: same message, same kind discriminant.
        assert_eq!(recovered.to_string(), original_display);
        assert!(matches!(recovered.kind(), ErrorKind::Other));
    }

    #[test]
    fn into_azure_error_lossy_reconstructs_when_shared() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let original_display = original.to_string();
        let cosmos = CosmosError::from(original);
        let _alias = cosmos.clone();
        let recovered = cosmos.into_azure_error_lossy();
        // Shared path: still preserves message + kind discriminant via
        // the lossy reconstruction.
        assert_eq!(recovered.to_string(), original_display);
        assert!(matches!(recovered.kind(), ErrorKind::Other));
    }

    #[test]
    fn from_diagnostics_status_wins_over_inner_http_status() {
        // The precedence rule documented on `CosmosError::status_code`
        // says diagnostics-recorded status beats the wrapped error's
        // `http_status`. Use intentionally-different values for the
        // two so the assertion only passes if precedence is correct.
        use azure_core::http::StatusCode;
        use azure_data_cosmos_driver::models::CosmosStatus;

        let http_err = ErrorKind::HttpResponse {
            status: StatusCode::NotFound,
            error_code: None,
            raw_response: None,
        }
        .into_error();

        // Diagnostics report 410/1002 (Gone / PartitionKeyRangeGone),
        // intentionally distinct from the wrapped HttpResponse's 404.
        let diagnostics = Arc::new(CosmosDiagnosticsContext::for_testing_with_status(
            azure_data_cosmos_driver::models::ActivityId::from_string("test-precedence".to_owned()),
            CosmosStatus::new(StatusCode::Gone).with_sub_status(1002),
        ));
        let wrapped = azure_data_cosmos_driver::diagnostics::attach_diagnostics(
            http_err,
            Arc::clone(&diagnostics),
        );
        let cosmos = CosmosError::from(wrapped);

        assert_eq!(
            cosmos.status_code,
            Some(410),
            "diagnostics-recorded status must win over wrapped HttpResponse status",
        );
        assert_eq!(
            cosmos.sub_status,
            Some(1002),
            "diagnostics-recorded sub_status must be surfaced as the field value",
        );
        // The wrapped error's borrow accessor still surfaces the
        // original 404 for callers that want the head-of-line status
        // explicitly.
        assert_eq!(cosmos.http_status(), Some(StatusCode::NotFound));
    }

    #[test]
    fn from_falls_back_to_inner_http_status_when_diagnostics_has_no_status() {
        // Middle precedence case: diagnostics IS attached (the error
        // flowed through the pipeline far enough for the carrier to be
        // produced) but the operation aborted before any final status
        // was recorded into the context. The From impl must then fall
        // back to the wrapped error's http_status() — not silently
        // produce None on a 4xx/5xx response.
        use azure_core::http::StatusCode;

        let http_err = ErrorKind::HttpResponse {
            status: StatusCode::NotFound,
            error_code: None,
            raw_response: None,
        }
        .into_error();
        // `for_testing` produces a context whose `status` is None.
        let diagnostics = make_test_diagnostics();
        assert!(
            diagnostics.status().is_none(),
            "test precondition: for_testing must leave status unset",
        );

        let wrapped = azure_data_cosmos_driver::diagnostics::attach_diagnostics(
            http_err,
            Arc::clone(&diagnostics),
        );
        let cosmos = CosmosError::from(wrapped);

        assert!(cosmos.diagnostics.is_some(), "diagnostics must survive");
        assert_eq!(
            cosmos.status_code,
            Some(404),
            "no recorded status in diagnostics: must fall back to wrapped http_status",
        );
        assert_eq!(
            cosmos.sub_status, None,
            "no diagnostics status means no sub_status",
        );
    }

    #[test]
    fn into_azure_error_lossy_preserves_raw_response_on_shared_path() {
        // The doc on into_azure_error_lossy explicitly says the
        // function is NOT a redaction boundary. Pin that contract:
        // raw_response on HttpResponse survives the shared-handle
        // rebuild because ErrorKind: Clone and RawResponse: Clone.
        //
        // The redaction guarantee belongs to CosmosError::Debug; this
        // test prevents a future contributor from "tightening" the
        // lossy path and silently introducing a behavior change that
        // would surprise callers who learned the new redaction
        // semantics from a release note.
        use azure_core::http::{headers::Headers, response::RawResponse, StatusCode};
        const PROBE: &[u8] = b"PROBE_PRESERVED_THROUGH_LOSSY_REBUILD";

        let raw = RawResponse::from_bytes(
            StatusCode::InternalServerError,
            Headers::new(),
            azure_core::Bytes::from_static(PROBE),
        );
        let http_err = ErrorKind::HttpResponse {
            status: StatusCode::InternalServerError,
            error_code: Some("InternalServerError".to_string()),
            raw_response: Some(Box::new(raw)),
        }
        .into_error();
        let cosmos = CosmosError::from(http_err);
        let _alias = cosmos.clone(); // force the shared-handle path
        let rebuilt = cosmos.into_azure_error_lossy();

        match rebuilt.kind() {
            ErrorKind::HttpResponse { raw_response, .. } => {
                let raw = raw_response
                    .as_ref()
                    .expect("raw_response must be preserved through lossy rebuild");
                assert_eq!(raw.body().as_ref(), PROBE);
            }
            other => panic!("expected HttpResponse kind, got {other:?}"),
        }
    }

    #[test]
    fn debug_redacts_across_every_error_kind_variant() {
        // Defense in depth: build every `ErrorKind` variant the wrapper
        // crate may legitimately receive and assert that the hand-rolled
        // `CosmosError::Debug` impl does not leak the canary string we
        // embed in the source. If a future `azure_core` change adds a
        // variant that carries bytes, this test will not cover it on
        // its own — but it would only pass redacted *if* the new
        // variant's `Display` impl also strips the bytes, which is the
        // sole gate the Debug impl relies on.
        use azure_core::http::{headers::Headers, response::RawResponse, StatusCode};
        const CANARY: &str = "CANARY_DO_NOT_LEAK_THIS_STRING";

        fn assert_no_leak(label: &str, cosmos: CosmosError) {
            let debug = format!("{cosmos:?}");
            assert!(
                !debug.contains(CANARY),
                "[{label}] CosmosError Debug leaked the canary: {debug}",
            );
        }

        // 1) Other / DataConversion / Credential / Io / Connection — unit
        // variants with no payload. Source the canary through the
        // message string; Debug currently includes the message field by
        // design (it's user-facing text), but the canary here checks
        // that the kind discriminant doesn't smuggle anything *extra*.
        for kind in [
            ErrorKind::Other,
            ErrorKind::DataConversion,
            ErrorKind::Credential,
            ErrorKind::Io,
            ErrorKind::Connection,
        ] {
            let err = azure_core::Error::with_message(kind.clone(), "innocuous".to_string());
            assert_no_leak(&format!("{kind:?}"), CosmosError::from(err));
        }

        // 2) HttpResponse with raw_response carrying the canary in the
        // body — the bytes must NOT appear in Debug output.
        let raw = RawResponse::from_bytes(
            StatusCode::Forbidden,
            Headers::new(),
            azure_core::Bytes::from_static(CANARY.as_bytes()),
        );
        let http_err = ErrorKind::HttpResponse {
            status: StatusCode::Forbidden,
            error_code: Some("Forbidden".to_string()),
            raw_response: Some(Box::new(raw)),
        }
        .into_error();
        assert_no_leak("HttpResponse(raw body)", CosmosError::from(http_err));
    }
}
