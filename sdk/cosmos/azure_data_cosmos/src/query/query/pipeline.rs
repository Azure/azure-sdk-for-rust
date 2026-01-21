// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::ffi::CStr;

use crate::{
    query::{
        node::AggregatePipelineNode, plan::HybridSearchQueryInfo, query_result::QueryResultShape,
        ItemIdentity, QueryChunk, QueryInfo,
    },
    ErrorKind,
};

use crate::hash::PartitionKeyKind;

use super::{
    node::{LimitPipelineNode, OffsetPipelineNode, PipelineNode, PipelineSlice},
    plan::{DistinctType, QueryRange},
    producer::ItemProducer,
    PartitionKeyRange, PipelineResponse, QueryFeature, QueryPlan,
};

/// Holds a list of [`QueryFeature`]s and a string representation suitable for being passed to the gateway when requesting a query plan.
pub struct SupportedFeatures {
    #[allow(dead_code)]
    supported_features: &'static [QueryFeature],
    supported_features_cstr: &'static CStr,
}

impl SupportedFeatures {
    /// Gets a slice of [`QueryFeature`] values representing the features supported by this engine.
    pub const fn as_slice(&self) -> &'static [QueryFeature] {
        self.supported_features
    }

    /// Gets a Rust string representing the supported features, suitable for being passed to the gateway when requesting a query plan.
    pub const fn as_str(&self) -> &'static str {
        match self.supported_features_cstr.to_str() {
            Ok(s) => s,
            Err(_) => panic!("supported_features_cstr is not valid UTF-8"),
        }
    }

    /// Gets a C string representing the supported features, suitable for being passed to the gateway when requesting a query plan.
    pub const fn as_cstr(&self) -> &'static CStr {
        self.supported_features_cstr
    }
}

macro_rules! supported_features {
    ($($feature:ident,)*) => {
        #[doc = "A [`SupportedFeatures`](SupportedFeatures) describing the features supported by this query engine."]
        pub const SUPPORTED_FEATURES: SupportedFeatures = SupportedFeatures {
            supported_features: &[$(QueryFeature::$feature),*],
            supported_features_cstr: make_cstr!(concat!($(
                stringify!($feature), ","
            ),*)),
        };
    };
}

supported_features!(
    OffsetAndLimit,
    OrderBy,
    MultipleOrderBy,
    Top,
    NonStreamingOrderBy,
    Aggregate,
    HybridSearch,
);

/// Represents a query pipeline capable of accepting single-partition results for a query and returning a cross-partition stream of results.
///
/// ## Overview
///
/// The [`QueryPipeline`] is the core of the Cosmos Client Engine's query engine.
/// To perform a cross-partition query, a client has to perform separate queries against each individual partition, then aggregate the results.
/// This aggregation process is non-trivial, it requires processing the incoming data and handling any `ORDER BY`, `GROUP BY`, etc. clauses to ensure accurate results.
///
/// For example, consider the query `SELECT * FROM c ORDER BY c.foo`, where `foo` is not the partition key.
/// To execute this query correctly, a client must:
///
/// 1. Parse the query into a query plan, to identify that it contains an `ORDER BY` operation and what property is being ordered.
/// 2. Fetch the list of Partition Key Ranges (PK Ranges) for the container.
/// 3. Execute the query separately against each PK Range, retrieving a set of _single-partition_ results that are each correctly ordered.
/// 4. Merge the separate single-partition result streams into a single stream, respecting the ordering as you go.
///
/// The first stage, parsing the query into a query plan, can be performed using the Gateway REST API.
/// Issuing a query request with the `x-ms-cosmos-is-query-plan-request` header set to `true` will cause the Gateway to return a query plan in JSON form.
/// The [`QueryPlan`] type can be deserialized from this type.
///
/// The second stage, fetching PK Ranges, can be performed using a call to the `/dbs/{dbname}/colls/{containername}/pkranges` REST API.
/// The [`PartitionKeyRange`] type can be deserialized from each returned PK Range.
///
/// The third stage can be performed by the per-language client, by executing the query using the Gateway REST API and specifying the `x-ms-documentdb-partitionkeyrangeid` header.
/// The response to this request will be the single-partition results for the query.
///
/// The fourth stage is what the [`QueryPipeline`] handles.
/// The pipeline accepts the query plan and partition key ranges as input.
/// This allows the pipeline to set up the state for tracking results from each individual partitions.
///
/// From there, the pipeline operates in "turns".
/// The language binding executes a turn by calling [`QueryPipeline::run`], which returns a [`PipelineResponse`] describing how to proceed.
/// See the documentation for [`QueryPipeline::run`] for more information on turns.
///
/// ## Query Rewriting
///
/// While the language binding has the original query provided by the user, the Gateway may rewrite it while generating a query plan.
/// The [`QueryInfo::rewritten_query`](crate::query::QueryInfo::rewritten_query) value, included in the query plan returned by the Gateway, includes that rewritten query.
/// Since most consumers of the pipeline don't actually parse the the query plan (instead, they pass the plan in as a string), the
/// pipeline exposes the rewritten query through the [`QueryPipeline::query()`] method.
/// If the query was *not* rewritten by the gateway, this method returns the unrewritten query,
/// so language bindings should *always* use this query when making the signal-partition queries.
pub struct QueryPipeline {
    query: Option<String>,
    pipeline: Vec<Box<dyn PipelineNode>>,
    producer: ItemProducer,

