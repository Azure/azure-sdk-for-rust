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
/// [module-level documentation](self) for the design rationale.
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
    /// Useful when surfacing a Cosmos error through code that holds
    /// `Result<T, azure_core::Error>`. Note that `azure_core::Error`
    /// is `!Clone`, so the returned value is an `Arc` handle (cheap
    /// fan-out, shared ownership) rather than an owned value.
    #[inline]
    pub fn into_source(self) -> Arc<azure_core::Error> {
        self.source
    }

    /// Convenience accessor — equivalent to `self.status_code`.
    ///
    /// Provided so call patterns like `err.status_code()` continue to
    /// compile across the v0.33 → v0.34 transition. New code should
    /// prefer the field directly.
    #[inline]
    pub fn status_code(&self) -> Option<u16> {
        self.status_code
    }

    /// Convenience accessor — equivalent to `self.sub_status`.
    ///
    /// Provided so call patterns like `err.sub_status()` continue to
    /// compile across the v0.33 → v0.34 transition. New code should
    /// prefer the field directly.
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
}
