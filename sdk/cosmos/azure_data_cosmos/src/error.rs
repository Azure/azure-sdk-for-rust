// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! [`CosmosError`] — the error type returned by every public Cosmos SDK
//! API.
//!
//! [`CosmosError`] is a thin newtype around [`azure_core::Error`] that adds
//! Cosmos-specific accessors for the operation's final HTTP status code,
//! sub-status code, and the per-operation [`CosmosDiagnosticsContext`]
//! (when the driver pipeline attached one).
//!
//! ## Why a Cosmos-specific error type
//!
//! Direct use of [`azure_core::Error`] in the public surface forced
//! callers to depend on driver-internal types
//! (`ErrorWithDiagnostics`, `CosmosStatus`, `SubStatusCode`) to read
//! the most useful per-operation diagnostics on failure. [`CosmosError`]
//! hides those driver types behind a stable public API and provides the
//! accessors directly.
//!
//! ## Source-chain behavior
//!
//! [`CosmosError`] implements [`std::error::Error`] but its
//! `source()` skips the internal diagnostics carrier so callers see the
//! same chain they would have seen if no diagnostics had been attached
//! (i.e. either the original wrapped error's inner, or `None`).
//!
//! ## Interop with `azure_core::Error`
//!
//! - `CosmosError: From<azure_core::Error>` — conversion at the SDK
//!   boundary is automatic, including via the `?` operator inside
//!   `CosmosResult<T>` returning code.
//! - `azure_core::Error: From<CosmosError>` — callers that need to
//!   surface a Cosmos error through an existing
//!   `Result<T, azure_core::Error>` can use `?` to convert upward.
//!
//! ## `From` impls and the `?` operator
//!
//! The Rust `?` operator only performs a *single* `From` hop, so
//! `?`-chaining a non-`azure_core` error (e.g. `serde_json::Error`)
//! into a function returning `CosmosResult<T>` requires either a
//! direct `From<E> for CosmosError` impl or an explicit
//! `.map_err(azure_core::Error::from)?`.
//!
//! `CosmosError` provides `From` impls for the conversions that
//! recur the most often inside the SDK itself:
//!
//! - `From<azure_core::Error>` (the boundary type)
//! - `From<serde_json::Error>` (request/response body
//!   serialize/deserialize is the dominant `?` site)
//!
//! Other conversion-error types (`base64::DecodeError`,
//! `url::ParseError`, `std::num::ParseIntError`, …) intentionally do
//! **not** have a direct impl. Reach for
//! `.map_err(azure_core::Error::from)?` at those call sites — the
//! existing `From<azure_core::Error> for CosmosError` then completes
//! the conversion. This keeps the `From` surface narrow enough that
//! reviewers can reason about it locally without scanning every
//! `?` in the crate.
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
//!             eprintln!("status_code = {:?}", err.status_code());
//!             eprintln!("sub_status  = {:?}", err.sub_status());
//!             if let Some(diag) = err.diagnostics() {
//!                 eprintln!("activity_id = {:?}", diag.activity_id());
//!             }
//!             Err(err)
//!         }
//!     }
//! }
//! ```

use std::sync::Arc;

use azure_core::error::ErrorKind;

use crate::models::CosmosDiagnosticsContext;

/// Convenience type alias for `Result<T, CosmosError>` returned by every
/// public Cosmos SDK API.
pub type CosmosResult<T> = Result<T, CosmosError>;

/// Error type returned by every public Cosmos SDK API.
///
/// `CosmosError` is a newtype around [`azure_core::Error`] that adds
/// Cosmos-specific accessors for the operation's final HTTP status code,
/// sub-status code, and the per-operation [`CosmosDiagnosticsContext`]
/// (when the driver pipeline attached one).
///
/// See the [module-level documentation](self) for design rationale.
pub struct CosmosError(azure_core::Error);

impl CosmosError {
    /// Returns a reference to the wrapped [`azure_core::Error`].
    ///
    /// Most callers should not need this — prefer [`Self::kind`],
    /// [`Self::http_status`], [`Self::diagnostics`],
    /// [`Self::status_code`], and [`Self::sub_status`]. Use this only
    /// when interop with code that holds an `&azure_core::Error` is
    /// required.
    pub fn as_azure_error(&self) -> &azure_core::Error {
        &self.0
    }

