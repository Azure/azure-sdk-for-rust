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
//! use azure_data_cosmos_driver::models::{PatchOperation, PatchInstructions};
//!
//! let spec = PatchInstructions::from(vec![
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
    ///
    /// Serializes as `"op": "incr"` — the wire tag Cosmos DB expects, which
    /// does not match the variant name. `"increment"` is still accepted on
    /// input so patch documents persisted by earlier versions of this crate
    /// keep deserializing.
    #[serde(rename = "incr", alias = "increment")]
    Increment {
        /// JSON Pointer path (RFC 6901) targeting an existing JSON number.
        path: String,
        /// Increment delta, preserving int/float fidelity.
        value: CosmosNumber,
    },
    /// Move (rename) the JSON value from `from` to `path`.
    Move {
        /// Source JSON Pointer path.
        from: String,
        /// Destination JSON Pointer path.
        path: String,
    },
}

impl PatchOperation {
    /// Returns the JSON Pointer path targeted by this op (the destination
    /// path for [`Move`](Self::Move)).
    pub fn path(&self) -> &str {
        match self {
            PatchOperation::Add { path, .. }
            | PatchOperation::Set { path, .. }
            | PatchOperation::Replace { path, .. }
            | PatchOperation::Remove { path }
            | PatchOperation::Increment { path, .. }
            | PatchOperation::Move { path, .. } => path,
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

    /// Builds a [`Move`](Self::Move) operation.
    pub fn move_value(from: impl Into<String>, path: impl Into<String>) -> Self {
        // 'move' is a keyword in Rust, so we can't use it as the function name.
        PatchOperation::Move {
            from: from.into(),
            path: path.into(),
        }
    }

    /// Returns `true` when re-applying this operation to an item it has already
    /// been applied to leaves both the item and the response status unchanged.
    ///
    /// This is the safety question for **server-side** patch. If a request fails
    /// in a way that leaves the outcome unknown, the driver may resend it; an
    /// operation that is not retry-safe would then be applied twice
    /// (`Increment`) or fail on the second pass (`Remove`, `Move`).
    ///
    /// Array positions are what make `Add` conditional: appending with `-` or
    /// inserting at an index shifts the remaining elements, so a resend inserts
    /// a second element. Adding to an object member is add-or-replace and is
    /// therefore safe.
    ///
    /// An ETag precondition makes even an unsafe list safe to send, because the
    /// resend fails with `412` instead of applying twice — so callers that
    /// supply one are not subject to this classification.
    pub fn is_retry_safe(&self) -> bool {
        match self {
            // Create-or-replace at an exact path; the second application is a no-op.
            PatchOperation::Set { .. } | PatchOperation::Replace { .. } => true,
            PatchOperation::Add { path, .. } => !targets_array_position(path),
            // Double-applies the delta.
            PatchOperation::Increment { .. } => false,
            // The path is gone on the second pass, so the resend errors; on an
            // array index it would delete a different element.
            PatchOperation::Remove { .. } => false,
            // `from` no longer exists on the second pass.
            PatchOperation::Move { .. } => false,
        }
    }
}

/// Returns `true` when the last JSON Pointer token of `path` addresses an array
/// position — either the append token `-` or a numeric index.
///
/// RFC 6901 escaping never produces a bare `-` or a bare run of digits for an
/// object key that isn't literally named that, so matching the raw token is
/// sufficient. A key that genuinely is named `"0"` is treated as an array index
/// and classified unsafe; that is the conservative direction.
fn targets_array_position(path: &str) -> bool {
    let last = path.rsplit('/').next().unwrap_or(path);
    last == "-" || (!last.is_empty() && last.bytes().all(|b| b.is_ascii_digit()))
}

/// A set of instructions for a Cosmos DB PATCH operation, consisting of an ordered list of
/// [`PatchOperation`] values representing the individual operations to apply to an item.
///
/// [`PatchOperation`]: crate::models::PatchOperation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct PatchInstructions {
    /// Ordered list of operations.
    pub operations: Vec<PatchOperation>,
}

impl PatchInstructions {
    /// Builds a [`PatchInstructions`] from a list of operations.
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Appends `operation` to the instruction set's list of operations.
    pub fn with_operation(mut self, operation: PatchOperation) -> Self {
        self.operations.push(operation);
        self
    }

