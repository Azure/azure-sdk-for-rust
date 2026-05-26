// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos DB PATCH operation types.
//!
//! These types describe a list of mutations to apply to a single item via the
//! driver's PATCH handler. Unlike the rest of the data plane the PATCH handler
//! operates on a `serde_json::Value` rather than opaque bytes — patching is the
//! one place the driver must inspect document contents in order to apply the
//! requested changes locally.
//!
//! The wire format matches the Cosmos DB REST API PATCH document format and
//! the existing patch documents emitted by the .NET SDK.
//!
//! # Example
//!
//! ```
//! use azure_data_cosmos_driver::models::{PatchOperation, PatchDocument};
//!
//! let spec = PatchDocument::new(vec![
//!     PatchOperation::set("/age", serde_json::json!(31)),
//!     PatchOperation::increment("/visits", 1i64),
//!     PatchOperation::add("/tags/-", serde_json::json!("new-tag")),
//! ]);
//! let bytes = serde_json::to_vec(&spec).unwrap();
//! // The wire payload is a JSON object with an `operations` array; the
//! // first op above is a `set`.
//! assert!(bytes.starts_with(b"{"));
//! assert!(bytes.windows(5).any(|w| w == b"\"set\""));
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A typed numeric increment delta for [`PatchOperation::Increment`].
///
/// Distinguishes integer increments (which preserve integer fidelity end-to-end)
/// from floating-point increments. Mixing an `Int(i64)` with a `f64` target —
/// or vice versa — is rejected by the patch evaluator at apply time rather than
/// silently demoting integer values to floating point.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum CosmosNumber {
    /// Integer increment. Preserves integer fidelity even when serialized into
    /// JSON, and refuses to merge with floating-point targets.
    Int(i64),
    /// Floating-point increment. Accepted on integer-valued numbers as well
    /// (the target is promoted to a JSON number with a fractional component).
    Float(f64),
}

impl From<i64> for CosmosNumber {
    fn from(v: i64) -> Self {
        CosmosNumber::Int(v)
    }
}

impl From<f64> for CosmosNumber {
    fn from(v: f64) -> Self {
        CosmosNumber::Float(v)
    }
}

impl Serialize for CosmosNumber {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CosmosNumber::Int(n) => serializer.serialize_i64(*n),
            CosmosNumber::Float(n) => serializer.serialize_f64(*n),
        }
    }
}

impl<'de> Deserialize<'de> for CosmosNumber {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Number::deserialize(deserializer)?;
        if let Some(n) = value.as_i64() {
            Ok(CosmosNumber::Int(n))
        } else if let Some(n) = value.as_f64() {
            Ok(CosmosNumber::Float(n))
        } else {
            Err(serde::de::Error::custom("increment value is not a number"))
        }
    }
}

/// A single operation in a Cosmos DB PATCH document.
///
/// PATCH operations follow the JSON Pointer (RFC 6901) path syntax. Mutation
/// semantics are evaluated locally by the driver's PATCH handler against the
/// item read from the service.
///
/// Both the enum variants and the equivalent factory functions (`PatchOperation::add`,
/// `PatchOperation::set`, ...) are part of the public API. The factories mirror the
/// .NET SDK's `PatchOperation.Add` / `.Set` / etc. methods and are the
/// recommended way to construct ops.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
#[non_exhaustive]
pub enum PatchOperation {
    /// Add (insert) `value` at `path`.
    ///
    /// * The parent path must already exist.
    /// * For arrays, the trailing token `-` appends to the array; numeric
    ///   tokens insert at that index (must be `<= len`).
    /// * For objects, adding to an existing key replaces its value.
    Add {
        /// JSON Pointer path (RFC 6901) targeting the location to add.
        path: String,
        /// JSON value to add.
        value: Value,
    },
    /// Set `value` at `path`, creating intermediate parents only when the
    /// parent path already exists. This is Cosmos-specific (Add-or-Replace).
    Set {
        /// JSON Pointer path (RFC 6901).
        path: String,
        /// JSON value to assign.
        value: Value,
    },
    /// Replace the value at an existing `path` with `value`.
    Replace {
        /// JSON Pointer path (RFC 6901) targeting an existing location.
        path: String,
        /// New JSON value.
        value: Value,
    },
    /// Remove the value at `path`. The path must exist; root removal is not
    /// permitted.
    Remove {
        /// JSON Pointer path (RFC 6901) targeting an existing location.
        path: String,
    },
    /// Increment the integer or floating-point number at `path` by the
    /// configured delta.
    Increment {
        /// JSON Pointer path (RFC 6901) targeting an existing JSON number.
        path: String,
        /// Increment delta, preserving int/float fidelity.
        value: CosmosNumber,
    },
    /// Move (rename) the JSON value from `from` to `path`.
    ///
    /// Renamed `move_op` in Rust because `move` is a keyword. Serialized as
    /// `"op": "move"` to remain compatible with the on-the-wire format.
    #[serde(rename = "move")]
    MoveOp {
        /// Source JSON Pointer path.
        from: String,
        /// Destination JSON Pointer path.
        path: String,
    },
}

