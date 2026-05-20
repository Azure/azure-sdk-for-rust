// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB-specific error type carrying typed status, parsed Cosmos response
//! headers, and diagnostics — for both service errors (real HTTP responses) and
//! synthetic client-side conditions (e.g. end-to-end operation timeouts).
//!
//! The error mirrors the shape of the Java SDK's `CosmosException` and the
//! .NET SDK's `CosmosException`: a single error type that surfaces typed Cosmos
//! status (status code + sub-status, including synthetic codes such as
//! `408 / 20008` for end-to-end timeout), the parsed [`CosmosResponseHeaders`],
//! and the operation [`DiagnosticsContext`] regardless of whether the failure
//! was generated server-side or client-side.
//!
//! ## Flow through the pipeline
//!
//! Internal driver functions continue to return `azure_core::Result<T>` so that
//! existing `?` propagation works unchanged. When a Cosmos HTTP error or
//! transport failure is converted to an `azure_core::Error` (see
//! `From<CosmosError> for azure_core::Error` and
//! `crate::driver::pipeline::retry_evaluation::build_transport_error`), the constructed `CosmosError` is embedded as the
//! `source` of the `azure_core::Error`. At the driver/SDK boundary, callers
//! convert with `CosmosError::from(azure_core_error)` (or
//! `azure_core::Error::into()`), which walks the source chain and recovers the
//! typed payload via downcasting. If no embedded `CosmosError` is present the
//! conversion classifies the error from `azure_core::ErrorKind`.

use std::{borrow::Cow, error::Error as StdError, fmt, sync::Arc};

use azure_core::http::StatusCode;

use crate::{
    diagnostics::DiagnosticsContext,
    models::{CosmosResponseHeaders, CosmosStatus, SubStatusCode},
};

pub mod backtrace;
pub use backtrace::{
    capture_limiter, BacktraceCaptureLimiter, CosmosBacktrace, ResolvedFrame,
    BACKTRACE_CAPTURES_PER_MINUTE_ENV, DEFAULT_BACKTRACE_CAPTURES_PER_MINUTE,
    DEFAULT_BACKTRACE_KIND_MASK,
};

/// Categorical kind for a [`CosmosError`].
///
/// This is intentionally coarse-grained — fine-grained discrimination is done
/// via [`CosmosError::status`] / [`CosmosError::sub_status`] and the
/// `is_*` predicates.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CosmosErrorKind {
    /// The Cosmos service returned a non-success HTTP response.
    Service,
    /// A network / transport failure occurred before a response was received,
    /// or an end-to-end operation timeout fired. Carries a synthetic
    /// [`CosmosStatus`] (e.g. `408 / 20008`).
    Transport,
    /// A precondition required for the operation was not met on the client
    /// (bad argument, invalid configuration evaluated at request time, etc.).
    Client,
    /// Authentication or credential acquisition failed (e.g. AAD token
    /// retrieval, missing key).
    Authentication,
    /// Serialization or deserialization of the request/response body failed.
    Serialization,
    /// Static client configuration (connection string, endpoint URL, etc.) is
    /// invalid.
    Configuration,
    /// Anything that does not fit the categories above.
    Other,
}

impl fmt::Display for CosmosErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Service => "Service",
            Self::Transport => "Transport",
            Self::Client => "Client",
            Self::Authentication => "Authentication",
            Self::Serialization => "Serialization",
            Self::Configuration => "Configuration",
            Self::Other => "Other",
        };
        f.write_str(name)
    }
}

