// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Prototype: Apache Arrow IPC decoding for the flat `list_blobs` response.
//!
//! This is wired through a custom [`Format`] ([`ArrowXmlFormat`]) so the generated
//! pager and [`Response::into_model`](azure_core::http::Response::into_model)
//! transparently dispatch between the Apache Arrow stream and XML based on the
//! response `Content-Type` header — no hand-written response wrapper or pager needed.
//!
//! All scalar, timestamp, and enum columns of the flat response that have a
//! corresponding field on [`BlobItem`] / [`BlobProperties`] are mapped, as are the
//! map-typed columns (`Tags`, `Metadata`, `OrMetadata`). Columns without a model field
//! (`ResourceType`, `Content-CRC64`, `SmartAccessTier`, `OrsPolicySourceBlob`,
//! `AffinityId`) are not yet mapped. Envelope fields (container name, prefix, marker,
//! max results, service endpoint) are not carried as Arrow columns and are left unset.

use crate::models::{
    BlobItem, BlobMetadata, BlobProperties, BlobTag, BlobTags, ListBlobsResponse,
    ObjectReplicationMetadata,
};
use arrow_array::{
    Array, BooleanArray, Int32Array, Int64Array, MapArray, RecordBatch, StringArray,
    TimestampMicrosecondArray, TimestampMillisecondArray, TimestampNanosecondArray,
    TimestampSecondArray, UInt32Array, UInt64Array,
};
use arrow_ipc::reader::StreamReader;
use arrow_schema::{ArrowError, DataType, TimeUnit};
use azure_core::{
    base64,
    error::{Error, ErrorKind},
    http::{
        headers::{self, Headers},
        response::ResponseBody,
        DeserializeWith, Etag, Format, RawResponse,
    },
    time::OffsetDateTime,
    Result,
};
use serde::{de::DeserializeOwned, Deserialize};

/// Metadata key on the Arrow schema holding the continuation token.
const NEXT_MARKER_KEY: &str = "NextMarker";
const ARROW_CONTENT_TYPE: &str = "application/vnd.apache.arrow.stream";
const XML_CONTENT_TYPE: &str = "application/xml";

#[derive(Clone, Copy)]
enum ListBlobsWireFormat {
    Arrow,
    Xml,
}

// HANDMADE: custom `Format` that gives `list_blobs` runtime Arrow-or-XML dispatch.
//
// This is the entire point of pulling in Core PR: `Response::into_model` now
// routes through `Format::deserialize_from(&RawResponse)`, which has access to the
// response headers. That lets a service crate own its own format dispatch without any
// hand-written response wrapper, and without `azure_core` needing to know about Arrow.
/// A [`Format`] that decodes the flat `list_blobs` response as an Apache Arrow stream
/// when the response `Content-Type` advertises Arrow, and falls back to XML otherwise.
#[derive(Debug, Clone)]
pub struct ArrowXmlFormat;

impl Format for ArrowXmlFormat {
    /// Bytes-only fallback used when no headers are available. Without the
    /// `Content-Type` header we cannot detect Arrow, so we assume XML.
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> Result<T> {
        azure_core::xml::from_xml(body.as_ref())
    }

    /// Header-aware dispatch. Inspects `Content-Type` and decodes the body as Arrow
    /// or XML accordingly.
    fn deserialize_from<T: DeserializeOwned>(response: &RawResponse) -> Result<T> {
        match wire_format(response.headers())? {
            ListBlobsWireFormat::Xml => azure_core::xml::from_xml(response.body()),
            ListBlobsWireFormat::Arrow => {
                // [Core Support]: `Format::deserialize_from<T>` is generic over `T` and
                // therefore assumes a serde-based format. The Arrow decoder is type-specific
                // — it hand-maps Arrow columns into a concrete `ListBlobsResponse` and cannot
                // produce an arbitrary `T`.

                // For this PR to currently work (not ideal, not production-ready):
                // to satisfy the generic bound we bridge with a serde
                // round-trip (Arrow -> ListBlobsResponse -> JSON -> T)
                let model = decode_arrow_list_blobs(response.body())?;
                let json = azure_core::json::to_json(&model)?;
                azure_core::json::from_json(json)
            }
        }
    }
}

