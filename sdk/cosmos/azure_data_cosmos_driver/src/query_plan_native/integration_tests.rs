// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Integration tests for the QueryPlanInterop native library.
//!
//! These tests mirror the .NET SDK's `QueryPlanBaselineTests` and validate
//! that the Rust FFI bindings produce equivalent query plans.
//!
//! Gated behind the `integration` feature. The native library must be
//! discoverable by the OS loader (e.g. via `PATH` on Windows or
//! `LD_LIBRARY_PATH` on Linux).
//!
//! # Running locally
//!
//! ```powershell
//! # Windows
//! $env:QUERY_PLAN_INTEROP_LIB_DIR = "Q:\QueryPlanInterop"
//!  cargo test -p azure_data_cosmos_driver --lib query_plan_native --features __query_plan_native_integration
//!
//! # Linux
//! QUERY_PLAN_INTEROP_LIB_DIR=/path/to/lib cargo test -p azure_data_cosmos_driver --lib query_plan_native --features __query_plan_native_integration
//! ```

use super::native::PartitionKind;
use super::provider::{QueryPlanOptions, QueryPlanProvider};
use super::{DistinctType, QueryInfo, QueryPlan, SortOrder};

// -------------------------------------------------------------------------
// Configuration -- matches QueryPartitionProviderTestInstance in the .NET SDK
// -------------------------------------------------------------------------

/// Query engine configuration identical to the .NET SDK's
/// `QueryPartitionProviderTestInstance.DefaultQueryEngineConfiguration`.
const QUERY_ENGINE_CONFIG: &str = r#"{
    "maxSqlQueryInputLength": 262144,
    "maxJoinsPerSqlQuery": 5,
    "maxLogicalAndPerSqlQuery": 2000,
    "maxLogicalOrPerSqlQuery": 2000,
    "maxUdfRefPerSqlQuery": 10,
    "maxInExpressionItemsCount": 16000,
    "queryMaxGroupByTableCellCount": 500000,
    "queryMaxInMemorySortDocumentCount": 500,
    "maxQueryRequestTimeoutFraction": 0.90,
    "sqlAllowNonFiniteNumbers": false,
    "sqlAllowAggregateFunctions": true,
    "sqlAllowSubQuery": true,
    "sqlAllowScalarSubQuery": true,
    "allowNewKeywords": true,
    "sqlAllowLike": true,
    "sqlAllowGroupByClause": true,
    "maxSpatialQueryCells": 12,
    "spatialMaxGeometryPointCount": 256,
    "sqlDisableQueryILOptimization": false,
    "sqlDisableFilterPlanOptimization": false,
    "queryEnableFullText": true
}"#;

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

fn create_provider() -> Option<QueryPlanProvider> {
    QueryPlanProvider::new(QUERY_ENGINE_CONFIG).ok()
}

/// Helper macro that skips the test if the native DLL is not available.
macro_rules! require_native_dll {
    () => {
        let Some(_provider) = create_provider() else {
            eprintln!("Skipping: native DLL not available");
            return;
        };
    };
    ($provider:ident) => {
        let Some($provider) = create_provider() else {
            eprintln!("Skipping: native DLL not available");
            return;
        };
    };
}

fn query_spec(query: &str) -> String {
    serde_json::json!({"query": query, "parameters": []}).to_string()
}

fn query_spec_with_params(query: &str, params: serde_json::Value) -> String {
    serde_json::json!({"query": query, "parameters": params}).to_string()
}

fn hash_options() -> QueryPlanOptions {
    QueryPlanOptions {
        require_formattable_order_by_query: true,
        is_continuation_expected: false,
        allow_non_value_aggregate_query: true,
        allow_dcount: true,
        ..QueryPlanOptions::default()
    }
}

fn multi_hash_options() -> QueryPlanOptions {
    QueryPlanOptions {
        partition_kind: PartitionKind::MultiHash,
        ..hash_options()
    }
}

/// Returns true if the query has any aggregate operators, checking both
/// the `aggregates` array (VALUE aggregates) and `groupByAliasToAggregateType`
/// map (non-VALUE aggregates). The native DLL only populates `aggregates`
/// for `SELECT VALUE` queries; non-value aggregates go into the map.
fn has_aggregates(qi: &QueryInfo) -> bool {
    !qi.aggregates.is_empty()
        || qi
            .group_by_alias_to_aggregate_type
            .values()
            .any(|v| !v.is_null() && v.as_str() != Some(""))
}

// =========================================================================
// Provider lifecycle
// =========================================================================

#[test]
fn create_service_provider_succeeds() {
    require_native_dll!();
}

// =========================================================================
// Basic queries (mirrors QueryPlanBaselineTests.Basic)
// =========================================================================

