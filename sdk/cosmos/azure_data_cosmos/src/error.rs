// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! SDK-owned newtype wrapper around the driver's [`CosmosError`].
//!
//! The wrapper is `#[repr(transparent)]` so converting between the SDK and
//! driver representations is a zero-cost move. All construction, status-code
//! constants, and predicates live in the driver crate
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

/// Typed Cosmos status (HTTP status code + optional sub-status) — type
/// alias re-exporting the driver definition so SDK-only callers can stay
/// on a single crate import.
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
    /// Returns a fluent [`CosmosErrorBuilder`] seeded with a synthetic
    /// `500 InternalServerError` default status. Callers typically follow
    /// with [`.with_status(...)`](CosmosErrorBuilder::with_status) using
    /// one of the well-known [`CosmosStatus`] constants
    /// ([`TRANSPORT_GENERATED_503`](CosmosStatus::TRANSPORT_GENERATED_503),
    /// [`AUTHENTICATION_TOKEN_ACQUISITION_FAILED`](CosmosStatus::AUTHENTICATION_TOKEN_ACQUISITION_FAILED),
    /// [`SERIALIZATION_RESPONSE_BODY_INVALID`](CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID),
    /// …), or with [`.with_response(...)`](CosmosErrorBuilder::with_response)
    /// for service errors received from the wire.
    pub fn builder() -> CosmosErrorBuilder {
        CosmosErrorBuilder(azure_data_cosmos_driver::error::CosmosError::builder())
    }

    /// Returns the typed Cosmos status (HTTP status code + optional
    /// sub-status). Always present — non-service errors carry a synthetic
    /// status with a placeholder HTTP code (e.g.
    /// [`CosmosStatus::TRANSPORT_GENERATED_503`] for transport failures).
    pub fn status(&self) -> CosmosStatus {
        self.0.status()
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
    /// rendered as a human-readable string, when capture was enabled and
    /// the production-safety gates allowed it.
    ///
    /// Backtrace capture is **opt-in**: by default it is off and this
    /// method returns `None` for every error. Operators enable it either
    /// by setting the stdlib `RUST_BACKTRACE` environment variable (safe
    /// defaults: 10 000 captures / second, 5 fresh symbol resolutions /
    /// second) or by passing explicit capacities to the driver's
    /// [`CosmosDriverRuntimeBuilder::with_max_error_backtrace_resolutions_per_second`](azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_resolutions_per_second)
    /// /
    /// [`with_max_error_backtrace_captures_per_second`](azure_data_cosmos_driver::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_captures_per_second)
    /// builder methods, or via the corresponding
    /// `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` /
    /// `AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND` environment variables.
    /// Explicit values (including `0` to force-disable) always win over
    /// `RUST_BACKTRACE`.
    pub fn backtrace(&self) -> Option<&Arc<str>> {
        self.0.backtrace()
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
            DriverCosmosError::builder()
                .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("JSON serialization or deserialization failed")
                .with_source(error)
                .build(),
        )
    }
}

impl From<url::ParseError> for CosmosError {
    fn from(error: url::ParseError) -> Self {
        Self(
            DriverCosmosError::builder()
                .with_status(CosmosStatus::CLIENT_INVALID_URL)
                .with_message("invalid URL")
                .with_source(error)
                .build(),
        )
    }
}