// HANDMADE / NEEDS CORE SUPPORT: `Response<T, F>::into_model` is bounded on
// `T: DeserializeWith<F>`, so `Response<ListBlobsResponse, ArrowXmlFormat>` requires this
// impl to exist even though `into_model` now dispatches through `Format::deserialize_from`
// and never calls `deserialize_with`. The bytes-only body here mirrors the XML fallback.
// Core could drop this redundant requirement once `into_model` no longer depends on the
// `DeserializeWith` bound for header-aware formats.
impl DeserializeWith<ArrowXmlFormat> for ListBlobsResponse {
    fn deserialize_with(body: ResponseBody) -> azure_core::Result<Self> {
        body.xml()
    }
}

/// Extracts the pagination continuation token from a `list_blobs` response,
/// dispatching on the wire format. Used by the generated pager.
pub(crate) fn decode_next_marker(headers: &Headers, bytes: &[u8]) -> Result<Option<String>> {
    match wire_format(headers)? {
        ListBlobsWireFormat::Arrow => arrow_next_marker(bytes),
        ListBlobsWireFormat::Xml => {
            #[derive(Deserialize)]
            struct ListBlobsPage {
                #[serde(rename = "NextMarker")]
                next_marker: Option<String>,
            }

            let page: ListBlobsPage = azure_core::xml::from_xml(bytes)?;
            Ok(page.next_marker.filter(|marker| !marker.is_empty()))
        }
    }
}

fn wire_format(headers: &Headers) -> Result<ListBlobsWireFormat> {
    let content_type = headers
        .get_optional_str(&headers::CONTENT_TYPE)
        .ok_or_else(|| {
            Error::with_message(
                ErrorKind::DataConversion,
                "list blobs response did not include Content-Type",
            )
        })?;
    let media_type = content_type.split(';').next().unwrap_or_default().trim();

    if media_type.eq_ignore_ascii_case(ARROW_CONTENT_TYPE) {
        Ok(ListBlobsWireFormat::Arrow)
    } else if media_type.eq_ignore_ascii_case(XML_CONTENT_TYPE) {
        Ok(ListBlobsWireFormat::Xml)
    } else {
        Err(Error::with_message(
            ErrorKind::DataConversion,
            format!("unsupported list blobs Content-Type: {content_type}"),
        ))
    }
}

fn arrow_next_marker(bytes: &[u8]) -> Result<Option<String>> {
    let reader = StreamReader::try_new(bytes, None).map_err(to_error)?;
    Ok(reader
        .schema()
        .metadata()
        .get(NEXT_MARKER_KEY)
        .filter(|marker| !marker.is_empty())
        .cloned())
}

/// Decodes an Apache Arrow IPC stream (`application/vnd.apache.arrow.stream`)
/// returned by the flat `list_blobs` API into a [`ListBlobsResponse`].
pub(crate) fn decode_arrow_list_blobs(bytes: &[u8]) -> Result<ListBlobsResponse> {
    let reader = StreamReader::try_new(bytes, None).map_err(to_error)?;

    // The continuation token lives in schema-level metadata, not a column.
    let next_marker = reader
        .schema()
        .metadata()
        .get(NEXT_MARKER_KEY)
        .filter(|marker| !marker.is_empty())
        .cloned();

    let mut blob_items = Vec::new();
    for batch in reader {
        let batch = batch.map_err(to_error)?;
        for row in 0..batch.num_rows() {
            blob_items.push(row_to_blob_item(&batch, row));
        }
    }

    Ok(ListBlobsResponse {
        blob_items,
        next_marker,
        ..Default::default()
    })
}

