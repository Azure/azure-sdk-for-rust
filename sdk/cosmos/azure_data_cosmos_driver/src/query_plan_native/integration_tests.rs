// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore nopk startswith codegen inlist

//! Integration tests for the QueryPlanInterop native library.
//!
//! These tests mirror the .NET SDK's `QueryPlanBaselineTests` and validate
//! that the Rust FFI bindings produce equivalent query plans.
//!
//! Every test asserts the **entire** `QueryInfo` struct so that any
//! regression in any field is caught immediately. Engine-version-dependent
//! text fields (`rewritten_query`, `order_by_expressions`, `group_by_*`)
//! are normalised before comparison -- only their *presence* and *counts*
//! are checked.
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
//!  cargo test -p azure_data_cosmos_driver --lib query_plan_native --features __internal_native_query_plan
//!
//! # Linux
//! QUERY_PLAN_INTEROP_LIB_DIR=/path/to/lib cargo test -p azure_data_cosmos_driver --lib query_plan_native --features __internal_native_query_plan
//! ```
#![allow(clippy::needless_update)]

use super::native::PartitionKind;
use super::provider::{QueryPlanOptions, QueryPlanProvider};
use super::{DistinctType, QueryInfo, QueryPlan, SortOrder};
use std::collections::HashMap;

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

/// Shorthand: the default QueryInfo with all fields at their zero/empty/false values.
fn qi() -> QueryInfo {
    QueryInfo::default()
}

/// Asserts the actual [`QueryInfo`] matches expected, normalising only
/// `rewritten_query` (the native DLL always returns `Some`, and the exact
/// text varies across engine versions). All other fields -- including
/// `order_by_expressions`, `group_by_expressions`, `group_by_aliases`, and
/// `group_by_alias_to_aggregate_type` -- are compared **exactly**.
///
/// Dedicated tests like `rewritten_query_for_order_by` check `rewritten_query`
/// content explicitly where it matters.
fn assert_query_info(actual: &QueryInfo, mut expected: QueryInfo) {
    // Normalise rewritten_query (always Some from DLL, text varies by version)
    expected.rewritten_query = actual.rewritten_query.clone();

    // Full structural comparison on all fields
    assert_eq!(actual, &expected);
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert!(!info.query_ranges.is_empty());
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(5),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(42),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(5),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(5),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(5),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.blah".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            offset: Some(5),
            limit: Some(10),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.blah".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            offset: Some(10),
            limit: Some(20),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.blah".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.blah".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.key".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            order_by: vec![SortOrder::Descending],
            order_by_expressions: vec!["c.blah".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            order_by: vec![SortOrder::Ascending, SortOrder::Descending],
            order_by_expressions: vec!["c.a".into(), "c.b".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(5),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.blah".into()],
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            distinct_type: DistinctType::Unordered,
            ..qi()
        },
    );
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
    let actual = info.query_info.unwrap();
    assert!(
        actual.distinct_type != DistinctType::None,
        "DISTINCT should set distinct_type"
    );
    assert_query_info(
        &actual,
        QueryInfo {
            distinct_type: actual.distinct_type.clone(),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            distinct_type: DistinctType::Ordered,
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.blah".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Average".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Min".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Max".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Sum".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Count".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["MakeList".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["MakeSet".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Average".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Average".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Average".into()],
            has_select_value: true,
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            top: Some(5),
            aggregates: vec!["Average".into()],
            has_select_value: true,
            ..qi()
        },
    );
}

// =========================================================================
// Non-value aggregates (mirrors QueryPlanBaselineTests.NonValueAggregates)
//
// Non-value aggregates (e.g. `SELECT MIN(c.x) FROM c` without VALUE)
// populate `group_by_alias_to_aggregate_type` and `group_by_aliases`
// instead of `aggregates`.
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_aliases: vec!["$1".into()],
            group_by_alias_to_aggregate_type: HashMap::from([(
                "$1".into(),
                serde_json::json!("Min"),
            )]),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_aliases: vec!["$1".into(), "$2".into()],
            group_by_alias_to_aggregate_type: HashMap::from([
                ("$1".into(), serde_json::json!("Min")),
                ("$2".into(), serde_json::json!("Max")),
            ]),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_aliases: vec!["minBlah".into()],
            group_by_alias_to_aggregate_type: HashMap::from([(
                "minBlah".into(),
                serde_json::json!("Min"),
            )]),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_aliases: vec!["$1".into()],
            group_by_alias_to_aggregate_type: HashMap::from([(
                "$1".into(),
                serde_json::json!("Min"),
            )]),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_expressions: vec!["c.age".into(), "c.name".into()],
            group_by_aliases: vec!["age".into(), "name".into()],
            group_by_alias_to_aggregate_type: HashMap::from([
                ("age".into(), serde_json::json!("")),
                ("name".into(), serde_json::json!("")),
            ]),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_expressions: vec!["c.team".into()],
            group_by_aliases: vec!["team".into(), "count".into(), "avg_age".into()],
            group_by_alias_to_aggregate_type: HashMap::from([
                ("team".into(), serde_json::json!("")),
                ("count".into(), serde_json::json!("Count")),
                ("avg_age".into(), serde_json::json!("Average")),
            ]),
            ..qi()
        },
    );
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
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            aggregates: vec!["Count".into()],
            has_select_value: true,
            group_by_expressions: vec!["c.age".into()],
            ..qi()
        },
    );
}

