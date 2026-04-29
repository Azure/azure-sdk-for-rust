// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory query evaluation: match documents against WHERE clauses and apply projections.
//!
//! This evaluator interprets the SQL AST directly against `serde_json::Value` documents.
//! It supports the most commonly used scalar expressions, comparisons, and built-in functions.

use std::cmp::Ordering;

use crate::ast::*;
use crate::value::CosmosValue;

/// Error during query evaluation.
#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError {
    #[error("unsupported expression: {0}")]
    Unsupported(String),
    #[error("unknown function: {0}")]
    UnknownFunction(String),
    #[error("type error: {0}")]
    TypeError(String),
    #[error("parameter not found: @{0}")]
    ParameterNotFound(String),
}

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

/// Execute a full query against an in-memory collection of documents.
///
/// Supports WHERE filtering, SELECT projection, and TOP/OFFSET/LIMIT.
/// Does NOT support ORDER BY, GROUP BY, aggregates, or JOINs.
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
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let program = crate::parse(sql)?;
    let query = &program.query;

    let mut results = Vec::new();
    for doc in documents {
        if matches_query(doc, query, parameters)? {
            let projected = project(doc, query, parameters)?;
            results.push(projected);
        }
    }

    // Apply TOP
    if let Some(top) = &query.select.top {
        let n = match top {
            SqlTopSpec::Literal(n) => *n as usize,
            SqlTopSpec::Parameter(_) => results.len(), // can't resolve params for TOP here
        };
        results.truncate(n);
    }

    // Apply OFFSET / LIMIT
    if let Some(ol) = &query.offset_limit {
        let offset = match &ol.offset {
            SqlOffsetSpec::Literal(n) => *n as usize,
            SqlOffsetSpec::Parameter(_) => 0,
        };
        let limit = match &ol.limit {
            SqlLimitSpec::Literal(n) => *n as usize,
            SqlLimitSpec::Parameter(_) => results.len(),
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
            for (pname, pval) in params {
                if pname == name {
                    return Ok(CosmosValue::from_json(pval));
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
        SqlLiteral::Integer(n) => CosmosValue::Number(*n as f64),
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
        SqlBinaryOp::And => {
            let l = left.is_truthy();
            let r = right.is_truthy();
            CosmosValue::Boolean(l && r)
        }
        SqlBinaryOp::Or => {
            let l = left.is_truthy();
            let r = right.is_truthy();
            CosmosValue::Boolean(l || r)
        }
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

fn eval_unary(op: SqlUnaryOp, val: &CosmosValue) -> CosmosValue {
    match op {
        SqlUnaryOp::Not => match val {
            CosmosValue::Boolean(b) => CosmosValue::Boolean(!b),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::Minus => match val {
            CosmosValue::Number(n) => CosmosValue::Number(-n),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::Plus => match val {
            CosmosValue::Number(n) => CosmosValue::Number(*n),
            _ => CosmosValue::Undefined,
        },
        SqlUnaryOp::BitwiseNot => match val {
            CosmosValue::Number(n) => CosmosValue::Number((!(*n as i64)) as f64),
            _ => CosmosValue::Undefined,
        },
    }
}

fn numeric_op(left: &CosmosValue, right: &CosmosValue, f: fn(f64, f64) -> f64) -> CosmosValue {
    match (left, right) {
        (CosmosValue::Number(a), CosmosValue::Number(b)) => CosmosValue::Number(f(*a, *b)),
        _ => CosmosValue::Undefined,
    }
}

fn int_op(left: &CosmosValue, right: &CosmosValue, f: fn(i64, i64) -> i64) -> CosmosValue {
    match (left, right) {
        (CosmosValue::Number(a), CosmosValue::Number(b)) => {
            CosmosValue::Number(f(*a as i64, *b as i64) as f64)
        }
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
            Some(CosmosValue::Number(_))
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
            Some(CosmosValue::String(s)) => Ok(CosmosValue::Number(s.len() as f64)),
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
        "SUBSTRING" => match args {
            [CosmosValue::String(s), CosmosValue::Number(start), CosmosValue::Number(len)] => {
                let start = *start as usize;
                let len = *len as usize;
                if start >= s.len() {
                    Ok(CosmosValue::String(String::new()))
                } else {
                    let end = (start + len).min(s.len());
                    Ok(CosmosValue::String(s[start..end].to_string()))
                }
            }
            _ => Ok(CosmosValue::Undefined),
        },
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
            _ => Ok(CosmosValue::Undefined),
        },
        "RIGHT" => match args {
            [CosmosValue::String(s), CosmosValue::Number(n)] => {
                let n = *n as usize;
                let chars: Vec<char> = s.chars().collect();
                let start = chars.len().saturating_sub(n);
                Ok(CosmosValue::String(chars[start..].iter().collect()))
            }
            _ => Ok(CosmosValue::Undefined),
        },
        "TOSTRING" => match args.first() {
            Some(CosmosValue::String(s)) => Ok(CosmosValue::String(s.clone())),
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
            Some(CosmosValue::Array(arr)) => Ok(CosmosValue::Number(arr.len() as f64)),
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
    like_match_recursive(&text_chars, 0, &pattern_chars, 0, escape_char)
}

fn like_match_recursive(
    text: &[char],
    ti: usize,
    pattern: &[char],
    pi: usize,
    escape: Option<char>,
) -> bool {
    if pi == pattern.len() {
        return ti == text.len();
    }

    let pc = pattern[pi];

    // Check for escape character
    if Some(pc) == escape && pi + 1 < pattern.len() {
        // Next character is literal
        if ti < text.len() && text[ti] == pattern[pi + 1] {
            return like_match_recursive(text, ti + 1, pattern, pi + 2, escape);
        }
        return false;
    }

    match pc {
        '%' => {
            // Match zero or more characters
            for i in ti..=text.len() {
                if like_match_recursive(text, i, pattern, pi + 1, escape) {
                    return true;
                }
            }
            false
        }
        '_' => {
            // Match exactly one character
            if ti < text.len() {
                like_match_recursive(text, ti + 1, pattern, pi + 1, escape)
            } else {
                false
            }
        }
        _ => {
            if ti < text.len() && text[ti] == pc {
                like_match_recursive(text, ti + 1, pattern, pi + 1, escape)
            } else {
                false
            }
        }
    }
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

fn get_root_alias(query: &SqlQuery) -> Option<String> {
    match &query.from {
        Some(from) => match &from.collection {
            SqlCollectionExpression::Aliased { collection, alias } => {
                alias.clone().or_else(|| match collection {
                    SqlCollection::Path { root, .. } => Some(root.clone()),
                    _ => None,
                })
            }
            SqlCollectionExpression::Join { left, .. } => match left.as_ref() {
                SqlCollectionExpression::Aliased { collection, alias } => {
                    alias.clone().or_else(|| match collection {
                        SqlCollection::Path { root, .. } => Some(root.clone()),
                        _ => None,
                    })
                }
                _ => None,
            },
            _ => None,
        },
        None => None,
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
}
