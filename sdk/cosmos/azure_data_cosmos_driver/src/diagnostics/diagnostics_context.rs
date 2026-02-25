// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The main diagnostics context for tracking operation-level diagnostics.
//!
//! This module contains all core diagnostics types including execution context,
//! request diagnostics, pipeline classification types, request events,
//! serialization helpers, and the diagnostics context itself.

use crate::{
    models::{ActivityId, CosmosStatus, RequestCharge, SubStatusCode},
    options::{DiagnosticsOptions, DiagnosticsVerbosity, Region},
    system::CpuMemoryMonitor,
};
use azure_core::http::StatusCode;
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
    time::{Duration, Instant},
};

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
    /// Retry due to transient error (e.g., 429, 503).
    Retry,
    /// Hedged request for latency reduction.
    Hedging,
    /// Region failover attempt.
    RegionFailover,
    /// Circuit breaker recovery probe.
    CircuitBreakerProbe,
}

impl std::fmt::Display for ExecutionContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionContext::Initial => write!(f, "initial"),
            ExecutionContext::Retry => write!(f, "retry"),
            ExecutionContext::Hedging => write!(f, "hedging"),
            ExecutionContext::RegionFailover => write!(f, "region_failover"),
            ExecutionContext::CircuitBreakerProbe => write!(f, "circuit_breaker_probe"),
        }
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
    /// Returns true if this is a metadata (control plane) pipeline.
    pub fn is_metadata(self) -> bool {
        matches!(self, PipelineType::Metadata)
    }

    /// Returns true if this is a data plane pipeline.
    pub fn is_data_plane(self) -> bool {
        matches!(self, PipelineType::DataPlane)
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

impl TransportSecurity {
    /// Returns true if this is a secure transport.
    pub fn is_secure(self) -> bool {
        matches!(self, TransportSecurity::Secure)
    }

    /// Returns true if this is an emulator transport with insecure certificates.
    pub fn is_emulator(self) -> bool {
        matches!(self, TransportSecurity::EmulatorWithInsecureCertificates)
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

    /// The pipeline type used for this request.
    pipeline_type: PipelineType,

    /// The transport security mode used for this request.
    transport_security: TransportSecurity,

    /// Region this request was sent to.
    region: Option<Region>,

    /// Endpoint URI contacted.
    endpoint: String,

    /// Combined HTTP status code and Cosmos sub-status code.
    #[serde(flatten)]
    status: CosmosStatus,

    /// Request charge (RU) for this individual request.
    pub(crate) request_charge: RequestCharge,

    /// Activity ID from response headers.
    activity_id: Option<ActivityId>,

    /// Session token from response (for session consistency).
    session_token: Option<String>,

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
}

impl RequestDiagnostics {
    /// Creates a new request diagnostics entry for a request being started.
    pub(crate) fn new(
        execution_context: ExecutionContext,
        pipeline_type: PipelineType,
        transport_security: TransportSecurity,
        region: Option<Region>,
        endpoint: String,
    ) -> Self {
        Self {
            execution_context,
            pipeline_type,
            transport_security,
            region,
            endpoint,
            // Status is set when the request completes via `complete()`.
            // Using 0 as sentinel value for "not yet completed".
            status: CosmosStatus::new(StatusCode::from(0)),
            request_charge: RequestCharge::default(),
            activity_id: None,
            session_token: None,
            started_at: Instant::now(),
            completed_at: None,
            duration_ms: 0,
            events: Vec::new(),
            timed_out: false,
            request_sent: RequestSentStatus::Unknown,
            error: None,
        }
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

    /// Records failure of this request with an error message.
    ///
    /// Use this for transport-level failures (connection errors, DNS failures, etc.)
    /// where no HTTP response was received.
    ///
    /// # Note on retry safety
    ///
    /// The `request_sent` parameter indicates whether the request bytes were
    /// written to the network. This is critical for determining retry safety:
    /// - `NotSent`: Safe to retry any operation
    /// - `Sent`: Only safe to retry idempotent operations
    /// - `Unknown`: Treat as potentially sent (conservative)
    pub(crate) fn fail(&mut self, error: impl Into<String>, request_sent: RequestSentStatus) {
        self.completed_at = Some(Instant::now());
        self.with_error(error);
        self.request_sent = request_sent;
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

    /// Adds a pipeline event.
    pub(crate) fn add_event(&mut self, event: RequestEvent) {
        self.events.push(event);
    }

    /// Returns whether this request has been completed.
    pub(crate) fn is_completed(&self) -> bool {
        self.completed_at.is_some()
    }

    // Public getters for read-only access to fields

    /// Returns the execution context describing why this request was made.
    pub fn execution_context(&self) -> ExecutionContext {
        self.execution_context
    }

    /// Returns the pipeline type used for this request.
    pub fn pipeline_type(&self) -> PipelineType {
        self.pipeline_type
    }

    /// Returns the transport security mode used for this request.
    pub fn transport_security(&self) -> TransportSecurity {
        self.transport_security
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
    Requests {
        requests: &'a [RequestDiagnostics],
    },
    /// Summary payload containing region-level summaries.
    Summary {
        regions: Vec<RegionSummary>,
    },
}

/// Diagnostics output structure for JSON serialization.
#[derive(Serialize)]
struct DiagnosticsOutput<'a> {
    activity_id: &'a ActivityId,
    total_duration_ms: u64,
    total_request_charge: RequestCharge,
    request_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    system_usage: Option<SystemUsageSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    machine_id: Option<&'a str>,
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
    total_duration_ms: u64,
    request_count: usize,
    truncated: bool,
    message: &'static str,
}

/// Snapshot of system CPU and memory usage at a point in time.
///
/// Captured lazily on first serialization of a [`DiagnosticsContext`] and
/// included in the JSON output under `"system_usage"`.
///
/// Field names mirror the Java SDK's `CosmosDiagnosticsSystemUsageSnapshot`:
/// - `"cpu"` – Recent CPU load history (e.g. `"(45.3%), (50.1%), ..."`)
/// - `"memory_available_mb"` – Most recent available memory in MB
/// - `"processor_count"` – Number of logical CPUs available to the process
#[derive(Clone, Debug, Serialize)]
struct SystemUsageSnapshot {
    /// Recent CPU load history formatted as a human-readable string.
    cpu: String,
    /// Available memory in megabytes (most recent sample).
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_available_mb: Option<u64>,
    /// Number of logical CPUs available to the process.
    processor_count: usize,
}

impl SystemUsageSnapshot {
    /// Captures a snapshot from the given CPU/memory monitor.
    fn capture(monitor: &CpuMemoryMonitor) -> Self {
        let history = monitor.snapshot();
        Self {
            cpu: history.to_string(),
            memory_available_mb: history.latest_memory_mb(),
            processor_count: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(1),
        }
    }
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

    /// Returns the operation-level activity ID.
    pub(crate) fn activity_id(&self) -> &ActivityId {
        &self.activity_id
    }

    /// Returns the number of tracked requests for this operation.
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
        region: Option<Region>,
        endpoint: String,
    ) -> RequestHandle {
        let request = RequestDiagnostics::new(
            execution_context,
            pipeline_type,
            transport_security,
            region,
            endpoint,
        );
        let handle = RequestHandle(self.requests.len());
        self.requests.push(request);
        handle
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
    }

    /// Records end-to-end timeout of a request.
    ///
    /// Should be called when a request times out before receiving a response
    /// due to hitting the end-to-end operation timeout. Sets the status to
    /// 408 (Request Timeout) with sub-status [`SubStatusCode::CLIENT_OPERATION_TIMEOUT`].
    ///
    /// For transport-level timeouts (connection timeouts, etc.), use
    /// [`fail_request`](Self::fail_request) instead with the appropriate error.
    pub(crate) fn timeout_request(&mut self, handle: RequestHandle) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.timeout();
        }
    }

    /// Records failure of a request with an error message.
    ///
    /// Should be called when a transport-level error occurs (connection failure,
    /// DNS error, TLS error, etc.) and no HTTP response was received.
    ///
    /// # Parameters
    ///
    /// - `handle`: The request handle from [`start_request`](Self::start_request)
    /// - `error`: The error message describing the failure
    /// - `request_sent`: Whether the request was sent on the wire before failure.
    ///   This is critical for retry safety - see [`RequestDiagnostics::fail`].
    pub(crate) fn fail_request(
        &mut self,
        handle: RequestHandle,
        error: impl Into<String>,
        request_sent: RequestSentStatus,
    ) {
        if let Some(request) = self.requests.get_mut(handle.0) {
            request.fail(error, request_sent);
        }
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

    /// Completes the builder and returns an immutable [`DiagnosticsContext`].
    ///
    /// This consumes the builder and creates a finalized diagnostics context
    /// with all data frozen. The `DiagnosticsContext` can then be safely
    /// shared via `Arc` without any locking overhead.
    pub(crate) fn complete(self) -> DiagnosticsContext {
        let duration = self.started_at.elapsed();
        DiagnosticsContext {
            activity_id: self.activity_id,
            duration,
            requests: Arc::new(self.requests),
            status: self.status,
            options: self.options,
            cpu_monitor: self.cpu_monitor,
            machine_id: self.machine_id,
            cached_json_detailed: OnceLock::new(),
            cached_json_summary: OnceLock::new(),
        }
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
#[derive(Debug)]
pub struct DiagnosticsContext {
    /// Operation-level activity ID.
    activity_id: ActivityId,

    /// Total duration of the operation (from start to completion).
    duration: Duration,

    /// All request diagnostics (shared via `Arc` for efficient multi-read).
    ///
    /// `Vec<T>` in Rust guarantees insertion order, so requests are stored in
    /// the order they were added.
    requests: Arc<Vec<RequestDiagnostics>>,

    /// Operation-level combined HTTP status and sub-status (final status after retries).
    status: Option<CosmosStatus>,

    /// Reference to diagnostics configuration.
    options: Arc<DiagnosticsOptions>,

    /// CPU/memory monitor for capturing system usage snapshots on first serialization.
    cpu_monitor: Option<CpuMemoryMonitor>,

    /// Machine identifier (VM ID on Azure, generated UUID otherwise).
    machine_id: Option<Arc<String>>,

    /// Cached JSON string for detailed verbosity.
    cached_json_detailed: OnceLock<String>,

    /// Cached JSON string for summary verbosity.
    cached_json_summary: OnceLock<String>,
}

impl DiagnosticsContext {
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

    /// Returns the total request charge (RU) across all requests.
    pub fn total_request_charge(&self) -> RequestCharge {
        self.requests.iter().map(|r| r.request_charge).sum()
    }

    /// Returns the number of requests made during this operation.
    pub fn request_count(&self) -> usize {
        self.requests.len()
    }

    /// Returns all regions contacted during this operation.
    pub fn regions_contacted(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = self.requests.iter().filter_map(|r| r.region.clone()).collect();
        regions.sort();
        regions.dedup();
        regions
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

    /// Returns the machine identifier, if available.
    ///
    /// On Azure VMs this is `"vmId_{vm-id}"` from IMDS; off Azure it is
    /// `"uuid_{generated-uuid}"` (stable for process lifetime).
    pub fn machine_id(&self) -> Option<&str> {
        self.machine_id.as_ref().map(|s| s.as_str())
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

    fn compute_json_detailed(&self) -> String {
        let total_duration_ms = self.duration.as_millis() as u64;
        let system_usage = self.cpu_monitor.as_ref().map(SystemUsageSnapshot::capture);
        let output = DiagnosticsOutput {
            activity_id: &self.activity_id,
            total_duration_ms,
            total_request_charge: self.requests.iter().map(|r| r.request_charge).sum(),
            request_count: self.requests.len(),
            system_usage,
            machine_id: self.machine_id.as_ref().map(|s| s.as_str()),
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
            total_duration_ms,
            total_request_charge: self.requests.iter().map(|r| r.request_charge).sum(),
            request_count: self.requests.len(),
            system_usage: self.cpu_monitor.as_ref().map(SystemUsageSnapshot::capture),
            machine_id: self.machine_id.as_ref().map(|s| s.as_str()),
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
                total_duration_ms,
                request_count: self.requests.len(),
                truncated: true,
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
            status: self.status,
            options: Arc::clone(&self.options),
            cpu_monitor: self.cpu_monitor.clone(),
            machine_id: self.machine_id.clone(),
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
        self.activity_id == other.activity_id
            && self.duration == other.duration
            && self.requests == other.requests
            && self.status == other.status
            && self.options == other.options
    }
}

impl Eq for DiagnosticsContext {}

/// Builds a summary for requests in a single region.
fn build_region_summary(region: Option<Region>, requests: Vec<&RequestDiagnostics>) -> RegionSummary {
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
fn percentile_sorted(values: &[u64], p: u8) -> u64 {
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
            endpoint: String,
        ) -> RequestHandle;
    }

    impl TestBuilderExt for DiagnosticsContextBuilder {
        fn start_test_request(
            &mut self,
            execution_context: ExecutionContext,
            region: Option<Region>,
            endpoint: String,
        ) -> RequestHandle {
            self.start_request(
                execution_context,
                PipelineType::DataPlane,
                TransportSecurity::Secure,
                region,
                endpoint,
            )
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
                "https://test.documents.azure.com".to_string(),
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
                "https://test.documents.azure.com".to_string(),
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
                "https://test.documents.azure.com".to_string(),
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
                "https://test.documents.azure.com".to_string(),
            );
            builder.update_request(h1, |req| req.request_charge = RequestCharge::new(3.0));

            let h2 = builder.start_test_request(
                ExecutionContext::Retry,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com".to_string(),
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
                "https://test.westus2.documents.azure.com".to_string(),
            );
            builder.start_test_request(
                ExecutionContext::Retry,
                Some(Region::WEST_US_2),
                "https://test.westus2.documents.azure.com".to_string(),
            );
            builder.start_test_request(
                ExecutionContext::RegionFailover,
                Some(Region::EAST_US_2),
                "https://test.eastus2.documents.azure.com".to_string(),
            );
        });

        let regions = ctx.regions_contacted();
        assert_eq!(regions.len(), 2);
    }

    #[test]
    fn to_json_detailed() {
        let ctx = make_context_with(ActivityId::from_string("test-id".to_string()), |builder| {
            let handle = builder.start_test_request(
                ExecutionContext::Initial,
                Some(Region::WEST_US_2),
                "https://test.documents.azure.com".to_string(),
            );
            builder.update_request(handle, |req| req.request_charge = RequestCharge::new(1.0));
            builder.complete_request(handle, StatusCode::Ok, None);
        });

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        assert!(json.contains("test-id"));
        assert!(json.contains("westus2")); // Region serializes to normalized name
    }

    #[test]
    fn to_json_summary() {
        let ctx = make_context_with(ActivityId::from_string("test-id".to_string()), |builder| {
            // Add several requests to trigger deduplication
            for i in 0..5 {
                let handle = builder.start_test_request(
                    ExecutionContext::Retry,
                    Some(Region::WEST_US_2),
                    "https://test.documents.azure.com".to_string(),
                );
                builder.update_request(handle, |req| {
                    req.request_charge = RequestCharge::new(i as f64)
                });
                builder.complete_request(handle, StatusCode::TooManyRequests, None);
            }
        });

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Summary));
        assert!(json.contains("test-id"));
        assert!(json.contains("request_count"));
    }

    #[test]
    fn json_caching_detailed() {
        let ctx = make_context_with(
            ActivityId::from_string("cache-test".to_string()),
            |builder| {
                let handle = builder.start_test_request(
                    ExecutionContext::Initial,
                    Some(Region::WEST_US_2),
                    "https://test.documents.azure.com".to_string(),
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
                "https://test.documents.azure.com".to_string(),
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
                "https://test.documents.azure.com".to_string(),
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
            "https://test.documents.azure.com".to_string(),
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
            "https://test.documents.azure.com".to_string(),
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
        assert_eq!(ExecutionContext::Retry.to_string(), "retry");
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
    fn transport_security_default() {
        assert_eq!(TransportSecurity::default(), TransportSecurity::Secure);
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
        // When no cpu_monitor or machine_id is set, the JSON should not contain those keys.
        let ctx = make_context_with(ActivityId::new_uuid(), |builder| {
            builder.set_operation_status(StatusCode::Ok, None);
        });
        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        assert!(!json.contains("system_usage"), "Expected no system_usage when monitor is not set");
        assert!(!json.contains("machine_id"), "Expected no machine_id when not set");
    }

    #[test]
    fn json_with_machine_id() {
        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        builder.set_operation_status(StatusCode::Ok, None);
        builder.set_machine_id(Arc::new("vmId_test-vm-123".to_string()));
        let ctx = builder.complete();

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        assert!(
            json.contains("\"machine_id\":\"vmId_test-vm-123\""),
            "Expected machine_id in JSON, got: {json}"
        );

        // Also in summary mode
        let json_summary = ctx.to_json_string(Some(DiagnosticsVerbosity::Summary));
        assert!(
            json_summary.contains("\"machine_id\":\"vmId_test-vm-123\""),
            "Expected machine_id in summary JSON, got: {json_summary}"
        );
    }

    #[test]
    fn json_with_system_usage() {
        use crate::system::CpuMemoryMonitor;

        let mut builder = DiagnosticsContextBuilder::new(ActivityId::new_uuid(), make_options());
        builder.set_operation_status(StatusCode::Ok, None);
        builder.set_cpu_monitor(CpuMemoryMonitor::get_or_init(Duration::from_secs(5)));
        let ctx = builder.complete();

        let json = ctx.to_json_string(Some(DiagnosticsVerbosity::Detailed));
        assert!(
            json.contains("\"system_usage\""),
            "Expected system_usage in JSON, got: {json}"
        );
        assert!(
            json.contains("\"processor_count\""),
            "Expected processor_count in system_usage, got: {json}"
        );
        assert!(
            json.contains("\"cpu\""),
            "Expected cpu field in system_usage, got: {json}"
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
}