/// Cosmos DB error returned from every public API in the driver (and, by
/// re-export, every public API in the SDK).
///
/// Unlike `azure_core::Error`, `CosmosError` always exposes Cosmos-typed
/// status and parsed response headers when they are available — for both real
/// service errors and synthetic client-side conditions (e.g. an end-to-end
/// operation timeout surfaces as `408 / 20008` even though no HTTP response
/// was received).
///
/// `azure_core::Error` (and any other underlying error) is reachable via
/// [`std::error::Error::source`].
///
/// `CosmosError` is `Clone` (a cheap `Arc` refcount bump) so that it can be
/// extracted from an `azure_core::Error`'s `source()` chain by reference and
/// returned by value. All fields are wrapped behind a single `Arc` so the
/// outer struct is one pointer wide, keeping `Result<T, CosmosError>` small.
#[derive(Clone)]
pub struct CosmosError {
    inner: Arc<CosmosErrorInner>,
}

struct CosmosErrorInner {
    kind: CosmosErrorKind,
    status: Option<CosmosStatus>,
    cosmos_headers: Option<CosmosResponseHeaders>,
    /// Raw service response body bytes (e.g. the JSON error payload returned
    /// for a 400 / BadRequest). Only populated for `Service` errors and only
    /// when the pipeline has captured the response body. Stored as `Bytes`
    /// for cheap (refcount) cloning.
    response_body: Option<bytes::Bytes>,
    diagnostics: Option<Arc<DiagnosticsContext>>,
    message: Cow<'static, str>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Captured stack backtrace, present when the global rate-limited
    /// backtrace capture budget allowed it. See [`backtrace`] module.
    backtrace: Option<CosmosBacktrace>,
}

impl Clone for CosmosErrorInner {
    fn clone(&self) -> Self {
        Self {
            kind: self.kind,
            status: self.status,
            cosmos_headers: self.cosmos_headers.clone(),
            response_body: self.response_body.clone(),
            diagnostics: self.diagnostics.clone(),
            message: self.message.clone(),
            source: self.source.clone(),
            backtrace: self.backtrace.clone(),
        }
    }
}

impl CosmosError {
    fn from_inner(mut inner: CosmosErrorInner) -> Self {
        if inner.backtrace.is_none() {
            inner.backtrace = CosmosBacktrace::try_capture_for_kind(inner.kind);
        }
        Self {
            inner: Arc::new(inner),
        }
    }

    // -----------------------------------------------------------------
    // Constructors
    // -----------------------------------------------------------------

