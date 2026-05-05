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

    /// Whether the maximum is inclusive. Cosmos partition-key ranges are
    /// `[min, max)` by default, so the JSON `isMaxInclusive` defaults to `false`.
    #[serde(default = "default_false")]
    pub(crate) is_max_inclusive: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::plan::{AggregateKind, DistinctType, SortOrder};

    #[test]
    fn deserializes_gateway_query_plan_into_shared_query_info() {
        let plan: GatewayQueryPlan = serde_json::from_value(serde_json::json!({
            "partitionedQueryExecutionInfoVersion": 2,
            "queryInfo": {
                "distinctType": "Ordered",
                "top": 5,
                "offset": 3,
                "limit": 10,
                "orderBy": ["Ascending", "Descending"],
                "orderByExpressions": ["c.city", "c.score"],
                "groupByExpressions": ["c.city"],
                "aggregates": ["Count"],
                "rewrittenQuery": "SELECT VALUE 1",
                "hasSelectValue": true,
                "hasNonStreamingOrderBy": true
            },
            "queryRanges": [
                {
                    "min": "05C1C9CD673398",
                    "max": "FF",
                    "isMinInclusive": false,
                    "isMaxInclusive": true
                }
            ]
        }))
        .unwrap();

        assert_eq!(plan.partitioned_query_execution_info_version, 2);
        assert_eq!(plan.query_info.distinct_type, DistinctType::Ordered);
        assert_eq!(plan.query_info.top, Some(5));
        assert_eq!(plan.query_info.offset, Some(3));
        assert_eq!(plan.query_info.limit, Some(10));
        assert_eq!(
            plan.query_info.order_by,
            vec![SortOrder::Ascending, SortOrder::Descending]
        );
        assert_eq!(
            plan.query_info.order_by_expressions,
            vec!["c.city", "c.score"]
        );
        assert_eq!(plan.query_info.group_by_expressions, vec!["c.city"]);
        assert_eq!(plan.query_info.aggregates, vec![AggregateKind::Count]);
        assert_eq!(
            plan.query_info.rewritten_query.as_deref(),
            Some("SELECT VALUE 1")
        );
        assert!(plan.query_info.has_select_value);
        assert!(plan.query_info.has_non_streaming_order_by);
        assert_eq!(plan.query_ranges.len(), 1);
        assert_eq!(plan.query_ranges[0].min, "05C1C9CD673398");
        assert_eq!(plan.query_ranges[0].max, "FF");
        assert!(!plan.query_ranges[0].is_min_inclusive);
        assert!(plan.query_ranges[0].is_max_inclusive);
    }

    #[test]
    fn gateway_query_range_defaults_match_gateway_contract() {
        let plan: GatewayQueryPlan = serde_json::from_value(serde_json::json!({
            "queryInfo": {},
            "queryRanges": [
                {
                    "min": "A",
                    "max": "B"
                }
            ]
        }))
        .unwrap();

        assert_eq!(plan.partitioned_query_execution_info_version, 0);
        assert_eq!(plan.query_ranges.len(), 1);
        assert_eq!(plan.query_ranges[0].min, "A");
        assert_eq!(plan.query_ranges[0].max, "B");
        assert!(plan.query_ranges[0].is_min_inclusive);
        assert!(!plan.query_ranges[0].is_max_inclusive);
        assert_eq!(plan.query_info, QueryInfo::default());
    }
}