fn row_to_blob_item(batch: &RecordBatch, row: usize) -> BlobItem {
    let properties = BlobProperties {
        access_tier: string_at(batch, "AccessTier", row).and_then(|s| s.parse().ok()),
        access_tier_change_time: timestamp_at(batch, "AccessTierChangeTime", row),
        access_tier_inferred: bool_at(batch, "AccessTierInferred", row),
        archive_status: string_at(batch, "ArchiveStatus", row).and_then(|s| s.parse().ok()),
        blob_sequence_number: i64_at(batch, "x-ms-blob-sequence-number", row),
        blob_type: string_at(batch, "BlobType", row).and_then(|s| s.parse().ok()),
        cache_control: string_at(batch, "Cache-Control", row),
        content_disposition: string_at(batch, "Content-Disposition", row),
        content_encoding: string_at(batch, "Content-Encoding", row),
        content_language: string_at(batch, "Content-Language", row),
        content_length: u64_at(batch, "Content-Length", row),
        content_md5: string_at(batch, "Content-MD5", row).and_then(|s| base64::decode(s).ok()),
        content_type: string_at(batch, "Content-Type", row),
        copy_completion_time: timestamp_at(batch, "CopyCompletionTime", row),
        copy_id: string_at(batch, "CopyId", row),
        copy_progress: string_at(batch, "CopyProgress", row),
        copy_source: string_at(batch, "CopySource", row),
        copy_status: string_at(batch, "CopyStatus", row).and_then(|s| s.parse().ok()),
        copy_status_description: string_at(batch, "CopyStatusDescription", row),
        creation_time: timestamp_at(batch, "Creation-Time", row),
        deleted_time: timestamp_at(batch, "DeletedTime", row),
        destination_snapshot: string_at(batch, "CopyDestinationSnapshot", row),
        encryption_key_sha256: string_at(batch, "CustomerProvidedKeySha256", row),
        encryption_scope: string_at(batch, "EncryptionScope", row),
        etag: string_at(batch, "Etag", row).map(Etag::from),
        immutability_policy_expires_on: timestamp_at(batch, "ImmutabilityPolicyUntilDate", row),
        immutability_policy_mode: string_at(batch, "ImmutabilityPolicyMode", row)
            .and_then(|s| s.parse().ok()),
        incremental_copy: bool_at(batch, "IncrementalCopy", row),
        is_sealed: bool_at(batch, "Sealed", row),
        last_accessed_on: timestamp_at(batch, "LastAccessTime", row),
        last_modified: timestamp_at(batch, "Last-Modified", row),
        lease_duration: string_at(batch, "LeaseDuration", row).and_then(|s| s.parse().ok()),
        lease_state: string_at(batch, "LeaseState", row).and_then(|s| s.parse().ok()),
        lease_status: string_at(batch, "LeaseStatus", row).and_then(|s| s.parse().ok()),
        legal_hold: bool_at(batch, "LegalHold", row),
        rehydrate_priority: string_at(batch, "RehydratePriority", row).and_then(|s| s.parse().ok()),
        remaining_retention_days: i32_at(batch, "RemainingRetentionDays", row),
        server_encrypted: bool_at(batch, "ServerEncrypted", row),
        tag_count: i32_at(batch, "TagCount", row),
        ..Default::default()
    };

    BlobItem {
        name: string_at(batch, "Name", row),
        deleted: bool_at(batch, "Deleted", row),
        has_versions_only: bool_at(batch, "HasVersionsOnly", row),
        is_current_version: bool_at(batch, "IsCurrentVersion", row),
        snapshot: string_at(batch, "Snapshot", row),
        version_id: string_at(batch, "VersionId", row),
        // [Phase 2] Arrow `map<utf8, utf8>` columns -> model map types.
        blob_tags: blob_tags_at(batch, "Tags", row),
        metadata: blob_metadata_at(batch, "Metadata", row),
        object_replication_metadata: or_metadata_at(batch, "OrMetadata", row),
        properties: Some(properties),
        ..Default::default()
    }
}