/// Per Azure SDK for Rust guideline: every service-crate error type provides a
/// [`From`] impl into [`azure_core::Error`] so callers using the foundation
/// error type via `?`/`From` continue to compose.
///
/// The conversion uses two discriminators that don't require an
/// architectural categorical enum on the Cosmos side:
///
/// 1. [`CosmosError::response`] is the primary signal for "did we get a
///    wire response from Cosmos" — when present, the error maps to
///    [`azure_core::error::ErrorKind::HttpResponse`].
/// 2. Synthetic errors (no wire response) are categorized by their
///    Cosmos sub-status code, which the SDK boundary mapper assigns from
///    a well-known set (`TRANSPORT_*`, `AUTHENTICATION_*`,
///    `SERIALIZATION_*`, `CLIENT_OPERATION_TIMEOUT`). The mapping is
///    intentionally finer than the prior architectural-kind version
///    could express — notably, `TRANSPORT_DNS_FAILED`,
///    `TRANSPORT_CONNECTION_FAILED`, and `TRANSPORT_HTTP2_INCOMPATIBLE`
///    map to [`azure_core::error::ErrorKind::Connection`] because those
///    failure modes provably never sent request bytes (safe to retry
///    non-idempotent writes per `azure_core`'s `Connection` semantics),
///    while generic `TRANSPORT_IO_FAILED` maps to
///    [`azure_core::error::ErrorKind::Io`].
///
/// The original [`CosmosError`] is preserved as the
/// [`azure_core::Error`] source so callers can `downcast_ref::<CosmosError>()`
/// for the typed Cosmos surface.
impl From<CosmosError> for azure_core::Error {
    fn from(err: CosmosError) -> Self {
        let core_kind = classify_for_azure_core(&err);
        azure_core::Error::new(core_kind, err)
    }
}

