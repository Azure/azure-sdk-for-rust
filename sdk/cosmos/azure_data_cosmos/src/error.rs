// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosError`] — the error type returned by every public Cosmos SDK
//! API.
//!
//! `CosmosError` is a transparent record (à la
//! [`azure_storage_blob::StorageError`](https://docs.rs/azure_storage_blob))
//! with public fields. The wrapped [`azure_core::Error`] is exposed via
//! `source`; the per-operation diagnostics (`status_code`, `sub_status`,
//! `diagnostics`) are extracted *once* at the SDK boundary and stored as
//! plain fields so callers can inspect, project, and serialize the error
//! without going through accessor methods or downcasts.
//!
//! ## Design (data-oriented)
//!
//! - **Transparent fields, not encapsulated state.** Every interesting
//!   datum is a `pub` field. Serialization, pattern matching, and
//!   field-by-field projection (e.g. into ADX columns) are first-class
//!   without bespoke API surface area.
//! - **Single computation site.** `From<azure_core::Error>` is the
//!   only place that extracts diagnostics from the driver's internal
//!   carrier and computes the final status fields. No duplicated
//!   precedence rules between accessors and `Debug`.
//! - **Carrier never escapes the driver.** The driver attaches a
//!   `pub(crate) ErrorWithDiagnostics` to its `azure_core::Error`s
//!   for transport between pipeline and SDK boundary; this module
//!   peels it back off via
//!   [`azure_data_cosmos_driver::diagnostics::split_diagnostics_carrier`],
//!   so `CosmosError::source.source()` chains contain only original
//!   wrapped errors. No `source_skipping_carrier` walk on every call.
//!
//! ## `Debug` redaction
//!
//! `CosmosError` hand-rolls `Debug` because `azure_core::Error`'s
//! derived `Debug` would expose `ErrorKind::HttpResponse::raw_response`
//! verbatim, including captured response headers (session tokens,
//! `WWW-Authenticate` challenges, masked-key reason text). The
//! hand-rolled impl prints the `ErrorKind` via its `Display`
//! implementation (which intentionally strips `raw_response`) plus
//! the top-level message and the four data fields.
//!
//! ## Interop with `azure_core::Error`
//!
//! - `From<azure_core::Error> for CosmosError` — automatic at the SDK
//!   boundary, including via `?` inside `CosmosResult<T>` returning
//!   code. This is the bridge from the driver's `azure_core::Result`
//!   surface to the wrapper crate's `CosmosResult`.
//! - `as_azure_error(&self) -> &azure_core::Error` — borrow the
//!   wrapped error for inspection (`http_status()`, `into_inner()`, …).
//!   The wrapped error has the driver's internal diagnostics carrier
//!   already stripped, so source-chain walks observe only original
//!   wrapped errors.
//! - `into_source(self) -> Arc<azure_core::Error>` — owned handle for
//!   bridging upward (e.g. surfacing through code that holds
//!   `Result<T, azure_core::Error>`). Note that `azure_core::Error`
//!   is `!Clone`, so the field is `Arc<…>` and ownership is a shared
//!   handle, not an owned value.
//!
//! ## `From` impls and the `?` operator
//!
//! `?` only performs a single `From` hop. `CosmosError` provides
//! direct `From` impls for the conversions that recur most often
//! inside the SDK:
//!
//! - `From<azure_core::Error>` (the SDK boundary type)
//! - `From<serde_json::Error>` (request/response body serialize and
//!   deserialize is the dominant `?` site)
//!
//! Other conversion-error types (`base64::DecodeError`,
//! `url::ParseError`, `std::num::ParseIntError`, …) intentionally do
//! **not** have a direct impl. Reach for
//! `.map_err(azure_core::Error::from)?` at those call sites — the
//! existing `From<azure_core::Error> for CosmosError` then completes
//! the conversion. This keeps the `From` surface narrow enough that
//! reviewers can reason about it locally without scanning every `?`
//! in the crate.
//!
//! ## Example
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
/// `error` module documentation in source for the design rationale.
///
/// The struct is `#[non_exhaustive]` so future preview revisions can
/// add fields without breaking pattern matches; callers should use
/// field access (`err.status_code`) and the canonical helper
/// constructors (`CosmosError::from(azure_err)`) rather than struct
/// literals.
#[derive(Clone)]
#[non_exhaustive]
pub struct CosmosError {
    /// The wrapped [`azure_core::Error`]. The driver's internal
    /// diagnostics carrier (if any) has already been stripped at
    /// construction time, so callers walking `source.source()` see
    /// only the original wrapped error chain.
    ///
    /// Stored as `Arc<…>` because [`azure_core::Error`] is `!Clone`
    /// and many call sites want to fan-out (retry, telemetry, ADX
    /// upload, return to caller). Cloning a `CosmosError` is a single
    /// atomic refcount bump.
    pub source: Arc<azure_core::Error>,
    /// The operation's final HTTP status code, if known.
    ///
    /// Populated from the driver-attached diagnostics context's
    /// recorded final status (which reflects the outcome after all
    /// retries and failovers); falls back to the
    /// [`azure_core::Error::http_status`] on the wrapped error when
    /// no diagnostics were attached (e.g. on credential or
    /// argument-validation failures).
    pub status_code: Option<u16>,
    /// The operation's Cosmos sub-status code
    /// (`x-ms-substatus` response header), if known.
    ///
    /// `None` when no diagnostics were attached or when the response
    /// did not include a sub-status header. The complete catalog of
    /// codes is documented at
    /// <https://learn.microsoft.com/en-us/rest/api/cosmos-db/http-status-codes-for-cosmosdb>.
    pub sub_status: Option<u32>,
    /// The per-operation diagnostics context attached by the driver
    /// pipeline, if any.
    ///
    /// `None` for errors that did not flow through the driver
    /// pipeline, or that escaped before diagnostics had been
    /// initialized. Stored as `Option<Arc<…>>` because the same
    /// context is also reachable from the success path
    /// (e.g. on `ItemResponse::diagnostics`) and the two should
    /// share storage when produced by the same operation.
    pub diagnostics: Option<Arc<CosmosDiagnosticsContext>>,
}