impl PatchOperation {
    /// Returns the JSON Pointer path targeted by this op (the destination
    /// path for [`MoveOp`](Self::MoveOp)).
    pub fn path(&self) -> &str {
        match self {
            PatchOperation::Add { path, .. }
            | PatchOperation::Set { path, .. }
            | PatchOperation::Replace { path, .. }
            | PatchOperation::Remove { path }
            | PatchOperation::Increment { path, .. }
            | PatchOperation::MoveOp { path, .. } => path,
        }
    }

    // -- Factory helpers --

    /// Builds an [`Add`](Self::Add) operation.
    pub fn add(path: impl Into<String>, value: Value) -> Self {
        PatchOperation::Add {
            path: path.into(),
            value,
        }
    }

    /// Builds a [`Set`](Self::Set) operation.
    pub fn set(path: impl Into<String>, value: Value) -> Self {
        PatchOperation::Set {
            path: path.into(),
            value,
        }
    }

    /// Builds a [`Replace`](Self::Replace) operation.
    pub fn replace(path: impl Into<String>, value: Value) -> Self {
        PatchOperation::Replace {
            path: path.into(),
            value,
        }
    }

    /// Builds a [`Remove`](Self::Remove) operation.
    pub fn remove(path: impl Into<String>) -> Self {
        PatchOperation::Remove { path: path.into() }
    }

    /// Builds an [`Increment`](Self::Increment) operation.
    pub fn increment(path: impl Into<String>, value: impl Into<CosmosNumber>) -> Self {
        PatchOperation::Increment {
            path: path.into(),
            value: value.into(),
        }
    }

    /// Builds a [`MoveOp`](Self::MoveOp) operation.
    pub fn move_op(from: impl Into<String>, path: impl Into<String>) -> Self {
        PatchOperation::MoveOp {
            from: from.into(),
            path: path.into(),
        }
    }
}

/// A PATCH document — the body of a `Patch` [`CosmosOperation`].
///
/// Wraps the ordered list of [`PatchOperation`]s to apply. PATCH is implemented
/// driver-side as a Read-Modify-Write loop guarded by an ETag precondition
/// (`If-Match`), which is managed entirely by the PATCH handler — callers do
/// not (and cannot) configure a precondition on this type.
///
/// A SQL filter predicate (peer SDKs' `FilterPredicate`) is **not** supported
/// in this preview. The peer SDKs place that knob on `PatchItemRequestOptions`,
/// not on the operation list, and implementing it correctly requires either
/// native wire-level PATCH (so the server evaluates the predicate inside the
/// same transaction) or a client-side SQL subset evaluator. Both are tracked
/// as follow-up work.
///
/// [`CosmosOperation`]: crate::models::CosmosOperation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct PatchDocument {
    /// Ordered list of operations.
    pub operations: Vec<PatchOperation>,
}

impl PatchDocument {
    /// Builds a [`PatchDocument`] from a list of operations.
    pub fn new(operations: Vec<PatchOperation>) -> Self {
        Self { operations }
    }