fn classify_for_azure_core(err: &CosmosError) -> azure_core::error::ErrorKind {
    use azure_core::error::ErrorKind as CoreKind;
    let status = err.status();
    let sub = status.sub_status();

    // Primary discriminator: did we get a wire response from Cosmos?
    if err.0.is_from_wire() {
        return CoreKind::HttpResponse {
            status: status.status_code(),
            error_code: sub.map(|s| s.value().to_string()),
            raw_response: None,
        };
    }

    // Synthetic error — categorize by well-known SDK boundary-mapping
    // sub-status codes.
    match sub {
        // Credential / auth boundary
        Some(SubStatusCode::AUTHENTICATION_TOKEN_ACQUISITION_FAILED)
        | Some(SubStatusCode::CLIENT_GENERATED_401) => CoreKind::Credential,

        // Serialization boundary
        Some(SubStatusCode::SERIALIZATION_RESPONSE_BODY_INVALID) => CoreKind::DataConversion,

        // Request provably NEVER reached the wire — safe to retry non-idempotent writes
        // (matches `azure_core::ErrorKind::Connection` semantics).
        Some(SubStatusCode::TRANSPORT_CONNECTION_FAILED)
        | Some(SubStatusCode::TRANSPORT_DNS_FAILED)
        | Some(SubStatusCode::TRANSPORT_HTTP2_INCOMPATIBLE) => CoreKind::Connection,

        // Generic transport I/O — might have fired mid-stream after request
        // bytes left the socket, so retry safety is `Unknown` (callers should
        // not blindly retry non-idempotent writes).
        Some(SubStatusCode::TRANSPORT_IO_FAILED)
        | Some(SubStatusCode::TRANSPORT_BODY_READ_FAILED)
        | Some(SubStatusCode::TRANSPORT_GENERATED_503)
        | Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT) => CoreKind::Io,

        // Synthetic error with no specific sub_status discriminator —
        // generic client/configuration validation, etc. There's no real
        // HTTP response, so `Other` is more honest than fabricating an
        // `HttpResponse` from a placeholder status code.
        _ => CoreKind::Other,
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
    pub fn with_message(self, message: impl Into<std::borrow::Cow<'static, str>>) -> Self {
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
    pub fn with_context(self, context: impl Into<std::borrow::Cow<'static, str>>) -> Self {
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
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
            .with_message("transport blew up")
            .with_source(inner_io)
            .build();
        let core_err: azure_core::Error = cosmos.into();
        // TRANSPORT_IO_FAILED maps to Io.
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
    fn from_cosmos_error_for_azure_core_error_maps_dns_failure_to_connection() {
        // DNS / connect-refused / H2-incompatibility never sent any bytes
        // on the wire — these map to `Connection`, which `azure_core`
        // documents as safe-to-retry for non-idempotent writes.
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_DNS_FAILED)
            .with_message("dns lookup failed")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(
            matches!(core_err.kind(), CoreErrorKind::Connection),
            "TRANSPORT_DNS_FAILED must map to Connection, got {:?}",
            core_err.kind()
        );
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_maps_auth_to_credential() {
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::AUTHENTICATION_TOKEN_ACQUISITION_FAILED)
            .with_message("token acquisition failed")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(matches!(core_err.kind(), CoreErrorKind::Credential));
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_maps_serialization_to_data_conversion() {
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("bad json")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(matches!(core_err.kind(), CoreErrorKind::DataConversion));
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_synthetic_without_substatus_is_other() {
        // Pure client-validation error: status BadRequest, no sub_status,
        // no wire response. Maps to `Other` — more honest than fabricating
        // an `HttpResponse` from a placeholder status code.
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
            .with_message("bad arg")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(matches!(core_err.kind(), CoreErrorKind::Other));
    }

    #[test]
    fn from_cosmos_error_for_azure_core_error_downcast_recovers_cosmos_error() {
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::new(azure_core::http::StatusCode::BadRequest))
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

    /// Asserts the sibling `Connection` mappings: alongside the
    /// already-tested `TRANSPORT_DNS_FAILED`, `TRANSPORT_CONNECTION_FAILED`
    /// and `TRANSPORT_HTTP2_INCOMPATIBLE` are the other two sub-statuses
    /// that provably never put bytes on the wire and are therefore
    /// safe-to-retry for non-idempotent writes per
    /// `azure_core::ErrorKind::Connection`.
    #[test]
    fn from_cosmos_error_for_azure_core_error_connection_siblings_all_map_to_connection() {
        for status in [
            CosmosStatus::TRANSPORT_CONNECTION_FAILED,
            CosmosStatus::TRANSPORT_HTTP2_INCOMPATIBLE,
        ] {
            let cosmos = CosmosError::builder()
                .with_status(status)
                .with_message("never sent")
                .build();
            let core_err: azure_core::Error = cosmos.into();
            assert!(
                matches!(core_err.kind(), CoreErrorKind::Connection),
                "{:?} must map to Connection, got {:?}",
                status.sub_status(),
                core_err.kind()
            );
        }
    }

    /// Asserts the sibling `Io` mappings: alongside the already-tested
    /// `TRANSPORT_IO_FAILED`, both `TRANSPORT_BODY_READ_FAILED` and
    /// `TRANSPORT_GENERATED_503` map to `Io` (retry safety is `Unknown`
    /// — bytes may have left the socket mid-stream). `CLIENT_OPERATION_TIMEOUT`
    /// is in the same Io bucket; it has no public `CosmosStatus` constant
    /// yet so it is not covered here.
    #[test]
    fn from_cosmos_error_for_azure_core_error_io_siblings_all_map_to_io() {
        for status in [
            CosmosStatus::TRANSPORT_BODY_READ_FAILED,
            CosmosStatus::TRANSPORT_GENERATED_503,
        ] {
            let cosmos = CosmosError::builder()
                .with_status(status)
                .with_message("mid-stream")
                .build();
            let core_err: azure_core::Error = cosmos.into();
            assert!(
                matches!(core_err.kind(), CoreErrorKind::Io),
                "{:?} must map to Io, got {:?}",
                status.sub_status(),
                core_err.kind()
            );
        }
    }

    /// Sibling `Credential` mapping: alongside
    /// `AUTHENTICATION_TOKEN_ACQUISITION_FAILED`, a client-generated 401
    /// (signing / authorization failure prior to the wire) also maps to
    /// `Credential`.
    #[test]
    fn from_cosmos_error_for_azure_core_error_client_generated_401_maps_to_credential() {
        let cosmos = CosmosError::builder()
            .with_status(CosmosStatus::CLIENT_GENERATED_401)
            .with_message("client-side auth failure")
            .build();
        let core_err: azure_core::Error = cosmos.into();
        assert!(
            matches!(core_err.kind(), CoreErrorKind::Credential),
            "CLIENT_GENERATED_401 must map to Credential, got {:?}",
            core_err.kind()
        );
    }
}
