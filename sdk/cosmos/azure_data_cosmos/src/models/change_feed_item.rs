// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Response types for the change feed wire-format envelope.
//!
//! Every change feed item is returned as a [`ChangeFeedItem<T>`] envelope. The
//! SDK always reads the change feed with the
//! `x-ms-cosmos-changefeed-wire-format-version` header set, so a conforming
//! service wraps each item as `{ "current": <document>, ... }` rather than
//! returning the bare document. The envelope carries the post-change document
//! (`current`), the pre-change document (`previous`, when the container is
//! configured to retain pre-images), and per-change [`ChangeFeedMetadata`].
//! Callers pass their own document type `D` to
//! [`ContainerClient::query_change_feed`](crate::clients::ContainerClient::query_change_feed),
//! which yields [`ChangeFeedItem<D>`] envelopes.
//!
//! For [`ChangeFeedMode::LatestVersion`](crate::options::ChangeFeedMode::LatestVersion)
//! reads the service surfaces the latest version of each created or replaced
//! item, so `current` is populated, `previous` is absent, and `metadata`
//! (when present) is partial — it may carry positional fields such as
//! `lsn`/`crts` but no operation type. The envelope also models `previous` and
//! full `metadata` because full-fidelity (all versions and deletes) reads
//! populate them; keeping every field optional lets a single type serve both
//! wire shapes without loss.

use azure_core::fmt::SafeDebug;
use serde::de::{DeserializeOwned, Error as _};
use serde::{Deserialize, Deserializer};
use std::time::Duration;

/// Deserializes an optional epoch-seconds integer into a [`Duration`].
fn deserialize_optional_duration_secs<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    let secs = Option::<i64>::deserialize(deserializer)?;
    Ok(secs.map(|secs| Duration::from_secs(secs.max(0) as u64)))
}

/// A logical sequence number (LSN) identifying a change's position within its
/// partition.
///
/// LSNs increase monotonically within a single physical partition and order the
/// changes in a feed. Read the underlying value with [`value`](Self::value), or
/// convert to and from `i64` via the standard `From`/`Into` conversions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct LogicalSequenceNumber(i64);

impl LogicalSequenceNumber {
    /// The underlying logical sequence number value.
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for LogicalSequenceNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl From<LogicalSequenceNumber> for i64 {
    fn from(value: LogicalSequenceNumber) -> Self {
        value.0
    }
}

/// The type of change that produced a change feed item.
///
/// Parsed from the `operationType` field of the change feed metadata envelope
/// (`"create"`, `"replace"`, or `"delete"`).
///
/// The wire format may add operation types in future service versions. An
/// unrecognized value deserializes to [`Unknown`](Self::Unknown) rather than
/// failing, so a single new operation type cannot fail the page it appears in
/// and permanently stall the feed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ChangeFeedOperationType {
    /// The item was created.
    Create,

    /// The item was replaced (updated).
    Replace,

    /// The item was deleted. For deletes, `current` may be absent or minimal
    /// (id and partition key only); the pre-image is available in `previous`
    /// when the container retains pre-images.
    Delete,

    /// An operation type not recognized by this SDK version.
    ///
    /// Any `operationType` value the SDK does not know maps here instead of
    /// failing deserialization, keeping the change feed alive across service
    /// upgrades that introduce new operation types.
    #[serde(other)]
    Unknown,
}

/// Per-change metadata returned with a change feed item.
///
/// Populated for full-fidelity (all versions and deletes) reads. For
/// [`ChangeFeedMode::LatestVersion`](crate::options::ChangeFeedMode::LatestVersion)
/// reads the service may omit the metadata envelope entirely, or return a
/// partial one that carries positional fields such as `lsn`/`crts` but no
/// `operationType`.
///
/// Every field is optional because the service only populates each one for the
/// operations and container configurations to which it applies.
#[derive(Clone, SafeDebug, Deserialize)]
#[safe(true)]
#[non_exhaustive]
pub struct ChangeFeedMetadata {
    /// The type of change (create, replace, or delete).
    ///
    /// Present for full-fidelity reads; absent for LatestVersion reads, whose
    /// metadata (when present) does not carry an operation type.
    #[serde(rename = "operationType", default)]
    operation_type: Option<ChangeFeedOperationType>,

