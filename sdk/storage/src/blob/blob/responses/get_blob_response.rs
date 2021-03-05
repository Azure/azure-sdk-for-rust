use crate::blob::blob::Blob;
use azure_core::errors::AzureError;
use azure_core::headers::{date_from_headers, request_id_from_headers};
use azure_core::RequestId;
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::Response;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub request_id: RequestId,
    pub data: Bytes,
    pub date: DateTime<Utc>,
    pub content_range: Option<String>,
}

impl TryFrom<Response<Bytes>> for GetBlobResponse {
    type Error = AzureError;
    fn try_from(response: Response<Bytes>) -> Result<Self, Self::Error> {
        println!("response.headers() == {:#?}", response.headers());

        let request_id = request_id_from_headers(response.headers())?;
        let date = date_from_headers(response.headers())?;

        let content_range = response
            .headers()
            .get(http::header::CONTENT_RANGE)
            .map(|h| h.to_str().unwrap().to_owned());

        Ok(GetBlobResponse {
            request_id,
            data: response.into_body(),
            date,
            content_range,
        })
    }
}