    // Indicates if the pipeline has been terminated early.
    terminated: bool,
}

impl std::fmt::Debug for QueryPipeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryPipeline")
            .field("query", &self.query)
            .field("pipeline", &self.pipeline)
            .field("producer", &self.producer)
            .field("terminated", &self.terminated)
            .finish()
    }
}

impl QueryPipeline {
    /// Creates a new query pipeline.
    ///
    /// # Parameters
    /// * `query` - The ORIGINAL query specified by the user. If the [`QueryPlan`] has a `rewritten_query`, the pipeline will handle rewriting it.
    /// * `plan` - The query plan that describes how to execute the query.
    /// * `pkranges` - An iterator that produces the [`PartitionKeyRange`]s that the query will be executed against.
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn new(
        query: &str,
        plan: QueryPlan,
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
    ) -> crate::Result<Self> {
        let mut pkranges: Vec<PartitionKeyRange> = pkranges.into_iter().collect();
        get_overlapping_pk_ranges(&mut pkranges, &plan.query_ranges);

        tracing::trace!(?query, ?plan, "creating query pipeline");

        let pipeline = if let Some(hybrid_search_query_info) = plan.hybrid_search_query_info {
            // This is a hybrid search query, which requires special handling.
            Self::from_hybrid_search_query_info(hybrid_search_query_info, pkranges)?
        } else if let Some(query_info) = plan.query_info {
            Self::from_query_info(query, query_info, pkranges)?
        } else {
            return Err(ErrorKind::UnsupportedQueryPlan.with_message(
                "query plan is missing both hybrid_search_query_info and query_info sections",
            ));
        };

        tracing::debug!(pipeline = ?pipeline, "created query pipeline");

        Ok(pipeline)
    }

    fn from_hybrid_search_query_info(
        hybrid_search_query_info: HybridSearchQueryInfo,
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
    ) -> crate::Result<Self> {
        let producer = ItemProducer::hybrid(pkranges, hybrid_search_query_info)?;

        // A hybrid search has no pipeline nodes, so we can just leave that empty.
        Ok(Self {
            query: None, // The original query isn't relevant.
            pipeline: Vec::new(),
            producer,
            terminated: false,
        })
    }

