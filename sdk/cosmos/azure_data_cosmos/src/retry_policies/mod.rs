// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod client_retry_policy;
pub mod metadata_request_retry_policy;
pub mod resource_throttle_retry_policy;

use crate::constants::{SubStatusCode, SUB_STATUS};
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::client_retry_policy::ClientRetryPolicy;
use crate::retry_policies::metadata_request_retry_policy::MetadataRequestRetryPolicy;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use azure_core::error::ErrorKind;
use azure_core::http::{RawResponse, StatusCode};
use azure_core::time::Duration;

/// Result of a retry policy decision
///
/// This enum represents the outcome of evaluating whether an HTTP request should be retried
/// after encountering an error or receiving a response that may warrant a retry (such as
/// transient failures, rate limiting, or service unavailability).
///
/// # Variants
///
/// * `DoNotRetry` - The operation should not be retried. This is returned for successful
///   responses, permanent failures, or when retry limits have been exhausted.
///
/// * `Retry { after }` - The operation should be retried after waiting for the specified
///   duration. The delay allows for exponential backoff or respects server-provided retry
///   hints (e.g., from `Retry-After` headers).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryResult {
    /// Indicates that the operation should not be retried.
    DoNotRetry,
    /// Indicates that the operation should be retried after waiting for the duration specified in `after`.
    Retry { after: Duration },
}

impl RetryResult {
    /// Returns `true` if the result indicates a retry should be performed.
    pub fn is_retry(&self) -> bool {
        matches!(self, RetryResult::Retry { .. })
    }
}

/// Enum representing different retry policy strategies for Cosmos DB operations.
///
/// This enum provides a type-safe way to handle different retry policies based on the
/// operation type. Each variant wraps a specific retry policy implementation that handles
/// particular failure scenarios and retry logic.
///
/// # Variants
///
/// * `Client` - Policy for data plane operations (document CRUD, queries). Handles session
///   token mismatches, endpoint failures, service unavailability, write forbidden errors,
///   and resource throttling with multi-region failover support.
///
/// * `Metadata` - Policy for metadata operations (database/container management). Handles
///   transient metadata failures with endpoint failover but simpler retry logic than data
///   plane operations.
///
/// * `ResourceThrottle` - Policy specifically for handling 429 (TooManyRequests) errors
///   with exponential backoff. Used when rate limiting is the primary concern.
#[allow(dead_code)]
pub enum RetryPolicy {
    /// Data plane retry policy for document operations with comprehensive failure handling
    Client(ClientRetryPolicy),
    /// Metadata operation retry policy for control plane operations with endpoint failover
    Metadata(MetadataRequestRetryPolicy),
    /// Throttling-focused retry policy for rate limit scenarios with exponential backoff
    ResourceThrottle(ResourceThrottleRetryPolicy),
}

impl RetryPolicy {
    /// Prepares a request before it is sent, allowing policy-specific modifications.
    ///
    /// # Summary
    /// Delegates to the appropriate concrete retry policy's `before_send_request` method
    /// based on the enum variant. This method is called before each request attempt
    /// (including retries) and allows the policy to configure endpoint routing, update
    /// request context, refresh location cache, and set retry-specific parameters.
    ///
    /// For Client policies, this includes endpoint resolution and multi-region routing.
    /// For Metadata policies, this includes location-based routing for metadata endpoints.
    /// For ResourceThrottle policies, this is typically a no-op as throttling doesn't
    /// require request modifications.
    ///
    /// # Arguments
    /// * `request` - Mutable reference to the request being prepared
    pub async fn before_send_request(&mut self, request: &mut CosmosRequest) {
        match self {
            RetryPolicy::Client(p) => p.before_send_request(request).await,
            RetryPolicy::ResourceThrottle(_p) => {}
            RetryPolicy::Metadata(p) => p.before_send_request(request).await,
        }
    }

