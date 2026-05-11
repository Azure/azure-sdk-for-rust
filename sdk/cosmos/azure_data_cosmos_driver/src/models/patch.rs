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
//! use azure_data_cosmos_driver::models::{PatchOp, PatchSpec};
//!
//! let spec = PatchSpec::new(vec![
//!     PatchOp::set("/age", serde_json::json!(31)),
//!     PatchOp::increment("/visits", 1i64),
//!     PatchOp::add("/tags/-", serde_json::json!("new-tag")),
//! ]);
//! let bytes = serde_json::to_vec(&spec).unwrap();
//! // The wire payload is a JSON object with an `operations` array; the
//! // first op above is a `set`.
//! assert!(bytes.starts_with(b"{"));
//! assert!(bytes.windows(5).any(|w| w == b"\"set\""));
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A typed numeric increment delta for [`PatchOp::Increment`].
///
/// Distinguishes integer increments (which preserve integer fidelity end-to-end)
/// from floating-point increments. Mixing an `Int(i64)` with a `f64` target —
/// or vice versa — is rejected by the patch evaluator at apply time rather than
/// silently demoting integer values to floating point.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum IncrValue {
    /// Integer increment. Preserves integer fidelity even when serialized into
    /// JSON, and refuses to merge with floating-point targets.
    Int(i64),
    /// Floating-point increment. Accepted on integer-valued numbers as well
    /// (the target is promoted to a JSON number with a fractional component).
    Float(f64),
}

impl From<i64> for IncrValue {
    fn from(v: i64) -> Self {
        IncrValue::Int(v)
    }
}

impl From<f64> for IncrValue {
    fn from(v: f64) -> Self {
        IncrValue::Float(v)
    }
}

impl Serialize for IncrValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            IncrValue::Int(n) => serializer.serialize_i64(*n),
            IncrValue::Float(n) => serializer.serialize_f64(*n),
        }
    }
}

impl<'de> Deserialize<'de> for IncrValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Number::deserialize(deserializer)?;
        if let Some(n) = value.as_i64() {
            Ok(IncrValue::Int(n))
        } else if let Some(n) = value.as_f64() {
            Ok(IncrValue::Float(n))
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
/// Both the enum variants and the equivalent factory functions (`PatchOp::add`,
/// `PatchOp::set`, ...) are part of the public API. The factories mirror the
/// .NET SDK's `PatchOperation.Add` / `.Set` / etc. methods and are the
/// recommended way to construct ops.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "lowercase")]
#[non_exhaustive]
pub enum PatchOp {
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
        value: IncrValue,
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

impl PatchOp {
    /// Returns the JSON Pointer path targeted by this op (the destination
    /// path for [`MoveOp`](Self::MoveOp)).
    pub fn path(&self) -> &str {
        match self {
            PatchOp::Add { path, .. }
            | PatchOp::Set { path, .. }
            | PatchOp::Replace { path, .. }
            | PatchOp::Remove { path }
            | PatchOp::Increment { path, .. }
            | PatchOp::MoveOp { path, .. } => path,
        }
    }

    // -- Factory helpers --

    /// Builds an [`Add`](Self::Add) operation.
    pub fn add(path: impl Into<String>, value: Value) -> Self {
        PatchOp::Add {
            path: path.into(),
            value,
        }
    }

    /// Builds a [`Set`](Self::Set) operation.
    pub fn set(path: impl Into<String>, value: Value) -> Self {
        PatchOp::Set {
            path: path.into(),
            value,
        }
    }

    /// Builds a [`Replace`](Self::Replace) operation.
    pub fn replace(path: impl Into<String>, value: Value) -> Self {
        PatchOp::Replace {
            path: path.into(),
            value,
        }
    }

    /// Builds a [`Remove`](Self::Remove) operation.
    pub fn remove(path: impl Into<String>) -> Self {
        PatchOp::Remove { path: path.into() }
    }

    /// Builds an [`Increment`](Self::Increment) operation.
    pub fn increment(path: impl Into<String>, value: impl Into<IncrValue>) -> Self {
        PatchOp::Increment {
            path: path.into(),
            value: value.into(),
        }
    }

    /// Builds a [`MoveOp`](Self::MoveOp) operation.
    pub fn move_op(from: impl Into<String>, path: impl Into<String>) -> Self {
        PatchOp::MoveOp {
            from: from.into(),
            path: path.into(),
        }
    }
}

/// A PATCH document — the body of a `Patch` [`CosmosOperation`].
///
/// Wraps the list of [`PatchOp`]s alongside an optional condition. The
/// condition is a Cosmos DB SQL fragment evaluated server-side as part of the
/// underlying Replace; callers typically use ETag preconditions instead, which
/// are managed by the PATCH handler internally.
///
/// [`CosmosOperation`]: crate::models::CosmosOperation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct PatchSpec {
    /// Optional SQL filter applied server-side after the patch operations are
    /// staged but before the Replace is committed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /// Ordered list of operations.
    pub operations: Vec<PatchOp>,
}

impl PatchSpec {
    /// Builds a [`PatchSpec`] from a list of operations.
    pub fn new(operations: Vec<PatchOp>) -> Self {
        Self {
            condition: None,
            operations,
        }
    }

    /// Builder-style setter for an optional condition.
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn patch_op_serializes_lowercase() {
        let op = PatchOp::add("/a", json!(1));
        let s = serde_json::to_string(&op).unwrap();
        assert_eq!(s, r#"{"op":"add","path":"/a","value":1}"#);
    }

    #[test]
    fn move_op_serializes_as_move() {
        let op = PatchOp::move_op("/a", "/b");
        let s = serde_json::to_string(&op).unwrap();
        assert_eq!(s, r#"{"op":"move","from":"/a","path":"/b"}"#);
    }

    #[test]
    fn increment_preserves_int_fidelity() {
        let op = PatchOp::increment("/n", 9_000_000_000_000_001i64);
        let s = serde_json::to_string(&op).unwrap();
        assert!(s.contains("9000000000000001"), "actual: {s}");
        // No scientific-notation drift on the value.
        assert!(!s.contains("e+"), "actual: {s}");
        assert!(!s.contains("E+"), "actual: {s}");
    }

    #[test]
    fn patch_spec_roundtrips() {
        let spec = PatchSpec::new(vec![
            PatchOp::set("/age", json!(31)),
            PatchOp::increment("/visits", 1i64),
            PatchOp::add("/tags/-", json!("rust")),
            PatchOp::remove("/legacy"),
            PatchOp::move_op("/from", "/to"),
        ])
        .with_condition("FROM c WHERE c.tier = 'gold'");
        let bytes = serde_json::to_vec(&spec).unwrap();
        let back: PatchSpec = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(back, spec);
    }

    #[test]
    fn incr_value_int_and_float_deserialize() {
        let i: IncrValue = serde_json::from_str("3").unwrap();
        assert_eq!(i, IncrValue::Int(3));
        let f: IncrValue = serde_json::from_str("3.5").unwrap();
        assert_eq!(f, IncrValue::Float(3.5));
    }
}
