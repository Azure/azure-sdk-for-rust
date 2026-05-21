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
//! Driver-internal code produces and propagates the typed [`Error`] directly
//! via `crate::error::Result<T>` wherever possible. The boundary mapper
//! [`classify_azure_core_error`] converts at the lowest layer that interacts
//! with `azure_core` machinery (HTTP client, credential provider, response
//! deserialization) — it inspects `azure_core::ErrorKind` plus the
//! source chain (`reqwest`/`hyper`/`h2`/`io`) and mints the most specific
//! [`CosmosStatus`] available, preserving the original `azure_core::Error`
//! as [`StdError::source`] so callers can still downcast through it.
//!
//! At seams that must continue to speak `azure_core::Result<T>` (trait impls
//! forced by `azure_core` such as [`azure_core::http::HttpClient::execute_request`],
//! [`TryFrom`]/[`FromStr`] impls, and the SDK/driver public-API boundary that
//! still exposes `azure_core::Result<T>` for back-compat), the
//! [`From<Error> for azure_core::Error`] impl wraps the typed `Error` as the
//! `source` of the produced `azure_core::Error` (using
//! `ErrorKind::HttpResponse { status, .. }` for `Service` errors and
//! `ErrorKind::Other` otherwise). The driver/SDK boundary recovers the typed
//! payload via [`Error::try_extract`], so the round-trip is lossless.

use std::{borrow::Cow, error::Error as StdError, fmt, sync::Arc};

use azure_core::http::StatusCode;

use crate::{
    diagnostics::DiagnosticsContext,
    models::{
        CosmosResponse, CosmosResponseHeaders, CosmosResponsePayload, CosmosStatus, ResponseBody,
        SubStatusCode,
    },
};

mod backtrace;
pub(crate) use backtrace::{
    capture_limiter, CosmosBacktrace, BACKTRACE_RESOLUTIONS_PER_SECOND_ENV,
    DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND,
};

/// Categorical kind for an [`Error`] — re-exported from
/// [`crate::models::Kind`] (where the canonical definition lives alongside
/// [`CosmosStatus`]).
pub use crate::models::Kind;

/// Cosmos DB error returned from every public API in the driver (and, by
/// re-export, every public API in the SDK).
///
/// Unlike `azure_core::Error`, `Error` always exposes Cosmos-typed
/// status and parsed response headers when they are available — for both real
/// service errors and synthetic client-side conditions (e.g. an end-to-end
/// operation timeout surfaces as `408 / 20008` even though no HTTP response
/// was received).
///
/// `azure_core::Error` (and any other underlying error) is reachable via
/// [`std::error::Error::source`].
///
/// `Error` is `Clone` (a cheap `Arc` refcount bump) so that it can be
/// extracted from an `azure_core::Error`'s `source()` chain by reference and
/// returned by value. All fields are wrapped behind a single `Arc` so the
/// outer struct is one pointer wide, keeping `Result<T, Error>` small.
#[derive(Clone)]
pub struct Error {
    inner: Arc<ErrorInner>,
}

struct ErrorInner {
    /// Cosmos status (HTTP status + sub-status + categorical [`Kind`]).
    /// Always present \u2014 non-service constructors mint a synthetic status
    /// carrying the correct [`Kind`] and a placeholder HTTP code.
    status: CosmosStatus,
    /// Wire-level payload (body + parsed headers) of the originating
    /// response, when available. Boxed so non-service errors cost only a
    /// null pointer for this slot.
    payload: Option<Box<CosmosResponsePayload>>,
    /// Operation diagnostics for the failed operation, when available.
    diagnostics: Option<Arc<DiagnosticsContext>>,
    message: Cow<'static, str>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Captured stack backtrace, present when the global rate-limited
    /// backtrace capture budget allowed it. See [`backtrace`] module.
    backtrace: Option<CosmosBacktrace>,
}

impl Clone for ErrorInner {
    fn clone(&self) -> Self {
        Self {
            status: self.status,
            payload: self.payload.clone(),
            diagnostics: self.diagnostics.clone(),
            message: self.message.clone(),
            source: self.source.clone(),
            backtrace: self.backtrace.clone(),
        }
    }
}

