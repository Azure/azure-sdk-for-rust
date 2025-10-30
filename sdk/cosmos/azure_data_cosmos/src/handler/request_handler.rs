// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::cosmos_request::{AuthorizationTokenType, CosmosRequest};
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::operation_context::OperationType;
use crate::pipeline::CosmosPipeline;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::{ItemOptions, PartitionKey};
use azure_core::http::Response;

/// Concrete retry handler implementation with exponential back off.
/// This handler provides automatic retry capabilities for Cosmos DB operations using
/// a pluggable retry policy system. It wraps HTTP requests with intelligent retry logic
/// that handles both transient network errors and HTTP error responses.
#[derive(Debug, Clone)]
pub struct RequestHandler {
    pipeline: CosmosPipeline,
    retry_handler: BackOffRetryHandler,
}

impl RequestHandler {
    /// Creates a new `RequestHandler` wrapping the provided `CosmosPipeline`.
    ///
    /// This is a lightweight constructor; the handler itself is stateless aside
    /// from holding a cloneable reference to the underlying `CosmosPipeline`.
    ///
    /// # Arguments
    /// * `pipeline` - An initialized `CosmosPipeline` used to send requests.
    ///
    /// # Examples
    /// ```ignore
    /// let handler = RequestHandler::new(pipeline.clone());
    /// ```
    pub fn new(pipeline: CosmosPipeline) -> Self {
        Self {
            pipeline,
            retry_handler: BackOffRetryHandler,
        }
    }

    pub async fn send<T>(
        &self,
        partition_key: PartitionKey,
        body: Option<Vec<u8>>,
        operation_type: OperationType,
        resource_type: ResourceType,
        options: Option<ItemOptions<'_>>,
        resource_link: ResourceLink,
    ) -> azure_core::Result<Response<T>> {
        let mut cosmos_request = CosmosRequest::new(
            operation_type,
            resource_type,
            None, // resource_id (RID) not yet known here
            partition_key,
            body,
            false,
            AuthorizationTokenType::Primary,
            options,
        );
        cosmos_request.request_context.location_endpoint_to_route =
            Option::from(resource_link.url(&self.pipeline.endpoint));

        let item_options = cosmos_request.clone().options.unwrap_or_default();

        let ctx = item_options
            .method_options
            .context
            .with_value(resource_link.clone());

        // Clone pipeline and convert context to owned so the closure can be Fn
        let pipeline = self.pipeline.clone();

        // Prepare a cloneable ResourceLink to avoid moving it, allowing the closure to be Fn
        let resource_link_for_sender = resource_link.clone();
        let sender = move |req: &mut CosmosRequest| {
            let pipeline = pipeline.clone();
            let ctx = ctx.clone();
            let mut raw_req = req.clone().to_raw_request();
            let url = resource_link_for_sender.clone();
            async move { pipeline.send_raw(ctx, &mut raw_req, url).await }
        };

        // Delegate to the retry handler, providing the sender callback
        let res = self.retry_handler.send(&mut cosmos_request, sender).await;

        // Convert RawResponse into typed Response<T>
        res.map(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
    use azure_core::time::{Duration, OffsetDateTime};
    use std::sync::Arc;

    #[derive(Debug)]
    struct TestTokenCredential;

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl TokenCredential for TestTokenCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            Ok(AccessToken::new(
                "test_token".to_string(),
                OffsetDateTime::now_utc().saturating_add(Duration::minutes(5)),
            ))
        }
    }

    fn test_pipeline() -> CosmosPipeline {
        let cred: Arc<dyn TokenCredential> = Arc::new(TestTokenCredential);
        let auth = crate::pipeline::AuthorizationPolicy::from_token_credential(cred);
        CosmosPipeline::new(
            "https://example.com/".parse().unwrap(),
            auth,
            Default::default(),
        )
    }

    #[test]
    fn new_preserves_endpoint() {
        let pipeline = test_pipeline();
        let endpoint = pipeline.endpoint.clone();
        let handler = RequestHandler::new(pipeline.clone());
        assert_eq!(
            endpoint, handler.pipeline.endpoint,
            "handler should retain pipeline endpoint"
        );
        // cloning handler should preserve endpoint
        let handler_clone = handler.clone();
        assert_eq!(handler_clone.pipeline.endpoint, handler.pipeline.endpoint);
    }
}
