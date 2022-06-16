use azure_core::{
    headers::{
        client_request_id_from_headers_optional, request_id_from_headers, version_from_headers,
    },
    RequestId,
};
use http::HeaderMap;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct SetBlobTierResponse {
    pub request_id: RequestId,
    pub client_request_id: Option<String>,
    pub version: String,
}

impl TryFrom<&HeaderMap> for SetBlobTierResponse {
    type Error = crate::Error;

    fn try_from(headers: &HeaderMap) -> Result<Self, Self::Error> {
        debug!("headers == {:#?}", headers);

        Ok(SetBlobTierResponse {
            request_id: request_id_from_headers(headers)?,
            client_request_id: client_request_id_from_headers_optional(headers),
            version: version_from_headers(headers)?.to_owned(),
        })
    }
}
