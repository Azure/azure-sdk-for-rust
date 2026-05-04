// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Query plan generation: partition key extraction and full structural query analysis.
//!
//! This module produces a `QueryPlan` that mirrors the structure returned by the
//! Cosmos DB Gateway query plan REST endpoint, enabling the SDK to make routing
//! and pipeline decisions without a Gateway roundtrip.

use serde::{Deserialize, Serialize};

use crate::query::ast::*;
use crate::query::common::get_root_alias;

// ─── Query Plan ──────────────────────────────────────────────────────────────

/// A client-side query plan produced by the local SQL parser.
///
/// Contains partition key targeting information and structural query info.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryPlan {
    /// Partition key filters extracted from the WHERE clause.
    pub(crate) pk_filters: PartitionKeyFilter,

    /// Structural information about the query for pipeline construction.
    pub(crate) query_info: QueryInfo,
}

/// Structural information about a query.
///
/// This is the unified type used for both local plan generation and gateway
/// response deserialization. Fields present only in gateway responses
/// (e.g., `rewritten_query`) default to `None`/empty when generated locally.
/// Fields present only in local analysis (e.g., `has_join`) default to `false`
/// when deserialized from the gateway.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct QueryInfo {
    /// The kind of DISTINCT, if any.
    #[serde(default)]
    pub(crate) distinct_type: DistinctType,

    /// TOP value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) top: Option<i64>,

    /// OFFSET value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<i64>,

    /// LIMIT value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) limit: Option<i64>,

    /// ORDER BY sort orders (one per ORDER BY item).
    #[serde(default)]
    pub(crate) order_by: Vec<SortOrder>,

    /// ORDER BY expressions as path strings (e.g., `["c.name", "c.age"]`).
    #[serde(default)]
    pub(crate) order_by_expressions: Vec<String>,

    /// GROUP BY expressions as path strings.
    #[serde(default)]
    pub(crate) group_by_expressions: Vec<String>,

    /// GROUP BY aliases (gateway only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) group_by_aliases: Vec<String>,

    /// Aggregate functions used in the query.
    #[serde(default)]
    pub(crate) aggregates: Vec<AggregateKind>,

    /// GROUP BY alias to aggregate type mapping (gateway only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) group_by_alias_to_aggregate_type: Option<serde_json::Value>,

    /// The rewritten query text, if the gateway rewrites it (gateway only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) rewritten_query: Option<String>,

    /// Whether the SELECT clause uses `SELECT VALUE`.
    #[serde(default)]
    pub(crate) has_select_value: bool,

    /// Whether the query contains non-streaming ORDER BY (gateway only).
    #[serde(default)]
    pub(crate) has_non_streaming_order_by: bool,

    /// DCount information (gateway only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) d_count_info: Option<serde_json::Value>,

    /// Whether the query contains a JOIN (local analysis only).
    #[serde(default)]
    pub(crate) has_join: bool,

    /// Whether the query contains subqueries (local analysis only).
    #[serde(default)]
    pub(crate) has_subquery: bool,

    /// Whether the query contains a WHERE clause (local analysis only).
    #[serde(default)]
    pub(crate) has_where: bool,

    /// Whether the query references UDF functions (local analysis only).
    #[serde(default)]
    pub(crate) has_udf: bool,
}

/// The kind of DISTINCT operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum DistinctType {
    /// No DISTINCT.
    #[default]
    None,
    /// Ordered DISTINCT (when ORDER BY is also present).
    Ordered,
    /// Unordered DISTINCT.
    Unordered,
}

/// Sort order for ORDER BY items (mirrors the Gateway's representation).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum SortOrder {
    Ascending,
    Descending,
}

/// Recognized aggregate function kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub(crate) enum AggregateKind {
    Count,
    Sum,
    Avg,
    Min,
    Max,
    ArrayAgg,
}

// ─── Partition Key Filter ────────────────────────────────────────────────────

/// Partition key filter extracted from a WHERE clause.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub(crate) enum PartitionKeyFilter {
    /// Exact equality on all PK components: `pk = <value>`.
    Equality(Vec<PartitionKeyValue>),

    /// IN list on first PK component: `pk IN (v1, v2, ...)`.
    InList(Vec<Vec<PartitionKeyValue>>),

    /// No PK filter found — cross-partition query.
    None,
}

/// A single partition key component value.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
#[non_exhaustive]
pub(crate) enum PartitionKeyValue {
    String(String),
    Number(f64),
    Integer(i64),
    Bool(bool),
    Null,
    Undefined,
    /// A reference to a query parameter that must be resolved at runtime.
    Parameter(String),
}

// ─── Public API ──────────────────────────────────────────────────────────────

