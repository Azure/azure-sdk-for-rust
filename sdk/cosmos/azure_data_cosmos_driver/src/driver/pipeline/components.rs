// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ECS-style state component types for the pipeline.
//!
//! Following the DOP principle of separating data from behavior, these types
//! represent the state that flows through pipeline stages. Each pipeline stage
//! operates on only the components it needs.

use std::sync::{atomic::AtomicBool, Arc};
use std::time::{Duration, Instant};

use azure_core::http::{headers::Headers, Method};
use url::Url;

use crate::{
    diagnostics::{ExecutionContext, RequestSentStatus},
    driver::{
        jitter::with_jitter,
        pipeline::hedging_diagnostics::HedgingStrategyConfig,
        routing::{
            partition_key_range_id::PartitionKeyRangeId, CosmosEndpoint, LocationEffect,
            LocationIndex,
        },
        transport::AuthorizationContext,
    },
    models::{CosmosResponseHeaders, CosmosStatus},
    options::{HedgeThreshold, Region},
};

// ── Operation-Level Components ─────────────────────────────────────────

/// Transport mode for a routed attempt.
///
/// Determines which HTTP transport (and protocol version) is used
/// for a given request attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransportMode {
    /// Standard gateway (HTTP/1.1 or HTTP/2 via ALPN).
    Gateway,
    /// Gateway 2.0 (HTTP/2 with prior knowledge).
    Gateway20,
}

/// Routing decision for the current attempt.
///
/// Wraps a `CosmosEndpoint` that carries the resolved URL and optional
/// region for diagnostics and routing purposes.
#[derive(Clone, Debug)]
pub(crate) struct RoutingDecision {
    /// The resolved endpoint for this attempt.
    pub endpoint: CosmosEndpoint,
    /// The concrete URL selected for this attempt.
    pub selected_url: Url,
    /// The transport mode for this attempt.
    pub transport_mode: TransportMode,
}

impl std::fmt::Display for RoutingDecision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(region) = self.endpoint.region() {
            write!(f, "{}({})", region, self.selected_url)
        } else {
            write!(f, "{}", self.selected_url)
        }
    }
}

