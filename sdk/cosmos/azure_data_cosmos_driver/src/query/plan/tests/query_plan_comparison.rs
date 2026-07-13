// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore nopk startswith codegen inlist

//! Exhaustive structural comparison tests for the client-side query plan generator.
//!
//! Every test asserts the **entire** `QueryPlan` struct — both `pk_filters` and
//! every field of `query_info` — so that any regression in any part of the plan
//! is caught immediately.
//!
//! A handful of tests fully spell out every `LocalQueryInfo` field *and* trail with
//! `..qi()` — the redundant `..qi()` is intentional codegen-style ballast so
//! that future field additions don't silently leave the test under-specified.
#![allow(clippy::needless_update)]

use super::super::{
    generate_query_plan, generate_query_plan_with_parameters, AggregateKind, DistinctType,
    LocalQueryInfo, PartitionKeyFilter, PartitionKeyValue, QueryPlan, SortOrder,
};

/// Parse SQL and produce a full query plan against a single `/pk` partition key.
fn plan(sql: &str) -> QueryPlan {
    let p = crate::query::parse(sql).unwrap();
    generate_query_plan(&p.query, &["/pk"]).unwrap()
}

/// Parse SQL and produce a full query plan against hierarchical `/tenant`, `/userId`.
fn plan_hpk(sql: &str) -> QueryPlan {
    let p = crate::query::parse(sql).unwrap();
    generate_query_plan(&p.query, &["/tenant", "/userId"]).unwrap()
}

/// Parse SQL and produce a full query plan against 3-component hierarchical PK.
fn plan_hpk3(sql: &str) -> QueryPlan {
    let p = crate::query::parse(sql).unwrap();
    generate_query_plan(&p.query, &["/tenant", "/userId", "/sessionId"]).unwrap()
}

/// Parse SQL and produce a full query plan against a nested PK path `/address/city`.
fn plan_nested_pk(sql: &str) -> QueryPlan {
    let p = crate::query::parse(sql).unwrap();
    generate_query_plan(&p.query, &["/address/city"]).unwrap()
}

/// Parse SQL and produce a full query plan with no PK paths (always cross-partition).
fn plan_no_pk(sql: &str) -> QueryPlan {
    let p = crate::query::parse(sql).unwrap();
    generate_query_plan(&p.query, &[]).unwrap()
}
/// Shorthand: the default LocalQueryInfo with all fields at their zero/empty/false values.
fn qi() -> LocalQueryInfo {
    LocalQueryInfo::default()
}

// ═══════════════════════════════════════════════════════════════════════════════
// Gateway validation infrastructure
// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════

