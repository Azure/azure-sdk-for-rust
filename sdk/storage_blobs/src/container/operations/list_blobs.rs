use crate::{blob::Blob, prelude::*};
use azure_core::Method;
use azure_core::{
    collect_pinned_stream,
    error::{Error, ErrorKind, ResultExt},
    headers::{date_from_headers, request_id_from_headers},
    prelude::*,
    Pageable, RequestId, Response as AzureResponse,
};
use azure_storage::xml::read_xml;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct ListBlobsBuilder {
    container_client: ContainerClient,
    prefix: Option<Prefix>,
    delimiter: Option<Delimiter>,
    max_results: Option<MaxResults>,
    include_snapshots: bool,
    include_metadata: bool,
    include_uncommitted_blobs: bool,
    include_copy: bool,
    include_deleted: bool,
    include_tags: bool,
    include_versions: bool,
    timeout: Option<Timeout>,
    context: Context,
}

impl ListBlobsBuilder {
    pub(crate) fn new(container_client: ContainerClient) -> Self {
        Self {
            container_client,
            prefix: None,
            delimiter: None,
            max_results: None,
            include_snapshots: false,
            include_metadata: false,
            include_uncommitted_blobs: false,
            include_copy: false,
            include_deleted: false,
            include_tags: false,
            include_versions: false,
            context: Context::new(),
            timeout: None,
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        delimiter: Delimiter => Some(delimiter),
        max_results: MaxResults => Some(max_results),
        include_snapshots: bool => include_snapshots,
        include_metadata: bool => include_metadata,
        include_uncommitted_blobs: bool => include_uncommitted_blobs,
        include_copy: bool => include_copy,
        include_deleted: bool => include_deleted,
        include_tags: bool => include_tags,
        include_versions: bool => include_versions,

        timeout: Timeout => Some(timeout),
    }

    pub fn into_stream(self) -> Pageable<ListBlobsResponse, Error> {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.container_client.url_with_segments(None)?;

                url.query_pairs_mut().append_pair("restype", "container");
                url.query_pairs_mut().append_pair("comp", "list");

                if let Some(continuation) = continuation {
                    url.query_pairs_mut()
                        .append_pair("marker", &continuation.as_string());
                }

                this.prefix.append_to_url_query(&mut url);
                this.delimiter.append_to_url_query(&mut url);
                this.max_results.append_to_url_query(&mut url);

                // This code will construct the "include" query pair
                // attribute. It only allocates a Vec of references ('static
                // str) and, finally, a single string.
                let mut optional_includes = Vec::new();
                if this.include_snapshots {
                    optional_includes.push("snapshots");
                }
                if this.include_metadata {
                    optional_includes.push("metadata");
                }
                if this.include_uncommitted_blobs {
                    optional_includes.push("uncommittedblobs");
                }
                if this.include_copy {
                    optional_includes.push("copy");
                }
                if this.include_deleted {
                    optional_includes.push("deleted");
                }
                if this.include_tags {
                    optional_includes.push("tags");
                }
                if this.include_versions {
                    optional_includes.push("versions");
                }
                if !optional_includes.is_empty() {
                    url.query_pairs_mut()
                        .append_pair("include", &optional_includes.join(","));
                }

                this.timeout.append_to_url_query(&mut url);

                let mut request =
                    this.container_client
                        .prepare_request(url.as_str(), Method::GET, None)?;

                let response = this.container_client.send(&mut ctx, &mut request).await?;

                ListBlobsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListBlobsResponse {
    pub prefix: Option<String>,
    pub max_results: Option<u32>,
    pub delimiter: Option<String>,
    pub next_marker: Option<String>,
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

impl ListBlobsResponse {
    pub async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let list_blobs_response_internal: ListBlobsResponseInternal =
            read_xml(&body).map_kind(ErrorKind::DataConversion)?;

        let next_marker = match list_blobs_response_internal.next_marker {
            Some(ref nm) if nm.is_empty() => None,
            Some(nm) => Some(nm),
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
    fn continuation(&self) -> Option<Continuation> {
        self.next_marker.clone().map(Continuation::from)
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
}
