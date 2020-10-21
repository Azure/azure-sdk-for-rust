use crate::from_headers::*;
use azure_core::errors::AzureError;
use azure_core::headers::session_token_from_headers;
use hyper::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for DeleteDocumentResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let _body = value.1;

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
