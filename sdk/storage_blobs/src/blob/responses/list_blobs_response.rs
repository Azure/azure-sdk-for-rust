use crate::blob::Blob;
use azure_core::error::{ErrorKind, Result, ResultExt};
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::prelude::NextMarker;
use azure_core::RequestId;
use azure_storage::xml::read_xml;
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
    type Error = crate::Error;

    fn try_from(response: &http::Response<Bytes>) -> Result<Self> {
        let body = response.body();

        trace!("body == {:?}", body);
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

#[cfg(test)]
mod tests {
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