// [Phase 2] Reads an Arrow `map<utf8, utf8>` column into ordered key/value pairs for one
// row. Returns `None` when the column is absent, the cell is null, or the map is empty.
fn map_entries_at(batch: &RecordBatch, name: &str, row: usize) -> Option<Vec<(String, String)>> {
    let array = column(batch, name)?;
    if array.is_null(row) {
        return None;
    }
    let map = array.as_any().downcast_ref::<MapArray>()?;
    let offsets = map.value_offsets();
    let start = offsets[row] as usize;
    let end = offsets[row + 1] as usize;
    let keys = map.keys().as_any().downcast_ref::<StringArray>()?;
    let values = map.values().as_any().downcast_ref::<StringArray>()?;
    let mut entries = Vec::with_capacity(end - start);
    for i in start..end {
        if keys.is_null(i) {
            continue;
        }
        let value = (!values.is_null(i))
            .then(|| values.value(i).to_string())
            .unwrap_or_default();
        entries.push((keys.value(i).to_string(), value));
    }
    (!entries.is_empty()).then_some(entries)
}

// [Phase 2] Maps the Arrow `Tags` map column to [`BlobTags`].
fn blob_tags_at(batch: &RecordBatch, name: &str, row: usize) -> Option<BlobTags> {
    let blob_tag_set = map_entries_at(batch, name, row)?
        .into_iter()
        .map(|(key, value)| BlobTag {
            key: Some(key),
            value: Some(value),
        })
        .collect();
    Some(BlobTags {
        blob_tag_set: Some(blob_tag_set),
    })
}

// [Phase 2] Maps the Arrow `Metadata` map column to [`BlobMetadata`].
fn blob_metadata_at(batch: &RecordBatch, name: &str, row: usize) -> Option<BlobMetadata> {
    Some(BlobMetadata {
        values: Some(map_entries_at(batch, name, row)?.into_iter().collect()),
        encrypted: None,
    })
}

// [Phase 2] Maps the Arrow `OrMetadata` map column to [`ObjectReplicationMetadata`].
fn or_metadata_at(
    batch: &RecordBatch,
    name: &str,
    row: usize,
) -> Option<ObjectReplicationMetadata> {
    Some(ObjectReplicationMetadata {
        additional_properties: Some(map_entries_at(batch, name, row)?.into_iter().collect()),
    })
}

fn to_error(error: ArrowError) -> Error {
    Error::new(ErrorKind::DataConversion, error)
}

/// Returns the column with the given name, or `None` if it is absent.
fn column<'a>(batch: &'a RecordBatch, name: &str) -> Option<&'a dyn Array> {
    let index = batch.schema().index_of(name).ok()?;
    Some(batch.column(index).as_ref())
}

fn string_at(batch: &RecordBatch, name: &str, row: usize) -> Option<String> {
    let array = column(batch, name)?
        .as_any()
        .downcast_ref::<StringArray>()?;
    (!array.is_null(row)).then(|| array.value(row).to_string())
}

fn bool_at(batch: &RecordBatch, name: &str, row: usize) -> Option<bool> {
    let array = column(batch, name)?
        .as_any()
        .downcast_ref::<BooleanArray>()?;
    (!array.is_null(row)).then(|| array.value(row))
}

fn u64_at(batch: &RecordBatch, name: &str, row: usize) -> Option<u64> {
    let array = column(batch, name)?;
    if array.is_null(row) {
        return None;
    }
    if let Some(array) = array.as_any().downcast_ref::<UInt64Array>() {
        Some(array.value(row))
    } else if let Some(array) = array.as_any().downcast_ref::<Int64Array>() {
        u64::try_from(array.value(row)).ok()
    } else {
        None
    }
}

fn i64_at(batch: &RecordBatch, name: &str, row: usize) -> Option<i64> {
    let array = column(batch, name)?;
    if array.is_null(row) {
        return None;
    }
    if let Some(array) = array.as_any().downcast_ref::<Int64Array>() {
        Some(array.value(row))
    } else if let Some(array) = array.as_any().downcast_ref::<UInt64Array>() {
        i64::try_from(array.value(row)).ok()
    } else if let Some(array) = array.as_any().downcast_ref::<Int32Array>() {
        Some(array.value(row) as i64)
    } else if let Some(array) = array.as_any().downcast_ref::<UInt32Array>() {
        Some(array.value(row) as i64)
    } else {
        None
    }
}

