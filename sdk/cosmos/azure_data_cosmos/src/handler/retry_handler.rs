use async_trait::async_trait;
use std::time::Duration;
use tokio::time::{sleep};
use azure_core::http::{
    request::Request,
    Context, RawResponse,
};
use crate::pipeline::CosmosPipeline;
use crate::resource_context::ResourceLink;
use crate::retry_policies::{
    BaseRetryPolicy,
    resource_throttle_retry_policy::{DocumentClientRetryPolicy, ResourceThrottleRetryPolicy}
};

#[allow(dead_code)]
#[async_trait]
pub trait AbstractRetryHandler: Send + Sync {
    // async fn get_retry_policy(&self, request: &RequestMessage) -> Arc<dyn IDocumentClientRetryPolicy>;

    fn pipeline(&self) -> &CosmosPipeline;

    /// Returns the base retry policy for this handler
    ///
    /// This method provides access to the BaseRetryPolicy which manages
    /// all available retry policies (throttle, gone, session, default).
    fn base_retry_policy(&self) -> &BaseRetryPolicy;

    async fn send_async(
        &self,
        ctx: Context<'_>,
        request: &mut Request,
        resource_link: ResourceLink,
    ) -> azure_core::Result<RawResponse> {
        // Get the appropriate retry policy based on the request
        let retry_policy = self.base_retry_policy().get_policy_for_request(request);
        retry_policy.on_before_send_request(request);

        loop {
            let result = self.inner_send_async(ctx.clone(), request, resource_link.clone()).await;

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
                    }
                    else {
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

    async fn inner_send_async(
        &self,
        ctx: Context<'_>,
        request: &mut Request,
        resource_link: ResourceLink,
    ) -> azure_core::Result<RawResponse> {
        self.pipeline().send_raw(ctx, request, resource_link).await
    }
}

/// Concrete implementation of AbstractRetryHandler
#[derive(Clone)]
pub struct RetryHandler {
    pipeline: CosmosPipeline,
    base_retry_policy: BaseRetryPolicy,
}

impl RetryHandler {
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
    ///
    /// # Example
    /// ```ignore
    /// use azure_data_cosmos::pipeline::CosmosPipeline;
    /// use azure_data_cosmos::handler::retry_handler::RetryHandler;
    ///
    /// let pipeline = CosmosPipeline::new(...);
    /// let retry_handler = RetryHandler::new(pipeline);
    /// ```
    pub fn new(pipeline: CosmosPipeline) -> Self {
        Self {
            pipeline,
            base_retry_policy: BaseRetryPolicy::new(),
        }
    }
}

#[async_trait]
impl AbstractRetryHandler for RetryHandler {
    fn pipeline(&self) -> &CosmosPipeline {
        &self.pipeline
    }

    fn base_retry_policy(&self) -> &BaseRetryPolicy {
        &self.base_retry_policy
    }
}