// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-owned newtype wrapper around the driver's [`Error`].
//!
//! The wrapper is `#[repr(transparent)]` so converting between the SDK and
//! driver representations is a zero-cost move. All construction, classification,
//! status-code constants, and predicates live in the driver crate
//! (`azure_data_cosmos_driver::error`); the SDK layer adds only thin
//! delegating accessors and the public [`Result`] alias.

use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use azure_core::http::StatusCode;
use azure_data_cosmos_driver::error::Error as DriverError;
pub use azure_data_cosmos_driver::error::Kind;
use azure_data_cosmos_driver::models::{CosmosStatus, SubStatusCode};

use crate::models::{DiagnosticsContext, ResponseHeaders};

/// The error type returned by every fallible public API in `azure_data_cosmos`.
///
/// `Error` carries the typed Cosmos status (HTTP status + sub-status,
/// including synthetic client-side codes such as `408 / 20008` for end-to-end
/// operation timeout), the parsed Cosmos response headers when a service
/// response was received, and the operation diagnostics — for both
/// service-side and client-side failures.
///
/// `azure_core::Error` (and any other underlying source) is reachable via
/// [`std::error::Error::source`].
#[repr(transparent)]
#[derive(Clone)]
pub struct Error(DriverError);

impl Error {
    /// Returns the categorical [`Kind`].
    pub fn kind(&self) -> Kind {
        self.0.kind()
    }

    /// Returns the typed Cosmos status. Always present — non-service errors
    /// carry a synthetic status with a placeholder HTTP code and the correct
    /// [`Kind`].
    pub fn status(&self) -> CosmosStatus {
        self.0.status()
    }

    /// Returns the HTTP status code. For non-service errors this is a
    /// placeholder code corresponding to the error's [`Kind`].
    pub fn status_code(&self) -> StatusCode {
        self.0.status_code()
    }

    /// Returns the sub-status code, if present.
    pub fn sub_status(&self) -> Option<SubStatusCode> {
        self.0.sub_status()
    }

    /// Returns the parsed Cosmos response headers (when a service response was
    /// received).
    pub fn cosmos_headers(&self) -> Option<&ResponseHeaders> {
        self.0
            .cosmos_headers()
            .map(ResponseHeaders::from_driver_ref)
    }

    /// Returns the diagnostics context for the failed operation.
    pub fn diagnostics(&self) -> Option<&Arc<DiagnosticsContext>> {
        self.0.diagnostics()
    }

    /// Returns the raw service response body bytes when available
    /// (e.g. the JSON error payload returned by Cosmos for a
    /// 400 / BadRequest response). Only populated for `Service` errors.
    ///
    /// Prefer [`cosmos_headers`](Self::cosmos_headers) and
    /// [`status`](Self::status) for structured access; this accessor
    /// exists for inspecting the wire-level service error payload.
    pub fn response_body(&self) -> Option<&[u8]> {
        self.0.response_body()
    }

    /// Returns the stack backtrace captured at error construction time,
    /// rendered as a human-readable string, when the global rate-limited
    /// capture budget allowed it.
    ///
    /// Capture itself is unconditional (cheap stack walk); the expensive
    /// part — resolving instruction pointers to symbol names — is
    /// rate-limited (default `5` resolutions per second, configurable via
    /// the driver's
    /// [`CosmosDriverRuntimeBuilder::with_max_error_backtraces_per_second`](azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder::with_max_error_backtraces_per_second)
    /// or the `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` environment
    /// variable). Cache hits do not consume budget. Returns `None` when
    /// the limiter denied fresh resolution for at least one cache-missed
    /// frame; partial backtraces are never produced.
    pub fn backtrace(&self) -> Option<&str> {
        self.0.backtrace()
    }

    // -- predicates --

    /// `true` if this is a service-side error (`Service` kind).
    pub fn is_service_error(&self) -> bool {
        self.0.is_service_error()
    }

    /// `true` if the request was throttled (HTTP 429).
    pub fn is_throttled(&self) -> bool {
        self.0.is_throttled()
    }

    /// `true` if the resource was not found (HTTP 404).
    pub fn is_not_found(&self) -> bool {
        self.0.is_not_found()
    }

    /// `true` if the operation hit a conflict (HTTP 409).
    pub fn is_conflict(&self) -> bool {
        self.0.is_conflict()
    }

    /// `true` if a precondition was not met (HTTP 412).
    pub fn is_precondition_failed(&self) -> bool {
        self.0.is_precondition_failed()
    }

    /// `true` if the status is HTTP 408 (server timeout or synthetic
    /// client-side end-to-end timeout).
    pub fn is_timeout(&self) -> bool {
        self.0.is_timeout()
    }

    /// `true` if this is an HTTP 410 Gone response.
    pub fn is_gone(&self) -> bool {
        self.0.is_gone()
    }

    /// `true` if the error is generally considered transient and could be
    /// retried by a higher layer.
    pub fn is_transient(&self) -> bool {
        self.0.is_transient()
    }

    // -- construction & interop helpers --

    /// Builds a `Client` error (caller misuse / precondition), optionally
    /// wrapping an underlying source error.
    pub(crate) fn client(
        message: impl Into<std::borrow::Cow<'static, str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self(DriverError::client(message, source))
    }

    /// Builds a `Configuration` error (bad endpoint URL, malformed connection
    /// string, etc.), optionally wrapping an underlying source error.
    pub(crate) fn configuration(
        message: impl Into<std::borrow::Cow<'static, str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self(DriverError::configuration(message, source))
    }

    /// Builds a `Serialization` error wrapping the underlying serde failure.
    pub(crate) fn serialization(
        message: impl Into<std::borrow::Cow<'static, str>>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self(DriverError::serialization(message, None, None, source))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

impl From<DriverError> for Error {
    fn from(inner: DriverError) -> Self {
        Self(inner)
    }
}

impl From<Error> for DriverError {
    fn from(value: Error) -> Self {
        value.0
    }
}

impl From<azure_core::Error> for Error {
    fn from(error: azure_core::Error) -> Self {
        Self(DriverError::from(error))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self(DriverError::serialization(
            "JSON serialization or deserialization failed",
            None,
            None,
            error,
        ))
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Self(DriverError::configuration(
            "invalid URL",
            Some(Arc::new(error)),
        ))
    }
}

/// `azure_data_cosmos` crate-wide `Result` alias.
pub type Result<T> = std::result::Result<T, Error>;
