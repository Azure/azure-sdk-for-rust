// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Client-side query plan generation.
//!
//! Parses the SQL query text locally using `azure_data_cosmos_query` to produce
//! a full query plan — equivalent to the Gateway's query plan REST endpoint.
//! The only fallback to Gateway is when the SQL text cannot be parsed locally.

use azure_data_cosmos_query::plan::{PartitionKeyFilter, PartitionKeyValue, QueryInfo};

use crate::models::PartitionKeyDefinition;
use crate::Query;

/// The result of client-side query plan generation.
///
/// Contains both partition targeting (which partitions to hit) and the full
/// structural query info (aggregates, ORDER BY, GROUP BY, DISTINCT, etc.)
/// needed to construct the execution pipeline.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) struct ClientQueryPlan {
    /// Partition targeting strategy.
    pub targeting: PartitionTargeting,

    /// Full structural query information — equivalent to Gateway's `queryInfo`.
    pub query_info: QueryInfo,
}

/// How the query should be routed to partitions.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub(crate) enum PartitionTargeting {
    /// The query targets a single partition identified by exact PK values.
    SinglePartition { pk_values: Vec<serde_json::Value> },
    /// The query targets multiple specific partitions.
    MultiplePartitions {
        pk_value_sets: Vec<Vec<serde_json::Value>>,
    },
    /// The query must hit all partitions.
    CrossPartition,
}

/// Error returned only when the SQL text cannot be parsed locally.
#[derive(Debug, Clone)]
pub(crate) struct QueryPlanError {
    pub reason: String,
}

impl std::fmt::Display for QueryPlanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "client-side SQL parse error: {}", self.reason)
    }
}

impl std::error::Error for QueryPlanError {}

/// Generate a complete client-side query plan from the query text and parameters.
///
/// Returns both partition targeting and full structural query info.
/// The only error case is an unparseable SQL text.
#[allow(dead_code)]
pub(crate) fn generate_query_plan(
    query: &Query,
    pk_definition: &PartitionKeyDefinition,
) -> Result<ClientQueryPlan, QueryPlanError> {
    let program = azure_data_cosmos_query::parse(&query.text).map_err(|e| {
        tracing::debug!("Client-side query plan: parse failed: {e}");
        QueryPlanError {
            reason: e.to_string(),
        }
    })?;

    let pk_paths: Vec<&str> = pk_definition.paths().iter().map(|p| p.as_ref()).collect();
    let raw_plan = azure_data_cosmos_query::plan::generate_query_plan(&program.query, &pk_paths);

    let targeting = build_targeting(&raw_plan.pk_filters, query)?;

    Ok(ClientQueryPlan {
        targeting,
        query_info: raw_plan.query_info,
    })
}

fn build_targeting(
    filters: &PartitionKeyFilter,
    query: &Query,
) -> Result<PartitionTargeting, QueryPlanError> {
    match filters {
        PartitionKeyFilter::Equality(values) => {
            let resolved = resolve_pk_values(values, query)?;
            Ok(PartitionTargeting::SinglePartition {
                pk_values: resolved,
            })
        }
        PartitionKeyFilter::InList(value_sets) => {
            let mut resolved_sets = Vec::with_capacity(value_sets.len());
            for values in value_sets {
                resolved_sets.push(resolve_pk_values(values, query)?);
            }
            Ok(PartitionTargeting::MultiplePartitions {
                pk_value_sets: resolved_sets,
            })
        }
        PartitionKeyFilter::None => Ok(PartitionTargeting::CrossPartition),
    }
}

fn resolve_pk_values(
    values: &[PartitionKeyValue],
    query: &Query,
) -> Result<Vec<serde_json::Value>, QueryPlanError> {
    let mut resolved = Vec::with_capacity(values.len());
    for val in values {
        resolved.push(match val {
            PartitionKeyValue::String(s) => serde_json::Value::String(s.clone()),
            PartitionKeyValue::Number(n) => serde_json::Number::from_f64(*n)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            PartitionKeyValue::Integer(n) => serde_json::Value::Number((*n).into()),
            PartitionKeyValue::Bool(b) => serde_json::Value::Bool(*b),
            PartitionKeyValue::Null | PartitionKeyValue::Undefined => serde_json::Value::Null,
            PartitionKeyValue::Parameter(name) => {
                find_parameter_value(query, name).ok_or_else(|| QueryPlanError {
                    reason: format!("query references parameter @{name} but it was not provided"),
                })?
            }
        });
    }
    Ok(resolved)
}

