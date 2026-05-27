// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Backend query plan models.
//!
//! These types model the response from the Cosmos DB Gateway when issuing a
//! query plan request (`OperationType::QueryPlan`). The planner uses them to
//! determine partition targeting, detect unsupported query features, and build
//! the dataflow pipeline.

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

/// Deserializes a boolean from either a JSON boolean (`true`/`false`) or
/// an integer (`0`/`1`). The native QueryPlanInterop library serializes
/// booleans as integers, while the Gateway uses standard JSON booleans.
fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;

    struct BoolVisitor;

    impl<'de> de::Visitor<'de> for BoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("a boolean or an integer (0/1)")
        }

        fn visit_bool<E: de::Error>(self, v: bool) -> Result<bool, E> {
            Ok(v)
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> Result<bool, E> {
            Ok(v != 0)
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> Result<bool, E> {
            Ok(v != 0)
        }
    }

    deserializer.deserialize_any(BoolVisitor)
}

/// Deserializes a value as a String. If the JSON value is already a string,
/// it is used directly. Otherwise (array, object, number, etc.), it is
/// serialized back to a JSON string. This handles the difference between
/// Gateway responses (hex EPK strings) and native DLL responses (structured
/// partition key values like arrays).
fn string_or_json<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        other => Ok(other.to_string()),
    }
}

/// The response returned by the Gateway for a query plan request.
#[derive(Debug, Default, Deserialize)]
#[derive(Serialize, PartialEq)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Wire-format fields; not all are consumed today.
pub struct QueryPlan {
    /// The version of the query plan format.
    pub partitioned_query_execution_info_version: usize,

    /// Detailed query information (ordering, aggregates, rewrites, etc.).
    #[serde(default)]
    pub query_info: Option<QueryInfo>,

    /// The EPK ranges that the query references.
    ///
    /// Used by the planner to limit which physical partitions get queried.
    pub query_ranges: Vec<QueryRange>,

    /// Information about hybrid search queries, if applicable.
    pub hybrid_search_query_info: Option<HybridSearchQueryInfo>,
}

/// Information about a hybrid search query.
#[derive(Debug, Deserialize)]
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Wire-format fields; hybrid search isn't fully wired yet.
pub struct HybridSearchQueryInfo {
    /// The query used for global statistics gathering.
    pub global_statistics_query: String,

    /// Individual component queries that make up the hybrid search.
    pub component_query_infos: Vec<QueryInfo>,

    /// Weights assigned to each component query.
    #[serde(default)]
    pub component_weights: Vec<f64>,

    /// Number of results to skip.
    pub skip: Option<u64>,

    /// Number of results to take (always present for hybrid search).
    pub take: Option<u64>,

    /// Whether global statistics are required.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub requires_global_statistics: bool,
}

/// The kind of DISTINCT tracking required by the query.
#[derive(Debug, Deserialize, Default, PartialEq, Eq)]
#[derive(Serialize)]
pub enum DistinctType {
    /// No deduplication required.
    #[default]
    None,

    /// Order-preserving deduplication.
    Ordered,

    /// Order-independent deduplication.
    Unordered,
}

/// Detailed query plan information.
#[derive(Debug, Deserialize, Default)]
#[derive(Serialize, PartialEq)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub struct QueryInfo {
    /// The kind of DISTINCT clause, if any.
    pub distinct_type: DistinctType,

    /// `TOP` clause limit.
    pub top: Option<u64>,

    /// `OFFSET` clause value.
    pub offset: Option<u64>,

    /// `LIMIT` clause value (from `OFFSET`/`LIMIT`).
    pub limit: Option<u64>,

    /// Sort orders for `ORDER BY` expressions.
    pub order_by: Vec<SortOrder>,

    /// Expressions used by `ORDER BY` clauses.
    pub order_by_expressions: Vec<String>,

    /// Expressions used by `GROUP BY` clauses.
    pub group_by_expressions: Vec<String>,

    /// Aliases used by `GROUP BY` clauses.
    pub group_by_aliases: Vec<String>,

    /// Aggregates used in the `SELECT` portion of a `GROUP BY` query.
    pub aggregates: Vec<String>,

    /// Mapping from GROUP BY aliases to aggregate types.
    /// Values may be null or empty strings in the native DLL output.
    #[serde(default)]
    pub group_by_alias_to_aggregate_type: HashMap<String, serde_json::Value>,

    /// Rewritten form of the query for single-partition sub-queries.
    ///
    /// When non-empty, this should be used instead of the original query text
    /// for individual partition requests. The native DLL may return null.
    #[serde(default)]
    pub rewritten_query: Option<String>,

    /// Whether the query contains a `SELECT VALUE` clause.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub has_select_value: bool,

    /// Whether the query contains a non-streaming `ORDER BY`.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub has_non_streaming_order_by: bool,
}

