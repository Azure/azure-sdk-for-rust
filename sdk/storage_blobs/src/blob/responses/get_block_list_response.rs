use crate::blob::BlockWithSizeList;
use azure_core::{
    headers::{
        date_from_headers, etag_from_headers_optional, last_modified_from_headers_optional,
        request_id_from_headers,
    },
    RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::str::from_utf8;

#[derive(Debug, Clone, PartialEq)]
pub struct GetBlockListResponse {
    pub etag: Option<String>,
    pub last_modified: Option<DateTime<Utc>>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub block_with_size_list: BlockWithSizeList,
}

impl GetBlockListResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        body: &[u8],
    ) -> crate::Result<GetBlockListResponse> {
        let etag = etag_from_headers_optional(headers)?;
        let last_modified = last_modified_from_headers_optional(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        let body = from_utf8(body)?;
        let block_with_size_list = BlockWithSizeList::try_from_xml(&body[3..] as &str)?;

        Ok(GetBlockListResponse {
            etag,
            last_modified,
            request_id,
            date,
            block_with_size_list,
        })
    }
}