    /// The logical sequence number (LSN) of the change within its partition.
    #[serde(rename = "lsn", default)]
    lsn: Option<LogicalSequenceNumber>,

    /// The conflict resolution timestamp (`crts`) of the change, measured since
    /// the Unix epoch.
    #[serde(
        rename = "crts",
        default,
        deserialize_with = "deserialize_optional_duration_secs"
    )]
    conflict_resolution_timestamp: Option<Duration>,

    /// The LSN of the previous image of the item, when a pre-image is available
    /// (replace and delete operations on containers that retain pre-images).
    #[serde(rename = "previousImageLSN", default)]
    previous_image_lsn: Option<LogicalSequenceNumber>,

    /// `Some(true)` when the change is a delete caused by the item's
    /// time-to-live (TTL) expiring, rather than an explicit delete.
    #[serde(rename = "timeToLiveExpired", default)]
    time_to_live_expired: Option<bool>,

    /// The id of the deleted item.
    ///
    /// Populated for delete operations in full-fidelity reads (a delete
    /// carries the removed item's identity here because `current` is empty);
    /// absent otherwise.
    #[serde(rename = "id", default)]
    id: Option<String>,

    /// The partition key of the deleted item, as returned on the wire.
    ///
    /// Populated for delete operations in full-fidelity reads; absent
    /// otherwise. Kept as the raw JSON value because the wire representation is
    /// an array of the partition key component values (one entry per level for
    /// a hierarchical partition key), which has no path-name context here to
    /// reshape into a map.
    #[serde(rename = "partitionKey", default)]
    partition_key: Option<serde_json::Value>,
}

impl ChangeFeedMetadata {
    /// The type of change (create, replace, or delete), when reported.
    ///
    /// Present for full-fidelity reads; `None` for LatestVersion reads, whose
    /// metadata does not carry an operation type.
    pub fn operation_type(&self) -> Option<ChangeFeedOperationType> {
        self.operation_type
    }

    /// The logical sequence number (LSN) of the change within its partition,
    /// when reported by the service.
    pub fn lsn(&self) -> Option<LogicalSequenceNumber> {
        self.lsn
    }

    /// The conflict resolution timestamp (`crts`) of the change, measured since
    /// the Unix epoch, when reported by the service.
    pub fn conflict_resolution_timestamp(&self) -> Option<Duration> {
        self.conflict_resolution_timestamp
    }

    /// The LSN of the previous image of the item, when a pre-image is available.
    pub fn previous_image_lsn(&self) -> Option<LogicalSequenceNumber> {
        self.previous_image_lsn
    }

    /// `Some(true)` when the change is a delete caused by the item's TTL
    /// expiring rather than an explicit delete.
    pub fn time_to_live_expired(&self) -> Option<bool> {
        self.time_to_live_expired
    }

    /// The id of the deleted item, when reported.
    ///
    /// Populated for delete operations in full-fidelity reads; `None`
    /// otherwise.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    /// The partition key of the deleted item, when reported.
    ///
    /// Populated for delete operations in full-fidelity reads; `None`
    /// otherwise. Returned as the raw JSON value (an array of the partition
    /// key component values, one per level for a hierarchical partition key).
    pub fn partition_key(&self) -> Option<&serde_json::Value> {
        self.partition_key.as_ref()
    }
}

