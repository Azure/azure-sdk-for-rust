// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod client_retry_policy;
pub mod metadata_request_retry_policy;
pub mod resource_throttle_retry_policy;

use crate::constants::{SubStatusCode, RETRY_WITH, SUB_STATUS};
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::client_retry_policy::ClientRetryPolicy;
use crate::retry_policies::metadata_request_retry_policy::MetadataRequestRetryPolicy;
use crate::retry_policies::resource_throttle_retry_policy::ResourceThrottleRetryPolicy;
use azure_core::error::ErrorKind;
use azure_core::http::headers::Headers;
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
    Client(Box<ClientRetryPolicy>),
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

    /// Returns the Cosmos sub-status to preserve when a terminal partition-key-range
    /// Gone (410.1000/1002/1007) must be surfaced as 503 Service Unavailable instead
    /// of a raw 410, or `None` when no such conversion is pending. Only the data-plane
    /// [`ClientRetryPolicy`] produces this signal.
    pub fn terminal_gone_substatus(&self) -> Option<SubStatusCode> {
        match self {
            RetryPolicy::Client(p) => p.terminal_gone_substatus(),
            RetryPolicy::ResourceThrottle(_) | RetryPolicy::Metadata(_) => None,
        }
    }
}

/// Rewrites a terminal partition-key-range Gone result (410 with sub-status
/// 1000/1002/1007) into a 503 Service Unavailable while preserving the original
/// Cosmos sub-status, so application-layer resilience logic (which is wired for
/// 503, not 410) can classify and react to it.
///
/// Cosmos gateway responses for non-success status codes arrive here as an
/// `Err(HttpResponse { status: Gone, .. })`; the rare `Ok(RawResponse)` form with a
/// Gone status is handled too. Any other result is returned unchanged.
pub(crate) fn convert_gone_to_service_unavailable(
    result: azure_core::Result<RawResponse>,
    sub_status: SubStatusCode,
) -> azure_core::Result<RawResponse> {
    let mut headers = match &result {
        Ok(response) if response.status() == StatusCode::Gone => response.headers().clone(),
        Err(err) if err.http_status() == Some(StatusCode::Gone) => Headers::new(),
        // Not a Gone result: leave it untouched.
        _ => return result,
    };
    headers.insert(SUB_STATUS, sub_status.to_string());

    match result {
        Ok(_) => Ok(RawResponse::from_bytes(
            StatusCode::ServiceUnavailable,
            headers,
            Vec::new(),
        )),
        Err(_) => Err(azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::ServiceUnavailable,
                error_code: Some("ServiceUnavailable".to_string()),
                raw_response: Some(Box::new(RawResponse::from_bytes(
                    StatusCode::ServiceUnavailable,
                    headers,
                    Vec::new(),
                ))),
            },
            format!(
                "partition key range is gone (sub-status {sub_status}); routing information was stale \
                 and could not be refreshed within the retry budget, surfaced as 503 Service Unavailable"
            ),
        )),
    }
}

