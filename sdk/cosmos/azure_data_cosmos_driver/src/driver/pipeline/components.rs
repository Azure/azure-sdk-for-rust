// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! ECS-style state component types for the pipeline.
//!
//! Following the DOP principle of separating data from behavior, these types
//! represent the state that flows through pipeline stages. Each pipeline stage
//! operates on only the components it needs.

use std::time::{Duration, Instant};

use azure_core::http::{headers::Headers, Method};
use url::Url;

use crate::{
    diagnostics::{ExecutionContext, RequestSentStatus},
    driver::{
        jitter::with_jitter,
        routing::{CosmosEndpoint, LocationIndex},
        transport::AuthorizationContext,
    },
    models::{CosmosResponseHeaders, CosmosStatus},
    options::Region,
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
    pub session_token_retry_count: u32,
    /// Maximum failover retries.
    pub max_failover_retries: u32,
    /// Maximum session retries.
    pub max_session_retries: u32,
    /// Whether multiple write locations can be used.
    pub can_use_multiple_write_locations: bool,
    /// Regions excluded for this operation.
    pub excluded_regions: Vec<Region>,
    /// Session-retry routing override for read operations.
    pub session_retry_routing: SessionRetryRouting,
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
            excluded_regions,
            session_retry_routing: SessionRetryRouting::PreferredEndpoints,
        }
    }

    /// Whether failover retry budget allows another attempt.
    pub fn can_retry_failover(&self) -> bool {
        self.failover_retry_count < self.max_failover_retries
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
            TransportOutcome::Success { status, .. } => f
                .debug_struct("Success")
                .field("status", status)
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