/// A single item from a Cosmos DB change feed.
///
/// Each item is an envelope describing one change: the document
/// after the change ([`current`](Self::current)), the document before the
/// change ([`previous`](Self::previous)), and the change
/// [`metadata`](Self::metadata). Pass your document type `T` to
/// [`ContainerClient::query_change_feed`](crate::clients::ContainerClient::query_change_feed);
/// it yields `ChangeFeedItem<T>` and does not strip the envelope.
///
/// For [`ChangeFeedMode::LatestVersion`](crate::options::ChangeFeedMode::LatestVersion)
/// reads [`current`](Self::current) holds the latest version of each created or
/// replaced document; [`previous`](Self::previous) is absent and
/// [`metadata`](Self::metadata) is either absent or partial (no operation
/// type). Full-fidelity (all versions and deletes) reads additionally populate
/// [`metadata`](Self::metadata) and, for
/// replaces and deletes on containers that retain pre-images,
/// [`previous`](Self::previous).
///
/// # Caveat for deletes
///
/// For delete operations, `current` may be absent or contain only a minimal
/// document (id and partition key). Your document type `T` should therefore
/// tolerate missing fields (e.g. wrap fields in `Option` or provide
/// `#[serde(default)]`) so a delete envelope still deserializes. The pre-image,
/// when retained, is available in `previous`.
///
/// Like [`FeedPage<T>`](crate::feed::FeedPage), this type derives a standard
/// [`Debug`] rather than `SafeDebug`: it is a generic envelope around the
/// caller's own document `T`, so its `Debug` output is only available when `T`
/// itself is `Debug`.
///
/// # Non-enveloped responses
///
/// A backend that returns the document without the change-feed envelope (for
/// example the Cosmos emulator, which does not produce it) yields the bare
/// document. In that case the whole document is read as
/// [`current`](Self::current), with no [`previous`](Self::previous) or
/// [`metadata`](Self::metadata), so `.current()` still returns it.
///
/// An item is treated as an envelope only when it is a non-empty object whose
/// keys are drawn entirely from `current`, `previous`, and `metadata`; any
/// other key marks it as a bare document. The sole ambiguous case is a
/// document whose top level consists only of those reserved names.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ChangeFeedItem<T> {
    /// The document after the change. Present for creates and replaces; for
    /// deletes it may be absent or a minimal document (id and partition key).
    current: Option<T>,

    /// The document before the change. Present for replaces and deletes when
    /// the container is configured to retain pre-images; otherwise absent.
    previous: Option<T>,

    /// Metadata describing the change (operation type, LSN, timestamps).
    ///
    /// Populated for full-fidelity reads. For LatestVersion reads it may be
    /// absent, or a partial object carrying `lsn`/`crts` but no operation type.
    metadata: Option<ChangeFeedMetadata>,
}

impl<'de, T> Deserialize<'de> for ChangeFeedItem<T>
where
    T: DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // An enveloped item is a JSON object whose top-level keys are drawn
        // entirely from the reserved set {current, previous, metadata}. A
        // non-enveloped (flat) document written by a caller almost always
        // carries other keys too — its id, partition key, and fields — so
        // requiring *every* key to be reserved keeps a flat document that
        // merely includes a `metadata`/`previous`/`current` field from being
        // misread as an (empty) envelope with its real contents dropped.
        // Detecting `previous`/`metadata` (not just `current`) still lets a
        // full-fidelity delete envelope, which has no `current`, be recognized.
        // See the type-level docs for the sole reserved-key caveat.
        //
        // Buffering into `Value` first is what lets us inspect the shape at
        // runtime and tolerate a non-enveloping backend (the vnext emulator
        // does not honor the wire-format header). It costs one extra DOM parse
        // per item. Once non-enveloping backends are no longer supported this
        // whole impl collapses to a plain `#[derive(Deserialize)]` with
        // `Option` fields, removing the buffering and the double-parse.
        let value = serde_json::Value::deserialize(deserializer)?;
        let is_envelope = value.as_object().is_some_and(|fields| {
            !fields.is_empty()
                && fields
                    .keys()
                    .all(|key| matches!(key.as_str(), "current" | "previous" | "metadata"))
        });

        if is_envelope {
            #[derive(Deserialize)]
            struct Envelope {
                current: Option<serde_json::Value>,
                previous: Option<serde_json::Value>,
                metadata: Option<ChangeFeedMetadata>,
            }

            // For deletes the service returns an empty `current` object (and,
            // for pre-image containers, the removed document in `previous`).
            // Treat an empty or null object as absent so callers with strict
            // document types don't fail to deserialize a delete.
            fn document<T: DeserializeOwned>(
                value: Option<serde_json::Value>,
            ) -> Result<Option<T>, serde_json::Error> {
                match value {
                    None | Some(serde_json::Value::Null) => Ok(None),
                    Some(serde_json::Value::Object(map)) if map.is_empty() => Ok(None),
                    Some(other) => serde_json::from_value(other).map(Some),
                }
            }

            let Envelope {
                current,
                previous,
                metadata,
            } = serde_json::from_value(value).map_err(D::Error::custom)?;
            Ok(ChangeFeedItem {
                current: document(current).map_err(D::Error::custom)?,
                previous: document(previous).map_err(D::Error::custom)?,
                metadata,
            })
        } else {
            let current = serde_json::from_value(value).map_err(D::Error::custom)?;
            Ok(ChangeFeedItem {
                current: Some(current),
                previous: None,
                metadata: None,
            })
        }
    }
}

