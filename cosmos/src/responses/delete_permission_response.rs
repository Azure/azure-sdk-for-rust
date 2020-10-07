use crate::from_headers::*;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::session_token_from_headers;
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DeletePermissionResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DeletePermissionResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let _body = value.1;

        debug!("headers == {:#?}", headers);
        debug!("_body == {:#?}", std::str::from_utf8(_body)?);

        Ok(Self {
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            content_path: content_path_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
        })
    }
}
