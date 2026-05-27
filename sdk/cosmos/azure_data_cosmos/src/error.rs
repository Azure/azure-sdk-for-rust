// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-owned newtype wrapper around the driver's [`CosmosError`].
//!
//! The wrapper is `#[repr(transparent)]` so converting between the SDK and
//! driver representations is a zero-cost move. All construction, classification,
//! status-code constants, and predicates live in the driver crate
//! (`azure_data_cosmos_driver::error`); the SDK layer adds only thin
//! delegating accessors, the [`From<CosmosError>`] bridge into
//! [`azure_core::Error`] required by the Azure SDK for Rust guidelines, and the
//! public [`Result`] alias.

use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use azure_data_cosmos_driver::error::CosmosError as DriverCosmosError;
use azure_data_cosmos_driver::models::CosmosResponse;

use crate::models::DiagnosticsContext;

/// Categorical kind for a [`CosmosError`] — owned by
/// [`CosmosStatus`](crate::CosmosStatus) and re-exported here for ergonomic
/// access alongside the SDK error surface. See the driver crate for the
/// canonical definition.
pub type CosmosStatusKind = azure_data_cosmos_driver::error::CosmosStatusKind;

/// Typed Cosmos status (HTTP status code + optional sub-status + categorical
/// [`CosmosStatusKind`]) — type alias re-exporting the driver definition so
/// SDK-only callers can stay on a single crate import.
pub type CosmosStatus = azure_data_cosmos_driver::error::CosmosStatus;

/// Sub-status code — type alias re-exporting the driver definition.
pub type SubStatusCode = azure_data_cosmos_driver::error::SubStatusCode;

/// The error type returned by every fallible public API in `azure_data_cosmos`.
///
/// `CosmosError` carries the typed Cosmos status (HTTP status + sub-status,
/// including synthetic client-side codes such as `408 / 20008` for end-to-end
/// operation timeout), the wire-level [`CosmosResponse`] when one was
/// received, and the operation diagnostics — for both service-side and
/// client-side failures.
///
/// Any underlying source error is reachable via
/// [`std::error::Error::source`].
#[repr(transparent)]
#[derive(Clone)]
pub struct CosmosError(DriverCosmosError);

impl CosmosError {
    /// Returns a fluent [`CosmosErrorBuilder`] seeded with sensible defaults
    /// for the given categorical [`CosmosStatusKind`].
    pub fn builder(kind: CosmosStatusKind) -> CosmosErrorBuilder {
        CosmosErrorBuilder(azure_data_cosmos_driver::error::CosmosError::builder(kind))
    }

    /// Returns the typed Cosmos status. Always present — non-service errors
    /// carry a synthetic status with a placeholder HTTP code and the correct
    /// [`CosmosStatusKind`].
    pub fn status(&self) -> CosmosStatus {
        self.0.status()
    }

    /// Returns the categorical [`CosmosStatusKind`]. Convenience for
    /// `self.status().kind()`.
    pub fn kind(&self) -> CosmosStatusKind {
        self.0.kind()
    }

    /// Returns the originating [`CosmosResponse`] when a wire response was
    /// received and fully assembled with finalized diagnostics. Returns
    /// `None` for synthetic errors (transport, client, configuration, …).
    pub fn response(&self) -> Option<&CosmosResponse> {
        self.0.response()
    }

    /// Returns the diagnostics context for the failed operation. For
    /// wire-response errors this is `Some(response.diagnostics())`; for
    /// synthetic errors it is whatever the pipeline attached, or `None`.
    pub fn diagnostics(&self) -> Option<&Arc<DiagnosticsContext>> {
        self.0.diagnostics()
    }

