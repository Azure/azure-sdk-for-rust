// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data exchanged between the operation and transport pipelines.

use azure_core::http::{headers::Headers, Method};
use url::Url;

use super::{deadline::Deadline, AuthorizationContext};
#[cfg(feature = "unstable_dtx")]
use crate::models::ResourceType;
use crate::{
    diagnostics::{ExecutionContext, RequestSentStatus},
    models::{
        effective_partition_key::EffectivePartitionKey, CosmosResponseHeaders, CosmosStatus,
        DefaultConsistencyLevel, OperationType,
    },
    options::ReadConsistencyStrategy,
    pipeline::operation::routing::{CosmosEndpoint, RoutingFallbackReason},
};

/// Transport mode for a routed attempt.
///
/// Determines which HTTP transport (and protocol version) is used
/// for a given request attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TransportMode {
    /// Standard gateway (HTTP/1.1 or HTTP/2 via ALPN).
    Gateway,
    /// Gateway 2.0 (HTTP/2 with prior knowledge).
    GatewayV2,
}

impl TransportMode {
    /// Returns whether requests need Gateway V2 framing.
    #[inline(always)]
    pub fn is_gateway_v2(self) -> bool {
        matches!(self, Self::GatewayV2)
    }
}

/// A fixed target and request data for one transport invocation.
///
/// The operation pipeline supplies this data for each operation attempt or
/// hedge leg. Transport retries rebuild and sign the request without changing
/// its URL, regional endpoint, or transport mode.
#[derive(Debug)]
pub(crate) struct TransportRequest {
    /// The HTTP method.
    pub method: Method,
    /// The endpoint selected for this attempt.
    pub endpoint: CosmosEndpoint,
    /// The routed transport mode for this attempt.
    pub transport_mode: TransportMode,
    /// The operation type being dispatched.
    pub operation_type: OperationType,
    /// Effective partition key precomputed in the operation pipeline for
    /// item-scoped Gateway 2.0 dispatch. Computing it before the transport
    /// pipeline keeps wire layers free of partition-key/definition logic.
    pub effective_partition_key: Option<EffectivePartitionKey>,
    /// Effective consistency resolved from account default and read options.
    pub effective_consistency: DefaultConsistencyLevel,
    /// Read consistency strategy as requested by the caller (per-request override
    /// preferred over client-level default; falls back to
    /// [`ReadConsistencyStrategy::Default`] when neither is set).
    ///
    /// Wire layers consult this *in addition* to `effective_consistency`: when it is
    /// non-`Default` on a read operation, V1 emits the
    /// `x-ms-cosmos-read-consistency-strategy` HTTP header (and strips
    /// `x-ms-consistency-level`); V2 emits the RNTBD `ReadConsistencyStrategy`
    /// token (`0x00FE`) and drops the `ConsistencyLevel` token.
    pub read_consistency_strategy: ReadConsistencyStrategy,
    /// The fully resolved URL for this attempt.
    pub url: Url,
    /// Headers to send (includes operation-specific and attempt-specific headers).
    pub headers: Headers,
    /// Type of resource targeted by the operation.
    #[cfg(feature = "unstable_dtx")]
    pub resource_type: ResourceType,
    /// Request body bytes (schema-agnostic).
    pub body: Option<azure_core::Bytes>,
    /// Authorization context for signing.
    pub auth_context: AuthorizationContext,
    /// The execution context (Initial/Retry/Hedging/Failover).
    pub execution_context: ExecutionContext,
    /// Why this request fell back from its operation-specific route.
    pub routing_fallback: Option<RoutingFallbackReason>,
    /// End-to-end deadline for the overall operation.
    pub deadline: Deadline,
}

/// The outcome of a transport attempt or a completed transport invocation.
///
/// The transport pipeline returns its final outcome to the operation pipeline.
#[derive(Debug)]
pub(crate) struct TransportResult {
    /// The outcome of this attempt.
    pub outcome: TransportOutcome,
}

impl TransportResult {
    /// Creates a timeout result when the end-to-end operation deadline is exceeded.
    // Inline the small result constructor.
    #[inline(always)]
    pub fn deadline_exceeded(request_sent: RequestSentStatus) -> Self {
        Self {
            outcome: TransportOutcome::DeadlineExceeded { request_sent },
        }
    }

    /// Creates a result from an HTTP response payload.
    ///
    /// Successful status codes are mapped to `Success`; non-success status codes
    /// are mapped to `HttpError` with `request_sent` set to `Sent`.
    // Inline result assembly when useful; response decoding remains separate.
    #[inline]
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
    #[inline(always)]
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

impl TransportOutcome {
    /// Returns whether the service returned a successful response.
    #[inline(always)]
    pub fn is_successful(&self) -> bool {
        matches!(self, Self::Success { .. })
    }

    /// Checks for a 429 eligible for transport retries rather than body-bearing DTX recovery.
    #[inline(always)]
    pub fn is_retryable_throttle(&self, is_distributed_transaction: bool) -> bool {
        matches!(
            self,
            Self::HttpError { status, body, .. }
                if status.is_throttled() && (!is_distributed_transaction || body.is_empty())
        )
    }
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
