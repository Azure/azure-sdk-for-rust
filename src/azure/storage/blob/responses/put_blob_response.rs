use azure::core::errors::AzureError;
use azure::core::headers::REQUEST_ID;
use azure::core::RequestId;
use chrono::{DateTime, Utc};
use http::HeaderMap;
use hyper::header::{ETAG, LAST_MODIFIED};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PutBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
}

impl PutBlobResponse {
    pub fn from_headers(headers: &HeaderMap) -> Result<PutBlobResponse, AzureError> {
        let etag = headers
            .get(ETAG)
            .ok_or_else(|| AzureError::HeaderNotFound(ETAG.as_str().to_owned()))?
            .to_str()?
            .to_owned();

        let last_modified = headers
            .get(LAST_MODIFIED)
            .ok_or_else(|| AzureError::HeaderNotFound(LAST_MODIFIED.as_str().to_owned()))?
            .to_str()?;
        let last_modified = DateTime::parse_from_rfc2822(last_modified)?;
        let last_modified = DateTime::from_utc(last_modified.naive_utc(), Utc);
        trace!("last_modified == {:?}", last_modified);

        let request_id = headers
            .get(REQUEST_ID)
            .ok_or_else(|| AzureError::HeaderNotFound(REQUEST_ID.to_owned()))?
            .to_str()?;

        let request_id = Uuid::parse_str(request_id)?;

        Ok(PutBlobResponse {
            etag,
            last_modified,
            request_id,
        })
    }
}
