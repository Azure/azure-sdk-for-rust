use crate::document_attributes::DocumentAttributes;
use crate::DocumentAdditionalHeaders;
use azure_sdk_core::errors::AzureError;
use http::{HeaderMap, StatusCode};

#[derive(Debug, Clone)]
pub struct CreateDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub additional_headers: DocumentAdditionalHeaders,
    pub is_update: bool,
}

impl std::convert::TryFrom<(StatusCode, &HeaderMap, &[u8])> for CreateDocumentResponse {
    type Error = AzureError;
    fn try_from(value: (StatusCode, &HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let status_code = value.0;
        let headers = value.1;
        let body = value.2;

        Ok(CreateDocumentResponse {
            document_attributes: DocumentAttributes::try_from((headers, body))?,
            additional_headers: DocumentAdditionalHeaders::try_from((headers, body))?,
            is_update: status_code == StatusCode::OK,
        })
    }
}
