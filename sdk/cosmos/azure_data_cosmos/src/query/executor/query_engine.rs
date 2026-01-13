// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{headers::Headers, Context, Method, RawResponse, Request};
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::{
    conditional_send::ConditionalSend,
    constants,
    pipeline::CosmosPipeline,
    pipeline::{self, GatewayPipeline},
    query::{OwnedQueryPipeline, QueryEngineRef, QueryResult},
    resource_context::{ResourceLink, ResourceType},
    FeedPage, Query,
};

/// A query executor that uses an external query engine to support cross-partition queries.
pub struct QueryEngineExecutor<T: DeserializeOwned + ConditionalSend> {
    http_pipeline: Arc<CosmosPipeline>,
    http_pipeline: Arc<GatewayPipeline>,
    container_link: ResourceLink,
    items_link: ResourceLink,
    context: Context<'static>,
    query_engine: QueryEngineRef,
    base_request: Option<Request>,
    query: Query,
    pipeline: Option<OwnedQueryPipeline>,
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: DeserializeOwned + ConditionalSend + 'static> QueryEngineExecutor<T> {
    pub fn new(
        http_pipeline: Arc<GatewayPipeline>,
        container_link: ResourceLink,
        context: Context<'static>,
        query: Query,
        query_engine: QueryEngineRef,
    ) -> azure_core::Result<Self> {
        let items_link = container_link.feed(ResourceType::Documents);
        Ok(Self {
            http_pipeline,
            container_link,
            items_link,
            context,
            query_engine,
            base_request: None,
            query,
            pipeline: None,
            phantom: std::marker::PhantomData,
        })
    }

    /// Fetches the next page of query results.
    ///
    /// Returns `None` if there are no more pages to fetch.
    #[tracing::instrument(skip_all)]
    pub async fn next_page(&mut self) -> azure_core::Result<Option<FeedPage<T>>> {
        let pipeline = match self.pipeline.as_mut() {
            Some(pipeline) => pipeline,
            None => {
                // Initialize the pipeline.
                let query_plan = get_query_plan(
                    &self.http_pipeline,
                    &self.items_link,
                    self.context.to_borrowed(),
                    &self.query,
                    self.query_engine.supported_features()?,
                )
                .await?
                .into_body();
                let pkranges = get_pkranges(
                    &self.http_pipeline,
                    &self.container_link,
                    self.context.to_borrowed(),
                )
                .await?
                .into_body();

                let pipeline =
                    self.query_engine
                        .create_pipeline(&self.query.text, &query_plan, &pkranges)?;
                if let Some(query) = pipeline.query() {
                    let query = Query::from(query).with_parameters_from(&self.query);
                    self.base_request = Some(create_base_query_request(
                        self.http_pipeline.url(&self.items_link),
                        &query,
                    )?);
                }

                self.pipeline = Some(pipeline);
                self.pipeline.as_mut().unwrap()
            }
        };

        // Execute the pipeline to get a page of results
        while !pipeline.complete() {
            // Run the pipeline
            let results = pipeline.run()?;

            // If we got items, go ahead and return them. The next time we call the pipeline, we'll get no items and just requests.
            if !results.items.is_empty() {
                // Deserialize the items
                let items = results
                    .items
                    .into_iter()
                    .map(|item| serde_json::from_str::<T>(item.get()))
                    .collect::<Result<Vec<_>, _>>()?;

                // TODO: Provide a continuation token.
                return Ok(Some(FeedPage::new(items, None, Headers::new())));
            }

            // No items, so make any requests we need to make and provide them to the pipeline.
            // TODO: We can absolutely parallelize these requests.
            for request in results.requests {
                let mut query_request = if let Some(query) = request.query {
                    let mut query = Query::from(query);
                    if request.include_parameters {
                        query = query.with_parameters_from(&self.query)
                    }
                    create_base_query_request(self.http_pipeline.url(&self.items_link), &query)?
                } else if let Some(base_request) = &self.base_request {
                    base_request.clone()
                } else {
                    if cfg!(debug_assertions) {
                        panic!(
                            "internal error: base_request should be set if no query is provided"
                        );
                    }
                    return Err(azure_core::error::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        "internal error: pipeline had no query, and neither did the query request",
                    ));
                };

                query_request.insert_header(
                    constants::PARTITION_KEY_RANGE_ID,
                    request.partition_key_range_id.clone(),
                );

                let mut fetch_more_pages = true;
                while fetch_more_pages {
                    if let Some(c) = request.continuation.clone() {
                        query_request.insert_header(constants::CONTINUATION, c);
                    } else {
                        // Make sure we don't send a continuation header if we don't have one, even if we did on a previous iteration.
                        query_request.headers_mut().remove(constants::CONTINUATION);
                    }

                    let resp = self
                        .http_pipeline
                        .send_raw(
                            self.context.to_borrowed(),
                            &mut query_request,
                            self.items_link.clone(),
                        )
                        .await?;

                    let next_continuation =
                        resp.headers().get_optional_string(&constants::CONTINUATION);

                    fetch_more_pages = request.drain && next_continuation.is_some();

                    let body = resp.into_body();
                    let result = QueryResult {
                        partition_key_range_id: &request.partition_key_range_id,
                        request_id: request.id,
                        next_continuation,
                        result: &body,
                    };

                    // For now, just provide a single result at a time.
                    // When we parallelize requests, we can more easily provide multiple results at once.
                    pipeline.provide_data(vec![result])?;
                }
            }

            // No items, but we provided more data (probably), so continue the loop.
            // If there were no more requests to make, the pipeline will be marked complete above
        }

        // If we get here, the pipeline is complete and we have no items to return.
        Ok(None)
    }
}

fn create_base_query_request(url: url::Url, query: &Query) -> azure_core::Result<Request> {
    let mut request = Request::new(url, Method::Post);
    request.insert_header(constants::QUERY, "True");
    request.add_mandatory_header(&constants::QUERY_CONTENT_TYPE);
    request.set_json(query)?;
    Ok(request)
}

#[tracing::instrument(skip_all)]
async fn get_query_plan(
    http_pipeline: &GatewayPipeline,
    items_link: &ResourceLink,
    context: Context<'_>,
    query: &Query,
    supported_features: &str,
) -> azure_core::Result<RawResponse> {
    let url = http_pipeline.url(items_link);
    let mut request = create_base_query_request(url, query)?;
    request.insert_header(constants::QUERY_ENABLE_CROSS_PARTITION, "True");
    request.insert_header(constants::IS_QUERY_PLAN_REQUEST, "True");
    request.insert_header(
        constants::SUPPORTED_QUERY_FEATURES,
        supported_features.to_string(),
    );

    http_pipeline
        .send_raw(context, &mut request, items_link.clone())
        .await
}

#[tracing::instrument(skip_all)]
async fn get_pkranges(
    http_pipeline: &GatewayPipeline,
    container_link: &ResourceLink,
    context: Context<'_>,
) -> azure_core::Result<RawResponse> {
    let pkranges_link = container_link.feed(ResourceType::PartitionKeyRanges);
    let url = http_pipeline.url(&pkranges_link);
    let mut base_request = Request::new(url, Method::Get);

    http_pipeline
        .send_raw(context, &mut base_request, pkranges_link)
        .await
}
