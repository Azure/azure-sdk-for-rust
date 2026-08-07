// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Arrow IPC stream decoding for `list_blobs` responses.

use crate::generated::models::{BlobItem, BlobProperties, ListBlobsResponse};
use arrow::{
    array::{
        Array, BooleanArray, Int64Array, RecordBatch, StringArray, TimestampMicrosecondArray,
        TimestampMillisecondArray, TimestampNanosecondArray, TimestampSecondArray, UInt64Array,
    },
    datatypes::{DataType, TimeUnit},
    error::ArrowError,
    ipc::reader::StreamReader,
};
use azure_core::{
    base64,
    error::{Error, ErrorKind},
    http::Etag,
    time::OffsetDateTime,
    Result,
};

const NEXT_MARKER_KEY: &str = "NextMarker";

/// Decodes an Arrow IPC stream into a [`ListBlobsResponse`].
pub(crate) fn decode_arrow_list_blobs(bytes: &[u8]) -> Result<ListBlobsResponse> {
    let reader = StreamReader::try_new(bytes, None).map_err(to_error)?;
    let next_marker = reader
        .schema()
        .metadata()
        .get(NEXT_MARKER_KEY)
        .filter(|m| !m.is_empty())
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

/// Extracts the next marker from Arrow schema metadata.
pub(crate) fn arrow_next_marker(bytes: &[u8]) -> Result<Option<String>> {
    let reader = StreamReader::try_new(bytes, None).map_err(to_error)?;
    Ok(reader
        .schema()
        .metadata()
        .get(NEXT_MARKER_KEY)
        .filter(|m| !m.is_empty())
        .cloned())
}

fn row_to_blob_item(batch: &RecordBatch, row: usize) -> BlobItem {
    let properties = BlobProperties {
        creation_time: timestamp_at(batch, "Creation-Time", row),
        last_modified: timestamp_at(batch, "Last-Modified", row),
        blob_type: string_at(batch, "BlobType", row).and_then(|s| s.parse().ok()),
        etag: string_at(batch, "Etag", row).map(Etag::from),
        content_length: u64_at(batch, "Content-Length", row),
        content_type: string_at(batch, "Content-Type", row),
        content_md5: string_at(batch, "Content-MD5", row).and_then(|s| base64::decode(s).ok()),
        access_tier: string_at(batch, "AccessTier", row).and_then(|s| s.parse().ok()),
        lease_state: string_at(batch, "LeaseState", row).and_then(|s| s.parse().ok()),
        lease_status: string_at(batch, "LeaseStatus", row).and_then(|s| s.parse().ok()),
        server_encrypted: bool_at(batch, "ServerEncrypted", row),
        ..Default::default()
    };
    BlobItem {
        name: string_at(batch, "Name", row),
        properties: Some(properties),
        ..Default::default()
    }
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
    use arrow::{
        array::builder::{StringBuilder, UInt64Builder},
        datatypes::{Field, Schema},
        ipc::writer::StreamWriter,
    };
    use std::{collections::HashMap, sync::Arc};

    fn build_arrow_stream(schema: &Arc<Schema>, batches: &[&RecordBatch]) -> Vec<u8> {
        let mut buf = Vec::new();
        let mut writer = StreamWriter::try_new(&mut buf, schema).unwrap();
        for batch in batches {
            writer.write(batch).unwrap();
        }
        writer.finish().unwrap();
        buf
    }

    #[test]
    fn decode_arrow_list_blobs_single_blob() {
        let schema = Arc::new(Schema::new_with_metadata(
            vec![
                Field::new("Name", DataType::Utf8, true),
                Field::new("Content-Length", DataType::UInt64, true),
            ],
            HashMap::new(),
        ));
        let mut names = StringBuilder::new();
        names.append_value("myblob.txt");
        let mut lengths = UInt64Builder::new();
        lengths.append_value(42);
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![Arc::new(names.finish()), Arc::new(lengths.finish())],
        )
        .unwrap();

        let bytes = build_arrow_stream(&schema, &[&batch]);
        let response = decode_arrow_list_blobs(&bytes).unwrap();
        assert_eq!(response.blob_items.len(), 1);
        assert_eq!(response.blob_items[0].name.as_deref(), Some("myblob.txt"));
        assert_eq!(
            response.blob_items[0]
                .properties
                .as_ref()
                .unwrap()
                .content_length,
            Some(42)
        );
        assert_eq!(response.next_marker, None);
    }

    #[test]
    fn decode_arrow_list_blobs_with_next_marker() {
        let schema = Arc::new(Schema::new_with_metadata(
            vec![Field::new("Name", DataType::Utf8, true)],
            HashMap::from([(NEXT_MARKER_KEY.to_string(), "page2token".to_string())]),
        ));
        let mut names = StringBuilder::new();
        names.append_value("blob1");
        let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(names.finish())]).unwrap();

        let bytes = build_arrow_stream(&schema, &[&batch]);
        let response = decode_arrow_list_blobs(&bytes).unwrap();
        assert_eq!(response.next_marker.as_deref(), Some("page2token"));
    }

    #[test]
    fn arrow_next_marker_empty_string_is_none() {
        let schema = Arc::new(Schema::new_with_metadata(
            vec![Field::new("Name", DataType::Utf8, true)],
            HashMap::from([(NEXT_MARKER_KEY.to_string(), String::new())]),
        ));
        let mut names = StringBuilder::new();
        names.append_value("blob");
        let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(names.finish())]).unwrap();

        let bytes = build_arrow_stream(&schema, &[&batch]);
        let result = arrow_next_marker(&bytes).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn decode_arrow_list_blobs_empty_stream() {
        let schema = Arc::new(Schema::new_with_metadata(
            vec![Field::new("Name", DataType::Utf8, true)],
            HashMap::new(),
        ));
        let bytes = build_arrow_stream(&schema, &[]);
        let response = decode_arrow_list_blobs(&bytes).unwrap();
        assert!(response.blob_items.is_empty());
        assert_eq!(response.next_marker, None);
    }
}
