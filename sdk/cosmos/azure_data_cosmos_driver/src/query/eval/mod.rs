// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore STARTSWITH ENDSWITH LTRIM RTRIM TOSTRING multibyte subpaths

//! In-memory query evaluation: match documents against WHERE clauses and apply projections.
//!
//! This evaluator interprets the SQL AST directly against `serde_json::Value` documents.
//! It supports the most commonly used scalar expressions, comparisons, and built-in functions.

use std::{cmp::Ordering, collections::HashMap};

use crate::query::ast::{
    SqlBinaryOp, SqlCollection, SqlCollectionExpression, SqlLimitSpec, SqlLiteral, SqlOffsetSpec,
    SqlOrderByClause, SqlPathSegment, SqlQuery, SqlScalarExpression, SqlSelectSpec, SqlSortOrder,
    SqlTopSpec, SqlUnaryOp, SqlWhereClause,
};
use crate::query::common::get_root_alias;
use crate::query::value::CosmosValue;

// (#16) Built-in scalar function dispatch lives in a sibling file to keep
// this module focused on AST traversal.
mod builtins;
use builtins::eval_function;

/// Error during query evaluation.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EvalError {
    /// An expression type that is not supported by the in-memory evaluator.
    Unsupported(String),
    /// An unknown built-in function was called.
    UnknownFunction(String),
    /// A type error occurred during evaluation.
    TypeError(String),
    /// A query parameter was referenced but not provided.
    ParameterNotFound(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported(s) => write!(f, "unsupported expression: {s}"),
            Self::UnknownFunction(s) => write!(f, "unknown function: {s}"),
            Self::TypeError(s) => write!(f, "type error: {s}"),
            Self::ParameterNotFound(s) => write!(f, "parameter not found: @{s}"),
        }
    }
}

impl std::error::Error for EvalError {}

use crate::query::common::{
    normalize_parameter_name, resolve_non_negative_integer_parameter, resolve_parameter_value,
    Params,
};

/// Check if a JSON document matches a query's WHERE clause.
///
/// # Examples
///
/// ```ignore
/// use azure_data_cosmos_driver::query::{parse, eval};
/// let p = parse("SELECT * FROM c WHERE c.age > 21").unwrap();
/// let doc = serde_json::json!({"age": 30});
/// assert!(eval::matches_query(&doc, &p.query, &[]).unwrap());
/// let doc2 = serde_json::json!({"age": 18});
/// assert!(!eval::matches_query(&doc2, &p.query, &[]).unwrap());
/// ```
pub fn matches_query(
    document: &serde_json::Value,
    query: &SqlQuery,
    parameters: &Params,
) -> Result<bool, EvalError> {
    let root_alias = get_root_alias(query);

    if let Some(where_clause) = &query.where_clause {
        let result = eval_scalar(
            &where_clause.expression,
            document,
            root_alias.as_deref(),
            parameters,
        )?;
        Ok(matches!(result, CosmosValue::Boolean(true)))
    } else {
        // No WHERE clause — all documents match
        Ok(true)
    }
}

/// Apply a query's SELECT projection to a document.
///
/// Returns the projected JSON value.
pub fn project(
    document: &serde_json::Value,
    query: &SqlQuery,
    parameters: &Params,
) -> Result<serde_json::Value, EvalError> {
    let root_alias = get_root_alias(query);

    match &query.select.spec {
        SqlSelectSpec::Star => Ok(document.clone()),
        SqlSelectSpec::Value(expr) => {
            let val = eval_scalar(expr, document, root_alias.as_deref(), parameters)?;
            Ok(val.to_json())
        }
        SqlSelectSpec::List(items) => {
            let mut obj = serde_json::Map::new();
            for (index, item) in items.iter().enumerate() {
                let val = eval_scalar(
                    &item.expression,
                    document,
                    root_alias.as_deref(),
                    parameters,
                )?;
                let key = if let Some(alias) = &item.alias {
                    alias.clone()
                } else {
                    infer_property_name(&item.expression, index + 1)
                };
                if !val.is_undefined() {
                    obj.insert(key, val.to_json());
                }
            }
            Ok(serde_json::Value::Object(obj))
        }
    }
}

// ─── JOIN expansion ──────────────────────────────────────────────────────────

fn is_plain_root_from(collection: &SqlCollectionExpression) -> bool {
    matches!(
        collection,
        SqlCollectionExpression::Aliased {
            collection: SqlCollection::Path { path, .. },
            ..
        } if path.is_empty()
    )
}

/// Resolve a `SqlCollection::Path` against a set of variable bindings.
fn resolve_collection_path(
    root_document: &serde_json::Value,
    collection: &SqlCollection,
    bindings: &serde_json::Map<String, serde_json::Value>,
) -> Result<serde_json::Value, EvalError> {
    match collection {
        SqlCollection::Path { root, path } => {
            let mut val = bindings
                .get(root)
                .cloned()
                .unwrap_or_else(|| root_document.clone());
            for segment in path {
                val = match segment {
                    SqlPathSegment::Identifier(name) => {
                        val.get(name).cloned().unwrap_or(serde_json::Value::Null)
                    }
                    SqlPathSegment::Index(i) => val
                        .get(*i as usize)
                        .cloned()
                        .unwrap_or(serde_json::Value::Null),
                    SqlPathSegment::StringIndex(s) => val
                        .get(s.as_str())
                        .cloned()
                        .unwrap_or(serde_json::Value::Null),
                };
            }
            Ok(val)
        }
        SqlCollection::Subquery(_) => Err(EvalError::Unsupported("FROM subqueries".into())),
    }
}

/// Expand a FROM clause (potentially with JOINs) into binding contexts.
///
/// Each returned map binds variable names to their values. For example,
/// `FROM c JOIN t IN c.tags` produces one binding context per tag element:
/// `{"c": <doc>, "t": <tag_element>}`.
fn expand_from(
    doc: &serde_json::Value,
    collection: &SqlCollectionExpression,
    bindings: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<serde_json::Map<String, serde_json::Value>>, EvalError> {
    match collection {
        SqlCollectionExpression::Aliased { collection, alias } => {
            let alias_name = alias.clone().unwrap_or_else(|| match collection {
                SqlCollection::Path { root, .. } => root.clone(),
                SqlCollection::Subquery(_) => "c".to_string(),
            });
            let source = resolve_collection_path(doc, collection, bindings)?;
            let mut map = serde_json::Map::new();
            map.insert(alias_name, source);
            Ok(vec![map])
        }
        SqlCollectionExpression::Join { left, right } => {
            let left_bindings = expand_from(doc, left, bindings)?;
            let mut result = Vec::new();
            for left_ctx in &left_bindings {
                let mut merged = bindings.clone();
                merged.extend(left_ctx.clone());
                let right_bindings = expand_from(doc, right, &merged)?;
                for right_ctx in right_bindings {
                    let mut combined = left_ctx.clone();
                    combined.extend(right_ctx);
                    result.push(combined);
                }
            }
            Ok(result)
        }
        SqlCollectionExpression::ArrayIterator {
            identifier,
            collection,
        } => {
            let arr = resolve_collection_path(doc, collection, bindings)?;
            match arr {
                serde_json::Value::Array(items) => Ok(items
                    .into_iter()
                    .map(|item| {
                        let mut map = serde_json::Map::new();
                        map.insert(identifier.clone(), item);
                        map
                    })
                    .collect()),
                _ => Ok(Vec::new()),
            }
        }
    }
}

// ─── Aggregate helpers ───────────────────────────────────────────────────────

/// Returns `true` if `name` is a recognized aggregate function.
fn is_aggregate_function(name: &str) -> bool {
    matches!(
        name.to_ascii_uppercase().as_str(),
        "COUNT" | "SUM" | "AVG" | "MIN" | "MAX"
    )
}

/// Walk an expression tree and return `true` if any aggregate function call is found.
fn contains_aggregate(expr: &SqlScalarExpression) -> bool {
    match expr {
        SqlScalarExpression::FunctionCall {
            name, is_udf, args, ..
        } => (!is_udf && is_aggregate_function(name)) || args.iter().any(contains_aggregate),
        SqlScalarExpression::Binary { left, right, .. } => {
            contains_aggregate(left) || contains_aggregate(right)
        }
        SqlScalarExpression::Unary { operand, .. } => contains_aggregate(operand),
        SqlScalarExpression::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            contains_aggregate(condition)
                || contains_aggregate(if_true)
                || contains_aggregate(if_false)
        }
        SqlScalarExpression::Coalesce { left, right } => {
            contains_aggregate(left) || contains_aggregate(right)
        }
        SqlScalarExpression::ArrayCreate(items) => items.iter().any(contains_aggregate),
        SqlScalarExpression::ObjectCreate(props) => {
            props.iter().any(|p| contains_aggregate(&p.expression))
        }
        _ => false,
    }
}

/// Check whether the SELECT clause references any aggregate functions.
fn select_has_aggregates(query: &SqlQuery) -> bool {
    match &query.select.spec {
        SqlSelectSpec::Star => false,
        SqlSelectSpec::Value(expr) => contains_aggregate(expr),
        SqlSelectSpec::List(items) => items.iter().any(|i| contains_aggregate(&i.expression)),
    }
}

/// Evaluate an aggregate function over a group of documents.
fn eval_aggregate(
    name: &str,
    args: &[SqlScalarExpression],
    group: &[serde_json::Value],
    root_alias: Option<&str>,
    params: &Params,
) -> Result<CosmosValue, EvalError> {
    match name.to_ascii_uppercase().as_str() {
        "COUNT" => {
            let mut count = 0i64;
            for doc in group {
                if let Some(arg) = args.first() {
                    let val = eval_scalar(arg, doc, root_alias, params)?;
                    if !val.is_undefined() {
                        count += 1;
                    }
                } else {
                    count += 1;
                }
            }
            Ok(CosmosValue::Integer(count))
        }
        "SUM" => {
            let arg = args
                .first()
                .ok_or_else(|| EvalError::TypeError("SUM requires an argument".into()))?;
            // integer-pure aggregation — mirror Cosmos' integer
            // discipline. While every operand observed is an `Integer` and
            // the running sum stays within `i64`, accumulate as `i64` so
            // the final JSON serializes as `6` rather than `6.0`. Promote
            // to `f64` on first non-integer operand or on overflow.
            let mut int_sum: i64 = 0;
            let mut float_sum: f64 = 0.0;
            let mut all_integer = true;
            let mut has_value = false;
            for doc in group {
                match eval_scalar(arg, doc, root_alias, params)? {
                    CosmosValue::Integer(n) => {
                        if all_integer {
                            match int_sum.checked_add(n) {
                                Some(v) => int_sum = v,
                                None => {
                                    float_sum = int_sum as f64 + n as f64;
                                    all_integer = false;
                                }
                            }
                        } else {
                            float_sum += n as f64;
                        }
                        has_value = true;
                    }
                    CosmosValue::Number(n) => {
                        if all_integer {
                            float_sum = int_sum as f64 + n;
                            all_integer = false;
                        } else {
                            float_sum += n;
                        }
                        has_value = true;
                    }
                    _ => {}
                }
            }
            if !has_value {
                Ok(CosmosValue::Undefined)
            } else if all_integer {
                Ok(CosmosValue::Integer(int_sum))
            } else {
                Ok(CosmosValue::Number(float_sum))
            }
        }
        "AVG" => {
            let arg = args
                .first()
                .ok_or_else(|| EvalError::TypeError("AVG requires an argument".into()))?;
            // AVG always yields a fractional result; the Cosmos engine
            // returns a JSON number that round-trips as `f64`. We do not
            // bother with the integer-pure path here.
            let mut sum = 0.0f64;
            let mut count = 0i64;
            for doc in group {
                match eval_scalar(arg, doc, root_alias, params)? {
                    CosmosValue::Number(n) => {
                        sum += n;
                        count += 1;
                    }
                    CosmosValue::Integer(n) => {
                        sum += n as f64;
                        count += 1;
                    }
                    _ => {}
                }
            }
            if count > 0 {
                Ok(CosmosValue::Number(sum / count as f64))
            } else {
                Ok(CosmosValue::Undefined)
            }
        }
        "MIN" => {
            let arg = args
                .first()
                .ok_or_else(|| EvalError::TypeError("MIN requires an argument".into()))?;
            // use Cosmos' cross-type total-ordering (`null<bool<num<str<arr<obj`)
            // so MIN over a heterogeneous group returns the smallest value
            // under that ordering rather than "the first non-undefined value
            // encountered". This matches what the Gateway returns.
            let mut min_val: Option<CosmosValue> = None;
            for doc in group {
                let val = eval_scalar(arg, doc, root_alias, params)?;
                if val.is_undefined() {
                    continue;
                }
                min_val = Some(match min_val {
                    None => val,
                    Some(current) => {
                        if total_cmp_for_sort(&val, &current) == Ordering::Less {
                            val
                        } else {
                            current
                        }
                    }
                });
            }
            Ok(min_val.unwrap_or(CosmosValue::Undefined))
        }
        "MAX" => {
            let arg = args
                .first()
                .ok_or_else(|| EvalError::TypeError("MAX requires an argument".into()))?;
            // same cross-type ordering as MIN.
            let mut max_val: Option<CosmosValue> = None;
            for doc in group {
                let val = eval_scalar(arg, doc, root_alias, params)?;
                if val.is_undefined() {
                    continue;
                }
                max_val = Some(match max_val {
                    None => val,
                    Some(current) => {
                        if total_cmp_for_sort(&val, &current) == Ordering::Greater {
                            val
                        } else {
                            current
                        }
                    }
                });
            }
            Ok(max_val.unwrap_or(CosmosValue::Undefined))
        }
        _ => Err(EvalError::UnknownFunction(name.to_string())),
    }
}

