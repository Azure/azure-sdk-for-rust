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
//! Underlying third-party errors (credential failures, HMAC failures, HTTP
//! transport errors, …) are wrapped at the call site that invokes the
//! third-party API — each such site picks the most specific typed
//! constructor ([`Error::client`], [`Error::authentication`],
//! [`Error::transport`], [`Error::serialization`], …) and attaches the
//! original error as [`StdError::source`] so callers can still downcast
//! through it.

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

/// Internal bench-only surface (gated by the `__internal_backtrace_bench`
/// feature) used by `azure_data_cosmos_benchmarks` to measure the
/// rate-limited backtrace machinery deterministically. Not covered by
/// SemVer; production code MUST NOT enable the feature.
#[cfg(feature = "__internal_backtrace_bench")]
#[doc(hidden)]
pub use backtrace::__bench as backtrace_bench;

/// Categorical kind for an [`Error`] — re-exported from
/// [`crate::models::Kind`] (where the canonical definition lives alongside
/// [`CosmosStatus`]).
pub use crate::models::Kind;

/// Cosmos DB error returned from every public API in the driver (and, by
/// re-export, every public API in the SDK).
///
/// Always exposes Cosmos-typed status and parsed response headers when they
/// are available — for both real service errors and synthetic client-side
/// conditions (e.g. an end-to-end operation timeout surfaces as
/// `408 / 20008` even though no HTTP response was received).
///
/// Underlying errors (transport, credential, deserialization, …) are
/// reachable via [`std::error::Error::source`].
///
/// `Error` is `Clone` (a cheap `Arc` refcount bump) so callers can pass it
/// by value through `Result` chains without re-allocating, and so the
/// pipeline can patch single fields (e.g. attaching diagnostics via
/// [`Error::with_diagnostics`]) cheaply. All fields are wrapped behind a
/// single `Arc` so the outer struct is one pointer wide, keeping
/// `Result<T, Error>` small.
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
    // Mutators (internal only — public callers go through ErrorBuilder).
    // -----------------------------------------------------------------

    /// Returns a copy of `self` with `diagnostics` attached (or replaced).
    ///
    /// Used by the operation pipeline's abort branch to graft the completed
    /// operation [`DiagnosticsContext`] (retry history, region attempts,
    /// per-request events) onto an error that was built deep in the
    /// pipeline before that context was available. Without this, the
    /// operation diagnostics would be silently dropped on every aborted
    /// operation — callers reading [`Error::diagnostics`] would see `None`
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
    ///   cache-missed frame.
    ///
    /// The two limiters are intentionally **independent** — capture
    /// pressure and resolution pressure do not feed back into one
    /// another. Capture is cheap (microseconds + a small allocation)
    /// and is bounded by the capture throttle alone; resolution is the
    /// expensive work and is bounded by the resolution limiter alone.
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
    /// * **Errors wrapping a third-party error** (e.g. credential or HMAC
    ///   failures lifted into [`Error::authentication`]) point at the
    ///   explicit construction site in driver code, not the originating
    ///   failure site inside the third-party crate. The typed [`Kind`],
    ///   status, and `std::error::Error::source()` chain (which preserves
    ///   the underlying error — `reqwest::Error`, `h2::Error`,
    ///   `io::Error`, …) remain the primary diagnostic signal in that
    ///   case.
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
/// (e.g. another Cosmos [`Error`], `io::Error`, `h2::Error`) surface their
/// full debug representation rather than a one-line `Display` summary.
/// Display mode (`alternate Display` on [`Error`]) keeps the
/// human-readable single-line form per entry.
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
        // Bound the walk by `MAX_SOURCE_CHAIN_DEPTH` so a pathological
        // or cyclic `source()` chain cannot pin a thread formatting an
        // error. This runs on every `tracing::error!`, `format!`, and
        // panic message.
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

/// Maximum number of `.source()` frames walked when rendering an
/// [`Error`] via [`fmt::Display`] / [`fmt::Debug`]. Generous relative to
/// real Cosmos transport chains (~5 frames) but bounded so a pathological
/// or cyclic chain cannot pin a thread formatting an error.
const MAX_SOURCE_CHAIN_DEPTH: usize = 64;

