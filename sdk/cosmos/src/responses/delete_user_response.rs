use crate::from_headers::*;
use azure_core::errors::AzureError;
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DeleteUserResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DeleteUserResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let _body = value.1;

        Ok(Self {
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
        })
    }
}