    /// Builds a `Service` error from a real Cosmos HTTP error response.
    ///
    /// `response_body` should be the raw service response body bytes when
    /// available — for example, the JSON error payload returned by the
    /// service for a 400 / BadRequest. Callers can inspect it later via
    /// [`response_body`](Self::response_body).
    pub fn service(
        status: CosmosStatus,
        headers: Option<CosmosResponseHeaders>,
        response_body: Option<bytes::Bytes>,
        diagnostics: Option<Arc<DiagnosticsContext>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Service,
            status: Some(status),
            cosmos_headers: headers,
            response_body,
            diagnostics,
            message: message.into(),
            source: None,
            backtrace: None,
        })
    }

    /// Builds a `Transport` error with an explicit synthetic Cosmos status
    /// (typically `503 / 21008` for transport-generated 503, or
    /// `408 / 20008` for end-to-end operation timeout).
    pub fn transport(
        status: CosmosStatus,
        message: impl Into<Cow<'static, str>>,
        diagnostics: Option<Arc<DiagnosticsContext>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Transport,
            status: Some(status),
            cosmos_headers: None,
            response_body: None,
            diagnostics,
            message: message.into(),
            source,
            backtrace: None,
        })
    }

    /// Convenience constructor for an end-to-end operation timeout
    /// (`408 / 20008`).
    pub fn end_to_end_timeout(
        message: impl Into<Cow<'static, str>>,
        diagnostics: Option<Arc<DiagnosticsContext>>,
    ) -> Self {
        Self::transport(
            CosmosStatus::from_parts(
                StatusCode::RequestTimeout,
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
            ),
            message,
            diagnostics,
            None,
        )
    }

    /// Builds a `Client` error (caller misuse / precondition).
    pub fn client(message: impl Into<Cow<'static, str>>) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Client,
            status: None,
            cosmos_headers: None,
            response_body: None,
            diagnostics: None,
            message: message.into(),
            source: None,
            backtrace: None,
        })
    }

    /// Builds a `Client` error wrapping a source error.
    pub fn client_with_source(
        message: impl Into<Cow<'static, str>>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Client,
            status: None,
            cosmos_headers: None,
            response_body: None,
            diagnostics: None,
            message: message.into(),
            source: Some(Arc::new(source)),
            backtrace: None,
        })
    }

    /// Builds an `Authentication` error.
    pub fn authentication(
        message: impl Into<Cow<'static, str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Authentication,
            status: None,
            cosmos_headers: None,
            response_body: None,
            diagnostics: None,
            message: message.into(),
            source,
            backtrace: None,
        })
    }

    /// Builds a `Serialization` error wrapping the underlying serde / JSON
    /// failure.
    ///
    /// `cosmos_headers` and `diagnostics` should be populated whenever the
    /// failure occurs while deserializing a response body or continuation
    /// token produced by a Cosmos operation — they give callers the request
    /// charge, activity id, and timeline needed to diagnose the failure.
    /// Pass `None` only when the failure is detached from any in-flight
    /// operation (e.g. parsing a user-supplied continuation token at the SDK
    /// boundary before any request has been issued).
    pub fn serialization(
        message: impl Into<Cow<'static, str>>,
        cosmos_headers: Option<CosmosResponseHeaders>,
        diagnostics: Option<Arc<DiagnosticsContext>>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Serialization,
            status: None,
            cosmos_headers,
            response_body: None,
            diagnostics,
            message: message.into(),
            source: Some(Arc::new(source)),
            backtrace: None,
        })
    }

    /// Builds a `Configuration` error (bad endpoint URL, malformed connection
    /// string, etc.).
    pub fn configuration(message: impl Into<Cow<'static, str>>) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Configuration,
            status: None,
            cosmos_headers: None,
            response_body: None,
            diagnostics: None,
            message: message.into(),
            source: None,
            backtrace: None,
        })
    }

    /// Builds a `Configuration` error wrapping a source error.
    pub fn configuration_with_source(
        message: impl Into<Cow<'static, str>>,
        source: impl StdError + Send + Sync + 'static,
    ) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Configuration,
            status: None,
            cosmos_headers: None,
            response_body: None,
            diagnostics: None,
            message: message.into(),
            source: Some(Arc::new(source)),
            backtrace: None,
        })
    }

    /// Builds an `Other` error.
    pub fn other(message: impl Into<Cow<'static, str>>) -> Self {
        Self::from_inner(CosmosErrorInner {
            kind: CosmosErrorKind::Other,
            status: None,
            cosmos_headers: None,
            response_body: None,
            diagnostics: None,
            message: message.into(),
            source: None,
            backtrace: None,
        })
    }

    // -----------------------------------------------------------------
    // Builders
    // -----------------------------------------------------------------

    /// Returns a mutable handle to the inner state, cloning the `Arc` payload
    /// if it is shared.
    fn inner_mut(&mut self) -> &mut CosmosErrorInner {
        Arc::make_mut(&mut self.inner)
    }

    /// Attaches parsed Cosmos response headers (replacing any existing value).
    #[must_use]
    pub fn with_cosmos_headers(mut self, headers: CosmosResponseHeaders) -> Self {
        self.inner_mut().cosmos_headers = Some(headers);
        self
    }

    /// Attaches diagnostics (replacing any existing value).
    #[must_use]
    pub fn with_diagnostics(mut self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        self.inner_mut().diagnostics = Some(diagnostics);
        self
    }

    /// Attaches a source error (replacing any existing value).
    #[must_use]
    pub fn with_source(mut self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self {
        self.inner_mut().source = Some(source);
        self
    }

    // -----------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------

    /// Returns the categorical kind of this error.
    pub fn kind(&self) -> CosmosErrorKind {
        self.inner.kind
    }

    /// Returns the typed Cosmos status (HTTP status code + optional sub-status)
    /// associated with this error. Populated for service errors and for
    /// transport / client errors that have a meaningful synthetic Cosmos code
    /// (e.g. `408 / 20008` for end-to-end timeout).
    pub fn status(&self) -> Option<CosmosStatus> {
        self.inner.status
    }

    /// Returns the HTTP status code, if known.
    pub fn status_code(&self) -> Option<StatusCode> {
        self.inner.status.map(|s| s.status_code())
    }

    /// Returns the sub-status code, if known.
    pub fn sub_status(&self) -> Option<SubStatusCode> {
        self.inner.status.and_then(|s| s.sub_status())
    }

    /// Returns the parsed Cosmos response headers (when a service response was
    /// received).
    pub fn cosmos_headers(&self) -> Option<&CosmosResponseHeaders> {
        self.inner.cosmos_headers.as_ref()
    }

    /// Returns the diagnostics context for the failed operation.
    pub fn diagnostics(&self) -> Option<&Arc<DiagnosticsContext>> {
        self.inner.diagnostics.as_ref()
    }

    /// Returns the error message.
    pub fn message(&self) -> &str {
        &self.inner.message
    }

    /// Returns the raw service response body bytes when available
    /// (e.g. the JSON error payload returned by Cosmos for a
    /// 400 / BadRequest response). Only populated for `Service` errors
    /// when the pipeline captured the body.
    ///
    /// Most callers should prefer [`cosmos_headers`](Self::cosmos_headers)
    /// and [`status`](Self::status) for structured access; this accessor
    /// exists for inspecting the wire-level service error payload.
    pub fn response_body(&self) -> Option<&[u8]> {
        self.inner.response_body.as_deref()
    }

    /// Returns the stack backtrace captured at error construction time, when
    /// the global rate-limited capture budget allowed it.
    ///
    /// Backtraces are captured by default for every `CosmosError` but are
    /// rate-limited via the global [`capture_limiter`] (default
    /// `1000` captures / minute). Returns `None` when the budget for the
    /// current 60-second window has been exhausted, or when backtrace
    /// capture has been disabled (budget = `0`).
    pub fn backtrace(&self) -> Option<&CosmosBacktrace> {
        self.inner.backtrace.as_ref()
    }

    // -----------------------------------------------------------------
    // Predicates
    // -----------------------------------------------------------------

    /// `true` if this is a service-side error (`Service` kind).
    pub fn is_service_error(&self) -> bool {
        matches!(self.inner.kind, CosmosErrorKind::Service)
    }

    /// `true` if the status indicates the request was throttled (HTTP 429).
    pub fn is_throttled(&self) -> bool {
        self.inner.status.is_some_and(|s| s.is_throttled())
    }

    /// `true` if the status indicates the resource was not found (HTTP 404).
    pub fn is_not_found(&self) -> bool {
        self.inner.status.is_some_and(|s| s.is_not_found())
    }

    /// `true` if the status indicates a conflict (HTTP 409).
    pub fn is_conflict(&self) -> bool {
        self.inner.status.is_some_and(|s| s.is_conflict())
    }

    /// `true` if the status indicates a precondition failure (HTTP 412).
    pub fn is_precondition_failed(&self) -> bool {
        self.inner
            .status
            .is_some_and(|s| s.is_precondition_failed())
    }

    /// `true` if the status is HTTP 408 (request timeout) for either a
    /// service-side timeout or a synthetic client-side end-to-end timeout.
    pub fn is_timeout(&self) -> bool {
        self.inner
            .status
            .is_some_and(|s| u16::from(s.status_code()) == 408)
    }

    /// `true` if the status indicates an HTTP 410 Gone response.
    pub fn is_gone(&self) -> bool {
        self.inner.status.is_some_and(|s| s.is_gone())
    }

    /// `true` if the error is generally considered transient and could
    /// reasonably be retried by a higher layer.
    pub fn is_transient(&self) -> bool {
        if matches!(self.inner.kind, CosmosErrorKind::Transport) {
            return true;
        }
        let Some(status) = self.inner.status else {
            return false;
        };
        let code = u16::from(status.status_code());
        // 408 timeout, 429 throttled, 449 retry-with, 503 service-unavailable.
        matches!(code, 408 | 429 | 449 | 503)
    }

    // -----------------------------------------------------------------
    // Interop with azure_core::Error
    // -----------------------------------------------------------------

    /// Walks the `.source()` chain of an `azure_core::Error` looking for an
    /// embedded `CosmosError` and returns a cloned copy if one is found.
    ///
    /// Used at the driver/SDK boundary to recover the typed payload from
    /// internal `azure_core::Error` values produced by the pipeline.
    pub fn try_extract(error: &azure_core::Error) -> Option<Self> {
        let mut source: Option<&(dyn StdError + 'static)> = error.source();
        while let Some(cause) = source {
            if let Some(cosmos) = cause.downcast_ref::<CosmosError>() {
                return Some(cosmos.clone());
            }
            source = cause.source();
        }
        None
    }
}

// -----------------------------------------------------------------
// Trait impls
// -----------------------------------------------------------------

impl fmt::Display for CosmosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.inner.kind, self.inner.message)?;
        if let Some(status) = self.inner.status {
            write!(f, " (status: {}", u16::from(status.status_code()))?;
            if let Some(sub) = status.sub_status() {
                write!(f, "/{}", sub.value())?;
            }
            f.write_str(")")?;
        }
        Ok(())
    }
}

