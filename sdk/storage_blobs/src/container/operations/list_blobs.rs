use crate::{blob::Blob, prelude::*};
use azure_core::{
    error::{Error, ErrorKind, ResultExt},
    headers::{date_from_headers, request_id_from_headers},
    prelude::{NextMarker, *},
    CollectedResponse, RequestId,
};
use azure_storage::xml::read_xml;
use chrono::{DateTime, Utc};
use futures::stream::{unfold, Stream};
use http::method::Method;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct ListBlobsBuilder<'a> {
    container_client: &'a ContainerClient,
    prefix: Option<Prefix>,
    delimiter: Option<Delimiter>,
    next_marker: Option<NextMarker>,
    max_results: Option<MaxResults>,
    include_snapshots: bool,
    include_metadata: bool,
    include_uncommitted_blobs: bool,
    include_copy: bool,
    include_deleted: bool,
    include_tags: bool,
    include_versions: bool,
    client_request_id: Option<ClientRequestId>,
    timeout: Option<Timeout>,
}

impl<'a> ListBlobsBuilder<'a> {
    pub(crate) fn new(container_client: &'a ContainerClient) -> Self {
        Self {
            container_client,
            prefix: None,
            delimiter: None,
            next_marker: None,
            max_results: None,
            include_snapshots: false,
            include_metadata: false,
            include_uncommitted_blobs: false,
            include_copy: false,
            include_deleted: false,
            include_tags: false,
            include_versions: false,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        delimiter: Delimiter => Some(delimiter),
        next_marker: NextMarker => Some(next_marker),
        max_results: MaxResults => Some(max_results),
        include_snapshots: bool => include_snapshots,
        include_metadata: bool => include_metadata,
        include_uncommitted_blobs: bool => include_uncommitted_blobs,
        include_copy: bool => include_copy,
        include_deleted: bool => include_deleted,
        include_tags: bool => include_tags,
        include_versions: bool => include_versions,
        client_request_id: ClientRequestId => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    async fn execute(&self) -> azure_core::Result<ListBlobsResponse> {
        let mut url = self.container_client.url_with_segments(None)?;

        url.query_pairs_mut().append_pair("restype", "container");
        url.query_pairs_mut().append_pair("comp", "list");

        self.prefix.append_to_url_query(&mut url);
        self.delimiter.append_to_url_query(&mut url);
        self.next_marker.append_to_url_query(&mut url);
        self.max_results.append_to_url_query(&mut url);

        // This code will construct the "include" query pair
        // attribute. It only allocates a Vec of references ('static
        // str) and, finally, a single string.
        let mut optional_includes = Vec::new();
        if self.include_snapshots {
            optional_includes.push("snapshots");
        }
        if self.include_metadata {
            optional_includes.push("metadata");
        }
        if self.include_uncommitted_blobs {
            optional_includes.push("uncommittedblobs");
        }
        if self.include_copy {
            optional_includes.push("copy");
        }
        if self.include_deleted {
            optional_includes.push("deleted");
        }
        if self.include_tags {
            optional_includes.push("tags");
        }
        if self.include_versions {
            optional_includes.push("versions");
        }
        if !optional_includes.is_empty() {
            url.query_pairs_mut()
                .append_pair("include", &optional_includes.join(","));
        }

        self.timeout.append_to_url_query(&mut url);

        let mut request = self
            .container_client
            .prepare_request(url.as_str(), Method::GET, None)?;
        request.add_optional_header(&self.client_request_id);

        let response = self
            .container_client
            .storage_client()
            .storage_account_client()
            .http_client()
            .execute_request_check_status(&request)
            .await?;

        response.try_into()
    }

    pub fn stream(self) -> impl Stream<Item = azure_core::Result<ListBlobsResponse>> + 'a {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(NextMarker),
        }

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                let response = match next_marker {
                    Some(States::Init) => req.execute().await,
                    Some(States::NextMarker(next_marker)) => {
                        req.next_marker(next_marker).execute().await
                    }
                    None => return None,
                };

                // the ? operator does not work in async move (yet?)
                // so we have to resort to this boilerplate
                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = response.next_marker.clone().map(States::NextMarker);

                Some((Ok(response), next_marker))
            }
        })
    }
}

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

impl TryFrom<CollectedResponse> for ListBlobsResponse {
    type Error = Error;

    fn try_from(response: CollectedResponse) -> azure_core::Result<Self> {
        let body = response.body();

        let list_blobs_response_internal: ListBlobsResponseInternal =
            read_xml(body).map_kind(ErrorKind::DataConversion)?;

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

pub type Response = futures::future::BoxFuture<'static, azure_core::Result<ListBlobsResponse>>;

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
