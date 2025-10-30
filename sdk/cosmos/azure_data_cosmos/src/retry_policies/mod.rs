// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod resource_throttle_retry_policy;
use async_trait::async_trait;
use azure_core::http::{RawResponse, Request};
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

/// Trait defining the retry policy interface for Cosmos DB operations
///
/// This trait provides a contract for implementing retry logic for transient failures
/// in Azure Cosmos DB operations. Implementers can define custom retry behavior for
/// both exceptions (errors) and HTTP responses based on their specific requirements.
#[async_trait]
pub trait RetryPolicy: Send + Sync {
    /// Called before sending a request to allow policy-specific modifications
    ///
    /// This method is invoked immediately before each request is sent (including retries).
    /// # Arguments
    /// * `request` - Mutable reference to the HTTP request being sent
    fn before_send_request(&self, _request: &mut Request) {}

    /// Determines whether an HTTP request should be retried based on the response or error
    ///
    /// This method evaluates the result of an HTTP request attempt and decides whether
    /// the operation should be retried, and if so, how long to wait before the next attempt.
    ///
    /// # Arguments
    ///
    /// * `response` - A reference to the result of the HTTP request attempt. This can be:
    ///   - `Ok(RawResponse)` - A successful HTTP response (which may still indicate an error via status code)
    ///   - `Err(azure_core::Error)` - A network or client-side error
    ///
    /// # Returns
    ///
    /// A `RetryResult` indicating the retry decision.
    async fn should_retry(&mut self, response: &azure_core::Result<RawResponse>) -> RetryResult;
}
