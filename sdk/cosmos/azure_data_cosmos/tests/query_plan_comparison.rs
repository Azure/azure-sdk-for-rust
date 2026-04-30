// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Exhaustive structural comparison tests for the client-side query plan generator.
//!
//! Every test asserts the **entire** `QueryPlan` struct — both `pk_filters` and
//! every field of `query_info` — so that any regression in any part of the plan
//! is caught immediately.

use azure_data_cosmos_query::plan::{
    generate_query_plan, AggregateKind, DistinctType, PartitionKeyFilter, PartitionKeyValue,
    QueryInfo, QueryPlan, SortOrder,
};

/// Parse SQL and produce a full query plan against a single `/pk` partition key.
fn plan(sql: &str) -> QueryPlan {
    let p = azure_data_cosmos_query::parse(sql).unwrap();
    generate_query_plan(&p.query, &["/pk"])
}

/// Parse SQL and produce a full query plan against hierarchical `/tenant`, `/userId`.
fn plan_hpk(sql: &str) -> QueryPlan {
    let p = azure_data_cosmos_query::parse(sql).unwrap();
    generate_query_plan(&p.query, &["/tenant", "/userId"])
}

/// Shorthand: the default QueryInfo with all fields at their zero/empty/false values.
fn qi() -> QueryInfo {
    QueryInfo::default()
}

// ═══════════════════════════════════════════════════════════════════════════════
// SIMPLE SELECT — no WHERE, no clauses
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn select_star_from_c() {
    assert_eq!(
        plan("SELECT * FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::None,
            query_info: qi(),
        }
    );
}

#[test]
fn select_fields_from_c() {
    assert_eq!(
        plan("SELECT c.name, c.age FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::None,
            query_info: qi(),
        }
    );
}

#[test]
fn select_value() {
    assert_eq!(
        plan("SELECT VALUE c.name FROM c"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Integer(42)]),
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Integer(-99)]),
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::Parameter(
                "val".into()
            )]),
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
        QueryInfo {
            has_where: true,
            ..qi()
        }
    );
}

#[test]
fn pk_in_list() {
    let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c')");
    assert!(matches!(qp.pk_filters, PartitionKeyFilter::InList(ref l) if l.len() == 3));
    assert_eq!(
        qp.query_info,
        QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
                has_join: true,
                ..qi()
            },
        }
    );
}

#[test]
fn join_with_pk_and_where() {
    assert_eq!(
        plan("SELECT c.id, t FROM c JOIN t IN c.tags WHERE c.pk = 'x'"),
        QueryPlan {
            pk_filters: PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())]),
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            pk_filters: PartitionKeyFilter::None,
            query_info: QueryInfo {
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
            query_info: QueryInfo {
                has_where: true,
                ..qi()
            },
        }
    );
}