/// Operation-level retry state.
///
/// Tracks failover retry count, session retry count, location index,
/// and write-region topology for the multi-region operation loop.
#[derive(Clone, Debug)]
pub(crate) struct OperationRetryState {
    /// Type-safe index into preferred endpoint lists.
    pub location: LocationIndex,
    /// Number of operation-level failover retries attempted.
    pub failover_retry_count: u32,
    /// Number of session-token retries attempted.
    ///
    /// **Invariant:** This counter is incremented only by
    /// [`Self::advance_session_retry`] on the 404/1002 path. The
    /// hub-region-processing-only latch trigger in
    /// `retry_evaluation::try_handle_read_session_not_available` reads
    /// `== 0` to detect the first 1002 within an operation; if a future
    /// refactor adds another increment site, that trigger gate must be
    /// re-validated.
    pub session_token_retry_count: u32,
    /// Maximum failover retries.
    pub max_failover_retries: u32,
    /// Maximum session retries.
    pub max_session_retries: u32,
    /// Whether multiple write locations can be used.
    pub can_use_multiple_write_locations: bool,
    /// Whether this operation is on the data-plane pipeline (vs metadata).
    ///
    /// Set once at the production call site in `execute_operation_pipeline`
    /// from `pipeline_type.is_data_plane()`. Used to gate the
    /// hub-region-processing-only latch so metadata-pipeline operations
    /// (which ride the same `execute_operation_pipeline` but are scoped out
    /// of the spec per HUB_REGION_PROCESSING_HEADER_SPEC.md §1.5) never
    /// emit the header.
    ///
    /// LOAD-BEARING for the metadata-pipeline scope gate (AC-8).
    /// The production call site MUST use the
    /// `PipelineType::is_data_plane()` accessor — NOT `==` matching —
    /// because `PipelineType` is `#[non_exhaustive]` and a future variant
    /// would silently bypass an equality gate.
    pub is_dataplane: bool,
    /// Hub-region-processing-only latch.
    ///
    /// Sticky within a single operation. Set on the retry triggered by the
    /// FIRST `404 / 1002 (READ_SESSION_NOT_AVAILABLE)` on a single-master
    /// data-plane account; once set, every subsequent transport attempt
    /// for this operation emits the
    /// `x-ms-cosmos-hub-region-processing-only: True` header.
    ///
    /// Bounded by operation lifetime — `OperationRetryState` is
    /// constructed fresh per call to `execute_operation_pipeline` (single
    /// production call site), so the latch never leaks across operations.
    ///
    /// LOAD-BEARING for SE-003 mitigation — see
    /// HUB_REGION_PROCESSING_HEADER_SPEC.md AG-1..AG-4. If a future
    /// refactor adds a second production construction site for
    /// `OperationRetryState`, the SE-003 mitigation argument needs to be
    /// re-validated.
    ///
    /// **Cross-region hedging coordination.** The orchestrator added in
    /// `azure_data_cosmos_driver/docs/HEDGING_SPEC.md` §9.5 constructs an
    /// `OperationRetryState` *per hedge*, so this per-state latch is
    /// per-hedge by default. The hedging spec requires augmenting this
    /// state with a `shared_hub_region_latch: Option<Arc<AtomicBool>>`
    /// that is `Some` only when running inside `execute_with_hedging()`,
    /// is CAS-set alongside this field in `build_session_retry_state`,
    /// and is OR'd into the emission decision in `apply_hub_region_header`.
    /// This mirrors .NET v3's `CrossRegionAvailabilityContext` shared
    /// object introduced by azure-cosmos-dotnet-v3#5815. Any change to
    /// the latch trigger or emission rule here MUST update both call
    /// sites and §9.5 of the hedging spec.
    pub hub_region_processing_only: bool,
    /// Cross-hedge shared hub-region-processing-only latch.
    ///
    /// `Some(_)` only when this `OperationRetryState` is running inside
    /// the [`execute_hedged`] cross-region race past the threshold
    /// (spec `docs/HEDGING_SPEC.md` §9.6). `None` on the non-hedged
    /// pipeline and on the zero-overhead happy path where the primary
    /// returns before the threshold elapses, so the §6.5 #3
    /// no-Arc-allocation invariant and PR #4389's allocator footprint
    /// are both preserved.
    ///
    /// Set with `Release` ordering by `build_session_retry_state` at
    /// the same point where it flips the per-state `hub_region_processing_only`
    /// latch; read with `Acquire` ordering by `apply_hub_region_header`
    /// (OR'd against the per-state latch). Mirrors .NET v3's
    /// `CrossRegionAvailabilityContext` injected into
    /// `RequestMessage.Properties` before the hedge fan-out
    /// ([azure-cosmos-dotnet-v3#5815](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5815)).
    pub shared_hub_region_latch: Option<Arc<AtomicBool>>,
    /// Regions excluded for this operation.
    pub excluded_regions: Vec<Region>,
    /// Session-retry routing override for read operations.
    pub session_retry_routing: SessionRetryRouting,
    /// Partition key range ID resolved from the first response headers.
    /// `None` until the first transport attempt returns headers.
    pub partition_key_range_id: Option<PartitionKeyRangeId>,
    /// Whether PPAF allows non-idempotent write retries on failover.
    ///
    /// When `true`, a non-idempotent write that receives a 503/429/410/408
    /// (or transport error) can be retried to a different region for write
    /// region discovery. Precomputed from partition-level automatic failover
    /// being enabled on a single-master account.
    pub ppaf_write_retry_allowed: bool,
    /// Whether the per-partition circuit breaker is active for this account.
    ///
    /// When `true`, endpoint-level `MarkEndpointUnavailable` effects are
    /// suppressed for PPCB-eligible requests (reads, or writes on
    /// multi-master). Failover is driven by the partition-level failure
    /// threshold instead of marking the entire endpoint unavailable.
    pub ppcb_active: bool,
    /// Write-path location effects deferred until the write definitively
    /// reaches a region.
    ///
    /// On the **single-master** write path we cannot tell from a single
    /// failed response (503, 429/3092, 410, 408, 403/3, transport error)
    /// whether the failure was a real per-region outage or a transient blip
    /// we'll never see again. Applying `MarkPartitionUnavailable` (and, for
    /// PPAF, `MarkEndpointUnavailable`) immediately on every such failure
    /// pollutes the routing state with unverified failures and makes
    /// failover behave non-deterministically across retries.
    ///
    /// **Multi-master** writes do NOT defer: the per-partition circuit
    /// breaker is the source of truth for failover, and it must observe
    /// every failure as it happens (otherwise the breaker can never trip
    /// for non-idempotent writes that abort).
    ///
    /// Deferred effects are flushed only when the write definitively
    /// reaches a region — either `OperationAction::Complete` (HTTP 2xx) or
    /// `OperationAction::Abort` with a region-confirming status such as 409
    /// Conflict or 412 Precondition Failed (statuses that prove the server
    /// processed the request). On any other abort path the buffer is
    /// discarded.
    ///
    /// **What gets deferred** is decided by `partition_effects_for_deferral`:
    /// - For single-master writes: `MarkPartitionUnavailable` (per-partition
    ///   state should never be polluted by unverified retries).
    /// - Additionally for PPAF on single-master writes
    ///   (`ppaf_write_retry_allowed`): `MarkEndpointUnavailable` is also
    ///   deferred so a transient retry doesn't darken the only write region.
    ///
    /// Read-path and multi-master write effects are NOT deferred — PPCB
    /// counters drive threshold-based failover and need the failure signal
    /// immediately.
    pub pending_write_effects: Vec<LocationEffect>,
}

