// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Local evaluator for Cosmos DB PATCH operations.
//!
//! This is the "M" of the PATCH handler's Read-Modify-Write loop. After the
//! handler reads the current item it asks [`apply_patch_ops`] to materialize
//! the requested mutations on the in-memory document; the handler then writes
//! the result back via an ETag-guarded Replace.
//!
//! The evaluator is intentionally schema-agnostic — it works against
//! [`serde_json::Value`] and never deserializes into the caller's type.
//!
//! # Semantics
//!
//! Path parsing follows RFC 6901 (JSON Pointer), with the standard escapes
//! `~0 -> ~` and `~1 -> /`. The trailing token `-` in an array context refers
//! to the past-the-end position (used only by [`PatchOp::Add`] to append).
//!
//! Op semantics follow Cosmos DB's documented PATCH behavior:
//!
//! * [`Add`](PatchOp::Add) — the parent path must exist. Arrays: numeric index
//!   `<= len`, or `-` to append. Objects: replaces existing keys.
//! * [`Set`](PatchOp::Set) — like Add but a missing leaf in an object always
//!   succeeds (Add-or-Replace). Cosmos-specific.
//! * [`Replace`](PatchOp::Replace) — the leaf must already exist.
//! * [`Remove`](PatchOp::Remove) — the leaf must exist; cannot target root.
//! * [`Increment`](PatchOp::Increment) — leaf must be a JSON number;
//!   [`IncrValue::Int`] refuses to merge with a fractional target.
//! * [`MoveOp`](PatchOp::MoveOp) — source must exist; source and destination
//!   must be distinct; destination cannot be a descendant of the source.
//!
//! Failures return [`PatchEvalError`], which the PATCH handler converts into
//! an `azure_core::Error` before surfacing it to callers.

use crate::models::{IncrValue, PatchOp};
use serde_json::Value;
use std::fmt;

/// Errors produced by [`apply_patch_ops`] when an op cannot be applied.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PatchEvalError {
    /// The path is not a valid JSON Pointer (e.g., does not start with `/`,
    /// or contains an invalid `~`-escape).
    InvalidPath(String),
    /// A non-final token addressed a non-container (e.g., navigating into a
    /// scalar) or referenced a missing key/index.
    MissingParent(String),
    /// The leaf path the op needed does not exist.
    MissingTarget(String),
    /// An array token was not `-` and not parseable as a non-negative integer.
    InvalidArrayIndex {
        /// The offending token, percent-decoded.
        token: String,
        /// The full path that contained it.
        path: String,
    },
    /// An array index was greater than the array's length.
    ArrayIndexOutOfRange {
        /// Index requested.
        index: usize,
        /// Array length.
        len: usize,
        /// Containing path.
        path: String,
    },
    /// The path resolved to a value of an unexpected JSON type (e.g.,
    /// [`Increment`](PatchOp::Increment) on a string).
    TypeMismatch {
        /// Human-readable expected description.
        expected: &'static str,
        /// Human-readable actual type.
        actual: &'static str,
        /// Containing path.
        path: String,
    },
    /// Attempted to remove the root document, which is not permitted.
    CannotRemoveRoot,
    /// A `move` op specified identical `from`/`path`, or attempted to move a
    /// subtree under its own descendant.
    InvalidMove(String),
}

impl fmt::Display for PatchEvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatchEvalError::InvalidPath(p) => write!(f, "invalid JSON Pointer: '{p}'"),
            PatchEvalError::MissingParent(p) => write!(f, "missing parent path: '{p}'"),
            PatchEvalError::MissingTarget(p) => write!(f, "missing path: '{p}'"),
            PatchEvalError::InvalidArrayIndex { token, path } => {
                write!(f, "invalid array index '{token}' in path '{path}'")
            }
            PatchEvalError::ArrayIndexOutOfRange { index, len, path } => write!(
                f,
                "array index {index} out of range (len={len}) in path '{path}'"
            ),
            PatchEvalError::TypeMismatch {
                expected,
                actual,
                path,
            } => write!(
                f,
                "type mismatch at '{path}': expected {expected}, got {actual}"
            ),
            PatchEvalError::CannotRemoveRoot => write!(f, "cannot remove root document"),
            PatchEvalError::InvalidMove(msg) => write!(f, "invalid move: {msg}"),
        }
    }
}

