// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The main diagnostics context for tracking operation-level diagnostics.
//!
//! This module contains all core diagnostics types including execution context,
//! request diagnostics, pipeline classification types, request events,
//! serialization helpers, and the diagnostics context itself.

use crate::{
    driver::{pipeline::hedging_diagnostics::HedgeDiagnostics, routing::CosmosEndpoint},
    models::{
        ActivityId, CosmosResponseHeaders, CosmosStatus, PatchTrackingId, RequestCharge,
        SubStatusCode,
    },
    options::{DiagnosticsOptions, DiagnosticsThresholds, DiagnosticsVerbosity, Region},
    system::CpuMemoryMonitor,
};
use azure_core::http::StatusCode;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
    time::{Duration, Instant},
};

use super::compaction::{compact_requests, merge_carried_runs, CompactedRun, CompactionInfo};

// =============================================================================
// Threshold breach classification
// =============================================================================

/// The specific sampling threshold a completed operation crossed.
///
/// Returned by
/// [`DiagnosticsContext::threshold_breach_for`](DiagnosticsContext::threshold_breach_for)
/// so emission handlers can record *why* a diagnostic was sampled (which bound
/// was breached), not just that one was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ThresholdBreach {
    /// Latency exceeded the point-operation latency threshold.
    PointLatency,
    /// Latency exceeded the non-point-operation latency threshold.
    NonPointLatency,
    /// Total request charge exceeded the request-charge (RU) threshold.
    RequestCharge,
}

// =============================================================================
// Execution Context
// =============================================================================

/// Context in which a request was executed.
///
/// This categorizes why a request was made, which is useful for understanding
/// operation patterns and debugging retry/hedging behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ExecutionContext {
    /// Initial request attempt (first try).
    Initial,
    /// An operation-level retry decided by the SDK's client-retry policy.
    ///
    /// Distinguishes user-visible operation retries from transport-layer
    /// retries ([`ExecutionContext::TransportRetry`]).
    OperationRetry,
    /// Transport-level shard retry within the same region.
    ///
    /// The initial attempt failed with a connectivity error and the transport
    /// pipeline retried on a different HTTP/2 shard before escalating to the
    /// operation pipeline.
    TransportRetry,
    /// Hedged request for latency reduction.
    Hedging,
    /// Region failover attempt.
    RegionFailover,
    /// Circuit breaker recovery probe.
    CircuitBreakerProbe,
}

impl ExecutionContext {
    /// Returns the string representation of this execution context.
    pub fn as_str(&self) -> &'static str {
        match self {
            ExecutionContext::Initial => "initial",
            ExecutionContext::OperationRetry => "operation_retry",
            ExecutionContext::TransportRetry => "transport_retry",
            ExecutionContext::Hedging => "hedging",
            ExecutionContext::RegionFailover => "region_failover",
            ExecutionContext::CircuitBreakerProbe => "circuit_breaker_probe",
        }
    }
}

/// A single region the SDK dispatched a request to, tagged with the reason the
/// orchestrator chose to send it.
///
/// Realizes the cross-SDK Hedging Detection API's `RequestedRegion` value type.
/// Returned by [`DiagnosticsContext::requested_regions`]. Region equality is
/// delegated to [`Region`]'s own `PartialEq`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct RequestedRegion {
    /// The region the SDK dispatched to.
    pub region: Region,
    /// The reason the SDK chose this region for this dispatch attempt.
    pub reason: ExecutionContext,
}

impl AsRef<str> for ExecutionContext {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// =============================================================================
// Pipeline Classification Types
// =============================================================================

/// The type of pipeline used to execute a request.
///
/// Cosmos DB operations are routed through different pipelines based on their
/// resource type and operation type. This enum captures which pipeline was used,
/// which is useful for debugging and understanding request behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PipelineType {
    /// Metadata pipeline for control plane operations.
    ///
    /// Used for database, container, throughput, and other management operations.
    /// Has a higher timeout (65 seconds) to accommodate operations that may take
    /// longer to complete.
    Metadata,

    /// Data plane pipeline for document operations.
    ///
    /// Used for CRUD operations on items/documents and queries.
    /// Has a lower timeout (6 seconds) optimized for high-throughput scenarios.
    DataPlane,
}

impl PipelineType {
    /// Returns the string representation of this pipeline type.
    pub fn as_str(self) -> &'static str {
        match self {
            PipelineType::Metadata => "metadata",
            PipelineType::DataPlane => "data_plane",
        }
    }

    /// Returns true if this is a metadata (control plane) pipeline.
    pub fn is_metadata(self) -> bool {
        matches!(self, PipelineType::Metadata)
    }

    /// Returns true if this is a data plane pipeline.
    pub fn is_data_plane(self) -> bool {
        matches!(self, PipelineType::DataPlane)
    }
}

impl std::fmt::Display for PipelineType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for PipelineType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// The transport security mode used for a request.
///
/// This captures whether the request was made with full TLS certificate
/// validation or with relaxed validation for emulator scenarios.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransportSecurity {
    /// Standard secure transport with full certificate validation.
    ///
    /// Used for production endpoints with valid TLS certificates.
    #[default]
    Secure,

    /// Emulator transport with insecure certificate acceptance.
    ///
    /// Used when connecting to the local Cosmos DB emulator, which uses
    /// self-signed certificates that would fail standard validation.
    EmulatorWithInsecureCertificates,
}

/// The concrete transport kind used for a request.
///
/// This distinguishes the standard gateway path from Gateway 2.0
/// routing while keeping TLS/emulator concerns in [`TransportSecurity`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransportKind {
    /// Standard gateway transport.
    #[default]
    Gateway,

    /// Gateway 2.0 transport.
    GatewayV2,
}

impl TransportKind {
    /// Returns the string representation of this transport kind.
    pub fn as_str(self) -> &'static str {
        match self {
            TransportKind::Gateway => "gateway",
            TransportKind::GatewayV2 => "gateway_v2",
        }
    }

    /// Returns true if this request used the standard gateway transport.
    pub fn is_gateway(self) -> bool {
        matches!(self, TransportKind::Gateway)
    }

    /// Returns true if this request used the Gateway 2.0 transport.
    pub fn is_gateway_v2(self) -> bool {
        matches!(self, TransportKind::GatewayV2)
    }
}

impl std::fmt::Display for TransportKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for TransportKind {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// The HTTP protocol version used by the selected transport.
///
/// This makes the negotiated standard gateway protocol visible in diagnostics,
/// which is especially important after a sticky fallback from HTTP/2 to HTTP/1.1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransportHttpVersion {
    /// HTTP/1.1 transport.
    Http11,

    /// HTTP/2 transport.
    Http2,
}

impl TransportHttpVersion {
    /// Returns the string representation of this transport HTTP version.
    pub fn as_str(self) -> &'static str {
        match self {
            TransportHttpVersion::Http11 => "http11",
            TransportHttpVersion::Http2 => "http2",
        }
    }

    /// Returns true if this request used HTTP/1.1.
    pub fn is_http11(self) -> bool {
        matches!(self, TransportHttpVersion::Http11)
    }

    /// Returns true if this request used HTTP/2.
    pub fn is_http2(self) -> bool {
        matches!(self, TransportHttpVersion::Http2)
    }
}

impl std::fmt::Display for TransportHttpVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for TransportHttpVersion {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl TransportSecurity {
    /// Returns the string representation of this transport security mode.
    pub fn as_str(self) -> &'static str {
        match self {
            TransportSecurity::Secure => "secure",
            TransportSecurity::EmulatorWithInsecureCertificates => "emulator_insecure",
        }
    }

    /// Returns true if this is a secure transport.
    pub fn is_secure(self) -> bool {
        matches!(self, TransportSecurity::Secure)
    }

    /// Returns true if this is an emulator transport with insecure certificates.
    pub fn is_emulator(self) -> bool {
        matches!(self, TransportSecurity::EmulatorWithInsecureCertificates)
    }
}

impl std::fmt::Display for TransportSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for TransportSecurity {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Request Sent Status
// =============================================================================

/// Tri-state indicating whether a request was sent on the wire.
///
/// This is critical for retry decisions:
/// - `Sent`: The request was definitely transmitted; non-idempotent operations
///   should not be retried without additional safeguards (etag checks).
/// - `NotSent`: The request definitely was NOT transmitted; safe to retry.
/// - `Unknown`: Cannot determine if request was sent; treat as potentially sent
///   for safety (don't retry non-idempotent operations).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RequestSentStatus {
    /// Request was definitely sent on the wire.
    /// This is confirmed when we receive response headers or the transport
    /// completes successfully.
    Sent,

    /// Request was definitely NOT sent on the wire.
    /// This is confirmed for errors that occur before transmission
    /// (e.g., DNS resolution failure, connection refused).
    NotSent,

    /// Cannot determine if request was sent.
    /// Treat as potentially sent for retry safety.
    #[default]
    Unknown,
}

impl RequestSentStatus {
    /// Returns `true` if the request may have been sent.
    ///
    /// This is conservative: returns `true` for both `Sent` and `Unknown`,
    /// since we must assume `Unknown` might have been sent for retry safety.
    pub fn may_have_been_sent(&self) -> bool {
        !matches!(self, RequestSentStatus::NotSent)
    }

    /// Returns `true` if we know for certain the request was sent.
    pub fn definitely_sent(&self) -> bool {
        matches!(self, RequestSentStatus::Sent)
    }

    /// Returns `true` if we know for certain the request was NOT sent.
    pub fn definitely_not_sent(&self) -> bool {
        matches!(self, RequestSentStatus::NotSent)
    }

    /// Returns the string representation of this request sent status.
    pub fn as_str(&self) -> &'static str {
        match self {
            RequestSentStatus::Sent => "sent",
            RequestSentStatus::NotSent => "not_sent",
            RequestSentStatus::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for RequestSentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for RequestSentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// =============================================================================
// Request Diagnostics
// =============================================================================

/// Diagnostics for a single HTTP request/response pair.
///
/// Each retry, hedged request, or failover produces a separate `RequestDiagnostics`
/// entry in the [`DiagnosticsContext`].
///
/// This type is non-exhaustive and new fields may be added in future releases.
/// Use the getter methods to access field values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RequestDiagnostics {
    /// Context describing why this request was made.
    execution_context: ExecutionContext,

    /// Canonical `db.operation.name` of the operation that issued this request.
    ///
    /// Normally redundant with the owning [`DiagnosticsContext`]'s
    /// `operation_name`, and therefore left unset. It is populated when an
    /// aggregate context spans requests from more than one operation, so the
    /// per-request identity is not lost to the aggregate's single name — today
    /// that means a PATCH, whose requests are the `patch_read_item` and
    /// `patch_replace_item` sub-ops of one caller-facing `patch_item`.
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_optional_shared_str"
    )]
    operation_name: Option<Arc<str>>,

    /// The pipeline type used for this request.
    pipeline_type: PipelineType,

    /// The transport security mode used for this request.
    transport_security: TransportSecurity,

    /// The concrete transport kind used for this request.
    transport_kind: TransportKind,

    /// The HTTP protocol version used by the selected transport.
    transport_http_version: TransportHttpVersion,

    /// Region this request was sent to.
    region: Option<Region>,

    /// Endpoint URI contacted.
    endpoint: String,

    /// Combined HTTP status code and Cosmos sub-status code.
    #[serde(flatten)]
    status: CosmosStatus,

    /// Request charge (RU) for this individual request.
    pub(crate) request_charge: RequestCharge,

    /// Activity ID for this attempt.
    activity_id: Option<ActivityId>,

    /// Session token from response (for session consistency).
    session_token: Option<String>,

    /// Server-side request processing duration in milliseconds (`x-ms-request-duration-ms`).
    server_duration_ms: Option<crate::models::FiniteF64>,

    /// When this request was started.
    #[serde(skip)]
    started_at: Instant,

    /// When this request completed (response received or error).
    #[serde(skip)]
    pub(crate) completed_at: Option<Instant>,

    /// Duration in milliseconds (computed from started_at/completed_at).
    duration_ms: u64,

    /// Pipeline events during this request.
    events: Vec<RequestEvent>,

    /// Transport shard state captured for sharded HTTP/2 requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    transport_shard: Option<TransportShardDiagnostics>,

    /// Prior shard-local transport failures before the final attempt outcome.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    failed_transport_shards: Vec<FailedTransportShardDiagnostics>,

    /// Number of transport-local shard retries performed for this request.
    #[serde(skip_serializing_if = "is_zero_u32")]
    local_shard_retry_count: u32,

    /// Whether this request timed out.
    pub(crate) timed_out: bool,

    /// Whether the request was sent on the wire.
    ///
    /// This is critical for retry decisions:
    /// - `Sent`: Request was transmitted; don't retry non-idempotent operations.
    /// - `NotSent`: Safe to retry any operation.
    /// - `Unknown`: Treat as potentially sent for safety.
    request_sent: RequestSentStatus,

    /// Error message if the request failed.
    error: Option<String>,

    /// Fault injection rule evaluations for this request.
    ///
    /// Populated only when the `fault_injection` feature is enabled and
    /// evaluations are propagated from the [`FaultClient`](crate::fault_injection::FaultClient)
    /// via an [`EvaluationCollector`](crate::fault_injection::EvaluationCollector) attached
    /// to the [`HttpRequest`](crate::driver::transport::cosmos_transport_client::HttpRequest).
    #[cfg(feature = "fault_injection")]
    fault_injection_evaluations: Vec<crate::fault_injection::FaultInjectionEvaluation>,
}

impl RequestDiagnostics {
    /// Creates a new request diagnostics entry for a request being started.
    pub(crate) fn new(
        execution_context: ExecutionContext,
        pipeline_type: PipelineType,
        transport_security: TransportSecurity,
        transport_kind: TransportKind,
        transport_http_version: TransportHttpVersion,
        endpoint: &CosmosEndpoint,
    ) -> Self {
        Self {
            execution_context,
            operation_name: None,
            pipeline_type,
            transport_security,
            transport_kind,
            transport_http_version,
            region: endpoint.region().cloned(),
            endpoint: endpoint.url().as_str().to_owned(),
            // Status is set when the request completes via `complete()`.
            // Using 0 as sentinel value for "not yet completed".
            status: CosmosStatus::new(StatusCode::from(0)),
            request_charge: RequestCharge::default(),
            activity_id: None,
            session_token: None,
            server_duration_ms: None,
            started_at: Instant::now(),
            completed_at: None,
            duration_ms: 0,
            events: Vec::new(),
            transport_shard: None,
            failed_transport_shards: Vec::new(),
            local_shard_retry_count: 0,
            timed_out: false,
            request_sent: RequestSentStatus::Unknown,
            error: None,
            #[cfg(feature = "fault_injection")]
            fault_injection_evaluations: Vec::new(),
        }
    }

    /// **Internal test helper — do not call.**
    ///
    /// Builds a completed [`RequestDiagnostics`] entry with explicit endpoint,
    /// region, status, charge, and start/completion instants, so emission-layer
    /// tests can synthesize realistic (and backdated) attempt spans. Gated
    /// behind the `__internal_test_diagnostics_construction` Cargo feature.
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    pub fn for_testing(
        endpoint: impl Into<String>,
        region: Option<Region>,
        status: CosmosStatus,
        request_charge: RequestCharge,
        started_at: Instant,
        completed_at: Instant,
    ) -> Self {
        let duration_ms = completed_at
            .saturating_duration_since(started_at)
            .as_millis() as u64;
        Self {
            execution_context: ExecutionContext::Initial,
            operation_name: None,
            pipeline_type: PipelineType::DataPlane,
            transport_security: TransportSecurity::Secure,
            transport_kind: TransportKind::Gateway,
            transport_http_version: TransportHttpVersion::Http2,
            region,
            endpoint: endpoint.into(),
            status,
            request_charge,
            activity_id: None,
            session_token: None,
            server_duration_ms: None,
            started_at,
            completed_at: Some(completed_at),
            duration_ms,
            events: Vec::new(),
            transport_shard: None,
            failed_transport_shards: Vec::new(),
            local_shard_retry_count: 0,
            timed_out: false,
            request_sent: RequestSentStatus::Sent,
            error: None,
            #[cfg(feature = "fault_injection")]
            fault_injection_evaluations: Vec::new(),
        }
    }

    /// **Internal test helper — do not call.**
    ///
    /// Stamps this attempt with the sub-operation that issued it, mirroring what
    /// [`DiagnosticsContext::aggregate_sub_operations`] does in production. Lets
    /// emission-layer tests exercise per-request naming without reaching into
    /// the driver's crate-private aggregation path.
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    #[must_use]
    pub fn for_testing_with_operation_name(mut self, operation_name: impl Into<Arc<str>>) -> Self {
        self.operation_name = Some(operation_name.into());
        self
    }

    /// **Internal test helper — do not call.**
    ///
    /// Overrides the [`ExecutionContext`] of a test-constructed request so
    /// emission-layer tests can synthesize a speculative hedge leg (or a retry
    /// / failover dispatch). Gated behind the
    /// `__internal_test_diagnostics_construction` Cargo feature and
    /// `#[doc(hidden)]`, mirroring [`for_testing`](Self::for_testing).
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    #[must_use]
    pub fn with_execution_context_for_testing(
        mut self,
        execution_context: ExecutionContext,
    ) -> Self {
        self.execution_context = execution_context;
        self
    }

    /// Records completion of this request.
    ///
    /// Since we received a response, the request was definitely sent.
    pub(crate) fn complete(&mut self, status_code: StatusCode, sub_status: Option<SubStatusCode>) {
        self.completed_at = Some(Instant::now());
        self.status = CosmosStatus::new(status_code);
        if let Some(sub_status) = sub_status {
            self.with_sub_status(sub_status);
        }
        // Clear any prior failure state. In the current pipeline each attempt
        // gets its own RequestDiagnostics, so `error` and `timed_out` should
        // always be their initial values here. These resets are defensive:
        // they ensure a valid state if a future flow (e.g., shard retry)
        // reuses a handle after a transport-level failure on the same attempt.
        self.error = None;
        self.timed_out = false;
        self.request_sent = RequestSentStatus::Sent;
        self.duration_ms = self
            .completed_at
            .unwrap()
            .duration_since(self.started_at)
            .as_millis() as u64;
    }

    /// Records end-to-end timeout of this request.
    ///
    /// Sets the status to 408 (Request Timeout) with sub-status
    /// [`SubStatusCode::CLIENT_OPERATION_TIMEOUT`] to indicate an end-to-end
    /// operation timeout from the client side.
    pub(crate) fn timeout(&mut self) {
        self.completed_at = Some(Instant::now());
        self.timed_out = true;
        self.status = CosmosStatus::from_parts(
            StatusCode::RequestTimeout,
            Some(SubStatusCode::CLIENT_OPERATION_TIMEOUT),
        );
        self.duration_ms = self
            .completed_at
            .unwrap()
            .duration_since(self.started_at)
            .as_millis() as u64;
    }

    /// Records a transport-level failure using the synthetic Cosmos status
    /// used across SDKs for client-generated gateway transport errors.
    pub(crate) fn fail_transport(
        &mut self,
        error: impl Into<String>,
        request_sent: RequestSentStatus,
        status: CosmosStatus,
    ) {
        self.completed_at = Some(Instant::now());
        self.status = status;
        self.with_error(error);
        self.request_sent = request_sent;
        self.timed_out = false;
        self.duration_ms = self
            .completed_at
            .unwrap()
            .duration_since(self.started_at)
            .as_millis() as u64;
    }

    /// Records an error for this request.
    pub(crate) fn with_error(&mut self, error: impl Into<String>) {
        self.error = Some(error.into());
    }

    /// Sets the sub-status code.
    pub(crate) fn with_sub_status(&mut self, sub_status: SubStatusCode) {
        self.status = CosmosStatus::from_parts(self.status.status_code(), Some(sub_status));
    }

    /// Sets the request charge.
    pub(crate) fn with_charge(&mut self, charge: RequestCharge) {
        self.request_charge = charge;
    }

    /// Sets the activity ID.
    pub(crate) fn with_activity_id(&mut self, activity_id: ActivityId) {
        self.activity_id = Some(activity_id);
    }

    /// Sets the session token.
    pub(crate) fn with_session_token(&mut self, token: String) {
        self.session_token = Some(token);
    }

    /// Sets the server-side request duration in milliseconds.
    pub(crate) fn with_server_duration_ms(&mut self, duration: f64) {
        self.server_duration_ms = Some(crate::models::FiniteF64::new_lossy(duration));
    }

    /// Adds a pipeline event.
    pub(crate) fn add_event(&mut self, event: RequestEvent) {
        self.events.push(event);
    }

    pub(crate) fn set_transport_shard(&mut self, transport_shard: TransportShardDiagnostics) {
        self.transport_shard = Some(transport_shard);
    }

    pub(crate) fn add_failed_transport_shard(
        &mut self,
        failed_transport_shard: FailedTransportShardDiagnostics,
    ) {
        self.failed_transport_shards.push(failed_transport_shard);
    }

    pub(crate) fn increment_local_shard_retry_count(&mut self) {
        self.local_shard_retry_count = self.local_shard_retry_count.saturating_add(1);
    }

    /// Returns whether this request has been completed.
    pub(crate) fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    /// Returns whether this request received an actual service response.
    ///
    /// `completed_at` alone is insufficient: the driver also sets it for
    /// client-side end-to-end timeouts ([`timeout`](Self::timeout)) and
    /// transport-level failures ([`fail_transport`](Self::fail_transport)).
    /// This predicate excludes those two cases so that only requests that
    /// produced a service reply (any HTTP status, including non-2xx) are
    /// counted. Used by [`DiagnosticsContext::responded_regions`].
    pub(crate) fn responded_with_service_reply(&self) -> bool {
        self.region.is_some()
            && self.completed_at.is_some()
            && !self.timed_out
            && self.error.is_none()
    }

    // Public getters for read-only access to fields

    /// Returns the execution context describing why this request was made.
    pub fn execution_context(&self) -> ExecutionContext {
        self.execution_context
    }

    /// Returns the canonical `db.operation.name` of the operation that issued
    /// this request, when it differs from the owning context's operation name.
    ///
    /// This is set only where a single [`DiagnosticsContext`] aggregates
    /// requests from more than one operation. Today that is the PATCH handler:
    /// the context reports the caller-facing `patch_item` while its requests
    /// report the `patch_read_item` / `patch_replace_item` sub-op that produced
    /// them. `None` — the common case — means the request shares the owning
    /// context's [`operation_name`](DiagnosticsContext::operation_name).
    pub fn operation_name(&self) -> Option<&str> {
        self.operation_name.as_deref()
    }

    /// Returns the pipeline type used for this request.
    pub fn pipeline_type(&self) -> PipelineType {
        self.pipeline_type
    }

    /// Returns the transport security mode used for this request.
    pub fn transport_security(&self) -> TransportSecurity {
        self.transport_security
    }

    /// Returns the concrete transport kind used for this request.
    pub fn transport_kind(&self) -> TransportKind {
        self.transport_kind
    }

    /// Returns the HTTP protocol version used by the selected transport.
    pub fn transport_http_version(&self) -> TransportHttpVersion {
        self.transport_http_version
    }

    /// Returns the region this request was sent to.
    pub fn region(&self) -> Option<&Region> {
        self.region.as_ref()
    }

    /// Returns the endpoint URI contacted.
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Returns the combined HTTP status and sub-status code.
    pub fn status(&self) -> &CosmosStatus {
        &self.status
    }

    /// Returns the request charge (RU) for this individual request.
    pub fn request_charge(&self) -> RequestCharge {
        self.request_charge
    }

    /// Returns the activity ID from response headers, if present.
    pub fn activity_id(&self) -> Option<&ActivityId> {
        self.activity_id.as_ref()
    }

    /// Returns the session token from response, if present.
    pub fn session_token(&self) -> Option<&str> {
        self.session_token.as_deref()
    }

    /// Returns the server-side request processing duration in milliseconds, if available.
    pub fn server_duration_ms(&self) -> Option<f64> {
        self.server_duration_ms.map(|f| f.value())
    }

    /// Returns when this request was started.
    pub fn started_at(&self) -> Instant {
        self.started_at
    }

    /// Returns when this request completed, if it has completed.
    pub fn completed_at(&self) -> Option<Instant> {
        self.completed_at
    }

    /// Returns the duration in milliseconds.
    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    /// Returns the pipeline events during this request.
    pub fn events(&self) -> &[RequestEvent] {
        &self.events
    }

    /// Returns the sharded transport state for the shard used by this request, if present.
    pub fn transport_shard(&self) -> Option<&TransportShardDiagnostics> {
        self.transport_shard.as_ref()
    }

    /// Returns prior shard-local failures recorded before the final attempt outcome.
    pub fn failed_transport_shards(&self) -> &[FailedTransportShardDiagnostics] {
        &self.failed_transport_shards
    }

    /// Returns how many shard-local transport retries were performed.
    pub fn local_shard_retry_count(&self) -> u32 {
        self.local_shard_retry_count
    }

    /// Returns whether this request timed out.
    pub fn timed_out(&self) -> bool {
        self.timed_out
    }

    /// Returns whether the request was sent on the wire.
    pub fn request_sent(&self) -> RequestSentStatus {
        self.request_sent
    }

    /// Returns the error message if the request failed.
    pub fn error(&self) -> Option<&str> {
        self.error.as_deref()
    }

    /// Returns fault injection rule evaluations for this request.
    ///
    /// Each entry describes why a rule was applied, skipped, or missed.
    /// Only populated when the `fault_injection` feature is enabled.
    #[cfg(feature = "fault_injection")]
    pub fn fault_injection_evaluations(
        &self,
    ) -> &[crate::fault_injection::FaultInjectionEvaluation] {
        &self.fault_injection_evaluations
    }

    /// Sets the fault injection evaluations for this request.
    #[cfg(feature = "fault_injection")]
    pub(crate) fn set_fault_injection_evaluations(
        &mut self,
        evaluations: Vec<crate::fault_injection::FaultInjectionEvaluation>,
    ) {
        self.fault_injection_evaluations = evaluations;
    }
}

/// Handle for tracking a request within [`DiagnosticsContext`].
///
/// This is an opaque index used to reference a specific request's diagnostics
/// for updates during request execution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RequestHandle(usize);

// =============================================================================
// Request Events
// =============================================================================

// # Reqwest Limitations
//
// Unlike Reactor Netty (used in the Java SDK), reqwest does not expose fine-grained
// connection lifecycle callbacks. We cannot directly track:
// - DNS resolution time (separate from connection time)
// - Connection pool acquisition vs new connection creation
// - TLS handshake time
// - Time to first byte after request sent
//
// What we **can** track:
// - Request start/end timing
// - Total elapsed time
// - Error categorization (connection refused, DNS failure, timeout, etc.)
// - Whether the request was likely sent before failure (for retry safety)
//
// # Future Improvements
//
// To get more granular metrics, we would need to either:
// 1. Use `hyper` directly with custom connectors
// 2. Subscribe to `tracing` events emitted by hyper/reqwest internals
// 3. Implement a custom `tower::Service` layer via `connector_layer`

/// The type of event in the request lifecycle.
///
/// These events track key milestones during HTTP request processing.
/// Note: Due to reqwest's high-level abstraction, we cannot track fine-grained
/// connection events (DNS, TLS handshake) separately. We track what we can observe.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RequestEventType {
    /// Normal routing replaced an operation-specific preferred route.
    RoutingFallback,

    /// Request sent to transport - we're now waiting for the HTTP client.
    /// From here, reqwest handles DNS, connection, TLS, and sending internally.
    /// We cannot distinguish these phases with reqwest's current API.
    TransportStart,

    /// Response headers received from the server.
    /// Emitted when `transport.send().await` returns `Ok(response)`.
    /// At this point, the response body is still a stream - not yet buffered.
    ResponseHeadersReceived,

    /// Transport fully completed - response headers received AND body buffered.
    /// Emitted after `try_into_raw_response().await` succeeds.
    TransportComplete,

    /// Transport failed - an error occurred during the request.
    /// The `details` field contains the error message.
    /// Use error analysis to determine if the request was likely sent.
    TransportFailed,
}