/// Sort order for an `ORDER BY` expression.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[derive(Serialize)]
pub enum SortOrder {
    Ascending,
    Descending,
}

/// An EPK range covered by the query.
#[derive(Debug, Deserialize)]
#[derive(Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Inclusivity flags are wire-format; planner treats ranges uniformly.
pub struct QueryRange {
    /// The minimum EPK value.
    #[serde(deserialize_with = "string_or_json")]
    pub min: String,

    /// The maximum EPK value.
    #[serde(deserialize_with = "string_or_json")]
    pub max: String,

    /// Whether the minimum value is inclusive.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub is_min_inclusive: bool,

    /// Whether the maximum value is inclusive.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub is_max_inclusive: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_minimal_query_plan() {
        let json = r#"{
            "partitionedQueryExecutionInfoVersion": 1,
            "queryRanges": [
                {
                    "min": "",
                    "max": "FF",
                    "isMinInclusive": true,
                    "isMaxInclusive": false
                }
            ]
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.partitioned_query_execution_info_version, 1);
        assert!(plan.query_info.is_none());
        assert!(plan.hybrid_search_query_info.is_none());
        assert_eq!(plan.query_ranges.len(), 1);
        assert_eq!(plan.query_ranges[0].min, "");
        assert_eq!(plan.query_ranges[0].max, "FF");
        assert!(plan.query_ranges[0].is_min_inclusive);
        assert!(!plan.query_ranges[0].is_max_inclusive);
    }

    #[test]
    fn deserializes_query_plan_with_order_by() {
        let json = r#"{
            "partitionedQueryExecutionInfoVersion": 2,
            "queryInfo": {
                "orderBy": ["Ascending", "Descending"],
                "orderByExpressions": ["c.name", "c.age"],
                "rewrittenQuery": "SELECT c.name, c.age FROM c ORDER BY c.name ASC, c.age DESC"
            },
            "queryRanges": []
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        let info = plan.query_info.unwrap();
        assert_eq!(
            info.order_by,
            vec![SortOrder::Ascending, SortOrder::Descending]
        );
        assert_eq!(info.order_by_expressions, vec!["c.name", "c.age"]);
    }

    #[test]
    fn deserializes_query_plan_with_top_and_aggregates() {
        let json = r#"{
            "partitionedQueryExecutionInfoVersion": 1,
            "queryInfo": {
                "top": 10,
                "aggregates": ["Count"],
                "distinctType": "Ordered"
            },
            "queryRanges": []
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        let info = plan.query_info.unwrap();
        assert_eq!(info.top, Some(10));
        assert_eq!(info.aggregates, vec!["Count"]);
        assert_eq!(info.distinct_type, DistinctType::Ordered);
    }

    #[test]
    fn deserializes_query_plan_with_hybrid_search() {
        let json = r#"{
            "partitionedQueryExecutionInfoVersion": 1,
            "queryRanges": [],
            "hybridSearchQueryInfo": {
                "globalStatisticsQuery": "SELECT COUNT(1) FROM c",
                "componentQueryInfos": [],
                "componentWeights": [0.5, 0.5],
                "skip": null,
                "take": 10,
                "requiresGlobalStatistics": true
            }
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        let hybrid = plan.hybrid_search_query_info.unwrap();
        assert_eq!(hybrid.global_statistics_query, "SELECT COUNT(1) FROM c");
        assert_eq!(hybrid.component_weights, vec![0.5, 0.5]);
        assert_eq!(hybrid.take, Some(10));
        assert!(hybrid.requires_global_statistics);
    }

    #[test]
    fn deserializes_query_plan_with_offset_limit() {
        let json = r#"{
            "partitionedQueryExecutionInfoVersion": 1,
            "queryInfo": {
                "offset": 5,
                "limit": 20
            },
            "queryRanges": []
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        let info = plan.query_info.unwrap();
        assert_eq!(info.offset, Some(5));
        assert_eq!(info.limit, Some(20));
    }

    #[test]
    fn deserializes_multiple_query_ranges() {
        let json = r#"{
            "partitionedQueryExecutionInfoVersion": 1,
            "queryRanges": [
                { "min": "", "max": "40", "isMinInclusive": true, "isMaxInclusive": false },
                { "min": "80", "max": "FF", "isMinInclusive": true, "isMaxInclusive": false }
            ]
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.query_ranges.len(), 2);
        assert_eq!(plan.query_ranges[0].max, "40");
        assert_eq!(plan.query_ranges[1].min, "80");
    }
}
