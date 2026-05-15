// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Common utilities shared across query modules.

use crate::query::ast::*;

/// Extract the root alias from a query's FROM clause.
///
/// For `FROM c` or `FROM root AS c`, this returns `Some("c")`.
/// For queries without a FROM clause, returns `None`.
pub(crate) fn get_root_alias(query: &SqlQuery) -> Option<String> {
    match &query.from {
        Some(from) => get_alias_from_collection(&from.collection),
        None => None,
    }
}

fn get_alias_from_collection(coll: &SqlCollectionExpression) -> Option<String> {
    match coll {
        SqlCollectionExpression::Aliased { collection, alias } => {
            alias.clone().or_else(|| match collection {
                SqlCollection::Path { root, .. } => Some(root.clone()),
                _ => None,
            })
        }
        SqlCollectionExpression::Join { left, .. } => get_alias_from_collection(left),
        SqlCollectionExpression::ArrayIterator { .. } => None,
    }
}

// ─── Parameter helpers (shared by `query::eval` and `query::plan`) ───────────

/// Slice of `@name → JSON value` pairs supplied for a parameterized query.
///
/// Names may be stored with or without a leading `@`; lookups normalize on
/// access. The plan/eval helpers operate on this shared shape so a single
/// parameter resolution path exists in the crate.
pub(crate) type Params = [(String, serde_json::Value)];

/// Strip a leading `@` from a parameter reference, if present.
pub(crate) fn normalize_parameter_name(name: &str) -> &str {
    name.trim_start_matches('@')
}

/// Resolve a parameter by name (with or without a leading `@`) to its JSON value.
pub(crate) fn resolve_parameter_value<'a>(
    parameters: &'a Params,
    name: &str,
) -> Option<&'a serde_json::Value> {
    let needle = normalize_parameter_name(name);
    parameters
        .iter()
        .find(|(param_name, _)| normalize_parameter_name(param_name) == needle)
        .map(|(_, value)| value)
}

/// Resolve a parameter to a non-negative `i64` for `TOP` / `OFFSET` / `LIMIT`.
///
/// Rejects floats (even integer-valued ones like `5.0`), strings, booleans,
/// missing parameters, and negative values. The error string is suitable for
/// embedding into a higher-level error type — call sites wrap it with their
/// own error kind.
pub(crate) fn resolve_non_negative_integer_parameter(
    parameters: &Params,
    name: &str,
) -> Result<i64, String> {
    let needle = normalize_parameter_name(name);
    let Some(value) = resolve_parameter_value(parameters, name) else {
        return Err(format!(
            "query references parameter @{needle} but no value was supplied"
        ));
    };
    match value {
        serde_json::Value::Number(n) => match n.as_i64() {
            Some(i) if i < 0 => Err(format!("parameter @{needle} must be non-negative; got {i}")),
            Some(i) => Ok(i),
            None => Err(format!("parameter @{needle} must be an integer; got {n}")),
        },
        other => Err(format!(
            "parameter @{needle} must be an integer; got {other}"
        )),
    }
}
