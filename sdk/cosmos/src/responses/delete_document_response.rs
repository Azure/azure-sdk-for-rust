use crate::headers::from_headers::*;
use azure_core::headers::session_token_from_headers;
use http::response::Response;

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for DeleteDocumentResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
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