    /// Returns the stack backtrace captured at error construction time,
    /// rendered as a human-readable string, when the production-safety
    /// gates allowed capture and resolution.
    ///
    /// Capture is bounded by two rolling-1-second limiters (capture
    /// throttle + resolution rate), both configurable via the driver's
    /// [`CosmosDriverRuntimeBuilder::with_max_error_backtrace_resolutions_per_second`](azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_resolutions_per_second)
    /// /
    /// [`with_max_error_backtrace_captures_per_second`](azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_captures_per_second)
    /// builder methods or the corresponding
    /// `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` /
    /// `AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND` environment variables.
    /// Cache hits do not consume budget. Returns `None` when capture was
    /// throttled or when the resolution limiter denied a cache-missed frame;
    /// partial backtraces are never produced.
    pub fn backtrace(&self) -> Option<&Arc<str>> {
        self.0.backtrace()
    }

    // -- construction helpers (pub(crate)) --

    /// Builds a `Client` error (caller misuse / precondition), optionally
    /// wrapping an underlying source error.
    pub(crate) fn client(
        message: impl Into<Arc<str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        let mut b = DriverCosmosError::builder(CosmosStatusKind::Client).with_message(message);
        if let Some(s) = source {
            b = b.with_arc_source(s);
        }
        Self(b.build())
    }

    /// Builds a `Configuration` error (bad endpoint URL, malformed connection
    /// string, etc.), optionally wrapping an underlying source error.
    pub(crate) fn configuration(
        message: impl Into<Arc<str>>,
        source: Option<Arc<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        let mut b =
            DriverCosmosError::builder(CosmosStatusKind::Configuration).with_message(message);
        if let Some(s) = source {
            b = b.with_arc_source(s);
        }
        Self(b.build())
    }
}

impl fmt::Display for CosmosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for CosmosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl StdError for CosmosError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

impl From<DriverCosmosError> for CosmosError {
    fn from(inner: DriverCosmosError) -> Self {
        Self(inner)
    }
}

impl From<serde_json::Error> for CosmosError {
    fn from(error: serde_json::Error) -> Self {
        Self(
            DriverCosmosError::builder(CosmosStatusKind::Serialization)
                .with_message("JSON serialization or deserialization failed")
                .with_source(error)
                .build(),
        )
    }
}

impl From<url::ParseError> for CosmosError {
    fn from(error: url::ParseError) -> Self {
        Self(
            DriverCosmosError::builder(CosmosStatusKind::Configuration)
                .with_message("invalid URL")
                .with_source(error)
                .build(),
        )
    }
}

/// Per Azure SDK for Rust guideline: every service-crate error type provides a
/// [`From`] impl into [`azure_core::Error`] so callers using the foundation
/// error type via `?`/`From` continue to compose. The conversion maps the
/// categorical [`CosmosStatusKind`] to the closest
/// [`azure_core::error::ErrorKind`] and preserves the original [`CosmosError`]
/// as the source so callers can `downcast_ref::<CosmosError>()` for the typed
/// Cosmos surface.
impl From<CosmosError> for azure_core::Error {
    fn from(err: CosmosError) -> Self {
        use azure_core::error::ErrorKind as CoreKind;
        let core_kind = match err.kind() {
            CosmosStatusKind::Service => CoreKind::HttpResponse {
                status: err.status().status_code(),
                error_code: err.status().sub_status().map(|s| s.value().to_string()),
                raw_response: None,
            },
            CosmosStatusKind::Transport => CoreKind::Io,
            CosmosStatusKind::Authentication => CoreKind::Credential,
            CosmosStatusKind::Serialization
            | CosmosStatusKind::Client
            | CosmosStatusKind::Configuration => CoreKind::DataConversion,
            // `CosmosStatusKind` is `#[non_exhaustive]`. New variants added to
            // the driver should be reviewed and explicitly mapped here; fall
            // back to `Other` so unknown future kinds don't silently mask the
            // typed Cosmos error (still recoverable via downcast on the source
            // chain).
            _ => CoreKind::Other,
        };
        azure_core::Error::new(core_kind, err)
    }
}

/// Fluent builder for [`CosmosError`]. Newtype around the driver's
/// [`CosmosErrorBuilder`](azure_data_cosmos_driver::error::CosmosErrorBuilder).
#[must_use = "CosmosErrorBuilder is inert until `.build()` is called"]
pub struct CosmosErrorBuilder(azure_data_cosmos_driver::error::CosmosErrorBuilder);

