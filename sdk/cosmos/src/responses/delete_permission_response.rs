use crate::headers::from_headers::*;
use azure_core::headers::session_token_from_headers;
use http::response::Response;

#[derive(Debug, Clone, PartialEq)]
pub struct DeletePermissionResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for DeletePermissionResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let headers = response.headers();

        Ok(Self {
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            content_path: content_path_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
        })
    }
}
