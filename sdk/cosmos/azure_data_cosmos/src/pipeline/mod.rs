// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;
mod signature_target;

use crate::cosmos_request::CosmosRequest;
pub use authorization_policy::AuthorizationPolicy;
use azure_core::http::{request::Request, response::Response, Context, RawResponse};
use url::Url;

use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::resource_context::ResourceLink;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::{
    constants,
    models::ThroughputProperties,
    resource_context::{ResourceLink, ResourceType},
    CosmosClientOptions, FeedPage, FeedPager, Query,
};
use crate::{cosmos_request::CosmosRequest, CosmosClientOptions};
pub use authorization_policy::AuthorizationPolicy;
use azure_core::error::CheckSuccessOptions;
use azure_core::http::{
    headers::AsHeaders,
    pager::{PagerOptions, PagerState},
    request::{options::ContentType, Request},
    response::Response,
    Context, Method, PipelineSendOptions, RawResponse,
};
use futures::TryStreamExt;
use serde::de::DeserializeOwned;
use url::Url;

/// Newtype that wraps an Azure Core pipeline to provide a Cosmos-specific pipeline which configures our authorization policy and enforces that a [`ResourceType`] is set on the context.
#[derive(Debug, Clone)]
pub struct GatewayPipeline {
    pub endpoint: Url,
    pipeline: azure_core::http::Pipeline,
    retry_handler: BackOffRetryHandler,
    options: CosmosClientOptions,
}

impl GatewayPipeline {
    pub fn new(
        endpoint: Url,
        pipeline: azure_core::http::Pipeline,
        global_endpoint_manager: GlobalEndpointManager,
        options: CosmosClientOptions,
    ) -> Self {
        let retry_handler = BackOffRetryHandler::new(global_endpoint_manager);
        GatewayPipeline {
            endpoint,
            pipeline,
            retry_handler,
            options,
        }
    }

    /// Creates a [`Url`] out of the provided [`ResourceLink`]
    ///
    /// This is a little backwards, ideally we'd accept [`ResourceLink`] in the [`GatewayPipeline::send`] method,
    /// but we need callers to be able to build an [`azure_core::Request`] so they need to be able to get the full URL.
    /// This allows the clients to hold a single thing representing the "connection" to a Cosmos DB account though.
    pub fn url(&self, link: &ResourceLink) -> Url {
        link.url(&self.endpoint)
    }

    pub async fn send_raw(
        &self,
        ctx: Context<'_>,
        request: &mut Request,
        resource_link: ResourceLink,
    ) -> azure_core::Result<RawResponse> {
        // Clone the pipeline and convert context to owned so the closure can be Fn
        let pipeline = self.pipeline.clone();
        let ctx_owned = ctx.with_value(resource_link).into_owned();

        let success_options = CheckSuccessOptions {
            success_codes: &[200, 201, 202, 204, 304],
        };
        let pipeline_send_options = PipelineSendOptions {
            skip_checks: false,
            check_success: success_options,
        };

        pipeline
            .send(&ctx_owned, request, Some(pipeline_send_options))
            .await
    }

    pub async fn send<T>(
        &self,
        mut cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<Response<T>> {
        cosmos_request.client_headers(&self.options);
        // Prepare a callback delegate to invoke the http request.
        let sender = move |req: &mut CosmosRequest| {
            let ctx = context.clone();
            let url = req.resource_link.clone();
            let mut raw_req = req.clone().into_raw_request();
            async move { self.send_raw(ctx, &mut raw_req, url).await }
        };

        // Delegate to the retry handler, providing the sender callback
        let res = self.retry_handler.send(&mut cosmos_request, sender).await;

        // Convert RawResponse into typed Response<T>
        res.map(Into::into)
    }
}
