use azure::core::RequestId;
use azure::storage::blob::BlockWithSizeList;
use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone)]
pub struct GetBlockListResponse {
    pub block_list: BlockWithSizeList<String>,
    pub last_modified: Option<DateTime<FixedOffset>>,
    pub etag: Option<String>,
    pub content_type: String,
    pub request_id: RequestId,
    pub date: DateTime<FixedOffset>,
}
