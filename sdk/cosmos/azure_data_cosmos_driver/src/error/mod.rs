// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB-specific error type carrying typed Cosmos status, the optional
//! wire-level [`CosmosResponse`], and operation diagnostics — for both
//! service errors (real HTTP responses) and synthetic client-side conditions
//! (transport failures, end-to-end timeouts, client validation, etc.).
//!
//! Mirrors the .NET / Java SDKs' `CosmosException`: a single error type that
//! surfaces typed Cosmos status (HTTP status + sub-status, including synthetic
//! codes such as `408 / 20008` for end-to-end timeout), the originating
//! [`CosmosResponse`] when one was received, and the operation
//! [`DiagnosticsContext`].
//!
//! Underlying third-party errors (credential failures, HMAC failures, HTTP
//! transport errors, …) are wrapped at the call site that invokes the
//! third-party API and attached as [`StdError::source`] so callers can still
//! downcast through the chain.

use std::{error::Error as StdError, fmt, sync::Arc};

use crate::{
    diagnostics::DiagnosticsContext,
    models::{CosmosResponse, CosmosResponsePayload},
};

pub mod cosmos_status;
pub use cosmos_status::{CosmosStatus, SubStatusCode};

pub(crate) mod backtrace;
pub(crate) use backtrace::Backtrace;

/// Internal bench-only surface (gated by the `__internal_backtrace_bench`
/// feature) used by `azure_data_cosmos_benchmarks` to measure the
/// rate-limited backtrace machinery deterministically. Not covered by
/// SemVer; production code MUST NOT enable the feature.
#[cfg(feature = "__internal_backtrace_bench")]
#[doc(hidden)]
pub use backtrace::__bench as backtrace_bench;

/// Cosmos DB error returned from every public API in the driver (and, by
/// re-export, every public API in the SDK).
///
/// Always exposes Cosmos-typed status — for both real service errors and
/// synthetic client-side conditions (e.g. an end-to-end operation timeout
/// surfaces as `408 / 20008` even though no HTTP response was received). The
/// originating [`CosmosResponse`] is reachable via [`Self::response`] when a
/// wire response was received, carrying the parsed Cosmos response headers,
/// the body, and the operation diagnostics together.
///
/// Underlying errors (transport, credential, deserialization, …) are
/// reachable via [`std::error::Error::source`].
///
/// `CosmosError` is `Clone` (a cheap `Arc` refcount bump) so callers can pass
/// it by value through `Result` chains without re-allocating, and so the
/// pipeline can patch single fields (e.g. attaching diagnostics) cheaply.
///
/// # Invariants
///
/// All construction goes through [`CosmosErrorBuilder`], which guarantees
/// the following relationships at `build()` time:
///
/// * [`status()`](Self::status) and [`kind()`](Self::kind) always reflect
///   the current [`CosmosStatus`].
/// * When [`response()`](Self::response) is `Some` (wire-response errors),
///   the builder enforces *"CosmosResponse wins"*:
///   - `status() == response().status()`
///   - `diagnostics() == Some(response().diagnostics())`
///
///   Any value supplied via [`CosmosErrorBuilder::with_status`] or
///   [`CosmosErrorBuilder::with_diagnostics`] in the same builder chain is
///   silently overridden — the [`CosmosResponse`] is the source of truth.
/// * When [`response()`](Self::response) is `None`,
///   [`diagnostics()`](Self::diagnostics) returns whatever the pipeline
///   attached via [`CosmosErrorBuilder::with_diagnostics`], or `None` if
///   none was attached.
///
/// These invariants imply the chain
/// `kind() == status().kind() == response().status().kind() ==
/// diagnostics().status().kind()` whenever each side is defined, since
/// [`CosmosResponse`] itself guarantees
/// `response.status() == response.diagnostics().status()`.
#[derive(Clone)]
pub struct CosmosError {
    inner: Arc<CosmosErrorInner>,
}