    /// Returns `true` when every operation is retry-safe, so the whole set can
    /// be resent after an ambiguous failure without changing the outcome.
    ///
    /// An empty set is trivially safe. See [`PatchOperation::is_retry_safe`] for
    /// the per-operation rules.
    pub fn is_retry_safe(&self) -> bool {
        self.operations.iter().all(PatchOperation::is_retry_safe)
    }
}

/// Maximum number of operations the service accepts in a single-document patch.
///
/// A longer list is rejected with `400`, so [`PatchStrategy::Auto`] falls back
/// to the client-side loop rather than sending one.
///
/// [`PatchStrategy::Auto`]: crate::options::PatchStrategy::Auto
pub const MAX_SERVER_SIDE_PATCH_OPERATIONS: usize = 10;

impl From<Vec<PatchOperation>> for PatchInstructions {
    /// Builds a [`PatchInstructions`] from an existing list of operations.
    fn from(operations: Vec<PatchOperation>) -> Self {
        PatchInstructions { operations }
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

    /// Pins the `op` tag of every variant against the Cosmos DB wire contract.
    /// See <https://learn.microsoft.com/rest/api/cosmos-db/patch-a-document>.
    /// Note `Increment` is `incr` on the wire, not the lowercased variant name.
    #[test]
    fn every_op_tag_matches_the_wire_contract() {
        let cases = [
            (PatchOperation::add("/a", json!(1)), "add"),
            (PatchOperation::set("/a", json!(1)), "set"),
            (PatchOperation::replace("/a", json!(1)), "replace"),
            (PatchOperation::remove("/a"), "remove"),
            (PatchOperation::increment("/a", 1i64), "incr"),
            (PatchOperation::move_value("/a", "/b"), "move"),
        ];

        for (op, expected_tag) in cases {
            let value = serde_json::to_value(&op).unwrap();
            assert_eq!(
                value.get("op").and_then(serde_json::Value::as_str),
                Some(expected_tag),
                "unexpected wire tag for {op:?}"
            );
            // The tag must also round-trip back to the same variant.
            let parsed: PatchOperation = serde_json::from_value(value).unwrap();
            assert_eq!(parsed, op);
        }
    }

    /// `PatchOperation` is a public `Deserialize` type, so JSON this crate
    /// emitted before the `incr` fix must keep parsing. Only the input tag is
    /// accepted — output is always the wire-correct `incr`.
    #[test]
    fn legacy_increment_tag_still_deserializes() {
        let legacy = r#"{"op":"increment","path":"/visits","value":3}"#;
        let parsed: PatchOperation = serde_json::from_str(legacy).unwrap();
        assert_eq!(parsed, PatchOperation::increment("/visits", 3i64));

        // Re-serializing upgrades the tag rather than echoing the legacy one.
        let reserialized = serde_json::to_string(&parsed).unwrap();
        assert_eq!(reserialized, r#"{"op":"incr","path":"/visits","value":3}"#);
    }

    #[test]
    fn legacy_increment_tag_still_deserializes_inside_instructions() {
        let legacy = concat!(
            r#"{"operations":["#,
            r#"{"op":"set","path":"/age","value":31},"#,
            r#"{"op":"increment","path":"/visits","value":1}"#,
            r#"]}"#,
        );
        let parsed: PatchInstructions = serde_json::from_str(legacy).unwrap();
        assert_eq!(
            parsed,
            PatchInstructions::from(vec![
                PatchOperation::set("/age", json!(31)),
                PatchOperation::increment("/visits", 1i64),
            ])
        );
    }

    /// The float half of the alias: `CosmosNumber` picks its variant from the
    /// JSON number, so the legacy tag must not disturb int/float fidelity.
    #[test]
    fn legacy_increment_tag_preserves_float_fidelity() {
        let parsed: PatchOperation =
            serde_json::from_str(r#"{"op":"increment","path":"/ratio","value":2.5}"#).unwrap();
        assert_eq!(parsed, PatchOperation::increment("/ratio", 2.5f64));
    }

    /// Pins the retry-safety classification that decides whether `Auto` may run
    /// a patch server-side. Getting a `false` wrong only costs a round trip;
    /// getting a `true` wrong double-applies a customer's mutation.
    #[test]
    fn retry_safety_matches_the_classification_table() {
        let cases = [
            (PatchOperation::set("/a", json!(1)), true),
            (PatchOperation::replace("/a", json!(1)), true),
            (PatchOperation::add("/obj/member", json!(1)), true),
            (PatchOperation::add("/tags/-", json!(1)), false),
            (PatchOperation::add("/tags/0", json!(1)), false),
            (PatchOperation::add("/tags/12", json!(1)), false),
            (PatchOperation::remove("/a"), false),
            (PatchOperation::increment("/a", 1i64), false),
            (PatchOperation::increment("/a", 1.5f64), false),
            (PatchOperation::move_value("/a", "/b"), false),
        ];

        for (op, expected) in cases {
            assert_eq!(
                op.is_retry_safe(),
                expected,
                "wrong retry safety for {op:?}"
            );
        }
    }

    /// An array position is only an array position in the *last* token — a
    /// numeric segment further up the path is just a parent index.
    #[test]
    fn only_the_last_token_decides_array_targeting() {
        assert!(PatchOperation::set("/items/0/name", json!("x")).is_retry_safe());
        assert!(PatchOperation::add("/items/0/name", json!("x")).is_retry_safe());
        assert!(!PatchOperation::add("/items/0/tags/-", json!("x")).is_retry_safe());
    }

    #[test]
    fn instruction_set_is_safe_only_when_every_op_is() {
        assert!(PatchInstructions::new().is_retry_safe());
        assert!(PatchInstructions::from(vec![
            PatchOperation::set("/a", json!(1)),
            PatchOperation::replace("/b", json!(2)),
        ])
        .is_retry_safe());
        assert!(!PatchInstructions::from(vec![
            PatchOperation::set("/a", json!(1)),
            PatchOperation::increment("/b", 1i64),
        ])
        .is_retry_safe());
    }

    #[test]
    fn move_value_serializes_as_move() {
        let op = PatchOperation::move_value("/a", "/b");
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

    #[test]
    fn increment_deserializes_legacy_wire_name() {
        let parsed: PatchOperation = serde_json::from_value(json!({
            "op": "increment",
            "path": "/n",
            "value": 1
        }))
        .unwrap();
        assert_eq!(parsed, PatchOperation::increment("/n", 1i64));
    }

    /// Canonical wire JSON for the `PatchInstructions` exercised by the
    /// serialize/deserialize tests below. Kept as a single source of
    /// truth so the two halves of the (former) round-trip test cannot
    /// drift apart silently. Matches `PATCH_HANDLER_SPEC.md` §"Wire
    /// format".
    const PATCH_SPEC_WIRE_JSON: &str = concat!(
        r#"{"operations":["#,
        r#"{"op":"set","path":"/age","value":31},"#,
        r#"{"op":"incr","path":"/visits","value":1},"#,
        r#"{"op":"add","path":"/tags/-","value":"rust"},"#,
        r#"{"op":"remove","path":"/legacy"},"#,
        r#"{"op":"move","from":"/from","path":"/to"}"#,
        r#"]}"#,
    );

    fn canonical_patch_spec() -> PatchInstructions {
        PatchInstructions::from(vec![
            PatchOperation::set("/age", json!(31)),
            PatchOperation::increment("/visits", 1i64),
            PatchOperation::add("/tags/-", json!("rust")),
            PatchOperation::remove("/legacy"),
            PatchOperation::move_value("/from", "/to"),
        ])
    }

    #[test]
    fn patch_spec_serializes_to_expected_json() {
        // Builds the PatchInstructions, serializes it, and compares to a known
        // JSON string. This pins the wire format (key names, op tags,
        // field ordering for each PatchOperation variant) independently of the
        // Deserialize impl, so a regression in only one direction is
        // detectable.
        let s = serde_json::to_string(&canonical_patch_spec()).unwrap();
        assert_eq!(s, PATCH_SPEC_WIRE_JSON);
    }

    #[test]
    fn patch_spec_deserializes_from_known_json() {
        // Parses a known JSON string and asserts the resulting PatchInstructions
        // matches the canonical value. This pins the wire-format -> model
        // direction independently of the Serialize impl.
        let parsed: PatchInstructions = serde_json::from_str(PATCH_SPEC_WIRE_JSON).unwrap();
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
        let spec = PatchInstructions::from(vec![PatchOperation::set("/x", json!(1))]);
        let s = serde_json::to_string(&spec).unwrap();
        assert!(
            !s.contains("condition"),
            "PatchInstructions serialization must not include a `condition` field: {s}"
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