/// How a session retry should resolve endpoints for a read operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SessionRetryRouting {
    /// Continue using the normal preferred endpoint list for the operation kind.
    PreferredEndpoints,
    /// Route a read session retry through the preferred write endpoint list.
    PreferredWriteEndpoints,
}

impl OperationRetryState {
    /// Creates the initial retry state for an operation.
    pub fn initial(
        generation: u64,
        can_use_multiple_write_locations: bool,
        excluded_regions: Vec<Region>,
        max_failover_retries: u32,
        max_session_retries: u32,
    ) -> Self {
        Self {
            location: LocationIndex::initial(generation),
            failover_retry_count: 0,
            session_token_retry_count: 0,
            max_failover_retries,
            max_session_retries,
            can_use_multiple_write_locations,
            is_dataplane: false,
            hub_region_processing_only: false,
            shared_hub_region_latch: None,
            excluded_regions,
            session_retry_routing: SessionRetryRouting::PreferredEndpoints,
            partition_key_range_id: None,
            ppaf_write_retry_allowed: false,
            ppcb_active: false,
            pending_write_effects: Vec::new(),
        }
    }

    /// Whether failover retry budget allows another attempt.
    pub fn can_retry_failover(&self) -> bool {
        self.failover_retry_count < self.max_failover_retries
    }

    /// Attaches the cross-hedge shared hub-region-processing-only
    /// latch (spec `docs/HEDGING_SPEC.md` §9.6). Called by
    /// `execute_hedged` after the threshold elapses and the
    /// eligibility predicate (data-plane ∧ single-master) holds, so the
    /// zero-overhead happy path never constructs an `Arc<AtomicBool>`.
    ///
    /// Currently exercised by Part 5 unit tests only — the production
    /// hedge path constructs the `Arc` directly inside `execute_hedged`
    /// without an intermediate `OperationRetryState` because each hedge
    /// today runs a single transport attempt (no per-hedge retry loop).
    /// When §8.4 lands per-hedge operation-level retry, the secondary's
    /// `OperationRetryState` constructor will switch to this builder.
    #[allow(dead_code)] // reserved for §8.4 per-hedge retry state
    pub fn with_shared_hub_region_latch(mut self, latch: Arc<AtomicBool>) -> Self {
        self.shared_hub_region_latch = Some(latch);
        self
    }

    /// Whether session retry budget allows another attempt.
    pub fn can_retry_session(&self) -> bool {
        self.session_token_retry_count < self.max_session_retries
    }

    /// Returns a new state advanced for failover retry.
    pub fn advance_failover(self) -> Self {
        Self {
            failover_retry_count: self.failover_retry_count + 1,
            session_retry_routing: SessionRetryRouting::PreferredEndpoints,
            ..self
        }
    }

