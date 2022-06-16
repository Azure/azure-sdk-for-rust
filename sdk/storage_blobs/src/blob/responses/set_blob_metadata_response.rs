use azure_core::{
    headers::{date_from_headers, etag_from_headers, request_id_from_headers, server_from_headers},
    RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct SetBlobMetadataResponse {
    pub request_id: RequestId,
    pub etag: String,
    pub server: String,
    pub date: DateTime<Utc>,
}

impl TryFrom<&HeaderMap> for SetBlobMetadataResponse {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:#?}", headers);

        Ok(SetBlobMetadataResponse {
            request_id: request_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            server: server_from_headers(headers)?.to_owned(),
            date: date_from_headers(headers)?,
        })
    }
}
