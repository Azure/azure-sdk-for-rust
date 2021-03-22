use crate::blob_storage::blob::Blob;
use azure_core::errors::AzureError;
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::prelude::NextMarker;
use azure_core::util::to_str_without_bom;
use azure_core::RequestId;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct ListBlobsResponse {
    pub prefix: Option<String>,
    pub max_results: Option<u32>,
    pub delimiter: Option<String>,
    pub next_marker: Option<NextMarker>,
    pub blobs: Blobs,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ListBlobsResponseInternal {
    pub prefix: Option<String>,
    pub max_results: Option<u32>,
    pub delimiter: Option<String>,
    pub next_marker: Option<String>,
    pub blobs: Blobs,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Blobs {
    pub blob_prefix: Option<Vec<BlobPrefix>>,
    #[serde(rename = "Blob", default = "Vec::new")]
    pub blobs: Vec<Blob>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlobPrefix {
    pub name: String,
}

impl TryFrom<&http::Response<Bytes>> for ListBlobsResponse {
    type Error = AzureError;

    fn try_from(response: &http::Response<Bytes>) -> Result<Self, Self::Error> {
        let body = to_str_without_bom(response.body())?;

        trace!("body == {}", body);
        let list_blobs_response_internal: ListBlobsResponseInternal = serde_xml_rs::from_str(body)?;

        Ok(Self {
            request_id: request_id_from_headers(response.headers())?,
            date: date_from_headers(response.headers())?,
            prefix: list_blobs_response_internal.prefix,
            max_results: list_blobs_response_internal.max_results,
            delimiter: list_blobs_response_internal.delimiter,
            blobs: list_blobs_response_internal.blobs,
            next_marker: NextMarker::from_possibly_empty_string(
                list_blobs_response_internal.next_marker,
            ),
        })
    }
}