#[test]
fn group_by_arbitrary_scalar() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec(
                "SELECT UPPER(c.name) AS name, AVG(c.income) AS income FROM c GROUP BY UPPER(c.name)",
            ),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert_query_info(
        &info.query_info.unwrap(),
        QueryInfo {
            group_by_expressions: vec!["UPPER(c.name)".into()],
            group_by_aliases: vec!["name".into(), "income".into()],
            group_by_alias_to_aggregate_type: HashMap::from([
                ("name".into(), serde_json::json!("")),
                ("income".into(), serde_json::json!("Average")),
            ]),
            ..qi()
        },
    );
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert!(
        info.query_ranges.len() >= 3,
        "IN list should produce multiple ranges"
    );
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert!(info.query_ranges.len() >= 2);
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
}

#[test]
fn subquery_as_filter() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec(
                "SELECT * FROM c WHERE (c.blah = (SELECT * FROM c WHERE c.key = 42 and c.id = 5)) and c.key = 32",
            ),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_eq!(
        info.query_ranges.len(),
        1,
        "equality should produce a single range"
    );
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_eq!(info.query_ranges.len(), 1);
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_eq!(info.query_ranges.len(), 1);
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_eq!(info.query_ranges.len(), 1);
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    assert_query_info(&info.query_info.unwrap(), qi());
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
    let actual = info.query_info.unwrap();
    assert!(
        actual
            .rewritten_query
            .as_ref()
            .map_or(false, |q| !q.is_empty()),
        "cross-partition ORDER BY should produce a non-empty rewritten query"
    );
    assert_query_info(
        &actual,
        QueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.name".into()],
            ..qi()
        },
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
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.name = '\u{4e16}\u{754c}'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert_query_info(&info.query_info.unwrap(), qi());
}

#[test]
fn unicode_surrogate_pair_in_query() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c WHERE c.name = '\u{1F600}'"),
            &["/key"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert_query_info(&info.query_info.unwrap(), qi());
}

#[test]
fn unicode_partition_key_path() {
    require_native_dll!(provider);
    let info = provider
        .get_partition_key_ranges(
            &query_spec("SELECT * FROM c"),
            &["/\u{00fc}ser"],
            &hash_options(),
            None,
        )
        .unwrap();
    assert!(!info.query_ranges.is_empty());
    assert_query_info(&info.query_info.unwrap(), qi());
}
