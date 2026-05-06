// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway query plan response envelope.
//!
//! Deserializes the JSON response from the Cosmos DB Gateway's query plan endpoint
//! (`x-ms-cosmos-is-query-plan-request: True`). The structural `queryInfo`
//! payload uses the schema-specific [`GatewayQueryInfo`] type — distinct from
//! the local plan generator's [`super::plan::LocalQueryInfo`] (F21).
//!
//! The two types share a structural core (TOP / OFFSET / LIMIT / DISTINCT /
//! ORDER BY / GROUP BY / aggregates / SELECT VALUE) plus disjoint extras:
//! Gateway-only fields (`rewritten_query`, `group_by_aliases`,
//! `group_by_alias_to_aggregate_type`, `has_non_streaming_order_by`,
//! `d_count_info`) and local-only booleans (`has_join`, `has_subquery`,
//! `has_where`, `has_udf`). Use
//! [`GatewayQueryInfo::shared_fields_match`] to compare a Gateway response
//! against a locally-generated plan — it intentionally ignores the disjoint
//! extras so a Gateway response is never silently coerced into a misleading
//! `LocalQueryInfo` value (no all-`false`-booleans `From` conversion).

use serde::Deserialize;

use super::plan::{AggregateKind, DistinctType, LocalQueryInfo, SortOrder};

/// Top-level response from the Gateway query plan endpoint.
///
/// Mirrors the .NET SDK's `PartitionedQueryExecutionInfo` type.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GatewayQueryPlan {
    /// Version of the query plan format.
    #[serde(default)]
    pub(crate) partitioned_query_execution_info_version: i32,

    /// Structural information about the query (gateway-shaped).
    pub(crate) query_info: GatewayQueryInfo,

    /// Effective partition key ranges the query targets.
    #[serde(default)]
    pub(crate) query_ranges: Vec<GatewayQueryRange>,
}

/// Structural information about a query as returned by the Gateway query plan endpoint.
///
/// F21: split from the previously-unified `QueryInfo`. Carries the shared
/// structural fields (TOP / OFFSET / LIMIT / DISTINCT / ORDER BY / GROUP BY /
/// aggregates / SELECT VALUE) plus the gateway-only fields
/// (`rewritten_query`, `group_by_aliases`,
/// `group_by_alias_to_aggregate_type`, `has_non_streaming_order_by`,
/// `d_count_info`). The SDK pipeline operates on
/// [`LocalQueryInfo`]; use [`GatewayQueryInfo::shared_fields_match`] when you
/// need to compare a Gateway response against a locally-generated plan
/// without manufacturing a `LocalQueryInfo` value from a Gateway response
/// (which would silently fabricate values for the local-only booleans).
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GatewayQueryInfo {
    /// The kind of DISTINCT, if any.
    #[serde(default)]
    pub(crate) distinct_type: DistinctType,

    /// TOP value, if present.
    #[serde(default)]
    pub(crate) top: Option<i64>,

    /// OFFSET value, if present.
    #[serde(default)]
    pub(crate) offset: Option<i64>,

    /// LIMIT value, if present.
    #[serde(default)]
    pub(crate) limit: Option<i64>,

    /// ORDER BY sort orders (one per ORDER BY item).
    #[serde(default)]
    pub(crate) order_by: Vec<SortOrder>,

    /// ORDER BY expressions as path strings.
    #[serde(default)]
    pub(crate) order_by_expressions: Vec<String>,

    /// GROUP BY expressions as path strings.
    #[serde(default)]
    pub(crate) group_by_expressions: Vec<String>,

    /// GROUP BY aliases (gateway only).
    #[serde(default)]
    pub(crate) group_by_aliases: Vec<String>,

    /// Aggregate functions used in the query.
    #[serde(default)]
    pub(crate) aggregates: Vec<AggregateKind>,

    /// GROUP BY alias to aggregate type mapping (gateway only).
    #[serde(default)]
    pub(crate) group_by_alias_to_aggregate_type: Option<serde_json::Value>,

    /// The rewritten query text, if the gateway rewrites it (gateway only).
    #[serde(default)]
    pub(crate) rewritten_query: Option<String>,

    /// Whether the SELECT clause uses `SELECT VALUE`.
    #[serde(default)]
    pub(crate) has_select_value: bool,

    /// Whether the query contains non-streaming ORDER BY (gateway only).
    #[serde(default)]
    pub(crate) has_non_streaming_order_by: bool,

    /// DCount information (gateway only).
    #[serde(default)]
    pub(crate) d_count_info: Option<serde_json::Value>,
}

