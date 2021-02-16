use crate::blob::blob::Blob;
use azure_core::errors::AzureError;
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::prelude::NextMarker;
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;

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
    #[serde(rename = "Blob")]
    pub blobs: Vec<Blob>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlobPrefix {
    pub name: String,
}

impl ListBlobsResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        body: &str,
    ) -> Result<ListBlobsResponse, AzureError> {
        trace!("body == {}", body);
        let response: ListBlobsResponseInternal = serde_xml_rs::from_str(body)?;

        Ok(Self {
            request_id: request_id_from_headers(headers)?,
            date: date_from_headers(headers)?,
            prefix: response.prefix,
            max_results: response.max_results,
            delimiter: response.delimiter,
            blobs: response.blobs,
            next_marker: NextMarker::from_possibly_empty_string(response.next_marker),
        })
    }
}