impl RequestEventType {
    /// Returns the string representation of the event type.
    pub fn as_str(&self) -> &str {
        match self {
            Self::RoutingFallback => "routing_fallback",
            Self::TransportStart => "transport_start",
            Self::ResponseHeadersReceived => "response_headers_received",
            Self::TransportComplete => "transport_complete",
            Self::TransportFailed => "transport_failed",
        }
    }

    /// Returns true if this event indicates the request was sent on the wire.
    ///
    /// For retry safety:
    /// - `ResponseHeadersReceived`, `TransportComplete` = definitely sent
    /// - `TransportFailed` = depends on error analysis (see `RequestSentExt` in
    ///   `tracked_transport.rs` which inspects the error type)
    /// - `TransportStart` = not yet sent (in progress)
    pub fn indicates_request_sent(&self) -> bool {
        matches!(
            self,
            Self::ResponseHeadersReceived | Self::TransportComplete
        )
    }
}

impl std::fmt::Display for RequestEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for RequestEventType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// An event in the request pipeline lifecycle.
///
/// Events are recorded at key points during request processing to enable
/// detailed timing analysis and debugging.
///
/// This type is non-exhaustive and new fields may be added in future releases.
/// Use the getter methods to access field values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RequestEvent {
    /// Type of the pipeline event.
    event_type: RequestEventType,

    /// When this event occurred.
    #[serde(skip)]
    timestamp: Instant,

    /// Duration of this stage, if applicable.
    duration_ms: Option<u64>,

    /// Additional context for this event.
    details: Option<String>,
}

/// Captured state for the HTTP/2 shard used by a request.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct TransportShardDiagnostics {
    shard_id: u64,
    /// Approximate inflight count at the time of capture. This is read from an
    /// atomic counter outside the shard's state mutex, so it may be slightly
    /// inconsistent with other fields.
    estimated_inflight: u32,
    consecutive_failures: u32,
    total_requests: u64,
    total_failures: u64,
    /// Requests started but never finished (e.g., cancelled by a timeout race).
    total_cancellations: u64,
    marked_for_eviction: bool,
}

impl TransportShardDiagnostics {
    pub(crate) fn new(
        shard_id: u64,
        estimated_inflight: u32,
        consecutive_failures: u32,
        total_requests: u64,
        total_failures: u64,
        total_cancellations: u64,
        marked_for_eviction: bool,
    ) -> Self {
        Self {
            shard_id,
            estimated_inflight,
            consecutive_failures,
            total_requests,
            total_failures,
            total_cancellations,
            marked_for_eviction,
        }
    }

    pub fn shard_id(&self) -> u64 {
        self.shard_id
    }

    pub fn estimated_inflight(&self) -> u32 {
        self.estimated_inflight
    }

    pub fn consecutive_failures(&self) -> u32 {
        self.consecutive_failures
    }

    pub fn total_requests(&self) -> u64 {
        self.total_requests
    }

    pub fn total_failures(&self) -> u64 {
        self.total_failures
    }

    pub fn total_cancellations(&self) -> u64 {
        self.total_cancellations
    }

    pub fn marked_for_eviction(&self) -> bool {
        self.marked_for_eviction
    }
}

/// Captured diagnostics for a shard that failed before a local shard retry.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct FailedTransportShardDiagnostics {
    #[serde(flatten)]
    transport_shard: TransportShardDiagnostics,
    request_sent: RequestSentStatus,
    error: String,
}

impl FailedTransportShardDiagnostics {
    pub(crate) fn new(
        transport_shard: TransportShardDiagnostics,
        request_sent: RequestSentStatus,
        error: impl Into<String>,
    ) -> Self {
        Self {
            transport_shard,
            request_sent,
            error: error.into(),
        }
    }

    pub fn transport_shard(&self) -> &TransportShardDiagnostics {
        &self.transport_shard
    }

    pub fn request_sent(&self) -> RequestSentStatus {
        self.request_sent
    }

    pub fn error(&self) -> &str {
        &self.error
    }
}

fn is_zero_u32(value: &u32) -> bool {
    *value == 0
}

/// Serializes an `Option<Arc<str>>` as a plain optional string.
///
/// `serde` only implements `Serialize` for `Arc<T>` under its `rc` feature,
/// which this crate does not enable, so the shared string is written through
/// its `str` view instead.
fn serialize_optional_shared_str<S>(
    value: &Option<Arc<str>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(value) => serializer.serialize_str(value),
        None => serializer.serialize_none(),
    }
}

impl RequestEvent {
    /// Creates a new request event.
    pub fn new(event_type: RequestEventType) -> Self {
        Self {
            event_type,
            timestamp: Instant::now(),
            duration_ms: None,
            details: None,
        }
    }

    /// Creates a request event with duration.
    pub fn with_duration(event_type: RequestEventType, duration: Duration) -> Self {
        Self {
            event_type,
            timestamp: Instant::now(),
            duration_ms: Some(duration.as_millis() as u64),
            details: None,
        }
    }

    /// Adds details to the event.
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    // Public getters for read-only access to fields

    /// Returns the type of the pipeline event.
    pub fn event_type(&self) -> &RequestEventType {
        &self.event_type
    }

    /// Returns when this event occurred.
    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }

    /// Returns the duration of this stage in milliseconds, if applicable.
    pub fn duration_ms(&self) -> Option<u64> {
        self.duration_ms
    }

    /// Returns additional context for this event, if present.
    pub fn details(&self) -> Option<&str> {
        self.details.as_deref()
    }
}

// =============================================================================
// JSON Serialization Structures
// =============================================================================

/// Payload for diagnostics output, varying by verbosity level.
#[derive(Serialize)]
#[serde(untagged)]
enum DiagnosticsPayload<'a> {
    /// Detailed payload containing all individual requests.
    Requests { requests: &'a [RequestDiagnostics] },
    /// Summary payload containing region-level summaries.
    Summary { regions: Vec<RegionSummary> },
}

/// Diagnostics output structure for JSON serialization.
#[derive(Serialize)]
struct DiagnosticsOutput<'a> {
    activity_id: &'a ActivityId,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_tracking_id: Option<&'a PatchTrackingId>,
    total_duration_ms: u64,
    total_request_charge: RequestCharge,
    request_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_usage: Option<SystemUsageSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_id: Option<&'a str>,
    /// Present only when the per-attempt list was compacted under a retry storm;
    /// absent (and thus byte-identical to prior output) otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    compaction: Option<&'a CompactionInfo>,
    #[serde(flatten)]
    payload: DiagnosticsPayload<'a>,
}

/// Summary of requests in a single region.
#[derive(Serialize)]
struct RegionSummary {
    region: String,
    request_count: usize,
    total_request_charge: RequestCharge,
    first: Option<RequestSummary>,
    last: Option<RequestSummary>,
    deduplicated_groups: Vec<DeduplicatedGroup>,
}

/// Summary of a single request.
#[derive(Serialize)]
struct RequestSummary {
    execution_context: ExecutionContext,
    endpoint: String,
    #[serde(flatten)]
    status: CosmosStatus,
    request_charge: RequestCharge,
    duration_ms: u64,
    timed_out: bool,
}

impl From<&RequestDiagnostics> for RequestSummary {
    fn from(req: &RequestDiagnostics) -> Self {
        Self {
            execution_context: req.execution_context,
            endpoint: req.endpoint.clone(),
            status: req.status,
            request_charge: req.request_charge,
            duration_ms: req.duration_ms,
            timed_out: req.timed_out,
        }
    }
}

/// Group of deduplicated similar requests.
#[derive(Serialize)]
struct DeduplicatedGroup {
    endpoint: String,
    #[serde(flatten)]
    status: CosmosStatus,
    execution_context: ExecutionContext,
    count: usize,
    total_request_charge: RequestCharge,
    min_duration_ms: u64,
    max_duration_ms: u64,
    p50_duration_ms: u64,
}

/// Truncated output indicator.
#[derive(Serialize)]
struct TruncatedOutput<'a> {
    activity_id: &'a ActivityId,
    #[serde(skip_serializing_if = "Option::is_none")]
    patch_tracking_id: Option<&'a PatchTrackingId>,
    total_duration_ms: u64,
    request_count: usize,
    truncated: bool,
    /// Present only when the per-attempt list was compacted under a retry storm.
    ///
    /// Counts-only: the per-run rollup (`CompactionInfo::runs`, up to `cap`
    /// endpoint-bearing entries) is deliberately omitted here — it is the
    /// unbounded part that can blow the size budget, and re-serializing it in the
    /// size-limited fallback is exactly what would keep the "truncated" summary
    /// oversized.
    #[serde(skip_serializing_if = "Option::is_none")]
    compaction: Option<CompactionSummary>,
    message: &'static str,
}

/// Counts-only projection of [`CompactionInfo`] for the size-limited truncated
/// summary. Carries the scalar counts (always tiny) but never the per-run rollup.
#[derive(Serialize)]
struct CompactionSummary {
    original_request_count: usize,
    retained_request_count: usize,
    collapsed_runs: usize,
    total_runs: usize,
    retained_truncated: bool,
    omitted_runs: usize,
    omitted_request_count: usize,
}

impl From<&CompactionInfo> for CompactionSummary {
    fn from(info: &CompactionInfo) -> Self {
        Self {
            original_request_count: info.original_request_count,
            retained_request_count: info.retained_request_count,
            collapsed_runs: info.collapsed_runs,
            total_runs: info.total_runs,
            retained_truncated: info.retained_truncated,
            omitted_runs: info.omitted_runs,
            omitted_request_count: info.omitted_request_count,
        }
    }
}

/// Status of the CPU sample history in a [`SystemUsageSnapshot`].
#[derive(Clone, Copy, Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum CpuUsageStatus {
    /// At least one CPU sample was collected.
    Available,
    /// No CPU samples were available (cold start, or the sampler is not running
    /// in the host environment).
    Unavailable,
}

/// Recent CPU load history, serialized as a structured object.
///
/// Replaces the previous behavior of serializing a human-readable string (which
/// emitted the literal `"empty"` sentinel when no samples existed). Mirrors the
/// structured CPU history shape used by the .NET and Java SDKs, so downstream
/// JSON consumers always see an object with a `samples` array and a `status`.
///
/// # Examples
///
/// - With samples: `{ "samples": ["(45.3%)", "(50.1%)"], "status": "available" }`
/// - Without samples: `{ "samples": [], "status": "unavailable" }`
#[derive(Clone, Debug, Serialize)]
struct CpuUsageHistory {
    /// Recent CPU load samples, oldest first (empty when none were collected).
    samples: Vec<String>,
    /// Whether any CPU samples were available.
    status: CpuUsageStatus,
}

impl CpuUsageHistory {
    /// Builds a CPU history from formatted sample strings, deriving the status
    /// from whether any samples are present.
    fn from_samples(samples: Vec<String>) -> Self {
        let status = if samples.is_empty() {
            CpuUsageStatus::Unavailable
        } else {
            CpuUsageStatus::Available
        };
        Self { samples, status }
    }
}

/// Snapshot of system CPU and memory usage at a point in time.
///
/// Captured lazily on first serialization of a [`DiagnosticsContext`] and
/// included in the JSON output under `"system_usage"`.
///
/// Field names mirror the Java SDK's `CosmosDiagnosticsSystemUsageSnapshot`:
/// - `"cpu"` – Recent CPU load history as a structured object (`{ samples, status }`)
/// - `"memory_available_mb"` – Most recent available memory in MB
/// - `"processor_count"` – Number of logical CPUs available to the process
/// - `"cpu_overloaded"` – Whether the CPU is considered overloaded
#[derive(Clone, Debug, Serialize)]
struct SystemUsageSnapshot {
    /// Recent CPU load history as a structured object.
    cpu: CpuUsageHistory,
    /// Available memory in megabytes (most recent sample).
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available_mb: Option<u64>,
    /// Number of logical CPUs available to the process.
    processor_count: usize,
    /// Whether the CPU is considered overloaded (any sample > 90% or scheduling delays).
    cpu_overloaded: bool,
}

impl SystemUsageSnapshot {
    /// Captures a snapshot from the given CPU/memory monitor.
    fn capture(monitor: &CpuMemoryMonitor) -> Self {
        let history = monitor.snapshot();
        Self {
            cpu: CpuUsageHistory::from_samples(history.cpu_sample_strings()),
            memory_available_mb: history.latest_memory_mb(),
            processor_count: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
            cpu_overloaded: history.is_cpu_overloaded(),
        }
    }

    /// Creates a snapshot with fixed, deterministic values for testing.
    #[cfg(test)]
    fn new_for_test(
        cpu_samples: Vec<String>,
        memory_available_mb: Option<u64>,
        processor_count: usize,
        cpu_overloaded: bool,
    ) -> Self {
        Self {
            cpu: CpuUsageHistory::from_samples(cpu_samples),
            memory_available_mb,
            processor_count,
            cpu_overloaded,
        }
    }
}

/// Operation-scoped state shared by the parent [`DiagnosticsContextBuilder`] and
/// every hedge leg cloned from it.
///
/// A hedge race resolves by *structurally dropping* the losing future. Without a
/// shared sink the loser's private builder — including service replies it had
/// already observed, their RU charge, and their timings — disappears with it, so
/// `responded_regions()` would omit real replies and the "exact totals"
/// guarantee would silently not hold. Every leg therefore mirrors each attempt
/// here the moment that attempt reaches a terminal state, tagged with the leg's
/// id; [`DiagnosticsContextBuilder::merge_hedge_attempt`] removes the winning
/// leg's copies (the winner's originals are merged directly), so exactly one
/// copy of every attempt survives into [`DiagnosticsContextBuilder::complete`].
#[derive(Debug, Default)]
struct HedgeJournal {
    /// Next leg id to hand out. Leg ids start at 1; `0` is reserved for the
    /// parent builder, which is never a race participant.
    next_leg_id: u64,
    /// Legs that reached [`DiagnosticsContextBuilder::start_request`] at least
    /// once, so materialization can tell "this leg described itself" from "this
    /// leg was dropped before it dispatched anything".
    dispatched_legs: Vec<u64>,
    /// Terminal attempts mirrored by each leg, tagged with the leg id. What
    /// remains at finalization is exactly the set of attempts that belonged to
    /// legs the race dropped.
    orphaned_attempts: Vec<(u64, RequestDiagnostics)>,
}

/// One leg of a cross-region hedge race, as described to
/// [`DiagnosticsContextBuilder::record_hedge_fanout`].
#[derive(Clone, Debug)]
pub(crate) struct HedgeLegDispatch {
    /// The region the leg was routed to. `None` when the routed endpoint
    /// carries no named region (global-endpoint accounts).
    pub(crate) region: Option<Region>,
    /// Why the orchestrator dispatched this leg to that region.
    pub(crate) reason: ExecutionContext,
    /// The leg builder's journal id, used to detect whether the leg ever
    /// dispatched a request of its own.
    leg_id: u64,
    /// When the leg was launched, on the same clock as
    /// [`RequestDiagnostics::started_at`].
    dispatched_at: Instant,
}

impl HedgeLegDispatch {
    /// Materializes this leg as a requested-region entry, if it was routed to a
    /// named region. Used only to reconstruct a leg that never dispatched.
    fn requested_region(&self) -> Option<RequestedRegion> {
        self.region.clone().map(|region| RequestedRegion {
            region,
            reason: self.reason,
        })
    }
}

/// One cross-region hedge fan-out, recorded on the parent builder at the moment
/// the race is dispatched.
///
/// Legs describe themselves through their own [`RequestDiagnostics`], which the
/// [`HedgeJournal`] keeps alive even for the leg the race drops. This record is
/// the fallback for the one case that leaves no attempt behind: a leg cancelled
/// before it dispatched anything at all — most commonly the alternate, which
/// `select` never polls when the primary is already resolved. It also marks the
/// operation as having fanned out at all, independently of which leg won, of
/// retry-storm compaction, and of sub-operation aggregation.
#[derive(Clone, Debug)]
pub(crate) struct HedgeFanout {
    /// The primary leg of the race.
    primary: HedgeLegDispatch,
    /// The speculative alternate leg of the race.
    alternate: HedgeLegDispatch,
}

/// Internal mutable builder for constructing a [`DiagnosticsContext`].
///
/// This type is used during operation execution to collect diagnostic data.
/// Once the operation completes, call [`complete`](Self::complete) to produce
/// an immutable [`DiagnosticsContext`].
///
/// All methods on this builder are `pub(crate)` as it is an internal type.
#[derive(Debug)]
pub(crate) struct DiagnosticsContextBuilder {
    /// Operation-level activity ID.
    activity_id: ActivityId,

    /// When this operation started.
    started_at: Instant,

    /// All request diagnostics collected during this operation.
    ///
    /// `Vec<T>` in Rust guarantees insertion order, so requests are stored in
    /// the order they were added.
    requests: Vec<RequestDiagnostics>,

    /// Operation-level combined HTTP status and sub-status (final status after retries).
    status: Option<CosmosStatus>,

    /// Reference to diagnostics configuration.
    options: Arc<DiagnosticsOptions>,

    /// CPU/memory monitor for capturing system usage snapshots.
    cpu_monitor: Option<CpuMemoryMonitor>,

    /// Machine identifier (VM ID on Azure, generated UUID otherwise).
    machine_id: Option<Arc<String>>,

    /// Canonical `db.operation.name` for the operation (e.g. `read_item`),
    /// when known. Set by the driver pipeline from
    /// [`CosmosOperation::db_operation_name`](crate::models::CosmosOperation::db_operation_name)
    /// and carried onto the finalized [`DiagnosticsContext::operation_name`].
    operation_name: Option<Arc<str>>,

    /// Whether fault injection is enabled for this operation's runtime.
    #[cfg(feature = "fault_injection")]
    fault_injection_enabled: bool,

    /// Diagnostics from a cross-region hedging execution, if one occurred.
    /// `None` when hedging was not selected for this operation.
    hedge_diagnostics: Option<HedgeDiagnostics>,

    /// Every hedge fan-out dispatched by this operation, in dispatch order.
    ///
    /// Recorded by the hedging orchestrator on the *parent* builder before the
    /// race starts, so a leg dropped before it dispatched anything is still
    /// described. Empty for the overwhelming majority of operations (no
    /// hedging), which keeps the common path allocation-free.
    hedge_fanouts: Vec<HedgeFanout>,

    /// Operation-scoped state shared with every hedge leg cloned from this
    /// builder, or `None` when the operation never hedged.
    ///
    /// Allocated lazily by the first
    /// [`clone_for_hedge_attempt`](Self::clone_for_hedge_attempt), so the
    /// non-hedged path — the overwhelming majority of operations — never pays
    /// for the `Arc`/`Mutex`.
    hedge_journal: Option<Arc<Mutex<HedgeJournal>>>,

    /// This builder's id within [`Self::hedge_journal`]. `0` on the parent
    /// builder, which never races.
    hedge_leg_id: u64,

    /// Test-only override for system usage snapshot, bypassing the CPU monitor.
    #[cfg(test)]
    test_system_usage: Option<SystemUsageSnapshot>,
}

impl DiagnosticsContextBuilder {
    /// Creates a new diagnostics context builder for an operation.
    pub(crate) fn new(activity_id: ActivityId, options: Arc<DiagnosticsOptions>) -> Self {
        Self {
            activity_id,
            started_at: Instant::now(),
            requests: Vec::with_capacity(4), // Expect 1-4 requests in most cases
            status: None,
            options,
            cpu_monitor: None,
            machine_id: None,
            operation_name: None,
            #[cfg(feature = "fault_injection")]
            fault_injection_enabled: false,
            hedge_diagnostics: None,
            hedge_fanouts: Vec::new(),
            hedge_journal: None,
            hedge_leg_id: 0,
            #[cfg(test)]
            test_system_usage: None,
        }
    }

    /// Sets the CPU/memory monitor for system usage snapshots.
    pub(crate) fn set_cpu_monitor(&mut self, monitor: CpuMemoryMonitor) {
        self.cpu_monitor = Some(monitor);
    }

    /// Sets the machine identifier (from [`VmMetadataService`](crate::system::VmMetadataService)).
    pub(crate) fn set_machine_id(&mut self, machine_id: Arc<String>) {
        self.machine_id = Some(machine_id);
    }

    /// Sets the canonical `db.operation.name` for this operation (e.g.
    /// `read_item`). Carried onto the finalized
    /// [`DiagnosticsContext::operation_name`].
    pub(crate) fn set_operation_name(&mut self, name: impl Into<Arc<str>>) {
        self.operation_name = Some(name.into());
    }

    /// Sets the hedging diagnostics for this operation.
    pub(crate) fn set_hedge_diagnostics(&mut self, diagnostics: HedgeDiagnostics) {
        self.hedge_diagnostics = Some(diagnostics);
    }

    /// Records a cross-region hedge fan-out at the moment the race is dispatched.
    ///
    /// Called by the hedging orchestrator on the *parent* builder once both leg
    /// builders exist. Each leg normally describes itself through its own
    /// attempts — the [`HedgeJournal`] keeps those alive even for the leg the
    /// race drops — so this record only materializes a
    /// [`RequestedRegion`](crate::diagnostics::RequestedRegion) for a leg that
    /// was cancelled before it dispatched anything. Each leg's `region` is
    /// `None` when its routed endpoint carries no named region (global-endpoint
    /// accounts), in which case that leg contributes nothing to
    /// [`requested_regions`](DiagnosticsContext::requested_regions) — matching
    /// how a region-less attempt is skipped.
    pub(crate) fn record_hedge_fanout(
        &mut self,
        primary: HedgeLegDispatch,
        alternate: HedgeLegDispatch,
    ) {
        self.hedge_fanouts.push(HedgeFanout { primary, alternate });
    }

    /// Describes this (leg) builder as one side of a hedge race, for
    /// [`record_hedge_fanout`](Self::record_hedge_fanout).
    ///
    /// The leg's `started_at` is its launch instant — `clone_for_hedge_attempt`
    /// stamps it fresh — so it sits on the same clock as every
    /// [`RequestDiagnostics::started_at`] and orders correctly against them.
    pub(crate) fn leg_dispatch(
        &self,
        region: Option<Region>,
        reason: ExecutionContext,
    ) -> HedgeLegDispatch {
        HedgeLegDispatch {
            region,
            reason,
            leg_id: self.hedge_leg_id,
            dispatched_at: self.started_at,
        }
    }

    /// Creates a fresh builder for a single hedge attempt.
    ///
    /// Shares operation-level context (`activity_id`, `options`,
    /// `cpu_monitor`, `machine_id`, fault-injection flag) with the parent
    /// but starts with an empty request list and a fresh `started_at`, so
    /// per-attempt durations measure from launch rather than from
    /// operation start. The winning attempt's requests are merged back
    /// via [`merge_hedge_attempt`](Self::merge_hedge_attempt); the losing
    /// attempt's are recovered from the shared [`HedgeJournal`], which is
    /// allocated here on the first call.
    pub(crate) fn clone_for_hedge_attempt(&mut self) -> Self {
        let journal = self
            .hedge_journal
            .get_or_insert_with(|| Arc::new(Mutex::new(HedgeJournal::default())));
        let leg_id = {
            let mut guard = journal.lock().unwrap_or_else(|e| e.into_inner());
            guard.next_leg_id += 1;
            guard.next_leg_id
        };
        Self {
            activity_id: self.activity_id.clone(),
            started_at: Instant::now(),
            requests: Vec::with_capacity(2),
            status: None,
            options: Arc::clone(&self.options),
            cpu_monitor: self.cpu_monitor.clone(),
            machine_id: self.machine_id.clone(),
            operation_name: self.operation_name.clone(),
            #[cfg(feature = "fault_injection")]
            fault_injection_enabled: self.fault_injection_enabled,
            hedge_diagnostics: None,
            // Fan-out records live on the parent builder only: a leg's own
            // builder is either merged back (winner) or dropped (loser), so
            // recording here would be lost exactly when it matters.
            hedge_fanouts: Vec::new(),
            hedge_journal: Some(Arc::clone(journal)),
            hedge_leg_id: leg_id,
            #[cfg(test)]
            test_system_usage: self.test_system_usage.clone(),
        }
    }

    /// Absorbs a hedge attempt's per-request diagnostics back into the
    /// parent builder.
    ///
    /// Only the request list is moved; the attempt's `status` and
    /// `hedge_diagnostics` are discarded because those operation-level
    /// fields are written directly on the parent.
    ///
    /// The merged leg's mirrored copies are dropped from the shared
    /// [`HedgeJournal`] at the same time: the originals are now owned by the
    /// parent, so leaving the mirror in place would double-count the winner.
    pub(crate) fn merge_hedge_attempt(&mut self, attempt: Self) {
        if let Some(journal) = self.hedge_journal.as_ref() {
            let mut guard = journal.lock().unwrap_or_else(|e| e.into_inner());
            guard
                .orphaned_attempts
                .retain(|(leg_id, _)| *leg_id != attempt.hedge_leg_id);
        }
        if self.requests.is_empty() {
            self.requests = attempt.requests;
        } else {
            self.requests.extend(attempt.requests);
        }
    }

    /// Sets whether fault injection is enabled for this operation's runtime.
    #[cfg(feature = "fault_injection")]
    pub(crate) fn set_fault_injection_enabled(&mut self, enabled: bool) {
        self.fault_injection_enabled = enabled;
    }

    /// Returns whether fault injection is enabled for this operation's runtime.
    #[cfg(feature = "fault_injection")]
    pub(crate) fn fault_injection_enabled(&self) -> bool {
        self.fault_injection_enabled
    }

    /// Returns the operation-level activity ID.
    // TODO(Step 2): remove this allow once Step 2 diagnostics assertions are
    // added in integration tests for operation pipeline retries/failover.
    #[allow(dead_code)]
    pub(crate) fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    /// Returns the number of tracked requests for this operation.
    // TODO(Step 2): remove this allow once Step 2 diagnostics assertions are
    // added in integration tests for operation pipeline retries/failover.
    #[allow(dead_code)]
    pub(crate) fn request_count(&self) -> usize {
        self.requests.len()
    }

    /// Sets the operation-level status codes.
    ///
    /// This should be called when the operation completes to record the
    /// final HTTP status and sub-status codes.
    pub(crate) fn set_operation_status(
        &mut self,
        status_code: StatusCode,
        sub_status_code: Option<SubStatusCode>,
    ) {
        self.status = Some(CosmosStatus::from_parts(status_code, sub_status_code));
    }

    /// Starts tracking a new request and returns a handle for updates.
    ///
    /// This should be called at the beginning of each HTTP request.
    /// The returned [`RequestHandle`] is used to record completion or timeout.
    pub(crate) fn start_request(
        &mut self,
        execution_context: ExecutionContext,
        pipeline_type: PipelineType,
        transport_security: TransportSecurity,
        transport_kind: TransportKind,
        transport_http_version: TransportHttpVersion,
        endpoint: &CosmosEndpoint,
    ) -> RequestHandle {
        let mut request = RequestDiagnostics::new(
            execution_context,
            pipeline_type,
            transport_security,
            transport_kind,
            transport_http_version,
            endpoint,
        );
        request.with_activity_id(self.activity_id.clone());
        let handle = RequestHandle(self.requests.len());
        self.requests.push(request);
        self.journal_dispatch();
        handle
    }

    /// Records that this hedge leg has dispatched at least one request.
    ///
    /// Lets materialization tell a leg that described itself through its own
    /// attempts from one the race cancelled before it dispatched anything —
    /// only the latter needs a synthetic entry from the fan-out record.
    fn journal_dispatch(&mut self) {
        if self.hedge_leg_id == 0 {
            return;
        }
        let Some(journal) = self.hedge_journal.as_ref() else {
            return;
        };
        let mut guard = journal.lock().unwrap_or_else(|e| e.into_inner());
        if !guard.dispatched_legs.contains(&self.hedge_leg_id) {
            guard.dispatched_legs.push(self.hedge_leg_id);
        }
    }

