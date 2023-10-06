use crate::{blob::Blob, prelude::*};
use azure_core::Method;
use azure_core::{
    error::Error,
    headers::{date_from_headers, request_id_from_headers, Headers},
    prelude::*,
    xml::read_xml,
    Pageable, RequestId, Response as AzureResponse,
};
use time::OffsetDateTime;

operation! {
    #[stream]
    ListBlobs,
    client: ContainerClient,
    ?prefix: Prefix,
    ?delimiter: Delimiter,
    ?max_results: MaxResults,
    ?include_snapshots: bool,
    ?include_metadata: bool,
    ?include_uncommitted_blobs: bool,
    ?include_copy: bool,
    ?include_deleted: bool,
    ?include_tags: bool,
    ?include_versions: bool,
}

impl ListBlobsBuilder {
    pub fn into_stream(self) -> Pageable<ListBlobsResponse, Error> {
        let make_request = move |continuation: Option<NextMarker>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.client.url()?;

                url.query_pairs_mut().append_pair("restype", "container");
                url.query_pairs_mut().append_pair("comp", "list");

                if let Some(next_marker) = continuation {
                    next_marker.append_to_url_query(&mut url);
                }

                this.prefix.append_to_url_query(&mut url);
                this.delimiter.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);

                // This code will construct the "include" query pair
                // attribute. It only allocates a Vec of references ('static
                // str) and, finally, a single string.
                let mut optional_includes = Vec::new();
                if this.include_snapshots.unwrap_or(false) {
                    optional_includes.push("snapshots");
                }
                if this.include_metadata.unwrap_or(false) {
                    optional_includes.push("metadata");
                }
                if this.include_uncommitted_blobs.unwrap_or(false) {
                    optional_includes.push("uncommittedblobs");
                }
                if this.include_copy.unwrap_or(false) {
                    optional_includes.push("copy");
                }
                if this.include_deleted.unwrap_or(false) {
                    optional_includes.push("deleted");
                }
                if this.include_tags.unwrap_or(false) {
                    optional_includes.push("tags");
                }
                if this.include_versions.unwrap_or(false) {
                    optional_includes.push("versions");
                }
                if !optional_includes.is_empty() {
                    url.query_pairs_mut()
                        .append_pair("include", &optional_includes.join(","));
                }

                let mut request =
                    ContainerClient::finalize_request(url, Method::Get, Headers::new(), None)?;

                let response = this.client.send(&mut ctx, &mut request).await?;

                ListBlobsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBlobsResponse {
    pub prefix: Option<String>,
    pub max_results: Option<u32>,
    pub delimiter: Option<String>,
    pub next_marker: Option<NextMarker>,
    pub blobs: Blobs,
    pub request_id: RequestId,
    pub date: OffsetDateTime,
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Blobs {
    #[serde(rename = "$value", default)]
    pub items: Vec<BlobItem>,
}

impl Blobs {
    pub fn blobs(&self) -> impl Iterator<Item = &Blob> {
        self.items.iter().filter_map(|item| match item {
            BlobItem::Blob(blob) => Some(blob),
            BlobItem::BlobPrefix(_) => None,
        })
    }

    pub fn prefixes(&self) -> impl Iterator<Item = &BlobPrefix> {
        self.items.iter().filter_map(|item| match item {
            BlobItem::BlobPrefix(prefix) => Some(prefix),
            BlobItem::Blob(_) => None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(clippy::large_enum_variant)]
pub enum BlobItem {
    Blob(Blob),
    BlobPrefix(BlobPrefix),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlobPrefix {
    pub name: String,
}

impl ListBlobsResponse {
    pub async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let list_blobs_response_internal: ListBlobsResponseInternal = read_xml(&body)?;

        let next_marker = match list_blobs_response_internal.next_marker {
            Some(ref nm) if nm.is_empty() => None,
            Some(nm) => Some(nm.into()),
            None => None,
        };

        Ok(Self {
            request_id: request_id_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
            prefix: list_blobs_response_internal.prefix,
            max_results: list_blobs_response_internal.max_results,
            delimiter: list_blobs_response_internal.delimiter,
            blobs: list_blobs_response_internal.blobs,
            next_marker,
        })
    }
}

impl Continuable for ListBlobsResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;

    use super::*;

    #[test]
    fn deserde_azure() {
        const S: &str = "<?xml version=\"1.0\" encoding=\"utf-8\"?>
<EnumerationResults ServiceEndpoint=\"https://azureskdforrust.blob.core.windows.net/\" ContainerName=\"osa2\">
    <Blobs>
        <Blob>
            <Name>blob0.txt</Name>
            <Properties>
                <Creation-Time>Thu, 01 Jul 2021 10:44:59 GMT</Creation-Time>
                <Last-Modified>Thu, 01 Jul 2021 10:44:59 GMT</Last-Modified>
                <Expiry-Time>Thu, 07 Jul 2022 14:38:48 GMT</Expiry-Time>
                <Etag>0x8D93C7D4629C227</Etag>
                <Content-Length>8</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-Encoding />
                <Content-Language />
                <Content-CRC64 />
                <Content-MD5>rvr3UC1SmUw7AZV2NqPN0g==</Content-MD5>
                <Cache-Control />
                <Content-Disposition />
                <BlobType>BlockBlob</BlobType>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
            </Properties>
            <Metadata><userkey>uservalue</userkey></Metadata>
            <OrMetadata />
        </Blob>
        <Blob>
            <Name>blob1.txt</Name>
            <Properties>
                <Creation-Time>Thu, 01 Jul 2021 10:44:59 GMT</Creation-Time>
                <Last-Modified>Thu, 01 Jul 2021 10:44:59 GMT</Last-Modified>
                <Etag>0x8D93C7D463004D6</Etag>
                <Content-Length>8</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-Encoding />
                <Content-Language />
                <Content-CRC64 />
                <Content-MD5>rvr3UC1SmUw7AZV2NqPN0g==</Content-MD5>
                <Cache-Control />
                <Content-Disposition />
                <BlobType>BlockBlob</BlobType>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
            </Properties>
            <OrMetadata />
        </Blob>
        <Blob>
            <Name>blob2.txt</Name>
            <Properties>
                <Creation-Time>Thu, 01 Jul 2021 10:44:59 GMT</Creation-Time>
                <Last-Modified>Thu, 01 Jul 2021 10:44:59 GMT</Last-Modified>
                <Etag>0x8D93C7D4636478A</Etag>
                <Content-Length>8</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-Encoding />
                <Content-Language />
                <Content-CRC64 />
                <Content-MD5>rvr3UC1SmUw7AZV2NqPN0g==</Content-MD5>
                <Cache-Control />
                <Content-Disposition />
                <BlobType>BlockBlob</BlobType>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
            </Properties>
            <OrMetadata />
        </Blob>
    </Blobs>
    <NextMarker />
</EnumerationResults>";

        let bytes = Bytes::from(S);
        let _list_blobs_response_internal: ListBlobsResponseInternal = read_xml(&bytes).unwrap();
    }

    #[test]
    fn deserde_azurite() {
        const S: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
<EnumerationResults ServiceEndpoint=\"http://127.0.0.1:10000/devstoreaccount1\" ContainerName=\"osa2\">
    <Prefix/>
    <Marker/>
    <MaxResults>5000</MaxResults>
    <Delimiter/>
    <Blobs>
        <Blob>
            <Name>blob0.txt</Name>
            <Properties>
                <Creation-Time>Thu, 01 Jul 2021 10:45:02 GMT</Creation-Time>
                <Last-Modified>Thu, 01 Jul 2021 10:45:02 GMT</Last-Modified>
                <Etag>0x228281B5D517B20</Etag>
                <Content-Length>8</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-MD5>rvr3UC1SmUw7AZV2NqPN0g==</Content-MD5>
                <BlobType>BlockBlob</BlobType>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <AccessTierChangeTime>Thu, 01 Jul 2021 10:45:02 GMT</AccessTierChangeTime>
            </Properties>
        </Blob>
        <Blob>
            <Name>blob1.txt</Name>
            <Properties>
                <Creation-Time>Thu, 01 Jul 2021 10:45:02 GMT</Creation-Time>
                <Last-Modified>Thu, 01 Jul 2021 10:45:02 GMT</Last-Modified>
                <Etag>0x1DD959381A8A860</Etag>
                <Content-Length>8</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-MD5>rvr3UC1SmUw7AZV2NqPN0g==</Content-MD5>
                <BlobType>BlockBlob</BlobType>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <AccessTierChangeTime>Thu, 01 Jul 2021 10:45:02 GMT</AccessTierChangeTime>
            </Properties>
        </Blob>
        <Blob>
            <Name>blob2.txt</Name>
            <Properties>
                <Creation-Time>Thu, 01 Jul 2021 10:45:02 GMT</Creation-Time>
                <Last-Modified>Thu, 01 Jul 2021 10:45:02 GMT</Last-Modified>
                <Etag>0x1FBE9C9B0C7B650</Etag>
                <Content-Length>8</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-MD5>rvr3UC1SmUw7AZV2NqPN0g==</Content-MD5>
                <BlobType>BlockBlob</BlobType>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <AccessTierChangeTime>Thu, 01 Jul 2021 10:45:02 GMT</AccessTierChangeTime>
            </Properties>
        </Blob>
    </Blobs>
    <NextMarker/>
</EnumerationResults>";

        let bytes = Bytes::from(S);
        let _list_blobs_response_internal: ListBlobsResponseInternal = read_xml(&bytes).unwrap();
    }

    #[test]
    fn deserde_properties_with_non_existent_field() {
        const XML: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
        <EnumerationResults ServiceEndpoint=\"http://127.0.0.1:10000/devstoreaccount1\" ContainerName=\"temp\">
            <Prefix>b39bc5c9-0f31-459c-a271-828467105470/</Prefix>
            <Marker/>
            <MaxResults>5000</MaxResults>
            <Delimiter/>
            <Blobs>
                <Blob>
                    <Name>b39bc5c9-0f31-459c-a271-828467105470/corrupted_data_2020-01-02T03_04_05.json</Name>
                    <Properties>
                        <Creation-Time>Mon, 02 Oct 2023 20:00:31 GMT</Creation-Time>
                        <Last-Modified>Mon, 02 Oct 2023 20:00:31 GMT</Last-Modified>
                        <Etag>0x23D9DB658CF7480</Etag>
                        <Content-Length>0</Content-Length>
                        <Content-Type>application/octet-stream</Content-Type>
                        <Content-Encoding/>
                        <Content-Language/>
                        <Content-CRC64/>
                        <Content-MD5/>
                        <Cache-Control/>
                        <Content-Disposition/>
                        <BlobType>BlockBlob</BlobType>
                        <AccessTier>Hot</AccessTier>
                        <AccessTierInferred>true</AccessTierInferred>
                        <LeaseStatus>unlocked</LeaseStatus>
                        <LeaseState>available</LeaseState>
                        <ServerEncrypted>true</ServerEncrypted>
                        <ResourceType>file</ResourceType>
                        <NotRealProperty>notRealValue</NotRealProperty> 
                    </Properties>
                </Blob>
            </Blobs>
            <NextMarker/>
        </EnumerationResults>";

        let bytes = Bytes::from(XML);
        let _list_blobs_response_internal: ListBlobsResponseInternal = read_xml(&bytes).unwrap();
    }

    #[test]
    fn deserde_azurite_without_server_encrypted() {
        const S: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>
        <EnumerationResults ServiceEndpoint=\"http://127.0.0.1:10000/devstoreaccount1\" ContainerName=\"temp\">
            <Prefix>b39bc5c9-0f31-459c-a271-828467105470/</Prefix>
            <Marker/>
            <MaxResults>5000</MaxResults>
            <Delimiter/>
            <Blobs>
                <Blob>
                    <Name>b39bc5c9-0f31-459c-a271-828467105470/corrupted_data_2020-01-02T03_04_05.json</Name>
                    <Properties>
                        <Creation-Time>Sat, 18 Feb 2023 22:39:00 GMT</Creation-Time>
                        <Last-Modified>Sat, 18 Feb 2023 22:39:00 GMT</Last-Modified>
                        <Etag>0x23D9DB658CF7480</Etag>
                        <Content-Length>64045</Content-Length>
                        <Content-Type>application/octet-stream</Content-Type>
                        <BlobType>BlockBlob</BlobType>
                        <LeaseStatus>unlocked</LeaseStatus>
                        <LeaseState>available</LeaseState>
                        <AccessTier>Hot</AccessTier>
                        <AccessTierInferred>true</AccessTierInferred>
                    </Properties>
                </Blob>
            </Blobs>
            <NextMarker/>
        </EnumerationResults>";

        let bytes = Bytes::from(S);
        let _list_blobs_response_internal: ListBlobsResponseInternal = read_xml(&bytes).unwrap();
    }

    #[test]
    fn parse_xml_with_blob_prefix() {
        const XML: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
        <EnumerationResults ServiceEndpoint="https://sisuautomatedtest.blob.core.windows.net/" ContainerName="lowlatencyrequests">
          <Prefix>get-most-recent-key-5/</Prefix>
          <Delimiter>/</Delimiter>
          <Blobs>
            <Blob>
              <Name>get-most-recent-key-5/2021-08-04-testfile1</Name>
              <Properties>
                <Creation-Time>Tue, 13 Sep 2022 08:20:48 GMT</Creation-Time>
                <Last-Modified>Tue, 13 Sep 2022 08:20:48 GMT</Last-Modified>
                <Etag>0x8DA9560DD170CFD</Etag>
                <Content-Length>19</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-Encoding />
                <Content-Language />
                <Content-CRC64 />
                <Content-MD5>3X/+gWTy92gIJFXx57gLYA==</Content-MD5>
                <Cache-Control />
                <Content-Disposition />
                <BlobType>BlockBlob</BlobType>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
              </Properties>
              <OrMetadata />
            </Blob>
            <BlobPrefix>
              <Name>get-most-recent-key-5/2021-08-04T21:48:48.592953Z-15839722113750148182/</Name>
            </BlobPrefix>
            <Blob>
              <Name>get-most-recent-key-5/2021-09-04-testfile2</Name>
              <Properties>
                <Creation-Time>Tue, 13 Sep 2022 08:07:01 GMT</Creation-Time>
                <Last-Modified>Tue, 13 Sep 2022 08:19:21 GMT</Last-Modified>
                <Etag>0x8DA9560A916932D</Etag>
                <Content-Length>19</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-Encoding />
                <Content-Language />
                <Content-CRC64 />
                <Content-MD5>b0CPJB6eDfKUzzW7dlboKQ==</Content-MD5>
                <Cache-Control />
                <Content-Disposition />
                <BlobType>BlockBlob</BlobType>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
              </Properties>
              <OrMetadata />
            </Blob>
            <Blob>
              <Name>get-most-recent-key-5/2022-08-04-testfile3</Name>
              <Properties>
                <Creation-Time>Tue, 13 Sep 2022 08:07:01 GMT</Creation-Time>
                <Last-Modified>Tue, 13 Sep 2022 08:19:21 GMT</Last-Modified>
                <Etag>0x8DA9560A91F9296</Etag>
                <Content-Length>34</Content-Length>
                <Content-Type>text/plain</Content-Type>
                <Content-Encoding />
                <Content-Language />
                <Content-CRC64 />
                <Content-MD5>1F1MssyZOvhY4OZevHWEsw==</Content-MD5>
                <Cache-Control />
                <Content-Disposition />
                <BlobType>BlockBlob</BlobType>
                <AccessTier>Hot</AccessTier>
                <AccessTierInferred>true</AccessTierInferred>
                <LeaseStatus>unlocked</LeaseStatus>
                <LeaseState>available</LeaseState>
                <ServerEncrypted>true</ServerEncrypted>
              </Properties>
              <OrMetadata />
            </Blob>
          </Blobs>
          <NextMarker />
        </EnumerationResults>"#;

        let _list_blobs_response_internal: ListBlobsResponseInternal = read_xml(XML).unwrap();
    }
}
