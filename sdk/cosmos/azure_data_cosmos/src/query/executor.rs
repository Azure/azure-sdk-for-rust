// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query execution implementation.

use azure_core::http::{
    headers::{Header, Headers},
    Context,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::{
    conditional_send::ConditionalSend, constants, cosmos_request::CosmosRequest,
    cosmos_request::CosmosRequestBuilder, operation_context::OperationType,
    pipeline::GatewayPipeline, resource_context::ResourceLink, FeedPage, Query,
};

/// A query executor that sends queries directly to the gateway endpoint.
///
/// This executor does not support cross-partition queries and requires a partition key to be specified.
pub struct QueryExecutor<T: DeserializeOwned + ConditionalSend> {
    http_pipeline: Arc<GatewayPipeline>,
    items_link: ResourceLink,
    context: Context<'static>,
    query: Query,
    base_headers: Headers,
    continuation: Option<String>,
    complete: bool,
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: DeserializeOwned + ConditionalSend + 'static> QueryExecutor<T> {
    pub fn new(
        http_pipeline: Arc<GatewayPipeline>,
        items_link: ResourceLink,
        context: Context<'static>,
        query: Query,
        base_headers: Headers,
    ) -> Self {
        Self {
            http_pipeline,
            items_link,
            context,
            query,
            base_headers,
            continuation: None,
            complete: false,
            phantom: std::marker::PhantomData,
        }
    }

    /// Consumes the executor and converts it into a stream of pages.
    pub fn into_stream(self) -> azure_core::Result<crate::FeedItemIterator<T>> {
        Ok(crate::FeedItemIterator::new(futures::stream::try_unfold(
            self,
            |mut state| async move {
                let val = state.next_page().await?;
                Ok(val.map(|item| (item, state)))
            },
        )))
    }

    /// Fetches the next page of query results.
    ///
    /// Returns `None` if there are no more pages to fetch.
    #[tracing::instrument(skip_all)]
    pub async fn next_page(&mut self) -> azure_core::Result<Option<FeedPage<T>>> {
        if self.complete {
            return Ok(None);
        }

        // Build CosmosRequest for this page
        let mut builder = create_query_cosmos_request_builder(&self.items_link, &self.query)?;

        // Apply base headers
        for (name, value) in self.base_headers.clone() {
            builder = builder.header(name, value);
        }

        // Apply continuation token if present
        if let Some(continuation) = &self.continuation {
            builder = builder.header(constants::CONTINUATION, continuation.clone());
        }

        let cosmos_request = builder.build()?;

        // Send through the pipeline
        let resp = self
            .http_pipeline
            .send_raw(cosmos_request, self.context.to_borrowed())
            .await?;

        let page = FeedPage::<T>::from_response(resp).await?;

        match page.continuation() {
            Some(token) => self.continuation = Some(token.to_string()),
            None => self.complete = true,
        }

        Ok(Some(page))
    }
}

fn create_query_cosmos_request_builder(
    items_link: &ResourceLink,
    query: &Query,
) -> azure_core::Result<CosmosRequestBuilder> {
    let builder = CosmosRequest::builder(OperationType::Query, items_link.clone())
        .header(constants::QUERY, "True")
        .header(
            constants::QUERY_CONTENT_TYPE.name(),
            constants::QUERY_CONTENT_TYPE.value(),
        )
        .json(query);
    Ok(builder)
}