/// Generate a complete query plan from parsed SQL and partition key paths.
///
/// `pk_paths` is a list of partition key paths (e.g., `["/pk"]` or `["/tenant", "/userId"]`).
///
/// # Examples
///
/// ```ignore
/// use azure_data_cosmos_driver::query::{parse, plan};
/// let program = parse("SELECT * FROM c WHERE c.pk = 'hello'").unwrap();
/// let qp = plan::generate_query_plan(&program.query, &["/pk"]);
/// assert!(matches!(qp.pk_filters, plan::PartitionKeyFilter::Equality(_)));
/// assert_eq!(qp.query_info.distinct_type, plan::DistinctType::None);
/// ```
pub(crate) fn generate_query_plan(query: &SqlQuery, pk_paths: &[&str]) -> QueryPlan {
    // Convenience wrapper used by tests that don't care about parameter substitution.
    // Parameterized TOP/OFFSET/LIMIT clauses without a corresponding parameter value
    // will produce an error here — callers must use `generate_query_plan_with_parameters`
    // and supply the parameter values for parameterized queries.
    generate_query_plan_with_parameters(query, pk_paths, &[])
        .expect("generate_query_plan called on a query with parameterized TOP/OFFSET/LIMIT but no parameters supplied; use generate_query_plan_with_parameters")
}

/// Type alias for query parameters used during plan generation.
///
/// Each entry is a `(name, value)` pair. Names are stored *without* the leading `@`.
/// Values are arbitrary JSON values; only integer values are accepted as substitutions
/// for parameterized `TOP` / `OFFSET` / `LIMIT` clauses.
pub(crate) type Params = [(String, serde_json::Value)];

/// Generate a complete query plan, substituting query parameters into parameterized
/// `TOP`, `OFFSET`, and `LIMIT` clauses.
///
/// Returns an error if the query references a parameter (in `TOP`, `OFFSET`, or `LIMIT`)
/// that is not present in `parameters`, or whose value is not a non-negative integer.
///
/// The Cosmos DB Gateway rejects query-plan requests for queries with parameterized
/// `TOP` / `OFFSET` / `LIMIT` (HTTP 400). Unlike the Gateway, this function can produce
/// a valid plan when the caller supplies the parameter values up-front.
pub(crate) fn generate_query_plan_with_parameters(
    query: &SqlQuery,
    pk_paths: &[&str],
    parameters: &Params,
) -> Result<QueryPlan, azure_core::Error> {
    let query_info = analyze_query(query, parameters)?;
    let root_alias = get_root_alias(query);

    let pk_filters = if pk_paths.is_empty() {
        PartitionKeyFilter::None
    } else {
        let pk_segments: Vec<Vec<&str>> = pk_paths
            .iter()
            .map(|p| p.strip_prefix('/').unwrap_or(p).split('/').collect())
            .collect();

        if let Some(where_clause) = &query.where_clause {
            extract_pk_from_expression(
                &where_clause.expression,
                &pk_segments,
                root_alias.as_deref(),
            )
        } else {
            PartitionKeyFilter::None
        }
    };

    Ok(QueryPlan {
        pk_filters,
        query_info,
    })
}

/// Look up a parameter value by name and return it as an `i64`.
///
/// Used to substitute parameterized `TOP` / `OFFSET` / `LIMIT` values. Accepts only
/// non-negative integer JSON values; rejects floats (even integer-valued ones like `5.0`),
/// strings, booleans, and missing parameters.
fn resolve_integer_parameter(name: &str, parameters: &Params) -> Result<i64, azure_core::Error> {
    let needle = name.trim_start_matches('@');
    let entry = parameters
        .iter()
        .find(|(n, _)| n.trim_start_matches('@') == needle)
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!(
                    "query references parameter @{needle} in a TOP/OFFSET/LIMIT clause but no value was supplied"
                ),
            )
        })?;
    match &entry.1 {
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if i < 0 {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        format!(
                            "parameter @{needle} used in TOP/OFFSET/LIMIT must be non-negative; got {i}"
                        ),
                    ));
                }
                Ok(i)
            } else {
                Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    format!(
                        "parameter @{needle} used in TOP/OFFSET/LIMIT must be an integer; got {n}"
                    ),
                ))
            }
        }
        other => Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("parameter @{needle} used in TOP/OFFSET/LIMIT must be an integer; got {other}"),
        )),
    }
}

// ─── Query Analysis ──────────────────────────────────────────────────────────

/// Returns true if the expression is a constant (literal) that doesn't reference
/// any collection variable. Used to detect cases where DISTINCT is a no-op.
fn is_constant_expression(expr: &SqlScalarExpression) -> bool {
    match expr {
        SqlScalarExpression::Literal(_) => true,
        SqlScalarExpression::ArrayCreate(items) => items.iter().all(is_constant_expression),
        SqlScalarExpression::ObjectCreate(props) => {
            props.iter().all(|p| is_constant_expression(&p.expression))
        }
        SqlScalarExpression::Unary { operand, .. } => is_constant_expression(operand),
        SqlScalarExpression::Binary { left, right, .. } => {
            is_constant_expression(left) && is_constant_expression(right)
        }
        _ => false,
    }
}