    fn from_query_info(
        query: &str,
        query_info: QueryInfo,
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
    ) -> crate::Result<Self> {
        // We don't support non-value aggregates, so make sure the query doesn't have any.
        if !query_info.aggregates.is_empty() && !query_info.has_select_value {
            return Err(ErrorKind::UnsupportedQueryPlan
                .with_message("non-value aggregates are not supported"));
        }

        if !query_info.aggregates.is_empty() && !query_info.order_by.is_empty() {
            return Err(ErrorKind::UnsupportedQueryPlan
                .with_message("queries with both ORDER BY and aggregates are not supported"));
        }

        let producer = if query_info.order_by.is_empty() {
            tracing::debug!("using unordered pipeline");
            // Determine the shape for unordered queries
            let result_shape = if !query_info.aggregates.is_empty() {
                QueryResultShape::ValueAggregate
            } else {
                QueryResultShape::RawPayload
            };
            ItemProducer::unordered(pkranges, result_shape)
        } else {
            if query_info.has_non_streaming_order_by {
                tracing::debug!(?query_info.order_by, "using non-streaming ORDER BY pipeline");
                ItemProducer::non_streaming(pkranges, query_info.order_by)
            } else {
                // We can stream results, there's no vector or full-text search in the query.
                tracing::debug!(?query_info.order_by, "using streaming ORDER BY pipeline");
                ItemProducer::streaming(pkranges, query_info.order_by)
            }
        };

        // We are building the pipeline outside-in.
        // That means the first node we push will be the first node executed.
        // This is relevant for nodes like OFFSET and LIMIT, which need to be ordered carefully.
        let mut pipeline: Vec<Box<dyn PipelineNode>> = Vec::new();

        // We have to do limiting at right at the outside of the pipeline, so that OFFSET can skip items without affecting the LIMIT counter.
        if let Some(limit) = query_info.limit {
            tracing::debug!(limit, "adding LIMIT node to pipeline");
            pipeline.push(Box::new(LimitPipelineNode::new(limit)));
        }

        if let Some(top) = query_info.top {
            tracing::debug!(top, "adding TOP node to pipeline");
            pipeline.push(Box::new(LimitPipelineNode::new(top)));
        }

        if let Some(offset) = query_info.offset {
            tracing::debug!(offset, "adding OFFSET node to pipeline");
            pipeline.push(Box::new(OffsetPipelineNode::new(offset)));
        }

        if !query_info.aggregates.is_empty() {
            pipeline.push(Box::new(AggregatePipelineNode::from_names(
                query_info.aggregates.clone(),
            )?));
        }

        if !query_info.group_by_expressions.is_empty()
            || !query_info.group_by_alias_to_aggregate_type.is_empty()
            || !query_info.group_by_aliases.is_empty()
        {
            return Err(
                ErrorKind::UnsupportedQueryPlan.with_message("GROUP BY queries are not supported")
            );
        }

        if query_info.distinct_type != DistinctType::None {
            return Err(
                ErrorKind::UnsupportedQueryPlan.with_message("DISTINCT queries are not supported")
            );
        }

        let query = if query_info.rewritten_query.is_empty() {
            Some(query.to_string())
        } else {
            let rewritten = format_query(&query_info.rewritten_query);
            tracing::debug!(
                original = ?query,
                ?rewritten,
                "rewrote query, per gateway query plan"
            );
            Some(rewritten)
        };

        Ok(Self {
            query,
            pipeline,
            producer,
            terminated: false,
        })
    }

    /// Retrieves the, possibly rewritten, query that this pipeline is executing.
    ///
    /// The pipeline has both the original query, AND the query plan that may have rewritten it.
    /// So, no matter whether or not the query was rewritten, this query will be accurate.
    pub fn query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// Indicates if the pipeline has been completed.
    pub fn complete(&self) -> bool {
        self.terminated
    }

    /// Provides more data for the specified partition key range.
    #[tracing::instrument(level = "debug", skip_all, err, fields(request_id, pkrange_id, data_len = data.len(), continuation = continuation.as_deref()))]
    pub fn provide_data(
        &mut self,
        pkrange_id: &str,
        request_id: u64,
        data: &[u8],
        continuation: Option<String>,
    ) -> crate::Result<()> {
        self.producer
            .provide_data(pkrange_id, request_id, data, continuation)
    }

    /// Advances the pipeline to the next batch of results.
    ///
    /// This method will return a [`PipelineResponse`] that describes the next action to take.
    ///
    /// 1. A list of results yielded by that turn, if any.
    /// 2. A list of requests for additional data from certain partitions, if any.
    ///
    /// The results provided represent the next set of results to be returned to the user.
    /// The language binding can return these to the user immediately.
    ///
    /// The requests provided describe any additional single-partition queries that must be completed in order to get more data.
    /// The language binding MUST perform ALL the provided requests before the pipeline will be able to yield additional results.
    /// The language binding MAY execute additional turns without having satisfied all the requests, and the pipeline will continue
    /// to return any requests that still need to be made.
    ///
    /// If the pipeline returns no items and no requests, then the query has completed and there are no further results to return.
    #[tracing::instrument(level = "debug", skip(self), err)]
    pub fn run(&mut self) -> crate::Result<PipelineResponse> {
        if self.terminated {
            return Ok(PipelineResponse::TERMINATED);
        }

        let mut slice = PipelineSlice::new(&mut self.pipeline, &mut self.producer);

        let mut items = Vec::new();
        while !self.terminated {
            let result = slice.run()?;

            // Termination MUST come from the pipeline, to ensure aggregates (which can only be emitted after all data is processed) work correctly.
            if result.terminated {
                tracing::trace!("pipeline node terminated the pipeline");
                self.terminated = true;
            }

            if let Some(item) = result.value {
                let payload = item.into_payload().ok_or_else(|| {
                    ErrorKind::InternalError
                        .with_message("items yielded by the pipeline must have a payload")
                })?;
                items.push(payload);
            } else {
                // The pipeline has finished for now, but we're not terminated yet.
                break;
            }
        }

        // For ReadMany, we aggregate all the requests from the available producers in order to parallelize them.
        let requests = self.producer.data_requests()?;

        Ok(PipelineResponse {
            items,
            requests,
            terminated: self.terminated,
        })
    }

