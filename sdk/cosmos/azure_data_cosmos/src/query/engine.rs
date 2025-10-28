/// Represents a request from the query pipeline for data from a specific partition key range.
pub struct QueryRequest {
    /// The ID of the partition key range to query.
    pub partition_key_range_id: String,

    /// The index of this request, within the partition key range.
    ///
    /// This value will always increase for subsequent requests for the same partition key range.
    /// It must be provided back to the pipeline when providing data, so that the pipeline can ensure that data is provided in order.
    pub index: usize,

    /// The continuation to use, if any.
    pub continuation: Option<String>,

    /// The query to execute for this partition key range, if different from the original query.
    pub query: Option<String>,

    /// If a query is specified, this flag indicates if the query parameters should be included with that query.
    ///
    /// Sometimes, when an override query is specified, it differs in structure from the original query, and the original parameters are not valid.
    pub include_parameters: bool,

    /// If specified, indicates that the SDK should IMMEDIATELY drain all remaining results from this partition key range, following continuation tokens, until no more results are available.
    /// All the data from this partition key range should be provided BEFORE any new items will be made available.
    ///
    /// This allows engines to optimize for non-streaming scenarios, where the entire result set must be provided to the engine before it can make progress.
    pub drain: bool,
}

/// The request of a single-partition query for a specific partition key range.
pub struct QueryResult<'a> {
    /// The ID of the partition key range that was queried.
    pub partition_key_range_id: &'a str,

    /// The index of the [`QueryRequest`] that generated this result.
    pub request_index: usize,

    /// The continuation token to be used for the next request, if any.
    pub next_continuation: Option<String>,

    /// The raw body of the response from the query.
    pub result: &'a [u8],
}

/// The result of running a single turn of the query pipeline.
pub struct PipelineResult {
    /// Indicates if the pipeline is complete.
    pub is_completed: bool,

    /// The items yielded by the pipeline.
    pub items: Vec<Box<serde_json::value::RawValue>>,

    /// Additional requests that must be made before the pipeline can continue.
    pub requests: Vec<QueryRequest>,
}

/// Provides an interface to a query pipeline, which aggregates data from multiple single partition queries into a single cross-partition result set.
pub trait QueryPipeline: Send {
    /// The query to be executed, which may have been modified by the gateway when generating a query plan.
    fn query(&self) -> &str;

    /// Indicates if the pipeline is complete.
    fn complete(&self) -> bool;

    /// Runs a single turn of the pipeline, returning the result of that turn.
    fn run(&mut self) -> azure_core::Result<PipelineResult>;

    /// Provides additional single-partition data to the pipeline.
    ///
    /// Data from multiple partition ranges may be provided at once.
    /// However, each page of data must be provided in order.
    /// So, for any given partition key range, page n's results must be earlier in the `data` vector than page n+1's results.
    /// Data from different partition key ranges may be interleaved, as long as each partition key range's pages are in order.
    ///
    /// The pipeline will use the [`QueryResult::request_index`] field to validate this.
    ///
    /// When providing data from a draining request (i.e. a request with `drain = true`), all pages for that draining request can share the same [`QueryResult::request_index`].
    fn provide_data(&mut self, data: Vec<QueryResult>) -> azure_core::Result<()>;
}

/// Provides an interface to a query engine, which constructs query pipelines.
///
/// ## Thread Safety
///
/// A [`QueryEngine`] must be [`Send`] and [`Sync`], as it may be shared across multiple threads.
/// However, the individual [`QueryPipeline`] created by the engine do not need to be thread-safe.
pub trait QueryEngine {
    /// Creates a new query pipeline for the given query, plan, and partition key ranges.
    ///
    /// ## Arguments
    /// * `query` - The query to be executed.
    /// * `plan` - The JSON-encoded query plan describing the query (usually provided by the gateway).
    /// * `pkranges` - The JSON-encoded partition key ranges to be queried (usually provided by the gateway).
    ///
    /// ## Shared Access
    ///
    /// A [`QueryEngine`] may be shared across multiple Cosmos Clients and multiple threads.
    /// As a result, this method accepts an immutable reference to the query engine.
    /// It is the responsibility of the query engine to ensure that the process of creating a query pipeline is thread-safe.
    /// However, a [`QueryPipeline`] need not be thread-safe, and is owned by a single query execution.
    fn create_pipeline(
        &self,
        query: &str,
        plan: &[u8],
        pkranges: &[u8],
    ) -> azure_core::Result<OwnedQueryPipeline>;

    /// Gets a comma-separates list of features supported by this query engine, suitable for use in the `x-ms-cosmos-supported-query-features` header when requesting a query plan.
    fn supported_features(&self) -> azure_core::Result<&str>;
}

#[cfg(target_arch = "wasm32")]
pub(crate) type OwnedQueryPipeline = Box<dyn QueryPipeline>;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type OwnedQueryPipeline = Box<dyn QueryPipeline + Send>;

#[cfg(target_arch = "wasm32")]
pub(crate) type QueryEngineRef = std::sync::Arc<dyn QueryEngine>;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type QueryEngineRef = std::sync::Arc<dyn QueryEngine + Send + Sync>;
