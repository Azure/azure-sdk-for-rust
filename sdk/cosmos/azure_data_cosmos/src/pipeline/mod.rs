// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;
mod signature_target;

use crate::cosmos_request::CosmosRequest;
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::models::CosmosResponse;
use crate::resource_context::ResourceLink;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::CosmosClientOptions;
pub use authorization_policy::AuthorizationPolicy;
use azure_core::error::CheckSuccessOptions;
use azure_core::http::{response::Response, Context, PipelineSendOptions, RawResponse};
use std::sync::Arc;
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
        global_endpoint_manager: Arc<GlobalEndpointManager>,
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

    pub async fn send<T>(
        &self,
        mut cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<CosmosResponse<T>> {
        cosmos_request.client_headers(&self.options);
        // Prepare a callback delegate to invoke the http request.
        let sender = |req: &mut CosmosRequest| {
            let pipeline = self.pipeline.clone();
            let ctx = context.clone();
            // Success codes: 200-299 range plus 304 (Not Modified)
            const SUCCESS_CODES: [u16; 101] = {
                let mut codes = [0u16; 101];
                let mut i = 0;
                while i < 100 {
                    codes[i] = 200 + i as u16;
                    i += 1;
                }
                codes[100] = 304;
                codes
            };
            let success_options = CheckSuccessOptions {
                success_codes: &SUCCESS_CODES,
            };
            let pipeline_send_options = PipelineSendOptions {
                skip_checks: false,
                check_success: success_options,
            };
            let resource_link = req.resource_link.clone();
            let mut raw_req = req.clone().into_raw_request();
            async move {
                let ctx_owned = ctx.with_value(resource_link).into_owned();
                pipeline
                    .send(&ctx_owned, &mut raw_req, Some(pipeline_send_options))
                    .await
            }
        };

        // Delegate to the retry handler, providing the sender callback
        let raw_response: RawResponse =
            self.retry_handler.send(&mut cosmos_request, sender).await?;
        let typed_response: Response<T> = raw_response.into();
        Ok(CosmosResponse::new(typed_response, cosmos_request))
    }
}