    /// Creates a new read many pipeline.
    ///
    /// # Parameters
    /// * `item_identities` - An iterator that produces the [`ItemIdentity`]s to be read.
    /// * `pkranges` - An iterator that produces the [`PartitionKeyRange`]s for the container to use for query generation.
    /// * `pk_kind` - The partition key kind.
    /// * `pk_version` - The partition key version.
    #[tracing::instrument(level = "debug", skip_all, err)]
    pub fn for_read_many(
        item_identities: impl IntoIterator<Item = ItemIdentity>,
        pkranges: impl IntoIterator<Item = PartitionKeyRange>,
        pk_kind: PartitionKeyKind,
        pk_version: u8,
        pk_paths: Vec<String>,
    ) -> crate::Result<Self> {
        let mut pkranges: Vec<PartitionKeyRange> = pkranges.into_iter().collect();
        // Grab item identities and start grouping them by partition key range.
        // Output should be a list of tuples of (pkrangeid, query_string) to go over and generate item producers for.
        let item_identities: Vec<ItemIdentity> = item_identities.into_iter().collect();
        // Create query chunks from the partitioned items, splitting total list into 1000 max item queries.
        // Each chunk is represented as vector of mappings of partition key range IDs to lists of tuples containing the original index, item ID, and partition key value.
        let query_chunks =
            QueryChunk::from_identities(item_identities, &mut pkranges, pk_kind, pk_version);
        // Create the item producer for read many.
        let producer = ItemProducer::read_many(query_chunks, pk_paths);
        let pipeline = Vec::new();
        Ok(Self {
            query: None,
            pipeline,
            producer,
            terminated: false,
        })
    }
}

/// Rewrites the incoming query by replacing tokens within it.
fn format_query(original: &str) -> String {
    original.replace("{documentdb-formattableorderbyquery-filter}", "true")
}

/// Filters the partition key ranges to include only those that overlap with the query ranges.
/// If no query ranges are provided, all partition key ranges are retained.
pub fn get_overlapping_pk_ranges(
    pkranges: &mut Vec<PartitionKeyRange>,
    query_ranges: &[QueryRange],
) {
    if query_ranges.is_empty() {
        return;
    }

    debug_assert!(
        pkranges.is_sorted_by_key(|pkrange| pkrange.min_inclusive.clone()),
        "partition key ranges must be sorted by minInclusive"
    );

    debug_assert!(
        query_ranges.is_sorted_by_key(|query_range| query_range.min.clone()),
        "query ranges must be sorted by min"
    );

    // Walks through both lists, finding overlaps.
    // We produce the final list by swapping overlapping ranges to the front of the list and then truncating the list to remove the remaining, non-overlapping, ranges.
    let mut write_idx = 0;
    let mut query_idx = 0;
    for read_idx in 0..pkranges.len() {
        let pkrange = &pkranges[read_idx];

        // Advance query_idx to skip query ranges that end before this pkrange starts
        while query_idx < query_ranges.len() {
            let query_range = &query_ranges[query_idx];
            if query_range.max < pkrange.min_inclusive
                || (query_range.max == pkrange.min_inclusive && !query_range.is_max_inclusive)
            {
                query_idx += 1;
            } else {
                break;
            }
        }

        // Check if any remaining query ranges overlap with this pkrange
        let mut found_overlap = false;
        for query_range in &query_ranges[query_idx..] {
            // If this query range starts after the pkrange ends, no more overlaps possible
            if query_range.min >= pkrange.max_exclusive {
                break;
            }

            // Check for actual overlap using simplified logic
            if pkrange_overlaps_query_range(pkrange, query_range) {
                found_overlap = true;
                break;
            }
        }

        if found_overlap {
            if write_idx != read_idx {
                pkranges.swap(write_idx, read_idx);
            }
            write_idx += 1;
        }
    }

    pkranges.truncate(write_idx);
}

