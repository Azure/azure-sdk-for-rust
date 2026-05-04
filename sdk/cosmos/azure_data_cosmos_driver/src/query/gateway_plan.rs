// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway query plan response envelope.
//!
//! Deserializes the JSON response from the Cosmos DB Gateway's query plan endpoint
//! (`x-ms-cosmos-is-query-plan-request: True`). The structural `queryInfo` field
//! uses the unified [`QueryInfo`](super::plan::QueryInfo) type shared with
//! the local query plan generator.

use serde::Deserialize;

use super::plan::QueryInfo;

/// Top-level response from the Gateway query plan endpoint.
///
/// Mirrors the .NET SDK's `PartitionedQueryExecutionInfo` type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GatewayQueryPlan {
    /// Version of the query plan format.
    #[serde(default)]
    pub(crate) partitioned_query_execution_info_version: i32,

    /// Structural information about the query (unified type).
    pub(crate) query_info: QueryInfo,

    /// Effective partition key ranges the query targets.
    #[serde(default)]
    pub(crate) query_ranges: Vec<GatewayQueryRange>,
}

/// An effective partition key range from the Gateway response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GatewayQueryRange {
    /// Minimum effective partition key (inclusive).
    #[serde(default)]
    pub(crate) min: String,

    /// Maximum effective partition key (exclusive).
    #[serde(default)]
    pub(crate) max: String,

    /// Whether the minimum is inclusive.
    #[serde(default = "default_true")]
    pub(crate) is_min_inclusive: bool,

    /// Whether the maximum is exclusive.
    #[serde(default = "default_false")]
    pub(crate) is_max_inclusive: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}