impl std::error::Error for PatchEvalError {}

impl From<PatchEvalError> for azure_core::Error {
    fn from(err: PatchEvalError) -> Self {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            err.to_string(),
        )
    }
}

/// Applies the ordered list of `ops` to `doc` in place, returning the first
/// failure (if any). Ops up to the failing one have already mutated `doc`.
pub(crate) fn apply_patch_ops(doc: &mut Value, ops: &[PatchOp]) -> Result<(), PatchEvalError> {
    for op in ops {
        apply_one(doc, op)?;
    }
    Ok(())
}

fn apply_one(doc: &mut Value, op: &PatchOp) -> Result<(), PatchEvalError> {
    match op {
        PatchOp::Add { path, value } => add_or_set(doc, path, value.clone(), AddOrSet::Add),
        PatchOp::Set { path, value } => add_or_set(doc, path, value.clone(), AddOrSet::Set),
        PatchOp::Replace { path, value } => replace(doc, path, value.clone()),
        PatchOp::Remove { path } => remove(doc, path).map(|_| ()),
        PatchOp::Increment { path, value } => increment(doc, path, *value),
        PatchOp::MoveOp { from, path } => move_op(doc, from, path),
    }
}

// ---------------------------------------------------------------------------
// JSON Pointer parsing (RFC 6901)
// ---------------------------------------------------------------------------

fn parse_pointer(path: &str) -> Result<Vec<String>, PatchEvalError> {
    if path.is_empty() {
        return Ok(Vec::new());
    }
    if !path.starts_with('/') {
        return Err(PatchEvalError::InvalidPath(path.to_string()));
    }
    let mut tokens = Vec::new();
    for raw in path[1..].split('/') {
        tokens.push(unescape_token(raw)?);
    }
    Ok(tokens)
}

fn unescape_token(raw: &str) -> Result<String, PatchEvalError> {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c == '~' {
            match chars.next() {
                Some('0') => out.push('~'),
                Some('1') => out.push('/'),
                _ => return Err(PatchEvalError::InvalidPath(raw.to_string())),
            }
        } else {
            out.push(c);
        }
    }
    Ok(out)
}

fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

// ---------------------------------------------------------------------------
// Navigation helpers
// ---------------------------------------------------------------------------

/// Walk to the parent of the path, returning a mutable reference plus the
/// final unescaped token. For root paths the caller treats this as "the
/// document itself" or rejects it.
fn navigate_parent_mut<'a, 'b>(
    doc: &'a mut Value,
    tokens: &'b [String],
    path: &str,
) -> Result<(&'a mut Value, &'b str), PatchEvalError> {
    if tokens.is_empty() {
        return Err(PatchEvalError::InvalidPath(path.to_string()));
    }
    let (last, prefix) = tokens.split_last().expect("non-empty");
    let mut cursor = doc;
    for token in prefix {
        cursor = step_into(cursor, token, path)?;
    }
    Ok((cursor, last.as_str()))
}

fn step_into<'a>(
    node: &'a mut Value,
    token: &str,
    full_path: &str,
) -> Result<&'a mut Value, PatchEvalError> {
    match node {
        Value::Object(map) => map
            .get_mut(token)
            .ok_or_else(|| PatchEvalError::MissingParent(full_path.to_string())),
        Value::Array(arr) => {
            let idx = parse_array_index(token, full_path, arr.len(), false)?;
            arr.get_mut(idx)
                .ok_or_else(|| PatchEvalError::MissingParent(full_path.to_string()))
        }
        _ => Err(PatchEvalError::MissingParent(full_path.to_string())),
    }
}

