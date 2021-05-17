use crate::{headers::consistency_from_headers, AzureStorageError, Consistency};
use azure_core::headers::{
    date_from_headers, request_id_from_headers, request_server_encrypted_from_headers,
};
use azure_core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PutBlockResponse {
    pub consistency: Consistency,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub request_server_encrypted: bool,
}

impl PutBlockResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<PutBlockResponse, AzureStorageError> {
        debug!("{:#?}", headers);

        let consistency = consistency_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;
        let request_server_encrypted = request_server_encrypted_from_headers(headers)?;

        Ok(PutBlockResponse {
            consistency,
            request_id,
            date,
            request_server_encrypted,
        })
    }
}
