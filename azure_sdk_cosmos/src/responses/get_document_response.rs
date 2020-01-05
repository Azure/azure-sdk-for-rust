use crate::{Document, DocumentAdditionalHeaders};
use azure_sdk_core::errors::AzureError;
use http::StatusCode;
use hyper::header::HeaderMap;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct GetDocumentResponse<T> {
    pub document: Option<Document<T>>,
    pub additional_headers: DocumentAdditionalHeaders,
    pub has_been_found: bool,
}

impl<T> std::convert::TryFrom<(StatusCode, &HeaderMap, &[u8])> for GetDocumentResponse<T>
where
    T: DeserializeOwned,
{
    type Error = AzureError;
    fn try_from(value: (StatusCode, &HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let status_code = value.0;
        let headers = value.1;
        let body = value.2;

        debug!("status_code == {:?}", status_code);
        debug!("headers == {:?}", headers);
        debug!("body == {:?}", std::str::from_utf8(body)?);

        let additional_headers = DocumentAdditionalHeaders::try_from(headers)?;

        let document = if status_code == StatusCode::NOT_FOUND {
            None
        } else {
            Some(Document::try_from((headers, body))?)
        };

        Ok(Self {
            additional_headers,
            document,
            has_been_found: status_code == StatusCode::OK
                || status_code == StatusCode::NOT_MODIFIED,
        })
    }
}
