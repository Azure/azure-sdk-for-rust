use azure::core::errors::AzureError;
use azure::core::headers::{CONTENT_MD5, REQUEST_ID};
use azure::core::RequestId;
use base64;
use chrono::{DateTime, Utc};
use http::HeaderMap;
use hyper::header::{ETAG, LAST_MODIFIED};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PutBlockBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: [u8; 16],
    pub request_id: RequestId,
}

impl PutBlockBlobResponse {
    pub fn from_headers(headers: &HeaderMap) -> Result<PutBlockBlobResponse, AzureError> {
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

        let content_md5 = headers
            .get(CONTENT_MD5)
            .ok_or_else(|| AzureError::HeaderNotFound(CONTENT_MD5.to_owned()))?
            .to_str()?;

        let content_md5_vec = base64::decode(&content_md5)?;

        if content_md5_vec.len() != 16 {
            return Err(AzureError::DigestNot16BytesLong(content_md5_vec.len() as u64));
        }
        let mut content_md5 = [0; 16];
        content_md5.copy_from_slice(&content_md5_vec[0..16]);

        let request_id = headers
            .get(REQUEST_ID)
            .ok_or_else(|| AzureError::HeaderNotFound(REQUEST_ID.to_owned()))?
            .to_str()?;

        let request_id = Uuid::parse_str(request_id)?;

        Ok(PutBlockBlobResponse {
            etag,
            last_modified,
            content_md5,
            request_id,
        })
    }
}
