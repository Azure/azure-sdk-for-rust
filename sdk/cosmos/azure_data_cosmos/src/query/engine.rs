/// Represents a request from the query pipeline for data from a specific partition key range.
pub struct QueryRequest {
    /// The ID of the partition key range to query.
    pub partition_key_range_id: String,

    /// The continuation to use, if any.
    pub continuation: Option<String>,
}

/// The request of a single-partition query for a specific partition key range.
pub struct QueryResult<'a> {
    pub partition_key_range_id: &'a str,
    pub next_continuation: Option<String>,
    pub result: &'a [u8],
}

/// The result of running a single turn of the query pipeline.
pub struct PipelineResult {
    /// Indicates if the pipeline is complete.
    pub is_completed: bool,

    /// The items yielded by the pipeline.
    pub items: Vec<Vec<u8>>,

    /// Additional requests that must be made before the pipeline can continue.
    pub requests: Vec<QueryRequest>,
}

/// Provides an interface to a query pipeline, which aggregates data from multiple single partition queries into a single cross-partition result set.
pub trait QueryPipeline {
    /// The query to be executed, which may have been modified by the gateway when generating a query plan.
    fn query(&self) -> &str;

    /// Indicates if the pipeline is complete.
    fn complete(&self) -> bool;

    /// Runs a single turn of the pipeline, returning the result of that turn.
    fn run(&mut self) -> azure_core::Result<PipelineResult>;

    /// Provides additional single-partition data to the pipeline.
    fn provide_data(&mut self, data: QueryResult) -> azure_core::Result<()>;
}

/// Provides an interface to a query engine, which constructs query pipelines.
pub trait QueryEngine {
    /// Creates a new query pipeline for the given query, plan, and partition key ranges.
    ///
    /// ## Arguments
    /// * `query` - The query to be executed.
    /// * `plan` - The JSON-encoded query plan describing the query (usually provided by the gateway).
    /// * `pkranges` - The JSON-encoded partition key ranges to be queried (usually provided by the gateway).
    fn create_pipeline(
        &self,
        query: &str,
        plan: &[u8],
        pkranges: &[u8],
    ) -> azure_core::Result<Box<dyn QueryPipeline>>;

    /// Gets a comma-separates list of features supported by this query engine, suitable for use in the 'x-ms-cosmos-supported-query-features' header when requesting a query plan.
    fn supported_features(&self) -> azure_core::Result<&str>;
}