    /// Mirrors a now-terminal attempt into the shared [`HedgeJournal`].
    ///
    /// Attempts are immutable once terminal (`update_request` rejects a
    /// completed handle), so the mirrored copy is final. If this leg goes on to
    /// win the race its copies are dropped again by
    /// [`merge_hedge_attempt`](Self::merge_hedge_attempt); if the race drops
    /// this leg instead, the copy is the only surviving record of a reply the
    /// service really sent.
    ///
    /// An attempt still in flight when the leg is cancelled is genuinely
    /// unobserved — no reply arrived — and is intentionally not recovered.
    fn journal_terminal_attempt(&mut self, handle: RequestHandle) {
        if self.hedge_leg_id == 0 {
            return;
        }
        let Some(journal) = self.hedge_journal.as_ref() else {
            return;
        };
        let Some(request) = self.requests.get(handle.0) else {
            return;
        };
        let mut guard = journal.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .orphaned_attempts
            .push((self.hedge_leg_id, request.clone()));
    }

    /// Records the response headers and completion of a request in one shot.
    ///
    /// Convenience wrapper around [`update_request`](Self::update_request) +
    /// [`complete_request`](Self::complete_request) that copies the standard Cosmos
    /// response-header fields (`request_charge`, `activity_id`, `session_token`,
    /// `server_duration_ms`) onto the request before marking it complete.
    ///
    /// Must be called at most once per [`RequestHandle`]. Subsequent calls
    /// `debug_assert!` (the `update_request` step rejects already-completed
    /// handles) and are no-ops in release builds.
    pub(crate) fn record_response(
        &mut self,
        handle: RequestHandle,
        status_code: StatusCode,
        headers: &CosmosResponseHeaders,
    ) {
        if let Some(request) = self.requests.get(handle.0) {
            debug_assert!(
                !request.is_completed(),
                "record_response called after the request was already completed"
            );
            if request.is_completed() {
                return;
            }
        }
        self.update_request(handle, |req| {
            if let Some(charge) = headers.request_charge {
                req.with_charge(charge);
            }
            if let Some(activity_id) = headers.activity_id.clone() {
                req.with_activity_id(activity_id);
            }
            if let Some(token) = headers.session_token.clone() {
                req.with_session_token(token.to_string());
            }
            if let Some(duration) = headers.server_duration_ms {
                req.with_server_duration_ms(duration);
            }
        });
        self.complete_request(handle, status_code, headers.substatus);
    }

    /// Records completion of a request.
    ///
    /// Should be called when the HTTP response is received.
    pub(crate) fn complete_request(
        &mut self,
        handle: RequestHandle,
        status_code: StatusCode,
        sub_status: Option<SubStatusCode>,
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.complete(status_code, sub_status);
        }
        self.journal_terminal_attempt(handle);
    }

    /// Records end-to-end timeout of a request.
    ///
    /// Should be called when a request times out before receiving a response
    /// due to hitting the end-to-end operation timeout. Sets the status to
    /// 408 (Request Timeout) with sub-status [`SubStatusCode::CLIENT_OPERATION_TIMEOUT`].
    ///
    /// For transport-level timeouts (connection timeouts, etc.), use
    /// [`fail_transport_request`](Self::fail_transport_request) with the
    /// appropriate synthetic Cosmos status.
    pub(crate) fn timeout_request(&mut self, handle: RequestHandle) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.timeout();
        }
        self.journal_terminal_attempt(handle);
    }

    /// Records a transport-level failure for a request that received no Cosmos response.
    pub(crate) fn fail_transport_request(
        &mut self,
        handle: RequestHandle,
        error: impl Into<String>,
        request_sent: RequestSentStatus,
        status: CosmosStatus,
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.fail_transport(error, request_sent, status);
        }
        self.journal_terminal_attempt(handle);
    }

    /// Updates a request's diagnostics with additional data.
    ///
    /// Use this to add response headers data (charge, activity ID, etc.).
    ///
    /// # Panics (debug builds)
    ///
    /// Panics if the request has already been completed via [`complete_request`](Self::complete_request).
    /// In release builds, the update is silently ignored.
    pub(crate) fn update_request(
        &mut self,
        handle: RequestHandle,
        f: impl FnOnce(&mut RequestDiagnostics),
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            debug_assert!(
                !request.is_completed(),
                "update_request called after complete_request - updates should occur before completion"
            );
            if !request.is_completed() {
                f(request);
            }
        }
    }

    /// Adds a pipeline event to a request.
    pub(crate) fn add_event(&mut self, handle: RequestHandle, event: RequestEvent) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.add_event(event);
        }
    }

    pub(crate) fn set_transport_shard(
        &mut self,
        handle: RequestHandle,
        transport_shard: TransportShardDiagnostics,
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.set_transport_shard(transport_shard);
        }
    }

    pub(crate) fn add_failed_transport_shard(
        &mut self,
        handle: RequestHandle,
        failed_transport_shard: FailedTransportShardDiagnostics,
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.add_failed_transport_shard(failed_transport_shard);
        }
    }

    pub(crate) fn increment_local_shard_retry_count(&mut self, handle: RequestHandle) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.increment_local_shard_retry_count();
        }
    }

    /// Sets fault injection evaluations on a request.
    #[cfg(feature = "fault_injection")]
    pub(crate) fn set_fault_injection_evaluations(
        &mut self,
        handle: RequestHandle,
        evaluations: Vec<crate::fault_injection::FaultInjectionEvaluation>,
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.set_fault_injection_evaluations(evaluations);
        }
    }

    /// Completes the builder and returns an immutable [`DiagnosticsContext`].
    ///
    /// This consumes the builder and creates a finalized diagnostics context
    /// with all data frozen. The `DiagnosticsContext` can then be safely
    /// shared via `Arc` without any locking overhead.
    pub(crate) fn complete(mut self) -> DiagnosticsContext {
        let duration = self.started_at.elapsed();

        // Recover the attempts of any hedge leg the race dropped before it
        // could be merged. Those legs may have observed real service replies
        // (a 429 answered just before the partner won, say); losing them would
        // under-report charge, drop a genuine entry from `responded_regions`,
        // and break the exactness of the `total_*_regions` counters.
        //
        // `started_at` is a single operation-wide clock, and every builder
        // appends in start order, so re-sorting the union restores true global
        // dispatch order across the parent and both legs — something no
        // parent-relative index could express while the legs ran concurrently.
        // Handles are never used after `complete` consumes the builder, so
        // reordering here cannot invalidate one.
        let (orphaned, dispatched_legs) = self
            .hedge_journal
            .take()
            .map(|journal| {
                let mut guard = journal.lock().unwrap_or_else(|e| e.into_inner());
                (
                    std::mem::take(&mut guard.orphaned_attempts),
                    std::mem::take(&mut guard.dispatched_legs),
                )
            })
            .unwrap_or_default();
        if !orphaned.is_empty() {
            self.requests
                .extend(orphaned.into_iter().map(|(_, request)| request));
            self.requests.sort_by_key(RequestDiagnostics::started_at);
        }

        // Exact operation-level total charge, summed from the FULL attempt list
        // before any compaction so it stays exact even when the retained list is
        // bounded under a retry storm.
        let total_request_charge: RequestCharge =
            self.requests.iter().map(|r| r.request_charge).sum();

        // Capture the contacted regions in first-contact order from the FULL
        // attempt list, before compaction can drop whole buckets: a region whose
        // only attempts are elided must still surface at the operation level.
        let regions_contacted = ordered_unique_regions(&self.requests);

        // Materialize the Hedging Detection API's region history from the FULL
        // attempt list plus the dispatch-time hedge fan-out log, for the same
        // reason: neither the retained (possibly compacted) attempt list nor the
        // winning leg alone is a complete dispatch history.
        //
        // Both histories are then bounded by the same `max_request_diagnostics`
        // cap as the attempt list, so a retry storm cannot grow them without
        // limit (DIAGNOSTICS-CONTRACT.md §8). The pre-truncation lengths are
        // retained so the truncation is explicit rather than silent.
        let cap = self.options.max_request_diagnostics();
        let requested_regions =
            requested_regions_from(&self.requests, &self.hedge_fanouts, &dispatched_legs);
        let responded_regions = responded_regions_from(&self.requests);
        let total_requested_regions = requested_regions.len();
        let total_responded_regions = responded_regions.len();
        let requested_regions = bound_region_history(requested_regions, cap);
        let responded_regions = bound_region_history(responded_regions, cap);
        let hedging_started = !self.hedge_fanouts.is_empty()
            || self
                .hedge_diagnostics
                .as_ref()
                .is_some_and(|hd| hd.alternate_region().is_some())
            || self
                .requests
                .iter()
                .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging));

        // Bound the finalized per-attempt list under a retry storm.
        //
        // Common path (attempts <= cap): the list is retained verbatim, no
        // `CompactionInfo` is attached, and the serialized output is
        // byte-identical to the pre-compaction behavior.
        //
        // Storm path (attempts > cap): run-length collapse (with a global
        // key-bucket fallback for order ping-pong) bounds the retained records
        // and per-run rollup to `cap`, while a `CompactionInfo` marker records
        // the true attempt count, the exact per-run aggregates, and every drop.
        //
        // Compaction runs here at finalization, never mid-operation, so any
        // outstanding `RequestHandle` indices are never invalidated. The bound
        // is on the finalized serialized artifact, not on live mid-operation
        // memory: `self.requests` still grows one entry per attempt while the
        // operation is in flight.
        let original_count = self.requests.len();
        let (requests, compaction) = if original_count > cap {
            let compacted = compact_requests(self.requests, cap);
            let info = CompactionInfo {
                original_request_count: original_count,
                retained_request_count: compacted.retained.len(),
                collapsed_runs: compacted.collapsed_runs,
                total_runs: compacted.total_runs,
                retained_truncated: compacted.retained_truncated,
                omitted_runs: compacted.omitted_runs,
                omitted_request_count: compacted.omitted_request_count,
                runs: compacted.runs,
            };
            (compacted.retained, Some(info))
        } else {
            (self.requests, None)
        };

        DiagnosticsContext {
            activity_id: self.activity_id,
            duration,
            requests: Arc::new(requests),
            total_request_charge,
            regions_contacted,
            requested_regions,
            responded_regions,
            total_requested_regions,
            total_responded_regions,
            hedging_started,
            status: self.status,
            options: self.options,
            cpu_monitor: self.cpu_monitor,
            machine_id: self.machine_id,
            operation_name: self.operation_name,
            patch_tracking_id: None,
            #[cfg(feature = "fault_injection")]
            fault_injection_enabled: self.fault_injection_enabled,
            #[cfg(not(feature = "fault_injection"))]
            fault_injection_enabled: false,
            hedge_diagnostics: self.hedge_diagnostics,
            compaction,
            #[cfg(test)]
            test_system_usage: self.test_system_usage,
            cached_json_detailed: OnceLock::new(),
            cached_json_summary: OnceLock::new(),
        }
    }

    /// Sets a pre-built system usage snapshot, bypassing the CPU monitor.
    ///
    /// This enables deterministic JSON output in tests by providing
    /// fixed system usage values instead of reading live OS metrics.
    #[cfg(test)]
    fn set_test_system_usage(&mut self, snapshot: SystemUsageSnapshot) {
        self.test_system_usage = Some(snapshot);
    }
}

/// Diagnostic context for a Cosmos DB operation.
///
/// This is an **immutable** type containing detailed information about request execution
/// including RU consumption, regions contacted, retry attempts, and timing information.
///
/// # Immutability
///
/// Once created from a `DiagnosticsContextBuilder`, a `DiagnosticsContext` is fully
/// immutable. All data is frozen at completion time, and no further mutations are possible.
/// This enables lock-free access and efficient sharing via `Arc`.
///
/// # Efficient Multi-Read
///
/// The [`requests`](Self::requests) method returns `Arc<Vec<RequestDiagnostics>>`,
/// allowing multiple readers to share the same allocation without cloning. This is
/// efficient for repeated access patterns.
///
/// # JSON Caching
///
/// JSON serialization via [`to_json_string`](Self::to_json_string) is lazily cached.
/// The first call computes the JSON; subsequent calls return the cached string.
///
/// # JSON Verbosity Levels
///
/// - **Summary**: Optimized for size constraints, deduplicates similar requests
/// - **Detailed**: Full information about every request
#[non_exhaustive]
pub struct DiagnosticsContext {
    /// Operation-level activity ID.
    activity_id: ActivityId,

    /// Total duration of the operation (from start to completion).
    duration: Duration,

    /// All request diagnostics (shared via `Arc` for efficient multi-read).
    ///
    /// `Vec<T>` in Rust guarantees insertion order, so requests are stored in
    /// the order they were added. Under a retry storm this list is compacted at
    /// finalization to at most
    /// [`max_request_diagnostics`](crate::options::DiagnosticsOptions::max_request_diagnostics)
    /// records; see [`compaction`](Self::compaction).
    requests: Arc<Vec<RequestDiagnostics>>,

    /// Total request charge (RU) across **all** attempts.
    ///
    /// Computed from the full attempt list at finalization, before any
    /// compaction, so it stays exact even when `requests` was bounded under a
    /// retry storm.
    total_request_charge: RequestCharge,

    /// Regions contacted during the operation, in first-contact order.
    ///
    /// Captured at finalization from the **full** attempt list — before any
    /// retry-storm compaction — so a region whose only attempts were dropped
    /// from `requests` is still reported. Duplicates are removed while
    /// preserving the order in which each region was first contacted, which the
    /// Cosmos semantic conventions require (it conveys failover order).
    regions_contacted: Vec<Region>,

    /// Regions this operation dispatched a request to, in dispatch order, each
    /// tagged with the reason the SDK chose it.
    ///
    /// Materialized at finalization from the **full** attempt list — before any
    /// retry-storm compaction — with every hedge fan-out spliced in at the point
    /// it was dispatched, so a structurally-dropped hedge loser leg and a
    /// compacted-away retry are both still reported. Unlike `regions_contacted`
    /// this list is *not* deduplicated: it is a dispatch log, so repeat
    /// dispatches to the same region each contribute an entry.
    ///
    /// Bounded by `max_request_diagnostics` (head + tail retained, repetitive
    /// middle elided) so a retry storm cannot grow it without limit;
    /// `total_requested_regions` records the pre-truncation length.
    requested_regions: Vec<RequestedRegion>,

    /// Regions that produced an actual service reply, in arrival (completion)
    /// order.
    ///
    /// Materialized at finalization from the **full** attempt list, for the same
    /// reason as `requested_regions`, and bounded the same way. Client-side
    /// timeouts and transport failures are excluded; a non-2xx service response
    /// still counts.
    responded_regions: Vec<Region>,

    /// Exact number of dispatches recorded before `requested_regions` was
    /// bounded. Equal to `requested_regions.len()` when no truncation occurred.
    total_requested_regions: usize,

    /// Exact number of service replies recorded before `responded_regions` was
    /// bounded. Equal to `responded_regions.len()` when no truncation occurred.
    total_responded_regions: usize,

    /// Whether this operation actually fanned out at least one hedge request.
    ///
    /// Materialized at finalization so it survives compaction dropping the
    /// hedge attempts and aggregation retaining only one representative
    /// `hedge_diagnostics`.
    hedging_started: bool,

    /// Operation-level combined HTTP status and sub-status (final status after retries).
    status: Option<CosmosStatus>,

    /// Reference to diagnostics configuration.
    options: Arc<DiagnosticsOptions>,

    /// CPU/memory monitor for capturing system usage snapshots on first serialization.
    cpu_monitor: Option<CpuMemoryMonitor>,

    /// Machine identifier (VM ID on Azure, generated UUID otherwise).
    machine_id: Option<Arc<String>>,

    /// Canonical `db.operation.name` for the operation (e.g. `read_item`,
    /// `query_items`), when known.
    ///
    /// This feeds the `db.operation.name` span/log attribute and lets
    /// [`is_threshold_violated`](Self::is_threshold_violated) pick the point vs.
    /// non-point latency threshold. The driver pipeline populates it from
    /// [`CosmosOperation::db_operation_name`](crate::models::CosmosOperation::db_operation_name);
    /// it stays `None` for operations without a canonical name.
    operation_name: Option<Arc<str>>,

    /// Effective duplicate-suppression identity for a tracked PATCH operation.
    patch_tracking_id: Option<PatchTrackingId>,

    /// Whether fault injection was enabled when this operation executed.
    fault_injection_enabled: bool,

    /// Diagnostics from a cross-region hedging execution, if one occurred.
    ///
    /// `Some(_)` if and only if a hedging strategy was resolved and active
    /// for the operation — i.e. `should_hedge()` returned `true` and
    /// `execute_hedged()` was entered (spec §10.1 attachment contract).
    /// `None` in all other cases (no strategy resolved, strategy
    /// `Disabled`, eligibility check failed, etc.).
    hedge_diagnostics: Option<HedgeDiagnostics>,

    /// Test-only override for system usage snapshot, bypassing the CPU monitor.
    #[cfg(test)]
    test_system_usage: Option<SystemUsageSnapshot>,

    /// Compaction metadata, present only when the per-attempt list exceeded the
    /// configured `max_request_diagnostics` cap under a retry storm and was
    /// compacted. `None` for normal operations, where `requests` is the full,
    /// unmodified set of attempts (and the serialized output is byte-identical
    /// to the pre-compaction behavior).
    compaction: Option<CompactionInfo>,

    /// Cached JSON string for detailed verbosity.
    cached_json_detailed: OnceLock<String>,

    /// Cached JSON string for summary verbosity.
    cached_json_summary: OnceLock<String>,
}

/// The per-run rollup for an aggregate of sub-operations, plus its counters.
struct AggregatedRollup {
    runs: Vec<CompactedRun>,
    collapsed_runs: usize,
    total_runs: usize,
    omitted_runs: usize,
}

/// Builds the per-run rollup for an aggregate of sub-operations.
///
/// A source that was itself compacted carries a rollup covering *all* of its
/// attempts, including the records it retained, so recomputing the rollup from
/// the concatenated records would count those twice. Instead the verbatim
/// (uncompacted) sources are rolled up fresh and the compacted sources' rollups
/// are folded in, which keeps every run's true count exact no matter how many
/// times an aggregate is re-aggregated.
fn aggregate_run_rollup(sources: &[Arc<DiagnosticsContext>], cap: usize) -> AggregatedRollup {
    let verbatim: Vec<RequestDiagnostics> = sources
        .iter()
        .filter(|c| c.compaction.is_none())
        .flat_map(|c| c.requests.iter().cloned())
        .collect();
    let fresh = compact_requests(verbatim, cap);
    let source_infos = || sources.iter().filter_map(|c| c.compaction.as_ref());
    let merged = merge_carried_runs(
        fresh.runs,
        source_infos().flat_map(|info| info.runs.iter().cloned()),
        cap,
    );
    // Runs a source already dropped from its own rollup cannot be carried, so
    // they count as both detected and omitted here too.
    let source_omitted_runs: usize = source_infos().map(|info| info.omitted_runs).sum();
    AggregatedRollup {
        runs: merged.runs,
        collapsed_runs: fresh.collapsed_runs
            + source_infos()
                .map(|info| info.collapsed_runs)
                .sum::<usize>(),
        total_runs: fresh.total_runs + merged.extra_total_runs + source_omitted_runs,
        omitted_runs: fresh.omitted_runs + merged.extra_omitted_runs + source_omitted_runs,
    }
}

impl DiagnosticsContext {
    /// **Internal escape hatch — do not call.**
    ///
    /// Synthesizes a placeholder [`DiagnosticsContext`] for legacy SDK code
    /// paths that have not yet been routed through the driver pipeline and
    /// therefore have no real per-operation diagnostics to surface. The
    /// returned context contains only the supplied [`ActivityId`]; all
    /// per-request diagnostics are empty and the operation duration is zero.
    ///
    /// New code MUST obtain its [`DiagnosticsContext`] from a driver
    /// [`CosmosResponse`](crate::models::CosmosResponse). This constructor is
    /// gated behind the `__internal_test_diagnostics_construction` Cargo
    /// feature, which is enabled only by the wrapper SDK
    /// (`azure_data_cosmos`) and is `#[doc(hidden)]` to keep it off the
    /// public surface. It exists solely so the wrapper SDK can finish
    /// migrating its remaining non-driver code paths and will be removed
    /// once that migration is complete.
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    pub fn for_testing(activity_id: ActivityId) -> Self {
        DiagnosticsContextBuilder::new(activity_id, Arc::new(DiagnosticsOptions::default()))
            .complete()
    }

    /// **Internal escape hatch — do not call.**
    ///
    /// Synthesizes a completed [`DiagnosticsContext`] carrying an explicit
    /// operation `duration` and final `status`, so the wrapper SDK
    /// (`azure_data_cosmos`) can exercise emission-layer code paths — such as
    /// the metrics handler — against a realistic operation-level rollup without
    /// standing up the full driver pipeline. Per-request diagnostics remain
    /// empty; only the operation-scope fields the emission layer reads are set.
    ///
    /// Gated behind the `__internal_test_diagnostics_construction` Cargo feature
    /// (enabled only by the wrapper SDK's own test build) and `#[doc(hidden)]`,
    /// so it never appears on the public surface. Mirrors [`for_testing`](Self::for_testing).
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    pub fn for_testing_completed(
        activity_id: ActivityId,
        duration: Duration,
        status: Option<CosmosStatus>,
    ) -> Self {
        let mut context =
            DiagnosticsContextBuilder::new(activity_id, Arc::new(DiagnosticsOptions::default()))
                .complete();
        context.duration = duration;
        context.status = status;
        context
    }

    /// **Internal test helper — do not call.**
    ///
    /// Builds a fully-populated [`DiagnosticsContext`] from explicit parts so
    /// the SDK's emission-layer tests (tracing, sampled logging) can exercise
    /// realistic operations — including backdated per-request timestamps —
    /// without a live driver pipeline. Gated behind the
    /// `__internal_test_diagnostics_construction` Cargo feature and
    /// `#[doc(hidden)]`, mirroring [`for_testing`](Self::for_testing).
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    pub fn for_testing_with_requests(
        activity_id: ActivityId,
        duration: Duration,
        status: Option<CosmosStatus>,
        operation_name: Option<&str>,
        requests: Vec<RequestDiagnostics>,
    ) -> Self {
        // Mirror the pipeline's exact-at-finalization rollup by summing the
        // per-attempt charges supplied by the test.
        let total_request_charge = RequestCharge::new(
            requests
                .iter()
                .map(|r| r.request_charge().value())
                .sum::<f64>(),
        );
        let regions_contacted = ordered_unique_regions(&requests);
        let requested_regions = requested_regions_from(&requests, &[], &[]);
        let responded_regions = responded_regions_from(&requests);
        // The helper builds a fixed, hand-supplied attempt list, so the history
        // is never over the cap and the totals are simply its length.
        let total_requested_regions = requested_regions.len();
        let total_responded_regions = responded_regions.len();
        let hedging_started = requests
            .iter()
            .any(|r| matches!(r.execution_context(), ExecutionContext::Hedging));
        DiagnosticsContext {
            activity_id,
            duration,
            requests: Arc::new(requests),
            total_request_charge,
            regions_contacted,
            requested_regions,
            responded_regions,
            total_requested_regions,
            total_responded_regions,
            hedging_started,
            status,
            options: Arc::new(DiagnosticsOptions::default()),
            cpu_monitor: None,
            machine_id: None,
            operation_name: operation_name.map(Arc::from),
            patch_tracking_id: None,
            fault_injection_enabled: false,
            hedge_diagnostics: None,
            #[cfg(test)]
            test_system_usage: None,
            compaction: None,
            cached_json_detailed: OnceLock::new(),
            cached_json_summary: OnceLock::new(),
        }
    }

