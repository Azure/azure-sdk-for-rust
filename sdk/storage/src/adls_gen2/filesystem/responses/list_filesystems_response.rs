use crate::filesystem::{incomplete_vector_from_response, Filesystem};
use azure_core::errors::AzureError;
use azure_core::headers::{
    content_type_from_headers, date_from_headers, request_id_from_headers, version_from_headers,
};
use azure_core::incompletevector::IncompleteVector;
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;

pub struct ListFilesystemsResponse {
    pub incomplete_vector: IncompleteVector<Filesystem>,
    pub date: DateTime<Utc>,
    pub request_id: RequestId,
    pub version: String,
    pub content_type: String,
}

impl ListFilesystemsResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        body: &str,
    ) -> Result<ListFilesystemsResponse, AzureError> {
        let incomplete_vector = incomplete_vector_from_response(headers, body)?;
        let date = date_from_headers(&headers)?;
        let request_id = request_id_from_headers(&headers)?;
        let version = version_from_headers(&headers)?.to_owned();
        let content_type = content_type_from_headers(&headers)?.to_owned();

        Ok(ListFilesystemsResponse {
            incomplete_vector,
            date,
            request_id,
            version,
            content_type,
        })
    }
}
