use crate::blob::Blob;
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{date_from_headers, request_id_from_headers},
    prelude::ContentRange,
    RequestId,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::Response;
use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub request_id: RequestId,
    pub blob: Blob,
    pub data: Bytes,
    pub date: DateTime<Utc>,
    pub content_range: Option<ContentRange>,
}

impl TryFrom<(&str, Response<Bytes>)> for GetBlobResponse {
    type Error = crate::Error;
    fn try_from((blob_name, response): (&str, Response<Bytes>)) -> azure_core::Result<Self> {
        debug!("response.headers() == {:#?}", response.headers());

        let request_id = request_id_from_headers(response.headers())?;
        let date = date_from_headers(response.headers())?;

        let content_range_header = response.headers().get(http::header::CONTENT_RANGE);
        let content_range = match content_range_header {
            Some(hv) => Some(
                ContentRange::from_str(hv.to_str().map_kind(ErrorKind::DataConversion)?)
                    .map_kind(ErrorKind::DataConversion)?,
            ),
            None => None,
        };

        Ok(GetBlobResponse {
            request_id,
            blob: Blob::from_headers(blob_name, response.headers())?,
            data: response.into_body(),
            date,
            content_range,
        })
    }
}