fn analyze_query(query: &SqlQuery, parameters: &Params) -> Result<QueryInfo, azure_core::Error> {
    let mut info = QueryInfo {
        has_select_value: matches!(query.select.spec, SqlSelectSpec::Value(_)),
        has_where: query.where_clause.is_some(),
        ..Default::default()
    };

    // DISTINCT — Gateway optimizes away DISTINCT when the SELECT expression is a
    // constant (literal) that doesn't reference any collection variable, because
    // a single constant value is always distinct by definition.
    if query.select.distinct {
        let is_constant_select = match &query.select.spec {
            SqlSelectSpec::Value(expr) => is_constant_expression(expr),
            _ => false,
        };
        if is_constant_select {
            // Gateway reports distinctType: "None" for constant expressions
            info.distinct_type = DistinctType::None;
        } else if query.order_by.is_some() {
            info.distinct_type = DistinctType::Ordered;
        } else {
            info.distinct_type = DistinctType::Unordered;
        }
    }

    // TOP — substitute parameterized values; error if unresolvable.
    info.top = match &query.select.top {
        Some(SqlTopSpec::Literal(n)) => Some(*n),
        Some(SqlTopSpec::Parameter(name)) => Some(resolve_integer_parameter(name, parameters)?),
        None => None,
    };

    // OFFSET / LIMIT — same substitution rules as TOP.
    if let Some(ol) = &query.offset_limit {
        info.offset = match &ol.offset {
            SqlOffsetSpec::Literal(n) => Some(*n),
            SqlOffsetSpec::Parameter(name) => Some(resolve_integer_parameter(name, parameters)?),
        };
        info.limit = match &ol.limit {
            SqlLimitSpec::Literal(n) => Some(*n),
            SqlLimitSpec::Parameter(name) => Some(resolve_integer_parameter(name, parameters)?),
        };
    }

    // ORDER BY
    if let Some(order_by) = &query.order_by {
        for item in &order_by.items {
            let sort = match item.order {
                SqlSortOrder::Descending => SortOrder::Descending,
                _ => SortOrder::Ascending,
            };
            info.order_by.push(sort);
            info.order_by_expressions
                .push(expr_to_path_string(&item.expression));
        }
    }

    // GROUP BY
    if let Some(group_by) = &query.group_by {
        for expr in &group_by.expressions {
            info.group_by_expressions.push(expr_to_path_string(expr));
        }
    }

    // JOIN
    if let Some(from) = &query.from {
        info.has_join = has_join(&from.collection);
    }

    // Aggregates, subqueries, UDFs — scan all expressions
    visit_select_for_info(&query.select, &mut info);
    if let Some(w) = &query.where_clause {
        visit_expr_for_info(&w.expression, &mut info);
    }
    if let Some(ob) = &query.order_by {
        for item in &ob.items {
            visit_expr_for_info(&item.expression, &mut info);
        }
    }
    if let Some(gb) = &query.group_by {
        for expr in &gb.expressions {
            visit_expr_for_info(expr, &mut info);
        }
    }

    Ok(info)
}

/// Convert an expression to a dot-separated path string for the plan output.
fn expr_to_path_string(expr: &SqlScalarExpression) -> String {
    let mut parts = Vec::new();
    if collect_path_parts(expr, &mut parts) {
        parts.join(".")
    } else {
        format!("{expr:?}")
    }
}

