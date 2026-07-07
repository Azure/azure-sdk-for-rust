// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ECS-style state component types for the pipeline.
//!
//! Data-only state carried between pipeline stages.

use std::sync::{atomic::AtomicBool, Arc};
use std::time::{Duration, Instant};

use azure_core::http::{headers::Headers, Method};
use url::Url;

#[cfg(feature = "preview_dtx")]
use crate::models::ResourceType;
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

/// Maximum retries for backend-driven topology signals; about two minutes at 1s spacing.
pub const MAX_BACKEND_FAILOVER_RETRIES: u32 = 120;

/// Delay between backend-failover attempts; matches Java/Python's 1000ms default.
pub const BACKEND_FAILOVER_RETRY_INTERVAL: std::time::Duration =
    std::time::Duration::from_millis(1000);

/// Maximum inner retries for bodyless DTX coordinator envelope failures.
#[cfg(feature = "preview_dtx")]
pub const MAX_DTX_COORDINATOR_RETRIES: u32 = 10;

/// Maximum inner retries for bodyless DTX infrastructure envelope failures.
#[cfg(feature = "preview_dtx")]
pub const MAX_DTX_INFRA_RETRIES: u32 = 9;

/// Default bodyless DTX coordinator retry delay when the service sends no retry hint.
#[cfg(feature = "preview_dtx")]
pub const DTX_COORDINATOR_RETRY_INTERVAL: Duration = Duration::from_secs(1);

/// Base bodyless DTX infrastructure retry delay.
#[cfg(feature = "preview_dtx")]
pub const DTX_INFRA_BASE_BACKOFF: Duration = Duration::from_millis(100);

/// Maximum bodyless DTX infrastructure retry delay.
#[cfg(feature = "preview_dtx")]
pub const DTX_INFRA_MAX_BACKOFF: Duration = Duration::from_secs(5);

