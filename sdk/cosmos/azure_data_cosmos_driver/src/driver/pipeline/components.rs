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
    models::{CosmosResponseHeaders, CosmosStatus},
    options::Region,
};

use super::super::transport::AuthorizationContext;

// ── Operation-Level Components ─────────────────────────────────────────

/// Routing decision for the current attempt.
///
/// In Step 1 this is a simple endpoint + region pair.
/// In Step 2+ this will carry `CosmosEndpoint` and partition overrides.
#[derive(Clone, Debug)]
pub(crate) struct RoutingDecision {
    /// The resolved endpoint URL for this attempt.
    pub endpoint: Url,
    /// The region this endpoint resides in, if known.
    pub region: Option<Region>,
}

/// Operation-level retry state.
///
/// Slim for Step 1 — only tracks transport-level retry count.
/// Expanded in Step 2 with failover, session, and location index fields.
#[derive(Clone, Debug)]
pub(crate) struct OperationRetryState {
    /// How many transport-level retries have been attempted.
    pub transport_retry_count: u32,
    /// Maximum number of transport retries allowed.
    pub max_transport_retries: u32,
}

impl OperationRetryState {
    /// Creates the initial retry state for an operation.
    pub fn initial() -> Self {
        Self {
            transport_retry_count: 0,
            // Step 1: 1 retry (same as current execute_operation behavior)
            max_transport_retries: 1,
        }
    }

    /// Whether the retry budget allows another transport retry.
    pub fn can_retry_transport(&self) -> bool {
        self.transport_retry_count < self.max_transport_retries
    }

    /// Advance to the next transport retry attempt.
    pub fn advance_transport_retry(&self) -> Self {
        Self {
            transport_retry_count: self.transport_retry_count + 1,
            max_transport_retries: self.max_transport_retries,
        }
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
    /// The fully resolved URL for this attempt.
    pub url: Url,
    /// Headers to send (includes operation-specific and attempt-specific headers).
    pub headers: Headers,
    /// Request body bytes (schema-agnostic).
    pub body: Option<Vec<u8>>,
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
}

/// Hard-coded defaults for throttle retry.
const DEFAULT_MAX_THROTTLE_ATTEMPTS: u32 = 9;
const DEFAULT_MAX_THROTTLE_WAIT: Duration = Duration::from_secs(30);
const DEFAULT_MAX_PER_RETRY_DELAY: Duration = Duration::from_secs(5);
const DEFAULT_FALLBACK_BASE_DELAY: Duration = Duration::from_millis(5);
const DEFAULT_BACKOFF_FACTOR: f64 = 2.0;

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
        }
    }

    /// Computes the fallback delay (used when the service does not send
    /// `x-ms-retry-after-ms`). Uses exponential backoff from a small base.
    pub fn fallback_delay(&self) -> Duration {
        let multiplier = self.backoff_factor.powi(self.attempt_count as i32);
        self.fallback_base_delay.mul_f64(multiplier)
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
    /// Whether this result represents a successful response.
    pub fn is_success(&self) -> bool {
        matches!(self.outcome, TransportOutcome::Success { .. })
    }

    /// Returns the `CosmosStatus` if this is an HTTP response (success or error).
    pub fn status(&self) -> Option<CosmosStatus> {
        match &self.outcome {
            TransportOutcome::Success { status, .. } => Some(*status),
            TransportOutcome::HttpError { status, .. } => Some(*status),
            TransportOutcome::TransportError { .. } => None,
        }
    }

    /// Returns the response headers if this is an HTTP response.
    pub fn response_headers(&self) -> Option<&Headers> {
        match &self.outcome {
            TransportOutcome::Success { headers, .. } => Some(headers),
            TransportOutcome::HttpError { headers, .. } => Some(headers),
            TransportOutcome::TransportError { .. } => None,
        }
    }

    /// Extracts `CosmosResponseHeaders` from the HTTP response headers, if available.
    pub fn cosmos_response_headers(&self) -> CosmosResponseHeaders {
        match self.response_headers() {
            Some(headers) => CosmosResponseHeaders::from_headers(headers),
            None => CosmosResponseHeaders::default(),
        }
    }

    /// Returns the request-sent status for retry safety decisions.
    pub fn request_sent_status(&self) -> RequestSentStatus {
        match &self.outcome {
            TransportOutcome::Success { .. } => RequestSentStatus::Sent,
            TransportOutcome::HttpError { request_sent, .. } => *request_sent,
            TransportOutcome::TransportError { request_sent, .. } => *request_sent,
        }
    }
}

/// The outcome of a single transport attempt.
#[derive(Debug)]
pub(crate) enum TransportOutcome {
    /// Successful response (2xx).
    Success {
        status: CosmosStatus,
        headers: Headers,
        body: Vec<u8>,
    },
    /// HTTP error response (non-2xx) that may be retryable at the operation level.
    HttpError {
        status: CosmosStatus,
        headers: Headers,
        body: Vec<u8>,
        request_sent: RequestSentStatus,
    },
    /// Transport/connection error (no HTTP response received).
    TransportError {
        error: azure_core::Error,
        request_sent: RequestSentStatus,
    },
}

// ── Decision Enums ─────────────────────────────────────────────────────

/// What the operation pipeline should do after a transport attempt.
///
/// Slim for Step 1 — only `Complete`, `TransportRetry`, and `Abort`.
/// Expanded in Step 2 with `FailoverRetry`, `SessionRetry`, `Hedge`.
#[derive(Debug)]
pub(crate) enum OperationAction {
    /// Return the successful response.
    Complete(TransportResult),
    /// Retry the transport attempt on the same endpoint.
    TransportRetry { new_state: OperationRetryState },
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
    }

    #[test]
    fn throttle_retry_fallback_exponential_backoff() {
        let state = ThrottleRetryState::new();
        // attempt 0: 5ms * 2^0 = 5ms
        assert_eq!(state.fallback_delay(), Duration::from_millis(5));

        let state = ThrottleRetryState {
            attempt_count: 1,
            ..ThrottleRetryState::new()
        };
        // attempt 1: 5ms * 2^1 = 10ms
        assert_eq!(state.fallback_delay(), Duration::from_millis(10));

        let state = ThrottleRetryState {
            attempt_count: 5,
            ..ThrottleRetryState::new()
        };
        // attempt 5: 5ms * 2^5 = 160ms
        assert_eq!(state.fallback_delay(), Duration::from_millis(160));
    }

    #[test]
    fn operation_retry_state_budget() {
        let state = OperationRetryState::initial();
        assert!(state.can_retry_transport());

        let state = state.advance_transport_retry();
        assert!(!state.can_retry_transport());
    }
}
