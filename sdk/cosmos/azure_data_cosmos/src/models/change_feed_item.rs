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
//! Callers bind `T = ChangeFeedItem<YourDoc>` when calling
//! [`ContainerClient::query_change_feed`](crate::clients::ContainerClient::query_change_feed).
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
use serde::Deserialize;

/// The type of change that produced a change feed item.
///
/// Parsed from the `operationType` field of the change feed metadata envelope
/// (`"create"`, `"replace"`, or `"delete"`).
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
    lsn: Option<i64>,

    /// The conflict resolution timestamp (`crts`) of the change, in seconds since
    /// the Unix epoch.
    #[serde(rename = "crts", default)]
    conflict_resolution_timestamp: Option<i64>,

    /// The LSN of the previous image of the item, when a pre-image is available
    /// (replace and delete operations on containers that retain pre-images).
    #[serde(rename = "previousImageLsn", default)]
    previous_image_lsn: Option<i64>,

    /// `Some(true)` when the change is a delete caused by the item's
    /// time-to-live (TTL) expiring, rather than an explicit delete.
    #[serde(rename = "timeToLiveExpired", default)]
    time_to_live_expired: Option<bool>,
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
    pub fn lsn(&self) -> Option<i64> {
        self.lsn
    }

    /// The conflict resolution timestamp (`crts`) of the change, in seconds since
    /// the Unix epoch, when reported by the service.
    pub fn conflict_resolution_timestamp(&self) -> Option<i64> {
        self.conflict_resolution_timestamp
    }

    /// The LSN of the previous image of the item, when a pre-image is available.
    pub fn previous_image_lsn(&self) -> Option<i64> {
        self.previous_image_lsn
    }

    /// `Some(true)` when the change is a delete caused by the item's TTL
    /// expiring rather than an explicit delete.
    pub fn time_to_live_expired(&self) -> Option<bool> {
        self.time_to_live_expired
    }
}

/// A single item from a Cosmos DB change feed.
///
/// Each item is a wire-format envelope describing one change: the document
/// after the change ([`current`](Self::current)), the document before the
/// change ([`previous`](Self::previous)), and the change
/// [`metadata`](Self::metadata). Bind `T = ChangeFeedItem<YourDoc>` when calling
/// [`ContainerClient::query_change_feed`](crate::clients::ContainerClient::query_change_feed);
/// the SDK does not strip the envelope, so the whole wire shape is preserved.
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
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
#[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
pub struct ChangeFeedItem<T> {
    /// The document after the change. Present for creates and replaces; for
    /// deletes it may be absent or a minimal document (id and partition key).
    #[serde(rename = "current", default)]
    current: Option<T>,

    /// The document before the change. Present for replaces and deletes when
    /// the container is configured to retain pre-images; otherwise absent.
    #[serde(rename = "previous", default)]
    previous: Option<T>,

    /// Metadata describing the change (operation type, LSN, timestamps).
    ///
    /// Populated for full-fidelity reads. For LatestVersion reads it may be
    /// absent, or a partial object carrying `lsn`/`crts` but no operation type.
    #[serde(rename = "metadata", default)]
    metadata: Option<ChangeFeedMetadata>,
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
        assert_eq!(metadata.lsn(), Some(100));
        assert_eq!(metadata.conflict_resolution_timestamp(), Some(1720322460));
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
    fn deserializes_replace_envelope_with_previous() {
        let envelope = json!({
            "current": { "id": "2", "value": 20 },
            "previous": { "id": "2", "value": 15 },
            "metadata": {
                "operationType": "replace",
                "lsn": 200,
                "crts": 1720322500,
                "previousImageLsn": 199
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
            Some(199)
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
        assert_eq!(metadata.lsn(), Some(400));
        assert!(metadata.time_to_live_expired().is_none());
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
        assert_eq!(metadata.lsn(), Some(100));
        assert_eq!(metadata.conflict_resolution_timestamp(), Some(1720322460));
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
}
