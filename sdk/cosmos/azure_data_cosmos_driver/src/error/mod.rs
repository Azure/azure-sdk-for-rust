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
//! deserialization), `classify_azure_core_error` inspects the
//! `azure_core::ErrorKind` plus the source chain
//! (`reqwest`/`hyper`/`h2`/`io`) and mints the most specific [`CosmosStatus`]
//! available, preserving the original `azure_core::Error` as
//! [`StdError::source`] so callers can still downcast through it.
//!
//! The conversion is one-way: nothing in the driver wraps a Cosmos
//! [`Error`] back inside an `azure_core::Error`. The transport layer
//! carries typed Cosmos errors end-to-end.

use std::{error::Error as StdError, fmt, sync::Arc};

use azure_core::http::StatusCode;

use crate::{
    diagnostics::DiagnosticsContext,
    models::{
        CosmosResponseHeaders, CosmosResponsePayload, CosmosStatus, ResponseBody, SubStatusCode,
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

    /// Builds a `Service` error from raw wire parts (status, headers, body,
    /// message) **without** any [`DiagnosticsContext`].
    ///
    /// Intended for retry/evaluation layers that classify HTTP error
    /// responses but do not own the operation-level
    /// [`DiagnosticsContextBuilder`](crate::diagnostics::DiagnosticsContextBuilder).
    /// The caller (typically the operation pipeline's abort branch) is
    /// responsible for grafting the completed diagnostics onto the returned
    /// error via [`Error::with_diagnostics`] before it crosses the SDK
    /// boundary. Decoupling this constructor from diagnostics keeps the
    /// retry-evaluation module free of any throw-away placeholder context
    /// that would immediately be overwritten downstream.
    pub(crate) fn service_from_parts(
        status: CosmosStatus,
        headers: CosmosResponseHeaders,
        body: &[u8],
        message: impl Into<Arc<str>>,
    ) -> Self {
        let payload = CosmosResponsePayload::new(
            ResponseBody::from_bytes(bytes::Bytes::copy_from_slice(body)),
            headers,
        );
        Self::from_inner(ErrorInner {
            status,
            payload: Some(Box::new(payload)),
            diagnostics: None,
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

    /// Returns a copy of `self` with `diagnostics` attached (or replaced).
    ///
    /// Used by the operation pipeline's abort branch to graft the completed
    /// operation [`DiagnosticsContext`] (retry history, region attempts,
    /// per-request events) onto an error that was built deep in the
    /// pipeline before that context was available. Without this, the
    /// operation diagnostics would be silently dropped on every aborted
    /// operation \u2014 callers reading [`Error::diagnostics`] would see `None`
    /// even though the operation pipeline was still tracking everything.
    ///
    /// Cheap: clones the inner [`Arc`]'s contents (one allocation) and
    /// patches the diagnostics slot. The original [`Error`] is unchanged
    /// and shareable. Inherited backtrace is preserved as-is so a `?`
    /// propagating through this helper does not re-capture.
    pub(crate) fn with_diagnostics(&self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        let mut next = (*self.inner).clone();
        next.diagnostics = Some(diagnostics);
        Self {
            inner: Arc::new(next),
        }
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
    /// `cosmos_headers` and `diagnostics` are best-effort: populate them
    /// when the failure occurs at a call site that already has access to
    /// the originating operation's headers and diagnostics context (e.g.
    /// custom response-body deserialization inside the driver pipeline),
    /// so the resulting error carries the request charge, activity id,
    /// and timeline needed to diagnose the failure.
    ///
    /// In practice the most common construction path is the SDK
    /// wrapper's blanket `impl From<serde_json::Error> for Error`, which
    /// is invoked by `?` at the SDK boundary and passes `None, None` —
    /// at that boundary the originating operation context is not
    /// reachable. Tolerating `None` here is therefore the rule, not the
    /// exception; the call sites that *can* enrich the error should
    /// pass it through, the rest should pass `None`.
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
    ///
    /// ## Async caveat
    ///
    /// Stack capture records the **synchronous call stack at the
    /// construction site**, which in an `async` context is the current
    /// poll frame — typically `tokio runtime → poll → your_async_fn`,
    /// not the chain of `.await` ancestors that logically led there. For
    /// errors constructed inside this driver's async pipeline that means
    /// the captured frames will frequently look like driver-internal
    /// poll machinery (retry loop, transport pipeline, tokio task
    /// scheduler) rather than the calling code that issued the
    /// operation. This is a fundamental limitation of stack capture in
    /// async Rust, not specific to this crate. For the logical async
    /// call chain, use `tracing` spans wrapping the calling code — the
    /// span context is preserved across `.await` points and shows up in
    /// structured logs alongside the captured backtrace.
    pub fn backtrace(&self) -> Option<&Arc<str>> {
        self.inner.backtrace.as_ref().and_then(Backtrace::rendered)
    }
}

// -----------------------------------------------------------------
// Trait impls
// -----------------------------------------------------------------

impl fmt::Display for Error {
    /// Default (`{e}`): a single-line `[Kind] status/sub (name): message`
    /// header. This intentionally diverges from the `anyhow` / `azure_core`
    /// / `io::Error` "bare message" convention so that every existing log
    /// site (`tracing::error!("{e}")`, `format!("op failed: {e}")`, panic
    /// messages) automatically surfaces the typed Cosmos status that this
    /// error type exists to expose — losing it silently in default rendering
    /// would defeat the purpose of the typed surface. The format is bounded
    /// in length (a few dozen bytes) and stays on a single line.
    ///
    /// Alternate (`{e:#}`): the single-line header followed by the
    /// `Caused by:` source chain, the structured diagnostics block, and
    /// (if captured) the rendered backtrace. Matches the `anyhow::Error` /
    /// `eyre::Report` convention of opting in to a richer multi-line
    /// representation via the alternate flag.
    ///
    /// Structured fields (kind, status, sub-status, headers, diagnostics,
    /// source chain, backtrace) are also reachable directly via the
    /// dedicated accessors on [`Error`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header(f, &self.inner)?;
        if f.alternate() {
            // Display form uses `{src}` / `{src:#}` per entry so the
            // chain remains human-readable; Debug uses `{src:?}` /
            // `{src:#?}` to expose structured state.
            write_source_chain(f, self, /* debug */ false, /* alternate */ true)?;
            write_diagnostics(
                f,
                &self.inner,
                /* debug */ false,
                /* alternate */ true,
            )?;
            write_backtrace(f, self)?;
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
        let alternate = f.alternate();
        write_header(f, &self.inner)?;
        write_source_chain(f, self, /* debug */ true, alternate)?;
        write_diagnostics(f, &self.inner, /* debug */ true, alternate)?;
        if alternate {
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

/// Writes the `source()` chain. When `debug` is true, each entry is
/// rendered with `{:?}` so that wrapped errors carrying structured state
/// (e.g. another Cosmos [`Error`], an `azure_core::Error`, `io::Error`,
/// `h2::Error`) surface their full debug representation rather than a
/// one-line `Display` summary. Display mode (`alternate Display` on
/// [`Error`]) keeps the human-readable single-line form per entry.
///
/// `alternate` is propagated so that `{e:#?}` cascades to `{src:#?}` on
/// each entry (and `{e:#}` to `{src:#}`), giving callers a way to opt
/// into the richer multi-line representation of wrapped errors.
fn write_source_chain(
    f: &mut fmt::Formatter<'_>,
    err: &Error,
    debug: bool,
    alternate: bool,
) -> fmt::Result {
    let mut cur: Option<&(dyn StdError + 'static)> = StdError::source(err);
    let mut depth = 0;
    while let Some(src) = cur {
        if depth == 0 {
            f.write_str("\n\nCaused by:")?;
        }
        // Bound the walk by the same cap as `refine_status_from_source_chain`
        // so a pathological or cyclic `source()` chain cannot pin a thread
        // formatting an error. This runs on every `tracing::error!`,
        // `format!`, and panic message, so the protection matters even more
        // here than at the boundary mapper.
        if depth >= MAX_SOURCE_CHAIN_DEPTH {
            write!(
                f,
                "\n  {depth}: ... <source chain truncated at {MAX_SOURCE_CHAIN_DEPTH} frames>"
            )?;
            break;
        }
        match (debug, alternate) {
            (true, true) => write!(f, "\n  {depth}: {src:#?}")?,
            (true, false) => write!(f, "\n  {depth}: {src:?}")?,
            (false, true) => write!(f, "\n  {depth}: {src:#}")?,
            (false, false) => write!(f, "\n  {depth}: {src}")?,
        }
        cur = src.source();
        depth += 1;
    }
    Ok(())
}

/// Appends the `DiagnosticsContext` (when present). The renderer is
/// chosen by the `debug` and `alternate` flags so the same helper can
/// serve both the Display and Debug paths on [`Error`]:
///
/// * Display path (`debug = false`) uses `DiagnosticsContext::Display`,
///   which renders the high-signal one-line summary
///   (`activity=… duration=…ms requests=N charge=…RU [status=…]`) and,
///   under `{:#}`, follows it with the summarized diagnostics JSON.
///   Keeping Display-mode output rendered via Display avoids splicing
///   derived-Debug `Field { … }` blocks into the user-facing rich
///   `{e:#}` rendering.
/// * Debug path (`debug = true`) uses `DiagnosticsContext::Debug` so
///   the structured representation cascades out of `{e:?}` / `{e:#?}`
///   alongside the rest of the Debug output.
fn write_diagnostics(
    f: &mut fmt::Formatter<'_>,
    inner: &ErrorInner,
    debug: bool,
    alternate: bool,
) -> fmt::Result {
    let Some(diag) = inner.diagnostics.as_deref() else {
        return Ok(());
    };
    f.write_str("\n\nDiagnostics:\n")?;
    match (debug, alternate) {
        (true, true) => write!(f, "{diag:#?}"),
        (true, false) => write!(f, "{diag:?}"),
        (false, true) => write!(f, "{diag:#}"),
        (false, false) => write!(f, "{diag}"),
    }
}

fn write_backtrace(f: &mut fmt::Formatter<'_>, err: &Error) -> fmt::Result {
    if let Some(bt) = err.backtrace() {
        f.write_str("\n\nStack backtrace:\n")?;
        f.write_str(bt.as_ref())?;
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
    // When the underlying failure is an HTTP response that already arrived
    // and was buffered by `azure_core`, lift the wire body + parsed Cosmos
    // headers onto the typed error so callers can reach them via
    // `Error::response_body()` / `Error::cosmos_headers()` without having to
    // downcast `source()` back to `azure_core::Error` and re-extract.
    //
    // `RawResponse: Clone` here is cheap: `Headers` is a small map, the body
    // is `Bytes` (refcount bump), and this path only runs at error
    // construction time — well off the steady-state hot path.
    let payload = match error.kind() {
        azure_core::error::ErrorKind::HttpResponse {
            raw_response: Some(raw),
            ..
        } => {
            let raw = (**raw).clone();
            let (_status, headers, body) = raw.deconstruct();
            let cosmos_headers = CosmosResponseHeaders::from_headers(&headers);
            let body_bytes = azure_core::Bytes::from(body);
            Some(Box::new(CosmosResponsePayload::new(
                ResponseBody::Bytes(body_bytes),
                cosmos_headers,
            )))
        }
        _ => None,
    };
    Error::from_inner(ErrorInner {
        status,
        payload,
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
///
/// The walk is bounded by [`MAX_SOURCE_CHAIN_DEPTH`] frames. Real Cosmos
/// transport chains are never deeper than ~5; the cap exists so this
/// function — which sits on the hot path of every
/// `azure_core::Error → driver::Error` conversion — cannot be pinned to a
/// CPU core by a pathological or cyclic source chain. `Error::source`
/// is not required to be acyclic, and arbitrary `azure_core::Error`
/// chains can originate from any transport / credential / wrapper layer
/// outside the driver.
fn refine_status_from_source_chain(
    start: Option<&(dyn StdError + 'static)>,
) -> Option<CosmosStatus> {
    let mut cur = start;
    for _ in 0..MAX_SOURCE_CHAIN_DEPTH {
        let Some(e) = cur else { return None };
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

/// Maximum number of `.source()` frames inspected by
/// [`refine_status_from_source_chain`]. Generous relative to real Cosmos
/// transport chains (~5 frames) so we never miss a meaningful inner cause,
/// but bounded so a pathological or cyclic chain cannot pin the boundary
/// mapper on a hot path.
const MAX_SOURCE_CHAIN_DEPTH: usize = 64;

/// Driver-wide `Result` alias.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind as AzKind;
    use azure_core::http::headers::Headers;

    #[test]
    fn service_from_parts_populates_status_and_headers() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let err =
            Error::service_from_parts(status, CosmosResponseHeaders::default(), b"{}", "throttled");
        assert_eq!(err.kind(), Kind::Service);
        assert!(err.status().is_throttled());
        assert!(err.status().is_transient());
        assert_eq!(err.status_code(), StatusCode::TooManyRequests);
        assert!(err.cosmos_headers().is_some());
        // No diagnostics attached by the constructor; the operation
        // pipeline grafts them downstream via `with_diagnostics`.
        assert!(err.diagnostics().is_none());
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
    fn from_azure_core_http_response_lifts_body_and_headers_onto_error() {
        // Regression guard: when the boundary mapper sees an
        // `AzKind::HttpResponse { raw_response: Some(..), .. }` it must
        // surface the wire body + parsed Cosmos headers on the resulting
        // `Error` so callers can read them via `response_body()` /
        // `cosmos_headers()` without downcasting `source()` back to
        // `azure_core::Error`.
        use azure_core::http::headers::HeaderName;
        let mut headers = Headers::new();
        // Two representative Cosmos headers: one numeric, one ETag-shaped,
        // so we can verify both wire-level shape and Cosmos parsing.
        headers.insert(HeaderName::from_static("x-ms-request-charge"), "12.34");
        headers.insert(HeaderName::from_static("etag"), "\"abc\"");

        let body = br#"{"code":"BadRequest","message":"missing partition key"}"#.to_vec();
        let raw = azure_core::Error::new(
            AzKind::HttpResponse {
                status: StatusCode::BadRequest,
                error_code: Some("BadRequest".to_string()),
                raw_response: Some(Box::new(azure_core::http::RawResponse::from_bytes(
                    StatusCode::BadRequest,
                    headers,
                    body.clone(),
                ))),
            },
            "bad request",
        );

        let cosmos: Error = raw.into();
        assert_eq!(cosmos.kind(), Kind::Service);
        assert_eq!(cosmos.status_code(), StatusCode::BadRequest);

        // Body lifted verbatim.
        assert_eq!(
            cosmos.response_body(),
            Some(body.as_slice()),
            "response body must be reachable from the typed error"
        );

        // Cosmos headers parsed from the wire headers.
        let ch = cosmos
            .cosmos_headers()
            .expect("parsed Cosmos headers must be reachable from the typed error");
        assert_eq!(
            ch.request_charge.map(|r| r.value()),
            Some(12.34),
            "x-ms-request-charge must round-trip into CosmosResponseHeaders"
        );
        assert!(
            ch.etag.is_some(),
            "etag must round-trip into CosmosResponseHeaders"
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

    /// Builds an `Error` carrying both a `DiagnosticsContext` and a
    /// nested Cosmos `Error` as its source, so format tests can exercise
    /// the source-chain + diagnostics propagation paths together.
    fn make_error_with_diagnostics_and_source() -> Error {
        let inner = Error::end_to_end_timeout("inner timeout", None);
        Error::transport(
            CosmosStatus::TRANSPORT_GENERATED_503,
            "outer transport failure",
            Some(make_test_diagnostics()),
            Some(Arc::new(inner)),
        )
    }

    /// Fabricates a fresh `Arc<DiagnosticsContext>` for tests that need
    /// any non-`None` diagnostics value. Produced via the real builder so
    /// no production-only fixture (`error_placeholder`) is required.
    fn make_test_diagnostics() -> Arc<DiagnosticsContext> {
        use crate::diagnostics::DiagnosticsContextBuilder;
        use crate::models::ActivityId;
        use crate::options::DiagnosticsOptions;
        Arc::new(
            DiagnosticsContextBuilder::new(
                ActivityId::new_uuid(),
                Arc::new(DiagnosticsOptions::default()),
            )
            .complete(),
        )
    }

    #[test]
    fn with_diagnostics_attaches_diagnostics_without_mutating_original() {
        // Starting from an error with no diagnostics, `with_diagnostics`
        // returns a new error carrying the supplied context. The original
        // error is left untouched (Clone-on-Arc semantics) and all other
        // fields survive the clone-and-patch path.
        let original = Error::end_to_end_timeout("no diags", None);
        assert!(original.diagnostics().is_none());

        let diag = make_test_diagnostics();
        let attached = original.with_diagnostics(Arc::clone(&diag));

        assert!(
            Arc::ptr_eq(attached.diagnostics().expect("diagnostics attached"), &diag),
            "with_diagnostics must store the supplied Arc verbatim"
        );
        assert!(
            original.diagnostics().is_none(),
            "original must be untouched by with_diagnostics"
        );
        assert_eq!(attached.status(), original.status());
    }

    #[test]
    fn display_plain_includes_typed_header_and_message_on_one_line() {
        // `{e}` must surface the typed `[Kind] status/sub (name): message`
        // header on a single line so existing log sites that didn't opt
        // into `{e:#}` still see the Cosmos status this error type exists
        // to expose. The source chain, diagnostics block, and backtrace
        // are reserved for the opt-in `{e:#}` form so they don't corrupt
        // callers concatenating the message into other strings.
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err}");
        assert!(
            !rendered.contains('\n'),
            "plain display must stay on one line, got:\n{rendered}"
        );
        assert!(
            rendered.contains("[Transport]"),
            "plain display must include the categorical kind, got:\n{rendered}"
        );
        assert!(
            rendered.ends_with(": outer transport failure"),
            "plain display must end with `: <message>`, got:\n{rendered}"
        );
        assert!(
            !rendered.contains("Caused by:"),
            "plain display must not emit the source chain, got:\n{rendered}"
        );
        assert!(
            !rendered.contains("Diagnostics:"),
            "plain display must not emit the diagnostics block, got:\n{rendered}"
        );
    }

    #[test]
    fn display_alternate_includes_header_source_chain_and_diagnostics() {
        // `{e:#}` is the opt-in rich multi-line form: it must surface the
        // typed status header, the `Caused by:` chain, and the structured
        // diagnostics block. Backtrace presence is best-effort
        // (rate-limited globally) and not asserted.
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:#}");
        assert!(
            rendered.contains("[Transport]"),
            "alternate display must include the categorical kind from CosmosStatus::Display, got:\n{rendered}"
        );
        assert!(
            rendered.contains("outer transport failure"),
            "alternate display must include the error message, got:\n{rendered}"
        );
        assert!(
            rendered.contains("Caused by:") && rendered.contains("inner timeout"),
            "alternate display must include the source chain, got:\n{rendered}"
        );
        assert!(
            rendered.contains("Diagnostics:"),
            "alternate display must include the diagnostics block, got:\n{rendered}"
        );
    }

    #[test]
    fn debug_omits_backtrace_block_in_plain_form() {
        // `{e:?}` is the everyday Debug form used by `tracing::error!(?e)`
        // and `Result::unwrap` — it must NOT emit the multi-line stack
        // backtrace block, which is reserved for the opt-in `{e:#?}`.
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:?}");
        assert!(
            !rendered.contains("Stack backtrace:"),
            "plain debug must not emit the backtrace block, got:\n{rendered}"
        );
        // The header and source chain must still be present.
        assert!(rendered.contains("outer transport failure"));
        assert!(rendered.contains("Caused by:"));
    }

    #[test]
    fn debug_alternate_propagates_to_source_and_diagnostics() {
        // `{e:#?}` must propagate the alternate flag into the wrapped
        // source entries and the diagnostics block, so callers opting
        // into the rich form get the pretty-printed multi-line layout
        // from every type that implements `Debug` along the chain.
        //
        // We assert propagation indirectly by comparing the plain and
        // alternate Debug renderings: the alternate form must be a
        // strict superset (additional whitespace / newlines from the
        // pretty layout, plus the optional backtrace block when one was
        // captured).
        let err = make_error_with_diagnostics_and_source();
        let plain = format!("{err:?}");
        let alternate = format!("{err:#?}");

        assert!(
            alternate.len() > plain.len(),
            "alternate debug must be richer than plain debug.\nPlain:\n{plain}\nAlternate:\n{alternate}"
        );
        // The diagnostics block must use multi-line Debug layout in the
        // alternate form. The derived `Debug` for `DiagnosticsContext`
        // emits field-per-line indentation under `{:#?}`, so a `\n    `
        // sequence after the `Diagnostics:` marker is a reliable signal
        // that the alternate flag propagated into it.
        let diag_idx = alternate
            .find("Diagnostics:")
            .expect("alternate debug must include the diagnostics block");
        let after_diag = &alternate[diag_idx..];
        assert!(
            after_diag.contains("\n    "),
            "alternate flag must cascade into DiagnosticsContext::Debug (expected indented multi-line layout), got:\n{after_diag}"
        );
    }

    /// Regression guard: a cyclic (or pathologically deep) `source()` chain
    /// must not cause `Display`/`Debug` on `Error` to run unbounded. The
    /// source-chain walker caps at `MAX_SOURCE_CHAIN_DEPTH` frames and
    /// emits a `<source chain truncated ...>` marker so a single
    /// `tracing::error!` cannot pin a thread.
    #[test]
    fn display_and_debug_bound_source_chain_walk() {
        // Self-referential `StdError::source` returning the same error
        // forever — simulates a cyclic chain without needing unsafe.
        #[derive(Debug)]
        struct CyclicError;
        impl fmt::Display for CyclicError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("cyclic")
            }
        }
        impl StdError for CyclicError {
            fn source(&self) -> Option<&(dyn StdError + 'static)> {
                // Return &'static self via a leaked static so the borrow
                // lifetime is satisfied without unsafe.
                static SELF: CyclicError = CyclicError;
                Some(&SELF)
            }
        }

        let err = Error::transport(
            CosmosStatus::TRANSPORT_GENERATED_503,
            "outer",
            None,
            Some(Arc::new(CyclicError)),
        );

        // Debug must terminate and emit the truncation marker. We only
        // exercise the Debug path (`{err:?}`) here: it emits the source
        // chain without rendering the backtrace block, so this test does
        // not pollute the process-global frame cache and cannot race with
        // sibling backtrace tests that assert on its size. The walker is
        // shared between Display and Debug, so covering one path proves
        // the cap fires on both.
        let rendered = format!("{err:?}");
        assert!(
            rendered.contains("<source chain truncated"),
            "expected truncation marker for cyclic source chain, got:\n{rendered}"
        );
        assert!(
            rendered.len() < 64 * 1024,
            "rendered length ({}) suggests unbounded walk",
            rendered.len(),
        );
    }
}
