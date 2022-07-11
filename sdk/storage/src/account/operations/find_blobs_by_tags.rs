use crate::{core::clients::ServiceType, core::prelude::*, xml::read_xml};
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::prelude::*;
use azure_core::{collect_pinned_stream, RequestId, Response as HttpResponse};
use chrono::{DateTime, Utc};

operation! {
    FindBlobsByTags,
    client: StorageClient,
    expression: String,
    ?next_marker: NextMarker,
    ?max_results: MaxResults
}

impl FindBlobsByTagsBuilder {
    // TODO: Make this a stream instead of a `Future`
    pub fn into_future(mut self) -> FindBlobsByTags {
        Box::pin(async move {
            let mut request = self.client.blob_storage_request(azure_core::Method::Get)?;

            request
                .url_mut()
                .query_pairs_mut()
                .append_pair("comp", "blobs");
            request
                .url_mut()
                .query_pairs_mut()
                .append_pair("where", &self.expression);

            let response = self
                .client
                .send(&mut self.context, &mut request, ServiceType::Blob)
                .await?;

            FindBlobsByTagsResponse::try_from(response).await
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindBlobsByTagsResponse {
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

impl FindBlobsByTagsResponse {
    async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, pinned_stream) = response.deconstruct();
        let body = collect_pinned_stream(pinned_stream).await?;
        let list_blobs_response_internal: ListBlobsByTagsResponseInternal = read_xml(&body)?;

        Ok(Self {
            request_id: request_id_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
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

        let bytes = bytes::Bytes::from(S);
        let _list_blobs_response_internal: ListBlobsByTagsResponseInternal =
            read_xml(&bytes).unwrap();
    }
}
