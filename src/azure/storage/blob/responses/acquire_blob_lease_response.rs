use azure::core::errors::AzureError;
use azure::core::lease::LeaseId;
use azure::core::RequestId;
use azure::core::{date_from_headers, etag_from_headers, last_modified_from_headers, lease_id_from_headers, request_id_from_headers};
use chrono::{DateTime, Utc};
use hyper::HeaderMap;

#[derive(Debug, Clone)]
pub struct AcquireBlobLeaseResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub lease_id: LeaseId,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
}

impl AcquireBlobLeaseResponse {
    pub(crate) fn from_headers(headers: &HeaderMap) -> Result<AcquireBlobLeaseResponse, AzureError> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let lease_id = lease_id_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        Ok(AcquireBlobLeaseResponse {
            etag,
            last_modified,
            lease_id,
            request_id,
            date,
        })
    }
}
