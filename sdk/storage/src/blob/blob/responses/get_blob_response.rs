use crate::{blob::blob::Blob, AzureStorageError};
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::RequestId;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::Response;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub request_id: RequestId,
    pub blob: Blob,
    pub data: Bytes,
    pub date: DateTime<Utc>,
    pub content_range: Option<String>,
}

impl TryFrom<(&str, Response<Bytes>)> for GetBlobResponse {
    type Error = AzureStorageError;
    fn try_from((blob_name, response): (&str, Response<Bytes>)) -> Result<Self, Self::Error> {
        debug!("response.headers() == {:#?}", response.headers());

        let request_id = request_id_from_headers(response.headers())?;
        let date = date_from_headers(response.headers())?;

        let content_range = response
            .headers()
            .get(http::header::CONTENT_RANGE)
            .map(|h| h.to_str().unwrap().to_owned());

        Ok(GetBlobResponse {
            request_id,
            blob: Blob::from_headers(blob_name, response.headers())?,
            data: response.into_body(),
            date,
            content_range,
        })
    }
}