    /// Returns a new state advanced for session retry.
    pub fn advance_session_retry(self) -> Self {
        Self {
            session_token_retry_count: self.session_token_retry_count + 1,
            session_retry_routing: if self.can_use_multiple_write_locations {
                SessionRetryRouting::PreferredEndpoints
            } else {
                SessionRetryRouting::PreferredWriteEndpoints
            },
            ..self
        }
    }

    /// Advances the location index for endpoint list of `list_len`.
    pub fn advance_location(self, list_len: usize, generation: u64) -> Self {
        Self {
            location: self.location.next_for_generation(list_len, generation),
            ..self
        }
    }

    /// Returns true when a read retry should use the write-endpoint list.
    pub fn route_reads_to_write_endpoints(&self) -> bool {
        matches!(
            self.session_retry_routing,
            SessionRetryRouting::PreferredWriteEndpoints
        )
    }
}

// ── Transport-Level Components ─────────────────────────────────────────

/// A single transport attempt's identity and config.
///
/// Produced by the operation pipeline for each attempt, consumed by the
/// transport pipeline. Contains everything needed to build, sign, and
/// send a single HTTP request.
#[derive(Debug)]
pub(crate) struct TransportRequest {
    /// The HTTP method.
    pub method: Method,
    /// The endpoint selected for this attempt.
    pub endpoint: CosmosEndpoint,
    /// The fully resolved URL for this attempt.
    pub url: Url,
    /// Headers to send (includes operation-specific and attempt-specific headers).
    pub headers: Headers,
    /// Request body bytes (schema-agnostic).
    pub body: Option<azure_core::Bytes>,
    /// Authorization context for signing.
    pub auth_context: AuthorizationContext,
    /// The execution context (Initial/Retry/Hedging/Failover).
    pub execution_context: ExecutionContext,
    /// End-to-end deadline for the overall operation.
    pub deadline: Option<Instant>,
}

/// Transport-level retry state for 429 throttling.
///
/// Scoped to a single transport pipeline invocation.
#[derive(Clone, Debug)]
pub(crate) struct ThrottleRetryState {
    /// How many 429 retries have been attempted.
    pub attempt_count: u32,
    /// Maximum number of 429 retries allowed.
    pub max_attempts: u32,
    /// Total delay accumulated across all retries.
    pub cumulative_delay: Duration,
    /// Maximum total wait time before giving up.
    pub max_wait_time: Duration,
    /// Maximum delay for a single retry (caps the service-specified value).
    pub max_per_retry_delay: Duration,
    /// Base delay used when the service does not specify `x-ms-retry-after-ms`.
    pub fallback_base_delay: Duration,
    /// Multiplicative backoff factor for the fallback delay.
    pub backoff_factor: f64,
    /// Jitter ratio applied to fallback backoff (for example 0.25 = +/- 25%).
    pub backoff_jitter_ratio: f64,
    /// Whether the one-time forced final throttle retry has been used.
    pub forced_final_retry_used: bool,
}

/// Hard-coded defaults for throttle retry.
const DEFAULT_MAX_THROTTLE_ATTEMPTS: u32 = 9;
const DEFAULT_MAX_THROTTLE_WAIT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_PER_RETRY_DELAY: Duration = Duration::from_secs(5);
const DEFAULT_FALLBACK_BASE_DELAY: Duration = Duration::from_millis(5);
const DEFAULT_BACKOFF_FACTOR: f64 = 2.0;
const DEFAULT_BACKOFF_JITTER_RATIO: f64 = 0.25;

impl ThrottleRetryState {
    /// Creates a new throttle retry state with default parameters.
    pub fn new() -> Self {
        Self {
            attempt_count: 0,
            max_attempts: DEFAULT_MAX_THROTTLE_ATTEMPTS,
            cumulative_delay: Duration::ZERO,
            max_wait_time: DEFAULT_MAX_THROTTLE_WAIT,
            max_per_retry_delay: DEFAULT_MAX_PER_RETRY_DELAY,
            fallback_base_delay: DEFAULT_FALLBACK_BASE_DELAY,
            backoff_factor: DEFAULT_BACKOFF_FACTOR,
            backoff_jitter_ratio: DEFAULT_BACKOFF_JITTER_RATIO,
            forced_final_retry_used: false,
        }
    }

    /// Returns true when a one-time forced final retry can still be attempted.
    pub fn can_use_forced_final_retry(&self) -> bool {
        !self.forced_final_retry_used
    }

