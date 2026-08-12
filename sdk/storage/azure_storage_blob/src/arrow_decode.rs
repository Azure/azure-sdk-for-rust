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
    use arrow_array::ArrayRef;
    use arrow_ipc::writer::StreamWriter;
    use arrow_schema::{Field, Schema};
    use std::{collections::HashMap, sync::Arc};

    #[test]
    fn arrow_response_decodes_items_and_continuation() {
        let schema = Arc::new(Schema::new_with_metadata(
            vec![
                Field::new("Name", DataType::Utf8, true),
                Field::new("Content-Length", DataType::UInt64, true),
            ],
            HashMap::from([(NEXT_MARKER_KEY.to_string(), "page-2".to_string())]),
        ));
        let columns: Vec<ArrayRef> = vec![
            Arc::new(StringArray::from(vec![Some("blob.txt")])),
            Arc::new(UInt64Array::from(vec![Some(42)])),
        ];
        let batch = RecordBatch::try_new(schema.clone(), columns).unwrap();

        let mut bytes = Vec::new();
        let mut writer = StreamWriter::try_new(&mut bytes, &schema).unwrap();
        writer.write(&batch).unwrap();
        writer.finish().unwrap();
        drop(writer);

        let response = decode_arrow_list_blobs(&bytes).unwrap();
        assert_eq!(response.next_marker.as_deref(), Some("page-2"));
        assert_eq!(response.blob_items.len(), 1);
        assert_eq!(response.blob_items[0].name.as_deref(), Some("blob.txt"));
        assert_eq!(
            response.blob_items[0]
                .properties
                .as_ref()
                .and_then(|properties| properties.content_length),
            Some(42)
        );
    }

    #[test]
    fn missing_content_type_defaults_to_xml() {
        assert!(matches!(wire_format(&Headers::new()), Ok(WireFormat::Xml)));
    }
}
