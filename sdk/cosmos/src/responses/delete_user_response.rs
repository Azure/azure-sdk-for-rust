use crate::from_headers::*;
use crate::CosmosError;
use http::response::Response;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteUserResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
}

impl std::convert::TryFrom<Response<Vec<u8>>> for DeleteUserResponse {
    type Error = CosmosError;

    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let headers = response.headers();

        Ok(Self {
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
        })
    }
}