fn i32_at(batch: &RecordBatch, name: &str, row: usize) -> Option<i32> {
    let array = column(batch, name)?;
    if array.is_null(row) {
        return None;
    }
    if let Some(array) = array.as_any().downcast_ref::<Int32Array>() {
        Some(array.value(row))
    } else if let Some(array) = array.as_any().downcast_ref::<UInt32Array>() {
        i32::try_from(array.value(row)).ok()
    } else if let Some(array) = array.as_any().downcast_ref::<Int64Array>() {
        i32::try_from(array.value(row)).ok()
    } else if let Some(array) = array.as_any().downcast_ref::<UInt64Array>() {
        i32::try_from(array.value(row)).ok()
    } else {
        None
    }
}

fn timestamp_at(batch: &RecordBatch, name: &str, row: usize) -> Option<OffsetDateTime> {
    let array = column(batch, name)?;
    if array.is_null(row) {
        return None;
    }
    let nanos: i128 = match array.data_type() {
        DataType::Timestamp(TimeUnit::Second, _) => {
            array
                .as_any()
                .downcast_ref::<TimestampSecondArray>()?
                .value(row) as i128
                * 1_000_000_000
        }
        DataType::Timestamp(TimeUnit::Millisecond, _) => {
            array
                .as_any()
                .downcast_ref::<TimestampMillisecondArray>()?
                .value(row) as i128
                * 1_000_000
        }
        DataType::Timestamp(TimeUnit::Microsecond, _) => {
            array
                .as_any()
                .downcast_ref::<TimestampMicrosecondArray>()?
                .value(row) as i128
                * 1_000
        }
        DataType::Timestamp(TimeUnit::Nanosecond, _) => array
            .as_any()
            .downcast_ref::<TimestampNanosecondArray>()?
            .value(row) as i128,
        _ => return None,
    };
    OffsetDateTime::from_unix_timestamp_nanos(nanos).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccessTier, ArchiveStatus, BlobType, CopyStatus, ImmutabilityPolicyMode, LeaseDuration,
        LeaseState, LeaseStatus, RehydratePriority,
    };
    use arrow_array::builder::{MapBuilder, StringBuilder};
    use arrow_array::ArrayRef;
    use arrow_ipc::writer::StreamWriter;
    use arrow_schema::{Field, Schema};
    use std::collections::HashMap;
    use std::sync::Arc;

    fn s(value: &str) -> ArrayRef {
        Arc::new(StringArray::from(vec![Some(value.to_string())]))
    }

    fn b(value: bool) -> ArrayRef {
        Arc::new(BooleanArray::from(vec![Some(value)]))
    }

    fn u64c(value: u64) -> ArrayRef {
        Arc::new(UInt64Array::from(vec![Some(value)]))
    }

    fn ts(millis: i64) -> ArrayRef {
        Arc::new(TimestampMillisecondArray::from(vec![Some(millis)]))
    }

    fn expected_ts(millis: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp_nanos(millis as i128 * 1_000_000).unwrap()
    }

    /// Builds a single-row [`RecordBatch`] from `(column_name, array)` pairs,
    /// deriving each nullable [`Field`] from the array's own data type.
    fn batch(columns: Vec<(&str, ArrayRef)>) -> RecordBatch {
        let fields: Vec<Field> = columns
            .iter()
            .map(|(name, array)| Field::new(*name, array.data_type().clone(), true))
            .collect();
        let arrays: Vec<ArrayRef> = columns.into_iter().map(|(_, array)| array).collect();
        RecordBatch::try_new(Arc::new(Schema::new(fields)), arrays).unwrap()
    }

    #[test]
    fn populated_row_maps_every_field() {
        let md5 = base64::encode([1u8, 2, 3, 4]);
        let batch = batch(vec![
            // BlobItem-level columns.
            ("Name", s("blob.txt")),
            ("Deleted", b(false)),
            ("HasVersionsOnly", b(true)),
            ("IsCurrentVersion", b(true)),
            ("Snapshot", s("2019-01-01T00:00:00.0000000Z")),
            ("VersionId", s("2021-01-01T00:00:00.0000000Z")),
            // BlobProperties columns.
            ("AccessTier", s("Hot")),
            ("AccessTierChangeTime", ts(3_000_000)),
            ("AccessTierInferred", b(true)),
            ("ArchiveStatus", s("rehydrate-pending-to-hot")),
            ("x-ms-blob-sequence-number", u64c(42)),
            ("BlobType", s("BlockBlob")),
            ("Cache-Control", s("no-cache")),
            ("Content-Disposition", s("inline")),
            ("Content-Encoding", s("gzip")),
            ("Content-Language", s("en-US")),
            ("Content-Length", u64c(12345)),
            ("Content-MD5", s(&md5)),
            ("Content-Type", s("text/plain")),
            ("CopyCompletionTime", ts(4_000_000)),
            ("CopyId", s("copy-id")),
            ("CopyProgress", s("1024/1024")),
            ("CopySource", s("https://example.com/source")),
            ("CopyStatus", s("success")),
            ("CopyStatusDescription", s("done")),
            ("Creation-Time", ts(1_000_000)),
            ("DeletedTime", ts(5_000_000)),
            ("CopyDestinationSnapshot", s("2020-01-01T00:00:00.0000000Z")),
            ("CustomerProvidedKeySha256", s("cpk-sha")),
            ("EncryptionScope", s("scope-1")),
            ("Etag", s("0xETAG")),
            ("ImmutabilityPolicyUntilDate", ts(6_000_000)),
            ("ImmutabilityPolicyMode", s("unlocked")),
            ("IncrementalCopy", b(false)),
            ("Sealed", b(true)),
            ("LastAccessTime", ts(7_000_000)),
            ("Last-Modified", ts(2_000_000)),
            ("LeaseDuration", s("infinite")),
            ("LeaseState", s("available")),
            ("LeaseStatus", s("unlocked")),
            ("LegalHold", b(true)),
            ("RehydratePriority", s("High")),
            ("RemainingRetentionDays", u64c(7)),
            ("ServerEncrypted", b(true)),
            ("TagCount", u64c(3)),
        ]);

        let item = row_to_blob_item(&batch, 0);

        // BlobItem-level fields.
        assert_eq!(Some("blob.txt".to_string()), item.name);
        assert_eq!(Some(false), item.deleted);
        assert_eq!(Some(true), item.has_versions_only);
        assert_eq!(Some(true), item.is_current_version);
        assert_eq!(
            Some("2019-01-01T00:00:00.0000000Z".to_string()),
            item.snapshot
        );
        assert_eq!(
            Some("2021-01-01T00:00:00.0000000Z".to_string()),
            item.version_id
        );

        let props = item.properties.expect("properties should be set");
        assert_eq!(Some(AccessTier::Hot), props.access_tier);
        assert_eq!(Some(expected_ts(3_000_000)), props.access_tier_change_time);
        assert_eq!(Some(true), props.access_tier_inferred);
        assert_eq!(
            Some(ArchiveStatus::RehydratePendingToHot),
            props.archive_status
        );
        assert_eq!(Some(42), props.blob_sequence_number);
        assert_eq!(Some(BlobType::BlockBlob), props.blob_type);
        assert_eq!(Some("no-cache".to_string()), props.cache_control);
        assert_eq!(Some("inline".to_string()), props.content_disposition);
        assert_eq!(Some("gzip".to_string()), props.content_encoding);
        assert_eq!(Some("en-US".to_string()), props.content_language);
        assert_eq!(Some(12345), props.content_length);
        assert_eq!(Some(vec![1u8, 2, 3, 4]), props.content_md5);
        assert_eq!(Some("text/plain".to_string()), props.content_type);
        assert_eq!(Some(expected_ts(4_000_000)), props.copy_completion_time);
        assert_eq!(Some("copy-id".to_string()), props.copy_id);
        assert_eq!(Some("1024/1024".to_string()), props.copy_progress);
        assert_eq!(
            Some("https://example.com/source".to_string()),
            props.copy_source
        );
        assert_eq!(Some(CopyStatus::Success), props.copy_status);
        assert_eq!(Some("done".to_string()), props.copy_status_description);
        assert_eq!(Some(expected_ts(1_000_000)), props.creation_time);
        assert_eq!(Some(expected_ts(5_000_000)), props.deleted_time);
        assert_eq!(
            Some("2020-01-01T00:00:00.0000000Z".to_string()),
            props.destination_snapshot
        );
        assert_eq!(Some("cpk-sha".to_string()), props.encryption_key_sha256);
        assert_eq!(Some("scope-1".to_string()), props.encryption_scope);
        assert_eq!(Some(Etag::from("0xETAG")), props.etag);
        assert_eq!(
            Some(expected_ts(6_000_000)),
            props.immutability_policy_expires_on
        );
        assert_eq!(
            Some(ImmutabilityPolicyMode::Unlocked),
            props.immutability_policy_mode
        );
        assert_eq!(Some(false), props.incremental_copy);
        assert_eq!(Some(true), props.is_sealed);
        assert_eq!(Some(expected_ts(7_000_000)), props.last_accessed_on);
        assert_eq!(Some(expected_ts(2_000_000)), props.last_modified);
        assert_eq!(Some(LeaseDuration::Infinite), props.lease_duration);
        assert_eq!(Some(LeaseState::Available), props.lease_state);
        assert_eq!(Some(LeaseStatus::Unlocked), props.lease_status);
        assert_eq!(Some(true), props.legal_hold);
        assert_eq!(Some(RehydratePriority::High), props.rehydrate_priority);
        assert_eq!(Some(7), props.remaining_retention_days);
        assert_eq!(Some(true), props.server_encrypted);
        assert_eq!(Some(3), props.tag_count);
    }

    #[test]
    fn null_values_map_to_none() {
        let batch = batch(vec![
            (
                "Name",
                Arc::new(StringArray::from(vec![Option::<String>::None])),
            ),
            (
                "Creation-Time",
                Arc::new(TimestampMillisecondArray::from(vec![Option::<i64>::None])),
            ),
            (
                "Content-Length",
                Arc::new(UInt64Array::from(vec![Option::<u64>::None])),
            ),
            (
                "ServerEncrypted",
                Arc::new(BooleanArray::from(vec![Option::<bool>::None])),
            ),
            (
                "BlobType",
                Arc::new(StringArray::from(vec![Option::<String>::None])),
            ),
            (
                "TagCount",
                Arc::new(UInt64Array::from(vec![Option::<u64>::None])),
            ),
            (
                "x-ms-blob-sequence-number",
                Arc::new(UInt64Array::from(vec![Option::<u64>::None])),
            ),
        ]);

        let item = row_to_blob_item(&batch, 0);
        assert_eq!(None, item.name);
        let props = item.properties.expect("properties should be set");
        assert_eq!(None, props.creation_time);
        assert_eq!(None, props.content_length);
        assert_eq!(None, props.server_encrypted);
        assert_eq!(None, props.blob_type);
        assert_eq!(None, props.tag_count);
        assert_eq!(None, props.blob_sequence_number);
    }

    #[test]
    fn absent_columns_map_to_none() {
        // Only `Name` is present; every other field must fall back to `None`.
        let batch = batch(vec![("Name", s("only-name"))]);

        let item = row_to_blob_item(&batch, 0);
        assert_eq!(Some("only-name".to_string()), item.name);
        assert_eq!(None, item.version_id);
        let props = item.properties.expect("properties should be set");
        assert_eq!(None, props.content_type);
        assert_eq!(None, props.access_tier);
        assert_eq!(None, props.creation_time);
        assert_eq!(None, props.blob_sequence_number);
    }

    // [Phase 2] Builds a single-row `map<utf8, utf8>` column from key/value pairs.
    fn map_col(entries: &[(&str, &str)]) -> ArrayRef {
        let mut builder = MapBuilder::new(None, StringBuilder::new(), StringBuilder::new());
        for (key, value) in entries {
            builder.keys().append_value(key);
            builder.values().append_value(value);
        }
        builder.append(true).unwrap();
        Arc::new(builder.finish())
    }

    #[test]
    fn map_columns_decode() {
        // [Phase 2] `Tags`, `Metadata`, and `OrMetadata` decode from Arrow map columns.
        let batch = batch(vec![
            ("Name", s("blob.txt")),
            ("Tags", map_col(&[("env", "test"), ("team", "sdk")])),
            ("Metadata", map_col(&[("team", "sdk")])),
            ("OrMetadata", map_col(&[("policy-id", "rule-id")])),
        ]);

        let item = row_to_blob_item(&batch, 0);

        let tags = item
            .blob_tags
            .expect("blob_tags should be populated")
            .blob_tag_set
            .expect("tag set should be present");
        assert!(tags
            .iter()
            .any(|t| t.key.as_deref() == Some("env") && t.value.as_deref() == Some("test")));
        assert!(tags
            .iter()
            .any(|t| t.key.as_deref() == Some("team") && t.value.as_deref() == Some("sdk")));

        let metadata = item
            .metadata
            .expect("metadata should be populated")
            .values
            .expect("metadata values should be present");
        assert_eq!(Some(&"sdk".to_string()), metadata.get("team"));

        let or_metadata = item
            .object_replication_metadata
            .expect("or_metadata should be populated")
            .additional_properties
            .expect("or_metadata properties should be present");
        assert_eq!(Some(&"rule-id".to_string()), or_metadata.get("policy-id"));
    }

    #[test]
    fn absent_map_columns_are_none() {
        // [Phase 2] Map fields fall back to `None` when the columns are absent.
        let batch = batch(vec![("Name", s("only-name"))]);
        let item = row_to_blob_item(&batch, 0);
        assert!(item.blob_tags.is_none());
        assert!(item.metadata.is_none());
        assert!(item.object_replication_metadata.is_none());
    }

    #[test]
    fn decode_stream_maps_rows_and_next_marker() {
        let columns = vec![
            ("Name", s("hello.txt")),
            ("BlobType", s("BlockBlob")),
            ("Content-Length", u64c(10)),
            ("ServerEncrypted", b(true)),
        ];
        let fields: Vec<Field> = columns
            .iter()
            .map(|(name, array)| Field::new(*name, array.data_type().clone(), true))
            .collect();
        let arrays: Vec<ArrayRef> = columns.into_iter().map(|(_, array)| array).collect();

        let mut metadata = HashMap::new();
        metadata.insert(NEXT_MARKER_KEY.to_string(), "next-page".to_string());
        let schema = Arc::new(Schema::new_with_metadata(fields, metadata));
        let record_batch = RecordBatch::try_new(schema.clone(), arrays).unwrap();

        let mut buffer = Vec::new();
        {
            let mut writer = StreamWriter::try_new(&mut buffer, &schema).unwrap();
            writer.write(&record_batch).unwrap();
            writer.finish().unwrap();
        }

        let response = decode_arrow_list_blobs(&buffer).unwrap();
        assert_eq!(Some("next-page".to_string()), response.next_marker);
        assert_eq!(1, response.blob_items.len());

        let item = &response.blob_items[0];
        assert_eq!(Some("hello.txt".to_string()), item.name);
        let props = item.properties.as_ref().expect("properties should be set");
        assert_eq!(Some(BlobType::BlockBlob), props.blob_type);
        assert_eq!(Some(10), props.content_length);
        assert_eq!(Some(true), props.server_encrypted);
    }

    #[test]
    fn arrow_next_marker_absent_is_none() {
        let columns = vec![("Name", s("hello.txt"))];
        let fields: Vec<Field> = columns
            .iter()
            .map(|(name, array)| Field::new(*name, array.data_type().clone(), true))
            .collect();
        let arrays: Vec<ArrayRef> = columns.into_iter().map(|(_, array)| array).collect();
        let schema = Arc::new(Schema::new(fields));
        let record_batch = RecordBatch::try_new(schema.clone(), arrays).unwrap();

        let mut buffer = Vec::new();
        {
            let mut writer = StreamWriter::try_new(&mut buffer, &schema).unwrap();
            writer.write(&record_batch).unwrap();
            writer.finish().unwrap();
        }

        assert_eq!(None, arrow_next_marker(&buffer).unwrap());
    }
}