/// Evaluate a scalar expression with aggregate awareness.
///
/// Aggregate function calls (COUNT, SUM, etc.) are evaluated over the entire
/// group. All other expressions are evaluated against the representative document.
fn eval_scalar_with_group(
    expr: &SqlScalarExpression,
    representative: &serde_json::Value,
    root_alias: Option<&str>,
    params: &Params,
    group: &[serde_json::Value],
) -> Result<CosmosValue, EvalError> {
    match expr {
        SqlScalarExpression::FunctionCall { name, args, is_udf }
            if !is_udf && is_aggregate_function(name) =>
        {
            eval_aggregate(name, args, group, root_alias, params)
        }
        SqlScalarExpression::FunctionCall { name, args, is_udf } => {
            if *is_udf {
                return Err(EvalError::Unsupported("UDF calls".into()));
            }
            let arg_vals: Result<Vec<CosmosValue>, _> = args
                .iter()
                .map(|a| eval_scalar_with_group(a, representative, root_alias, params, group))
                .collect();
            eval_function(name, &arg_vals?)
        }
        SqlScalarExpression::Binary { op, left, right } => {
            let l = eval_scalar_with_group(left, representative, root_alias, params, group)?;
            let r = eval_scalar_with_group(right, representative, root_alias, params, group)?;
            Ok(eval_binary(*op, &l, &r))
        }
        SqlScalarExpression::Unary { op, operand } => {
            let val = eval_scalar_with_group(operand, representative, root_alias, params, group)?;
            Ok(eval_unary(*op, &val))
        }
        SqlScalarExpression::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            let cond =
                eval_scalar_with_group(condition, representative, root_alias, params, group)?;
            // (#1) Cosmos SQL `?:` is strict-Boolean — see the matching arm in
            // `eval_scalar` for the rationale.
            match cond {
                CosmosValue::Boolean(true) => {
                    eval_scalar_with_group(if_true, representative, root_alias, params, group)
                }
                CosmosValue::Boolean(false) => {
                    eval_scalar_with_group(if_false, representative, root_alias, params, group)
                }
                _ => Ok(CosmosValue::Undefined),
            }
        }
        SqlScalarExpression::Coalesce { left, right } => {
            let val = eval_scalar_with_group(left, representative, root_alias, params, group)?;
            if val.is_undefined() {
                eval_scalar_with_group(right, representative, root_alias, params, group)
            } else {
                Ok(val)
            }
        }
        _ => eval_scalar(expr, representative, root_alias, params),
    }
}

// ─── Projection helpers ──────────────────────────────────────────────────────

/// Project a single row with an explicit root alias (supports JOIN binding contexts).
fn project_row(
    doc: &serde_json::Value,
    query: &SqlQuery,
    root_alias: Option<&str>,
    params: &Params,
) -> Result<serde_json::Value, EvalError> {
    match &query.select.spec {
        SqlSelectSpec::Star => Ok(if root_alias.is_none() {
            project_star_row(doc)
        } else {
            doc.clone()
        }),
        SqlSelectSpec::Value(expr) => {
            let val = eval_scalar(expr, doc, root_alias, params)?;
            Ok(val.to_json())
        }
        SqlSelectSpec::List(items) => {
            let mut obj = serde_json::Map::new();
            for (index, item) in items.iter().enumerate() {
                let val = eval_scalar(&item.expression, doc, root_alias, params)?;
                let key = item
                    .alias
                    .clone()
                    .unwrap_or_else(|| infer_property_name(&item.expression, index + 1));
                if !val.is_undefined() {
                    obj.insert(key, val.to_json());
                }
            }
            Ok(serde_json::Value::Object(obj))
        }
    }
}

/// Project an aggregated group of rows.
fn project_group(
    group: &[serde_json::Value],
    query: &SqlQuery,
    root_alias: Option<&str>,
    params: &Params,
) -> Result<serde_json::Value, EvalError> {
    let empty_obj = serde_json::Value::Object(serde_json::Map::new());
    let representative = group.first().unwrap_or(&empty_obj);
    match &query.select.spec {
        SqlSelectSpec::Star => Ok(if root_alias.is_none() {
            project_star_row(representative)
        } else {
            representative.clone()
        }),
        SqlSelectSpec::Value(expr) => {
            let val = eval_scalar_with_group(expr, representative, root_alias, params, group)?;
            Ok(val.to_json())
        }
        SqlSelectSpec::List(items) => {
            let mut obj = serde_json::Map::new();
            for (index, item) in items.iter().enumerate() {
                let val = eval_scalar_with_group(
                    &item.expression,
                    representative,
                    root_alias,
                    params,
                    group,
                )?;
                let key = item
                    .alias
                    .clone()
                    .unwrap_or_else(|| infer_property_name(&item.expression, index + 1));
                if !val.is_undefined() {
                    obj.insert(key, val.to_json());
                }
            }
            Ok(serde_json::Value::Object(obj))
        }
    }
}

// ─── ORDER BY helpers ────────────────────────────────────────────────────────

/// Type ordering for cross-type ORDER BY comparisons.
fn sort_type_order(v: &CosmosValue) -> u8 {
    match v {
        CosmosValue::Null => 0,
        CosmosValue::Boolean(_) => 1,
        CosmosValue::Number(_) | CosmosValue::Integer(_) => 2,
        CosmosValue::String(_) => 3,
        CosmosValue::Array(_) => 4,
        CosmosValue::Object(_) => 5,
        CosmosValue::Undefined => 6,
    }
}

/// Total comparison for ORDER BY (handles cross-type and undefined).
fn total_cmp_for_sort(a: &CosmosValue, b: &CosmosValue) -> Ordering {
    a.cosmos_cmp(b)
        .unwrap_or_else(|| sort_type_order(a).cmp(&sort_type_order(b)))
}

