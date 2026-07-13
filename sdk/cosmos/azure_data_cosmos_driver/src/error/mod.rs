// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cSpell:ignore peekable disambiguator

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

use std::{borrow::Cow, error::Error as StdError, fmt, sync::Arc};

use crate::{
    diagnostics::DiagnosticsContext,
    models::{CosmosResponse, CosmosResponsePayload},
};

pub mod cosmos_status;
pub use cosmos_status::{CosmosStatus, SubStatusCode};

pub(crate) mod backtrace;
pub(crate) use backtrace::Backtrace;
pub use backtrace::{set_backtrace_options, BacktraceOptions};

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
/// * [`status()`](Self::status) always reflects the current
///   [`CosmosStatus`].
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
/// These invariants imply
/// `status() == response().status() == diagnostics().status()`
/// whenever each side is defined, since [`CosmosResponse`] itself
/// guarantees `response.status() == response.diagnostics().status()`.
#[derive(Clone)]
pub struct CosmosError {
    inner: Arc<CosmosErrorInner>,
}

#[derive(Clone)]
struct CosmosErrorInner {
    /// Cosmos status (HTTP status + sub-status). Always present, shared
    /// across all
    /// [`ErrorContext`] variants — for the `Wire` variant this is
    /// reconciled to match `response.status()` at `build()` time.
    status: CosmosStatus,
    /// Discriminates wire-response errors (carrying a full
    /// [`CosmosResponse`]) from synthetic errors (carrying at most a
    /// standalone [`DiagnosticsContext`]) and the internal
    /// pre-diagnostics-finalization `ErrorContext::WirePending` state.
    /// Modelled as an enum so the storage rules are enforced by the type
    /// system rather than by runtime convention.
    context: ErrorContext,
    /// Static literal (`Cow::Borrowed`) for fixed-string error messages,
    /// or an owned `String` (`Cow::Owned`) for messages that need to
    /// interpolate case-specific information. `Cow<'static, str>` keeps
    /// the literal-message path allocation-free while still allowing
    /// `format!`-built strings without an extra round-trip through
    /// `Arc::<str>::from`.
    message: Cow<'static, str>,
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
            // If we are wrapping another Cosmos `CosmosError` somewhere in
            // the source chain (status-changing re-wrap, e.g. promoting a
            // service error to a transport error, or a Cosmos error
            // re-imported through a third-party wrapper like
            // `azure_core::Error`), inherit that error's backtrace instead
            // of paying for a fresh capture at the wrap site. The wrap
            // site is always the same handful of lines in the pipeline
            // and adds no diagnostic value over the originating call
            // stack — inheriting also saves one capture-throttle token
            // per re-wrap, doubling the effective capture budget on
            // retry-heavy paths.
            //
            // The walk is bounded by [`MAX_BACKTRACE_INHERITANCE_DEPTH`]
            // so a pathological / cyclic `source()` chain cannot pin a
            // thread on the error-construction hot path. Typical
            // production chains are 1–2 deep; the cap leaves generous
            // headroom while staying O(depth) per construction.
            let mut cur: Option<&(dyn StdError + 'static)> =
                inner.source.as_deref().map(|s| s as _);
            for _ in 0..MAX_BACKTRACE_INHERITANCE_DEPTH {
                let Some(src) = cur else { break };
                if let Some(inner_cosmos) = src.downcast_ref::<CosmosError>() {
                    inner.backtrace = inner_cosmos.inner.backtrace.clone();
                    break;
                }
                cur = src.source();
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
    /// the service **that has been fully assembled with finalized
    /// diagnostics** — i.e. the same state the public
    /// [`response()`](Self::response) accessor exposes (`Some(_)`).
    ///
    /// Returns `false` for purely synthetic errors (transport failures,
    /// client validation, configuration, …) **and** for the internal
    /// pre-finalization `WirePending` staging state. Keeping this
    /// predicate in lockstep with [`response()`](Self::response) means
    /// external classifiers (notably the SDK boundary's
    /// `From<CosmosError> for azure_core::Error`) can rely on
    /// `is_from_wire() ⇔ response().is_some()` and never observe an
    /// `HttpResponse`-classified error with no payload reachable.
    pub fn is_from_wire(&self) -> bool {
        matches!(&self.inner.context, ErrorContext::Wire { .. })
    }

    /// Returns the diagnostics context for the failed operation.
    ///
    /// For wire-response errors (`Wire` variant), this returns the
    /// diagnostics owned by [`response()`](Self::response). For synthetic
    /// errors, this returns whatever the pipeline attached via
    /// [`CosmosErrorBuilder::with_diagnostics`] (typically late, when the
    /// operation pipeline finalizes diagnostics around an aborted
    /// transport call); `None` when no diagnostics were attached.
    pub fn diagnostics(&self) -> Option<Arc<DiagnosticsContext>> {
        match &self.inner.context {
            ErrorContext::Wire { response } => Some(response.diagnostics()),
            ErrorContext::WirePending { .. } => None,
            ErrorContext::Synthetic { diagnostics } => diagnostics.clone(),
        }
    }

    /// `pub(crate)`: borrowing version of [`diagnostics()`](Self::diagnostics)
    /// for internal hot paths that only need to read the diagnostics
    /// (e.g. formatting in `Display` / `Debug`, structural assertions
    /// in tests) and want to avoid the per-call `Arc` refcount bump.
    pub(crate) fn diagnostics_ref(&self) -> Option<&Arc<DiagnosticsContext>> {
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
    pub fn backtrace(&self) -> Option<Arc<str>> {
        self.inner
            .backtrace
            .as_ref()
            .and_then(Backtrace::rendered)
            .cloned()
    }

    // -----------------------------------------------------------------
    // Crate-internal accessors (pub(crate)) — used by the operation
    // pipeline to read back staged wire parts on `WirePending` errors
    // and to peek at the per-attempt status / payload before diagnostics
    // finalization. Never exposed externally.
    // -----------------------------------------------------------------

    /// `pub(crate)`: returns the staged wire payload (body + parsed
    /// headers) for a `WirePending` error, or the wire payload of an
    /// already-assembled `Wire` error. Returns
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
    /// Default (`{e}`): a single-line `status/sub (name): message` header
    /// (the status portion is rendered by [`CosmosStatus`]'s `Display`).
    /// This intentionally diverges from the `anyhow` / `azure_core`
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
    /// Default (`{e:?}`): structured header (status + message) plus
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
    // `CosmosStatus::Display` renders `<status>/<sub> (<name>)` (or
    // `<status>/<sub>` when the sub-status has no canonical name, or
    // just `<status>` when there is no sub-status), so reuse it for a
    // single, consistent representation.
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
    let Some(diag) = err.diagnostics_ref() else {
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

/// Maximum number of `.source()` frames walked by [`CosmosError::from_inner`]
/// looking for an inheritable [`CosmosError`] backtrace.
///
/// Picked low (4) because realistic Cosmos wrap chains are 1–2 deep — the
/// only motivating case for >1 is an indirect re-wrap through a
/// third-party error type (e.g. `azure_core::Error` wrapping a
/// `CosmosError` re-imported through a credential or policy boundary).
/// The bound keeps the hot error-construction path O(depth) and prevents
/// a pathological / cyclic chain from pinning a thread.
const MAX_BACKTRACE_INHERITANCE_DEPTH: usize = 4;

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
/// Obtain one via [`CosmosError::builder()`](CosmosError::builder) to
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
/// When the builder carries `WirePending`
/// staging (via `with_response_parts`, an
/// internal-only setter) and a [`with_diagnostics`](Self::with_diagnostics)
/// is supplied — typically via the operation pipeline's
/// `from_error(err).with_diagnostics(d).build()` finalization — the
/// builder **promotes** the error to a fully assembled
/// `Wire` variant by constructing a
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
    /// `ErrorContext::Wire`.
    response: Option<CosmosResponse>,
    /// Internal-only: staged wire payload captured before the operation's
    /// diagnostics builder was finalized. When set without `response`
    /// **and without** `diagnostics`, the builder produces
    /// `ErrorContext::WirePending`. When set together with
    /// `diagnostics`, the builder **promotes** to `ErrorContext::Wire`
    /// by assembling a [`CosmosResponse`] from the staged parts + the
    /// supplied diagnostics + the resolved status.
    response_parts: Option<Box<CosmosResponsePayload>>,
    /// Standalone diagnostics. Ignored if `response` is set (the
    /// response carries its own); used to promote `WirePending` to
    /// `Wire`, or attached as the synthetic diagnostics slot.
    diagnostics: Option<Arc<DiagnosticsContext>>,
    message: Option<Cow<'static, str>>,
    source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    /// Prepended to the final message as `"{context}: {message}"` when set.
    context_prefix: Option<Cow<'static, str>>,
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
    /// `WirePending` error into a `Wire` one
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

    /// Sets the human-readable error message. Accepts any
    /// `Into<Cow<'static, str>>` — string literals are stored as
    /// `Cow::Borrowed` (no allocation), `String` / `format!` results as
    /// `Cow::Owned`.
    pub fn with_message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
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
    ///   called on the same builder** — the freshly-supplied response's
    ///   own diagnostics is authoritative.
    /// * **Promotes a `WirePending` base error to a `Wire` one** when
    ///   chained via [`from_error`](Self::from_error): the staged body +
    ///   headers carried by the base error are assembled with the supplied
    ///   diagnostics and the resolved status into a [`CosmosResponse`].
    ///   This is the operation pipeline's per-operation finalization
    ///   path.
    /// * **Overrides the diagnostics on a `Wire` base error** when
    ///   chained via [`from_error`](Self::from_error): the base
    ///   response's body, headers, and status are preserved verbatim,
    ///   and a new [`CosmosResponse`] is assembled with the supplied
    ///   diagnostics in place of the original. This is the path
    ///   `patch_handler::exhaustion_error` uses to graft the aggregated
    ///   cross-attempt diagnostics onto a wrapped service 412, and the
    ///   path any future caller would use to re-decorate a wire error
    ///   with operation-level diagnostics.
    pub fn with_diagnostics(mut self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    /// Prepends operational context to the final message as
    /// `"{context}: {message}"`. Repeated calls override (the most recent
    /// context wins); chain multiple `with_context` calls into one
    /// combined string at the call site if multiple layers of context are
    /// needed. Accepts any `Into<Cow<'static, str>>`.
    pub fn with_context(mut self, context: impl Into<Cow<'static, str>>) -> Self {
        self.context_prefix = Some(context.into());
        self
    }

    /// **Internal-only.** Stages a wire payload (body + parsed headers)
    /// captured during a Cosmos response attempt **before** the
    /// operation's `DiagnosticsContextBuilder` was finalized. At
    /// [`build()`](Self::build) the resulting error becomes either:
    ///
    /// * `WirePending` when no
    ///   [`with_diagnostics`](Self::with_diagnostics) was supplied — the
    ///   per-attempt state the operation pipeline carries between
    ///   retries; or
    /// * `Wire` when diagnostics is supplied — the
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
        //  4. base = Wire + `with_diagnostics`                     -> Wire (response rebuilt with the new diagnostics; body+headers+status preserved)
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
                    // (4) Base already Wire.
                    //
                    // * If the caller did NOT supply `with_diagnostics`,
                    //   carry the response forward verbatim — its
                    //   diagnostics is the truth.
                    // * If the caller DID supply `with_diagnostics` via
                    //   `from_error(wire).with_diagnostics(d)`, rebuild
                    //   the response with `d` replacing the original
                    //   diagnostics. This is the path used by
                    //   `patch_handler::exhaustion_error` (and any future
                    //   caller that needs to graft aggregated /
                    //   operation-level diagnostics onto an existing
                    //   wire error). Body, headers, and status all stay
                    //   pinned to the base response — "CosmosResponse
                    //   wins" still holds for body / headers / status;
                    //   only the diagnostics slot is overridable on the
                    //   re-decoration path. Note this differs from rule
                    //   (1) (`with_response` on this same builder),
                    //   where the caller just supplied the full response
                    //   and the response's own diagnostics is therefore
                    //   authoritative.
                    let payload = response.payload().clone();
                    let status = response.status();
                    let diagnostics = self
                        .diagnostics
                        .clone()
                        .unwrap_or_else(|| response.diagnostics());
                    let response = finalize_response(payload, status, diagnostics);
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
        // apply any overrides supplied on this builder. `Cow::clone`
        // is free for `Borrowed` (pointer copy) and allocates for
        // `Owned` (deep `String` clone); since re-decoration is an
        // error path, the extra `Owned` clone is acceptable.
        let (mut message, mut source, backtrace) = match &self.base {
            Some(base) => (
                base.inner.message.clone(),
                base.inner.source.clone(),
                base.inner.backtrace.clone(),
            ),
            None => (Cow::Borrowed(""), None, None),
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
            message = Cow::Owned(buf);
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
    use std::sync::Mutex;

    /// Re-export of `error::backtrace::tests::TEST_LOCK` for tests in
    /// this module. It is **the same mutex** used by tests in
    /// `error::backtrace::tests` — sharing one lock across both modules
    /// is what serializes process-global throttle/limiter mutations and
    /// keeps these tests deterministic. A separate per-module lock
    /// would not serialize and would race.
    static BACKTRACE_TEST_LOCK: &Mutex<()> = &crate::error::backtrace::tests::TEST_LOCK;

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
        assert!(Arc::ptr_eq(&err.diagnostics().unwrap(), &diag));
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
        assert!(Arc::ptr_eq(&err.diagnostics().unwrap(), &resp_diag));
        assert!(!Arc::ptr_eq(&err.diagnostics().unwrap(), &unrelated_diag));
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
        assert!(Arc::ptr_eq(&err.diagnostics().unwrap(), &diag));
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
        assert!(Arc::ptr_eq(&finalized.diagnostics().unwrap(), &diag));
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
        assert_eq!(format!("{decorated}"), "503: op=createItem: attempt-failed",);
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
        assert!(Arc::ptr_eq(&decorated.diagnostics().unwrap(), &diag));
    }

    /// Re-decorating a `Wire` base error via
    /// `from_error(wire).with_diagnostics(d)` must override the response's
    /// diagnostics with `d` while preserving the base response's body,
    /// headers, and status. This is the path
    /// `patch_handler::exhaustion_error` uses to graft the aggregated
    /// cross-attempt diagnostics onto a wrapped service 412 — without
    /// this rule the override would be silently discarded by an earlier
    /// "CosmosResponse wins" formulation of builder rule (4) and the
    /// aggregated history would never reach the caller.
    #[test]
    fn from_error_wire_with_diagnostics_overrides_response_diagnostics() {
        let original_diag = make_test_diagnostics();
        let response = make_test_response(
            CosmosStatus::new(StatusCode::PreconditionFailed),
            Arc::clone(&original_diag),
        );
        let original = CosmosError::builder()
            .with_response(response)
            .with_message("etag mismatch")
            .build();

        let override_diag = make_test_diagnostics();
        let decorated = CosmosErrorBuilder::from_error(original)
            .with_diagnostics(Arc::clone(&override_diag))
            .with_context("op=patch")
            .build();

        // The override wins for `diagnostics()` — both on the outer error
        // and (because the response is rebuilt) on the wire response too.
        assert!(
            Arc::ptr_eq(&decorated.diagnostics().unwrap(), &override_diag),
            "with_diagnostics override must replace the base response's diagnostics"
        );
        let wire = decorated.response().expect("still Wire after override");
        assert!(
            Arc::ptr_eq(wire.diagnostics_ref(), &override_diag),
            "rebuilt response must carry the override diagnostics, not the original"
        );
        // Body / headers / status are pinned to the base response.
        assert_eq!(wire.status().status_code(), StatusCode::PreconditionFailed);
        assert!(!Arc::ptr_eq(wire.diagnostics_ref(), &original_diag));
    }

    #[test]
    fn builder_with_context_prepends_to_message() {
        let err = CosmosError::builder()
            .with_message("bad payload")
            .with_context("op=createItem")
            .build();
        // No status set → synthetic 500 default; no sub-status → just `500`.
        // `with_context` prepends `"op=createItem: "` to the message.
        assert_eq!(format!("{err}"), "500: op=createItem: bad payload");
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
        assert!(Arc::ptr_eq(&cloned.diagnostics().unwrap(), &diag));
    }

    #[test]
    fn builder_message_setter_overrides_base_message() {
        let original = CosmosError::builder().with_message("orig").build();
        let patched = CosmosErrorBuilder::from_error(original)
            .with_message("replaced")
            .build();
        assert_eq!(format!("{patched}"), "500: replaced");
    }

    #[test]
    fn builder_repeated_setters_last_write_wins() {
        let err = CosmosError::builder()
            .with_message("first")
            .with_message("second")
            .with_context("ctx-a")
            .with_context("ctx-b")
            .build();
        // Last `with_message` wins; last `with_context` wins; the context
        // prepends to the resolved message with `": "`.
        assert_eq!(format!("{err}"), "500: ctx-b: second");
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
        // Serialize against sibling tests that also mutate the
        // process-global capture throttle, and snapshot/restore so this
        // test does not leak `set_capacity(1000)` into tests that
        // depend on the default-off behavior.
        let _guard = BACKTRACE_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        // Snapshot both limiters so we restore via the public API and
        // don't leak capture-on state into sibling tests.
        let throttle = crate::error::backtrace::global_capture_throttle();
        let resolution = crate::error::backtrace::global_resolution_limiter();
        let prev_cap = throttle.capacity();
        let prev_res = resolution.capacity();
        let result = std::panic::catch_unwind(|| {
            // Enable capture via the public API — this trips
            // `PROGRAMMATIC_OVERRIDE`, so a concurrent first
            // `Backtrace::capture()` from another test cannot clobber
            // the throttle via `ensure_initialized()`'s env-derived
            // init path. Resolution capacity is kept at its current
            // value so the test doesn't accidentally change render
            // behavior.
            crate::error::backtrace::set_backtrace_options(
                crate::error::backtrace::BacktraceOptions {
                    max_captures_per_second: 1000,
                    max_resolutions_per_second: prev_res,
                },
            );
            let inner = end_to_end_timeout_error("inner");
            let inner_bt_id = inner
                .inner
                .backtrace
                .as_ref()
                .map(crate::error::backtrace::tests::backtrace_inner_arc_identity);
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
                .map(crate::error::backtrace::tests::backtrace_inner_arc_identity);
            assert_eq!(
                outer_bt_id, inner_bt_id,
                "outer error must share the inner's backtrace Arc, not capture a new one"
            );
        });
        // Restore via the public API too — `PROGRAMMATIC_OVERRIDE` stays
        // set (sticky for the rest of the process) but the limiters
        // return to their pre-test values.
        crate::error::backtrace::set_backtrace_options(crate::error::backtrace::BacktraceOptions {
            max_captures_per_second: prev_cap,
            max_resolutions_per_second: prev_res,
        });
        if let Err(payload) = result {
            std::panic::resume_unwind(payload);
        }
    }

    /// Custom non-Cosmos error type that carries an arbitrary
    /// `dyn StdError` as its source. Used to simulate a third-party
    /// wrapper (e.g. `azure_core::Error`) sitting between an outer
    /// `CosmosError` and an inner `CosmosError` re-imported through a
    /// policy / credential boundary.
    #[derive(Debug)]
    struct ThirdPartyWrapper {
        source: Arc<dyn StdError + Send + Sync + 'static>,
    }

    impl fmt::Display for ThirdPartyWrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("third-party wrapper")
        }
    }

    impl StdError for ThirdPartyWrapper {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            Some(self.source.as_ref())
        }
    }

    /// Regression guard for the indirect-wrap path: when a `CosmosError`
    /// is re-imported into another `CosmosError` via a third-party
    /// wrapper (e.g. `azure_core::Error` from a policy boundary),
    /// inheritance must walk the source chain — bounded by
    /// `MAX_BACKTRACE_INHERITANCE_DEPTH` — and find the inner Cosmos
    /// backtrace instead of paying for a fresh capture at the wrap site.
    #[test]
    fn wrap_inherits_backtrace_through_indirect_third_party_wrapper() {
        // Serialize + snapshot/restore — see `BACKTRACE_TEST_LOCK`.
        let _guard = BACKTRACE_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let throttle = crate::error::backtrace::global_capture_throttle();
        let resolution = crate::error::backtrace::global_resolution_limiter();
        let prev_cap = throttle.capacity();
        let prev_res = resolution.capacity();
        let result = std::panic::catch_unwind(|| {
            // Enable capture via the public API — trips
            // `PROGRAMMATIC_OVERRIDE` so a concurrent first
            // `Backtrace::capture()` can't clobber the throttle. See
            // `wrap_inherits_backtrace_from_cosmos_source`.
            crate::error::backtrace::set_backtrace_options(
                crate::error::backtrace::BacktraceOptions {
                    max_captures_per_second: 1000,
                    max_resolutions_per_second: prev_res,
                },
            );

            let inner = end_to_end_timeout_error("deeply nested");
            let inner_bt_id = inner
                .inner
                .backtrace
                .as_ref()
                .map(crate::error::backtrace::tests::backtrace_inner_arc_identity);
            assert!(
                inner_bt_id.is_some(),
                "inner must have a captured backtrace for this test to be meaningful",
            );

            // Wrap `inner` in a non-Cosmos third-party error type, then
            // wrap THAT as the source of an outer `CosmosError`. The
            // outer error's immediate source is `ThirdPartyWrapper`, not
            // `CosmosError`, so the previous-immediate-source-only
            // implementation would have missed the inheritance and
            // captured a fresh backtrace at the wrap site.
            let wrapper = ThirdPartyWrapper {
                source: Arc::new(inner),
            };
            let outer = CosmosError::builder()
                .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
                .with_message("outer")
                .with_source(wrapper)
                .build();

            let outer_bt_id = outer
                .inner
                .backtrace
                .as_ref()
                .map(crate::error::backtrace::tests::backtrace_inner_arc_identity);
            assert_eq!(
                outer_bt_id, inner_bt_id,
                "outer error must inherit the inner Cosmos backtrace through the third-party wrapper, not capture a fresh one",
            );
        });
        crate::error::backtrace::set_backtrace_options(crate::error::backtrace::BacktraceOptions {
            max_captures_per_second: prev_cap,
            max_resolutions_per_second: prev_res,
        });
        if let Err(payload) = result {
            std::panic::resume_unwind(payload);
        }
    }

    /// Bounds test: an indirect chain that exceeds
    /// `MAX_BACKTRACE_INHERITANCE_DEPTH` does NOT inherit (so the cap is
    /// actually enforced) and falls back to a fresh capture. This is
    /// the deliberate trade-off: bound the per-construction walk so a
    /// pathological or cyclic chain cannot pin a thread on the error
    /// hot path.
    #[test]
    fn wrap_falls_back_to_fresh_capture_when_chain_exceeds_inheritance_depth() {
        // Serialize + snapshot/restore — see `BACKTRACE_TEST_LOCK`.
        let _guard = BACKTRACE_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        let throttle = crate::error::backtrace::global_capture_throttle();
        let resolution = crate::error::backtrace::global_resolution_limiter();
        let prev_cap = throttle.capacity();
        let prev_res = resolution.capacity();
        let result = std::panic::catch_unwind(|| {
            // Enable capture via the public API — trips
            // `PROGRAMMATIC_OVERRIDE`. See
            // `wrap_inherits_backtrace_from_cosmos_source`.
            crate::error::backtrace::set_backtrace_options(
                crate::error::backtrace::BacktraceOptions {
                    max_captures_per_second: 1000,
                    max_resolutions_per_second: prev_res,
                },
            );

            let inner = end_to_end_timeout_error("deeply nested");
            let inner_bt_id = inner
                .inner
                .backtrace
                .as_ref()
                .map(crate::error::backtrace::tests::backtrace_inner_arc_identity);
            assert!(inner_bt_id.is_some());

            // Build a chain of `MAX_BACKTRACE_INHERITANCE_DEPTH + 1`
            // third-party wrappers, so the inner Cosmos error sits one
            // frame past the cap. The walk should stop before reaching
            // it and the outer error captures a fresh backtrace.
            let mut src: Arc<dyn StdError + Send + Sync + 'static> = Arc::new(inner);
            for _ in 0..=MAX_BACKTRACE_INHERITANCE_DEPTH {
                src = Arc::new(ThirdPartyWrapper {
                    source: src.clone(),
                });
            }
            let outer = CosmosError::builder()
                .with_status(CosmosStatus::TRANSPORT_GENERATED_503)
                .with_message("outer")
                .with_arc_source(src)
                .build();

            let outer_bt_id = outer
                .inner
                .backtrace
                .as_ref()
                .map(crate::error::backtrace::tests::backtrace_inner_arc_identity);
            assert!(
                outer_bt_id.is_some(),
                "fresh capture must succeed when inheritance is bounded out"
            );
            assert_ne!(
                outer_bt_id, inner_bt_id,
                "wrap chain deeper than MAX_BACKTRACE_INHERITANCE_DEPTH must NOT inherit; a fresh backtrace must be captured at the wrap site",
            );
        });
        crate::error::backtrace::set_backtrace_options(crate::error::backtrace::BacktraceOptions {
            max_captures_per_second: prev_cap,
            max_resolutions_per_second: prev_res,
        });
        if let Err(payload) = result {
            std::panic::resume_unwind(payload);
        }
    }

    /// Documents — by way of full-string equality on the deterministic
    /// prefix plus a hand-rolled structural parse on the backtrace
    /// tail — how a captured backtrace shows up in each of
    /// `CosmosError`'s four formatting flags.
    ///
    /// The header / source-chain / diagnostics / separator portions are
    /// fully reproducible across machines and builds, so they are
    /// asserted byte-for-byte. The backtrace tail itself embeds
    /// absolute file paths, line numbers, and a frame count that all
    /// depend on the local source tree / OS / toolchain version, so we
    /// instead validate its *shape*:
    ///
    /// ```text
    /// {N:>4}: <symbol>\n                       // every frame
    ///           at <prefix>[.rs[:<line>]]\n    // optional per frame
    /// ```
    ///
    /// Example of the first few frames on a Windows developer
    /// workstation (re-recorded as a documentation aid, NOT asserted):
    ///
    /// ```text
    ///    0: backtrace::backtrace::win64::trace
    ///           at C:\Users\…\.cargo\registry\…\backtrace-0.3.76\src\backtrace\win64.rs:85
    ///    1: backtrace::backtrace::trace<azure_data_cosmos_driver::error::backtrace::impl$0::capture::closure_env$0>
    ///           at C:\Users\…\.cargo\registry\…\backtrace-0.3.76\src\backtrace\mod.rs:53
    ///    2: azure_data_cosmos_driver::error::backtrace::Backtrace::capture
    ///           at E:\…\sdk\cosmos\azure_data_cosmos_driver\src\error\backtrace.rs:234
    ///    3: azure_data_cosmos_driver::error::CosmosError::from_inner
    ///           at E:\…\sdk\cosmos\azure_data_cosmos_driver\src\error\mod.rs:159
    ///    …
    /// ```
    ///
    /// In addition to the shape, we require **at least one** frame to
    /// carry the test function's fully-qualified symbol — proof that the
    /// captured stack actually originates from the call site under
    /// test rather than (say) an empty / broken backtrace.
    #[test]
    fn backtrace_emission_paths_render_as_documented() {
        // Serialize against sibling tests that touch the process-global
        // throttle/limiter, and snapshot+restore so this test does not
        // leak capture-on state into tests that depend on the
        // default-off behavior.
        let _guard = BACKTRACE_TEST_LOCK
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        // Prime the OnceLock-gated env init BEFORE snapshotting / setting
        // capacities so the first `Backtrace::capture()` inside
        // `CosmosError::builder().build()` doesn't fire the lazy init
        // and clobber our test-set values back to env defaults (0 when
        // RUST_BACKTRACE is unset).
        crate::error::backtrace::ensure_initialized();
        let throttle = crate::error::backtrace::global_capture_throttle();
        let resolution = crate::error::backtrace::global_resolution_limiter();
        let prev_capture = throttle.capacity();
        let prev_resolution = resolution.capacity();

        let result = std::panic::catch_unwind(|| {
            // Generous capacities so capture is allowed AND fresh symbol
            // resolution is allowed (otherwise the rendered backtrace
            // would be `<unresolved> @ 0xIP` placeholders).
            throttle.set_capacity(1_000_000);
            resolution.set_capacity(1_000_000);

            let err = CosmosError::builder().with_message("bt-test").build();

            // Capture each of the four formatted forms into its own
            // string so the assertion failures below print the exact
            // current rendering for easy reviewer inspection.
            let display = format!("{err}");
            let display_alt = format!("{err:#}");
            let debug = format!("{err:?}");
            let debug_alt = format!("{err:#?}");

            // (1) Header-only forms are fully reproducible.
            assert_eq!(display, "500: bt-test");
            assert_eq!(debug, "500: bt-test");

            // (2) Alternate Display / Debug both prepend the same
            //     deterministic prefix to the backtrace tail.
            const ALT_PREFIX: &str = "500: bt-test\n\nStack backtrace:\n";
            let display_alt_tail = display_alt.strip_prefix(ALT_PREFIX).unwrap_or_else(|| {
                panic!("alternate Display must start with {ALT_PREFIX:?}, got:\n{display_alt}")
            });
            let debug_alt_tail = debug_alt.strip_prefix(ALT_PREFIX).unwrap_or_else(|| {
                panic!("alternate Debug must start with {ALT_PREFIX:?}, got:\n{debug_alt}")
            });

            // (3) Both alternate forms emit the same backtrace tail
            //     (no per-instance re-rendering or re-resolution).
            assert_eq!(display_alt_tail, debug_alt_tail);

            // (4) Structural parse of the backtrace tail.
            // Use just the suffix (without the crate name) so the check
            // is robust to rustc's symbol-mangling disambiguator, which
            // some platforms (notably macOS) render as
            // `azure_data_cosmos_driver[<hash>]::error::tests::…`.
            assert_backtrace_tail_shape(
                display_alt_tail,
                "::error::tests::backtrace_emission_paths_render_as_documented",
            );
        });

        // Always restore, even on panic, so a failure here does not
        // cascade into sibling tests that depend on the default-off
        // throttle / limiter capacities.
        throttle.set_capacity(prev_capture);
        resolution.set_capacity(prev_resolution);
        if let Err(payload) = result {
            std::panic::resume_unwind(payload);
        }
    }

    /// Parses the backtrace tail emitted by [`write_backtrace`] and
    /// validates that:
    ///
    /// 1. At least one frame is present.
    /// 2. Frame indices start at `0` and increment by `1` (no gaps,
    ///    no reorderings).
    /// 3. Each frame is a `   N: <symbol>\n` line, optionally followed
    ///    by `          at <prefix>[:<line>]\n` (kernel / stripped
    ///    frames legitimately have no source location).
    /// 4. At least one frame's symbol contains `required_symbol_substring`
    ///    — typically the fully-qualified path of the test under
    ///    inspection, so callers can prove the captured stack actually
    ///    walks through their call site rather than (say) an empty or
    ///    broken backtrace. Pass `""` to skip this check.
    fn assert_backtrace_tail_shape(tail: &str, required_symbol_substring: &str) {
        const AT_INDENT: &str = "          at ";

        let mut lines = tail.lines().peekable();
        let mut frame_index: u32 = 0;
        let mut saw_required_symbol = false;

        while let Some(line) = lines.next() {
            // Expect a `"%4d: <symbol>"` symbol line. `try_render`
            // writes `{:>4}: ` so the index is right-aligned in 4
            // columns followed by `": "`.
            let after_colon = line
                .split_once(": ")
                .and_then(|(idx_part, sym)| {
                    let idx: u32 = idx_part.trim_start().parse().ok()?;
                    Some((idx, sym))
                })
                .unwrap_or_else(|| {
                    panic!(
                        "expected `{frame_index:>4}: <symbol>` symbol line, got: {line:?}\n\
                         (full tail under inspection:\n{tail})",
                    )
                });
            let (idx, symbol) = after_colon;
            assert_eq!(
                idx, frame_index,
                "frame indices must increment by 1; got idx={idx} for expected index {frame_index}\nline: {line:?}",
            );
            assert!(
                !symbol.is_empty(),
                "frame {frame_index} has an empty symbol, line: {line:?}",
            );
            if !required_symbol_substring.is_empty() && symbol.contains(required_symbol_substring) {
                saw_required_symbol = true;
            }

            // Optionally consume a `          at <path>[:<line>]` line.
            if let Some(next) = lines.peek() {
                if let Some(rest) = next.strip_prefix(AT_INDENT) {
                    // `rest` is `<path>` or `<path>:<digits>` (the
                    // `:<digits>` suffix is only present when the
                    // resolver returned a line number; kernel paths
                    // like `/rustc/<hash>/library\…` also reach this
                    // branch and that is fine — we accept any
                    // non-empty `<path>`).
                    assert!(
                        !rest.is_empty(),
                        "`at` line is empty for frame {frame_index}: {next:?}",
                    );
                    // If a `:<line>` suffix is present, it must be all
                    // digits. Split on the LAST `:` because Windows
                    // paths begin with `C:\` and contain colons.
                    if let Some((_path, line_no)) = rest.rsplit_once(':') {
                        if line_no.chars().all(|c| c.is_ascii_digit()) && !line_no.is_empty() {
                            // OK — `<path>:<line>` form.
                        } else {
                            // The last `:` was part of the path
                            // (Windows drive letter, generic angle
                            // brackets, etc.) — no `<line>` suffix,
                            // still valid.
                        }
                    }
                    lines.next();
                }
            }

            frame_index += 1;
        }

        assert!(
            frame_index > 0,
            "backtrace tail must contain at least one frame, got:\n{tail}",
        );
        if !required_symbol_substring.is_empty() {
            assert!(
                saw_required_symbol,
                "no frame symbol contained `{required_symbol_substring}` — the \
                 captured stack does not appear to originate from the call \
                 site under inspection. Tail under inspection:\n{tail}",
            );
        }
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

    fn diagnostics_json_section(diag_section: &str) -> &str {
        diag_section
            .split_once("\n\nStack backtrace:\n")
            .map(|(json, _)| json)
            .unwrap_or(diag_section)
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
            Arc::ptr_eq(
                &attached.diagnostics().expect("diagnostics attached"),
                &diag
            ),
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
        // Plain `{e}` is the bare header — single line, no source chain,
        // no diagnostics block, no backtrace. Fully deterministic.
        assert_eq!(
            format!("{err}"),
            "503/20003 (TransportGenerated503): outer transport failure",
        );
    }

    #[test]
    fn display_alternate_includes_header_source_chain_and_diagnostics() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:#}");
        // The alternate form is `<header>\n\nCaused by:\n  0: <src>[\n\nStack backtrace:\n<…>]\n\nDiagnostics:\n<diag>`.
        // The diagnostics block embeds a freshly-generated UUID
        // (`activity={uuid}`) and a wall-clock duration, neither of which
        // is reproducible, so we split at the diagnostics boundary and
        // assert structurally on the deterministic prefix. The Stack
        // backtrace block is conditionally present depending on whether
        // backtrace capture is enabled (off by default in local test
        // runs; on with `RUST_BACKTRACE=1` in CI or when a sibling test
        // programmatically enables it), so we accept either shape.
        let (prefix, diag_section) = rendered
            .split_once("\n\nDiagnostics:\n")
            .expect("alternate Display must include a Diagnostics: block");
        let header_and_source = "503/20003 (TransportGenerated503): outer transport failure\n\n\
             Caused by:\n  \
             0: 408/20008 (ClientOperationTimeout): inner timeout";
        assert!(
            prefix.starts_with(header_and_source),
            "alternate Display prefix must start with the header+source-chain block, got: {prefix}",
        );
        let interposed = &prefix[header_and_source.len()..];
        assert!(
            interposed.is_empty() || interposed.starts_with("\n\nStack backtrace:\n"),
            "interposed content between source chain and diagnostics must be empty or a Stack backtrace block, got: {interposed}",
        );
        // Diagnostics block: bounded structural check — every line of the
        // `DiagnosticsContext` `Display` impl begins with `activity=…`.
        assert!(
            diag_section.starts_with("activity="),
            "Diagnostics section must start with `activity=…`, got: {diag_section}",
        );
    }

    #[test]
    fn debug_omits_backtrace_block_in_plain_form() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:?}");
        // Plain `{e:?}` = header + source chain (with `{src:?}` per
        // source) + diagnostics. The captured backtrace is intentionally
        // omitted in non-alternate Debug. The inner source is itself a
        // `CosmosError` with no further source / diagnostics, so its
        // own `Debug` reduces to the bare header.
        let (prefix, diag_section) = rendered
            .split_once("\n\nDiagnostics:\n")
            .expect("plain Debug must include a Diagnostics: block");
        assert_eq!(
            prefix,
            "503/20003 (TransportGenerated503): outer transport failure\n\n\
             Caused by:\n  \
             0: 408/20008 (ClientOperationTimeout): inner timeout",
        );
        let diagnostics_json = diagnostics_json_section(diag_section);
        serde_json::from_str::<serde_json::Value>(diagnostics_json).unwrap_or_else(|e| {
            panic!("Diagnostics section must be valid JSON: {e}\n{diagnostics_json}")
        });
        assert!(
            !diagnostics_json.contains("DiagnosticsContext {"),
            "Diagnostics section must be JSON, not a Rust Debug dump: {diagnostics_json}",
        );
        assert!(
            !rendered.contains("Stack backtrace:"),
            "plain Debug must NOT include the backtrace block, got:\n{rendered}",
        );
    }

