use azure::core::errors::AzureError;
use azure::core::headers::CONTENT_MD5;
use azure::core::RequestId;
use azure::storage::blob::responses::PutBlobResponse;
use base64;
use chrono::{DateTime, Utc};
use http::HeaderMap;

#[derive(Debug, Clone)]
pub struct PutBlockBlobResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub content_md5: [u8; 16],
    pub request_id: RequestId,
}

impl PutBlockBlobResponse {
    pub fn from_headers(headers: &HeaderMap) -> Result<PutBlockBlobResponse, AzureError> {
        let pbp = PutBlobResponse::from_headers(headers)?;

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

        Ok(PutBlockBlobResponse {
            etag: pbp.etag,
            last_modified: pbp.last_modified,
            content_md5,
            request_id: pbp.request_id,
        })
    }
}
