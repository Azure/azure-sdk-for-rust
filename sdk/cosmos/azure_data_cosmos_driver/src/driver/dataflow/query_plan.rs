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

/// Accepts either a JSON boolean or a JSON integer (0 / 1) and returns a
/// `bool`. The Gateway V2 thin-client proxy serializes several `bool` fields
/// (`hasSelectValue`, `hasNonStreamingOrderBy`, `isMinInclusive`,
/// `isMaxInclusive`) as integers — matching Java's `JsonNode::asBoolean()`
/// behavior — while the standard Gateway returns proper booleans.
fn bool_from_int_or_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, Visitor};
    struct BoolOrInt;
    impl<'de> Visitor<'de> for BoolOrInt {
        type Value = bool;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("bool or integer 0/1")
        }
        fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
            Ok(v)
        }
        fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
            Ok(v != 0)
        }
        fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
            Ok(v != 0)
        }
    }
    deserializer.deserialize_any(BoolOrInt)
}

/// Accepts the proxy's `queryRanges[].min` / `queryRanges[].max` field shapes
/// and returns the canonical EPK string the planner expects.
///
/// The standard Gateway returns hex EPK strings (`""`, `"FF"`, or a specific
/// hash hex). The Gateway V2 thin-client proxy returns the raw
/// `PartitionKeyInternal` JSON shape, which is one of:
/// * empty JSON array `[]` — the minimum EPK sentinel (`MinValue`) → `""`.
/// * JSON array containing the `Infinity` object `[{"type":"Infinity"}]` —
///   the maximum EPK sentinel → `"FF"`.
/// * a bare string `"Infinity"` — same as the `Infinity` sentinel → `"FF"`.
/// * a string already in hex form — passed through unchanged.
///
/// Arrays containing concrete values (e.g. `["foo"]`) would require
/// per-container EPK computation against the partition key definition.
/// That path isn't wired here today: the current cross-partition Rust
/// pipeline only consumes the keyspace boundaries returned for unconstrained
/// `SELECT` queries. A non-sentinel value falls back to an empty EPK so the
/// planner widens to the full topology rather than panicking.
fn epk_string_from_proxy_or_gateway<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Error, SeqAccess, Visitor};
    struct EpkVisitor;
    impl<'de> Visitor<'de> for EpkVisitor {
        type Value = String;
        fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("EPK hex string, the literal \"Infinity\", or a PartitionKeyInternal array")
        }
        fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
            Ok(if v == "Infinity" {
                "FF".to_owned()
            } else {
                v.to_owned()
            })
        }
        fn visit_string<E: Error>(self, v: String) -> Result<Self::Value, E> {
            Ok(if v == "Infinity" { "FF".to_owned() } else { v })
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            // Walk the PartitionKeyInternal array. Empty → MinValue ("").
            // Contains the `Infinity` token → MaxValue ("FF"). Any concrete
            // value or unknown sentinel collapses to "" so the planner
            // widens to all partitions rather than failing.
            let mut saw_infinity = false;
            let mut saw_non_sentinel = false;
            while let Some(elem) = seq.next_element::<serde_json::Value>()? {
                match elem {
                    serde_json::Value::Object(map) => {
                        if let Some(serde_json::Value::String(t)) = map.get("type") {
                            if t == "Infinity" {
                                saw_infinity = true;
                                continue;
                            }
                        }
                        saw_non_sentinel = true;
                    }
                    _ => saw_non_sentinel = true,
                }
            }
            Ok(if saw_infinity && !saw_non_sentinel {
                "FF".to_owned()
            } else {
                String::new()
            })
        }
    }
    deserializer.deserialize_any(EpkVisitor)
}

/// The response returned by the Gateway for a query plan request.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Wire-format fields; not all are consumed today.
pub(crate) struct QueryPlan {
    /// The version of the query plan format.
    #[serde(default)]
    pub partitioned_query_execution_info_version: usize,

