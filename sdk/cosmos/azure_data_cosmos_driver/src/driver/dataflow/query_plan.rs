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

use crate::error::{CosmosError, CosmosStatus, Result};
use crate::models::{EffectivePartitionKey, PartitionKeyDefinition, PartitionKeyValue};

/// Deserializes a boolean from either a JSON boolean (`true`/`false`) or
/// an integer (`0`/`1`). The native QueryPlanInterop library serializes
/// booleans as integers, while the Gateway uses standard JSON booleans.
fn bool_from_int_or_bool<'de, D>(deserializer: D) -> std::result::Result<bool, D::Error>
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

        fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<bool, E> {
            Ok(v)
        }

        fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<bool, E> {
            Ok(v != 0)
        }

        fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<bool, E> {
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
fn string_or_json<'de, D>(deserializer: D) -> std::result::Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) => Ok(s),
        other => Ok(other.to_string()),
    }
}

/// Raw query plan as deserialized off the wire.
///
/// The standard Gateway returns each `queryRanges[].min` / `max` as a hex EPK
/// string (`""`, `"FF"`, or a specific hash hex). The Gateway V2 thin-client
/// proxy returns the raw `PartitionKeyInternal` JSON shape — an array of
/// component values or the special sentinels `[]` (MIN) and
/// `[{"type":"Infinity"}]` (MAX). To accept both shapes we keep the bounds
/// as opaque `serde_json::Value` here and resolve them to canonical EPK hex
/// strings via [`RawQueryPlan::resolve`].
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawQueryPlan {
    #[serde(default)]
    pub partitioned_query_execution_info_version: usize,

    #[serde(default)]
    pub query_info: Option<QueryInfo>,

    #[serde(default)]
    pub query_ranges: Vec<RawQueryRange>,

    pub hybrid_search_query_info: Option<HybridSearchQueryInfo>,
}

/// A raw query range as deserialized off the wire, with `min` / `max` still
/// in proxy or Gateway form. See [`RawQueryPlan`] for the resolution path.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawQueryRange {
    pub min: serde_json::Value,
    pub max: serde_json::Value,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub is_min_inclusive: bool,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub is_max_inclusive: bool,
}

impl RawQueryPlan {
    /// Resolves the wire-format ranges into a planner-ready [`QueryPlan`].
    ///
    /// Walks each `queryRanges[].min` / `max`. Strings are accepted as-is
    /// (with the literal `"Infinity"` collapsed to the `"FF"` sentinel).
    /// PartitionKeyInternal JSON arrays are decoded against `pk_definition`:
    /// the empty array becomes the MIN sentinel; `[{"type":"Infinity"}]`
    /// becomes MAX; concrete components are hashed via
    /// [`EffectivePartitionKey::compute`].
    ///
    /// After conversion the ranges are sorted by `min` so the planner can
    /// match them against the routing map.
    pub(crate) fn resolve(self, pk_definition: &PartitionKeyDefinition) -> Result<QueryPlan> {
        let mut query_ranges = Vec::with_capacity(self.query_ranges.len());
        for raw in self.query_ranges {
            let min = resolve_epk_bound(&raw.min, pk_definition)?;
            let max = resolve_epk_bound(&raw.max, pk_definition)?;
            query_ranges.push(QueryRange {
                min,
                max,
                is_min_inclusive: raw.is_min_inclusive,
                is_max_inclusive: raw.is_max_inclusive,
            });
        }
        query_ranges.sort_by(|a, b| a.min.cmp(&b.min));
        Ok(QueryPlan {
            partitioned_query_execution_info_version: self.partitioned_query_execution_info_version,
            query_info: self.query_info,
            query_ranges,
            hybrid_search_query_info: self.hybrid_search_query_info,
        })
    }
}

/// Converts a single `queryRanges[].min|max` JSON value into the canonical
/// EPK hex string the planner expects.
fn resolve_epk_bound(
    value: &serde_json::Value,
    pk_definition: &PartitionKeyDefinition,
) -> Result<String> {
    use serde_json::Value;
    match value {
        Value::String(s) if s == "Infinity" => Ok("FF".to_owned()),
        Value::String(s) => Ok(s.clone()),
        Value::Array(items) => {
            // Empty PartitionKeyInternal array = MIN sentinel.
            if items.is_empty() {
                return Ok(String::new());
            }
            // Single `{"type":"Infinity"}` element = MAX sentinel.
            if items.len() == 1 {
                if let Value::Object(obj) = &items[0] {
                    if matches!(obj.get("type"), Some(Value::String(t)) if t == "Infinity") {
                        return Ok("FF".to_owned());
                    }
                }
            }
            let pk_values: Vec<PartitionKeyValue> = items
                .iter()
                .map(pki_component_to_pk_value)
                .collect::<Result<_>>()?;
            let epk = EffectivePartitionKey::compute(
                &pk_values,
                pk_definition.kind(),
                pk_definition.version(),
            );
            Ok(epk.to_hex())
        }
        other => Err(CosmosError::builder()
            .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message(format!(
                "queryRanges bound has unsupported shape (expected string or PartitionKeyInternal array, got {})",
                json_type_name(other)
            ))
            .build()),
    }
}