impl GatewayQueryInfo {
    /// Compare the structural core of a Gateway response against a
    /// locally-generated [`LocalQueryInfo`].
    ///
    /// Compares the fields the two types share — `distinct_type`, `top`,
    /// `offset`, `limit`, `order_by`, `order_by_expressions`,
    /// `group_by_expressions`, `aggregates`, `has_select_value` — and
    /// intentionally ignores the disjoint extras on either side
    /// (gateway-only `rewritten_query`, `group_by_aliases`,
    /// `group_by_alias_to_aggregate_type`, `has_non_streaming_order_by`,
    /// `d_count_info`; local-only `has_join`, `has_subquery`, `has_where`,
    /// `has_udf`).
    ///
    /// This is the intended comparison surface for plan-vs-Gateway parity
    /// checks. A `From<GatewayQueryInfo> for LocalQueryInfo` conversion is
    /// deliberately *not* provided: it would have to fabricate values for the
    /// local-only booleans, and downstream code receiving the converted value
    /// would have no way to tell whether a `false` came from local AST
    /// analysis or from the conversion default.
    pub(crate) fn shared_fields_match(&self, local: &LocalQueryInfo) -> bool {
        self.distinct_type == local.distinct_type
            && self.top == local.top
            && self.offset == local.offset
            && self.limit == local.limit
            && self.order_by == local.order_by
            && self.order_by_expressions == local.order_by_expressions
            && self.group_by_expressions == local.group_by_expressions
            && self.aggregates == local.aggregates
            && self.has_select_value == local.has_select_value
    }
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
    fn deserializes_gateway_query_plan_into_gateway_query_info() {
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
        assert_eq!(plan.query_info, GatewayQueryInfo::default());
    }

    #[test]
    fn shared_fields_match_ignores_disjoint_extras() {
        // F21 (revised): comparing a Gateway response against a local plan
        // ignores the disjoint extras on either side — gateway-only
        // (rewrittenQuery, has_non_streaming_order_by, d_count_info,
        // group_by_aliases, group_by_alias_to_aggregate_type) and local-only
        // (has_join, has_subquery, has_where, has_udf). This avoids a
        // `From<GatewayQueryInfo> for LocalQueryInfo` conversion that would
        // silently fabricate `false` for the local-only booleans.
        let gw = GatewayQueryInfo {
            distinct_type: DistinctType::Ordered,
            top: Some(5),
            offset: Some(3),
            limit: Some(10),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.city".into()],
            group_by_expressions: vec!["c.city".into()],
            group_by_aliases: vec!["alias_0".into()],
            aggregates: vec![AggregateKind::Count],
            group_by_alias_to_aggregate_type: Some(serde_json::json!({"alias_0": "Count"})),
            rewritten_query: Some("SELECT VALUE 1".into()),
            has_select_value: true,
            has_non_streaming_order_by: true,
            d_count_info: Some(serde_json::json!({"dCountAlias": null})),
        };
        let local = crate::query::plan::LocalQueryInfo {
            distinct_type: DistinctType::Ordered,
            top: Some(5),
            offset: Some(3),
            limit: Some(10),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.city".into()],
            group_by_expressions: vec!["c.city".into()],
            aggregates: vec![AggregateKind::Count],
            has_select_value: true,
            // Local-only booleans differ from the Gateway-only extras above —
            // shared_fields_match must still return true.
            has_join: true,
            has_subquery: true,
            has_where: true,
            has_udf: true,
        };

        assert!(gw.shared_fields_match(&local));
    }

    #[test]
    fn shared_fields_match_detects_shared_field_divergence() {
        // Sanity: a divergence in any shared field must surface.
        let gw = GatewayQueryInfo {
            top: Some(5),
            ..Default::default()
        };
        let local_diff = crate::query::plan::LocalQueryInfo {
            top: Some(6),
            ..Default::default()
        };
        assert!(!gw.shared_fields_match(&local_diff));

        let local_same = crate::query::plan::LocalQueryInfo {
            top: Some(5),
            ..Default::default()
        };
        assert!(gw.shared_fields_match(&local_same));
    }
}