#[allow(clippy::collapsible_match)] // clippy suggests a match guard, but that won't compile with &mut
fn collect_path_parts(expr: &SqlScalarExpression, parts: &mut Vec<String>) -> bool {
    match expr {
        SqlScalarExpression::PropertyRef(name) => {
            parts.push(name.clone());
            true
        }
        SqlScalarExpression::MemberRef { source, member } => {
            if collect_path_parts(source, parts) {
                parts.push(member.clone());
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn has_join(coll: &SqlCollectionExpression) -> bool {
    matches!(coll, SqlCollectionExpression::Join { .. })
}

fn visit_select_for_info(select: &SqlSelectClause, info: &mut QueryInfo) {
    match &select.spec {
        SqlSelectSpec::List(items) => {
            for item in items {
                visit_expr_for_info(&item.expression, info);
            }
        }
        SqlSelectSpec::Value(expr) => visit_expr_for_info(expr, info),
        SqlSelectSpec::Star => {}
    }
}

fn visit_expr_for_info(expr: &SqlScalarExpression, info: &mut QueryInfo) {
    match expr {
        SqlScalarExpression::FunctionCall {
            name, args, is_udf, ..
        } => {
            if *is_udf {
                info.has_udf = true;
            } else {
                let upper = name.to_ascii_uppercase();
                match upper.as_str() {
                    "COUNT" => info.aggregates.push(AggregateKind::Count),
                    "SUM" => info.aggregates.push(AggregateKind::Sum),
                    "AVG" => info.aggregates.push(AggregateKind::Avg),
                    "MIN" => info.aggregates.push(AggregateKind::Min),
                    "MAX" => info.aggregates.push(AggregateKind::Max),
                    "ARRAY_AGG" => info.aggregates.push(AggregateKind::ArrayAgg),
                    _ => {}
                }
            }
            for arg in args {
                visit_expr_for_info(arg, info);
            }
        }
        SqlScalarExpression::Exists(_)
        | SqlScalarExpression::Subquery(_)
        | SqlScalarExpression::Array(_) => {
            info.has_subquery = true;
        }
        SqlScalarExpression::Binary { left, right, .. } => {
            visit_expr_for_info(left, info);
            visit_expr_for_info(right, info);
        }
        SqlScalarExpression::Unary { operand, .. } => {
            visit_expr_for_info(operand, info);
        }
        SqlScalarExpression::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            visit_expr_for_info(condition, info);
            visit_expr_for_info(if_true, info);
            visit_expr_for_info(if_false, info);
        }
        SqlScalarExpression::Coalesce { left, right } => {
            visit_expr_for_info(left, info);
            visit_expr_for_info(right, info);
        }
        SqlScalarExpression::In {
            expression, items, ..
        } => {
            visit_expr_for_info(expression, info);
            for item in items {
                visit_expr_for_info(item, info);
            }
        }
        SqlScalarExpression::Between {
            expression,
            low,
            high,
            ..
        } => {
            visit_expr_for_info(expression, info);
            visit_expr_for_info(low, info);
            visit_expr_for_info(high, info);
        }
        SqlScalarExpression::Like {
            expression,
            pattern,
            ..
        } => {
            visit_expr_for_info(expression, info);
            visit_expr_for_info(pattern, info);
        }
        SqlScalarExpression::ArrayCreate(items) => {
            for item in items {
                visit_expr_for_info(item, info);
            }
        }
        SqlScalarExpression::ObjectCreate(props) => {
            for prop in props {
                visit_expr_for_info(&prop.expression, info);
            }
        }
        _ => {}
    }
}

// ─── PK Extraction (unchanged logic) ────────────────────────────────────────

fn extract_pk_from_expression(
    expr: &SqlScalarExpression,
    pk_segments: &[Vec<&str>],
    root_alias: Option<&str>,
) -> PartitionKeyFilter {
    if pk_segments.len() == 1 {
        return extract_single_pk(expr, &pk_segments[0], root_alias);
    }
    extract_hierarchical_pk(expr, pk_segments, root_alias)
}

fn extract_single_pk(
    expr: &SqlScalarExpression,
    pk_path: &[&str],
    root_alias: Option<&str>,
) -> PartitionKeyFilter {
    match expr {
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::Equal,
            left,
            right,
        } => {
            if is_pk_reference(left, pk_path, root_alias) {
                if let Some(val) = extract_literal_value(right) {
                    return PartitionKeyFilter::Equality(vec![val]);
                }
            }
            if is_pk_reference(right, pk_path, root_alias) {
                if let Some(val) = extract_literal_value(left) {
                    return PartitionKeyFilter::Equality(vec![val]);
                }
            }
            PartitionKeyFilter::None
        }
        SqlScalarExpression::In {
            expression,
            items,
            not: false,
        } => {
            if is_pk_reference(expression, pk_path, root_alias) {
                let values: Vec<Vec<PartitionKeyValue>> = items
                    .iter()
                    .filter_map(|item| extract_literal_value(item).map(|v| vec![v]))
                    .collect();
                if values.len() == items.len() {
                    return PartitionKeyFilter::InList(values);
                }
            }
            PartitionKeyFilter::None
        }
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::And,
            left,
            right,
        } => {
            let left_pk = extract_single_pk(left, pk_path, root_alias);
            let right_pk = extract_single_pk(right, pk_path, root_alias);
            intersect_pk_filters(left_pk, right_pk)
        }
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::Or,
            left,
            right,
        } => {
            let left_pk = extract_single_pk(left, pk_path, root_alias);
            let right_pk = extract_single_pk(right, pk_path, root_alias);
            match (left_pk, right_pk) {
                (PartitionKeyFilter::Equality(a), PartitionKeyFilter::Equality(b)) => {
                    PartitionKeyFilter::InList(vec![a, b])
                }
                (PartitionKeyFilter::Equality(a), PartitionKeyFilter::InList(mut list))
                | (PartitionKeyFilter::InList(mut list), PartitionKeyFilter::Equality(a)) => {
                    list.push(a);
                    PartitionKeyFilter::InList(list)
                }
                (PartitionKeyFilter::InList(mut a), PartitionKeyFilter::InList(b)) => {
                    a.extend(b);
                    PartitionKeyFilter::InList(a)
                }
                _ => PartitionKeyFilter::None,
            }
        }
        _ => PartitionKeyFilter::None,
    }
}

