// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use std::borrow::Cow;
use std::collections::HashMap;

use serde::Deserialize;

mod aggregators;
pub mod node;
mod pipeline;
mod plan;
mod producer;
mod query_result;

#[cfg(feature = "query_engine")]
mod engine;

#[cfg(feature = "query_engine")]
pub use engine::*;

use crate::hash::{get_hashed_partition_key_string, PartitionKeyKind, PartitionKeyValue};
pub use pipeline::{
    get_overlapping_pk_ranges, QueryPipeline, SupportedFeatures, SUPPORTED_FEATURES,
};
pub use plan::{DistinctType, QueryInfo, QueryPlan, QueryRange, SortOrder};
pub use query_result::{QueryClauseItem, QueryResult, QueryResultShape};

/// Features that may be required by the Query Engine.
///
/// The query pipeline provides the language bindings a list of features that it can support, using these values.
/// The language binding can then forward that information to the gateway when generating a query plan, which allows the gateway to reject queries that the engine cannot support.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryFeature {
    None,
    Aggregate,
    CompositeAggregate,
    Distinct,
    GroupBy,
    MultipleAggregates,
    MultipleOrderBy,
    OffsetAndLimit,
    OrderBy,
    Top,
    NonValueAggregate,
    DCount,
    NonStreamingOrderBy,
    ListAndSetAggregate,
    CountIf,
    HybridSearch,
    WeightedRankFusion,
    HybridSearchSkipOrderByRewrite,
}

#[derive(Debug, Clone)]
pub struct Query {
    /// The text of the query.
    pub text: String,

    /// The parameters of the query, pre-encoded as a JSON object suitable to being the `parameters` field of a Cosmos query.
    pub encoded_parameters: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(
    feature = "python_conversions",
    derive(pyo3::FromPyObject),
    pyo3(from_item_all)
)]
#[serde(rename_all = "camelCase")]
pub struct PartitionKeyRange {
    id: String,
    #[cfg_attr(feature = "python_conversions", pyo3(item("minInclusive")))]
    min_inclusive: String,
    #[allow(dead_code)]
    #[cfg_attr(feature = "python_conversions", pyo3(item("maxExclusive")))]
    max_exclusive: String,
}