/// Maximum exponent for bodyless DTX infrastructure retry backoff.
#[cfg(feature = "preview_dtx")]
pub const DTX_INFRA_MAX_EXPONENT: u32 = 6;

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
    /// Invariant: only [`Self::advance_session_retry`] increments this; the 404/1002 latch reads `== 0`.
    pub session_token_retry_count: u32,
    /// Multi-write backend-failover counter, separate from generic failover retries.
    pub backend_failover_retry_count: u32,
    /// Bodyless DTX coordinator retry counter.
    #[cfg(feature = "preview_dtx")]
    pub dtx_coordinator_retry_count: u32,
    /// Bodyless DTX infrastructure retry counter.
    #[cfg(feature = "preview_dtx")]
    pub dtx_infra_retry_count: u32,
    /// Maximum failover retries.
    pub max_failover_retries: u32,
    /// Maximum retries for backend-driven topology signals; not customer-tunable.
    pub max_backend_failover_retries: u32,
    /// Maximum session retries.
    pub max_session_retries: u32,
    /// Whether multiple write locations can be used.
    pub can_use_multiple_write_locations: bool,
    /// Whether this operation is on the data-plane pipeline (vs metadata).
    ///
    /// Load-bearing: set via `PipelineType::is_data_plane()` so non-exhaustive variants stay scoped out.
    pub is_dataplane: bool,
    /// Hub-region-processing-only latch.
    ///
    /// Sticky per operation after the first data-plane 404/1002 single-master retry.
    pub hub_region_processing_only: bool,
    /// Cross-hedge shared hub-region-processing-only latch.
    ///
    /// `Some(_)` only inside a post-threshold cross-region hedge race.
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
    /// Allows non-idempotent write retries for single-master PPAF write-region discovery.
    pub ppaf_write_retry_allowed: bool,
    /// Whether the per-partition circuit breaker is active for this account.
    ///
    /// Suppresses endpoint marks because PPCB failover is partition-threshold driven.
    pub ppcb_active: bool,
    /// Write-path location effects deferred until the write definitively
    /// reaches a region.
    ///
    /// Single-master writes defer unconfirmed marks until the write reaches a region.
    /// Multi-master writes and reads apply marks immediately so PPCB sees every failure.
    pub pending_write_effects: Vec<LocationEffect>,
    /// Whether a cross-region hedge race has already been dispatched for
    /// this operation.
    ///
    /// Sticky within a single operation. Set when the operation pipeline
    /// dispatches `execute_hedged` (either from STAGE 2b on the first
    /// attempt, or from STAGE 7 after upgrading a `FailoverRetry` /
    /// `SessionRetry` action). Read by STAGE 5b to suppress a second
    /// hedge upgrade on the post-`BothTransient` fallback path — each
    /// race already burned two regions of RU, and re-upgrading on the
    /// next iteration would burn another two without giving the
    /// surrounding failover loop a chance to make sequential progress.
    pub hedge_already_fired: bool,
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
            backend_failover_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_coordinator_retry_count: 0,
            #[cfg(feature = "preview_dtx")]
            dtx_infra_retry_count: 0,
            max_failover_retries,
            max_backend_failover_retries: MAX_BACKEND_FAILOVER_RETRIES,
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
            hedge_already_fired: false,
        }
    }

    /// Whether failover retry budget allows another attempt.
    pub fn can_retry_failover(&self) -> bool {
        self.failover_retry_count < self.max_failover_retries
    }

    /// Whether multi-write backend-failover budget allows another 403/3 or 403/1008 retry.
    pub fn can_retry_backend_failover(&self) -> bool {
        self.backend_failover_retry_count < self.max_backend_failover_retries
    }

    /// Whether the bodyless DTX coordinator retry budget allows another attempt.
    #[cfg(feature = "preview_dtx")]
    pub fn can_retry_dtx_coordinator(&self) -> bool {
        self.dtx_coordinator_retry_count < MAX_DTX_COORDINATOR_RETRIES
    }

    /// Whether the bodyless DTX infrastructure retry budget allows another attempt.
    #[cfg(feature = "preview_dtx")]
    pub fn can_retry_dtx_infra(&self) -> bool {
        self.dtx_infra_retry_count < MAX_DTX_INFRA_RETRIES
    }

    /// Attaches the cross-hedge shared hub-region-processing-only
    /// latch. Called by
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

    /// Advances the dedicated backend-failover counter without touching generic failover retries.
    pub fn advance_backend_failover(self) -> Self {
        Self {
            backend_failover_retry_count: self.backend_failover_retry_count + 1,
            session_retry_routing: SessionRetryRouting::PreferredEndpoints,
            ..self
        }
    }

    /// Advances the bodyless DTX coordinator retry counter.
    #[cfg(feature = "preview_dtx")]
    pub fn advance_dtx_coordinator_retry(self) -> Self {
        Self {
            dtx_coordinator_retry_count: self.dtx_coordinator_retry_count + 1,
            ..self
        }
    }

    /// Advances the bodyless DTX infrastructure retry counter.
    #[cfg(feature = "preview_dtx")]
    pub fn advance_dtx_infra_retry(self) -> Self {
        Self {
            dtx_infra_retry_count: self.dtx_infra_retry_count + 1,
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
    /// Type of resource targeted by the operation.
    #[cfg(feature = "preview_dtx")]
    pub resource_type: ResourceType,
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
pub(crate) const DEFAULT_MAX_THROTTLE_ATTEMPTS: u32 = 9;
pub(crate) const DEFAULT_MAX_THROTTLE_WAIT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_PER_RETRY_DELAY: Duration = Duration::from_secs(5);
const DEFAULT_FALLBACK_BASE_DELAY: Duration = Duration::from_millis(5);
const DEFAULT_BACKOFF_FACTOR: f64 = 2.0;
const DEFAULT_BACKOFF_JITTER_RATIO: f64 = 0.25;

impl ThrottleRetryState {
    /// Creates a new throttle retry state with default parameters.
    #[cfg(test)]
    pub fn new() -> Self {
        Self::with_limits(DEFAULT_MAX_THROTTLE_ATTEMPTS, DEFAULT_MAX_THROTTLE_WAIT)
    }

    /// Creates a new throttle retry state with caller-supplied limits for the
    /// maximum number of 429 retries (`max_attempts`) and the cumulative wait
    /// budget (`max_wait_time`). All other backoff parameters keep their
    /// defaults.
    ///
    /// `max_attempts == 0` disables throttle retries: the first 429 is
    /// propagated to the caller.
    pub fn with_limits(max_attempts: u32, max_wait_time: Duration) -> Self {
        Self {
            attempt_count: 0,
            max_attempts,
            cumulative_delay: Duration::ZERO,
            max_wait_time,
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
        /// Parsed Cosmos-specific response headers.
        cosmos_headers: CosmosResponseHeaders,
        body: Vec<u8>,
        request_sent: RequestSentStatus,
    },
    /// Transport/connection error (no HTTP response received).
    TransportError {
        status: CosmosStatus,
        error: crate::error::CosmosError,
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
                status,
                cosmos_headers,
                ..
            } => f
                .debug_struct("HttpError")
                .field("status", status)
                .field("cosmos_headers", &cosmos_headers)
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
    /// Retry a bodyless DTX envelope failure with DTX-specific budget.
    #[cfg(feature = "preview_dtx")]
    DtxRetry {
        new_state: OperationRetryState,
        delay: Duration,
    },
    /// Race the primary attempt against a single secondary cross-region hedge.
    ///
    /// Emitted in two places:
    ///
    /// * On the **first** transport attempt, when the eligibility check
    ///   (`should_hedge` + threshold strategy + ≥ 2 preferred read
    ///   endpoints) holds. The primary is launched immediately and races
    ///   the threshold timer; if it elapses, the secondary is spawned.
    /// * On a **retry upgrade**, when `evaluate_transport_result` would
    ///   otherwise return `FailoverRetry` / `SessionRetry` and the
    ///   operation is still hedge-eligible.
    ///
    /// The pipeline dispatches this variant to `execute_hedged()`, which
    /// races the primary against the `secondary_routing` after
    /// `threshold` elapses. `strategy_config` rides along as the snapshot
    /// for the winning response's `HedgeDiagnostics`.
    ///
    /// `new_state` carries the post-transition `OperationRetryState`
    /// forwarded from `FailoverRetry` / `SessionRetry` on the
    /// retry-upgrade path; STAGE 7 applies it via `advance_to_next_attempt`
    /// before building `AttemptContext` so the race inherits the failover
    /// exclusions, hub-region latch, and advanced retry counters.
    /// First-attempt (STAGE 2b) callers pass the live `retry_state` so
    /// the variant stays uniform.
    Hedge {
        secondary_routing: RoutingDecision,
        threshold: HedgeThreshold,
        strategy_config: HedgingStrategyConfig,
        new_state: OperationRetryState,
    },
    /// Abort the operation with this error.
    ///
    /// The typed `CosmosStatus` is always available via `error.status()`;
    /// callers that need the status for routing decisions (e.g.
    /// flush-on-confirming-status) read it from there.
    Abort { error: crate::error::CosmosError },
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
    fn throttle_retry_state_with_limits_overrides_attempts_and_wait() {
        let state = ThrottleRetryState::with_limits(3, Duration::from_secs(7));
        assert_eq!(state.attempt_count, 0);
        assert_eq!(state.max_attempts, 3);
        assert_eq!(state.max_wait_time, Duration::from_secs(7));
        // Backoff parameters keep their defaults.
        assert_eq!(state.fallback_base_delay, Duration::from_millis(5));
        assert_eq!(state.max_per_retry_delay, Duration::from_secs(5));
        assert_eq!(state.backoff_jitter_ratio, 0.25);
    }

    #[test]
    fn throttle_retry_state_new_matches_with_limits_defaults() {
        let from_new = ThrottleRetryState::new();
        let from_limits = ThrottleRetryState::with_limits(
            DEFAULT_MAX_THROTTLE_ATTEMPTS,
            DEFAULT_MAX_THROTTLE_WAIT,
        );
        assert_eq!(from_new.max_attempts, from_limits.max_attempts);
        assert_eq!(from_new.max_wait_time, from_limits.max_wait_time);
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
