// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;
mod cosmos_headers_policy;
mod signature_target;

use crate::availability_strategy::{self, AvailabilityStrategy};
use crate::cosmos_request::CosmosRequest;
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::models::CosmosResponse;
use crate::resource_context::ResourceLink;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use crate::CosmosClientOptions;
pub(crate) use authorization_policy::AuthorizationPolicy;
use azure_core::error::CheckSuccessOptions;
use azure_core::http::{response::Response, Context, PipelineSendOptions, RawResponse};
pub(crate) use cosmos_headers_policy::CosmosHeadersPolicy;
use std::sync::Arc;
use url::Url;

/// Success codes: 200-299 range plus 304 (Not Modified)
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

/// Newtype that wraps an Azure Core pipeline to provide a Cosmos-specific pipeline which configures our authorization policy and enforces that a [`ResourceType`] is set on the context.
#[derive(Debug, Clone)]
pub(crate) struct GatewayPipeline {
    pub endpoint: Url,
    pipeline: azure_core::http::Pipeline,
    retry_handler: BackOffRetryHandler,
    global_endpoint_manager: Arc<GlobalEndpointManager>,
    options: CosmosClientOptions,
    #[allow(dead_code)]
    pub fault_injection_enabled: bool,
}

impl GatewayPipeline {
    pub fn new(
        endpoint: Url,
        pipeline: azure_core::http::Pipeline,
        global_endpoint_manager: Arc<GlobalEndpointManager>,
        global_partition_endpoint_manager: Arc<GlobalPartitionEndpointManager>,
        options: CosmosClientOptions,
        fault_injection_enabled: bool,
    ) -> Self {
        let retry_handler = BackOffRetryHandler::new(
            global_endpoint_manager.clone(),
            global_partition_endpoint_manager,
        );
        GatewayPipeline {
            endpoint,
            pipeline,
            retry_handler,
            global_endpoint_manager,
            options,
            fault_injection_enabled,
        }
    }

    /// Creates a [`Url`] out of the provided [`ResourceLink`]
    ///
    /// This is a little backwards, ideally we'd accept [`ResourceLink`] in the [`GatewayPipeline::send`] method,
    /// but we need callers to be able to build an [`azure_core::Request`] so they need to be able to get the full URL.
    /// This allows the clients to hold a single thing representing the "connection" to a Cosmos DB account though.
    #[allow(dead_code)]
    pub(crate) fn url(&self, link: &ResourceLink) -> Url {
        link.url(&self.endpoint)
    }

    pub async fn send<T>(
        &self,
        mut cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<CosmosResponse<T>> {
        self.options.apply_headers(&mut cosmos_request.headers);

        #[cfg(feature = "fault_injection")]
        // added flag because possible that fault injection feature flag is enabled
        // but the transport isn't injected with the fault injection client
        if self.fault_injection_enabled {
            cosmos_request.add_fault_injection_headers();
        }

        // Check if hedging should be applied
        let should_hedge = self.should_hedge(&cosmos_request);

        let raw_response = if should_hedge {
            self.send_with_hedging(cosmos_request.clone(), context.clone())
                .await?
        } else {
            // Non-hedging path: use in-line sender closure (original code path)
            let sender = |req: &mut CosmosRequest| {
                let pipeline = self.pipeline.clone();
                let ctx = context.clone();
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
            self.retry_handler.send(&mut cosmos_request, sender).await?
        };

        let typed_response: Response<T> = raw_response.into();
        Ok(CosmosResponse::new(typed_response, cosmos_request))
    }

    /// Sends a request with cross-region hedging.
    ///
    /// The hedging strategy sends parallel requests to multiple regions and returns
    /// the first non-transient response.
    async fn send_with_hedging(
        &self,
        cosmos_request: CosmosRequest,
        context: Context<'_>,
    ) -> azure_core::Result<RawResponse> {
        let strategy = match &self.options.availability_strategy {
            Some(AvailabilityStrategy::CrossRegionHedging(s)) => s,
            _ => unreachable!("send_with_hedging called without hedging strategy"),
        };

        let retry_handler = self.retry_handler.clone();
        let pipeline = self.pipeline.clone();
        // Convert the borrowed context into an owned one for 'static usage
        let ctx = context.into_owned();

        // The hedging sender: for each hedged request, run through the full retry pipeline
        let hedging_sender = move |mut req: CosmosRequest| {
            let retry_handler = retry_handler.clone();
            let pipeline = pipeline.clone();
            let ctx = ctx.clone();
            async move {
                let sender = |req: &mut CosmosRequest| {
                    let pipeline = pipeline.clone();
                    let ctx = ctx.clone();
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
                retry_handler.send(&mut req, sender).await
            }
        };

        availability_strategy::execute_hedging(
            strategy,
            &cosmos_request,
            &self.global_endpoint_manager,
            hedging_sender,
        )
        .await
    }

    /// Determines whether the current request should use hedging.
    fn should_hedge(&self, request: &CosmosRequest) -> bool {
        match &self.options.availability_strategy {
            Some(AvailabilityStrategy::CrossRegionHedging(strategy)) => {
                strategy.should_hedge(request, &self.global_endpoint_manager)
            }
            _ => false,
        }
    }
}
