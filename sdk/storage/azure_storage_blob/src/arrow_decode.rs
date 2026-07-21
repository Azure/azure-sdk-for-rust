// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Prototype: decode an Apache Arrow IPC stream `list_blobs` response into the
//! generated [`ListBlobsResponse`] model. TODO: Not exhaustive, needs further testing for all.
//!
//! This exists to prototype Arrow-stream support for the flat blob listing API.
//! Only the subset of columns needed by the prototype is mapped. Envelope
//! fields (container name, prefix, marker, max results, service endpoint) are
//! not carried as Arrow columns and are left unset.

use crate::models::{BlobItem, BlobProperties, ListBlobsResponse};
use arrow_array::{
    Array, BooleanArray, Int64Array, RecordBatch, StringArray, TimestampMicrosecondArray,
    TimestampMillisecondArray, TimestampNanosecondArray, TimestampSecondArray, UInt64Array,
};
use arrow_ipc::reader::StreamReader;
use arrow_schema::{ArrowError, DataType, TimeUnit};
use azure_core::{
    base64,
    error::{Error, ErrorKind},
    http::{
        headers::{self, Headers},
        Etag,
    },
    time::OffsetDateTime,
    Result,
};
/// Metadata key on the Arrow schema holding the continuation token.
const NEXT_MARKER_KEY: &str = "NextMarker";
const ARROW_CONTENT_TYPE: &str = "application/vnd.apache.arrow.stream";
const XML_CONTENT_TYPE: &str = "application/xml";

#[derive(Clone, Copy)]
enum ListBlobsWireFormat {
    Arrow,
    Xml,
}

pub(crate) fn decode_list_blobs(headers: &Headers, bytes: &[u8]) -> Result<ListBlobsResponse> {
    match wire_format(headers)? {
        ListBlobsWireFormat::Arrow => decode_arrow_list_blobs(bytes),
        ListBlobsWireFormat::Xml => azure_core::xml::from_xml(bytes),
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