    /// Marks the one-time forced final retry as consumed.
    pub fn mark_forced_final_retry_used(&self) -> Self {
        Self {
            forced_final_retry_used: true,
            ..self.clone()
        }
    }

    /// Computes the pure exponential fallback delay without jitter.
    fn fallback_exponential_delay(&self) -> Duration {
        let multiplier = self.backoff_factor.powi(self.attempt_count as i32);
        self.fallback_base_delay.mul_f64(multiplier)
    }

    /// Computes the fallback delay (used when the service does not send
    /// `x-ms-retry-after-ms`). Uses exponential backoff from a small base and
    /// applies jitter to reduce synchronized retry waves.
    pub fn fallback_delay(&self) -> Duration {
        let base_delay = self.fallback_exponential_delay();

        // Apply jitter only to the already-computed exponential delay.
        let ratio = self.backoff_jitter_ratio.clamp(0.0, 1.0);
        if ratio == 0.0 || base_delay.is_zero() {
            return base_delay;
        }

        Duration::from_secs_f64(with_jitter(base_delay.as_secs_f64(), ratio))
    }
}

/// Result of a single transport attempt.
///
/// Returned from the transport pipeline to the operation pipeline.
#[derive(Debug)]
pub(crate) struct TransportResult {
    /// The outcome of this attempt.
    pub outcome: TransportOutcome,
}

impl TransportResult {
    /// Creates a timeout result when the end-to-end operation deadline is exceeded.
    pub fn deadline_exceeded(request_sent: RequestSentStatus) -> Self {
        Self {
            outcome: TransportOutcome::DeadlineExceeded { request_sent },
        }
    }

    /// Creates a result from an HTTP response payload.
    ///
    /// Successful status codes are mapped to `Success`; non-success status codes
    /// are mapped to `HttpError` with `request_sent` set to `Sent`.
    pub fn from_http_response(
        status: CosmosStatus,
        headers: Headers,
        cosmos_headers: CosmosResponseHeaders,
        body: Vec<u8>,
    ) -> Self {
        if status.is_success() {
            Self {
                outcome: TransportOutcome::Success {
                    status,
                    cosmos_headers,
                    body,
                },
            }
        } else {
            Self {
                outcome: TransportOutcome::HttpError {
                    status,
                    headers,
                    cosmos_headers,
                    body,
                    request_sent: RequestSentStatus::Sent,
                },
            }
        }
    }

    /// Returns the parsed Cosmos response headers if this is an HTTP response.
    pub fn cosmos_headers(&self) -> Option<&CosmosResponseHeaders> {
        match &self.outcome {
            TransportOutcome::Success { cosmos_headers, .. } => Some(cosmos_headers),
            TransportOutcome::HttpError { cosmos_headers, .. } => Some(cosmos_headers),
            TransportOutcome::TransportError { .. } | TransportOutcome::DeadlineExceeded { .. } => {
                None
            }
        }
    }

    /// Returns the raw response headers for HTTP error responses.
    ///
    /// Raw headers are only retained for error responses (needed to build a `RawResponse`
    /// for callers). For success responses, only parsed `CosmosResponseHeaders` are kept.
    pub fn response_headers(&self) -> Option<&Headers> {
        match &self.outcome {
            TransportOutcome::HttpError { headers, .. } => Some(headers),
            _ => None,
        }
    }
}

/// The outcome of a single transport attempt.
pub(crate) enum TransportOutcome {
    /// Successful response (2xx).
    Success {
        status: CosmosStatus,
        /// Parsed Cosmos-specific response headers.
        cosmos_headers: CosmosResponseHeaders,
        body: Vec<u8>,
    },
    /// HTTP error response (non-2xx) that may be retryable at the operation level.
    HttpError {
        status: CosmosStatus,
        /// Raw headers retained for building `RawResponse` in error reporting.
        headers: Headers,
        /// Parsed Cosmos-specific response headers.
        cosmos_headers: CosmosResponseHeaders,
        body: Vec<u8>,
        request_sent: RequestSentStatus,
    },
    /// Transport/connection error (no HTTP response received).
    TransportError {
        status: CosmosStatus,
        error: azure_core::Error,
        request_sent: RequestSentStatus,
    },
    /// End-to-end deadline exceeded while this transport attempt was pending.
    DeadlineExceeded { request_sent: RequestSentStatus },
}

