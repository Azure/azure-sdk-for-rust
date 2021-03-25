use crate::blob::blob::Blob;
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::RequestId;
use azure_core::{errors::AzureError, Streamable};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::Response;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetBlobResponse<T> {
    pub request_id: RequestId,
    pub blob: Blob,
    pub data: T,
    pub date: DateTime<Utc>,
    pub content_range: Option<String>,
}

impl TryFrom<(&str, Response<Bytes>)> for GetBlobResponse<Bytes> {
    type Error = AzureError;
    fn try_from((blob_name, response): (&str, Response<Bytes>)) -> Result<Self, Self::Error> {
        debug!("response.headers() == {:#?}", response.headers());

        Ok(GetBlobResponse {
            request_id: request_id_from_headers(response.headers())?,
            blob: Blob::from_headers(blob_name, response.headers())?,
            date: date_from_headers(response.headers())?,
            content_range: response
                .headers()
                .get(http::header::CONTENT_RANGE)
                .map(|h| h.to_str().unwrap().to_owned()),
            data: response.into_body(),
        })
    }
}

impl TryFrom<(&str, Streamable)> for GetBlobResponse<Streamable> {
    type Error = AzureError;
    fn try_from((blob_name, streamable): (&str, Streamable)) -> Result<Self, Self::Error> {
        println!("streamable.headers() == {:#?}", streamable.headers());

        Ok(GetBlobResponse {
            request_id: request_id_from_headers(streamable.headers())?,
            blob: Blob::from_headers(blob_name, streamable.headers())?,
            date: date_from_headers(streamable.headers())?,
            content_range: streamable
                .headers()
                .get(http::header::CONTENT_RANGE)
                .map(|h| h.to_str().unwrap().to_owned()),
            data: streamable,
        })
    }
}