    /// **Internal test helper — do not call.**
    ///
    /// Like [`for_testing_with_requests`](Self::for_testing_with_requests), but
    /// also attaches `hedge_diagnostics` so the wrapper SDK's emission-layer
    /// tests can exercise the hedging-surfacing paths (tracing / metrics /
    /// logging). Gated behind the `__internal_test_diagnostics_construction`
    /// Cargo feature and `#[doc(hidden)]`, mirroring [`for_testing`](Self::for_testing).
    #[cfg(feature = "__internal_test_diagnostics_construction")]
    #[doc(hidden)]
    pub fn for_testing_with_hedge(
        activity_id: ActivityId,
        duration: Duration,
        status: Option<CosmosStatus>,
        operation_name: Option<&str>,
        requests: Vec<RequestDiagnostics>,
        hedge_diagnostics: Option<HedgeDiagnostics>,
    ) -> Self {
        // Reconstruct the fan-out the pipeline records at race dispatch, so the
        // materialized region history is built the same way it is in production.
        // The sentinel stands in for a global-endpoint account with no named
        // region, which contributes no requested-region entry.
        let not_sentinel = |region: &Region| {
            (region.as_str() != HedgeDiagnostics::UNKNOWN_REGION_SENTINEL).then(|| region.clone())
        };
        // Both legs are stamped at (or before) the first supplied attempt, so a
        // leg that has to be reconstructed lands at the head of the history —
        // the same position the pipeline would produce for a race dispatched
        // before any attempt was recorded.
        let dispatched_at = requests
            .iter()
            .map(RequestDiagnostics::started_at)
            .min()
            .unwrap_or_else(Instant::now);
        const PRIMARY_LEG: u64 = 1;
        const ALTERNATE_LEG: u64 = 2;
        let fanouts: Vec<HedgeFanout> = hedge_diagnostics
            .as_ref()
            .and_then(|hedge| {
                let alternate = hedge.alternate_region()?;
                Some(HedgeFanout {
                    primary: HedgeLegDispatch {
                        region: not_sentinel(hedge.primary_region()),
                        reason: ExecutionContext::Initial,
                        leg_id: PRIMARY_LEG,
                        dispatched_at,
                    },
                    alternate: HedgeLegDispatch {
                        region: not_sentinel(alternate),
                        reason: ExecutionContext::Hedging,
                        leg_id: ALTERNATE_LEG,
                        dispatched_at,
                    },
                })
            })
            .into_iter()
            .collect();
        // A leg "dispatched" when the supplied attempt list already describes
        // it; only a leg with no attempt of its own needs reconstructing.
        let dispatched_legs: Vec<u64> = fanouts
            .first()
            .map(|fanout| {
                [&fanout.primary, &fanout.alternate]
                    .into_iter()
                    .filter(|leg| {
                        leg.region.as_ref().is_some_and(|region| {
                            requests.iter().any(|request| {
                                request.region() == Some(region)
                                    && request.execution_context() == leg.reason
                            })
                        })
                    })
                    .map(|leg| leg.leg_id)
                    .collect()
            })
            .unwrap_or_default();

        let mut context = Self::for_testing_with_requests(
            activity_id,
            duration,
            status,
            operation_name,
            requests,
        );
        if !fanouts.is_empty() {
            context.requested_regions =
                requested_regions_from(&context.requests, &fanouts, &dispatched_legs);
            context.total_requested_regions = context.requested_regions.len();
            context.hedging_started = true;
        }
        context.hedge_diagnostics = hedge_diagnostics;
        context
    }
    /// sub-operation contexts into a single aggregated [`DiagnosticsContext`].
    ///
    /// Used by the PATCH handler to surface **one operation = one
    /// [`DiagnosticsContext`]** even though the handler internally executes
    /// 2+ pipeline runs (Read + Replace, possibly with 412 retries). Each
    /// source is one sub-op's finalized context; the aggregated context's
    /// `requests` is the concatenation, in input order, of every sub-op's
    /// `RequestDiagnostics`.
    ///
    /// The aggregated context inherits its `activity_id`, `options`,
    /// `cpu_monitor`, `machine_id`, and `fault_injection_enabled` from the
    /// **last** source — which corresponds to the last sub-op the handler
    /// issued and whose status it already surfaces to callers. Operation
    /// `status` likewise comes from the last source. `duration` is the sum
    /// of the sources' durations (sub-ops are issued sequentially), so
    /// callers see a single total time for the operation.
    ///
    /// This method is public only as a doc-hidden diagnostics seam for the
    /// higher-level `azure_data_cosmos` crate.
    ///
    /// Returns `None` only when `sources` is empty.
    #[doc(hidden)]
    pub fn aggregate_sub_operations(sources: &[Arc<DiagnosticsContext>]) -> Option<Self> {
        let last = sources.last()?;
        // Carry each source's operation name down onto the requests it
        // contributed. The aggregate reports a single operation name (the
        // caller-facing one — `patch_item`), so without this the sub-op
        // identity would be lost the moment the contexts are concatenated and
        // every attempt span would inherit the aggregate's name. Sources that
        // are themselves aggregates may already carry per-request names; those
        // are preserved rather than overwritten.
        let aggregated_requests: Vec<RequestDiagnostics> = sources
            .iter()
            .flat_map(|c| {
                c.requests.iter().map(|req| {
                    let mut req = req.clone();
                    if req.operation_name.is_none() {
                        req.operation_name = c.operation_name.clone();
                    }
                    req
                })
            })
            .collect();
        let aggregated_duration = sources
            .iter()
            .map(|c| c.duration)
            .fold(Duration::ZERO, |a, b| a.saturating_add(b));
        // Sum each source's exact total charge (which already accounts for any
        // per-sub-op compaction) rather than re-summing the possibly-compacted
        // concatenated records.
        let total_request_charge: RequestCharge =
            sources.iter().map(|c| c.total_request_charge).sum();

        // Exact total attempts across all sub-ops. Each source's own count is
        // already exact — even if that source was individually compacted — so
        // summing the per-source counts keeps `request_count()` exact on the
        // aggregate instead of collapsing to the retained-record count.
        let original_request_count: usize = sources.iter().map(|c| c.request_count()).sum();
        let cap = last.options.max_request_diagnostics();

        // Re-bound the concatenated retained records so the aggregate artifact
        // stays within the cap regardless of how many sub-ops contributed, and
        // attach a `CompactionInfo` whenever the retained records under-count the
        // true attempts (from re-bounding here or from a sub-op's own
        // compaction) so the exact original count is never lost.
        let (requests, compaction) = if aggregated_requests.len() > cap {
            let compacted = compact_requests(aggregated_requests, cap);
            let retained_request_count = compacted.retained.len();
            let rollup = if sources.iter().any(|c| c.compaction.is_some()) {
                aggregate_run_rollup(sources, cap)
            } else {
                // Nothing to carry, so the rollup just computed over the whole
                // concatenation is already exact — skip the second pass.
                AggregatedRollup {
                    runs: compacted.runs,
                    collapsed_runs: compacted.collapsed_runs,
                    total_runs: compacted.total_runs,
                    omitted_runs: compacted.omitted_runs,
                }
            };
            let info = CompactionInfo {
                original_request_count,
                retained_request_count,
                collapsed_runs: rollup.collapsed_runs,
                total_runs: rollup.total_runs,
                retained_truncated: compacted.retained_truncated,
                omitted_runs: rollup.omitted_runs,
                omitted_request_count: original_request_count
                    .saturating_sub(retained_request_count),
                runs: rollup.runs,
            };
            (compacted.retained, Some(info))
        } else if original_request_count > aggregated_requests.len() {
            // The concatenation fits the cap, but at least one sub-op was itself
            // compacted, so the retained records under-count the true attempts.
            // Attach a counts-only marker carrying the merged per-run rollup so
            // `request_count()` stays exact and the storm shape is preserved.
            let retained_request_count = aggregated_requests.len();
            let rollup = aggregate_run_rollup(sources, cap);
            let info = CompactionInfo {
                original_request_count,
                retained_request_count,
                collapsed_runs: rollup.collapsed_runs,
                total_runs: rollup.total_runs,
                retained_truncated: false,
                omitted_runs: rollup.omitted_runs,
                omitted_request_count: original_request_count
                    .saturating_sub(retained_request_count),
                runs: rollup.runs,
            };
            (aggregated_requests, Some(info))
        } else {
            // No sub-op was compacted and the concatenation fits the cap: the
            // aggregate is exact and verbatim.
            (aggregated_requests, None)
        };

        // First-contact-ordered union of the sub-ops' contacted regions. Each
        // source already captured its exact ordered regions from its full
        // attempt list, so concatenating them in sub-op order (dedup preserving
        // order) yields the operation-level failover order without re-deriving
        // from the possibly-compacted concatenation.
        let mut regions_contacted: Vec<Region> = Vec::new();
        for source in sources {
            for region in &source.regions_contacted {
                if !regions_contacted.contains(region) {
                    regions_contacted.push(region.clone());
                }
            }
        }

        // Concatenate the sub-ops' materialized dispatch histories in sub-op
        // order. Each source already captured its own exact history from its
        // full attempt list plus its own fan-out records, so every sub-op's
        // hedge fan-out is preserved — unlike the single representative
        // `hedge_diagnostics` below, which can only describe one of them.
        //
        // Concatenation is an independent unbounded path — a PATCH conflict loop
        // adds a sub-op per retry, so the aggregate grows with sub-op count even
        // when each source is individually bounded. Re-bound the result under
        // the same cap and carry the summed pre-truncation totals, so the
        // aggregate honours the same guarantee as a single operation.
        //
        // Bounding the concatenation of already-bounded sources compresses the
        // middle twice: a sub-op in the middle can have had its own middle
        // elided, and then be dropped wholesale here. That is intentional, not
        // an oversight, and a proportionally larger cap is deliberately *not*
        // used:
        //
        // - The contract is a flat "the finalized artifact holds at most `cap`
        //   region entries", the same promise a single operation makes. Scaling
        //   the cap with sub-op count would make an aggregate's size grow with
        //   the length of a retry loop — exactly the unbounded growth the bound
        //   exists to prevent.
        // - Nothing exact is lost to it: `total_requested_regions` and
        //   `total_responded_regions` are summed from the sources'
        //   pre-truncation totals, `total_request_charge` and `request_count()`
        //   stay exact via the compaction marker, and `hedging_started` is a
        //   disjunction over the sources, so a fan-out elided from the ordered
        //   view is still reported.
        // - The interesting entries in a conflict loop are the head (how the
        //   operation started) and the tail (where it finally landed), which
        //   head/tail bounding keeps verbatim by construction.
        let total_requested_regions: usize =
            sources.iter().map(|c| c.total_requested_regions).sum();
        let total_responded_regions: usize =
            sources.iter().map(|c| c.total_responded_regions).sum();
        let requested_regions = bound_region_history(
            sources
                .iter()
                .flat_map(|c| c.requested_regions.iter().cloned())
                .collect(),
            cap,
        );
        let responded_regions = bound_region_history(
            sources
                .iter()
                .flat_map(|c| c.responded_regions.iter().cloned())
                .collect(),
            cap,
        );
        let hedging_started = sources.iter().any(|c| c.hedging_started);

        Some(DiagnosticsContext {
            activity_id: last.activity_id.clone(),
            duration: aggregated_duration,
            requests: Arc::new(requests),
            total_request_charge,
            regions_contacted,
            requested_regions,
            responded_regions,
            total_requested_regions,
            total_responded_regions,
            hedging_started,
            status: last.status,
            options: Arc::clone(&last.options),
            cpu_monitor: last.cpu_monitor.clone(),
            machine_id: last.machine_id.clone(),
            operation_name: last.operation_name.clone(),
            patch_tracking_id: last.patch_tracking_id,
            fault_injection_enabled: sources.iter().any(|c| c.fault_injection_enabled),
            // Propagate a representative hedge diagnostics so an aggregated
            // operation (e.g. PATCH, whose internal Read sub-op can itself
            // hedge) still reports hedging consistently: `hedging_started()`,
            // the hedged metric's `hedge_terminal_state` dimension, and the
            // tracing/log hedge fields must not silently drop just because the
            // operation was aggregated. Prefer a sub-op that actually fanned
            // out (an alternate region is present); otherwise fall back to any
            // attached hedge diagnostics. Taking the last match mirrors how the
            // aggregate inherits its operation-level fields from the last
            // sub-op. Limitation: only one representative survives, so if two
            // sub-ops both fanned out to different alternates, `requested_regions`
            // recovers only the representative one's dropped leg.
            hedge_diagnostics: sources
                .iter()
                .rev()
                .find_map(|c| {
                    c.hedge_diagnostics
                        .clone()
                        .filter(|hd| hd.alternate_region().is_some())
                })
                .or_else(|| {
                    sources
                        .iter()
                        .rev()
                        .find_map(|c| c.hedge_diagnostics.clone())
                }),
            compaction,
            #[cfg(test)]
            test_system_usage: last.test_system_usage.clone(),
            cached_json_detailed: OnceLock::new(),
            cached_json_summary: OnceLock::new(),
        })
    }

    /// Returns the operation's activity ID.
    pub fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    /// Returns the operation duration.
    ///
    /// This is the total time from operation start to completion.
    pub fn duration(&self) -> Duration {
        self.duration
    }

    /// Returns the operation-level combined HTTP status and sub-status code.
    ///
    /// This is the final status after all retries and failovers.
    pub fn status(&self) -> Option<&CosmosStatus> {
        self.status.as_ref()
    }

    /// Returns the **effective** final status for the operation.
    ///
    /// This is the operation-level [`status`](Self::status) when one was recorded,
    /// otherwise the terminal attempt's status — mirroring the fallback used by
    /// [`is_failure`](Self::is_failure). Some driver error-finalization paths graft
    /// diagnostics onto the returned error without stamping an operation-level
    /// status; this accessor lets emitters (metrics/tracing) report an accurate
    /// status/`error.type` for those failures instead of treating them as unknown.
    /// `None` only when there is neither an operation status nor any attempt.
    pub fn effective_status(&self) -> Option<CosmosStatus> {
        self.status
            .or_else(|| self.requests.last().map(|request| *request.status()))
    }

    /// Returns the total request charge (RU) across all requests.
    ///
    /// This stays exact even under a retry storm: it is summed from the full
    /// attempt list at finalization, before any compaction of
    /// [`requests`](Self::requests).
    pub fn total_request_charge(&self) -> RequestCharge {
        self.total_request_charge
    }

    /// Returns the number of requests made during this operation.
    ///
    /// This is always the **true** total number of attempts, even when the
    /// per-attempt list was compacted under a retry storm. Use
    /// [`retained_request_count`](Self::retained_request_count) for the number
    /// of records actually retained in [`requests`](Self::requests).
    pub fn request_count(&self) -> usize {
        self.compaction
            .as_ref()
            .map(|c| c.original_request_count)
            .unwrap_or(self.requests.len())
    }

    /// Returns the number of per-attempt records retained in
    /// [`requests`](Self::requests).
    ///
    /// Equal to [`request_count`](Self::request_count) for normal operations;
    /// bounded by the configured
    /// [`max_request_diagnostics`](crate::options::DiagnosticsOptions::max_request_diagnostics)
    /// cap under a retry storm.
    pub fn retained_request_count(&self) -> usize {
        self.requests.len()
    }

    /// Returns compaction metadata when a retry storm exceeded the configured
    /// [`max_request_diagnostics`](crate::options::DiagnosticsOptions::max_request_diagnostics)
    /// cap and the per-attempt list was compacted.
    ///
    /// `None` for normal operations, where the retained list is the full,
    /// unmodified set of attempts.
    pub fn compaction(&self) -> Option<&CompactionInfo> {
        self.compaction.as_ref()
    }

    /// Returns all regions contacted during this operation, in first-contact
    /// order.
    ///
    /// The list is captured at finalization from the **full** attempt list —
    /// before any retry-storm compaction — so a region whose only attempts were
    /// dropped from [`requests`](Self::requests) is still reported here.
    /// Duplicates are removed while preserving the order in which each region was
    /// first contacted, as the Cosmos semantic conventions require for the
    /// contacted-regions attribute (it conveys failover order).
    pub fn regions_contacted(&self) -> Vec<Region> {
        self.regions_contacted.clone()
    }

    /// Returns the regions to which this operation dispatched a request, each
    /// tagged with the reason the SDK chose it.
    ///
    /// Each dispatched attempt with a resolved region contributes one entry.
    /// Duplicates are allowed: the same region may appear more than once if it
    /// was dispatched multiple times (e.g., a retry to the same region, or a
    /// hedge request to a region that was also the primary). The initial attempt
    /// is included and tagged [`ExecutionContext::Initial`].
    ///
    /// **Hedge fan-out recovery.** When a hedge race resolves as a clean win
    /// (the primary wins after the threshold, or the alternate wins outright),
    /// the losing leg's future — and its per-request [`RequestDiagnostics`] — is
    /// structurally dropped before it can be merged, so [`requests`](Self::requests)
    /// holds only the winning leg. Every fan-out is therefore recorded on the
    /// parent at dispatch time, so **both** legs always appear here: the primary
    /// leg tagged with the reason it was actually dispatched under (`Initial` for
    /// a first attempt, or the failover/session reason when the hedge upgraded a
    /// retry), and the alternate leg tagged [`ExecutionContext::Hedging`].
    /// A dropped leg has no corresponding [`responded_regions`](Self::responded_regions)
    /// entry, since it never produced a service reply.
    ///
    /// For an aggregated operation (e.g. PATCH) stitched from multiple
    /// sub-operations, recovery uses the single representative `hedge_diagnostics`
    /// retained by `aggregate_sub_operations`, so if more than one sub-operation
    /// fanned out, only the representative fan-out's dropped leg is recovered.
    ///
    /// Order is dispatch order: each fan-out's two legs are spliced in at the
    /// point the race was dispatched, primary before alternate.
    ///
    /// # Bounded under a retry storm
    ///
    /// The list is capped at `DiagnosticsOptions::max_request_diagnostics`
    /// (default 512). Past that, the head and tail are kept verbatim and the
    /// repetitive middle is elided, so the initial dispatch, any early hedge
    /// fan-out, and the final landing region all survive. Truncation is never
    /// silent: [`total_requested_regions`](Self::total_requested_regions)
    /// reports the exact pre-truncation count, so
    /// `requested_regions().len() < total_requested_regions()` detects it.
    pub fn requested_regions(&self) -> Vec<RequestedRegion> {
        self.requested_regions.clone()
    }

    /// Returns the exact number of dispatches recorded for this operation,
    /// including any elided by the bound on
    /// [`requested_regions`](Self::requested_regions).
    ///
    /// Equal to `requested_regions().len()` unless the history was truncated.
    pub fn total_requested_regions(&self) -> usize {
        self.total_requested_regions
    }

    /// Returns the regions from which this operation received a response, in
    /// arrival (completion) order.
    ///
    /// Each request that produced a service reply contributes one entry.
    /// Duplicates are allowed: the same region may appear more than once if
    /// multiple completed responses arrived from it (e.g., a late hedge
    /// response after the hedge winner). `responded_regions().len() > 1` does
    /// NOT imply more than one distinct region responded.
    ///
    /// Only requests that received an actual service response are included;
    /// client-side timeouts and transport failures are excluded (via the
    /// internal `responded_with_service_reply` predicate on each request).
    /// A non-2xx HTTP status (e.g., 404/429) still counts — it is a response
    /// from the region.
    ///
    /// Unlike [`requested_regions`](Self::requested_regions), this accessor does
    /// **not** recover a structurally-dropped hedge loser leg: on a clean hedge
    /// win the losing leg is cancelled before it produces a service reply, so it
    /// correctly does not appear here. A clean hedge win therefore lists only the
    /// winning region even though `requested_regions()` lists both fan-out legs.
    ///
    /// To deduplicate, callers can collect into a set, for example:
    /// `ctx.responded_regions().into_iter().collect::<std::collections::BTreeSet<_>>()`.
    ///
    /// # Bounded under a retry storm
    ///
    /// Capped the same way as [`requested_regions`](Self::requested_regions);
    /// [`total_responded_regions`](Self::total_responded_regions) reports the
    /// exact pre-truncation count.
    pub fn responded_regions(&self) -> Vec<&Region> {
        self.responded_regions.iter().collect()
    }

    /// Returns the exact number of service replies recorded for this operation,
    /// including any elided by the bound on
    /// [`responded_regions`](Self::responded_regions).
    ///
    /// Equal to `responded_regions().len()` unless the history was truncated.
    pub fn total_responded_regions(&self) -> usize {
        self.total_responded_regions
    }

    /// Returns `true` iff this operation actually dispatched at least one hedge
    /// request (i.e., fan-out occurred), and `false` otherwise.
    ///
    /// `false` does NOT mean hedging was disabled or misconfigured; it means no
    /// fan-out occurred. In particular, when the primary returns before the
    /// hedging threshold elapses, this returns `false` even though a hedging
    /// strategy was active.
    ///
    /// [`hedge_diagnostics`](Self::hedge_diagnostics) is a related but distinct
    /// surface: it is `Some` when the hedge race recorded a terminal outcome —
    /// including the primary-wins-under-threshold case where no fan-out happened
    /// — but it is `None` when hedging was configured yet ineligible, and also
    /// for a both-transient hedge that was subsequently resolved by a failover
    /// attempt (the non-terminal both-transient path deliberately records no
    /// terminal outcome). So `hedge_diagnostics().is_some()` is not a reliable
    /// "was hedging configured" probe, and it can disagree with
    /// `hedging_started()` on that both-transient→failover path (where the
    /// recorded fan-out keeps this accessor `true`). This accessor is the
    /// authoritative fan-out signal: the SDK's hedged metric counter, sampled
    /// log line, and root-span hedging attributes all gate on it, so an
    /// operation that demonstrably fanned out is reported on every surface even
    /// when no terminal outcome was retained.
    pub fn hedging_started(&self) -> bool {
        self.hedging_started
    }

    /// Returns a shared reference to all request diagnostics.
    ///
    /// This returns an `Arc<Vec<RequestDiagnostics>>`, enabling efficient
    /// sharing without cloning the entire vector. Cloning the `Arc` is
    /// a cheap atomic increment (~5 CPU cycles).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let requests = diagnostics.requests();
    /// for req in requests.iter() {
    ///     println!("Request to {} took {}ms", req.endpoint, req.duration_ms);
    /// }
    /// // requests can be stored or passed elsewhere cheaply
    /// ```
    pub fn requests(&self) -> Arc<Vec<RequestDiagnostics>> {
        Arc::clone(&self.requests)
    }

    /// Returns the hedging diagnostics for this operation, if the hedge race
    /// recorded a terminal outcome.
    ///
    /// This is `Some(_)` when the hedge race recorded a terminal outcome —
    /// including the primary-wins-under-threshold case where `execute_hedged()`
    /// ran but no alternate leg fanned out. It is `None` when hedging was not
    /// selected for this operation (no strategy resolved, strategy `Disabled`,
    /// or eligibility check failed), **and also** on the both-transient→failover
    /// path: when both hedge legs return transient failures but the deadline has
    /// not elapsed and failover budget remains, `finalize_both_transient`
    /// deliberately does not stamp a terminal outcome (a later successful retry
    /// would otherwise carry a misleading `BothTransient` state), so a retained
    /// `ExecutionContext::Hedging` request can leave `hedging_started()` `true`
    /// while this returns `None`.
    pub fn hedge_diagnostics(&self) -> Option<&HedgeDiagnostics> {
        self.hedge_diagnostics.as_ref()
    }

    /// Returns the machine identifier, if available.
    ///
    /// On Azure VMs this is `"vmId_{vm-id}"` from IMDS; off Azure it is
    /// `"uuid_{generated-uuid}"` (stable for process lifetime).
    pub fn machine_id(&self) -> Option<&str> {
        self.machine_id.as_ref().map(|s| s.as_str())
    }

    /// Returns whether fault injection was enabled when this operation executed.
    pub fn fault_injection_enabled(&self) -> bool {
        self.fault_injection_enabled
    }

    /// Returns the canonical `db.operation.name` for this operation, if known.
    ///
    /// Values are the semantic-convention operation names such as `read_item`,
    /// `create_item`, or `query_items`. The driver pipeline populates this from
    /// [`CosmosOperation::db_operation_name`](crate::models::CosmosOperation::db_operation_name);
    /// it is `None` only for operations without a canonical name (query plans,
    /// partition-key-range reads, and other internal requests).
    pub fn operation_name(&self) -> Option<&str> {
        self.operation_name.as_deref()
    }

    /// Returns the effective duplicate-suppression identity for this PATCH.
    pub fn patch_tracking_id(&self) -> Option<PatchTrackingId> {
        self.patch_tracking_id
    }

    /// Returns this context stamped with the effective PATCH tracking identity.
    pub(crate) fn with_patch_tracking_id(mut self, id: PatchTrackingId) -> Self {
        self.patch_tracking_id = Some(id);
        self.cached_json_detailed = OnceLock::new();
        self.cached_json_summary = OnceLock::new();
        self
    }

    /// Returns this context with its canonical `db.operation.name` replaced.
    ///
    /// Used by aggregating callers (notably the PATCH handler) that build a
    /// single operation-level context out of sub-operation contexts: the
    /// aggregate would otherwise inherit the *last* sub-op's name (e.g.
    /// `replace_item` for a PATCH's final Replace) instead of the virtual
    /// operation's own name (`patch_item`). Consumes `self` before it is shared
    /// via `Arc`, preserving the type's immutability contract.
    pub(crate) fn with_operation_name(mut self, operation_name: Option<Arc<str>>) -> Self {
        self.requests = Self::preserve_request_operation_names(
            &self.requests,
            self.operation_name.as_ref(),
            operation_name.as_ref(),
        );
        self.operation_name = operation_name;
        self
    }

    /// Pushes a context-level operation name down onto the requests that were
    /// issued under it, so relabeling the context does not erase where its
    /// requests came from.
    ///
    /// Relabeling happens when a virtual operation is assembled from real
    /// sub-operations: a PATCH context is stamped `patch_item`, but its
    /// requests were issued by the `patch_read_item` / `patch_replace_item`
    /// sub-ops. Without this, the attempt-level view would report the
    /// aggregate's name for every request and the read/modify/write
    /// decomposition would be invisible.
    ///
    /// Requests that already carry their own name keep it (they came from a
    /// context that was itself an aggregate). When the name is unchanged, or
    /// there is no displaced name to record, the existing `Arc` is shared
    /// rather than the request list being cloned.
    fn preserve_request_operation_names(
        requests: &Arc<Vec<RequestDiagnostics>>,
        previous: Option<&Arc<str>>,
        replacement: Option<&Arc<str>>,
    ) -> Arc<Vec<RequestDiagnostics>> {
        let Some(previous) = previous else {
            return Arc::clone(requests);
        };
        if replacement.is_some_and(|new| new == previous) {
            return Arc::clone(requests);
        }
        if requests.iter().all(|req| req.operation_name.is_some()) {
            return Arc::clone(requests);
        }
        Arc::new(
            requests
                .iter()
                .map(|req| {
                    let mut req = req.clone();
                    req.operation_name.get_or_insert_with(|| previous.clone());
                    req
                })
                .collect(),
        )
    }

    /// Returns a copy of this context with its canonical `db.operation.name`
    /// replaced, leaving every other field — including status, hedging
    /// diagnostics, and compaction metadata — intact.
    ///
    /// [`with_operation_name`](Self::with_operation_name) consumes `self`, which
    /// works when the caller still owns a freshly aggregated context. Error
    /// paths instead hold an `Arc<DiagnosticsContext>` that a deeper layer
    /// already attached to a [`CosmosError`](crate::error::CosmosError), so they
    /// need to re-stamp the identity without taking ownership. The JSON caches
    /// are intentionally not carried over: they may already have been rendered
    /// with the old name.
    pub(crate) fn clone_with_operation_name(&self, operation_name: Option<Arc<str>>) -> Self {
        DiagnosticsContext {
            activity_id: self.activity_id.clone(),
            duration: self.duration,
            requests: Self::preserve_request_operation_names(
                &self.requests,
                self.operation_name.as_ref(),
                operation_name.as_ref(),
            ),
            total_request_charge: self.total_request_charge,
            regions_contacted: self.regions_contacted.clone(),
            requested_regions: self.requested_regions.clone(),
            responded_regions: self.responded_regions.clone(),
            total_requested_regions: self.total_requested_regions,
            total_responded_regions: self.total_responded_regions,
            hedging_started: self.hedging_started,
            status: self.status,
            options: Arc::clone(&self.options),
            cpu_monitor: self.cpu_monitor.clone(),
            machine_id: self.machine_id.clone(),
            operation_name,
            patch_tracking_id: self.patch_tracking_id,
            fault_injection_enabled: self.fault_injection_enabled,
            hedge_diagnostics: self.hedge_diagnostics.clone(),
            #[cfg(test)]
            test_system_usage: self.test_system_usage.clone(),
            compaction: self.compaction.clone(),
            cached_json_detailed: OnceLock::new(),
            cached_json_summary: OnceLock::new(),
        }
    }

    /// Returns `true` when this context represents a finished operation.
    ///
    /// A [`DiagnosticsContext`] is immutable and finalized at construction, so
    /// any context with a recorded final status or at least one request is
    /// complete. This is the gate the emission handlers check before deciding
    /// whether to emit.
    pub fn is_completed(&self) -> bool {
        self.status.is_some() || !self.requests.is_empty()
    }

    /// Returns `true` when the operation completed with a non-success status.
    ///
    /// Derived from [`status`](Self::status): an operation is a failure when its
    /// final [`CosmosStatus`] is not a success. When no operation-level status was
    /// recorded — some driver error-finalization paths graft diagnostics onto the
    /// returned error without first stamping the operation status — this falls
    /// back to the terminal attempt's status, so a genuine failure still gates the
    /// tail-sampled handlers. A context with neither a status nor any request is
    /// not treated as a failure.
    pub fn is_failure(&self) -> bool {
        match self.status.as_ref() {
            Some(status) => !status.is_success(),
            None => self
                .requests
                .last()
                .is_some_and(|request| !request.status().is_success()),
        }
    }

    /// Returns `true` when the operation crossed one of the sampling
    /// [`thresholds`](DiagnosticsThresholds) — the tail-based sampling signal.
    ///
    /// The latency check uses the point-operation threshold when
    /// [`operation_name`](Self::operation_name) identifies a single-item
    /// operation, the non-point threshold when it identifies a query/batch/etc.,
    /// and — when the operation name is unknown — falls back to the (stricter)
    /// point-operation threshold so genuinely slow operations are still caught.
    ///
    /// The request-charge threshold is compared against the operation's total
    /// RU. The payload-size threshold is not evaluated yet (the context does not
    /// carry body sizes).
    pub fn is_threshold_violated(&self, thresholds: &DiagnosticsThresholds) -> bool {
        self.is_threshold_violated_for(thresholds, None)
    }

    /// Like [`is_threshold_violated`](Self::is_threshold_violated), but takes an
    /// explicit operation name for point/non-point latency classification.
    ///
    /// The driver stamps its own canonical name onto the context, so the
    /// explicit argument is an *override*: the SDK's emission handlers pass the
    /// caller-facing name from the `CosmosOperationContext` so classification
    /// matches the name the caller sees, which also covers operations the driver
    /// leaves unmapped (throughput reads, for instance, whose canonical name is
    /// scope-dependent). When `operation_name` is `None` this falls back to
    /// [`operation_name`](Self::operation_name), then to the stricter point
    /// threshold.
    pub fn is_threshold_violated_for(
        &self,
        thresholds: &DiagnosticsThresholds,
        operation_name: Option<&str>,
    ) -> bool {
        self.threshold_breach_for(thresholds, operation_name)
            .is_some()
    }

    /// Like [`is_threshold_violated_for`](Self::is_threshold_violated_for), but
    /// reports *which* threshold was crossed rather than just whether one was.
    ///
    /// Latency is checked before request charge, so when an operation is both
    /// slow and expensive the latency breach is reported. Returns `None` when no
    /// threshold was crossed. The point/non-point latency threshold is chosen
    /// from `operation_name` exactly as in
    /// [`is_threshold_violated_for`](Self::is_threshold_violated_for).
    pub fn threshold_breach_for(
        &self,
        thresholds: &DiagnosticsThresholds,
        operation_name: Option<&str>,
    ) -> Option<ThresholdBreach> {
        let operation_name = operation_name.or_else(|| self.operation_name());
        let (latency_threshold, latency_breach) = match operation_name {
            Some(name) if crate::options::is_point_operation(name) => (
                thresholds.point_operation_latency(),
                ThresholdBreach::PointLatency,
            ),
            Some(_) => (
                thresholds.non_point_operation_latency(),
                ThresholdBreach::NonPointLatency,
            ),
            None => (
                thresholds.point_operation_latency(),
                ThresholdBreach::PointLatency,
            ),
        };
        if self.duration > latency_threshold {
            return Some(latency_breach);
        }
        if self.total_request_charge().value() > thresholds.request_charge() {
            return Some(ThresholdBreach::RequestCharge);
        }
        None
    }

