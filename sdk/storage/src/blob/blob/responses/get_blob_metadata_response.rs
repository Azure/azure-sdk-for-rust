use azure_core::errors::AzureError;
use azure_core::headers::{
    date_from_headers, etag_from_headers, request_id_from_headers, server_from_headers,
};
use azure_core::{Metadata, RequestId};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetBlobMetadataResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: DateTime<Utc>,
    pub metadata: Metadata,
}

impl TryFrom<&HeaderMap> for GetBlobMetadataResponse {
    type Error = AzureError;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:#?}", headers);

        let mut metadata = Metadata::new();
        headers
            .iter()
            .filter(|header| header.0.as_str().starts_with("x-ms-meta-"))
            .for_each(|header| {
                metadata.insert(
                    header
                        .0
                        .as_str()
                        .strip_prefix("x-ms-meta-")
                        .unwrap()
                        .to_owned(),
                    Bytes::from(header.1.as_bytes().to_owned()),
                );
            });

        Ok(GetBlobMetadataResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
            metadata,
        })
    }
}
