use crate::from_headers::*;
use crate::User;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{etag_from_headers, session_token_from_headers};
use http::HeaderMap;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUserResponse {
    pub user: User,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for CreateUserResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        Ok(Self {
            user: body.try_into()?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
        })
    }
}
