// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;
mod cosmos_headers_policy;
mod signature_target;

use crate::cosmos_request::CosmosRequest;
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::models::CosmosResponse;
use crate::resource_context::ResourceLink;
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::routing::global_partition_endpoint_manager::GlobalPartitionEndpointManager;
use crate::{constants, CosmosClientOptions};
pub(crate) use authorization_policy::AuthorizationPolicy;
use azure_core::error::CheckSuccessOptions;
use azure_core::http::{
    request::Request,
    response::Response,
    Context, Method, PipelineSendOptions, RawResponse,
};
pub(crate) use cosmos_headers_policy::CosmosHeadersPolicy;
use serde::de::DeserializeOwned;
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
    options: CosmosClientOptions,
    #[allow(dead_code)]
    pub fault_injection_enabled: bool,
}

/// Constants for change feed header values.
pub(crate) mod change_feed_headers {
    pub const INCREMENTAL_FEED: &str = "Incremental feed";
    pub const FULL_FIDELITY_FEED: &str = "FullFidelityFeed";
    pub const WIRE_FORMAT_VERSION: &str = "2021-09-15";
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
        let retry_handler =
            BackOffRetryHandler::new(global_endpoint_manager, global_partition_endpoint_manager);
        GatewayPipeline {
            endpoint,
            pipeline,
            retry_handler,
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
        // Prepare a callback delegate to invoke the http request.
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

        #[cfg(feature = "fault_injection")]
        // added flag because possible that fault injection feature flag is enabled
        // but the transport isn't injected with the fault injection client
        if self.fault_injection_enabled {
            cosmos_request.add_fault_injection_headers();
        }

        // Delegate to the retry handler, providing the sender callback
        let raw_response: RawResponse =
            self.retry_handler.send(&mut cosmos_request, sender).await?;
        let typed_response: Response<T> = raw_response.into();
        Ok(CosmosResponse::new(typed_response, cosmos_request))
    }

    /// Creates a change feed request for reading changes from a container.
    ///
    /// This method sets up the appropriate headers and returns an iterator that
    /// can be used to iterate through changes.
    #[allow(clippy::too_many_arguments)]
    pub fn send_change_feed_request<T: DeserializeOwned + Send + 'static>(
        &self,
        ctx: Context<'_>,
        url: Url,
        resource_link: ResourceLink,
        partition_key_range_id: String,
        is_full_fidelity: bool,
        if_none_match: Option<String>,
        if_modified_since: Option<String>,
        apply_request_headers: impl Fn(&mut Request) -> azure_core::Result<()>,
    ) -> azure_core::Result<crate::FeedItemIterator<T>> {
        let mut base_request = Request::new(url, Method::Get);

        // Set change feed headers
        if is_full_fidelity {
            base_request.insert_header(constants::A_IM, change_feed_headers::FULL_FIDELITY_FEED);
            base_request.insert_header(
                constants::COSMOS_CHANGEFEED_WIRE_FORMAT_VERSION,
                change_feed_headers::WIRE_FORMAT_VERSION,
            );
        } else {
            base_request.insert_header(constants::A_IM, change_feed_headers::INCREMENTAL_FEED);
        }

        // Set partition key range ID
        base_request.insert_header(constants::PARTITION_KEY_RANGE_ID, partition_key_range_id);

        // Set If-None-Match header (etag for continuation)
        if let Some(etag) = if_none_match {
            base_request.insert_header(constants::IF_NONE_MATCH, etag);
        }

        // Set If-Modified-Since header for point-in-time queries
        if let Some(date) = if_modified_since {
            base_request.insert_header("if-modified-since", date);
        }

        apply_request_headers(&mut base_request)?;

        // Apply client-level headers if not already present
        self.options.apply_headers(base_request.headers_mut());

        let pipeline = self.pipeline.clone();
        let context = ctx.with_value(resource_link).into_owned();

        Ok(crate::FeedItemIterator::new(
            futures::stream::try_unfold(
                (pipeline, base_request, context, None::<String>, false),
                |(pipeline, base_request, context, continuation, done)| async move {
                    if done {
                        return Ok(None);
                    }

                    let mut req = base_request.clone();
                    if let Some(ref cont) = continuation {
                        // For change feed, continuation is the etag
                        req.insert_header(constants::IF_NONE_MATCH, cont.clone());
                    }

                    let resp = pipeline.send(&context, &mut req, None).await?;
                    let headers = resp.headers().clone();
                    let next_continuation =
                        headers.get_optional_string(&constants::CONTINUATION);
                    let body: crate::feed::FeedBody<T> = resp.into_body().json()?;
                    let page = crate::FeedPage::new(
                        body.items,
                        next_continuation.clone(),
                        headers,
                    );
                    let is_done = next_continuation.is_none();

                    Ok(Some((
                        page,
                        (pipeline, base_request, context, next_continuation, is_done),
                    )))
                },
            ),
        ))
    }
}