impl fmt::Debug for CosmosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CosmosError")
            .field("kind", &self.inner.kind)
            .field("status", &self.inner.status)
            .field("message", &self.inner.message)
            .field("has_cosmos_headers", &self.inner.cosmos_headers.is_some())
            .field("has_response_body", &self.inner.response_body.is_some())
            .field("has_diagnostics", &self.inner.diagnostics.is_some())
            .field("has_source", &self.inner.source.is_some())
            .field("has_backtrace", &self.inner.backtrace.is_some())
            .finish()
    }
}

impl StdError for CosmosError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .source
            .as_deref()
            .map(|s| s as &(dyn StdError + 'static))
    }
}

impl From<azure_core::Error> for CosmosError {
    /// Recovers an embedded `CosmosError` from the source chain when present,
    /// or classifies the error from its `azure_core::ErrorKind` otherwise.
    fn from(error: azure_core::Error) -> Self {
        if let Some(extracted) = Self::try_extract(&error) {
            return extracted;
        }
        classify_azure_core_error(error)
    }
}

impl From<CosmosError> for azure_core::Error {
    /// Converts a typed `CosmosError` into an `azure_core::Error` for
    /// propagation through `azure_core::Result<T>` channels in the pipeline.
    ///
    /// For `Service` errors with a known status, the resulting error uses
    /// `ErrorKind::HttpResponse { status, error_code, raw_response }` where
    /// `raw_response` carries the captured body bytes (if any) so callers
    /// can match on the standard azure_core surface. The original
    /// `CosmosError` is embedded as the source so the driver/SDK boundary
    /// can recover the typed payload via
    /// [`CosmosError::try_extract`] / [`CosmosError::from`].
    fn from(cosmos: CosmosError) -> Self {
        let message = cosmos.inner.message.to_string();
        let kind = if let Some(status) = cosmos.inner.status {
            if cosmos.inner.kind == CosmosErrorKind::Service {
                let raw_response = cosmos.inner.response_body.as_ref().map(|body| {
                    Box::new(azure_core::http::RawResponse::from_bytes(
                        status.status_code(),
                        azure_core::http::headers::Headers::new(),
                        body.to_vec(),
                    ))
                });
                azure_core::error::ErrorKind::HttpResponse {
                    status: status.status_code(),
                    error_code: status.sub_status().map(|s| s.value().to_string()),
                    raw_response,
                }
            } else {
                azure_core::error::ErrorKind::Other
            }
        } else {
            azure_core::error::ErrorKind::Other
        };
        azure_core::Error::with_error(kind, cosmos, message)
    }
}