fn parse_array_index(
    token: &str,
    full_path: &str,
    len: usize,
    allow_append: bool,
) -> Result<usize, PatchEvalError> {
    if token == "-" {
        if allow_append {
            return Ok(len);
        }
        return Err(PatchEvalError::InvalidArrayIndex {
            token: token.to_string(),
            path: full_path.to_string(),
        });
    }
    let idx: usize = token
        .parse()
        .map_err(|_| PatchEvalError::InvalidArrayIndex {
            token: token.to_string(),
            path: full_path.to_string(),
        })?;
    // when `allow_append=false` (Set / Replace on a numeric index), the
    // array must contain that index. `len.saturating_sub(1)` is `0` for an
    // empty array, which would let `idx=0` slip through and cause the caller's
    // `arr[idx] = v` to panic with "index out of bounds". Reject empty arrays
    // up front so the typed `ArrayIndexOutOfRange` surfaces instead of a
    // panic.
    if !allow_append && len == 0 {
        return Err(PatchEvalError::ArrayIndexOutOfRange {
            index: idx,
            len,
            path: full_path.to_string(),
        });
    }
    let upper = if allow_append {
        len
    } else {
        len.saturating_sub(1)
    };
    if idx > upper {
        return Err(PatchEvalError::ArrayIndexOutOfRange {
            index: idx,
            len,
            path: full_path.to_string(),
        });
    }
    Ok(idx)
}

// ---------------------------------------------------------------------------
// Op evaluators
// ---------------------------------------------------------------------------

#[derive(Copy, Clone)]
enum AddOrSet {
    Add,
    Set,
}

fn add_or_set(
    doc: &mut Value,
    path: &str,
    new_value: Value,
    mode: AddOrSet,
) -> Result<(), PatchEvalError> {
    let tokens = parse_pointer(path)?;
    if tokens.is_empty() {
        // Replacing the entire document is allowed by Set; Add at root is also
        // accepted (treated as a Replace of root).
        *doc = new_value;
        return Ok(());
    }
    let (parent, last) = navigate_parent_mut(doc, &tokens, path)?;
    match parent {
        Value::Object(map) => {
            match mode {
                AddOrSet::Add | AddOrSet::Set => {
                    map.insert(last.to_string(), new_value);
                }
            }
            Ok(())
        }
        Value::Array(arr) => {
            // Set on a numeric array index replaces the element (no shift).
            // Add inserts at the index, shifting the tail right. The trailing
            // '-' token always appends, regardless of mode. This matches the
            // .NET / Java SDK's "Set is similar to Add… in an array, Set
            // replaces an existing element value" semantics.
            if matches!(mode, AddOrSet::Set) && last != "-" {
                let idx = parse_array_index(last, path, arr.len(), false)?;
                arr[idx] = new_value;
                return Ok(());
            }
            let idx = parse_array_index(last, path, arr.len(), true)?;
            if idx == arr.len() {
                arr.push(new_value);
            } else {
                arr.insert(idx, new_value);
            }
            Ok(())
        }
        other => Err(PatchEvalError::TypeMismatch {
            expected: "object or array",
            actual: json_type_name(other),
            path: path.to_string(),
        }),
    }
}

fn replace(doc: &mut Value, path: &str, new_value: Value) -> Result<(), PatchEvalError> {
    let tokens = parse_pointer(path)?;
    if tokens.is_empty() {
        *doc = new_value;
        return Ok(());
    }
    let (parent, last) = navigate_parent_mut(doc, &tokens, path)?;
    match parent {
        Value::Object(map) => {
            if !map.contains_key(last) {
                return Err(PatchEvalError::MissingTarget(path.to_string()));
            }
            map.insert(last.to_string(), new_value);
            Ok(())
        }
        Value::Array(arr) => {
            let idx = parse_array_index(last, path, arr.len(), false)?;
            arr[idx] = new_value;
            Ok(())
        }
        other => Err(PatchEvalError::TypeMismatch {
            expected: "object or array",
            actual: json_type_name(other),
            path: path.to_string(),
        }),
    }
}

fn remove(doc: &mut Value, path: &str) -> Result<Value, PatchEvalError> {
    let tokens = parse_pointer(path)?;
    if tokens.is_empty() {
        return Err(PatchEvalError::CannotRemoveRoot);
    }
    let (parent, last) = navigate_parent_mut(doc, &tokens, path)?;
    match parent {
        Value::Object(map) => map
            .remove(last)
            .ok_or_else(|| PatchEvalError::MissingTarget(path.to_string())),
        Value::Array(arr) => {
            let idx = parse_array_index(last, path, arr.len(), false)?;
            if arr.is_empty() {
                return Err(PatchEvalError::MissingTarget(path.to_string()));
            }
            Ok(arr.remove(idx))
        }
        other => Err(PatchEvalError::TypeMismatch {
            expected: "object or array",
            actual: json_type_name(other),
            path: path.to_string(),
        }),
    }
}