fn find_parameter_value(query: &Query, name: &str) -> Option<serde_json::Value> {
    for param in &query.parameters {
        let param_name = param.name.as_str();
        let clean_name = param_name.strip_prefix('@').unwrap_or(param_name);
        if clean_name == name || param_name == name {
            return Some(param.value.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_data_cosmos_query::plan::{AggregateKind, DistinctType, SortOrder};

    fn make_pk_def(paths: &[&str]) -> PartitionKeyDefinition {
        PartitionKeyDefinition::new(
            paths
                .iter()
                .map(|s| std::borrow::Cow::from(s.to_string()))
                .collect(),
        )
    }

    fn plan(sql: &str) -> ClientQueryPlan {
        generate_query_plan(&Query::from(sql), &make_pk_def(&["/pk"])).unwrap()
    }

    // ── Targeting ────────────────────────────────────────────────────────

    #[test]
    fn single_pk() {
        let p = plan("SELECT * FROM c WHERE c.pk = 'hello'");
        assert_eq!(
            p.targeting,
            PartitionTargeting::SinglePartition {
                pk_values: vec![serde_json::json!("hello")]
            }
        );
    }

    #[test]
    fn cross_partition() {
        assert_eq!(
            plan("SELECT * FROM c").targeting,
            PartitionTargeting::CrossPartition
        );
    }

    #[test]
    fn cross_partition_with_non_pk_filter() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.age > 21").targeting,
            PartitionTargeting::CrossPartition
        );
    }

    // ── Full query info ──────────────────────────────────────────────────

    #[test]
    fn aggregate_info() {
        let p = plan("SELECT COUNT(1), SUM(c.price) FROM c WHERE c.pk = 'x'");
        assert!(p.query_info.aggregates.contains(&AggregateKind::Count));
        assert!(p.query_info.aggregates.contains(&AggregateKind::Sum));
        assert_eq!(
            p.targeting,
            PartitionTargeting::SinglePartition {
                pk_values: vec![serde_json::json!("x")]
            }
        );
    }

    #[test]
    fn order_by_info() {
        let p = plan("SELECT * FROM c WHERE c.pk = 'x' ORDER BY c.name DESC");
        assert_eq!(p.query_info.order_by, vec![SortOrder::Descending]);
        assert_eq!(p.query_info.order_by_expressions, vec!["c.name"]);
    }

    #[test]
    fn group_by_info() {
        let p = plan("SELECT c.city, COUNT(1) FROM c WHERE c.pk = 'x' GROUP BY c.city");
        assert_eq!(p.query_info.group_by_expressions, vec!["c.city"]);
        assert!(p.query_info.aggregates.contains(&AggregateKind::Count));
    }

    #[test]
    fn distinct_info() {
        let p = plan("SELECT DISTINCT c.name FROM c ORDER BY c.name");
        assert_eq!(p.query_info.distinct_type, DistinctType::Ordered);
    }

    #[test]
    fn top_offset_limit_info() {
        let p = plan("SELECT TOP 10 * FROM c OFFSET 5 LIMIT 20");
        assert_eq!(p.query_info.top, Some(10));
        assert_eq!(p.query_info.offset, Some(5));
        assert_eq!(p.query_info.limit, Some(20));
    }

    #[test]
    fn select_value_info() {
        assert!(
            plan("SELECT VALUE c.name FROM c")
                .query_info
                .has_select_value
        );
    }

    #[test]
    fn join_info() {
        let p = plan("SELECT * FROM c JOIN t IN c.tags WHERE c.pk = 'x'");
        assert!(p.query_info.has_join);
    }

    #[test]
    fn subquery_info() {
        let p = plan("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags)");
        assert!(p.query_info.has_subquery);
    }

    #[test]
    fn udf_info() {
        let p = plan("SELECT * FROM c WHERE udf.myFunc(c.x) > 0");
        assert!(p.query_info.has_udf);
    }

    #[test]
    fn cross_partition_aggregate() {
        let p = plan("SELECT COUNT(1) FROM c");
        assert_eq!(p.targeting, PartitionTargeting::CrossPartition);
        assert!(p.query_info.aggregates.contains(&AggregateKind::Count));
    }

    #[test]
    fn complex_query_full_plan() {
        let p = plan(
            "SELECT DISTINCT TOP 5 c.city, COUNT(1) AS cnt \
             FROM c WHERE c.pk = 'x' \
             GROUP BY c.city \
             ORDER BY c.city ASC",
        );
        assert_eq!(
            p.targeting,
            PartitionTargeting::SinglePartition {
                pk_values: vec![serde_json::json!("x")]
            }
        );
        assert_eq!(p.query_info.distinct_type, DistinctType::Ordered);
        assert_eq!(p.query_info.top, Some(5));
        assert_eq!(p.query_info.group_by_expressions, vec!["c.city"]);
        assert_eq!(p.query_info.order_by, vec![SortOrder::Ascending]);
        assert!(p.query_info.aggregates.contains(&AggregateKind::Count));
    }

    #[test]
    fn invalid_sql_returns_error() {
        let result = generate_query_plan(&Query::from("NOT VALID SQL !!!"), &make_pk_def(&["/pk"]));
        assert!(result.is_err());
    }

    #[test]
    fn missing_parameter_is_error() {
        let result = generate_query_plan(
            &Query::from("SELECT * FROM c WHERE c.pk = @missing"),
            &make_pk_def(&["/pk"]),
        );
        assert!(result.is_err());
    }
}
