use azure_core::http::{headers::Headers, Context, Method, RawResponse, Request};
use serde::de::DeserializeOwned;

use crate::{
    constants,
    pipeline::{self, CosmosPipeline},
    query::{OwnedQueryPipeline, QueryEngineRef, QueryResult},
    resource_context::{ResourceLink, ResourceType},
    FeedPage, FeedPager, Query, QueryOptions,
};

pub struct QueryExecutor<T: DeserializeOwned> {
    http_pipeline: CosmosPipeline,
    container_link: ResourceLink,
    items_link: ResourceLink,
    context: Context<'static>,
    query_engine: QueryEngineRef,
    base_request: Option<Request>,
    query: Query,
    pipeline: Option<OwnedQueryPipeline>,

    // Why is our phantom type a function? Because that represents how we _use_ the type T.
    // Normally, PhantomData<T> is only Send/Sync if T is, because PhantomData is indicating that while we don't _name_ T in a field, we should act as though we have a field of type T.
    // However, we don't store any T values in this, we only RETURN them.
    // That means we use a function pointer to indicate that we don't actually operate on T directly, we just return it.
    // Because of this, PhantomData<fn() -> T> is Send/Sync even if T isn't (see https://doc.rust-lang.org/stable/nomicon/phantom-data.html#table-of-phantomdata-patterns)
    phantom: std::marker::PhantomData<fn() -> T>,
}

impl<T: DeserializeOwned + Send + 'static> QueryExecutor<T> {
    pub fn new(
        http_pipeline: CosmosPipeline,
        container_link: ResourceLink,
        query: Query,
        options: QueryOptions<'_>,
        query_engine: QueryEngineRef,
    ) -> azure_core::Result<Self> {
        let items_link = container_link.feed(ResourceType::Items);
        let context = options.method_options.context.into_owned();
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

    pub fn into_stream(self) -> azure_core::Result<FeedPager<T>> {
        Ok(FeedPager::from_stream(futures::stream::try_unfold(
            self,
            |mut state| async move {
                let val = state.step().await?;
                Ok(val.map(|item| (item, state)))
            },
        )))
    }

    /// Executes a single step of the query execution.
    ///
    /// # Returns
    ///
    /// An item to yield, or None if execution is complete.
    #[tracing::instrument(skip_all)]
    async fn step(&mut self) -> azure_core::Result<Option<FeedPage<T>>> {
        let (pipeline, base_request) = match self.pipeline.as_mut() {
            Some(pipeline) => (
                pipeline,
                self.base_request
                    .as_ref()
                    .expect("base_request should be set when pipeline is set"),
            ),
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
                self.query.text = pipeline.query().into();
                self.base_request = Some(crate::pipeline::create_base_query_request(
                    self.http_pipeline.url(&self.items_link),
                    &self.query,
                )?);
                self.pipeline = Some(pipeline);
                (
                    self.pipeline.as_mut().unwrap(),
                    self.base_request.as_ref().unwrap(),
                )
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
                    crate::pipeline::create_base_query_request(
                        self.http_pipeline.url(&self.items_link),
                        &query,
                    )?
                } else {
                    base_request.clone()
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
                        request_index: request.index,
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

// This isn't an inherent method on QueryExecutor because that would force the whole executor to be Sync, which would force the pipeline to be Sync.
#[tracing::instrument(skip_all)]
async fn get_query_plan(
    http_pipeline: &CosmosPipeline,
    items_link: &ResourceLink,
    context: Context<'_>,
    query: &Query,
    supported_features: &str,
) -> azure_core::Result<RawResponse> {
    let url = http_pipeline.url(items_link);
    let mut request = pipeline::create_base_query_request(url, query)?;
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

// This isn't an inherent method on QueryExecutor because that would force the whole executor to be Sync, which would force the pipeline to be Sync.
#[tracing::instrument(skip_all)]
async fn get_pkranges(
    http_pipeline: &CosmosPipeline,
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
