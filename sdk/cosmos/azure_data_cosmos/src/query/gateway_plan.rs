// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway query plan response model.
//!
//! Deserializes the JSON response from the Cosmos DB Gateway's query plan endpoint
//! (`x-ms-cosmos-is-query-plan-request: True`). The response is a
//! `PartitionedQueryExecutionInfo` envelope containing `queryInfo` and `queryRanges`.

use serde::Deserialize;

/// Top-level response from the Gateway query plan endpoint.
///
/// Mirrors the .NET SDK's `PartitionedQueryExecutionInfo` type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayQueryPlan {
    /// Version of the query plan format.
    #[serde(default)]
    pub partitioned_query_execution_info_version: i32,

    /// Structural information about the query.
    pub query_info: GatewayQueryInfo,

    /// Effective partition key ranges the query targets.
    #[serde(default)]
    pub query_ranges: Vec<GatewayQueryRange>,
}

/// Structural information about a query, as returned by the Gateway.
///
/// Field names match the Gateway's JSON property names exactly.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayQueryInfo {
    /// The kind of DISTINCT: `"None"`, `"Ordered"`, or `"Unordered"`.
    #[serde(default)]
    pub distinct_type: String,

    /// TOP value, if present.
    #[serde(default)]
    pub top: Option<serde_json::Value>,

    /// OFFSET value, if present.
    #[serde(default)]
    pub offset: Option<serde_json::Value>,

    /// LIMIT value, if present.
    #[serde(default)]
    pub limit: Option<serde_json::Value>,

    /// ORDER BY sort orders: `["Ascending"]`, `["Descending"]`, etc.
    #[serde(default)]
    pub order_by: Vec<String>,

    /// ORDER BY expressions as path strings.
    #[serde(default)]
    pub order_by_expressions: Vec<String>,

    /// GROUP BY expressions as path strings.
    #[serde(default)]
    pub group_by_expressions: Vec<String>,

    /// GROUP BY aliases.
    #[serde(default)]
    pub group_by_aliases: Vec<String>,

    /// Aggregate functions used: `["Count"]`, `["Sum"]`, etc.
    #[serde(default)]
    pub aggregates: Vec<String>,

    /// GROUP BY alias to aggregate type mapping.
    #[serde(default)]
    pub group_by_alias_to_aggregate_type: serde_json::Value,

    /// The rewritten query text (if the Gateway rewrites it).
    #[serde(default)]
    pub rewritten_query: Option<String>,

    /// Whether the SELECT clause uses `SELECT VALUE`.
    #[serde(default)]
    pub has_select_value: bool,

    /// Whether the query contains non-streaming ORDER BY.
    #[serde(default)]
    pub has_non_streaming_order_by: bool,

    /// DCount information, if present.
    #[serde(default)]
    pub d_count_info: Option<serde_json::Value>,
}

/// An effective partition key range from the Gateway response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GatewayQueryRange {
    /// Minimum effective partition key (inclusive).
    #[serde(default)]
    pub min: String,

    /// Maximum effective partition key (exclusive).
    #[serde(default)]
    pub max: String,

    /// Whether the minimum is inclusive.
    #[serde(default = "default_true")]
    pub is_min_inclusive: bool,

    /// Whether the maximum is exclusive.
    #[serde(default = "default_false")]
    pub is_max_inclusive: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}