    /// Detailed query information (ordering, aggregates, rewrites, etc.).
    #[serde(default)]
    pub query_info: Option<QueryInfo>,

    /// The EPK ranges that the query references.
    ///
    /// Used by the planner to limit which physical partitions get queried.
    #[serde(default)]
    pub query_ranges: Vec<QueryRange>,

    /// Information about hybrid search queries, if applicable.
    pub hybrid_search_query_info: Option<HybridSearchQueryInfo>,
}

/// Information about a hybrid search query.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Wire-format fields; hybrid search isn't fully wired yet.
pub(crate) struct HybridSearchQueryInfo {
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
pub(crate) enum DistinctType {
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
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryInfo {
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
    pub group_by_alias_to_aggregate_type: HashMap<String, String>,

    /// Rewritten form of the query for single-partition sub-queries.
    ///
    /// When non-empty, this should be used instead of the original query text
    /// for individual partition requests.
    pub rewritten_query: String,

    /// Whether the query contains a `SELECT VALUE` clause.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub has_select_value: bool,

    /// Whether the query contains a non-streaming `ORDER BY`.
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub has_non_streaming_order_by: bool,
}

/// Sort order for an `ORDER BY` expression.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SortOrder {
    Ascending,
    Descending,
}

/// An EPK range covered by the query.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Inclusivity flags are wire-format; planner treats ranges uniformly.
pub(crate) struct QueryRange {
    /// The minimum EPK value.
    #[serde(deserialize_with = "epk_string_from_proxy_or_gateway")]
    pub min: String,

    /// The maximum EPK value.
    #[serde(deserialize_with = "epk_string_from_proxy_or_gateway")]
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

    /// The Gateway V2 thin-client proxy returns the QueryPlan response with
    /// `hasSelectValue` / `hasNonStreamingOrderBy` as integers (0/1) instead
    /// of booleans, with `queryRanges[].min` as the empty `PartitionKeyInternal`
    /// array (`[]`), and with `queryRanges[].max` as the literal string
    /// `"Infinity"`. Pin that we deserialize this shape into the same
    /// canonical EPK form (`""` / `"FF"`) the planner already handles.
    #[test]
    fn deserializes_thin_client_proxy_response_shape() {
        let json = r#"{
            "queryInfo": {
                "distinctType": "None",
                "groupByExpressions": [],
                "groupByAliases": [],
                "orderBy": [],
                "orderByExpressions": [],
                "aggregates": [],
                "hasSelectValue": 0,
                "rewrittenQuery": "",
                "groupByAliasToAggregateType": {},
                "hasNonStreamingOrderBy": 0
            },
            "queryRanges": [
                {
                    "min": [],
                    "max": "Infinity",
                    "isMinInclusive": true,
                    "isMaxInclusive": false
                }
            ]
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        let info = plan.query_info.expect("queryInfo present");
        assert!(!info.has_select_value);
        assert!(!info.has_non_streaming_order_by);
        assert_eq!(plan.query_ranges.len(), 1);
        assert_eq!(plan.query_ranges[0].min, "");
        assert_eq!(plan.query_ranges[0].max, "FF");
        assert!(plan.query_ranges[0].is_min_inclusive);
        assert!(!plan.query_ranges[0].is_max_inclusive);
    }

    #[test]
    fn deserializes_thin_client_infinity_sentinel_in_array() {
        let json = r#"{
            "queryRanges": [
                {
                    "min": [],
                    "max": [{"type":"Infinity"}],
                    "isMinInclusive": 1,
                    "isMaxInclusive": 0
                }
            ]
        }"#;
        let plan: QueryPlan = serde_json::from_str(json).unwrap();
        assert_eq!(plan.query_ranges[0].min, "");
        assert_eq!(plan.query_ranges[0].max, "FF");
        assert!(plan.query_ranges[0].is_min_inclusive);
        assert!(!plan.query_ranges[0].is_max_inclusive);
    }
}