    /// Appends `operation` to the document's list of operations.
    pub fn with_operation(mut self, operation: PatchOperation) -> Self {
        self.operations.push(operation);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn patch_op_serializes_lowercase() {
        let op = PatchOperation::add("/a", json!(1));
        let s = serde_json::to_string(&op).unwrap();
        assert_eq!(s, r#"{"op":"add","path":"/a","value":1}"#);
    }

    #[test]
    fn move_op_serializes_as_move() {
        let op = PatchOperation::move_op("/a", "/b");
        let s = serde_json::to_string(&op).unwrap();
        assert_eq!(s, r#"{"op":"move","from":"/a","path":"/b"}"#);
    }

    #[test]
    fn increment_preserves_int_fidelity() {
        let op = PatchOperation::increment("/n", 9_000_000_000_000_001i64);
        let s = serde_json::to_string(&op).unwrap();
        assert!(s.contains("9000000000000001"), "actual: {s}");
        // No scientific-notation drift on the value.
        assert!(!s.contains("e+"), "actual: {s}");
        assert!(!s.contains("E+"), "actual: {s}");
    }

    /// Canonical wire JSON for the `PatchDocument` exercised by the
    /// serialize/deserialize tests below. Kept as a single source of
    /// truth so the two halves of the (former) round-trip test cannot
    /// drift apart silently. Matches `PATCH_HANDLER_SPEC.md` §"Wire
    /// format".
    const PATCH_SPEC_WIRE_JSON: &str = concat!(
        r#"{"operations":["#,
        r#"{"op":"set","path":"/age","value":31},"#,
        r#"{"op":"increment","path":"/visits","value":1},"#,
        r#"{"op":"add","path":"/tags/-","value":"rust"},"#,
        r#"{"op":"remove","path":"/legacy"},"#,
        r#"{"op":"move","from":"/from","path":"/to"}"#,
        r#"]}"#,
    );

    fn canonical_patch_spec() -> PatchDocument {
        PatchDocument::new(vec![
            PatchOperation::set("/age", json!(31)),
            PatchOperation::increment("/visits", 1i64),
            PatchOperation::add("/tags/-", json!("rust")),
            PatchOperation::remove("/legacy"),
            PatchOperation::move_op("/from", "/to"),
        ])
    }

    #[test]
    fn patch_spec_serializes_to_expected_json() {
        // Builds the PatchDocument, serializes it, and compares to a known
        // JSON string. This pins the wire format (key names, op tags,
        // field ordering for each PatchOperation variant) independently of the
        // Deserialize impl, so a regression in only one direction is
        // detectable.
        let s = serde_json::to_string(&canonical_patch_spec()).unwrap();
        assert_eq!(s, PATCH_SPEC_WIRE_JSON);
    }

    #[test]
    fn patch_spec_deserializes_from_known_json() {
        // Parses a known JSON string and asserts the resulting PatchDocument
        // matches the canonical value. This pins the wire-format -> model
        // direction independently of the Serialize impl.
        let parsed: PatchDocument = serde_json::from_str(PATCH_SPEC_WIRE_JSON).unwrap();
        assert_eq!(parsed, canonical_patch_spec());
    }

    #[test]
    fn patch_spec_does_not_serialize_condition_field() {
        // The SQL filter predicate (peer SDKs' `FilterPredicate`) is not part
        // of the public PATCH surface yet; serialization MUST NOT include a
        // `condition` key, and deserialization MUST refuse one (since the
        // struct is `#[non_exhaustive]` plus there is no `condition` field,
        // serde's default `deny_unknown_fields = false` would silently drop
        // an unknown field — verify the round-trip is condition-free).
        let spec = PatchDocument::new(vec![PatchOperation::set("/x", json!(1))]);
        let s = serde_json::to_string(&spec).unwrap();
        assert!(
            !s.contains("condition"),
            "PatchDocument serialization must not include a `condition` field: {s}"
        );
    }

    #[test]
    fn incr_value_int_and_float_deserialize() {
        let i: CosmosNumber = serde_json::from_str("3").unwrap();
        assert_eq!(i, CosmosNumber::Int(3));
        let f: CosmosNumber = serde_json::from_str("3.5").unwrap();
        assert_eq!(f, CosmosNumber::Float(3.5));
    }
}
