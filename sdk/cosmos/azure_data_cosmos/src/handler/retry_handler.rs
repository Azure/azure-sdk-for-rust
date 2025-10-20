// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::retry_policies::BaseRetryPolicy;
use async_trait::async_trait;
use azure_core::http::{request::Request, RawResponse};
use std::time::Duration;
use tokio::time::sleep;

// Helper trait to conditionally require Send on non-WASM targets
#[cfg(not(target_arch = "wasm32"))]
pub trait CosmosConditionalSend: Send {}
#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> CosmosConditionalSend for T {}

#[cfg(target_arch = "wasm32")]
pub trait CosmosConditionalSend {}
#[cfg(target_arch = "wasm32")]
impl<T> CosmosConditionalSend for T {}

/// Trait defining the interface for retry handlers in Cosmos DB operations
///
/// This trait provides a contract for implementing retry logic that wraps HTTP requests
/// with automatic retry capabilities. Implementations can inject custom retry policies
/// and handle both transient failures (errors) and non-success HTTP responses.
#[allow(dead_code)]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait AbstractRetryHandler: Send + Sync {
    /// Sends an HTTP request with automatic retry logic
    ///
    /// This method wraps the provided sender callback with retry logic, automatically
    /// handling transient failures and implementing exponential backoff. The method
    /// will continue retrying until either:
    /// - The request succeeds (non-error 2xx status)
    /// - The retry policy determines no more retries should be attempted
    /// - Maximum retry attempts are exceeded
    ///
    /// # Arguments
    /// * `request` - Mutable reference to the HTTP request to send (may be modified by retry policy)
    /// * `sender` - Callback function that performs the actual HTTP request. This function
    ///              takes a mutable request reference and returns a future that resolves to
    ///              a `RawResponse` or error.
    ///
    /// # Type Parameters
    /// * `Sender` - Function type that takes `&mut Request` and returns a Future
    /// * `Fut` - Future type returned by the sender that resolves to `Result<RawResponse>`
    ///
    /// # Returns
    /// `Result<RawResponse>` - The final response (success or failure after all retry attempts)
    async fn send<Sender, Fut>(
        &self,
        request: &mut Request,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut Request) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + CosmosConditionalSend;
}

/// Concrete retry handler implementation with exponential backoff.
/// This handler provides automatic retry capabilities for Cosmos DB operations using
/// a pluggable retry policy system. It wraps HTTP requests with intelligent retry logic
/// that handles both transient network errors and HTTP error responses.
#[derive(Debug, Clone)]
pub struct BackoffRetryHandler {
    base_retry_policy: BaseRetryPolicy,
}

impl BackoffRetryHandler {

    /// Creates a new retry handler with default retry policies
    ///
    /// Initializes a `BackoffRetryHandler` with a `BaseRetryPolicy` using default
    /// configuration values:
    /// - Max throttle retry count: 3
    /// - Max throttle wait time: 100 seconds
    /// - Throttle backoff factor: 30
    pub fn new() -> Self {
        Self {
            base_retry_policy: BaseRetryPolicy::new(),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl AbstractRetryHandler for BackoffRetryHandler {

    /// Sends an HTTP request with automatic retry and exponential backoff
    ///
    /// This implementation of the `AbstractRetryHandler::send` method provides robust
    /// retry logic with the following behavior:
    ///
    /// # Arguments
    /// * `request` - Mutable HTTP request (may be modified by retry policy between attempts)
    /// * `sender` - Callback that performs the actual HTTP request
    ///
    /// # Returns
    /// * `Ok(RawResponse)` - Successful response (may be from initial or retry attempt)
    /// * `Err(Error)` - Final error after all retry attempts exhausted
    async fn send<Sender, Fut>(
        &self,
        request: &mut Request,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut Request) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + CosmosConditionalSend,
    {
        // Get the appropriate retry policy based on the request
        let retry_policy = self.base_retry_policy.get_policy_for_request(request);
        retry_policy.on_before_send_request(request);

        loop {
            // Invoke the provided sender callback instead of calling inner_send_async directly
            let result = sender(request).await;

            match &result {
                Ok(resp) => {
                    if resp.status().is_server_error() || resp.status().is_client_error() {
                        let retry_result = retry_policy.should_retry_response(resp).await;
                        if !retry_result.should_retry {
                            return result;
                        }

                        if retry_result.backoff_time > Duration::ZERO {
                            tracing::warn!(
                                "Retry backoff requested for {:?}.",
                                retry_result.backoff_time
                            );
                            sleep(retry_result.backoff_time).await;
                        }
                    } else {
                        // Success - return immediately
                        return result;
                    }
                }
                Err(err) => {
                    let retry_result = retry_policy.should_retry_exception(err).await;
                    if !retry_result.should_retry {
                        return result;
                    }
                    if retry_result.backoff_time > Duration::ZERO {
                        sleep(retry_result.backoff_time).await;
                    }
                }
            }
        }
    }
}