/// Driver-wide `Result` alias.
pub type Result<T> = std::result::Result<T, Error>;

// =========================================================================
// ErrorBuilder
// =========================================================================

impl Error {
    /// Returns a fluent [`ErrorBuilder`] seeded with sensible defaults for
    /// the given categorical [`Kind`]. This is the only public way to
    /// construct an [`Error`] from outside the crate.
    ///
    /// ```
    /// use azure_data_cosmos_driver::error::{Error, Kind};
    ///
    /// let err = Error::builder(Kind::Client)
    ///     .with_message("missing partition key")
    ///     .build();
    /// assert_eq!(err.kind(), Kind::Client);
    /// ```
    pub fn builder(kind: Kind) -> ErrorBuilder {
        ErrorBuilder::new(kind)
    }
}

/// Fluent builder for [`Error`]. The only public way to construct or
/// re-decorate a Cosmos [`Error`] from outside the driver crate.
///
/// Obtain one via [`Error::builder(kind)`](Error::builder) to start fresh,
/// or [`ErrorBuilder::from_error`] to patch an existing error (add
/// context, attach headers, swap status, etc.). Finalize with
/// [`build()`](Self::build).
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos_driver::error::{Error, ErrorBuilder, Kind};
///
/// let inner = Error::builder(Kind::Client)
///     .with_message("bad payload")
///     .build();
/// let outer = ErrorBuilder::from_error(inner)
///     .with_context("uploadItem(id=42)")
///     .build();
/// assert!(format!("{outer}").contains("uploadItem(id=42): bad payload"));
/// ```
#[must_use = "ErrorBuilder is inert until `.build()` is called"]
pub struct ErrorBuilder {
    /// When `Some`, build clones this error's inner state and patches the
    /// overridden fields. When `None`, build constructs a fresh error from
    /// `kind` defaults.
    base: Option<Error>,
    /// Categorical kind (sets default status when `status` is `None`).
    kind: Kind,
    /// Override status. When `None`, falls back to the kind default (or
    /// the base error's status when `base` is set).
    status: Option<CosmosStatus>,
    message: Option<Arc<str>>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    diagnostics: Option<Arc<DiagnosticsContext>>,
    cosmos_headers: Option<CosmosResponseHeaders>,
    response_body: Option<bytes::Bytes>,
    /// Prepended to the final message as `"{context}: {message}"` when set.
    context_prefix: Option<Arc<str>>,
}

impl ErrorBuilder {
    fn new(kind: Kind) -> Self {
        Self {
            base: None,
            kind,
            status: None,
            message: None,
            source: None,
            diagnostics: None,
            cosmos_headers: None,
            response_body: None,
            context_prefix: None,
        }
    }

    /// Starts a builder pre-populated from an existing [`Error`]. Any
    /// subsequent setter overrides the corresponding field; unset fields
    /// are carried forward from `err`. Useful for re-decorating an error
    /// returned from a deeper layer (attaching operation context, swapping
    /// the categorical status, attaching diagnostics, etc.).
    pub fn from_error(err: Error) -> Self {
        let kind = err.kind();
        Self {
            base: Some(err),
            kind,
            status: None,
            message: None,
            source: None,
            diagnostics: None,
            cosmos_headers: None,
            response_body: None,
            context_prefix: None,
        }
    }

    /// Overrides the [`CosmosStatus`]. The builder's [`Kind`] is forced
    /// onto the status so the categorical kind stays consistent.
    pub fn with_status(mut self, status: CosmosStatus) -> Self {
        self.status = Some(status.with_kind(self.kind));
        self
    }

    /// Sets the human-readable error message.
    pub fn with_message(mut self, message: impl Into<Arc<str>>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Attaches an underlying source error reachable via
    /// [`std::error::Error::source`].
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        self.source = Some(Arc::new(source));
        self
    }

