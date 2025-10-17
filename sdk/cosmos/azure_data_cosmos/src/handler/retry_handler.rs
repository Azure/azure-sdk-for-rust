// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use std::time::Duration;
use tokio::time::sleep;
use azure_core::http::{
    request::Request, RawResponse,
};
use crate::retry_policies::BaseRetryPolicy;

#[allow(dead_code)]
#[async_trait]
pub trait AbstractRetryHandler: Send + Sync {

    async fn send<Sender, Fut>(&self,
        request: &mut Request,
        sender: Sender,
    ) -> azure_core::Result<RawResponse>
    where
        Sender: Fn(&mut Request) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = azure_core::Result<RawResponse>> + Send;
}

/// Concrete implementation of AbstractRetryHandler
#[derive(Debug, Clone)]
pub struct BackoffRetryHandler {
    base_retry_policy: BaseRetryPolicy,
}

impl BackoffRetryHandler {
    /// Creates a new RetryHandler with the given pipeline
    ///
    /// This constructor initializes a BaseRetryPolicy with default configuration,
    /// which includes all available retry policies:
    /// - ResourceThrottleRetryPolicy for handling 429 TooManyRequests
    /// - GoneRetryPolicy for handling 410 Gone (partition splits)
    /// - SessionRetryPolicy for session consistency issues
    /// - DefaultRetryPolicy for general connection errors
    ///
    /// # Arguments
    /// * `pipeline` - The CosmosPipeline to use for sending requests
    pub fn new() -> Self {
        Self {
            base_retry_policy: BaseRetryPolicy::new(),
        }
    }
}

#[async_trait]
impl AbstractRetryHandler for BackoffRetryHandler {

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