impl CosmosErrorBuilder {
    /// Starts a builder pre-populated from an existing [`CosmosError`].
    pub fn from_error(err: CosmosError) -> Self {
        Self(azure_data_cosmos_driver::error::CosmosErrorBuilder::from_error(err.0))
    }

    /// Overrides the [`CosmosStatus`].
    pub fn with_status(self, status: CosmosStatus) -> Self {
        Self(self.0.with_status(status))
    }

    /// Sets the human-readable error message.
    pub fn with_message(self, message: impl Into<Arc<str>>) -> Self {
        Self(self.0.with_message(message))
    }

    /// Attaches an underlying source error reachable via
    /// [`std::error::Error::source`].
    pub fn with_source<E>(self, source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self(self.0.with_source(source))
    }

    /// Attaches an already-shared `Arc`-wrapped source.
    pub fn with_arc_source(self, source: Arc<dyn StdError + Send + Sync + 'static>) -> Self {
        Self(self.0.with_arc_source(source))
    }

    /// Attaches the wire-level [`CosmosResponse`]. The response carries
    /// status and diagnostics together — see the driver-side docs for the
    /// reconciliation rules ("CosmosResponse wins").
    pub fn with_response(self, response: CosmosResponse) -> Self {
        Self(self.0.with_response(response))
    }

    /// Attaches a standalone operation [`DiagnosticsContext`]. Ignored if
    /// [`with_response`](Self::with_response) was also called.
    pub fn with_diagnostics(self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        Self(self.0.with_diagnostics(diagnostics))
    }

    /// Prepends operational context to the final message as
    /// `"{context}: {message}"`.
    pub fn with_context(self, context: impl Into<Arc<str>>) -> Self {
        Self(self.0.with_context(context))
    }

    /// Finalizes the builder into a [`CosmosError`].
    pub fn build(self) -> CosmosError {
        CosmosError(self.0.build())
    }
}

/// `azure_data_cosmos` crate-wide `Result` alias.
pub type Result<T> = std::result::Result<T, CosmosError>;

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::error::ErrorKind as CoreErrorKind;

    #[test]
    fn from_cosmos_error_for_azure_core_error_preserves_chain_and_kind() {
        let inner_io = std::io::Error::new(std::io::ErrorKind::Other, "io fail");
        let cosmos = CosmosError::builder(CosmosStatusKind::Transport)
            .with_message("transport blew up")
            .with_source(inner_io)
            .build();
        let core_err: azure_core::Error = cosmos.into();
        // Kind maps Transport → Io.
        assert!(matches!(core_err.kind(), CoreErrorKind::Io));
        // Message + source chain preserved (the `CosmosError` becomes the
        // azure_core::Error's source so callers can downcast).
        let rendered = format!("{core_err}");
        assert!(
            rendered.contains("transport blew up") || rendered.contains("io fail"),
            "azure_core::Error rendering must surface the cosmos message or chain: {rendered}",
        );
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_maps_service_kind() {
        let cosmos = CosmosError::builder(CosmosStatusKind::Service)
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::NotFound))
            .with_message("missing")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        match core_err.kind() {
            CoreErrorKind::HttpResponse { status, .. } => {
                assert_eq!(*status, azure_core::http::StatusCode::NotFound);
            }
            other => panic!("expected HttpResponse, got {other:?}"),
        }
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_downcast_recovers_cosmos_error() {
        let cosmos = CosmosError::builder(CosmosStatusKind::Client)
            .with_message("bad arg")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        let chain: &(dyn std::error::Error + 'static) = &core_err;
        let mut cur = chain.source();
        let mut found = false;
        while let Some(s) = cur {
            if s.downcast_ref::<CosmosError>().is_some() {
                found = true;
                break;
            }
            cur = s.source();
        }
        assert!(
            found,
            "azure_core::Error source chain must let callers downcast back to CosmosError"
        );
    }
}
