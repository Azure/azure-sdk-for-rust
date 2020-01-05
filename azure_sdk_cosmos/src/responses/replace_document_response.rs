use crate::document_attributes::DocumentAttributes;
use crate::DocumentAdditionalHeaders;
use azure_sdk_core::errors::AzureError;
use http::HeaderMap;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplaceDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub additional_headers: DocumentAdditionalHeaders,
}

impl std::convert::TryFrom<(&HeaderMap, &[u8])> for ReplaceDocumentResponse {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:?}", headers);
        debug!("body == {:#?}", body);

        Ok(Self {
            document_attributes: value.try_into()?,
            additional_headers: value.try_into()?,
        })
    }
}
