use crate::blob::Blob;
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::{date_from_headers, request_id_from_headers, RequestId};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub blob: Blob,
    pub request_id: RequestId,
    pub data: Vec<u8>,
    pub date: DateTime<Utc>,
}

impl GetBlobResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        blob: Blob,
        body: &[u8],
    ) -> Result<GetBlobResponse, AzureError> {
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(GetBlobResponse {
            blob,
            request_id,
            data: body.to_vec(),
            date,
        })
    }
}
