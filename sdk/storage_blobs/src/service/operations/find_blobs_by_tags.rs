use crate::prelude::BlobServiceClient;
use azure_core::{prelude::*, Response as HttpResponse};
use azure_storage::headers::CommonStorageResponseHeaders;
use url::Url;

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
                let mut url = this.client.url()?;

                url.query_pairs_mut().append_pair("comp", "blobs");
                if let Some(next_marker) = next_marker {
                    next_marker.append_to_url_query(&mut url);
                }
                url.query_pairs_mut().append_pair("where", &this.expression);

                make_url_compatible_with_api(&mut url);

                let mut request = BlobServiceClient::finalize_request(
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

/// ` AND ` in the spec can be converted to `+AND+`, however, azure rest api won't accept it unless we do a `%20AND%20`
fn make_url_compatible_with_api(url: &mut Url) {
    let compatible_query = url.query().map(|q| q.replace("+", "%20"));

    url.set_query(compatible_query.as_deref());
}

pub type FindBlobsByTags = azure_core::Pageable<FindBlobsByTagsResponse, azure_core::error::Error>;

#[derive(Debug, Clone)]
pub struct FindBlobsByTagsResponse {
    pub blobs: Vec<Blob>,
    pub delimiter: Option<String>,
    pub next_marker: Option<NextMarker>,
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
        let body: ListBlobsByTagsBody = body.xml().await?;

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
    pub tags: Tags,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tags {
    pub tag_set: TagSet,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TagSet {
    #[serde(rename = "Tag", default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::ClientBuilder;
    use azure_core::xml::read_xml;
    use azure_storage::StorageCredentials;
    use futures::StreamExt;
    use std::str::FromStr;

    #[test]
    fn parse_body() -> azure_core::Result<()> {
        const BODY: &[u8] = b"<?xml version=\"1.0\" encoding=\"utf-8\"?>  
        <EnumerationResults ServiceEndpoint=\"http://myaccount.blob.core.windows.net/\">  
          <Where>tag1='value1'</Where>
          <Blobs>  
            <Blob>  
              <Name>test1</Name>  
              <ContainerName>container-name</ContainerName>  
              <Tags>
                <TagSet>
                  <Tag>
                    <Key>matching-tag-name1</Key>
                    <Value>matching-tag-value1</Value>
                  </Tag>
                  <Tag>
                    <Key>matching-tag-name2</Key>
                    <Value>matching-tag-value2</Value>
                  </Tag>
                </TagSet>
              </Tags> 
            </Blob>  
          </Blobs>  
          <NextMarker />  
        </EnumerationResults>";

        let body: ListBlobsByTagsBody = read_xml(BODY)?;

        assert_eq!(body.blobs.blobs.len(), 1);
        assert_eq!(body.blobs.blobs[0].name, "test1");
        assert_eq!(
            body.blobs.blobs[0].tags.tag_set.tags[0].key,
            "matching-tag-name1"
        );
        assert_eq!(
            body.blobs.blobs[0].tags.tag_set.tags[0].value,
            "matching-tag-value1"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_multi_tags_query() {
        let account_name =
            std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
        let key = std::env::var("STORAGE_ACCESS_KEY")
            .expect("Set env variable STORAGE_ACCESS_KEY first!");

        let storage_credentials = StorageCredentials::access_key(&account_name, key.to_string());
        let client_builder = ClientBuilder::new(&account_name, storage_credentials);
        let container_name = "valid-container-name";

        let service_client = client_builder.blob_service_client();

        let query_expression = format!("@container='{container_name}' AND tagis='a'");

        let _filtered_blobs = service_client
            .find_blobs_by_tags(query_expression)
            .into_stream()
            .next()
            .await
            .unwrap()
            .unwrap();
    }

    #[test]
    fn test_make_url_compatible() {
        let mut url = Url::from_str("https://test.blob.core.windows.net/?comp=blobs&where=%40container%3D%27valid-container-name%27+AND+tagis%3D%27a%27").unwrap();

        make_url_compatible_with_api(&mut url);

        assert_eq!("https://test.blob.core.windows.net/?comp=blobs&where=%40container%3D%27valid-container-name%27%20AND%20tagis%3D%27a%27", url.as_str() );
    }
}
