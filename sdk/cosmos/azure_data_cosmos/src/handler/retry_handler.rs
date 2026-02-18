// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::conditional_send::{ConditionalSend, ConditionalSync};
use crate::cosmos_request::CosmosRequest;
use crate::retry_policies::client_retry_policy::ClientRetryPolicy;
use crate::retry_policies::metadata_request_retry_policy::MetadataRequestRetryPolicy;
use crate::retry_policies::{RetryPolicy, RetryResult};
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use async_trait::async_trait;
use azure_core::{async_runtime::get_async_runtime, http::RawResponse};
use std::sync::Arc;
use tracing::debug;

/// Trait defining the interface for retry handlers in Cosmos DB operations
///
/// This trait provides a contract for implementing retry logic that wraps HTTP requests
/// with automatic retry capabilities. Implementations can inject custom retry policies
/// and handle both transient failures (errors) and non-success HTTP responses.
#[allow(dead_code)]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait RetryHandler: ConditionalSend + ConditionalSync {
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
        request: &mut CosmosRequest,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut CosmosRequest) -> Fut + ConditionalSend + ConditionalSync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + ConditionalSend;
}

/// Concrete retry handler implementation with exponential back off.
/// This handler provides automatic retry capabilities for Cosmos DB operations using
/// a pluggable retry policy system. It wraps HTTP requests with intelligent retry logic
/// that handles both transient network errors and HTTP error responses.
#[derive(Debug, Clone)]
pub(crate) struct BackOffRetryHandler {
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
}

impl BackOffRetryHandler {
    /// Returns the appropriate retry policy based on the request
    ///
    /// This method examines the underlying operation and resource types and determines
    /// which retry policy should be used for this specific request. Metadata operations
    /// use the MetadataRequestRetryPolicy, while data plane operations use the
    /// ClientRetryPolicy.
    ///
    /// # Arguments
    /// * `request` - The HTTP request to analyze
    ///
    /// # Returns
    /// A `RetryPolicy` enum variant appropriate for the request type
    pub fn retry_policy_for_request(&self, request: &CosmosRequest) -> RetryPolicy {
        if request.resource_type.is_meta_data() {
            RetryPolicy::Metadata(MetadataRequestRetryPolicy::new(
                self.global_endpoint_manager.clone(),
            ))
        } else {
            RetryPolicy::Client(Box::from(ClientRetryPolicy::new(
                self.global_endpoint_manager.clone(),
                self.global_partition_endpoint_manager.clone(),
                request.excluded_regions.clone(),
            )))
        }
    }

    pub fn new(
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
    ) -> Self {
        Self {
            global_endpoint_manager,
            global_partition_endpoint_manager,
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl RetryHandler for BackOffRetryHandler {
    /// Sends an HTTP request with automatic retry and exponential back off
    ///
    /// This implementation of the `RetryHandler::send` method provides robust
    /// retry logic.
    ///
    /// # Arguments
    /// * `request` - Mutable HTTP request (may be modified by retry policy between attempts)
    /// * `sender` - Callback that performs the actual HTTP request
    async fn send<Sender, Fut>(
        &self,
        request: &mut CosmosRequest,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut CosmosRequest) -> Fut + ConditionalSend + ConditionalSync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + ConditionalSend,
    {
        // Get the appropriate retry policy based on the request
        let mut retry_policy = self.retry_policy_for_request(request);

        loop {
            retry_policy.before_send_request(request).await;

            // Log the endpoint URL being used for this request
            debug!(
                target: "azure_data_cosmos::retry_handler",
                "Sending request - endpoint: {:?}, region: {:?}, operation: {:?}, resource: {:?}",
                request.request_context.location_endpoint_to_route,
                request.request_context.region_name,
                request.operation_type,
                request.resource_type
            );

            // Invoke the provided sender callback instead of calling inner_send_async directly
            let result = sender(request).await;
            let retry_result = retry_policy.should_retry(&result).await;

            match retry_result {
                RetryResult::DoNotRetry => return result,
                RetryResult::Retry { after } => get_async_runtime().sleep(after).await,
            }
        }
    }
}