/// Determines if a partition key range overlaps with a query range.
/// PartitionKeyRange is always [min_inclusive, max_exclusive).
fn pkrange_overlaps_query_range(pkrange: &PartitionKeyRange, query_range: &QueryRange) -> bool {
    // Check for non-overlap cases (easier to reason about)

    // PKRange ends before query starts
    if pkrange.max_exclusive < query_range.min {
        return false;
    }
    if pkrange.max_exclusive == query_range.min && !query_range.is_min_inclusive {
        return false;
    }

    // Query ends before PKRange starts
    if query_range.max < pkrange.min_inclusive {
        return false;
    }
    if query_range.max == pkrange.min_inclusive && !query_range.is_max_inclusive {
        return false;
    }

    true
}

// The tests for the pipeline are found in integration tests (in the `tests`) directory, since we want to test an end-to-end experience that matches what the user will see.
// Individual components of the pipeline are tested in the other modules.

#[cfg(test)]
mod tests {
    use super::*;

    fn create_pkrange(id: &str, min: &str, max: &str) -> PartitionKeyRange {
        PartitionKeyRange::new(id, min, max)
    }

    fn create_query_range(
        min: &str,
        max: &str,
        min_inclusive: bool,
        max_inclusive: bool,
    ) -> QueryRange {
        QueryRange {
            min: min.to_string(),
            max: max.to_string(),
            is_min_inclusive: min_inclusive,
            is_max_inclusive: max_inclusive,
        }
    }

