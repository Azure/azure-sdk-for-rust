use crate::headers::from_headers::*;
use crate::resources::Permission;
use crate::CosmosError;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePermissionResponse<'a> {
    pub permission: Permission<'a>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl<'a> std::convert::TryFrom<Response<Vec<u8>>> for CreatePermissionResponse<'a> {
    type Error = CosmosError;

    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body: &[u8] = response.body();

        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", std::str::from_utf8(body)?);

        Ok(Self {
            permission: body.try_into()?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            content_path: content_path_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
        })
    }
}
