use azure_core::http::{headers::Headers, Context, Method, RawResponse, Request, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    clients::ContainerClient,
    constants,
    models::PartitionKeyRange,
    pipeline::{self, CosmosPipeline},
    query::{OwnedQueryPipeline, QueryEngineRef, QueryResult},
    resource_context::{ResourceLink, ResourceType},
    FeedPage, FeedPager, Query, QueryOptions,
};

pub struct QueryExecutor<T: DeserializeOwned> {
    container_client: ContainerClient,
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
        container_client: ContainerClient,
        query: Query,
        options: QueryOptions<'_>,
        query_engine: QueryEngineRef,
    ) -> azure_core::Result<Self> {
        let context = options.method_options.context.into_owned();
        Ok(Self {
            container_client,
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
                let query_plan = self
                    .container_client
                    .get_query_plan(
                        Context::with_context(&self.context),
                        &self.query,
                        self.query_engine.supported_features()?,
                    )
                    .await?
                    .into_body()
                    .collect()
                    .await?;
                let pkranges = self
                    .container_client
                    .get_partition_key_ranges(Context::with_context(&self.context))
                    .await?;

                let pipeline =
                    self.query_engine
                        .create_pipeline(&self.query.text, &query_plan, &pkranges)?;
                self.query.text = pipeline.query().into();
                self.base_request = Some(crate::pipeline::create_base_query_request(
                    self.container_client
                        .pipeline
                        .url(&self.container_client.items_link),
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
            for request in results.requests {
                let mut query_request = base_request.clone();
                query_request.insert_header(
                    constants::PARTITION_KEY_RANGE_ID,
                    request.partition_key_range_id.clone(),
                );
                if let Some(continuation) = request.continuation {
                    query_request.insert_header(constants::CONTINUATION, continuation);
                }

                let resp = self
                    .container_client
                    .pipeline
                    .send_raw(
                        Context::with_context(&self.context),
                        &mut query_request,
                        self.container_client.items_link.clone(),
                    )
                    .await?;

                let next_continuation =
                    resp.headers().get_optional_string(&constants::CONTINUATION);
                let body = resp.into_body().collect().await?;

                let result = QueryResult {
                    partition_key_range_id: &request.partition_key_range_id,
                    next_continuation,
                    result: &body,
                };

                pipeline.provide_data(result)?;
            }

            // No items, but we provided more data (probably), so continue the loop.
            // If there were no more requests to make, the pipeline will be marked complete above
        }

        // If we get here, the pipeline is complete and we have no items to return.
        Ok(None)
    }
}
