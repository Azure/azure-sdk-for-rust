use crate::blob::PageRangeList;
use azure_core::{
    headers::{
        date_from_headers, etag_from_headers, last_modified_from_headers, request_id_from_headers,
    },
    RequestId,
};
use chrono::{DateTime, Utc};
use http::HeaderMap;
use std::str::from_utf8;

#[derive(Debug, Clone, PartialEq)]
pub struct GetPageRangesResponse {
    pub etag: String,
    pub last_modified: DateTime<Utc>,
    pub request_id: RequestId,
    pub date: DateTime<Utc>,
    pub page_list: PageRangeList,
}

impl GetPageRangesResponse {
    pub(crate) fn from_response(
        headers: &HeaderMap,
        body: &[u8],
    ) -> azure_core::Result<GetPageRangesResponse> {
        let etag = etag_from_headers(headers)?;
        let last_modified = last_modified_from_headers(headers)?;
        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        let body = from_utf8(body)?;
        let page_list = PageRangeList::try_from_xml(&body[3..] as &str)?;

        Ok(GetPageRangesResponse {
            etag,
            last_modified,
            request_id,
            date,
            page_list,
        })
    }
}
