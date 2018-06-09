use azure::core::RequestId;
use azure::storage::blob::BlockWithSizeList;
use chrono::{DateTime, FixedOffset};
use hyper::header::ContentType;

#[derive(Debug, Clone)]
pub struct GetBlockListResponse {
    pub block_list: BlockWithSizeList<String>,
    pub last_modified: Option<DateTime<FixedOffset>>,
    pub etag: Option<String>,
    pub content_type: ContentType,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
}