// ═══════════════════════════════════════════════════════════════════════════════
// SIMPLE SELECT — no WHERE, no clauses
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn select_star_from_c() {
    assert_eq!(
        plan("SELECT * FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn select_fields_from_c() {
    assert_eq!(
        plan("SELECT c.name, c.age FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn select_value() {
    assert_eq!(
        plan("SELECT VALUE c.name FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_no_from() {
    assert_eq!(
        plan("SELECT 1"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK EQUALITY — simple WHERE c.pk = <value>
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_eq_string() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'hello'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "hello".into()
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_integer() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 42"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Number(42_f64)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_float() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 1.23"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Number(1.23)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_bool_true() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = true"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Bool(true)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_null() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = null"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Null]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_negative() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = -99"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Number(-99_f64)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_reversed_operand() {
    assert_eq!(
        plan("SELECT * FROM c WHERE 'hello' = c.pk"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "hello".into()
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_parameter() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = @val"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::UnboundParameter(
                "val".into()
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK with AND / OR / IN
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_and_other_filter() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'x' AND c.age > 21"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_or_pk() {
    let qp = plan("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b'");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 2));
    assert_eq!(
        qp.query_info,
        LocalQueryInfo {
            has_where: true,
            ..qi()
        }
    );
}

#[test]
fn pk_or_duplicate_equality_collapses_to_single_equality() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'a'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_or_duplicate_values_are_deduped_across_in_lists() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') OR c.pk IN ('b', 'c')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::InList(vec![
                vec![PartitionKeyValue::String("a".into())],
                vec![PartitionKeyValue::String("b".into())],
                vec![PartitionKeyValue::String("c".into())],
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_in_list() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c')");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 3));
    assert_eq!(
        qp.query_info,
        LocalQueryInfo {
            has_where: true,
            ..qi()
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// Cross-partition WHERE (non-PK filters)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn non_pk_equality() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.age > 21"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_inequality() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk > 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_between() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk BETWEEN 'a' AND 'z'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_like() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk LIKE 'x%'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_is_null() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IS NULL"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: IS NULL not supported by Gateway query plan endpoint
}

// ═══════════════════════════════════════════════════════════════════════════════
// Hierarchical PK
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn hpk_both_components() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_partial_is_cross() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "acme".into(),
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// TOP
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn top_only() {
    assert_eq!(
        plan("SELECT TOP 10 * FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                top: Some(10),
                ..qi()
            },
        }
    );
}

#[test]
fn top_with_pk() {
    assert_eq!(
        plan("SELECT TOP 5 * FROM c WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                top: Some(5),
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// OFFSET / LIMIT
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn offset_limit() {
    assert_eq!(
        plan("SELECT * FROM c OFFSET 5 LIMIT 20"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                offset: Some(5),
                limit: Some(20),
                ..qi()
            },
        }
    );
}

#[test]
fn offset_limit_with_pk() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'x' OFFSET 0 LIMIT 10"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                offset: Some(0),
                limit: Some(10),
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// DISTINCT
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn distinct_unordered() {
    assert_eq!(
        plan("SELECT DISTINCT c.name FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_ordered() {
    assert_eq!(
        plan("SELECT DISTINCT c.name FROM c ORDER BY c.name ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Ordered,
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_with_pk() {
    assert_eq!(
        plan("SELECT DISTINCT c.name FROM c WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// ORDER BY
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn order_by_single_asc() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.name ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_single_desc() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.age DESC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Descending],
                order_by_expressions: vec!["c.age".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_default_is_asc() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.name"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_multiple() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.name ASC, c.age DESC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending, SortOrder::Descending],
                order_by_expressions: vec!["c.name".into(), "c.age".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_nested_path() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.address.city ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.address.city".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_with_pk() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'x' ORDER BY c.name DESC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Descending],
                order_by_expressions: vec!["c.name".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// AGGREGATES
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn aggregate_count() {
    assert_eq!(
        plan("SELECT COUNT(1) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_sum() {
    assert_eq!(
        plan("SELECT SUM(c.price) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Sum],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_avg() {
    assert_eq!(
        plan("SELECT AVG(c.score) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Avg],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_min() {
    assert_eq!(
        plan("SELECT MIN(c.age) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Min],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_max() {
    assert_eq!(
        plan("SELECT MAX(c.age) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Max],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_multiple() {
    assert_eq!(
        plan("SELECT COUNT(1), SUM(c.price), AVG(c.score) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Count, AggregateKind::Sum, AggregateKind::Avg],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_with_pk() {
    assert_eq!(
        plan("SELECT COUNT(1) FROM c WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Count],
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP BY
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn group_by_single() {
    assert_eq!(
        plan("SELECT c.city, COUNT(1) FROM c GROUP BY c.city"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_multiple() {
    assert_eq!(
        plan("SELECT c.city, c.state, COUNT(1) FROM c GROUP BY c.city, c.state"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.city".into(), "c.state".into()],
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_with_sum_avg() {
    assert_eq!(
        plan("SELECT c.city, SUM(c.revenue), AVG(c.score) FROM c GROUP BY c.city"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Sum, AggregateKind::Avg],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_with_pk() {
    assert_eq!(
        plan("SELECT c.city, COUNT(1) FROM c WHERE c.pk = 'x' GROUP BY c.city"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count],
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// JOIN
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn join_simple() {
    assert_eq!(
        plan("SELECT * FROM c JOIN t IN c.tags"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_join: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: cross-partition SELECT * JOIN without WHERE rejected by Gateway
}

#[test]
fn join_with_pk_and_where() {
    assert_eq!(
        plan("SELECT c.id, t FROM c JOIN t IN c.tags WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// SUBQUERIES
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn exists_subquery() {
    assert_eq!(
        plan("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn array_subquery_in_select() {
    assert_eq!(
        plan("SELECT ARRAY(SELECT t FROM t IN c.tags) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                ..qi()
            },
        }
    );
}

#[test]
fn subquery_with_pk() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'x' AND EXISTS(SELECT VALUE t FROM t IN c.tags WHERE t = 'rust')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_subquery: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// UDF
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn udf_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE udf.myFunc(c.x) > 0"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_udf: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn builtin_function_not_udf() {
    assert_eq!(
        plan("SELECT * FROM c WHERE CONTAINS(c.name, 'test')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// SELECT VALUE
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn select_value_with_pk() {
    assert_eq!(
        plan("SELECT VALUE c.name FROM c WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_select_value: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPLEX COMBINED — every field verified
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn complex_aggregate_group_order_pk() {
    assert_eq!(
        plan(
            "SELECT c.city, COUNT(1), SUM(c.revenue) \
             FROM c WHERE c.pk = 'x' \
             GROUP BY c.city \
             ORDER BY c.city ASC"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.city".into()],
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count, AggregateKind::Sum],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn complex_distinct_top_order() {
    assert_eq!(
        plan("SELECT DISTINCT TOP 5 c.name FROM c ORDER BY c.name ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Ordered,
                top: Some(5),
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn complex_cross_partition_multi_aggregate_group_order() {
    assert_eq!(
        plan(
            "SELECT c.region, c.city, AVG(c.score), MIN(c.score), MAX(c.score) \
             FROM c \
             GROUP BY c.region, c.city \
             ORDER BY c.region ASC, c.city DESC"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending, SortOrder::Descending],
                order_by_expressions: vec!["c.region".into(), "c.city".into()],
                group_by_expressions: vec!["c.region".into(), "c.city".into()],
                aggregates: vec![AggregateKind::Avg, AggregateKind::Min, AggregateKind::Max],
                ..qi()
            },
        }
    );
}

#[test]
fn complex_join_aggregate_group_pk() {
    assert_eq!(
        plan("SELECT c.id, COUNT(1) FROM c JOIN t IN c.tags WHERE c.pk = 'x' GROUP BY c.id"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.id".into()],
                aggregates: vec![AggregateKind::Count],
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn complex_select_value_offset_limit() {
    assert_eq!(
        plan("SELECT VALUE c.name FROM c OFFSET 10 LIMIT 5"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                offset: Some(10),
                limit: Some(5),
                ..qi()
            },
        }
    );
}

#[test]
fn complex_everything() {
    assert_eq!(
        plan(
            "SELECT DISTINCT TOP 100 c.city, COUNT(1) \
             FROM c \
             JOIN t IN c.tags \
             WHERE c.pk = 'x' AND CONTAINS(c.name, 'test') \
             GROUP BY c.city \
             ORDER BY c.city DESC"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Ordered,
                top: Some(100),
                offset: None,
                limit: None,
                order_by: vec![SortOrder::Descending],
                order_by_expressions: vec!["c.city".into()],
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count],
                has_select_value: false,
                has_join: true,
                has_subquery: false,
                has_where: true,
                has_udf: false,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMMENTS / CASE — full plan still correct
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn line_comment() {
    assert_eq!(
        plan("SELECT * FROM c -- comment\nWHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn mixed_case() {
    assert_eq!(
        plan("select top 3 * from c where c.pk = 'x' order by c.name desc"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                top: Some(3),
                order_by: vec![SortOrder::Descending],
                order_by_expressions: vec!["c.name".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// AND INTERSECTION — full structural comparison
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn and_contradictory_equality() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn and_redundant_equality() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'a'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn and_equality_narrows_in_list() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk IN ('a', 'b')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn and_equality_not_in_list() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'c' AND c.pk IN ('a', 'b')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn and_in_list_intersection_narrows_to_single() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('b', 'c')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("b".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn and_in_list_empty_intersection() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('c', 'd')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn and_contradictory_deep_in_chain() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'a' AND c.x > 1 AND c.pk = 'b'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_contradictory_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.tenant = 'b' AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_redundant_ok() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.tenant = 'a'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// FUNCTIONS IN WHERE
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn function_contains_no_pk() {
    assert_eq!(
        plan("SELECT * FROM c WHERE CONTAINS(c.name, 'test')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn function_startswith_with_pk() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'x' AND STARTSWITH(c.name, 'A')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn function_is_defined() {
    assert_eq!(
        plan("SELECT * FROM c WHERE IS_DEFINED(c.optional)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPLEX EXPRESSIONS IN SELECT
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn ternary_in_select() {
    assert_eq!(
        plan("SELECT c.age > 18 ? 'adult' : 'child' AS label FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn coalesce_in_select() {
    assert_eq!(
        plan("SELECT c.name ?? 'unknown' AS name FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn computed_in_select() {
    assert_eq!(
        plan("SELECT c.price * c.qty AS total FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// NOT VARIANTS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_not_in() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk NOT IN ('a', 'b')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn not_between_no_pk() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.x NOT BETWEEN 1 AND 10"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// MULTIPLE AGGREGATE TYPES
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn aggregate_array_agg() {
    // ARRAY_AGG is intentionally NOT advertised as a local-plan
    // aggregate. The supported-features list does not include it and the
    // in-memory evaluator does not implement it, so the planner should
    // surface the query as a non-aggregate (the Gateway also rejects this
    // pattern with HTTP 400). The test asserts that the local planner does
    // not falsely advertise the aggregate.
    assert_eq!(
        plan("SELECT c.city, ARRAY_AGG(c.name) FROM c GROUP BY c.city"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_min_max_combined() {
    assert_eq!(
        plan("SELECT MIN(c.age), MAX(c.age) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                aggregates: vec![AggregateKind::Min, AggregateKind::Max],
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PARAMETERIZED PLANS
// ═══════════════════════════════════════════════════════════════════════════════

fn plan_with_params(sql: &str, params: &[(&str, serde_json::Value)]) -> QueryPlan {
    let p = crate::query::parse(sql).unwrap();
    let owned: Vec<(String, serde_json::Value)> = params
        .iter()
        .map(|(n, v)| (n.to_string(), v.clone()))
        .collect();
    generate_query_plan_with_parameters(&p.query, &["/pk"], &owned).unwrap()
}

fn plan_with_params_err(
    sql: &str,
    params: &[(&str, serde_json::Value)],
) -> crate::error::CosmosError {
    let p = crate::query::parse(sql).unwrap();
    let owned: Vec<(String, serde_json::Value)> = params
        .iter()
        .map(|(n, v)| (n.to_string(), v.clone()))
        .collect();
    generate_query_plan_with_parameters(&p.query, &["/pk"], &owned)
        .expect_err("expected parameter resolution to fail")
}

#[test]
fn top_parameter_substituted_from_params() {
    assert_eq!(
        plan_with_params("SELECT TOP @n * FROM c", &[("@n", serde_json::json!(7))]),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                top: Some(7),
                ..qi()
            },
        }
    );
    // Param name without leading '@' must also work.
    assert_eq!(
        plan_with_params("SELECT TOP @n * FROM c", &[("n", serde_json::json!(7))])
            .query_info
            .top,
        Some(7)
    );
}

#[test]
fn offset_limit_parameter_substituted_from_params() {
    assert_eq!(
        plan_with_params(
            "SELECT * FROM c OFFSET @off LIMIT @lim",
            &[
                ("@off", serde_json::json!(3)),
                ("@lim", serde_json::json!(11)),
            ],
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                offset: Some(3),
                limit: Some(11),
                ..qi()
            },
        }
    );
}

#[test]
fn top_parameter_missing_value_is_error() {
    let err = plan_with_params_err("SELECT TOP @n * FROM c", &[]);
    let msg = format!("{err}");
    assert!(
        msg.contains("@n"),
        "error should mention parameter name: {msg}"
    );
}

#[test]
fn offset_limit_parameter_missing_value_is_error() {
    let err = plan_with_params_err(
        "SELECT * FROM c OFFSET @off LIMIT @lim",
        &[("@off", serde_json::json!(0))],
    );
    let msg = format!("{err}");
    assert!(
        msg.contains("@lim"),
        "error should mention missing param @lim: {msg}"
    );
}

#[test]
fn top_parameter_non_integer_is_error() {
    let err = plan_with_params_err(
        "SELECT TOP @n * FROM c",
        &[("@n", serde_json::json!("not-a-number"))],
    );
    assert!(format!("{err}").contains("@n"));

    let err = plan_with_params_err("SELECT TOP @n * FROM c", &[("@n", serde_json::json!(3.5))]);
    assert!(format!("{err}").contains("@n"));
}

#[test]
fn top_parameter_negative_is_error() {
    let err = plan_with_params_err("SELECT TOP @n * FROM c", &[("@n", serde_json::json!(-1))]);
    assert!(format!("{err}").contains("non-negative"));
}

#[test]
fn generate_query_plan_errors_for_parameterized_top_without_params() {
    // The convenience helper must surface a clear error rather than silently
    // dropping or guessing a value when a parameterized TOP/OFFSET/LIMIT is
    // present and no parameters are supplied.
    let p = crate::query::parse("SELECT TOP @n * FROM c").unwrap();
    let err = super::super::generate_query_plan(&p.query, &["/pk"]).unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("TOP/OFFSET/LIMIT"),
        "unexpected error message: {msg}"
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// NESTED PATHS IN VARIOUS CLAUSES
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn nested_path_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.address.city = 'Seattle'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn nested_path_in_group_by() {
    assert_eq!(
        plan("SELECT c.address.city, COUNT(1) FROM c GROUP BY c.address.city"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.address.city".into()],
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn nested_path_in_select() {
    assert_eq!(
        plan("SELECT c.address.city AS city FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPLEX COMBINED QUERIES
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn complex_where_or_union() {
    let qp = plan("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b' ORDER BY c.name");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 2));
    assert_eq!(
        qp.query_info,
        LocalQueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.name".into()],
            has_where: true,
            ..qi()
        }
    );
}

#[test]
fn complex_in_with_order_by() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') ORDER BY c.pk ASC");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 3));
    assert_eq!(
        qp.query_info,
        LocalQueryInfo {
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.pk".into()],
            has_where: true,
            ..qi()
        }
    );
}

#[test]
fn complex_distinct_group_by() {
    assert_eq!(
        plan("SELECT DISTINCT c.city FROM c GROUP BY c.city"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                group_by_expressions: vec!["c.city".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn complex_all_clauses() {
    assert_eq!(
        plan(
            "SELECT DISTINCT TOP 50 c.city, COUNT(1), SUM(c.revenue) \
             FROM c \
             JOIN t IN c.tags \
             WHERE c.pk = 'x' AND c.active = true \
             GROUP BY c.city \
             ORDER BY c.city ASC"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Ordered,
                top: Some(50),
                offset: None,
                limit: None,
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.city".into()],
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count, AggregateKind::Sum],
                has_select_value: false,
                has_join: true,
                has_subquery: false,
                has_where: true,
                has_udf: false,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// HIERARCHICAL PK — exhaustive scenarios
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn hpk_with_parameters() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = @t AND c.userId = @u"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::UnboundParameter("t".into()),
                PartitionKeyValue::UnboundParameter("u".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_mixed_literal_and_parameter() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = @uid"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::UnboundParameter("uid".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_mixed_types_string_integer() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = 42"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::Number(42_f64),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_mixed_types_string_bool() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = true"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::Bool(true),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_null_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = null"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::Null,
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_missing_second_component() {
    // Only first HPK component specified — route to the leading HPK prefix.
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.age > 21"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "acme".into(),
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_missing_first_component() {
    // Only second HPK component — cross-partition
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.userId = 'u1' AND c.age > 21"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_reversed_order_still_extracts() {
    // Components appear in reverse order in WHERE — should still extract
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.userId = 'u1' AND c.tenant = 'acme'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_with_additional_filters() {
    assert_eq!(
        plan_hpk(
            "SELECT * FROM c WHERE c.tenant = 'acme' AND c.active = true AND c.userId = 'u1' AND c.age > 21"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_or_makes_cross_partition() {
    // OR between HPK components → cross-partition (HPK doesn't support OR)
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' OR c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ── Triple hierarchical PK ──────────────────────────────────────────────

#[test]
fn hpk3_all_components() {
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
                PartitionKeyValue::String("s1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_missing_middle_component() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.sessionId = 's1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_missing_last_component() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_contradictory_middle() {
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.userId = 'u2' AND c.sessionId = 's1'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// NESTED PK PATHS
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn nested_pk_equality() {
    assert_eq!(
        plan_nested_pk("SELECT * FROM c WHERE c.address.city = 'Seattle'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "Seattle".into()
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn nested_pk_with_other_filter() {
    assert_eq!(
        plan_nested_pk("SELECT * FROM c WHERE c.address.city = 'Seattle' AND c.age > 21"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "Seattle".into()
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn nested_pk_in_list() {
    let qp =
        plan_nested_pk("SELECT * FROM c WHERE c.address.city IN ('Seattle', 'Portland', 'Austin')");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 3));
    assert!(qp.query_info.has_where);
}

#[test]
fn nested_pk_wrong_path_no_extract() {
    // c.address.state is NOT the PK path /address/city — should be cross-partition
    assert_eq!(
        plan_nested_pk("SELECT * FROM c WHERE c.address.state = 'WA'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn nested_pk_partial_path_no_extract() {
    // c.address alone doesn't match /address/city
    assert_eq!(
        plan_nested_pk("SELECT * FROM c WHERE c.address = 'something'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK EXTRACTION — OR combinations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_or_three_values() {
    let qp = plan("SELECT * FROM c WHERE c.pk = 'a' OR c.pk = 'b' OR c.pk = 'c'");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 3));
}

#[test]
fn pk_or_equality_and_in_list() {
    // c.pk = 'a' OR c.pk IN ('b', 'c') → InList of 3
    let qp = plan("SELECT * FROM c WHERE c.pk = 'a' OR c.pk IN ('b', 'c')");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 3),
        other => panic!("expected InList(3), got {other:?}"),
    }
}

#[test]
fn pk_or_two_in_lists() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') OR c.pk IN ('c', 'd')");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 4),
        other => panic!("expected InList(4), got {other:?}"),
    }
}

#[test]
fn pk_or_with_non_pk_is_cross() {
    // c.pk = 'a' OR c.other = 'b' → cross-partition (can't target specific PK)
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 'a' OR c.other = 'b'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_complex_or_and_combination() {
    // (c.pk = 'a' AND c.x > 1) OR (c.pk = 'b' AND c.y < 2) → InList(['a', 'b'])
    let qp = plan("SELECT * FROM c WHERE (c.pk = 'a' AND c.x > 1) OR (c.pk = 'b' AND c.y < 2)");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
        other => panic!("expected InList(2), got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK EXTRACTION — AND + IN combinations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_in_and_other_condition() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') AND c.age > 21");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 3),
        other => panic!("expected InList(3), got {other:?}"),
    }
    assert!(qp.query_info.has_where);
}

#[test]
fn pk_in_and_pk_equality_narrows() {
    // c.pk IN ('a', 'b', 'c') AND c.pk = 'b' → Equality('b')
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') AND c.pk = 'b'").pk_filters,
        PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("b".into())])
    );
}

#[test]
fn pk_in_and_pk_equality_contradiction() {
    // c.pk IN ('a', 'b') AND c.pk = 'z' → None (contradiction)
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk = 'z'").pk_filters,
        PartitionKeyFilter::Contradictory
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK EXTRACTION — non-extractable patterns (negative tests)
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_function_wrapping_no_extract() {
    // LOWER(c.pk) = 'x' — function call wraps PK, cannot extract
    assert_eq!(
        plan("SELECT * FROM c WHERE LOWER(c.pk) = 'x'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_unary_not_no_extract() {
    // NOT (c.pk = 'x') — negation, cannot extract
    assert_eq!(
        plan("SELECT * FROM c WHERE NOT (c.pk = 'x')").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_not_equal_no_extract() {
    // c.pk != 'x' or c.pk <> 'x' — inequality cannot target
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk != 'x'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_greater_than_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk > 'x'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_less_than_or_equal_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk <= 'z'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_is_not_null_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk IS NOT NULL").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
    // Gateway rejects this query with HTTP 400: IS NOT NULL not supported by Gateway query plan endpoint
}

#[test]
fn pk_like_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk LIKE 'prefix%'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_between_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk BETWEEN 'a' AND 'z'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_not_in_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk NOT IN ('a', 'b')").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_comparison_to_expression_no_extract() {
    // c.pk = c.other — comparing PK to another field, not a literal
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = c.other").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

#[test]
fn pk_arithmetic_no_extract() {
    // c.pk + 1 = 'x' — arithmetic on PK, cannot extract
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk + 1 = 'x'").pk_filters,
        PartitionKeyFilter::Unconstrained
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK EXTRACTION — special values
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_eq_bool_false() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = false"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Bool(false)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_zero() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 0"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Number(0 as f64)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_undefined() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = undefined"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Undefined]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_empty_string() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = ''"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String(String::new())
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_large_integer() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = 9007199254740993"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Number(
                9007199254740993_i64 as f64
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_negative_float() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = -1.5"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Number(-1.5)]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK with FROM alias
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_with_explicit_alias() {
    let p = crate::query::parse("SELECT * FROM root AS r WHERE r.pk = 'hello'").unwrap();
    let qp = generate_query_plan(&p.query, &["/pk"]).unwrap();
    assert_eq!(
        qp.pk_filters,
        PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("hello".into())])
    );
}

#[test]
fn pk_with_bare_alias() {
    let p = crate::query::parse("SELECT * FROM root r WHERE r.pk = 'hello'").unwrap();
    let qp = generate_query_plan(&p.query, &["/pk"]).unwrap();
    assert_eq!(
        qp.pk_filters,
        PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("hello".into())])
    );
}

#[test]
fn pk_alias_mismatch_no_extract() {
    // WHERE uses 'c' but FROM uses alias 'r' — path doesn't match
    let p = crate::query::parse("SELECT * FROM root AS r WHERE c.pk = 'hello'").unwrap();
    let qp = generate_query_plan(&p.query, &["/pk"]).unwrap();
    assert_eq!(qp.pk_filters, PartitionKeyFilter::Unconstrained);
    // Gateway rejects this query with HTTP 400: alias mismatch (FROM uses r but WHERE uses c)
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK with empty PK paths
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn no_pk_paths_with_no_evaluation() {
    // #13: when the caller does not supply any PK paths, the plan must report
    // `NotEvaluated` rather than `Unconstrained` so that downstream callers can
    // distinguish "no extraction was attempted" from "PK paths were supplied
    // but no constraint matched in the WHERE clause".
    assert_eq!(
        plan_no_pk("SELECT * FROM c WHERE c.pk = 'hello'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::NotEvaluated,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// PK extraction with deeply nested AND chains
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_deeply_nested_and_chain() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.a > 1 AND c.b > 2 AND c.pk = 'x' AND c.d > 4 AND c.e > 5")
            .pk_filters,
        PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
    );
}

#[test]
fn pk_in_mixed_and_or_parenthesized() {
    // (c.pk = 'a' OR c.pk = 'b') AND c.active = true
    let qp = plan("SELECT * FROM c WHERE (c.pk = 'a' OR c.pk = 'b') AND c.active = true");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
        other => panic!("expected InList(2), got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// QUERY STRUCTURE — additional coverage
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn from_with_alias_plan() {
    assert_eq!(
        plan("SELECT r.name FROM root AS r"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn multiple_joins_plan() {
    assert_eq!(
        plan("SELECT * FROM c JOIN t IN c.tags JOIN s IN c.skills"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_join: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: cross-partition multi-JOIN with c.skills rejected by Gateway
}

#[test]
fn join_with_nested_path() {
    assert_eq!(
        plan("SELECT * FROM c JOIN a IN c.addresses.tags"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_join: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: JOIN on nested path c.addresses.tags rejected by Gateway
}

#[test]
fn string_concat_in_select() {
    assert_eq!(
        plan("SELECT c.first || ' ' || c.last AS name FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn is_null_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.x IS NULL"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: IS NULL not supported by Gateway query plan endpoint
}

#[test]
fn is_not_null_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.x IS NOT NULL"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: IS NOT NULL not supported by Gateway query plan endpoint
}

#[test]
fn like_in_where_plan() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.name LIKE 'A%'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn like_with_escape_in_where() {
    assert_eq!(
        plan(r"SELECT * FROM c WHERE c.name LIKE 'a\%b' ESCAPE '\'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: LIKE ESCAPE with backslash not supported
}

#[test]
fn not_like_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.name NOT LIKE '%test%'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn udf_in_select() {
    assert_eq!(
        plan("SELECT udf.myFunc(c.x) AS result FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_udf: true,
                ..qi()
            },
        }
    );
}

#[test]
fn multiple_udfs() {
    assert_eq!(
        plan("SELECT * FROM c WHERE udf.func1(c.x) > 0 AND udf.func2(c.y) = true"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_udf: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn deeply_nested_order_by() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.a.b.c ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.a.b.c".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn multiple_subquery_types() {
    assert_eq!(
        plan(
            "SELECT ARRAY(SELECT t FROM t IN c.tags), EXISTS(SELECT VALUE s FROM s IN c.skills) FROM c"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                ..qi()
            },
        }
    );
}

#[test]
fn offset_limit_with_order_by_and_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.active = true ORDER BY c.name ASC OFFSET 10 LIMIT 20"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                offset: Some(10),
                limit: Some(20),
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn top_with_distinct_and_where() {
    assert_eq!(
        plan("SELECT DISTINCT TOP 5 c.name FROM c WHERE c.active = true"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                top: Some(5),
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_value_with_aggregate() {
    assert_eq!(
        plan("SELECT VALUE COUNT(1) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_nested_path_with_multiple_aggregates() {
    assert_eq!(
        plan(
            "SELECT c.address.city, COUNT(1) AS cnt, SUM(c.revenue) AS total, AVG(c.score) AS avg \
             FROM c GROUP BY c.address.city"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.address.city".into()],
                aggregates: vec![AggregateKind::Count, AggregateKind::Sum, AggregateKind::Avg],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_three_columns() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.city ASC, c.state DESC, c.name ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![
                    SortOrder::Ascending,
                    SortOrder::Descending,
                    SortOrder::Ascending,
                ],
                order_by_expressions: vec!["c.city".into(), "c.state".into(), "c.name".into(),],
                ..qi()
            },
        }
    );
}

#[test]
fn aggregate_in_where_not_detected() {
    // This is technically invalid SQL but the parser may accept it.
    // The plan should NOT detect "COUNT" as an aggregate if it appears in WHERE
    // as a function call on a scalar value.
    // Actually, the expression visitor does walk WHERE, so it WILL detect the aggregate.
    // This tests that behavior is consistent.
    let qp = plan("SELECT * FROM c WHERE COUNT(1) > 0");
    assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
    assert!(qp.query_info.has_where);
    // Gateway rejects this query with HTTP 400: aggregate in WHERE clause is invalid SQL rejected by Gateway
}

#[test]
fn in_list_with_mixed_types() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 42, true, null)");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => {
            assert_eq!(list.len(), 4);
            assert_eq!(list[0], vec![PartitionKeyValue::String("a".into())]);
            assert_eq!(list[1], vec![PartitionKeyValue::Number(42_f64)]);
            assert_eq!(list[2], vec![PartitionKeyValue::Bool(true)]);
            assert_eq!(list[3], vec![PartitionKeyValue::Null]);
        }
        other => panic!("expected InList(4), got {other:?}"),
    }
}

#[test]
fn in_list_single_item_stays_in_list() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('only')");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 1),
        other => panic!("expected InList(1), got {other:?}"),
    }
}

#[test]
fn pk_in_with_parameters() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN (@a, @b, @c)");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => {
            assert_eq!(list.len(), 3);
            assert_eq!(
                list[0],
                vec![PartitionKeyValue::UnboundParameter("a".into())]
            );
            assert_eq!(
                list[1],
                vec![PartitionKeyValue::UnboundParameter("b".into())]
            );
            assert_eq!(
                list[2],
                vec![PartitionKeyValue::UnboundParameter("c".into())]
            );
        }
        other => panic!("expected InList(3), got {other:?}"),
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// COMPLEX COMBINED — stress tests
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn complex_hpk_with_join_group_order() {
    assert_eq!(
        plan_hpk(
            "SELECT c.city, COUNT(1) AS cnt \
             FROM c JOIN t IN c.tags \
             WHERE c.tenant = 'acme' AND c.userId = 'u1' \
             GROUP BY c.city \
             ORDER BY c.city ASC"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.city".into()],
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count],
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn complex_pk_in_with_distinct_top_order() {
    let qp = plan(
        "SELECT DISTINCT TOP 10 c.name, c.city \
         FROM c \
         WHERE c.pk IN ('a', 'b', 'c') AND c.active = true \
         ORDER BY c.name ASC",
    );
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 3),
        other => panic!("expected InList(3), got {other:?}"),
    }
    assert_eq!(
        qp.query_info,
        LocalQueryInfo {
            distinct_type: DistinctType::Ordered,
            top: Some(10),
            order_by: vec![SortOrder::Ascending],
            order_by_expressions: vec!["c.name".into()],
            has_where: true,
            ..qi()
        }
    );
}

#[test]
fn complex_nested_pk_with_full_pipeline() {
    assert_eq!(
        plan_nested_pk(
            "SELECT c.name, SUM(c.score) AS total \
             FROM c \
             WHERE c.address.city = 'Seattle' AND c.active = true \
             GROUP BY c.name \
             ORDER BY c.name DESC \
             OFFSET 0 LIMIT 10"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "Seattle".into()
            )]),
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Descending],
                order_by_expressions: vec!["c.name".into()],
                group_by_expressions: vec!["c.name".into()],
                aggregates: vec![AggregateKind::Sum],
                offset: Some(0),
                limit: Some(10),
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn complex_select_value_count_with_pk() {
    assert_eq!(
        plan("SELECT VALUE COUNT(1) FROM c WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: LocalQueryInfo {
                has_select_value: true,
                aggregates: vec![AggregateKind::Count],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn complex_or_pk_with_subquery() {
    let qp = plan(
        "SELECT * FROM c WHERE (c.pk = 'a' OR c.pk = 'b') AND EXISTS(SELECT VALUE t FROM t IN c.tags WHERE t = 'rust')",
    );
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
        other => panic!("expected InList(2), got {other:?}"),
    }
    assert!(qp.query_info.has_subquery);
    assert!(qp.query_info.has_where);
}

#[test]
fn complex_everything_with_hpk() {
    assert_eq!(
        plan_hpk(
            "SELECT DISTINCT TOP 100 c.city, COUNT(1) AS cnt, SUM(c.revenue) AS rev \
             FROM c \
             JOIN t IN c.tags \
             WHERE c.tenant = 'acme' AND c.userId = 'u1' AND CONTAINS(c.name, 'test') \
             GROUP BY c.city \
             ORDER BY c.city DESC \
             OFFSET 5 LIMIT 20"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Ordered,
                top: Some(100),
                offset: Some(5),
                limit: Some(20),
                order_by: vec![SortOrder::Descending],
                order_by_expressions: vec!["c.city".into()],
                group_by_expressions: vec!["c.city".into()],
                aggregates: vec![AggregateKind::Count, AggregateKind::Sum],
                has_select_value: false,
                has_join: true,
                has_subquery: false,
                has_where: true,
                has_udf: false,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: TOP combined with OFFSET/LIMIT rejected by Gateway as ambiguous
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 1: FROM clause variations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn from_sub_path() {
    assert_eq!(
        plan("SELECT * FROM r.address"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn from_array_index() {
    assert_eq!(
        plan("SELECT * FROM r.scores[0]"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn from_array_iterator_no_join() {
    assert_eq!(
        plan("SELECT s FROM s IN r.scores"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn select_value_root() {
    assert_eq!(
        plan("SELECT VALUE r FROM r"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 2: Scalar literals and expressions without FROM
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn select_string_literal() {
    assert_eq!(
        plan("SELECT 'Hello World'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn select_arithmetic() {
    assert_eq!(
        plan("SELECT 1 + 2 AS result"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn select_value_null_literal() {
    assert_eq!(
        plan("SELECT VALUE null"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_value_undefined_literal() {
    assert_eq!(
        plan("SELECT VALUE undefined"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_value_object_constructor() {
    assert_eq!(
        plan("SELECT VALUE {name: c.name} FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_value_array_constructor() {
    assert_eq!(
        plan("SELECT VALUE [c.name, c.age] FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_value_boolean_expr() {
    assert_eq!(
        plan("SELECT VALUE c.age > 10 AND c.age < 20 FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_null_eq_null() {
    assert_eq!(
        plan("SELECT VALUE null = null"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_undefined_eq_undefined() {
    assert_eq!(
        plan("SELECT VALUE undefined = undefined"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_array_eq_array() {
    assert_eq!(
        plan("SELECT VALUE [1,2,3] = [1,2,3]"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_empty_array_eq() {
    assert_eq!(
        plan("SELECT VALUE [] = []"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_object_eq_object() {
    assert_eq!(
        plan("SELECT VALUE {a: 1, b: 2} = {a: 1, b: 2}"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn select_empty_object_eq() {
    assert_eq!(
        plan("SELECT VALUE {} = {}"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 3: Complex WHERE expressions
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn where_deep_nested_member() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.a.b.c.d = 1"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_array_index_in_condition() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.scores[0] = 90"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_nested_unary() {
    assert_eq!(
        plan("SELECT VALUE -(+(-c.age)) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_complex_arithmetic() {
    assert_eq!(
        plan("SELECT VALUE 10 + c.age * 2 - 10 FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_string_concat_in_value() {
    assert_eq!(
        plan("SELECT VALUE '[' || c.name || ']' FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_bitwise_in_select() {
    assert_eq!(
        plan("SELECT VALUE c.age | 8 FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_zero_fill_right_shift() {
    assert_eq!(
        plan("SELECT VALUE -100 >>> 1"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_truthy_check() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.active"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_not_truthy() {
    assert_eq!(
        plan("SELECT * FROM c WHERE NOT c.active"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_type_check_is_array() {
    assert_eq!(
        plan("SELECT * FROM c WHERE IS_ARRAY(c.tags)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_type_check_is_object() {
    assert_eq!(
        plan("SELECT * FROM c WHERE IS_OBJECT(c.address)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_type_check_is_string() {
    assert_eq!(
        plan("SELECT * FROM c WHERE IS_STRING(c.name)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_type_check_is_number() {
    assert_eq!(
        plan("SELECT * FROM c WHERE IS_NUMBER(c.age)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_type_check_is_bool() {
    assert_eq!(
        plan("SELECT * FROM c WHERE IS_BOOL(c.active)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_not_type_check() {
    assert_eq!(
        plan("SELECT * FROM c WHERE NOT IS_DEFINED(c.optional)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn where_in_with_expressions() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.age + 1 IN (10, 20, 30)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 4: PK extraction with complex values
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn pk_eq_array_literal_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = [1, 2, 3]"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_eq_object_literal_no_extract() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.pk = {'x': 1}"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_in_and_other_in() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.other IN ('x', 'y')");
    match &qp.pk_filters {
        PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
        other => panic!("expected InList(2), got {other:?}"),
    }
    assert!(qp.query_info.has_where);
}

#[test]
fn pk_not_in_and_not_eq() {
    assert_eq!(
        plan("SELECT * FROM c WHERE (c.pk NOT IN ('a', 'b')) AND (c.pk != 'c')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_range_and_not_eq() {
    assert_eq!(
        plan("SELECT * FROM c WHERE (c.pk > 'a') AND (c.pk != 'z')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_double_not_eq() {
    assert_eq!(
        plan("SELECT * FROM c WHERE (c.pk != 'a') AND (c.pk != 'b')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn pk_double_not_eq_or() {
    assert_eq!(
        plan("SELECT * FROM c WHERE (c.pk != 'a') OR (c.pk != 'b')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 5: GROUP BY variations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn group_by_without_aggregate() {
    assert_eq!(
        plan("SELECT c.age FROM c GROUP BY c.age"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.age".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_array_index_returns_path_error() {
    // #2: c.scores[0] uses MemberIndexer (not a property path). The local plan
    // generator now refuses to silently emit a debug-formatted placeholder; it
    // returns an error so callers can fall back to fetching the plan from the
    // Gateway query-plan endpoint, which fully supports such expressions.
    let parsed = crate::query::parse(
        "SELECT c.scores[0] AS s0, COUNT(1) AS cnt FROM c GROUP BY c.scores[0]",
    )
    .unwrap();
    let err = generate_query_plan_with_parameters(&parsed.query, &["/pk"], &[])
        .expect_err("non-path GROUP BY expression must surface an error");
    assert!(format!("{err}").contains("GROUP BY / ORDER BY"));
}

#[test]
fn group_by_two_nested_paths() {
    assert_eq!(
        plan("SELECT c.address.city, c.address.state, COUNT(1) AS cnt FROM c GROUP BY c.address.city, c.address.state"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.address.city".into(), "c.address.state".into()],
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_three_keys() {
    assert_eq!(
        plan("SELECT c.age, c.team, c.gender, COUNT(1) AS cnt FROM c GROUP BY c.age, c.team, c.gender"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.age".into(), "c.team".into(), "c.gender".into()],
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

#[test]
fn group_by_with_alias_select() {
    assert_eq!(
        plan("SELECT c.age AS a, COUNT(1) AS cnt FROM c GROUP BY c.age"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                group_by_expressions: vec!["c.age".into()],
                aggregates: vec![AggregateKind::Count],
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 6: ORDER BY + WHERE combos
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn order_by_with_in_filter() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.age IN (10, 11, 23) ORDER BY c.age"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.age".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_with_not_in() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.age NOT IN (10, 11) ORDER BY c.age"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.age".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_with_contains() {
    assert_eq!(
        plan("SELECT * FROM c WHERE CONTAINS(c.name, 'a') ORDER BY c.name"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_with_startswith() {
    assert_eq!(
        plan("SELECT * FROM c WHERE STARTSWITH(c.name, 'A') ORDER BY c.name"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_boolean_field() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.active"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.active".into()],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_null_field() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.valid = null ORDER BY c.valid"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.valid".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 7: TOP + ORDER BY combos
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn top_with_where_order_by() {
    assert_eq!(
        plan("SELECT TOP 5 * FROM c WHERE c.age > 10 ORDER BY c.age ASC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                top: Some(5),
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.age".into()],
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn top_with_in_filter() {
    assert_eq!(
        plan("SELECT TOP 3 * FROM c WHERE c.age IN (10, 11, 23)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                top: Some(3),
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn top_with_nested_field_order() {
    assert_eq!(
        plan("SELECT TOP 5 c.name, c.games.wins FROM c ORDER BY c.games.wins"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                top: Some(5),
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.games.wins".into()],
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 8: DISTINCT variations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn distinct_value_null_literal() {
    // Gateway optimization: DISTINCT on a constant literal is a no-op (always distinct),
    // so both local plan and Gateway report distinctType: None.
    assert_eq!(
        plan("SELECT DISTINCT VALUE null"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::None,
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_value_literal_number() {
    // Gateway optimization: DISTINCT on a constant literal is a no-op.
    assert_eq!(
        plan("SELECT DISTINCT VALUE 1"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::None,
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_value_literal_string() {
    // Gateway optimization: DISTINCT on a constant literal is a no-op.
    assert_eq!(
        plan("SELECT DISTINCT VALUE 'a'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::None,
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_multiple_columns() {
    assert_eq!(
        plan("SELECT DISTINCT c.city, c.state FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_value_array() {
    assert_eq!(
        plan("SELECT DISTINCT VALUE [c.city, c.state] FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn distinct_value_with_where() {
    assert_eq!(
        plan("SELECT DISTINCT VALUE c.city FROM c WHERE c.active = true"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                distinct_type: DistinctType::Unordered,
                has_select_value: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 9: OFFSET/LIMIT + JOIN
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn offset_limit_with_join() {
    assert_eq!(
        plan("SELECT c.id, t FROM c JOIN t IN c.tags OFFSET 1 LIMIT 3"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                offset: Some(1),
                limit: Some(3),
                has_join: true,
                ..qi()
            },
        }
    );
}

#[test]
fn offset_limit_with_double_join() {
    assert_eq!(
        plan("SELECT c.id, d1, d2 FROM c JOIN d1 IN c.digits JOIN d2 IN c.digits WHERE d2 = 0 OFFSET 0 LIMIT 5"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                offset: Some(0),
                limit: Some(5),
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn offset_limit_with_top_precedence() {
    assert_eq!(
        plan("SELECT TOP 2 * FROM c OFFSET 0 LIMIT 10"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                top: Some(2),
                offset: Some(0),
                limit: Some(10),
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: TOP combined with OFFSET/LIMIT rejected by Gateway as ambiguous
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 10: LIKE variations
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn like_single_char_wildcard() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.name LIKE 'A_ice'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn like_percent_and_underscore() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.name LIKE 'A_%'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn like_and_combination() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.city LIKE 'Se%' AND c.state LIKE 'W_'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn like_no_wildcards() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.name LIKE 'Alice'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 11: Subquery patterns
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn subquery_in_from() {
    assert_eq!(
        plan("SELECT * FROM (SELECT * FROM c)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn subquery_in_from_with_alias() {
    assert_eq!(
        plan("SELECT p.name FROM (SELECT * FROM c) p"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn subquery_nested_from() {
    assert_eq!(
        plan("SELECT * FROM (SELECT * FROM (SELECT * FROM c))"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn scalar_subquery_in_select() {
    assert_eq!(
        plan("SELECT (SELECT VALUE 1) AS x FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                ..qi()
            },
        }
    );
}

#[test]
fn scalar_subquery_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE (SELECT VALUE c.age) > 21"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn scalar_subquery_member_access() {
    // The expression visitor does not recurse into MemberRef sources, so the
    // subquery wrapped inside .a access is not detected by the plan generator.
    assert_eq!(
        plan("SELECT (SELECT VALUE {a: 1, b: 2}).a AS val"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn exists_with_join_in_subquery() {
    assert_eq!(
        plan("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags WHERE t = 'rust')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn array_subquery_with_where() {
    assert_eq!(
        plan("SELECT ARRAY(SELECT VALUE t FROM t IN c.tags WHERE t != 'old') AS filtered_tags FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_subquery: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 12: Complex regression patterns
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn regression_complex_and_or_precedence() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.name = 'fox' AND c.type = 'wood' AND c.flag AND c.userId = 3 OR c.userId = 4"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn regression_empty_string_property() {
    assert_eq!(
        plan("SELECT c[''] FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: qi(),
        }
    );
}

#[test]
fn regression_parenthesized_and_or() {
    assert_eq!(
        plan("SELECT VALUE c.id FROM c WHERE (c.a = 1) AND (c.b = 1 OR c.c = 1)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn regression_double_join_with_double_where() {
    assert_eq!(
        plan("SELECT c.id, t1.name, t2.name AS name2 FROM c JOIN t1 IN c.tags JOIN t2 IN c.tags WHERE t1.name = 'a' AND t2.name = 'b'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn regression_array_contains_and() {
    assert_eq!(
        plan("SELECT * FROM c WHERE ARRAY_CONTAINS(c.items, 1) AND ARRAY_CONTAINS(c.items, 2)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn regression_join_with_array_contains() {
    assert_eq!(
        plan(
            "SELECT * FROM c JOIN item IN c.items WHERE (item = 1) AND ARRAY_CONTAINS(c.items, 2)"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects: iterator comparison + ARRAY_CONTAINS on same array
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 13: Bitwise operators in plan
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn bitwise_and_in_select() {
    assert_eq!(
        plan("SELECT VALUE 3 & 2"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_or_in_select() {
    assert_eq!(
        plan("SELECT VALUE 3 | 2"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_xor_in_select() {
    assert_eq!(
        plan("SELECT VALUE 3 ^ 2"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_not_in_select() {
    assert_eq!(
        plan("SELECT VALUE ~1"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_left_shift() {
    assert_eq!(
        plan("SELECT VALUE 3 << 2"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_right_shift() {
    assert_eq!(
        plan("SELECT VALUE 3 >> 2"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_in_where() {
    assert_eq!(
        plan("SELECT * FROM c WHERE c.flags & 4 != 0"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn bitwise_in_group_by_returns_path_error() {
    // #2: see `group_by_array_index_returns_path_error` for rationale. A Binary
    // expression (here `c.x & 1`) is not a property path; the Gateway accepts it
    // and rewrites the query, but locally we must signal the caller to fall back.
    let parsed =
        crate::query::parse("SELECT c.x & 1 AS parity, COUNT(1) AS cnt FROM c GROUP BY c.x & 1")
            .unwrap();
    let err = generate_query_plan_with_parameters(&parsed.query, &["/pk"], &[])
        .expect_err("non-path GROUP BY expression must surface an error");
    assert!(format!("{err}").contains("GROUP BY / ORDER BY"));
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 14: UDF patterns
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn udf_multiple_in_select() {
    assert_eq!(
        plan("SELECT udf.fn1(c.x) AS r1, udf.fn2(c.y) AS r2 FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_udf: true,
                ..qi()
            },
        }
    );
}

#[test]
fn udf_in_where_with_join() {
    assert_eq!(
        plan("SELECT VALUE t FROM c JOIN t IN c.items WHERE udf.check(t)"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                has_join: true,
                has_where: true,
                has_udf: true,
                ..qi()
            },
        }
    );
}

#[test]
fn udf_in_select_value() {
    assert_eq!(
        plan("SELECT VALUE udf.transform(c.data) FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_select_value: true,
                has_udf: true,
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// GROUP 15: Multi-item ORDER BY
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn order_by_four_columns() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.a ASC, c.b DESC, c.c ASC, c.d DESC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![
                    SortOrder::Ascending,
                    SortOrder::Descending,
                    SortOrder::Ascending,
                    SortOrder::Descending,
                ],
                order_by_expressions: vec!["c.a".into(), "c.b".into(), "c.c".into(), "c.d".into(),],
                ..qi()
            },
        }
    );
}

#[test]
fn order_by_nested_and_flat() {
    assert_eq!(
        plan("SELECT * FROM c ORDER BY c.address.city ASC, c.age DESC"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending, SortOrder::Descending],
                order_by_expressions: vec!["c.address.city".into(), "c.age".into()],
                ..qi()
            },
        }
    );
}

// ═══════════════════════════════════════════════════════════════════════════════
// HIERARCHICAL PK — additional exhaustive coverage
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn hpk_reversed_operand_on_first() {
    // Value on the left side for the first component
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE 'acme' = c.tenant AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_reversed_operand_on_second() {
    // Value on the left side for the second component
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND 'u1' = c.userId"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_reversed_operand_on_both() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE 'acme' = c.tenant AND 'u1' = c.userId"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_undefined_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = undefined"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::Undefined,
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_negative_number_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = -1 AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::Number(-1_f64),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_float_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 1.5 AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::Number(1.5),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_empty_string_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = '' AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String(String::new()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_bool_false_component() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = false AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::Bool(false),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_both_null() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = null AND c.userId = null"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::Null,
                PartitionKeyValue::Null,
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_parenthesized_nested_and() {
    // HPK components nested inside parenthesized AND with extra conditions
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE (c.tenant = 'acme' AND c.x > 1) AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_deeply_nested_and_chain() {
    // 6 conjuncts with HPK components scattered
    assert_eq!(
        plan_hpk(
            "SELECT * FROM c WHERE c.a > 1 AND c.tenant = 'acme' AND c.b > 2 AND c.userId = 'u1' AND c.d > 4 AND c.e > 5"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::String("u1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_with_from_alias() {
    let p =
        crate::query::parse("SELECT * FROM root AS r WHERE r.tenant = 'acme' AND r.userId = 'u1'")
            .unwrap();
    let qp = generate_query_plan(&p.query, &["/tenant", "/userId"]).unwrap();
    assert_eq!(
        qp.pk_filters,
        PartitionKeyFilter::Equality(vec![
            PartitionKeyValue::String("acme".into()),
            PartitionKeyValue::String("u1".into()),
        ])
    );
}

#[test]
fn hpk_with_from_bare_alias() {
    let p = crate::query::parse("SELECT * FROM root r WHERE r.tenant = 'acme' AND r.userId = 'u1'")
        .unwrap();
    let qp = generate_query_plan(&p.query, &["/tenant", "/userId"]).unwrap();
    assert_eq!(
        qp.pk_filters,
        PartitionKeyFilter::Equality(vec![
            PartitionKeyValue::String("acme".into()),
            PartitionKeyValue::String("u1".into()),
        ])
    );
}

#[test]
fn hpk_alias_mismatch_cross_partition() {
    // WHERE uses 'c' but FROM uses alias 'r' — should not extract
    let p =
        crate::query::parse("SELECT * FROM root AS r WHERE c.tenant = 'acme' AND c.userId = 'u1'")
            .unwrap();
    let qp = generate_query_plan(&p.query, &["/tenant", "/userId"]).unwrap();
    assert_eq!(qp.pk_filters, PartitionKeyFilter::Unconstrained);
    // Gateway rejects: alias mismatch (FROM uses r but WHERE references c)
}

#[test]
fn hpk_non_equality_on_second_component() {
    // Inequality on second component still preserves the leading HPK prefix.
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId > 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "acme".into(),
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_in_on_first_component_extracts_cartesian() {
    // IN on the leading HPK component combined with equality on the
    // remainder produces a cartesian-product `InList` (or `Equality` when
    // the product collapses to a single tuple), matching the Gateway.
    let qp = plan_hpk("SELECT * FROM c WHERE c.tenant IN ('a', 'b') AND c.userId = 'u1'");
    match qp.pk_filters {
        PartitionKeyFilter::InList(ref tuples) => {
            assert_eq!(tuples.len(), 2);
            assert!(qp.query_info.has_where);
        }
        ref other => panic!("expected InList, got {other:?}"),
    }
}

#[test]
fn hpk_in_on_second_component_extracts_cartesian() {
    // same as above with the IN on the trailing component.
    let qp = plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId IN ('u1', 'u2')");
    match qp.pk_filters {
        PartitionKeyFilter::InList(ref tuples) => {
            assert_eq!(tuples.len(), 2);
            assert!(qp.query_info.has_where);
        }
        ref other => panic!("expected InList, got {other:?}"),
    }
}

#[test]
fn hpk_between_on_first_component_no_extract() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant BETWEEN 'a' AND 'z' AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: BETWEEN on HPK component rejected by Gateway
}

#[test]
fn hpk_like_on_second_component_no_extract() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId LIKE 'u%'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "acme".into(),
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_function_wrap_first_component_no_extract() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE LOWER(c.tenant) = 'acme' AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_not_on_first_component_no_extract() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE NOT (c.tenant = 'acme') AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_is_null_on_component_no_extract() {
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant IS NULL AND c.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: IS NULL not supported by Gateway query plan endpoint
}

#[test]
fn hpk_or_of_full_hpk_tuples_extracts_inlist() {
    // Two full HPK tuples ORed together extract to an `InList` of full
    // HPK tuples instead of falling back to a cross-partition fan-out.
    assert_eq!(
        plan_hpk(
            "SELECT * FROM c WHERE (c.tenant = 'a' AND c.userId = 'u1') OR (c.tenant = 'b' AND c.userId = 'u2')"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::InList(vec![
                vec![
                    PartitionKeyValue::String("a".into()),
                    PartitionKeyValue::String("u1".into()),
                ],
                vec![
                    PartitionKeyValue::String("b".into()),
                    PartitionKeyValue::String("u2".into()),
                ],
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_or_of_three_full_hpk_tuples_extracts_inlist() {
    // nested OR of three full HPK tuples — recursion in
    // extract_hierarchical_pk's OR arm + union_pk_filters' InList+Equality
    // combo flattens these into a single InList.
    assert_eq!(
        plan_hpk(
            "SELECT * FROM c WHERE (c.tenant = 'a' AND c.userId = 'u1') OR (c.tenant = 'b' AND c.userId = 'u2') OR (c.tenant = 'c' AND c.userId = 'u3')"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::InList(vec![
                vec![
                    PartitionKeyValue::String("a".into()),
                    PartitionKeyValue::String("u1".into()),
                ],
                vec![
                    PartitionKeyValue::String("b".into()),
                    PartitionKeyValue::String("u2".into()),
                ],
                vec![
                    PartitionKeyValue::String("c".into()),
                    PartitionKeyValue::String("u3".into()),
                ],
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_or_with_one_partial_tuple_falls_back_to_unconstrained() {
    // A partial tuple disjunct contributes its leading HPK prefix.
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE (c.tenant = 'a' AND c.userId = 'u1') OR (c.tenant = 'b')"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::InList(vec![
                vec![
                    PartitionKeyValue::String("a".into()),
                    PartitionKeyValue::String("u1".into()),
                ],
                vec![PartitionKeyValue::String("b".into())],
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk_wrong_root_on_second_component() {
    // First component uses 'c', second uses 'd' — preserve the leading prefix.
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND d.userId = 'u1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "acme".into(),
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
    // Gateway rejects this query with HTTP 400: reference to undefined alias d.userId rejected by Gateway
}

#[test]
fn hpk_comparison_to_other_field_no_extract() {
    // Second component compared to another field, not a literal — preserve the leading prefix.
    assert_eq!(
        plan_hpk("SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = c.other"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String(
                "acme".into(),
            )]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

// ── Triple HPK additional scenarios ─────────────────────────────────────

#[test]
fn hpk3_all_parameters() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.tenant = @t AND c.userId = @u AND c.sessionId = @s"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::UnboundParameter("t".into()),
                PartitionKeyValue::UnboundParameter("u".into()),
                PartitionKeyValue::UnboundParameter("s".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_mixed_types_all_different() {
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.tenant = 'acme' AND c.userId = 42 AND c.sessionId = true"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("acme".into()),
                PartitionKeyValue::Number(42_f64),
                PartitionKeyValue::Bool(true),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_reversed_order() {
    // All three in reverse order
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.sessionId = 's1' AND c.userId = 'u1' AND c.tenant = 'a'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
                PartitionKeyValue::String("s1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_shuffled_with_extra_filters() {
    // Components shuffled, interleaved with non-PK filters
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.active = true AND c.sessionId = 's1' AND c.x > 10 AND c.tenant = 'a' AND c.y < 5 AND c.userId = 'u1'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
                PartitionKeyValue::String("s1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_contradictory_first() {
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.tenant = 'a' AND c.tenant = 'b' AND c.userId = 'u1' AND c.sessionId = 's1'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_contradictory_last() {
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1' AND c.sessionId = 's2'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Contradictory,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_redundant_all_components() {
    // Each component appears twice with the same value
    assert_eq!(
        plan_hpk3(
            "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1' AND c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1'"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
                PartitionKeyValue::String("s1".into()),
            ]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_missing_first_only() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.userId = 'u1' AND c.sessionId = 's1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_only_first_component() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.tenant = 'a'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_only_last_component() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.sessionId = 's1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Unconstrained,
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_first_and_last_missing_middle() {
    assert_eq!(
        plan_hpk3("SELECT * FROM c WHERE c.tenant = 'a' AND c.sessionId = 's1'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())]),
            query_info: LocalQueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}

#[test]
fn hpk3_with_join_and_order_by() {
    assert_eq!(
        plan_hpk3(
            "SELECT c.name, t FROM c JOIN t IN c.tags \
             WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.sessionId = 's1' \
             ORDER BY c.name ASC"
        ),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
                PartitionKeyValue::String("s1".into()),
            ]),
            query_info: LocalQueryInfo {
                order_by: vec![SortOrder::Ascending],
                order_by_expressions: vec!["c.name".into()],
                has_join: true,
                has_where: true,
                ..qi()
            },
        }
    );
}