fn increment(doc: &mut Value, path: &str, delta: IncrValue) -> Result<(), PatchEvalError> {
    let tokens = parse_pointer(path)?;
    if tokens.is_empty() {
        return Err(PatchEvalError::MissingTarget(path.to_string()));
    }
    let (parent, last) = navigate_parent_mut(doc, &tokens, path)?;
    let leaf = match parent {
        Value::Object(map) => map
            .get_mut(last)
            .ok_or_else(|| PatchEvalError::MissingTarget(path.to_string()))?,
        Value::Array(arr) => {
            let idx = parse_array_index(last, path, arr.len(), false)?;
            arr.get_mut(idx)
                .ok_or_else(|| PatchEvalError::MissingTarget(path.to_string()))?
        }
        other => {
            return Err(PatchEvalError::TypeMismatch {
                expected: "object or array",
                actual: json_type_name(other),
                path: path.to_string(),
            });
        }
    };

    let num = match leaf {
        Value::Number(n) => n.clone(),
        other => {
            return Err(PatchEvalError::TypeMismatch {
                expected: "number",
                actual: json_type_name(other),
                path: path.to_string(),
            });
        }
    };

    let new_value = match (delta, num.as_i64(), num.as_f64()) {
        // Integer delta on integer target — preserve integer fidelity.
        (IncrValue::Int(d), Some(target), _) => {
            let sum = target
                .checked_add(d)
                .ok_or_else(|| PatchEvalError::TypeMismatch {
                    expected: "i64 (within range)",
                    actual: "i64 (overflow)",
                    path: path.to_string(),
                })?;
            Value::Number(sum.into())
        }
        // Integer delta on floating target — fail rather than demote.
        (IncrValue::Int(_), None, Some(_)) => {
            return Err(PatchEvalError::TypeMismatch {
                expected: "integer number",
                actual: "fractional number",
                path: path.to_string(),
            });
        }
        // Float delta on integer target — promote to f64.
        (IncrValue::Float(d), Some(target), _) => {
            let sum = (target as f64) + d;
            match serde_json::Number::from_f64(sum) {
                Some(n) => Value::Number(n),
                None => {
                    return Err(PatchEvalError::TypeMismatch {
                        expected: "finite number",
                        actual: "non-finite",
                        path: path.to_string(),
                    });
                }
            }
        }
        // Float delta on floating target.
        (IncrValue::Float(d), None, Some(target)) => {
            let sum = target + d;
            match serde_json::Number::from_f64(sum) {
                Some(n) => Value::Number(n),
                None => {
                    return Err(PatchEvalError::TypeMismatch {
                        expected: "finite number",
                        actual: "non-finite",
                        path: path.to_string(),
                    });
                }
            }
        }
        _ => {
            return Err(PatchEvalError::TypeMismatch {
                expected: "number",
                actual: "non-numeric",
                path: path.to_string(),
            });
        }
    };

    *leaf = new_value;
    Ok(())
}

