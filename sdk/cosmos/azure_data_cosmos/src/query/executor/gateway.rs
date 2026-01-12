// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{Context, Method, Request};
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::{constants, pipeline::CosmosPipeline, resource_context::ResourceLink, FeedPage, Query};

/// A query executor that sends queries directly to the gateway endpoint.
///
/// This executor does not support cross-partition queries and requires a partition key to be specified.
pub struct GatewayExecutor<T: DeserializeOwned> {
    http_pipeline: Arc<CosmosPipeline>,
    items_link: ResourceLink,
    context: Context<'static>,
    base_request: Request,
    continuation: Option<String>,
    complete: bool,
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: DeserializeOwned + Send + 'static> GatewayExecutor<T> {
    pub fn new(
        http_pipeline: Arc<CosmosPipeline>,
        items_link: ResourceLink,
        context: Context<'static>,
        query: Query,
        apply_headers: impl FnOnce(&mut Request) -> azure_core::Result<()>,
    ) -> azure_core::Result<Self> {
        let url = http_pipeline.url(&items_link);
        let mut base_request = create_base_query_request(url, &query)?;
        apply_headers(&mut base_request)?;

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

        Ok(Self {
            http_pipeline,
            items_link,
            context,
            base_request,
            continuation: None,
            complete: false,
            phantom: std::marker::PhantomData,
        })
    }

    /// Fetches the next page of query results.
    ///
    /// Returns `None` if there are no more pages to fetch.
    #[tracing::instrument(skip_all)]
    pub async fn next_page(&mut self) -> azure_core::Result<Option<FeedPage<T>>> {
        if self.complete {
            return Ok(None);
        }

        let mut req = self.base_request.clone();
        if let Some(continuation) = self.continuation.clone() {
            req.insert_header(constants::CONTINUATION, continuation);
        }

        let resp = self
            .http_pipeline
            .send_raw(
                self.context.to_borrowed(),
                &mut req,
                self.items_link.clone(),
            )
            .await?;

        let page = FeedPage::<T>::from_response(resp).await?;

        match page.continuation() {
            Some(token) => self.continuation = Some(token.to_string()),
            None => self.complete = true,
        }

        Ok(Some(page))
    }
}

fn create_base_query_request(
    url: url::Url,
    query: &Query,
) -> azure_core::Result<azure_core::http::Request> {
    let mut request = Request::new(url, Method::Post);
    request.insert_header(constants::QUERY, "True");
    request.add_mandatory_header(&constants::QUERY_CONTENT_TYPE);
    request.set_json(query)?;
    Ok(request)
}