#[test]
fn basic_select_constant() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(&query_spec("SELECT 5"), &["/key"], &hash_options(), None)
        .expect("SELECT 5 should succeed");
    assert!(!info.query_ranges.is_empty());
}

#[test]
fn basic_select_top_constant() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 2 5"),
            &["/key"],
            &hash_options(),
            None,
        )
        .expect("SELECT TOP 2 5 should succeed");
    // No FROM clause -- top may or may not be populated depending on version.
    assert!(!info.query_ranges.is_empty());
}

#[test]
fn basic_select_star() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .expect("SELECT * FROM c should succeed");
    assert!(!info.query_ranges.is_empty());
}

#[test]
fn basic_where_true() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE true"),
            &["/key"],
            &hash_options(),
            None,
        )
        .expect("WHERE true should succeed");
    assert!(info.query_info.is_some());
}

#[test]
fn basic_where_false() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE false"),
            &["/key"],
            &hash_options(),
            None,
        )
        .expect("WHERE false should succeed");
    // The query should succeed; range semantics depend on engine version.
    assert!(!info.query_ranges.is_empty());
}

// =========================================================================
// TOP (mirrors QueryPlanBaselineTests.Top)
// =========================================================================

#[test]
fn top_just_top() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 * FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.top, Some(5));
    assert!(qi.order_by.is_empty());
    assert!(qi.distinct_type == DistinctType::None);
}

#[test]
fn top_parameterized() {
    require_native_dll!(provider);
    let spec = query_spec_with_params(
        "SELECT TOP @TOPCOUNT * FROM c",
        serde_json::json!([{"name": "@TOPCOUNT", "value": 42}]),
    );
    let info = provider
        .get_partition_key_ranges(&spec, &["/key"], &hash_options(), None)
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.top, Some(42));
}

#[test]
fn top_with_non_partition_filter() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 * FROM c WHERE c.blah = 5"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.top, Some(5));
}

#[test]
fn top_with_partition_filter() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 * FROM c WHERE c.key = 5"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.top, Some(5));
}

#[test]
fn top_with_order_by() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 * FROM c ORDER BY c.blah"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.top, Some(5));
    assert!(!qi.order_by.is_empty());
}

// =========================================================================
// OFFSET / LIMIT (mirrors QueryPlanBaselineTests.OffsetLimit)
// =========================================================================

#[test]
fn offset_limit_basic() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c ORDER BY c.blah OFFSET 5 LIMIT 10"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.offset, Some(5));
    assert_eq!(qi.limit, Some(10));
    assert!(!qi.order_by.is_empty());
}

#[test]
fn offset_limit_parameterized() {
    require_native_dll!(provider);
    let spec = query_spec_with_params(
        "SELECT * FROM c ORDER BY c.blah OFFSET @skip LIMIT @take",
        serde_json::json!([
            {"name": "@skip", "value": 10},
            {"name": "@take", "value": 20}
        ]),
    );
    let info = provider
        .get_partition_key_ranges(&spec, &["/key"], &hash_options(), None)
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.offset, Some(10));
    assert_eq!(qi.limit, Some(20));
}

// =========================================================================
// ORDER BY (mirrors QueryPlanBaselineTests.OrderBy)
// =========================================================================

#[test]
fn order_by_non_partition_key_asc() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c ORDER BY c.blah"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(!qi.order_by.is_empty());
    assert_eq!(qi.order_by[0], SortOrder::Ascending);
    assert!(qi.rewritten_query.as_ref().map_or(false, |q| !q.is_empty()));
}

#[test]
fn order_by_partition_key() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c ORDER BY c.key"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(!qi.order_by.is_empty());
}

#[test]
fn order_by_desc() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c ORDER BY c.blah DESC"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.order_by[0], SortOrder::Descending);
}

#[test]
fn multi_order_by() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c ORDER BY c.a ASC, c.b DESC"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.order_by.len(), 2);
    assert_eq!(qi.order_by[0], SortOrder::Ascending);
    assert_eq!(qi.order_by[1], SortOrder::Descending);
}

#[test]
fn order_by_with_top_and_projection() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 c.blah FROM c ORDER BY c.blah"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert_eq!(qi.top, Some(5));
    assert!(!qi.order_by.is_empty());
}

// =========================================================================
// DISTINCT (mirrors QueryPlanBaselineTests.Distinct)
// =========================================================================

#[test]
fn distinct_select_star() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT DISTINCT * FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.distinct_type != DistinctType::None);
    assert_eq!(qi.distinct_type, DistinctType::Unordered);
}

#[test]
fn distinct_field() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT DISTINCT c.blah FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.distinct_type != DistinctType::None);
}