    /// Attaches an already-shared `Arc`-wrapped source. Use this when the
    /// caller already owns an `Arc` (e.g. propagating a wrapped Cosmos
    /// [`Error`] as the source). For plain `StdError` values prefer
    /// [`with_source`](Self::with_source).
    pub fn with_arc_source(mut self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self {
        self.source = Some(source);
        self
    }

    /// Attaches the operation [`DiagnosticsContext`].
    pub fn with_diagnostics(mut self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    /// Attaches parsed Cosmos response headers.
    pub fn with_cosmos_headers(mut self, headers: CosmosResponseHeaders) -> Self {
        self.cosmos_headers = Some(headers);
        self
    }

    /// Attaches the raw service response body bytes (typically a Cosmos
    /// JSON error payload). Stored cheaply as [`bytes::Bytes`].
    pub fn with_response_body(mut self, body: impl Into<bytes::Bytes>) -> Self {
        self.response_body = Some(body.into());
        self
    }

    /// Prepends operational context to the final message as
    /// `"{context}: {message}"`. Repeated calls override (the most recent
    /// context wins); chain multiple `with_context` calls into one combined
    /// string at the call site if multiple layers of context are needed.
    pub fn with_context(mut self, context: impl Into<Arc<str>>) -> Self {
        self.context_prefix = Some(context.into());
        self
    }

    /// Finalizes the builder into an [`Error`]. Allocation-cheap (single
    /// `Arc<ErrorInner>` regardless of which fields were set).
    pub fn build(self) -> Error {
        // Start from either the base error's inner state or a fresh
        // ErrorInner seeded from the kind's default status.
        let mut inner = match &self.base {
            Some(base) => (*base.inner).clone(),
            None => ErrorInner {
                status: default_status_for(self.kind),
                payload: None,
                diagnostics: None,
                message: Arc::<str>::from(""),
                source: None,
                backtrace: None,
            },
        };

        // Apply overrides. We force the builder's kind onto whatever status
        // the caller (or the base error) provides so the categorical kind
        // matches the construction intent.
        if let Some(status) = self.status {
            inner.status = status.with_kind(self.kind);
        } else {
            inner.status = inner.status.with_kind(self.kind);
        }
        if let Some(message) = self.message {
            inner.message = message;
        }
        if self.source.is_some() {
            inner.source = self.source;
        }
        if self.diagnostics.is_some() {
            inner.diagnostics = self.diagnostics;
        }
        // Body/headers updates rebuild the optional payload; either can be
        // set independently (e.g. headers without a body for a non-service
        // error that still carries parsed Cosmos response headers).
        if self.cosmos_headers.is_some() || self.response_body.is_some() {
            let existing_body = inner
                .payload
                .as_deref()
                .map(|p| p.body().clone())
                .unwrap_or(ResponseBody::NoPayload);
            let existing_headers = inner
                .payload
                .as_deref()
                .map(|p| p.headers().clone())
                .unwrap_or_default();
            let headers = self.cosmos_headers.unwrap_or(existing_headers);
            let body = match self.response_body {
                Some(bytes) => ResponseBody::Bytes(bytes),
                None => existing_body,
            };
            inner.payload = Some(Box::new(CosmosResponsePayload::new(body, headers)));
        }
        if let Some(prefix) = self.context_prefix {
            let mut buf =
                String::with_capacity(prefix.len() + 2 + inner.message.len());
            buf.push_str(&prefix);
            buf.push_str(": ");
            buf.push_str(&inner.message);
            inner.message = Arc::<str>::from(buf);
        }

        Error::from_inner(inner)
    }
}

fn default_status_for(kind: Kind) -> CosmosStatus {
    match kind {
        Kind::Service => CosmosStatus::new(StatusCode::InternalServerError).with_kind(kind),
        Kind::Transport => CosmosStatus::TRANSPORT_GENERATED_503,
        Kind::Client => CosmosStatus::new(StatusCode::BadRequest).with_kind(kind),
        Kind::Authentication => CosmosStatus::AUTHENTICATION_TOKEN_ACQUISITION_FAILED,
        Kind::Serialization => CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID,
        Kind::Configuration => CosmosStatus::new(StatusCode::BadRequest).with_kind(kind),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // Public ErrorBuilder surface
    // -----------------------------------------------------------------

    #[test]
    fn builder_kind_defaults_pick_sensible_status() {
        // Each kind seeds a default status whose Kind matches the builder
        // so callers that only set a message still produce a coherent
        // error.
        for kind in [
            Kind::Client,
            Kind::Configuration,
            Kind::Authentication,
            Kind::Serialization,
            Kind::Transport,
            Kind::Service,
        ] {
            let err = Error::builder(kind).with_message("m").build();
            assert_eq!(err.kind(), kind, "kind mismatch for {kind:?}");
            assert_eq!(err.status().kind(), kind, "status kind mismatch for {kind:?}");
            assert_eq!(&*format!("{err}").split(": ").last().unwrap(), "m");
        }
    }

    #[test]
    fn builder_with_status_overrides_default_but_forces_kind() {
        let err = Error::builder(Kind::Transport)
            .with_status(CosmosStatus::new(StatusCode::ServiceUnavailable))
            .with_message("nope")
            .build();
        assert_eq!(err.kind(), Kind::Transport);
        assert_eq!(err.status_code(), StatusCode::ServiceUnavailable);
        // Status's own kind was Service by default; builder forces Transport.
        assert_eq!(err.status().kind(), Kind::Transport);
    }

    #[test]
    fn builder_with_source_preserves_via_std_error_source() {
        let io = std::io::Error::new(std::io::ErrorKind::Other, "underlying");
        let err = Error::builder(Kind::Transport)
            .with_message("wrapped")
            .with_source(io)
            .build();
        let src = StdError::source(&err).expect("source preserved");
        assert!(src.to_string().contains("underlying"));
    }

    #[test]
    fn builder_with_arc_source_accepts_shared_handle() {
        let inner = Arc::new(Error::builder(Kind::Client).with_message("inner").build())
            as Arc<dyn StdError + Send + Sync + 'static>;
        let outer = Error::builder(Kind::Transport)
            .with_arc_source(inner)
            .with_message("outer")
            .build();
        let src = StdError::source(&outer).expect("source preserved");
        assert!(src.to_string().contains("inner"));
    }

    #[test]
    fn builder_with_diagnostics_attaches() {
        let diag = make_test_diagnostics();
        let err = Error::builder(Kind::Client)
            .with_message("m")
            .with_diagnostics(Arc::clone(&diag))
            .build();
        assert!(Arc::ptr_eq(err.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_with_cosmos_headers_and_body_round_trip() {
        let mut headers = CosmosResponseHeaders::default();
        headers.substatus = Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE);
        let body = b"{\"code\":\"X\"}".to_vec();
        let err = Error::builder(Kind::Service)
            .with_status(CosmosStatus::new(StatusCode::NotFound).with_sub_status(1002))
            .with_message("session miss")
            .with_cosmos_headers(headers)
            .with_response_body(body.clone())
            .build();
        assert_eq!(err.status_code(), StatusCode::NotFound);
        assert_eq!(err.response_body(), Some(body.as_slice()));
        assert_eq!(
            err.cosmos_headers().and_then(|h| h.substatus),
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE)
        );
    }

    #[test]
    fn builder_with_context_prepends_to_message() {
        let err = Error::builder(Kind::Client)
            .with_message("bad payload")
            .with_context("op=createItem")
            .build();
        let rendered = format!("{err}");
        assert!(
            rendered.ends_with(": op=createItem: bad payload"),
            "got: {rendered}"
        );
    }

    #[test]
    fn builder_from_error_carries_forward_unset_fields() {
        let diag = make_test_diagnostics();
        let original = Error::builder(Kind::Client)
            .with_message("first")
            .with_diagnostics(Arc::clone(&diag))
            .build();

        // No setters \u2014 build should clone original unchanged (modulo a
        // re-captured backtrace at the construction site, since
        // from_error doesn't preserve the inner Arc).
        let cloned = ErrorBuilder::from_error(original.clone()).build();
        assert_eq!(cloned.kind(), Kind::Client);
        assert_eq!(cloned.status(), original.status());
        assert_eq!(format!("{cloned}"), format!("{original}"));
        assert!(Arc::ptr_eq(cloned.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_from_error_with_context_preserves_status_and_source() {
        let inner_io = std::io::Error::new(std::io::ErrorKind::Other, "io fail");
        let original = Error::builder(Kind::Transport)
            .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
            .with_message("base")
            .with_source(inner_io)
            .build();

        let decorated = ErrorBuilder::from_error(original.clone())
            .with_context("op=read")
            .build();

        assert_eq!(decorated.status(), original.status());
        // Source chain preserved.
        let src = StdError::source(&decorated).expect("source carried forward");
        assert!(src.to_string().contains("io fail"));
        // Context prepended.
        assert!(format!("{decorated}").contains("op=read: base"));
    }

    #[test]
    fn builder_from_error_swap_status_keeps_other_fields() {
        let diag = make_test_diagnostics();
        let original = Error::builder(Kind::Service)
            .with_status(CosmosStatus::new(StatusCode::TooManyRequests))
            .with_message("throttled")
            .with_diagnostics(Arc::clone(&diag))
            .build();

        // Re-decorate as a Transport error (e.g. retry-budget exhausted
        // synthesizes a synthetic 503 wrapping the original Service error
        // \u2014 the abort path in the operation pipeline).
        let promoted = ErrorBuilder::from_error(original)
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .build();
        // Builder's Kind is still Service (inherited from base); status's
        // Kind is forced to match. Demonstrates that callers wanting a
        // kind switch should re-issue Error::builder(new_kind) and chain
        // .with_source() / .with_diagnostics(); from_error preserves the
        // original Kind so context-only patches stay consistent.
        assert_eq!(promoted.kind(), Kind::Service);
        assert_eq!(promoted.status_code(), StatusCode::ServiceUnavailable);
        assert!(Arc::ptr_eq(promoted.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_message_setter_overrides_base_message() {
        let original = Error::builder(Kind::Client).with_message("orig").build();
        let patched = ErrorBuilder::from_error(original)
            .with_message("replaced")
            .build();
        assert!(format!("{patched}").ends_with(": replaced"));
    }

    #[test]
    fn builder_repeated_setters_last_write_wins() {
        let err = Error::builder(Kind::Client)
            .with_message("first")
            .with_message("second")
            .with_context("ctx-a")
            .with_context("ctx-b")
            .build();
        let rendered = format!("{err}");
        assert!(rendered.ends_with(": ctx-b: second"), "got: {rendered}");
    }

    // -----------------------------------------------------------------
    // Existing internal-surface tests
    // -----------------------------------------------------------------

    #[test]
    fn service_from_parts_populates_status_and_headers() {
        let status = CosmosStatus::new(StatusCode::TooManyRequests).with_sub_status(3200);
        let err = Error::builder(Kind::Service)
            .with_status(status)
            .with_message("throttled")
            .with_cosmos_headers(CosmosResponseHeaders::default())
            .with_response_body(b"{}".to_vec())
            .build();
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
        let err = Error::builder(Kind::Transport)
            .with_status(CosmosStatus::from_parts(
                StatusCode::RequestTimeout,
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
            ))
            .with_message("e2e timeout")
            .build();
        assert_eq!(err.kind(), Kind::Transport);
        assert_eq!(err.status_code(), StatusCode::RequestTimeout);
        assert_eq!(
            err.sub_status(),
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
        assert!(err.status().is_timeout());
        assert!(err.status().is_transient());
    }

    fn end_to_end_timeout_error(message: &'static str) -> Error {
        Error::builder(Kind::Transport)
            .with_status(CosmosStatus::from_parts(
                StatusCode::RequestTimeout,
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
            ))
            .with_message(message)
            .build()
    }

    #[test]
    fn wrap_inherits_backtrace_from_cosmos_source() {
        // Build an inner Cosmos error so it carries a captured backtrace.
        let inner = end_to_end_timeout_error("inner");
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
        let outer = Error::builder(Kind::Transport)
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("outer")
            .with_arc_source(Arc::new(inner))
            .build();
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
        let inner = end_to_end_timeout_error("inner timeout");
        Error::builder(Kind::Transport)
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("outer transport failure")
            .with_diagnostics(make_test_diagnostics())
            .with_arc_source(Arc::new(inner))
            .build()
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
        let original = end_to_end_timeout_error("no diags");
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

        let err = Error::builder(Kind::Transport)
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("outer")
            .with_arc_source(Arc::new(CyclicError))
            .build();

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