impl CosmosError {
    /// Borrow the wrapped [`azure_core::Error`].
    ///
    /// Equivalent to `&*self.source`; provided for ergonomics on
    /// `&CosmosError` so call sites don't have to deref the `Arc`
    /// explicitly. The wrapped error has the driver's internal
    /// diagnostics carrier already stripped, so source-chain walks
    /// observe only original wrapped errors.
    #[inline]
    pub fn as_azure_error(&self) -> &azure_core::Error {
        &self.source
    }

    /// Consume the `CosmosError` and return an owned `Arc` handle to
    /// the wrapped [`azure_core::Error`].
    ///
    /// The returned value is an `Arc` because [`azure_core::Error`] is
    /// `!Clone` and this `CosmosError` itself supports cheap fan-out
    /// via `Clone` — there is no way to unconditionally hand out an
    /// owned `azure_core::Error` from a possibly-shared handle.
    /// Callers that have a `Result<T, azure_core::Error>` to satisfy
    /// can either:
    ///
    /// 1. Use [`Self::into_azure_error_lossy`] (always succeeds; may
    ///    drop `raw_response` / inner if the handle is shared).
    /// 2. Call `Arc::try_unwrap(err.into_source())` themselves and
    ///    fall back to whatever projection their use case prefers.
    #[inline]
    pub fn into_source(self) -> Arc<azure_core::Error> {
        self.source
    }

    /// Consume the `CosmosError` and return an owned [`azure_core::Error`].
    ///
    /// If this `CosmosError` is the sole owner of the wrapped
    /// [`Arc`], the original error is returned unchanged. Otherwise a
    /// fresh `azure_core::Error` is constructed from the wrapped
    /// error's `kind()` (cloned, including any `raw_response`
    /// payload) and `to_string()` — this preserves the `ErrorKind`
    /// variant data and the user-visible message but drops the
    /// original inner source (callers walking `Error::source()` will
    /// see an empty chain on the shared-handle path).
    ///
    /// **Redaction note.** This converter is *not* a redaction
    /// boundary. The returned `azure_core::Error` uses
    /// `azure_core`'s derived `Debug`, which walks `ErrorKind` and
    /// emits `HttpResponse::raw_response` verbatim — including any
    /// captured response headers and body bytes. `CosmosError`'s
    /// hand-rolled `Debug` is the surface that scrubs `raw_response`;
    /// if you need that guarantee, log `{cosmos_err:?}` directly
    /// rather than converting first.
    ///
    /// Prefer [`Self::as_azure_error`] when you only need a borrow.
    pub fn into_azure_error_lossy(self) -> azure_core::Error {
        match Arc::try_unwrap(self.source) {
            Ok(err) => err,
            Err(shared) => {
                // Shared-handle path: rebuild from the cloned kind +
                // top-level message. The original inner source chain
                // is dropped; raw_response (if any) is cloned through
                // because `ErrorKind` derives `Clone` and so does
                // `RawResponse`. See the redaction note in the
                // rustdoc above.
                let kind = shared.kind().clone();
                let display = shared.to_string();
                azure_core::Error::with_message(kind, display)
            }
        }
    }

    /// Borrow the wrapped error's [`ErrorKind`](azure_core::error::ErrorKind).
    ///
    /// Forwarded from `self.source.kind()` — provided so the most
    /// common error-classification pattern doesn't need to spell out
    /// the borrow accessor.
    #[inline]
    pub fn kind(&self) -> &azure_core::error::ErrorKind {
        self.source.kind()
    }

