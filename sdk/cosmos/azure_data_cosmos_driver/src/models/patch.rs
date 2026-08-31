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
use std::{num::NonZeroU16, time::Duration};
use uuid::Uuid;

/// Maximum number of instructions accepted by one server-side PATCH request.
///
/// Client-side read-modify-write PATCH does not have this service limit.
pub(crate) const MAX_SERVER_SIDE_PATCH_OPERATIONS: usize = 10;

/// Reserved item property used to persist PATCH tracking entries.
pub const PATCH_TRACKING_PROPERTY: &str = "_azsdkPatchTracking";

/// Time PATCH tracking entries remain protected from age-based pruning.
///
/// A matching entry is honored for as long as it remains on the item, but a
/// later PATCH may prune it after this interval has elapsed or evict it earlier
/// when the marker array reaches capacity.
pub const PATCH_TRACKING_RETENTION: Duration = Duration::from_secs(5 * 60);

/// Default maximum number of idempotency markers retained on one item.
/// The oldest entry is evicted when this capacity is reached.
pub const DEFAULT_PATCH_TRACKING_CAPACITY: NonZeroU16 =
    NonZeroU16::new(1024).expect("default PATCH tracking capacity is non-zero");

/// Stable identity for an unsafe client-side PATCH operation.
///
/// Reuse the same ID when retrying the same logical operation across process
/// restarts. When no ID is supplied, the driver generates one for the current
/// invocation, which protects only retries made within that invocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchTrackingId(Uuid);

impl PatchTrackingId {
    /// Generates a new tracking ID from cryptographically secure OS entropy,
    /// falling back to the UUID crate's generator if OS entropy is unavailable.
    pub fn new() -> Self {
        let mut bytes = [0_u8; 16];
        if getrandom::fill(&mut bytes).is_err() {
            return Self(Uuid::new_v4());
        }
        // Mark the random bytes as an RFC 4122 variant, version 4 UUID.
        bytes[6] = (bytes[6] & 0x0f) | 0x40;
        bytes[8] = (bytes[8] & 0x3f) | 0x80;
        Self(Uuid::from_bytes(bytes))
    }

    /// Returns the underlying UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for PatchTrackingId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for PatchTrackingId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for PatchTrackingId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for PatchTrackingId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<Uuid>().map(Self)
    }
}

impl Serialize for PatchTrackingId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PatchTrackingId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

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
}

/// A set of instructions for a Cosmos DB PATCH operation, consisting of an ordered list of
/// [`PatchOperation`] values representing the individual operations to apply to an item.
///
/// Server-side PATCH accepts at most 10 operations. Lists with more than 10
/// operations require client-side PATCH execution.
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

    /// Returns whether applying these instructions more than once converges on
    /// the same document state.
    ///
    /// `Replace` and non-append `Set` operations assign a fixed value and are
    /// retry-safe. Every other operation is conservatively treated as unsafe:
    /// `Add` can insert into an array, `Increment` accumulates, and `Remove` /
    /// `Move` change their own preconditions after the first application.
    /// `Set` at the RFC 6901 `-` array token appends and is therefore unsafe.
    /// A list of otherwise-safe operations is also unsafe when one path is an
    /// ancestor of another: an earlier operation may no longer satisfy its
    /// preconditions after the complete list has committed.
    pub fn is_retry_safe(&self) -> bool {
        let operations_are_safe = self.operations.iter().all(|operation| match operation {
            PatchOperation::Replace { .. } => true,
            PatchOperation::Set { path, .. } => !path.ends_with("/-"),
            PatchOperation::Add { .. }
            | PatchOperation::Remove { .. }
            | PatchOperation::Increment { .. }
            | PatchOperation::Move { .. } => false,
        });
        if !operations_are_safe {
            return false;
        }

        let mut paths: Vec<&str> = self.operations.iter().map(PatchOperation::path).collect();
        // Segment ordering keeps each descendant range immediately after its
        // ancestor, so any strict ancestor pair appears in adjacent windows.
        paths.sort_unstable_by(|left, right| path_segments(left).cmp(path_segments(right)));
        !paths
            .windows(2)
            .any(|paths| path_is_strict_ancestor(paths[0], paths[1]))
    }
}

fn path_segments(path: &str) -> impl Iterator<Item = &str> {
    path.strip_prefix('/')
        .unwrap_or(path)
        .split('/')
        .skip(usize::from(path.is_empty()))
}