    /// Serializes diagnostics to a JSON string.
    ///
    /// The result is lazily cached - the first call computes the JSON,
    /// subsequent calls return the cached string (for the same verbosity level).
    ///
    /// # Arguments
    ///
    /// * `verbosity` - Output verbosity level. Pass `None` to use the default from options.
    ///
    /// # Returns
    ///
    /// JSON string representation of diagnostics, truncated in Summary mode to fit
    /// within configured size limits.
    pub fn to_json_string(&self, verbosity: Option<DiagnosticsVerbosity>) -> &str {
        let effective_verbosity = match verbosity.unwrap_or(self.options.default_verbosity()) {
            DiagnosticsVerbosity::Default => self.options.default_verbosity(),
            v => v,
        };

        match effective_verbosity {
            DiagnosticsVerbosity::Default | DiagnosticsVerbosity::Detailed => self
                .cached_json_detailed
                .get_or_init(|| self.compute_json_detailed()),
            DiagnosticsVerbosity::Summary => self
                .cached_json_summary
                .get_or_init(|| self.compute_json_summary(self.options.max_summary_size_bytes())),
        }
    }

    /// Returns the system usage snapshot: test override if set, else captured from the CPU monitor.
    fn resolve_system_usage(&self) -> Option<SystemUsageSnapshot> {
        #[cfg(test)]
        if let Some(snapshot) = &self.test_system_usage {
            return Some(snapshot.clone());
        }
        self.cpu_monitor.as_ref().map(SystemUsageSnapshot::capture)
    }

    fn compute_json_detailed(&self) -> String {
        let total_duration_ms = self.duration.as_millis() as u64;
        let system_usage = self.resolve_system_usage();
        let output = DiagnosticsOutput {
            activity_id: &self.activity_id,
            patch_tracking_id: self.patch_tracking_id.as_ref(),
            total_duration_ms,
            total_request_charge: self.total_request_charge(),
            request_count: self.request_count(),
            system_usage,
            machine_id: self.machine_id.as_ref().map(|s| s.as_str()),
            compaction: self.compaction.as_ref(),
            payload: DiagnosticsPayload::Requests {
                requests: &self.requests,
            },
        };
        serde_json::to_string(&output)
            .unwrap_or_else(|e| serde_json::json!({"error": e.to_string()}).to_string())
    }

    fn compute_json_summary(&self, max_size: usize) -> String {
        let total_duration_ms = self.duration.as_millis() as u64;

        // Group requests by region
        let mut region_groups = HashMap::<Option<Region>, Vec<&RequestDiagnostics>>::new();
        for req in self.requests.iter() {
            region_groups
                .entry(req.region.clone())
                .or_default()
                .push(req);
        }

        // Build summary for each region
        let mut region_summaries = Vec::new();
        for (region, requests) in region_groups {
            region_summaries.push(build_region_summary(region, requests));
        }

        // Sort by region name for deterministic output
        region_summaries.sort_by(|a, b| a.region.cmp(&b.region));

        let output = DiagnosticsOutput {
            activity_id: &self.activity_id,
            patch_tracking_id: self.patch_tracking_id.as_ref(),
            total_duration_ms,
            total_request_charge: self.total_request_charge(),
            request_count: self.request_count(),
            system_usage: self.resolve_system_usage(),
            machine_id: self.machine_id.as_ref().map(|s| s.as_str()),
            compaction: self.compaction.as_ref(),
            payload: DiagnosticsPayload::Summary {
                regions: region_summaries,
            },
        };

        let json = serde_json::to_string(&output)
            .unwrap_or_else(|e| serde_json::json!({"error": e.to_string()}).to_string());

        // Truncate if needed
        if json.len() <= max_size {
            json
        } else {
            // Return a truncated indicator
            let truncated = TruncatedOutput {
                activity_id: &self.activity_id,
                patch_tracking_id: self.patch_tracking_id.as_ref(),
                total_duration_ms,
                request_count: self.request_count(),
                truncated: true,
                compaction: self.compaction.as_ref().map(CompactionSummary::from),
                message:
                    "Output truncated to fit size limit. Use Detailed verbosity for full diagnostics.",
            };
            serde_json::to_string(&truncated)
                .unwrap_or_else(|e| serde_json::json!({"error": e.to_string()}).to_string())
        }
    }
}

impl Clone for DiagnosticsContext {
    fn clone(&self) -> Self {
        Self {
            activity_id: self.activity_id.clone(),
            duration: self.duration,
            requests: Arc::clone(&self.requests),
            total_request_charge: self.total_request_charge,
            regions_contacted: self.regions_contacted.clone(),
            requested_regions: self.requested_regions.clone(),
            responded_regions: self.responded_regions.clone(),
            total_requested_regions: self.total_requested_regions,
            total_responded_regions: self.total_responded_regions,
            hedging_started: self.hedging_started,
            status: self.status,
            options: Arc::clone(&self.options),
            cpu_monitor: self.cpu_monitor.clone(),
            machine_id: self.machine_id.clone(),
            operation_name: self.operation_name.clone(),
            patch_tracking_id: self.patch_tracking_id,
            fault_injection_enabled: self.fault_injection_enabled,
            hedge_diagnostics: self.hedge_diagnostics.clone(),
            compaction: self.compaction.clone(),
            #[cfg(test)]
            test_system_usage: self.test_system_usage.clone(),
            // OnceLock does not implement Clone, so we propagate any cached
            // value into a fresh lock.
            cached_json_detailed: self
                .cached_json_detailed
                .get()
                .cloned()
                .map(OnceLock::from)
                .unwrap_or_default(),
            cached_json_summary: self
                .cached_json_summary
                .get()
                .cloned()
                .map(OnceLock::from)
                .unwrap_or_default(),
        }
    }
}

impl PartialEq for DiagnosticsContext {
    fn eq(&self, other: &Self) -> bool {
        // Compare semantic data only; cached JSON is derived and excluded.
        // `total_request_charge` IS compared: after compaction it is no longer
        // derivable from `requests` (dropped runs still carry charge), so
        // excluding it would let two contexts with a different public
        // `total_request_charge()` result compare equal.
        self.activity_id == other.activity_id
            && self.duration == other.duration
            && self.requests == other.requests
            && self.total_request_charge == other.total_request_charge
            && self.regions_contacted == other.regions_contacted
            && self.requested_regions == other.requested_regions
            && self.responded_regions == other.responded_regions
            // Compared for the same reason as `total_request_charge`: after the
            // region histories are bounded these are no longer derivable from
            // the retained vectors, so excluding them would let two contexts
            // with different public `total_requested_regions()` /
            // `total_responded_regions()` results compare equal.
            && self.total_requested_regions == other.total_requested_regions
            && self.total_responded_regions == other.total_responded_regions
            && self.hedging_started == other.hedging_started
            && self.status == other.status
            && self.options == other.options
            && self.operation_name == other.operation_name
            && self.patch_tracking_id == other.patch_tracking_id
            && self.hedge_diagnostics == other.hedge_diagnostics
            && self.compaction == other.compaction
    }
}

impl Eq for DiagnosticsContext {}

impl std::fmt::Debug for DiagnosticsContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_json_string(Some(DiagnosticsVerbosity::Default)))
    }
}

impl std::fmt::Display for DiagnosticsContext {
    /// `{ctx}` — one-line summary suitable for `tracing` fields and log
    /// lines: `activity=… duration=…ms requests=N charge=…RU [status=…]`.
    ///
    /// `{ctx:#}` — the one-line summary followed by the summarized
    /// diagnostics JSON (`DiagnosticsVerbosity::Summary`). The detailed
    /// JSON remains available via
    /// [`to_json_string`](Self::to_json_string).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "activity={} duration={}ms requests={} charge={}RU",
            self.activity_id(),
            self.duration().as_millis(),
            self.request_count(),
            self.total_request_charge(),
        )?;
        if let Some(status) = self.status() {
            write!(f, " status={status}")?;
        }
        if f.alternate() {
            f.write_str("\n")?;
            f.write_str(self.to_json_string(Some(DiagnosticsVerbosity::Summary)))?;
        }
        Ok(())
    }
}

/// Collects the regions contacted across `requests` in first-contact order.
///
/// Duplicates are removed while preserving the order in which each region was
/// first seen. Unlike a sort+dedup this keeps the failover order intact, which
/// the Cosmos semantic conventions require for the contacted-regions attribute.
fn ordered_unique_regions(requests: &[RequestDiagnostics]) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    for request in requests {
        if let Some(region) = request.region() {
            if !regions.contains(region) {
                regions.push(region.clone());
            }
        }
    }
    regions
}

/// Bounds a materialized region history to `cap` entries, independently of
/// attempt count, per the bounded-size guarantee in `DIAGNOSTICS-CONTRACT.md`
/// §8.
///
/// Under a `410`/`429` retry storm the dispatch history grows one entry per
/// attempt, so it needs its own bound — the per-attempt list's compaction does
/// not apply to it. The repetitive middle of a storm is elided while the head
/// (the initial dispatch and any early hedge fan-out) and the tail (where the
/// operation finally landed) are kept verbatim, mirroring the "first + last of
/// each run" policy `compact_requests` already uses.
///
/// # Deliberately atomicity-oblivious
///
/// This helper takes no stride or grouping parameter, and none is planned. A
/// hedge fan-out contributes two adjacent entries that straddle the head/tail
/// boundary at exactly `cap`; one leg is kept and the other drained, so the
/// bounded view can show a "half" race. That is accepted, because:
///
/// - It only occurs on an operation whose history already exceeds `cap`, i.e.
///   one that has been failing over or storming long enough that a single race
///   in the middle is not the interesting signal.
/// - Nothing exact is lost.
///   [`total_requested_regions`](DiagnosticsContext::total_requested_regions)
///   and [`total_responded_regions`](DiagnosticsContext::total_responded_regions)
///   are computed pre-truncation and stay exact, and
///   [`hedging_started`](DiagnosticsContext::hedging_started) is derived from
///   the fan-out records, not from this list — so "did this operation hedge?"
///   is still answered correctly even if both legs were elided.
/// - Keeping pairs intact would make the output length depend on where the
///   boundary falls, breaking the flat `<= cap` bound this exists to provide.
///
/// The two histories are also *not* index-paired with each other:
/// `requested_regions` is dispatch-ordered and `responded_regions` is
/// arrival-ordered over the subset that actually replied, so they routinely
/// differ in both length and order. Bounding them independently therefore
/// cannot break a correspondence that never existed.
///
/// Truncation is never silent: the caller records the pre-truncation length,
/// which [`DiagnosticsContext::total_requested_regions`] and
/// [`DiagnosticsContext::total_responded_regions`] expose.
fn bound_region_history<T>(mut history: Vec<T>, cap: usize) -> Vec<T> {
    if history.len() <= cap {
        return history;
    }
    let head = cap.div_ceil(2);
    let tail = cap - head;
    history.drain(head..history.len() - tail);
    history
}

/// Builds the dispatch-ordered requested-region history from the **full**
/// (pre-compaction) attempt list.
///
/// `requests` is already in global dispatch order — every builder appends in
/// start order and [`DiagnosticsContextBuilder::complete`] re-sorts the union of
/// the parent's attempts and any recovered hedge-leg attempts by `started_at` —
/// so attempts are emitted verbatim in the order given.
///
/// A hedge leg normally describes itself through its own attempts, including
/// when the race drops it (the [`HedgeJournal`] preserves them). The one leg
/// that leaves nothing behind is one cancelled before it dispatched anything —
/// most commonly the alternate, which `select` never polls when the primary is
/// already resolved. Only such a leg gets a synthetic entry here, positioned by
/// its dispatch instant, which is why `dispatched_legs` is required: it is the
/// set of legs that reached `start_request` at least once.
fn requested_regions_from(
    requests: &[RequestDiagnostics],
    fanouts: &[HedgeFanout],
    dispatched_legs: &[u64],
) -> Vec<RequestedRegion> {
    // Legs that never dispatched anything, in dispatch order. Nothing to
    // reconstruct on the overwhelmingly common non-hedged path.
    let mut silent_legs: Vec<&HedgeLegDispatch> = if fanouts.is_empty() {
        Vec::new()
    } else {
        fanouts
            .iter()
            .flat_map(|fanout| [&fanout.primary, &fanout.alternate])
            .filter(|leg| leg.region.is_some() && !dispatched_legs.contains(&leg.leg_id))
            .collect()
    };
    silent_legs.sort_by_key(|leg| leg.dispatched_at);

    let mut regions: Vec<RequestedRegion> = Vec::with_capacity(requests.len());
    let mut next_silent_leg = 0usize;

    for request in requests {
        // Emit any never-dispatched leg launched at or before this attempt, so
        // the reconstructed entry lands in true dispatch order.
        while let Some(leg) = silent_legs.get(next_silent_leg) {
            if leg.dispatched_at > request.started_at() {
                break;
            }
            regions.extend(leg.requested_region());
            next_silent_leg += 1;
        }

        if let Some(region) = request.region() {
            regions.push(RequestedRegion {
                region: region.clone(),
                reason: request.execution_context(),
            });
        }
    }

    for leg in &silent_legs[next_silent_leg.min(silent_legs.len())..] {
        regions.extend(leg.requested_region());
    }

    regions
}

/// Builds the arrival-ordered responded-region history from the **full**
/// (pre-compaction) attempt list.
///
/// Only attempts that received an actual service reply contribute; a
/// structurally-dropped hedge leg never does, which is why this list can be
/// shorter than [`requested_regions_from`]'s.
fn responded_regions_from(requests: &[RequestDiagnostics]) -> Vec<Region> {
    let mut responded: Vec<&RequestDiagnostics> = requests
        .iter()
        .filter(|r| r.responded_with_service_reply())
        .collect();
    // Stable sort by completion time to yield arrival order while preserving
    // dispatch order among ties.
    responded.sort_by_key(|r| r.completed_at());
    responded
        .iter()
        .filter_map(|r| r.region())
        .cloned()
        .collect()
}

/// Builds a summary for requests in a single region.
fn build_region_summary(
    region: Option<Region>,
    requests: Vec<&RequestDiagnostics>,
) -> RegionSummary {
    let count = requests.len();
    let total_charge: RequestCharge = requests.iter().map(|r| r.request_charge).sum();

    // Keep first and last in full detail
    let first = requests.first().map(|r| RequestSummary::from(*r));
    let last = if count > 1 {
        requests.last().map(|r| RequestSummary::from(*r))
    } else {
        None
    };

    // Deduplicate middle requests
    let middle_requests: Vec<_> = if count > 2 {
        requests[1..count - 1].to_vec()
    } else {
        Vec::new()
    };

    let deduped_groups = deduplicate_requests(middle_requests);

    RegionSummary {
        region: region.as_ref().map(|r| r.to_string()).unwrap_or_default(),
        request_count: count,
        total_request_charge: total_charge,
        first,
        last,
        deduplicated_groups: deduped_groups,
    }
}

/// Key for deduplicating requests.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct DeduplicationKey {
    endpoint: String,
    status: CosmosStatus,
    execution_context: ExecutionContext,
}

/// Deduplicates requests by grouping similar ones.
fn deduplicate_requests(requests: Vec<&RequestDiagnostics>) -> Vec<DeduplicatedGroup> {
    let mut groups = HashMap::<DeduplicationKey, Vec<&RequestDiagnostics>>::new();

    for req in requests {
        let key = DeduplicationKey {
            endpoint: req.endpoint.clone(),
            status: req.status,
            execution_context: req.execution_context,
        };
        groups.entry(key).or_default().push(req);
    }

    groups
        .into_iter()
        .map(|(key, reqs)| {
            let mut durations: Vec<u64> = reqs.iter().map(|r| r.duration_ms).collect();
            durations.sort_unstable();
            let total_charge: RequestCharge = reqs.iter().map(|r| r.request_charge).sum();

            DeduplicatedGroup {
                endpoint: key.endpoint,
                status: key.status,
                execution_context: key.execution_context,
                count: reqs.len(),
                total_request_charge: total_charge,
                min_duration_ms: durations.first().copied().unwrap_or(0),
                max_duration_ms: durations.last().copied().unwrap_or(0),
                p50_duration_ms: percentile_sorted(&durations, 50),
            }
        })
        .collect()
}