    /// Consumes this `CosmosError` and returns the wrapped
    /// [`azure_core::Error`].
    ///
    /// Useful when bubbling the error through code that expects the
    /// generic Azure SDK error type. The diagnostics context (if any)
    /// remains attached to the returned error and can be retrieved by
    /// calling [`Self::diagnostics`] before conversion.
    pub fn into_azure_error(self) -> azure_core::Error {
        self.0
    }

    /// Returns the [`ErrorKind`] of the wrapped [`azure_core::Error`].
    ///
    /// For HTTP failures this is [`ErrorKind::HttpResponse`] and
    /// includes the status, error code, and (when retained) the raw
    /// response body.
    ///
    /// **Note on driver-type hiding.** [`ErrorKind`] is re-exported
    /// directly from [`azure_core`]; the `CosmosError` newtype hides
    /// driver-internal types (`ErrorWithDiagnostics`, `CosmosStatus`,
    /// `SubStatusCode`) but does **not** abstract over `azure_core`'s
    /// public error vocabulary. Callers that need richer inspection
    /// than the convenience accessors below provide should reach for
    /// [`Self::as_azure_error`] explicitly.
    pub fn kind(&self) -> &ErrorKind {
        self.0.kind()
    }

    /// Returns the per-operation [`CosmosDiagnosticsContext`] attached
    /// to this error by the driver pipeline, if any.
    ///
    /// Returns `None` for errors that did not flow through the driver
    /// pipeline, or that escaped before diagnostics had been
    /// initialized (e.g. argument-validation failures).
    pub fn diagnostics(&self) -> Option<Arc<CosmosDiagnosticsContext>> {
        azure_data_cosmos_driver::diagnostics::try_extract_diagnostics(&self.0)
    }

    /// Returns the operation's final HTTP status code, if known.
    ///
    /// Prefers the status recorded on the diagnostics context (which
    /// reflects the final outcome after retries and failovers); falls
    /// back to the [`ErrorKind::HttpResponse`] status on the wrapped
    /// [`azure_core::Error`] when no diagnostics are available.
    ///
    /// The pre-retry / head-of-line status (which can differ when a
    /// retry recovered then ultimately failed for a different reason)
    /// is reachable via `err.as_azure_error().http_status()`.
    pub fn status_code(&self) -> Option<u16> {
        if let Some(diag) = self.diagnostics() {
            if let Some(code) = diag.status_code() {
                return Some(code);
            }
        }
        self.0.http_status().map(u16::from)
    }

    /// Returns the operation's Cosmos sub-status code, if any.
    ///
    /// Read from the diagnostics context's recorded final status.
    /// Returns `None` when diagnostics are absent or when the
    /// response did not include an `x-ms-substatus` header.
    pub fn sub_status(&self) -> Option<u32> {
        self.diagnostics().and_then(|d| d.sub_status())
    }
}

impl std::fmt::Display for CosmosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for CosmosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Build a stable public Debug representation that does NOT delegate
        // to `azure_core::Error::Debug`. The underlying Debug walks the
        // entire `source()` chain and would (a) expose the internal
        // `ErrorWithDiagnostics` carrier verbatim and (b) print the
        // `raw_response` bytes/headers nested under `ErrorKind::HttpResponse`
        // — including session tokens, `WWW-Authenticate` challenges, and
        // (on 403/2) the masked-key reason text. Neither belongs in
        // user-facing `format!("{err:?}")` output.
        let diag = self.diagnostics();
        let status = diag
            .as_ref()
            .and_then(|d| d.status_code())
            .or_else(|| self.0.http_status().map(u16::from));
        let sub_status = diag.as_ref().and_then(|d| d.sub_status());

        let mut s = f.debug_struct("CosmosError");
        match self.0.kind() {
            // Strip `raw_response` so headers / body bytes never enter
            // log output through `{:?}`. Callers that need them must
            // reach for `as_azure_error().kind()` explicitly.
            ErrorKind::HttpResponse {
                status,
                error_code,
                raw_response: _,
            } => {
                s.field("http_status", status)
                    .field("error_code", error_code);
            }
            other => {
                s.field("kind", other);
            }
        }
        s.field("message", &self.0.to_string());
        if let Some(code) = status {
            s.field("status_code", &code);
        }
        if let Some(code) = sub_status {
            s.field("sub_status", &code);
        }
        s.field("has_diagnostics", &diag.is_some());
        s.finish()
    }
}