/// Converts one component of a PartitionKeyInternal array (proxy form) into
/// a [`PartitionKeyValue`] suitable for [`EffectivePartitionKey::compute`].
fn pki_component_to_pk_value(value: &serde_json::Value) -> Result<PartitionKeyValue> {
    use serde_json::Value;
    match value {
        Value::Null => Ok(PartitionKeyValue::NULL),
        Value::Bool(b) => Ok(PartitionKeyValue::from(*b)),
        Value::Number(n) => {
            let f = n.as_f64().ok_or_else(|| {
                CosmosError::builder()
                    .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(format!(
                        "queryRanges component number {n} cannot be represented as f64"
                    ))
                    .build()
            })?;
            Ok(PartitionKeyValue::from(f))
        }
        Value::String(s) => Ok(PartitionKeyValue::from(s.clone())),
        Value::Object(obj) => {
            // PartitionKeyInternal serializes its special sentinels as
            // `{"type":"Infinity"}` and `{"type":"Undefined"}`. Map the
            // former to the EPK MAX sentinel and the latter to the SDK's
            // UNDEFINED value.
            match obj.get("type").and_then(|t| t.as_str()) {
                Some("Infinity") => Ok(PartitionKeyValue::INFINITY),
                Some("Undefined") => Ok(PartitionKeyValue::UNDEFINED),
                _ => Err(CosmosError::builder()
                    .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(format!(
                        "queryRanges component object is not a recognized PartitionKeyInternal sentinel: {value}"
                    ))
                    .build()),
            }
        }
        Value::Array(_) => Err(CosmosError::builder()
            .with_status(CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("queryRanges component cannot be a nested array")
            .build()),
    }
}

fn json_type_name(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "bool",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

/// The resolved query plan the planner consumes.
///
/// Built from a [`RawQueryPlan`] via [`RawQueryPlan::resolve`] on the Gateway
/// path. The native QueryPlanInterop path deserializes JSON directly into
/// this type (it does not need the EPK-hashing step the Gateway path needs,
/// because the native DLL emits ranges in a form `string_or_json` can flatten
/// in place).
#[derive(Debug, Default, Deserialize, Serialize, PartialEq)]
#[serde(default)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryPlan {
    /// The version of the query plan format.
    pub partitioned_query_execution_info_version: usize,

    /// Detailed query information (ordering, aggregates, rewrites, etc.).
    pub query_info: Option<QueryInfo>,

    /// The EPK ranges that the query references.
    ///
    /// Used by the planner to limit which physical partitions get queried.
    pub query_ranges: Vec<QueryRange>,

    /// Information about hybrid search queries, if applicable.
    pub hybrid_search_query_info: Option<HybridSearchQueryInfo>,
}

/// Information about a hybrid search query.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
#[derive(Debug, Clone, Deserialize, Default, PartialEq, Eq, Serialize)]
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
#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
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
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Serialize)]
pub(crate) enum SortOrder {
    Ascending,
    Descending,
}

/// An EPK range covered by the query.
///
/// On the Gateway path, ranges arrive in canonical hex EPK form (`""`, `"FF"`,
/// or a hash hex). On the native QueryPlanInterop path and the Gateway V2
/// thin-client proxy path, ranges arrive as `PartitionKeyInternal` JSON values
/// (arrays); `string_or_json` flattens those to the JSON text. The Gateway
/// path additionally runs each `RawQueryRange` through
/// [`RawQueryPlan::resolve`] to hash structured PK values into proper EPK hex.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)] // Inclusivity flags are wire-format; planner treats ranges uniformly.
pub(crate) struct QueryRange {
    /// The minimum EPK value (hex string, `""` for MIN, `"FF"` for MAX).
    #[serde(deserialize_with = "string_or_json")]
    pub min: String,

    /// The maximum EPK value (hex string, `""` for MIN, `"FF"` for MAX).
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
    use std::borrow::Cow;

    /// Single-path Hash/V2 partition key definition used by the parser tests
    /// that don't care about concrete-value EPK computation.
    fn default_pk_def() -> PartitionKeyDefinition {
        PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")])
    }

