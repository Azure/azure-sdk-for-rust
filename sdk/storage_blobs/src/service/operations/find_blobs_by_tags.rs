use azure_core::prelude::*;
use azure_core::Response as HttpResponse;
use azure_storage::{headers::CommonStorageResponseHeaders, xml::read_xml};

use crate::prelude::BlobServiceClient;

operation! {
    #[stream]
    FindBlobsByTags,
    client: BlobServiceClient,
    expression: String,
    ?next_marker: NextMarker,
    ?max_results: MaxResults
}

impl FindBlobsByTagsBuilder {
    pub fn into_stream(self) -> FindBlobsByTags {
        let make_request = move |next_marker: Option<NextMarker>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.client.storage_client.blob_storage_url().clone();

                url.query_pairs_mut().append_pair("comp", "blobs");
                if let Some(next_marker) = next_marker {
                    url.query_pairs_mut()
                        .append_pair("next", next_marker.as_str());
                }
                url.query_pairs_mut().append_pair("where", &this.expression);
                let mut request = this.client.storage_client.finalize_request(
                    url,
                    azure_core::Method::Get,
                    azure_core::headers::Headers::new(),
                    None,
                )?;

                let response = this.client.send(&mut ctx, &mut request).await?;

                FindBlobsByTagsResponse::try_from(response).await
            }
        };
        azure_core::Pageable::new(make_request)
    }
}

pub type FindBlobsByTags = azure_core::Pageable<FindBlobsByTagsResponse, azure_core::error::Error>;

#[derive(Debug, Clone)]
pub struct FindBlobsByTagsResponse {
    pub blobs: Vec<Blob>,
    pub delimiter: Option<String>,
    next_marker: Option<NextMarker>,
    pub r#where: Option<String>,
    pub common: CommonStorageResponseHeaders,
}

impl Continuable for FindBlobsByTagsResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}

impl FindBlobsByTagsResponse {
    async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        let body: ListBlobsByTagsBody = read_xml(&body)?;

        Ok(Self {
            blobs: body.blobs.blobs,
            delimiter: body.delimiter,
            r#where: body.r#where,
            next_marker: NextMarker::from_possibly_empty_string(body.next_marker),
            common: CommonStorageResponseHeaders::try_from(&headers)?,
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ListBlobsByTagsBody {
    pub delimiter: Option<String>,
    pub next_marker: Option<String>,
    pub r#where: Option<String>,
    pub blobs: Blobs,
}

#[derive(Debug, Clone, Deserialize)]
struct Blobs {
    #[serde(rename = "Blob", default)]
    pub blobs: Vec<Blob>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Blob {
    pub name: String,
    pub container_name: String,
    pub tag_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_body() {
        const BODY: &[u8] = b"<?xml version=\"1.0\" encoding=\"utf-8\"?>
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

        let body: ListBlobsByTagsBody = read_xml(BODY).unwrap();
        assert_eq!(body.blobs.blobs.len(), 1);
        assert_eq!(body.blobs.blobs[0].name, "test1");
    }
}
