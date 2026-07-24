// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Prototype: Apache Arrow IPC decoding for the flat `list_blobs` response.
//!
//! This is wired through a custom [`Format`] ([`ArrowXmlFormat`]) so the generated
//! pager and [`Response::into_model`](azure_core::http::Response::into_model)
//! transparently dispatch between the Apache Arrow stream and XML based on the
//! response `Content-Type` header — no hand-written response wrapper or pager needed.
//!
//! Only the subset of columns needed by the prototype is mapped. Envelope fields
//! (container name, prefix, marker, max results, service endpoint) are not carried
//! as Arrow columns and are left unset.

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