fn path_is_strict_ancestor(ancestor: &str, descendant: &str) -> bool {
    let mut descendant_segments = path_segments(descendant);
    for ancestor_segment in path_segments(ancestor) {
        if descendant_segments.next() != Some(ancestor_segment) {
            return false;
        }
    }
    descendant_segments.next().is_some()
}

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

    #[test]
    fn tracking_id_round_trips_as_uuid_string() {
        let id = PatchTrackingId::from(Uuid::from_u128(42));
        assert_eq!(id.to_string().parse::<PatchTrackingId>().unwrap(), id);
        assert_eq!(serde_json::to_string(&id).unwrap(), format!("\"{id}\""));
    }

    #[test]
    fn generated_tracking_id_is_rfc_4122_version_4() {
        let id = PatchTrackingId::new();
        let bytes = id.as_uuid().into_bytes();

        assert_eq!(bytes[6] & 0xf0, 0x40);
        assert_eq!(bytes[8] & 0xc0, 0x80);
    }

    #[test]
    fn retry_safety_is_conservative_for_non_convergent_operations() {
        let safe = PatchInstructions::from(vec![
            PatchOperation::set("/name", json!("updated")),
            PatchOperation::replace("/status", json!("complete")),
        ]);
        assert!(safe.is_retry_safe());

        for operation in [
            PatchOperation::add("/tags/0", json!("tag")),
            PatchOperation::set("/tags/-", json!("tag")),
            PatchOperation::remove("/obsolete"),
            PatchOperation::increment("/visits", 1i64),
            PatchOperation::move_value("/from", "/to"),
        ] {
            assert!(
                !PatchInstructions::from(vec![operation]).is_retry_safe(),
                "non-convergent operation must require tracking"
            );
        }

        let overlapping_paths = PatchInstructions::from(vec![
            PatchOperation::replace("/a/b", json!(1)),
            PatchOperation::set("/a", json!({})),
        ]);
        assert!(
            !overlapping_paths.is_retry_safe(),
            "an ancestor write can invalidate an earlier operation on replay"
        );

        let root_and_child = PatchInstructions::from(vec![
            PatchOperation::replace("/a", json!(1)),
            PatchOperation::set("", json!({})),
        ]);
        assert!(
            !root_and_child.is_retry_safe(),
            "the JSON Pointer root is an ancestor of every child path"
        );

        let lexically_interleaved = PatchInstructions::from(vec![
            PatchOperation::set("/a", json!(1)),
            PatchOperation::set("/a-", json!(2)),
            PatchOperation::set("/a/b", json!(3)),
        ]);
        assert!(
            !lexically_interleaved.is_retry_safe(),
            "segment sorting must keep descendants adjacent to their ancestor"
        );

        let duplicate_paths = PatchInstructions::from(vec![
            PatchOperation::set("a", json!(1)),
            PatchOperation::replace("/a", json!(2)),
        ]);
        assert!(
            duplicate_paths.is_retry_safe(),
            "equal normalized paths are not strict ancestors"
        );

        let long_safe_list = PatchInstructions::from(
            (0..20)
                .map(|index| PatchOperation::set(format!("/field{index}"), json!(index)))
                .collect::<Vec<_>>(),
        );
        assert!(
            long_safe_list.is_retry_safe(),
            "client-side PATCH supports instruction lists above the service limit"
        );
    }

    #[test]
    fn sorted_overlap_detection_matches_pairwise_reference() {
        fn reference_overlap(left: &str, right: &str) -> bool {
            if left != right && (left.is_empty() || right.is_empty()) {
                return true;
            }
            let normalize = |path: &str| {
                if path.starts_with('/') {
                    path.to_owned()
                } else {
                    format!("/{path}")
                }
            };
            let left = normalize(left);
            let right = normalize(right);
            left != right
                && (right.starts_with(&format!("{left}/"))
                    || left.starts_with(&format!("{right}/")))
        }

        let paths = [
            "", "/", "a", "/a", "/a-", "/a/b", "/a/b/c", "/a//b", "//", "//a", "/b", "/b/a", "/~1",
            "/a~1b",
        ];
        for first in paths {
            for second in paths {
                for third in paths {
                    let list = [first, second, third];
                    let expected_safe = !(0..list.len()).any(|left| {
                        (left + 1..list.len())
                            .any(|right| reference_overlap(list[left], list[right]))
                    });
                    let instructions = PatchInstructions::from(
                        list.into_iter()
                            .map(|path| PatchOperation::set(path, json!(1)))
                            .collect::<Vec<_>>(),
                    );
                    assert_eq!(
                        instructions.is_retry_safe(),
                        expected_safe,
                        "overlap result differed for {list:?}"
                    );
                }
            }
        }
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
