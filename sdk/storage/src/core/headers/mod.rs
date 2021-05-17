use crate::{AzureStorageError, Consistency};
use http::HeaderMap;

pub const CONTENT_CRC64: &str = "x-ms-content-crc64";
pub const CONTENT_MD5: &str = "Content-MD5";
pub const COPY_ID: &str = "x-ms-copy-id";

pub fn content_crc64_from_headers(headers: &HeaderMap) -> Result<[u8; 8], AzureStorageError> {
    let content_crc64 = headers
        .get(CONTENT_CRC64)
        .ok_or_else(|| AzureStorageError::HeaderNotFound(CONTENT_CRC64.to_owned()))?
        .to_str()?;

    let content_crc64_vec = base64::decode(&content_crc64)?;

    if content_crc64_vec.len() != 8 {
        return Err(AzureStorageError::CRC64Not8BytesLong(
            content_crc64_vec.len() as u64,
        ));
    }
    let mut content_crc64 = [0; 8];
    content_crc64.copy_from_slice(&content_crc64_vec[0..8]);

    trace!("content_crc64 == {:?}", content_crc64);
    Ok(content_crc64)
}

pub fn content_crc64_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<[u8; 8]>, AzureStorageError> {
    if headers.contains_key(CONTENT_CRC64) {
        Ok(Some(content_crc64_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn content_md5_from_headers(headers: &HeaderMap) -> Result<[u8; 16], AzureStorageError> {
    let content_md5 = headers
        .get(CONTENT_MD5)
        .ok_or_else(|| AzureStorageError::HeaderNotFound(CONTENT_MD5.to_owned()))?
        .to_str()?;

    let content_md5_vec = base64::decode(&content_md5)?;

    if content_md5_vec.len() != 16 {
        return Err(AzureStorageError::DigestNot16BytesLong(
            content_md5_vec.len() as u64,
        ));
    }
    let mut content_md5 = [0; 16];
    content_md5.copy_from_slice(&content_md5_vec[0..16]);

    trace!("content_md5 == {:?}", content_md5);
    Ok(content_md5)
}

pub fn content_md5_from_headers_optional(
    headers: &HeaderMap,
) -> Result<Option<[u8; 16]>, AzureStorageError> {
    if headers.contains_key(CONTENT_MD5) {
        Ok(Some(content_md5_from_headers(headers)?))
    } else {
        Ok(None)
    }
}

pub fn consistency_from_headers(headers: &HeaderMap) -> Result<Consistency, AzureStorageError> {
    if let Some(content_crc64) = content_crc64_from_headers_optional(headers)? {
        return Ok(Consistency::Crc64(content_crc64));
    } else if let Some(content_md5) = content_md5_from_headers_optional(headers)? {
        return Ok(Consistency::Md5(content_md5));
    }

    Err(AzureStorageError::HeadersNotFound(vec![
        CONTENT_CRC64.to_owned(),
        CONTENT_MD5.to_owned(),
    ]))
}