fn classify_azure_core_error(error: azure_core::Error) -> CosmosError {
    use azure_core::error::ErrorKind;

    let kind = error.kind().clone();
    let message = error.to_string();

    let cosmos_kind = match &kind {
        ErrorKind::HttpResponse { .. } => CosmosErrorKind::Service,
        ErrorKind::Credential => CosmosErrorKind::Authentication,
        ErrorKind::DataConversion => CosmosErrorKind::Serialization,
        ErrorKind::Io => CosmosErrorKind::Transport,
        _ => CosmosErrorKind::Other,
    };

    let status = match &kind {
        ErrorKind::HttpResponse { status, .. } => Some(CosmosStatus::new(*status)),
        _ => None,
    };

    CosmosError::from_inner(CosmosErrorInner {
        kind: cosmos_kind,
        status,
        cosmos_headers: None,
        response_body: None,
        diagnostics: None,
        message: Cow::Owned(message),
        source: Some(Arc::new(error)),
        backtrace: None,
    })
}

/// Driver-wide `Result` alias.
pub type Result<T> = std::result::Result<T, CosmosError>;

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind;
    use azure_core::http::headers::Headers;

    #[test]
    fn service_constructor_populates_status_and_headers() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let err = CosmosError::service(
            status,
            Some(CosmosResponseHeaders::default()),
            None,
            None,
            "throttled",
        );
        assert_eq!(err.kind(), CosmosErrorKind::Service);
        assert!(err.is_throttled());
        assert!(err.is_transient());
        assert_eq!(err.status_code(), Some(StatusCode::TooManyRequests));
        assert!(err.cosmos_headers().is_some());
    }

    #[test]
    fn end_to_end_timeout_uses_synthetic_status() {
        let err = CosmosError::end_to_end_timeout("e2e timeout", None);
        assert_eq!(err.kind(), CosmosErrorKind::Transport);
        assert_eq!(err.status_code(), Some(StatusCode::RequestTimeout));
        assert_eq!(
            err.sub_status(),
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
        assert!(err.is_timeout());
        assert!(err.is_transient());
    }

    #[test]
    fn try_extract_recovers_embedded_cosmos_error() {
        let original = CosmosError::service(
            CosmosStatus::new(StatusCode::NotFound),
            Some(CosmosResponseHeaders::default()),
            None,
            None,
            "not found",
        );
        let wrapped = azure_core::Error::new(
            ErrorKind::HttpResponse {
                status: StatusCode::NotFound,
                error_code: None,
                raw_response: None,
            },
            original.clone(),
        );
        let recovered = CosmosError::try_extract(&wrapped).expect("embedded error");
        assert_eq!(recovered.kind(), CosmosErrorKind::Service);
        assert!(recovered.is_not_found());
    }

    #[test]
    fn from_azure_core_error_classifies_when_no_embedded_payload() {
        let raw = azure_core::Error::new(
            ErrorKind::HttpResponse {
                status: StatusCode::Conflict,
                error_code: None,
                raw_response: Some(Box::new(azure_core::http::RawResponse::from_bytes(
                    StatusCode::Conflict,
                    Headers::new(),
                    Vec::new(),
                ))),
            },
            "conflict",
        );
        let cosmos: CosmosError = raw.into();
        assert_eq!(cosmos.kind(), CosmosErrorKind::Service);
        assert_eq!(cosmos.status_code(), Some(StatusCode::Conflict));
        assert!(cosmos.is_conflict());
    }

    #[test]
    fn from_azure_core_error_recovers_embedded_payload() {
        let original = CosmosError::end_to_end_timeout("e2e", None);
        let wrapped = azure_core::Error::new(ErrorKind::Other, original.clone());
        let cosmos: CosmosError = wrapped.into();
        assert_eq!(cosmos.kind(), CosmosErrorKind::Transport);
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
    }
}
