use crate::from_headers::*;
use crate::CosmosError;
use azure_core::headers::session_token_from_headers;
use http::response::Response;

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl std::convert::TryFrom<Response<Vec<u8>>> for DeleteDocumentResponse {
    type Error = CosmosError;

    fn try_from(response: Response<Vec<u8>>) -> Result<Self, Self::Error> {
        let headers = response.headers();

        let charge = request_charge_from_headers(headers)?;
        let activity_id = activity_id_from_headers(headers)?;
        let session_token = session_token_from_headers(headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token,
        })
    }
}