impl<T> ChangeFeedItem<T> {
    /// The document after the change, if present.
    ///
    /// Present for creates and replaces; for deletes it may be absent or a
    /// minimal document (id and partition key only).
    pub fn current(&self) -> Option<&T> {
        self.current.as_ref()
    }

    /// The document before the change, if a pre-image is available.
    ///
    /// Present for replaces and deletes when the container retains pre-images.
    pub fn previous(&self) -> Option<&T> {
        self.previous.as_ref()
    }

    /// The metadata describing this change, when reported.
    ///
    /// Populated for full-fidelity reads. For LatestVersion reads it may be
    /// absent, or a partial object carrying `lsn`/`crts` but no operation type.
    pub fn metadata(&self) -> Option<&ChangeFeedMetadata> {
        self.metadata.as_ref()
    }

    /// The type of change (create, replace, or delete), when metadata is
    /// present.
    ///
    /// Convenience accessor that delegates to
    /// [`metadata().operation_type()`](ChangeFeedMetadata::operation_type).
    /// Returns `None` when no metadata is reported (e.g. LatestVersion reads).
    pub fn operation_type(&self) -> Option<ChangeFeedOperationType> {
        self.metadata
            .as_ref()
            .and_then(ChangeFeedMetadata::operation_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use serde_json::json;

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    struct Doc {
        id: String,
        #[serde(default)]
        value: Option<i64>,
    }

    #[test]
    fn deserializes_create_envelope() {
        let envelope = json!({
            "current": { "id": "1", "value": 10 },
            "metadata": {
                "operationType": "create",
                "lsn": 100,
                "crts": 1720322460
            }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(item.operation_type(), Some(ChangeFeedOperationType::Create));
        assert_eq!(
            item.current(),
            Some(&Doc {
                id: "1".into(),
                value: Some(10)
            })
        );
        assert!(item.previous().is_none());
        let metadata = item.metadata().expect("metadata should be present");
        assert_eq!(metadata.lsn(), Some(LogicalSequenceNumber::from(100)));
        assert_eq!(
            metadata.conflict_resolution_timestamp(),
            Some(Duration::from_secs(1720322460))
        );
        assert!(metadata.previous_image_lsn().is_none());
        assert!(metadata.time_to_live_expired().is_none());
    }

    #[test]
    fn deserializes_latest_version_envelope_without_metadata() {
        // LatestVersion reads envelope only the post-change document under
        // `current` — there is no `metadata` and no `previous`.
        let envelope = json!({ "current": { "id": "1", "value": 42 } });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(
            item.current(),
            Some(&Doc {
                id: "1".into(),
                value: Some(42)
            })
        );
        assert!(item.previous().is_none());
        assert!(item.metadata().is_none());
        assert!(item.operation_type().is_none());
    }

    #[test]
    fn deserializes_flat_non_enveloped_document() {
        // A backend that does not honor the wire-format header (an older
        // gateway or an emulator without change-feed enveloping) returns the
        // bare document with no envelope keys. The whole document must map onto
        // `current` so the caller still reads it via `.current()`.
        let flat = json!({ "id": "9", "value": 99 });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(flat).unwrap();

        assert_eq!(
            item.current(),
            Some(&Doc {
                id: "9".into(),
                value: Some(99)
            })
        );
        assert!(item.previous().is_none());
        assert!(item.metadata().is_none());
        assert!(item.operation_type().is_none());
    }

    #[test]
    fn flat_document_without_optional_fields_still_deserializes() {
        // The bare document need not carry every field; only what `T` requires.
        let flat = json!({ "id": "10" });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(flat).unwrap();

        assert_eq!(
            item.current(),
            Some(&Doc {
                id: "10".into(),
                value: None
            })
        );
        assert!(item.previous().is_none());
        assert!(item.metadata().is_none());
    }

    #[test]
    fn flat_document_with_reserved_field_name_is_not_misread_as_envelope() {
        // Regression guard for the non-enveloped path: a flat document that
        // happens to carry a top-level `metadata` (or `previous`) field must
        // still be read as the document. Because it also carries its own keys
        // (`id`/`value`), not every key is reserved, so it is a flat document —
        // its contents must survive rather than being dropped for an empty
        // envelope.
        let flat = json!({
            "id": "42",
            "value": 7,
            "metadata": { "author": "bob" }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(flat).unwrap();

        assert_eq!(
            item.current(),
            Some(&Doc {
                id: "42".into(),
                value: Some(7)
            })
        );
        assert!(item.previous().is_none());
        assert!(item.metadata().is_none());

        // The same holds for a stray top-level `previous` field.
        let flat = json!({ "id": "43", "previous": "unrelated" });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(flat).unwrap();
        assert_eq!(item.current().map(|d| d.id.as_str()), Some("43"));
        assert!(item.previous().is_none());
    }

    #[test]
    fn delete_envelope_with_only_metadata_is_treated_as_envelope() {
        // Counterpart to the flat-document guard: an object whose keys are all
        // reserved is an envelope even when `metadata` is the only key, so a
        // full-fidelity delete that carries neither `current` nor `previous`
        // still surfaces its metadata rather than being parsed as a document.
        let envelope = json!({ "metadata": { "operationType": "delete", "lsn": 400 } });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert!(item.current().is_none());
        assert!(item.previous().is_none());
        assert_eq!(item.operation_type(), Some(ChangeFeedOperationType::Delete));
    }

    #[test]
    fn delete_envelope_without_current_is_treated_as_envelope() {
        // A full-fidelity delete envelope carries `previous`/`metadata` but no
        // `current`; it must be recognized as an envelope (not mistaken for a
        // flat document) so `previous` and `metadata` survive.
        let envelope = json!({
            "previous": { "id": "3", "value": 30 },
            "metadata": { "operationType": "delete", "lsn": 300 }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert!(item.current().is_none());
        assert_eq!(item.previous().map(|d| d.id.as_str()), Some("3"));
        assert_eq!(item.operation_type(), Some(ChangeFeedOperationType::Delete));
    }

    #[test]
    fn deserializes_replace_envelope_with_previous() {
        let envelope = json!({
            "current": { "id": "2", "value": 20 },
            "previous": { "id": "2", "value": 15 },
            "metadata": {
                "operationType": "replace",
                "lsn": 200,
                "crts": 1720322500,
                "previousImageLSN": 199
            }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(
            item.operation_type(),
            Some(ChangeFeedOperationType::Replace)
        );
        assert_eq!(item.current().and_then(|d| d.value), Some(20));
        assert_eq!(item.previous().and_then(|d| d.value), Some(15));
        assert_eq!(
            item.metadata()
                .and_then(ChangeFeedMetadata::previous_image_lsn),
            Some(LogicalSequenceNumber::from(199))
        );
    }

    #[test]
    fn deserializes_delete_envelope_with_previous_and_ttl() {
        let envelope = json!({
            "previous": { "id": "3", "value": 30 },
            "metadata": {
                "operationType": "delete",
                "lsn": 300,
                "timeToLiveExpired": true
            }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(item.operation_type(), Some(ChangeFeedOperationType::Delete));
        // `current` is absent for this delete envelope.
        assert!(item.current().is_none());
        assert_eq!(item.previous().map(|d| d.id.as_str()), Some("3"));
        assert_eq!(
            item.metadata()
                .and_then(ChangeFeedMetadata::time_to_live_expired),
            Some(true)
        );
    }

    #[test]
    fn deserializes_delete_envelope_without_previous() {
        // A delete with neither a post-image nor a retained pre-image: only the
        // metadata is present. The item must still deserialize.
        let envelope = json!({
            "metadata": {
                "operationType": "delete",
                "lsn": 400
            }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(item.operation_type(), Some(ChangeFeedOperationType::Delete));
        assert!(item.current().is_none());
        assert!(item.previous().is_none());
        let metadata = item.metadata().expect("metadata should be present");
        assert_eq!(metadata.lsn(), Some(LogicalSequenceNumber::from(400)));
        assert!(metadata.time_to_live_expired().is_none());
    }

    #[test]
    fn deserializes_delete_envelope_with_empty_current() {
        // A full-fidelity delete returns an empty `current` object with the
        // deleted item's identity carried in `metadata`. The empty object must
        // map to `None` so callers with strict document types (a required `id`
        // here) can still deserialize the delete instead of failing on `{}`.
        let envelope = json!({
            "current": {},
            "metadata": {
                "operationType": "delete",
                "id": "item-1",
                "partitionKey": ["tenant-a"]
            }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(item.operation_type(), Some(ChangeFeedOperationType::Delete));
        assert!(item.current().is_none());
        assert!(item.previous().is_none());
        let metadata = item.metadata().expect("metadata should be present");
        // The deleted item's identity is surfaced from the metadata.
        assert_eq!(metadata.id(), Some("item-1"));
        assert_eq!(metadata.partition_key(), Some(&json!(["tenant-a"])));
    }

    #[test]
    fn deserializes_latest_version_envelope_with_partial_metadata() {
        // Against the real service a LatestVersion read can return a metadata
        // object that carries positional fields (lsn/crts) but no
        // `operationType`. The item must still deserialize (regression: a
        // required `operationType` previously failed these responses).
        let envelope = json!({
            "current": { "id": "1", "value": 7 },
            "metadata": {
                "lsn": 100,
                "crts": 1720322460
            }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();

        assert_eq!(item.current().and_then(|d| d.value), Some(7));
        assert!(item.previous().is_none());
        let metadata = item.metadata().expect("metadata should be present");
        assert!(metadata.operation_type().is_none());
        assert!(item.operation_type().is_none());
        assert_eq!(metadata.lsn(), Some(LogicalSequenceNumber::from(100)));
        assert_eq!(
            metadata.conflict_resolution_timestamp(),
            Some(Duration::from_secs(1720322460))
        );
    }

    #[test]
    fn operation_type_parses_all_variants() {
        for (wire, expected) in [
            ("create", ChangeFeedOperationType::Create),
            ("replace", ChangeFeedOperationType::Replace),
            ("delete", ChangeFeedOperationType::Delete),
        ] {
            let parsed: ChangeFeedOperationType = serde_json::from_value(json!(wire)).unwrap();
            assert_eq!(parsed, expected);
        }
    }

    #[test]
    fn unknown_operation_type_maps_to_unknown_variant() {
        // A future/unknown `operationType` must not fail deserialization: it
        // maps to `Unknown` so one new value cannot fail the whole page and
        // permanently stall the feed.
        let parsed: ChangeFeedOperationType = serde_json::from_value(json!("resurrect")).unwrap();
        assert_eq!(parsed, ChangeFeedOperationType::Unknown);

        // The same must hold when it arrives inside a full envelope.
        let envelope = json!({
            "current": { "id": "1", "value": 1 },
            "metadata": { "operationType": "resurrect", "lsn": 500 }
        });
        let item: ChangeFeedItem<Doc> = serde_json::from_value(envelope).unwrap();
        assert_eq!(
            item.operation_type(),
            Some(ChangeFeedOperationType::Unknown)
        );
        assert_eq!(item.current().and_then(|d| d.value), Some(1));
    }
}
