use crate::from_headers::*;
use crate::Permission;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{etag_from_headers, session_token_from_headers};
use http::HeaderMap;
use std::borrow::Cow;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePermissionResponse<'a> {
    pub permission: Permission<'a, Cow<'a, str>>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl<'a> std::convert::TryFrom<(&HeaderMap, &[u8])> for CreatePermissionResponse<'a> {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

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