    fn parse(json: &str) -> QueryPlan {
        let raw: RawQueryPlan = serde_json::from_str(json).expect("valid raw JSON");
        raw.resolve(&default_pk_def()).expect("resolution succeeds")
    }

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
        let plan = parse(json);
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
        let plan = parse(json);
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
        let plan = parse(json);
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
        let plan = parse(json);
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
        let plan = parse(json);
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
        let plan = parse(json);
        assert_eq!(plan.query_ranges.len(), 2);
        assert_eq!(plan.query_ranges[0].max, "40");
        assert_eq!(plan.query_ranges[1].min, "80");
    }

    /// The Gateway V2 thin-client proxy returns the QueryPlan response with
    /// `hasSelectValue` / `hasNonStreamingOrderBy` as integers (0/1) instead
    /// of booleans, with `queryRanges[].min` as the empty `PartitionKeyInternal`
    /// array (`[]`), and with `queryRanges[].max` as the literal string
    /// `"Infinity"`. Pin that we resolve this shape into the same canonical
    /// EPK form (`""` / `"FF"`) the planner already handles.
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
        let plan = parse(json);
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
        let plan = parse(json);
        assert_eq!(plan.query_ranges[0].min, "");
        assert_eq!(plan.query_ranges[0].max, "FF");
        assert!(plan.query_ranges[0].is_min_inclusive);
        assert!(!plan.query_ranges[0].is_max_inclusive);
    }

    /// Proxy returns a concrete partition-key value in the PKI array; the
    /// resolver must hash it to the same EPK the SDK would compute client-side
    /// for that value.
    #[test]
    fn resolves_concrete_single_path_pk_value() {
        let json = r#"{
            "queryRanges": [
                {
                    "min": ["myKey"],
                    "max": ["myKey"],
                    "isMinInclusive": true,
                    "isMaxInclusive": true
                }
            ]
        }"#;
        let raw: RawQueryPlan = serde_json::from_str(json).unwrap();
        let pk_def = PartitionKeyDefinition::new(vec![Cow::Borrowed("/pk")]);
        let plan = raw.resolve(&pk_def).expect("resolve succeeds");

        let expected = EffectivePartitionKey::compute(
            &[PartitionKeyValue::from("myKey")],
            pk_def.kind(),
            pk_def.version(),
        );
        assert_eq!(plan.query_ranges[0].min, expected.to_hex());
        assert_eq!(plan.query_ranges[0].max, expected.to_hex());
        assert!(plan.query_ranges[0].is_min_inclusive);
        assert!(plan.query_ranges[0].is_max_inclusive);
        // Non-empty EPK string (the actual hash hex).
        assert!(!plan.query_ranges[0].min.is_empty());
        assert!(!plan.query_ranges[0].min.starts_with("FF"));
    }

    /// Multi-path (HPK) container: a prefix PKI array resolves via MultiHash
    /// V2 — each component hashed and concatenated.
    #[test]
    fn resolves_concrete_multi_hash_prefix() {
        let json = r#"{
            "queryRanges": [
                {
                    "min": ["tenantA"],
                    "max": ["tenantA"],
                    "isMinInclusive": true,
                    "isMaxInclusive": true
                }
            ]
        }"#;
        let raw: RawQueryPlan = serde_json::from_str(json).unwrap();
        let pk_def =
            PartitionKeyDefinition::new(vec![Cow::Borrowed("/tenantId"), Cow::Borrowed("/userId")]);
        let plan = raw.resolve(&pk_def).expect("resolve succeeds");

        let expected = EffectivePartitionKey::compute(
            &[PartitionKeyValue::from("tenantA")],
            pk_def.kind(),
            pk_def.version(),
        );
        assert_eq!(plan.query_ranges[0].min, expected.to_hex());
        assert_eq!(plan.query_ranges[0].max, expected.to_hex());
    }

    /// The resolver sorts ranges by `min` so the planner sees a stable order
    /// regardless of how the proxy emits them.
    #[test]
    fn resolves_sorts_ranges_by_min() {
        let json = r#"{
            "queryRanges": [
                { "min": "80", "max": "FF", "isMinInclusive": true, "isMaxInclusive": false },
                { "min": "",   "max": "40", "isMinInclusive": true, "isMaxInclusive": false },
                { "min": "40", "max": "80", "isMinInclusive": true, "isMaxInclusive": false }
            ]
        }"#;
        let plan = parse(json);
        assert_eq!(plan.query_ranges.len(), 3);
        assert_eq!(plan.query_ranges[0].min, "");
        assert_eq!(plan.query_ranges[1].min, "40");
        assert_eq!(plan.query_ranges[2].min, "80");
    }

    /// An object that is not the `Infinity` sentinel is malformed and must
    /// error out (no silent collapse to the full keyspace).
    #[test]
    fn resolves_rejects_unknown_pki_object() {
        let json = r#"{
            "queryRanges": [
                { "min": [{"weird": true}], "max": "FF", "isMinInclusive": true, "isMaxInclusive": false }
            ]
        }"#;
        let raw: RawQueryPlan = serde_json::from_str(json).unwrap();
        let err = raw.resolve(&default_pk_def()).unwrap_err();
        assert!(format!("{err:?}").contains("PartitionKeyInternal"));
    }
}