    /// Determines whether a request should be retried based on the response or error.
    ///
    /// # Summary
    /// Delegates to the appropriate concrete retry policy's `should_retry` method based
    /// on the enum variant. Evaluates the result of a request attempt and decides whether
    /// to retry and with what delay. Each policy variant has specific retry logic:
    ///
    /// - **Client**: Handles 503/500/410/403/404.1022 with endpoint failover, session token
    ///   retries, and delegates 429 to throttling logic.
    /// - **Metadata**: Handles 503/500/410/403 for metadata operations with simpler failover.
    /// - **ResourceThrottle**: Handles 429 (TooManyRequests) with exponential backoff based
    ///   on Retry-After headers or calculated delays.
    ///
    /// # Arguments
    /// * `response` - Result of the request attempt (Ok with response or Err with error)
    ///
    /// # Returns
    /// A `RetryResult`:
    /// - `Retry { after: Duration }` if retry should occur with specified delay
    /// - `DoNotRetry` for successful requests or non-retryable failures
    pub async fn should_retry(
        &mut self,
        response: &azure_core::Result<RawResponse>,
    ) -> RetryResult {
        match self {
            RetryPolicy::Client(p) => p.should_retry(response).await,
            RetryPolicy::ResourceThrottle(p) => p.should_retry(response).await,
            RetryPolicy::Metadata(p) => p.should_retry(response).await,
        }
    }
}

/// Returns `true` if the given status code is non-retryable.
///
/// These status codes indicate client-side errors that will not succeed on retry,
/// regardless of which endpoint handles the request. Note that `TooManyRequests` (429)
/// is included here because it should not be retried by the client/metadata retry
/// policies; instead, it is handled by the dedicated `ResourceThrottleRetryPolicy`
/// which implements proper exponential backoff with `x-ms-retry-after-ms` headers.
fn is_non_retryable_status_code(status_code: StatusCode) -> bool {
    matches!(
        status_code,
        StatusCode::BadRequest
            | StatusCode::Unauthorized
            | StatusCode::NotFound
            | StatusCode::Conflict
            | StatusCode::PreconditionFailed
            | StatusCode::PayloadTooLarge
            | StatusCode::TooManyRequests
    )
}

/// Extracts the Cosmos DB sub-status code from an error.
///
/// # Summary
/// Attempts to retrieve the x-ms-substatus header value from an HTTP response error.
/// Sub-status codes provide additional context about Cosmos DB errors (e.g., LeaseNotFound,
/// READ_SESSION_NOT_AVAILABLE, WriteForbidden, DATABASE_ACCOUNT_NOT_FOUND) that help
/// retry policies make more informed decisions about whether to retry and how to route
/// the next attempt.
///
/// # Arguments
/// * `err` - The error to extract sub-status from
///
/// # Returns
/// `Some(SubStatusCode)` if the error contains a valid sub-status, `None` otherwise
fn get_substatus_code_from_error(err: &azure_core::Error) -> Option<SubStatusCode> {
    if let ErrorKind::HttpResponse { raw_response, .. } = err.kind() {
        raw_response
            .as_ref()
            .and_then(|r| {
                r.headers()
                    .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
                    .ok()
            })
            .map(SubStatusCode::from)
    } else {
        None
    }
}

/// Extracts the Cosmos DB sub-status code from an HTTP response.
///
/// # Summary
/// Attempts to retrieve the x-ms-substatus header value from a raw HTTP response.
/// Sub-status codes provide additional context about Cosmos DB errors that help retry
/// policies distinguish between different failure scenarios (e.g., lease conflicts,
/// session token issues, write forbidden, account not found) and make appropriate
/// retry decisions.
///
/// # Arguments
/// * `response` - The HTTP response to extract sub-status from
///
/// # Returns
/// `Some(SubStatusCode)` if the response contains a valid sub-status header, `None` otherwise
fn get_substatus_code_from_response(response: &RawResponse) -> Option<SubStatusCode> {
    response
        .headers()
        .get_as::<u32, std::num::ParseIntError>(&SUB_STATUS)
        .ok()
        .map(SubStatusCode::from)
}
