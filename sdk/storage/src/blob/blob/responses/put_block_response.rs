use azure_core::errors::AzureError;
use azure_core::headers::{
    consistency_from_headers, date_from_headers, request_id_from_headers,
    request_server_encrypted_from_headers,
};
use azure_core::{ConsistencyCRC64, ConsistencyMD5, RequestId};
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockResponse {
    pub content_md5: ConsistencyMD5,
    pub content_crc64: Option<ConsistencyCRC64>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlockResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<PutBlockResponse, AzureError> {
        debug!("{:#?}", headers);

        let (content_md5, content_crc64) = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockResponse {
            content_md5,
            content_crc64,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
