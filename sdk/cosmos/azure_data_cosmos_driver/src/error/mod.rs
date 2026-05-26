// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB-specific error type carrying typed status, parsed Cosmos response
//! headers, and diagnostics — for both service errors (real HTTP responses)
//! and synthetic client-side conditions (e.g. end-to-end operation timeouts).
//!
//! Mirrors the .NET / Java SDKs' `CosmosException`: a single error type that
//! surfaces typed Cosmos status (status code + sub-status, including synthetic
//! codes such as `408 / 20008` for end-to-end timeout), the parsed
//! [`CosmosResponseHeaders`], and the operation [`DiagnosticsContext`].
//!
//! ## Boundary with `azure_core`
//!
//! Driver-internal code produces and propagates [`Error`] directly via
//! [`crate::error::Result<T>`]. At the lowest layer that interacts with
//! `azure_core` machinery (HTTP client, credential provider, response
//! deserialization), [`classify_azure_core_error`] inspects the
//! `azure_core::ErrorKind` plus the source chain
//! (`reqwest`/`hyper`/`h2`/`io`) and mints the most specific [`CosmosStatus`]
//! available, preserving the original `azure_core::Error` as
//! [`StdError::source`] so callers can still downcast through it.
//!
//! The conversion is one-way: nothing in the driver wraps a Cosmos
//! [`Error`] back inside an `azure_core::Error`. The transport layer
//! carries typed Cosmos errors end-to-end (see
//! [`TransportError`](crate::driver::transport::TransportError)).

use std::{error::Error as StdError, fmt, sync::Arc};

use azure_core::http::StatusCode;

use crate::{
    diagnostics::DiagnosticsContext,
    models::{
        CosmosResponse, CosmosResponseHeaders, CosmosResponsePayload, CosmosStatus, ResponseBody,
        SubStatusCode,
    },
};

pub(crate) mod backtrace;
pub(crate) use backtrace::Backtrace;

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

#[derive(Clone)]
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
    message: Arc<str>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Captured stack backtrace, present when the global rate-limited
    /// backtrace capture budget allowed it. See [`backtrace`] module.
    backtrace: Option<Backtrace>,
}

