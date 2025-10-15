use async_trait::async_trait;
use std::time::Duration;
use azure_core::http::{
    request::Request,
    Context, RawResponse,
};
use crate::pipeline::CosmosPipeline;
use crate::resource_context::ResourceLink;
use crate::retry_policies::resource_throttle_retry_policy::{
    DocumentClientRetryPolicy, ResourceThrottleRetryPolicy
};

#[allow(dead_code)]
#[async_trait]
pub trait AbstractRetryHandler: Send + Sync {
    // async fn get_retry_policy(&self, request: &RequestMessage) -> Arc<dyn IDocumentClientRetryPolicy>;

    fn pipeline(&self) -> &CosmosPipeline;

    async fn send_async(
        &self,
        ctx: Context<'_>,
        request: &mut Request,
        resource_link: ResourceLink,
    ) -> azure_core::Result<RawResponse> {
        let retry_policy = ResourceThrottleRetryPolicy::new(3, 100, 30);
        retry_policy.on_before_send_request(request);

        loop {
            let result = self.inner_send_async(ctx.clone(), request, resource_link.clone()).await;

            match &result {
                Ok(_resp) => {
                    // Success - return immediately
                    return result;
                }
                Err(err) => {
                    let retry_result = retry_policy.should_retry_exception(err).await;
                    if !retry_result.should_retry {
                        return result;
                    }
                    if retry_result.backoff_time > Duration::ZERO {
                        // TODO: Add async sleep support when tokio or futures-timer is available
                        // For now, continue immediately
                        // sleep(retry_result.backoff_time).await;
                    }
                }
            }
        }
    }

    // This would be the next handler in the chain, or the actual HTTP call
    // async fn inner_send_async(
    //     &self,
    //     request: &mut Request,
    // ) -> Result<azure_core::Result<RawResponse>, Box<dyn Error + Send + Sync>>;

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
}

impl RetryHandler {
    pub fn new(pipeline: CosmosPipeline) -> Self {
        Self { pipeline }
    }
}

#[async_trait]
impl AbstractRetryHandler for RetryHandler {
    fn pipeline(&self) -> &CosmosPipeline {
        &self.pipeline
    }
}