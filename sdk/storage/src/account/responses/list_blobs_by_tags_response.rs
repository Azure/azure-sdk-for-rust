use crate::xml::read_xml;
use azure_core::error::{Error, Result};
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::prelude::NextMarker;
use azure_core::RequestId;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;

#[derive(Debug, Clone, PartialEq)]
pub struct ListBlobsByTagsResponse {
    pub max_results: Option<u32>,
    pub delimiter: Option<String>,
    pub next_marker: Option<NextMarker>,
    pub r#where: Option<String>,
    pub blobs: Blobs,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ListBlobsByTagsResponseInternal {
    pub max_results: Option<u32>,
    pub delimiter: Option<String>,
    pub next_marker: Option<String>,
    pub r#where: Option<String>,
    pub blobs: Blobs,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Blobs {
    #[serde(rename = "Blob", default = "Vec::new")]
    pub blobs: Vec<Blob>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Blob {
    pub name: String,
    pub container_name: String,
    pub tag_value: String,
}

impl TryFrom<&http::Response<Bytes>> for ListBlobsByTagsResponse {
    type Error = Error;

    fn try_from(response: &http::Response<Bytes>) -> Result<Self> {
        let body = response.body();

        trace!("body == {:?}", body);
        let list_blobs_response_internal: ListBlobsByTagsResponseInternal = read_xml(body)?;

        Ok(Self {
            request_id: request_id_from_headers(response.headers())?,
            date: date_from_headers(response.headers())?,
            max_results: list_blobs_response_internal.max_results,
            delimiter: list_blobs_response_internal.delimiter,
            r#where: list_blobs_response_internal.r#where,
            blobs: list_blobs_response_internal.blobs,
            next_marker: NextMarker::from_possibly_empty_string(
                list_blobs_response_internal.next_marker,
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserde_azure() {
        const S: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
        <EnumerationResults ServiceEndpoint=\"https://hsdgeventstoredev.blob.core.windows.net/\">
          <Where>tag1='value1'</Where>
          <Blobs>
            <Blob>
              <Name>test1</Name>
              <ContainerName>container1</ContainerName>
              <TagValue>value1</TagValue>
            </Blob>
          </Blobs>
          <NextMarker/>
        </EnumerationResults>";

        let bytes = Bytes::from(S);
        let _list_blobs_response_internal: ListBlobsByTagsResponseInternal =
            read_xml(&bytes).unwrap();
    }
}
