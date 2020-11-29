use crate::from_headers::*;
use crate::resources::User;
use crate::CosmosError;
use azure_core::headers::{etag_from_headers, session_token_from_headers};
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateUserResponse {
    pub user: User,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub etag: String,
    pub session_token: String,
}

impl std::convert::TryFrom<Response<Vec<u8>>> for CreateUserResponse {
    type Error = CosmosError;

    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let headers = response.headers();
        let body: &[u8] = response.body();

        Ok(Self {
            user: body.try_into()?,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            etag: etag_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
        })
    }
}
