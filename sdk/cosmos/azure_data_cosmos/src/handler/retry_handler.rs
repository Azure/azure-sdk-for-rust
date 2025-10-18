// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;
use azure_core::http::{
    request::Request, RawResponse,
};
use crate::retry_policies::BaseRetryPolicy;

/// Trait defining the interface for retry handlers in Cosmos DB operations
///
/// This trait provides a contract for implementing retry logic that wraps HTTP requests
/// with automatic retry capabilities. Implementations can inject custom retry policies
/// and handle both transient failures (errors) and non-success HTTP responses.
///
/// # Design Pattern
/// The trait uses a callback-based design where the caller provides a `sender` function
/// that performs the actual HTTP request. This allows the retry handler to:
/// - Intercept and examine requests before they're sent
/// - Inspect responses and errors to determine retry eligibility
/// - Apply backoff delays between retry attempts
/// - Track retry attempts and enforce limits
///
/// # Thread Safety
/// Implementers must be `Send + Sync` to allow use across async runtime boundaries
/// and concurrent access from multiple tasks.
#[allow(dead_code)]
#[async_trait]
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
    ///
    /// # Retry Behavior
    /// - **Success responses** (2xx): Returned immediately without retry
    /// - **Client/Server errors** (4xx/5xx): Passed to retry policy for evaluation
    /// - **Network errors**: Passed to retry policy for evaluation
    /// - **Backoff delays**: Applied between retry attempts as determined by the policy
    ///
    /// # Example Flow
    /// ```text
    /// 1. Request → Retry Policy (pre-send hook)
    /// 2. Send via callback
    /// 3. Response/Error → Retry Policy (should retry?)
    /// 4. If yes: Sleep(backoff) → Go to step 1
    /// 5. If no: Return result
    /// ```
    async fn send<Sender, Fut>(&self,
        request: &mut Request,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut Request) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send;
}

/// Concrete retry handler implementation with exponential backoff
///
/// This handler provides automatic retry capabilities for Cosmos DB operations using
/// a pluggable retry policy system. It wraps HTTP requests with intelligent retry logic
/// that handles both transient network errors and HTTP error responses.
///
/// # Features
/// - **Automatic Retry**: Transparently retries failed requests based on policy decisions
/// - **Exponential Backoff**: Implements progressive delay increases between retries
/// - **Policy-Based**: Uses `BaseRetryPolicy` to select appropriate retry strategy per request
/// - **Error Handling**: Handles both network exceptions and HTTP error responses (4xx/5xx)
/// - **Thread-Safe**: Can be cloned and shared across async tasks
///
/// # Retry Policies
/// The handler delegates retry decisions to `BaseRetryPolicy`, which selects the
/// appropriate policy based on the request:
/// - **ResourceThrottleRetryPolicy**: For 429 TooManyRequests errors
/// - **GoneRetryPolicy**: For 410 Gone errors (partition splits) - planned
/// - **SessionRetryPolicy**: For session consistency issues - planned
/// - **DefaultRetryPolicy**: For general transient failures - planned
///
/// # Example
/// ```ignore
/// use azure_data_cosmos::handler::retry_handler::BackoffRetryHandler;
///
/// let handler = BackoffRetryHandler::new();
/// // Handler is now ready to wrap requests with retry logic
/// ```
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
    ///
    /// The handler is ready to use immediately and can be cloned for concurrent use.
    ///
    /// # Example
    /// ```
    /// use azure_data_cosmos::handler::retry_handler::BackoffRetryHandler;
    ///
    /// let handler = BackoffRetryHandler::new();
    /// // Use handler with pipeline to send requests with retry logic
    /// ```
    pub fn new() -> Self {
        Self {
            base_retry_policy: BaseRetryPolicy::new(),
        }
    }
}

#[async_trait]
impl AbstractRetryHandler for BackoffRetryHandler {

    /// Sends an HTTP request with automatic retry and exponential backoff
    ///
    /// This implementation of the `AbstractRetryHandler::send` method provides robust
    /// retry logic with the following behavior:
    ///
    /// # Retry Decision Flow
    /// 1. **Pre-send Hook**: Calls retry policy's `on_before_send_request` for any request modifications
    /// 2. **Send Request**: Invokes the provided sender callback to perform the HTTP request
    /// 3. **Evaluate Response**:
    ///    - **Success (2xx)**: Returns immediately without retry
    ///    - **Client/Server Error (4xx/5xx)**: Consults retry policy
    ///    - **Network Error**: Consults retry policy
    /// 4. **Retry Decision**: Policy determines if retry should occur and calculates backoff
    /// 5. **Backoff Sleep**: If retrying, sleeps for the calculated duration
    /// 6. **Loop**: Returns to step 1 if retry approved, otherwise returns result
    ///
    /// # Arguments
    /// * `request` - Mutable HTTP request (may be modified by retry policy between attempts)
    /// * `sender` - Callback that performs the actual HTTP request
    ///
    /// # Returns
    /// * `Ok(RawResponse)` - Successful response (may be from initial or retry attempt)
    /// * `Err(Error)` - Final error after all retry attempts exhausted
    ///
    /// # Logging
    /// Emits warning logs when backoff delays are applied, helping with debugging
    /// and monitoring retry behavior in production.
    ///
    /// # Policy Selection
    /// The handler automatically selects the appropriate retry policy based on request
    /// characteristics (headers, method, etc.) via `BaseRetryPolicy::get_policy_for_request`.
    async fn send<Sender, Fut>(&self,
                               request: &mut Request,
                               sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn( &mut Request) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send,
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