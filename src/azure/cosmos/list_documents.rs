use azure::cosmos::ConsistencyLevel;
use azure::core::incompletevector::ContinuationToken;

#[derive(Debug, Clone)]
pub struct ListDocumentsOptions<'a> {
    pub max_item_count: Option<u64>,
    pub continuation_token: Option<&'a ContinuationToken>,
    pub consistency_level_override: Option<ConsistencyLevel>,
    pub session_token: Option<&'a str>,
    pub incremental_feed: bool,
    pub if_none_match: Option<&'a str>,
    pub partition_range_id: Option<&'a str>,
}

pub const LIST_DOCUMENTS_OPTIONS_DEFAULT: ListDocumentsOptions = ListDocumentsOptions {
    max_item_count: None,
    continuation_token: None,
    consistency_level_override: None,
    session_token: None,
    incremental_feed: false,
    if_none_match: None,
    partition_range_id: None,
};

pub struct ListDocumentsResponseAdditionalHeaders {
    pub continuation_token: String,
    pub charge: u64,
    pub etag: String,
}