impl Error {
    fn from_inner(mut inner: ErrorInner) -> Self {
        if inner.backtrace.is_none() {
            inner.backtrace = CosmosBacktrace::capture();
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
    /// The error stores the [`CosmosStatus`] and operation diagnostics
    /// directly, plus the wire-level [`CosmosResponsePayload`] (body +
    /// parsed headers) from the response so the failure can be inspected at
    /// the wire level.
    pub(crate) fn service(response: CosmosResponse, message: impl Into<Cow<'static, str>>) -> Self {
        let status = response.status();
        let diagnostics = response.diagnostics();
        let payload = response.into_payload();
        Self::from_inner(ErrorInner {
            status,
            payload: Some(Box::new(payload)),
            diagnostics: Some(diagnostics),
            message: message.into(),
            source: None,
            backtrace: None,
        })
    }

    /// Builds a `Transport` error with an explicit synthetic Cosmos status
    /// (typically `503 / 21008` for transport-generated 503, or
    /// `408 / 20008` for end-to-end operation timeout).
    pub(crate) fn transport(
        status: CosmosStatus,
        message: impl Into<Cow<'static, str>>,
        diagnostics: Option<Arc<DiagnosticsContext>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        // Force `Kind::Transport` onto the status so the categorical kind on
        // `CosmosStatus` matches the construction intent regardless of the
        // default the caller built `status` with.
        let status = status.with_kind(Kind::Transport);
        Self::from_inner(ErrorInner {
            status,
            payload: None,
            diagnostics,
            message: message.into(),
            source,
            backtrace: None,
        })
    }

    /// Convenience constructor for an end-to-end operation timeout
    /// (`408 / 20008`).
    pub(crate) fn end_to_end_timeout(
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

    /// Builds a `Client` error (caller misuse / precondition), optionally
    /// wrapping an underlying source error.
    pub fn client(
        message: impl Into<Cow<'static, str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self::from_inner(ErrorInner {
            status: CosmosStatus::new(StatusCode::BadRequest).with_kind(Kind::Client),
            payload: None,
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
        let payload = cosmos_headers
            .map(|headers| Box::new(CosmosResponsePayload::new(ResponseBody::NoPayload, headers)));
        Self::from_inner(ErrorInner {
            status: CosmosStatus::new(StatusCode::InternalServerError)
                .with_kind(Kind::Serialization),
            payload,
            diagnostics,
            message: message.into(),
            source: Some(Arc::new(source)),
            backtrace: None,
        })
    }

    /// Builds a `Configuration` error (bad endpoint URL, malformed connection
    /// string, etc.), optionally wrapping an underlying source error.
    pub fn configuration(
        message: impl Into<Cow<'static, str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self::from_inner(ErrorInner {
            status: CosmosStatus::new(StatusCode::BadRequest).with_kind(Kind::Configuration),
            payload: None,
            diagnostics: None,
            message: message.into(),
            source,
            backtrace: None,
        })
    }

    // -----------------------------------------------------------------
    // Builders
    // -----------------------------------------------------------------

    /// Returns a mutable handle to the inner state, cloning the `Arc` payload
    /// if it is shared.
    fn inner_mut(&mut self) -> &mut ErrorInner {
        Arc::make_mut(&mut self.inner)
    }

    /// Attaches parsed Cosmos response headers (replacing any existing value
    /// while preserving the body, when one is already attached).
    #[must_use]
    pub(crate) fn with_cosmos_headers(mut self, headers: CosmosResponseHeaders) -> Self {
        let inner = self.inner_mut();
        let body = inner
            .payload
            .as_deref()
            .map(|p| p.body().clone())
            .unwrap_or(ResponseBody::NoPayload);
        inner.payload = Some(Box::new(CosmosResponsePayload::new(body, headers)));
        self
    }

    /// Attaches diagnostics (replacing any existing value).
    #[must_use]
    #[allow(dead_code)]
    pub(crate) fn with_diagnostics(mut self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        self.inner_mut().diagnostics = Some(diagnostics);
        self
    }

    /// Attaches a source error (replacing any existing value).
    #[must_use]
    pub(crate) fn with_source(mut self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self {
        self.inner_mut().source = Some(source);
        self
    }

    /// Prepends operational context to the error message, preserving all
    /// other typed fields (status, sub-status, headers, diagnostics, source,
    /// backtrace).
    ///
    /// Use this at sites that have request-specific context the boundary
    /// mapper cannot see (operation name, container/database, endpoint,
    /// partition-key range, activity id) to enrich an otherwise generic
    /// mapper-classified error before propagating it further.
    ///
    /// The resulting message has the shape `"{context}: {original}"`.
    #[must_use]
    pub fn with_context(mut self, context: impl Into<Cow<'static, str>>) -> Self {
        let inner = self.inner_mut();
        let context: Cow<'static, str> = context.into();
        let combined = format!("{context}: {}", inner.message);
        inner.message = Cow::Owned(combined);
        self
    }

    // -----------------------------------------------------------------
    // Accessors
    // -----------------------------------------------------------------

    /// Returns the categorical kind of this error — read from
    /// [`CosmosStatus::kind`].
    pub fn kind(&self) -> Kind {
        self.inner.status.kind()
    }

    /// Returns the typed Cosmos status (HTTP status code + optional sub-status
    /// + categorical [`Kind`]) associated with this error. Always present —
    /// non-service errors carry a synthetic status with a placeholder HTTP
    /// code and the correct [`Kind`].
    pub fn status(&self) -> CosmosStatus {
        self.inner.status
    }

    /// Returns the HTTP status code. For non-service errors this is a
    /// placeholder code corresponding to the error's [`Kind`].
    pub fn status_code(&self) -> StatusCode {
        self.inner.status.status_code()
    }

    /// Returns the sub-status code, if present.
    pub fn sub_status(&self) -> Option<SubStatusCode> {
        self.inner.status.sub_status()
    }

    /// Returns the parsed Cosmos response headers (when a service response was
    /// received).
    pub fn cosmos_headers(&self) -> Option<&CosmosResponseHeaders> {
        self.inner
            .payload
            .as_deref()
            .map(CosmosResponsePayload::headers)
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
        match self.inner.payload.as_deref()?.body() {
            ResponseBody::Bytes(b) => Some(b.as_ref()),
            ResponseBody::NoPayload | ResponseBody::Items(_) => None,
        }
    }

    /// Returns the stack backtrace captured at error construction time,
    /// rendered as a human-readable string.
    ///
    /// Capture itself is unconditional (cheap: just walking the stack). The
    /// expensive part — resolving instruction pointers to symbol names — is
    /// rate-limited via a process-global limiter (default `5` resolutions /
    /// second). Cache hits do **not** consume budget, so backtraces whose
    /// frames are already known render at full fidelity regardless of
    /// limiter state.
    ///
    /// Returns `None` only when the limiter denies fresh resolution for at
    /// least one cache-missed frame. Partial backtraces are never produced —
    /// callers either get a fully-resolved render or nothing. `None` results
    /// are not cached: a later call may succeed once the limiter window
    /// reopens (and frames resolved by other errors meanwhile have been
    /// added to the cache).
    pub fn backtrace(&self) -> Option<&str> {
        self.inner
            .backtrace
            .as_ref()
            .and_then(CosmosBacktrace::rendered)
    }

    // -----------------------------------------------------------------
    // Predicates
    // -----------------------------------------------------------------

    /// `true` if this is a service-side error (`Service` kind).
    pub fn is_service_error(&self) -> bool {
        matches!(self.kind(), Kind::Service)
    }

    /// `true` if the status indicates the request was throttled (HTTP 429).
    pub fn is_throttled(&self) -> bool {
        self.inner.status.is_throttled()
    }

    /// `true` if the status indicates the resource was not found (HTTP 404).
    pub fn is_not_found(&self) -> bool {
        self.inner.status.is_not_found()
    }

    /// `true` if the status indicates a conflict (HTTP 409).
    pub fn is_conflict(&self) -> bool {
        self.inner.status.is_conflict()
    }

    /// `true` if the status indicates a precondition failure (HTTP 412).
    pub fn is_precondition_failed(&self) -> bool {
        self.inner.status.is_precondition_failed()
    }

    /// `true` if the status is HTTP 408 (request timeout) for either a
    /// service-side timeout or a synthetic client-side end-to-end timeout.
    pub fn is_timeout(&self) -> bool {
        u16::from(self.inner.status.status_code()) == 408
    }

    /// `true` if the status indicates an HTTP 410 Gone response.
    pub fn is_gone(&self) -> bool {
        self.inner.status.is_gone()
    }

    /// `true` if the error is generally considered transient and could
    /// reasonably be retried by a higher layer.
    pub fn is_transient(&self) -> bool {
        if matches!(self.kind(), Kind::Transport) {
            return true;
        }
        let code = u16::from(self.inner.status.status_code());
        // 408 timeout, 429 throttled, 449 retry-with, 503 service-unavailable.
        matches!(code, 408 | 429 | 449 | 503)
    }

    // -----------------------------------------------------------------
    // Interop with azure_core::Error
    // -----------------------------------------------------------------

    /// Walks the `.source()` chain of an `azure_core::Error` looking for an
    /// embedded `Error` and returns a cloned copy if one is found.
    ///
    /// Used at the driver/SDK boundary to recover the typed payload from
    /// internal `azure_core::Error` values produced by the pipeline.
    pub(crate) fn try_extract(error: &azure_core::Error) -> Option<Self> {
        let mut source: Option<&(dyn StdError + 'static)> = error.source();
        while let Some(cause) = source {
            if let Some(cosmos) = cause.downcast_ref::<Error>() {
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = self.inner.status;
        write!(
            f,
            "[{}] {} (status: {}",
            status.kind(),
            self.inner.message,
            u16::from(status.status_code())
        )?;
        if let Some(sub) = status.sub_status() {
            write!(f, "/{}", sub.value())?;
        }
        f.write_str(")")
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("status", &self.inner.status)
            .field("message", &self.inner.message)
            .field("has_payload", &self.inner.payload.is_some())
            .field("has_diagnostics", &self.inner.diagnostics.is_some())
            .field("has_source", &self.inner.source.is_some())
            .field("has_backtrace", &self.inner.backtrace.is_some())
            .finish()
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .source
            .as_deref()
            .map(|s| s as &(dyn StdError + 'static))
    }
}

impl From<azure_core::Error> for Error {
    /// Recovers an embedded `Error` from the source chain when present,
    /// or classifies the error from its `azure_core::ErrorKind` otherwise.
    fn from(error: azure_core::Error) -> Self {
        if let Some(extracted) = Self::try_extract(&error) {
            return extracted;
        }
        classify_azure_core_error(error)
    }
}

impl From<Error> for azure_core::Error {
    /// Converts a typed `Error` into an `azure_core::Error` for
    /// propagation through `azure_core::Result<T>` channels in the pipeline.
    ///
    /// For `Service` errors with a known status, the resulting error uses
    /// `Kind::HttpResponse { status, error_code, raw_response }` where
    /// `raw_response` carries the captured body bytes (if any) so callers
    /// can match on the standard azure_core surface. The original
    /// `Error` is embedded as the source so the driver/SDK boundary
    /// can recover the typed payload via
    /// [`Error::try_extract`] / [`Error::from`].
    fn from(cosmos: Error) -> Self {
        let message = cosmos.inner.message.to_string();
        let status = cosmos.inner.status;
        let kind = if status.kind() == Kind::Service {
            let raw_response = cosmos
                .inner
                .payload
                .as_deref()
                .and_then(|p| match p.body() {
                    ResponseBody::Bytes(b) => Some(b.to_vec()),
                    ResponseBody::NoPayload | ResponseBody::Items(_) => None,
                })
                .map(|body| {
                    Box::new(azure_core::http::RawResponse::from_bytes(
                        status.status_code(),
                        azure_core::http::headers::Headers::new(),
                        body,
                    ))
                });
            azure_core::error::ErrorKind::HttpResponse {
                status: status.status_code(),
                error_code: status.sub_status().map(|s| s.value().to_string()),
                raw_response,
            }
        } else {
            azure_core::error::ErrorKind::Other
        };
        azure_core::Error::with_error(kind, cosmos, message)
    }
}

/// Boundary mapper: converts an `azure_core::Error` (typically produced by
/// the HTTP pipeline, credential provider, or response deserialization) into
/// a typed [`Error`] carrying the most specific [`CosmosStatus`] the source
/// chain allows.
///
/// The original `azure_core::Error` is always preserved as the
/// [`StdError::source`] of the returned Cosmos error so callers can still
/// downcast through the underlying `reqwest`/`hyper`/`h2`/`io` chain when
/// needed; the typed status is the preferred discriminator.
fn classify_azure_core_error(error: azure_core::Error) -> Error {
    let message = error.to_string();
    let status = derive_status_from_azure_core_error(&error);
    Error::from_inner(ErrorInner {
        status,
        payload: None,
        diagnostics: None,
        message: Cow::Owned(message),
        source: Some(Arc::new(error)),
        backtrace: None,
    })
}

fn derive_status_from_azure_core_error(error: &azure_core::Error) -> CosmosStatus {
    use azure_core::error::ErrorKind as AzKind;

    // HttpResponse is the only kind that already carries a real wire status,
    // so it wins over any source-chain refinement.
    if let AzKind::HttpResponse { status, .. } = error.kind() {
        return CosmosStatus::new(*status).with_kind(Kind::Service);
    }

    // Otherwise inspect the source chain for a more specific cause than
    // azure_core's coarse `ErrorKind` exposes (h2 protocol errors, io DNS
    // errors, etc.).
    if let Some(refined) = refine_status_from_source_chain(error.source()) {
        return refined;
    }

    match error.kind() {
        AzKind::Credential => CosmosStatus::AUTHENTICATION_TOKEN_ACQUISITION_FAILED,
        AzKind::DataConversion => CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
        AzKind::Connection => CosmosStatus::TRANSPORT_CONNECTION_FAILED,
        AzKind::Io => CosmosStatus::TRANSPORT_IO_FAILED,
        // Unknown `azure_core` kinds at this boundary are most likely
        // transport-layer surprises; treat as transient transport failures.
        // `azure_core::ErrorKind` is `#[non_exhaustive]`, so any future
        // variant lands here too.
        _ => CosmosStatus::TRANSPORT_IO_FAILED,
    }
}

/// Walks the `.source()` chain looking for downcasts that map to a more
/// specific [`CosmosStatus`] than the top-level `azure_core::ErrorKind`
/// provides. Returns `None` if nothing more specific is found.
fn refine_status_from_source_chain(
    start: Option<&(dyn StdError + 'static)>,
) -> Option<CosmosStatus> {
    let mut cur = start;
    while let Some(e) = cur {
        #[cfg(feature = "reqwest")]
        {
            if let Some(h2_err) = e.downcast_ref::<h2::Error>() {
                if matches!(
                    h2_err.reason(),
                    Some(
                        h2::Reason::HTTP_1_1_REQUIRED
                            | h2::Reason::PROTOCOL_ERROR
                            | h2::Reason::FRAME_SIZE_ERROR
                    )
                ) {
                    return Some(CosmosStatus::TRANSPORT_HTTP2_INCOMPATIBLE);
                }
            }
        }
        if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
            // Best-effort DNS detection. `reqwest`/`hyper` typically surface
            // resolver failures as `io::ErrorKind::NotFound` /
            // `AddrNotAvailable`. TLS / generic socket I/O falls through to
            // the caller's base classification.
            if matches!(
                io_err.kind(),
                std::io::ErrorKind::NotFound | std::io::ErrorKind::AddrNotAvailable
            ) {
                return Some(CosmosStatus::TRANSPORT_DNS_FAILED);
            }
        }
        cur = e.source();
    }
    None
}

/// Driver-wide `Result` alias.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind as AzKind;
    use azure_core::http::headers::Headers;

    #[test]
    fn service_constructor_populates_status_and_headers() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let response = CosmosResponse::new(
            ResponseBody::NoPayload,
            CosmosResponseHeaders::default(),
            status,
            DiagnosticsContext::error_placeholder(),
        );
        let err = Error::service(response, "throttled");
        assert_eq!(err.kind(), Kind::Service);
        assert!(err.is_throttled());
        assert!(err.is_transient());
        assert_eq!(err.status_code(), StatusCode::TooManyRequests);
        assert!(err.cosmos_headers().is_some());
    }

    #[test]
    fn end_to_end_timeout_uses_synthetic_status() {
        let err = Error::end_to_end_timeout("e2e timeout", None);
        assert_eq!(err.kind(), Kind::Transport);
        assert_eq!(err.status_code(), StatusCode::RequestTimeout);
        assert_eq!(
            err.sub_status(),
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
        assert!(err.is_timeout());
        assert!(err.is_transient());
    }

    #[test]
    fn try_extract_recovers_embedded_cosmos_error() {
        let response = CosmosResponse::new(
            ResponseBody::NoPayload,
            CosmosResponseHeaders::default(),
            CosmosStatus::new(StatusCode::NotFound),
            DiagnosticsContext::error_placeholder(),
        );
        let original = Error::service(response, "not found");
        let wrapped = azure_core::Error::new(
            AzKind::HttpResponse {
                status: StatusCode::NotFound,
                error_code: None,
                raw_response: None,
            },
            original.clone(),
        );
        let recovered = Error::try_extract(&wrapped).expect("embedded error");
        assert_eq!(recovered.kind(), Kind::Service);
        assert!(recovered.is_not_found());
    }

    #[test]
    fn from_azure_core_error_classifies_when_no_embedded_payload() {
        let raw = azure_core::Error::new(
            AzKind::HttpResponse {
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
        let cosmos: Error = raw.into();
        assert_eq!(cosmos.kind(), Kind::Service);
        assert_eq!(cosmos.status_code(), StatusCode::Conflict);
        assert!(cosmos.is_conflict());
    }

    #[test]
    fn from_azure_core_error_recovers_embedded_payload() {
        let original = Error::end_to_end_timeout("e2e", None);
        let wrapped = azure_core::Error::new(AzKind::Other, original.clone());
        let cosmos: Error = wrapped.into();
        assert_eq!(cosmos.kind(), Kind::Transport);
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
    }

    #[test]
    fn classify_preserves_azure_core_error_as_source() {
        // No embedded Cosmos payload — must classify and keep the original
        // `azure_core::Error` in the source chain so callers can downcast
        // through it for transport-level checks (e.g. reqwest connection
        // errors).
        let original = azure_core::Error::with_message(AzKind::Io, "connection reset");
        let cosmos: Error = original.into();
        assert_eq!(cosmos.kind(), Kind::Transport);

        let source = StdError::source(&cosmos).expect("source preserved");
        let recovered = source
            .downcast_ref::<azure_core::Error>()
            .expect("downcast back to azure_core::Error");
        assert!(matches!(recovered.kind(), AzKind::Io));
        assert!(recovered.to_string().contains("connection reset"));
    }

    #[test]
    fn classify_io_kind_maps_to_transport_io_failed() {
        let raw = azure_core::Error::with_message(AzKind::Io, "io");
        let cosmos: Error = raw.into();
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::TRANSPORT_IO_FAILED)
        );
    }

    #[test]
    fn classify_connection_kind_maps_to_transport_connection_failed() {
        let raw = azure_core::Error::with_message(AzKind::Connection, "refused");
        let cosmos: Error = raw.into();
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::TRANSPORT_CONNECTION_FAILED)
        );
    }

    #[test]
    fn classify_credential_kind_maps_to_token_acquisition_failed() {
        let raw = azure_core::Error::with_message(AzKind::Credential, "no token");
        let cosmos: Error = raw.into();
        assert_eq!(cosmos.kind(), Kind::Authentication);
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::AUTHENTICATION_TOKEN_ACQUISITION_FAILED)
        );
    }

    #[test]
    fn classify_data_conversion_kind_maps_to_response_body_invalid() {
        let raw = azure_core::Error::with_message(AzKind::DataConversion, "bad json");
        let cosmos: Error = raw.into();
        assert_eq!(cosmos.kind(), Kind::Serialization);
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::SERIALIZATION_RESPONSE_BODY_INVALID)
        );
    }

    #[test]
    fn classify_refines_io_dns_via_source_chain() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "dns lookup failed");
        let raw = azure_core::Error::new(AzKind::Io, io_err);
        let cosmos: Error = raw.into();
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::TRANSPORT_DNS_FAILED)
        );
    }

    #[cfg(feature = "reqwest")]
    #[test]
    fn classify_refines_h2_protocol_via_source_chain() {
        let h2_err: h2::Error = h2::Reason::HTTP_1_1_REQUIRED.into();
        let raw = azure_core::Error::new(AzKind::Io, h2_err);
        let cosmos: Error = raw.into();
        assert_eq!(
            cosmos.sub_status(),
            Some(SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE)
        );
    }
}
