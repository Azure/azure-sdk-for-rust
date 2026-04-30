// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore STARTSWITH ENDSWITH LTRIM RTRIM TOSTRING multibyte

//! In-memory query evaluation: match documents against WHERE clauses and apply projections.
//!
//! This evaluator interprets the SQL AST directly against `serde_json::Value` documents.
//! It supports the most commonly used scalar expressions, comparisons, and built-in functions.

use std::{cmp::Ordering, collections::HashMap};

use crate::ast::*;
use crate::common::get_root_alias;
use crate::value::CosmosValue;

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

type Params = [(String, serde_json::Value)];

/// Check if a JSON document matches a query's WHERE clause.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_query::{parse, eval};
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
            for item in items {
                let val = eval_scalar(
                    &item.expression,
                    document,
                    root_alias.as_deref(),
                    parameters,
                )?;
                let key = if let Some(alias) = &item.alias {
                    alias.clone()
                } else {
                    infer_property_name(&item.expression)
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

/// Check whether a FROM clause contains JOINs.
fn has_join(collection: &SqlCollectionExpression) -> bool {
    matches!(collection, SqlCollectionExpression::Join { .. })
}

/// Resolve a `SqlCollection::Path` against a set of variable bindings.
fn resolve_collection_path(
    collection: &SqlCollection,
    bindings: &serde_json::Map<String, serde_json::Value>,
) -> serde_json::Value {
    match collection {
        SqlCollection::Path { root, path } => {
            let mut val = bindings
                .get(root)
                .cloned()
                .unwrap_or(serde_json::Value::Null);
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
            val
        }
        SqlCollection::Subquery(_) => serde_json::Value::Null,
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
            let mut map = serde_json::Map::new();
            map.insert(alias_name, doc.clone());
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
            let arr = resolve_collection_path(collection, bindings);
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
            let mut sum = 0.0f64;
            let mut has_value = false;
            for doc in group {
                match eval_scalar(arg, doc, root_alias, params)? {
                    CosmosValue::Number(n) => {
                        sum += n;
                        has_value = true;
                    }
                    CosmosValue::Integer(n) => {
                        sum += n as f64;
                        has_value = true;
                    }
                    _ => {}
                }
            }
            if has_value {
                Ok(CosmosValue::Number(sum))
            } else {
                Ok(CosmosValue::Undefined)
            }
        }
        "AVG" => {
            let arg = args
                .first()
                .ok_or_else(|| EvalError::TypeError("AVG requires an argument".into()))?;
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
            let mut min_val: Option<CosmosValue> = None;
            for doc in group {
                let val = eval_scalar(arg, doc, root_alias, params)?;
                if val.is_undefined() {
                    continue;
                }
                min_val = Some(match min_val {
                    None => val,
                    Some(current) => match val.cosmos_cmp(&current) {
                        Some(Ordering::Less) => val,
                        _ => current,
                    },
                });
            }
            Ok(min_val.unwrap_or(CosmosValue::Undefined))
        }
        "MAX" => {
            let arg = args
                .first()
                .ok_or_else(|| EvalError::TypeError("MAX requires an argument".into()))?;
            let mut max_val: Option<CosmosValue> = None;
            for doc in group {
                let val = eval_scalar(arg, doc, root_alias, params)?;
                if val.is_undefined() {
                    continue;
                }
                max_val = Some(match max_val {
                    None => val,
                    Some(current) => match val.cosmos_cmp(&current) {
                        Some(Ordering::Greater) => val,
                        _ => current,
                    },
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
            if cond.is_truthy() {
                eval_scalar_with_group(if_true, representative, root_alias, params, group)
            } else {
                eval_scalar_with_group(if_false, representative, root_alias, params, group)
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
        SqlSelectSpec::Star => Ok(doc.clone()),
        SqlSelectSpec::Value(expr) => {
            let val = eval_scalar(expr, doc, root_alias, params)?;
            Ok(val.to_json())
        }
        SqlSelectSpec::List(items) => {
            let mut obj = serde_json::Map::new();
            for item in items {
                let val = eval_scalar(&item.expression, doc, root_alias, params)?;
                let key = item
                    .alias
                    .clone()
                    .unwrap_or_else(|| infer_property_name(&item.expression));
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
        SqlSelectSpec::Star => Ok(representative.clone()),
        SqlSelectSpec::Value(expr) => {
            let val = eval_scalar_with_group(expr, representative, root_alias, params, group)?;
            Ok(val.to_json())
        }
        SqlSelectSpec::List(items) => {
            let mut obj = serde_json::Map::new();
            for item in items {
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
                    .unwrap_or_else(|| infer_property_name(&item.expression));
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
/// ```
/// use azure_data_cosmos_query::eval;
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
    let program = crate::parse(sql)
        .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e))?;
    let query = &program.query;
    let root_alias = get_root_alias(query);

    // Determine whether the FROM clause contains JOINs.
    let use_join = query.from.as_ref().is_some_and(|f| has_join(&f.collection));

    // With JOINs, PropertyRef looks up bindings by name in the context object,
    // so root_alias must be None. Without JOINs, use the FROM alias as before.
    let eval_alias = if use_join {
        None
    } else {
        root_alias.as_deref()
    };

    // ── Step 1: expand JOINs + apply WHERE filter ────────────────────────
    let mut filtered_rows: Vec<serde_json::Value> = Vec::new();

    for doc in documents {
        if use_join {
            let from = &query.from.as_ref().unwrap().collection;
            let bindings_list = expand_from(doc, from, &serde_json::Map::new())
                .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;
            for bindings in bindings_list {
                let ctx = serde_json::Value::Object(bindings);
                if eval_where(&ctx, &query.where_clause, None, parameters)
                    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?
                {
                    filtered_rows.push(ctx);
                }
            }
        } else if eval_where(doc, &query.where_clause, eval_alias, parameters)
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?
        {
            filtered_rows.push(doc.clone());
        }
    }

    // ── Step 2: GROUP BY / aggregates, or plain projection ───────────────
    let use_aggregates = query.group_by.is_some() || select_has_aggregates(query);

    let (mut results, originals): (Vec<serde_json::Value>, Vec<serde_json::Value>) =
        if use_aggregates {
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
                    let key = serde_json::to_string(&key_parts.map_err(|e| {
                        azure_core::Error::new(azure_core::error::ErrorKind::Other, e)
                    })?)
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
                    projected.push(project_group(group, query, eval_alias, parameters).map_err(
                        |e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e),
                    )?);
                    reps.push(group[0].clone());
                }
                (projected, reps)
            } else {
                // Aggregates without GROUP BY → implicit single group over all rows.
                let projected = project_group(&filtered_rows, query, eval_alias, parameters)
                    .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;
                let rep = filtered_rows
                    .first()
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);
                (vec![projected], vec![rep])
            }
        } else {
            // No aggregates — project each row individually.
            let mut projected = Vec::new();
            let originals = filtered_rows.clone();
            for row in &filtered_rows {
                projected.push(
                    project_row(row, query, eval_alias, parameters).map_err(|e| {
                        azure_core::Error::new(azure_core::error::ErrorKind::Other, e)
                    })?,
                );
            }
            (projected, originals)
        };

    // ── Step 3: ORDER BY ─────────────────────────────────────────────────
    if let Some(order_by) = &query.order_by {
        let mut indices: Vec<usize> = (0..results.len()).collect();
        indices.sort_by(|&a, &b| {
            compare_for_order_by(
                &originals[a],
                &originals[b],
                order_by,
                eval_alias,
                parameters,
            )
        });
        results = indices.iter().map(|&i| results[i].clone()).collect();
    }

    // ── Step 4: TOP ──────────────────────────────────────────────────────
    if let Some(top) = &query.select.top {
        let n = match top {
            SqlTopSpec::Literal(n) => *n as usize,
            SqlTopSpec::Parameter(name) => resolve_integer_param(parameters, name)
                .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?
                as usize,
        };
        results.truncate(n);
    }

    // ── Step 5: OFFSET / LIMIT ───────────────────────────────────────────
    if let Some(ol) = &query.offset_limit {
        let offset = match &ol.offset {
            SqlOffsetSpec::Literal(n) => *n as usize,
            SqlOffsetSpec::Parameter(name) => resolve_integer_param(parameters, name)
                .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?
                as usize,
        };
        let limit = match &ol.limit {
            SqlLimitSpec::Literal(n) => *n as usize,
            SqlLimitSpec::Parameter(name) => resolve_integer_param(parameters, name)
                .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?
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

/// Resolve a parameter to an integer value for TOP/OFFSET/LIMIT.
fn resolve_integer_param(parameters: &Params, name: &str) -> Result<i64, EvalError> {
    for (param_name, param_value) in parameters {
        let clean = param_name.strip_prefix('@').unwrap_or(param_name);
        if clean == name || param_name == name {
            return match param_value {
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Ok(i)
                    } else if let Some(f) = n.as_f64() {
                        Ok(f as i64)
                    } else {
                        Err(EvalError::TypeError(format!(
                            "parameter @{name} is not a valid integer"
                        )))
                    }
                }
                _ => Err(EvalError::TypeError(format!(
                    "parameter @{name} must be a number, got {}",
                    param_value
                ))),
            };
        }
    }
    Err(EvalError::ParameterNotFound(name.to_string()))
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
            if cond.is_truthy() {
                eval_scalar(if_true, doc, root_alias, params)
            } else {
                eval_scalar(if_false, doc, root_alias, params)
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
            for (param_name, param_value) in params {
                if param_name == name {
                    return Ok(CosmosValue::from_json(param_value));
                }
            }
            Err(EvalError::ParameterNotFound(name.clone()))
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
        SqlBinaryOp::Add => numeric_op(left, right, |a, b| a + b),
        SqlBinaryOp::Subtract => numeric_op(left, right, |a, b| a - b),
        SqlBinaryOp::Multiply => numeric_op(left, right, |a, b| a * b),
        SqlBinaryOp::Divide => {
            numeric_op(left, right, |a, b| if b == 0.0 { f64::NAN } else { a / b })
        }
        SqlBinaryOp::Modulo => {
            numeric_op(left, right, |a, b| if b == 0.0 { f64::NAN } else { a % b })
        }
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

/// Three-valued AND: `undefined AND false` = `false`, `undefined AND true` = `undefined`.
fn eval_and(left: &CosmosValue, right: &CosmosValue) -> CosmosValue {
    match (left.is_undefined(), right.is_undefined()) {
        (true, true) => CosmosValue::Undefined,
        (true, false) => {
            if !right.is_truthy() {
                CosmosValue::Boolean(false)
            } else {
                CosmosValue::Undefined
            }
        }
        (false, true) => {
            if !left.is_truthy() {
                CosmosValue::Boolean(false)
            } else {
                CosmosValue::Undefined
            }
        }
        (false, false) => CosmosValue::Boolean(left.is_truthy() && right.is_truthy()),
    }
}

/// Three-valued OR: `undefined OR true` = `true`, `undefined OR false` = `undefined`.
fn eval_or(left: &CosmosValue, right: &CosmosValue) -> CosmosValue {
    match (left.is_undefined(), right.is_undefined()) {
        (true, true) => CosmosValue::Undefined,
        (true, false) => {
            if right.is_truthy() {
                CosmosValue::Boolean(true)
            } else {
                CosmosValue::Undefined
            }
        }
        (false, true) => {
            if left.is_truthy() {
                CosmosValue::Boolean(true)
            } else {
                CosmosValue::Undefined
            }
        }
        (false, false) => CosmosValue::Boolean(left.is_truthy() || right.is_truthy()),
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
            CosmosValue::Integer(n) => CosmosValue::Integer(-n),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::Plus => match val {
            CosmosValue::Number(n) => CosmosValue::Number(*n),
            CosmosValue::Integer(n) => CosmosValue::Integer(*n),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::BitwiseNot => match val {
            CosmosValue::Number(n) => CosmosValue::Integer(!(*n as i64)),
            CosmosValue::Integer(n) => CosmosValue::Integer(!n),
            _ => CosmosValue::Undefined,
        },
    }
}

fn numeric_op(left: &CosmosValue, right: &CosmosValue, f: fn(f64, f64) -> f64) -> CosmosValue {
    match (left, right) {
        (CosmosValue::Number(a), CosmosValue::Number(b)) => CosmosValue::Number(f(*a, *b)),
        (CosmosValue::Integer(a), CosmosValue::Integer(b)) => {
            CosmosValue::Number(f(*a as f64, *b as f64))
        }
        (CosmosValue::Number(a), CosmosValue::Integer(b)) => CosmosValue::Number(f(*a, *b as f64)),
        (CosmosValue::Integer(a), CosmosValue::Number(b)) => CosmosValue::Number(f(*a as f64, *b)),
        _ => CosmosValue::Undefined,
    }
}

fn int_op(left: &CosmosValue, right: &CosmosValue, f: fn(i64, i64) -> i64) -> CosmosValue {
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

fn eval_function(name: &str, args: &[CosmosValue]) -> Result<CosmosValue, EvalError> {
    let upper = name.to_ascii_uppercase();
    match upper.as_str() {
        // Type checking
        "IS_DEFINED" => Ok(CosmosValue::Boolean(
            args.first().is_some_and(|v| !v.is_undefined()),
        )),
        "IS_NULL" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Null)
        ))),
        "IS_BOOL" | "IS_BOOLEAN" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Boolean(_))
        ))),
        "IS_NUMBER" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Number(_) | CosmosValue::Integer(_))
        ))),
        "IS_STRING" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::String(_))
        ))),
        "IS_ARRAY" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Array(_))
        ))),
        "IS_OBJECT" => Ok(CosmosValue::Boolean(matches!(
            args.first(),
            Some(CosmosValue::Object(_))
        ))),

        // String functions
        "CONTAINS" => match args {
            [CosmosValue::String(s), CosmosValue::String(sub), ..] => {
                let case_insensitive = matches!(args.get(2), Some(CosmosValue::Boolean(true)));
                if case_insensitive {
                    Ok(CosmosValue::Boolean(
                        s.to_lowercase().contains(&sub.to_lowercase()),
                    ))
                } else {
                    Ok(CosmosValue::Boolean(s.contains(sub.as_str())))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "STARTSWITH" => match args {
            [CosmosValue::String(s), CosmosValue::String(prefix), ..] => {
                let case_insensitive = matches!(args.get(2), Some(CosmosValue::Boolean(true)));
                if case_insensitive {
                    Ok(CosmosValue::Boolean(
                        s.to_lowercase().starts_with(&prefix.to_lowercase()),
                    ))
                } else {
                    Ok(CosmosValue::Boolean(s.starts_with(prefix.as_str())))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "ENDSWITH" => match args {
            [CosmosValue::String(s), CosmosValue::String(suffix), ..] => {
                let case_insensitive = matches!(args.get(2), Some(CosmosValue::Boolean(true)));
                if case_insensitive {
                    Ok(CosmosValue::Boolean(
                        s.to_lowercase().ends_with(&suffix.to_lowercase()),
                    ))
                } else {
                    Ok(CosmosValue::Boolean(s.ends_with(suffix.as_str())))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "UPPER" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.to_uppercase())),
            _ => Ok(CosmosValue::Undefined),
        },
        "LOWER" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.to_lowercase())),
            _ => Ok(CosmosValue::Undefined),
        },
        "LENGTH" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::Integer(s.chars().count() as i64)),
            _ => Ok(CosmosValue::Undefined),
        },
        "LTRIM" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.trim_start().to_string())),
            _ => Ok(CosmosValue::Undefined),
        },
        "RTRIM" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.trim_end().to_string())),
            _ => Ok(CosmosValue::Undefined),
        },
        "TRIM" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.trim().to_string())),
            _ => Ok(CosmosValue::Undefined),
        },
        "CONCAT" => {
            let mut result = String::new();
            for arg in args {
                match arg {
                    CosmosValue::String(s) => result.push_str(s),
                    CosmosValue::Undefined => return Ok(CosmosValue::Undefined),
                    _ => return Ok(CosmosValue::Undefined),
                }
            }
            Ok(CosmosValue::String(result))
        }
        "SUBSTRING" => {
            let s = match args.first() {
                Some(CosmosValue::String(s)) => s,
                _ => return Ok(CosmosValue::Undefined),
            };
            let start = match args.get(1) {
                Some(CosmosValue::Number(n)) => *n as usize,
                Some(CosmosValue::Integer(n)) => *n as usize,
                _ => return Ok(CosmosValue::Undefined),
            };
            let len = match args.get(2) {
                Some(CosmosValue::Number(n)) => *n as usize,
                Some(CosmosValue::Integer(n)) => *n as usize,
                _ => return Ok(CosmosValue::Undefined),
            };
            Ok(CosmosValue::String(
                s.chars().skip(start).take(len).collect(),
            ))
        }
        "REPLACE" => match args {
            [CosmosValue::String(s), CosmosValue::String(old), CosmosValue::String(new)] => {
                Ok(CosmosValue::String(s.replace(old.as_str(), new.as_str())))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "LEFT" => match args {
            [CosmosValue::String(s), CosmosValue::Number(n)] => {
                let n = *n as usize;
                Ok(CosmosValue::String(s.chars().take(n).collect()))
            }
            [CosmosValue::String(s), CosmosValue::Integer(n)] => {
                let n = *n as usize;
                Ok(CosmosValue::String(s.chars().take(n).collect()))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "RIGHT" => match args {
            [CosmosValue::String(s), CosmosValue::Number(n)] => {
                let n = *n as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Ok(CosmosValue::String(chars[start..].iter().collect()))
            }
            [CosmosValue::String(s), CosmosValue::Integer(n)] => {
                let n = *n as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Ok(CosmosValue::String(chars[start..].iter().collect()))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "TOSTRING" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.clone())),
            Some(CosmosValue::Integer(n)) => Ok(CosmosValue::String(format!("{n}"))),
            Some(CosmosValue::Number(n)) => Ok(CosmosValue::String(format!("{n}"))),
            Some(CosmosValue::Boolean(b)) => Ok(CosmosValue::String(
                if *b { "true" } else { "false" }.into(),
            )),
            Some(CosmosValue::Null) => Ok(CosmosValue::String("null".into())),
            _ => Ok(CosmosValue::Undefined),
        },

        // Math functions
        "ABS" => num_fn1(args, |n| n.abs()),
        "CEILING" => num_fn1(args, |n| n.ceil()),
        "FLOOR" => num_fn1(args, |n| n.floor()),
        "ROUND" => num_fn1(args, |n| n.round()),
        "POWER" => num_fn2(args, |a, b| a.powf(b)),
        "SQRT" => num_fn1(args, |n| n.sqrt()),
        "LOG" => num_fn1(args, |n| n.ln()),
        "LOG10" => num_fn1(args, |n| n.log10()),
        "EXP" => num_fn1(args, |n| n.exp()),
        "SIGN" => num_fn1(args, |n| {
            if n > 0.0 {
                1.0
            } else if n < 0.0 {
                -1.0
            } else {
                0.0
            }
        }),

        // Array functions
        "ARRAY_CONTAINS" => match args {
            [CosmosValue::Array(arr), search, ..] => {
                let found = arr
                    .iter()
                    .any(|item| matches!(item.cosmos_eq(search), CosmosValue::Boolean(true)));
                Ok(CosmosValue::Boolean(found))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "ARRAY_LENGTH" => match args.first() {
            Some(CosmosValue::Array(arr)) => Ok(CosmosValue::Integer(arr.len() as i64)),
            _ => Ok(CosmosValue::Undefined),
        },
        "ARRAY_SLICE" => match args {
            [CosmosValue::Array(arr), CosmosValue::Number(start), ..] => {
                let start = *start as i64;
                let start = if start < 0 {
                    (arr.len() as i64 + start).max(0) as usize
                } else {
                    start as usize
                };
                let len = match args.get(2) {
                    Some(CosmosValue::Number(n)) => Some(*n as usize),
                    _ => None,
                };
                let end = match len {
                    Some(l) => (start + l).min(arr.len()),
                    None => arr.len(),
                };
                if start >= arr.len() {
                    Ok(CosmosValue::Array(Vec::new()))
                } else {
                    Ok(CosmosValue::Array(arr[start..end].to_vec()))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },

        // Aggregate placeholders (return undefined — they need special handling)
        "COUNT" | "SUM" | "AVG" | "MIN" | "MAX" => Err(EvalError::Unsupported(format!(
            "aggregate function {upper}"
        ))),

        _ => Err(EvalError::UnknownFunction(name.to_string())),
    }
}

fn num_fn1(args: &[CosmosValue], f: fn(f64) -> f64) -> Result<CosmosValue, EvalError> {
    match args.first() {
        Some(CosmosValue::Number(n)) => Ok(CosmosValue::Number(f(*n))),
        _ => Ok(CosmosValue::Undefined),
    }
}

fn num_fn2(args: &[CosmosValue], f: fn(f64, f64) -> f64) -> Result<CosmosValue, EvalError> {
    match args {
        [CosmosValue::Number(a), CosmosValue::Number(b)] => Ok(CosmosValue::Number(f(*a, *b))),
        _ => Ok(CosmosValue::Undefined),
    }
}

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
fn infer_property_name(expr: &SqlScalarExpression) -> String {
    match expr {
        SqlScalarExpression::PropertyRef(name) => name.clone(),
        SqlScalarExpression::MemberRef { member, .. } => member.clone(),
        SqlScalarExpression::FunctionCall { name, .. } => name.clone(),
        _ => "$1".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_simple_where() {
        let p = crate::parse("SELECT * FROM c WHERE c.age > 21").unwrap();
        let doc = serde_json::json!({"age": 30});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"age": 18});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn match_equality() {
        let p = crate::parse("SELECT * FROM c WHERE c.name = 'Alice'").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"name": "Bob"});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn match_and_or() {
        let p = crate::parse("SELECT * FROM c WHERE c.age > 18 AND c.name = 'Alice'").unwrap();
        let doc = serde_json::json!({"name": "Alice", "age": 30});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"name": "Alice", "age": 16});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn match_no_where() {
        let p = crate::parse("SELECT * FROM c").unwrap();
        let doc = serde_json::json!({"anything": true});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn project_star() {
        let p = crate::parse("SELECT * FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, doc);
    }

    #[test]
    fn project_fields() {
        let p = crate::parse("SELECT c.name, c.age FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice", "age": 30, "extra": true});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!({"name": "Alice", "age": 30}));
    }

    #[test]
    fn project_value() {
        let p = crate::parse("SELECT VALUE c.name FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("Alice"));
    }

    #[test]
    fn project_with_alias() {
        let p = crate::parse("SELECT c.name AS n FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!({"n": "Alice"}));
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
        let p = crate::parse("SELECT * FROM c WHERE CONTAINS(c.name, 'lic')").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn function_startswith() {
        let p = crate::parse("SELECT * FROM c WHERE STARTSWITH(c.name, 'Al')").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn function_is_defined() {
        let p = crate::parse("SELECT * FROM c WHERE IS_DEFINED(c.name)").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"age": 30});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn function_array_contains() {
        let p = crate::parse("SELECT * FROM c WHERE ARRAY_CONTAINS(c.tags, 'rust')").unwrap();
        let doc = serde_json::json!({"tags": ["rust", "azure"]});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn like_pattern() {
        let p = crate::parse("SELECT * FROM c WHERE c.name LIKE 'A%e'").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"name": "Bob"});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn between_expression() {
        let p = crate::parse("SELECT * FROM c WHERE c.age BETWEEN 18 AND 65").unwrap();
        let doc = serde_json::json!({"age": 30});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"age": 10});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn in_expression() {
        let p = crate::parse("SELECT * FROM c WHERE c.status IN ('active', 'pending')").unwrap();
        let doc = serde_json::json!({"status": "active"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"status": "closed"});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn parameter_resolution() {
        let p = crate::parse("SELECT * FROM c WHERE c.id = @id").unwrap();
        let params = vec![("id".to_string(), serde_json::json!("abc"))];
        let doc = serde_json::json!({"id": "abc"});
        assert!(matches_query(&doc, &p.query, &params).unwrap());
    }

    #[test]
    fn nested_property_access() {
        let p = crate::parse("SELECT * FROM c WHERE c.address.city = 'Seattle'").unwrap();
        let doc = serde_json::json!({"address": {"city": "Seattle"}});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn is_null_expression() {
        let p = crate::parse("SELECT * FROM c WHERE c.x IS NULL").unwrap();
        let doc = serde_json::json!({"x": null});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
        let doc2 = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc2, &p.query, &[]).unwrap());
    }

    #[test]
    fn coalesce_expression() {
        let p = crate::parse("SELECT VALUE c.nickname ?? c.name FROM c").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("Alice"));
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
    fn top_parameter_float_truncated() {
        let docs = vec![
            serde_json::json!({"x": 1}),
            serde_json::json!({"x": 2}),
            serde_json::json!({"x": 3}),
        ];
        let params = vec![("n".to_string(), serde_json::json!(2.7))];
        let results = query_documents("SELECT TOP @n * FROM c", &params, &docs).unwrap();
        assert_eq!(results.len(), 2);
    }

    // ── Bug fix tests: SUBSTRING character indexing ─────────────────────

    #[test]
    fn substring_multibyte_characters() {
        let p = crate::parse("SELECT VALUE SUBSTRING(c.name, 0, 2) FROM c").unwrap();
        let doc = serde_json::json!({"name": "日本語"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("日本"));
    }

    #[test]
    fn substring_emoji() {
        let p = crate::parse("SELECT VALUE SUBSTRING(c.name, 1, 2) FROM c").unwrap();
        let doc = serde_json::json!({"name": "A😀B😀C"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!("😀B"));
    }

    #[test]
    fn substring_past_end() {
        let p = crate::parse("SELECT VALUE SUBSTRING(c.name, 10, 5) FROM c").unwrap();
        let doc = serde_json::json!({"name": "short"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(""));
    }

    // ── Bug fix tests: LENGTH character count ───────────────────────────

    #[test]
    fn length_multibyte_characters() {
        let p = crate::parse("SELECT VALUE LENGTH(c.name) FROM c").unwrap();
        let doc = serde_json::json!({"name": "日本語"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(3));
    }

    #[test]
    fn length_emoji() {
        let p = crate::parse("SELECT VALUE LENGTH(c.name) FROM c").unwrap();
        let doc = serde_json::json!({"name": "A😀B"});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(3));
    }

    // ── Bug fix tests: negative array indexer ───────────────────────────

    #[test]
    fn negative_array_index_returns_undefined() {
        let p = crate::parse("SELECT VALUE c.items[-1] FROM c").unwrap();
        let doc = serde_json::json!({"items": [10, 20, 30]});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::Value::Null);
    }

    #[test]
    fn fractional_array_index_returns_undefined() {
        let p = crate::parse("SELECT VALUE c.items[1.5] FROM c").unwrap();
        let doc = serde_json::json!({"items": [10, 20, 30]});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::Value::Null);
    }

    // ── Bug fix tests: AND/OR three-valued logic ────────────────────────

    #[test]
    fn and_undefined_and_true_is_not_matching() {
        let p = crate::parse("SELECT * FROM c WHERE c.missing > 5 AND c.present = true").unwrap();
        let doc = serde_json::json!({"present": true});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn or_undefined_or_true_matches() {
        let p = crate::parse("SELECT * FROM c WHERE c.missing > 5 OR c.present = true").unwrap();
        let doc = serde_json::json!({"present": true});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn or_both_undefined_does_not_match() {
        let p = crate::parse("SELECT * FROM c WHERE c.missing1 > 5 OR c.missing2 > 5").unwrap();
        let doc = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn and_both_undefined_does_not_match() {
        let p = crate::parse("SELECT * FROM c WHERE c.missing1 > 5 AND c.missing2 > 5").unwrap();
        let doc = serde_json::json!({"x": 1});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    // ── Bug fix tests: LIKE pattern performance ─────────────────────────

    #[test]
    fn like_worst_case_pattern_completes_quickly() {
        let p = crate::parse("SELECT * FROM c WHERE c.name LIKE '%a%a%a%a%a%a%a%a%a%a%a%a%a%a%a%'")
            .unwrap();
        let doc = serde_json::json!({"name": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"});
        assert!(!matches_query(&doc, &p.query, &[]).unwrap());
    }

    #[test]
    fn like_still_matches_correctly() {
        let p = crate::parse("SELECT * FROM c WHERE c.name LIKE '%Al%ce%'").unwrap();
        let doc = serde_json::json!({"name": "Alice"});
        assert!(matches_query(&doc, &p.query, &[]).unwrap());
    }

    // ── Bug fix tests: Integer precision ────────────────────────────────

    #[test]
    fn integer_literal_preserved() {
        let p = crate::parse("SELECT VALUE c.id FROM c WHERE c.id = 9007199254740993").unwrap();
        let doc = serde_json::json!({"id": 9007199254740993_i64});
        let result = project(&doc, &p.query, &[]).unwrap();
        assert_eq!(result, serde_json::json!(9007199254740993_i64));
    }

    #[test]
    fn integer_equality_exact() {
        let p = crate::parse("SELECT * FROM c WHERE c.x = 42").unwrap();
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
}