impl PartitionKeyRange {
    pub fn new(
        id: impl Into<String>,
        min_inclusive: impl Into<String>,
        max_exclusive: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            min_inclusive: min_inclusive.into(),
            max_exclusive: max_exclusive.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
// TODO: pk values here are all currently strings - we need the same sort of PartitionKeyValue
// logic used in the main Rust SDK in order to compare and be able to use it within this method.
pub struct ItemIdentity {
    #[serde(rename = "PartitionKeyValue")]
    partition_key_value: String,
    #[serde(rename = "ID")]
    id: String,
}

impl ItemIdentity {
    pub fn new(id: impl Into<String>, partition_key_value: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            partition_key_value: partition_key_value.into(),
        }
    }
}

#[derive(Debug)]
pub struct QueryChunk {
    pub pk_range_id: String,
    pub items: Vec<QueryChunkItem>,
}

#[derive(Debug)]
pub struct QueryChunkItem {
    pub index: usize,
    pub id: String,
    pub partition_key_value: String,
}

impl QueryChunk {
    pub fn from_identities(
        item_identities: Vec<ItemIdentity>,
        pkranges: &mut Vec<PartitionKeyRange>,
        pk_kind: PartitionKeyKind,
        pk_version: u8,
    ) -> Vec<Self> {
        // Group items by their partition key range ID.
        let items_by_range =
            partition_items_by_range(item_identities, pkranges, pk_kind, pk_version).unwrap();
        // Create query chunks from the partitioned items, splitting total list into 1000 max item queries.
        // Each chunk is represented as vector of mappings of partition key range IDs to lists of tuples containing the original index, item ID, and partition key value.
        let query_chunks = create_query_chunks_from_partitioned_items(&items_by_range);
        query_chunks
    }
}

// Groups items by their partition key range ID efficiently while preserving original order.
// Returns a mapping of partition key range IDs to lists of tuples containing the original index, item ID, and partition key value.
fn partition_items_by_range(
    item_identities: Vec<ItemIdentity>,
    pkranges: &mut Vec<PartitionKeyRange>,
    pk_kind: PartitionKeyKind,
    pk_version: u8,
) -> Result<HashMap<String, Vec<(usize, String, String)>>, Box<dyn std::error::Error>> {
    // TODO: Partition key values here are all currently strings - we need the same sort of PartitionKeyValue
    // logic used in the main Rust SDK in order to compare and be able to use it within this method.
    let mut items_by_partition: HashMap<String, Vec<(usize, String, String)>> = HashMap::new();
    let mut items_by_pk_value: HashMap<String, Vec<(usize, String, String)>> = HashMap::new();

    // Group items by PK value (string or number) - only string for now since we don't have PartitionKeyValue logic yet.
    for (idx, identity) in item_identities.iter().enumerate() {
        let pk_value = identity.partition_key_value.clone(); // PartitionKeyValue is enum { String(String), Number(f64) }
        items_by_pk_value
            .entry(pk_value.clone())
            .or_default()
            .push((idx, identity.id.clone(), pk_value));
    }

    // For each PK group, compute EPK range and find overlapping ranges
    for pk_items in items_by_pk_value.values() {
        let pk_value = &pk_items[0].2;
        // The Go SDK passes in a pk_value JSON string that comes in as "["value"]". We need to
        // get the actual value inside, otherwise hashing fails to get values in the right ranges
        let inner_value = extract_inner_pk_value(pk_value)?;
        let pk_value_val = PartitionKeyValue::String(inner_value.clone());
        // TODO: Also needs PK to be updated here

        let epk_range_string =
            get_hashed_partition_key_string(&[pk_value_val], pk_kind, pk_version)
                .map_err(|e| format!("Failed to compute effective partition key: {}", e))?;
        let epk_range = QueryRange {
            min: epk_range_string.clone(),
            max: epk_range_string,
            is_min_inclusive: true,
            is_max_inclusive: true,
        };
        // Here we have to create a clone because get_overlapping_pk_ranges modifies the input pkranges.
        // For ReadMany we need to keep the full list intact for the next set of items, since a range may be used more than once.
        let mut pkranges_clone = pkranges.clone();
        get_overlapping_pk_ranges(&mut pkranges_clone, &[epk_range]);
        if !pkranges.is_empty() {
            let range_id = pkranges_clone[0].id.clone();
            items_by_partition
                .entry(range_id)
                .or_default()
                .extend(pk_items.clone());
        }
    }
    Ok(items_by_partition)
}

// Creates [`QueryChunk`]s from partitioned items, ensuring no chunk exceeds the maximum item limit.
// Each chunk is represented as vector of partition key range IDs to lists of [`QueryChunkItem`]s containing the original index, item ID, and partition key value.
fn create_query_chunks_from_partitioned_items(
    items_by_partition: &HashMap<String, Vec<(usize, String, String)>>,
) -> Vec<QueryChunk> {
    let mut query_chunks: Vec<QueryChunk> = Vec::new();
    let max_items_per_query = 1000;
    for (partition_id, partition_items) in items_by_partition {
        for chunk_start in (0..partition_items.len()).step_by(max_items_per_query) {
            let chunk_end = (chunk_start + max_items_per_query).min(partition_items.len());
            let chunks = partition_items[chunk_start..chunk_end].to_vec();
            let chunk_items = chunks
                .into_iter()
                .map(|(index, id, partition_key_value)| {
                    // Extract the inner partition key value before creating the QueryChunkItem
                    let inner_pk_value = extract_inner_pk_value(&partition_key_value).unwrap();
                    QueryChunkItem {
                        index,
                        id,
                        partition_key_value: inner_pk_value,
                    }
                })
                .collect();
            query_chunks.push(QueryChunk {
                pk_range_id: partition_id.clone(),
                items: chunk_items,
            });
        }
    }
    query_chunks
}

fn extract_inner_pk_value(raw: &str) -> Result<String, String> {
    // First try: input is a JSON array like ["odd"]
    if let Ok(vec) = serde_json::from_str::<Vec<String>>(raw) {
        if let Some(first) = vec.into_iter().next() {
            return Ok(first);
        }
        return Err(format!("failed to parse empty partition key value: {raw}").into());
    }

    // Second try: input is a JSON string like "odd"
    if let Ok(val) = serde_json::from_str::<String>(raw) {
        return Ok(val);
    }

    // Third try: input is already a plain string value (not JSON-encoded)
    Ok(raw.to_string())
}

/// Describes a request for additional data from the pipeline.
///
/// This value is returned when the pipeline needs more data to continue processing.
/// It contains the information necessary for the caller to make an HTTP request to the Cosmos APIs to fetch the next batch of data.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "python_conversions", derive(pyo3::IntoPyObject))]
pub struct DataRequest {
    /// A unique identifier for this request that can be used to match it with it's response.
    pub id: u64,
    pub pkrange_id: Cow<'static, str>,
    pub continuation: Option<String>,
    pub query: Option<String>,
    pub include_parameters: bool,
}

impl DataRequest {
    pub fn new(
        id: u64,
        pkrange_id: impl Into<Cow<'static, str>>,
        continuation: Option<String>,
        query: Option<String>,
    ) -> Self {
        Self {
            id,
            pkrange_id: pkrange_id.into(),
            continuation,
            query: query,
            include_parameters: true,
        }
    }

    pub fn with_query(
        id: u64,
        pkrange_id: impl Into<Cow<'static, str>>,
        continuation: Option<String>,
        query: impl Into<String>,
        include_parameters: bool,
    ) -> Self {
        Self {
            id,
            pkrange_id: pkrange_id.into(),
            continuation,
            query: Some(query.into()),
            include_parameters,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PipelineResponse {
    /// The items returned by the pipeline.
    pub items: Vec<Box<serde_json::value::RawValue>>,

    /// Requests for additional data from the pipeline.
    ///
    /// If [`PipelineResponse::terminated`] is `true`, this will be empty and can be ignored.
    pub requests: Vec<DataRequest>,

    /// Indicates if the pipeline has terminated.
    ///
    /// If this is true, no further items will be produced, even if more data is provided.
    pub terminated: bool,
}

impl PipelineResponse {
    pub const TERMINATED: Self = Self {
        items: Vec::new(),
        requests: Vec::new(),
        terminated: true,
    };
}