/// Intersect two PK filters from the two sides of an AND expression.
///
/// - `None AND X` → `X` (no constraint on one side, keep the other)
/// - `Equality(a) AND Equality(b)` → `Equality(a)` if a == b, else `None` (contradiction)
/// - `Equality(a) AND InList(list)` → `Equality(a)` if a is in list, else `None`
/// - `InList(a) AND InList(b)` → `InList(intersection)`, or `None` if empty
fn intersect_pk_filters(a: PartitionKeyFilter, b: PartitionKeyFilter) -> PartitionKeyFilter {
    match (a, b) {
        // One side has no PK constraint — the other side's constraint stands.
        (PartitionKeyFilter::None, other) | (other, PartitionKeyFilter::None) => other,

        // Both sides have equality — they must agree.
        (PartitionKeyFilter::Equality(a), PartitionKeyFilter::Equality(b)) => {
            if a == b {
                PartitionKeyFilter::Equality(a)
            } else {
                // Contradictory: c.pk = 'a' AND c.pk = 'b' — logically empty result set.
                // Return None because no single partition can be targeted.
                PartitionKeyFilter::None
            }
        }

        // Equality AND InList — narrow the IN list to just the equality value if present.
        (PartitionKeyFilter::Equality(eq), PartitionKeyFilter::InList(list))
        | (PartitionKeyFilter::InList(list), PartitionKeyFilter::Equality(eq)) => {
            if list.contains(&eq) {
                PartitionKeyFilter::Equality(eq)
            } else {
                PartitionKeyFilter::None
            }
        }

        // InList AND InList — compute intersection.
        (PartitionKeyFilter::InList(a), PartitionKeyFilter::InList(b)) => {
            let intersection: Vec<Vec<PartitionKeyValue>> =
                a.into_iter().filter(|item| b.contains(item)).collect();
            match intersection.len() {
                0 => PartitionKeyFilter::None,
                1 => PartitionKeyFilter::Equality(intersection.into_iter().next().unwrap()),
                _ => PartitionKeyFilter::InList(intersection),
            }
        }
    }
}

fn extract_hierarchical_pk(
    expr: &SqlScalarExpression,
    pk_segments: &[Vec<&str>],
    root_alias: Option<&str>,
) -> PartitionKeyFilter {
    let mut conjuncts = Vec::new();
    flatten_and(expr, &mut conjuncts);
    let mut pk_values = Vec::with_capacity(pk_segments.len());
    for pk_path in pk_segments {
        // Collect ALL equality constraints for this PK component across all conjuncts.
        let mut component_value: Option<PartitionKeyValue> = None;
        let mut conflict = false;
        for conjunct in &conjuncts {
            if let SqlScalarExpression::Binary {
                op: SqlBinaryOp::Equal,
                left,
                right,
            } = conjunct
            {
                let val = if is_pk_reference(left, pk_path, root_alias) {
                    extract_literal_value(right)
                } else if is_pk_reference(right, pk_path, root_alias) {
                    extract_literal_value(left)
                } else {
                    None
                };
                if let Some(v) = val {
                    match &component_value {
                        None => component_value = Some(v),
                        Some(existing) => {
                            if *existing != v {
                                // Contradictory constraints on same component
                                conflict = true;
                                break;
                            }
                            // Same value — redundant but consistent, skip.
                        }
                    }
                }
            }
        }
        if conflict {
            return PartitionKeyFilter::None;
        }
        match component_value {
            Some(v) => pk_values.push(v),
            None => return PartitionKeyFilter::None,
        }
    }
    PartitionKeyFilter::Equality(pk_values)
}

fn flatten_and<'a>(expr: &'a SqlScalarExpression, out: &mut Vec<&'a SqlScalarExpression>) {
    match expr {
        SqlScalarExpression::Binary {
            op: SqlBinaryOp::And,
            left,
            right,
        } => {
            flatten_and(left, out);
            flatten_and(right, out);
        }
        _ => out.push(expr),
    }
}

fn is_pk_reference(expr: &SqlScalarExpression, pk_path: &[&str], root_alias: Option<&str>) -> bool {
    let mut resolved_path = Vec::new();
    if !resolve_property_path(expr, &mut resolved_path) {
        return false;
    }
    if let Some(alias) = root_alias {
        if resolved_path.first().map(String::as_str) == Some(alias) {
            return resolved_path[1..]
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
                == pk_path;
        }
    }
    resolved_path.iter().map(String::as_str).collect::<Vec<_>>() == pk_path
}