#[test]
fn distinct_value_with_order_by() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT DISTINCT VALUE c.blah FROM c ORDER BY c.blah"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.distinct_type != DistinctType::None);
    assert_eq!(qi.distinct_type, DistinctType::Ordered);
    assert!(!qi.order_by.is_empty());
}

// =========================================================================
// Aggregates (mirrors QueryPlanBaselineTests.Aggregates)
// =========================================================================

#[test]
fn aggregate_avg() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE AVG(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
    assert!(qi.aggregates.contains(&"Average".to_string()));
}

#[test]
fn aggregate_min() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE MIN(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.aggregates.contains(&"Min".to_string()));
}

#[test]
fn aggregate_max() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE MAX(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.aggregates.contains(&"Max".to_string()));
}

#[test]
fn aggregate_sum() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE SUM(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.aggregates.contains(&"Sum".to_string()));
}

#[test]
fn aggregate_count() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE COUNT(1) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.aggregates.contains(&"Count".to_string()));
}

#[test]
fn aggregate_makelist() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE MAKELIST(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.aggregates.contains(&"MakeList".to_string()));
}

#[test]
fn aggregate_makeset() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE MAKESET(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(qi.aggregates.contains(&"MakeSet".to_string()));
}

#[test]
fn aggregate_no_partition_key() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE AVG(c.blah) FROM c"),
            &[] as &[&str],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

#[test]
fn aggregate_with_filter() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE AVG(c.blah) FROM c WHERE c.key = 5"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

#[test]
fn aggregate_with_join() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE AVG(j) FROM c JOIN j IN c.blah"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

#[test]
fn aggregate_with_top() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 VALUE AVG(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
    assert!(qi.top.is_some());
}

// =========================================================================
// Non-value aggregates (mirrors QueryPlanBaselineTests.NonValueAggregates)
// =========================================================================

#[test]
fn non_value_aggregate_min() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT MIN(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

#[test]
fn non_value_aggregate_multiple() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT MIN(c.blah), MAX(c.blah) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

#[test]
fn non_value_aggregate_with_alias() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT MIN(c.blah) AS minBlah FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

#[test]
fn non_value_aggregate_with_partition_filter() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT MIN(c.blah) FROM c WHERE c.key = 1"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(has_aggregates(&qi));
}

// =========================================================================
// GROUP BY (mirrors QueryPlanBaselineTests.GroupBy)
// =========================================================================

#[test]
fn group_by_simple() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT c.age, c.name FROM c GROUP BY c.age, c.name"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(!qi.group_by_expressions.is_empty());
}

#[test]
fn group_by_with_aggregates() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec(
                "SELECT c.team, COUNT(1) AS count, AVG(c.age) AS avg_age FROM c GROUP BY c.team",
            ),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(!qi.group_by_expressions.is_empty());
    assert!(has_aggregates(&qi));
    assert!(!qi.group_by_alias_to_aggregate_type.is_empty());
}

#[test]
fn group_by_value_count() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT VALUE COUNT(1) FROM c GROUP BY c.age"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(!qi.group_by_expressions.is_empty());
    assert!(has_aggregates(&qi));
    assert!(qi.has_select_value);
}

#[test]
fn group_by_arbitrary_scalar() {
    require_native_dll!(provider);
    let info = provider
            .get_partition_key_ranges(
                &query_spec("SELECT UPPER(c.name) AS name, AVG(c.income) AS income FROM c GROUP BY UPPER(c.name)"),
                &["/key"],
                &hash_options(),
                None,
            )
            .unwrap();
    let qi = info.query_info.unwrap();
    assert!(!qi.group_by_expressions.is_empty());
    assert!(has_aggregates(&qi));
}

// =========================================================================
// LIKE (mirrors QueryPlanBaselineTests.Like)
// =========================================================================

#[test]
fn like_simple() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.name LIKE '%test%'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn like_parameterized() {
    require_native_dll!(provider);
    let spec = query_spec_with_params(
        "SELECT * FROM c WHERE c.name LIKE @pattern",
        serde_json::json!([{"name": "@pattern", "value": "%test%"}]),
    );
    let info = provider
        .get_partition_key_ranges(&spec, &["/key"], &hash_options(), None)
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn like_with_partition_key_filter() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key = 'abc' AND c.name LIKE '%test%'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

// =========================================================================
// Multiple keys (mirrors QueryPlanBaselineTests.MultipleKeys)
// =========================================================================

#[test]
fn multi_key_is_defined() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM Root r WHERE r.a.b.c"),
            &["/a/b/c", "/a/c"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(!info.query_ranges.is_empty());
}

#[test]
fn multi_key_point_lookup() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM Root r WHERE r.a.b.c = null AND r.a.c = false"),
            &["/a/b/c", "/a/c"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(!info.query_ranges.is_empty());
}

#[test]
fn multi_hash_point_lookup() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.tenantId = 't1' AND c.userId = 'u1'"),
            &["/tenantId", "/userId"],
            &multi_hash_options(),
            None,
        )
        .unwrap();
    assert!(!info.query_ranges.is_empty());
}