    /// Returns the HTTP status code carried by the wrapped error's
    /// [`ErrorKind::HttpResponse`](azure_core::error::ErrorKind::HttpResponse),
    /// if any.
    ///
    /// This reflects the status code present on the wrapped
    /// [`azure_core::Error`] (i.e. the response that produced this
    /// error). For the operation's **final** status code (which can
    /// differ from this when a retry recovered and then ultimately
    /// failed for a different reason), use [`Self::status_code`].
    #[inline]
    pub fn http_status(&self) -> Option<azure_core::http::StatusCode> {
        self.source.http_status()
    }

    /// Convenience accessor — equivalent to `self.status_code`.
    ///
    /// Provided so callers can write `err.status_code()` without
    /// touching the field directly. New code may use either; the
    /// field access is preferred in places that already destructure
    /// the error or want to forward the value via `serde`.
    #[inline]
    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }

    /// Convenience accessor — equivalent to `self.sub_status`.
    ///
    /// Provided so callers can write `err.sub_status()` without
    /// touching the field directly. New code may use either; the
    /// field access is preferred in places that already destructure
    /// the error or want to forward the value via `serde`.
    #[inline]
    pub fn sub_status(&self) -> Option<u32> {
        self.sub_status
    }

    /// Borrow the per-operation diagnostics context, if attached.
    ///
    /// Returns `Option<&CosmosDiagnosticsContext>` rather than
    /// `Option<Arc<…>>` to avoid an unnecessary atomic refcount bump
    /// on every read (and to match the
    /// [`std::error::Error::source`]-style borrow convention). For an
    /// owned handle, clone the field directly:
    /// `err.diagnostics.clone()`.
    #[inline]
    pub fn diagnostics(&self) -> Option<&CosmosDiagnosticsContext> {
        self.diagnostics.as_deref()
    }
}

impl std::fmt::Display for CosmosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Delegate to the wrapped error's Display. azure_core::Error's
        // Display prints the top-level message only and does not walk
        // the source chain or expose `raw_response`.
        std::fmt::Display::fmt(&*self.source, f)
    }
}

impl std::fmt::Debug for CosmosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // CRITICAL: do NOT delegate to `azure_core::Error`'s derived
        // Debug. The derived Debug walks `Repr` and emits
        // `ErrorKind::HttpResponse { raw_response: Some(RawResponse {
        // headers: Headers { … session tokens, WWW-Authenticate, … },
        // body: <bytes> }) }` verbatim — anything that ends up in
        // `format!("{err:?}")` (panic messages, tracing fields, etc.)
        // would leak that data into logs.
        //
        // Instead we project a redacted, stable shape:
        //   - `kind`: the `ErrorKind` formatted via its *Display* impl
        //     (which is the canonical user-facing rendering and is
        //     hand-rolled to strip `raw_response`).
        //   - `message`: the wrapped error's top-level Display string.
        //   - the four data fields.
        //
        // If `azure_core::error::ErrorKind` ever gains a new variant
        // that carries bytes, the Display impl is the same gate that
        // protects this Debug impl — keeping a single redaction
        // contract in one place.
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
        // The wrapped error has had the diagnostics carrier already
        // stripped by `From::from` (via `split_diagnostics_carrier`),
        // so a plain delegation is correct — callers walking
        // `.source()` see the original error chain without the
        // internal carrier.
        Some(&*self.source)
    }
}

impl From<azure_core::Error> for CosmosError {
    fn from(err: azure_core::Error) -> Self {
        // **Construction-invariant gate** — this `From` impl is the
        // canonical (and only intended) way to construct a
        // `CosmosError`. The `pub` fields make struct-literal
        // construction technically possible from anywhere inside this
        // crate, but doing so would bypass the carrier-strip step
        // below and break the docstring contract on `source` that
        // "the driver's internal diagnostics carrier (if any) has
        // already been stripped at construction time." Any new
        // in-crate construction site SHOULD go through `From` or one
        // of its conversion variants (e.g. `From<serde_json::Error>`,
        // which itself defers to this impl). External callers are
        // blocked from struct-literal construction by
        // `#[non_exhaustive]`.
        //
        // Single-shot extraction at the SDK boundary:
        //   1. peel the driver's diagnostics carrier off the chain
        //      (if present); the rebuilt `azure_core::Error` is the
        //      pre-carrier error with the original inner intact.
        //   2. compute `status_code` / `sub_status` from the
        //      diagnostics context's recorded final status; fall back
        //      to the rebuilt error's `http_status()` for errors that
        //      never reached the pipeline (e.g. credential failures).
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

// `serde_json::Error` is the dominant `?` site in request-body
// serialization and response-body parsing — provide a direct
// conversion so call sites don't have to spell `.map_err` on every
// `?`. See the module-level docs ("`From` impls and the `?` operator")
// for the policy that governs which conversions are direct vs
// explicit.
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