fn move_op(doc: &mut Value, from: &str, to: &str) -> Result<(), PatchEvalError> {
    if from == to {
        return Ok(());
    }

    // descendant check at the JSON-Pointer token level, not byte level.
    // The previous byte-prefix check happens to be correct for canonical
    // encodings (escapes only encode '/' and '~'), but it's brittle: e.g.
    // `from = "/a"` would erroneously look like a prefix of `to = "/ab/c"`
    // without the trailing-'/' guard. Comparing parsed token slices makes
    // the invariant explicit and robust to any future encoding changes.
    let from_tokens = parse_pointer(from)?;
    let to_tokens = parse_pointer(to)?;
    if to_tokens.len() > from_tokens.len() && to_tokens[..from_tokens.len()] == from_tokens[..] {
        return Err(PatchEvalError::InvalidMove(format!(
            "destination '{to}' is a descendant of source '{from}'"
        )));
    }

    // atomic move via clone-stage-commit. The previous implementation
    // performed `remove` directly on `doc`, then `add_or_set`. If the
    // second step failed (e.g. the destination's parent didn't exist), the
    // original document was already mutated and the caller observed a
    // partial state — violating the "fail leaves doc unchanged" invariant.
    // Stage both steps on a clone, then commit on success.
    let mut staged = doc.clone();
    let value = remove(&mut staged, from)?;
    add_or_set(&mut staged, to, value, AddOrSet::Add)?;
    *doc = staged;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn apply(doc: Value, ops: &[PatchOp]) -> Result<Value, PatchEvalError> {
        let mut d = doc;
        apply_patch_ops(&mut d, ops)?;
        Ok(d)
    }

    #[test]
    fn add_into_object_inserts_key() {
        let doc = json!({"a": 1});
        let out = apply(doc, &[PatchOp::add("/b", json!(2))]).unwrap();
        assert_eq!(out, json!({"a": 1, "b": 2}));
    }

    #[test]
    fn add_with_dash_appends_to_array() {
        let doc = json!({"xs": [1, 2]});
        let out = apply(doc, &[PatchOp::add("/xs/-", json!(3))]).unwrap();
        assert_eq!(out, json!({"xs": [1, 2, 3]}));
    }

    #[test]
    fn add_at_array_index_inserts() {
        let doc = json!({"xs": [1, 3]});
        let out = apply(doc, &[PatchOp::add("/xs/1", json!(2))]).unwrap();
        assert_eq!(out, json!({"xs": [1, 2, 3]}));
    }

    #[test]
    fn add_with_missing_parent_fails() {
        let doc = json!({});
        let err = apply(doc, &[PatchOp::add("/a/b", json!(1))]).unwrap_err();
        assert!(matches!(err, PatchEvalError::MissingParent(_)), "{err}");
    }

    #[test]
    fn set_creates_missing_leaf_in_object() {
        let doc = json!({"a": 1});
        let out = apply(doc, &[PatchOp::set("/b", json!(2))]).unwrap();
        assert_eq!(out, json!({"a": 1, "b": 2}));
    }

    #[test]
    fn replace_missing_leaf_fails() {
        let doc = json!({"a": 1});
        let err = apply(doc, &[PatchOp::replace("/b", json!(2))]).unwrap_err();
        assert!(matches!(err, PatchEvalError::MissingTarget(_)), "{err}");
    }

    #[test]
    fn replace_allows_type_change() {
        // RFC 6902 permits replace to change a value's type.
        let doc = json!({"a": 1});
        let out = apply(doc, &[PatchOp::replace("/a", json!("hi"))]).unwrap();
        assert_eq!(out, json!({"a": "hi"}));
    }

    #[test]
    fn remove_strips_leaf() {
        let doc = json!({"a": 1, "b": 2});
        let out = apply(doc, &[PatchOp::remove("/a")]).unwrap();
        assert_eq!(out, json!({"b": 2}));
    }

    #[test]
    fn remove_root_fails() {
        let doc = json!({"a": 1});
        let err = apply(doc, &[PatchOp::remove("")]).unwrap_err();
        assert!(matches!(err, PatchEvalError::CannotRemoveRoot));
    }

    #[test]
    fn remove_missing_fails() {
        let doc = json!({"a": 1});
        let err = apply(doc, &[PatchOp::remove("/missing")]).unwrap_err();
        assert!(matches!(err, PatchEvalError::MissingTarget(_)), "{err}");
    }

    #[test]
    fn increment_int_preserves_fidelity() {
        let doc = json!({"n": 9_000_000_000_000_000i64});
        let out = apply(doc, &[PatchOp::increment("/n", 1i64)]).unwrap();
        let n = out["n"].as_i64().unwrap();
        assert_eq!(n, 9_000_000_000_000_001);
    }

    #[test]
    fn increment_int_on_float_fails() {
        let doc = json!({"n": 1.5});
        let err = apply(doc, &[PatchOp::increment("/n", 1i64)]).unwrap_err();
        assert!(matches!(err, PatchEvalError::TypeMismatch { .. }), "{err}");
    }

    #[test]
    fn increment_float_on_int_promotes_to_float() {
        let doc = json!({"n": 2});
        let out = apply(doc, &[PatchOp::increment("/n", 0.5f64)]).unwrap();
        assert!((out["n"].as_f64().unwrap() - 2.5).abs() < f64::EPSILON);
    }

    #[test]
    fn increment_on_non_number_fails() {
        let doc = json!({"n": "five"});
        let err = apply(doc, &[PatchOp::increment("/n", 1i64)]).unwrap_err();
        assert!(matches!(err, PatchEvalError::TypeMismatch { .. }), "{err}");
    }

    #[test]
    fn move_renames_field() {
        let doc = json!({"a": 1});
        let out = apply(doc, &[PatchOp::move_op("/a", "/b")]).unwrap();
        assert_eq!(out, json!({"b": 1}));
    }

    #[test]
    fn move_to_same_path_is_noop() {
        let doc = json!({"a": 1});
        let out = apply(doc, &[PatchOp::move_op("/a", "/a")]).unwrap();
        assert_eq!(out, json!({"a": 1}));
    }

    #[test]
    fn move_into_own_descendant_fails() {
        let doc = json!({"a": {"x": 1}});
        let err = apply(doc, &[PatchOp::move_op("/a", "/a/inner")]).unwrap_err();
        assert!(matches!(err, PatchEvalError::InvalidMove(_)), "{err}");
    }

    #[test]
    fn move_into_escaped_descendant_fails() {
        // a token containing an escaped '/' must still count as a single
        // pointer segment when computing the descendant relationship. The
        // unescaped key is `a/b`, so `/a~1b/c` is a true descendant of
        // `/a~1b` even though the byte-prefix check is also correct in this
        // specific case.
        let doc = json!({"a/b": {"x": 1}});
        let err = apply(doc, &[PatchOp::move_op("/a~1b", "/a~1b/c")]).unwrap_err();
        assert!(matches!(err, PatchEvalError::InvalidMove(_)), "{err}");
    }

    #[test]
    fn move_to_sibling_with_shared_prefix_succeeds() {
        // `/a` is NOT a descendant of `/ab` (byte-prefix would say yes
        // without a `/` guard; token-level compare says no). The move must
        // proceed.
        let doc = json!({"a": 1, "ab": {"x": 2}});
        let out = apply(doc, &[PatchOp::move_op("/a", "/ab/y")]).unwrap();
        assert_eq!(out, json!({"ab": {"x": 2, "y": 1}}));
    }

    #[test]
    fn move_failure_leaves_doc_unchanged() {
        // when the destination is invalid (parent missing), the source
        // must not be removed. The pre-fix implementation would have left
        // the doc as `{"b": {}}` (source removed before add_or_set failed).
        let doc = json!({"a": 1, "b": {}});
        let original = doc.clone();
        let mut d = doc;
        let err =
            apply_patch_ops(&mut d, &[PatchOp::move_op("/a", "/missing/parent/x")]).unwrap_err();
        assert!(matches!(err, PatchEvalError::MissingParent(_)), "{err}");
        assert_eq!(
            d, original,
            "move_op must be atomic: a failed move must leave the document unchanged"
        );
    }

    #[test]
    fn set_at_array_index_replaces_in_place() {
        // Set on a numeric array index replaces the existing element
        // (no shift). Cosmos backend semantics: "set is similar to add… in
        // an array, Set replaces an existing element value".
        let doc = json!({"xs": [1, 2, 3]});
        let out = apply(doc, &[PatchOp::set("/xs/1", json!(9))]).unwrap();
        assert_eq!(out, json!({"xs": [1, 9, 3]}));
    }

    #[test]
    fn set_with_dash_appends_to_array() {
        // Set with the trailing '-' token still appends, matching Add.
        let doc = json!({"xs": [1, 2]});
        let out = apply(doc, &[PatchOp::set("/xs/-", json!(3))]).unwrap();
        assert_eq!(out, json!({"xs": [1, 2, 3]}));
    }

    #[test]
    fn set_at_array_index_out_of_range_fails() {
        // Set on an out-of-range numeric index must fail rather than
        // silently insert (the Add codepath uses allow_append=true; Set
        // does not).
        let doc = json!({"xs": [1, 2]});
        let err = apply(doc, &[PatchOp::set("/xs/5", json!(9))]).unwrap_err();
        assert!(
            matches!(err, PatchEvalError::ArrayIndexOutOfRange { .. }),
            "{err}"
        );
    }

    #[test]
    fn pointer_escapes() {
        // ~1 -> '/', ~0 -> '~'
        let doc = json!({"a/b": 1, "tilde~": "x"});
        let out = apply(doc.clone(), &[PatchOp::replace("/a~1b", json!(2))]).unwrap();
        assert_eq!(out["a/b"], json!(2));
        let out2 = apply(doc, &[PatchOp::replace("/tilde~0", json!("y"))]).unwrap();
        assert_eq!(out2["tilde~"], json!("y"));
    }

    #[test]
    fn invalid_pointer_fails() {
        let doc = json!({});
        let err = apply(doc, &[PatchOp::add("invalid", json!(1))]).unwrap_err();
        assert!(matches!(err, PatchEvalError::InvalidPath(_)), "{err}");
    }

    #[test]
    fn array_index_out_of_range_fails() {
        let doc = json!({"xs": [1]});
        let err = apply(doc, &[PatchOp::add("/xs/5", json!(2))]).unwrap_err();
        assert!(
            matches!(err, PatchEvalError::ArrayIndexOutOfRange { .. }),
            "{err}"
        );
    }

    #[test]
    fn ops_apply_in_order() {
        // Increment is transitive, so three Increments alone would pass even
        // if ops were reordered. Use Increment -> Move -> Increment so the
        // assertion is only satisfied when ops execute in the listed order.
        // Sequence: start at {a: 0}.
        // 1. `Increment /a 1`   -> {a: 1}
        // 2. `Move /a /b`       -> {b: 1}        (a is now absent)
        // 3. `Increment /b 1`   -> {b: 2}
        // If any pair is reordered, step 3 either errors (path missing) or
        // observes a different intermediate state.
        let doc = json!({"a": 0});
        let out = apply(
            doc,
            &[
                PatchOp::increment("/a", 1i64),
                PatchOp::move_op("/a", "/b"),
                PatchOp::increment("/b", 1i64),
            ],
        )
        .unwrap();
        assert_eq!(out["b"], json!(2));
        assert!(
            out.get("a").is_none(),
            "after Move /a /b, /a must be absent; got {out}"
        );
    }

    #[test]
    fn set_on_empty_array_index_zero_errors_not_panics() {
        // `Set('/xs/0', v)` against `{xs: []}` would previously slip
        // through `parse_array_index` (because `len.saturating_sub(1) = 0`
        // when `len == 0`) and panic at `arr[0] = v`. The empty-array guard
        // must convert this into a typed `ArrayIndexOutOfRange`.
        let doc = json!({"xs": []});
        let err = apply(doc, &[PatchOp::set("/xs/0", json!("v"))]).unwrap_err();
        assert!(
            matches!(
                err,
                PatchEvalError::ArrayIndexOutOfRange {
                    index: 0,
                    len: 0,
                    ..
                }
            ),
            "{err}"
        );
    }

    #[test]
    fn replace_on_empty_array_index_zero_errors_not_panics() {
        // same shape as the Set case but on the pre-existing Replace
        // branch (`arr[idx] = v` at the bottom of `replace`).
        let doc = json!({"xs": []});
        let err = apply(doc, &[PatchOp::replace("/xs/0", json!("v"))]).unwrap_err();
        assert!(
            matches!(
                err,
                PatchEvalError::ArrayIndexOutOfRange {
                    index: 0,
                    len: 0,
                    ..
                }
            ),
            "{err}"
        );
    }

    #[test]
    fn add_on_empty_array_index_zero_succeeds() {
        // F14 regression guard: the empty-array fix only applies when
        // `allow_append=false`. `Add('/xs/0', v)` on `{xs: []}` is an insert
        // at index 0 on an empty array (equivalent to push), which RFC 6902
        // permits and which `allow_append=true` correctly handles.
        let doc = json!({"xs": []});
        let out = apply(doc, &[PatchOp::add("/xs/0", json!("v"))]).unwrap();
        assert_eq!(out, json!({"xs": ["v"]}));
    }

    #[test]
    fn add_on_empty_array_with_dash_succeeds() {
        // F14 regression guard: `-` (append) on an empty array still works.
        let doc = json!({"xs": []});
        let out = apply(doc, &[PatchOp::add("/xs/-", json!("v"))]).unwrap();
        assert_eq!(out, json!({"xs": ["v"]}));
    }
}