#[derive(Clone)]
struct CosmosErrorInner {
    /// Cosmos status (HTTP status + sub-status + categorical
    /// Always present, shared across all
    /// [`ErrorContext`] variants — for the `Wire` variant this is
    /// reconciled to match `response.status()` at `build()` time.
    status: CosmosStatus,
    /// Discriminates wire-response errors (carrying a full
    /// [`CosmosResponse`]) from synthetic errors (carrying at most a
    /// standalone [`DiagnosticsContext`]) and the internal
    /// pre-diagnostics-finalization [`ErrorContext::WirePending`] state.
    /// Modelled as an enum so the storage rules are enforced by the type
    /// system rather than by runtime convention.
    context: ErrorContext,
    message: Arc<str>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Captured stack backtrace, present when capture is enabled (opt-in
    /// via `RUST_BACKTRACE` or the runtime builder) and the global
    /// rate-limited backtrace capture budget allowed it. See the
    /// [`backtrace`] module for the cost model and tuning knobs.
    backtrace: Option<Backtrace>,
}

/// Three-state carrier discriminating "no wire response" (`Synthetic`),
/// "wire data captured but diagnostics not finalized yet" (`WirePending`,
/// internal-only), and "fully assembled wire response" (`Wire`). Private —
/// public accessors on [`CosmosError`] surface the appropriate
/// `Option`-returning view.
#[derive(Clone)]
enum ErrorContext {
    /// No wire response was received (transport failure, client
    /// validation, configuration error, end-to-end timeout, …).
    /// Diagnostics may be attached by the pipeline.
    Synthetic {
        diagnostics: Option<Arc<DiagnosticsContext>>,
    },
    /// Wire data (body + parsed headers) was captured during a Cosmos
    /// response attempt **before** the operation's
    /// `DiagnosticsContextBuilder` was finalized. Internal-only — the
    /// public [`CosmosError::response`] accessor returns `None` for this
    /// variant, so an accidental leak would surface as a Synthetic-like
    /// error externally. The operation pipeline promotes this to `Wire`
    /// at the abort branch by calling
    /// `CosmosErrorBuilder::from_error(err).with_diagnostics(d).build()`
    /// once `DiagnosticsContextBuilder::complete()` has produced a
    /// finalized [`DiagnosticsContext`]. Status lives on the outer
    /// [`CosmosErrorInner`].
    WirePending { payload: Box<CosmosResponsePayload> },
    /// Wire response fully assembled with finalized diagnostics. The
    /// only variant `response()` exposes externally.
    Wire { response: Box<CosmosResponse> },
}

impl CosmosError {
    fn from_inner(mut inner: CosmosErrorInner) -> Self {
        if inner.backtrace.is_none() {
            // If we are wrapping another Cosmos `CosmosError` as the source
            // (status-changing re-wrap, e.g. promoting a service error to a
            // transport error), inherit that error's backtrace instead of
            // paying for a fresh capture at the wrap site. The wrap site is
            // always the same handful of lines in the pipeline and adds no
            // diagnostic value over the originating call stack — inheriting
            // also saves one capture-throttle token per re-wrap, doubling
            // the effective capture budget on retry-heavy paths.
            if let Some(src) = inner.source.as_deref() {
                let src_dyn: &(dyn StdError + 'static) = src;
                if let Some(inner_cosmos) = src_dyn.downcast_ref::<CosmosError>() {
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
    // Public accessors
    // -----------------------------------------------------------------

    /// Returns the typed Cosmos status (HTTP status code + optional
    /// sub-status) associated with this error. Always present — non-service
    /// errors carry a synthetic status with a placeholder HTTP code (e.g.
    /// [`CosmosStatus::TRANSPORT_GENERATED_503`] for transport failures,
    /// [`CosmosStatus::CLIENT_GENERATED_401`] for authorization failures).
    ///
    /// When [`response()`](Self::response) is `Some`, this is guaranteed
    /// to equal `response().status()` (the builder reconciles them at
    /// `build()` time).
    pub fn status(&self) -> CosmosStatus {
        self.inner.status
    }

    /// Returns the originating [`CosmosResponse`] when a wire response was
    /// received and fully assembled with finalized diagnostics (service
    /// errors past the per-operation finalization point). Returns `None`
    /// for synthetic errors (transport, client, configuration, …) and
    /// for the internal pre-finalization staging state.
    ///
    /// When `Some`, the response carries the body, the parsed Cosmos
    /// response headers, the status, and the operation diagnostics
    /// together. Access them as `response.body()`, `response.headers()`,
    /// `response.status()`, and `response.diagnostics()` respectively.
    pub fn response(&self) -> Option<&CosmosResponse> {
        match &self.inner.context {
            ErrorContext::Wire { response } => Some(response),
            ErrorContext::WirePending { .. } | ErrorContext::Synthetic { .. } => None,
        }
    }

    /// Returns `true` if this error originated from a wire response from
    /// the service (either fully finalized [`Wire`](ErrorContext::Wire) or
    /// the pre-finalization [`WirePending`](ErrorContext::WirePending)
    /// staging state). Returns `false` for purely synthetic errors
    /// (transport failures, client validation, configuration, …) which
    /// have no associated server response.
    pub fn is_from_wire(&self) -> bool {
        matches!(
            &self.inner.context,
            ErrorContext::Wire { .. } | ErrorContext::WirePending { .. }
        )
    }

    /// Returns the diagnostics context for the failed operation.
    ///
    /// For wire-response errors (`Wire` variant), this returns the
    /// diagnostics owned by [`response()`](Self::response). For synthetic
    /// errors, this returns whatever the pipeline attached via
    /// [`CosmosErrorBuilder::with_diagnostics`] (typically late, when the
    /// operation pipeline finalizes diagnostics around an aborted
    /// transport call); `None` when no diagnostics were attached.
    pub fn diagnostics(&self) -> Option<&Arc<DiagnosticsContext>> {
        match &self.inner.context {
            ErrorContext::Wire { response } => Some(response.diagnostics_ref()),
            ErrorContext::WirePending { .. } => None,
            ErrorContext::Synthetic { diagnostics } => diagnostics.as_ref(),
        }
    }

    /// Returns the stack backtrace captured at error construction time,
    /// rendered as a human-readable string.
    ///
    /// Backtrace capture is **opt-in** (matching idiomatic Rust): off by
    /// default, on whenever the stdlib `RUST_BACKTRACE` environment
    /// variable is set, and always overridable via the runtime builder.
    /// When enabled, capture is bounded by two production-safety gates
    /// (resolution-rate limiter + per-second capture throttle, both
    /// rolling 1-second windows). Cache hits do **not** consume budget,
    /// so backtraces whose frames are already known render at full
    /// fidelity regardless of limiter state.
    ///
    /// Returns `None` when:
    /// * Capture was disabled at construction time (`RUST_BACKTRACE`
    ///   unset and no explicit capacity, or either limiter set to `0`),
    /// * the capture throttle was exhausted at construction time, or
    /// * the resolution limiter denied fresh resolution for at least one
    ///   cache-missed frame.
    ///
    /// Partial backtraces are never produced — callers either get a fully-
    /// resolved render or nothing. **The outcome of the first call is
    /// cached on this [`CosmosError`] instance**, so every subsequent call
    /// returns the same answer regardless of later changes in limiter or
    /// throttle state.
    ///
    /// ## What the backtrace points at
    ///
    /// * **Errors originating inside the Cosmos pipeline** (HTTP error
    ///   responses, end-to-end timeouts, internal validation failures)
    ///   resolve to the actual construction site.
    /// * **Errors wrapping another Cosmos [`CosmosError`]** as their source
    ///   inherit the inner error's backtrace, so the originating site is
    ///   still visible.
    /// * **Errors wrapping a third-party error** (e.g. credential or HMAC
    ///   failures) point at the explicit construction site in driver code,
    ///   not the originating failure site inside the third-party crate.
    ///   The typed [`CosmosStatus`] and
    ///   [`std::error::Error::source`] chain remain the primary diagnostic
    ///   signal in that case.
    ///
    /// ## Async caveat
    ///
    /// Stack capture records the **synchronous call stack at the
    /// construction site**, which in an `async` context is the current
    /// poll frame — typically `tokio runtime → poll → your_async_fn`,
    /// not the chain of `.await` ancestors that logically led there.
    /// This is a fundamental limitation of stack capture in async Rust.
    /// For the logical async call chain, use `tracing` spans wrapping
    /// the calling code.
    pub fn backtrace(&self) -> Option<&Arc<str>> {
        self.inner.backtrace.as_ref().and_then(Backtrace::rendered)
    }

    // -----------------------------------------------------------------
    // Crate-internal accessors (pub(crate)) — used by the operation
    // pipeline to read back staged wire parts on `WirePending` errors
    // and to peek at the per-attempt status / payload before diagnostics
    // finalization. Never exposed externally.
    // -----------------------------------------------------------------

    /// `pub(crate)`: returns the staged wire payload (body + parsed
    /// headers) for a `WirePending` error, or the wire payload of an
    /// already-assembled [`Wire`](ErrorContext::Wire) error. Returns
    /// `None` for `Synthetic` errors. Used by internal pipeline code
    /// that needs to inspect the wire body / headers regardless of
    /// whether diagnostics finalization has happened yet.
    pub(crate) fn wire_payload(&self) -> Option<&CosmosResponsePayload> {
        match &self.inner.context {
            ErrorContext::WirePending { payload } => Some(payload),
            ErrorContext::Wire { response } => Some(response.payload()),
            ErrorContext::Synthetic { .. } => None,
        }
    }
}

// -----------------------------------------------------------------
// Trait impls
// -----------------------------------------------------------------

impl fmt::Display for CosmosError {
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
    /// (if captured) the rendered backtrace.
    ///
    /// Structured fields (status, response, diagnostics, source chain,
    /// backtrace) are also reachable directly via the dedicated accessors
    /// on [`CosmosError`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_header(f, &self.inner)?;
        if f.alternate() {
            write_source_chain(f, self, /* debug */ false, /* alternate */ true)?;
            write_diagnostics(f, self, /* debug */ false, /* alternate */ true)?;
            write_backtrace(f, self)?;
        }
        Ok(())
    }
}

impl fmt::Debug for CosmosError {
    /// Default (`{e:?}`): structured header (kind + message + status) plus
    /// the source chain. The captured backtrace is **omitted** so that
    /// high-volume `tracing::error!(err = ?e)` / `Result::unwrap` /
    /// `assert_eq!` call sites do not emit multi-line stack frame blocks
    /// per error.
    ///
    /// Alternate (`{e:#?}`): same as default plus the rendered backtrace
    /// block — opt in for full diagnostic reports.
    ///
    /// Callers that always want the backtrace regardless of format flag
    /// should read it explicitly via [`CosmosError::backtrace`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let alternate = f.alternate();
        write_header(f, &self.inner)?;
        write_source_chain(f, self, /* debug */ true, alternate)?;
        write_diagnostics(f, self, /* debug */ true, alternate)?;
        if alternate {
            write_backtrace(f, self)?;
        }
        Ok(())
    }
}

fn write_header(f: &mut fmt::Formatter<'_>, inner: &CosmosErrorInner) -> fmt::Result {
    // `CosmosStatus::Display` already renders the categorical `[Kind]`
    // plus `<status>/<sub> (<name>)` (or `<status>` when no sub-status),
    // so reuse it for a single, consistent representation.
    write!(f, "{}: {}", inner.status, inner.message)
}

/// Writes the `source()` chain. When `debug` is true, each entry is
/// rendered with `{:?}` so that wrapped errors carrying structured state
/// (e.g. another Cosmos [`CosmosError`], `io::Error`, `h2::Error`) surface
/// their full debug representation rather than a one-line `Display`
/// summary.
fn write_source_chain(
    f: &mut fmt::Formatter<'_>,
    err: &CosmosError,
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
        // error.
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

/// Appends the [`DiagnosticsContext`] (when present). Sourced via
/// [`CosmosError::diagnostics`] so the wire-response vs. synthetic
/// distinction is transparent to formatting.
fn write_diagnostics(
    f: &mut fmt::Formatter<'_>,
    err: &CosmosError,
    debug: bool,
    alternate: bool,
) -> fmt::Result {
    let Some(diag) = err.diagnostics() else {
        return Ok(());
    };
    let diag = diag.as_ref();
    f.write_str("\n\nDiagnostics:\n")?;
    match (debug, alternate) {
        (true, true) => write!(f, "{diag:#?}"),
        (true, false) => write!(f, "{diag:?}"),
        (false, true) => write!(f, "{diag:#}"),
        (false, false) => write!(f, "{diag}"),
    }
}

fn write_backtrace(f: &mut fmt::Formatter<'_>, err: &CosmosError) -> fmt::Result {
    if let Some(bt) = err.backtrace() {
        f.write_str("\n\nStack backtrace:\n")?;
        f.write_str(bt.as_ref())?;
    }
    Ok(())
}

impl StdError for CosmosError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.inner
            .source
            .as_deref()
            .map(|s| s as &(dyn StdError + 'static))
    }
}

/// Maximum number of `.source()` frames walked when rendering a
/// [`CosmosError`] via [`fmt::Display`] / [`fmt::Debug`]. Generous
/// relative to real Cosmos transport chains (~5 frames) but bounded so a
/// pathological or cyclic chain cannot pin a thread formatting an error.
const MAX_SOURCE_CHAIN_DEPTH: usize = 64;

/// Driver-wide `Result` alias.
pub type Result<T> = std::result::Result<T, CosmosError>;

// =========================================================================
// CosmosErrorBuilder
// =========================================================================

impl CosmosError {
    /// Returns a fluent [`CosmosErrorBuilder`] seeded with sensible
    /// defaults (a synthetic `500 InternalServerError` status). Callers
    /// typically follow with [`.with_status(...)`](CosmosErrorBuilder::with_status)
    /// to set the appropriate typed status — the well-known
    /// [`CosmosStatus`] constants ([`TRANSPORT_GENERATED_503`](CosmosStatus::TRANSPORT_GENERATED_503),
    /// [`AUTHENTICATION_TOKEN_ACQUISITION_FAILED`](CosmosStatus::AUTHENTICATION_TOKEN_ACQUISITION_FAILED),
    /// [`SERIALIZATION_RESPONSE_BODY_INVALID`](CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID),
    /// [`CLIENT_GENERATED_401`](CosmosStatus::CLIENT_GENERATED_401), etc.)
    /// cover the common synthetic cases; for service errors received from
    /// the wire, use [`.with_response(...)`](CosmosErrorBuilder::with_response).
    ///
    /// ```
    /// use azure_data_cosmos_driver::error::{CosmosError, CosmosStatus};
    /// use azure_core::http::StatusCode;
    ///
    /// let err = CosmosError::builder()
    ///     .with_status(CosmosStatus::new(StatusCode::BadRequest))
    ///     .with_message("missing partition key")
    ///     .build();
    /// assert_eq!(err.status().status_code(), StatusCode::BadRequest);
    /// ```
    pub fn builder() -> CosmosErrorBuilder {
        CosmosErrorBuilder::new()
    }
}

/// Fluent builder for [`CosmosError`]. The only way to construct or
/// re-decorate a Cosmos [`CosmosError`].
///
/// Obtain one via [`CosmosError::builder(kind)`](CosmosError::builder) to
/// start fresh, or [`CosmosErrorBuilder::from_error`] to patch an existing
/// error (add context, swap status, attach diagnostics, etc.). Finalize
/// with [`build()`](Self::build).
///
/// # Invariants enforced at `build()`
///
/// When [`with_response`](Self::with_response) was called on the builder,
/// the resulting [`CosmosError`] is reconciled so that the [`CosmosResponse`]
/// is the source of truth ("**CosmosResponse wins**"):
///
/// * The error's [`CosmosError::status`] is overwritten with
///   `response.status()`.
/// * The error's [`CosmosError::diagnostics`] is sourced from
///   `response.diagnostics()`. Any value supplied via
///   [`with_diagnostics`](Self::with_diagnostics) in the same chain is
///   silently discarded.
///
/// When the builder carries [`WirePending`](ErrorContext::WirePending)
/// staging (via [`with_response_parts`](Self::with_response_parts), an
/// internal-only setter) and a [`with_diagnostics`](Self::with_diagnostics)
/// is supplied — typically via the operation pipeline's
/// `from_error(err).with_diagnostics(d).build()` finalization — the
/// builder **promotes** the error to a fully assembled
/// [`Wire`](ErrorContext::Wire) variant by constructing a
/// [`CosmosResponse`] from the staged body + headers + status + the
/// supplied diagnostics.
///
/// These overrides are silent (no panic) by design — they let pipeline
/// code attach a wire response unconditionally without first having to
/// reset other builder fields.
///
/// ```
/// use std::sync::Arc;
/// use azure_data_cosmos_driver::error::{CosmosError, CosmosErrorBuilder, CosmosStatus};
/// use azure_core::http::StatusCode;
///
/// let inner = CosmosError::builder()
///     .with_status(CosmosStatus::new(StatusCode::BadRequest))
///     .with_message("bad payload")
///     .build();
/// let outer = CosmosErrorBuilder::from_error(inner)
///     .with_context("uploadItem(id=42)")
///     .build();
/// assert!(format!("{outer}").contains("uploadItem(id=42): bad payload"));
/// ```
#[must_use = "CosmosErrorBuilder is inert until `.build()` is called"]
pub struct CosmosErrorBuilder {
    /// When `Some`, build clones this error's inner state and patches the
    /// overridden fields. When `None`, build constructs a fresh error
    /// with a synthetic `500 InternalServerError` status.
    base: Option<CosmosError>,
    /// Override status. Ignored if `response` is set ("CosmosResponse
    /// wins"); otherwise falls back to the base error's status or the
    /// synthetic 500 default.
    status: Option<CosmosStatus>,
    /// Wire-level response captured by the pipeline. When set, its status
    /// and diagnostics become authoritative; the builder produces
    /// [`ErrorContext::Wire`].
    response: Option<CosmosResponse>,
    /// Internal-only: staged wire payload captured before the operation's
    /// diagnostics builder was finalized. When set without `response`
    /// **and without** `diagnostics`, the builder produces
    /// [`ErrorContext::WirePending`]. When set together with
    /// `diagnostics`, the builder **promotes** to [`ErrorContext::Wire`]
    /// by assembling a [`CosmosResponse`] from the staged parts + the
    /// supplied diagnostics + the resolved status.
    response_parts: Option<Box<CosmosResponsePayload>>,
    /// Standalone diagnostics. Ignored if `response` is set (the
    /// response carries its own); used to promote `WirePending` to
    /// `Wire`, or attached as the synthetic diagnostics slot.
    diagnostics: Option<Arc<DiagnosticsContext>>,
    message: Option<Arc<str>>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Prepended to the final message as `"{context}: {message}"` when set.
    context_prefix: Option<Arc<str>>,
}

impl CosmosErrorBuilder {
    fn new() -> Self {
        Self {
            base: None,
            status: None,
            response: None,
            response_parts: None,
            diagnostics: None,
            message: None,
            source: None,
            context_prefix: None,
        }
    }

    /// Starts a builder pre-populated from an existing [`CosmosError`]. Any
    /// subsequent setter overrides the corresponding field; unset fields
    /// are carried forward from `err`. Useful for re-decorating an error
    /// returned from a deeper layer — attaching operation context,
    /// swapping status, or — most importantly — finalizing a
    /// [`WirePending`](ErrorContext::WirePending) error into a `Wire` one
    /// via [`with_diagnostics`](Self::with_diagnostics).
    pub fn from_error(err: CosmosError) -> Self {
        Self {
            base: Some(err),
            status: None,
            response: None,
            response_parts: None,
            diagnostics: None,
            message: None,
            source: None,
            context_prefix: None,
        }
    }

    /// Overrides the [`CosmosStatus`].
    ///
    /// **Ignored if [`with_response`](Self::with_response) was also
    /// called** — the [`CosmosResponse`]'s status wins.
    pub fn with_status(mut self, status: CosmosStatus) -> Self {
        self.status = Some(status);
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
    /// [`CosmosError`] as the source). For plain `StdError` values prefer
    /// [`with_source`](Self::with_source).
    pub fn with_arc_source(mut self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self {
        self.source = Some(source);
        self
    }

    /// Attaches the wire-level [`CosmosResponse`] that produced this error.
    /// The response carries the body, parsed Cosmos response headers,
    /// typed status, and operation diagnostics together — by design, the
    /// [`CosmosResponse`] becomes the source of truth at
    /// [`build()`](Self::build):
    ///
    /// * [`CosmosError::status`] is overwritten with `response.status()`.
    /// * [`CosmosError::diagnostics`] flows through `response.diagnostics()`.
    /// * Any prior [`with_status`](Self::with_status) /
    ///   [`with_diagnostics`](Self::with_diagnostics) values in the same
    ///   chain are silently discarded.
    pub fn with_response(mut self, response: CosmosResponse) -> Self {
        self.response = Some(response);
        self
    }

    /// Attaches a standalone operation [`DiagnosticsContext`].
    ///
    /// * **Ignored if [`with_response`](Self::with_response) was also
    ///   called** — diagnostics then flow through `response.diagnostics()`.
    /// * **Promotes a [`WirePending`](ErrorContext::WirePending) base
    ///   error to a [`Wire`](ErrorContext::Wire) one** when chained via
    ///   [`from_error`](Self::from_error): the staged body + headers
    ///   carried by the base error are assembled with the supplied
    ///   diagnostics and the resolved status into a [`CosmosResponse`].
    ///   This is the operation pipeline's per-operation finalization
    ///   path.
    pub fn with_diagnostics(mut self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    /// Prepends operational context to the final message as
    /// `"{context}: {message}"`. Repeated calls override (the most recent
    /// context wins); chain multiple `with_context` calls into one
    /// combined string at the call site if multiple layers of context are
    /// needed.
    pub fn with_context(mut self, context: impl Into<Arc<str>>) -> Self {
        self.context_prefix = Some(context.into());
        self
    }

    /// **Internal-only.** Stages a wire payload (body + parsed headers)
    /// captured during a Cosmos response attempt **before** the
    /// operation's `DiagnosticsContextBuilder` was finalized. At
    /// [`build()`](Self::build) the resulting error becomes either:
    ///
    /// * [`WirePending`](ErrorContext::WirePending) when no
    ///   [`with_diagnostics`](Self::with_diagnostics) was supplied — the
    ///   per-attempt state the operation pipeline carries between
    ///   retries; or
    /// * [`Wire`](ErrorContext::Wire) when diagnostics is supplied — the
    ///   per-attempt staging is promoted by assembling a
    ///   [`CosmosResponse`] from the staged parts + the resolved status +
    ///   the supplied diagnostics. This is the finalization performed by
    ///   the operation pipeline's abort branch.
    ///
    /// **Ignored if [`with_response`](Self::with_response) was also
    /// called** — the full [`CosmosResponse`] supersedes the staged parts.
    pub(crate) fn with_response_parts(mut self, payload: CosmosResponsePayload) -> Self {
        self.response_parts = Some(Box::new(payload));
        self
    }

    /// Finalizes the builder into a [`CosmosError`]. Allocation-cheap
    /// (single `Arc<CosmosErrorInner>` regardless of which fields were
    /// set). See the type-level docs for the reconciliation rules.
    pub fn build(self) -> CosmosError {
        // Resolve the effective status before deciding the context, since
        // `WirePending` and `Synthetic` both need it stored on the outer
        // inner and `Wire` overrides it from the response.
        let base_status = self.base.as_ref().map(|b| b.inner.status);
        let resolved_status = self.status.or(base_status).unwrap_or_else(|| {
            CosmosStatus::new(azure_core::http::StatusCode::InternalServerError)
        });

        // Pull base context (if any) to support carry-forward of
        // WirePending staging through `from_error(...).build()` without
        // any setter, and to inherit synthetic diagnostics.
        let base_context = self.base.as_ref().map(|b| &b.inner.context);

        // Compute (status, context) according to the locked rules:
        //  1. `with_response`               -> Wire (CosmosResponse wins)
        //  2. `with_response_parts`         -> Wire (if diagnostics also set) or WirePending
        //  3. base = WirePending + `with_diagnostics` (no setters) -> promote to Wire
        //  4. base = Wire + `with_diagnostics`                     -> Wire (response's diag is the truth; user diag ignored)
        //  5. else                                                  -> Synthetic
        let (status, context) = if let Some(response) = self.response {
            // (1) Full response supplied; it wins.
            let status = response.status();
            (
                status,
                ErrorContext::Wire {
                    response: Box::new(response),
                },
            )
        } else if let Some(parts) = self.response_parts {
            // (2) Staged parts supplied on this builder.
            match self.diagnostics {
                Some(diag) => {
                    // Promotion: assemble a CosmosResponse and become Wire.
                    let payload = *parts;
                    let response = finalize_response(payload, resolved_status, diag);
                    let status = response.status();
                    (
                        status,
                        ErrorContext::Wire {
                            response: Box::new(response),
                        },
                    )
                }
                None => (
                    resolved_status,
                    ErrorContext::WirePending { payload: parts },
                ),
            }
        } else {
            // No setter on this builder for response or staged parts —
            // consult the base error.
            match base_context {
                Some(ErrorContext::WirePending { payload }) => match self.diagnostics {
                    Some(diag) => {
                        // (3) Promote: assemble a CosmosResponse and become Wire.
                        let payload = (**payload).clone();
                        let response = finalize_response(payload, resolved_status, diag);
                        let status = response.status();
                        (
                            status,
                            ErrorContext::Wire {
                                response: Box::new(response),
                            },
                        )
                    }
                    None => {
                        // Carry WirePending staging forward unchanged.
                        let payload = (**payload).clone();
                        (
                            resolved_status,
                            ErrorContext::WirePending {
                                payload: Box::new(payload),
                            },
                        )
                    }
                },
                Some(ErrorContext::Wire { response }) => {
                    // (4) Base already Wire. Carry the response forward
                    // verbatim — its diagnostics is the truth; any
                    // `with_diagnostics` on this builder is discarded by
                    // the "CosmosResponse wins" rule.
                    let response = (**response).clone();
                    let status = response.status();
                    (
                        status,
                        ErrorContext::Wire {
                            response: Box::new(response),
                        },
                    )
                }
                Some(ErrorContext::Synthetic {
                    diagnostics: base_diag,
                }) => {
                    // (5a) Synthetic base — explicit `with_diagnostics`
                    // overrides, else inherit base's.
                    let diagnostics = self.diagnostics.or_else(|| base_diag.clone());
                    (resolved_status, ErrorContext::Synthetic { diagnostics })
                }
                None => {
                    // (5b) No base — pure new synthetic error.
                    (
                        resolved_status,
                        ErrorContext::Synthetic {
                            diagnostics: self.diagnostics,
                        },
                    )
                }
            }
        };

        // Carry forward message / source / backtrace from the base, then
        // apply any overrides supplied on this builder.
        let (mut message, mut source, backtrace) = match &self.base {
            Some(base) => (
                Arc::clone(&base.inner.message),
                base.inner.source.clone(),
                base.inner.backtrace.clone(),
            ),
            None => (Arc::<str>::from(""), None, None),
        };
        if let Some(m) = self.message {
            message = m;
        }
        if self.source.is_some() {
            source = self.source;
        }
        if let Some(prefix) = self.context_prefix {
            let mut buf = String::with_capacity(prefix.len() + 2 + message.len());
            buf.push_str(&prefix);
            buf.push_str(": ");
            buf.push_str(&message);
            message = Arc::<str>::from(buf);
        }

        CosmosError::from_inner(CosmosErrorInner {
            status,
            context,
            message,
            source,
            backtrace,
        })
    }
}

/// Assembles a finalized [`CosmosResponse`] from staged wire parts +
/// resolved status + finalized diagnostics. Used by the `WirePending` →
/// `Wire` promotion path inside [`CosmosErrorBuilder::build`].
fn finalize_response(
    payload: CosmosResponsePayload,
    status: CosmosStatus,
    diagnostics: Arc<DiagnosticsContext>,
) -> CosmosResponse {
    let (body, headers) = (payload.body().clone(), payload.headers().clone());
    CosmosResponse::new(body, headers, status, diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{CosmosResponseHeaders, ResponseBody};
    use azure_core::http::StatusCode;

    // -----------------------------------------------------------------
    // Test fixtures
    // -----------------------------------------------------------------

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

    fn make_test_response(
        status: CosmosStatus,
        diagnostics: Arc<DiagnosticsContext>,
    ) -> CosmosResponse {
        CosmosResponse::new(
            ResponseBody::NoPayload,
            CosmosResponseHeaders::default(),
            status,
            diagnostics,
        )
    }

    fn make_test_payload() -> CosmosResponsePayload {
        CosmosResponsePayload::new(b"{\"x\":1}".to_vec(), CosmosResponseHeaders::default())
    }

    // -----------------------------------------------------------------
    // Public CosmosErrorBuilder surface
    // -----------------------------------------------------------------

    #[test]
    fn builder_default_status_is_internal_server_error() {
        let err = CosmosError::builder().with_message("m").build();
        assert_eq!(err.status().status_code(), StatusCode::InternalServerError);
        assert_eq!(format!("{err}").split(": ").last().unwrap(), "m");
        assert!(err.response().is_none());
    }

    #[test]
    fn builder_with_status_is_preserved_verbatim() {
        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::ServiceUnavailable))
            .with_message("nope")
            .build();
        assert_eq!(err.status().status_code(), StatusCode::ServiceUnavailable);
    }

    #[test]
    fn builder_with_source_preserves_via_std_error_source() {
        let io = std::io::Error::new(std::io::ErrorKind::Other, "underlying");
        let err = CosmosError::builder()
            .with_message("wrapped")
            .with_source(io)
            .build();
        let src = StdError::source(&err).expect("source preserved");
        assert!(src.to_string().contains("underlying"));
    }

    #[test]
    fn builder_with_arc_source_accepts_shared_handle() {
        let inner = Arc::new(CosmosError::builder().with_message("inner").build())
            as Arc<dyn StdError + Send + Sync + 'static>;
        let outer = CosmosError::builder()
            .with_arc_source(inner)
            .with_message("outer")
            .build();
        let src = StdError::source(&outer).expect("source preserved");
        assert!(src.to_string().contains("inner"));
    }

    #[test]
    fn builder_with_diagnostics_attaches_to_synthetic_error() {
        let diag = make_test_diagnostics();
        let err = CosmosError::builder()
            .with_message("m")
            .with_diagnostics(Arc::clone(&diag))
            .build();
        assert!(err.response().is_none());
        assert!(Arc::ptr_eq(err.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_with_response_sets_wire_context_and_wins_status_and_diagnostics() {
        let resp_diag = make_test_diagnostics();
        let response = make_test_response(
            CosmosStatus::new(StatusCode::NotFound),
            Arc::clone(&resp_diag),
        );
        let unrelated_diag = make_test_diagnostics();

        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::TooManyRequests)) // discarded
            .with_diagnostics(Arc::clone(&unrelated_diag)) // discarded
            .with_response(response)
            .with_message("oh")
            .build();

        assert_eq!(err.status().status_code(), StatusCode::NotFound);
        assert!(Arc::ptr_eq(err.diagnostics().unwrap(), &resp_diag));
        assert!(!Arc::ptr_eq(err.diagnostics().unwrap(), &unrelated_diag));
        let wire = err.response().expect("wire response present");
        assert_eq!(wire.status().status_code(), StatusCode::NotFound);
    }

    #[test]
    fn builder_with_response_invariant_chain_holds() {
        let response = make_test_response(
            CosmosStatus::new(StatusCode::Conflict),
            make_test_diagnostics(),
        );
        let err = CosmosError::builder()
            .with_response(response)
            .with_message("conflict")
            .build();

        let s_err = err.status().status_code();
        let s_resp = err.response().unwrap().status().status_code();
        // DiagnosticsContext::status is `Option<&CosmosStatus>` (set by the
        // pipeline at operation completion); whenever it is set, the
        // `CosmosResponse` construction invariant guarantees it equals
        // `response.status()`. The test fixture above does not set it.
        let s_resp_diag = err
            .response()
            .unwrap()
            .diagnostics_ref()
            .status()
            .map(|s| s.status_code());
        assert_eq!(s_err, s_resp);
        if let Some(s) = s_resp_diag {
            assert_eq!(s_resp, s);
        }
    }

    #[test]
    fn builder_with_response_parts_no_diagnostics_yields_wire_pending() {
        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::TooManyRequests))
            .with_message("staged")
            .with_response_parts(make_test_payload())
            .build();

        // Externally visible: WirePending presents as no response and no diagnostics.
        assert!(
            err.response().is_none(),
            "WirePending must not expose response()"
        );
        assert!(
            err.diagnostics().is_none(),
            "WirePending must not expose diagnostics()"
        );
        // Status was supplied on the builder and is preserved.
        assert_eq!(err.status().status_code(), StatusCode::TooManyRequests);
        // Internal pub(crate) accessor sees the staged payload.
        assert!(
            err.wire_payload().is_some(),
            "internal wire_payload must surface staged parts"
        );
    }

    #[test]
    fn builder_with_response_parts_and_diagnostics_promotes_to_wire() {
        let diag = make_test_diagnostics();
        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::NotFound))
            .with_message("not found")
            .with_response_parts(make_test_payload())
            .with_diagnostics(Arc::clone(&diag))
            .build();

        // Promotion: a Wire context with the assembled response is produced.
        let wire = err.response().expect("promotion to Wire");
        assert_eq!(wire.status().status_code(), StatusCode::NotFound);
        assert!(Arc::ptr_eq(err.diagnostics().unwrap(), &diag));
        assert!(Arc::ptr_eq(wire.diagnostics_ref(), &diag));
    }

    #[test]
    fn from_error_wire_pending_with_diagnostics_promotes_to_wire() {
        // Simulate the operation pipeline finalization path:
        //   1. per-attempt: build WirePending error (no diagnostics yet)
        //   2. abort: from_error(err).with_diagnostics(real_diag).build()
        let staged = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::ServiceUnavailable))
            .with_message("attempt-failed")
            .with_response_parts(make_test_payload())
            .build();
        assert!(staged.response().is_none(), "staged must be WirePending");

        let diag = make_test_diagnostics();
        let finalized = CosmosErrorBuilder::from_error(staged)
            .with_diagnostics(Arc::clone(&diag))
            .build();

        let wire = finalized.response().expect("finalization promoted to Wire");
        assert_eq!(wire.status().status_code(), StatusCode::ServiceUnavailable);
        assert!(Arc::ptr_eq(finalized.diagnostics().unwrap(), &diag));
        assert!(Arc::ptr_eq(wire.diagnostics_ref(), &diag));
    }

    #[test]
    fn from_error_wire_pending_without_diagnostics_carries_forward() {
        // from_error(WirePending) with only a context decoration must
        // preserve the WirePending state — promotion only happens when
        // diagnostics is supplied.
        let staged = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::ServiceUnavailable))
            .with_message("attempt-failed")
            .with_response_parts(make_test_payload())
            .build();

