use crate::blob::Blob;
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::{self, date_from_headers, request_id_from_headers},
    prelude::ContentRange,
    CollectedResponse, RequestId,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::{convert::TryFrom, str::FromStr};

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub request_id: RequestId,
    pub blob: Blob,
    pub data: Bytes,
    pub date: DateTime<Utc>,
    pub content_range: Option<ContentRange>,
}

impl TryFrom<(&str, CollectedResponse)> for GetBlobResponse {
    type Error = crate::Error;
    fn try_from((blob_name, response): (&str, CollectedResponse)) -> azure_core::Result<Self> {
        let request_id = request_id_from_headers(response.headers())?;
        let date = date_from_headers(response.headers())?;

        let content_range_header = response.headers().get(&headers::CONTENT_RANGE);
        let content_range = match content_range_header {
            Some(hv) => {
                Some(ContentRange::from_str(hv.as_str()).map_kind(ErrorKind::DataConversion)?)
            }
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