    #[test]
    fn test_empty_query_ranges() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
        ];
        let query_ranges = vec![];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 2);
        assert_eq!(pkranges[0].id, "pk1");
        assert_eq!(pkranges[1].id, "pk2");
    }

    #[test]
    fn test_no_overlaps() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "20000000"),
            create_pkrange("pk2", "20000000", "40000000"),
            create_pkrange("pk3", "40000000", "60000000"),
        ];
        let query_ranges = vec![
            create_query_range("70000000", "80000000", true, true),
            create_query_range("90000000", "A0000000", true, true),
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 0);
    }

    #[test]
    fn test_single_exact_overlap() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
            create_pkrange("pk3", "80000000", "C0000000"),
        ];
        let query_ranges = vec![
            create_query_range("40000000", "80000000", true, false), // Exactly matches pk2
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 1);
        assert_eq!(pkranges[0].id, "pk2");
    }

    #[test]
    fn test_multiple_overlaps() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "20000000"),
            create_pkrange("pk2", "20000000", "40000000"),
            create_pkrange("pk3", "40000000", "60000000"),
            create_pkrange("pk4", "60000000", "80000000"),
        ];
        let query_ranges = vec![
            create_query_range("10000000", "30000000", true, true), // Overlaps pk1, pk2
            create_query_range("50000000", "70000000", true, true), // Overlaps pk3, pk4
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 4);
        let ids: Vec<&str> = pkranges.iter().map(|pk| pk.id.as_str()).collect();
        assert_eq!(ids, vec!["pk1", "pk2", "pk3", "pk4"]);
    }

    #[test]
    fn test_partial_overlap_start() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
        ];
        let query_ranges = vec![
            create_query_range("20000000", "60000000", true, true), // Overlaps end of pk1, start of pk2
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 2);
        let ids: Vec<&str> = pkranges.iter().map(|pk| pk.id.as_str()).collect();
        assert_eq!(ids, vec!["pk1", "pk2"]);
    }

    #[test]
    fn test_boundary_conditions_inclusive_exclusive() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
        ];

        // Query range ends exactly where pk2 starts, but pk2 is min-inclusive
        let query_ranges = vec![
            create_query_range("20000000", "40000000", true, true), // max_inclusive=true
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 2); // Should overlap both pk1 and pk2
        let ids: Vec<&str> = pkranges.iter().map(|pk| pk.id.as_str()).collect();
        assert_eq!(ids, vec!["pk1", "pk2"]);
    }

    #[test]
    fn test_boundary_conditions_exclusive() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
        ];

        // Query range ends exactly where pk2 starts, but query is max-exclusive
        let query_ranges = vec![
            create_query_range("20000000", "40000000", true, false), // max_inclusive=false
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 1); // Should only overlap pk1
        assert_eq!(pkranges[0].id, "pk1");
    }

    #[test]
    fn test_query_range_spans_multiple_partitions() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "20000000"),
            create_pkrange("pk2", "20000000", "40000000"),
            create_pkrange("pk3", "40000000", "60000000"),
            create_pkrange("pk4", "60000000", "80000000"),
        ];
        let query_ranges = vec![
            create_query_range("10000000", "70000000", true, true), // Spans pk1 through pk4
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 4);
        let ids: Vec<&str> = pkranges.iter().map(|pk| pk.id.as_str()).collect();
        assert_eq!(ids, vec!["pk1", "pk2", "pk3", "pk4"]);
    }

    #[test]
    fn test_query_range_contained_within_partition() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
            create_pkrange("pk3", "80000000", "C0000000"),
        ];
        let query_ranges = vec![
            create_query_range("50000000", "60000000", true, true), // Entirely within pk2
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 1);
        assert_eq!(pkranges[0].id, "pk2");
    }

    #[test]
    fn test_multiple_query_ranges_different_overlaps() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "20000000"),
            create_pkrange("pk2", "20000000", "40000000"),
            create_pkrange("pk3", "40000000", "60000000"),
            create_pkrange("pk4", "60000000", "80000000"),
            create_pkrange("pk5", "80000000", "A0000000"),
        ];
        let query_ranges = vec![
            create_query_range("10000000", "30000000", true, true), // Overlaps pk1, pk2
            create_query_range("70000000", "90000000", true, true), // Overlaps pk4, pk5
                                                                    // pk3 should not be included
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 4);
        let ids: Vec<&str> = pkranges.iter().map(|pk| pk.id.as_str()).collect();
        assert_eq!(ids, vec!["pk1", "pk2", "pk4", "pk5"]);
    }

    #[test]
    fn test_query_ranges_out_of_order_should_panic() {
        let pkranges = vec![create_pkrange("pk1", "00000000", "40000000")];
        let query_ranges = vec![
            create_query_range("60000000", "80000000", true, true),
            create_query_range("40000000", "50000000", true, true), // Out of order
        ];

        // This should panic in debug mode due to the debug_assert!
        // We'll test this by making a copy to avoid UnwindSafe issues
        let result = std::panic::catch_unwind(|| {
            let mut test_pkranges = pkranges.clone();
            get_overlapping_pk_ranges(&mut test_pkranges, &query_ranges);
        });

        // In debug mode, this should panic. In release mode, it might not.
        if cfg!(debug_assertions) {
            assert!(
                result.is_err(),
                "Should panic in debug mode with unsorted query ranges"
            );
        } else {
            // In release mode, just ensure it doesn't crash
            assert!(result.is_ok(), "Should not crash in release mode");
        }
    }

    #[test]
    fn test_edge_case_single_point_overlap() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
        ];

        // Query range that touches the boundary point
        let query_ranges = vec![
            create_query_range("40000000", "40000000", true, true), // Single point at boundary
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 1);
        assert_eq!(pkranges[0].id, "pk2"); // pk2 includes 40000000, pk1 excludes it
    }

    #[test]
    fn test_query_range_before_all_partitions() {
        let mut pkranges = vec![
            create_pkrange("pk1", "40000000", "80000000"),
            create_pkrange("pk2", "80000000", "C0000000"),
        ];
        let query_ranges = vec![
            create_query_range("00000000", "20000000", true, true), // Before all partitions
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 0);
    }

    #[test]
    fn test_query_range_after_all_partitions() {
        let mut pkranges = vec![
            create_pkrange("pk1", "00000000", "40000000"),
            create_pkrange("pk2", "40000000", "80000000"),
        ];
        let query_ranges = vec![
            create_query_range("A0000000", "C0000000", true, true), // After all partitions
        ];

        get_overlapping_pk_ranges(&mut pkranges, &query_ranges);

        assert_eq!(pkranges.len(), 0);
    }
}
