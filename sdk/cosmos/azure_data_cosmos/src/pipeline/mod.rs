// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod authorization_policy;
mod signature_target;

pub use authorization_policy::AuthorizationPolicy;
use azure_core::http::{
    headers::AsHeaders,
    pager::{PagerOptions, PagerState},
    request::{options::ContentType, Request},
    response::Response,
    Context, Method, RawResponse,
};
use futures::TryStreamExt;
use serde::de::DeserializeOwned;
use url::Url;

use crate::cosmos_request::CosmosRequest;
use crate::handler::retry_handler::{BackOffRetryHandler, RetryHandler};
use crate::routing::global_endpoint_manager::GlobalEndpointManager;
use crate::{
    constants,
    models::ThroughputProperties,
    resource_context::{ResourceLink, ResourceType},
    CosmosClientOptions, FeedPage, FeedPager, Query,
};

/// Newtype that wraps an Azure Core pipeline to provide a Cosmos-specific pipeline which configures our authorization policy and enforces that a [`ResourceType`] is set on the context.
#[derive(Debug, Clone)]
pub struct CosmosPipeline {
    pub endpoint: Url,
    pipeline: azure_core::http::Pipeline,
    retry_handler: BackOffRetryHandler,
    options: CosmosClientOptions,
}

impl CosmosPipeline {
    pub fn new(
        endpoint: Url,
        pipeline: azure_core::http::Pipeline,
        global_endpoint_manager: GlobalEndpointManager,
        options: CosmosClientOptions,
    ) -> Self {
        let retry_handler = BackOffRetryHandler::new(global_endpoint_manager);
        CosmosPipeline {
            endpoint,
            pipeline,
            retry_handler,
            options,
        }
    }

    /// Creates a [`Url`] out of the provided [`ResourceLink`]
    ///
    /// This is a little backwards, ideally we'd accept [`ResourceLink`] in the [`CosmosPipeline::send`] method,
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
        // Clone pipeline and convert context to owned so the closure can be Fn
        let pipeline = self.pipeline.clone();
        let ctx_owned = ctx.with_value(resource_link).into_owned();
        pipeline.send(&ctx_owned, request, None).await
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

    pub fn send_query_request<T: DeserializeOwned + Send>(
        &self,
        ctx: Context<'_>,
        query: Query,
        url: Url,
        resource_link: ResourceLink,
        apply_request_headers: impl Fn(&mut Request) -> azure_core::Result<()>,
    ) -> azure_core::Result<FeedPager<T>> {
        let mut base_request = create_base_query_request(url, &query)?;
        apply_request_headers(&mut base_request)?;

        // Only apply client-level headers if they aren't already present on the request.
        // Caller-provided request headers must take precedence.
        for (name, value) in self
            .options
            .as_headers()
            .expect("CosmosClientOptions is infallible")
        {
            let header_val = base_request.headers().get_optional_str(&name);
            if header_val.is_none() {
                base_request.insert_header(name, value);
            }
        }

        // We have to double-clone here.
        // First we clone the pipeline to pass it in to the closure
        let pipeline = self.pipeline.clone();
        let options = PagerOptions {
            context: ctx.with_value(resource_link).into_owned(),
            ..Default::default()
        };
        Ok(FeedPager::new(
            move |continuation, pager_options| {
                // Then we have to clone it again to pass it in to the async block.
                // This is because Pageable can't borrow any data, it has to own it all.
                // That's probably good, because it means a Pageable can outlive the client that produced it, but it requires some extra cloning.
                let pipeline = pipeline.clone();
                let mut req = base_request.clone();
                Box::pin(async move {
                    if let PagerState::More(continuation) = continuation {
                        req.insert_header(constants::CONTINUATION, continuation);
                    }

                    let resp = pipeline
                        .send(&pager_options.context, &mut req, None)
                        .await?;
                    let page = FeedPage::<T>::from_response(resp).await?;

                    Ok(page.into())
                })
            },
            Some(options),
        ))
    }

    /// Helper function to read a throughput offer given a resource ID.
    ///
    /// ## Arguments
    /// * `context` - The context for the request.
    /// * `resource_id` - The resource ID to read the throughput offer for.
    pub async fn read_throughput_offer(
        &self,
        context: Context<'_>,
        resource_id: &str,
    ) -> azure_core::Result<Option<ThroughputProperties>> {
        // We only have to into_owned here in order to call send_query_request below,
        // since it returns `Pager` which must own it's data.
        // See https://github.com/Azure/azure-sdk-for-rust/issues/1911 for further discussion
        let context = context.into_owned();

        // Now, query for the offer for this resource.
        let query = Query::from("SELECT * FROM c WHERE c.offerResourceId = @rid")
            .with_parameter("@rid", resource_id)?;
        let offers_link = ResourceLink::root(ResourceType::Offers);
        let mut results = self.send_query_request::<ThroughputProperties>(
            context.clone(),
            query,
            self.url(&offers_link),
            offers_link.clone(),
            |_| Ok(()),
        )?;

        // There should only be one offer for a given resource ID.
        let offer = results.try_next().await?;
        Ok(offer)
    }

    /// Helper function to update a throughput offer given a resource ID.
    ///
    /// ## Arguments
    /// * `context` - The context for the request.
    /// * `resource_id` - The resource ID to update the throughput offer for.
    /// * `throughput` - The new throughput to set.
    pub async fn replace_throughput_offer(
        &self,
        context: Context<'_>,
        resource_id: &str,
        throughput: ThroughputProperties,
    ) -> azure_core::Result<Response<ThroughputProperties>> {
        let response = self
            .read_throughput_offer(context.clone(), resource_id)
            .await?;
        let mut current_throughput = response.unwrap_or_default();
        current_throughput.offer = throughput.offer;

        // NOTE: Offers API doesn't allow Enable Content Response On Write to be false, so once we support that option, we'll need to ignore it here.
        let offer_link =
            ResourceLink::root(ResourceType::Offers).item_by_rid(&current_throughput.offer_id);
        let mut req = Request::new(self.url(&offer_link), Method::Put);
        req.insert_headers(&ContentType::APPLICATION_JSON)?;
        req.set_json(&current_throughput)?;

        self.send_raw(context, &mut req, offer_link)
            .await
            .map(Into::into)
    }
}

pub(crate) fn create_base_query_request(url: Url, query: &Query) -> azure_core::Result<Request> {
    let mut request = Request::new(url, Method::Post);
    request.insert_header(constants::QUERY, "True");
    request.add_mandatory_header(&constants::QUERY_CONTENT_TYPE);
    request.set_json(query)?;
    Ok(request)
}