impl std::error::Error for CosmosError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // Skip the internal diagnostics carrier (`ErrorWithDiagnostics`)
        // so callers see the same source chain they would have seen
        // without any diagnostics being attached.
        azure_data_cosmos_driver::diagnostics::source_skipping_carrier(&self.0)
    }
}

impl From<azure_core::Error> for CosmosError {
    fn from(err: azure_core::Error) -> Self {
        CosmosError(err)
    }
}

impl From<CosmosError> for azure_core::Error {
    fn from(err: CosmosError) -> Self {
        err.0
    }
}

// `serde_json::Error` arises in serialization / deserialization paths
// (request body construction and response body parsing). Provide a
// direct conversion so `?` works in public methods returning
// [`CosmosResult`] without an explicit `.map_err` per call site.
impl From<serde_json::Error> for CosmosError {
    fn from(err: serde_json::Error) -> Self {
        CosmosError(azure_core::Error::from(err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;

    #[test]
    fn from_round_trip_preserves_kind_and_display() {
        let original = azure_core::Error::with_message(ErrorKind::Other, "boom".to_string());
        let original_display = original.to_string();
        let cosmos: CosmosError = original.into();
        assert_eq!(cosmos.to_string(), original_display);
        assert!(matches!(cosmos.kind(), ErrorKind::Other));
        let back: azure_core::Error = cosmos.into();
        assert_eq!(back.to_string(), original_display);
    }

    #[test]
    fn diagnostics_none_for_plain_error() {
        let plain = azure_core::Error::with_message(ErrorKind::Other, "x".to_string());
        let cosmos = CosmosError::from(plain);
        assert!(cosmos.diagnostics().is_none());
        assert!(cosmos.status_code().is_none());
        assert!(cosmos.sub_status().is_none());
    }

    #[test]
    fn status_code_falls_through_to_inner_http_status() {
        // Without a diagnostics context attached, status_code() must
        // surface the HTTP status carried by the wrapped
        // `azure_core::Error` so 4xx/5xx responses are still
        // observable through the SDK accessor.
        use azure_core::http::StatusCode;
        let err = ErrorKind::HttpResponse {
            status: StatusCode::NotFound,
            error_code: None,
            raw_response: None,
        }
        .into_error();
        let cosmos = CosmosError::from(err);
        assert_eq!(cosmos.status_code(), Some(404));
        // The escape hatch still surfaces the typed StatusCode.
        assert_eq!(
            cosmos.as_azure_error().http_status(),
            Some(StatusCode::NotFound),
        );
    }

    #[test]
    fn source_skips_diagnostics_carrier_when_no_inner() {
        // When the original error has no inner, the public source chain
        // must be empty even though the internal carrier is present.
        use std::error::Error as _;
        use std::sync::Arc;

        let original = azure_core::Error::with_message(ErrorKind::Other, "no inner".to_string());

        let diagnostics = Arc::new(CosmosDiagnosticsContext::for_testing(
            azure_data_cosmos_driver::models::ActivityId::from_string(
                "test-cosmos-error".to_owned(),
            ),
        ));

        let with_diag = azure_data_cosmos_driver::diagnostics::attach_diagnostics(
            original,
            Arc::clone(&diagnostics),
        );
        let cosmos = CosmosError::from(with_diag);

        assert!(
            cosmos.source().is_none(),
            "CosmosError::source() must skip the internal diagnostics carrier",
        );
        // Diagnostics are still recoverable via the SDK accessor.
        assert!(cosmos.diagnostics().is_some());
    }
}