        let decorated = CosmosErrorBuilder::from_error(staged)
            .with_context("op=createItem")
            .build();

        assert!(decorated.response().is_none(), "WirePending preserved");
        assert!(decorated.diagnostics().is_none());
        assert!(decorated.wire_payload().is_some());
        assert!(format!("{decorated}").contains("op=createItem"));
    }

    #[test]
    fn from_error_wire_carries_response_forward() {
        let diag = make_test_diagnostics();
        let response =
            make_test_response(CosmosStatus::new(StatusCode::Conflict), Arc::clone(&diag));
        let original = CosmosError::builder()
            .with_response(response)
            .with_message("conflict")
            .build();

        let decorated = CosmosErrorBuilder::from_error(original)
            .with_context("op=replace")
            .build();

        let wire = decorated.response().expect("Wire carried forward");
        assert_eq!(wire.status().status_code(), StatusCode::Conflict);
        assert!(Arc::ptr_eq(decorated.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_with_context_prepends_to_message() {
        let err = CosmosError::builder()
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
        let original = CosmosError::builder()
            .with_message("first")
            .with_diagnostics(Arc::clone(&diag))
            .build();

        let cloned = CosmosErrorBuilder::from_error(original.clone()).build();
        assert_eq!(
            cloned.status().status_code(),
            original.status().status_code()
        );
        assert_eq!(format!("{cloned}"), format!("{original}"));
        assert!(Arc::ptr_eq(cloned.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_message_setter_overrides_base_message() {
        let original = CosmosError::builder().with_message("orig").build();
        let patched = CosmosErrorBuilder::from_error(original)
            .with_message("replaced")
            .build();
        assert!(format!("{patched}").ends_with(": replaced"));
    }

    #[test]
    fn builder_repeated_setters_last_write_wins() {
        let err = CosmosError::builder()
            .with_message("first")
            .with_message("second")
            .with_context("ctx-a")
            .with_context("ctx-b")
            .build();
        let rendered = format!("{err}");
        assert!(rendered.ends_with(": ctx-b: second"), "got: {rendered}");
    }

    #[test]
    fn end_to_end_timeout_uses_synthetic_status() {
        let err = CosmosError::builder()
            .with_status(CosmosStatus::from_parts(
                StatusCode::RequestTimeout,
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
            ))
            .with_message("e2e timeout")
            .build();
        assert_eq!(err.status().status_code(), StatusCode::RequestTimeout);
        assert_eq!(
            err.status().sub_status(),
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
        assert!(err.status().is_timeout());
        assert!(err.status().is_transient());
        assert!(err.response().is_none());
    }

    fn end_to_end_timeout_error(message: &'static str) -> CosmosError {
        CosmosError::builder()
            .with_status(CosmosStatus::from_parts(
                StatusCode::RequestTimeout,
                Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
            ))
            .with_message(message)
            .build()
    }

    #[test]
    fn wrap_inherits_backtrace_from_cosmos_source() {
        // Capture is opt-in; enable it for this test so the inheritance
        // check is actually meaningful.
        crate::error::backtrace::global_capture_throttle().set_capacity(1000);
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

        let outer = CosmosError::builder()
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

    /// Builds a [`CosmosError`] carrying both a `DiagnosticsContext` and
    /// a nested Cosmos `CosmosError` as its source, so format tests can
    /// exercise the source-chain + diagnostics propagation paths
    /// together.
    fn make_error_with_diagnostics_and_source() -> CosmosError {
        let inner = end_to_end_timeout_error("inner timeout");
        CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("outer transport failure")
            .with_diagnostics(make_test_diagnostics())
            .with_arc_source(Arc::new(inner))
            .build()
    }

    #[test]
    fn from_error_with_diagnostics_does_not_mutate_original() {
        let original = end_to_end_timeout_error("no diags");
        assert!(original.diagnostics().is_none());

        let diag = make_test_diagnostics();
        let attached = CosmosErrorBuilder::from_error(original.clone())
            .with_diagnostics(Arc::clone(&diag))
            .build();

        assert!(
            Arc::ptr_eq(attached.diagnostics().expect("diagnostics attached"), &diag),
            "builder must store the supplied diagnostics Arc verbatim"
        );
        assert!(
            original.diagnostics().is_none(),
            "original must be untouched by CosmosErrorBuilder::from_error"
        );
        assert_eq!(
            attached.status().status_code(),
            original.status().status_code()
        );
    }

    #[test]
    fn display_plain_includes_typed_header_and_message_on_one_line() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err}");
        assert!(
            !rendered.contains('\n'),
            "plain display must stay on one line, got:\n{rendered}"
        );
        assert!(
            rendered.contains("503"),
            "plain display must include the status, got:\n{rendered}"
        );
        assert!(
            rendered.ends_with(": outer transport failure"),
            "plain display must end with `: <message>`, got:\n{rendered}"
        );
        assert!(!rendered.contains("Caused by:"));
        assert!(!rendered.contains("Diagnostics:"));
    }

    #[test]
    fn display_alternate_includes_header_source_chain_and_diagnostics() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:#}");
        assert!(rendered.contains("503"));
        assert!(rendered.contains("outer transport failure"));
        assert!(rendered.contains("Caused by:") && rendered.contains("inner timeout"));
        assert!(rendered.contains("Diagnostics:"));
    }

    #[test]
    fn debug_omits_backtrace_block_in_plain_form() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:?}");
        assert!(!rendered.contains("Stack backtrace:"));
        assert!(rendered.contains("outer transport failure"));
        assert!(rendered.contains("Caused by:"));
    }

    #[test]
    fn debug_alternate_propagates_to_source_and_diagnostics() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:#?}");
        assert!(rendered.contains("outer transport failure"));
        assert!(rendered.contains("Caused by:"));
    }

    #[test]
    fn source_chain_truncation_caps_pathological_chains() {
        #[derive(Debug)]
        struct CyclicError;
        impl std::fmt::Display for CyclicError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("cyclic")
            }
        }
        impl StdError for CyclicError {
            fn source(&self) -> Option<&(dyn StdError + 'static)> {
                static SELF: CyclicError = CyclicError;
                Some(&SELF)
            }
        }

        let err = CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
            .with_message("outer")
            .with_arc_source(Arc::new(CyclicError))
            .build();

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