impl Error {
    fn from_inner(mut inner: ErrorInner) -> Self {
        if inner.backtrace.is_none() {
            // If we are wrapping another Cosmos `Error` as the source
            // (status-changing re-wrap, e.g. `build_transport_error`
            // promoting a service error to a transport error), inherit
            // that error's backtrace instead of paying for a fresh
            // capture at the wrap site. The wrap site is always the same
            // handful of lines in the pipeline and adds no diagnostic
            // value over the originating call stack \u2014 inheriting also
            // saves one capture-throttle token per re-wrap, doubling the
            // effective capture budget on retry-heavy paths.
            if let Some(src) = inner.source.as_deref() {
                let src_dyn: &(dyn StdError + 'static) = src;
                if let Some(inner_cosmos) = src_dyn.downcast_ref::<Error>() {
                    inner.backtrace = inner_cosmos.inner.backtrace.clone();
                }
            }
            if inner.backtrace.is_none() {
                inner.backtrace = Backtrace::capture();
            }
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
    pub(crate) fn service(response: CosmosResponse, message: impl Into<Arc<str>>) -> Self {
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
        message: impl Into<Arc<str>>,
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
        message: impl Into<Arc<str>>,
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
    ///
    /// **Internal use only.** Reachable cross-crate so the SDK wrapper
    /// (`azure_data_cosmos`) and other in-tree consumers can construct
    /// typed errors; not part of the public surface.
    #[doc(hidden)]
    pub fn client(
        message: impl Into<Arc<str>>,
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
    ///
    /// **Internal use only.** Reachable cross-crate so the SDK wrapper
    /// (`azure_data_cosmos`) and other in-tree consumers can construct
    /// typed errors; not part of the public surface.
    #[doc(hidden)]
    pub fn serialization(
        message: impl Into<Arc<str>>,
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
    ///
    /// **Internal use only.** Reachable cross-crate so the SDK wrapper
    /// (`azure_data_cosmos`) and other in-tree consumers can construct
    /// typed errors; not part of the public surface.
    #[doc(hidden)]
    pub fn configuration(
        message: impl Into<Arc<str>>,
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
    ///
    /// **Internal use only.** Reachable cross-crate so the SDK wrapper
    /// (`azure_data_cosmos`) and other in-tree consumers can enrich
    /// errors with request context; not part of the public surface.
    #[doc(hidden)]
    #[must_use]
    pub fn with_context(mut self, context: impl Into<Arc<str>>) -> Self {
        let inner = self.inner_mut();
        let context: Arc<str> = context.into();
        // Single-allocation concatenation: pre-size a String to the exact
        // final length so `format!`-style growth doublings are avoided, then
        // hand it off to `Arc::<str>::from` for the final shared buffer.
        let mut buf = String::with_capacity(context.len() + 2 + inner.message.len());
        buf.push_str(&context);
        buf.push_str(": ");
        buf.push_str(&inner.message);
        inner.message = Arc::<str>::from(buf);
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
    /// Capture is bounded by two production-safety gates (resolution-rate
    /// limiter + per-second capture throttle, both rolling 1-second
    /// windows). Cache hits do **not** consume budget, so backtraces whose
    /// frames are already known render at full fidelity regardless of
    /// limiter state.
    ///
    /// Returns `None` when:
    /// * The capture throttle was exhausted at construction time, or
    /// * the resolution limiter denied fresh resolution for at least one
    ///   cache-missed frame, or
    /// * the auto-disable flag was set by a recent resolution denial and
    ///   the window has not yet reopened.
    ///
    /// Partial backtraces are never produced — callers either get a fully-
    /// resolved render or nothing. **The outcome of the first call is
    /// cached on this [`Error`] instance**, so every subsequent call
    /// returns the same answer regardless of later changes in limiter or
    /// throttle state. Callers may call this multiple times (logging,
    /// telemetry, panic message) without risk of inconsistent results.
    ///
    /// ## What the backtrace points at
    ///
    /// * **Errors originating inside the Cosmos pipeline** (HTTP error
    ///   responses, end-to-end timeouts, internal validation failures)
    ///   resolve to the actual construction site.
    /// * **Errors wrapping another Cosmos [`Error`]** as their source
    ///   (status-changing re-wraps such as `build_transport_error`
    ///   promoting a service error to a transport error) **inherit** the
    ///   inner error's backtrace, so the originating site is still
    ///   visible.
    /// * **Errors produced by the `From<azure_core::Error>` boundary
    ///   mapper** (transport / credential / serialization failures
    ///   arriving from `azure_core` without an embedded Cosmos error)
    ///   point at the boundary mapper itself, not at the original failure
    ///   site. `azure_core::Error` does not carry its own backtrace, so
    ///   the originating call stack is unrecoverable at this layer. The
    ///   typed [`Kind`], status, and `std::error::Error::source()` chain
    ///   (which preserves the underlying `azure_core::Error`,
    ///   `reqwest::Error`, `h2::Error`, `io::Error`, …) remain the
    ///   primary diagnostic signal in that case.
    pub fn backtrace(&self) -> Option<&str> {
        self.inner.backtrace.as_ref().and_then(Backtrace::rendered)
    }
}

// -----------------------------------------------------------------
// Trait impls
// -----------------------------------------------------------------

impl fmt::Display for Error {
    /// Default (`{e}`): the bare error message text — matching the
    /// `anyhow::Error` / `azure_core::Error` / `std::io::Error` convention
    /// that `e.to_string()` returns the human-readable message. Typed
    /// metadata (kind, status, sub-status, headers, diagnostics, source,
    /// backtrace) is reachable via the dedicated accessors on [`Error`].
    ///
    /// Alternate (`{e:#}`): the message prefixed with the categorical
    /// [`Kind`] and the typed status, followed by the source chain and
    /// (if captured) the rendered backtrace. Matches the `anyhow::Error` /
    /// `eyre::Report` convention of opting in to a richer multi-line
    /// representation via the alternate flag.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write_header(f, &self.inner)?;
            write_source_chain(f, self)?;
            write_backtrace(f, self)?;
        } else {
            f.write_str(&self.inner.message)?;
        }
        Ok(())
    }
}

impl fmt::Debug for Error {
    /// Default (`{e:?}`): structured header (kind + message + status) plus
    /// the source chain. The captured backtrace is **omitted** so that
    /// high-volume `tracing::error!(err = ?e)` / `Result::unwrap` /
    /// `assert_eq!` call sites do not emit multi-line stack frame blocks
    /// per error.
    ///
    /// Alternate (`{e:#?}`): same as default plus the rendered backtrace
    /// block \u2014 opt in for full diagnostic reports. Matches the
    /// `anyhow::Error` / `eyre::Report` convention of opting in to a
    /// richer multi-line representation via the alternate flag.
    ///
    /// Callers that always want the backtrace regardless of format flag
    /// should read it explicitly via [`Error::backtrace`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header(f, &self.inner)?;
        write_source_chain(f, self)?;
        if f.alternate() {
            write_backtrace(f, self)?;
        }
        Ok(())
    }
}

fn write_header(f: &mut fmt::Formatter<'_>, inner: &ErrorInner) -> fmt::Result {
    // `CosmosStatus::Display` already renders the categorical `[Kind]`
    // plus `<status>/<sub> (<name>)` (or `<status>` when no sub-status),
    // so reuse it for a single, consistent representation.
    write!(f, "{}: {}", inner.status, inner.message)
}

fn write_source_chain(f: &mut fmt::Formatter<'_>, err: &Error) -> fmt::Result {
    let mut cur: Option<&(dyn StdError + 'static)> = StdError::source(err);
    let mut depth = 0;
    while let Some(src) = cur {
        if depth == 0 {
            f.write_str("\n\nCaused by:")?;
        }
        write!(f, "\n  {depth}: {src}")?;
        cur = src.source();
        depth += 1;
    }
    Ok(())
}

fn write_backtrace(f: &mut fmt::Formatter<'_>, err: &Error) -> fmt::Result {
    if let Some(bt) = err.backtrace() {
        f.write_str("\n\nStack backtrace:\n")?;
        f.write_str(bt)?;
    }
    Ok(())
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
    /// Boundary mapper from `azure_core::Error`. The driver no longer
    /// embeds typed Cosmos errors inside `azure_core::Error` containers,
    /// so this is a one-way classification — no embedded-payload
    /// recovery is needed.
    fn from(error: azure_core::Error) -> Self {
        classify_azure_core_error(error)
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
        message: Arc::<str>::from(message),
        source: Some(Arc::new(error)),
        backtrace: None,
    })
}

fn derive_status_from_azure_core_error(error: &azure_core::Error) -> CosmosStatus {
    use azure_core::error::ErrorKind as AzKind;

    // HttpResponse is the only kind that already carries a real wire status,
    // so it wins over any source-chain refinement.
    if let AzKind::HttpResponse {
        status, error_code, ..
    } = error.kind()
    {
        let mut cs = CosmosStatus::new(*status).with_kind(Kind::Service);
        if let Some(sub) = error_code.as_deref().and_then(|c| c.parse::<u32>().ok()) {
            cs = cs.with_sub_status(sub);
        }
        return cs;
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
        assert!(err.status().is_throttled());
        assert!(err.status().is_transient());
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
        assert!(err.status().is_timeout());
        assert!(err.status().is_transient());
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
        assert!(cosmos.status().is_conflict());
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

    #[test]
    fn wrap_inherits_backtrace_from_cosmos_source() {
        // Build an inner Cosmos error so it carries a captured backtrace.
        let inner = Error::end_to_end_timeout("inner", None);
        let inner_bt_id = inner
            .inner
            .backtrace
            .as_ref()
            .map(|bt| bt.inner_arc_identity_for_tests());
        assert!(
            inner_bt_id.is_some(),
            "inner must have a captured backtrace for this test to be meaningful"
        );

        // Wrap the inner error as the source of an outer transport error.
        // The outer constructor must inherit the inner's backtrace rather
        // than capturing a fresh one at the wrap site.
        let outer = Error::transport(
            CosmosStatus::TRANSPORT_GENERATED_503,
            "outer",
            None,
            Some(Arc::new(inner)),
        );
        let outer_bt_id = outer
            .inner
            .backtrace
            .as_ref()
            .map(|bt| bt.inner_arc_identity_for_tests());
        assert_eq!(
            outer_bt_id, inner_bt_id,
            "outer error must share the inner's backtrace Arc, not capture a new one"
        );
    }
}