/// Calculates the Nth percentile from a **pre-sorted** slice.
///
/// The caller must ensure `values` is sorted in ascending order.
/// This avoids redundant sorting when min, max, and percentiles are all
/// computed from the same data.
pub(crate) fn percentile_sorted(values: &[u64], p: u8) -> u64 {
    if values.is_empty() {
        return 0;
    }
    let index = ((p as f64 / 100.0) * (values.len() - 1) as f64).round() as usize;
    values[index.min(values.len() - 1)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_options() -> Arc<DiagnosticsOptions> {
        Arc::new(DiagnosticsOptions::default())
    }

    fn make_options_with_default_verbosity(
        verbosity: DiagnosticsVerbosity,
    ) -> Arc<DiagnosticsOptions> {
        Arc::new(
            DiagnosticsOptions::builder()
                .with_default_verbosity(verbosity)
                .build()
                .unwrap(),
        )
    }

    /// Helper to create a completed DiagnosticsContext from a builder.
    fn make_context_with<F>(activity_id: ActivityId, f: F) -> DiagnosticsContext
    where
        F: FnOnce(&mut DiagnosticsContextBuilder),
    {
        let mut builder = DiagnosticsContextBuilder::new(activity_id, make_options());
        f(&mut builder);
        builder.complete()
    }

    /// Helper extension trait for test-friendly start_request.
    trait TestBuilderExt {
        fn start_test_request(
            &mut self,
            execution_context: ExecutionContext,
            region: Option<Region>,
            endpoint: &str,
        ) -> RequestHandle;
    }

    impl TestBuilderExt for DiagnosticsContextBuilder {
        fn start_test_request(
            &mut self,
            execution_context: ExecutionContext,
            region: Option<Region>,
            endpoint: &str,
        ) -> RequestHandle {
            let cosmos_endpoint = match region {
                Some(r) => CosmosEndpoint::regional(r, url::Url::parse(endpoint).unwrap()),
                None => CosmosEndpoint::global(url::Url::parse(endpoint).unwrap()),
            };
            self.start_request(
                execution_context,
                PipelineType::DataPlane,
                TransportSecurity::Secure,
                TransportKind::Gateway,
                TransportHttpVersion::Http11,
                &cosmos_endpoint,
            )
        }
    }

    /// Normalizes dynamic fields in diagnostics JSON for deterministic comparison.
    ///
    /// Replaces `total_duration_ms` and per-request `duration_ms` values with `0`
    /// so that tests can compare the full JSON structure without being affected
    /// by wall-clock timing variations.
    fn normalize_diagnostics_json(json: &str) -> serde_json::Value {
        let mut value: serde_json::Value = serde_json::from_str(json)
            .unwrap_or_else(|e| panic!("Failed to parse diagnostics JSON: {e}\nJSON: {json}"));

        // Normalize top-level total_duration_ms
        if let Some(obj) = value.as_object_mut() {
            if obj.contains_key("total_duration_ms") {
                obj.insert(
                    "total_duration_ms".to_string(),
                    serde_json::Value::Number(0.into()),
                );
            }
        }

        // Normalize duration_ms in individual requests (detailed mode)
        if let Some(requests) = value.get_mut("requests").and_then(|v| v.as_array_mut()) {
            for req in requests {
                if let Some(obj) = req.as_object_mut() {
                    if obj.contains_key("duration_ms") {
                        obj.insert(
                            "duration_ms".to_string(),
                            serde_json::Value::Number(0.into()),
                        );
                    }
                }
            }
        }

        // Normalize duration_ms in region summaries (summary mode)
        if let Some(regions) = value.get_mut("regions").and_then(|v| v.as_array_mut()) {
            for region in regions {
                // Normalize first/last request summaries
                for key in &["first", "last"] {
                    if let Some(summary) = region.get_mut(*key).and_then(|v| v.as_object_mut()) {
                        if summary.contains_key("duration_ms") {
                            summary.insert(
                                "duration_ms".to_string(),
                                serde_json::Value::Number(0.into()),
                            );
                        }
                    }
                }
                // Normalize deduplicated groups
                if let Some(groups) = region
                    .get_mut("deduplicated_groups")
                    .and_then(|v| v.as_array_mut())
                {
                    for group in groups {
                        if let Some(obj) = group.as_object_mut() {
                            for key in &["min_duration_ms", "max_duration_ms", "p50_duration_ms"] {
                                if obj.contains_key(*key) {
                                    obj.insert(
                                        key.to_string(),
                                        serde_json::Value::Number(0.into()),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }

        value
    }

    fn assert_no_rust_debug_internals(rendered: &str) {
        for leaked in [
            "DiagnosticsContext {",
            "RequestDiagnostics {",
            "RequestEvent {",
            "Instant {",
            "CpuMemoryMonitorInner",
            "RwLock {",
            "OnceLock(",
            "cached_json",
        ] {
            assert!(
                !rendered.contains(leaked),
                "diagnostics Debug output must be JSON, not a Rust Debug dump containing `{leaked}`:\n{rendered}"
            );
        }
    }

    #[test]
    fn builder_new_context_has_activity_id() {
        let activity_id = ActivityId::new_uuid();
        let ctx = make_context_with(activity_id.clone(), |_| {});
        assert_eq!(ctx.activity_id(), &activity_id);
    }

    #[test]
    fn builder_start_and_complete_request() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );

            std::thread::sleep(std::time::Duration::from_millis(10));
            builder.complete_request(handle, StatusCode::Ok, None);
        });

        let requests = ctx.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].status().status_code(), StatusCode::Ok);
        assert!(requests[0].duration_ms >= 10);
        assert!(requests[0].completed_at.is_some());
    }

    #[test]
    fn builder_timeout_request() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.timeout_request(handle);
        });

        let requests = ctx.requests();
        assert!(requests[0].timed_out);
    }

    #[test]
    fn builder_update_request_with_charge() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.update_request(handle, |req| {
                req.request_charge = RequestCharge::new(5.5);
            });
        });

        assert_eq!(ctx.total_request_charge(), RequestCharge::new(5.5));
    }

    #[test]
    fn total_charge_sums_all_requests() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let h1 = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.update_request(h1, |req| req.request_charge = RequestCharge::new(3.0));

            let h2 = builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.update_request(h2, |req| req.request_charge = RequestCharge::new(2.5));
        });

        assert!((ctx.total_request_charge().value() - 5.5).abs() < f64::EPSILON);
    }

    #[test]
    fn regions_contacted_deduplicates() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.start_test_request(
                ExecutionContext::RegionFailover,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
        });

        let regions = ctx.regions_contacted();
        assert_eq!(regions.len(), 2);
    }

    #[test]
    fn regions_contacted_preserves_order_and_survives_compaction() {
        // Regions must surface in first-contact (failover) order — not sorted —
        // and a region contacted only by a bucket that global-bucket compaction
        // drops from the retained per-attempt list must still be reported,
        // because the set is captured from the full attempt list before
        // compaction. Covers the metrics/tracing contacted-region attribute
        // contract and the compaction bucket-drop gap.
        let cap = 16;
        let region_count = 20usize;
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("regions-storm".to_string()),
            options_with_cap(cap),
        );
        // Contact `region_count` distinct single-attempt region buckets in
        // reverse-numeric order, so first-contact order differs from sorted
        // order. Global compaction reserves the operation-wide first ("Region
        // 19") and terminal ("Region 00") buckets and drops middle ones.
        for i in (0..region_count).rev() {
            record_run(
                &mut b,
                ExecutionContext::RegionFailover,
                &format!("Region {i:02}"),
                "https://acct/",
                CosmosStatus::new(StatusCode::Gone),
                1.0,
                1,
            );
        }
        b.set_operation_status(StatusCode::Ok, None);
        let ctx = b.complete();

        let info = ctx
            .compaction()
            .expect("more distinct buckets than the cap must compact");
        assert!(
            info.omitted_runs >= 1,
            "at least one whole bucket must have been dropped from the retained list"
        );

        // First-contact order across all contacted regions, none lost.
        let expected: Vec<Region> = (0..region_count)
            .rev()
            .map(|i| Region::new(format!("Region {i:02}")))
            .collect();
        let regions = ctx.regions_contacted();
        assert_eq!(
            regions, expected,
            "all contacted regions must survive compaction, in first-contact order"
        );

        // Prove the order is first-contact, not sorted.
        let mut sorted = regions.clone();
        sorted.sort();
        assert_ne!(
            regions, sorted,
            "first-contact order must differ from a sorted list for these inputs"
        );

        // A region whose only attempt was a dropped MIDDLE bucket must still
        // appear at the operation level even though it's gone from the retained
        // list. "Region 01" is neither the first nor the last attempt, so it is
        // eligible to be ranked out.
        let retained_regions: Vec<Region> = ctx
            .requests()
            .iter()
            .filter_map(|r| r.region().cloned())
            .collect();
        let dropped = Region::new("Region 01".to_string());
        assert!(
            !retained_regions.contains(&dropped),
            "a low-ranked middle bucket should have been dropped from the retained list"
        );
        assert!(
            regions.contains(&dropped),
            "a region dropped from the retained list must still be reported"
        );

        // The operation-wide first and terminal attempts are reserved regardless
        // of bucket ranking: their regions must survive in the retained list, and
        // the terminal attempt ("Region 00") must be the LAST retained record so
        // downstream status/span-end fallbacks see the true terminal.
        let first = Region::new("Region 19".to_string());
        let terminal = Region::new("Region 00".to_string());
        assert!(
            retained_regions.contains(&first),
            "the operation-wide first attempt's bucket must be retained"
        );
        assert_eq!(
            retained_regions.last(),
            Some(&terminal),
            "the operation-wide terminal attempt must be the last retained record"
        );
    }

    #[test]
    fn effective_status_falls_back_to_terminal_attempt() {
        // A context with no operation-level status but a failed terminal attempt
        // must report that attempt's status as the effective status, so metrics
        // and tracing can classify status-less error-finalization paths
        // accurately instead of dropping the status / using the _OTHER catch-all.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let h = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::TooManyRequests, None);
            // Intentionally no set_operation_status: the operation-level status
            // stays `None`, mirroring the graft-onto-error finalization paths.
        });

        assert!(
            ctx.status().is_none(),
            "no operation-level status should be set"
        );
        assert_eq!(
            ctx.effective_status().map(|s| s.status_code()),
            Some(StatusCode::TooManyRequests),
            "effective status must fall back to the terminal attempt"
        );
        assert!(ctx.is_failure());
    }

    #[test]
    fn size_limited_summary_fallback_omits_run_rollup() {
        // Under a storm whose full summary exceeds the size budget, the truncated
        // fallback must NOT re-serialize the (large) per-run rollup — re-emitting
        // it is exactly what would keep the "truncated" summary oversized. It
        // keeps counts only.
        let cap = 64;
        let options = Arc::new(
            DiagnosticsOptions::builder()
                .with_max_request_diagnostics(cap)
                .with_max_summary_size_bytes(4096)
                .build()
                .expect("valid options"),
        );
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("storm-size".to_string()),
            options,
        );
        // Many distinct long-endpoint single-attempt buckets, so the per-run
        // rollup (bounded to `cap`) alone is large enough that the full summary
        // blows the size budget.
        for i in 0..(cap + 4) {
            let endpoint = format!(
                "https://very-long-endpoint-{i:04}.documents.azure.com:443/padding/segment/path"
            );
            record_run(
                &mut b,
                ExecutionContext::RegionFailover,
                &format!("Region {i:04}"),
                &endpoint,
                CosmosStatus::new(StatusCode::Gone),
                1.0,
                1,
            );
        }
        b.set_operation_status(StatusCode::ServiceUnavailable, None);
        let ctx = b.complete();
        assert!(ctx.compaction().is_some(), "the storm must compact");

        let summary = ctx.to_json_string(Some(DiagnosticsVerbosity::Summary));
        assert!(
            summary.contains("\"truncated\":true"),
            "summary must have fallen back to the truncated form:\n{summary}"
        );
        assert!(
            !summary.contains("\"runs\""),
            "the truncated summary must omit the per-run rollup:\n{summary}"
        );
        assert!(
            summary.contains("original_request_count"),
            "the counts-only compaction summary must still be present:\n{summary}"
        );
        assert!(
            summary.len() <= 4096,
            "the truncated summary must respect the size budget, was {} bytes",
            summary.len()
        );
    }

    #[test]
    fn aggregate_sub_operations_concatenates_request_diagnostics() {
        // Concatenates sub-op RequestDiagnostics in input order, inherits
        // operation-level fields (activity_id, status) from the LAST source,
        // and sums per-source durations. This is the contract the PATCH
        // handler depends on to surface "one operation = one
        // DiagnosticsContext" across its Read + Replace sub-ops.
        let read_activity = ActivityId::new_uuid();
        let read_ctx = Arc::new(make_context_with(read_activity.clone(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_status(StatusCode::Ok, None);
        }));

        let replace_activity = ActivityId::new_uuid();
        let replace_ctx = Arc::new(make_context_with(replace_activity.clone(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.set_operation_status(StatusCode::Created, None);
        }));

        let aggregated =
            DiagnosticsContext::aggregate_sub_operations(&[read_ctx.clone(), replace_ctx.clone()])
                .expect("aggregation must succeed for non-empty source");

        assert_eq!(
            aggregated.request_count(),
            2,
            "aggregated context must contain one RequestDiagnostics per sub-op"
        );
        assert_eq!(
            aggregated.activity_id(),
            &replace_activity,
            "operation-level activity_id must come from the last source"
        );
        assert_eq!(
            aggregated.status().map(|s| s.status_code()),
            Some(StatusCode::Created),
            "operation-level status must come from the last source"
        );
        // Both source regions are reachable through the aggregated context.
        let regions = aggregated.regions_contacted();
        assert!(regions.contains(&Region::WEST_US_2));
        assert!(regions.contains(&Region::EAST_US_2));
    }

    #[test]
    fn aggregate_sub_operations_preserves_exact_counts_when_sources_compacted() {
        // Two sub-ops, each individually compacted past a small cap (the PATCH
        // Read + Replace shape under a retry storm). The aggregate must report
        // the exact total attempt count — the sum of the sub-ops' true counts,
        // not just the retained records — and keep the combined artifact within
        // the cap, rather than dropping the compaction metadata.
        let cap = 16;

        let mut read = DiagnosticsContextBuilder::new(
            ActivityId::from_string("agg-read".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut read,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::TooManyRequests),
            2.0,
            600,
        );
        read.set_operation_status(StatusCode::Ok, None);
        let read_ctx = Arc::new(read.complete());
        assert!(
            read_ctx.compaction().is_some(),
            "source must be individually compacted"
        );

        let mut replace = DiagnosticsContextBuilder::new(
            ActivityId::from_string("agg-replace".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut replace,
            ExecutionContext::OperationRetry,
            "West US",
            "https://west/",
            CosmosStatus::new(StatusCode::Gone),
            3.0,
            400,
        );
        replace.set_operation_status(StatusCode::Created, None);
        let replace_ctx = Arc::new(replace.complete());

        let aggregated =
            DiagnosticsContext::aggregate_sub_operations(&[read_ctx.clone(), replace_ctx.clone()])
                .expect("aggregation must succeed");

        // Exact total across sub-ops, not just the retained records.
        assert_eq!(
            aggregated.request_count(),
            read_ctx.request_count() + replace_ctx.request_count()
        );
        assert_eq!(aggregated.request_count(), 1000);
        // The combined artifact respects the cap.
        assert!(
            aggregated.retained_request_count() <= cap,
            "retained {} exceeds cap {cap}",
            aggregated.retained_request_count()
        );
        // Exact total charge preserved (600*2.0 + 400*3.0).
        assert!((aggregated.total_request_charge().value() - 2400.0).abs() < f64::EPSILON);
        // Compaction metadata reports the exact original attempt count.
        let info = aggregated
            .compaction()
            .expect("aggregate of compacted sources must carry compaction metadata");
        assert_eq!(info.original_request_count, 1000);
    }

    /// The under-cap branch must roll up verbatim sources too. A fold that mixes
    /// an already-compacted accumulator with fresh uncompacted pages stays under
    /// the cap, and previously dropped the fresh pages from the rollup entirely.
    #[test]
    fn aggregate_sub_operations_rolls_up_verbatim_sources_under_cap() {
        const STORM: usize = 600;
        // 2 retained records from the storm + 4 pages x 3 attempts = 14 <= cap,
        // so the concatenation stays under the cap and takes the counts-only
        // branch rather than being re-compacted.
        const FRESH_PAGES: usize = 4;
        const PER_PAGE: usize = 3;
        let cap = 16;

        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("mixed-storm".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut b,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::TooManyRequests),
            2.0,
            STORM,
        );
        b.set_operation_status(StatusCode::Ok, None);
        let compacted_source = Arc::new(b.complete());
        assert!(compacted_source.compaction().is_some());

        let mut batch = vec![compacted_source];
        for i in 0..FRESH_PAGES {
            let mut page = DiagnosticsContextBuilder::new(
                ActivityId::from_string(format!("mixed-page-{i}")),
                options_with_cap(cap),
            );
            record_run(
                &mut page,
                ExecutionContext::OperationRetry,
                "West US",
                "https://west/",
                CosmosStatus::new(StatusCode::Ok),
                1.0,
                PER_PAGE,
            );
            page.set_operation_status(StatusCode::Ok, None);
            let page = Arc::new(page.complete());
            assert!(
                page.compaction().is_none(),
                "fresh pages must be verbatim for this test to cover the mixed path"
            );
            batch.push(page);
        }

        let aggregated =
            DiagnosticsContext::aggregate_sub_operations(&batch).expect("aggregation must succeed");
        let total = STORM + FRESH_PAGES * PER_PAGE;

        // The concatenation fits the cap, so this exercises the counts-only
        // branch: every record is retained verbatim.
        assert_eq!(
            aggregated.retained_request_count(),
            2 + FRESH_PAGES * PER_PAGE
        );
        assert_eq!(aggregated.request_count(), total);
        let info = aggregated
            .compaction()
            .expect("a compacted source must yield compaction metadata");
        assert_eq!(
            info.runs.iter().map(|r| r.count).sum::<usize>(),
            total,
            "the verbatim sources' attempts must appear in the rollup"
        );
    }

    #[test]
    fn aggregate_sub_operations_returns_none_for_empty_input() {
        // Edge case: defensive None for callers that don't pre-check —
        // exercised by the patch handler's `.unwrap_or_else(...)` safety
        // net even though the real call site always has at least one
        // source.
        let aggregated = DiagnosticsContext::aggregate_sub_operations(&[]);
        assert!(aggregated.is_none());
    }

    #[test]
    fn to_json_detailed() {
        let ctx = make_context_with(ActivityId::from_string("test-id".to_string()), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.update_request(handle, |req| req.request_charge = RequestCharge::new(1.0));
            builder.complete_request(handle, StatusCode::Ok, None);
        });

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let actual = normalize_diagnostics_json(json);
        let expected: serde_json::Value = {
            #[cfg(feature = "fault_injection")]
            {
                serde_json::json!({
                    "activity_id": "test-id",
                    "total_duration_ms": 0,
                    "total_request_charge": 1.0,
                    "request_count": 1,
                    "requests": [{
                        "execution_context": "initial",
                        "pipeline_type": "data_plane",
                        "transport_security": "secure",
                        "transport_kind": "gateway",
                        "transport_http_version": "http11",
                        "region": "westus2",
                        "endpoint": "https://test.documents.azure.com/",
                        "status": "200",
                        "request_charge": 1.0,
                        "activity_id": "test-id",
                        "session_token": null,
                        "server_duration_ms": null,
                        "duration_ms": 0,
                        "events": [],
                        "timed_out": false,
                        "request_sent": "sent",
                        "error": null,
                        "fault_injection_evaluations": []
                    }]
                })
            }
            #[cfg(not(feature = "fault_injection"))]
            {
                serde_json::json!({
                    "activity_id": "test-id",
                    "total_duration_ms": 0,
                    "total_request_charge": 1.0,
                    "request_count": 1,
                    "requests": [{
                        "execution_context": "initial",
                        "pipeline_type": "data_plane",
                        "transport_security": "secure",
                        "transport_kind": "gateway",
                        "transport_http_version": "http11",
                        "region": "westus2",
                        "endpoint": "https://test.documents.azure.com/",
                        "status": "200",
                        "request_charge": 1.0,
                        "activity_id": "test-id",
                        "session_token": null,
                        "server_duration_ms": null,
                        "duration_ms": 0,
                        "events": [],
                        "timed_out": false,
                        "request_sent": "sent",
                        "error": null
                    }]
                })
            }
        };
        assert_eq!(actual, expected, "Detailed JSON mismatch.\nActual:\n{json}");
    }

    #[test]
    fn to_json_detailed_with_known_sub_status() {
        // Verifies that when a request completes with a sub-status that has
        // a well-known name (e.g. 3200 → RUBudgetExceeded), the serialized
        // `status` field carries the full `[Kind] {code}/{sub} ({name})`
        // form produced by `CosmosStatus::Display`.
        let ctx = make_context_with(ActivityId::from_string("test-id".to_string()), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.complete_request(
                handle,
                StatusCode::TooManyRequests,
                Some(SubStatusCode::RU_BUDGET_EXCEEDED),
            );
        });

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let value = normalize_diagnostics_json(json);
        let status = value
            .get("requests")
            .and_then(|r| r.as_array())
            .and_then(|a| a.first())
            .and_then(|r| r.get("status"))
            .and_then(|s| s.as_str())
            .expect("status field must be a string");
        assert_eq!(
            status, "429/3200 (RUBudgetExceeded)",
            "named sub-status must serialize as `[Kind] {{code}}/{{sub}} ({{name}})`"
        );
    }

    #[test]
    fn to_json_detailed_with_unknown_sub_status() {
        // Verifies the `[Kind] {code}/{sub}` form (no name suffix) when the
        // sub-status code is not in the well-known table.
        let ctx = make_context_with(ActivityId::from_string("test-id".to_string()), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            builder.complete_request(
                handle,
                StatusCode::TooManyRequests,
                Some(SubStatusCode::new(65000)),
            );
        });

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let value = normalize_diagnostics_json(json);
        let status = value
            .get("requests")
            .and_then(|r| r.as_array())
            .and_then(|a| a.first())
            .and_then(|r| r.get("status"))
            .and_then(|s| s.as_str())
            .expect("status field must be a string");
        assert_eq!(
            status, "429/65000",
            "unknown sub-status must serialize as `[Kind] {{code}}/{{sub}}` with no name suffix"
        );
    }

    #[test]
    fn to_json_summary() {
        let ctx = make_context_with(ActivityId::from_string("test-id".to_string()), |builder| {
            // Add several requests to trigger deduplication
            for i in 0..5 {
                let handle = builder.start_test_request(
                    ExecutionContext::OperationRetry,
                    Some(Region::WEST_US_2),
                    "https://test.documents.azure.com",
                );
                builder.update_request(handle, |req| {
                    req.request_charge = RequestCharge::new(i as f64)
                });
                builder.complete_request(
                    handle,
                    StatusCode::TooManyRequests,
                    Some(SubStatusCode::RU_BUDGET_EXCEEDED),
                );
            }
        });

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Summary));
        let actual = normalize_diagnostics_json(json);
        let expected: serde_json::Value = serde_json::json!({
            "activity_id": "test-id",
            "total_duration_ms": 0,
            "total_request_charge": 10.0,
            "request_count": 5,
            "regions": [{
                "region": "westus2",
                "request_count": 5,
                "total_request_charge": 10.0,
                "first": {
                    "execution_context": "operation_retry",
                    "endpoint": "https://test.documents.azure.com/",
                    "status": "429/3200 (RUBudgetExceeded)",
                    "request_charge": 0.0,
                    "duration_ms": 0,
                    "timed_out": false
                },
                "last": {
                    "execution_context": "operation_retry",
                    "endpoint": "https://test.documents.azure.com/",
                    "status": "429/3200 (RUBudgetExceeded)",
                    "request_charge": 4.0,
                    "duration_ms": 0,
                    "timed_out": false
                },
                "deduplicated_groups": [{
                    "endpoint": "https://test.documents.azure.com/",
                    "status": "429/3200 (RUBudgetExceeded)",
                    "execution_context": "operation_retry",

                    "count": 3,
                    "total_request_charge": 6.0,
                    "min_duration_ms": 0,
                    "max_duration_ms": 0,
                    "p50_duration_ms": 0
                }]
            }]
        });
        assert_eq!(actual, expected, "Summary JSON mismatch.\nActual:\n{json}");
    }

    #[test]
    fn debug_renders_summary_json() {
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::from_string("debug-json-test".to_string()),
            make_options(),
        );
        builder.set_cpu_monitor(CpuMemoryMonitor::get_or_init(Duration::from_secs(5)));
        builder.set_test_system_usage(SystemUsageSnapshot::new_for_test(
            vec!["(12.3%)".to_string()],
            Some(2048),
            16,
            true,
        ));
        builder.set_machine_id(Arc::new("uuid_debug-json-machine".to_string()));
        builder.set_operation_status(
            StatusCode::ServiceUnavailable,
            Some(SubStatusCode::TRANSPORT_GENERATED_503),
        );
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::EAST_US_2),
            "https://test.eastus2.documents.azure.com",
        );
        builder.add_event(handle, RequestEvent::new(RequestEventType::TransportStart));
        builder.add_event(
            handle,
            RequestEvent::new(RequestEventType::TransportFailed)
                .with_details("transport failed with quoted value: \"timeout\""),
        );
        builder.set_transport_shard(
            handle,
            TransportShardDiagnostics::new(1, 2, 3, 4, 5, 6, true),
        );
        builder.add_failed_transport_shard(
            handle,
            FailedTransportShardDiagnostics::new(
                TransportShardDiagnostics::new(2, 3, 4, 5, 6, 7, false),
                RequestSentStatus::Unknown,
                "previous shard failed",
            ),
        );
        builder.increment_local_shard_retry_count(handle);
        builder.fail_transport_request(
            handle,
            "503/20011: error sending request for url (https://test.eastus2.documents.azure.com/dbs/db/colls/coll/docs)",
            RequestSentStatus::Unknown,
            CosmosStatus::TRANSPORT_GENERATED_503,
        );
        let ctx = builder.complete();

        let rendered = format!("{ctx:?}");
        assert_eq!(
            rendered,
            ctx.to_json_string(Some(DiagnosticsVerbosity::Default)),
            "DiagnosticsContext Debug must stay delegated to default JSON"
        );
        let actual = normalize_diagnostics_json(&rendered);
        let expected: serde_json::Value = serde_json::json!({
            "activity_id": "debug-json-test",
            "total_duration_ms": 0,
            "total_request_charge": 0.0,
            "request_count": 1,
            "system_usage": {
                "cpu": {
                    "samples": ["(12.3%)"],
                    "status": "available"
                },
                "memory_available_mb": 2048,
                "processor_count": 16,
                "cpu_overloaded": true
            },
            "machine_id": "uuid_debug-json-machine",
            "regions": [{
                "region": "eastus2",
                "request_count": 1,
                "total_request_charge": 0.0,
                "first": {
                    "execution_context": "initial",
                    "endpoint": "https://test.eastus2.documents.azure.com/",
                    "status": "503/20003 (TransportGenerated503)",
                    "request_charge": 0.0,
                    "duration_ms": 0,
                    "timed_out": false
                },
                "last": null,
                "deduplicated_groups": []
            }]
        });

        assert_eq!(
            actual, expected,
            "Debug JSON mismatch.\nActual:\n{rendered}"
        );
        assert_no_rust_debug_internals(&rendered);

        let alternate_rendered = format!("{ctx:#?}");
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&alternate_rendered).unwrap(),
            serde_json::from_str::<serde_json::Value>(
                ctx.to_json_string(Some(DiagnosticsVerbosity::Summary))
            )
            .unwrap(),
            "alternate Debug must also render valid summary diagnostics JSON"
        );
        assert_no_rust_debug_internals(&alternate_rendered);
    }

    #[test]
    fn debug_honors_configured_detailed_default_verbosity() {
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::from_string("debug-detailed-default".to_string()),
            make_options_with_default_verbosity(DiagnosticsVerbosity::Detailed),
        );
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );
        builder.complete_request(handle, StatusCode::Ok, None);
        let ctx = builder.complete();

        let rendered = format!("{ctx:?}");
        assert_eq!(
            rendered,
            ctx.to_json_string(Some(DiagnosticsVerbosity::Default)),
            "DiagnosticsContext Debug must use the configured default verbosity"
        );
        assert_eq!(
            rendered,
            ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed)),
            "configured default verbosity should allow Detailed Debug JSON"
        );
        let value: serde_json::Value = serde_json::from_str(&rendered).unwrap();
        assert!(
            value.get("requests").and_then(|v| v.as_array()).is_some(),
            "detailed diagnostics JSON must include the requests array: {rendered}"
        );
        assert_no_rust_debug_internals(&rendered);
    }

    #[test]
    fn json_caching_detailed() {
        let ctx = make_context_with(
            ActivityId::from_string("cache-test".to_string()),
            |builder| {
                let handle = builder.start_test_request(
                    ExecutionContext::Initial,
                    Some(Region::WEST_US_2),
                    "https://test.documents.azure.com",
                );
                builder.complete_request(handle, StatusCode::Ok, None);
            },
        );

        // First call computes
        let json1 = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        // Second call should return cached
        let json2 = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));

        // Both should be identical (pointer comparison proves caching)
        assert_eq!(json1, json2);
        assert!(std::ptr::eq(json1, json2)); // Same string reference
    }

    #[test]
    fn requests_returns_arc() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
        });

        let requests1 = ctx.requests();
        let requests2 = ctx.requests();

        // Both should point to the same allocation (Arc::ptr_eq)
        assert!(Arc::ptr_eq(&requests1, &requests2));
    }

    #[test]
    fn duration_is_captured() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
        });

        assert!(ctx.duration().as_millis() >= 10);
    }

    #[test]
    fn status_codes_stored() {
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        builder.set_operation_status(
            StatusCode::NotFound,
            Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE),
        );
        let ctx = builder.complete();

        let status = ctx.status().unwrap();
        assert_eq!(status.status_code(), StatusCode::NotFound);
        assert!(status.is_read_session_not_available());
    }

    #[test]
    fn transport_failure_request_uses_transport_generated_503() {
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );

        builder.fail_transport_request(
            handle,
            "connection refused",
            RequestSentStatus::Unknown,
            CosmosStatus::TRANSPORT_GENERATED_503,
        );

        let ctx = builder.complete();
        let requests = ctx.requests();
        let status = requests[0].status();
        assert_eq!(status, &CosmosStatus::TRANSPORT_GENERATED_503);
        assert_eq!(requests[0].error(), Some("connection refused"));
    }

    #[test]
    fn transport_failure_records_sent_activity_id_not_null() {
        // (A) On a transport failure there is no response, so `record_response`
        // never runs. The per-attempt activity_id must still be populated with
        // the operation-level id the SDK placed on the wire, rather than `null`.
        let activity_id = ActivityId::from_string("op-activity-id".to_string());
        let mut builder = DiagnosticsContextBuilder::new(activity_id.clone(), make_options());
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );
        builder.fail_transport_request(
            handle,
            "connection refused",
            RequestSentStatus::Sent,
            CosmosStatus::TRANSPORT_GENERATED_503,
        );

        let ctx = builder.complete();
        let requests = ctx.requests();
        assert_eq!(
            requests[0].activity_id(),
            Some(&activity_id),
            "a transport-failed attempt must record the activity id sent on the wire"
        );
    }

    #[test]
    fn successful_response_overwrites_seeded_activity_id_with_header_echo() {
        // (A) The success path is unchanged: `record_response` overwrites the
        // seeded operation-level id with the activity id echoed in the response
        // headers (the same value in the common case).
        let op_activity_id = ActivityId::from_string("op-activity-id".to_string());
        let echoed = ActivityId::from_string("echoed-activity-id".to_string());
        let mut builder = DiagnosticsContextBuilder::new(op_activity_id, make_options());
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );
        let headers = CosmosResponseHeaders {
            activity_id: Some(echoed.clone()),
            ..Default::default()
        };
        builder.record_response(handle, StatusCode::Ok, &headers);

        let ctx = builder.complete();
        let requests = ctx.requests();
        assert_eq!(
            requests[0].activity_id(),
            Some(&echoed),
            "a successful response must record the response-header activity id"
        );
    }

    #[test]
    fn multi_attempt_failed_then_succeeded_keep_independent_activity_ids() {
        // A failed attempt retains its seeded operation-level id while a later
        // successful attempt records the response-header echo. The failed
        // attempt's id must not be perturbed by the later success.
        let op_activity_id = ActivityId::from_string("op-activity-id".to_string());
        let echoed = ActivityId::from_string("echoed-activity-id".to_string());
        let mut builder = DiagnosticsContextBuilder::new(op_activity_id.clone(), make_options());

        let failed = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );
        builder.fail_transport_request(
            failed,
            "connection refused",
            RequestSentStatus::Sent,
            CosmosStatus::TRANSPORT_GENERATED_503,
        );

        let succeeded = builder.start_test_request(
            ExecutionContext::OperationRetry,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );
        let headers = CosmosResponseHeaders {
            activity_id: Some(echoed.clone()),
            ..Default::default()
        };
        builder.record_response(succeeded, StatusCode::Ok, &headers);

        let ctx = builder.complete();
        let requests = ctx.requests();
        assert_eq!(
            requests[0].activity_id(),
            Some(&op_activity_id),
            "the failed attempt must retain the seeded operation-level activity id"
        );
        assert_eq!(
            requests[1].activity_id(),
            Some(&echoed),
            "the succeeded attempt must record the response-header echo"
        );
    }

    #[test]
    fn percentile_calculation() {
        assert_eq!(percentile_sorted(&[], 50), 0);
        assert_eq!(percentile_sorted(&[100], 50), 100);
        assert_eq!(percentile_sorted(&[10, 20, 30, 40, 50], 50), 30);
        assert_eq!(percentile_sorted(&[10, 20, 30, 40, 50], 0), 10);
        assert_eq!(percentile_sorted(&[10, 20, 30, 40, 50], 100), 50);
    }

    #[test]
    fn update_before_complete_succeeds() {
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );

        // Update before complete - should work
        builder.update_request(handle, |req| {
            req.request_charge = RequestCharge::new(5.5);
        });

        // Now complete
        builder.complete_request(handle, StatusCode::Ok, None);

        let ctx = builder.complete();
        let requests = ctx.requests();
        assert_eq!(requests[0].request_charge, RequestCharge::new(5.5));
    }

    #[test]
    fn update_after_complete_is_ignored_in_release() {
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        let handle = builder.start_test_request(
            ExecutionContext::Initial,
            Some(Region::WEST_US_2),
            "https://test.documents.azure.com",
        );

        // Update with initial value
        builder.update_request(handle, |req| {
            req.request_charge = RequestCharge::new(5.5);
        });

        // Complete the request
        builder.complete_request(handle, StatusCode::Ok, None);

        // In release builds, this update should be silently ignored
        // In debug builds, this would panic (tested separately)
        #[cfg(not(debug_assertions))]
        {
            builder.update_request(handle, |req| {
                req.request_charge = RequestCharge::new(10.0); // Attempt to change after completion
            });

            let ctx = builder.complete();
            let requests = ctx.requests();
            // Value should remain 5.5, not 10.0
            assert_eq!(requests[0].request_charge, RequestCharge::new(5.5));
        }
    }

    // =========================================================================
    // ExecutionContext tests (merged from execution_context.rs)
    // =========================================================================

    #[test]
    fn execution_context_display() {
        assert_eq!(ExecutionContext::Initial.to_string(), "initial");
        assert_eq!(
            ExecutionContext::OperationRetry.to_string(),
            "operation_retry"
        );
        assert_eq!(
            ExecutionContext::TransportRetry.to_string(),
            "transport_retry"
        );
        assert_eq!(ExecutionContext::Hedging.to_string(), "hedging");
        assert_eq!(
            ExecutionContext::RegionFailover.to_string(),
            "region_failover"
        );
        assert_eq!(
            ExecutionContext::CircuitBreakerProbe.to_string(),
            "circuit_breaker_probe"
        );
    }

    #[test]
    fn requested_regions_preserves_dispatch_order_and_reason() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.start_test_request(
                ExecutionContext::RegionFailover,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
        });

        let requested = ctx.requested_regions();
        assert_eq!(requested.len(), 3);
        // Dispatch order preserved, duplicates kept.
        assert_eq!(requested[0].region, Region::WEST_US_2);
        assert_eq!(requested[0].reason, ExecutionContext::Initial);
        assert_eq!(requested[1].region, Region::WEST_US_2);
        assert_eq!(requested[1].reason, ExecutionContext::OperationRetry);
        assert_eq!(requested[2].region, Region::EAST_US_2);
        assert_eq!(requested[2].reason, ExecutionContext::RegionFailover);
    }

    #[test]
    fn requested_regions_empty_without_region() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                None,
                "https://test.documents.azure.com",
            );
        });

        assert!(ctx.requested_regions().is_empty());
    }

    #[test]
    fn responded_regions_excludes_timeouts_and_transport_failures() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            // A real service reply.
            let h1 = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.complete_request(h1, StatusCode::Ok, None);

            // A client-side timeout (completed_at set, but no service reply).
            let h2 = builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.timeout_request(h2);

            // A non-2xx response still counts as a reply from the region.
            let h3 = builder.start_test_request(
                ExecutionContext::RegionFailover,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(h3, StatusCode::NotFound, None);
        });

        let responded = ctx.responded_regions();
        assert_eq!(responded, vec![&Region::WEST_US_2, &Region::EAST_US_2]);
    }

    #[test]
    fn hedging_started_false_without_hedge_dispatch() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let h = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::Ok, None);
        });

        assert!(!ctx.hedging_started());
    }

    #[test]
    fn hedging_started_true_when_hedge_dispatched() {
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
        });

        assert!(ctx.hedging_started());
    }

    fn hedge_config() -> crate::driver::pipeline::hedging_diagnostics::HedgingStrategyConfig {
        crate::driver::pipeline::hedging_diagnostics::HedgingStrategyConfig::new(
            crate::options::HedgeThreshold::new(std::time::Duration::from_millis(500))
                .expect("500ms is a valid hedge threshold"),
        )
    }

    /// Mirrors the orchestrator's STAGE 1: spawn the primary leg's builder and
    /// capture the dispatch record the parent keeps for it.
    ///
    /// Returned separately from [`spawn_alternate_leg`] so a test can dispatch
    /// on the primary *between* the two, exactly as the pipeline does — the
    /// alternate is only built once the hedge threshold has elapsed.
    fn spawn_primary_leg(
        parent: &mut DiagnosticsContextBuilder,
        region: Option<Region>,
        reason: ExecutionContext,
    ) -> (DiagnosticsContextBuilder, HedgeLegDispatch) {
        let leg = parent.clone_for_hedge_attempt();
        let dispatch = leg.leg_dispatch(region, reason);
        (leg, dispatch)
    }

    /// Mirrors the orchestrator's STAGE 3: spawn the alternate leg's builder
    /// once the threshold has elapsed and record the fan-out on the parent.
    fn spawn_alternate_leg(
        parent: &mut DiagnosticsContextBuilder,
        primary: HedgeLegDispatch,
        region: Option<Region>,
    ) -> DiagnosticsContextBuilder {
        let leg = parent.clone_for_hedge_attempt();
        let dispatch = leg.leg_dispatch(region, ExecutionContext::Hedging);
        parent.record_hedge_fanout(primary, dispatch);
        leg
    }

    #[test]
    fn requested_regions_keeps_dropped_hedge_leg_on_primary_win() {
        // PrimaryWonAfterHedge: the alternate leg is cancelled before it
        // dispatches anything, so it leaves no attempt behind. Both legs must
        // still appear, in dispatch order, from the fan-out the orchestrator
        // recorded on the parent.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let h = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(h, StatusCode::Ok, None);
            // Threshold elapsed, so the alternate launched — and was then
            // dropped, unpolled, when the primary won the race.
            drop(spawn_alternate_leg(
                builder,
                primary_dispatch,
                Some(Region::WEST_US_2),
            ));
            builder.merge_hedge_attempt(primary);
            builder.set_hedge_diagnostics(HedgeDiagnostics::primary_won_after_hedge(
                hedge_config(),
                Region::EAST_US_2,
                Region::WEST_US_2,
            ));
        });

        assert!(ctx.hedging_started());
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
            ]
        );
        // The dropped alternate leg never produced a service reply, so only the
        // winning primary appears in responded_regions.
        assert_eq!(ctx.responded_regions(), vec![&Region::EAST_US_2]);
    }

    #[test]
    fn requested_regions_keeps_dropped_primary_leg_on_alternate_win() {
        // AlternateWon while the primary was still connecting: the primary leg
        // is dropped without having dispatched, so the Initial primary must
        // still be listed first (dispatch order) from the fan-out record.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let h = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(h, StatusCode::Ok, None);
            drop(primary);
            builder.merge_hedge_attempt(alternate);
            builder.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                hedge_config(),
                Region::EAST_US_2,
                Region::WEST_US_2,
            ));
        });

        assert!(ctx.hedging_started());
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
            ]
        );
        // Only the winning alternate produced a service reply.
        assert_eq!(ctx.responded_regions(), vec![&Region::WEST_US_2]);
    }

    #[test]
    fn requested_regions_tags_primary_leg_with_upgrade_reason() {
        // A hedge upgraded after a failover retry (STAGE 7) dispatches its
        // primary leg as a failover, not as a first attempt — the recorded
        // fan-out must carry that reason rather than defaulting to `Initial`.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let h = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::ServiceUnavailable, None);
            let (mut primary, primary_dispatch) = spawn_primary_leg(
                builder,
                Some(Region::WEST_US_2),
                ExecutionContext::RegionFailover,
            );
            let h = primary.start_test_request(
                ExecutionContext::RegionFailover,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            primary.complete_request(h, StatusCode::Ok, None);
            drop(spawn_alternate_leg(
                builder,
                primary_dispatch,
                Some(Region::CENTRAL_US),
            ));
            builder.merge_hedge_attempt(primary);
        });

        assert!(ctx.hedging_started());
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::RegionFailover,
                },
                RequestedRegion {
                    region: Region::CENTRAL_US,
                    reason: ExecutionContext::Hedging,
                },
            ]
        );
    }

    #[test]
    fn requested_regions_bounded_but_exact_under_retry_storm() {
        // The dispatch history is materialized from the FULL attempt list, so a
        // retry storm that compacts `requests` down to the cap must not lose the
        // hedge fan-out recorded mid-storm. But the history itself is also
        // bounded (DIAGNOSTICS-CONTRACT.md §8) — the elision keeps head and tail
        // and the exact count stays available.
        let cap = 16;
        let options = Arc::new(
            DiagnosticsOptions::builder()
                .with_max_request_diagnostics(cap)
                .build()
                .expect("valid options"),
        );
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), options);
        for _ in 0..40 {
            let h = builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::TooManyRequests, None);
        }
        let (primary, primary_dispatch) = spawn_primary_leg(
            &mut builder,
            Some(Region::EAST_US_2),
            ExecutionContext::OperationRetry,
        );
        let mut alternate =
            spawn_alternate_leg(&mut builder, primary_dispatch, Some(Region::WEST_US_2));
        let h = alternate.start_test_request(
            ExecutionContext::Hedging,
            Some(Region::WEST_US_2),
            "https://test.westus2.documents.azure.com",
        );
        alternate.complete_request(h, StatusCode::Ok, None);
        drop(primary);
        builder.merge_hedge_attempt(alternate);
        let ctx = builder.complete();

        // The retained attempt list really was bounded...
        assert!(ctx.requests().len() <= cap);
        assert!(ctx.compaction().is_some());
        // ...and so is the dispatch history, independently of attempt count
        // (DIAGNOSTICS-CONTRACT.md §8) — but the elision is explicit, not
        // silent: the exact count is still reported. 40 retries + the hedge's
        // reconstructed primary leg + the alternate's own attempt.
        assert_eq!(ctx.total_requested_regions(), 42);
        assert_eq!(ctx.requested_regions().len(), cap);
        assert_eq!(ctx.total_responded_regions(), 41);
        assert_eq!(ctx.responded_regions().len(), cap);
        assert!(ctx.hedging_started());

        // Head and tail survive: the storm's opening dispatch and the hedge
        // fan-out that ended it are both still visible.
        let requested = ctx.requested_regions();
        assert_eq!(requested[0].region, Region::EAST_US_2);
        assert_eq!(requested[0].reason, ExecutionContext::OperationRetry);
        assert_eq!(
            requested.last().expect("non-empty").reason,
            ExecutionContext::Hedging
        );
        assert_eq!(
            requested.last().expect("non-empty").region,
            Region::WEST_US_2
        );
    }

    #[test]
    fn region_history_is_bounded_without_a_hedge() {
        // The bound is a property of the history itself, not of hedging: a plain
        // retry storm must not produce an unbounded artifact either.
        let cap = 16;
        let options = Arc::new(
            DiagnosticsOptions::builder()
                .with_max_request_diagnostics(cap)
                .build()
                .expect("valid options"),
        );
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), options);
        for _ in 0..500 {
            let h = builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::TooManyRequests, None);
        }
        let ctx = builder.complete();

        assert_eq!(ctx.total_requested_regions(), 500);
        assert_eq!(ctx.requested_regions().len(), cap);
        assert_eq!(ctx.total_responded_regions(), 500);
        assert_eq!(ctx.responded_regions().len(), cap);
        assert!(!ctx.hedging_started());
    }

    #[test]
    fn region_history_is_verbatim_under_the_cap() {
        // The common path must be untouched: at or below the cap the history is
        // exact and the reported total agrees with it.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            for region in [Region::EAST_US_2, Region::WEST_US_2] {
                let h = builder.start_test_request(
                    ExecutionContext::RegionFailover,
                    Some(region),
                    "https://test.documents.azure.com",
                );
                builder.complete_request(h, StatusCode::Ok, None);
            }
        });

        assert_eq!(ctx.requested_regions().len(), 2);
        assert_eq!(ctx.total_requested_regions(), 2);
        assert_eq!(ctx.responded_regions().len(), 2);
        assert_eq!(ctx.total_responded_regions(), 2);
    }

    #[test]
    fn aggregated_region_history_is_rebounded() {
        // Aggregation concatenates each sub-op's already-bounded history, which
        // is an independent unbounded path: a PATCH conflict loop adds a sub-op
        // per retry. The aggregate must be re-bounded and must report the summed
        // exact totals.
        let cap = 16;
        let options = Arc::new(
            DiagnosticsOptions::builder()
                .with_max_request_diagnostics(cap)
                .build()
                .expect("valid options"),
        );
        let sub_ops: Vec<Arc<DiagnosticsContext>> = (0..20)
            .map(|_| {
                let mut builder =
                    DiagnosticsContextBuilder::new(ActivityId::new_uuid(), Arc::clone(&options));
                for _ in 0..5 {
                    let h = builder.start_test_request(
                        ExecutionContext::OperationRetry,
                        Some(Region::EAST_US_2),
                        "https://test.eastus2.documents.azure.com",
                    );
                    builder.complete_request(h, StatusCode::Conflict, None);
                }
                Arc::new(builder.complete())
            })
            .collect();

        // Each sub-op is individually under the cap, so none is truncated.
        assert_eq!(sub_ops[0].requested_regions().len(), 5);
        assert_eq!(sub_ops[0].total_requested_regions(), 5);

        let aggregated =
            DiagnosticsContext::aggregate_sub_operations(&sub_ops).expect("non-empty sources");

        // 20 sub-ops x 5 dispatches = 100, bounded back down to the cap.
        assert_eq!(aggregated.total_requested_regions(), 100);
        assert_eq!(aggregated.requested_regions().len(), cap);
        assert_eq!(aggregated.total_responded_regions(), 100);
        assert_eq!(aggregated.responded_regions().len(), cap);
    }

    #[test]
    fn bound_region_history_keeps_head_and_tail() {
        // The elision targets the repetitive middle, so both ends survive.
        let bounded = bound_region_history((0..100).collect::<Vec<u32>>(), 10);
        assert_eq!(bounded, vec![0, 1, 2, 3, 4, 95, 96, 97, 98, 99]);

        // An odd cap favors the head by one.
        let bounded = bound_region_history((0..100).collect::<Vec<u32>>(), 5);
        assert_eq!(bounded, vec![0, 1, 2, 98, 99]);

        // At or under the cap the input is returned verbatim.
        let bounded = bound_region_history(vec![1, 2, 3], 10);
        assert_eq!(bounded, vec![1, 2, 3]);
        let bounded = bound_region_history(vec![1, 2, 3], 3);
        assert_eq!(bounded, vec![1, 2, 3]);
    }

    #[test]
    fn requested_regions_preserves_repeat_dispatches_around_a_fanout() {
        // Duplicates are meaningful: each leg's own attempts are listed as they
        // were dispatched, and a genuine repeat dispatch to the same region
        // under the same reason is never collapsed into a neighbor.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            for _ in 0..2 {
                let h = builder.start_test_request(
                    ExecutionContext::OperationRetry,
                    Some(Region::EAST_US_2),
                    "https://test.eastus2.documents.azure.com",
                );
                builder.complete_request(h, StatusCode::TooManyRequests, None);
            }
            // Both legs dispatch and are harvested back into the parent, so
            // both describe themselves and neither is reconstructed...
            let (mut primary, primary_dispatch) = spawn_primary_leg(
                builder,
                Some(Region::EAST_US_2),
                ExecutionContext::OperationRetry,
            );
            let h = primary.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(h, StatusCode::Ok, None);
            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let h = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(h, StatusCode::Ok, None);
            builder.merge_hedge_attempt(primary);
            builder.merge_hedge_attempt(alternate);
            // ...and a later retry to the same region is still its own entry.
            let late = builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(late, StatusCode::Ok, None);
        });

        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
            ]
        );
    }

    #[test]
    fn requested_regions_skips_fanout_without_named_regions() {
        // Global-endpoint accounts route to endpoints with no named region; a
        // fan-out there contributes no requested-region entries, but still
        // counts as a fan-out.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (primary, primary_dispatch) =
                spawn_primary_leg(builder, None, ExecutionContext::Initial);
            drop(spawn_alternate_leg(builder, primary_dispatch, None));
            drop(primary);
        });

        assert!(ctx.hedging_started());
        assert!(ctx.requested_regions().is_empty());
    }
    #[test]
    fn requested_regions_no_recovery_when_no_fanout() {
        // PrimaryWonPreThreshold: a strategy was active but no alternate fanned
        // out (alternate_region = None), so nothing is recovered and
        // hedging_started() is false.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let h = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::Ok, None);
            builder.set_hedge_diagnostics(HedgeDiagnostics::primary_only(
                hedge_config(),
                Region::EAST_US_2,
            ));
        });

        assert!(!ctx.hedging_started());
        assert_eq!(
            ctx.requested_regions(),
            vec![RequestedRegion {
                region: Region::EAST_US_2,
                reason: ExecutionContext::Initial,
            }]
        );
    }

    #[test]
    fn hedge_loser_keeps_the_service_reply_it_already_observed() {
        // The losing leg is not always empty. A leg that gets a 429, enters the
        // transport pipeline's throttle-retry backoff, and only then loses the
        // race has *already observed a real service reply*. Dropping its
        // builder with the race would erase that reply, under-reporting charge
        // and omitting a genuine entry from responded_regions.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            // Primary is throttled and sleeps before its next attempt.
            let throttled = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.update_request(throttled, |req| {
                req.request_charge = RequestCharge::new(1.5)
            });
            primary.complete_request(throttled, StatusCode::TooManyRequests, None);

            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let won = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.update_request(won, |req| req.request_charge = RequestCharge::new(2.5));
            alternate.complete_request(won, StatusCode::Ok, None);

            // The alternate wins; `select` drops the primary mid-backoff.
            drop(primary);
            builder.merge_hedge_attempt(alternate);
            builder.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                hedge_config(),
                Region::EAST_US_2,
                Region::WEST_US_2,
            ));
        });

        // Both attempts survive, in true dispatch order.
        assert_eq!(ctx.request_count(), 2);
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
            ]
        );
        // The dropped leg's 429 is a real reply from East US 2, so it belongs
        // in the arrival-ordered responded history — and both totals are exact.
        assert_eq!(
            ctx.responded_regions(),
            vec![&Region::EAST_US_2, &Region::WEST_US_2]
        );
        assert_eq!(ctx.total_requested_regions(), 2);
        assert_eq!(ctx.total_responded_regions(), 2);
        assert_eq!(
            ctx.regions_contacted(),
            vec![Region::EAST_US_2, Region::WEST_US_2]
        );
        // The throttled attempt's RU is billed even though its leg lost.
        assert_eq!(ctx.total_request_charge(), RequestCharge::new(4.0));
    }

    #[test]
    fn requested_regions_orders_primary_retry_before_the_alternate() {
        // A hedge leg can retry *before* the threshold elapses, so the alternate
        // is dispatched after the primary's second attempt. The history must
        // report that true order, not group both legs' fan-out ahead of the
        // primary's own attempts.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let first = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(first, StatusCode::TooManyRequests, None);
            let retry = primary.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(retry, StatusCode::Ok, None);

            // Threshold elapses only now, so the alternate is genuinely last.
            drop(spawn_alternate_leg(
                builder,
                primary_dispatch,
                Some(Region::WEST_US_2),
            ));
            builder.merge_hedge_attempt(primary);
        });

        assert!(ctx.hedging_started());
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
            ]
        );
    }

    #[test]
    fn interleaved_leg_retries_report_true_global_dispatch_order() {
        // Both legs retry concurrently and the loser is dropped. The finalized
        // list must interleave the two legs by real dispatch time rather than
        // concatenating one leg after the other.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let p1 = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(p1, StatusCode::TooManyRequests, None);

            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let a1 = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(a1, StatusCode::TooManyRequests, None);

            let p2 = primary.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(p2, StatusCode::TooManyRequests, None);

            let a2 = alternate.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(a2, StatusCode::Ok, None);

            drop(primary);
            builder.merge_hedge_attempt(alternate);
        });

        assert_eq!(ctx.request_count(), 4);
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::OperationRetry,
                },
            ]
        );
        assert_eq!(ctx.total_responded_regions(), 4);
    }

    #[test]
    fn both_transient_legs_keep_every_observed_reply() {
        // BothTransient: neither leg wins, so neither is merged and the parent
        // continues into the failover loop. Everything both legs observed must
        // still reach the finalized context.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let p = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(p, StatusCode::ServiceUnavailable, None);

            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let a = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(a, StatusCode::ServiceUnavailable, None);

            // Both transient: the race returns to the failover loop and neither
            // leg builder is merged.
            drop(primary);
            drop(alternate);

            let failover = builder.start_test_request(
                ExecutionContext::RegionFailover,
                Some(Region::CENTRAL_US),
                "https://test.centralus.documents.azure.com",
            );
            builder.complete_request(failover, StatusCode::Ok, None);
        });

        assert_eq!(ctx.request_count(), 3);
        assert_eq!(
            ctx.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
                RequestedRegion {
                    region: Region::CENTRAL_US,
                    reason: ExecutionContext::RegionFailover,
                },
            ]
        );
        assert_eq!(
            ctx.responded_regions(),
            vec![&Region::EAST_US_2, &Region::WEST_US_2, &Region::CENTRAL_US]
        );
        assert_eq!(ctx.total_responded_regions(), 3);
    }

    #[test]
    fn hedge_leg_dropped_before_completing_is_not_recovered() {
        // Only *observed* completions are recovered. An attempt still in flight
        // when its leg is cancelled saw no reply, so it is intentionally absent
        // from both histories — reporting it would invent a response.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            // Dispatched, never completed.
            let _in_flight = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );

            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let won = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(won, StatusCode::Ok, None);
            drop(primary);
            builder.merge_hedge_attempt(alternate);
        });

        assert_eq!(ctx.request_count(), 1);
        // The primary leg dispatched, so it is not reconstructed from the
        // fan-out record either — that fallback is only for a leg that never
        // reached the wire at all.
        assert_eq!(
            ctx.requested_regions(),
            vec![RequestedRegion {
                region: Region::WEST_US_2,
                reason: ExecutionContext::Hedging,
            }]
        );
        assert_eq!(ctx.responded_regions(), vec![&Region::WEST_US_2]);
    }

    #[test]
    fn aggregate_sub_operations_propagates_hedge_diagnostics() {
        // An aggregated operation (e.g. PATCH) whose sub-op hedged must still
        // report hedging so the metric/log/span surfaces stay consistent, even
        // though the aggregate is stitched from multiple sub-op contexts.
        let read = make_context_with(ActivityId::new_uuid(), |builder| {
            let h = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::Ok, None);
        });
        let patch = make_context_with(ActivityId::new_uuid(), |builder| {
            let h = builder.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.complete_request(h, StatusCode::Ok, None);
            builder.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                hedge_config(),
                Region::EAST_US_2,
                Region::WEST_US_2,
            ));
        });

        let aggregate =
            DiagnosticsContext::aggregate_sub_operations(&[Arc::new(read), Arc::new(patch)])
                .expect("non-empty sources");

        assert!(aggregate.hedging_started());
        let hedge = aggregate
            .hedge_diagnostics()
            .expect("aggregate must inherit a representative hedge diagnostics");
        assert_eq!(
            hedge.terminal_state(),
            crate::driver::pipeline::hedging_diagnostics::HedgeTerminalState::AlternateWon,
        );
    }

    #[test]
    fn aggregate_sub_operations_keeps_every_sub_op_fanout() {
        // Only one representative `hedge_diagnostics` survives aggregation, so
        // the Hedging Detection API must not depend on it: each sub-op's own
        // materialized dispatch history is concatenated in sub-op order.
        let read = make_context_with(ActivityId::new_uuid(), |builder| {
            let (primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let mut alternate =
                spawn_alternate_leg(builder, primary_dispatch, Some(Region::WEST_US_2));
            let h = alternate.start_test_request(
                ExecutionContext::Hedging,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            alternate.complete_request(h, StatusCode::Ok, None);
            drop(primary);
            builder.merge_hedge_attempt(alternate);
            builder.set_hedge_diagnostics(HedgeDiagnostics::hedge_won(
                hedge_config(),
                Region::EAST_US_2,
                Region::WEST_US_2,
            ));
        });
        let replace = make_context_with(ActivityId::new_uuid(), |builder| {
            let (mut primary, primary_dispatch) =
                spawn_primary_leg(builder, Some(Region::EAST_US_2), ExecutionContext::Initial);
            let h = primary.start_test_request(
                ExecutionContext::Initial,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com",
            );
            primary.complete_request(h, StatusCode::Ok, None);
            drop(spawn_alternate_leg(
                builder,
                primary_dispatch,
                Some(Region::CENTRAL_US),
            ));
            builder.merge_hedge_attempt(primary);
            builder.set_hedge_diagnostics(HedgeDiagnostics::primary_won_after_hedge(
                hedge_config(),
                Region::EAST_US_2,
                Region::CENTRAL_US,
            ));
        });

        let aggregate =
            DiagnosticsContext::aggregate_sub_operations(&[Arc::new(read), Arc::new(replace)])
                .expect("non-empty sources");

        assert!(aggregate.hedging_started());
        // Both fan-outs survive, including the non-representative one.
        assert_eq!(
            aggregate.requested_regions(),
            vec![
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::WEST_US_2,
                    reason: ExecutionContext::Hedging,
                },
                RequestedRegion {
                    region: Region::EAST_US_2,
                    reason: ExecutionContext::Initial,
                },
                RequestedRegion {
                    region: Region::CENTRAL_US,
                    reason: ExecutionContext::Hedging,
                },
            ]
        );
        assert_eq!(
            aggregate.responded_regions(),
            vec![&Region::WEST_US_2, &Region::EAST_US_2]
        );
    }
    // =========================================================================
    // Pipeline/Transport/RequestSentStatus tests (merged from request_diagnostics.rs)
    // =========================================================================

    #[test]
    fn pipeline_type_classification() {
        assert!(PipelineType::Metadata.is_metadata());
        assert!(!PipelineType::Metadata.is_data_plane());
        assert!(PipelineType::DataPlane.is_data_plane());
        assert!(!PipelineType::DataPlane.is_metadata());
    }

    #[test]
    fn transport_security_classification() {
        assert!(TransportSecurity::Secure.is_secure());
        assert!(!TransportSecurity::Secure.is_emulator());
        assert!(TransportSecurity::EmulatorWithInsecureCertificates.is_emulator());
        assert!(!TransportSecurity::EmulatorWithInsecureCertificates.is_secure());
    }

    #[test]
    fn transport_kind_classification() {
        assert!(TransportKind::Gateway.is_gateway());
        assert!(!TransportKind::Gateway.is_gateway_v2());
        assert!(TransportKind::GatewayV2.is_gateway_v2());
        assert!(!TransportKind::GatewayV2.is_gateway());
    }

    #[test]
    fn transport_http_version_classification() {
        assert!(TransportHttpVersion::Http11.is_http11());
        assert!(!TransportHttpVersion::Http11.is_http2());
        assert!(TransportHttpVersion::Http2.is_http2());
        assert!(!TransportHttpVersion::Http2.is_http11());
    }

    #[test]
    fn transport_security_default() {
        assert_eq!(TransportSecurity::default(), TransportSecurity::Secure);
    }

    #[test]
    fn transport_kind_default() {
        assert_eq!(TransportKind::default(), TransportKind::Gateway);
    }

    #[test]
    fn pipeline_type_serialization() {
        assert_eq!(
            serde_json::to_string(&PipelineType::Metadata).unwrap(),
            "\"metadata\""
        );
        assert_eq!(
            serde_json::to_string(&PipelineType::DataPlane).unwrap(),
            "\"data_plane\""
        );
    }

    #[test]
    fn transport_security_serialization() {
        assert_eq!(
            serde_json::to_string(&TransportSecurity::Secure).unwrap(),
            "\"secure\""
        );
        assert_eq!(
            serde_json::to_string(&TransportSecurity::EmulatorWithInsecureCertificates).unwrap(),
            "\"emulator_with_insecure_certificates\""
        );
    }

    #[test]
    fn transport_kind_serialization() {
        assert_eq!(
            serde_json::to_string(&TransportKind::Gateway).unwrap(),
            "\"gateway\""
        );
        assert_eq!(
            serde_json::to_string(&TransportKind::GatewayV2).unwrap(),
            "\"gateway_v2\""
        );
    }

    #[test]
    fn transport_http_version_serialization() {
        assert_eq!(
            serde_json::to_string(&TransportHttpVersion::Http11).unwrap(),
            "\"http11\""
        );
        assert_eq!(
            serde_json::to_string(&TransportHttpVersion::Http2).unwrap(),
            "\"http2\""
        );
    }

    // =========================================================================
    // RequestEvent tests (merged from request_event.rs)
    // =========================================================================

    #[test]
    fn event_type_indicates_sent() {
        // Before/during sending - not confirmed sent
        assert!(!RequestEventType::TransportStart.indicates_request_sent());

        // TransportFailed is ambiguous - requires error analysis
        assert!(!RequestEventType::TransportFailed.indicates_request_sent());

        // After headers received or transport complete - definitely sent
        assert!(RequestEventType::ResponseHeadersReceived.indicates_request_sent());
        assert!(RequestEventType::TransportComplete.indicates_request_sent());
    }

    #[test]
    fn event_creation() {
        let event = RequestEvent::new(RequestEventType::TransportStart);
        assert_eq!(event.event_type, RequestEventType::TransportStart);
        assert!(event.duration_ms.is_none());
        assert!(event.details.is_none());
    }

    #[test]
    fn event_with_details() {
        let event = RequestEvent::new(RequestEventType::TransportFailed)
            .with_details("connection reset by peer");
        assert_eq!(event.details, Some("connection reset by peer".to_string()));
    }

    #[test]
    fn event_with_duration() {
        let event = RequestEvent::with_duration(
            RequestEventType::TransportComplete,
            Duration::from_millis(50),
        );
        assert_eq!(event.duration_ms, Some(50));
    }

    // =========================================================================
    // System Usage / Machine ID integration tests
    // =========================================================================

    #[test]
    fn json_without_system_info_omits_fields() {
        // When no cpu_monitor or machine_id is set, the JSON should not contain those keys
        // (validated by skip_serializing_if on both optional fields).
        let ctx = make_context_with(
            ActivityId::from_string("test-no-system-info".to_string()),
            |builder| {
                builder.set_operation_status(StatusCode::Ok, None);
            },
        );
        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let actual = normalize_diagnostics_json(json);
        let expected: serde_json::Value = serde_json::json!({
            "activity_id": "test-no-system-info",
            "total_duration_ms": 0,
            "total_request_charge": 0.0,
            "request_count": 0,
            "requests": []
        });
        assert_eq!(
            actual, expected,
            "JSON without system info mismatch.\nActual:\n{json}"
        );
    }

    #[test]
    fn json_with_machine_id() {
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::from_string("test-machine-id".to_string()),
            make_options(),
        );
        builder.set_operation_status(StatusCode::Ok, None);
        builder.set_machine_id(Arc::new("vmId_test-vm-123".to_string()));
        let ctx = builder.complete();

        // Detailed mode
        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let actual = normalize_diagnostics_json(json);
        let expected: serde_json::Value = serde_json::json!({
            "activity_id": "test-machine-id",
            "total_duration_ms": 0,
            "total_request_charge": 0.0,
            "request_count": 0,
            "machine_id": "vmId_test-vm-123",
            "requests": []
        });
        assert_eq!(
            actual, expected,
            "Detailed JSON with machine_id mismatch.\nActual:\n{json}"
        );

        // Summary mode
        let json_summary = ctx.to_json_string(Some(DiagnosticsVerbosity::Summary));
        let actual_summary = normalize_diagnostics_json(json_summary);
        let expected_summary: serde_json::Value = serde_json::json!({
            "activity_id": "test-machine-id",
            "total_duration_ms": 0,
            "total_request_charge": 0.0,
            "request_count": 0,
            "machine_id": "vmId_test-vm-123",
            "regions": []
        });
        assert_eq!(
            actual_summary, expected_summary,
            "Summary JSON with machine_id mismatch.\nActual:\n{json_summary}"
        );
    }

    #[test]
    fn json_with_system_usage() {
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::from_string("test-system-usage".to_string()),
            make_options(),
        );
        builder.set_operation_status(StatusCode::Ok, None);
        builder.set_test_system_usage(SystemUsageSnapshot::new_for_test(
            vec!["(50.0%)".to_string(), "(60.0%)".to_string()],
            Some(4096),
            4,
            false,
        ));
        let ctx = builder.complete();

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let actual = normalize_diagnostics_json(json);
        let expected: serde_json::Value = serde_json::json!({
            "activity_id": "test-system-usage",
            "total_duration_ms": 0,
            "total_request_charge": 0.0,
            "request_count": 0,
            "system_usage": {
                "cpu": {
                    "samples": ["(50.0%)", "(60.0%)"],
                    "status": "available"
                },
                "memory_available_mb": 4096,
                "processor_count": 4,
                "cpu_overloaded": false
            },
            "requests": []
        });
        assert_eq!(
            actual, expected,
            "JSON with system_usage mismatch.\nActual:\n{json}"
        );
    }

    #[test]
    fn json_system_usage_without_cpu_samples_is_structured_not_empty_sentinel() {
        // When the CPU sampler has no samples (cold start, or not running in the
        // host environment), the `cpu` field must serialize as a structured
        // object with an empty `samples` array and `status: "unavailable"` -
        // never the legacy literal string "empty" (a type-punned sentinel that
        // breaks downstream JSON consumers).
        let mut builder = DiagnosticsContextBuilder::new(
            ActivityId::from_string("test-system-usage-empty".to_string()),
            make_options(),
        );
        builder.set_operation_status(StatusCode::Ok, None);
        builder.set_test_system_usage(SystemUsageSnapshot::new_for_test(
            Vec::new(),
            None,
            4,
            false,
        ));
        let ctx = builder.complete();

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        let actual = normalize_diagnostics_json(json);
        let expected: serde_json::Value = serde_json::json!({
            "activity_id": "test-system-usage-empty",
            "total_duration_ms": 0,
            "total_request_charge": 0.0,
            "request_count": 0,
            "system_usage": {
                "cpu": {
                    "samples": [],
                    "status": "unavailable"
                },
                "processor_count": 4,
                "cpu_overloaded": false
            },
            "requests": []
        });
        assert_eq!(
            actual, expected,
            "JSON with empty system_usage cpu mismatch.\nActual:\n{json}"
        );
    }

    #[test]
    fn machine_id_getter() {
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        builder.set_machine_id(Arc::new("uuid_abc-123".to_string()));
        let ctx = builder.complete();

        assert_eq!(ctx.machine_id(), Some("uuid_abc-123"));
    }

    #[test]
    fn machine_id_none_when_not_set() {
        let builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        let ctx = builder.complete();
        assert_eq!(ctx.machine_id(), None);
    }

    // ---- Compaction (retry-storm bounding, WS6) -------------------------------------------

    fn options_with_cap(cap: usize) -> Arc<DiagnosticsOptions> {
        Arc::new(
            DiagnosticsOptions::builder()
                .with_max_request_diagnostics(cap)
                .build()
                .expect("valid cap"),
        )
    }

    /// Records `count` attempts sharing one (region, endpoint, status, exec-ctx)
    /// key, each charged `charge` RU, so per-run aggregates are deterministic.
    fn record_run(
        builder: &mut DiagnosticsContextBuilder,
        exec: ExecutionContext,
        region: &str,
        endpoint: &str,
        status: CosmosStatus,
        charge: f64,
        count: usize,
    ) {
        for _ in 0..count {
            let h =
                builder.start_test_request(exec, Some(Region::new(region.to_string())), endpoint);
            builder.update_request(h, |req| req.with_charge(RequestCharge::new(charge)));
            builder.complete_request(h, status.status_code(), status.sub_status());
        }
    }

    /// Repeated aggregation (as `PageAggregator` does when folding to bound
    /// retained sources) must not lose the per-run rollup: a run's `count` is
    /// exact across any number of folds, never double-counted and never reduced
    /// to the retained-sample size.
    #[test]
    fn aggregate_sub_operations_preserves_run_counts_across_repeated_folds() {
        const RUN: usize = 600;
        const PER_FOLD: usize = 9;
        const FOLDS: usize = 3;
        let cap = 16;
        let storm = |id: String| {
            let mut b =
                DiagnosticsContextBuilder::new(ActivityId::from_string(id), options_with_cap(cap));
            record_run(
                &mut b,
                ExecutionContext::OperationRetry,
                "East US",
                "https://east/",
                CosmosStatus::new(StatusCode::TooManyRequests),
                2.0,
                RUN,
            );
            b.set_operation_status(StatusCode::Ok, None);
            Arc::new(b.complete())
        };

        let first = storm("fold-seed".to_string());
        assert_eq!(
            first
                .compaction()
                .expect("source must be compacted")
                .runs
                .iter()
                .map(|r| r.count)
                .sum::<usize>(),
            RUN
        );

        // Fold in batches, feeding each result back in as a source — the shape
        // `PageAggregator` produces once it starts folding. Each batch retains
        // more records than the cap, so the over-cap branch is exercised.
        let mut folded = first;
        for fold in 0..FOLDS {
            let mut batch = vec![folded];
            batch.extend((0..PER_FOLD).map(|i| storm(format!("fold-{fold}-{i}"))));
            folded = Arc::new(
                DiagnosticsContext::aggregate_sub_operations(&batch)
                    .expect("aggregation must succeed"),
            );
            assert!(
                folded
                    .compaction()
                    .is_some_and(|i| i.retained_truncated
                        || i.retained_request_count < i.original_request_count),
                "each fold must exercise real compaction"
            );
        }

        let total = RUN * (1 + PER_FOLD * FOLDS);
        assert_eq!(folded.request_count(), total);
        assert!(folded.retained_request_count() <= cap);
        let info = folded
            .compaction()
            .expect("aggregate of compacted sources must carry compaction metadata");
        assert_eq!(info.original_request_count, total);
        assert_eq!(
            info.runs.iter().map(|r| r.count).sum::<usize>(),
            total,
            "the run rollup must account for every attempt across folds"
        );
        // Exact charge survives in the rollup too (2.0 RU per attempt).
        let rollup_charge: f64 = info
            .runs
            .iter()
            .map(|r| r.total_request_charge.value())
            .sum();
        assert!((rollup_charge - total as f64 * 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn compaction_does_not_collapse_distinct_patch_sub_operations() {
        // A run is meant to be a storm of retries of *the same* attempt. A
        // PATCH's Read and Replace hit the same endpoint and can return the
        // same status, so without the issuing operation in the compaction key
        // the aggregate's re-bounding pass collapses them into one run whose RU
        // total and duration percentiles silently mix a read with a write.
        //
        // Sized so neither sub-op compacts on its own (9 < cap) but their
        // concatenation does (18 > cap), which is the only path where a single
        // compaction pass ever sees requests from more than one operation.
        let cap = 16;
        let per_sub_op = 9;
        let mut read_b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("patch-read".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut read_b,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::Ok),
            1.0,
            per_sub_op,
        );
        read_b.set_operation_name("patch_read_item");
        read_b.set_operation_status(StatusCode::Ok, None);
        let read_ctx = Arc::new(read_b.complete());
        assert!(
            read_ctx.compaction().is_none(),
            "sub-op must be under the cap so the aggregate does the compacting"
        );

        let mut replace_b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("patch-replace".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut replace_b,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::Ok),
            10.0,
            per_sub_op,
        );
        replace_b.set_operation_name("patch_replace_item");
        replace_b.set_operation_status(StatusCode::Ok, None);
        let replace_ctx = Arc::new(replace_b.complete());
        assert!(replace_ctx.compaction().is_none());

        let aggregated = DiagnosticsContext::aggregate_sub_operations(&[read_ctx, replace_ctx])
            .expect("aggregation of two contexts yields Some")
            .with_operation_name(Some(Arc::from("patch_item")));

        let info = aggregated
            .compaction()
            .expect("18 concatenated attempts past a cap of 16 must compact");
        assert_eq!(
            info.runs.len(),
            2,
            "the read and the replace must be reported as separate runs"
        );

        // Each run's RU total reflects one sub-op, not a blend of both.
        let mut charges: Vec<f64> = info
            .runs
            .iter()
            .map(|r| r.total_request_charge.value())
            .collect();
        charges.sort_by(f64::total_cmp);
        assert_eq!(
            charges,
            vec![9.0, 90.0],
            "runs must not blend the read's 1 RU attempts with the replace's 10 RU attempts"
        );
    }

    #[test]
    fn retry_storm_429_is_bounded_and_lossless() {
        // A single partition hammered with 429 for the whole retry budget: one
        // run of many near-identical attempts. The retained list must be bounded
        // by the cap while the true count and exact aggregates survive.
        let cap = 16;
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("storm-429".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut b,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::TooManyRequests),
            2.0,
            1000,
        );
        b.set_operation_status(StatusCode::TooManyRequests, None);
        let ctx = b.complete();

        // True total preserved; retained records bounded by the cap.
        assert_eq!(ctx.request_count(), 1000);
        assert!(
            ctx.retained_request_count() <= cap,
            "retained {} exceeds cap {cap}",
            ctx.retained_request_count()
        );

        let info = ctx.compaction().expect("a storm past the cap must compact");
        assert_eq!(info.original_request_count, 1000);
        assert_eq!(info.retained_request_count, ctx.retained_request_count());
        assert_eq!(info.runs.len(), 1);
        assert_eq!(info.runs[0].count, 1000);

        // Aggregates stay EXACT despite the bounded retained list.
        assert_eq!(
            info.runs[0].total_request_charge,
            RequestCharge::new(2000.0)
        );
        assert_eq!(ctx.total_request_charge(), RequestCharge::new(2000.0));
        assert!(info.runs[0].min_duration_ms <= info.runs[0].p50_duration_ms);
        assert!(info.runs[0].p50_duration_ms <= info.runs[0].max_duration_ms);

        // Truncation is visible in the serialized output and the output is bounded.
        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        assert!(json.contains("\"compaction\""), "compaction marker missing");
        assert!(json.contains("\"original_request_count\":1000"));
        assert!(
            json.len() < 16 * 1024,
            "detailed json {} bytes is not bounded",
            json.len()
        );

        // First and last of the run are retained in full.
        let requests = ctx.requests();
        assert_eq!(
            u16::from(requests.first().unwrap().status().status_code()),
            429
        );
        assert_eq!(
            u16::from(requests.last().unwrap().status().status_code()),
            429
        );
    }

    #[test]
    fn mixed_429_410_runs_preserve_boundaries_and_exact_counts() {
        // A 429 storm that escalates to a 410/1002 (PartitionKeyRangeGone) storm
        // then recovers with a 200. Order-preserving run-length collapse keeps
        // each run's boundaries and exact per-run aggregates.
        let cap = 16;
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("mixed-429-410".to_string()),
            options_with_cap(cap),
        );
        record_run(
            &mut b,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::TooManyRequests),
            1.0,
            100,
        );
        record_run(
            &mut b,
            ExecutionContext::OperationRetry,
            "East US",
            "https://east/",
            CosmosStatus::new(StatusCode::Gone).with_sub_status(1002),
            1.0,
            50,
        );
        record_run(
            &mut b,
            ExecutionContext::OperationRetry,
            "West US",
            "https://west/",
            CosmosStatus::new(StatusCode::Ok),
            1.0,
            1,
        );
        b.set_operation_status(StatusCode::Ok, None);
        let ctx = b.complete();

        assert_eq!(ctx.request_count(), 151);
        assert!(ctx.retained_request_count() <= cap);
        let info = ctx.compaction().expect("compacted");
        assert_eq!(info.runs.len(), 3);
        assert_eq!(info.runs[0].count, 100);
        assert_eq!(info.runs[1].count, 50);
        assert_eq!(info.runs[2].count, 1);

        // The 410 run carries its sub-status exactly.
        assert_eq!(
            info.runs[1].status,
            CosmosStatus::new(StatusCode::Gone).with_sub_status(1002)
        );

        // Exact, lossless totals across all three runs.
        let run_attempts: usize = info.runs.iter().map(|r| r.count).sum();
        assert_eq!(run_attempts, 151);
        assert_eq!(ctx.total_request_charge(), RequestCharge::new(151.0));

        // Onset (429) and terminal (200) boundaries retained; order preserved.
        let requests = ctx.requests();
        assert_eq!(
            u16::from(requests.first().unwrap().status().status_code()),
            429
        );
        assert_eq!(
            u16::from(requests.last().unwrap().status().status_code()),
            200
        );
    }

    #[test]
    fn region_ping_pong_is_bounded_via_global_fallback() {
        // Alternating regions: every consecutive run is length one, defeating the
        // order-preserving run-length collapse and forcing the order-robust
        // global key-bucket fallback. The artifact must still stay bounded.
        let cap = 16;
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("pingpong".to_string()),
            options_with_cap(cap),
        );
        for _ in 0..200 {
            let he = b.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::new("East US")),
                "https://east/",
            );
            b.complete_request(he, StatusCode::ServiceUnavailable, None);
            let hw = b.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::new("West US")),
                "https://west/",
            );
            b.complete_request(hw, StatusCode::ServiceUnavailable, None);
        }
        b.set_operation_status(StatusCode::ServiceUnavailable, None);
        let ctx = b.complete();

        assert_eq!(ctx.request_count(), 400);
        assert!(
            ctx.retained_request_count() <= cap,
            "retained {} exceeds cap {cap}",
            ctx.retained_request_count()
        );
        let info = ctx.compaction().expect("ping-pong storm must compact");
        // Two distinct keys -> two runs, each covering 200 attempts.
        assert_eq!(info.runs.len(), 2);
        assert!(info.runs.iter().all(|r| r.count == 200));

        // Both regions still reported (normalized: lowercase, no spaces).
        let mut regions: Vec<String> = ctx
            .regions_contacted()
            .iter()
            .map(|r| r.as_str().to_string())
            .collect();
        regions.sort();
        assert_eq!(regions, ["eastus".to_string(), "westus".to_string()]);
    }

    #[test]
    fn distinct_endpoint_410_fanout_is_bounded() {
        // A 410 fan-out across thousands of physical-partition endpoints is
        // high-cardinality exactly when the storm is worst: every attempt is a
        // distinct key. Both the retained records AND the per-run rollup must
        // stay bounded by the cap, the omission explicit, the total exact.
        let cap = 16;
        let distinct = 5000usize;
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("fanout".to_string()),
            options_with_cap(cap),
        );
        for i in 0..distinct {
            record_run(
                &mut b,
                ExecutionContext::OperationRetry,
                "East US",
                &format!("https://pkrange-{i}/"),
                CosmosStatus::new(StatusCode::Gone).with_sub_status(1002),
                1.0,
                1,
            );
        }
        b.set_operation_status(StatusCode::Gone, Some(SubStatusCode::new(1002)));
        let ctx = b.complete();

        assert_eq!(ctx.request_count(), distinct);
        let info = ctx
            .compaction()
            .expect("a high-cardinality storm must compact");
        assert_eq!(info.original_request_count, distinct);
        assert_eq!(info.total_runs, distinct);

        // Both the retained records and the per-run rollup are bounded by the cap.
        assert!(
            ctx.retained_request_count() <= cap,
            "retained {} exceeds cap {cap}",
            ctx.retained_request_count()
        );
        assert!(
            info.runs.len() <= cap,
            "runs {} not bounded by cap {cap}",
            info.runs.len()
        );

        // Every drop is explicit, never silent, and lossless in aggregate.
        assert!(info.omitted_runs > 0, "run omission must be marked");
        assert_eq!(info.omitted_runs, info.total_runs - info.runs.len());
        let retained_run_attempts: usize = info.runs.iter().map(|r| r.count).sum();
        assert_eq!(retained_run_attempts + info.omitted_request_count, distinct);

        // Detailed JSON size is bounded by the cap, independent of the topology.
        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        assert!(json.contains("\"omitted_runs\""), "omission not surfaced");
        assert!(
            json.len() < 32 * 1024,
            "detailed json {} bytes grows with topology (distinct={distinct})",
            json.len()
        );
    }

    #[test]
    fn phase2_heterogeneous_runs_keep_largest_and_stay_coherent() {
        // Phase 2 with heterogeneous run counts: a few "hot" keys (introduced
        // LAST) each retry many times, interleaved with many "cold" single-attempt
        // keys (introduced FIRST). The rollup must keep the largest runs, the
        // retained records must be drawn from the SAME kept set (so a span emitter
        // never sees an attempt whose run was omitted), and totals stay exact.
        let cap = 16;
        let cold = 40usize;
        let hot = 3usize;
        let hot_retries = 100usize;
        let mut b = DiagnosticsContextBuilder::new(
            ActivityId::from_string("heterogeneous".to_string()),
            options_with_cap(cap),
        );

        for i in 0..cold {
            let h = b.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::new("East US")),
                &format!("https://cold-{i}/"),
            );
            b.complete_request(h, StatusCode::TooManyRequests, None);
        }
        // Interleaved so no two consecutive attempts share a key (forces Phase 2).
        for _round in 0..hot_retries {
            for j in 0..hot {
                let h = b.start_test_request(
                    ExecutionContext::OperationRetry,
                    Some(Region::new("West US")),
                    &format!("https://hot-{j}/"),
                );
                b.complete_request(h, StatusCode::ServiceUnavailable, None);
            }
        }
        b.set_operation_status(StatusCode::ServiceUnavailable, None);
        let ctx = b.complete();

        let total = cold + hot * hot_retries;
        assert_eq!(ctx.request_count(), total);
        let info = ctx
            .compaction()
            .expect("a heterogeneous storm must compact");
        assert_eq!(info.total_runs, cold + hot);

        assert!(
            info.runs.len() <= cap,
            "runs {} exceed cap {cap}",
            info.runs.len()
        );
        assert!(
            ctx.retained_request_count() <= cap,
            "retained {} exceed cap {cap}",
            ctx.retained_request_count()
        );

        // The largest (hot) runs are the ones kept in the rollup.
        let kept_hot = info.runs.iter().filter(|r| r.count == hot_retries).count();
        assert_eq!(
            kept_hot, hot,
            "all hot runs must survive the by-count rollup"
        );

        // Coherence: every retained record's key is represented by a run.
        let run_keys: std::collections::HashSet<(
            Option<String>,
            String,
            CosmosStatus,
            ExecutionContext,
        )> = info
            .runs
            .iter()
            .map(|r| {
                (
                    r.region.clone(),
                    r.endpoint.clone(),
                    r.status,
                    r.execution_context,
                )
            })
            .collect();
        for rec in ctx.requests().iter() {
            let id = (
                rec.region().map(|r| r.as_str().to_string()),
                rec.endpoint().to_string(),
                *rec.status(),
                rec.execution_context(),
            );
            assert!(
                run_keys.contains(&id),
                "retained record {id:?} has no matching run in the bounded rollup"
            );
        }

        // Exact, lossless totals despite the bounded rollup + retained list.
        let kept_run_attempts: usize = info.runs.iter().map(|r| r.count).sum();
        assert_eq!(kept_run_attempts + info.omitted_request_count, total);
        assert_eq!(info.omitted_runs, info.total_runs - info.runs.len());
    }

    #[test]
    fn under_cap_is_not_compacted_and_output_has_no_marker() {
        // The default cap (512) is far above a normal operation's attempts, so
        // the list is retained verbatim and the output is byte-identical to the
        // pre-compaction behavior (no compaction marker).
        let ctx = make_context_with(ActivityId::from_string("normal".to_string()), |b| {
            for _ in 0..3 {
                let h = b.start_test_request(
                    ExecutionContext::OperationRetry,
                    Some(Region::new("East US")),
                    "https://east/",
                );
                b.complete_request(h, StatusCode::TooManyRequests, None);
            }
        });

        assert!(ctx.compaction().is_none());
        assert_eq!(ctx.request_count(), 3);
        assert_eq!(ctx.retained_request_count(), 3);
        for verbosity in [
            DiagnosticsVerbosity::Detailed,
            DiagnosticsVerbosity::Summary,
        ] {
            let json = ctx.to_json_string(Some(verbosity));
            assert!(
                !json.contains("compaction"),
                "{verbosity} output must not carry a compaction marker: {json}"
            );
        }
    }

    #[test]
    fn operation_name_defaults_to_none() {
        let ctx = make_context_with(ActivityId::new_uuid(), |_| {});
        assert_eq!(ctx.operation_name(), None);
    }

    #[test]
    fn patch_tracking_id_is_accessible_serialized_and_preserved_by_restamping() {
        let id = PatchTrackingId::from(uuid::Uuid::from_u128(42));
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.set_operation_name("patch_read_item");
        })
        .with_patch_tracking_id(id)
        .with_operation_name(Some(Arc::from("patch_item")));

        assert_eq!(ctx.patch_tracking_id(), Some(id));
        for verbosity in [
            DiagnosticsVerbosity::Detailed,
            DiagnosticsVerbosity::Summary,
        ] {
            let json: serde_json::Value =
                serde_json::from_str(ctx.to_json_string(Some(verbosity))).unwrap();
            assert_eq!(json["patch_tracking_id"], id.to_string());
        }

        let cloned = ctx.clone_with_operation_name(Some(Arc::from("patch_item")));
        assert_eq!(cloned.patch_tracking_id(), Some(id));
    }

    #[test]
    fn set_operation_name_populates_completed_context() {
        let ctx = make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_name("read_item");
        });
        assert_eq!(ctx.operation_name(), Some("read_item"));
    }

    #[test]
    fn with_operation_name_overrides_after_construction() {
        let ctx = make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_name("replace_item");
        });
        assert_eq!(ctx.operation_name(), Some("replace_item"));

        let overridden = ctx.with_operation_name(Some(Arc::from("patch_item")));
        assert_eq!(overridden.operation_name(), Some("patch_item"));

        let cleared = overridden.with_operation_name(None);
        assert_eq!(cleared.operation_name(), None);
    }

    #[test]
    fn aggregate_sub_operations_can_override_operation_name() {
        // Mirrors the PATCH handler: a Read + Replace aggregate would inherit
        // `replace_item` from the last source, but `with_operation_name` lets
        // the caller stamp the virtual operation's own name.
        let read = Arc::new(make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_name("read_item");
        }));
        let replace = Arc::new(make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_name("replace_item");
        }));

        let inherited =
            DiagnosticsContext::aggregate_sub_operations(&[read.clone(), replace.clone()])
                .expect("aggregation of two contexts yields Some");
        assert_eq!(inherited.operation_name(), Some("replace_item"));

        let stamped = DiagnosticsContext::aggregate_sub_operations(&[read, replace])
            .expect("aggregation of two contexts yields Some")
            .with_operation_name(Some(Arc::from("patch_item")));
        assert_eq!(stamped.operation_name(), Some("patch_item"));
    }

    #[test]
    fn aggregate_sub_operations_preserves_per_request_operation_names() {
        // A PATCH reports `patch_item` at the operation level, but its requests
        // were issued by the `patch_read_item` / `patch_replace_item` sub-ops.
        // Aggregation must push each source's name down onto the requests it
        // contributed, or the decomposition is lost the moment the contexts are
        // concatenated.
        let read_ctx = Arc::new(make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_name("patch_read_item");
            builder.set_operation_status(StatusCode::Ok, None);
        }));
        let replace_ctx = Arc::new(make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_name("patch_replace_item");
            builder.set_operation_status(StatusCode::Ok, None);
        }));

        let aggregated = DiagnosticsContext::aggregate_sub_operations(&[read_ctx, replace_ctx])
            .expect("aggregation of two contexts yields Some")
            .with_operation_name(Some(Arc::from("patch_item")));

        assert_eq!(aggregated.operation_name(), Some("patch_item"));
        let requests = aggregated.requests();
        let names: Vec<Option<&str>> = requests
            .iter()
            .map(RequestDiagnostics::operation_name)
            .collect();
        assert_eq!(
            names,
            vec![Some("patch_read_item"), Some("patch_replace_item")],
            "each request must keep the sub-op that issued it"
        );
    }

    #[test]
    fn single_operation_requests_carry_no_redundant_name() {
        // The overwhelmingly common case: one operation, N attempts. The
        // per-request name stays unset so it costs nothing and the diagnostics
        // JSON is unchanged; consumers fall back to the context's name.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.start_test_request(
                ExecutionContext::OperationRetry,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_name("read_item");
            builder.set_operation_status(StatusCode::Ok, None);
        });

        assert_eq!(ctx.operation_name(), Some("read_item"));
        assert!(
            ctx.requests()
                .iter()
                .all(|req| req.operation_name().is_none()),
            "a single-operation context must not duplicate its name onto every request"
        );

        // Re-stamping with the *same* name is a no-op rather than a reason to
        // populate every request.
        let restamped = ctx.clone_with_operation_name(Some(Arc::from("read_item")));
        assert!(restamped
            .requests()
            .iter()
            .all(|req| req.operation_name().is_none()));
    }

    #[test]
    fn clone_with_operation_name_preserves_displaced_request_identity() {
        // The PATCH error path re-stamps a single sub-op context (it does not
        // aggregate when only one context exists). The requests must still
        // remember they came from the Read, otherwise a PATCH that fails during
        // its internal Read reports `patch_item` on the attempt span and the
        // read/replace split disappears exactly when it matters most.
        let read_ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_name("patch_read_item");
            builder.set_operation_status(StatusCode::NotFound, None);
        });

        let stamped = read_ctx.clone_with_operation_name(Some(Arc::from("patch_item")));

        assert_eq!(stamped.operation_name(), Some("patch_item"));
        assert_eq!(
            stamped.requests()[0].operation_name(),
            Some("patch_read_item")
        );
        // The source context is untouched.
        assert_eq!(read_ctx.requests()[0].operation_name(), None);
    }

    #[test]
    fn nested_aggregation_keeps_the_innermost_request_identity() {
        // Aggregating an aggregate (a PATCH whose sub-ops were themselves
        // aggregated) must not overwrite names that are already more specific
        // than the enclosing context's.
        let read_ctx = Arc::new(make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_name("patch_read_item");
            builder.set_operation_status(StatusCode::Ok, None);
        }));
        let replace_ctx = Arc::new(make_context_with(ActivityId::new_uuid(), |builder| {
            builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com",
            );
            builder.set_operation_name("patch_replace_item");
            builder.set_operation_status(StatusCode::Ok, None);
        }));

        let inner = Arc::new(
            DiagnosticsContext::aggregate_sub_operations(&[read_ctx, replace_ctx])
                .expect("aggregation of two contexts yields Some")
                .with_operation_name(Some(Arc::from("patch_item"))),
        );
        let outer = DiagnosticsContext::aggregate_sub_operations(&[inner])
            .expect("aggregation of one context yields Some")
            .with_operation_name(Some(Arc::from("patch_item")));

        let requests = outer.requests();
        let names: Vec<Option<&str>> = requests
            .iter()
            .map(RequestDiagnostics::operation_name)
            .collect();
        assert_eq!(
            names,
            vec![Some("patch_read_item"), Some("patch_replace_item")]
        );
    }

    #[test]
    fn is_failure_reflects_operation_status() {
        let ok = make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_status(StatusCode::Ok, None);
        });
        assert!(!ok.is_failure());

        let failed = make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_status(StatusCode::TooManyRequests, None);
        });
        assert!(failed.is_failure());

        // No recorded status is not a failure.
        let no_status = make_context_with(ActivityId::new_uuid(), |_| {});
        assert!(!no_status.is_failure());
    }

    #[test]
    fn is_failure_falls_back_to_terminal_request_status() {
        // Some driver error-finalization paths graft diagnostics onto the error
        // without stamping the operation status. The terminal attempt's failed
        // status must still surface as a failure so tail-sampled handlers emit.
        let failed = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.complete_request(handle, StatusCode::TooManyRequests, None);
        });
        assert!(failed.is_failure());

        // A successful terminal attempt with no operation status is not a failure.
        let ok = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.complete_request(handle, StatusCode::Ok, None);
        });
        assert!(!ok.is_failure());
    }

    #[test]
    fn is_completed_requires_status_or_request() {
        let empty = make_context_with(ActivityId::new_uuid(), |_| {});
        assert!(!empty.is_completed());

        let with_status = make_context_with(ActivityId::new_uuid(), |b| {
            b.set_operation_status(StatusCode::Ok, None);
        });
        assert!(with_status.is_completed());

        let with_request = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.complete_request(handle, StatusCode::Ok, None);
        });
        assert!(with_request.is_completed());
    }

    #[test]
    fn is_threshold_violated_on_request_charge() {
        let thresholds = DiagnosticsThresholds::default().with_request_charge(100.0);

        let cheap = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.update_request(handle, |req| req.request_charge = RequestCharge::new(10.0));
            b.complete_request(handle, StatusCode::Ok, None);
        });
        assert!(!cheap.is_threshold_violated(&thresholds));

        let expensive = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.update_request(handle, |req| req.request_charge = RequestCharge::new(150.0));
            b.complete_request(handle, StatusCode::Ok, None);
        });
        assert!(expensive.is_threshold_violated(&thresholds));
    }

    #[test]
    fn threshold_breach_reports_the_specific_bound() {
        let thresholds = DiagnosticsThresholds::default().with_request_charge(100.0);

        // Cheap, fast success: no breach.
        let cheap = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.update_request(handle, |req| req.request_charge = RequestCharge::new(10.0));
            b.complete_request(handle, StatusCode::Ok, None);
        });
        assert_eq!(
            cheap.threshold_breach_for(&thresholds, Some("read_item")),
            None
        );

        // Over the RU bound: the breach names the request charge.
        let expensive = make_context_with(ActivityId::new_uuid(), |b| {
            let handle = b.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com",
            );
            b.update_request(handle, |req| req.request_charge = RequestCharge::new(150.0));
            b.complete_request(handle, StatusCode::Ok, None);
        });
        assert_eq!(
            expensive.threshold_breach_for(&thresholds, Some("read_item")),
            Some(ThresholdBreach::RequestCharge)
        );
    }
}