#[allow(clippy::collapsible_match)] // clippy suggests a match guard, but that won't compile with &mut
fn resolve_property_path(expr: &SqlScalarExpression, path: &mut Vec<String>) -> bool {
    match expr {
        SqlScalarExpression::PropertyRef(name) => {
            path.push(name.clone());
            true
        }
        SqlScalarExpression::MemberRef { source, member } => {
            if resolve_property_path(source, path) {
                path.push(member.clone());
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn extract_literal_value(expr: &SqlScalarExpression) -> Option<PartitionKeyValue> {
    match expr {
        SqlScalarExpression::Literal(lit) => match lit {
            SqlLiteral::String(s) => Some(PartitionKeyValue::String(s.clone())),
            SqlLiteral::Number(n) => Some(PartitionKeyValue::Number(*n)),
            SqlLiteral::Integer(n) => Some(PartitionKeyValue::Integer(*n)),
            SqlLiteral::Boolean(b) => Some(PartitionKeyValue::Bool(*b)),
            SqlLiteral::Null => Some(PartitionKeyValue::Null),
            SqlLiteral::Undefined => Some(PartitionKeyValue::Undefined),
        },
        SqlScalarExpression::ParameterRef(name) => Some(PartitionKeyValue::Parameter(name.clone())),
        _ => None,
    }
}

/// Generate a query plan as a JSON value from SQL text, partition key paths, and
/// query parameters.
///
/// Substitutes parameter values into parameterized `TOP` / `OFFSET` / `LIMIT` clauses.
/// Returns an error if the query references a parameter in one of those clauses and
/// no matching integer value is supplied. Pass an empty slice for queries that do not
/// use parameters in those clauses.
///
/// Used for cross-crate testing (gateway comparison) where internal types can't be accessed.
#[cfg(any(test, feature = "__internal_testing"))]
pub fn generate_query_plan_for_pk_paths(
    sql: &str,
    pk_paths: &[&str],
    parameters: &[(String, serde_json::Value)],
) -> Result<serde_json::Value, azure_core::Error> {
    let program = crate::query::parse(sql)
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;

    let raw_plan = generate_query_plan_with_parameters(&program.query, pk_paths, parameters)?;

    serde_json::to_value(&raw_plan)
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::parse;

    fn plan(sql: &str) -> QueryPlan {
        let p = parse(sql).unwrap();
        generate_query_plan(&p.query, &["/pk"])
    }

    // Include the exhaustive comparison tests from the external file.
    mod query_plan_comparison;

    // ── PK extraction ────────────────────────────────────────────────────

    #[test]
    fn pk_equality() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'hello'").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("hello".into())])
        );
    }

    #[test]
    fn pk_with_and() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'x' AND c.age > 21").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
        );
    }

    #[test]
    fn pk_in_list() {
        match plan("SELECT * FROM c WHERE c.pk IN ('a', 'b')").pk_filters {
            PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
            other => panic!("expected InList, got {other:?}"),
        }
    }

    #[test]
    fn no_pk_filter() {
        assert_eq!(
            plan("SELECT * FROM c WHERE c.age > 21").pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn no_where_clause() {
        assert_eq!(plan("SELECT * FROM c").pk_filters, PartitionKeyFilter::None);
    }

    // ── QueryInfo: DISTINCT ──────────────────────────────────────────────

    #[test]
    fn distinct_unordered() {
        let qp = plan("SELECT DISTINCT c.name FROM c");
        assert_eq!(qp.query_info.distinct_type, DistinctType::Unordered);
    }

    #[test]
    fn distinct_ordered() {
        let qp = plan("SELECT DISTINCT c.name FROM c ORDER BY c.name");
        assert_eq!(qp.query_info.distinct_type, DistinctType::Ordered);
    }

    #[test]
    fn no_distinct() {
        let qp = plan("SELECT c.name FROM c");
        assert_eq!(qp.query_info.distinct_type, DistinctType::None);
    }

    // ── QueryInfo: TOP / OFFSET / LIMIT ──────────────────────────────────

    #[test]
    fn top_value() {
        assert_eq!(plan("SELECT TOP 10 * FROM c").query_info.top, Some(10));
    }

    #[test]
    fn offset_limit() {
        let qp = plan("SELECT * FROM c OFFSET 5 LIMIT 20");
        assert_eq!(qp.query_info.offset, Some(5));
        assert_eq!(qp.query_info.limit, Some(20));
    }

    // ── QueryInfo: ORDER BY ──────────────────────────────────────────────

    #[test]
    fn order_by_single_asc() {
        let qp = plan("SELECT * FROM c ORDER BY c.name ASC");
        assert_eq!(qp.query_info.order_by, vec![SortOrder::Ascending]);
        assert_eq!(qp.query_info.order_by_expressions, vec!["c.name"]);
    }

    #[test]
    fn order_by_single_desc() {
        let qp = plan("SELECT * FROM c ORDER BY c.name DESC");
        assert_eq!(qp.query_info.order_by, vec![SortOrder::Descending]);
    }

    #[test]
    fn order_by_multiple() {
        let qp = plan("SELECT * FROM c ORDER BY c.name ASC, c.age DESC");
        assert_eq!(
            qp.query_info.order_by,
            vec![SortOrder::Ascending, SortOrder::Descending]
        );
        assert_eq!(qp.query_info.order_by_expressions, vec!["c.name", "c.age"]);
    }

    // ── QueryInfo: GROUP BY ──────────────────────────────────────────────

    #[test]
    fn group_by_single() {
        let qp = plan("SELECT c.city, COUNT(1) FROM c GROUP BY c.city");
        assert_eq!(qp.query_info.group_by_expressions, vec!["c.city"]);
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
    }

    #[test]
    fn group_by_multiple() {
        let qp = plan("SELECT c.city, c.state, COUNT(1) FROM c GROUP BY c.city, c.state");
        assert_eq!(
            qp.query_info.group_by_expressions,
            vec!["c.city", "c.state"]
        );
    }

    // ── QueryInfo: Aggregates ────────────────────────────────────────────

    #[test]
    fn aggregate_count() {
        let qp = plan("SELECT COUNT(1) FROM c");
        assert_eq!(qp.query_info.aggregates, vec![AggregateKind::Count]);
    }

    #[test]
    fn aggregate_sum() {
        let qp = plan("SELECT SUM(c.price) FROM c");
        assert_eq!(qp.query_info.aggregates, vec![AggregateKind::Sum]);
    }

    #[test]
    fn aggregate_avg() {
        let qp = plan("SELECT AVG(c.score) FROM c");
        assert_eq!(qp.query_info.aggregates, vec![AggregateKind::Avg]);
    }

    #[test]
    fn aggregate_min_max() {
        let qp = plan("SELECT MIN(c.age), MAX(c.age) FROM c");
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Min));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Max));
    }

    #[test]
    fn multiple_aggregates() {
        let qp = plan("SELECT COUNT(1), SUM(c.price), AVG(c.score) FROM c");
        assert_eq!(qp.query_info.aggregates.len(), 3);
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Sum));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Avg));
    }

    #[test]
    fn no_aggregates() {
        let qp = plan("SELECT * FROM c");
        assert!(qp.query_info.aggregates.is_empty());
    }

    // ── QueryInfo: SELECT VALUE ──────────────────────────────────────────

    #[test]
    fn select_value_detected() {
        assert!(
            plan("SELECT VALUE c.name FROM c")
                .query_info
                .has_select_value
        );
    }

    #[test]
    fn select_star_not_value() {
        assert!(!plan("SELECT * FROM c").query_info.has_select_value);
    }

    // ── QueryInfo: JOIN ──────────────────────────────────────────────────

    #[test]
    fn join_detected() {
        assert!(plan("SELECT * FROM c JOIN t IN c.tags").query_info.has_join);
    }

    #[test]
    fn no_join() {
        assert!(!plan("SELECT * FROM c").query_info.has_join);
    }

    // ── QueryInfo: Subqueries ────────────────────────────────────────────

    #[test]
    fn exists_subquery_detected() {
        assert!(
            plan("SELECT * FROM c WHERE EXISTS(SELECT VALUE t FROM t IN c.tags)")
                .query_info
                .has_subquery
        );
    }

    #[test]
    fn array_subquery_detected() {
        assert!(
            plan("SELECT ARRAY(SELECT t FROM t IN c.tags) FROM c")
                .query_info
                .has_subquery
        );
    }

    // ── QueryInfo: UDF ───────────────────────────────────────────────────

    #[test]
    fn udf_detected() {
        assert!(
            plan("SELECT * FROM c WHERE udf.myFunc(c.x) > 0")
                .query_info
                .has_udf
        );
    }

    #[test]
    fn builtin_function_not_udf() {
        assert!(
            !plan("SELECT * FROM c WHERE CONTAINS(c.name, 'x')")
                .query_info
                .has_udf
        );
    }

    // ── QueryInfo: WHERE ─────────────────────────────────────────────────

    #[test]
    fn has_where() {
        assert!(plan("SELECT * FROM c WHERE c.x = 1").query_info.has_where);
    }

    #[test]
    fn no_where() {
        assert!(!plan("SELECT * FROM c").query_info.has_where);
    }

    // ── Combined: PK + full query info ───────────────────────────────────

    #[test]
    fn aggregate_with_pk_and_group_by() {
        let qp = plan(
            "SELECT c.city, COUNT(1) AS cnt, SUM(c.revenue) AS total \
             FROM c WHERE c.pk = 'x' GROUP BY c.city",
        );
        assert_eq!(
            qp.pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
        );
        assert_eq!(qp.query_info.group_by_expressions, vec!["c.city"]);
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Count));
        assert!(qp.query_info.aggregates.contains(&AggregateKind::Sum));
    }

    #[test]
    fn order_by_with_pk_and_top() {
        let qp = plan("SELECT TOP 5 * FROM c WHERE c.pk = 'x' ORDER BY c.name DESC");
        assert_eq!(
            qp.pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("x".into())])
        );
        assert_eq!(qp.query_info.top, Some(5));
        assert_eq!(qp.query_info.order_by, vec![SortOrder::Descending]);
    }

    #[test]
    fn cross_partition_aggregate_with_order_by() {
        let qp = plan("SELECT c.city, COUNT(1) FROM c GROUP BY c.city ORDER BY c.city ASC");
        assert_eq!(qp.pk_filters, PartitionKeyFilter::None);
        assert!(!qp.query_info.group_by_expressions.is_empty());
        assert!(!qp.query_info.order_by.is_empty());
        assert!(!qp.query_info.aggregates.is_empty());
    }

    // ── AND intersection logic ───────────────────────────────────────────

    #[test]
    fn and_contradictory_equality_is_none() {
        // c.pk = 'a' AND c.pk = 'b' — contradiction, no partition can match
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'b'").pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn and_redundant_equality_is_ok() {
        // c.pk = 'a' AND c.pk = 'a' — redundant but consistent
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk = 'a'").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_equality_narrows_in_list() {
        // c.pk = 'a' AND c.pk IN ('a', 'b') — narrows to 'a'
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.pk IN ('a', 'b')").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_equality_not_in_list_is_none() {
        // c.pk = 'c' AND c.pk IN ('a', 'b') — contradiction
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'c' AND c.pk IN ('a', 'b')").pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn and_in_list_narrows_in_list() {
        // c.pk IN ('a', 'b', 'c') AND c.pk IN ('b', 'c', 'd') — intersection is ('b', 'c')
        let qp = plan("SELECT * FROM c WHERE c.pk IN ('a', 'b', 'c') AND c.pk IN ('b', 'c', 'd')");
        match qp.pk_filters {
            PartitionKeyFilter::InList(ref list) => {
                assert_eq!(list.len(), 2);
                assert!(list.contains(&vec![PartitionKeyValue::String("b".into())]));
                assert!(list.contains(&vec![PartitionKeyValue::String("c".into())]));
            }
            _ => panic!("expected InList, got {:?}", qp.pk_filters),
        }
    }

    #[test]
    fn and_in_list_intersection_single_becomes_equality() {
        // c.pk IN ('a', 'b') AND c.pk IN ('b', 'c') — intersection is just 'b'
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('b', 'c')").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("b".into())])
        );
    }

    #[test]
    fn and_in_list_empty_intersection_is_none() {
        // c.pk IN ('a', 'b') AND c.pk IN ('c', 'd') — empty intersection
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.pk IN ('c', 'd')").pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn and_pk_with_non_pk_keeps_pk() {
        // c.pk = 'a' AND c.other > 5 — non-PK side is None, keep PK side
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.other > 5").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_non_pk_with_pk_keeps_pk() {
        // c.other > 5 AND c.pk = 'a' — reversed order
        assert_eq!(
            plan("SELECT * FROM c WHERE c.other > 5 AND c.pk = 'a'").pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_chain_multiple_consistent() {
        // c.pk = 'a' AND c.x > 1 AND c.pk = 'a' AND c.y < 10 — consistent
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.x > 1 AND c.pk = 'a' AND c.y < 10")
                .pk_filters,
            PartitionKeyFilter::Equality(vec![PartitionKeyValue::String("a".into())])
        );
    }

    #[test]
    fn and_chain_contradictory() {
        // c.pk = 'a' AND c.x > 1 AND c.pk = 'b' — contradiction deep in chain
        assert_eq!(
            plan("SELECT * FROM c WHERE c.pk = 'a' AND c.x > 1 AND c.pk = 'b'").pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn and_in_list_with_non_pk() {
        // c.pk IN ('a', 'b') AND c.other > 5 — non-PK on one side
        match plan("SELECT * FROM c WHERE c.pk IN ('a', 'b') AND c.other > 5").pk_filters {
            PartitionKeyFilter::InList(list) => assert_eq!(list.len(), 2),
            other => panic!("expected InList, got {other:?}"),
        }
    }

    // ── Hierarchical PK AND conflict detection ──────────────────────────

    fn plan_hpk(sql: &str) -> QueryPlan {
        let p = parse(sql).unwrap();
        generate_query_plan(&p.query, &["/tenant", "/userId"])
    }

    #[test]
    fn hpk_contradictory_first_component() {
        assert_eq!(
            plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.tenant = 'b' AND c.userId = 'u1'")
                .pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn hpk_contradictory_second_component() {
        assert_eq!(
            plan_hpk(
                "SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.userId = 'u2'"
            )
            .pk_filters,
            PartitionKeyFilter::None
        );
    }

    #[test]
    fn hpk_redundant_constraints_ok() {
        assert_eq!(
            plan_hpk("SELECT * FROM c WHERE c.tenant = 'a' AND c.userId = 'u1' AND c.tenant = 'a'")
                .pk_filters,
            PartitionKeyFilter::Equality(vec![
                PartitionKeyValue::String("a".into()),
                PartitionKeyValue::String("u1".into()),
            ])
        );
    }
}