// =========================================================================
// Many ranges (mirrors QueryPlanBaselineTests.ManyRanges)
// =========================================================================

#[test]
fn in_list_produces_multiple_ranges() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key IN (1, 2, 3)"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let ranges = info.query_ranges;
    assert!(ranges.len() >= 3, "IN list should produce multiple ranges");
}

#[test]
fn or_filter_produces_ranges() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key = 1 OR c.key = 2"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let ranges = info.query_ranges;
    assert!(ranges.len() >= 2);
}

// =========================================================================
// Subqueries (mirrors QueryPlanBaselineTests.Subqueries)
// =========================================================================

#[test]
fn subquery_basic() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT (SELECT * FROM c) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn subquery_with_filter_in_outer_query() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT (SELECT * FROM c) FROM c WHERE c.key = 42"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn subquery_with_filter_in_inner_query() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT (SELECT * FROM c WHERE c.key = 42) FROM c"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn subquery_as_filter() {
    require_native_dll!(provider);
    let info = provider
            .get_partition_key_ranges(
                &query_spec("SELECT * FROM c WHERE (c.blah = (SELECT * FROM c WHERE c.key = 42 and c.id = 5)) and c.key = 32"),
                &["/key"],
                &hash_options(),
                None,
            )
            .unwrap();
    assert!(info.query_info.is_some());
}

// =========================================================================
// Point ranges (mirrors QueryPlanBaselineTests.PointRange)
// =========================================================================

#[test]
fn point_range_string_equality() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key = 'value'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let ranges = info.query_ranges;
    assert_eq!(ranges.len(), 1, "equality should produce a single range");
}

#[test]
fn point_range_number_equality() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key = 5"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let ranges = info.query_ranges;
    assert_eq!(ranges.len(), 1);
}

#[test]
fn point_range_null_equality() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key = null"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let ranges = info.query_ranges;
    assert_eq!(ranges.len(), 1);
}

#[test]
fn point_range_bool_equality() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.key = true"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let ranges = info.query_ranges;
    assert_eq!(ranges.len(), 1);
}

// =========================================================================
// System functions (mirrors QueryPlanBaselineTests.SystemFunctions subset)
// =========================================================================

#[test]
fn system_function_abs() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE ABS(c.key) = 1"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn system_function_is_defined() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE IS_DEFINED(c.key)"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

// =========================================================================
// Negative cases (mirrors QueryPlanBaselineTests.Negative)
// =========================================================================

#[test]
fn negative_bad_function() {
    require_native_dll!(provider);
    let result = provider.get_partition_key_ranges(
        &query_spec("SELECT BADFUNC(r.age) FROM Root r"),
        &["/key"],
        &hash_options(),
        None,
    );
    assert!(result.is_err(), "unrecognized function should fail");
}

// =========================================================================
// Rewritten query
// =========================================================================

#[test]
fn rewritten_query_for_order_by() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c ORDER BY c.name"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    let qi = info.query_info.unwrap();
    assert!(
        qi.rewritten_query.as_ref().map_or(false, |q| !q.is_empty()),
        "cross-partition ORDER BY should produce a rewritten query"
    );
}

// =========================================================================
// Round-trip serialization
// =========================================================================

#[test]
fn query_plan_json_round_trip() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT TOP 5 c.name FROM c ORDER BY c.name"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();

    let json = serde_json::to_string(&info).unwrap();
    let roundtripped: QueryPlan = serde_json::from_str(&json).unwrap();
    assert_eq!(info, roundtripped);
}

// =========================================================================
// Unicode and special character handling
// =========================================================================

#[test]
fn unicode_bmp_characters_in_query() {
    require_native_dll!(provider);
    // BMP characters: Chinese, Japanese, emoji (BMP range)
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.name = '\u{4e16}\u{754c}'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn unicode_surrogate_pair_in_query() {
    require_native_dll!(provider);
    // U+1F600 (grinning face) requires a surrogate pair in UTF-16
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.name = '\u{1F600}'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(info.query_info.is_some());
}

#[test]
fn unicode_partition_key_path() {
    require_native_dll!(provider);
    // Partition key path with non-ASCII characters
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c"),
            &["/\u{00fc}ser"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(!info.query_ranges.is_empty());
}