/// Display implementation for logging the high-level outcome in a compact format
impl std::fmt::Display for TransportOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportOutcome::Success { status, .. } => write!(f, "Success({})", status),
            TransportOutcome::HttpError { status, .. } => write!(f, "HttpError({})", status),
            TransportOutcome::TransportError { error, .. } => {
                write!(f, "TransportError({})", error)
            }
            TransportOutcome::DeadlineExceeded { .. } => write!(f, "DeadlineExceeded"),
        }
    }
}

impl std::fmt::Debug for TransportOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportOutcome::Success {
                status,
                cosmos_headers,
                ..
            } => f
                .debug_struct("Success")
                .field("status", status)
                .field("cosmos_headers", &cosmos_headers)
                .field("body", &"...")
                .finish(),
            TransportOutcome::HttpError {
                status, headers, ..
            } => f
                .debug_struct("HttpError")
                .field("status", status)
                .field("headers", headers)
                .field("body", &"...")
                .finish(),
            TransportOutcome::TransportError {
                error,
                request_sent,
                ..
            } => f
                .debug_struct("TransportError")
                .field("error", error)
                .field("request_sent", request_sent)
                .finish(),
            TransportOutcome::DeadlineExceeded { request_sent } => f
                .debug_struct("DeadlineExceeded")
                .field("request_sent", request_sent)
                .finish(),
        }
    }
}

// ── Decision Enums ─────────────────────────────────────────────────────

/// What the operation pipeline should do after a transport attempt.
///
/// Slim for Step 1 — only `Complete`, `TransportRetry`, and `Abort`.
/// Expanded in Step 2 with `FailoverRetry`, `SessionRetry`, `Hedge`.
#[derive(Debug)]
pub(crate) enum OperationAction {
    /// Return the successful response.
    Complete(Box<TransportResult>),
    /// Retry in another endpoint/region.
    FailoverRetry {
        new_state: OperationRetryState,
        delay: Option<Duration>,
    },
    /// Retry for session consistency.
    SessionRetry { new_state: OperationRetryState },
    /// Race the primary attempt against a single secondary cross-region hedge.
    ///
    /// Emitted by `evaluate_transport_result` per `docs/HEDGING_SPEC.md`
    /// §6.1 when:
    ///
    /// 1. The per-attempt result was *transient* (would otherwise produce
    ///    `FailoverRetry`/`SessionRetry`).
    /// 2. `should_hedge` (§5.1) is `true` and an alternate read endpoint
    ///    can be pinned.
    /// 3. A `HedgingStrategy` resolved via §11.3.1.
    ///
    /// STAGE 7 of the operation pipeline dispatches this to
    /// `execute_hedged()`, which races a fresh primary attempt against the
    /// `secondary_routing` after `threshold` elapses. Both
    /// `secondary_excluded_regions` (the per-hedge `ExcludeRegions` set
    /// computed by `build_secondary_excluded_regions`) and
    /// `strategy_config` (snapshot for the winning response's
    /// `HedgeDiagnostics`, §10.1) ride along.
    ///
    /// **Wired in Part 4b** — the evaluator and `execute_hedged()` now
    /// produce and consume this variant. The fields are read by the
    /// STAGE 7 hedge arm in `operation_pipeline.rs::execute_operation_pipeline`
    /// to construct the race; `secondary_excluded_regions` is reserved for
    /// future in-hedge retry layers (spec §8.4) and is not consumed yet.
    Hedge {
        secondary_routing: RoutingDecision,
        /// Reserved for the in-hedge retry layer (spec §8.4) that will
        /// pin the alternate-region attempt against any region drift.
        /// Not consumed yet — Part 4b uses `secondary_routing` directly
        /// because each hedge fires a single transport attempt with no
        /// operation-level retry on top.
        #[allow(dead_code)]
        secondary_excluded_regions: Vec<Region>,
        threshold: HedgeThreshold,
        strategy_config: HedgingStrategyConfig,
    },
    /// Abort the operation with this error.
    Abort {
        error: azure_core::Error,
        status: Option<CosmosStatus>,
    },
}

