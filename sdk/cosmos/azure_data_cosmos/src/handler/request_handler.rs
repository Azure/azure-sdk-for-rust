// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use typespec_client_core::http::{Response};
use crate::{ItemOptions, PartitionKey};
use crate::cosmos_request::{AuthorizationTokenType, CosmosRequest};
use crate::operation_context::OperationType;
use crate::pipeline::CosmosPipeline;
use crate::resource_context::{ResourceLink, ResourceType};
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};

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
            retry_handler: BackOffRetryHandler
        }
    }

    pub async fn send<T>(
        &self,
        partition_key: PartitionKey,
        body: Option<Vec<u8>>,
        operation_type: OperationType,
        resource_type: ResourceType,
        options: Option<ItemOptions<'_>>,
        resource_link: ResourceLink
    ) -> azure_core::Result<Response<T>> {

        // TODO: Pass the real resource id (RID) if available; None means it may be resolved later.
        // `CosmosRequest::new` signature:
        // (operation_type, resource_type, resource_id: Option<String>, partition_key, body, headers: Option<Headers>, is_name_based, auth_token_type, options)
        let mut cosmos_request = CosmosRequest::new(
            operation_type,
            resource_type,
            None,                // resource_id (RID) not yet known here
            partition_key,
            body,                 // raw body bytes (if any)
            false,                // is_name_based
            AuthorizationTokenType::Primary,
            options,
        );
        cosmos_request.request_context.location_endpoint_to_route = Option::from(resource_link.url(&self.pipeline.endpoint));
        let item_options = cosmos_request.clone().options.unwrap_or_default();
        let ctx = item_options.method_options.context.with_value(resource_link.clone());

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