/// Returns `true` if the given status code and sub-status code combination is non-retryable.
///
/// Most status codes listed here indicate client-side errors that will not succeed on retry,
/// regardless of which endpoint handles the request. The sub-status code is needed for
/// `NotFound` (404): a plain 404 is non-retryable, but `404:1002` (ReadSessionNotAvailable)
/// is retryable and handled by session-aware retry logic. `TooManyRequests` (429) is also
/// included because it should not be retried by the client/metadata retry policies; instead,
/// it is handled by the dedicated `ResourceThrottleRetryPolicy` which implements proper
/// exponential backoff with `x-ms-retry-after-ms` headers.
fn is_non_retryable_status_code(
    status_code: StatusCode,
    sub_status_code: Option<SubStatusCode>,
) -> bool {
    // 404 is non-retryable unless the sub-status indicates ReadSessionNotAvailable (1002),
    // which is a transient routing condition retried via session-aware logic.
    if status_code == StatusCode::NotFound {
        return sub_status_code != Some(SubStatusCode::READ_SESSION_NOT_AVAILABLE);
    }

    matches!(
        status_code,
        StatusCode::BadRequest
            | StatusCode::Unauthorized
            | StatusCode::MethodNotAllowed
            | StatusCode::Conflict
            | StatusCode::PreconditionFailed
            | StatusCode::PayloadTooLarge
            | StatusCode::Locked
            | StatusCode::TooManyRequests
            | RETRY_WITH
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

/// Whether the HTTP request was actually sent to the server.
///
/// This determines retry safety:
/// - [`NotSent`](RequestSentStatus::NotSent): safe to retry both reads and writes.
/// - [`Sent`](RequestSentStatus::Sent) or [`Unknown`](RequestSentStatus::Unknown):
///   only safe to retry reads because writes may have been applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RequestSentStatus {
    /// The request was definitely not sent (e.g., connection refused, DNS failure).
    NotSent,
    /// The request was definitely sent (e.g., got an HTTP response, decode error).
    Sent,
    /// Cannot determine whether the request was sent (e.g., timeout, body error).
    Unknown,
}

/// Extension trait for determining request sent status from errors.
///
/// Uses [`ErrorKind`] to classify errors. The HTTP client layer (e.g., reqwest)
/// is responsible for mapping transport errors to the appropriate `ErrorKind`
/// variants (`Connection`, etc.).
pub(crate) trait RequestSentExt {
    /// Returns the [`RequestSentStatus`] based on error analysis.
    fn request_sent_status(&self) -> RequestSentStatus;
}

impl RequestSentExt for azure_core::Error {
    fn request_sent_status(&self) -> RequestSentStatus {
        match self.kind() {
            ErrorKind::Connection | ErrorKind::Credential | ErrorKind::DataConversion => {
                RequestSentStatus::NotSent
            }
            ErrorKind::HttpResponse { .. } => RequestSentStatus::Sent,
            _ => RequestSentStatus::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_error_is_not_sent() {
        let err = azure_core::Error::with_message(ErrorKind::Connection, "connection refused");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    #[test]
    fn io_error_is_unknown() {
        let err = azure_core::Error::with_message(ErrorKind::Io, "some io error");
        assert_eq!(err.request_sent_status(), RequestSentStatus::Unknown);
    }

    #[test]
    fn http_response_is_sent() {
        let err = azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: azure_core::http::StatusCode::InternalServerError,
                error_code: None,
                raw_response: None,
            },
            "server error",
        );
        assert_eq!(err.request_sent_status(), RequestSentStatus::Sent);
    }

    #[test]
    fn credential_is_not_sent() {
        let err = azure_core::Error::with_message(ErrorKind::Credential, "auth failed");
        assert_eq!(err.request_sent_status(), RequestSentStatus::NotSent);
    }

    fn gone_error_with_substatus(value: &str) -> azure_core::Error {
        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert(SUB_STATUS, value.to_string());
        let raw = RawResponse::from_bytes(StatusCode::Gone, headers, Vec::new());
        azure_core::Error::new(
            ErrorKind::HttpResponse {
                status: StatusCode::Gone,
                error_code: None,
                raw_response: Some(Box::new(raw)),
            },
            "partition key range gone",
        )
    }

    #[test]
    fn convert_gone_error_to_service_unavailable_preserves_substatus() {
        let converted = convert_gone_to_service_unavailable(
            Err(gone_error_with_substatus("1002")),
            SubStatusCode::PARTITION_KEY_RANGE_GONE,
        );
        let err = converted.expect_err("a Gone error must remain an error");
        assert_eq!(err.http_status(), Some(StatusCode::ServiceUnavailable));
        assert_eq!(
            get_substatus_code_from_error(&err),
            Some(SubStatusCode::PARTITION_KEY_RANGE_GONE),
            "the original sub-status must be preserved on the 503"
        );
    }

    #[test]
    fn convert_gone_ok_response_to_service_unavailable_preserves_substatus() {
        let mut headers = azure_core::http::headers::Headers::new();
        headers.insert(SUB_STATUS, "1002");
        let response = RawResponse::from_bytes(StatusCode::Gone, headers, Vec::new());

        let converted = convert_gone_to_service_unavailable(
            Ok(response),
            SubStatusCode::PARTITION_KEY_RANGE_GONE,
        )
        .expect("ok response must remain ok");
        assert_eq!(converted.status(), StatusCode::ServiceUnavailable);
        assert_eq!(
            get_substatus_code_from_response(&converted),
            Some(SubStatusCode::PARTITION_KEY_RANGE_GONE)
        );
    }

    #[test]
    fn convert_leaves_non_gone_results_untouched() {
        let response = RawResponse::from_bytes(
            StatusCode::Ok,
            azure_core::http::headers::Headers::new(),
            Vec::new(),
        );
        let converted = convert_gone_to_service_unavailable(
            Ok(response),
            SubStatusCode::PARTITION_KEY_RANGE_GONE,
        )
        .expect("non-gone ok stays ok");
        assert_eq!(converted.status(), StatusCode::Ok);
    }
}