/// What the transport pipeline should do after a 429.
#[derive(Debug)]
pub(crate) enum ThrottleAction {
    /// Retry after a delay.
    Retry {
        delay: Duration,
        new_state: ThrottleRetryState,
    },
    /// Do not retry; propagate to the operation pipeline.
    Propagate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throttle_retry_state_defaults() {
        let state = ThrottleRetryState::new();
        assert_eq!(state.attempt_count, 0);
        assert_eq!(state.max_attempts, 9);
        assert_eq!(state.max_wait_time, Duration::from_secs(30));
        assert_eq!(state.fallback_base_delay, Duration::from_millis(5));
        assert_eq!(state.max_per_retry_delay, Duration::from_secs(5));
        assert_eq!(state.backoff_jitter_ratio, 0.25);
        assert!(!state.forced_final_retry_used);
    }

    #[test]
    fn throttle_retry_state_marks_forced_final_retry_as_used() {
        let state = ThrottleRetryState::new();
        assert!(state.can_use_forced_final_retry());

        let updated = state.mark_forced_final_retry_used();
        assert!(!updated.can_use_forced_final_retry());
    }

    #[test]
    fn throttle_retry_fallback_backoff_with_jitter_bounds() {
        let state = ThrottleRetryState::new();
        // attempt 0 base: 5ms; with +/-25% jitter => [3.75ms, 6.25ms]
        let delay = state.fallback_delay();
        assert!(delay >= Duration::from_nanos(3_750_000));
        assert!(delay <= Duration::from_nanos(6_250_000));

        let state = ThrottleRetryState {
            attempt_count: 1,
            ..ThrottleRetryState::new()
        };
        // attempt 1 base: 10ms; with +/-25% jitter => [7.5ms, 12.5ms]
        let delay = state.fallback_delay();
        assert!(delay >= Duration::from_nanos(7_500_000));
        assert!(delay <= Duration::from_nanos(12_500_000));

        let state = ThrottleRetryState {
            attempt_count: 5,
            ..ThrottleRetryState::new()
        };
        // attempt 5 base: 160ms; with +/-25% jitter => [120ms, 200ms]
        let delay = state.fallback_delay();
        assert!(delay >= Duration::from_millis(120));
        assert!(delay <= Duration::from_millis(200));
    }

    #[test]
    fn throttle_retry_fallback_exponential_when_jitter_disabled() {
        let state = ThrottleRetryState {
            backoff_jitter_ratio: 0.0,
            ..ThrottleRetryState::new()
        };
        assert_eq!(state.fallback_delay(), Duration::from_millis(5));

        let state = ThrottleRetryState {
            attempt_count: 1,
            backoff_jitter_ratio: 0.0,
            ..ThrottleRetryState::new()
        };
        assert_eq!(state.fallback_delay(), Duration::from_millis(10));

        let state = ThrottleRetryState {
            attempt_count: 5,
            backoff_jitter_ratio: 0.0,
            ..ThrottleRetryState::new()
        };
        assert_eq!(state.fallback_delay(), Duration::from_millis(160));
    }

    #[test]
    fn operation_retry_state_budget() {
        let state = OperationRetryState::initial(0, false, Vec::new(), 1, 1);
        assert!(state.can_retry_failover());

        let state = state.advance_failover();
        assert!(!state.can_retry_failover());
    }

    #[test]
    fn advance_session_retry_single_write_routes_to_write_endpoints() {
        let state = OperationRetryState::initial(0, false, Vec::new(), 3, 2);
        assert_eq!(
            state.session_retry_routing,
            SessionRetryRouting::PreferredEndpoints
        );

        let state = state.advance_session_retry();
        assert_eq!(state.session_token_retry_count, 1);
        assert_eq!(
            state.session_retry_routing,
            SessionRetryRouting::PreferredWriteEndpoints
        );
    }

    #[test]
    fn advance_session_retry_multi_write_stays_on_preferred_endpoints() {
        let state = OperationRetryState::initial(0, true, Vec::new(), 3, 2);

        let state = state.advance_session_retry();
        assert_eq!(state.session_token_retry_count, 1);
        assert_eq!(
            state.session_retry_routing,
            SessionRetryRouting::PreferredEndpoints
        );
    }
}