/// Compare two documents according to an ORDER BY clause.
///
/// F20 note: superseded by inline pre-computed-keys sort in `query_documents`
/// (eval errors must propagate, which `sort_by` cannot do). Kept for now
/// behind `#[allow(dead_code)]` until callers outside this module are removed.
#[allow(dead_code)]
fn compare_for_order_by(
    a: &serde_json::Value,
    b: &serde_json::Value,
    order_by: &SqlOrderByClause,
    root_alias: Option<&str>,
    params: &Params,
) -> Ordering {
    for item in &order_by.items {
        let va =
            eval_scalar(&item.expression, a, root_alias, params).unwrap_or(CosmosValue::Undefined);
        let vb =
            eval_scalar(&item.expression, b, root_alias, params).unwrap_or(CosmosValue::Undefined);
        let cmp = match item.order {
            SqlSortOrder::Descending => total_cmp_for_sort(&va, &vb).reverse(),
            _ => total_cmp_for_sort(&va, &vb),
        };
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}

#[allow(dead_code)] // superseded by inline pre-computed-keys sort.
fn compare_for_grouped_order_by(
    projected_a: &serde_json::Value,
    group_a: &[serde_json::Value],
    projected_b: &serde_json::Value,
    group_b: &[serde_json::Value],
    order_by: &SqlOrderByClause,
    root_alias: Option<&str>,
    params: &Params,
) -> Ordering {
    let null = serde_json::Value::Null;
    let representative_a = group_a.first().unwrap_or(&null);
    let representative_b = group_b.first().unwrap_or(&null);

    for item in &order_by.items {
        let va = eval_grouped_order_by_value(
            projected_a,
            representative_a,
            group_a,
            &item.expression,
            root_alias,
            params,
        )
        .unwrap_or(CosmosValue::Undefined);
        let vb = eval_grouped_order_by_value(
            projected_b,
            representative_b,
            group_b,
            &item.expression,
            root_alias,
            params,
        )
        .unwrap_or(CosmosValue::Undefined);
        let cmp = match item.order {
            SqlSortOrder::Descending => total_cmp_for_sort(&va, &vb).reverse(),
            _ => total_cmp_for_sort(&va, &vb),
        };
        if cmp != Ordering::Equal {
            return cmp;
        }
    }

    Ordering::Equal
}

fn eval_grouped_order_by_value(
    projected_row: &serde_json::Value,
    representative: &serde_json::Value,
    group: &[serde_json::Value],
    expression: &SqlScalarExpression,
    root_alias: Option<&str>,
    params: &Params,
) -> Result<CosmosValue, EvalError> {
    match eval_scalar(expression, projected_row, None, params) {
        Ok(value) if !value.is_undefined() => Ok(value),
        Ok(_) | Err(_) => {
            eval_scalar_with_group(expression, representative, root_alias, params, group)
        }
    }
}

/// Evaluate a WHERE clause against a document with an explicit root alias.
fn eval_where(
    doc: &serde_json::Value,
    where_clause: &Option<SqlWhereClause>,
    root_alias: Option<&str>,
    params: &Params,
) -> Result<bool, EvalError> {
    if let Some(wc) = where_clause {
        let result = eval_scalar(&wc.expression, doc, root_alias, params)?;
        Ok(matches!(result, CosmosValue::Boolean(true)))
    } else {
        Ok(true)
    }
}

/// Execute a full query against an in-memory collection of documents.
///
/// Supports WHERE filtering, SELECT projection, TOP/OFFSET/LIMIT,
/// ORDER BY, GROUP BY with aggregates, and intra-document JOINs.
///
/// # Examples
///
/// ```ignore
/// use azure_data_cosmos_driver::query::eval;
/// let docs = vec![
///     serde_json::json!({"name": "Alice", "age": 30}),
///     serde_json::json!({"name": "Bob", "age": 20}),
/// ];
/// let results = eval::query_documents(
///     "SELECT c.name FROM c WHERE c.age > 21",
///     &[],
///     &docs,
/// ).unwrap();
/// assert_eq!(results.len(), 1);
/// assert_eq!(results[0]["name"], "Alice");
/// ```
pub fn query_documents(
    sql: &str,
    parameters: &Params,
    documents: &[serde_json::Value],
) -> azure_core::Result<Vec<serde_json::Value>> {
    let program = crate::query::parse(sql).map_err(|e| {
        crate::error::Error::serialization(format!("failed to parse query: {e}"), None, None, e)
    })?;
    let query = &program.query;
    let root_alias = get_root_alias(query);

    let use_binding_context = query
        .from
        .as_ref()
        .is_some_and(|from| !is_plain_root_from(&from.collection));

    // Binding-context queries (joins, array iterators, aliased subpaths) must
    // resolve PropertyRef against the row context rather than treating the root
    // alias as the full current document.
    let eval_alias = if use_binding_context {
        None
    } else {
        root_alias.as_deref()
    };

    // ── Step 1: expand JOINs + apply WHERE filter ────────────────────────
    let mut filtered_rows: Vec<serde_json::Value> = Vec::new();

    for doc in documents {
        if use_binding_context {
            let from = &query.from.as_ref().unwrap().collection;
            let bindings_list = expand_from(doc, from, &serde_json::Map::new())
                .map_err(|e| crate::error::Error::client(e.to_string(), None))?;
            for bindings in bindings_list {
                let ctx = serde_json::Value::Object(bindings);
                if eval_where(&ctx, &query.where_clause, None, parameters)
                    .map_err(|e| crate::error::Error::client(e.to_string(), None))?
                {
                    filtered_rows.push(ctx);
                }
            }
        } else if eval_where(doc, &query.where_clause, eval_alias, parameters)
            .map_err(|e| crate::error::Error::client(e.to_string(), None))?
        {
            filtered_rows.push(doc.clone());
        }
    }

    // ── Step 2: GROUP BY / aggregates, or plain projection ───────────────
    let use_aggregates = query.group_by.is_some() || select_has_aggregates(query);

    let (mut results, originals, groups): (
        Vec<serde_json::Value>,
        Vec<serde_json::Value>,
        Option<Vec<Vec<serde_json::Value>>>,
    ) = if use_aggregates {
        if let Some(group_by) = &query.group_by {
            // Explicit GROUP BY — partition rows into groups by key.
            let mut groups: Vec<Vec<serde_json::Value>> = Vec::new();
            let mut key_map: HashMap<String, usize> = HashMap::new();

            for row in &filtered_rows {
                let key_parts: Result<Vec<serde_json::Value>, _> = group_by
                    .expressions
                    .iter()
                    .map(|e| eval_scalar(e, row, eval_alias, parameters).map(|v| v.to_json()))
                    .collect();
                let key = serde_json::to_string(
                    &key_parts.map_err(|e| crate::error::Error::client(e.to_string(), None))?,
                )
                .unwrap_or_default();

                if let Some(&idx) = key_map.get(&key) {
                    groups[idx].push(row.clone());
                } else {
                    key_map.insert(key, groups.len());
                    groups.push(vec![row.clone()]);
                }
            }

            let mut projected = Vec::new();
            let mut reps = Vec::new();
            for group in &groups {
                projected.push(
                    project_group(group, query, eval_alias, parameters)
                        .map_err(|e| crate::error::Error::client(e.to_string(), None))?,
                );
                reps.push(group[0].clone());
            }
            (projected, reps, Some(groups))
        } else {
            // Aggregates without GROUP BY → implicit single group over all rows.
            let projected = project_group(&filtered_rows, query, eval_alias, parameters)
                .map_err(|e| crate::error::Error::client(e.to_string(), None))?;
            let rep = filtered_rows
                .first()
                .cloned()
                .unwrap_or(serde_json::Value::Null);
            (
                vec![projected],
                vec![rep],
                Some(vec![filtered_rows.clone()]),
            )
        }
    } else {
        // No aggregates — project each row individually.
        let mut projected = Vec::new();
        let originals = filtered_rows.clone();
        for row in &filtered_rows {
            projected.push(
                project_row(row, query, eval_alias, parameters)
                    .map_err(|e| crate::error::Error::client(e.to_string(), None))?,
            );
        }
        (projected, originals, None)
    };

    // ── Step 3: ORDER BY ─────────────────────────────────────────────────
    //
    // pre-compute ORDER BY keys so eval errors propagate (the previous
    // `sort_by` swallowed them as `Undefined`, hiding bugs like an unbound
    // parameter and producing nondeterministic ordering). The emulator now
    // surfaces a typed error rather than silently returning incorrect rows.
    if let Some(order_by) = &query.order_by {
        let mut keys: Vec<Vec<CosmosValue>> = Vec::with_capacity(results.len());
        for i in 0..results.len() {
            let mut row_keys = Vec::with_capacity(order_by.items.len());
            for item in &order_by.items {
                let v = if let Some(groups) = &groups {
                    let null = serde_json::Value::Null;
                    let representative = groups[i].first().unwrap_or(&null);
                    eval_grouped_order_by_value(
                        &results[i],
                        representative,
                        &groups[i],
                        &item.expression,
                        eval_alias,
                        parameters,
                    )
                    .map_err(|e| crate::error::Error::client(e.to_string(), None))?
                } else {
                    eval_scalar(&item.expression, &originals[i], eval_alias, parameters)
                        .map_err(|e| crate::error::Error::client(e.to_string(), None))?
                };
                row_keys.push(v);
            }
            keys.push(row_keys);
        }
        let mut indices: Vec<usize> = (0..results.len()).collect();
        indices.sort_by(|&a, &b| {
            for (idx, item) in order_by.items.iter().enumerate() {
                let cmp = match item.order {
                    SqlSortOrder::Descending => {
                        total_cmp_for_sort(&keys[a][idx], &keys[b][idx]).reverse()
                    }
                    _ => total_cmp_for_sort(&keys[a][idx], &keys[b][idx]),
                };
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        });
        results = indices.iter().map(|&i| results[i].clone()).collect();
    }

    // ── Step 4: TOP ──────────────────────────────────────────────────────
    if let Some(top) = &query.select.top {
        let n = match top {
            SqlTopSpec::Literal(n) => usize::try_from(*n).map_err(|_| {
                crate::error::Error::client(
                    format!("TOP literal must be non-negative; got {n}"),
                    None,
                )
            })?,
            SqlTopSpec::Parameter(name) => resolve_integer_param(parameters, name)
                .map_err(|e| crate::error::Error::client(e.to_string(), None))?
                as usize,
        };
        results.truncate(n);
    }

    // ── Step 5: OFFSET / LIMIT ───────────────────────────────────────────
    if let Some(ol) = &query.offset_limit {
        let offset = match &ol.offset {
            SqlOffsetSpec::Literal(n) => usize::try_from(*n).map_err(|_| {
                crate::error::Error::client(
                    format!("OFFSET literal must be non-negative; got {n}"),
                    None,
                )
            })?,
            SqlOffsetSpec::Parameter(name) => resolve_integer_param(parameters, name)
                .map_err(|e| crate::error::Error::client(e.to_string(), None))?
                as usize,
        };
        let limit = match &ol.limit {
            SqlLimitSpec::Literal(n) => usize::try_from(*n).map_err(|_| {
                crate::error::Error::client(
                    format!("LIMIT literal must be non-negative; got {n}"),
                    None,
                )
            })?,
            SqlLimitSpec::Parameter(name) => resolve_integer_param(parameters, name)
                .map_err(|e| crate::error::Error::client(e.to_string(), None))?
                as usize,
        };
        if offset < results.len() {
            results = results[offset..].to_vec();
        } else {
            results.clear();
        }
        results.truncate(limit);
    }

    Ok(results)
}

/// Resolve a parameter to a non-negative integer value for TOP/OFFSET/LIMIT.
///
/// Thin `EvalError`-flavored wrapper around the shared
/// [`resolve_non_negative_integer_parameter`] helper so the eval and plan
/// pipelines validate parameters identically.
fn resolve_integer_param(parameters: &Params, name: &str) -> Result<i64, EvalError> {
    if resolve_parameter_value(parameters, name).is_none() {
        return Err(EvalError::ParameterNotFound(
            normalize_parameter_name(name).to_string(),
        ));
    }
    resolve_non_negative_integer_parameter(parameters, name).map_err(EvalError::TypeError)
}

/// Evaluate a scalar expression against a document.
fn eval_scalar(
    expr: &SqlScalarExpression,
    doc: &serde_json::Value,
    root_alias: Option<&str>,
    params: &Params,
) -> Result<CosmosValue, EvalError> {
    match expr {
        SqlScalarExpression::Literal(lit) => Ok(eval_literal(lit)),

        SqlScalarExpression::PropertyRef(name) => {
            // If name matches the root alias, return the whole document
            if root_alias == Some(name.as_str()) {
                Ok(CosmosValue::from_json(doc))
            } else {
                // Try as a direct property of the document
                match doc.get(name) {
                    Some(v) => Ok(CosmosValue::from_json(v)),
                    None => Ok(CosmosValue::Undefined),
                }
            }
        }

        SqlScalarExpression::MemberRef { source, member } => {
            let source_val = eval_scalar(source, doc, root_alias, params)?;
            Ok(member_access(&source_val, member))
        }

        SqlScalarExpression::MemberIndexer { source, index } => {
            let source_val = eval_scalar(source, doc, root_alias, params)?;
            let index_val = eval_scalar(index, doc, root_alias, params)?;
            Ok(indexer_access(&source_val, &index_val))
        }

        SqlScalarExpression::Binary { op, left, right } => {
            let left_val = eval_scalar(left, doc, root_alias, params)?;
            let right_val = eval_scalar(right, doc, root_alias, params)?;
            Ok(eval_binary(*op, &left_val, &right_val))
        }

        SqlScalarExpression::Unary { op, operand } => {
            let val = eval_scalar(operand, doc, root_alias, params)?;
            Ok(eval_unary(*op, &val))
        }

        SqlScalarExpression::FunctionCall {
            name, args, is_udf, ..
        } => {
            if *is_udf {
                return Err(EvalError::Unsupported("UDF calls".into()));
            }
            let arg_vals: Result<Vec<CosmosValue>, _> = args
                .iter()
                .map(|a| eval_scalar(a, doc, root_alias, params))
                .collect();
            eval_function(name, &arg_vals?)
        }

        SqlScalarExpression::In {
            expression,
            items,
            not,
        } => {
            let val = eval_scalar(expression, doc, root_alias, params)?;
            let mut found = false;
            for item in items {
                let item_val = eval_scalar(item, doc, root_alias, params)?;
                if matches!(val.cosmos_eq(&item_val), CosmosValue::Boolean(true)) {
                    found = true;
                    break;
                }
            }
            Ok(CosmosValue::Boolean(if *not { !found } else { found }))
        }

        SqlScalarExpression::Between {
            expression,
            low,
            high,
            not,
        } => {
            let val = eval_scalar(expression, doc, root_alias, params)?;
            let low_val = eval_scalar(low, doc, root_alias, params)?;
            let high_val = eval_scalar(high, doc, root_alias, params)?;
            let in_range = match (val.cosmos_cmp(&low_val), val.cosmos_cmp(&high_val)) {
                (Some(lo), Some(hi)) => {
                    (lo == Ordering::Greater || lo == Ordering::Equal)
                        && (hi == Ordering::Less || hi == Ordering::Equal)
                }
                _ => false,
            };
            Ok(CosmosValue::Boolean(if *not {
                !in_range
            } else {
                in_range
            }))
        }

        SqlScalarExpression::Like {
            expression,
            pattern,
            escape,
            not,
        } => {
            let val = eval_scalar(expression, doc, root_alias, params)?;
            let pattern_val = eval_scalar(pattern, doc, root_alias, params)?;
            // validate that the ESCAPE clause supplies exactly one
            // character. Cosmos rejects multi-character escape literals; the
            // previous code silently used the first char and dropped the
            // rest, hiding caller mistakes. Treat invalid escapes as
            // `Undefined` (the row will not match) which is the closest
            // emulator-friendly approximation of the Gateway's error.
            if let Some(esc) = escape.as_deref() {
                if esc.chars().count() != 1 {
                    return Ok(CosmosValue::Undefined);
                }
            }
            match (&val, &pattern_val) {
                (CosmosValue::String(s), CosmosValue::String(p)) => {
                    let matched = sql_like_match(s, p, escape.as_deref());
                    Ok(CosmosValue::Boolean(if *not { !matched } else { matched }))
                }
                _ => Ok(CosmosValue::Undefined),
            }
        }

        SqlScalarExpression::Conditional {
            condition,
            if_true,
            if_false,
        } => {
            let cond = eval_scalar(condition, doc, root_alias, params)?;
            // (#1) Cosmos SQL `?:` is strict-Boolean: a non-Boolean condition
            // (Number, String, Null, Undefined, Array, Object) yields
            // `Undefined`, which causes the surrounding row to be filtered out.
            // This is *not* JS truthiness — do not call `internal_js_truthy`.
            match cond {
                CosmosValue::Boolean(true) => eval_scalar(if_true, doc, root_alias, params),
                CosmosValue::Boolean(false) => eval_scalar(if_false, doc, root_alias, params),
                _ => Ok(CosmosValue::Undefined),
            }
        }

        SqlScalarExpression::Coalesce { left, right } => {
            let val = eval_scalar(left, doc, root_alias, params)?;
            if val.is_undefined() {
                eval_scalar(right, doc, root_alias, params)
            } else {
                Ok(val)
            }
        }

        SqlScalarExpression::ArrayCreate(items) => {
            let vals: Result<Vec<CosmosValue>, _> = items
                .iter()
                .map(|i| eval_scalar(i, doc, root_alias, params))
                .collect();
            Ok(CosmosValue::Array(vals?))
        }

        SqlScalarExpression::ObjectCreate(props) => {
            let mut result = Vec::new();
            for prop in props {
                let val = eval_scalar(&prop.expression, doc, root_alias, params)?;
                result.push((prop.name.clone(), val));
            }
            Ok(CosmosValue::Object(result))
        }

        SqlScalarExpression::ParameterRef(name) => {
            if let Some(value) = resolve_parameter_value(params, name) {
                Ok(CosmosValue::from_json(value))
            } else {
                Err(EvalError::ParameterNotFound(name.clone()))
            }
        }

        SqlScalarExpression::IsNull { expression, not } => {
            let val = eval_scalar(expression, doc, root_alias, params)?;
            let is_null = matches!(val, CosmosValue::Null);
            Ok(CosmosValue::Boolean(if *not { !is_null } else { is_null }))
        }

        SqlScalarExpression::Exists(_)
        | SqlScalarExpression::Subquery(_)
        | SqlScalarExpression::Array(_) => Err(EvalError::Unsupported("subqueries".into())),
    }
}

fn eval_literal(lit: &SqlLiteral) -> CosmosValue {
    match lit {
        SqlLiteral::String(s) => CosmosValue::String(s.clone()),
        SqlLiteral::Number(n) => CosmosValue::Number(*n),
        SqlLiteral::Integer(n) => CosmosValue::Integer(*n),
        SqlLiteral::Boolean(b) => CosmosValue::Boolean(*b),
        SqlLiteral::Null => CosmosValue::Null,
        SqlLiteral::Undefined => CosmosValue::Undefined,
    }
}

fn member_access(source: &CosmosValue, member: &str) -> CosmosValue {
    match source {
        CosmosValue::Object(props) => {
            for (k, v) in props {
                if k == member {
                    return v.clone();
                }
            }
            CosmosValue::Undefined
        }
        _ => CosmosValue::Undefined,
    }
}

fn indexer_access(source: &CosmosValue, index: &CosmosValue) -> CosmosValue {
    match (source, index) {
        (CosmosValue::Array(arr), CosmosValue::Number(n)) => {
            if *n < 0.0 || n.fract() != 0.0 {
                return CosmosValue::Undefined;
            }
            let idx = *n as usize;
            arr.get(idx).cloned().unwrap_or(CosmosValue::Undefined)
        }
        (CosmosValue::Array(arr), CosmosValue::Integer(n)) => {
            if *n < 0 {
                return CosmosValue::Undefined;
            }
            let idx = *n as usize;
            arr.get(idx).cloned().unwrap_or(CosmosValue::Undefined)
        }
        (CosmosValue::Object(props), CosmosValue::String(key)) => {
            for (k, v) in props {
                if k == key {
                    return v.clone();
                }
            }
            CosmosValue::Undefined
        }
        _ => CosmosValue::Undefined,
    }
}

fn eval_binary(op: SqlBinaryOp, left: &CosmosValue, right: &CosmosValue) -> CosmosValue {
    match op {
        SqlBinaryOp::Equal => left.cosmos_eq(right),
        SqlBinaryOp::NotEqual => match left.cosmos_eq(right) {
            CosmosValue::Boolean(b) => CosmosValue::Boolean(!b),
            other => other,
        },
        SqlBinaryOp::LessThan => match left.cosmos_cmp(right) {
            Some(Ordering::Less) => CosmosValue::Boolean(true),
            Some(_) => CosmosValue::Boolean(false),
            None => CosmosValue::Undefined,
        },
        SqlBinaryOp::GreaterThan => match left.cosmos_cmp(right) {
            Some(Ordering::Greater) => CosmosValue::Boolean(true),
            Some(_) => CosmosValue::Boolean(false),
            None => CosmosValue::Undefined,
        },
        SqlBinaryOp::LessThanOrEqual => match left.cosmos_cmp(right) {
            Some(Ordering::Less | Ordering::Equal) => CosmosValue::Boolean(true),
            Some(_) => CosmosValue::Boolean(false),
            None => CosmosValue::Undefined,
        },
        SqlBinaryOp::GreaterThanOrEqual => match left.cosmos_cmp(right) {
            Some(Ordering::Greater | Ordering::Equal) => CosmosValue::Boolean(true),
            Some(_) => CosmosValue::Boolean(false),
            None => CosmosValue::Undefined,
        },
        SqlBinaryOp::And => eval_and(left, right),
        SqlBinaryOp::Or => eval_or(left, right),
        // when both sides are `Integer`, prefer i64 arithmetic and only
        // promote to `f64` on overflow. The previous `(a as f64) + (b as f64)`
        // path silently lost precision past 2^53 and changed the JSON
        // serialization from `6` to `6.0`, breaking gateway-comparison parity.
        SqlBinaryOp::Add => arith_op(left, right, i64::checked_add, |a, b| Some(a + b)),
        SqlBinaryOp::Subtract => arith_op(left, right, i64::checked_sub, |a, b| Some(a - b)),
        SqlBinaryOp::Multiply => arith_op(left, right, i64::checked_mul, |a, b| Some(a * b)),
        // Division and modulo by zero return `Undefined` (matches Cosmos SQL
        // semantics) rather than producing a non-finite `f64`. The local plan
        // generator's PK-value invariant (`#13`) and the JSON serializer in
        // `value::to_json` both rely on `CosmosValue::Number` always carrying a
        // finite value, so we never produce `NaN` / `+Inf` / `-Inf` here.
        SqlBinaryOp::Divide => numeric_op(left, right, |a, b| {
            if b == 0.0 {
                None
            } else {
                let r = a / b;
                if r.is_finite() {
                    Some(r)
                } else {
                    None
                }
            }
        }),
        SqlBinaryOp::Modulo => numeric_op(left, right, |a, b| {
            if b == 0.0 {
                None
            } else {
                let r = a % b;
                if r.is_finite() {
                    Some(r)
                } else {
                    None
                }
            }
        }),
        SqlBinaryOp::StringConcat => match (left, right) {
            (CosmosValue::String(a), CosmosValue::String(b)) => {
                CosmosValue::String(format!("{a}{b}"))
            }
            _ => CosmosValue::Undefined,
        },
        SqlBinaryOp::BitwiseAnd => int_op(left, right, |a, b| a & b),
        SqlBinaryOp::BitwiseOr => int_op(left, right, |a, b| a | b),
        SqlBinaryOp::BitwiseXor => int_op(left, right, |a, b| a ^ b),
        SqlBinaryOp::LeftShift => int_op(left, right, |a, b| a << (b & 0x3F)),
        SqlBinaryOp::RightShift => int_op(left, right, |a, b| a >> (b & 0x3F)),
        SqlBinaryOp::ZeroFillRightShift => int_op(left, right, |a, b| {
            ((a as u64) >> ((b as u64) & 0x3F)) as i64
        }),
    }
}

/// Coerce a value to a strict Boolean operand for SQL three-valued logic.
///
/// In Cosmos DB SQL, `AND`/`OR`/`NOT` operate only on `Boolean` values; any
/// other type (including non-zero numbers or non-empty strings) is treated as
/// `Undefined`. This mirrors the engine's behavior — `WHERE 1 AND TRUE` does
/// **not** match documents because `1` is not a Boolean.
fn as_bool(value: &CosmosValue) -> Option<bool> {
    match value {
        CosmosValue::Boolean(b) => Some(*b),
        _ => None,
    }
}

/// Three-valued AND with strict-Boolean operands.
///
/// Truth table (`U` = `Undefined`):
///   T AND T = T,  T AND F = F,  T AND U = U
///   F AND _ = F,  U AND F = F,  U AND U = U,  U AND T = U
/// Any non-Boolean operand is coerced to `U` per Cosmos semantics.
fn eval_and(left: &CosmosValue, right: &CosmosValue) -> CosmosValue {
    match (as_bool(left), as_bool(right)) {
        // `false` short-circuits regardless of the other side.
        (Some(false), _) | (_, Some(false)) => CosmosValue::Boolean(false),
        (Some(true), Some(true)) => CosmosValue::Boolean(true),
        // `true AND undefined` and `undefined AND undefined` are both undefined.
        _ => CosmosValue::Undefined,
    }
}

/// Three-valued OR with strict-Boolean operands.
///
/// Truth table (`U` = `Undefined`):
///   T OR _ = T,  _ OR T = T
///   F OR F = F,  F OR U = U,  U OR F = U,  U OR U = U
fn eval_or(left: &CosmosValue, right: &CosmosValue) -> CosmosValue {
    match (as_bool(left), as_bool(right)) {
        (Some(true), _) | (_, Some(true)) => CosmosValue::Boolean(true),
        (Some(false), Some(false)) => CosmosValue::Boolean(false),
        _ => CosmosValue::Undefined,
    }
}

fn eval_unary(op: SqlUnaryOp, val: &CosmosValue) -> CosmosValue {
    match op {
        SqlUnaryOp::Not => match val {
            CosmosValue::Boolean(b) => CosmosValue::Boolean(!b),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::Minus => match val {
            CosmosValue::Number(n) => CosmosValue::Number(-n),
            // Cosmos backend (the C++ engine this is ported from) wraps
            // on integer negation overflow rather than panicking. `-i64::MIN`
            // would panic in debug and wrap in release with the default
            // `Neg`; use `wrapping_neg` for predictable behavior in both.
            CosmosValue::Integer(n) => CosmosValue::Integer(n.wrapping_neg()),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::Plus => match val {
            CosmosValue::Number(n) => CosmosValue::Number(*n),
            CosmosValue::Integer(n) => CosmosValue::Integer(*n),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::BitwiseNot => match val {
            // Cosmos rejects non-integral bitwise input — a fractional
            // `Number` cannot be bitwise-negated. Match that behavior by
            // returning `Undefined` instead of silently truncating.
            CosmosValue::Number(n) if n.fract() == 0.0 && n.is_finite() => {
                CosmosValue::Integer(!(*n as i64))
            }
            CosmosValue::Number(_) => CosmosValue::Undefined,
            CosmosValue::Integer(n) => CosmosValue::Integer(!n),
            _ => CosmosValue::Undefined,
        },
    }
}

fn numeric_op(
    left: &CosmosValue,
    right: &CosmosValue,
    f: fn(f64, f64) -> Option<f64>,
) -> CosmosValue {
    let pair = match (left, right) {
        (CosmosValue::Number(a), CosmosValue::Number(b)) => Some((*a, *b)),
        (CosmosValue::Integer(a), CosmosValue::Integer(b)) => Some((*a as f64, *b as f64)),
        (CosmosValue::Number(a), CosmosValue::Integer(b)) => Some((*a, *b as f64)),
        (CosmosValue::Integer(a), CosmosValue::Number(b)) => Some((*a as f64, *b)),
        _ => None,
    };
    match pair.and_then(|(a, b)| f(a, b)) {
        Some(n) => CosmosValue::Number(n),
        None => CosmosValue::Undefined,
    }
}

/// Integer-pure arithmetic with f64 fallback.
///
/// When both operands are `Integer`, evaluate via `int_fn` (a `checked_*`
/// `i64` op). On `Some(v)` keep the result as `Integer(v)` so that the JSON
/// serialization preserves Cosmos' integer type discipline (`6` rather than
/// `6.0`). On `None` (overflow), promote to `f64` so the operation still
/// yields a well-defined numeric result. When either operand is already a
/// floating-point `Number`, fall back to `float_fn` directly.
fn arith_op(
    left: &CosmosValue,
    right: &CosmosValue,
    int_fn: fn(i64, i64) -> Option<i64>,
    float_fn: fn(f64, f64) -> Option<f64>,
) -> CosmosValue {
    match (left, right) {
        (CosmosValue::Integer(a), CosmosValue::Integer(b)) => match int_fn(*a, *b) {
            Some(r) => CosmosValue::Integer(r),
            None => match float_fn(*a as f64, *b as f64) {
                Some(r) if r.is_finite() => CosmosValue::Number(r),
                _ => CosmosValue::Undefined,
            },
        },
        _ => numeric_op(left, right, float_fn),
    }
}

fn int_op(left: &CosmosValue, right: &CosmosValue, f: fn(i64, i64) -> i64) -> CosmosValue {
    // (#5) `f64 as i64` is a saturating conversion in Rust >= 1.45 (values
    // outside `i64::MIN..=i64::MAX` clamp to the boundary, NaN converts to 0).
    // This is intentionally distinct from JS bitwise semantics (which truncate
    // to int32) - the in-memory evaluator targets emulator scenarios and the
    // Gateway is the source of truth for parity-sensitive workloads.
    let to_i64 = |v: &CosmosValue| -> Option<i64> {
        match v {
            CosmosValue::Number(n) => Some(*n as i64),
            CosmosValue::Integer(n) => Some(*n),
            _ => None,
        }
    };
    match (to_i64(left), to_i64(right)) {
        (Some(a), Some(b)) => CosmosValue::Integer(f(a, b)),
        _ => CosmosValue::Undefined,
    }
}

// ─── Built-in functions ──────────────────────────────────────────────────────
// (#16) The built-in function dispatch table and its helpers eval_function /
// num_fn1 / num_fn2 / as_number live in the sibling builtins module to keep
// this file focused on AST traversal.

/// SQL LIKE pattern matching.
fn sql_like_match(text: &str, pattern: &str, escape: Option<&str>) -> bool {
    let escape_char = escape.and_then(|e| e.chars().next());
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    like_match_dp(&text_chars, &pattern_chars, escape_char)
}

fn like_match_dp(text: &[char], pattern: &[char], escape: Option<char>) -> bool {
    let n = text.len();
    let m = pattern.len();
    // dp[i][j] = true means text[i..] matches pattern[j..]
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[n][m] = true;

    // Fill backwards
    for pi in (0..m).rev() {
        for ti in (0..=n).rev() {
            let pc = pattern[pi];

            // Check for escape character
            if Some(pc) == escape && pi + 1 < m {
                // Next character is literal
                dp[ti][pi] = ti < n && text[ti] == pattern[pi + 1] && dp[ti + 1][pi + 2];
                continue;
            }

            dp[ti][pi] = match pc {
                '%' => {
                    // Match zero or more: either skip % or consume one char
                    dp[ti][pi + 1] || (ti < n && dp[ti + 1][pi])
                }
                '_' => ti < n && dp[ti + 1][pi + 1],
                _ => ti < n && text[ti] == pc && dp[ti + 1][pi + 1],
            };
        }
    }
    dp[0][0]
}

/// Infer a property name from a select expression for unnamed columns.
fn infer_property_name(expr: &SqlScalarExpression, position: usize) -> String {
    match expr {
        SqlScalarExpression::PropertyRef(name) => name.clone(),
        SqlScalarExpression::MemberRef { member, .. } => member.clone(),
        SqlScalarExpression::FunctionCall { name, .. } => name.clone(),
        _ => format!("${position}"),
    }
}

fn project_star_row(doc: &serde_json::Value) -> serde_json::Value {
    match doc {
        serde_json::Value::Object(map) if map.len() == 1 => map
            .values()
            .next()
            .cloned()
            .unwrap_or(serde_json::Value::Null),
        _ => doc.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_simple_where() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.age > 21").unwrap();
        let doc = serde_json::json!({"age": 30});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"age": 18});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn match_equality() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.name = 'Alice'").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"name": "Bob"});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn match_and_or() {
        let p =
            crate::query::parse("SELECT * FROM c WHERE c.age > 18 AND c.name = 'Alice'").unwrap();
        let doc = serde_json::json!({"name": "Alice", "age": 30});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"name": "Alice", "age": 16});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn match_no_where() {
        let p = crate::query::parse("SELECT * FROM c").unwrap();
        let doc = serde_json::json!({"anything": true});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn project_star() {
        let p = crate::query::parse("SELECT * FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, doc);
    }

    #[test]
    fn project_fields() {
        let p = crate::query::parse("SELECT c.name, c.age FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice", "age": 30, "extra": true});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!({"name": "Alice", "age": 30}));
    }

    #[test]
    fn project_value() {
        let p = crate::query::parse("SELECT VALUE c.name FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("Alice"));
    }

    #[test]
    fn project_with_alias() {
        let p = crate::query::parse("SELECT c.name AS n FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!({"n": "Alice"}));
    }

    #[test]
    fn math_functions_accept_integer_literals() {
        let doc = serde_json::json!({});
        let abs = project(
            &doc,
            &crate::query::parse("SELECT VALUE ABS(-20) FROM c")
                .unwrap()
                .query,
            &[],
        )
        .unwrap();
        let power = project(
            &doc,
            &crate::query::parse("SELECT VALUE POWER(12, 2) FROM c")
                .unwrap()
                .query,
            &[],
        )
        .unwrap();
        let round = project(
            &doc,
            &crate::query::parse("SELECT VALUE ROUND(12) FROM c")
                .unwrap()
                .query,
            &[],
        )
        .unwrap();
        let sqrt = project(
            &doc,
            &crate::query::parse("SELECT VALUE SQRT(144) FROM c")
                .unwrap()
                .query,
            &[],
        )
        .unwrap();

        assert_eq!(abs, serde_json::json!(20.0));
        assert_eq!(power, serde_json::json!(144.0));
        assert_eq!(round, serde_json::json!(12.0));
        assert_eq!(sqrt, serde_json::json!(12.0));
    }

    #[test]
    fn array_slice_accepts_integer_arguments() {
        let p = crate::query::parse("SELECT VALUE ARRAY_SLICE(c.scores, 0, 1) FROM c").unwrap();
        let doc = serde_json::json!({"scores": [99, 42]});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!([99]));
    }

    #[test]
    fn unnamed_computed_projections_use_unique_synthesized_names() {
        // integer-pure arithmetic stays as `Integer`, so `1 + 1` serializes
        // as `2` (matching Cosmos' integer discipline) rather than `2.0`.
        let p = crate::query::parse("SELECT 1 + 1, 2 + 2 FROM c").unwrap();
        let doc = serde_json::json!({});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!({"$1": 2, "$2": 4}));
    }

    /// large integer arithmetic preserves precision (no `as f64` collapse).
    #[test]
    fn integer_arithmetic_preserves_i64_precision() {
        let p = crate::query::parse("SELECT VALUE 9007199254740992 + 1 FROM c").unwrap();
        let doc = serde_json::json!({});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(9007199254740993i64));
    }

    /// integer arithmetic that overflows i64 promotes to f64 (no panic).
    #[test]
    fn integer_arithmetic_overflow_promotes_to_f64() {
        let p = crate::query::parse(&format!("SELECT VALUE {} + 1 FROM c", i64::MAX)).unwrap();
        let doc = serde_json::json!({});
        let result = project(&doc, &p.query, &[]).unwrap();
        // Promoted to f64 — exact value is i64::MAX as f64 + 1.0, which rounds.
        assert!(result.is_number());
    }

    /// MIN over a heterogeneous group uses Cosmos' total ordering, so the
    /// smallest item is the boolean (which sorts below any number/string).
    #[test]
    fn min_aggregate_uses_cross_type_total_ordering() {
        let docs = vec![
            serde_json::json!({"v": "alpha"}),
            serde_json::json!({"v": 1}),
            serde_json::json!({"v": false}),
        ];
        let results = query_documents("SELECT VALUE MIN(c.v) FROM c", &[], &docs).unwrap();
        assert_eq!(results, vec![serde_json::json!(false)]);
    }

    /// MAX over a heterogeneous group returns the largest item under
    /// Cosmos' total ordering — the string `"alpha"` outranks the number `1`
    /// and the boolean `false`.
    #[test]
    fn max_aggregate_uses_cross_type_total_ordering() {
        let docs = vec![
            serde_json::json!({"v": "alpha"}),
            serde_json::json!({"v": 1}),
            serde_json::json!({"v": false}),
        ];
        let results = query_documents("SELECT VALUE MAX(c.v) FROM c", &[], &docs).unwrap();
        assert_eq!(results, vec![serde_json::json!("alpha")]);
    }

    /// unary `-i64::MIN` must not panic and must wrap (matches the
    /// upstream C++ engine's behavior). The SQL parser cannot represent
    /// `i64::MIN` as a positive literal, so we exercise the helper directly.
    #[test]
    fn unary_minus_on_i64_min_wraps_without_panic() {
        let v = CosmosValue::Integer(i64::MIN);
        let r = eval_unary(SqlUnaryOp::Minus, &v);
        // wrapping_neg(i64::MIN) == i64::MIN.
        assert!(matches!(r, CosmosValue::Integer(n) if n == i64::MIN));
    }

    /// bitwise NOT on a fractional number must yield `Undefined`.
    #[test]
    fn bitwise_not_on_fractional_number_returns_undefined() {
        let p = crate::query::parse("SELECT VALUE ~3.7 FROM c").unwrap();
        let doc = serde_json::json!({});
        let result = project(&doc, &p.query, &[]).unwrap();
        // `Undefined` round-trips to JSON `null` via `to_json`.
        assert_eq!(result, serde_json::Value::Null);
    }

    /// SUM over integer-only inputs returns an integer JSON number.
    #[test]
    fn sum_over_integers_returns_integer() {
        let docs = vec![
            serde_json::json!({"v": 1}),
            serde_json::json!({"v": 2}),
            serde_json::json!({"v": 3}),
        ];
        let results = query_documents("SELECT VALUE SUM(c.v) FROM c", &[], &docs).unwrap();
        assert_eq!(results, vec![serde_json::json!(6)]);
    }

    /// SUM with any float operand returns a float JSON number.
    #[test]
    fn sum_with_float_operand_returns_float() {
        let docs = vec![serde_json::json!({"v": 1}), serde_json::json!({"v": 2.5})];
        let results = query_documents("SELECT VALUE SUM(c.v) FROM c", &[], &docs).unwrap();
        assert_eq!(results, vec![serde_json::json!(3.5)]);
    }

    /// a multi-character `ESCAPE` argument makes the LIKE return undefined
    /// (the row does not match) rather than silently using only the first char.
    #[test]
    fn like_with_multi_char_escape_returns_undefined() {
        let p = crate::query::parse("SELECT VALUE c.s LIKE 'a' ESCAPE 'xy' FROM c").unwrap();
        let doc = serde_json::json!({"s": "a"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::Value::Null);
    }

    #[test]
    fn query_documents_full() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 20}),
            serde_json::json!({"name": "Charlie", "age": 25}),
        ];
        let results = query_documents("SELECT c.name FROM c WHERE c.age > 21", &[], &docs).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[1]["name"], "Charlie");
    }

    #[test]
    fn query_with_top() {
        let docs = vec![
            serde_json::json!({"x": 1}),
            serde_json::json!({"x": 2}),
            serde_json::json!({"x": 3}),
        ];
        let results = query_documents("SELECT TOP 2 * FROM c", &[], &docs).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn function_contains() {
        let p = crate::query::parse("SELECT * FROM c WHERE CONTAINS(c.name, 'lic')").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn function_startswith() {
        let p = crate::query::parse("SELECT * FROM c WHERE STARTSWITH(c.name, 'Al')").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn function_is_defined() {
        let p = crate::query::parse("SELECT * FROM c WHERE IS_DEFINED(c.name)").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"age": 30});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn function_array_contains() {
        let p =
            crate::query::parse("SELECT * FROM c WHERE ARRAY_CONTAINS(c.tags, 'rust')").unwrap();
        let doc = serde_json::json!({"tags": ["rust", "azure"]});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn like_pattern() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.name LIKE 'A%e'").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"name": "Bob"});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn between_expression() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.age BETWEEN 18 AND 65").unwrap();
        let doc = serde_json::json!({"age": 30});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"age": 10});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn in_expression() {
        let p =
            crate::query::parse("SELECT * FROM c WHERE c.status IN ('active', 'pending')").unwrap();
        let doc = serde_json::json!({"status": "active"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"status": "closed"});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn parameter_resolution() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.id = @id").unwrap();
        let params = vec![("id".to_string(), serde_json::json!("abc"))];
        let doc = serde_json::json!({"id": "abc"});
        assert!(matches_query(&doc, &p.query, &params).unwrap());
    }

    #[test]
    fn parameter_resolution_accepts_at_prefixed_values() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.id = @id").unwrap();
        let params = vec![("@id".to_string(), serde_json::json!("abc"))];
        let doc = serde_json::json!({"id": "abc"});
        assert!(matches_query(&doc, &p.query, &params).unwrap());
    }

    #[test]
    fn nested_property_access() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.address.city = 'Seattle'").unwrap();
        let doc = serde_json::json!({"address": {"city": "Seattle"}});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn is_null_expression() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.x IS NULL").unwrap();
        let doc = serde_json::json!({"x": null});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn coalesce_expression() {
        let p = crate::query::parse("SELECT VALUE c.nickname ?? c.name FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("Alice"));
    }

    // ── #1: Conditional (?:) is strict-Boolean, NOT JS-truthy ────────────
    //
    // Cosmos SQL `cond ? a : b` yields `Undefined` when `cond` is anything
    // other than `Boolean(true)` or `Boolean(false)`. Earlier the evaluator
    // routed Conditional through the JS-style `internal_js_truthy`, which
    // (a) treated `Number(1)`, non-empty strings, arrays, and objects as
    // truthy and (b) treated `Number(0)` and the empty string as falsy. Both
    // diverge from Cosmos / Gateway semantics; the tests below pin the
    // strict-Boolean contract in place so the regression cannot return.

    /// `WHERE c.x ? 'a' : 'b' = 'a'` must NOT match a row with a non-Boolean
    /// `c.x`, because the ternary returns `Undefined` and the WHERE only
    /// accepts `Boolean(true)`.
    #[test]
    fn conditional_with_non_boolean_condition_does_not_match() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.x ? 'a' : 'b' = 'a'").unwrap();
        for doc in [
            serde_json::json!({"x": 1}),           // non-zero number
            serde_json::json!({"x": 0}),           // zero number
            serde_json::json!({"x": "non-empty"}), // non-empty string
            serde_json::json!({"x": ""}),          // empty string
            serde_json::json!({"x": null}),        // null
            serde_json::json!({}),                 // undefined
            serde_json::json!({"x": [1, 2]}),      // array
            serde_json::json!({"x": {"a": 1}}),    // object
        ] {
            assert!(
                !matches_query(&doc, &p.query, &[]).unwrap(),
                "non-Boolean condition must yield Undefined; doc={doc}"
            );
        }
    }

    /// `WHERE c.x ? true : false` matches exactly when `c.x = true`, never
    /// when `c.x = false` or any non-Boolean. Mirrors the Gateway.
    #[test]
    fn conditional_with_boolean_condition_picks_correct_branch() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.x ? true : false").unwrap();
        assert!(matches_query(&serde_json::json!({"x": true}), &p.query, &[]).unwrap());
        assert!(!matches_query(&serde_json::json!({"x": false}), &p.query, &[]).unwrap());
        // Sanity: non-Boolean still filters out (regression for the bug above).
        assert!(!matches_query(&serde_json::json!({"x": 1}), &p.query, &[]).unwrap());
    }

    /// SELECT projection: a non-Boolean condition projects `Undefined`
    /// (which is omitted from the resulting object), regardless of branch
    /// values. Earlier the JS-truthy path would have projected the truthy
    /// branch for `c.x = 1` and the falsy branch for `c.x = 0`.
    #[test]
    fn conditional_projection_with_non_boolean_condition_is_omitted() {
        let p = crate::query::parse("SELECT (c.x ? 'a' : 'b') AS r FROM c").unwrap();
        for doc in [
            serde_json::json!({"x": 1}),
            serde_json::json!({"x": 0}),
            serde_json::json!({"x": ""}),
            serde_json::json!({"x": "y"}),
            serde_json::json!({"x": null}),
            serde_json::json!({}),
        ] {
            let result = project(&doc, &p.query, &[]).unwrap();
            assert_eq!(
                result,
                serde_json::json!({}),
                "non-Boolean condition must project to omitted (Undefined); doc={doc}"
            );
        }
        // Boolean conditions still pick the correct branch.
        assert_eq!(
            project(&serde_json::json!({"x": true}), &p.query, &[]).unwrap(),
            serde_json::json!({"r": "a"})
        );
        assert_eq!(
            project(&serde_json::json!({"x": false}), &p.query, &[]).unwrap(),
            serde_json::json!({"r": "b"})
        );
    }

    /// Coalesce (`??`) returns the left operand whenever it is *defined*,
    /// even if defined-but-falsy (`false`, `0`, `""`, `null`). This pins the
    /// IS_DEFINED contract \u2014 a future regression that swapped this for
    /// `internal_js_truthy` would change the result for every case below.
    #[test]
    fn coalesce_returns_defined_left_even_when_falsy() {
        let p = crate::query::parse("SELECT VALUE c.x ?? 'fallback' FROM c").unwrap();
        for (doc, expected) in [
            (serde_json::json!({"x": false}), serde_json::json!(false)),
            (serde_json::json!({"x": 0}), serde_json::json!(0)),
            (serde_json::json!({"x": ""}), serde_json::json!("")),
            (serde_json::json!({"x": null}), serde_json::json!(null)),
        ] {
            let result = project(&doc, &p.query, &[]).unwrap();
            assert_eq!(
                result, expected,
                "coalesce must return defined-but-falsy left; doc={doc}"
            );
        }
        // And falls back when left is Undefined.
        assert_eq!(
            project(&serde_json::json!({}), &p.query, &[]).unwrap(),
            serde_json::json!("fallback")
        );
    }

    // ── TOP / OFFSET / LIMIT with parameters ────────────────────────────

    #[test]
    fn top_parameter_resolved() {
        let docs = vec![
            serde_json::json!({"x": 1}),
            serde_json::json!({"x": 2}),
            serde_json::json!({"x": 3}),
        ];
        let params = vec![("n".to_string(), serde_json::json!(2))];
        let results = query_documents("SELECT TOP @n * FROM c", &params, &docs).unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn top_parameter_zero() {
        let docs = vec![serde_json::json!({"x": 1})];
        let params = vec![("n".to_string(), serde_json::json!(0))];
        let results = query_documents("SELECT TOP @n * FROM c", &params, &docs).unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn top_parameter_missing_is_error() {
        let docs = vec![serde_json::json!({"x": 1})];
        let result = query_documents("SELECT TOP @n * FROM c", &[], &docs);
        assert!(result.is_err());
    }

    #[test]
    fn top_parameter_non_numeric_is_error() {
        let docs = vec![serde_json::json!({"x": 1})];
        let params = vec![("n".to_string(), serde_json::json!("not a number"))];
        let result = query_documents("SELECT TOP @n * FROM c", &params, &docs);
        assert!(result.is_err());
    }

    #[test]
    fn offset_limit_parameters_resolved() {
        let docs: Vec<serde_json::Value> = (0..10).map(|i| serde_json::json!({"x": i})).collect();
        let params = vec![
            ("off".to_string(), serde_json::json!(3)),
            ("lim".to_string(), serde_json::json!(2)),
        ];
        let results =
            query_documents("SELECT * FROM c OFFSET @off LIMIT @lim", &params, &docs).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["x"], 3);
        assert_eq!(results[1]["x"], 4);
    }

    #[test]
    fn offset_parameter_missing_is_error() {
        let docs = vec![serde_json::json!({"x": 1})];
        let params = vec![("lim".to_string(), serde_json::json!(10))];
        let result = query_documents("SELECT * FROM c OFFSET @off LIMIT @lim", &params, &docs);
        assert!(result.is_err());
    }

    #[test]
    fn limit_parameter_missing_is_error() {
        let docs = vec![serde_json::json!({"x": 1})];
        let params = vec![("off".to_string(), serde_json::json!(0))];
        let result = query_documents("SELECT * FROM c OFFSET @off LIMIT @lim", &params, &docs);
        assert!(result.is_err());
    }

    #[test]
    fn top_parameter_with_at_prefix() {
        let docs = vec![
            serde_json::json!({"x": 1}),
            serde_json::json!({"x": 2}),
            serde_json::json!({"x": 3}),
        ];
        let params = vec![("@n".to_string(), serde_json::json!(1))];
        let results = query_documents("SELECT TOP @n * FROM c", &params, &docs).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn top_parameter_float_is_error() {
        let docs = vec![
            serde_json::json!({"x": 1}),
            serde_json::json!({"x": 2}),
            serde_json::json!({"x": 3}),
        ];
        let params = vec![("n".to_string(), serde_json::json!(2.7))];
        let result = query_documents("SELECT TOP @n * FROM c", &params, &docs);
        assert!(result.is_err());
    }

    #[test]
    fn top_parameter_negative_is_error() {
        let docs = vec![serde_json::json!({"x": 1})];
        let params = vec![("n".to_string(), serde_json::json!(-1))];
        let result = query_documents("SELECT TOP @n * FROM c", &params, &docs);
        assert!(result.is_err());
    }

    #[test]
    fn top_literal_negative_is_error() {
        let docs = vec![serde_json::json!({"id": "1"})];
        let err = query_documents("SELECT TOP -1 * FROM c", &[], &docs)
            .expect_err("negative TOP literal must error");
        assert!(format!("{err}").to_ascii_uppercase().contains("TOP"));
    }

    #[test]
    fn offset_literal_negative_is_error() {
        let docs = vec![serde_json::json!({"id": "1"})];
        let err = query_documents("SELECT * FROM c OFFSET -1 LIMIT 5", &[], &docs)
            .expect_err("negative OFFSET literal must error");
        assert!(format!("{err}").to_ascii_uppercase().contains("OFFSET"));
    }

    #[test]
    fn limit_literal_negative_is_error() {
        let docs = vec![serde_json::json!({"id": "1"})];
        let err = query_documents("SELECT * FROM c OFFSET 0 LIMIT -1", &[], &docs)
            .expect_err("negative LIMIT literal must error");
        assert!(format!("{err}").to_ascii_uppercase().contains("LIMIT"));
    }

    // ── Bug fix tests: SUBSTRING character indexing ─────────────────────

    #[test]
    fn substring_multibyte_characters() {
        let p = crate::query::parse("SELECT VALUE SUBSTRING(c.name, 0, 2) FROM c").unwrap();
        let doc = serde_json::json!({"name": "日本語"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("日本"));
    }

    #[test]
    fn substring_emoji() {
        let p = crate::query::parse("SELECT VALUE SUBSTRING(c.name, 1, 2) FROM c").unwrap();
        let doc = serde_json::json!({"name": "A😀B😀C"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("😀B"));
    }

    #[test]
    fn substring_past_end() {
        let p = crate::query::parse("SELECT VALUE SUBSTRING(c.name, 10, 5) FROM c").unwrap();
        let doc = serde_json::json!({"name": "short"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(""));
    }

    // ── Bug fix tests: LENGTH character count ───────────────────────────

    #[test]
    fn length_multibyte_characters() {
        let p = crate::query::parse("SELECT VALUE LENGTH(c.name) FROM c").unwrap();
        let doc = serde_json::json!({"name": "日本語"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(3));
    }

    #[test]
    fn length_emoji() {
        let p = crate::query::parse("SELECT VALUE LENGTH(c.name) FROM c").unwrap();
        let doc = serde_json::json!({"name": "A😀B"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(3));
    }

    // ── Bug fix tests: negative array indexer ───────────────────────────

    #[test]
    fn negative_array_index_returns_undefined() {
        let p = crate::query::parse("SELECT VALUE c.items[-1] FROM c").unwrap();
        let doc = serde_json::json!({"items": [10, 20, 30]});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::Value::Null);
    }

    #[test]
    fn fractional_array_index_returns_undefined() {
        let p = crate::query::parse("SELECT VALUE c.items[1.5] FROM c").unwrap();
        let doc = serde_json::json!({"items": [10, 20, 30]});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::Value::Null);
    }

    // ── Bug fix tests: AND/OR three-valued logic ────────────────────────

    #[test]
    fn and_undefined_and_true_is_not_matching() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.missing > 5 AND c.present = true")
            .unwrap();
        let doc = serde_json::json!({"present": true});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn or_undefined_or_true_matches() {
        let p =
            crate::query::parse("SELECT * FROM c WHERE c.missing > 5 OR c.present = true").unwrap();
        let doc = serde_json::json!({"present": true});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn or_both_undefined_does_not_match() {
        let p =
            crate::query::parse("SELECT * FROM c WHERE c.missing1 > 5 OR c.missing2 > 5").unwrap();
        let doc = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn and_both_undefined_does_not_match() {
        let p =
            crate::query::parse("SELECT * FROM c WHERE c.missing1 > 5 AND c.missing2 > 5").unwrap();
        let doc = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    // ── Bug fix tests: LIKE pattern performance ─────────────────────────

    #[test]
    fn like_worst_case_pattern_completes_quickly() {
        let p = crate::query::parse(
            "SELECT * FROM c WHERE c.name LIKE '%a%a%a%a%a%a%a%a%a%a%a%a%a%a%a%'",
        )
        .unwrap();
        let doc = serde_json::json!({"name": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn like_still_matches_correctly() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.name LIKE '%Al%ce%'").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    // ── Bug fix tests: Integer precision ────────────────────────────────

    #[test]
    fn integer_literal_preserved() {
        let p =
            crate::query::parse("SELECT VALUE c.id FROM c WHERE c.id = 9007199254740993").unwrap();
        let doc = serde_json::json!({"id": 9007199254740993_i64});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(9007199254740993_i64));
    }

    #[test]
    fn integer_equality_exact() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.x = 42").unwrap();
        let doc = serde_json::json!({"x": 42});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    // ── ORDER BY tests ──────────────────────────────────────────────────

    #[test]
    fn order_by_asc() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 25}),
            serde_json::json!({"name": "Charlie", "age": 35}),
        ];
        let results = query_documents("SELECT * FROM c ORDER BY c.age ASC", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["age"], 25);
        assert_eq!(results[1]["age"], 30);
        assert_eq!(results[2]["age"], 35);
    }

    #[test]
    fn order_by_desc() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 25}),
            serde_json::json!({"name": "Charlie", "age": 35}),
        ];
        let results = query_documents("SELECT * FROM c ORDER BY c.age DESC", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["age"], 35);
        assert_eq!(results[1]["age"], 30);
        assert_eq!(results[2]["age"], 25);
    }

    #[test]
    fn order_by_default_asc() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 25}),
            serde_json::json!({"name": "Charlie", "age": 35}),
        ];
        let results = query_documents("SELECT * FROM c ORDER BY c.age", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["age"], 25);
        assert_eq!(results[1]["age"], 30);
        assert_eq!(results[2]["age"], 35);
    }

    #[test]
    fn order_by_multiple_keys() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30, "city": "Seattle"}),
            serde_json::json!({"name": "Bob", "age": 25, "city": "Portland"}),
            serde_json::json!({"name": "Charlie", "age": 35, "city": "Seattle"}),
            serde_json::json!({"name": "Diana", "age": 28, "city": "Portland"}),
        ];
        let results = query_documents(
            "SELECT * FROM c ORDER BY c.city ASC, c.age DESC",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 4);
        // Portland group first (ASC), age DESC within
        assert_eq!(results[0]["name"], "Diana"); // Portland, 28
        assert_eq!(results[1]["name"], "Bob"); // Portland, 25
                                               // Seattle group second, age DESC within
        assert_eq!(results[2]["name"], "Charlie"); // Seattle, 35
        assert_eq!(results[3]["name"], "Alice"); // Seattle, 30
    }

    #[test]
    fn order_by_string() {
        let docs = vec![
            serde_json::json!({"name": "Charlie"}),
            serde_json::json!({"name": "Alice"}),
            serde_json::json!({"name": "Bob"}),
        ];
        let results = query_documents("SELECT * FROM c ORDER BY c.name ASC", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[1]["name"], "Bob");
        assert_eq!(results[2]["name"], "Charlie");
    }

    #[test]
    fn order_by_with_where() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30, "city": "Seattle"}),
            serde_json::json!({"name": "Bob", "age": 25, "city": "Portland"}),
            serde_json::json!({"name": "Charlie", "age": 35, "city": "Seattle"}),
            serde_json::json!({"name": "Diana", "age": 28, "city": "Portland"}),
        ];
        let results = query_documents(
            "SELECT * FROM c WHERE c.city = 'Seattle' ORDER BY c.age ASC",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[1]["name"], "Charlie");
    }

    #[test]
    fn order_by_with_top() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 25}),
            serde_json::json!({"name": "Charlie", "age": 35}),
            serde_json::json!({"name": "Diana", "age": 28}),
        ];
        let results =
            query_documents("SELECT TOP 2 * FROM c ORDER BY c.age ASC", &[], &docs).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["age"], 25);
        assert_eq!(results[1]["age"], 28);
    }

    #[test]
    fn order_by_missing_field() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob"}),
            serde_json::json!({"name": "Charlie", "age": 25}),
        ];
        let results = query_documents("SELECT * FROM c ORDER BY c.age ASC", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        // Documents with defined age sort first in ASC
        assert_eq!(results[0]["age"], 25);
        assert_eq!(results[1]["age"], 30);
        // Document missing age sorts last
        assert_eq!(results[2]["name"], "Bob");
    }

    #[test]
    fn order_by_mixed_types() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "val": 10}),
            serde_json::json!({"name": "Bob", "val": "hello"}),
            serde_json::json!({"name": "Charlie", "val": 5}),
        ];
        let results = query_documents("SELECT * FROM c ORDER BY c.val ASC", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        // Numbers sort before strings in Cosmos type ordering
        assert_eq!(results[0]["val"], 5);
        assert_eq!(results[1]["val"], 10);
        assert_eq!(results[2]["val"], "hello");
    }

    #[test]
    fn order_by_nested_path() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "address": {"city": "Seattle"}}),
            serde_json::json!({"name": "Bob", "address": {"city": "Portland"}}),
            serde_json::json!({"name": "Charlie", "address": {"city": "Austin"}}),
        ];
        let results =
            query_documents("SELECT * FROM c ORDER BY c.address.city ASC", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["address"]["city"], "Austin");
        assert_eq!(results[1]["address"]["city"], "Portland");
        assert_eq!(results[2]["address"]["city"], "Seattle");
    }

    // ── GROUP BY + Aggregates tests ─────────────────────────────────────

    #[test]
    fn group_by_count() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, COUNT(1) AS cnt FROM c GROUP BY c.city",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["cnt"], 2);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["cnt"], 2);
    }

    #[test]
    fn group_by_sum() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, SUM(c.revenue) AS total_revenue FROM c GROUP BY c.city",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["total_revenue"], 500.0);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["total_revenue"], 250.0);
    }

    #[test]
    fn group_by_avg() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, AVG(c.score) AS avg_score FROM c GROUP BY c.city",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["avg_score"], 86.5);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["avg_score"], 92.5);
    }

    #[test]
    fn group_by_min_max() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, MIN(c.age) AS min_age, MAX(c.age) AS max_age FROM c GROUP BY c.city",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["min_age"], 25);
        assert_eq!(results[0]["max_age"], 28);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["min_age"], 30);
        assert_eq!(results[1]["max_age"], 35);
    }

    #[test]
    fn group_by_multiple_aggregates() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, COUNT(1) AS cnt, SUM(c.revenue) AS total, AVG(c.score) AS avg_score FROM c GROUP BY c.city",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["cnt"], 2);
        assert_eq!(results[0]["total"], 500.0);
        assert_eq!(results[0]["avg_score"], 86.5);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["cnt"], 2);
        assert_eq!(results[1]["total"], 250.0);
        assert_eq!(results[1]["avg_score"], 92.5);
    }

    #[test]
    fn group_by_multiple_keys() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, c.state, COUNT(1) AS cnt FROM c GROUP BY c.city, c.state",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["state"], "OR");
        assert_eq!(results[0]["cnt"], 2);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["state"], "WA");
        assert_eq!(results[1]["cnt"], 2);
    }

    #[test]
    fn group_by_with_where() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "city": "Seattle", "state": "WA", "age": 30, "score": 90, "revenue": 100}),
            serde_json::json!({"name": "Bob", "city": "Portland", "state": "OR", "age": 25, "score": 85, "revenue": 200}),
            serde_json::json!({"name": "Charlie", "city": "Seattle", "state": "WA", "age": 35, "score": 95, "revenue": 150}),
            serde_json::json!({"name": "Diana", "city": "Portland", "state": "OR", "age": 28, "score": 88, "revenue": 300}),
        ];
        let mut results = query_documents(
            "SELECT c.city, COUNT(1) AS cnt FROM c WHERE c.age >= 28 GROUP BY c.city",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        results.sort_by(|a, b| a["city"].as_str().cmp(&b["city"].as_str()));
        assert_eq!(results[0]["city"], "Portland");
        assert_eq!(results[0]["cnt"], 1);
        assert_eq!(results[1]["city"], "Seattle");
        assert_eq!(results[1]["cnt"], 2);
    }

    #[test]
    fn group_by_order_by_count_expression() {
        let docs = vec![
            serde_json::json!({"city": "Seattle"}),
            serde_json::json!({"city": "Seattle"}),
            serde_json::json!({"city": "Seattle"}),
            serde_json::json!({"city": "Portland"}),
        ];
        let results = query_documents(
            "SELECT c.city, COUNT(1) AS cnt FROM c GROUP BY c.city ORDER BY COUNT(1) ASC",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(
            results[0],
            serde_json::json!({"city": "Portland", "cnt": 1})
        );
        assert_eq!(results[1], serde_json::json!({"city": "Seattle", "cnt": 3}));
    }

    #[test]
    fn group_by_order_by_aggregate_alias() {
        let docs = vec![
            serde_json::json!({"city": "Seattle"}),
            serde_json::json!({"city": "Seattle"}),
            serde_json::json!({"city": "Portland"}),
        ];
        let results = query_documents(
            "SELECT c.city, COUNT(1) AS cnt FROM c GROUP BY c.city ORDER BY cnt ASC",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(
            results[0],
            serde_json::json!({"city": "Portland", "cnt": 1})
        );
        assert_eq!(results[1], serde_json::json!({"city": "Seattle", "cnt": 2}));
    }

    #[test]
    fn aggregate_without_group_by() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 25}),
            serde_json::json!({"name": "Charlie", "age": 35}),
        ];
        let results = query_documents("SELECT COUNT(1) AS cnt FROM c", &[], &docs).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["cnt"], 3);
    }

    #[test]
    fn aggregate_sum_without_group_by() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "age": 30}),
            serde_json::json!({"name": "Bob", "age": 25}),
            serde_json::json!({"name": "Charlie", "age": 35}),
        ];
        let results = query_documents("SELECT SUM(c.age) AS total_age FROM c", &[], &docs).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["total_age"], 90.0);
    }

    #[test]
    fn aggregate_avg_empty() {
        let docs: Vec<serde_json::Value> = vec![];
        let results = query_documents("SELECT AVG(c.age) AS avg_age FROM c", &[], &docs).unwrap();
        assert_eq!(results.len(), 1);
        // AVG on empty set produces undefined (null in JSON)
        assert_eq!(results[0]["avg_age"], serde_json::Value::Null);
    }

    #[test]
    fn array_iterator_without_join_expands_rows() {
        let docs = vec![
            serde_json::json!({"tags": ["a", "b"]}),
            serde_json::json!({"tags": ["c"]}),
        ];
        let results = query_documents("SELECT VALUE t FROM t IN c.tags", &[], &docs).unwrap();
        assert_eq!(
            results,
            vec![
                serde_json::json!("a"),
                serde_json::json!("b"),
                serde_json::json!("c"),
            ]
        );
    }

    #[test]
    fn aliased_path_without_join_uses_collection_value() {
        let docs = vec![serde_json::json!({
            "address": {"city": "Seattle", "zip": 98052}
        })];
        let results = query_documents("SELECT * FROM c.address a", &[], &docs).unwrap();
        assert_eq!(
            results,
            vec![serde_json::json!({"city": "Seattle", "zip": 98052})]
        );
    }

    // ── JOIN tests ──────────────────────────────────────────────────────

    #[test]
    fn join_simple() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "tags": ["rust", "azure"]}),
            serde_json::json!({"name": "Bob", "tags": ["python"]}),
        ];
        let results = query_documents("SELECT * FROM c JOIN t IN c.tags", &[], &docs).unwrap();
        // Alice expands to 2 rows, Bob to 1 row
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn join_with_where() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "tags": ["rust", "azure"]}),
            serde_json::json!({"name": "Bob", "tags": ["python"]}),
            serde_json::json!({"name": "Charlie", "tags": ["rust", "python", "go"]}),
        ];
        let results = query_documents(
            "SELECT * FROM c JOIN t IN c.tags WHERE t = 'rust'",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn join_select_both() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "tags": ["rust", "azure"]}),
            serde_json::json!({"name": "Bob", "tags": ["python"]}),
        ];
        let results =
            query_documents("SELECT c.name, t FROM c JOIN t IN c.tags", &[], &docs).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[0]["t"], "rust");
        assert_eq!(results[1]["name"], "Alice");
        assert_eq!(results[1]["t"], "azure");
        assert_eq!(results[2]["name"], "Bob");
        assert_eq!(results[2]["t"], "python");
    }

    #[test]
    fn join_empty_array() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "tags": ["rust"]}),
            serde_json::json!({"name": "Diana", "tags": []}),
        ];
        let results =
            query_documents("SELECT c.name, t FROM c JOIN t IN c.tags", &[], &docs).unwrap();
        // Diana's empty array produces no rows
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[0]["t"], "rust");
    }

    #[test]
    fn join_missing_array() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "tags": ["rust"]}),
            serde_json::json!({"name": "Eve"}),
        ];
        let results =
            query_documents("SELECT c.name, t FROM c JOIN t IN c.tags", &[], &docs).unwrap();
        // Eve has no tags property — produces no rows
        assert_eq!(results.len(), 1);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[0]["t"], "rust");
    }

    #[test]
    fn join_multiple() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "tags": ["rust", "azure"], "skills": ["coding", "design"]}),
            serde_json::json!({"name": "Bob", "tags": ["python"], "skills": ["data"]}),
        ];
        let results = query_documents(
            "SELECT c.name, t, s FROM c JOIN t IN c.tags JOIN s IN c.skills",
            &[],
            &docs,
        )
        .unwrap();
        // Alice: 2 tags * 2 skills = 4 rows; Bob: 1 tag * 1 skill = 1 row
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn nested_join_uses_join_alias_bindings() {
        let docs = vec![serde_json::json!({
            "name": "Alice",
            "children": [
                {"name": "Amy", "grades": [95, 97]},
                {"name": "Ben", "grades": [88]}
            ]
        })];
        let results = query_documents(
            "SELECT p.name, c.name AS child, g FROM p JOIN c IN p.children JOIN g IN c.grades",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            serde_json::json!({"name": "Alice", "child": "Amy", "g": 95})
        );
        assert_eq!(
            results[1],
            serde_json::json!({"name": "Alice", "child": "Amy", "g": 97})
        );
        assert_eq!(
            results[2],
            serde_json::json!({"name": "Alice", "child": "Ben", "g": 88})
        );
    }

    #[test]
    fn join_with_filter_on_parent() {
        let docs = vec![
            serde_json::json!({"name": "Alice", "active": true, "tags": ["rust", "azure"]}),
            serde_json::json!({"name": "Bob", "active": false, "tags": ["rust", "python"]}),
            serde_json::json!({"name": "Charlie", "active": true, "tags": ["go", "rust"]}),
        ];
        let results = query_documents(
            "SELECT c.name, t FROM c JOIN t IN c.tags WHERE c.active = true AND t = 'rust'",
            &[],
            &docs,
        )
        .unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "Alice");
        assert_eq!(results[0]["t"], "rust");
        assert_eq!(results[1]["name"], "Charlie");
        assert_eq!(results[1]["t"], "rust");
    }

    // ── #10: AND/OR strict-Boolean three-valued logic ────────────────────

    /// In Cosmos SQL, `AND` / `OR` only accept `Boolean` operands. Any
    /// non-Boolean value (number, string, array, object) is coerced to
    /// `Undefined`, which means `WHERE c.x AND TRUE` does **not** match a
    /// document where `c.x = 1`. The earlier implementation used JS-style
    /// truthiness and would have wrongly matched.
    #[test]
    fn where_number_and_true_does_not_match() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.x AND true").unwrap();
        let doc = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn where_string_or_false_does_not_match() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.s OR false").unwrap();
        let doc = serde_json::json!({"s": "non-empty"});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn and_false_short_circuits_over_undefined() {
        // `false AND <undefined>` is still `false` (absorbing element).
        let p = crate::query::parse("SELECT * FROM c WHERE false AND c.missing").unwrap();
        assert!(!matches_query(&serde_json::json!({}), &p.query, &[]).unwrap());
    }

    #[test]
    fn or_true_short_circuits_over_undefined() {
        // `true OR <undefined>` is still `true` (absorbing element).
        let p = crate::query::parse("SELECT * FROM c WHERE true OR c.missing").unwrap();
        assert!(matches_query(&serde_json::json!({}), &p.query, &[]).unwrap());
    }

    #[test]
    fn and_two_booleans_evaluates_normally() {
        let p = crate::query::parse("SELECT * FROM c WHERE c.a AND c.b").unwrap();
        assert!(matches_query(&serde_json::json!({"a": true, "b": true}), &p.query, &[]).unwrap());
        assert!(
            !matches_query(&serde_json::json!({"a": true, "b": false}), &p.query, &[]).unwrap()
        );
    }

    // (#3) Regression: `c.x / 0` and `c.x % 0` previously produced
    // `CosmosValue::Number(NaN)`, which then silently coerced to `Value::Null`
    // inside object/array projections. Both must now produce `Undefined` so
    // they are elided from projections (and so the PK-value finiteness
    // invariant in `value::to_json` cannot be violated by user expressions).
    #[test]
    fn divide_by_zero_is_undefined_in_projection() {
        let docs = vec![serde_json::json!({"x": 1})];
        let results = query_documents("SELECT VALUE { v: c.x / 0 } FROM c", &[], &docs).unwrap();
        // The `v` property holds `Undefined` and must be elided from the
        // projected object - NOT serialized as `null`.
        assert_eq!(results, vec![serde_json::json!({})]);
    }

    #[test]
    fn modulo_by_zero_is_undefined_in_projection() {
        let docs = vec![serde_json::json!({"x": 7})];
        let results = query_documents("SELECT VALUE { v: c.x % 0 } FROM c", &[], &docs).unwrap();
        assert_eq!(results, vec![serde_json::json!({})]);
    }

    #[test]
    fn divide_by_zero_undefined_filters_where_clause() {
        // `WHERE (c.x / 0) > 0` must NOT match - `Undefined > 0` is
        // `Undefined`, which fails the strict-Boolean check in the WHERE
        // pass.
        let p = crate::query::parse("SELECT * FROM c WHERE (c.x / 0) > 0").unwrap();
        let doc = serde_json::json!({"x": 5});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }
}
