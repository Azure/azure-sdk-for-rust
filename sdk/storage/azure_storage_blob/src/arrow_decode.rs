// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

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

const NEXT_MARKER_KEY: &str = "NextMarker";
const ARROW_CONTENT_TYPE: &str = "application/vnd.apache.arrow.stream";
const XML_CONTENT_TYPE: &str = "application/xml";

#[derive(Clone, Copy)]
enum WireFormat {
    Arrow,
    Xml,
}

/// Selects the list blobs deserializer from the response `Content-Type`.
#[derive(Debug, Clone)]
pub struct AutoFormat;

impl Format for AutoFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> Result<T> {
        azure_core::xml::from_xml(body.as_ref())
    }
}

impl DeserializeWith<AutoFormat> for ListBlobsResponse {
    fn deserialize_with(body: ResponseBody) -> Result<Self> {
        body.xml()
    }

    fn deserialize_from(response: RawResponse) -> Result<Self> {
        match wire_format(response.headers())? {
            WireFormat::Arrow => decode_arrow_list_blobs(response.body()),
            WireFormat::Xml => azure_core::xml::from_xml(response.body()),
        }
    }
}

pub(crate) fn decode_next_marker(headers: &Headers, bytes: &[u8]) -> Result<Option<String>> {
    match wire_format(headers)? {
        WireFormat::Arrow => arrow_next_marker(bytes),
        WireFormat::Xml => {
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

fn wire_format(headers: &Headers) -> Result<WireFormat> {
    let Some(content_type) = headers.get_optional_str(&headers::CONTENT_TYPE) else {
        return Ok(WireFormat::Xml);
    };
    let media_type = content_type.split(';').next().unwrap_or_default().trim();
    if media_type.eq_ignore_ascii_case(ARROW_CONTENT_TYPE) {
        Ok(WireFormat::Arrow)
    } else if media_type.eq_ignore_ascii_case(XML_CONTENT_TYPE) {
        Ok(WireFormat::Xml)
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

fn decode_arrow_list_blobs(bytes: &[u8]) -> Result<ListBlobsResponse> {
    let reader = StreamReader::try_new(bytes, None).map_err(to_error)?;
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
        access_tier: string_at(batch, "AccessTier", row).and_then(|value| value.parse().ok()),
        access_tier_change_time: timestamp_at(batch, "AccessTierChangeTime", row),
        access_tier_inferred: bool_at(batch, "AccessTierInferred", row),
        archive_status: string_at(batch, "ArchiveStatus", row).and_then(|value| value.parse().ok()),
        blob_sequence_number: i64_at(batch, "x-ms-blob-sequence-number", row),
        blob_type: string_at(batch, "BlobType", row).and_then(|value| value.parse().ok()),
        cache_control: string_at(batch, "Cache-Control", row),
        content_disposition: string_at(batch, "Content-Disposition", row),
        content_encoding: string_at(batch, "Content-Encoding", row),
        content_language: string_at(batch, "Content-Language", row),
        content_length: u64_at(batch, "Content-Length", row),
        content_md5: string_at(batch, "Content-MD5", row)
            .and_then(|value| base64::decode(value).ok()),
        content_type: string_at(batch, "Content-Type", row),
        copy_completion_time: timestamp_at(batch, "CopyCompletionTime", row),
        copy_id: string_at(batch, "CopyId", row),
        copy_progress: string_at(batch, "CopyProgress", row),
        copy_source: string_at(batch, "CopySource", row),
        copy_status: string_at(batch, "CopyStatus", row).and_then(|value| value.parse().ok()),
        copy_status_description: string_at(batch, "CopyStatusDescription", row),
        creation_time: timestamp_at(batch, "Creation-Time", row),
        deleted_time: timestamp_at(batch, "DeletedTime", row),
        destination_snapshot: string_at(batch, "CopyDestinationSnapshot", row),
        encryption_key_sha256: string_at(batch, "CustomerProvidedKeySha256", row),
        encryption_scope: string_at(batch, "EncryptionScope", row),
        etag: string_at(batch, "Etag", row).map(Etag::from),
        expires_on: timestamp_at(batch, "Expiry-Time", row),
        immutability_policy_expires_on: timestamp_at(batch, "ImmutabilityPolicyUntilDate", row),
        immutability_policy_mode: string_at(batch, "ImmutabilityPolicyMode", row)
            .and_then(|value| value.parse().ok()),
        incremental_copy: bool_at(batch, "IncrementalCopy", row),
        is_sealed: bool_at(batch, "Sealed", row),
        last_accessed_on: timestamp_at(batch, "LastAccessTime", row),
        last_modified: timestamp_at(batch, "Last-Modified", row),
        lease_duration: string_at(batch, "LeaseDuration", row).and_then(|value| value.parse().ok()),
        lease_state: string_at(batch, "LeaseState", row).and_then(|value| value.parse().ok()),
        lease_status: string_at(batch, "LeaseStatus", row).and_then(|value| value.parse().ok()),
        legal_hold: bool_at(batch, "LegalHold", row),
        rehydrate_priority: string_at(batch, "RehydratePriority", row)
            .and_then(|value| value.parse().ok()),
        remaining_retention_days: i32_at(batch, "RemainingRetentionDays", row),
        server_encrypted: bool_at(batch, "ServerEncrypted", row),
        smart_access_tier: string_at(batch, "SmartAccessTier", row)
            .and_then(|value| value.parse().ok()),
        tag_count: i32_at(batch, "TagCount", row),
        ..Default::default()
    };

    BlobItem {
        blob_tags: blob_tags_at(batch, "Tags", row),
        deleted: bool_at(batch, "Deleted", row),
        has_versions_only: bool_at(batch, "HasVersionsOnly", row),
        is_current_version: bool_at(batch, "IsCurrentVersion", row),
        metadata: blob_metadata_at(batch, "Metadata", row),
        name: string_at(batch, "Name", row),
        object_replication_metadata: object_replication_metadata_at(batch, "OrMetadata", row),
        properties: Some(properties),
        snapshot: string_at(batch, "Snapshot", row),
        version_id: string_at(batch, "VersionId", row),
        ..Default::default()
    }
}

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
    for index in start..end {
        if keys.is_null(index) {
            continue;
        }
        let value = (!values.is_null(index))
            .then(|| values.value(index).to_string())
            .unwrap_or_default();
        entries.push((keys.value(index).to_string(), value));
    }
    (!entries.is_empty()).then_some(entries)
}

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

fn blob_metadata_at(batch: &RecordBatch, name: &str, row: usize) -> Option<BlobMetadata> {
    Some(BlobMetadata {
        values: Some(map_entries_at(batch, name, row)?.into_iter().collect()),
        encrypted: None,
    })
}

fn object_replication_metadata_at(
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
        Some(i64::from(array.value(row)))
    } else if let Some(array) = array.as_any().downcast_ref::<UInt32Array>() {
        Some(i64::from(array.value(row)))
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
    let nanos = match array.data_type() {
        DataType::Timestamp(TimeUnit::Second, _) => {
            i128::from(
                array
                    .as_any()
                    .downcast_ref::<TimestampSecondArray>()?
                    .value(row),
            ) * 1_000_000_000
        }
        DataType::Timestamp(TimeUnit::Millisecond, _) => {
            i128::from(
                array
                    .as_any()
                    .downcast_ref::<TimestampMillisecondArray>()?
                    .value(row),
            ) * 1_000_000
        }
        DataType::Timestamp(TimeUnit::Microsecond, _) => {
            i128::from(
                array
                    .as_any()
                    .downcast_ref::<TimestampMicrosecondArray>()?
                    .value(row),
            ) * 1_000
        }
        DataType::Timestamp(TimeUnit::Nanosecond, _) => i128::from(
            array
                .as_any()
                .downcast_ref::<TimestampNanosecondArray>()?
                .value(row),
        ),
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
            ("Expiry-Time", ts(8_000_000)),
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
        assert_eq!(Some(expected_ts(8_000_000)), props.expires_on);
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

    // Builds a single-row `map<utf8, utf8>` column from key/value pairs.
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
        // `Tags`, `Metadata`, and `OrMetadata` decode from Arrow map columns.
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
        // Map fields fall back to `None` when the columns are absent.
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

    #[test]
    fn missing_content_type_defaults_to_xml() {
        assert!(matches!(wire_format(&Headers::new()), Ok(WireFormat::Xml)));
    }

    #[test]
    fn arrow_contract_covers_every_model_field() {
        // Compile-time guard: adding a field to `BlobItem`/`BlobProperties` breaks this
        // exhaustive destructure until `row_to_blob_item` is updated to map it, preventing
        // the Arrow path from silently dropping model fields the XML path would populate.
        let item = row_to_blob_item(&batch(vec![("Name", s("guard"))]), 0);
        let BlobItem {
            blob_tags: _,
            deleted: _,
            has_versions_only: _,
            is_current_version: _,
            metadata: _,
            name: _,
            object_replication_metadata: _,
            properties,
            snapshot: _,
            version_id: _,
        } = item;
        let BlobProperties {
            access_tier: _,
            access_tier_change_time: _,
            access_tier_inferred: _,
            archive_status: _,
            blob_sequence_number: _,
            blob_type: _,
            cache_control: _,
            content_disposition: _,
            content_encoding: _,
            content_language: _,
            content_length: _,
            content_md5: _,
            content_type: _,
            copy_completion_time: _,
            copy_id: _,
            copy_progress: _,
            copy_source: _,
            copy_status: _,
            copy_status_description: _,
            creation_time: _,
            deleted_time: _,
            destination_snapshot: _,
            encryption_key_sha256: _,
            encryption_scope: _,
            etag: _,
            expires_on: _,
            immutability_policy_expires_on: _,
            immutability_policy_mode: _,
            incremental_copy: _,
            is_sealed: _,
            last_accessed_on: _,
            last_modified: _,
            lease_duration: _,
            lease_state: _,
            lease_status: _,
            legal_hold: _,
            rehydrate_priority: _,
            remaining_retention_days: _,
            server_encrypted: _,
            smart_access_tier: _,
            tag_count: _,
        } = properties.expect("properties should be set");
    }
}