    #[test]
    fn debug_alternate_propagates_to_source_and_diagnostics() {
        let err = make_error_with_diagnostics_and_source();
        let rendered = format!("{err:#?}");
        // Alternate `{e:#?}` matches plain `{e:?}` in this fixture when
        // backtrace capture is disabled (the default in local test runs);
        // when capture IS enabled (e.g. `RUST_BACKTRACE=1` in CI or a
        // sibling test that programmatically enables it), the rendered
        // form additionally interposes `\n\nStack backtrace:\n<…>`
        // between the source chain and the diagnostics block. The test
        // is tolerant of either shape: it asserts the deterministic
        // header + source-chain prefix and the diagnostics suffix, and
        // ignores any intervening backtrace block.
        let (prefix, diag_section) = rendered
            .split_once("\n\nDiagnostics:\n")
            .expect("alternate Debug must include a Diagnostics: block");
        let header_and_source = "503/20003 (TransportGenerated503): outer transport failure\n\n\
             Caused by:\n  \
             0: 408/20008 (ClientOperationTimeout): inner timeout";
        assert!(
            prefix.starts_with(header_and_source),
            "alternate Debug prefix must start with the header+source-chain block, got: {prefix}",
        );
        // Anything between the deterministic prefix and the diagnostics
        // suffix must be either empty or a `Stack backtrace:` block.
        let interposed = &prefix[header_and_source.len()..];
        assert!(
            interposed.is_empty() || interposed.starts_with("\n\nStack backtrace:\n"),
            "interposed content between source chain and diagnostics must be empty or a Stack backtrace block, got: {interposed}",
        );
        let diagnostics_json = diagnostics_json_section(diag_section);
        serde_json::from_str::<serde_json::Value>(diagnostics_json).unwrap_or_else(|e| {
            panic!("Diagnostics section must be valid JSON: {e}\n{diagnostics_json}")
        });
        assert!(
            !diagnostics_json.contains("DiagnosticsContext {"),
            "Diagnostics section must be JSON, not a Rust Debug dump: {diagnostics_json}",
        );
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

    /// Pins the surface the SDK's `From<CosmosError> for azure_core::Error`
    /// mapping reads when classifying a wire-response error into
    /// `azure_core::ErrorKind::HttpResponse { status, error_code, .. }`.
    /// The SDK test cannot exercise this branch directly because the only
    /// public way to attach a wire response (`CosmosResponse::new`) is
    /// `pub(crate)` to the driver. Asserting the inputs here keeps the
    /// driver-side contract honest.
    #[test]
    fn wire_response_error_exposes_status_and_substatus_for_sdk_classifier() {
        let diag = make_test_diagnostics();
        let response = make_test_response(
            CosmosStatus::from_parts(
                StatusCode::TooManyRequests,
                Some(SubStatusCode::THROTTLE_DUE_TO_SPLIT),
            ),
            Arc::clone(&diag),
        );
        let err = CosmosError::builder()
            .with_response(response)
            .with_message("throttled")
            .build();

        // These are the three driver-side reads the SDK classifier
        // performs on the wire-response branch.
        assert!(
            err.is_from_wire(),
            "is_from_wire must return true so the SDK classifier picks HttpResponse"
        );
        assert_eq!(err.status().status_code(), StatusCode::TooManyRequests);
        assert_eq!(
            err.status().sub_status(),
            Some(SubStatusCode::THROTTLE_DUE_TO_SPLIT),
            "sub-status must round-trip to the SDK as `error_code` on the HttpResponse kind"
        );
        // And the response is reachable for further inspection.
        let wire = err.response().expect("wire response present");
        assert_eq!(wire.status().status_code(), StatusCode::TooManyRequests);
    }

    /// Companion of the wire-response test: synthetic errors (no
    /// `with_response`) must report `is_from_wire() == false` and
    /// `response() == None`, which is what drives the SDK classifier
    /// into its sub-status-based bucket (`Connection` / `Io` /
    /// `Credential` / `DataConversion` / `Other`) instead of
    /// `HttpResponse`.
    #[test]
    fn synthetic_error_reports_not_from_wire_for_sdk_classifier() {
        let err = CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_DNS_FAILED)
            .with_message("dns failure")
            .build();
        assert!(!err.is_from_wire());
        assert!(err.response().is_none());
        // Sub-status is still readable so the SDK classifier can route on it.
        assert_eq!(
            err.status().sub_status(),
            Some(SubStatusCode::TRANSPORT_DNS_FAILED)
        );
    }

    /// `WirePending` is an internal-only staging state. The public
    /// [`CosmosError::is_from_wire`] predicate must stay in lockstep
    /// with [`CosmosError::response`] (both report "no wire response
    /// reachable externally") so the SDK boundary classifier cannot
    /// observe an `HttpResponse`-classified error with no payload
    /// reachable. The internal `wire_payload()` accessor still
    /// surfaces the staged parts for in-pipeline finalization.
    #[test]
    fn wire_pending_reports_not_from_wire() {
        let err = CosmosError::builder()
            .with_status(CosmosStatus::new(StatusCode::TooManyRequests))
            .with_message("staged")
            .with_response_parts(make_test_payload())
            .build();
        assert!(err.response().is_none());
        assert!(
            !err.is_from_wire(),
            "WirePending must not advertise is_from_wire()==true; it would lie to the SDK classifier"
        );
        assert!(
            err.wire_payload().is_some(),
            "internal accessor must still expose staged parts for in-pipeline finalization"
        );
    }
}
